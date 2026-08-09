//! Character owner: the single writer of character bodies (stamina).
//!
//! Internals are private. The only mutation path is `validate_spend`
//! (fallible, read-only) followed by `apply_spend`, which consumes the
//! proof token by value — one token, one apply. The token carries the
//! entity revision it was minted against — the one character it
//! touches — so spends for different characters never conflict.
//! Applying a stale token panics, because that is a boundary bug,
//! never a game outcome.
//!
//! This owner is verb-agnostic: the cost arrives as a parameter decided by
//! the boundary's verb policy. The owner enforces only resource semantics:
//! the actor exists and has exact headroom — no clamping.

use std::collections::BTreeMap;

use crate::boundary::{CharacterId, FixtureFault, Fnv1a, RefusalReason, Stamina};

pub struct CharacterOwner {
    stamina: BTreeMap<CharacterId, Stamina>,
    /// Per-character conflict granularity: a token binds to the revision
    /// of the one character it touches, so independent plans never
    /// false-conflict. Derived bookkeeping, not truth state — excluded
    /// from the world hash (the owner-wide apply counter is hashed).
    entity_revisions: BTreeMap<CharacterId, u64>,
    revision: u64,
}

/// Proof that a stamina spend was validated against a specific
/// character's entity revision. Private fields: only the character owner
/// can construct one.
pub struct StaminaSpend {
    id: CharacterId,
    cost: u8,
    from_revision: u64,
}

impl StaminaSpend {
    pub fn cost(&self) -> u8 {
        self.cost
    }
}

impl CharacterOwner {
    /// Seeding rejects duplicate IDs — no silent last-write-wins.
    pub fn seed(
        entries: impl IntoIterator<Item = (CharacterId, Stamina)>,
    ) -> Result<Self, FixtureFault> {
        let mut stamina = BTreeMap::new();
        for (id, points) in entries {
            if stamina.insert(id, points).is_some() {
                return Err(FixtureFault::DuplicateCharacter(id));
            }
        }
        Ok(Self {
            stamina,
            entity_revisions: BTreeMap::new(),
            revision: 0,
        })
    }

    fn entity_revision(&self, id: CharacterId) -> u64 {
        self.entity_revisions.get(&id).copied().unwrap_or(0)
    }

    /// True when the token still matches the revision of the character it
    /// was validated against. The boundary's commit phase checks every
    /// token in a plan BEFORE any owner mutates, so a stale plan is
    /// all-or-nothing.
    pub fn spend_is_fresh(&self, spend: &StaminaSpend) -> bool {
        self.entity_revision(spend.id) == spend.from_revision
    }

    pub fn stamina(&self, id: CharacterId) -> Option<Stamina> {
        self.stamina.get(&id).copied()
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn iter(&self) -> impl Iterator<Item = (CharacterId, Stamina)> + '_ {
        self.stamina.iter().map(|(id, stamina)| (*id, *stamina))
    }

    /// Read-only validation: the actor must exist and must have exact
    /// headroom for the requested cost. No clamping — a 12-point actor is
    /// refused a 15-point spend, not silently zeroed.
    pub fn validate_spend(&self, id: CharacterId, cost: u8) -> Result<StaminaSpend, RefusalReason> {
        let before = self.stamina(id).ok_or(RefusalReason::UnknownActor)?;
        if before.spend_exact(cost).is_none() {
            return Err(RefusalReason::InsufficientStamina);
        }
        Ok(StaminaSpend {
            id,
            cost,
            from_revision: self.entity_revision(id),
        })
    }

    /// Consumes the token by value: reuse is a compile error. Panics on a
    /// stale token (minted against an older revision of the same
    /// character) — a boundary bug, never a game outcome. Spends for
    /// different characters are independent and never conflict. The exact
    /// spend cannot fail for a fresh validated token.
    pub fn apply_spend(&mut self, spend: StaminaSpend) {
        assert!(
            self.spend_is_fresh(&spend),
            "stale proof token (character) — boundary bug"
        );
        let stamina = self
            .stamina
            .get_mut(&spend.id)
            .expect("fresh token: validated actor exists");
        *stamina = stamina
            .spend_exact(spend.cost)
            .expect("fresh token: validated headroom");
        *self.entity_revisions.entry(spend.id).or_insert(0) += 1;
        self.revision += 1;
    }

    /// Deterministic: BTreeMap iterates in key order.
    pub fn hash_into(&self, hasher: &mut Fnv1a) {
        for (id, stamina) in &self.stamina {
            hasher.update(b"chr");
            hasher.update(&id.0.to_be_bytes());
            hasher.update(&[stamina.points()]);
        }
        hasher.update(b"chr-rev");
        hasher.update(&self.revision.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owner() -> CharacterOwner {
        CharacterOwner::seed([
            (CharacterId(1), Stamina::new(90).unwrap()),
            (CharacterId(3), Stamina::new(5).unwrap()),
        ])
        .unwrap()
    }

    #[test]
    fn duplicate_seed_id_is_a_fixture_fault() {
        let result = CharacterOwner::seed([
            (CharacterId(1), Stamina::new(90).unwrap()),
            (CharacterId(1), Stamina::new(10).unwrap()),
        ]);
        assert_eq!(
            result.err(),
            Some(FixtureFault::DuplicateCharacter(CharacterId(1)))
        );
    }

    #[test]
    fn unknown_actor_is_refused() {
        assert_eq!(
            owner().validate_spend(CharacterId(99), 10).err(),
            Some(RefusalReason::UnknownActor)
        );
    }

    #[test]
    fn falsification_low_headroom_must_be_refused() {
        let owner = CharacterOwner::seed([(CharacterId(9), Stamina::new(12).unwrap())]).unwrap();
        assert_eq!(
            owner.validate_spend(CharacterId(9), 15).err(),
            Some(RefusalReason::InsufficientStamina)
        );
    }

    #[test]
    fn validated_spend_applies_exactly() {
        let mut owner = owner();
        let spend = owner.validate_spend(CharacterId(1), 10).unwrap();
        owner.apply_spend(spend);
        assert_eq!(owner.stamina(CharacterId(1)).unwrap().points(), 80);
        assert_eq!(owner.revision(), 1);
    }

    #[test]
    #[should_panic(expected = "stale proof token")]
    fn falsification_stale_token_panics_loudly() {
        let mut owner = owner();
        let first = owner.validate_spend(CharacterId(1), 10).unwrap();
        let second = owner.validate_spend(CharacterId(1), 10).unwrap();
        owner.apply_spend(first);
        owner.apply_spend(second);
    }

    /// Falsifier (trial/003): spends for two DIFFERENT characters
    /// validated against the same snapshot are independent — neither
    /// invalidates the other, and both must apply. Owner-wide revision
    /// binding false-conflicts them.
    #[test]
    fn falsification_independent_spends_must_not_false_conflict() {
        let mut owner = CharacterOwner::seed([
            (CharacterId(1), Stamina::new(90).unwrap()),
            (CharacterId(2), Stamina::new(50).unwrap()),
        ])
        .unwrap();
        let plan_a = owner.validate_spend(CharacterId(1), 10).unwrap();
        let plan_b = owner.validate_spend(CharacterId(2), 5).unwrap();
        owner.apply_spend(plan_a);
        owner.apply_spend(plan_b);
        assert_eq!(owner.stamina(CharacterId(1)).unwrap().points(), 80);
        assert_eq!(owner.stamina(CharacterId(2)).unwrap().points(), 45);
    }
}
