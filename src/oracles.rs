//! Exactly nine bounded oracles.
//!
//! Each oracle is a pure check over bounded inputs — the current world,
//! the receipt log, and a replayable fixture — and returns a verdict.
//! `run_all` returns a fixed-size array, so the count of nine is enforced
//! by the type system.
//!
//! Oracles 1–2 audit state, 3–6 audit the receipt log, 7 replays the whole
//! trial through the real implementation, 8 checks the hash chain and that
//! refusals mutate nothing, and 9 recomputes every expected outcome with
//! an independent shadow evaluator that never trusts receipt fields — so a
//! receipt lie that is internally consistent (and would pass 3–6) is still
//! caught.

use std::collections::BTreeMap;

use crate::boundary::{
    Command, MassGrams, OutcomeKind, Receipt, STAMINA_COST_BY_BAND, Stamina, StaminaBand, Verb,
    WITNESS_COST, World, YIELD_TABLE_GRAMS, submit,
};

pub const ORACLE_COUNT: usize = 9;

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

pub struct OracleCtx<'a> {
    pub world: &'a World,
    pub baseline_mass: MassGrams,
    pub build_fixture: fn() -> World,
    pub commands: &'a [Command],
    pub log: &'a [Receipt],
}

pub fn run_all(ctx: &OracleCtx<'_>) -> [OracleVerdict; ORACLE_COUNT] {
    [
        stamina_in_bounds(ctx),
        mass_conserved(ctx),
        witnessed_gate(ctx),
        exhausted_gate(ctx),
        closed_reasons(ctx),
        cell_bounds(ctx),
        replay_determinism(ctx),
        refusal_zero_mutation(ctx),
        shadow_expectation(ctx),
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

/// 2. Total mass (sites + inventories) equals the fixture baseline.
fn mass_conserved(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let current = ctx.world.economy.total_mass();
    OracleVerdict::new(
        "mass_conserved",
        current == ctx.baseline_mass,
        format!(
            "baseline={}g current={}g",
            ctx.baseline_mass.grams(),
            current.grams()
        ),
    )
}

/// 3. The witnessed claim is a boolean gate: no receipt moves mass without
///    a witnessed claim. Keyed on actual mass movement, not outcome kind —
///    the witness verb is Accepted with zero mass, which the second verb
///    exposed as a distinct case.
fn witnessed_gate(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations = ctx
        .log
        .iter()
        .filter(|r| !r.mass_moved.is_zero() && !r.witnessed)
        .count();
    OracleVerdict::new(
        "witnessed_gate",
        violations == 0,
        format!("{violations} unwitnessed receipts moved mass"),
    )
}

/// 4. An exhausted actor never yields mass: every mass-moving receipt sits
///    in a non-exhausted stamina band. Keyed on actual mass movement — an
///    exhausted character may still witness (zero mass) by verb policy.
fn exhausted_gate(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations = ctx
        .log
        .iter()
        .filter(|r| !r.mass_moved.is_zero())
        .filter(|r| !matches!(r.band, Some(band) if band != StaminaBand::Exhausted))
        .count();
    OracleVerdict::new(
        "exhausted_gate",
        violations == 0,
        format!("{violations} exhausted or band-less receipts moved mass"),
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
///    real implementation reproduces the same receipts and final hash.
fn replay_determinism(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let mut replay_world = (ctx.build_fixture)();
    let replay_lines: Vec<String> = ctx
        .commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| submit(&mut replay_world, i as u64 + 1, *cmd).canonical_line())
        .collect();
    let original_lines: Vec<String> = ctx.log.iter().map(Receipt::canonical_line).collect();
    let hashes_match = replay_world.hash() == ctx.world.hash();
    let lines_match = replay_lines == original_lines;
    OracleVerdict::new(
        "replay_determinism",
        hashes_match && lines_match,
        format!("hashes_match={hashes_match} receipts_match={lines_match}"),
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

/// Independent re-interpretation of the grammar. Deliberately does not use
/// the owners, the boundary orchestrator, `Stamina::band`, or any receipt
/// field — plain integers, its own threshold literals, and the shared spec
/// tables only.
struct ShadowState {
    stamina: BTreeMap<u64, u8>,
    sites: BTreeMap<u64, (usize, u64)>,
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
            sites: fixture
                .economy
                .sites_iter()
                .map(|(id, tier, stock)| (id.0, (tier.index(), stock.grams())))
                .collect(),
            claims: fixture
                .social
                .claims_iter()
                .map(|(id, holder, site, witnessed)| (id.0, (holder.0, site.0, witnessed)))
                .collect(),
        }
    }

    fn step(&mut self, cmd: &Command) -> ShadowExpectation {
        match cmd {
            Command::Gather(gather) => self.step_gather(gather),
            Command::Witness(witness) => self.step_witness(witness),
        }
    }

    fn step_gather(&mut self, cmd: &crate::boundary::GatherCommand) -> ShadowExpectation {
        let witnessed = self
            .claims
            .get(&cmd.claim.0)
            .is_some_and(|(_, _, flag)| *flag);
        let refuse = |reason: &'static str| ShadowExpectation {
            verb_code: "gather",
            outcome_code: "refused",
            reason_code: reason,
            witnessed,
            spent: 0,
            mass_grams: 0,
            band_index: None,
            tier_index: None,
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
        let Some(&(tier, stock)) = self.sites.get(&cmd.site.0) else {
            return refuse("unknown_site");
        };
        if stock == 0 {
            return refuse("site_empty");
        }
        let requested = YIELD_TABLE_GRAMS[band][tier];
        let granted = requested.min(stock);
        // Apply to shadow state.
        self.stamina.insert(cmd.actor.0, points - cost);
        self.sites.insert(cmd.site.0, (tier, stock - granted));
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
        }
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
        codes_match && cell_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::{
        CharacterId, ClaimId, GatherCommand, InfraTier, PartialReason, RefusalReason, SiteId,
        WitnessCommand, grammar_fingerprint, validate_world_coherence,
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
                (SiteId(1), InfraTier::Established, MassGrams::new(2000)),
                (SiteId(2), InfraTier::Crude, MassGrams::new(300)),
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

    fn run_fixture() -> (World, Vec<Command>, Vec<Receipt>, MassGrams) {
        let mut world = fixture();
        validate_world_coherence(&world).unwrap();
        let baseline = world.economy.total_mass();
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
    fn all_nine_oracles_pass_on_the_fixture_run() {
        let (world, cmds, log, baseline) = run_fixture();
        let ctx = OracleCtx {
            world: &world,
            baseline_mass: baseline,
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
            economy: EconomyOwner::seed_sites([(SiteId(1), InfraTier::Crude, MassGrams::new(100))])
                .unwrap(),
            social: SocialOwner::seed_claims([(ClaimId(1), CharacterId(99), SiteId(1), true)])
                .unwrap(),
        };
        assert!(validate_world_coherence(&world).is_err());
    }

    #[test]
    fn witnessed_gate_oracle_catches_a_doctored_receipt() {
        let (world, cmds, mut log, baseline) = run_fixture();
        log[0].witnessed = false;
        let ctx = OracleCtx {
            world: &world,
            baseline_mass: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        assert!(!witnessed_gate(&ctx).pass);
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
            baseline_mass: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        assert!(witnessed_gate(&ctx).pass);
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
            baseline_mass: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        assert!(!refusal_zero_mutation(&ctx).pass);
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
}
