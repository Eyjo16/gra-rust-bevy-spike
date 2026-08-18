//! Exactly ten bounded oracles.
//!
//! Each oracle is a pure check over bounded inputs — the current world,
//! the receipt log, and a replayable fixture — and returns a verdict.
//! `run_all` returns a fixed-size array, so the count of ten is enforced
//! by the type system.
//!
//! Oracles 1–2 audit state, 3–6 audit the receipt log, 7 replays the whole
//! trial through the real implementation, 8 checks the hash chain and that
//! refusals mutate nothing, and 9–10 recompute the trial with an
//! independent shadow evaluator that never trusts receipt fields: 9 checks
//! every expected receipt, 10 checks the final world state itself — so a
//! receipt lie that is internally consistent (passing 3–6) fails 9, and a
//! final world diverging from the commands fails 10 even when run and
//! replay share the same bug (which satisfies 7).

use std::collections::BTreeMap;

use crate::boundary::{
    Command, GIVE_COST, KIND_COUNT, MassGrams, OutcomeKind, Receipt, ResourceKind,
    STAMINA_COST_BY_BAND, Stamina, StaminaBand, Verb, WITNESS_COST, World, YIELD_TABLE_GRAMS,
    submit,
};

pub const ORACLE_COUNT: usize = 10;

/// Verifier version for the proof envelope. The oracle *count* is
/// type-enforced; this constant records which judge evaluated a run and
/// must be bumped on any change to an oracle's behavior or to the set of
/// oracles — a run's envelope then names both the language it was judged
/// in (grammar fingerprint) and the judge that evaluated it.
/// v2: added oracle 10 `shadow_final_state`.
/// v3: oracle 7 compares the exact canonical final-state serialization,
/// not only the hash — hash equality is checksum evidence, not state
/// equality.
/// v4: oracle 2's mass total is exact checked arithmetic under the
/// coherence-validated aggregate bound; saturation can no longer make a
/// mass-loss defect appear conserved.
/// v5 (RES01): oracle 2 checks conservation PER KIND as well as in
/// aggregate. Strictly stronger — the aggregate check passes on a world
/// where a gram of fodder became a gram of timber, and this one does not.
/// v6 (V01): oracle 3 `witnessed_gate` becomes `mass_authority_gate` —
/// its old clause is unchanged in force (no receipt moves mass out of a
/// site without a witnessed claim) and it gains a transfer clause (a
/// mass-moving receipt with no site must name a counterparty distinct
/// from the actor, and a kind). Oracle 4 keys its exhausted gate on site
/// extraction, which is the gather verb's policy made explicit rather
/// than a weakening: give follows the witness policy of a flat cost with
/// no exhausted gate.
pub const ORACLE_SUITE_VERSION: u32 = 6;

pub struct OracleVerdict {
    pub name: &'static str,
    pub pass: bool,
    pub detail: String,
}

impl OracleVerdict {
    fn new(name: &'static str, pass: bool, detail: String) -> Self {
        Self { name, pass, detail }
    }
}

/// The fixture's per-kind mass baseline, indexed by `ResourceKind::index`.
pub type KindBaseline = [MassGrams; KIND_COUNT];

/// The per-kind baseline of a seeded fixture — the value oracle 2 audits
/// every later state against.
pub fn baseline_by_kind(world: &World) -> KindBaseline {
    let mut baseline = [MassGrams::ZERO; KIND_COUNT];
    for kind in ResourceKind::ALL {
        baseline[kind.index()] = world.economy.total_mass_of(kind);
    }
    baseline
}

pub struct OracleCtx<'a> {
    pub world: &'a World,
    pub baseline_by_kind: KindBaseline,
    pub build_fixture: fn() -> World,
    pub commands: &'a [Command],
    pub log: &'a [Receipt],
}

pub fn run_all(ctx: &OracleCtx<'_>) -> [OracleVerdict; ORACLE_COUNT] {
    [
        stamina_in_bounds(ctx),
        mass_conserved(ctx),
        mass_authority_gate(ctx),
        exhausted_gate(ctx),
        closed_reasons(ctx),
        cell_bounds(ctx),
        replay_determinism(ctx),
        refusal_zero_mutation(ctx),
        shadow_expectation(ctx),
        shadow_final_state(ctx),
    ]
}

/// 1. Every character's stamina stays within `0..=Stamina::MAX`.
fn stamina_in_bounds(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations: Vec<String> = ctx
        .world
        .characters
        .iter()
        .filter(|(_, stamina)| stamina.points() > Stamina::MAX)
        .map(|(id, stamina)| format!("C{}={}", id.0, stamina.points()))
        .collect();
    OracleVerdict::new(
        "stamina_in_bounds",
        violations.is_empty(),
        if violations.is_empty() {
            "all stamina within 0..=100".to_owned()
        } else {
            violations.join(",")
        },
    )
}

/// 2. Exact mass conservation, per kind and in aggregate (RES01). Every
///    kind's total across sites and holdings equals its own fixture
///    baseline, and the sum of the kinds equals the whole. The aggregate
///    check alone cannot see a gram of fodder becoming a gram of timber;
///    the per-kind check can. Fixture coherence proves the sums fit
///    `u64`, so no saturating arithmetic can make a mass loss look
///    conserved.
fn mass_conserved(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let mut detail = Vec::with_capacity(KIND_COUNT + 1);
    let mut pass = true;
    let mut baseline_total = MassGrams::ZERO;
    for kind in ResourceKind::ALL {
        let baseline = ctx.baseline_by_kind[kind.index()];
        let current = ctx.world.economy.total_mass_of(kind);
        pass &= current == baseline;
        baseline_total = baseline_total
            .checked_add(baseline)
            .expect("coherent world: baseline total fits u64");
        detail.push(format!(
            "{}={}g/{}g",
            kind.code(),
            current.grams(),
            baseline.grams()
        ));
    }
    let current_total = ctx.world.economy.total_mass();
    pass &= current_total == baseline_total;
    detail.push(format!(
        "total={}g/{}g",
        current_total.grams(),
        baseline_total.grams()
    ));
    OracleVerdict::new("mass_conserved", pass, detail.join(" "))
}

/// 3. Every mass movement had authority for it, of the kind its shape
///    requires (V01; named `witnessed_gate` through v5).
///    - Extraction (the receipt names a site): the claim must have been
///      witnessed. Unchanged in force from v5.
///    - Transfer (mass moved, no site): the receipt must name a
///      counterparty distinct from the actor, and a kind — otherwise
///      mass moved with nobody to have consented to it.
///
///    Keyed on actual mass movement, not outcome kind: the witness verb
///    is Accepted with zero mass, which the second verb exposed as a
///    distinct case.
fn mass_authority_gate(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let unwitnessed_extractions = ctx
        .log
        .iter()
        .filter(|r| !r.mass_moved.is_zero() && r.site.is_some() && !r.witnessed)
        .count();
    let unconsented_transfers = ctx
        .log
        .iter()
        .filter(|r| !r.mass_moved.is_zero() && r.site.is_none())
        .filter(|r| match (r.recipient, r.kind) {
            (Some(recipient), Some(_)) => recipient == r.actor,
            _ => true,
        })
        .count();
    let violations = unwitnessed_extractions + unconsented_transfers;
    OracleVerdict::new(
        "mass_authority_gate",
        violations == 0,
        format!(
            "{unwitnessed_extractions} unwitnessed extractions, {unconsented_transfers} unconsented transfers"
        ),
    )
}

/// 4. An exhausted actor never *extracts* mass: every receipt that
///    drains a site sits in a non-exhausted stamina band. Keyed on site
///    extraction (V01): the exhausted gate has always been the gather
///    verb's policy, and both other verbs deliberately lack it — an
///    exhausted character may still witness a claim (zero mass) and may
///    still hand over what they already hold, which moves mass between
///    holdings and drains no site.
fn exhausted_gate(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations = ctx
        .log
        .iter()
        .filter(|r| !r.mass_moved.is_zero() && r.site.is_some())
        .filter(|r| !matches!(r.band, Some(band) if band != StaminaBand::Exhausted))
        .count();
    OracleVerdict::new(
        "exhausted_gate",
        violations == 0,
        format!("{violations} exhausted or band-less receipts drained a site"),
    )
}

/// 5. Outcome reasons are closed: every receipt's codes round-trip through
///    the closed enums.
fn closed_reasons(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations = ctx
        .log
        .iter()
        .filter(|r| !r.outcome.codes_round_trip())
        .count();
    OracleVerdict::new(
        "closed_reasons",
        violations == 0,
        format!("{violations} receipts with unclosed reason codes"),
    )
}

/// 6. Every yield stays inside the single active 4x4 cell: mass moved never
///    exceeds the table value for the recorded band and tier, and an
///    Accepted outcome matches it exactly. The cell belongs to the gather
///    verb; witness receipts never move mass (oracle 9 enforces that
///    independently).
fn cell_bounds(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations = ctx
        .log
        .iter()
        .filter(|r| r.verb == Verb::Gather && r.outcome.yields_mass())
        .filter(|r| {
            let (Some(band), Some(tier)) = (r.band, r.tier) else {
                return true;
            };
            let cell = YIELD_TABLE_GRAMS[band.index()][tier.index()];
            match r.outcome {
                OutcomeKind::Accepted => r.mass_moved.grams() != cell,
                _ => r.mass_moved.grams() >= cell || r.mass_moved.is_zero(),
            }
        })
        .count();
    OracleVerdict::new(
        "cell_bounds",
        violations == 0,
        format!("{violations} receipts outside the 4x4 cell"),
    )
}

/// 7. Determinism: replaying the same fixture and commands through the
///    real implementation reproduces the same receipts, the same exact
///    canonical final state, and the same hash. The exact serialization
///    carries the equality claim; the hash is its checksum address.
fn replay_determinism(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let mut replay_world = (ctx.build_fixture)();
    let replay_lines: Vec<String> = ctx
        .commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| submit(&mut replay_world, i as u64 + 1, *cmd).canonical_line())
        .collect();
    let original_lines: Vec<String> = ctx.log.iter().map(Receipt::canonical_line).collect();
    let states_match = replay_world.canonical_state() == ctx.world.canonical_state();
    let hashes_match = replay_world.hash() == ctx.world.hash();
    let lines_match = replay_lines == original_lines;
    OracleVerdict::new(
        "replay_determinism",
        states_match && hashes_match && lines_match,
        format!(
            "states_match={states_match} hashes_match={hashes_match} receipts_match={lines_match}"
        ),
    )
}

/// 8. The hash chain holds and refusals are byte-identical no-ops: each
///    receipt's before-hash equals the previous after-hash (starting from
///    the fixture hash), every Refused receipt leaves the hash unchanged,
///    and every mass-moving receipt changes it.
fn refusal_zero_mutation(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let fixture_hash = (ctx.build_fixture)().hash();
    let mut violations = 0usize;
    let mut expected_before = fixture_hash;
    for receipt in ctx.log {
        if receipt.world_hash_before != expected_before {
            violations += 1;
        }
        match receipt.outcome {
            OutcomeKind::Refused(_) => {
                if receipt.world_hash_after != receipt.world_hash_before {
                    violations += 1;
                }
            }
            _ => {
                if receipt.world_hash_after == receipt.world_hash_before {
                    violations += 1;
                }
            }
        }
        expected_before = receipt.world_hash_after;
    }
    OracleVerdict::new(
        "refusal_zero_mutation",
        violations == 0,
        format!("{violations} hash-chain or mutation violations"),
    )
}

/// 9. Independent expectation: a shadow evaluator recomputes every step
///    from the immutable fixture and inputs — its own state tracking, its
///    own band thresholds, the shared spec tables — and compares against
///    the receipts. It never trusts a receipt field, so an internally
///    consistent receipt lie still fails here.
fn shadow_expectation(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let mut shadow = ShadowState::from_fixture(&(ctx.build_fixture)());
    let mut violations = 0usize;
    if ctx.log.len() != ctx.commands.len() {
        violations += 1;
    }
    for (cmd, receipt) in ctx.commands.iter().zip(ctx.log) {
        let expected = shadow.step(cmd);
        if !expected.matches(receipt) {
            violations += 1;
        }
    }
    OracleVerdict::new(
        "shadow_expectation",
        violations == 0,
        format!("{violations} receipts diverge from the shadow evaluator"),
    )
}

/// 10. Independent final-state proof: after stepping every command, the
///     shadow evaluator's final state must equal the actual final world —
///     stamina, per-kind holdings, site stocks and kinds, and claim gates.
///     `replay_determinism` trusts the implementation twice (run and
///     replay); this oracle trusts it zero times, so a bug shared by run
///     and replay still fails here.
fn shadow_final_state(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let mut shadow = ShadowState::from_fixture(&(ctx.build_fixture)());
    for cmd in ctx.commands {
        let _ = shadow.step(cmd);
    }
    let violations = shadow.final_state_divergences(ctx.world);
    OracleVerdict::new(
        "shadow_final_state",
        violations == 0,
        format!("{violations} truth domains diverge from the shadow final state"),
    )
}

/// Independent re-interpretation of the grammar. Deliberately does not use
/// the owners, the boundary orchestrator, `Stamina::band`, or any receipt
/// field — plain integers, its own threshold literals, and the shared spec
/// tables only.
struct ShadowState {
    stamina: BTreeMap<u64, u8>,
    /// Keyed by (character, kind index) — zero-valued entries are never
    /// stored, matching the owner's normalization, so the comparison in
    /// oracle 10 is between two canonical maps.
    holdings: BTreeMap<(u64, usize), u64>,
    sites: BTreeMap<u64, (usize, usize, u64)>,
    claims: BTreeMap<u64, (u64, u64, bool)>,
}

struct ShadowExpectation {
    verb_code: &'static str,
    outcome_code: &'static str,
    reason_code: &'static str,
    witnessed: bool,
    spent: u8,
    mass_grams: u64,
    band_index: Option<usize>,
    tier_index: Option<usize>,
    /// Recomputed from the fixture, never read from the receipt, and
    /// compared on every receipt — including refusals, where the kind is
    /// the addressed site's kind and a wrong one is still a lie.
    kind_index: Option<usize>,
}

fn shadow_band_index(points: u8) -> usize {
    match points {
        0..=9 => 0,
        10..=39 => 1,
        40..=79 => 2,
        _ => 3,
    }
}

impl ShadowState {
    /// Reads only the fixture's seeded data through read-only iterators.
    fn from_fixture(fixture: &World) -> Self {
        Self {
            stamina: fixture
                .characters
                .iter()
                .map(|(id, s)| (id.0, s.points()))
                .collect(),
            holdings: fixture
                .economy
                .holdings_iter()
                .map(|(id, kind, grams)| ((id.0, kind.index()), grams.grams()))
                .collect(),
            sites: fixture
                .economy
                .sites_iter()
                .map(|(id, tier, kind, stock)| (id.0, (tier.index(), kind.index(), stock.grams())))
                .collect(),
            claims: fixture
                .social
                .claims_iter()
                .map(|(id, holder, site, witnessed)| (id.0, (holder.0, site.0, witnessed)))
                .collect(),
        }
    }

    /// Mirrors the owner's zero-normalization: a holding that reaches
    /// zero is removed, so two states that print alike compare alike.
    fn add_holding(&mut self, id: u64, kind: usize, grams: u64) {
        let entry = self.holdings.entry((id, kind)).or_insert(0);
        *entry += grams;
        if *entry == 0 {
            self.holdings.remove(&(id, kind));
        }
    }

    fn step(&mut self, cmd: &Command) -> ShadowExpectation {
        match cmd {
            Command::Gather(gather) => self.step_gather(gather),
            Command::Witness(witness) => self.step_witness(witness),
            Command::Give(give) => self.step_give(give),
        }
    }

    fn step_gather(&mut self, cmd: &crate::boundary::GatherCommand) -> ShadowExpectation {
        let witnessed = self
            .claims
            .get(&cmd.claim.0)
            .is_some_and(|(_, _, flag)| *flag);
        let site_kind = self.sites.get(&cmd.site.0).map(|(_, kind, _)| *kind);
        let refuse = |reason: &'static str| ShadowExpectation {
            verb_code: "gather",
            outcome_code: "refused",
            reason_code: reason,
            witnessed,
            spent: 0,
            mass_grams: 0,
            band_index: None,
            tier_index: None,
            kind_index: site_kind,
        };

        // Gate 1: social.
        let Some(&(holder, claim_site, claim_witnessed)) = self.claims.get(&cmd.claim.0) else {
            return refuse("unknown_claim");
        };
        if holder != cmd.actor.0 {
            return refuse("claim_not_held_by_actor");
        }
        if claim_site != cmd.site.0 {
            return refuse("claim_site_mismatch");
        }
        if !claim_witnessed {
            return refuse("claim_not_witnessed");
        }
        // Gate 2: character.
        let Some(&points) = self.stamina.get(&cmd.actor.0) else {
            return refuse("unknown_actor");
        };
        let band = shadow_band_index(points);
        if band == 0 {
            return refuse("actor_exhausted");
        }
        let cost = STAMINA_COST_BY_BAND[band];
        if points < cost {
            return refuse("insufficient_stamina");
        }
        // Gate 3: economy and the 4x4 cell.
        let Some(&(tier, kind, stock)) = self.sites.get(&cmd.site.0) else {
            return refuse("unknown_site");
        };
        if stock == 0 {
            return refuse("site_empty");
        }
        let requested = YIELD_TABLE_GRAMS[band][tier];
        let granted = requested.min(stock);
        // Apply to shadow state. The grant lands in the holding of the
        // site's kind — the shadow decides that independently, so a
        // boundary that leaked across kinds diverges here.
        self.stamina.insert(cmd.actor.0, points - cost);
        self.sites.insert(cmd.site.0, (tier, kind, stock - granted));
        self.add_holding(cmd.actor.0, kind, granted);
        let (outcome_code, reason_code) = if granted < requested {
            ("partial", "site_nearly_depleted")
        } else {
            ("accepted", "-")
        };
        ShadowExpectation {
            verb_code: "gather",
            outcome_code,
            reason_code,
            witnessed,
            spent: cost,
            mass_grams: granted,
            band_index: Some(band),
            tier_index: Some(tier),
            kind_index: Some(kind),
        }
    }

    /// Independent reimplementation of the witness verb: flat cost, no
    /// exhausted gate, social-then-character order, economy untouched.
    fn step_witness(&mut self, cmd: &crate::boundary::WitnessCommand) -> ShadowExpectation {
        let witnessed = self
            .claims
            .get(&cmd.claim.0)
            .is_some_and(|(_, _, flag)| *flag);
        let refuse = |reason: &'static str| ShadowExpectation {
            verb_code: "witness",
            outcome_code: "refused",
            reason_code: reason,
            witnessed,
            spent: 0,
            mass_grams: 0,
            band_index: None,
            tier_index: None,
            kind_index: None,
        };

        // Gate 1: social.
        let Some(&(holder, claim_site, claim_witnessed)) = self.claims.get(&cmd.claim.0) else {
            return refuse("unknown_claim");
        };
        if holder == cmd.witness.0 {
            return refuse("cannot_witness_own_claim");
        }
        if claim_witnessed {
            return refuse("claim_already_witnessed");
        }
        // Gate 2: character — flat cost, no exhausted gate.
        let Some(&points) = self.stamina.get(&cmd.witness.0) else {
            return refuse("unknown_actor");
        };
        if points < WITNESS_COST {
            return refuse("insufficient_stamina");
        }
        // Apply to shadow state: economy untouched.
        self.stamina.insert(cmd.witness.0, points - WITNESS_COST);
        self.claims.insert(cmd.claim.0, (holder, claim_site, true));
        ShadowExpectation {
            verb_code: "witness",
            outcome_code: "accepted",
            reason_code: "-",
            witnessed,
            spent: WITNESS_COST,
            mass_grams: 0,
            band_index: None,
            tier_index: None,
            kind_index: None,
        }
    }

    /// Independent reimplementation of the give verb: parties gate, then
    /// flat cost with no exhausted gate, then an exact holding check.
    /// Uses its own literals and its own state — never the boundary's
    /// plan, never a receipt field.
    fn step_give(&mut self, cmd: &crate::boundary::GiveCommand) -> ShadowExpectation {
        let witnessed = cmd.witness.is_some();
        let kind = cmd.kind.index();
        let refuse = |reason: &'static str| ShadowExpectation {
            verb_code: "give",
            outcome_code: "refused",
            reason_code: reason,
            witnessed,
            spent: 0,
            mass_grams: 0,
            band_index: None,
            tier_index: None,
            kind_index: Some(kind),
        };

        // Gate 1: parties.
        if cmd.giver == cmd.recipient {
            return refuse("cannot_give_to_self");
        }
        if !self.stamina.contains_key(&cmd.recipient.0) {
            return refuse("unknown_recipient");
        }
        if let Some(witness) = cmd.witness {
            if !self.stamina.contains_key(&witness.0) {
                return refuse("unknown_witness");
            }
            if witness == cmd.giver || witness == cmd.recipient {
                return refuse("witness_is_party");
            }
        }
        if cmd.grams.grams() == 0 {
            return refuse("empty_transfer");
        }
        // Gate 2: character — flat cost, no exhausted gate.
        let Some(&points) = self.stamina.get(&cmd.giver.0) else {
            return refuse("unknown_actor");
        };
        if points < GIVE_COST {
            return refuse("insufficient_stamina");
        }
        // Gate 3: economy — exact, never partial.
        let held = self
            .holdings
            .get(&(cmd.giver.0, kind))
            .copied()
            .unwrap_or(0);
        let grams = cmd.grams.grams();
        if held < grams {
            return refuse("insufficient_holding");
        }
        // Apply to shadow state, with the same zero-normalization the
        // owner uses, so oracle 10 compares two canonical maps.
        self.stamina.insert(cmd.giver.0, points - GIVE_COST);
        if held == grams {
            self.holdings.remove(&(cmd.giver.0, kind));
        } else {
            self.holdings.insert((cmd.giver.0, kind), held - grams);
        }
        self.add_holding(cmd.recipient.0, kind, grams);
        ShadowExpectation {
            verb_code: "give",
            outcome_code: "accepted",
            reason_code: "-",
            witnessed,
            spent: GIVE_COST,
            mass_grams: grams,
            band_index: None,
            tier_index: None,
            kind_index: Some(kind),
        }
    }

    /// Counts truth domains (stamina, inventories, sites, claims) where
    /// the actual world differs from the shadow's final state. Reads the
    /// world only through the owners' read-only iterators — never a
    /// receipt, never the implementation's replay.
    fn final_state_divergences(&self, world: &World) -> usize {
        let actual_stamina: BTreeMap<u64, u8> = world
            .characters
            .iter()
            .map(|(id, s)| (id.0, s.points()))
            .collect();
        let actual_holdings: BTreeMap<(u64, usize), u64> = world
            .economy
            .holdings_iter()
            .map(|(id, kind, grams)| ((id.0, kind.index()), grams.grams()))
            .collect();
        let actual_sites: BTreeMap<u64, (usize, usize, u64)> = world
            .economy
            .sites_iter()
            .map(|(id, tier, kind, stock)| (id.0, (tier.index(), kind.index(), stock.grams())))
            .collect();
        let actual_claims: BTreeMap<u64, (u64, u64, bool)> = world
            .social
            .claims_iter()
            .map(|(id, holder, site, witnessed)| (id.0, (holder.0, site.0, witnessed)))
            .collect();
        usize::from(actual_stamina != self.stamina)
            + usize::from(actual_holdings != self.holdings)
            + usize::from(actual_sites != self.sites)
            + usize::from(actual_claims != self.claims)
    }
}

impl ShadowExpectation {
    fn matches(&self, receipt: &Receipt) -> bool {
        let codes_match = receipt.verb.code() == self.verb_code
            && receipt.outcome.code() == self.outcome_code
            && receipt.outcome.reason_code() == self.reason_code
            && receipt.witnessed == self.witnessed
            && receipt.stamina_spent == self.spent
            && receipt.mass_moved.grams() == self.mass_grams;
        // Band/tier on refusals are informational; on yields they are part
        // of the claim being audited.
        let cell_match = match (self.band_index, self.tier_index) {
            (Some(band), Some(tier)) => {
                receipt.band.map(|b| b.index()) == Some(band)
                    && receipt.tier.map(|t| t.index()) == Some(tier)
            }
            _ => true,
        };
        // The kind is compared on EVERY receipt, refusals included: the
        // shadow derives it from the fixture, so a receipt naming a kind
        // its site does not yield is caught whether or not mass moved.
        let kind_match = receipt.kind.map(|k| k.index()) == self.kind_index;
        codes_match && cell_match && kind_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::{
        CharacterId, ClaimId, GatherCommand, InfraTier, PartialReason, RefusalReason, SiteId,
        WitnessCommand, grammar_fingerprint, receipt_chain_digest, validate_world_coherence,
    };
    use crate::character::CharacterOwner;
    use crate::economy::EconomyOwner;
    use crate::social::SocialOwner;

    // Fixture numbers are mechanical examples only — not balance.
    fn fixture() -> World {
        World {
            characters: CharacterOwner::seed([
                (CharacterId(1), Stamina::new(90).unwrap()),
                (CharacterId(2), Stamina::new(25).unwrap()),
                (CharacterId(3), Stamina::new(5).unwrap()),
            ])
            .unwrap(),
            economy: EconomyOwner::seed_sites([
                (
                    SiteId(1),
                    InfraTier::Established,
                    ResourceKind::Fodder,
                    MassGrams::new(2000),
                ),
                (
                    SiteId(2),
                    InfraTier::Crude,
                    ResourceKind::Timber,
                    MassGrams::new(300),
                ),
            ])
            .unwrap(),
            social: SocialOwner::seed_claims([
                (ClaimId(1), CharacterId(1), SiteId(1), true),
                (ClaimId(2), CharacterId(2), SiteId(2), true),
                (ClaimId(3), CharacterId(2), SiteId(1), false),
                (ClaimId(4), CharacterId(3), SiteId(2), true),
            ])
            .unwrap(),
        }
    }

    fn commands() -> Vec<Command> {
        let gather = |actor, claim, site| {
            Command::Gather(GatherCommand {
                actor: CharacterId(actor),
                claim: ClaimId(claim),
                site: SiteId(site),
            })
        };
        let witness = |witness, claim| {
            Command::Witness(WitnessCommand {
                witness: CharacterId(witness),
                claim: ClaimId(claim),
            })
        };
        vec![
            gather(1, 1, 1),
            gather(2, 2, 2),
            gather(2, 3, 1),
            gather(3, 4, 2),
            witness(1, 3),
        ]
    }

    fn run_fixture() -> (World, Vec<Command>, Vec<Receipt>, KindBaseline) {
        let mut world = fixture();
        validate_world_coherence(&world).unwrap();
        let baseline = baseline_by_kind(&world);
        let cmds = commands();
        let log: Vec<Receipt> = cmds
            .iter()
            .enumerate()
            .map(|(i, cmd)| submit(&mut world, i as u64 + 1, *cmd))
            .collect();
        (world, cmds, log, baseline)
    }

    #[test]
    fn fixture_run_covers_accepted_partial_and_refused() {
        let (_, _, log, _) = run_fixture();
        assert_eq!(log[0].outcome, OutcomeKind::Accepted);
        assert_eq!(
            log[1].outcome,
            OutcomeKind::Partial(PartialReason::SiteNearlyDepleted)
        );
        assert_eq!(
            log[2].outcome,
            OutcomeKind::Refused(RefusalReason::ClaimNotWitnessed)
        );
        assert_eq!(
            log[3].outcome,
            OutcomeKind::Refused(RefusalReason::ActorExhausted)
        );
        assert_eq!(log[4].verb, Verb::Witness);
        assert_eq!(log[4].outcome, OutcomeKind::Accepted);
        assert!(!log[4].witnessed, "claim was unwitnessed before the verb");
        assert_eq!(log[4].mass_moved, MassGrams::ZERO);
    }

    #[test]
    fn all_ten_oracles_pass_on_the_fixture_run() {
        let (world, cmds, log, baseline) = run_fixture();
        let ctx = OracleCtx {
            world: &world,
            baseline_by_kind: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        let verdicts = run_all(&ctx);
        assert_eq!(verdicts.len(), ORACLE_COUNT);
        for verdict in &verdicts {
            assert!(verdict.pass, "{} failed: {}", verdict.name, verdict.detail);
        }
    }

    #[test]
    fn every_receipt_carries_the_grammar_fingerprint() {
        let (_, _, log, _) = run_fixture();
        let fingerprint = grammar_fingerprint();
        assert!(log.iter().all(|r| r.grammar == fingerprint));
    }

    #[test]
    fn dangling_claim_is_a_fixture_fault() {
        let world = World {
            characters: CharacterOwner::seed([(CharacterId(1), Stamina::new(90).unwrap())])
                .unwrap(),
            economy: EconomyOwner::seed_sites([(
                SiteId(1),
                InfraTier::Crude,
                ResourceKind::Fodder,
                MassGrams::new(100),
            )])
            .unwrap(),
            social: SocialOwner::seed_claims([(ClaimId(1), CharacterId(99), SiteId(1), true)])
                .unwrap(),
        };
        assert!(validate_world_coherence(&world).is_err());
    }

    /// Falsifier F2 (RES01), oracle form: a world where 300 g of fodder
    /// became 300 g of timber conserves mass in aggregate and must still
    /// fail conservation. The v4 oracle could not see this; v5 must.
    #[test]
    fn falsification_kind_swap_must_fail_conservation_at_equal_total() {
        let (world, cmds, log, baseline) = run_fixture();
        let mut swapped = baseline;
        swapped[ResourceKind::Fodder.index()] =
            MassGrams::new(baseline[ResourceKind::Fodder.index()].grams() + 300);
        swapped[ResourceKind::Timber.index()] =
            MassGrams::new(baseline[ResourceKind::Timber.index()].grams() - 300);
        let aggregate_before: u64 = baseline.iter().map(|m| m.grams()).sum();
        let aggregate_after: u64 = swapped.iter().map(|m| m.grams()).sum();
        assert_eq!(
            aggregate_before, aggregate_after,
            "the staged swap must be invisible to an aggregate-only check"
        );
        let ctx = OracleCtx {
            world: &world,
            baseline_by_kind: swapped,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        assert!(
            !mass_conserved(&ctx).pass,
            "a kind swap at an equal total passed conservation"
        );
    }

    /// Falsifier F3 (RES01), shadow form: a receipt that names a kind its
    /// site does not yield is internally consistent — mass, band, tier
    /// and codes all agree — and only an independent recomputation
    /// refuses it.
    #[test]
    fn falsification_shadow_oracle_catches_a_mislabelled_kind() {
        let (world, cmds, mut log, baseline) = run_fixture();
        log[0].kind = Some(ResourceKind::Timber);
        let ctx = OracleCtx {
            world: &world,
            baseline_by_kind: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        assert!(cell_bounds(&ctx).pass);
        assert!(mass_conserved(&ctx).pass);
        assert!(
            !shadow_expectation(&ctx).pass,
            "a mislabelled kind survived the shadow evaluator"
        );
    }

    #[test]
    fn mass_authority_gate_oracle_catches_a_doctored_receipt() {
        let (world, cmds, mut log, baseline) = run_fixture();
        log[0].witnessed = false;
        let ctx = OracleCtx {
            world: &world,
            baseline_by_kind: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        assert!(!mass_authority_gate(&ctx).pass);
    }

    /// A consistent lie: the receipt claims the actor was in the Low band
    /// and moved exactly the Low x Established cell value, with matching
    /// spent. Every receipt-trusting oracle (3–6) accepts it; only an
    /// independent recomputation can refuse it.
    #[test]
    fn falsification_shadow_oracle_catches_a_consistent_receipt_lie() {
        let (world, cmds, mut log, baseline) = run_fixture();
        log[0].band = Some(StaminaBand::Low);
        log[0].mass_moved = MassGrams::new(600);
        log[0].stamina_spent = 15;
        let ctx = OracleCtx {
            world: &world,
            baseline_by_kind: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        assert!(mass_authority_gate(&ctx).pass);
        assert!(exhausted_gate(&ctx).pass);
        assert!(closed_reasons(&ctx).pass);
        assert!(cell_bounds(&ctx).pass);
        assert!(!shadow_expectation(&ctx).pass);
    }

    #[test]
    fn falsification_zero_mutation_oracle_catches_a_broken_chain() {
        let (world, cmds, mut log, baseline) = run_fixture();
        log[2].world_hash_after = log[2].world_hash_after.wrapping_add(1);
        let ctx = OracleCtx {
            world: &world,
            baseline_by_kind: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        assert!(!refusal_zero_mutation(&ctx).pass);
    }

    #[test]
    fn receipt_chain_digest_is_reproducible_and_tamper_sensitive() {
        let (_, _, log_a, _) = run_fixture();
        let (_, _, log_b, _) = run_fixture();
        assert_eq!(receipt_chain_digest(&log_a), receipt_chain_digest(&log_b));
        let mut doctored = log_a.clone();
        doctored[0].stamina_spent += 1;
        assert_ne!(
            receipt_chain_digest(&log_a),
            receipt_chain_digest(&doctored)
        );
    }

    #[test]
    fn two_runs_produce_identical_hashes_and_receipts() {
        let (world_a, _, log_a, _) = run_fixture();
        let (world_b, _, log_b, _) = run_fixture();
        assert_eq!(world_a.hash(), world_b.hash());
        let lines_a: Vec<String> = log_a.iter().map(Receipt::canonical_line).collect();
        let lines_b: Vec<String> = log_b.iter().map(Receipt::canonical_line).collect();
        assert_eq!(lines_a, lines_b);
    }

    /// Falsifier for trial/004. A final world that diverges from what the
    /// logged commands produce — here via one extra command applied after
    /// the logged trial — must fail at least one oracle that does NOT
    /// replay through the implementation under audit. replay_determinism
    /// runs the same `submit` twice, so an implementation whose run and
    /// replay share a bug satisfies it; final-state truth needs a judge
    /// that trusts the implementation zero times.
    #[test]
    fn falsification_divergent_final_world_must_fail_an_independent_oracle() {
        let (mut world, cmds, log, baseline) = run_fixture();
        // Reachable divergence: one more gather, absent from the log.
        // Mass is conserved (site stock moves to inventory), so oracle 2
        // cannot see it either.
        submit(
            &mut world,
            99,
            Command::Gather(GatherCommand {
                actor: CharacterId(1),
                claim: ClaimId(1),
                site: SiteId(1),
            }),
        );
        let ctx = OracleCtx {
            world: &world,
            baseline_by_kind: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        let independent_failures = run_all(&ctx)
            .iter()
            .filter(|v| v.name != "replay_determinism" && !v.pass)
            .count();
        assert!(
            independent_failures > 0,
            "divergent final world was visible only to the self-trusting replay oracle"
        );
    }
}
