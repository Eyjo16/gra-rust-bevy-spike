//! Bevy ECS host adapter (trial/002).
//!
//! Hosts the truth layer as an ECS resource and submits the same fixture
//! and command sequence through the same boundary, one command per
//! schedule tick. The host contributes scheduling only — no gameplay
//! semantics — and the parity gate demands that it reproduce the pure
//! run's canonical receipts and exact final canonical state. The world
//! hash remains a redundant checksum. The truth layer itself never
//! references Bevy; only this adapter does.

use bevy_ecs::prelude::{ResMut, Resource, Schedule};

use crate::boundary::{Command, Receipt, World, submit};

/// The whole truth layer as one ECS resource: the host schedules access
/// to it but never holds a second mutation path into it — every write
/// still goes through `submit`.
#[derive(Resource)]
struct Truth {
    world: World,
    /// Pending commands, reversed so `pop` yields the original order.
    queue: Vec<Command>,
    seq: u64,
    log: Vec<Receipt>,
}

/// One tick: submit the next queued command through the boundary.
fn submit_next(mut truth: ResMut<Truth>) {
    let truth = &mut *truth;
    if let Some(cmd) = truth.queue.pop() {
        truth.seq += 1;
        let receipt = submit(&mut truth.world, truth.seq, cmd);
        truth.log.push(receipt);
    }
}

/// Runs the whole trial inside a Bevy ECS world and returns the final
/// truth world and receipt log for parity comparison against the pure
/// host.
pub fn run_hosted(build_fixture: fn() -> World, commands: &[Command]) -> (World, Vec<Receipt>) {
    let mut ecs = bevy_ecs::world::World::new();
    let mut schedule = Schedule::default();
    schedule.add_systems(submit_next);
    ecs.insert_resource(Truth {
        world: build_fixture(),
        queue: commands.iter().rev().copied().collect(),
        seq: 0,
        log: Vec::with_capacity(commands.len()),
    });
    for _ in 0..commands.len() {
        schedule.run(&mut ecs);
    }
    let truth = ecs
        .remove_resource::<Truth>()
        .expect("truth resource survives the schedule");
    (truth.world, truth.log)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    use crate::boundary::{
        CharacterId, ClaimId, GatherCommand, OutcomeKind, RefusalReason, SiteId, Verb,
        WitnessCommand,
    };

    const TRACE_SEED: u64 = 0x0070_0700_6d61_7065;
    const TRACE_COUNT: usize = 1_000;
    const TRACE_DEPTH: usize = 32;
    const ACTORS: [u64; 5] = [1, 2, 3, 4, 9];
    const CLAIMS: [u64; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 99];
    const SITES: [u64; 5] = [1, 2, 3, 4, 9];
    const GATHER_COMMANDS: usize = ACTORS.len() * CLAIMS.len() * SITES.len();
    const WITNESS_COMMANDS: usize = ACTORS.len() * CLAIMS.len();
    const COMMAND_SPACE: usize = GATHER_COMMANDS + WITNESS_COMMANDS;

    struct Lcg {
        state: u64,
    }

    impl Lcg {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        fn next_index(&mut self, upper: usize) -> usize {
            self.state = self
                .state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((self.state >> 32) as usize) % upper
        }
    }

    struct Divergence {
        pure_receipts: Vec<String>,
        hosted_receipts: Vec<String>,
        pure_state: Vec<String>,
        hosted_state: Vec<String>,
        pure_hash: u64,
        hosted_hash: u64,
    }

    impl Divergence {
        fn evidence(&self) -> String {
            format!(
                "pure_receipts={:#?}\nhosted_receipts={:#?}\n\
                 pure_state={:#?}\nhosted_state={:#?}\n\
                 pure_hash=0x{:016x} hosted_hash=0x{:016x}",
                self.pure_receipts,
                self.hosted_receipts,
                self.pure_state,
                self.hosted_state,
                self.pure_hash,
                self.hosted_hash
            )
        }
    }

    #[derive(Default)]
    struct Coverage {
        command_indices: BTreeSet<usize>,
        verbs: BTreeMap<String, usize>,
        outcomes: BTreeMap<String, usize>,
        bands_observed: BTreeMap<String, usize>,
        tiers_observed: BTreeMap<String, usize>,
        gather_cost_cells: BTreeMap<String, usize>,
        gather_yield_cells: BTreeMap<String, usize>,
        witness_flat_cost_uses: usize,
    }

    impl Coverage {
        fn observe(&mut self, command_index: usize, receipt: &Receipt) {
            self.command_indices.insert(command_index);
            increment(&mut self.verbs, receipt.verb.code().to_owned());
            increment(
                &mut self.outcomes,
                format!(
                    "{}:{}",
                    receipt.outcome.code(),
                    receipt.outcome.reason_code()
                ),
            );
            if let Some(band) = receipt.band {
                increment(&mut self.bands_observed, band.code().to_owned());
            }
            if let Some(tier) = receipt.tier {
                increment(&mut self.tiers_observed, tier.code().to_owned());
            }

            match receipt.verb {
                Verb::Gather => {
                    let cost_was_consulted = matches!(
                        receipt.outcome,
                        OutcomeKind::Accepted
                            | OutcomeKind::Partial(_)
                            | OutcomeKind::Refused(
                                RefusalReason::InsufficientStamina
                                    | RefusalReason::UnknownSite
                                    | RefusalReason::SiteEmpty
                            )
                    );
                    if cost_was_consulted {
                        let band = receipt
                            .band
                            .expect("a consulted gather cost has a known actor and band");
                        increment(&mut self.gather_cost_cells, band.code().to_owned());
                    }

                    let yield_was_consulted = matches!(
                        receipt.outcome,
                        OutcomeKind::Accepted
                            | OutcomeKind::Partial(_)
                            | OutcomeKind::Refused(RefusalReason::SiteEmpty)
                    );
                    if yield_was_consulted {
                        let band = receipt
                            .band
                            .expect("a consulted gather yield has a known actor and band");
                        let tier = receipt
                            .tier
                            .expect("a consulted gather yield has a known site and tier");
                        increment(
                            &mut self.gather_yield_cells,
                            format!("{}/{}", band.code(), tier.code()),
                        );
                    }
                }
                Verb::Witness => {
                    if matches!(
                        receipt.outcome,
                        OutcomeKind::Accepted
                            | OutcomeKind::Refused(
                                RefusalReason::UnknownActor | RefusalReason::InsufficientStamina
                            )
                    ) {
                        self.witness_flat_cost_uses += 1;
                    }
                }
            }
        }
    }

    fn increment(counts: &mut BTreeMap<String, usize>, key: String) {
        *counts.entry(key).or_default() += 1;
    }

    fn command_at(index: usize) -> Command {
        if index < GATHER_COMMANDS {
            let site_index = index % SITES.len();
            let remaining = index / SITES.len();
            let claim_index = remaining % CLAIMS.len();
            let actor_index = remaining / CLAIMS.len();
            Command::Gather(GatherCommand {
                actor: CharacterId(ACTORS[actor_index]),
                claim: ClaimId(CLAIMS[claim_index]),
                site: SiteId(SITES[site_index]),
            })
        } else {
            let witness_index = index - GATHER_COMMANDS;
            let claim_index = witness_index % CLAIMS.len();
            let actor_index = witness_index / CLAIMS.len();
            Command::Witness(WitnessCommand {
                witness: CharacterId(ACTORS[actor_index]),
                claim: ClaimId(CLAIMS[claim_index]),
            })
        }
    }

    fn compare_trace(commands: &[Command]) -> Result<Vec<Receipt>, Box<Divergence>> {
        let mut pure_world = crate::fixture();
        let pure_log: Vec<Receipt> = commands
            .iter()
            .enumerate()
            .map(|(i, command)| submit(&mut pure_world, i as u64 + 1, *command))
            .collect();
        let (hosted_world, hosted_log) = run_hosted(crate::fixture, commands);

        let pure_receipts: Vec<String> = pure_log.iter().map(Receipt::canonical_line).collect();
        let hosted_receipts: Vec<String> = hosted_log.iter().map(Receipt::canonical_line).collect();
        let pure_state = pure_world.canonical_state();
        let hosted_state = hosted_world.canonical_state();
        let pure_hash = pure_world.hash();
        let hosted_hash = hosted_world.hash();

        if pure_receipts == hosted_receipts
            && pure_state == hosted_state
            && pure_hash == hosted_hash
        {
            Ok(pure_log)
        } else {
            Err(Box::new(Divergence {
                pure_receipts,
                hosted_receipts,
                pure_state,
                hosted_state,
                pure_hash,
                hosted_hash,
            }))
        }
    }

    /// Greedy command removal produces a one-minimal counterexample: after
    /// this returns, removing any one remaining command makes parity hold.
    fn shrink_divergence(mut commands: Vec<Command>) -> Vec<Command> {
        let mut index = 0;
        while index < commands.len() {
            let mut candidate = commands.clone();
            candidate.remove(index);
            if compare_trace(&candidate).is_err() {
                commands = candidate;
                index = 0;
            } else {
                index += 1;
            }
        }
        commands
    }

    /// The parity falsifier for trial/002: the hosted run must reproduce
    /// the pure run's receipts and exact final state, on the identical
    /// fixture and command sequence — otherwise the host has acquired
    /// semantics of its own. The hash remains a redundant checksum.
    #[test]
    fn hosted_run_reproduces_pure_receipts_and_hash() {
        let cmds = crate::commands();
        let mut pure_world = crate::fixture();
        let pure_log: Vec<Receipt> = cmds
            .iter()
            .enumerate()
            .map(|(i, cmd)| submit(&mut pure_world, i as u64 + 1, *cmd))
            .collect();

        let (host_world, host_log) = run_hosted(crate::fixture, &cmds);

        assert_eq!(
            host_world.canonical_state(),
            pure_world.canonical_state(),
            "exact canonical final state"
        );
        assert_eq!(host_world.hash(), pure_world.hash(), "final world hash");
        let pure_lines: Vec<String> = pure_log.iter().map(Receipt::canonical_line).collect();
        let host_lines: Vec<String> = host_log.iter().map(Receipt::canonical_line).collect();
        assert_eq!(host_lines, pure_lines, "canonical receipt lines");
    }

    /// Falsifier (trial/007): parity on one recorded history cannot prove
    /// parity over transition inputs that history never visited.
    #[test]
    fn falsification_bounded_transition_domain_matches_exactly() {
        let mut generator = Lcg::new(TRACE_SEED);
        let mut coverage = Coverage::default();

        for trace_index in 0..TRACE_COUNT {
            let indexed_commands: Vec<(usize, Command)> = (0..TRACE_DEPTH)
                .map(|_| {
                    let command_index = generator.next_index(COMMAND_SPACE);
                    (command_index, command_at(command_index))
                })
                .collect();
            let commands: Vec<Command> = indexed_commands
                .iter()
                .map(|(_, command)| *command)
                .collect();

            match compare_trace(&commands) {
                Ok(pure_log) => {
                    for ((command_index, _), receipt) in indexed_commands.iter().zip(&pure_log) {
                        coverage.observe(*command_index, receipt);
                    }
                }
                Err(original) => {
                    let minimal = shrink_divergence(commands);
                    let minimal_evidence = compare_trace(&minimal)
                        .expect_err("a shrunk counterexample must still diverge");
                    panic!(
                        "transition-domain parity divergence\nseed=0x{TRACE_SEED:016x} \
                         traces={TRACE_COUNT} depth={TRACE_DEPTH} trace_index={trace_index}\n\
                         original={}\nminimal_commands={minimal:#?}\nminimal_evidence={}",
                        original.evidence(),
                        minimal_evidence.evidence()
                    );
                }
            }
        }

        assert_eq!(
            coverage.command_indices.len(),
            COMMAND_SPACE,
            "the seeded run must visit every enumerated command input"
        );
        println!(
            "transition_domain_parity seed=0x{TRACE_SEED:016x} traces={TRACE_COUNT} \
             depth={TRACE_DEPTH} commands={} command_space={} unique_commands={} \
             receipts_match=true state_match=true world_match=true",
            TRACE_COUNT * TRACE_DEPTH,
            COMMAND_SPACE,
            coverage.command_indices.len()
        );
        println!("transition_domain_verbs {:?}", coverage.verbs);
        println!("transition_domain_outcomes {:?}", coverage.outcomes);
        println!("transition_domain_bands {:?}", coverage.bands_observed);
        println!("transition_domain_tiers {:?}", coverage.tiers_observed);
        println!(
            "transition_domain_cost_cells gather={:?} witness_flat_uses={}",
            coverage.gather_cost_cells, coverage.witness_flat_cost_uses
        );
        println!(
            "transition_domain_yield_cells {:?}",
            coverage.gather_yield_cells
        );
    }
}
