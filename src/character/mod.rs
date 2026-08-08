//! Character owner: the single writer of character bodies (stamina).
//!
//! Internals are private. The only mutation path is
//! `validate_spend` (fallible, read-only) followed by `apply_spend`
//! (infallible). `StaminaSpend` has private fields, so only this module can
//! mint the proof that a spend was validated.

use std::collections::BTreeMap;

use crate::boundary::{
    CharacterId, Fnv1a, RefusalReason, STAMINA_COST_BY_BAND, Stamina, StaminaBand,
};

pub struct CharacterOwner {
    stamina: BTreeMap<CharacterId, Stamina>,
}

/// Proof that a stamina spend was validated. Private fields: only the
/// character owner can construct one.
pub struct StaminaSpend {
    id: CharacterId,
    cost: u8,
    before: Stamina,
}

impl StaminaSpend {
    pub fn cost(&self) -> u8 {
        self.cost
    }

    pub fn band(&self) -> StaminaBand {
        self.before.band()
    }
}

impl CharacterOwner {
    pub fn seed(entries: impl IntoIterator<Item = (CharacterId, Stamina)>) -> Self {
        Self {
            stamina: entries.into_iter().collect(),
        }
    }

    pub fn stamina(&self, id: CharacterId) -> Option<Stamina> {
        self.stamina.get(&id).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (CharacterId, Stamina)> + '_ {
        self.stamina.iter().map(|(id, stamina)| (*id, *stamina))
    }

    /// Read-only validation: the actor must exist and must not be exhausted.
    pub fn validate_spend(&self, id: CharacterId) -> Result<StaminaSpend, RefusalReason> {
        let before = self.stamina(id).ok_or(RefusalReason::UnknownActor)?;
        let band = before.band();
        if band == StaminaBand::Exhausted {
            return Err(RefusalReason::ActorExhausted);
        }
        Ok(StaminaSpend {
            id,
            cost: STAMINA_COST_BY_BAND[band.index()],
            before,
        })
    }

    /// Infallible apply: the spend was validated against this owner, and
    /// stamina can never underflow.
    pub fn apply_spend(&mut self, spend: &StaminaSpend) {
        if let Some(stamina) = self.stamina.get_mut(&spend.id) {
            *stamina = stamina.saturating_spend(spend.cost);
        }
    }

    /// Deterministic: BTreeMap iterates in key order.
    pub fn hash_into(&self, hasher: &mut Fnv1a) {
        for (id, stamina) in &self.stamina {
            hasher.update(b"chr");
            hasher.update(&id.0.to_be_bytes());
            hasher.update(&[stamina.points()]);
        }
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
    }

    #[test]
    fn unknown_actor_is_refused() {
        assert_eq!(
            owner().validate_spend(CharacterId(99)).err(),
            Some(RefusalReason::UnknownActor)
        );
    }

    #[test]
    fn exhausted_actor_is_refused() {
        assert_eq!(
            owner().validate_spend(CharacterId(3)).err(),
            Some(RefusalReason::ActorExhausted)
        );
    }

    #[test]
    fn validated_spend_applies() {
        let mut owner = owner();
        let spend = owner.validate_spend(CharacterId(1)).unwrap();
        assert_eq!(spend.band(), StaminaBand::Fresh);
        owner.apply_spend(&spend);
        assert_eq!(owner.stamina(CharacterId(1)).unwrap().points(), 80);
    }
}
