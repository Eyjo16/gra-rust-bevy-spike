//! Bevy ECS host adapter (trial/002).
//!
//! Hosts the truth layer as an ECS resource and submits the same fixture
//! and command sequence through the same boundary, one command per
//! schedule tick. The host contributes scheduling only — no gameplay
//! semantics — and the parity gate demands that it reproduce the pure
//! run's canonical receipts and final world hash byte-for-byte. The
//! truth layer itself never references Bevy; only this adapter does.

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

    /// The parity falsifier for trial/002: the hosted run must reproduce
    /// the pure run's receipts and final hash exactly, on the identical
    /// fixture and command sequence — otherwise the host has acquired
    /// semantics of its own.
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
}
