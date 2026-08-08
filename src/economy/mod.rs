//! Economy owner: the single writer of mass — site stock and inventories.
//!
//! Internals are private. The only mutation path is `validate_extract`
//! (fallible, read-only) followed by `apply_extract` (infallible).
//! `Extraction` has private fields, so only this module can mint the proof
//! that an extraction was validated. Mass is `MassGrams` (backed by `u64`),
//! so negative mass is unrepresentable anywhere in this owner.

use std::collections::BTreeMap;

use crate::boundary::{CharacterId, Fnv1a, InfraTier, MassGrams, RefusalReason, SiteId};

struct SiteState {
    tier: InfraTier,
    stock: MassGrams,
}

pub struct EconomyOwner {
    sites: BTreeMap<SiteId, SiteState>,
    inventories: BTreeMap<CharacterId, MassGrams>,
}

/// Proof that an extraction was validated. Private fields: only the economy
/// owner can construct one. `granted <= stock` held at validation time.
pub struct Extraction {
    site: SiteId,
    to: CharacterId,
    granted: MassGrams,
}

impl Extraction {
    pub fn granted(&self) -> MassGrams {
        self.granted
    }
}

impl EconomyOwner {
    pub fn seed_sites(entries: impl IntoIterator<Item = (SiteId, InfraTier, MassGrams)>) -> Self {
        Self {
            sites: entries
                .into_iter()
                .map(|(id, tier, stock)| (id, SiteState { tier, stock }))
                .collect(),
            inventories: BTreeMap::new(),
        }
    }

    pub fn tier(&self, site: SiteId) -> Option<InfraTier> {
        self.sites.get(&site).map(|s| s.tier)
    }

    pub fn stock(&self, site: SiteId) -> Option<MassGrams> {
        self.sites.get(&site).map(|s| s.stock)
    }

    pub fn inventory(&self, id: CharacterId) -> MassGrams {
        self.inventories
            .get(&id)
            .copied()
            .unwrap_or(MassGrams::ZERO)
    }

    /// Total mass across sites and inventories; conserved by every apply.
    pub fn total_mass(&self) -> MassGrams {
        let sites = self
            .sites
            .values()
            .fold(MassGrams::ZERO, |acc, s| acc.saturating_add(s.stock));
        self.inventories
            .values()
            .fold(sites, |acc, inv| acc.saturating_add(*inv))
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
        Ok(Extraction { site, to, granted })
    }

    /// Infallible apply: moves granted mass from site stock to the
    /// receiver's inventory. Total mass is conserved, and the checked
    /// subtraction (validated as `granted <= stock`) can never go below
    /// zero — the fallback keeps the apply infallible regardless.
    pub fn apply_extract(&mut self, extraction: &Extraction) {
        if let Some(state) = self.sites.get_mut(&extraction.site) {
            state.stock = state
                .stock
                .checked_sub(extraction.granted)
                .unwrap_or(MassGrams::ZERO);
        }
        let inventory = self
            .inventories
            .entry(extraction.to)
            .or_insert(MassGrams::ZERO);
        *inventory = inventory.saturating_add(extraction.granted);
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
        owner.apply_extract(&extraction);
        assert_eq!(owner.stock(SiteId(2)), Some(MassGrams::ZERO));
        assert_eq!(owner.inventory(CharacterId(2)), MassGrams::new(300));
    }

    #[test]
    fn empty_site_is_refused() {
        let mut owner = owner();
        let extraction = owner
            .validate_extract(SiteId(2), CharacterId(2), MassGrams::new(400))
            .unwrap();
        owner.apply_extract(&extraction);
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
        owner.apply_extract(&extraction);
        assert_eq!(owner.total_mass(), before);
    }
}
