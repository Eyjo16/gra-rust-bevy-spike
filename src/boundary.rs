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
//! the owner revision it was minted against; applying a stale token panics
//! loudly, because a stale token reaching an apply is a boundary bug, not
//! a game outcome.
//!
//! All fixture and table numbers in this crate are mechanical examples —
//! they are not balance and not historical truth.

use crate::character::{CharacterOwner, StaminaSpend};
use crate::economy::{EconomyOwner, Extraction};
use crate::social::{SocialOwner, WitnessPass};

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

    pub fn saturating_add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
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
}

impl RefusalReason {
    pub const ALL: [Self; 9] = [
        Self::UnknownActor,
        Self::UnknownSite,
        Self::UnknownClaim,
        Self::ClaimNotHeldByActor,
        Self::ClaimSiteMismatch,
        Self::ClaimNotWitnessed,
        Self::ActorExhausted,
        Self::InsufficientStamina,
        Self::SiteEmpty,
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
}

/// Referential integrity across owners: every claim must point at a known
/// holder and a known site. Duplicate IDs are already rejected at seed
/// time by each owner.
pub fn validate_world_coherence(world: &World) -> Result<(), FixtureFault> {
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

#[derive(Debug, Clone, Copy)]
pub struct GatherCommand {
    pub actor: CharacterId,
    pub claim: ClaimId,
    pub site: SiteId,
}

/// Canonical receipt for one submitted command. Every field is either a
/// closed code or a bounded integer, so the canonical line is
/// deterministic. It records the world hash both before and after the
/// command and the grammar fingerprint that produced it.
#[derive(Debug, Clone)]
pub struct Receipt {
    pub seq: u64,
    pub actor: CharacterId,
    pub claim: ClaimId,
    pub site: SiteId,
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
        format!(
            "seq={} actor=C{} claim=K{} site=S{} outcome={} reason={} witnessed={} \
             stamina_before={} band={} tier={} spent={} mass_g={} \
             grammar=0x{:016x} world_before=0x{:016x} world=0x{:016x}",
            self.seq,
            self.actor.0,
            self.claim.0,
            self.site.0,
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

/// Submit one command through the boundary. All validation happens before
/// any owner apply; the applies consume their proof tokens by value and
/// panic only on a stale token, which is a boundary bug, never a game
/// outcome.
pub fn submit(world: &mut World, seq: u64, cmd: GatherCommand) -> Receipt {
    let world_hash_before = world.hash();
    let grammar = grammar_fingerprint();
    let witnessed = world.social.is_witnessed(cmd.claim).unwrap_or(false);
    let stamina_before = world.characters.stamina(cmd.actor);

    match plan_gather(world, &cmd) {
        Err(reason) => Receipt {
            seq,
            actor: cmd.actor,
            claim: cmd.claim,
            site: cmd.site,
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
            // Applies consume the validated tokens by value.
            world.characters.apply_spend(plan.spend);
            world.economy.apply_extract(plan.extraction);
            // The social owner's state is unchanged by a gather in this slice.
            Receipt {
                seq,
                actor: cmd.actor,
                claim,
                site: cmd.site,
                outcome,
                witnessed,
                stamina_before,
                band: Some(plan.band),
                tier: Some(plan.tier),
                stamina_spent,
                mass_moved,
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
    fn fnv1a_is_stable() {
        let mut hasher = Fnv1a::default();
        hasher.update(b"truth");
        let first = hasher.finish();
        let mut again = Fnv1a::default();
        again.update(b"truth");
        assert_eq!(first, again.finish());
    }
}
