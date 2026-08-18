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
use crate::economy::{EconomyOwner, Extraction, Transfer};
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

/// The closed resource-kind vocabulary (RES01). Three kinds, licensed by
/// the author on 2026-08-18 as the first list: what the winter scene
/// needs and nothing more. There is deliberately no generic catch-all
/// member — a catch-all would absorb exactly the pressure that is
/// supposed to force a named kind and a permissioned move, and it would
/// make cross-kind leakage unfalsifiable. Kinds carry identity and
/// conservation only; no kind has behaviour of its own yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceKind {
    Fodder,
    Food,
    Timber,
}

pub const KIND_COUNT: usize = 3;

impl ResourceKind {
    pub const ALL: [Self; KIND_COUNT] = [Self::Fodder, Self::Food, Self::Timber];

    pub fn index(self) -> usize {
        match self {
            Self::Fodder => 0,
            Self::Food => 1,
            Self::Timber => 2,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::Fodder => "fodder",
            Self::Food => "food",
            Self::Timber => "timber",
        }
    }

    /// Test-only, following the `parse_text_command` precedent: nothing
    /// in the running truth layer parses a kind from text, and a code
    /// path that could would be a language seam needing its own trial.
    #[cfg(test)]
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.code() == code)
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

/// Stamina cost of one give. Verb policy, owned by the boundary: a flat
/// cost with no band table and no exhausted gate — the witness verb's
/// policy family. An exhausted person may still hand over what they
/// hold; a person with nothing left cannot. Mechanical example number.
pub const GIVE_COST: u8 = 3;

/// Fingerprint of the grammar that produced a receipt: the yield table,
/// the cost table, every resource-kind code, the actual band mapping over
/// the full stamina range, and every closed reason code. Change any of them and every subsequent
/// receipt carries a different fingerprint, so a trial record always says
/// which grammar version produced it.
/// Separator between declared fields in a format fingerprint: without
/// it, `("ab", "c")` and `("a", "bc")` would hash alike.
const FORMAT_FIELD_SEP: u8 = 0x1f;

/// The canonical command encoding, declared: each verb code, then every
/// field it contributes to `Command::canonical_bytes`, in order, with
/// the byte width that field occupies. An optional id is width 9 — one
/// presence byte plus eight. This table is the *declaration* of the
/// encoding; `canonical_bytes` is its implementation, and
/// `command_encoding_matches_its_declaration` proves they agree.
pub const COMMAND_ENCODING: [(&str, &[(&str, u8)]); 3] = [
    ("gather", &[("actor", 8), ("claim", 8), ("site", 8)]),
    ("witness", &[("witness", 8), ("claim", 8)]),
    (
        "give",
        &[
            ("giver", 8),
            ("recipient", 8),
            ("kind", 1),
            ("grams", 8),
            ("witness", 9),
        ],
    ),
];

/// The canonical receipt format, declared: the field names of
/// `Receipt::canonical_line`, in printed order. Same relationship as
/// above — `receipt_format_matches_its_declaration` proves the printed
/// line agrees with this list.
pub const RECEIPT_FIELDS: [&str; 19] = [
    "seq",
    "verb",
    "actor",
    "claim",
    "site",
    "to",
    "outcome",
    "reason",
    "claim_witnessed",
    "transfer_witness",
    "stamina_before",
    "band",
    "tier",
    "kind",
    "spent",
    "mass_g",
    "grammar",
    "world_before",
    "world",
];

/// Identity of the canonical command bytes — what a foreign caller must
/// produce for its input to mean anything. Deliberately separate from
/// the grammar: renaming a command field is not a gameplay change, and
/// conflating the two makes both unreadable (review finding 3).
pub fn command_encoding_fingerprint() -> u64 {
    let mut hasher = Fnv1a::default();
    for (verb, fields) in COMMAND_ENCODING {
        hasher.update(verb.as_bytes());
        hasher.update(&[FORMAT_FIELD_SEP]);
        for (name, width) in fields {
            hasher.update(name.as_bytes());
            hasher.update(&[*width, FORMAT_FIELD_SEP]);
        }
    }
    hasher.finish()
}

/// Identity of the canonical receipt line — what a reader of the ledger
/// must expect. Separate from the grammar for the same reason.
pub fn receipt_format_fingerprint() -> u64 {
    let mut hasher = Fnv1a::default();
    for field in RECEIPT_FIELDS {
        hasher.update(field.as_bytes());
        hasher.update(&[FORMAT_FIELD_SEP]);
    }
    hasher.finish()
}

/// The grammar identity this crate is licensed to carry. Moving it is a
/// declared spec evolution: the author licenses the move, a trial
/// pre-registers the new value from the declared inputs, and this
/// constant is edited in the same commit that changes them.
/// History: `0x530003916889b952` (two verbs, undifferentiated mass) ->
/// `0xc5d782ec145af0a5` (RES01: fodder/food/timber) ->
/// `0x7dd8c6706e0b949f` (V01: the give verb).
#[cfg(test)]
pub const LICENSED_GRAMMAR_FINGERPRINT: u64 = 0x7dd8_c670_6e0b_949f;

/// The canonical-command-encoding identity this crate is licensed to
/// carry (author licence, 2026-08-18). Pre-registered in
/// `docs/trial-v01-repair-preregistration.md` §2 before implementation.
#[cfg(test)]
pub const LICENSED_COMMAND_ENCODING_FINGERPRINT: u64 = 0xfa37_eefa_3594_cfe3;

/// The canonical-receipt-format identity this crate is licensed to
/// carry, pre-registered in the same place.
#[cfg(test)]
pub const LICENSED_RECEIPT_FORMAT_FINGERPRINT: u64 = 0x7e62_1526_22bb_9132;

pub fn grammar_fingerprint() -> u64 {
    let mut hasher = Fnv1a::default();
    for row in YIELD_TABLE_GRAMS {
        for cell in row {
            hasher.update(&cell.to_be_bytes());
        }
    }
    hasher.update(&STAMINA_COST_BY_BAND);
    hasher.update(&[WITNESS_COST]);
    hasher.update(&[GIVE_COST]);
    for kind in ResourceKind::ALL {
        hasher.update(kind.code().as_bytes());
    }
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
    UnknownRecipient,
    CannotGiveToSelf,
    InsufficientHolding,
    EmptyTransfer,
    UnknownWitness,
    WitnessIsParty,
}

impl RefusalReason {
    pub const ALL: [Self; 17] = [
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
        Self::UnknownRecipient,
        Self::CannotGiveToSelf,
        Self::InsufficientHolding,
        Self::EmptyTransfer,
        Self::UnknownWitness,
        Self::WitnessIsParty,
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
            Self::UnknownRecipient => "unknown_recipient",
            Self::CannotGiveToSelf => "cannot_give_to_self",
            Self::InsufficientHolding => "insufficient_holding",
            Self::EmptyTransfer => "empty_transfer",
            Self::UnknownWitness => "unknown_witness",
            Self::WitnessIsParty => "witness_is_party",
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
    Give,
}

impl Verb {
    pub fn code(self) -> &'static str {
        match self {
            Self::Gather => "gather",
            Self::Witness => "witness",
            Self::Give => "give",
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

/// An attributed transfer (V01). The command names its source, and no
/// accepted transfer debits a holding other than that named source — so
/// a command shape that moves someone else's stock does not exist.
///
/// That is **attribution, not consent**: nothing here proves the named
/// giver wanted it. Whether a character wills an act needs an issuer, a
/// player seat, delegation, or actor intent, none of which exists yet
/// (review finding 2). The design intent is a voluntary transfer; the
/// evidence is attribution, and the two must not be confused.
///
/// The witness, when named, is recorded on the receipt by identity and
/// pays nothing — nobody else's act may spend a third party.
#[derive(Debug, Clone, Copy)]
pub struct GiveCommand {
    pub giver: CharacterId,
    pub recipient: CharacterId,
    pub kind: ResourceKind,
    pub grams: MassGrams,
    pub witness: Option<CharacterId>,
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Gather(GatherCommand),
    Witness(WitnessCommand),
    Give(GiveCommand),
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
            Self::Give(give) => {
                let mut bytes = Vec::with_capacity(38);
                bytes.extend_from_slice(b"give");
                bytes.extend_from_slice(&give.giver.0.to_be_bytes());
                bytes.extend_from_slice(&give.recipient.0.to_be_bytes());
                bytes.push(give.kind.index() as u8);
                bytes.extend_from_slice(&give.grams.grams().to_be_bytes());
                // Presence byte first: an absent witness and a witness
                // whose id happens to be zero must not share an encoding.
                match give.witness {
                    Some(witness) => {
                        bytes.push(1);
                        bytes.extend_from_slice(&witness.0.to_be_bytes());
                    }
                    None => bytes.push(0),
                }
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
    UnknownKind,
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
        Some("give") => {
            if fields.len() != 6 {
                return Err(TextCommandFault::WrongFieldCount);
            }
            let kind_code = field_value(fields[3], "kind")?;
            let kind = ResourceKind::from_code(kind_code).ok_or(TextCommandFault::UnknownKind)?;
            let witness_field = field_value(fields[5], "witness")?;
            // "-" is the only spelling of an absent witness; an empty
            // value is still a fault, so absence is explicit rather than
            // inferred from a missing field.
            let witness = if witness_field == "-" {
                None
            } else {
                Some(CharacterId(parse_text_u64(witness_field)?))
            };
            Ok(Command::Give(GiveCommand {
                giver: CharacterId(parse_text_u64(field_value(fields[1], "giver")?)?),
                recipient: CharacterId(parse_text_u64(field_value(fields[2], "to")?)?),
                kind,
                grams: MassGrams::new(parse_text_u64(field_value(fields[4], "g")?)?),
                witness,
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
    /// The claim a verb acted through. `None` for verbs that act on no
    /// claim — a transfer moves what the giver already holds, and
    /// pretending it had a claim would be a receipt that lies.
    pub claim: Option<ClaimId>,
    pub site: Option<SiteId>,
    /// The other party of a transfer (V01). `None` for verbs that have
    /// no counterparty; the actor is always the character whose holding
    /// or stamina is spent.
    pub recipient: Option<CharacterId>,
    pub outcome: OutcomeKind,
    /// Whether the claim this verb acted through was witnessed. Claim
    /// witnessing only — a transfer acts through no claim and always
    /// reports `false` here (review finding 1: one boolean must not
    /// carry two verb-local meanings).
    pub claim_witnessed: bool,
    /// The third party a transfer named as its witness, by identity.
    /// `None` for an unwitnessed transfer and for every verb that has no
    /// transfer witness. Recorded, never stateful: the named witness
    /// pays nothing and no owner state changes.
    pub transfer_witness: Option<CharacterId>,
    pub stamina_before: Option<Stamina>,
    pub band: Option<StaminaBand>,
    pub tier: Option<InfraTier>,
    /// The resource kind the addressed site yields. Informational on a
    /// refusal, exactly like `band` and `tier`; on a mass-moving receipt
    /// it is part of the audited claim, and the shadow oracles recompute
    /// it from the fixture rather than reading it here.
    pub kind: Option<ResourceKind>,
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
        let kind = self.kind.map_or("-", ResourceKind::code);
        let site = self
            .site
            .map_or_else(|| "-".to_owned(), |s| format!("S{}", s.0));
        let recipient = self
            .recipient
            .map_or_else(|| "-".to_owned(), |c| format!("C{}", c.0));
        let claim = self
            .claim
            .map_or_else(|| "-".to_owned(), |k| format!("K{}", k.0));
        let transfer_witness = self
            .transfer_witness
            .map_or_else(|| "-".to_owned(), |c| format!("C{}", c.0));
        format!(
            "seq={} verb={} actor=C{} claim={} site={} to={} outcome={} reason={} \
             claim_witnessed={} transfer_witness={} \
             stamina_before={} band={} tier={} kind={} spent={} mass_g={} \
             grammar=0x{:016x} world_before=0x{:016x} world=0x{:016x}",
            self.seq,
            self.verb.code(),
            self.actor.0,
            claim,
            site,
            recipient,
            self.outcome.code(),
            self.outcome.reason_code(),
            self.claim_witnessed,
            transfer_witness,
            stamina_before,
            band,
            tier,
            kind,
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
            let mut line = format!("character C{} stamina={}", id.0, stamina.points());
            // Every kind is printed, always, including zeros: the closed
            // vocabulary drives the format, so a new kind cannot appear
            // in truth without appearing here.
            for kind in ResourceKind::ALL {
                line.push_str(&format!(
                    " {}_g={}",
                    kind.code(),
                    self.economy.holding(id, kind).grams()
                ));
            }
            lines.push(line);
        }
        for (id, tier, kind, stock) in self.economy.sites_iter() {
            lines.push(format!(
                "site S{} tier={} kind={} stock_g={}",
                id.0,
                tier.code(),
                kind.code(),
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
    kind: ResourceKind,
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
    let kind = extraction.kind();
    Ok(GatherPlan {
        witness,
        spend,
        extraction,
        partial,
        band,
        tier,
        kind,
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

/// A fully validated give. Same doctrine as the other plans: every gate
/// passed before any apply, tokens consumed by value.
struct GivePlan {
    spend: StaminaSpend,
    transfer: Transfer,
}

impl GivePlan {
    /// Commit phase for a planned transfer; same two-phase doctrine as
    /// `GatherPlan::apply`. The social owner is untouched: witnessing a
    /// give is receipted, never stateful.
    fn apply(self, world: &mut World) {
        assert!(
            world.characters.spend_is_fresh(&self.spend)
                && world.economy.transfer_is_fresh(&self.transfer),
            "stale plan token — boundary bug (commit refused before any mutation)"
        );
        world.characters.apply_spend(self.spend);
        world.economy.apply_transfer(self.transfer);
    }
}

fn plan_give(world: &World, cmd: &GiveCommand) -> Result<GivePlan, RefusalReason> {
    // 1. Parties gate (boundary verb policy). A transfer needs two
    //    distinct, existing characters and a nonzero mass; a named
    //    witness must exist and must not be either party, because a
    //    party attesting its own transfer attests nothing.
    if cmd.giver == cmd.recipient {
        return Err(RefusalReason::CannotGiveToSelf);
    }
    if world.characters.stamina(cmd.recipient).is_none() {
        return Err(RefusalReason::UnknownRecipient);
    }
    if let Some(witness) = cmd.witness {
        if world.characters.stamina(witness).is_none() {
            return Err(RefusalReason::UnknownWitness);
        }
        if witness == cmd.giver || witness == cmd.recipient {
            return Err(RefusalReason::WitnessIsParty);
        }
    }
    if cmd.grams.is_zero() {
        return Err(RefusalReason::EmptyTransfer);
    }
    // 2. Character gate. Give verb policy: flat cost, no band table, no
    //    exhausted gate — the witness verb's policy family. The owner
    //    only checks existence and exact headroom.
    let spend = world.characters.validate_spend(cmd.giver, GIVE_COST)?;
    // 3. Economy gate: the giver must actually hold the named mass of
    //    the named kind. Exact — no partial, no clamping.
    let transfer =
        world
            .economy
            .validate_transfer(cmd.giver, cmd.recipient, cmd.kind, cmd.grams)?;
    Ok(GivePlan { spend, transfer })
}

/// Submit one command through the boundary. All validation happens before
/// any owner apply; the applies consume their proof tokens by value and
/// panic only on a stale token, which is a boundary bug, never a game
/// outcome.
pub fn submit(world: &mut World, seq: u64, cmd: Command) -> Receipt {
    match cmd {
        Command::Gather(gather) => submit_gather(world, seq, gather),
        Command::Witness(witness) => submit_witness(world, seq, witness),
        Command::Give(give) => submit_give(world, seq, give),
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
            claim: Some(cmd.claim),
            site: Some(cmd.site),
            recipient: None,
            outcome: OutcomeKind::Refused(reason),
            claim_witnessed: witnessed,
            transfer_witness: None,
            stamina_before,
            band: stamina_before.map(Stamina::band),
            tier: world.economy.tier(cmd.site),
            kind: world.economy.site_kind(cmd.site),
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
            let (band, tier, kind) = (plan.band, plan.tier, plan.kind);
            // The commit phase consumes the plan and its tokens by value.
            // The social owner's state is unchanged by a gather.
            plan.apply(world);
            Receipt {
                seq,
                verb: Verb::Gather,
                actor: cmd.actor,
                claim: Some(claim),
                site: Some(cmd.site),
                recipient: None,
                outcome,
                claim_witnessed: witnessed,
                transfer_witness: None,
                stamina_before,
                band: Some(band),
                tier: Some(tier),
                kind: Some(kind),
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
            claim: Some(cmd.claim),
            site,
            recipient: None,
            outcome: OutcomeKind::Refused(reason),
            claim_witnessed: witnessed,
            transfer_witness: None,
            stamina_before,
            band: stamina_before.map(Stamina::band),
            tier: None,
            kind: None,
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
                claim: Some(claim),
                site,
                recipient: None,
                outcome: OutcomeKind::Accepted,
                claim_witnessed: witnessed,
                transfer_witness: None,
                stamina_before,
                band: stamina_before.map(Stamina::band),
                tier: None,
                kind: None,
                stamina_spent,
                mass_moved: MassGrams::ZERO,
                grammar,
                world_hash_before,
                world_hash_after: world.hash(),
            }
        }
    }
}

/// A give's receipt records what a transfer is: who acted, who received,
/// which kind, how much, and **which third party** was named as its
/// witness. The witness is recorded by identity, not by a flag: two
/// transfers attested by different people are different facts, and a
/// ledger that cannot tell them apart is not a ledger (review finding
/// 1). `claim_witnessed` is always false here — a transfer acts through
/// no claim, and `claim=-` says so.
fn submit_give(world: &mut World, seq: u64, cmd: GiveCommand) -> Receipt {
    let world_hash_before = world.hash();
    let grammar = grammar_fingerprint();
    let stamina_before = world.characters.stamina(cmd.giver);

    match plan_give(world, &cmd) {
        Err(reason) => Receipt {
            seq,
            verb: Verb::Give,
            actor: cmd.giver,
            claim: None,
            site: None,
            recipient: Some(cmd.recipient),
            outcome: OutcomeKind::Refused(reason),
            claim_witnessed: false,
            transfer_witness: cmd.witness,
            stamina_before,
            band: stamina_before.map(Stamina::band),
            tier: None,
            kind: Some(cmd.kind),
            stamina_spent: 0,
            mass_moved: MassGrams::ZERO,
            grammar,
            world_hash_before,
            world_hash_after: world.hash(),
        },
        Ok(plan) => {
            let stamina_spent = plan.spend.cost();
            let mass_moved = plan.transfer.grams();
            let kind = plan.transfer.kind();
            plan.apply(world);
            Receipt {
                seq,
                verb: Verb::Give,
                actor: cmd.giver,
                claim: None,
                site: None,
                recipient: Some(cmd.recipient),
                outcome: OutcomeKind::Accepted,
                claim_witnessed: false,
                transfer_witness: cmd.witness,
                stamina_before,
                band: stamina_before.map(Stamina::band),
                tier: None,
                kind: Some(kind),
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

    /// The licensed grammar identity (RES01). `AGENTS.md` §4 freezes the
    /// fingerprint; this pin makes a move a *declared* edit of one
    /// constant rather than a side effect noticed in an envelope line.
    /// It was pre-registered before the change: see
    /// `docs/trial-res01-resource-kinds-report.md` §4.
    #[test]
    fn grammar_fingerprint_matches_the_licensed_value() {
        assert_eq!(
            grammar_fingerprint(),
            LICENSED_GRAMMAR_FINGERPRINT,
            "the grammar moved without a licensed, pre-registered move"
        );
    }

    /// The licensed canonical-language identities (V01 repair). Each is
    /// computed from a disjoint declared input set, so an edit to one
    /// input moves exactly one number and turns exactly one pin red —
    /// which is what makes the three-way split real rather than
    /// cosmetic (falsifier R3).
    #[test]
    fn canonical_language_identities_match_their_licensed_values() {
        assert_eq!(
            command_encoding_fingerprint(),
            LICENSED_COMMAND_ENCODING_FINGERPRINT,
            "the canonical command encoding moved without a licensed, pre-registered move"
        );
        assert_eq!(
            receipt_format_fingerprint(),
            LICENSED_RECEIPT_FORMAT_FINGERPRINT,
            "the canonical receipt format moved without a licensed, pre-registered move"
        );
        // The three identities are distinct numbers over disjoint
        // inputs: a coincidence here would hide a move.
        assert_ne!(command_encoding_fingerprint(), receipt_format_fingerprint());
        assert_ne!(command_encoding_fingerprint(), grammar_fingerprint());
        assert_ne!(receipt_format_fingerprint(), grammar_fingerprint());
    }

    /// The command-encoding declaration is not decorative: the bytes a
    /// command actually produces have exactly the declared width, verb
    /// by verb.
    #[test]
    fn command_encoding_matches_its_declaration() {
        let samples = [
            Command::Gather(GatherCommand {
                actor: CharacterId(1),
                claim: ClaimId(2),
                site: SiteId(3),
            }),
            Command::Witness(WitnessCommand {
                witness: CharacterId(1),
                claim: ClaimId(2),
            }),
            Command::Give(GiveCommand {
                giver: CharacterId(1),
                recipient: CharacterId(2),
                kind: ResourceKind::Food,
                grams: MassGrams::new(5),
                witness: Some(CharacterId(3)),
            }),
        ];
        for (sample, (verb, fields)) in samples.iter().zip(COMMAND_ENCODING) {
            let bytes = sample.canonical_bytes();
            let declared: usize = verb.len()
                + fields
                    .iter()
                    .map(|(_, width)| *width as usize)
                    .sum::<usize>();
            assert_eq!(
                bytes.len(),
                declared,
                "{verb} encodes {} bytes, declares {declared}",
                bytes.len()
            );
            assert!(bytes.starts_with(verb.as_bytes()), "{verb} prefix");
        }
        // The optional witness field is the declared 9 bytes only when
        // present; absent, it is the presence byte alone.
        let silent = Command::Give(GiveCommand {
            giver: CharacterId(1),
            recipient: CharacterId(2),
            kind: ResourceKind::Food,
            grams: MassGrams::new(5),
            witness: None,
        });
        assert_eq!(
            silent.canonical_bytes().len(),
            samples[2].canonical_bytes().len() - 8
        );
    }

    /// The receipt-format declaration is not decorative either: the
    /// printed line's field names are exactly `RECEIPT_FIELDS`, in
    /// order, for every verb.
    #[test]
    fn receipt_format_matches_its_declaration() {
        let mut world = give_world();
        let receipts = [
            submit(
                &mut world,
                1,
                Command::Gather(GatherCommand {
                    actor: CharacterId(1),
                    claim: ClaimId(1),
                    site: SiteId(1),
                }),
            ),
            submit(
                &mut world,
                2,
                Command::Witness(WitnessCommand {
                    witness: CharacterId(2),
                    claim: ClaimId(1),
                }),
            ),
            submit(
                &mut world,
                3,
                give(1, 2, ResourceKind::Fodder, 100, Some(3)),
            ),
        ];
        for receipt in receipts {
            let line = receipt.canonical_line();
            let names: Vec<&str> = line
                .split(' ')
                .map(|field| {
                    field
                        .split_once('=')
                        .expect("every receipt field is name=value")
                        .0
                })
                .collect();
            assert_eq!(
                names, RECEIPT_FIELDS,
                "receipt field names or order drifted"
            );
        }
    }

    /// Falsifier R1 (V01 repair): two transfers alike in everything but
    /// the witness's identity are different facts. The receipts must
    /// differ; the world state must not.
    #[test]
    fn falsification_two_witnesses_must_produce_two_different_receipts() {
        let mut by_c3 = give_world();
        let mut by_c4 = give_world();
        let first = submit(
            &mut by_c3,
            3,
            give(1, 2, ResourceKind::Fodder, 400, Some(3)),
        );
        let second = submit(
            &mut by_c4,
            3,
            give(1, 2, ResourceKind::Fodder, 400, Some(4)),
        );
        assert_eq!(first.transfer_witness, Some(CharacterId(3)));
        assert_eq!(second.transfer_witness, Some(CharacterId(4)));
        assert_ne!(
            first.canonical_line(),
            second.canonical_line(),
            "two different attesters produced the same receipt"
        );
        assert!(first.canonical_line().contains(" transfer_witness=C3 "));
        assert!(second.canonical_line().contains(" transfer_witness=C4 "));
        assert_eq!(
            by_c3.canonical_state(),
            by_c4.canonical_state(),
            "naming a different witness changed canonical state"
        );
        assert_eq!(by_c3.hash(), by_c4.hash());
    }

    /// Falsifier R4 (V01 repair): claim witnessing and transfer
    /// witnessing are separately named on every receipt. A gather
    /// through a witnessed claim reports `claim_witnessed=true` and no
    /// transfer witness; a witnessed transfer reports the reverse.
    #[test]
    fn claim_witnessing_and_transfer_witnessing_are_never_the_same_field() {
        let mut world = give_world();
        let gathered = submit(
            &mut world,
            1,
            Command::Gather(GatherCommand {
                actor: CharacterId(1),
                claim: ClaimId(1),
                site: SiteId(1),
            }),
        );
        assert!(gathered.claim_witnessed);
        assert_eq!(gathered.transfer_witness, None);

        let given = submit(
            &mut world,
            2,
            give(1, 2, ResourceKind::Fodder, 100, Some(3)),
        );
        assert!(
            !given.claim_witnessed,
            "a transfer acts through no claim and must never report one witnessed"
        );
        assert_eq!(given.transfer_witness, Some(CharacterId(3)));
    }

    /// Falsifier F4 (RES01): the kind vocabulary is closed — every code
    /// round-trips, codes are distinct, and `ALL` is exhaustive in the
    /// sense that indexes cover `0..KIND_COUNT` exactly once.
    #[test]
    fn resource_kinds_are_a_closed_vocabulary() {
        let mut seen = [false; KIND_COUNT];
        for kind in ResourceKind::ALL {
            assert_eq!(ResourceKind::from_code(kind.code()), Some(kind));
            assert!(!kind.code().is_empty());
            assert!(!seen[kind.index()], "two kinds share an index");
            seen[kind.index()] = true;
        }
        assert!(seen.iter().all(|hit| *hit), "an index has no kind");
        assert_eq!(ResourceKind::from_code("turf"), None);
    }

    /// Falsifier F3 (RES01): a gather receipt names the kind of the site
    /// it drained — the actor cannot choose it, and a two-kind world
    /// proves the binding is to the site rather than to a default.
    #[test]
    fn falsification_receipt_kind_is_bound_to_the_drained_site() {
        let mut world = two_actor_world();
        let fodder = submit(
            &mut world,
            1,
            Command::Gather(GatherCommand {
                actor: CharacterId(1),
                claim: ClaimId(1),
                site: SiteId(1),
            }),
        );
        let timber = submit(
            &mut world,
            2,
            Command::Gather(GatherCommand {
                actor: CharacterId(2),
                claim: ClaimId(2),
                site: SiteId(2),
            }),
        );
        assert_eq!(fodder.kind, Some(ResourceKind::Fodder));
        assert_eq!(timber.kind, Some(ResourceKind::Timber));
        assert!(fodder.canonical_line().contains(" kind=fodder "));
        assert!(timber.canonical_line().contains(" kind=timber "));
        assert_eq!(
            world.economy.holding(CharacterId(1), ResourceKind::Timber),
            MassGrams::ZERO,
            "a fodder gather reached a timber holding"
        );
        assert_eq!(
            world.economy.holding(CharacterId(2), ResourceKind::Fodder),
            MassGrams::ZERO,
            "a timber gather reached a fodder holding"
        );
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
                (
                    SiteId(1),
                    InfraTier::Established,
                    ResourceKind::Fodder,
                    MassGrams::new(5000),
                ),
                (
                    SiteId(2),
                    InfraTier::Crude,
                    ResourceKind::Timber,
                    MassGrams::new(3000),
                ),
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
                (
                    SiteId(1),
                    InfraTier::Established,
                    ResourceKind::Fodder,
                    MassGrams::new(u64::MAX),
                ),
                (
                    SiteId(2),
                    InfraTier::Crude,
                    ResourceKind::Fodder,
                    MassGrams::new(1),
                ),
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
            economy: EconomyOwner::seed_sites([(
                SiteId(1),
                tier,
                ResourceKind::Fodder,
                MassGrams::new(stock),
            )])
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

    fn assert_cell_mass_state(world: &World, expected_inventory: u64, expected_stock: u64) {
        assert_eq!(
            world.economy.holding(CharacterId(1), ResourceKind::Fodder),
            MassGrams::new(expected_inventory)
        );
        let remaining_stock = world
            .economy
            .sites_iter()
            .next()
            .map(|(_, _, _, stock)| stock)
            .expect("the purpose-built site exists");
        assert_eq!(remaining_stock, MassGrams::new(expected_stock));
    }

    /// Falsifier (trial/010): every non-exhausted band/tier cell must be
    /// reachable through real gather execution at its full-path and
    /// empty-stock boundaries, plus a one-gram-short partial boundary when
    /// that boundary has positive stock. This deliberately admits zero and
    /// one-gram table values rather than turning the test into a value floor.
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
        let mut partial_boundaries = 0;
        let mut empty_cases = 0;

        for band in bands {
            for tier in tiers {
                let expected_yield = YIELD_TABLE_GRAMS[band.index()][tier.index()];
                let expected_cost = STAMINA_COST_BY_BAND[band.index()];

                // Keep the site nonempty even when the selected yield is zero:
                // reachability must observe the selected cell without imposing
                // a hidden lower bound on that cell's value.
                let full_stock = expected_yield.max(1);
                let mut full_world = active_cell_world(band, tier, full_stock);
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
                assert_cell_mass_state(&full_world, expected_yield, full_stock - expected_yield);
                assert_eq!(full_world.economy.total_mass(), MassGrams::new(full_stock));
                full_cases += 1;

                // A one-gram-short Partial exists only when that stock is
                // positive. Yield 0 has no lower value, and yield 1 reaches the
                // existing SiteEmpty guard at stock 0; neither is a failure.
                let partial_stock = expected_yield.checked_sub(1).filter(|stock| *stock > 0);
                if let Some(partial_stock) = partial_stock {
                    partial_boundaries += 1;
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
                    assert_cell_mass_state(&partial_world, partial_stock, 0);
                    assert_eq!(
                        partial_world.economy.total_mass(),
                        MassGrams::new(partial_stock)
                    );
                    partial_cases += 1;
                }

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
                     full=accepted partial_g={partial_stock:?} empty=site_empty",
                    band.code(),
                    tier.code(),
                    expected_yield,
                    expected_cost
                );
                cells_reached += 1;
            }
        }

        assert_eq!(cells_reached, 12);
        assert_eq!(full_cases, 12);
        assert_eq!(partial_cases, partial_boundaries);
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
            world.economy.holding(CharacterId(1), ResourceKind::Fodder),
            MassGrams::new(1800)
        );
        assert_eq!(
            world.economy.holding(CharacterId(2), ResourceKind::Timber),
            MassGrams::new(800)
        );
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

    /// Trial/013 training fixture. This proves conformance to the installed
    /// H-A mechanics at the Low boundary; by the sealed selection rule it
    /// cannot turn that mechanical match into authorial meaning.
    #[test]
    fn trial_013_low_actionability_training_does_not_select_meaning() {
        struct Expected {
            start: u8,
            band: StaminaBand,
            table_cost: u8,
            outcome: &'static str,
            reason: &'static str,
            spent: u8,
            mass: u64,
            post: u8,
            mutated: bool,
        }

        let expected = [
            Expected {
                start: 9,
                band: StaminaBand::Exhausted,
                table_cost: 0,
                outcome: "refused",
                reason: "actor_exhausted",
                spent: 0,
                mass: 0,
                post: 9,
                mutated: false,
            },
            Expected {
                start: 10,
                band: StaminaBand::Low,
                table_cost: 15,
                outcome: "refused",
                reason: "insufficient_stamina",
                spent: 0,
                mass: 0,
                post: 10,
                mutated: false,
            },
            Expected {
                start: 14,
                band: StaminaBand::Low,
                table_cost: 15,
                outcome: "refused",
                reason: "insufficient_stamina",
                spent: 0,
                mass: 0,
                post: 14,
                mutated: false,
            },
            Expected {
                start: 15,
                band: StaminaBand::Low,
                table_cost: 15,
                outcome: "accepted",
                reason: "-",
                spent: 15,
                mass: 600,
                post: 0,
                mutated: true,
            },
        ];

        let mut accepted_low = 0;
        let mut low_spent = 0_u64;
        let mut low_mass = 0_u64;
        for expected in expected {
            let mut world = World {
                characters: CharacterOwner::seed([(
                    CharacterId(1),
                    Stamina::new(expected.start).unwrap(),
                )])
                .unwrap(),
                economy: EconomyOwner::seed_sites([(
                    SiteId(1),
                    InfraTier::Established,
                    ResourceKind::Fodder,
                    MassGrams::new(10_000),
                )])
                .unwrap(),
                social: SocialOwner::seed_claims([(ClaimId(1), CharacterId(1), SiteId(1), true)])
                    .unwrap(),
            };
            validate_world_coherence(&world).unwrap();
            let before_hash = world.hash();
            let stamina = world.characters.stamina(CharacterId(1)).unwrap();
            let band = stamina.band();
            let table_cost = STAMINA_COST_BY_BAND[band.index()];
            let exact_headroom = stamina.spend_exact(table_cost).is_some();
            let receipt = submit(
                &mut world,
                1,
                Command::Gather(GatherCommand {
                    actor: CharacterId(1),
                    claim: ClaimId(1),
                    site: SiteId(1),
                }),
            );
            let post = world.characters.stamina(CharacterId(1)).unwrap().points();
            let mutated = before_hash != world.hash();
            println!(
                "trial013 training start={} band={} table_cost={} exact_headroom={} outcome={} reason={} spent={} mass={} post={} mutated={}",
                expected.start,
                band.code(),
                table_cost,
                exact_headroom,
                receipt.outcome.code(),
                receipt.outcome.reason_code(),
                receipt.stamina_spent,
                receipt.mass_moved.grams(),
                post,
                mutated,
            );

            assert_eq!(band, expected.band);
            assert_eq!(table_cost, expected.table_cost);
            assert_eq!(receipt.outcome.code(), expected.outcome);
            assert_eq!(receipt.outcome.reason_code(), expected.reason);
            assert_eq!(receipt.stamina_spent, expected.spent);
            assert_eq!(receipt.mass_moved.grams(), expected.mass);
            assert_eq!(post, expected.post);
            assert_eq!(mutated, expected.mutated);

            if band == StaminaBand::Low {
                accepted_low += usize::from(receipt.outcome == OutcomeKind::Accepted);
                low_spent += u64::from(receipt.stamina_spent);
                low_mass += receipt.mass_moved.grams();
            }
        }

        assert_eq!(accepted_low, 1);
        assert_eq!(low_spent, 15);
        assert_eq!(low_mass, 600);
        println!(
            "trial013 training_summary accepted_low=1/3 low_spent=15 low_mass=600 meaning_signal=none verdict=inconclusive holdout=sealed_unrevealed"
        );
    }

    // -----------------------------------------------------------------
    // V01: the give verb
    // -----------------------------------------------------------------

    /// A two-character world where C1 already holds fodder and timber,
    /// so a transfer has something to move. Mechanical numbers only.
    fn give_world() -> World {
        let mut world = World {
            characters: CharacterOwner::seed([
                (CharacterId(1), Stamina::new(90).unwrap()),
                (CharacterId(2), Stamina::new(50).unwrap()),
                (CharacterId(3), Stamina::new(2).unwrap()),
                // A fourth character exists only so that two DIFFERENT
                // third parties can attest the same transfer (R1).
                (CharacterId(4), Stamina::new(40).unwrap()),
            ])
            .unwrap(),
            economy: EconomyOwner::seed_sites([
                (
                    SiteId(1),
                    InfraTier::Established,
                    ResourceKind::Fodder,
                    MassGrams::new(5000),
                ),
                (
                    SiteId(2),
                    InfraTier::Crude,
                    ResourceKind::Timber,
                    MassGrams::new(3000),
                ),
            ])
            .unwrap(),
            social: SocialOwner::seed_claims([
                (ClaimId(1), CharacterId(1), SiteId(1), true),
                (ClaimId(2), CharacterId(1), SiteId(2), true),
            ])
            .unwrap(),
        };
        validate_world_coherence(&world).expect("give fixture is coherent");
        for (seq, claim, site) in [(1u64, 1u64, 1u64), (2, 2, 2)] {
            submit(
                &mut world,
                seq,
                Command::Gather(GatherCommand {
                    actor: CharacterId(1),
                    claim: ClaimId(claim),
                    site: SiteId(site),
                }),
            );
        }
        world
    }

    fn give(giver: u64, to: u64, kind: ResourceKind, grams: u64, witness: Option<u64>) -> Command {
        Command::Give(GiveCommand {
            giver: CharacterId(giver),
            recipient: CharacterId(to),
            kind,
            grams: MassGrams::new(grams),
            witness: witness.map(CharacterId),
        })
    }

    /// Falsifier G1 (V01), boundary form: the giver loses exactly what
    /// the recipient gains, in exactly one kind.
    #[test]
    fn falsification_give_conserves_the_transferred_kind() {
        let mut world = give_world();
        let before_total = world.economy.total_mass();
        let before_giver = world.economy.holding(CharacterId(1), ResourceKind::Fodder);
        let receipt = submit(&mut world, 3, give(1, 2, ResourceKind::Fodder, 500, None));
        assert_eq!(receipt.outcome, OutcomeKind::Accepted);
        assert_eq!(receipt.verb, Verb::Give);
        assert_eq!(receipt.mass_moved, MassGrams::new(500));
        assert_eq!(receipt.kind, Some(ResourceKind::Fodder));
        assert_eq!(receipt.recipient, Some(CharacterId(2)));
        assert_eq!(
            world.economy.holding(CharacterId(1), ResourceKind::Fodder),
            MassGrams::new(before_giver.grams() - 500)
        );
        assert_eq!(
            world.economy.holding(CharacterId(2), ResourceKind::Fodder),
            MassGrams::new(500)
        );
        assert_eq!(world.economy.total_mass(), before_total);
    }

    /// Falsifier G2 (V01, restated after review): **attribution**, not
    /// consent. The receipt's actor is always the character whose
    /// holding decreases, a give never reduces a third party, and no
    /// command shape could ask it to. What this does NOT show is that
    /// the named giver willed the transfer — any caller may submit a
    /// command naming any character until an issuer or seat exists.
    #[test]
    fn falsification_give_debits_only_the_commands_named_source() {
        let mut world = give_world();
        let before_c3 = world.economy.holding(CharacterId(3), ResourceKind::Timber);
        let receipt = submit(
            &mut world,
            3,
            give(1, 2, ResourceKind::Timber, 100, Some(3)),
        );
        assert_eq!(receipt.outcome, OutcomeKind::Accepted);
        assert_eq!(receipt.actor, CharacterId(1), "the actor is the giver");
        assert_eq!(
            world.economy.holding(CharacterId(3), ResourceKind::Timber),
            before_c3,
            "a named witness paid mass for someone else's act"
        );
        assert_eq!(
            world.characters.stamina(CharacterId(3)).unwrap().points(),
            2,
            "a named witness paid stamina for someone else's act"
        );
    }

    /// Falsifier G3/G4 (V01): every give refusal is reachable, names its
    /// closed reason, and mutates nothing — the world hash is identical
    /// across the refusal.
    #[test]
    fn falsification_give_refusals_are_closed_and_byte_stable() {
        let mut world = give_world();
        let held = world
            .economy
            .holding(CharacterId(1), ResourceKind::Fodder)
            .grams();
        let cases = [
            (
                give(1, 1, ResourceKind::Fodder, 10, None),
                RefusalReason::CannotGiveToSelf,
            ),
            (
                give(1, 9, ResourceKind::Fodder, 10, None),
                RefusalReason::UnknownRecipient,
            ),
            (
                give(1, 2, ResourceKind::Fodder, 10, Some(9)),
                RefusalReason::UnknownWitness,
            ),
            (
                give(1, 2, ResourceKind::Fodder, 10, Some(2)),
                RefusalReason::WitnessIsParty,
            ),
            (
                give(1, 2, ResourceKind::Fodder, 0, None),
                RefusalReason::EmptyTransfer,
            ),
            (
                give(1, 2, ResourceKind::Fodder, held + 1, None),
                RefusalReason::InsufficientHolding,
            ),
            (
                give(1, 2, ResourceKind::Food, 1, None),
                RefusalReason::InsufficientHolding,
            ),
            (
                give(9, 2, ResourceKind::Fodder, 10, None),
                RefusalReason::UnknownActor,
            ),
            (
                give(3, 2, ResourceKind::Fodder, 10, None),
                RefusalReason::InsufficientStamina,
            ),
        ];
        for (cmd, expected) in cases {
            let before = world.hash();
            let receipt = submit(&mut world, 3, cmd);
            assert_eq!(
                receipt.outcome,
                OutcomeKind::Refused(expected),
                "wrong reason for {:?}",
                receipt.canonical_line()
            );
            assert_eq!(receipt.stamina_spent, 0);
            assert_eq!(receipt.mass_moved, MassGrams::ZERO);
            assert_eq!(world.hash(), before, "a refused give mutated the world");
        }
    }

    /// Falsifier G5 (V01): a witnessed and an unwitnessed give differ in
    /// their receipts and are identical in world state.
    #[test]
    fn falsification_witnessing_a_give_is_receipted_but_never_stateful() {
        let mut witnessed_world = give_world();
        let mut silent_world = give_world();
        let witnessed = submit(
            &mut witnessed_world,
            3,
            give(1, 2, ResourceKind::Fodder, 400, Some(3)),
        );
        let silent = submit(
            &mut silent_world,
            3,
            give(1, 2, ResourceKind::Fodder, 400, None),
        );
        assert_eq!(witnessed.transfer_witness, Some(CharacterId(3)));
        assert_eq!(silent.transfer_witness, None);
        assert_ne!(witnessed.canonical_line(), silent.canonical_line());
        assert_eq!(
            witnessed_world.canonical_state(),
            silent_world.canonical_state(),
            "witnessing changed canonical state"
        );
        assert_eq!(witnessed_world.hash(), silent_world.hash());
    }

    /// Falsifier G6 (V01): giving everything of a kind leaves the giver
    /// indistinguishable from someone who never held it — in canonical
    /// text and in the hash.
    #[test]
    fn falsification_giving_everything_erases_the_holding_completely() {
        let mut world = give_world();
        let all = world
            .economy
            .holding(CharacterId(1), ResourceKind::Timber)
            .grams();
        let receipt = submit(&mut world, 3, give(1, 2, ResourceKind::Timber, all, None));
        assert_eq!(receipt.outcome, OutcomeKind::Accepted);
        assert_eq!(
            world.economy.holding(CharacterId(1), ResourceKind::Timber),
            MassGrams::ZERO
        );
        assert!(
            world
                .economy
                .holdings_iter()
                .all(|(id, kind, _)| !(id == CharacterId(1) && kind == ResourceKind::Timber)),
            "an emptied holding stayed in the map"
        );
        assert!(
            world
                .canonical_state()
                .iter()
                .any(|line| line.starts_with("character C1 ") && line.contains("timber_g=0")),
            "the canonical line must still print the kind, at zero"
        );
    }

    /// The text seam accepts the give spelling and nothing looser.
    #[test]
    fn text_seam_accepts_the_give_spelling() {
        let witnessed = Command::Give(GiveCommand {
            giver: CharacterId(1),
            recipient: CharacterId(2),
            kind: ResourceKind::Fodder,
            grams: MassGrams::new(500),
            witness: Some(CharacterId(3)),
        });
        let parsed = parse_text_command("give giver=1 to=2 kind=fodder g=500 witness=3")
            .expect("canonical give spelling");
        assert_eq!(parsed.canonical_bytes(), witnessed.canonical_bytes());

        let silent = Command::Give(GiveCommand {
            giver: CharacterId(1),
            recipient: CharacterId(2),
            kind: ResourceKind::Fodder,
            grams: MassGrams::new(500),
            witness: None,
        });
        let parsed_silent = parse_text_command("give giver=1 to=2 kind=fodder g=500 witness=-")
            .expect("canonical unwitnessed give spelling");
        assert_eq!(parsed_silent.canonical_bytes(), silent.canonical_bytes());
        assert_ne!(silent.canonical_bytes(), witnessed.canonical_bytes());

        for (source, expected) in [
            (
                "give giver=1 to=2 kind=turf g=500 witness=-",
                TextCommandFault::UnknownKind,
            ),
            (
                "give giver=1 to=2 kind=fodder g=+500 witness=-",
                TextCommandFault::NonCanonicalInteger,
            ),
            (
                "give giver=1 to=2 kind=fodder g=500",
                TextCommandFault::WrongFieldCount,
            ),
            (
                "give giver=1 to=2 kind=fodder g=500 witness=",
                TextCommandFault::EmptyValue,
            ),
        ] {
            assert!(
                matches!(parse_text_command(source), Err(actual) if actual == expected),
                "source {source:?} was not rejected as {expected:?}"
            );
        }
    }
}
