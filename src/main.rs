//! Truth-layer slice host (pure Rust).
//!
//! Seeds a mechanical-example fixture, submits a fixed command sequence
//! through the boundary, prints the canonical receipts and deterministic
//! world hash, then runs the seven bounded oracles. Exits non-zero if any
//! oracle fails, so `cargo run` is part of the compiler gate.
//!
//! The Bevy host is ON HOLD behind the off-by-default `bevy-host` feature
//! until this pure boundary passes the gate.

mod boundary;
mod character;
mod economy;
mod oracles;
mod social;

use std::process::ExitCode;

use boundary::{
    CharacterId, ClaimId, GatherCommand, InfraTier, MassGrams, Receipt, SiteId, Stamina, World,
    submit,
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
            (CharacterId(2), Stamina::new(25).expect("within bounds")),
            (CharacterId(3), Stamina::new(5).expect("within bounds")),
        ]),
        economy: EconomyOwner::seed_sites([
            (SiteId(1), InfraTier::Established, MassGrams::new(2000)),
            (SiteId(2), InfraTier::Crude, MassGrams::new(300)),
            (SiteId(3), InfraTier::Advanced, MassGrams::new(5000)),
            (SiteId(4), InfraTier::None, MassGrams::new(1000)),
        ]),
        social: SocialOwner::seed_claims([
            (ClaimId(1), CharacterId(1), SiteId(1), true),
            (ClaimId(2), CharacterId(2), SiteId(2), true),
            (ClaimId(3), CharacterId(2), SiteId(1), false),
            (ClaimId(4), CharacterId(3), SiteId(2), true),
            (ClaimId(5), CharacterId(1), SiteId(3), true),
            (ClaimId(6), CharacterId(2), SiteId(4), true),
        ]),
    }
}

/// A fixed sequence that exercises Accepted, Partial, and every reachable
/// refusal in the fixture: unwitnessed claim, exhausted actor, empty site,
/// and claim/site mismatch.
fn commands() -> Vec<GatherCommand> {
    let gather = |actor, claim, site| GatherCommand {
        actor: CharacterId(actor),
        claim: ClaimId(claim),
        site: SiteId(site),
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
    ]
}

fn main() -> ExitCode {
    let mut world = fixture();
    let baseline_mass = world.economy.total_mass();
    let cmds = commands();

    let mut log: Vec<Receipt> = Vec::with_capacity(cmds.len());
    for (i, cmd) in cmds.iter().enumerate() {
        let receipt = submit(&mut world, i as u64 + 1, *cmd);
        println!("{}", receipt.canonical_line());
        log.push(receipt);
    }
    println!("world_hash=0x{:016x}", world.hash());

    for (id, stamina) in world.characters.iter() {
        println!(
            "character C{} stamina={} inventory_g={}",
            id.0,
            stamina.points(),
            world.economy.inventory(id).grams()
        );
    }
    for site in [SiteId(1), SiteId(2), SiteId(3), SiteId(4)] {
        if let Some(stock) = world.economy.stock(site) {
            println!("site S{} stock_g={}", site.0, stock.grams());
        }
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

    if all_pass {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
