//! Economy owner: the single writer of mass — site stock and per-kind
//! holdings.
//!
//! Internals are private. The mutation paths are `validate_extract` /
//! `apply_extract` (site -> holding) and `validate_transfer` /
//! `apply_transfer` (holding -> holding, V01); each is a fallible
//! read-only validation followed by an apply that consumes the proof
//! token by value — one token, one apply. The token carries the
//! entity revisions it was minted against — the site it drains and
//! the (character, kind) holding it fills — so extractions touching
//! disjoint entities never conflict. Applying a stale token panics, because that is a
//! boundary bug, never a game outcome. Mass is
//! `MassGrams` (backed by `u64`), so negative mass is unrepresentable
//! anywhere in this owner.

use std::collections::BTreeMap;

use crate::boundary::{
    CharacterId, FixtureFault, Fnv1a, InfraTier, MassGrams, RefusalReason, ResourceKind, SiteId,
};

struct SiteState {
    tier: InfraTier,
    /// Fixed at seed time: a site yields exactly one kind. Nothing in
    /// this owner can change it, so a gather cannot choose what a site
    /// gives up.
    kind: ResourceKind,
    stock: MassGrams,
}

pub struct EconomyOwner {
    sites: BTreeMap<SiteId, SiteState>,
    /// Holdings are per (character, kind). A zero-valued entry is never
    /// stored, so the hash and the canonical text are both functions of
    /// the same visible truth: a holding spent to nothing is
    /// indistinguishable from one that never existed.
    holdings: BTreeMap<(CharacterId, ResourceKind), MassGrams>,
    /// Per-entity conflict granularity: an extraction binds to the one
    /// site and the one inventory it touches, so extractions at
    /// different sites for different actors never false-conflict.
    /// Derived bookkeeping, not truth state — excluded from the world
    /// hash (the owner-wide apply counter is hashed).
    site_revisions: BTreeMap<SiteId, u64>,
    holding_revisions: BTreeMap<(CharacterId, ResourceKind), u64>,
    revision: u64,
}

/// Proof that a transfer between two characters was validated against
/// specific `(character, kind)` holding revisions, with
/// `grams <= giver's holding` at validation time. Private fields: only
/// the economy owner can construct one. The giver is named first and is
/// the only holding that can decrease — the owner has no path that
/// reduces a holding the actor does not own.
pub struct Transfer {
    from: CharacterId,
    to: CharacterId,
    kind: ResourceKind,
    grams: MassGrams,
    from_holding_revision: u64,
    to_holding_revision: u64,
}

impl Transfer {
    pub fn grams(&self) -> MassGrams {
        self.grams
    }

    pub fn kind(&self) -> ResourceKind {
        self.kind
    }
}

/// Proof that an extraction was validated against specific entity
/// revisions of the site it drains and the inventory it fills, with
/// `granted <= stock` at validation time. Private fields: only the
/// economy owner can construct one.
pub struct Extraction {
    site: SiteId,
    to: CharacterId,
    kind: ResourceKind,
    granted: MassGrams,
    from_site_revision: u64,
    from_holding_revision: u64,
}

impl Extraction {
    pub fn granted(&self) -> MassGrams {
        self.granted
    }

    /// The kind the drained site yields — never a caller's choice.
    pub fn kind(&self) -> ResourceKind {
        self.kind
    }
}

impl EconomyOwner {
    /// Seeding rejects duplicate IDs — no silent last-write-wins.
    pub fn seed_sites(
        entries: impl IntoIterator<Item = (SiteId, InfraTier, ResourceKind, MassGrams)>,
    ) -> Result<Self, FixtureFault> {
        let mut sites = BTreeMap::new();
        for (id, tier, kind, stock) in entries {
            if sites.insert(id, SiteState { tier, kind, stock }).is_some() {
                return Err(FixtureFault::DuplicateSite(id));
            }
        }
        Ok(Self {
            sites,
            holdings: BTreeMap::new(),
            site_revisions: BTreeMap::new(),
            holding_revisions: BTreeMap::new(),
            revision: 0,
        })
    }

    fn site_revision(&self, site: SiteId) -> u64 {
        self.site_revisions.get(&site).copied().unwrap_or(0)
    }

    fn holding_revision(&self, id: CharacterId, kind: ResourceKind) -> u64 {
        self.holding_revisions
            .get(&(id, kind))
            .copied()
            .unwrap_or(0)
    }

    /// The one write path for a holding. Normalizes zero to absence, so
    /// no state that prints identically can hash differently.
    fn set_holding(&mut self, id: CharacterId, kind: ResourceKind, grams: MassGrams) {
        if grams.is_zero() {
            self.holdings.remove(&(id, kind));
        } else {
            self.holdings.insert((id, kind), grams);
        }
        *self.holding_revisions.entry((id, kind)).or_insert(0) += 1;
    }

    /// True when the token still matches the revisions of both entities
    /// it touches. The boundary's commit phase checks every token in a
    /// plan BEFORE any owner mutates, so a stale plan is all-or-nothing.
    pub fn extraction_is_fresh(&self, extraction: &Extraction) -> bool {
        self.site_revision(extraction.site) == extraction.from_site_revision
            && self.holding_revision(extraction.to, extraction.kind)
                == extraction.from_holding_revision
    }

    pub fn tier(&self, site: SiteId) -> Option<InfraTier> {
        self.sites.get(&site).map(|s| s.tier)
    }

    pub fn site_kind(&self, site: SiteId) -> Option<ResourceKind> {
        self.sites.get(&site).map(|s| s.kind)
    }

    pub fn holding(&self, id: CharacterId, kind: ResourceKind) -> MassGrams {
        self.holdings
            .get(&(id, kind))
            .copied()
            .unwrap_or(MassGrams::ZERO)
    }

    /// Every stored holding, in deterministic key order. Zero-valued
    /// entries are never stored, so this iterator is exactly the set of
    /// characters who hold something.
    pub fn holdings_iter(
        &self,
    ) -> impl Iterator<Item = (CharacterId, ResourceKind, MassGrams)> + '_ {
        self.holdings
            .iter()
            .map(|((id, kind), grams)| (*id, *kind, *grams))
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn sites_iter(
        &self,
    ) -> impl Iterator<Item = (SiteId, InfraTier, ResourceKind, MassGrams)> + '_ {
        self.sites
            .iter()
            .map(|(id, state)| (*id, state.tier, state.kind, state.stock))
    }

    /// Exact total mass across sites and inventories, or `None` when an
    /// invalid fixture exceeds the canonical `u64` representation.
    pub fn checked_total_mass(&self) -> Option<MassGrams> {
        self.sites
            .values()
            .map(|site| site.stock)
            .chain(self.holdings.values().copied())
            .try_fold(MassGrams::ZERO, MassGrams::checked_add)
    }

    /// Exact total of one kind across sites and holdings, or `None` when
    /// an invalid fixture exceeds the canonical `u64` representation.
    pub fn checked_total_mass_of(&self, kind: ResourceKind) -> Option<MassGrams> {
        self.sites
            .values()
            .filter(|site| site.kind == kind)
            .map(|site| site.stock)
            .chain(
                self.holdings
                    .iter()
                    .filter(|((_, held), _)| *held == kind)
                    .map(|(_, grams)| *grams),
            )
            .try_fold(MassGrams::ZERO, MassGrams::checked_add)
    }

    /// Total of one kind in a world that passed `validate_world_coherence`.
    pub fn total_mass_of(&self, kind: ResourceKind) -> MassGrams {
        self.checked_total_mass_of(kind)
            .expect("coherent world: per-kind total fits u64")
    }

    /// Total mass in a world that passed `validate_world_coherence`.
    /// Every extraction only moves mass, so the established bound is
    /// inductive and this remains representable for the whole run.
    pub fn total_mass(&self) -> MassGrams {
        self.checked_total_mass()
            .expect("coherent world: total mass fits u64")
    }

    /// Read-only validation: the site must exist and hold stock. Grants the
    /// full request or, when stock is short, whatever remains (the boundary
    /// reports that as a Partial outcome).
    pub fn validate_extract(
        &self,
        site: SiteId,
        to: CharacterId,
        requested: MassGrams,
    ) -> Result<Extraction, RefusalReason> {
        let state = self.sites.get(&site).ok_or(RefusalReason::UnknownSite)?;
        if state.stock.is_zero() {
            return Err(RefusalReason::SiteEmpty);
        }
        let granted = requested.min(state.stock);
        let kind = state.kind;
        Ok(Extraction {
            site,
            to,
            kind,
            granted,
            from_site_revision: self.site_revision(site),
            from_holding_revision: self.holding_revision(to, kind),
        })
    }

    /// Consumes the token by value: reuse is a compile error. Panics on a
    /// stale token (an older revision of the same site or inventory) — a
    /// boundary bug, never a game outcome. Extractions touching disjoint
    /// entities are independent and never conflict. For a fresh validated
    /// token the checked subtraction cannot fail, and total mass is
    /// conserved.
    pub fn apply_extract(&mut self, extraction: Extraction) {
        assert!(
            self.extraction_is_fresh(&extraction),
            "stale proof token (economy) — boundary bug"
        );
        // Compute every fallible arithmetic result before the first write.
        // Coherence establishes total mass <= u64::MAX; extraction only
        // moves mass, therefore inventory + grant also fits that bound.
        let state = self
            .sites
            .get(&extraction.site)
            .expect("fresh token: validated site exists");
        let next_stock = state
            .stock
            .checked_sub(extraction.granted)
            .expect("fresh token: validated granted <= stock");
        // The grant lands in the holding of the SITE's kind. Nothing on
        // this path can name a different kind, so cross-kind leakage is
        // unreachable rather than merely untested.
        let next_holding = self
            .holding(extraction.to, extraction.kind)
            .checked_add(extraction.granted)
            .expect("coherent world: holding addition fits total-mass bound");

        self.sites
            .get_mut(&extraction.site)
            .expect("fresh token: validated site exists")
            .stock = next_stock;
        self.set_holding(extraction.to, extraction.kind, next_holding);
        *self.site_revisions.entry(extraction.site).or_insert(0) += 1;
        self.revision += 1;
    }

    /// True when the token still matches the revisions of both holdings
    /// it touches.
    pub fn transfer_is_fresh(&self, transfer: &Transfer) -> bool {
        self.holding_revision(transfer.from, transfer.kind) == transfer.from_holding_revision
            && self.holding_revision(transfer.to, transfer.kind) == transfer.to_holding_revision
    }

    /// Read-only validation of a voluntary transfer (V01). Exact: a giver
    /// short of `grams` is refused, never partially satisfied — a giver's
    /// own store is not a fact they can be surprised by, unlike a site's
    /// remaining stock. The owner enforces resource semantics only; who
    /// may give, at what cost, and whether the parties are distinct is
    /// verb policy and lives in the boundary.
    pub fn validate_transfer(
        &self,
        from: CharacterId,
        to: CharacterId,
        kind: ResourceKind,
        grams: MassGrams,
    ) -> Result<Transfer, RefusalReason> {
        let held = self.holding(from, kind);
        if held.checked_sub(grams).is_none() {
            return Err(RefusalReason::InsufficientHolding);
        }
        Ok(Transfer {
            from,
            to,
            kind,
            grams,
            from_holding_revision: self.holding_revision(from, kind),
            to_holding_revision: self.holding_revision(to, kind),
        })
    }

    /// Consumes the token by value: reuse is a compile error. Panics on a
    /// stale token — a boundary bug, never a game outcome. Both sides of
    /// the transfer are computed before the first write, so the applied
    /// move is all-or-nothing and the kind total is unchanged by
    /// construction: the same grams leave one holding and enter the other.
    pub fn apply_transfer(&mut self, transfer: Transfer) {
        assert!(
            self.transfer_is_fresh(&transfer),
            "stale proof token (economy transfer) — boundary bug"
        );
        let next_from = self
            .holding(transfer.from, transfer.kind)
            .checked_sub(transfer.grams)
            .expect("fresh token: validated grams <= holding");
        let next_to = self
            .holding(transfer.to, transfer.kind)
            .checked_add(transfer.grams)
            .expect("coherent world: transfer addition fits total-mass bound");

        self.set_holding(transfer.from, transfer.kind, next_from);
        self.set_holding(transfer.to, transfer.kind, next_to);
        self.revision += 1;
    }

    /// Deterministic: BTreeMaps iterate in key order.
    pub fn hash_into(&self, hasher: &mut Fnv1a) {
        for (id, state) in &self.sites {
            hasher.update(b"sit");
            hasher.update(&id.0.to_be_bytes());
            hasher.update(&[state.tier.index() as u8]);
            hasher.update(&[state.kind.index() as u8]);
            hasher.update(&state.stock.grams().to_be_bytes());
        }
        for ((id, kind), grams) in &self.holdings {
            hasher.update(b"hld");
            hasher.update(&id.0.to_be_bytes());
            hasher.update(&[kind.index() as u8]);
            hasher.update(&grams.grams().to_be_bytes());
        }
        hasher.update(b"eco-rev");
        hasher.update(&self.revision.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FODDER: ResourceKind = ResourceKind::Fodder;

    fn owner() -> EconomyOwner {
        EconomyOwner::seed_sites([
            (
                SiteId(1),
                InfraTier::Established,
                FODDER,
                MassGrams::new(2000),
            ),
            (SiteId(2), InfraTier::Crude, FODDER, MassGrams::new(300)),
        ])
        .unwrap()
    }

    fn stock_of(owner: &EconomyOwner, site: SiteId) -> MassGrams {
        owner
            .sites_iter()
            .find(|(id, _, _, _)| *id == site)
            .map(|(_, _, _, stock)| stock)
            .expect("site exists")
    }

    #[test]
    fn duplicate_seed_id_is_a_fixture_fault() {
        let result = EconomyOwner::seed_sites([
            (
                SiteId(1),
                InfraTier::Established,
                FODDER,
                MassGrams::new(2000),
            ),
            (SiteId(1), InfraTier::Crude, FODDER, MassGrams::new(300)),
        ]);
        assert_eq!(result.err(), Some(FixtureFault::DuplicateSite(SiteId(1))));
    }

    #[test]
    fn unknown_site_is_refused() {
        assert_eq!(
            owner()
                .validate_extract(SiteId(9), CharacterId(1), MassGrams::new(100))
                .err(),
            Some(RefusalReason::UnknownSite)
        );
    }

    #[test]
    fn short_stock_grants_the_remainder() {
        let mut owner = owner();
        let extraction = owner
            .validate_extract(SiteId(2), CharacterId(2), MassGrams::new(400))
            .unwrap();
        assert_eq!(extraction.granted(), MassGrams::new(300));
        owner.apply_extract(extraction);
        assert_eq!(stock_of(&owner, SiteId(2)), MassGrams::ZERO);
        assert_eq!(owner.holding(CharacterId(2), FODDER), MassGrams::new(300));
    }

    #[test]
    fn empty_site_is_refused() {
        let mut owner = owner();
        let extraction = owner
            .validate_extract(SiteId(2), CharacterId(2), MassGrams::new(400))
            .unwrap();
        owner.apply_extract(extraction);
        assert_eq!(
            owner
                .validate_extract(SiteId(2), CharacterId(2), MassGrams::new(1))
                .err(),
            Some(RefusalReason::SiteEmpty)
        );
    }

    #[test]
    fn extraction_conserves_total_mass() {
        let mut owner = owner();
        let before = owner.total_mass();
        let extraction = owner
            .validate_extract(SiteId(1), CharacterId(1), MassGrams::new(1800))
            .unwrap();
        owner.apply_extract(extraction);
        assert_eq!(owner.total_mass(), before);
        assert_eq!(owner.revision(), 1);
    }

    /// The red form of this test applied one `&Extraction` twice and
    /// created 1600 g out of thin air (2300 g -> 3900 g). Reusing one
    /// token is now a compile error (consumed by value); the remaining
    /// runtime attack — two tokens minted against the same revision —
    /// must panic loudly instead of silently minting mass.
    #[test]
    #[should_panic(expected = "stale proof token")]
    fn falsification_token_replay_must_not_create_mass() {
        let mut owner = owner();
        let first = owner
            .validate_extract(SiteId(1), CharacterId(1), MassGrams::new(1800))
            .unwrap();
        let second = owner
            .validate_extract(SiteId(1), CharacterId(1), MassGrams::new(1800))
            .unwrap();
        owner.apply_extract(first);
        owner.apply_extract(second);
    }

    /// Falsifier (trial/008): an invalid overfull fixture can put
    /// `u64::MAX` grams in one holding while another site still holds
    /// one gram. Applying that last gram must fail loudly before any
    /// mutation; silently clamping the holding destroys mass while the
    /// saturating total reports the same value on both sides.
    #[test]
    fn falsification_overfull_holding_must_not_silently_clamp() {
        let mut owner = EconomyOwner::seed_sites([
            (
                SiteId(1),
                InfraTier::Established,
                FODDER,
                MassGrams::new(u64::MAX),
            ),
            (SiteId(2), InfraTier::Crude, FODDER, MassGrams::new(1)),
        ])
        .unwrap();

        let fill = owner
            .validate_extract(SiteId(1), CharacterId(1), MassGrams::new(u64::MAX))
            .unwrap();
        owner.apply_extract(fill);
        let before_hash = {
            let mut hasher = Fnv1a::default();
            owner.hash_into(&mut hasher);
            hasher.finish()
        };

        let overflow = owner
            .validate_extract(SiteId(2), CharacterId(1), MassGrams::new(1))
            .unwrap();
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            owner.apply_extract(overflow);
        }));

        assert!(
            outcome.is_err(),
            "u64::MAX + 1 holding transfer silently clamped instead of failing"
        );
        let mut after = Fnv1a::default();
        owner.hash_into(&mut after);
        assert_eq!(
            after.finish(),
            before_hash,
            "failed apply mutated economy before reporting overflow"
        );
    }

    /// Falsifier F1 (RES01). Two sites of different kinds. Extracting
    /// timber must not move a single gram into the actor's fodder or
    /// food holding. Undifferentiated mass cannot express this test at
    /// all: there is one number per character, so a "timber" gather and
    /// a "fodder" gather are the same fact.
    #[test]
    fn falsification_cross_kind_leakage_must_be_impossible() {
        let mut owner = EconomyOwner::seed_sites([
            (
                SiteId(1),
                InfraTier::Established,
                ResourceKind::Fodder,
                MassGrams::new(2000),
            ),
            (
                SiteId(2),
                InfraTier::Established,
                ResourceKind::Timber,
                MassGrams::new(2000),
            ),
        ])
        .unwrap();

        let extraction = owner
            .validate_extract(SiteId(2), CharacterId(1), MassGrams::new(500))
            .unwrap();
        assert_eq!(extraction.kind(), ResourceKind::Timber);
        owner.apply_extract(extraction);

        assert_eq!(
            owner.holding(CharacterId(1), ResourceKind::Timber),
            MassGrams::new(500)
        );
        for leaked in [ResourceKind::Fodder, ResourceKind::Food] {
            assert_eq!(
                owner.holding(CharacterId(1), leaked),
                MassGrams::ZERO,
                "timber extraction leaked into the {} holding",
                leaked.code()
            );
        }
    }

    /// Falsifier F2 (RES01): every kind's total is conserved on its own,
    /// not merely in aggregate.
    #[test]
    fn falsification_each_kind_total_is_conserved_separately() {
        let mut owner = EconomyOwner::seed_sites([
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
        .unwrap();
        let before: Vec<MassGrams> = ResourceKind::ALL
            .into_iter()
            .map(|kind| owner.total_mass_of(kind))
            .collect();

        for (site, actor, grams) in [(1u64, 1u64, 900u64), (2, 2, 400), (1, 2, 100)] {
            let extraction = owner
                .validate_extract(SiteId(site), CharacterId(actor), MassGrams::new(grams))
                .unwrap();
            owner.apply_extract(extraction);
        }

        let after: Vec<MassGrams> = ResourceKind::ALL
            .into_iter()
            .map(|kind| owner.total_mass_of(kind))
            .collect();
        assert_eq!(before, after, "a kind total changed under extraction only");
        assert_eq!(owner.total_mass(), MassGrams::new(2300));
    }

    /// Falsifier F5 (RES01 half): the owner never stores a zero-valued
    /// holding entry, so the world hash stays a function of visible
    /// truth. Extraction alone cannot reach zero (a granted extraction
    /// is always positive), so the reachable half of this falsifier is
    /// V01's give-to-zero; what is proved here is the storage rule.
    #[test]
    fn zero_holdings_are_never_stored() {
        let mut owner = EconomyOwner::seed_sites([(
            SiteId(1),
            InfraTier::Established,
            ResourceKind::Fodder,
            MassGrams::new(2000),
        )])
        .unwrap();
        let extraction = owner
            .validate_extract(SiteId(1), CharacterId(1), MassGrams::new(500))
            .unwrap();
        owner.apply_extract(extraction);
        assert!(
            owner.holdings_iter().all(|(_, _, grams)| !grams.is_zero()),
            "a zero-valued holding entry was stored"
        );
    }

    /// Falsifier G1 (V01): a transfer moves exactly one kind between
    /// exactly two holdings and touches nothing else.
    #[test]
    fn falsification_transfer_conserves_the_kind_and_touches_nothing_else() {
        let mut owner = EconomyOwner::seed_sites([
            (
                SiteId(1),
                InfraTier::Established,
                FODDER,
                MassGrams::new(2000),
            ),
            (
                SiteId(2),
                InfraTier::Established,
                ResourceKind::Timber,
                MassGrams::new(2000),
            ),
        ])
        .unwrap();
        for (site, actor, grams) in [(1u64, 1u64, 500u64), (2, 1, 300), (1, 2, 100)] {
            let extraction = owner
                .validate_extract(SiteId(site), CharacterId(actor), MassGrams::new(grams))
                .unwrap();
            owner.apply_extract(extraction);
        }
        let before: Vec<MassGrams> = ResourceKind::ALL
            .into_iter()
            .map(|kind| owner.total_mass_of(kind))
            .collect();

        let transfer = owner
            .validate_transfer(CharacterId(1), CharacterId(2), FODDER, MassGrams::new(200))
            .unwrap();
        assert_eq!(transfer.grams(), MassGrams::new(200));
        owner.apply_transfer(transfer);

        assert_eq!(owner.holding(CharacterId(1), FODDER), MassGrams::new(300));
        assert_eq!(owner.holding(CharacterId(2), FODDER), MassGrams::new(300));
        assert_eq!(
            owner.holding(CharacterId(1), ResourceKind::Timber),
            MassGrams::new(300),
            "the giver's other kind moved"
        );
        assert_eq!(
            owner.holding(CharacterId(2), ResourceKind::Timber),
            MassGrams::ZERO,
            "mass appeared in a kind nobody gave"
        );
        let after: Vec<MassGrams> = ResourceKind::ALL
            .into_iter()
            .map(|kind| owner.total_mass_of(kind))
            .collect();
        assert_eq!(before, after, "a transfer changed a kind total");
    }

    /// Falsifier G3 (V01): a giver short of the named mass is refused,
    /// never partially satisfied — the giver's own store is not a fact
    /// they can be surprised by.
    #[test]
    fn falsification_short_transfer_is_refused_not_clamped() {
        let mut owner = EconomyOwner::seed_sites([(
            SiteId(1),
            InfraTier::Established,
            FODDER,
            MassGrams::new(2000),
        )])
        .unwrap();
        let extraction = owner
            .validate_extract(SiteId(1), CharacterId(1), MassGrams::new(500))
            .unwrap();
        owner.apply_extract(extraction);

        assert_eq!(
            owner
                .validate_transfer(CharacterId(1), CharacterId(2), FODDER, MassGrams::new(501))
                .err(),
            Some(RefusalReason::InsufficientHolding)
        );
        assert_eq!(
            owner
                .validate_transfer(
                    CharacterId(1),
                    CharacterId(2),
                    ResourceKind::Timber,
                    MassGrams::new(1)
                )
                .err(),
            Some(RefusalReason::InsufficientHolding),
            "a kind the giver has never held is not a source of mass"
        );
        assert_eq!(owner.holding(CharacterId(1), FODDER), MassGrams::new(500));
    }

    /// Falsifier G6 (V01), owner half: giving everything of a kind
    /// leaves no zero entry, so the hash equals that of a world where
    /// the giver never held it.
    #[test]
    fn falsification_giving_everything_leaves_no_zero_entry() {
        let mut owner = EconomyOwner::seed_sites([(
            SiteId(1),
            InfraTier::Established,
            FODDER,
            MassGrams::new(2000),
        )])
        .unwrap();
        let extraction = owner
            .validate_extract(SiteId(1), CharacterId(1), MassGrams::new(500))
            .unwrap();
        owner.apply_extract(extraction);
        let transfer = owner
            .validate_transfer(CharacterId(1), CharacterId(2), FODDER, MassGrams::new(500))
            .unwrap();
        owner.apply_transfer(transfer);

        assert!(
            owner
                .holdings_iter()
                .all(|(id, _, grams)| { id != CharacterId(1) && !grams.is_zero() }),
            "an emptied holding stayed in the map"
        );

        // A reference world in which C1 never held anything, reached in
        // the same number of applies so the hashed owner counter matches:
        // same visible state must mean the same hash.
        let mut reference = EconomyOwner::seed_sites([(
            SiteId(1),
            InfraTier::Established,
            FODDER,
            MassGrams::new(2000),
        )])
        .unwrap();
        for _ in 0..2 {
            let direct = reference
                .validate_extract(SiteId(1), CharacterId(2), MassGrams::new(250))
                .unwrap();
            reference.apply_extract(direct);
        }
        let hash_of = |owner: &EconomyOwner| {
            let mut hasher = Fnv1a::default();
            owner.hash_into(&mut hasher);
            hasher.finish()
        };
        assert_eq!(
            hash_of(&owner),
            hash_of(&reference),
            "an emptied holding is still visible in the hash"
        );
    }
}
