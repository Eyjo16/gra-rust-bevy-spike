//! Exactly seven bounded oracles.
//!
//! Each oracle is a pure check over bounded inputs — the current world,
//! the receipt log, and a replayable fixture — and returns a verdict.
//! `run_all` returns a fixed-size array, so the count of seven is enforced
//! by the type system.

use crate::boundary::{
    GatherCommand, MassGrams, OutcomeKind, Receipt, Stamina, StaminaBand, World, YIELD_TABLE_GRAMS,
    submit,
};

pub const ORACLE_COUNT: usize = 7;

pub struct OracleVerdict {
    pub name: &'static str,
    pub pass: bool,
    pub detail: String,
}

impl OracleVerdict {
    fn new(name: &'static str, pass: bool, detail: String) -> Self {
        Self { name, pass, detail }
    }
}

pub struct OracleCtx<'a> {
    pub world: &'a World,
    pub baseline_mass: MassGrams,
    pub build_fixture: fn() -> World,
    pub commands: &'a [GatherCommand],
    pub log: &'a [Receipt],
}

pub fn run_all(ctx: &OracleCtx<'_>) -> [OracleVerdict; ORACLE_COUNT] {
    [
        stamina_in_bounds(ctx),
        mass_conserved(ctx),
        witnessed_gate(ctx),
        exhausted_gate(ctx),
        closed_reasons(ctx),
        cell_bounds(ctx),
        replay_determinism(ctx),
    ]
}

/// 1. Every character's stamina stays within `0..=Stamina::MAX`.
fn stamina_in_bounds(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations: Vec<String> = ctx
        .world
        .characters
        .iter()
        .filter(|(_, stamina)| stamina.points() > Stamina::MAX)
        .map(|(id, stamina)| format!("C{}={}", id.0, stamina.points()))
        .collect();
    OracleVerdict::new(
        "stamina_in_bounds",
        violations.is_empty(),
        if violations.is_empty() {
            "all stamina within 0..=100".to_owned()
        } else {
            violations.join(",")
        },
    )
}

/// 2. Total mass (sites + inventories) equals the fixture baseline.
fn mass_conserved(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let current = ctx.world.economy.total_mass();
    OracleVerdict::new(
        "mass_conserved",
        current == ctx.baseline_mass,
        format!(
            "baseline={}g current={}g",
            ctx.baseline_mass.grams(),
            current.grams()
        ),
    )
}

/// 3. The witnessed claim is a boolean gate: no receipt moves mass without
///    a witnessed claim.
fn witnessed_gate(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations = ctx
        .log
        .iter()
        .filter(|r| r.outcome.yields_mass() && !r.witnessed)
        .count();
    OracleVerdict::new(
        "witnessed_gate",
        violations == 0,
        format!("{violations} unwitnessed receipts moved mass"),
    )
}

/// 4. An exhausted actor never yields: every mass-moving receipt sits in a
///    non-exhausted stamina band.
fn exhausted_gate(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations = ctx
        .log
        .iter()
        .filter(|r| r.outcome.yields_mass())
        .filter(|r| !matches!(r.band, Some(band) if band != StaminaBand::Exhausted))
        .count();
    OracleVerdict::new(
        "exhausted_gate",
        violations == 0,
        format!("{violations} exhausted or band-less receipts moved mass"),
    )
}

/// 5. Outcome reasons are closed: every receipt's codes round-trip through
///    the closed enums.
fn closed_reasons(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations = ctx
        .log
        .iter()
        .filter(|r| !r.outcome.codes_round_trip())
        .count();
    OracleVerdict::new(
        "closed_reasons",
        violations == 0,
        format!("{violations} receipts with unclosed reason codes"),
    )
}

/// 6. Every yield stays inside the single active 4x4 cell: mass moved never
///    exceeds the table value for the recorded band and tier, and an
///    Accepted outcome matches it exactly.
fn cell_bounds(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let violations = ctx
        .log
        .iter()
        .filter(|r| r.outcome.yields_mass())
        .filter(|r| {
            let (Some(band), Some(tier)) = (r.band, r.tier) else {
                return true;
            };
            let cell = YIELD_TABLE_GRAMS[band.index()][tier.index()];
            match r.outcome {
                OutcomeKind::Accepted => r.mass_moved.grams() != cell,
                _ => r.mass_moved.grams() >= cell || r.mass_moved.is_zero(),
            }
        })
        .count();
    OracleVerdict::new(
        "cell_bounds",
        violations == 0,
        format!("{violations} receipts outside the 4x4 cell"),
    )
}

/// 7. Determinism: replaying the same fixture and commands reproduces the
///    same receipts and the same final world hash.
fn replay_determinism(ctx: &OracleCtx<'_>) -> OracleVerdict {
    let mut replay_world = (ctx.build_fixture)();
    let replay_lines: Vec<String> = ctx
        .commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| submit(&mut replay_world, i as u64 + 1, *cmd).canonical_line())
        .collect();
    let original_lines: Vec<String> = ctx.log.iter().map(Receipt::canonical_line).collect();
    let hashes_match = replay_world.hash() == ctx.world.hash();
    let lines_match = replay_lines == original_lines;
    OracleVerdict::new(
        "replay_determinism",
        hashes_match && lines_match,
        format!("hashes_match={hashes_match} receipts_match={lines_match}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::{CharacterId, ClaimId, InfraTier, PartialReason, RefusalReason, SiteId};
    use crate::character::CharacterOwner;
    use crate::economy::EconomyOwner;
    use crate::social::SocialOwner;

    // Fixture numbers are mechanical examples only — not balance.
    fn fixture() -> World {
        World {
            characters: CharacterOwner::seed([
                (CharacterId(1), Stamina::new(90).unwrap()),
                (CharacterId(2), Stamina::new(25).unwrap()),
                (CharacterId(3), Stamina::new(5).unwrap()),
            ]),
            economy: EconomyOwner::seed_sites([
                (SiteId(1), InfraTier::Established, MassGrams::new(2000)),
                (SiteId(2), InfraTier::Crude, MassGrams::new(300)),
            ]),
            social: SocialOwner::seed_claims([
                (ClaimId(1), CharacterId(1), SiteId(1), true),
                (ClaimId(2), CharacterId(2), SiteId(2), true),
                (ClaimId(3), CharacterId(2), SiteId(1), false),
                (ClaimId(4), CharacterId(3), SiteId(2), true),
            ]),
        }
    }

    fn commands() -> Vec<GatherCommand> {
        vec![
            GatherCommand {
                actor: CharacterId(1),
                claim: ClaimId(1),
                site: SiteId(1),
            },
            GatherCommand {
                actor: CharacterId(2),
                claim: ClaimId(2),
                site: SiteId(2),
            },
            GatherCommand {
                actor: CharacterId(2),
                claim: ClaimId(3),
                site: SiteId(1),
            },
            GatherCommand {
                actor: CharacterId(3),
                claim: ClaimId(4),
                site: SiteId(2),
            },
        ]
    }

    fn run_fixture() -> (World, Vec<GatherCommand>, Vec<Receipt>, MassGrams) {
        let mut world = fixture();
        let baseline = world.economy.total_mass();
        let cmds = commands();
        let log: Vec<Receipt> = cmds
            .iter()
            .enumerate()
            .map(|(i, cmd)| submit(&mut world, i as u64 + 1, *cmd))
            .collect();
        (world, cmds, log, baseline)
    }

    #[test]
    fn fixture_run_covers_accepted_partial_and_refused() {
        let (_, _, log, _) = run_fixture();
        assert_eq!(log[0].outcome, OutcomeKind::Accepted);
        assert_eq!(
            log[1].outcome,
            OutcomeKind::Partial(PartialReason::SiteNearlyDepleted)
        );
        assert_eq!(
            log[2].outcome,
            OutcomeKind::Refused(RefusalReason::ClaimNotWitnessed)
        );
        assert_eq!(
            log[3].outcome,
            OutcomeKind::Refused(RefusalReason::ActorExhausted)
        );
    }

    #[test]
    fn all_seven_oracles_pass_on_the_fixture_run() {
        let (world, cmds, log, baseline) = run_fixture();
        let ctx = OracleCtx {
            world: &world,
            baseline_mass: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        let verdicts = run_all(&ctx);
        assert_eq!(verdicts.len(), ORACLE_COUNT);
        for verdict in &verdicts {
            assert!(verdict.pass, "{} failed: {}", verdict.name, verdict.detail);
        }
    }

    #[test]
    fn witnessed_gate_oracle_catches_a_doctored_receipt() {
        let (world, cmds, mut log, baseline) = run_fixture();
        log[0].witnessed = false;
        let ctx = OracleCtx {
            world: &world,
            baseline_mass: baseline,
            build_fixture: fixture,
            commands: &cmds,
            log: &log,
        };
        assert!(!witnessed_gate(&ctx).pass);
    }

    #[test]
    fn two_runs_produce_identical_hashes_and_receipts() {
        let (world_a, _, log_a, _) = run_fixture();
        let (world_b, _, log_b, _) = run_fixture();
        assert_eq!(world_a.hash(), world_b.hash());
        let lines_a: Vec<String> = log_a.iter().map(Receipt::canonical_line).collect();
        let lines_b: Vec<String> = log_b.iter().map(Receipt::canonical_line).collect();
        assert_eq!(lines_a, lines_b);
    }
}
