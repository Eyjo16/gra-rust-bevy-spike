//! W01 — the winter-crisis scene.
//!
//! One household, one winter, three defensible plans, run through the
//! same boundary and the same ten oracles as every other trial. The
//! scene adds no rule: no verb, no kind, no value, no oracle. It is a
//! fixture and three command sequences.
//!
//! Language discipline (review finding 5): the plans **stockpile**. They
//! do not feed cattle, close roofs or feed people, because gathering and
//! transferring stock is all the truth layer does. Every consequence word
//! was removed from this file rather than left to be read as a promise.
//!
//! Authority note, hard: `WINTER_NEED` and the shortfall table are
//! **scene arithmetic**, not truth. The truth layer has no concept of a
//! need, a cow, a mouth or a roof — nothing consumes anything. The
//! shortfall is printed as a projection over canonical state, is read
//! back by nothing, and no oracle depends on it. When consumption
//! becomes a rule it arrives through a licensed trial, not through this
//! file.

use crate::boundary::{
    CharacterId, ClaimId, Command, GatherCommand, GiveCommand, InfraTier, KIND_COUNT, MassGrams,
    Receipt, ResourceKind, SiteId, Stamina, WitnessCommand, World, command_encoding_fingerprint,
    fixture_identity, grammar_fingerprint, receipt_chain_digest, receipt_format_fingerprint,
    submit, validate_world_coherence,
};
use crate::character::CharacterOwner;
use crate::economy::EconomyOwner;
use crate::oracles::{self, OracleCtx, baseline_by_kind};
use crate::social::SocialOwner;

/// What the household must hold to reach spring. Scene arithmetic —
/// mechanical example numbers, not balance, and not a rule the world
/// enforces. Indexed by `ResourceKind::index`.
pub const WINTER_NEED: [u64; KIND_COUNT] = [6_000, 2_500, 1_200];

/// Vígslóði in the ninth week of winter: a hayfield already cut over, a
/// stand of scrub wood, a shore, and four people of unequal strength.
/// Hallr's claim on the hayfield is unwitnessed, so legitimacy costs
/// someone stamina before he may work at all.
pub fn fixture() -> World {
    World {
        characters: CharacterOwner::seed([
            (CharacterId(1), Stamina::new(70).expect("in range")),
            (CharacterId(2), Stamina::new(60).expect("in range")),
            (CharacterId(3), Stamina::new(45).expect("in range")),
            (CharacterId(4), Stamina::new(25).expect("in range")),
        ])
        .expect("no duplicate characters"),
        economy: EconomyOwner::seed_sites([
            (
                SiteId(1),
                InfraTier::Established,
                ResourceKind::Fodder,
                MassGrams::new(4_000),
            ),
            (
                SiteId(2),
                InfraTier::Crude,
                ResourceKind::Timber,
                MassGrams::new(2_500),
            ),
            (
                SiteId(3),
                InfraTier::Crude,
                ResourceKind::Food,
                MassGrams::new(1_800),
            ),
        ])
        .expect("no duplicate sites"),
        social: SocialOwner::seed_claims([
            (ClaimId(1), CharacterId(1), SiteId(1), true),
            (ClaimId(2), CharacterId(2), SiteId(1), true),
            (ClaimId(3), CharacterId(3), SiteId(1), true),
            (ClaimId(4), CharacterId(4), SiteId(1), false),
            (ClaimId(5), CharacterId(1), SiteId(2), true),
            (ClaimId(6), CharacterId(2), SiteId(2), true),
            (ClaimId(7), CharacterId(3), SiteId(3), true),
            (ClaimId(8), CharacterId(4), SiteId(3), true),
        ])
        .expect("no duplicate claims"),
    }
}

fn gather(actor: u64, claim: u64, site: u64) -> Command {
    Command::Gather(GatherCommand {
        actor: CharacterId(actor),
        claim: ClaimId(claim),
        site: SiteId(site),
    })
}

fn witness(witness: u64, claim: u64) -> Command {
    Command::Witness(WitnessCommand {
        witness: CharacterId(witness),
        claim: ClaimId(claim),
    })
}

fn give(giver: u64, to: u64, kind: ResourceKind, grams: u64, attested: Option<u64>) -> Command {
    Command::Give(GiveCommand {
        giver: CharacterId(giver),
        recipient: CharacterId(to),
        kind,
        grams: MassGrams::new(grams),
        witness: attested.map(CharacterId),
    })
}

pub struct Plan {
    pub id: &'static str,
    pub name: &'static str,
    pub intent: &'static str,
    pub commands: Vec<Command>,
}

/// The three plans. Each is a defensible answer to the same winter, and
/// none of them is right — the world holds less fodder and less food
/// than the household needs, so every plan is a triage.
pub fn plans() -> Vec<Plan> {
    vec![
        Plan {
            id: "A",
            name: "stockpile fodder",
            intent: "every hand to the hayfield, including the boy's — \
                     which first costs the head the stamina to attest his claim. \
                     Whether the cattle live is not a fact this layer holds",
            commands: vec![
                gather(1, 1, 1),
                gather(2, 2, 1),
                gather(3, 3, 1),
                gather(4, 4, 1), // refused: the boy's claim is unwitnessed
                witness(1, 4),   // the head pays for his legitimacy
                gather(4, 4, 1), // partial: the field is nearly bare
                gather(1, 1, 1), // refused: site empty
                gather(2, 2, 1), // refused: site empty
                give(4, 1, ResourceKind::Fodder, 400, Some(2)),
            ],
        },
        Plan {
            id: "B",
            name: "stockpile building material",
            intent: "two to the wood and two to the hay; the household ends \
                     holding more timber than the stated roof figure, and less \
                     fodder. Nothing is repaired: no structure exists",
            commands: vec![
                gather(1, 5, 2),
                gather(2, 6, 2),
                gather(3, 3, 1),
                witness(3, 4), // the tiring woman pays this time
                gather(4, 4, 1),
                gather(1, 5, 2),
                gather(3, 3, 1),
                give(1, 2, ResourceKind::Timber, 1_600, Some(3)),
            ],
        },
        Plan {
            id: "C",
            name: "stockpile food",
            intent: "the shore and the hayfield; the household ends closest to \
                     the stated food figure. Nobody eats: no consumption exists",
            commands: vec![
                gather(3, 7, 3),
                gather(4, 8, 3),
                gather(1, 1, 1),
                gather(2, 2, 1),
                gather(3, 7, 3),
                gather(4, 8, 3), // refused: 10 points cannot cover a 15-point spend
                give(4, 1, ResourceKind::Food, 400, Some(3)),
            ],
        },
    ]
}

/// Everything the household holds, by kind — the sum over its people.
/// A projection: it reads canonical state and returns numbers, and
/// nothing reads it back.
pub fn household_totals(world: &World) -> [u64; KIND_COUNT] {
    let mut totals = [0u64; KIND_COUNT];
    for (_, kind, grams) in world.economy.holdings_iter() {
        totals[kind.index()] += grams.grams();
    }
    totals
}

/// Need minus held, floored at zero. Scene arithmetic, never a rule.
pub fn shortfall(world: &World) -> [u64; KIND_COUNT] {
    let held = household_totals(world);
    let mut short = [0u64; KIND_COUNT];
    for kind in ResourceKind::ALL {
        short[kind.index()] = WINTER_NEED[kind.index()].saturating_sub(held[kind.index()]);
    }
    short
}

pub struct PlanRun {
    pub world: World,
    pub log: Vec<Receipt>,
    pub commands: Vec<Command>,
}

pub fn run_plan(plan: &Plan) -> PlanRun {
    let mut world = fixture();
    validate_world_coherence(&world).expect("the winter fixture is coherent");
    let log = plan
        .commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| submit(&mut world, i as u64 + 1, *cmd))
        .collect();
    PlanRun {
        world,
        log,
        commands: plan.commands.clone(),
    }
}

/// Exact pure/host parity for one plan: the Bevy-hosted replay must
/// reproduce the pure run's receipt lines, its receipt-chain digest, its
/// exact canonical state and its world hash. Every winter plan carries
/// an attested transfer, so this also covers receipts whose only unusual
/// field is the attester's identity (review finding 6).
#[cfg(feature = "bevy-host")]
pub fn host_parity(plan: &Plan, run: &PlanRun) -> bool {
    let mut host = crate::host_bevy::Host::new(fixture);
    host.run_trial(&run.commands);
    let pure_lines: Vec<String> = run.log.iter().map(Receipt::canonical_line).collect();
    let receipts_match = host.receipts() == pure_lines
        && receipt_chain_digest(host.receipt_log()) == receipt_chain_digest(&run.log);
    let state_match = host.truth_state() == run.world.canonical_state();
    let world_match = host.truth_hash() == run.world.hash();
    let attested = run
        .log
        .iter()
        .filter(|receipt| receipt.transfer_witness.is_some())
        .count();
    println!(
        "winter_host_parity plan={} receipts_match={} state_match={} world_match={} \
         attested_transfers={} receipts=0x{:016x} world=0x{:016x}",
        plan.id,
        receipts_match,
        state_match,
        world_match,
        attested,
        receipt_chain_digest(host.receipt_log()),
        host.truth_hash(),
    );
    receipts_match && state_match && world_match
}

/// Runs all three plans, prints receipts, state, oracle verdicts,
/// shortfall and one envelope per plan. Returns false if any oracle
/// fails, so `cargo run winter` is a gate like every other run.
pub fn run() -> bool {
    let fixture_hash = fixture().hash();
    println!("winter_scene grammar=0x{:016x}", grammar_fingerprint());
    println!(
        "winter_need fodder={}g food={}g timber={}g (scene arithmetic, not a rule)",
        WINTER_NEED[ResourceKind::Fodder.index()],
        WINTER_NEED[ResourceKind::Food.index()],
        WINTER_NEED[ResourceKind::Timber.index()],
    );
    let mut all_pass = true;
    for plan in plans() {
        println!("\nplan {} {} — {}", plan.id, plan.name, plan.intent);
        let run = run_plan(&plan);
        for receipt in &run.log {
            println!("{}", receipt.canonical_line());
        }
        for line in run.world.canonical_state() {
            println!("{line}");
        }
        let held = household_totals(&run.world);
        let short = shortfall(&run.world);
        for kind in ResourceKind::ALL {
            println!(
                "winter_shortfall plan={} kind={} held={}g need={}g short={}g",
                plan.id,
                kind.code(),
                held[kind.index()],
                WINTER_NEED[kind.index()],
                short[kind.index()],
            );
        }
        let ctx = OracleCtx {
            world: &run.world,
            baseline_by_kind: baseline_by_kind(&fixture()),
            build_fixture: fixture,
            commands: &run.commands,
            log: &run.log,
        };
        for verdict in &oracles::run_all(&ctx) {
            let status = if verdict.pass { "PASS" } else { "FAIL" };
            println!(
                "oracle {status} plan={} {} ({})",
                plan.id, verdict.name, verdict.detail
            );
            all_pass &= verdict.pass;
        }
        #[cfg(feature = "bevy-host")]
        {
            all_pass &= host_parity(&plan, &run);
        }
        println!(
            "envelope scene=W01 plan={} baseline_commit={} grammar=0x{:016x} cmdfmt=0x{:016x} \
             rcptfmt=0x{:016x} fixture=0x{:016x} receipts=0x{:016x} world=0x{:016x} \
             oracles={}v{}",
            plan.id,
            std::env::var("BASELINE_COMMIT").unwrap_or_else(|_| "-".to_owned()),
            grammar_fingerprint(),
            command_encoding_fingerprint(),
            receipt_format_fingerprint(),
            fixture_identity(fixture_hash, &run.commands),
            receipt_chain_digest(&run.log),
            run.world.hash(),
            oracles::ORACLE_COUNT,
            oracles::ORACLE_SUITE_VERSION,
        );
    }
    all_pass
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::{OutcomeKind, PartialReason, RefusalReason};

    /// The pre-registered end state of every plan
    /// (`docs/trial-w01-winter-crisis-report.md` §4), committed at
    /// `fa28712` before this module existed. These are predictions, not
    /// balance: if a licensed value ever moves, this table moves with it
    /// and the trial is re-run, never quietly re-fitted.
    struct Predicted {
        stamina: [u8; 4],
        totals: [u64; KIND_COUNT],
        shortfall: [u64; KIND_COUNT],
        sites: [u64; 3],
    }

    const PREDICTED: [Predicted; 3] = [
        Predicted {
            stamina: [53, 48, 33, 7],
            totals: [4_000, 0, 0],
            shortfall: [2_000, 2_500, 1_200],
            sites: [0, 2_500, 1_800],
        },
        Predicted {
            stamina: [43, 48, 13, 10],
            totals: [2_400, 0, 2_400],
            shortfall: [3_600, 2_500, 0],
            sites: [1_600, 100, 1_800],
        },
        Predicted {
            stamina: [58, 48, 18, 7],
            totals: [2_400, 1_600, 0],
            shortfall: [3_600, 900, 1_200],
            sites: [1_600, 2_500, 200],
        },
    ];

    #[test]
    fn every_plan_matches_its_pre_registered_end_state() {
        for (plan, predicted) in plans().iter().zip(PREDICTED.iter()) {
            let run = run_plan(plan);
            for (index, expected) in predicted.stamina.iter().enumerate() {
                let id = CharacterId(index as u64 + 1);
                assert_eq!(
                    run.world.characters.stamina(id).unwrap().points(),
                    *expected,
                    "plan {} stamina for C{}",
                    plan.id,
                    id.0
                );
            }
            assert_eq!(
                household_totals(&run.world),
                predicted.totals,
                "plan {} household totals",
                plan.id
            );
            assert_eq!(
                shortfall(&run.world),
                predicted.shortfall,
                "plan {} shortfall",
                plan.id
            );
            let stocks: Vec<u64> = run
                .world
                .economy
                .sites_iter()
                .map(|(_, _, _, stock)| stock.grams())
                .collect();
            assert_eq!(stocks, predicted.sites, "plan {} site stocks", plan.id);
        }
    }

    #[test]
    fn every_plan_passes_all_ten_oracles() {
        for plan in plans() {
            let run = run_plan(&plan);
            let ctx = OracleCtx {
                world: &run.world,
                baseline_by_kind: baseline_by_kind(&fixture()),
                build_fixture: fixture,
                commands: &run.commands,
                log: &run.log,
            };
            for verdict in &oracles::run_all(&ctx) {
                assert!(
                    verdict.pass,
                    "plan {} failed {}: {}",
                    plan.id, verdict.name, verdict.detail
                );
            }
        }
    }

    /// The scene's dramatic beats are the ones a player would feel, so
    /// they are asserted rather than left to the eye: legitimacy costs
    /// stamina before the boy may work, the hayfield runs out under the
    /// household's hands, and the tired boy is refused.
    #[test]
    fn the_scene_beats_are_the_ones_the_scene_claims() {
        let a = run_plan(&plans()[0]);
        assert_eq!(
            a.log[3].outcome,
            OutcomeKind::Refused(RefusalReason::ClaimNotWitnessed),
            "the unwitnessed claim must stop the boy"
        );
        assert_eq!(a.log[4].stamina_spent, 5, "attesting costs the head");
        assert_eq!(
            a.log[5].outcome,
            OutcomeKind::Partial(PartialReason::SiteNearlyDepleted),
            "the field must run short mid-work"
        );
        assert_eq!(
            a.log[6].outcome,
            OutcomeKind::Refused(RefusalReason::SiteEmpty),
            "and then run out entirely"
        );

        let c = run_plan(&plans()[2]);
        assert_eq!(
            c.log[5].outcome,
            OutcomeKind::Refused(RefusalReason::InsufficientStamina),
            "the boy must be too tired for one more haul"
        );
    }

    /// No *registered* plan meets every need — the scene is a triage,
    /// and each plan pays for what it saves. Scope, stated because the
    /// original wording did not (review finding 6): "best" here means
    /// best among these three sequences, not best over the reachable
    /// strategy space, which is unbounded and unsearched.
    #[test]
    fn no_registered_plan_can_meet_every_need() {
        let mut best_by_kind = [u64::MAX; KIND_COUNT];
        for plan in plans() {
            let run = run_plan(&plan);
            let short = shortfall(&run.world);
            assert!(
                short.iter().any(|missing| *missing > 0),
                "plan {} met every winter need — the scene is no longer a crisis",
                plan.id
            );
            for kind in ResourceKind::ALL {
                best_by_kind[kind.index()] = best_by_kind[kind.index()].min(short[kind.index()]);
            }
        }
        // Each kind is best served by a different one of the three: the
        // triage is real, not a dominant strategy wearing three names.
        // This is the best among the REGISTERED plans only.
        assert_eq!(
            best_by_kind,
            [2_000, 900, 0],
            "the best shortfall per kind among the three registered plans changed"
        );
    }

    /// Falsifier (review finding 6): every winter plan must replay
    /// byte-for-byte inside the Bevy host — receipts, chain digest,
    /// exact canonical state and world hash. Each plan carries at least
    /// one attested transfer, so the attester's identity travels through
    /// the host too.
    #[cfg(feature = "bevy-host")]
    #[test]
    fn every_plan_replays_identically_inside_the_host() {
        for plan in plans() {
            let run = run_plan(&plan);
            assert!(
                run.log
                    .iter()
                    .any(|receipt| receipt.transfer_witness.is_some()),
                "plan {} has no attested transfer, so this parity check is weaker than claimed",
                plan.id
            );
            assert!(
                host_parity(&plan, &run),
                "plan {} diverged between the pure and hosted runs",
                plan.id
            );
        }
    }

    /// The scene changes no law. The original form of this test compared
    /// `grammar_fingerprint()` to itself and proved nothing — the review
    /// was right to call it tautological. What proves the claim is the
    /// STANDARD trial's identities being unmoved by this branch, so they
    /// are pinned here: if the winter scene ever reaches into the
    /// grammar, a value, a verb or the standard fixture, one of these
    /// six numbers moves and this test fails.
    #[test]
    fn the_scene_moves_no_identity_of_the_standard_trial() {
        let mut world = crate::fixture();
        let fixture_hash = world.hash();
        let cmds = crate::commands();
        let log: Vec<Receipt> = cmds
            .iter()
            .enumerate()
            .map(|(i, cmd)| submit(&mut world, i as u64 + 1, *cmd))
            .collect();

        assert_eq!(
            grammar_fingerprint(),
            0x7dd8_c670_6e0b_949f,
            "the winter scene moved the grammar"
        );
        assert_eq!(
            command_encoding_fingerprint(),
            0xfa37_eefa_3594_cfe3,
            "the winter scene moved the canonical command encoding"
        );
        assert_eq!(
            receipt_format_fingerprint(),
            0x7e62_1526_22bb_9132,
            "the winter scene moved the canonical receipt format"
        );
        assert_eq!(
            fixture_identity(fixture_hash, &cmds),
            0x93af_ba3f_312b_d89d,
            "the winter scene moved the standard fixture identity"
        );
        assert_eq!(
            receipt_chain_digest(&log),
            0xc0b4_da51_744b_cf19,
            "the winter scene changed what the standard trial receipts"
        );
        assert_eq!(
            world.hash(),
            0xb500_dee0_e5d8_83d8,
            "the winter scene changed where the standard trial ends"
        );

        validate_world_coherence(&fixture()).expect("winter fixture is coherent");
        // Total mass in the scene is the sum of its three sites; nothing
        // is created by seeding people who hold nothing.
        assert_eq!(fixture().economy.total_mass(), MassGrams::new(8_300));
    }
}
