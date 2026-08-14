//! Trial/014 anticipation-drive experiment.
//!
//! This module is test-gated on purpose: the trial may rank already-legal
//! intents before an experimental plan seal, but has no authority to change
//! canonical commands, receipts, values, or the ten-oracle suite.

use std::{cmp::Ordering, collections::BTreeMap};

use crate::boundary::{CharacterId, Fnv1a, OutcomeKind, Receipt, Verb};

/// Trial-local bound on the score contribution of anticipation drive.
const DRIVE_MODIFIER_CAP: i8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Drive {
    Bad,
    Average,
    Good,
    Superb,
}

impl Drive {
    fn code(self) -> &'static str {
        match self {
            Self::Bad => "bad",
            Self::Average => "average",
            Self::Good => "good",
            Self::Superb => "superb",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BeliefRecord {
    agent: CharacterId,
    drive: Drive,
}

impl BeliefRecord {
    fn new(agent: CharacterId, drive: Drive) -> Self {
        Self { agent, drive }
    }
}

/// Trial-only single-writer store. Intent evaluation receives shared access,
/// so it cannot mutate belief state while ranking or sealing a plan.
struct AnticipationBeliefs {
    records: BTreeMap<CharacterId, BeliefRecord>,
}

impl AnticipationBeliefs {
    fn seed(records: impl IntoIterator<Item = BeliefRecord>) -> Option<Self> {
        let mut by_agent = BTreeMap::new();
        for record in records {
            if by_agent.insert(record.agent, record).is_some() {
                return None;
            }
        }
        Some(Self { records: by_agent })
    }

    fn record(&self, agent: CharacterId) -> Option<BeliefRecord> {
        self.records.get(&agent).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntentKind {
    Cheap,
    Costly,
}

impl IntentKind {
    fn index(self) -> usize {
        match self {
            Self::Cheap => 0,
            Self::Costly => 1,
        }
    }

    fn code(self) -> &'static str {
        match self {
            Self::Cheap => "cheap",
            Self::Costly => "costly",
        }
    }
}

/// An immutable commitment copied from an already accepted canonical gather
/// receipt. Evaluation may rank these bytes but cannot rewrite them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ValidatedIntent {
    committed_cost: u8,
    committed_yield: u64,
}

impl ValidatedIntent {
    fn from_receipt(receipt: &Receipt) -> Option<Self> {
        (receipt.verb == Verb::Gather
            && matches!(
                receipt.outcome,
                OutcomeKind::Accepted | OutcomeKind::Partial(_)
            ))
        .then_some(Self {
            committed_cost: receipt.stamina_spent,
            committed_yield: receipt.mass_moved.grams(),
        })
    }

    fn committed_bytes(self) -> [u8; 9] {
        let mut bytes = [0; 9];
        bytes[0] = self.committed_cost;
        bytes[1..].copy_from_slice(&self.committed_yield.to_be_bytes());
        bytes
    }
}

/// Derive semantic roles from the commitments rather than caller labels.
/// The trial shape requires one cheaper/lower-yield intent and one
/// costlier/higher-yield intent; ties and crossed trade-offs fail closed.
fn semantic_intents(intents: &[ValidatedIntent; 2]) -> Option<[ValidatedIntent; 2]> {
    let [left, right] = *intents;
    match (
        left.committed_cost.cmp(&right.committed_cost),
        left.committed_yield.cmp(&right.committed_yield),
    ) {
        (Ordering::Less, Ordering::Less) => Some([left, right]),
        (Ordering::Greater, Ordering::Greater) => Some([right, left]),
        _ => None,
    }
}

fn legal_intent_set_hash(intents: &[ValidatedIntent; 2]) -> u64 {
    let mut commitments = intents.map(ValidatedIntent::committed_bytes);
    commitments.sort_unstable();
    let mut hasher = Fnv1a::default();
    for commitment in commitments {
        hasher.update(&commitment);
    }
    hasher.finish()
}

fn bounded_modifier(raw: i8) -> Option<i8> {
    (-DRIVE_MODIFIER_CAP..=DRIVE_MODIFIER_CAP)
        .contains(&raw)
        .then_some(raw)
}

/// Pure score contribution from one belief record. The cheap intent remains
/// the zero reference; drive can rank but never alter a committed intent.
/// A table entry outside the declared cap rejects evaluation rather than
/// being silently clamped into a different ranking.
fn drive_modifier(record: BeliefRecord, kind: IntentKind) -> Option<i8> {
    let raw = match kind {
        IntentKind::Cheap => 0,
        IntentKind::Costly => match record.drive {
            Drive::Bad => -1,
            Drive::Average => 0,
            Drive::Good | Drive::Superb => 1,
        },
    };
    bounded_modifier(raw)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvaluationReceipt {
    agent: CharacterId,
    drive: Drive,
    legal_intent_set_hash: u64,
    modifiers: [i8; 2],
    selected: IntentKind,
}

impl EvaluationReceipt {
    fn canonical_line(&self) -> String {
        format!(
            "agent=C{} drive={} legal_intents=0x{:016x} cheap_modifier={} \
             costly_modifier={} selected={} cap={}",
            self.agent.0,
            self.drive.code(),
            self.legal_intent_set_hash,
            self.modifiers[IntentKind::Cheap.index()],
            self.modifiers[IntentKind::Costly.index()],
            self.selected.code(),
            DRIVE_MODIFIER_CAP,
        )
    }

    fn hash(&self) -> u64 {
        let mut hasher = Fnv1a::default();
        hasher.update(self.canonical_line().as_bytes());
        hasher.finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SealedPlan {
    committed_intent: [u8; 9],
    evaluation_receipt_hash: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvaluatedPlan {
    receipt: EvaluationReceipt,
    plan: SealedPlan,
}

/// Rank immutable legal intents, emit the evaluation receipt, then seal the
/// selected commitment against that receipt. Shared inputs make the operation
/// a pure function of belief records plus the already-frozen legal set.
fn evaluate_and_seal(
    beliefs: &AnticipationBeliefs,
    agent: CharacterId,
    intents: &[ValidatedIntent; 2],
) -> Option<EvaluatedPlan> {
    let belief = beliefs.record(agent)?;
    let [cheap, costly] = semantic_intents(intents)?;
    let modifiers = [
        drive_modifier(belief, IntentKind::Cheap)?,
        drive_modifier(belief, IntentKind::Costly)?,
    ];

    // A tie resolves to the semantic cheap intent, independent of input order.
    let selected_kind =
        if modifiers[IntentKind::Costly.index()] > modifiers[IntentKind::Cheap.index()] {
            IntentKind::Costly
        } else {
            IntentKind::Cheap
        };
    let selected = match selected_kind {
        IntentKind::Cheap => cheap,
        IntentKind::Costly => costly,
    };
    let receipt = EvaluationReceipt {
        agent,
        drive: belief.drive,
        legal_intent_set_hash: legal_intent_set_hash(intents),
        modifiers,
        selected: selected_kind,
    };

    // The receipt must exist before the plan can bind to its hash.
    let plan = SealedPlan {
        committed_intent: selected.committed_bytes(),
        evaluation_receipt_hash: receipt.hash(),
    };
    Some(EvaluatedPlan { receipt, plan })
}

#[cfg(test)]
mod tests {
    use super::{
        AnticipationBeliefs, BeliefRecord, DRIVE_MODIFIER_CAP, Drive, IntentKind, ValidatedIntent,
        bounded_modifier, evaluate_and_seal, legal_intent_set_hash,
    };
    use crate::boundary::{
        CharacterId, ClaimId, Command, GatherCommand, InfraTier, MassGrams, OutcomeKind, Receipt,
        RefusalReason, SiteId, Stamina, World, submit, validate_world_coherence,
    };
    use crate::character::CharacterOwner;
    use crate::economy::EconomyOwner;
    use crate::social::SocialOwner;

    fn gather_receipt(start: u8, tier: InfraTier) -> Receipt {
        let mut world = World {
            characters: CharacterOwner::seed([(
                CharacterId(1),
                Stamina::new(start).expect("trial start is bounded"),
            )])
            .expect("one unique character"),
            economy: EconomyOwner::seed_sites([(SiteId(1), tier, MassGrams::new(10_000))])
                .expect("one unique site"),
            social: SocialOwner::seed_claims([(ClaimId(1), CharacterId(1), SiteId(1), true)])
                .expect("one coherent witnessed claim"),
        };
        validate_world_coherence(&world).expect("trial fixture is coherent");
        submit(
            &mut world,
            1,
            Command::Gather(GatherCommand {
                actor: CharacterId(1),
                claim: ClaimId(1),
                site: SiteId(1),
            }),
        )
    }

    fn low_dead_zone_receipts() -> Vec<String> {
        (10..=14)
            .map(|start| {
                let receipt = gather_receipt(start, InfraTier::Established);
                assert_eq!(
                    receipt.outcome,
                    OutcomeKind::Refused(RefusalReason::InsufficientStamina)
                );
                assert_eq!(receipt.stamina_spent, 0);
                assert_eq!(receipt.mass_moved, MassGrams::ZERO);
                assert_eq!(receipt.world_hash_before, receipt.world_hash_after);
                receipt.canonical_line()
            })
            .collect()
    }

    #[test]
    fn trial_014_drive_only_selection_is_capped_and_membership_preserving() {
        let cheap_receipt = gather_receipt(100, InfraTier::None);
        let costly_receipt = gather_receipt(79, InfraTier::Advanced);
        assert_eq!(cheap_receipt.outcome, OutcomeKind::Accepted);
        assert_eq!(costly_receipt.outcome, OutcomeKind::Accepted);

        let intents = [
            ValidatedIntent::from_receipt(&cheap_receipt).expect("accepted cheap intent is legal"),
            ValidatedIntent::from_receipt(&costly_receipt)
                .expect("accepted costly intent is legal"),
        ];
        assert!(intents[0].committed_cost < intents[1].committed_cost);
        assert!(intents[0].committed_yield < intents[1].committed_yield);

        let legal_bytes_before = intents.map(ValidatedIntent::committed_bytes);
        let legal_hash_before = legal_intent_set_hash(&intents);
        let refusals_before = low_dead_zone_receipts();

        let beliefs = AnticipationBeliefs::seed([
            BeliefRecord::new(CharacterId(101), Drive::Bad),
            BeliefRecord::new(CharacterId(102), Drive::Average),
            BeliefRecord::new(CharacterId(103), Drive::Good),
            BeliefRecord::new(CharacterId(104), Drive::Superb),
        ])
        .expect("one belief record per agent");

        let cases = [
            (CharacterId(101), Drive::Bad, IntentKind::Cheap),
            (CharacterId(102), Drive::Average, IntentKind::Cheap),
            (CharacterId(103), Drive::Good, IntentKind::Costly),
            (CharacterId(104), Drive::Superb, IntentKind::Costly),
        ];

        assert_eq!(DRIVE_MODIFIER_CAP, 1);
        assert_eq!(bounded_modifier(-2), None);
        assert_eq!(bounded_modifier(-1), Some(-1));
        assert_eq!(bounded_modifier(1), Some(1));
        assert_eq!(bounded_modifier(2), None);
        let reversed_intents = [intents[1], intents[0]];
        for (agent, drive, expected) in cases {
            let evaluated = evaluate_and_seal(&beliefs, agent, &intents)
                .expect("agent has an anticipation belief");
            let replay =
                evaluate_and_seal(&beliefs, agent, &intents).expect("pure evaluation replays");
            let reversed = evaluate_and_seal(&beliefs, agent, &reversed_intents)
                .expect("input order does not change semantic validity");

            assert_eq!(evaluated, replay, "belief-only evaluation must be pure");
            assert_eq!(
                reversed, evaluated,
                "semantic selection and set identity must ignore input order"
            );
            assert_eq!(evaluated.receipt.drive, drive);
            assert_eq!(evaluated.receipt.selected, expected);
            assert_eq!(evaluated.receipt.legal_intent_set_hash, legal_hash_before);
            assert!(
                evaluated
                    .receipt
                    .modifiers
                    .iter()
                    .all(|modifier| modifier.abs() <= DRIVE_MODIFIER_CAP)
            );
            assert_eq!(
                evaluated.plan.committed_intent,
                intents[expected.index()].committed_bytes()
            );
            assert_eq!(
                evaluated.plan.evaluation_receipt_hash,
                evaluated.receipt.hash()
            );
            println!("{}", evaluated.receipt.canonical_line());
        }

        assert_eq!(
            legal_intent_set_hash(&reversed_intents),
            legal_hash_before,
            "set identity must ignore input order",
        );

        let legal_bytes_after = intents.map(ValidatedIntent::committed_bytes);
        let refusals_after = low_dead_zone_receipts();
        assert_eq!(legal_bytes_after, legal_bytes_before);
        assert_eq!(legal_intent_set_hash(&intents), legal_hash_before);
        assert_eq!(refusals_after, refusals_before);
        println!(
            "trial014 summary cap={} legal_hash=0x{:016x} membership_unchanged=true \
             costs_yields_unchanged=true low_10_14_refusals_unchanged=true",
            DRIVE_MODIFIER_CAP, legal_hash_before
        );
    }
}
