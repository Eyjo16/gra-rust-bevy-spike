//! Bevy ECS host adapter (trials 002 and R01).
//!
//! Custodies the truth layer as an ECS resource and submits the same
//! fixture and command sequence through the same boundary, one command
//! per schedule tick. The host contributes scheduling and projection
//! only — no gameplay semantics — and the parity gate demands that it
//! reproduce the pure run's canonical receipts and exact final canonical
//! state. The world hash remains a redundant checksum. The truth layer
//! itself never references Bevy; only this adapter does.
//!
//! Custody doctrine (Runtime Contract R1 amendment): custody of the
//! canonical `World` inside an ECS resource grants no semantic
//! authority. Exactly one registered system holds mutable access to the
//! `Truth` resource — `submit_next`, the commit system — and a topology
//! test pins that count. The command queue is transport, not truth, so
//! it lives in its own resource. Projections (R01) are disposable view
//! entities derived from canonical state, carrying the canonical hash
//! they were derived from; they can be corrupted or lost without truth
//! noticing, and every publish replaces them in full.

use bevy_ecs::prelude::{Component, Entity, Or, ResMut, Resource, Schedule, With};

use crate::boundary::{Command, Receipt, World, submit};

/// The whole truth layer as one ECS resource: the host custodies it but
/// never holds a second mutation path into it — every write still goes
/// through `submit`, from exactly one registered system.
#[derive(Resource)]
struct Truth {
    world: World,
    seq: u64,
    log: Vec<Receipt>,
}

/// Pending commands, reversed so `pop` yields the original order.
/// Transport, not truth — deliberately outside the `Truth` resource so
/// loading a trial never requires mutable canonical access.
#[derive(Resource, Default)]
struct CommandQueue(Vec<Command>);

/// One tick: submit the next queued command through the boundary. The
/// only registered system with mutable access to `Truth`.
fn submit_next(mut truth: ResMut<Truth>, mut queue: ResMut<CommandQueue>) {
    if let Some(cmd) = queue.0.pop() {
        let truth = &mut *truth;
        truth.seq += 1;
        let receipt = submit(&mut truth.world, truth.seq, cmd);
        truth.log.push(receipt);
    }
}

/// Disposable projection of one character; plain copied facts plus the
/// canonical state hash the projection was derived from. No `World`, no
/// owner storage, no proof tokens, no mutable handle back to truth.
#[derive(Component)]
struct CharacterView {
    id: u64,
    stamina: u8,
    inventory_g: u64,
    derived_from: u64,
}

/// Disposable projection of one site; same doctrine as `CharacterView`.
#[derive(Component)]
struct SiteView {
    id: u64,
    tier: &'static str,
    stock_g: u64,
    derived_from: u64,
}

/// Disposable projection of one claim; same doctrine as `CharacterView`.
#[derive(Component)]
struct ClaimView {
    id: u64,
    holder: u64,
    site: u64,
    witnessed: bool,
    derived_from: u64,
}

/// Host-local faults: transport and presentation failures that occur
/// outside `submit`. A closed set, reported beside canonical receipts,
/// never inside them — a host fault is not a game outcome, produces no
/// canonical receipt, and consumes no canonical sequence (Runtime
/// Contract R5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostFault {
    /// The intention failed host admission before reaching the boundary.
    AdmissionFailed,
    /// A projection consumer failed downstream of a successful commit.
    ProjectionConsumerFailed,
}

impl HostFault {
    pub fn code(self) -> &'static str {
        match self {
            Self::AdmissionFailed => "admission_failed",
            Self::ProjectionConsumerFailed => "projection_consumer_failed",
        }
    }
}

/// A Bevy-hosted trial: schedule-driven canonical execution plus a
/// replaceable projection layer and a host-local fault log.
pub struct Host {
    ecs: bevy_ecs::world::World,
    schedule: Schedule,
    host_faults: Vec<HostFault>,
    injected_admission_fault: bool,
}

impl Host {
    pub fn new(build_fixture: fn() -> World) -> Self {
        let mut ecs = bevy_ecs::world::World::new();
        let mut schedule = Schedule::default();
        schedule.add_systems(submit_next);
        ecs.insert_resource(Truth {
            world: build_fixture(),
            seq: 0,
            log: Vec::new(),
        });
        ecs.insert_resource(CommandQueue::default());
        Self {
            ecs,
            schedule,
            host_faults: Vec::new(),
            injected_admission_fault: false,
        }
    }

    /// Admits each command through the transport gate, then runs one
    /// schedule tick per admitted command. An admission fault is
    /// recorded host-locally and the intention never reaches the
    /// boundary: no receipt, no canonical sequence consumed. May be
    /// called again to continue the trial.
    pub fn run_trial(&mut self, commands: &[Command]) {
        for cmd in commands {
            if self.admit() {
                self.ecs.resource_mut::<CommandQueue>().0.push(*cmd);
                self.schedule.run(&mut self.ecs);
            }
        }
    }

    /// Arms a one-shot injected transport failure for the next admission
    /// — the R02 falsifier and gate-probe surface. Simulation only: no
    /// real transport exists yet, and a real one must route its failures
    /// through the same closed host-fault vocabulary.
    pub fn fail_next_admission(&mut self) {
        self.injected_admission_fault = true;
    }

    fn admit(&mut self) -> bool {
        if self.injected_admission_fault {
            self.injected_admission_fault = false;
            self.host_faults.push(HostFault::AdmissionFailed);
            return false;
        }
        true
    }

    /// The host-local fault log — reported beside canonical receipts,
    /// never inside them.
    pub fn host_faults(&self) -> &[HostFault] {
        &self.host_faults
    }

    /// Publishes the projection, then hands the rendered view lines to a
    /// downstream consumer. A consumer failure is recorded host-locally
    /// and returns `false`; it cannot invalidate the commit the
    /// projection was derived from.
    pub fn publish_to<E>(&mut self, consumer: impl FnOnce(&[String]) -> Result<(), E>) -> bool {
        self.publish();
        let views = self.view_state();
        match consumer(&views) {
            Ok(()) => true,
            Err(_) => {
                self.host_faults.push(HostFault::ProjectionConsumerFailed);
                false
            }
        }
    }

    /// Read-only canonical observations.
    pub fn truth_state(&self) -> Vec<String> {
        self.ecs.resource::<Truth>().world.canonical_state()
    }

    pub fn truth_hash(&self) -> u64 {
        self.ecs.resource::<Truth>().world.hash()
    }

    pub fn receipts(&self) -> Vec<String> {
        self.ecs
            .resource::<Truth>()
            .log
            .iter()
            .map(Receipt::canonical_line)
            .collect()
    }

    pub fn receipt_log(&self) -> &[Receipt] {
        &self.ecs.resource::<Truth>().log
    }

    /// Publish: replace the entire projection with fresh view entities
    /// derived from canonical truth, each carrying the canonical hash it
    /// was derived from. Reads truth immutably; writes only views.
    pub fn publish(&mut self) {
        let (derived_from, characters, sites, claims) = {
            let world = &self.ecs.resource::<Truth>().world;
            let derived_from = world.hash();
            let characters: Vec<(u64, u8, u64)> = world
                .characters
                .iter()
                .map(|(id, s)| (id.0, s.points(), world.economy.inventory(id).grams()))
                .collect();
            let sites: Vec<(u64, &'static str, u64)> = world
                .economy
                .sites_iter()
                .map(|(id, tier, stock)| (id.0, tier.code(), stock.grams()))
                .collect();
            let claims: Vec<(u64, u64, u64, bool)> = world
                .social
                .claims_iter()
                .map(|(claim, holder, site, witnessed)| (claim.0, holder.0, site.0, witnessed))
                .collect();
            (derived_from, characters, sites, claims)
        };
        let mut stale = self
            .ecs
            .query_filtered::<Entity, Or<(With<CharacterView>, With<SiteView>, With<ClaimView>)>>();
        let stale: Vec<Entity> = stale.iter(&self.ecs).collect();
        for entity in stale {
            self.ecs.despawn(entity);
        }
        for (id, stamina, inventory_g) in characters {
            self.ecs.spawn(CharacterView {
                id,
                stamina,
                inventory_g,
                derived_from,
            });
        }
        for (id, tier, stock_g) in sites {
            self.ecs.spawn(SiteView {
                id,
                tier,
                stock_g,
                derived_from,
            });
        }
        for (id, holder, site, witnessed) in claims {
            self.ecs.spawn(ClaimView {
                id,
                holder,
                site,
                witnessed,
                derived_from,
            });
        }
    }

    /// The projection rendered in canonical-state line format (without
    /// the revisions line, which is not projected). Sorted per domain,
    /// so it is directly comparable against `truth_state()`.
    pub fn view_state(&mut self) -> Vec<String> {
        let mut characters: Vec<(u64, String)> = {
            let mut query = self.ecs.query::<&CharacterView>();
            query
                .iter(&self.ecs)
                .map(|v| {
                    (
                        v.id,
                        format!(
                            "character C{} stamina={} inventory_g={}",
                            v.id, v.stamina, v.inventory_g
                        ),
                    )
                })
                .collect()
        };
        let mut sites: Vec<(u64, String)> = {
            let mut query = self.ecs.query::<&SiteView>();
            query
                .iter(&self.ecs)
                .map(|v| {
                    (
                        v.id,
                        format!("site S{} tier={} stock_g={}", v.id, v.tier, v.stock_g),
                    )
                })
                .collect()
        };
        let mut claims: Vec<(u64, String)> = {
            let mut query = self.ecs.query::<&ClaimView>();
            query
                .iter(&self.ecs)
                .map(|v| {
                    (
                        v.id,
                        format!(
                            "claim K{} holder=C{} site=S{} witnessed={}",
                            v.id, v.holder, v.site, v.witnessed
                        ),
                    )
                })
                .collect()
        };
        characters.sort();
        sites.sort();
        claims.sort();
        characters
            .into_iter()
            .chain(sites)
            .chain(claims)
            .map(|(_, line)| line)
            .collect()
    }

    /// The canonical-state identity every current view claims to derive
    /// from.
    pub fn view_identities(&mut self) -> Vec<u64> {
        let mut identities = Vec::new();
        let mut characters = self.ecs.query::<&CharacterView>();
        identities.extend(characters.iter(&self.ecs).map(|v| v.derived_from));
        let mut sites = self.ecs.query::<&SiteView>();
        identities.extend(sites.iter(&self.ecs).map(|v| v.derived_from));
        let mut claims = self.ecs.query::<&ClaimView>();
        identities.extend(claims.iter(&self.ecs).map(|v| v.derived_from));
        identities
    }

    #[cfg(test)]
    fn into_truth(mut self) -> (World, Vec<Receipt>) {
        let truth = self
            .ecs
            .remove_resource::<Truth>()
            .expect("truth resource survives the schedule");
        (truth.world, truth.log)
    }
}

/// Out-of-band corruption surface for the R01 falsifier — test builds
/// only. Simulates a buggy or hostile downstream consumer damaging the
/// projection; canonical truth must not notice.
#[cfg(test)]
impl Host {
    fn corrupt_first_character_view(&mut self, stamina: u8) {
        let mut query = self.ecs.query::<&mut CharacterView>();
        let mut view = query
            .iter_mut(&mut self.ecs)
            .next()
            .expect("a character view exists");
        view.stamina = stamina;
    }

    fn despawn_first_claim_view(&mut self) {
        let mut query = self.ecs.query_filtered::<Entity, With<ClaimView>>();
        let entity = query.iter(&self.ecs).next().expect("a claim view exists");
        self.ecs.despawn(entity);
    }
}

/// Runs a whole trial inside a Bevy ECS world and returns the final
/// truth world and receipt log — the test-side parity helper used by the
/// trial/002 parity test and the trial/007 trace harness. The runtime
/// gate in `main.rs` drives `Host` directly.
#[cfg(test)]
pub fn run_hosted(build_fixture: fn() -> World, commands: &[Command]) -> (World, Vec<Receipt>) {
    let mut host = Host::new(build_fixture);
    host.run_trial(commands);
    host.into_truth()
}

#[cfg(test)]
mod r02_host_failure_tests {
    use super::*;
    use crate::boundary::{CharacterId, ClaimId, Command, WitnessCommand, submit};

    fn extra_witness() -> Command {
        Command::Witness(WitnessCommand {
            witness: CharacterId(1),
            claim: ClaimId(9),
        })
    }

    /// Falsifier (R02, Runtime Contract R5 row "host failure before
    /// submit"): an intention that fails host admission never reaches
    /// the boundary — zero truth mutation, no canonical receipt, no
    /// canonical sequence consumed — and the fault is reported in the
    /// host-local closed vocabulary, never as a game outcome. The same
    /// intention re-admitted afterwards must be byte-identical to a pure
    /// reference that never saw a host fault at all.
    #[test]
    fn falsification_admission_failure_must_leave_zero_canonical_trace() {
        let cmds = crate::commands();
        let mut host = Host::new(crate::fixture);
        host.run_trial(&cmds);
        let truth_before = host.truth_state();
        let hash_before = host.truth_hash();
        let receipts_before = host.receipts();

        host.fail_next_admission();
        host.run_trial(std::slice::from_ref(&extra_witness()));
        assert_eq!(host.truth_state(), truth_before, "zero truth mutation");
        assert_eq!(host.truth_hash(), hash_before);
        assert_eq!(host.receipts(), receipts_before, "no canonical receipt");
        assert_eq!(host.host_faults(), &[HostFault::AdmissionFailed]);

        // Re-admission: canonical sequence was never consumed by the
        // host fault, so the retry matches a fault-free pure reference.
        host.run_trial(std::slice::from_ref(&extra_witness()));
        let mut reference = crate::fixture();
        let mut reference_lines = Vec::new();
        for (i, cmd) in cmds.iter().chain([&extra_witness()]).enumerate() {
            reference_lines.push(submit(&mut reference, i as u64 + 1, *cmd).canonical_line());
        }
        assert_eq!(host.receipts(), reference_lines);
        assert_eq!(host.truth_state(), reference.canonical_state());
        assert_eq!(host.host_faults(), &[HostFault::AdmissionFailed]);
    }

    /// Falsifier (R02, Runtime Contract R5 row "presentation failure"):
    /// a projection consumer that fails downstream of a successful
    /// commit cannot invalidate the commit — canonical state and
    /// receipts stay authoritative, the fault is reported separately,
    /// and the next publish still serves the projection in full.
    #[test]
    fn falsification_projection_consumer_failure_must_not_invalidate_commit() {
        let cmds = crate::commands();
        let mut host = Host::new(crate::fixture);
        host.run_trial(&cmds);
        let truth_before = host.truth_state();
        let receipts_before = host.receipts();

        let delivered = host.publish_to(|_| Err("render exploded"));
        assert!(!delivered, "the consumer failure is visible to the host");
        assert_eq!(host.host_faults(), &[HostFault::ProjectionConsumerFailed]);
        assert_eq!(host.truth_state(), truth_before, "commit remains valid");
        assert_eq!(host.receipts(), receipts_before);

        let mut seen: Vec<String> = Vec::new();
        let delivered = host.publish_to(|views| {
            seen = views.to_vec();
            Ok::<(), &str>(())
        });
        assert!(delivered, "the next publish serves the projection in full");
        assert_eq!(seen.as_slice(), &truth_before[..truth_before.len() - 1]);
    }

    /// Topology pin (R02, Runtime Contract R5 row "internal invariant
    /// failure"): the host must never translate a truth-layer panic into
    /// a disposition — a stale proof token or impossible apply stays
    /// loud. The host source therefore contains no unwind-catching at
    /// all; patterns are built at runtime so this test's own source does
    /// not count.
    #[test]
    fn host_never_catches_truth_panics() {
        let source = include_str!("host_bevy.rs");
        for pattern in [
            format!("catch_{}", "unwind"),
            format!("panic::{}", "catch"),
            format!("set_{}", "hook"),
        ] {
            assert_eq!(
                source.matches(pattern.as_str()).count(),
                0,
                "host must not intercept truth-layer panics ({pattern})"
            );
        }
    }
}

#[cfg(test)]
mod r01_projection_tests {
    use super::*;
    use crate::boundary::{CharacterId, ClaimId, Command, WitnessCommand, submit};

    /// Custody invariant (Runtime Contract R1 amendment): exactly one
    /// registered host system may hold mutable canonical access — the
    /// commit system that calls `submit`. Code-topology check; the
    /// pattern is built at runtime so this test's own source does not
    /// count as an occurrence.
    #[test]
    fn custody_exactly_one_mutable_truth_access() {
        let source = include_str!("host_bevy.rs");
        let pattern = format!("ResMut<{}>", "Truth");
        assert_eq!(
            source.matches(pattern.as_str()).count(),
            1,
            "exactly one registered host system may hold mutable canonical access"
        );
    }

    /// Falsifier (R01, Runtime Contract R4): a projection derived from
    /// canonical state can be corrupted out of band without canonical
    /// truth, receipts, or hash changing; the next publish replaces the
    /// projection in full from canonical truth; and canonical execution
    /// after the corruption is byte-identical to a pure reference that
    /// never had a projection at all.
    #[test]
    fn falsification_corrupted_projection_must_not_touch_truth() {
        let cmds = crate::commands();
        let mut host = Host::new(crate::fixture);
        host.run_trial(&cmds);
        host.publish();

        let truth_before = host.truth_state();
        let hash_before = host.truth_hash();
        let receipts_before = host.receipts();
        let expected_views = &truth_before[..truth_before.len() - 1];
        assert_eq!(host.view_state(), expected_views, "publish projects truth");
        assert!(
            host.view_identities()
                .iter()
                .all(|derived_from| *derived_from == hash_before),
            "every view names the canonical state it was derived from"
        );

        // Out-of-band corruption: falsify one character view and drop a
        // claim view entirely.
        host.corrupt_first_character_view(255);
        host.despawn_first_claim_view();
        assert_ne!(host.view_state(), expected_views, "projection is broken");

        // Canonical truth is untouched by projection damage.
        assert_eq!(host.truth_state(), truth_before);
        assert_eq!(host.truth_hash(), hash_before);
        assert_eq!(host.receipts(), receipts_before);

        // The next publish rebuilds the projection in full from truth.
        host.publish();
        assert_eq!(host.view_state(), expected_views, "republish rebuilds");
        assert!(
            host.view_identities()
                .iter()
                .all(|derived_from| *derived_from == hash_before)
        );

        // Canonical execution after the corruption matches a pure
        // reference that never had a projection.
        let extra = Command::Witness(WitnessCommand {
            witness: CharacterId(1),
            claim: ClaimId(9),
        });
        host.run_trial(std::slice::from_ref(&extra));
        let mut reference = crate::fixture();
        let mut reference_lines = Vec::new();
        for (i, cmd) in cmds.iter().chain([&extra]).enumerate() {
            reference_lines.push(submit(&mut reference, i as u64 + 1, *cmd).canonical_line());
        }
        assert_eq!(host.receipts(), reference_lines);
        assert_eq!(
            host.truth_state(),
            reference.canonical_state(),
            "projection lifecycle left zero trace in canonical truth"
        );
    }
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
