//! Economy owner: the single writer of mass — site stock and inventories.
//!
//! Internals are private. The only mutation path is `validate_extract`
//! (fallible, read-only) followed by `apply_extract`, which consumes the
//! proof token by value — one token, one apply. The token carries the
//! entity revisions it was minted against — the site it drains and
//! the inventory it fills — so extractions touching disjoint entities
//! never conflict. Applying a stale token panics, because that is a
//! boundary bug, never a game outcome. Mass is
//! `MassGrams` (backed by `u64`), so negative mass is unrepresentable
//! anywhere in this owner.

use std::collections::BTreeMap;

use crate::boundary::{
    CharacterId, FixtureFault, Fnv1a, InfraTier, MassGrams, RefusalReason, SiteId,
};

struct SiteState {
    tier: InfraTier,
    stock: MassGrams,
}

pub struct EconomyOwner {
    sites: BTreeMap<SiteId, SiteState>,
    inventories: BTreeMap<CharacterId, MassGrams>,
    /// Per-entity conflict granularity: an extraction binds to the one
    /// site and the one inventory it touches, so extractions at
    /// different sites for different actors never false-conflict.
    /// Derived bookkeeping, not truth state — excluded from the world
    /// hash (the owner-wide apply counter is hashed).
    site_revisions: BTreeMap<SiteId, u64>,
    inventory_revisions: BTreeMap<CharacterId, u64>,
    revision: u64,
}

/// Proof that an extraction was validated against specific entity
/// revisions of the site it drains and the inventory it fills, with
/// `granted <= stock` at validation time. Private fields: only the
/// economy owner can construct one.
pub struct Extraction {
    site: SiteId,
    to: CharacterId,
    granted: MassGrams,
    from_site_revision: u64,
    from_inventory_revision: u64,
}

impl Extraction {
    pub fn granted(&self) -> MassGrams {
        self.granted
    }
}

impl EconomyOwner {
    /// Seeding rejects duplicate IDs — no silent last-write-wins.
    pub fn seed_sites(
        entries: impl IntoIterator<Item = (SiteId, InfraTier, MassGrams)>,
    ) -> Result<Self, FixtureFault> {
        let mut sites = BTreeMap::new();
        for (id, tier, stock) in entries {
            if sites.insert(id, SiteState { tier, stock }).is_some() {
                return Err(FixtureFault::DuplicateSite(id));
            }
        }
        Ok(Self {
            sites,
            inventories: BTreeMap::new(),
            site_revisions: BTreeMap::new(),
            inventory_revisions: BTreeMap::new(),
            revision: 0,
        })
    }

    fn site_revision(&self, site: SiteId) -> u64 {
        self.site_revisions.get(&site).copied().unwrap_or(0)
    }

    fn inventory_revision(&self, id: CharacterId) -> u64 {
        self.inventory_revisions.get(&id).copied().unwrap_or(0)
    }

    /// True when the token still matches the revisions of both entities
    /// it touches. The boundary's commit phase checks every token in a
    /// plan BEFORE any owner mutates, so a stale plan is all-or-nothing.
    pub fn extraction_is_fresh(&self, extraction: &Extraction) -> bool {
        self.site_revision(extraction.site) == extraction.from_site_revision
            && self.inventory_revision(extraction.to) == extraction.from_inventory_revision
    }

    pub fn tier(&self, site: SiteId) -> Option<InfraTier> {
        self.sites.get(&site).map(|s| s.tier)
    }

    pub fn inventory(&self, id: CharacterId) -> MassGrams {
        self.inventories
            .get(&id)
            .copied()
            .unwrap_or(MassGrams::ZERO)
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn sites_iter(&self) -> impl Iterator<Item = (SiteId, InfraTier, MassGrams)> + '_ {
        self.sites
            .iter()
            .map(|(id, state)| (*id, state.tier, state.stock))
    }

    /// Exact total mass across sites and inventories, or `None` when an
    /// invalid fixture exceeds the canonical `u64` representation.
    pub fn checked_total_mass(&self) -> Option<MassGrams> {
        self.sites
            .values()
            .map(|site| site.stock)
            .chain(self.inventories.values().copied())
            .try_fold(MassGrams::ZERO, MassGrams::checked_add)
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
        Ok(Extraction {
            site,
            to,
            granted,
            from_site_revision: self.site_revision(site),
            from_inventory_revision: self.inventory_revision(to),
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
        let next_inventory = self
            .inventories
            .get(&extraction.to)
            .copied()
            .unwrap_or(MassGrams::ZERO)
            .checked_add(extraction.granted)
            .expect("coherent world: inventory addition fits total-mass bound");

        self.sites
            .get_mut(&extraction.site)
            .expect("fresh token: validated site exists")
            .stock = next_stock;
        self.inventories.insert(extraction.to, next_inventory);
        *self.site_revisions.entry(extraction.site).or_insert(0) += 1;
        *self.inventory_revisions.entry(extraction.to).or_insert(0) += 1;
        self.revision += 1;
    }

    /// Deterministic: BTreeMaps iterate in key order.
    pub fn hash_into(&self, hasher: &mut Fnv1a) {
        for (id, state) in &self.sites {
            hasher.update(b"sit");
            hasher.update(&id.0.to_be_bytes());
            hasher.update(&[state.tier.index() as u8]);
            hasher.update(&state.stock.grams().to_be_bytes());
        }
        for (id, inventory) in &self.inventories {
            hasher.update(b"inv");
            hasher.update(&id.0.to_be_bytes());
            hasher.update(&inventory.grams().to_be_bytes());
        }
        hasher.update(b"eco-rev");
        hasher.update(&self.revision.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owner() -> EconomyOwner {
        EconomyOwner::seed_sites([
            (SiteId(1), InfraTier::Established, MassGrams::new(2000)),
            (SiteId(2), InfraTier::Crude, MassGrams::new(300)),
        ])
        .unwrap()
    }

    fn stock_of(owner: &EconomyOwner, site: SiteId) -> MassGrams {
        owner
            .sites_iter()
            .find(|(id, _, _)| *id == site)
            .map(|(_, _, stock)| stock)
            .expect("site exists")
    }

    #[test]
    fn duplicate_seed_id_is_a_fixture_fault() {
        let result = EconomyOwner::seed_sites([
            (SiteId(1), InfraTier::Established, MassGrams::new(2000)),
            (SiteId(1), InfraTier::Crude, MassGrams::new(300)),
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
        assert_eq!(owner.inventory(CharacterId(2)), MassGrams::new(300));
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
    /// `u64::MAX` grams in one inventory while another site still holds
    /// one gram. Applying that last gram must fail loudly before any
    /// mutation; silently clamping the inventory destroys mass while the
    /// saturating total reports the same value on both sides.
    #[test]
    fn falsification_overfull_inventory_must_not_silently_clamp() {
        let mut owner = EconomyOwner::seed_sites([
            (SiteId(1), InfraTier::Established, MassGrams::new(u64::MAX)),
            (SiteId(2), InfraTier::Crude, MassGrams::new(1)),
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
            "u64::MAX + 1 inventory transfer silently clamped instead of failing"
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
            owner
                .holdings_iter()
                .all(|(_, _, grams)| !grams.is_zero()),
            "a zero-valued holding entry was stored"
        );
    }

}
