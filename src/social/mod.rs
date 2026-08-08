//! Social owner: the single writer of claims and witnessing.
//!
//! Internals are private. A gather never mutates social state in this
//! slice, so the owner exposes only read-only validation: the witnessed
//! claim is a boolean gate — an unwitnessed claim can never yield mass.
//! `WitnessPass` has private fields, so only this module can mint the proof
//! that the gate was passed.

use std::collections::BTreeMap;

use crate::boundary::{CharacterId, ClaimId, FixtureFault, Fnv1a, RefusalReason, SiteId};

struct Claim {
    holder: CharacterId,
    site: SiteId,
    witnessed: bool,
}

pub struct SocialOwner {
    claims: BTreeMap<ClaimId, Claim>,
}

/// Proof that the witness gate was passed for a claim. Private fields:
/// only the social owner can construct one.
pub struct WitnessPass {
    claim: ClaimId,
}

impl WitnessPass {
    pub fn claim(&self) -> ClaimId {
        self.claim
    }
}

impl SocialOwner {
    /// Seeding rejects duplicate IDs — no silent last-write-wins.
    pub fn seed_claims(
        entries: impl IntoIterator<Item = (ClaimId, CharacterId, SiteId, bool)>,
    ) -> Result<Self, FixtureFault> {
        let mut claims = BTreeMap::new();
        for (id, holder, site, witnessed) in entries {
            let claim = Claim {
                holder,
                site,
                witnessed,
            };
            if claims.insert(id, claim).is_some() {
                return Err(FixtureFault::DuplicateClaim(id));
            }
        }
        Ok(Self { claims })
    }

    pub fn claims_iter(&self) -> impl Iterator<Item = (ClaimId, CharacterId, SiteId, bool)> + '_ {
        self.claims
            .iter()
            .map(|(id, c)| (*id, c.holder, c.site, c.witnessed))
    }

    pub fn is_witnessed(&self, claim: ClaimId) -> Option<bool> {
        self.claims.get(&claim).map(|c| c.witnessed)
    }

    /// Read-only validation: the claim must exist, be held by the actor,
    /// cover the site, and be witnessed. Witnessing is a boolean gate.
    pub fn validate_witness_gate(
        &self,
        claim: ClaimId,
        actor: CharacterId,
        site: SiteId,
    ) -> Result<WitnessPass, RefusalReason> {
        let state = self.claims.get(&claim).ok_or(RefusalReason::UnknownClaim)?;
        if state.holder != actor {
            return Err(RefusalReason::ClaimNotHeldByActor);
        }
        if state.site != site {
            return Err(RefusalReason::ClaimSiteMismatch);
        }
        if !state.witnessed {
            return Err(RefusalReason::ClaimNotWitnessed);
        }
        Ok(WitnessPass { claim })
    }

    /// Deterministic: BTreeMap iterates in key order.
    pub fn hash_into(&self, hasher: &mut Fnv1a) {
        for (id, claim) in &self.claims {
            hasher.update(b"clm");
            hasher.update(&id.0.to_be_bytes());
            hasher.update(&claim.holder.0.to_be_bytes());
            hasher.update(&claim.site.0.to_be_bytes());
            hasher.update(&[u8::from(claim.witnessed)]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owner() -> SocialOwner {
        SocialOwner::seed_claims([
            (ClaimId(1), CharacterId(1), SiteId(1), true),
            (ClaimId(3), CharacterId(2), SiteId(1), false),
        ])
        .unwrap()
    }

    #[test]
    fn duplicate_seed_id_is_a_fixture_fault() {
        let result = SocialOwner::seed_claims([
            (ClaimId(1), CharacterId(1), SiteId(1), true),
            (ClaimId(1), CharacterId(2), SiteId(2), false),
        ]);
        assert_eq!(result.err(), Some(FixtureFault::DuplicateClaim(ClaimId(1))));
    }

    #[test]
    fn witnessed_claim_passes() {
        let pass = owner()
            .validate_witness_gate(ClaimId(1), CharacterId(1), SiteId(1))
            .unwrap();
        assert_eq!(pass.claim(), ClaimId(1));
    }

    #[test]
    fn unwitnessed_claim_is_gated() {
        assert_eq!(
            owner()
                .validate_witness_gate(ClaimId(3), CharacterId(2), SiteId(1))
                .err(),
            Some(RefusalReason::ClaimNotWitnessed)
        );
    }

    #[test]
    fn wrong_holder_is_refused() {
        assert_eq!(
            owner()
                .validate_witness_gate(ClaimId(1), CharacterId(2), SiteId(1))
                .err(),
            Some(RefusalReason::ClaimNotHeldByActor)
        );
    }

    #[test]
    fn wrong_site_is_refused() {
        assert_eq!(
            owner()
                .validate_witness_gate(ClaimId(1), CharacterId(1), SiteId(2))
                .err(),
            Some(RefusalReason::ClaimSiteMismatch)
        );
    }

    #[test]
    fn unknown_claim_is_refused() {
        assert_eq!(
            owner()
                .validate_witness_gate(ClaimId(9), CharacterId(1), SiteId(1))
                .err(),
            Some(RefusalReason::UnknownClaim)
        );
    }
}
