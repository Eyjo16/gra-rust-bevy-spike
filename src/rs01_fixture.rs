//! RS01 fixture and trace driver (host-only).
//!
//! A dedicated coherent world and three-command trace, driven through
//! the real boundary inside the real Bevy host, yielding one identified
//! publication per beat. Nothing here hardcodes the aftermath: every
//! displayed fact downstream derives from these live publications.
//!
//! Discrepancy record (see docs/rs01-trial-log.md): the dispatch
//! envelope tracked one stamina line through both verbs (65→60→48).
//! Executable truth refuses that arc — the holder cannot witness their
//! own claim (`cannot_witness_own_claim`) and only the holder may
//! gather (`claim_not_held_by_actor`) — so the corrected trace uses two
//! characters. The grammar forces witnessing to be a second person's
//! act. All fixture numbers remain mechanical examples.

use crate::boundary::{
    CharacterId, ClaimId, Command, GatherCommand, InfraTier, MassGrams, Receipt, SiteId, Stamina,
    WitnessCommand, World, validate_world_coherence,
};
use crate::character::CharacterOwner;
use crate::economy::EconomyOwner;
use crate::host_bevy::{Host, Publication};
use crate::social::SocialOwner;

/// The holder and gatherer (presentation alias lives in the renderer).
pub const RS01_HOLDER: CharacterId = CharacterId(1);
/// The witness — the second person the grammar requires.
pub const RS01_WITNESS: CharacterId = CharacterId(2);
pub const RS01_CLAIM: ClaimId = ClaimId(1);
pub const RS01_SITE: SiteId = SiteId(1);

/// One equal projection unit of turf: 200 g. Every displayed block
/// represents exactly this mass; the RS01 fixture keeps all stocks and
/// yields divisible by it (envelope falsifier F5).
pub const TURF_BLOCK_GRAMS: u64 = 200;

/// Mechanical example numbers — not balance, not historical truth.
pub fn rs01_fixture() -> World {
    let world = World {
        characters: CharacterOwner::seed([
            (RS01_HOLDER, Stamina::new(65).expect("within bounds")),
            (RS01_WITNESS, Stamina::new(65).expect("within bounds")),
        ])
        .expect("no duplicate characters"),
        economy: EconomyOwner::seed_sites([(
            RS01_SITE,
            InfraTier::Established,
            MassGrams::new(2000),
        )])
        .expect("no duplicate sites"),
        social: SocialOwner::seed_claims([(RS01_CLAIM, RS01_HOLDER, RS01_SITE, false)])
            .expect("no duplicate claims"),
    };
    validate_world_coherence(&world).expect("RS01 fixture is referentially coherent");
    world
}

/// The three player-submitted commands, in required-flow order.
pub fn rs01_commands() -> [Command; 3] {
    [
        Command::Gather(GatherCommand {
            actor: RS01_HOLDER,
            claim: RS01_CLAIM,
            site: RS01_SITE,
        }),
        Command::Witness(WitnessCommand {
            witness: RS01_WITNESS,
            claim: RS01_CLAIM,
        }),
        Command::Gather(GatherCommand {
            actor: RS01_HOLDER,
            claim: RS01_CLAIM,
            site: RS01_SITE,
        }),
    ]
}

/// One beat of the RS01 trace: the publication after this beat's command
/// (beat 0 carries the initial publication and no command), plus the
/// canonical receipt that produced it.
pub struct Rs01Beat {
    pub index: usize,
    pub command: Option<Command>,
    pub receipt: Option<Receipt>,
    pub publication: Publication,
}

/// Drives the whole trace through the real boundary inside the real
/// Bevy host and returns one identified publication per beat. The host
/// is returned too so a caller (the renderer) can keep submitting or
/// re-publishing against the same custody.
pub fn run_rs01_trace() -> (Vec<Rs01Beat>, Host) {
    let mut host = Host::new(rs01_fixture);
    let mut beats = Vec::with_capacity(4);
    beats.push(Rs01Beat {
        index: 0,
        command: None,
        receipt: None,
        publication: host.publication(),
    });
    for (i, cmd) in rs01_commands().into_iter().enumerate() {
        host.run_trial(std::slice::from_ref(&cmd));
        let receipt = host
            .receipt_log()
            .last()
            .expect("submitted command produced a receipt")
            .clone();
        beats.push(Rs01Beat {
            index: i + 1,
            command: Some(cmd),
            receipt: Some(receipt),
            publication: host.publication(),
        });
    }
    (beats, host)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::{OutcomeKind, RefusalReason, submit};

    fn fact(publication: &Publication, id: CharacterId) -> (u8, u64) {
        let c = publication
            .facts
            .characters
            .iter()
            .find(|c| c.id == id.0)
            .expect("published character fact");
        (c.stamina, c.inventory_g)
    }

    /// The corrected canonical trace, beat by beat — the executable
    /// facts every RS01 visual must derive from. These assertions are
    /// against live publications from the real hosted boundary run, not
    /// against any stored aftermath.
    #[test]
    fn rs01_trace_produces_the_corrected_beats() {
        let (beats, _host) = run_rs01_trace();
        assert_eq!(beats.len(), 4);
        // Every beat after the opening carries the canonical command it
        // rendered from — the Sönnun overlay displays exactly these.
        assert!(beats[0].command.is_none());
        assert!(beats[1..].iter().all(|b| b.command.is_some()));

        // Beat 0 — initial publication.
        let b0 = &beats[0].publication;
        assert_eq!(fact(b0, RS01_HOLDER), (65, 0));
        assert_eq!(fact(b0, RS01_WITNESS), (65, 0));
        assert_eq!(b0.facts.sites[0].stock_g, 2000);
        assert!(!b0.facts.claims[0].witnessed);

        // Beat 1 — gather refused, zero mutation: publication identity
        // (canonical hash and revisions) unchanged from beat 0.
        let r1 = beats[1].receipt.as_ref().unwrap();
        assert_eq!(
            r1.outcome,
            OutcomeKind::Refused(RefusalReason::ClaimNotWitnessed)
        );
        assert_eq!(beats[1].publication.derived_from, b0.derived_from);
        assert_eq!(beats[1].publication.revisions, b0.revisions);
        assert_eq!(fact(&beats[1].publication, RS01_HOLDER), (65, 0));

        // Beat 2 — witness accepted: C2 65→60, claim flips, no mass.
        let r2 = beats[2].receipt.as_ref().unwrap();
        assert_eq!(r2.outcome, OutcomeKind::Accepted);
        assert_eq!(r2.stamina_spent, 5);
        assert!(r2.mass_moved.is_zero());
        let b2 = &beats[2].publication;
        assert_eq!(fact(b2, RS01_WITNESS), (60, 0));
        assert_eq!(fact(b2, RS01_HOLDER), (65, 0));
        assert!(b2.facts.claims[0].witnessed);
        assert!(b2.revisions > b0.revisions);

        // Beat 3 — gather accepted: C1 65→53, 1200 g moves.
        let r3 = beats[3].receipt.as_ref().unwrap();
        assert_eq!(r3.outcome, OutcomeKind::Accepted);
        assert_eq!(r3.stamina_spent, 12);
        assert_eq!(r3.mass_moved, MassGrams::new(1200));
        let b3 = &beats[3].publication;
        assert_eq!(fact(b3, RS01_HOLDER), (53, 1200));
        assert_eq!(fact(b3, RS01_WITNESS), (60, 0));
        assert_eq!(b3.facts.sites[0].stock_g, 800);
        assert!(b3.facts.claims[0].witnessed);
    }

    /// The discrepancy record made executable (envelope rule: executable
    /// truth wins). The envelope's single-actor arc — the holder
    /// witnessing their own claim before gathering — must be refused by
    /// the ratified grammar. This is why the RS01 trace has two people.
    #[test]
    fn falsification_envelope_single_actor_arc_is_unrepresentable() {
        let mut world = rs01_fixture();
        let receipt = submit(
            &mut world,
            1,
            Command::Witness(WitnessCommand {
                witness: RS01_HOLDER,
                claim: RS01_CLAIM,
            }),
        );
        assert_eq!(
            receipt.outcome,
            OutcomeKind::Refused(RefusalReason::CannotWitnessOwnClaim)
        );
        // And the non-holder who can witness cannot gather instead.
        let receipt = submit(
            &mut world,
            2,
            Command::Gather(GatherCommand {
                actor: RS01_WITNESS,
                claim: RS01_CLAIM,
                site: RS01_SITE,
            }),
        );
        assert_eq!(
            receipt.outcome,
            OutcomeKind::Refused(RefusalReason::ClaimNotHeldByActor)
        );
    }

    /// F5 groundwork: every mass the trace displays is an exact multiple
    /// of the equal projection unit.
    #[test]
    fn rs01_masses_are_exact_block_multiples() {
        let (beats, _host) = run_rs01_trace();
        for beat in &beats {
            for site in &beat.publication.facts.sites {
                assert_eq!(site.stock_g % TURF_BLOCK_GRAMS, 0);
            }
            for character in &beat.publication.facts.characters {
                assert_eq!(character.inventory_g % TURF_BLOCK_GRAMS, 0);
            }
        }
    }

    /// F9 groundwork: the same fixture and commands produce identical
    /// canonical evidence on every run.
    #[test]
    fn rs01_trace_is_deterministic() {
        let (beats_a, _) = run_rs01_trace();
        let (beats_b, _) = run_rs01_trace();
        let lines = |beats: &[Rs01Beat]| -> Vec<String> {
            beats
                .iter()
                .filter_map(|b| b.receipt.as_ref().map(Receipt::canonical_line))
                .collect()
        };
        assert_eq!(lines(&beats_a), lines(&beats_b));
        assert_eq!(
            beats_a.last().unwrap().publication.derived_from,
            beats_b.last().unwrap().publication.derived_from
        );
    }
}
