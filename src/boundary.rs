//! Truth-layer boundary.
//!
//! Shared primitive types (typed IDs, `Stamina`, `MassGrams`), the single
//! active 4x4 design cell (Stamina x GatheringInfrastructure), the closed
//! outcome/reason vocabulary, canonical receipts, deterministic world
//! hashing, and the orchestrator that runs *every* validation before any
//! owner apply.
//!
//! Verb policy lives here, not in the owners: the boundary maps stamina to
//! a band, decides the gather cost, and refuses exhausted actors. Owners
//! only enforce their own resource semantics (existence, bounds, exact
//! deltas), so a second verb with a different policy never has to touch an
//! owner.
//!
//! Applies never produce a wrong game outcome: a proof token is consumed
//! by value (one token, one apply — reuse is a compile error) and carries
//! the entity revisions it was minted against, so plans touching disjoint
//! entities never false-conflict. A plan's commit phase checks every
//! token fresh BEFORE any owner mutates — a stale plan panics loudly and
//! all-or-nothing, because a stale token reaching a commit is a boundary
//! bug, not a game outcome, and a partial commit is a world state no
//! receipt accounts for.
//!
//! All fixture and table numbers in this crate are mechanical examples —
//! they are not balance and not historical truth.

use crate::character::{CharacterOwner, StaminaSpend};
use crate::economy::{EconomyOwner, Extraction};
use crate::social::{SocialOwner, WitnessGrant, WitnessPass};

// ---------------------------------------------------------------------------
// Typed IDs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharacterId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SiteId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClaimId(pub u64);

// ---------------------------------------------------------------------------
// Bounded quantities
// ---------------------------------------------------------------------------

/// Stamina points, bounded to `0..=MAX`. Constructed only through `new`,
/// so an out-of-range value is unrepresentable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stamina(u8);

impl Stamina {
    pub const MAX: u8 = 100;

    pub fn new(points: u8) -> Option<Self> {
        (points <= Self::MAX).then_some(Self(points))
    }

    pub fn points(self) -> u8 {
        self.0
    }

    /// Exact spend: `None` when the cost exceeds the points. There is no
    /// clamping path — an overdraw is refused at validation, never hidden.
    pub fn spend_exact(self, cost: u8) -> Option<Self> {
        self.0.checked_sub(cost).map(Self)
    }

    pub fn band(self) -> StaminaBand {
        match self.0 {
            0..=9 => StaminaBand::Exhausted,
            10..=39 => StaminaBand::Low,
            40..=79 => StaminaBand::Steady,
            _ => StaminaBand::Fresh,
        }
    }
}

/// Mass in whole grams. Backed by `u64`, so negative mass is
/// unrepresentable by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MassGrams(u64);

impl MassGrams {
    pub const ZERO: Self = Self(0);

    pub fn new(grams: u64) -> Self {
        Self(grams)
    }

    pub fn grams(self) -> u64 {
        self.0
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns `None` instead of silently clamping an unrepresentable
    /// mass sum. A coherent world proves this cannot fail during apply.
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    /// Returns `None` instead of ever producing a below-zero mass.
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }
}

// ---------------------------------------------------------------------------
// The single active 4x4 cell: Stamina x GatheringInfrastructure
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaminaBand {
    Exhausted,
    Low,
    Steady,
    Fresh,
}

impl StaminaBand {
    pub fn index(self) -> usize {
        match self {
            Self::Exhausted => 0,
            Self::Low => 1,
            Self::Steady => 2,
            Self::Fresh => 3,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::Exhausted => "exhausted",
            Self::Low => "low",
            Self::Steady => "steady",
            Self::Fresh => "fresh",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfraTier {
    None,
    Crude,
    Established,
    Advanced,
}

impl InfraTier {
    pub fn index(self) -> usize {
        match self {
            Self::None => 0,
            Self::Crude => 1,
            Self::Established => 2,
            Self::Advanced => 3,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Crude => "crude",
            Self::Established => "established",
            Self::Advanced => "advanced",
        }
    }
}

pub const CELL_ROWS: usize = 4;
pub const CELL_COLS: usize = 4;

/// Gather yield in grams by `[StaminaBand][InfraTier]`.
/// Mechanical example numbers only — not balance, not historical truth.
/// The `Exhausted` row is unreachable: validation refuses exhausted actors
/// before the table is consulted.
pub const YIELD_TABLE_GRAMS: [[u64; CELL_COLS]; CELL_ROWS] = [
    [0, 0, 0, 0],
    [250, 400, 600, 900],
    [500, 800, 1200, 1800],
    [750, 1200, 1800, 2700],
];

/// Stamina cost of one gather by band. Verb policy — owned by the
/// boundary, never by the character owner. Mechanical example numbers.
pub const STAMINA_COST_BY_BAND: [u8; CELL_ROWS] = [0, 15, 12, 10];

/// Stamina cost of witnessing a claim. Deliberately a *different* policy
/// from gather: flat cost, no band table, and no exhausted gate — an
/// exhausted character may still attest. Mechanical example number.
pub const WITNESS_COST: u8 = 5;

/// Fingerprint of the grammar that produced a receipt: the yield table,
/// the cost table, the actual band mapping over the full stamina range,
/// and every closed reason code. Change any of them and every subsequent
/// receipt carries a different fingerprint, so a trial record always says
/// which grammar version produced it.
pub fn grammar_fingerprint() -> u64 {
    let mut hasher = Fnv1a::default();
    for row in YIELD_TABLE_GRAMS {
        for cell in row {
            hasher.update(&cell.to_be_bytes());
        }
    }
    hasher.update(&STAMINA_COST_BY_BAND);
    hasher.update(&[WITNESS_COST]);
    for points in 0..=Stamina::MAX {
        let stamina = Stamina::new(points).expect("in range by construction");
        hasher.update(&[stamina.band().index() as u8]);
    }
    for reason in RefusalReason::ALL {
        hasher.update(reason.code().as_bytes());
    }
    for reason in PartialReason::ALL {
        hasher.update(reason.code().as_bytes());
    }
    hasher.finish()
}

// ---------------------------------------------------------------------------
// Closed outcome and reason vocabulary
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusalReason {
    UnknownActor,
    UnknownSite,
    UnknownClaim,
    ClaimNotHeldByActor,
    ClaimSiteMismatch,
    ClaimNotWitnessed,
    ActorExhausted,
    InsufficientStamina,
    SiteEmpty,
    ClaimAlreadyWitnessed,
    CannotWitnessOwnClaim,
}

impl RefusalReason {
    pub const ALL: [Self; 11] = [
        Self::UnknownActor,
        Self::UnknownSite,
        Self::UnknownClaim,
        Self::ClaimNotHeldByActor,
        Self::ClaimSiteMismatch,
        Self::ClaimNotWitnessed,
        Self::ActorExhausted,
        Self::InsufficientStamina,
        Self::SiteEmpty,
        Self::ClaimAlreadyWitnessed,
        Self::CannotWitnessOwnClaim,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Self::UnknownActor => "unknown_actor",
            Self::UnknownSite => "unknown_site",
            Self::UnknownClaim => "unknown_claim",
            Self::ClaimNotHeldByActor => "claim_not_held_by_actor",
            Self::ClaimSiteMismatch => "claim_site_mismatch",
            Self::ClaimNotWitnessed => "claim_not_witnessed",
            Self::ActorExhausted => "actor_exhausted",
            Self::InsufficientStamina => "insufficient_stamina",
            Self::SiteEmpty => "site_empty",
            Self::ClaimAlreadyWitnessed => "claim_already_witnessed",
            Self::CannotWitnessOwnClaim => "cannot_witness_own_claim",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.code() == code)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartialReason {
    SiteNearlyDepleted,
}

impl PartialReason {
    pub const ALL: [Self; 1] = [Self::SiteNearlyDepleted];

    pub fn code(self) -> &'static str {
        match self {
            Self::SiteNearlyDepleted => "site_nearly_depleted",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.code() == code)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeKind {
    Accepted,
    Partial(PartialReason),
    Refused(RefusalReason),
}

impl OutcomeKind {
    pub fn code(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Partial(_) => "partial",
            Self::Refused(_) => "refused",
        }
    }

    pub fn reason_code(self) -> &'static str {
        match self {
            Self::Accepted => "-",
            Self::Partial(reason) => reason.code(),
            Self::Refused(reason) => reason.code(),
        }
    }

    /// True when the code pair round-trips through the closed enums.
    pub fn codes_round_trip(self) -> bool {
        match self {
            Self::Accepted => self.reason_code() == "-",
            Self::Partial(_) => PartialReason::from_code(self.reason_code()).is_some(),
            Self::Refused(_) => RefusalReason::from_code(self.reason_code()).is_some(),
        }
    }

    pub fn yields_mass(self) -> bool {
        matches!(self, Self::Accepted | Self::Partial(_))
    }
}

/// Faults in seeded data, caught before any trial runs. Closed set: a
/// fixture either seeds cleanly or names exactly what is incoherent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureFault {
    DuplicateCharacter(CharacterId),
    DuplicateSite(SiteId),
    DuplicateClaim(ClaimId),
    ClaimHolderUnknown(ClaimId),
    ClaimSiteUnknown(ClaimId),
    TotalMassOverflow,
}

/// Cross-owner fixture integrity: total mass must fit its canonical `u64`
/// representation, and every claim must point at a known holder and site.
/// Duplicate IDs are already rejected at seed time by each owner.
pub fn validate_world_coherence(world: &World) -> Result<(), FixtureFault> {
    if world.economy.checked_total_mass().is_none() {
        return Err(FixtureFault::TotalMassOverflow);
    }
    for (claim, holder, site, _witnessed) in world.social.claims_iter() {
        if world.characters.stamina(holder).is_none() {
            return Err(FixtureFault::ClaimHolderUnknown(claim));
        }
        if world.economy.tier(site).is_none() {
            return Err(FixtureFault::ClaimSiteUnknown(claim));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Proof envelope: the identity of one comparable run
// ---------------------------------------------------------------------------

/// Identity of a trial's immutable inputs: the seeded world hash plus the
/// canonical encoding of every command in order. Two runs are
/// cross-comparable only when fixture identity AND grammar fingerprint
/// both match — receipts from different fixtures or grammars are evidence
/// about different experiments.
pub fn fixture_identity(fixture_hash: u64, cmds: &[Command]) -> u64 {
    let mut hasher = Fnv1a::default();
    hasher.update(&fixture_hash.to_be_bytes());
    for cmd in cmds {
        hasher.update(&cmd.canonical_bytes());
    }
    hasher.finish()
}

/// Digest of the whole receipt chain, in order. Together with the final
/// world hash this seals the run's outcome; a host reproducing the trial
/// must reproduce this digest byte-for-byte.
pub fn receipt_chain_digest(log: &[Receipt]) -> u64 {
    let mut hasher = Fnv1a::default();
    for receipt in log {
        hasher.update(receipt.canonical_line().as_bytes());
    }
    hasher.finish()
}

// ---------------------------------------------------------------------------
// Deterministic hashing (FNV-1a 64, no platform or ordering dependence)
// ---------------------------------------------------------------------------

pub struct Fnv1a(u64);

impl Default for Fnv1a {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl Fnv1a {
    pub fn update(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    pub fn finish(&self) -> u64 {
        self.0
    }
}

// ---------------------------------------------------------------------------
// Commands, receipts, world
// ---------------------------------------------------------------------------

/// The closed verb vocabulary. Every command is exactly one verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verb {
    Gather,
    Witness,
}

impl Verb {
    pub fn code(self) -> &'static str {
        match self {
            Self::Gather => "gather",
            Self::Witness => "witness",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GatherCommand {
    pub actor: CharacterId,
    pub claim: ClaimId,
    pub site: SiteId,
}

#[derive(Debug, Clone, Copy)]
pub struct WitnessCommand {
    pub witness: CharacterId,
    pub claim: ClaimId,
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Gather(GatherCommand),
    Witness(WitnessCommand),
}

impl Command {
    /// The language-seam observation point. Foreign input has preserved
    /// command meaning only once it produces these bytes; receipt and host
    /// parity claims begin after this boundary, never before it.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        match self {
            Self::Gather(gather) => {
                let mut bytes = Vec::with_capacity(30);
                bytes.extend_from_slice(b"gather");
                bytes.extend_from_slice(&gather.actor.0.to_be_bytes());
                bytes.extend_from_slice(&gather.claim.0.to_be_bytes());
                bytes.extend_from_slice(&gather.site.0.to_be_bytes());
                bytes
            }
            Self::Witness(witness) => {
                let mut bytes = Vec::with_capacity(23);
                bytes.extend_from_slice(b"witness");
                bytes.extend_from_slice(&witness.witness.0.to_be_bytes());
                bytes.extend_from_slice(&witness.claim.0.to_be_bytes());
                bytes
            }
        }
    }
}

/// Closed failures for the deliberately small text ingestion seam. The
/// parser either names one of these failures or returns a command whose
/// canonical bytes are the boundary artifact; it never coerces input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
pub enum TextCommandFault {
    EmptyLine,
    NonAscii,
    NonCanonicalWhitespace,
    UnknownVerb,
    WrongFieldCount,
    UnexpectedField,
    EmptyValue,
    NonCanonicalInteger,
    IntegerOutOfRange,
}

/// Parse the minimal external command spelling used to test the language
/// seam: `gather actor=1 claim=1 site=1` or
/// `witness witness=3 claim=8`.
#[cfg(test)]
pub fn parse_text_command(line: &str) -> Result<Command, TextCommandFault> {
    if line.is_empty() {
        return Err(TextCommandFault::EmptyLine);
    }
    if !line.is_ascii() {
        return Err(TextCommandFault::NonAscii);
    }
    if line.trim() != line || line.contains("  ") || line.contains(['\t', '\r', '\n']) {
        return Err(TextCommandFault::NonCanonicalWhitespace);
    }

    let fields: Vec<&str> = line.split(' ').collect();
    match fields.first().copied() {
        Some("gather") => {
            if fields.len() != 4 {
                return Err(TextCommandFault::WrongFieldCount);
            }
            Ok(Command::Gather(GatherCommand {
                actor: CharacterId(parse_text_u64(field_value(fields[1], "actor")?)?),
                claim: ClaimId(parse_text_u64(field_value(fields[2], "claim")?)?),
                site: SiteId(parse_text_u64(field_value(fields[3], "site")?)?),
            }))
        }
        Some("witness") => {
            if fields.len() != 3 {
                return Err(TextCommandFault::WrongFieldCount);
            }
            Ok(Command::Witness(WitnessCommand {
                witness: CharacterId(parse_text_u64(field_value(fields[1], "witness")?)?),
                claim: ClaimId(parse_text_u64(field_value(fields[2], "claim")?)?),
            }))
        }
        Some(_) => Err(TextCommandFault::UnknownVerb),
        None => Err(TextCommandFault::EmptyLine),
    }
}

#[cfg(test)]
fn field_value<'a>(field: &'a str, expected: &str) -> Result<&'a str, TextCommandFault> {
    let Some((name, value)) = field.split_once('=') else {
        return Err(TextCommandFault::UnexpectedField);
    };
    if name != expected {
        return Err(TextCommandFault::UnexpectedField);
    }
    if value.is_empty() {
        return Err(TextCommandFault::EmptyValue);
    }
    Ok(value)
}

#[cfg(test)]
fn parse_text_u64(value: &str) -> Result<u64, TextCommandFault> {
    if !value.bytes().all(|byte| byte.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(TextCommandFault::NonCanonicalInteger);
    }
    value
        .parse::<u64>()
        .map_err(|_| TextCommandFault::IntegerOutOfRange)
}

/// Canonical receipt for one submitted command. Every field is either a
/// closed code or a bounded integer, so the canonical line is
/// deterministic. It records the world hash both before and after the
/// command and the grammar fingerprint that produced it.
#[derive(Debug, Clone)]
pub struct Receipt {
    pub seq: u64,
    pub verb: Verb,
    pub actor: CharacterId,
    pub claim: ClaimId,
    pub site: Option<SiteId>,
    pub outcome: OutcomeKind,
    pub witnessed: bool,
    pub stamina_before: Option<Stamina>,
    pub band: Option<StaminaBand>,
    pub tier: Option<InfraTier>,
    pub stamina_spent: u8,
    pub mass_moved: MassGrams,
    pub grammar: u64,
    pub world_hash_before: u64,
    pub world_hash_after: u64,
}

impl Receipt {
    pub fn canonical_line(&self) -> String {
        let stamina_before = self
            .stamina_before
            .map_or_else(|| "-".to_owned(), |s| s.points().to_string());
        let band = self.band.map_or("-", StaminaBand::code);
        let tier = self.tier.map_or("-", InfraTier::code);
        let site = self
            .site
            .map_or_else(|| "-".to_owned(), |s| format!("S{}", s.0));
        format!(
            "seq={} verb={} actor=C{} claim=K{} site={} outcome={} reason={} witnessed={} \
             stamina_before={} band={} tier={} spent={} mass_g={} \
             grammar=0x{:016x} world_before=0x{:016x} world=0x{:016x}",
            self.seq,
            self.verb.code(),
            self.actor.0,
            self.claim.0,
            site,
            self.outcome.code(),
            self.outcome.reason_code(),
            self.witnessed,
            stamina_before,
            band,
            tier,
            self.stamina_spent,
            self.mass_moved.grams(),
            self.grammar,
            self.world_hash_before,
            self.world_hash_after,
        )
    }
}

/// The whole truth-layer state, split across exactly three single-writer
/// owners. Each owner's internals are private to its module; nothing
/// outside a module can mutate that module's state.
pub struct World {
    pub characters: CharacterOwner,
    pub economy: EconomyOwner,
    pub social: SocialOwner,
}

impl World {
    pub fn hash(&self) -> u64 {
        let mut hasher = Fnv1a::default();
        self.characters.hash_into(&mut hasher);
        self.economy.hash_into(&mut hasher);
        self.social.hash_into(&mut hasher);
        hasher.finish()
    }

    /// Exact canonical serialization of the whole truth state: one line
    /// per fact, in deterministic owner/key order — the same lines the
    /// pure host prints after a trial. Exact-equality claims (host
    /// parity, replay) compare these lines; `hash()` is only their
    /// address — FNV-1a is not injective, so hash equality alone is
    /// checksum evidence, never state equality.
    pub fn canonical_state(&self) -> Vec<String> {
        let mut lines = Vec::new();
        for (id, stamina) in self.characters.iter() {
            lines.push(format!(
                "character C{} stamina={} inventory_g={}",
                id.0,
                stamina.points(),
                self.economy.inventory(id).grams()
            ));
        }
        for (id, tier, stock) in self.economy.sites_iter() {
            lines.push(format!(
                "site S{} tier={} stock_g={}",
                id.0,
                tier.code(),
                stock.grams()
            ));
        }
        for (claim, holder, site, witnessed) in self.social.claims_iter() {
            lines.push(format!(
                "claim K{} holder=C{} site=S{} witnessed={}",
                claim.0, holder.0, site.0, witnessed
            ));
        }
        lines.push(format!(
            "revisions character={} economy={} social={}",
            self.characters.revision(),
            self.economy.revision(),
            self.social.revision()
        ));
        lines
    }
}

// ---------------------------------------------------------------------------
// Orchestration: validate everything, then apply
// ---------------------------------------------------------------------------

/// A fully validated gather. Holding one proves that every owner has
/// approved its part against its current revision. The tokens inside are
/// consumed by value when applied — a plan can be applied at most once.
struct GatherPlan {
    witness: WitnessPass,
    spend: StaminaSpend,
    extraction: Extraction,
    partial: Option<PartialReason>,
    band: StaminaBand,
    tier: InfraTier,
}

impl GatherPlan {
    /// Commit phase for a planned gather: the one place a multi-owner
    /// plan mutates the world. Consumes the plan by value — at most one
    /// commit per plan. Two-phase: every token is checked fresh BEFORE
    /// any owner mutates, so a stale plan panics all-or-nothing — a
    /// partial commit is a world state no receipt accounts for.
    fn apply(self, world: &mut World) {
        assert!(
            world.characters.spend_is_fresh(&self.spend)
                && world.economy.extraction_is_fresh(&self.extraction),
            "stale plan token — boundary bug (commit refused before any mutation)"
        );
        world.characters.apply_spend(self.spend);
        world.economy.apply_extract(self.extraction);
    }
}

fn plan_gather(world: &World, cmd: &GatherCommand) -> Result<GatherPlan, RefusalReason> {
    // 1. Social gate: the claim must exist, be held by the actor, cover the
    //    site, and be witnessed (boolean gate).
    let witness = world
        .social
        .validate_witness_gate(cmd.claim, cmd.actor, cmd.site)?;
    // 2. Character gate. Verb policy first (boundary): band the actor and
    //    refuse the exhausted; then resource semantics (owner): exact
    //    headroom for the verb's cost — no clamping.
    let stamina = world
        .characters
        .stamina(cmd.actor)
        .ok_or(RefusalReason::UnknownActor)?;
    let band = stamina.band();
    if band == StaminaBand::Exhausted {
        return Err(RefusalReason::ActorExhausted);
    }
    let cost = STAMINA_COST_BY_BAND[band.index()];
    let spend = world.characters.validate_spend(cmd.actor, cost)?;
    // 3. Economy gate: the site must exist and hold stock; the requested
    //    yield comes from the single active 4x4 cell.
    let tier = world
        .economy
        .tier(cmd.site)
        .ok_or(RefusalReason::UnknownSite)?;
    let requested = MassGrams::new(YIELD_TABLE_GRAMS[band.index()][tier.index()]);
    let extraction = world
        .economy
        .validate_extract(cmd.site, cmd.actor, requested)?;
    let partial = (extraction.granted() < requested).then_some(PartialReason::SiteNearlyDepleted);
    Ok(GatherPlan {
        witness,
        spend,
        extraction,
        partial,
        band,
        tier,
    })
}

/// A fully validated witness attestation. Different verb, same doctrine:
/// every gate passed before any apply, tokens consumed by value.
struct WitnessPlan {
    grant: WitnessGrant,
    spend: StaminaSpend,
}

impl WitnessPlan {
    /// Commit phase for a planned witness attestation; same two-phase
    /// doctrine as `GatherPlan::apply`.
    fn apply(self, world: &mut World) {
        assert!(
            world.characters.spend_is_fresh(&self.spend)
                && world.social.grant_is_fresh(&self.grant),
            "stale plan token — boundary bug (commit refused before any mutation)"
        );
        world.characters.apply_spend(self.spend);
        world.social.apply_witness(self.grant);
    }
}

fn plan_witness(world: &World, cmd: &WitnessCommand) -> Result<WitnessPlan, RefusalReason> {
    // 1. Social gate: the claim must exist, not already be witnessed, and
    //    the witness must not be its holder.
    let grant = world
        .social
        .validate_witness_grant(cmd.claim, cmd.witness)?;
    // 2. Character gate. Witness verb policy (boundary): flat cost, no
    //    band table, no exhausted gate — the owner only checks existence
    //    and exact headroom, exactly as for gather.
    let spend = world.characters.validate_spend(cmd.witness, WITNESS_COST)?;
    // 3. Economy: untouched by this verb.
    Ok(WitnessPlan { grant, spend })
}

/// Submit one command through the boundary. All validation happens before
/// any owner apply; the applies consume their proof tokens by value and
/// panic only on a stale token, which is a boundary bug, never a game
/// outcome.
pub fn submit(world: &mut World, seq: u64, cmd: Command) -> Receipt {
    match cmd {
        Command::Gather(gather) => submit_gather(world, seq, gather),
        Command::Witness(witness) => submit_witness(world, seq, witness),
    }
}

fn submit_gather(world: &mut World, seq: u64, cmd: GatherCommand) -> Receipt {
    let world_hash_before = world.hash();
    let grammar = grammar_fingerprint();
    let witnessed = world.social.is_witnessed(cmd.claim).unwrap_or(false);
    let stamina_before = world.characters.stamina(cmd.actor);

    match plan_gather(world, &cmd) {
        Err(reason) => Receipt {
            seq,
            verb: Verb::Gather,
            actor: cmd.actor,
            claim: cmd.claim,
            site: Some(cmd.site),
            outcome: OutcomeKind::Refused(reason),
            witnessed,
            stamina_before,
            band: stamina_before.map(Stamina::band),
            tier: world.economy.tier(cmd.site),
            stamina_spent: 0,
            mass_moved: MassGrams::ZERO,
            grammar,
            world_hash_before,
            world_hash_after: world.hash(),
        },
        Ok(plan) => {
            let outcome = plan
                .partial
                .map_or(OutcomeKind::Accepted, OutcomeKind::Partial);
            let stamina_spent = plan.spend.cost();
            let mass_moved = plan.extraction.granted();
            let claim = plan.witness.claim();
            let (band, tier) = (plan.band, plan.tier);
            // The commit phase consumes the plan and its tokens by value.
            // The social owner's state is unchanged by a gather.
            plan.apply(world);
            Receipt {
                seq,
                verb: Verb::Gather,
                actor: cmd.actor,
                claim,
                site: Some(cmd.site),
                outcome,
                witnessed,
                stamina_before,
                band: Some(band),
                tier: Some(tier),
                stamina_spent,
                mass_moved,
                grammar,
                world_hash_before,
                world_hash_after: world.hash(),
            }
        }
    }
}

fn submit_witness(world: &mut World, seq: u64, cmd: WitnessCommand) -> Receipt {
    let world_hash_before = world.hash();
    let grammar = grammar_fingerprint();
    let witnessed = world.social.is_witnessed(cmd.claim).unwrap_or(false);
    let stamina_before = world.characters.stamina(cmd.witness);
    let site = world.social.claim_site(cmd.claim);

    match plan_witness(world, &cmd) {
        Err(reason) => Receipt {
            seq,
            verb: Verb::Witness,
            actor: cmd.witness,
            claim: cmd.claim,
            site,
            outcome: OutcomeKind::Refused(reason),
            witnessed,
            stamina_before,
            band: stamina_before.map(Stamina::band),
            tier: None,
            stamina_spent: 0,
            mass_moved: MassGrams::ZERO,
            grammar,
            world_hash_before,
            world_hash_after: world.hash(),
        },
        Ok(plan) => {
            let stamina_spent = plan.spend.cost();
            let claim = plan.grant.claim();
            // The commit phase consumes the plan and its tokens by value.
            // The economy owner is untouched by a witness.
            plan.apply(world);
            Receipt {
                seq,
                verb: Verb::Witness,
                actor: cmd.witness,
                claim,
                site,
                outcome: OutcomeKind::Accepted,
                witnessed,
                stamina_before,
                band: stamina_before.map(Stamina::band),
                tier: None,
                stamina_spent,
                mass_moved: MassGrams::ZERO,
                grammar,
                world_hash_before,
                world_hash_after: world.hash(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamina_rejects_out_of_range() {
        assert!(Stamina::new(101).is_none());
        assert!(Stamina::new(100).is_some());
    }

    #[test]
    fn negative_mass_is_unrepresentable() {
        let small = MassGrams::new(5);
        let large = MassGrams::new(10);
        assert_eq!(small.checked_sub(large), None);
        assert_eq!(large.checked_sub(small), Some(MassGrams::new(5)));
    }

    #[test]
    fn exact_spend_never_clamps() {
        let stamina = Stamina::new(12).unwrap();
        assert_eq!(stamina.spend_exact(15), None);
        assert_eq!(stamina.spend_exact(12), Stamina::new(0));
    }

    #[test]
    fn bands_cover_full_stamina_range() {
        assert_eq!(Stamina::new(0).unwrap().band(), StaminaBand::Exhausted);
        assert_eq!(Stamina::new(9).unwrap().band(), StaminaBand::Exhausted);
        assert_eq!(Stamina::new(10).unwrap().band(), StaminaBand::Low);
        assert_eq!(Stamina::new(39).unwrap().band(), StaminaBand::Low);
        assert_eq!(Stamina::new(40).unwrap().band(), StaminaBand::Steady);
        assert_eq!(Stamina::new(79).unwrap().band(), StaminaBand::Steady);
        assert_eq!(Stamina::new(80).unwrap().band(), StaminaBand::Fresh);
        assert_eq!(Stamina::new(100).unwrap().band(), StaminaBand::Fresh);
    }

    #[test]
    fn reason_codes_round_trip() {
        for reason in RefusalReason::ALL {
            assert_eq!(RefusalReason::from_code(reason.code()), Some(reason));
        }
        for reason in PartialReason::ALL {
            assert_eq!(PartialReason::from_code(reason.code()), Some(reason));
        }
    }

    #[test]
    fn grammar_fingerprint_is_stable() {
        assert_eq!(grammar_fingerprint(), grammar_fingerprint());
    }

    #[test]
    fn fixture_identity_is_order_and_input_sensitive() {
        let a = Command::Gather(GatherCommand {
            actor: CharacterId(1),
            claim: ClaimId(1),
            site: SiteId(1),
        });
        let b = Command::Witness(WitnessCommand {
            witness: CharacterId(2),
            claim: ClaimId(3),
        });
        assert_eq!(fixture_identity(7, &[a, b]), fixture_identity(7, &[a, b]));
        assert_ne!(fixture_identity(7, &[a, b]), fixture_identity(7, &[b, a]));
        assert_ne!(fixture_identity(7, &[a, b]), fixture_identity(8, &[a, b]));
    }

    /// Falsifier (trial/009): Rust's integer parser accepts a leading `+`,
    /// silently normalizing a non-canonical source spelling before the
    /// command observation point.
    #[test]
    fn falsification_text_seam_rejects_leading_plus() {
        assert!(
            matches!(
                parse_text_command("gather actor=+1 claim=1 site=1"),
                Err(TextCommandFault::NonCanonicalInteger)
            ),
            "leading plus silently normalized before canonical command bytes"
        );
    }

    #[test]
    fn text_seam_accepts_only_canonical_command_meaning() {
        let gather = Command::Gather(GatherCommand {
            actor: CharacterId(1),
            claim: ClaimId(2),
            site: SiteId(3),
        });
        let parsed_gather =
            parse_text_command("gather actor=1 claim=2 site=3").expect("canonical gather spelling");
        assert_eq!(parsed_gather.canonical_bytes(), gather.canonical_bytes());

        let witness = Command::Witness(WitnessCommand {
            witness: CharacterId(3),
            claim: ClaimId(8),
        });
        let parsed_witness =
            parse_text_command("witness witness=3 claim=8").expect("canonical witness spelling");
        assert_eq!(parsed_witness.canonical_bytes(), witness.canonical_bytes());

        let max = Command::Witness(WitnessCommand {
            witness: CharacterId(u64::MAX),
            claim: ClaimId(8),
        });
        let parsed_max = parse_text_command("witness witness=18446744073709551615 claim=8")
            .expect("u64::MAX has a canonical decimal spelling");
        assert_eq!(parsed_max.canonical_bytes(), max.canonical_bytes());
    }

    #[test]
    fn text_seam_rejects_noncanonical_or_ambiguous_sources() {
        let cases = [
            ("", TextCommandFault::EmptyLine),
            (
                "dance actor=1 claim=1 site=1",
                TextCommandFault::UnknownVerb,
            ),
            (
                "gather actor=+1 claim=1 site=1",
                TextCommandFault::NonCanonicalInteger,
            ),
            (
                "gather actor=-1 claim=1 site=1",
                TextCommandFault::NonCanonicalInteger,
            ),
            (
                "gather actor=01 claim=1 site=1",
                TextCommandFault::NonCanonicalInteger,
            ),
            ("gather actor=１ claim=1 site=1", TextCommandFault::NonAscii),
            (
                "gather actor=18446744073709551616 claim=1 site=1",
                TextCommandFault::IntegerOutOfRange,
            ),
            (
                "gather claim=1 actor=1 site=1",
                TextCommandFault::UnexpectedField,
            ),
            (
                "gather actor=1 actor=1 site=1",
                TextCommandFault::UnexpectedField,
            ),
            (
                "gather actor=1 claim=1 zone=1",
                TextCommandFault::UnexpectedField,
            ),
            ("gather actor=1 claim=1", TextCommandFault::WrongFieldCount),
            (
                "gather actor=1 claim=1 site=1 mode=fast",
                TextCommandFault::WrongFieldCount,
            ),
            ("gather actor= claim=1 site=1", TextCommandFault::EmptyValue),
            (
                "\u{feff}gather actor=1 claim=1 site=1",
                TextCommandFault::NonAscii,
            ),
            (
                " gather actor=1 claim=1 site=1",
                TextCommandFault::NonCanonicalWhitespace,
            ),
            (
                "gather actor=1  claim=1 site=1",
                TextCommandFault::NonCanonicalWhitespace,
            ),
            (
                "gather\tactor=1 claim=1 site=1",
                TextCommandFault::NonCanonicalWhitespace,
            ),
            (
                "gather actor=1 claim=1 site=1\n",
                TextCommandFault::NonCanonicalWhitespace,
            ),
        ];

        for (source, expected) in cases {
            assert!(
                matches!(parse_text_command(source), Err(actual) if actual == expected),
                "source {source:?} was not rejected as {expected:?}"
            );
        }
    }

    #[test]
    fn fnv1a_is_stable() {
        let mut hasher = Fnv1a::default();
        hasher.update(b"truth");
        let first = hasher.finish();
        let mut again = Fnv1a::default();
        again.update(b"truth");
        assert_eq!(first, again.finish());
    }

    /// Two actors, two sites, both claims witnessed — the smallest world
    /// where two plans can be fully independent. Mechanical example
    /// numbers only.
    fn two_actor_world() -> World {
        World {
            characters: CharacterOwner::seed([
                (CharacterId(1), Stamina::new(90).unwrap()),
                (CharacterId(2), Stamina::new(50).unwrap()),
            ])
            .unwrap(),
            economy: EconomyOwner::seed_sites([
                (SiteId(1), InfraTier::Established, MassGrams::new(5000)),
                (SiteId(2), InfraTier::Crude, MassGrams::new(3000)),
            ])
            .unwrap(),
            social: SocialOwner::seed_claims([
                (ClaimId(1), CharacterId(1), SiteId(1), true),
                (ClaimId(2), CharacterId(2), SiteId(2), true),
                (ClaimId(3), CharacterId(2), SiteId(1), false),
            ])
            .unwrap(),
        }
    }

    /// Falsifier (trial/008): the representable-mass bound is established
    /// before commands run. Without this guard, two individually valid
    /// site stocks can overflow the aggregate and later permit an
    /// inventory transfer to discard mass silently.
    #[test]
    fn falsification_overfull_mass_fixture_is_rejected() {
        let world = World {
            characters: CharacterOwner::seed([(CharacterId(1), Stamina::new(90).unwrap())])
                .unwrap(),
            economy: EconomyOwner::seed_sites([
                (SiteId(1), InfraTier::Established, MassGrams::new(u64::MAX)),
                (SiteId(2), InfraTier::Crude, MassGrams::new(1)),
            ])
            .unwrap(),
            social: SocialOwner::seed_claims([]).unwrap(),
        };
        assert_eq!(
            validate_world_coherence(&world),
            Err(FixtureFault::TotalMassOverflow)
        );
    }

    /// Falsifier (trial/006): hash equality is checksum evidence, not
    /// state equality — FNV-1a is not injective. Exact-equality claims
    /// must compare a canonical final-state serialization that is stable
    /// across identical histories and sees every truth-domain mutation;
    /// the hash is that serialization's address, nothing more.
    #[test]
    fn falsification_canonical_state_must_see_every_domain_mutation() {
        let mut world = two_actor_world();
        let seeded = world.canonical_state();
        assert_eq!(
            seeded,
            two_actor_world().canonical_state(),
            "stable across identical construction"
        );
        // A gather moves stamina, site stock, and an inventory.
        submit(
            &mut world,
            1,
            Command::Gather(GatherCommand {
                actor: CharacterId(1),
                claim: ClaimId(1),
                site: SiteId(1),
            }),
        );
        let after_gather = world.canonical_state();
        assert_ne!(seeded, after_gather, "a gather must be visible");
        // A witness flips a claim gate and moves stamina, zero mass.
        submit(
            &mut world,
            2,
            Command::Witness(WitnessCommand {
                witness: CharacterId(1),
                claim: ClaimId(3),
            }),
        );
        let after_witness = world.canonical_state();
        assert_ne!(after_gather, after_witness, "a witness must be visible");
    }

    fn active_cell_world(band: StaminaBand, tier: InfraTier, stock: u64) -> World {
        let points = match band {
            StaminaBand::Low => 39,
            StaminaBand::Steady => 79,
            StaminaBand::Fresh => 100,
            StaminaBand::Exhausted => {
                panic!("the exhausted row is gated before the yield lookup")
            }
        };
        let world = World {
            characters: CharacterOwner::seed([(
                CharacterId(1),
                Stamina::new(points).expect("representative stamina is in range"),
            )])
            .expect("one character is unique"),
            economy: EconomyOwner::seed_sites([(SiteId(1), tier, MassGrams::new(stock))])
                .expect("one site is unique"),
            social: SocialOwner::seed_claims([(ClaimId(1), CharacterId(1), SiteId(1), true)])
                .expect("one claim is unique"),
        };
        validate_world_coherence(&world).expect("purpose-built cell fixture is coherent");
        world
    }

    fn submit_active_cell(world: &mut World) -> Receipt {
        submit(
            world,
            1,
            Command::Gather(GatherCommand {
                actor: CharacterId(1),
                claim: ClaimId(1),
                site: SiteId(1),
            }),
        )
    }

    fn assert_cell_mass_landed(world: &World, expected_inventory: u64) {
        assert_eq!(
            world.economy.inventory(CharacterId(1)),
            MassGrams::new(expected_inventory)
        );
        let remaining_stock = world
            .economy
            .sites_iter()
            .next()
            .map(|(_, _, stock)| stock)
            .expect("the purpose-built site exists");
        assert_eq!(remaining_stock, MassGrams::ZERO);
    }

    /// Falsifier (trial/010): every non-exhausted band/tier cell must be
    /// reachable through real gather execution at its exact-full,
    /// one-gram-short partial, and empty-stock boundaries.
    #[test]
    fn falsification_all_active_cells_are_reachable() {
        let bands = [StaminaBand::Low, StaminaBand::Steady, StaminaBand::Fresh];
        let tiers = [
            InfraTier::None,
            InfraTier::Crude,
            InfraTier::Established,
            InfraTier::Advanced,
        ];
        let mut cells_reached = 0;
        let mut full_cases = 0;
        let mut partial_cases = 0;
        let mut empty_cases = 0;

        for band in bands {
            for tier in tiers {
                let expected_yield = YIELD_TABLE_GRAMS[band.index()][tier.index()];
                let expected_cost = STAMINA_COST_BY_BAND[band.index()];
                assert!(
                    expected_yield > 0,
                    "every active cell needs a nonzero partial boundary"
                );

                // Exact requested stock is the Accepted/Partial boundary:
                // equality grants the full cell yield and remains Accepted.
                let mut full_world = active_cell_world(band, tier, expected_yield);
                let full = submit_active_cell(&mut full_world);
                assert_eq!(
                    full.outcome,
                    OutcomeKind::Accepted,
                    "exact-full cell {}/{}",
                    band.code(),
                    tier.code()
                );
                assert_eq!(full.band, Some(band));
                assert_eq!(full.tier, Some(tier));
                assert_eq!(full.stamina_spent, expected_cost);
                assert_eq!(full.mass_moved, MassGrams::new(expected_yield));
                assert_cell_mass_landed(&full_world, expected_yield);
                assert_eq!(
                    full_world.economy.total_mass(),
                    MassGrams::new(expected_yield)
                );
                full_cases += 1;

                // One gram below the requested cell value must expose the
                // partial boundary without changing the value under test.
                let partial_stock = expected_yield - 1;
                let mut partial_world = active_cell_world(band, tier, partial_stock);
                let partial = submit_active_cell(&mut partial_world);
                assert_eq!(
                    partial.outcome,
                    OutcomeKind::Partial(PartialReason::SiteNearlyDepleted),
                    "one-short cell {}/{}",
                    band.code(),
                    tier.code()
                );
                assert_eq!(partial.band, Some(band));
                assert_eq!(partial.tier, Some(tier));
                assert_eq!(partial.stamina_spent, expected_cost);
                assert_eq!(partial.mass_moved, MassGrams::new(partial_stock));
                assert_cell_mass_landed(&partial_world, partial_stock);
                assert_eq!(
                    partial_world.economy.total_mass(),
                    MassGrams::new(partial_stock)
                );
                partial_cases += 1;

                // Empty stock is reached only after the coherent social,
                // band/cost, tier, and yield-selection path. It refuses
                // before apply, so the world stays byte-identical.
                let mut empty_world = active_cell_world(band, tier, 0);
                let empty_before = empty_world.canonical_state();
                let empty = submit_active_cell(&mut empty_world);
                assert_eq!(
                    empty.outcome,
                    OutcomeKind::Refused(RefusalReason::SiteEmpty),
                    "empty cell {}/{}",
                    band.code(),
                    tier.code()
                );
                assert_eq!(empty.band, Some(band));
                assert_eq!(empty.tier, Some(tier));
                assert_eq!(empty.stamina_spent, 0);
                assert_eq!(empty.mass_moved, MassGrams::ZERO);
                assert_eq!(empty_world.canonical_state(), empty_before);
                empty_cases += 1;

                println!(
                    "active_cell band={} tier={} yield_g={} gather_cost={} \
                     full=accepted partial_g={} empty=site_empty",
                    band.code(),
                    tier.code(),
                    expected_yield,
                    expected_cost,
                    partial_stock
                );
                cells_reached += 1;
            }
        }

        assert_eq!(cells_reached, 12);
        assert_eq!(full_cases, 12);
        assert_eq!(partial_cases, 12);
        assert_eq!(empty_cases, 12);
        println!(
            "active_cell_reachability cells={cells_reached}/12 cases={} \
             full={full_cases} partial={partial_cases} empty={empty_cases}",
            full_cases + partial_cases + empty_cases
        );
    }

    /// Falsifier (trial/003, part 1): two plans for two different
    /// characters at two different sites, validated against the same
    /// snapshot, are fully independent — neither invalidates the other,
    /// and both must commit without a stale panic.
    #[test]
    fn falsification_independent_plans_against_one_snapshot_must_both_commit() {
        let mut world = two_actor_world();
        let plan_a = plan_gather(
            &world,
            &GatherCommand {
                actor: CharacterId(1),
                claim: ClaimId(1),
                site: SiteId(1),
            },
        )
        .expect("plan A validates");
        let plan_b = plan_gather(
            &world,
            &GatherCommand {
                actor: CharacterId(2),
                claim: ClaimId(2),
                site: SiteId(2),
            },
        )
        .expect("plan B validates against the same snapshot");
        plan_a.apply(&mut world);
        plan_b.apply(&mut world);
        assert_eq!(
            world.characters.stamina(CharacterId(1)).unwrap().points(),
            80
        );
        assert_eq!(
            world.characters.stamina(CharacterId(2)).unwrap().points(),
            38
        );
        assert_eq!(
            world.economy.inventory(CharacterId(1)),
            MassGrams::new(1800)
        );
        assert_eq!(world.economy.inventory(CharacterId(2)), MassGrams::new(800));
    }

    /// Falsifier (trial/003, part 2): when a later owner's token in a
    /// plan has gone stale, the commit must be all-or-nothing. A partial
    /// commit — the character spend landing while the extraction panics —
    /// is a world state no receipt accounts for.
    #[test]
    fn falsification_stale_later_token_must_not_leave_partial_commit() {
        let mut world = two_actor_world();
        let plan = plan_gather(
            &world,
            &GatherCommand {
                actor: CharacterId(1),
                claim: ClaimId(1),
                site: SiteId(1),
            },
        )
        .expect("plan validates");
        // An economy-only commit lands between planning and committing:
        // C2 extracts from the same site, staling the plan's extraction
        // token but not its character token.
        let other = world
            .economy
            .validate_extract(SiteId(1), CharacterId(2), MassGrams::new(100))
            .unwrap();
        world.economy.apply_extract(other);
        let hash_before_commit = world.hash();
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            plan.apply(&mut world);
        }));
        assert!(
            outcome.is_err(),
            "the economy token is stale — the plan must refuse to commit"
        );
        assert_eq!(
            world.hash(),
            hash_before_commit,
            "stale plan committed partially: the character spend landed without the extraction"
        );
    }
}
