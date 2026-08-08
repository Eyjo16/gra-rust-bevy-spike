//! Truth-layer boundary.
//!
//! Shared primitive types (typed IDs, `Stamina`, `MassGrams`), the single
//! active 4x4 design cell (Stamina x GatheringInfrastructure), the closed
//! outcome/reason vocabulary, canonical receipts, deterministic world
//! hashing, and the orchestrator that runs *every* validation before any
//! infallible owner apply.
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

    /// Spending can never underflow below zero.
    pub fn saturating_spend(self, cost: u8) -> Self {
        Self(self.0.saturating_sub(cost))
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
/// before the table is consulted (oracle 4 and oracle 6 check this).
pub const YIELD_TABLE_GRAMS: [[u64; CELL_COLS]; CELL_ROWS] = [
    [0, 0, 0, 0],
    [250, 400, 600, 900],
    [500, 800, 1200, 1800],
    [750, 1200, 1800, 2700],
];

/// Stamina cost of one gather by band. Mechanical example numbers only.
pub const STAMINA_COST_BY_BAND: [u8; CELL_ROWS] = [0, 15, 12, 10];

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
    SiteEmpty,
}

impl RefusalReason {
    pub const ALL: [Self; 8] = [
        Self::UnknownActor,
        Self::UnknownSite,
        Self::UnknownClaim,
        Self::ClaimNotHeldByActor,
        Self::ClaimSiteMismatch,
        Self::ClaimNotWitnessed,
        Self::ActorExhausted,
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
/// closed code or a bounded integer, so the canonical line is deterministic.
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
             stamina_before={} band={} tier={} spent={} mass_g={} world=0x{:016x}",
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
// Orchestration: validate everything, then apply infallibly
// ---------------------------------------------------------------------------

/// A fully validated gather. Holding one proves that every owner has
/// approved its part; the applies below cannot fail.
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
    // 2. Character gate: the actor must exist and not be exhausted.
    let spend = world.characters.validate_spend(cmd.actor)?;
    let band = spend.band();
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
/// any owner apply; the applies themselves are infallible.
pub fn submit(world: &mut World, seq: u64, cmd: GatherCommand) -> Receipt {
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
            world_hash_after: world.hash(),
        },
        Ok(plan) => {
            let outcome = plan
                .partial
                .map_or(OutcomeKind::Accepted, OutcomeKind::Partial);
            let stamina_spent = plan.spend.cost();
            let mass_moved = plan.extraction.granted();
            // Infallible applies — every check already passed.
            world.characters.apply_spend(&plan.spend);
            world.economy.apply_extract(&plan.extraction);
            // The social owner's state is unchanged by a gather in this slice.
            Receipt {
                seq,
                actor: cmd.actor,
                claim: plan.witness.claim(),
                site: cmd.site,
                outcome,
                witnessed,
                stamina_before,
                band: Some(plan.band),
                tier: Some(plan.tier),
                stamina_spent,
                mass_moved,
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
    fn fnv1a_is_stable() {
        let mut hasher = Fnv1a::default();
        hasher.update(b"truth");
        let first = hasher.finish();
        let mut again = Fnv1a::default();
        again.update(b"truth");
        assert_eq!(first, again.finish());
    }
}
