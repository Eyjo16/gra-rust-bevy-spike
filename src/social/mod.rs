//! Social owner: the single writer of claims and witnessing.
//!
//! Internals are private. A gather never mutates social state — it only
//! passes the read-only witness gate (`WitnessPass`): the witnessed claim
//! is a boolean gate, and an unwitnessed claim can never yield mass. The
//! witness verb is this owner's one mutation path: `validate_witness_grant`
//! (fallible, read-only) followed by `apply_witness`, which consumes the
//! `WitnessGrant` token by value — one token, one apply, revision-bound,
//! stale tokens panic. Same doctrine as the other two owners.

use std::collections::BTreeMap;

use crate::boundary::{CharacterId, ClaimId, FixtureFault, Fnv1a, RefusalReason, SiteId};

struct Claim {
    holder: CharacterId,
    site: SiteId,
    witnessed: bool,
}

pub struct SocialOwner {
    claims: BTreeMap<ClaimId, Claim>,
    revision: u64,
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

/// Proof that witnessing a claim was validated against a specific owner
/// revision. Private fields: only the social owner can construct one.
pub struct WitnessGrant {
    claim: ClaimId,
    from_revision: u64,
}

impl WitnessGrant {
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
        Ok(Self {
            claims,
            revision: 0,
        })
    }

    pub fn claim_site(&self, claim: ClaimId) -> Option<SiteId> {
        self.claims.get(&claim).map(|c| c.site)
    }

    pub fn revision(&self) -> u64 {
        self.revision
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

    /// Read-only validation for the witness verb: the claim must exist,
    /// must not already be witnessed, and the witness must not be its own
    /// holder.
    pub fn validate_witness_grant(
        &self,
        claim: ClaimId,
        witness: CharacterId,
    ) -> Result<WitnessGrant, RefusalReason> {
        let state = self.claims.get(&claim).ok_or(RefusalReason::UnknownClaim)?;
        if state.holder == witness {
            return Err(RefusalReason::CannotWitnessOwnClaim);
        }
        if state.witnessed {
            return Err(RefusalReason::ClaimAlreadyWitnessed);
        }
        Ok(WitnessGrant {
            claim,
            from_revision: self.revision,
        })
    }

    /// Consumes the token by value: reuse is a compile error. Panics on a
    /// stale token (minted against an older revision) — a boundary bug,
    /// never a game outcome.
    pub fn apply_witness(&mut self, grant: WitnessGrant) {
        assert_eq!(
            grant.from_revision, self.revision,
            "stale proof token (social) — boundary bug"
        );
        self.claims
            .get_mut(&grant.claim)
            .expect("fresh token: validated claim exists")
            .witnessed = true;
        self.revision += 1;
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
        hasher.update(b"soc-rev");
        hasher.update(&self.revision.to_be_bytes());
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

    #[test]
    fn witnessing_flips_the_gate_exactly_once() {
        let mut owner = owner();
        let grant = owner
            .validate_witness_grant(ClaimId(3), CharacterId(1))
            .unwrap();
        owner.apply_witness(grant);
        assert_eq!(owner.is_witnessed(ClaimId(3)), Some(true));
        assert_eq!(owner.revision(), 1);
        assert_eq!(
            owner
                .validate_witness_grant(ClaimId(3), CharacterId(1))
                .err(),
            Some(RefusalReason::ClaimAlreadyWitnessed)
        );
    }

    #[test]
    fn holder_cannot_witness_own_claim() {
        assert_eq!(
            owner()
                .validate_witness_grant(ClaimId(3), CharacterId(2))
                .err(),
            Some(RefusalReason::CannotWitnessOwnClaim)
        );
    }

    #[test]
    #[should_panic(expected = "stale proof token")]
    fn falsification_stale_witness_grant_panics_loudly() {
        let mut owner = owner();
        let first = owner
            .validate_witness_grant(ClaimId(3), CharacterId(1))
            .unwrap();
        let second = owner
            .validate_witness_grant(ClaimId(3), CharacterId(1))
            .unwrap();
        owner.apply_witness(first);
        owner.apply_witness(second);
    }
}
