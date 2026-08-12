//! Truth-layer slice host (pure Rust).
//!
//! Seeds a mechanical-example fixture, submits a fixed command sequence
//! through the boundary, prints the canonical receipts and deterministic
//! world hash, then runs the ten bounded oracles. Exits non-zero if any
//! oracle fails, so `cargo run` is part of the compiler gate.
//!
//! The Bevy ECS host adapter lives behind the off-by-default `bevy-host`
//! feature: `cargo run --features bevy-host` additionally replays the
//! whole trial inside a Bevy ECS world and exits non-zero unless the
//! hosted run reproduces the pure run's receipts and final hash exactly.

mod boundary;
mod character;
mod economy;
#[cfg(feature = "bevy-host")]
mod host_bevy;
mod oracles;
mod social;

use std::process::ExitCode;

use boundary::{
    CharacterId, ClaimId, Command, GatherCommand, InfraTier, MassGrams, Receipt, SiteId, Stamina,
    WitnessCommand, World, fixture_identity, grammar_fingerprint, receipt_chain_digest, submit,
    validate_world_coherence,
};
use character::CharacterOwner;
use economy::EconomyOwner;
use oracles::OracleCtx;
use social::SocialOwner;

/// Fixture numbers are mechanical examples only — not balance and not
/// historical truth.
fn fixture() -> World {
    World {
        characters: CharacterOwner::seed([
            (CharacterId(1), Stamina::new(90).expect("within bounds")),
            (CharacterId(2), Stamina::new(39).expect("within bounds")),
            (CharacterId(3), Stamina::new(5).expect("within bounds")),
            (CharacterId(4), Stamina::new(12).expect("within bounds")),
        ])
        .expect("no duplicate characters"),
        economy: EconomyOwner::seed_sites([
            (SiteId(1), InfraTier::Established, MassGrams::new(2000)),
            (SiteId(2), InfraTier::Crude, MassGrams::new(300)),
            (SiteId(3), InfraTier::Advanced, MassGrams::new(5000)),
            (SiteId(4), InfraTier::None, MassGrams::new(1000)),
        ])
        .expect("no duplicate sites"),
        social: SocialOwner::seed_claims([
            (ClaimId(1), CharacterId(1), SiteId(1), true),
            (ClaimId(2), CharacterId(2), SiteId(2), true),
            (ClaimId(3), CharacterId(2), SiteId(1), false),
            (ClaimId(4), CharacterId(3), SiteId(2), true),
            (ClaimId(5), CharacterId(1), SiteId(3), true),
            (ClaimId(6), CharacterId(2), SiteId(4), true),
            (ClaimId(7), CharacterId(4), SiteId(4), true),
            (ClaimId(8), CharacterId(4), SiteId(4), false),
            (ClaimId(9), CharacterId(1), SiteId(1), false),
        ])
        .expect("no duplicate claims"),
    }
}

/// A fixed two-verb sequence that exercises Accepted, Partial, and every
/// reachable refusal in the fixture — including the witness verb's own
/// refusals and the interplay between the verbs (a witnessed claim
/// unlocking a gather gate, an exhausted character still allowed to
/// witness).
fn commands() -> Vec<Command> {
    let gather = |actor, claim, site| {
        Command::Gather(GatherCommand {
            actor: CharacterId(actor),
            claim: ClaimId(claim),
            site: SiteId(site),
        })
    };
    let witness = |witness, claim| {
        Command::Witness(WitnessCommand {
            witness: CharacterId(witness),
            claim: ClaimId(claim),
        })
    };
    vec![
        gather(1, 1, 1), // accepted: fresh x established
        gather(2, 2, 2), // partial: crude site runs short
        gather(2, 3, 1), // refused: claim not witnessed
        gather(3, 4, 2), // refused: actor exhausted
        gather(1, 1, 1), // partial: site nearly depleted
        gather(1, 1, 1), // refused: site empty
        gather(1, 1, 2), // refused: claim/site mismatch
        gather(1, 5, 3), // accepted: steady x advanced
        gather(2, 6, 4), // accepted: low x no infrastructure
        gather(4, 7, 4), // refused: 12 points cannot cover a 15-point spend
        witness(1, 3),   // accepted: C1 attests C2's claim (flat cost)
        witness(1, 3),   // refused: claim already witnessed
        witness(2, 3),   // refused: cannot witness own claim
        gather(2, 3, 1), // refused: claim now witnessed, but gatherer exhausted
        witness(3, 8),   // accepted: exhausted C3 may still witness (5 >= 5)
        witness(3, 9),   // refused: 0 points cannot cover the witness cost
    ]
}

fn main() -> ExitCode {
    let mut world = fixture();
    validate_world_coherence(&world).expect("fixture is referentially coherent");
    println!("grammar=0x{:016x}", grammar_fingerprint());
    let fixture_hash = world.hash();
    let baseline_mass = world.economy.total_mass();
    let cmds = commands();

    let mut log: Vec<Receipt> = Vec::with_capacity(cmds.len());
    for (i, cmd) in cmds.iter().enumerate() {
        let receipt = submit(&mut world, i as u64 + 1, *cmd);
        println!("{}", receipt.canonical_line());
        log.push(receipt);
    }
    println!("world_hash=0x{:016x}", world.hash());

    // The end-of-run state summary IS the canonical serialization —
    // exactly what parity and replay compare, printed rather than
    // paraphrased.
    for line in world.canonical_state() {
        println!("{line}");
    }

    let ctx = OracleCtx {
        world: &world,
        baseline_mass,
        build_fixture: fixture,
        commands: &cmds,
        log: &log,
    };
    let verdicts = oracles::run_all(&ctx);
    let mut all_pass = true;
    for verdict in &verdicts {
        let status = if verdict.pass { "PASS" } else { "FAIL" };
        println!("oracle {status} {} ({})", verdict.name, verdict.detail);
        all_pass &= verdict.pass;
    }

    // Host parity gate (trial/002): the Bevy-hosted replay must reproduce
    // the pure run's receipts and final hash byte-for-byte, or the host
    // has acquired semantics of its own. Trial R01 extends the gate with
    // projection non-authority: a published projection must equal the
    // canonical facts and name the canonical state it derives from.
    #[cfg(feature = "bevy-host")]
    {
        let mut host = host_bevy::Host::new(fixture);
        host.run_trial(&cmds);
        let pure_lines: Vec<String> = log.iter().map(Receipt::canonical_line).collect();
        let receipts_match = host.receipts() == pure_lines
            && receipt_chain_digest(host.receipt_log()) == receipt_chain_digest(&log);
        // Exact serialization carries the equality claim; the hash is
        // its checksum address (FNV-1a is not injective).
        let canonical = world.canonical_state();
        let state_match = host.truth_state() == canonical;
        let world_match = host.truth_hash() == world.hash();
        println!(
            "bevy_host_parity receipts_match={} state_match={} world_match={} \
             receipts=0x{:016x} world=0x{:016x}",
            receipts_match,
            state_match,
            world_match,
            receipt_chain_digest(host.receipt_log()),
            host.truth_hash(),
        );
        host.publish();
        let views_match = host.view_state().as_slice() == &canonical[..canonical.len() - 1]
            && host
                .view_identities()
                .iter()
                .all(|derived_from| *derived_from == world.hash());
        println!(
            "bevy_projection views_match={} derived_from=0x{:016x}",
            views_match,
            world.hash(),
        );
        all_pass &= receipts_match && state_match && world_match && views_match;
    }

    // Proof envelope: the full identity of this run for cross-trial
    // comparison. baseline_commit is runner-supplied (git knows it, the
    // binary does not); everything else is recomputed from the run itself.
    println!(
        "envelope baseline_commit={} grammar=0x{:016x} fixture=0x{:016x} \
         receipts=0x{:016x} world=0x{:016x} oracles={}v{}",
        std::env::var("BASELINE_COMMIT").unwrap_or_else(|_| "-".to_owned()),
        grammar_fingerprint(),
        fixture_identity(fixture_hash, &cmds),
        receipt_chain_digest(&log),
        world.hash(),
        oracles::ORACLE_COUNT,
        oracles::ORACLE_SUITE_VERSION,
    );

    if all_pass {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
