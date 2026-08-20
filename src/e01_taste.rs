//! Trial/E01 belief and actionability render taste.
//!
//! The canonical world never lies; the fixture-local character belief may.
//! Both scene facts and belief inputs are copied from identified Publications,
//! while actual outcomes come only from receipts produced by Host submissions.
//! The overlay has no command, mutation, or outcome-selection path.

use std::{
    fs,
    path::{Path, PathBuf},
};

use bevy::{
    app::AppExit,
    prelude::*,
    render::{
        RenderPlugin,
        view::screenshot::{Screenshot, save_to_disk},
    },
    text::{LineBreak, TextBounds},
    window::{WindowPlugin, WindowResolution},
};

use crate::{
    boundary::{
        CharacterId, ClaimId, Command, GatherCommand, InfraTier, MassGrams, OutcomeKind, Receipt,
        RefusalReason, ResourceKind, SiteId, Stamina, World as TruthWorld,
        validate_world_coherence,
    },
    character::CharacterOwner,
    economy::EconomyOwner,
    host_bevy::{Host, Publication},
    social::SocialOwner,
};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 800;
const BLOCK_GRAMS: u64 = 200;
/// The E01 scene is winter feed: both sites yield fodder under the
/// licensed RES01 vocabulary. Grams, beats and outcomes are unchanged by
/// RES01 — the kind names what was already moving.
const SCENE_KIND: ResourceKind = ResourceKind::Fodder;
const BELIEF_CONFIDENCE_FLOOR: u8 = 14;
const WARMUP_FRAMES: u16 = 12;
const CAPTURE_TIMEOUT_FRAMES: u16 = 600;
const BELIEF_LINE: &str = "I believe I can manage one last gather.";
const POLICY_FOOTER: &str = "Fixture-local belief policy: confidence at 14+ stamina. Belief is perspective, never command authority or canonical fact.";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BeatKind {
    Initial,
    HrafnRefused,
    EgilAccepted,
    Aftermath,
}

impl BeatKind {
    const ALL: [Self; 4] = [
        Self::Initial,
        Self::HrafnRefused,
        Self::EgilAccepted,
        Self::Aftermath,
    ];

    const fn slug(self) -> &'static str {
        match self {
            Self::Initial => "00-two-beliefs",
            Self::HrafnRefused => "01-belief-wrong",
            Self::EgilAccepted => "02-belief-matched",
            Self::Aftermath => "03-belief-is-not-truth",
        }
    }

    const fn heading(self) -> &'static str {
        match self {
            Self::Initial => "TWO EXPECTATIONS",
            Self::HrafnRefused => "BELIEF WAS WRONG",
            Self::EgilAccepted => "BELIEF MATCHED THIS TIME",
            Self::Aftermath => "BELIEF IS NOT TRUTH",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct BeliefOverlay {
    actor: u64,
    observed_stamina: u8,
    expects_success: bool,
    observed_from: u64,
}

fn belief_from(publication: &Publication, actor: u64) -> Result<BeliefOverlay, String> {
    let character = publication
        .character(actor)
        .ok_or_else(|| format!("publication missing belief actor C{actor}"))?;
    if character.derived_from != publication.derived_from {
        return Err("belief input identity differs from publication".to_owned());
    }
    Ok(BeliefOverlay {
        actor,
        observed_stamina: character.stamina,
        expects_success: character.stamina >= BELIEF_CONFIDENCE_FLOOR,
        observed_from: publication.derived_from,
    })
}

#[derive(Clone)]
struct Beat {
    kind: BeatKind,
    narrative: &'static str,
    publication: Publication,
    belief_source: Publication,
    receipt: Option<Receipt>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SceneFacts {
    hrafn_stamina: u8,
    hrafn_inventory_g: u64,
    egil_stamina: u8,
    egil_inventory_g: u64,
    hrafn_site_g: u64,
    egil_site_g: u64,
    hrafn_claim_witnessed: bool,
    egil_claim_witnessed: bool,
}

impl Beat {
    fn facts(&self) -> Result<SceneFacts, String> {
        let hrafn = self
            .publication
            .character(1)
            .ok_or_else(|| "publication missing C1".to_owned())?;
        let egil = self
            .publication
            .character(2)
            .ok_or_else(|| "publication missing C2".to_owned())?;
        let hrafn_site = self
            .publication
            .site(1)
            .ok_or_else(|| "publication missing S1".to_owned())?;
        let egil_site = self
            .publication
            .site(2)
            .ok_or_else(|| "publication missing S2".to_owned())?;
        let hrafn_claim = self
            .publication
            .claim(1)
            .ok_or_else(|| "publication missing K1".to_owned())?;
        let egil_claim = self
            .publication
            .claim(2)
            .ok_or_else(|| "publication missing K2".to_owned())?;

        if hrafn_site.tier != "established" || egil_site.tier != "established" {
            return Err("E01 sites must remain Established".to_owned());
        }
        // RES01: both scene sites must still yield the kind this scene
        // reads and draws; a drifted kind makes every number below a
        // statement about a different material.
        if hrafn_site.kind != SCENE_KIND.code() || egil_site.kind != SCENE_KIND.code() {
            return Err("E01 sites must remain Fodder".to_owned());
        }
        if (hrafn_claim.holder, hrafn_claim.site) != (1, 1)
            || (egil_claim.holder, egil_claim.site) != (2, 2)
        {
            return Err("E01 claim bindings drifted".to_owned());
        }
        for identity in [
            hrafn.derived_from,
            egil.derived_from,
            hrafn_site.derived_from,
            egil_site.derived_from,
            hrafn_claim.derived_from,
            egil_claim.derived_from,
        ] {
            if identity != self.publication.derived_from {
                return Err("typed scene fact identity differs from publication".to_owned());
            }
        }

        Ok(SceneFacts {
            hrafn_stamina: hrafn.stamina,
            hrafn_inventory_g: hrafn.holding_g(SCENE_KIND),
            egil_stamina: egil.stamina,
            egil_inventory_g: egil.holding_g(SCENE_KIND),
            hrafn_site_g: hrafn_site.stock_g,
            egil_site_g: egil_site.stock_g,
            hrafn_claim_witnessed: hrafn_claim.witnessed,
            egil_claim_witnessed: egil_claim.witnessed,
        })
    }

    fn beliefs(&self) -> Result<[BeliefOverlay; 2], String> {
        Ok([
            belief_from(&self.belief_source, 1)?,
            belief_from(&self.belief_source, 2)?,
        ])
    }
}

fn fixture() -> TruthWorld {
    TruthWorld {
        characters: CharacterOwner::seed([
            (CharacterId(1), Stamina::new(14).expect("fixture stamina")),
            (CharacterId(2), Stamina::new(15).expect("fixture stamina")),
        ])
        .expect("unique E01 characters"),
        economy: EconomyOwner::seed_sites([
            (
                SiteId(1),
                InfraTier::Established,
                SCENE_KIND,
                MassGrams::new(2_000),
            ),
            (
                SiteId(2),
                InfraTier::Established,
                SCENE_KIND,
                MassGrams::new(2_000),
            ),
        ])
        .expect("unique E01 sites"),
        social: SocialOwner::seed_claims([
            (ClaimId(1), CharacterId(1), SiteId(1), true),
            (ClaimId(2), CharacterId(2), SiteId(2), true),
        ])
        .expect("unique E01 claims"),
    }
}

fn commands() -> [Command; 2] {
    [
        Command::Gather(GatherCommand {
            actor: CharacterId(1),
            claim: ClaimId(1),
            site: SiteId(1),
        }),
        Command::Gather(GatherCommand {
            actor: CharacterId(2),
            claim: ClaimId(2),
            site: SiteId(2),
        }),
    ]
}

fn initial_beat(host: &mut Host) -> Beat {
    let publication = host.publication();
    Beat {
        kind: BeatKind::Initial,
        narrative: "Hrafn and Egil each believe they can manage one last gather. Their claims are witnessed. Only the world can answer.",
        belief_source: publication.clone(),
        publication,
        receipt: None,
    }
}

fn submit_action(
    host: &mut Host,
    index: usize,
    belief_source: &Publication,
) -> Result<Beat, String> {
    let (command, kind, narrative) = match index {
        0 => (
            commands()[0],
            BeatKind::HrafnRefused,
            "Hrafn expected success. The gather is refused: nothing moves and his strength remains.",
        ),
        1 => (
            commands()[1],
            BeatKind::EgilAccepted,
            "Egil held the same expectation. This time the gather is accepted: strength is spent and peat moves.",
        ),
        _ => return Err(format!("E01 has no action at trace index {index}")),
    };
    host.run_trial(std::slice::from_ref(&command));
    let receipt = host
        .receipt_log()
        .last()
        .cloned()
        .ok_or_else(|| "submitted E01 command produced no receipt".to_owned())?;
    Ok(Beat {
        kind,
        narrative,
        publication: host.publication(),
        belief_source: belief_source.clone(),
        receipt: Some(receipt),
    })
}

fn aftermath_beat(publication: Publication, belief_source: Publication) -> Beat {
    Beat {
        kind: BeatKind::Aftermath,
        narrative: "The same belief met two adjacent thresholds. Belief described perspective; receipts described the world.",
        publication,
        belief_source,
        receipt: None,
    }
}

fn build_trace() -> Result<Vec<Beat>, String> {
    let seeded = fixture();
    validate_world_coherence(&seeded).map_err(|fault| format!("E01 fixture fault: {fault:?}"))?;
    let mut host = Host::new(fixture);
    let initial = initial_beat(&mut host);
    let belief_source = initial.belief_source.clone();
    let refused = submit_action(&mut host, 0, &belief_source)?;
    let accepted = submit_action(&mut host, 1, &belief_source)?;
    let aftermath = aftermath_beat(accepted.publication.clone(), belief_source);
    let beats = vec![initial, refused, accepted, aftermath];
    validate_trace(&beats)?;
    Ok(beats)
}

fn validate_trace(beats: &[Beat]) -> Result<(), String> {
    if beats.len() != BeatKind::ALL.len() {
        return Err(format!("expected four E01 beats, got {}", beats.len()));
    }
    let facts: Vec<SceneFacts> = beats.iter().map(Beat::facts).collect::<Result<_, _>>()?;
    let initial = SceneFacts {
        hrafn_stamina: 14,
        hrafn_inventory_g: 0,
        egil_stamina: 15,
        egil_inventory_g: 0,
        hrafn_site_g: 2_000,
        egil_site_g: 2_000,
        hrafn_claim_witnessed: true,
        egil_claim_witnessed: true,
    };
    if facts[0] != initial || facts[1] != initial {
        return Err("start-14 refusal changed Publication facts".to_owned());
    }
    let accepted = SceneFacts {
        egil_stamina: 0,
        egil_inventory_g: 600,
        egil_site_g: 1_400,
        ..initial
    };
    if facts[2] != accepted || facts[3] != accepted {
        return Err("start-15 accepted facts differ from installed H-A".to_owned());
    }

    let refused = beats[1]
        .receipt
        .as_ref()
        .ok_or_else(|| "missing start-14 receipt".to_owned())?;
    if refused.outcome != OutcomeKind::Refused(RefusalReason::InsufficientStamina)
        || refused.stamina_spent != 0
        || refused.mass_moved != MassGrams::ZERO
        || refused.world_hash_before != refused.world_hash_after
    {
        return Err("start-14 receipt is not the required zero-mutation refusal".to_owned());
    }
    let accepted_receipt = beats[2]
        .receipt
        .as_ref()
        .ok_or_else(|| "missing start-15 receipt".to_owned())?;
    if accepted_receipt.outcome != OutcomeKind::Accepted
        || accepted_receipt.stamina_spent != 15
        || accepted_receipt.mass_moved != MassGrams::new(600)
    {
        return Err("start-15 receipt is not the required 15-for-600 acceptance".to_owned());
    }

    let beliefs = beats[0].beliefs()?;
    if beliefs
        != [
            BeliefOverlay {
                actor: 1,
                observed_stamina: 14,
                expects_success: true,
                observed_from: beats[0].publication.derived_from,
            },
            BeliefOverlay {
                actor: 2,
                observed_stamina: 15,
                expects_success: true,
                observed_from: beats[0].publication.derived_from,
            },
        ]
    {
        return Err("fixture beliefs do not expose the intended wrong/right pair".to_owned());
    }
    if beats.iter().any(|beat| {
        beat.belief_source.revisions != beats[0].publication.revisions
            || beat.belief_source.derived_from != beats[0].publication.derived_from
            || beat.belief_source.views != beats[0].publication.views
    }) {
        return Err(
            "belief overlay did not retain its identified pre-action Publication".to_owned(),
        );
    }
    if beats[0].publication.revisions != beats[1].publication.revisions
        || beats[0].publication.derived_from != beats[1].publication.derived_from
        || beats[0].publication.views != beats[1].publication.views
        || beats[2].publication.revisions <= beats[1].publication.revisions
        || beats[2].publication.derived_from == beats[1].publication.derived_from
        || beats[3].publication.revisions != beats[2].publication.revisions
        || beats[3].publication.derived_from != beats[2].publication.derived_from
    {
        return Err(
            "E01 Publication identity chain is not refusal-stable and apply-monotone".to_owned(),
        );
    }
    Ok(())
}

#[derive(Resource)]
struct InteractiveRun {
    host: Host,
    beats: Vec<Beat>,
    proof: bool,
}

impl InteractiveRun {
    fn new(proof: bool) -> Result<Self, String> {
        let seeded = fixture();
        validate_world_coherence(&seeded)
            .map_err(|fault| format!("E01 fixture fault: {fault:?}"))?;
        let mut host = Host::new(fixture);
        let initial = initial_beat(&mut host);
        Ok(Self {
            host,
            beats: vec![initial],
            proof,
        })
    }

    fn current(&self) -> &Beat {
        self.beats.last().expect("E01 run always has one beat")
    }

    fn advance(&mut self) -> Result<Option<Beat>, String> {
        let kind = self.current().kind;
        let publication = self.current().publication.clone();
        let belief_source = self.current().belief_source.clone();
        let next = match kind {
            BeatKind::Initial => submit_action(&mut self.host, 0, &belief_source)?,
            BeatKind::HrafnRefused => submit_action(&mut self.host, 1, &belief_source)?,
            BeatKind::EgilAccepted => aftermath_beat(publication, belief_source),
            BeatKind::Aftermath => return Ok(None),
        };
        self.beats.push(next.clone());
        if next.kind == BeatKind::Aftermath {
            validate_trace(&self.beats)?;
        }
        Ok(Some(next))
    }
}

#[derive(Resource)]
struct CaptureRun {
    beats: Vec<Beat>,
    output_dir: PathBuf,
    beat_index: usize,
    phase: CapturePhase,
    frames: u16,
    proof: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CapturePhase {
    Warmup,
    AwaitFile,
}

#[derive(Component)]
struct RenderedBeat;

fn add_render_plugins(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "E01 - Belief is not truth".to_owned(),
                    resolution: WindowResolution::new(WIDTH, HEIGHT),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                synchronous_pipeline_compilation: true,
                ..default()
            }),
    );
}

pub fn run_interactive(proof: bool) -> bool {
    let run = match InteractiveRun::new(proof) {
        Ok(run) => run,
        Err(error) => {
            eprintln!("e01_trace: {error}");
            return false;
        }
    };
    print_beat(run.current());
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb_u8(231, 234, 229)))
        .insert_resource(run);
    add_render_plugins(&mut app);
    app.add_systems(Startup, setup_interactive)
        .add_systems(Update, advance_interactive);
    app.run().is_success()
}

pub fn capture(output_dir: &Path, proof: bool) -> bool {
    let beats = match build_trace() {
        Ok(beats) => beats,
        Err(error) => {
            eprintln!("e01_trace: {error}");
            return false;
        }
    };
    if let Err(error) = fs::create_dir_all(output_dir) {
        eprintln!(
            "e01_render: cannot create {}: {error}",
            output_dir.display()
        );
        return false;
    }
    for beat in &beats {
        let path = output_dir.join(format!("{}.png", beat.kind.slug()));
        if let Err(error) = fs::remove_file(&path)
            && error.kind() != std::io::ErrorKind::NotFound
        {
            eprintln!(
                "e01_render: cannot remove stale {}: {error}",
                path.display()
            );
            return false;
        }
        print_beat(beat);
    }

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb_u8(231, 234, 229)))
        .insert_resource(CaptureRun {
            beats,
            output_dir: output_dir.to_path_buf(),
            beat_index: 0,
            phase: CapturePhase::Warmup,
            frames: 0,
            proof,
        });
    add_render_plugins(&mut app);
    app.add_systems(Startup, setup_capture)
        .add_systems(Update, capture_sequence);
    let exit = app.run();
    let complete = BeatKind::ALL
        .iter()
        .all(|kind| output_dir.join(format!("{}.png", kind.slug())).is_file());
    exit.is_success() && complete
}

fn print_beat(beat: &Beat) {
    let beliefs = beat.beliefs().expect("validated E01 Publication");
    println!(
        "e01_publication beat={} revisions={} derived_from=0x{:016x} beliefs=C{}:{}:{},C{}:{}:{}",
        beat.kind.slug(),
        beat.publication.revisions,
        beat.publication.derived_from,
        beliefs[0].actor,
        beliefs[0].observed_stamina,
        beliefs[0].expects_success,
        beliefs[1].actor,
        beliefs[1].observed_stamina,
        beliefs[1].expects_success,
    );
    if let Some(receipt) = &beat.receipt {
        println!("e01_receipt {}", receipt.canonical_line());
    }
}

fn setup_capture(mut commands: Commands, run: Res<CaptureRun>) {
    commands.spawn(Camera2d);
    spawn_beat(&mut commands, &run.beats[0], run.proof, None);
}

fn setup_interactive(mut commands: Commands, run: Res<InteractiveRun>) {
    commands.spawn(Camera2d);
    spawn_beat(
        &mut commands,
        run.current(),
        run.proof,
        Some(interaction_prompt(run.current().kind)),
    );
}

fn advance_interactive(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut run: ResMut<InteractiveRun>,
    rendered: Query<Entity, With<RenderedBeat>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::KeyQ) {
        exit.write(AppExit::Success);
        return;
    }
    if !keyboard.just_pressed(KeyCode::Space) && !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }
    let next = match run.advance() {
        Ok(Some(next)) => next,
        Ok(None) => return,
        Err(error) => {
            eprintln!("e01_interactive: {error}");
            exit.write(AppExit::error());
            return;
        }
    };
    print_beat(&next);
    for entity in &rendered {
        commands.entity(entity).despawn();
    }
    spawn_beat(
        &mut commands,
        &next,
        run.proof,
        Some(interaction_prompt(next.kind)),
    );
}

fn capture_sequence(
    mut commands: Commands,
    mut run: ResMut<CaptureRun>,
    rendered: Query<Entity, With<RenderedBeat>>,
    mut exit: MessageWriter<AppExit>,
) {
    run.frames = run.frames.saturating_add(1);
    match run.phase {
        CapturePhase::Warmup if run.frames >= WARMUP_FRAMES => {
            let path = run
                .output_dir
                .join(format!("{}.png", run.beats[run.beat_index].kind.slug()));
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path));
            run.phase = CapturePhase::AwaitFile;
            run.frames = 0;
        }
        CapturePhase::Warmup => {}
        CapturePhase::AwaitFile => {
            let beat = &run.beats[run.beat_index];
            let path = run.output_dir.join(format!("{}.png", beat.kind.slug()));
            if fs::metadata(&path).is_ok_and(|metadata| metadata.len() > 0) {
                println!(
                    "e01_capture beat={} path={}",
                    beat.kind.slug(),
                    path.display()
                );
                if run.beat_index + 1 == run.beats.len() {
                    exit.write(AppExit::Success);
                    return;
                }
                for entity in &rendered {
                    commands.entity(entity).despawn();
                }
                run.beat_index += 1;
                run.phase = CapturePhase::Warmup;
                run.frames = 0;
                let next = run.beats[run.beat_index].clone();
                spawn_beat(&mut commands, &next, run.proof, None);
            } else if run.frames >= CAPTURE_TIMEOUT_FRAMES {
                eprintln!("e01_capture: timed out waiting for {}", path.display());
                exit.write(AppExit::error());
            }
        }
    }
}

fn interaction_prompt(kind: BeatKind) -> &'static str {
    match kind {
        BeatKind::Initial => "SPACE / ENTER - let Hrafn try",
        BeatKind::HrafnRefused => "SPACE / ENTER - let Egil try",
        BeatKind::EgilAccepted => "SPACE / ENTER - compare belief and world",
        BeatKind::Aftermath => "ESC / Q - close",
    }
}

fn spawn_beat(commands: &mut Commands, beat: &Beat, proof: bool, prompt: Option<&str>) {
    let facts = beat.facts().expect("validated E01 Publication");
    let beliefs = beat.beliefs().expect("validated E01 beliefs");
    rect(
        commands,
        Vec2::new(0.0, 45.0),
        Vec2::new(1200.0, 525.0),
        Color::srgb_u8(245, 244, 238),
        0.0,
    );
    text(
        commands,
        beat.kind.heading(),
        Vec2::new(0.0, 374.0),
        16.0,
        Color::srgb_u8(102, 105, 96),
        Vec2::new(1120.0, 28.0),
        Justify::Center,
        5.0,
    );
    text(
        commands,
        beat.narrative,
        Vec2::new(0.0, 325.0),
        22.0,
        Color::srgb_u8(39, 43, 38),
        Vec2::new(1080.0, 72.0),
        Justify::Center,
        5.0,
    );

    draw_actor_panel(
        commands,
        "Hrafn",
        -300.0,
        facts.hrafn_stamina,
        facts.hrafn_inventory_g,
        facts.hrafn_site_g,
        facts.hrafn_claim_witnessed,
        beliefs[0],
        Color::srgb_u8(151, 94, 59),
    );
    draw_actor_panel(
        commands,
        "Egil",
        300.0,
        facts.egil_stamina,
        facts.egil_inventory_g,
        facts.egil_site_g,
        facts.egil_claim_witnessed,
        beliefs[1],
        Color::srgb_u8(69, 101, 83),
    );

    if let Some(receipt) = &beat.receipt {
        draw_outcome(commands, receipt);
    } else if beat.kind == BeatKind::Aftermath {
        draw_aftermath(commands);
    }
    if proof {
        draw_proof(commands, beat);
    }
    if let Some(prompt) = prompt {
        text(
            commands,
            prompt,
            Vec2::new(0.0, -362.0),
            14.0,
            Color::srgb_u8(64, 69, 62),
            Vec2::new(1160.0, 24.0),
            Justify::Center,
            8.0,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_actor_panel(
    commands: &mut Commands,
    name: &str,
    x: f32,
    stamina: u8,
    inventory_g: u64,
    site_g: u64,
    witnessed: bool,
    belief: BeliefOverlay,
    accent: Color,
) {
    rect(
        commands,
        Vec2::new(x, 62.0),
        Vec2::new(520.0, 390.0),
        Color::WHITE,
        1.0,
    );
    text(
        commands,
        name,
        Vec2::new(x, 224.0),
        27.0,
        Color::srgb_u8(39, 43, 38),
        Vec2::new(440.0, 36.0),
        Justify::Center,
        4.0,
    );
    text(
        commands,
        format!("STAMINA {stamina}"),
        Vec2::new(x, 183.0),
        16.0,
        accent,
        Vec2::new(300.0, 26.0),
        Justify::Center,
        4.0,
    );
    rect(
        commands,
        Vec2::new(x, 157.0),
        Vec2::new(240.0, 13.0),
        Color::srgb_u8(218, 221, 214),
        2.0,
    );
    let fill = f32::from(stamina) / 20.0 * 240.0;
    rect(
        commands,
        Vec2::new(x - (240.0 - fill) / 2.0, 157.0),
        Vec2::new(fill, 13.0),
        accent,
        3.0,
    );
    rect(
        commands,
        Vec2::new(x, 100.0),
        Vec2::new(450.0, 76.0),
        Color::srgb_u8(244, 237, 218),
        2.0,
    );
    text(
        commands,
        format!(
            "{name} THOUGHT AT STAMINA {}\n\"{BELIEF_LINE}\"",
            belief.observed_stamina,
        ),
        Vec2::new(x, 100.0),
        15.0,
        Color::srgb_u8(105, 78, 26),
        Vec2::new(420.0, 62.0),
        Justify::Center,
        4.0,
    );
    text(
        commands,
        if witnessed {
            "CLAIM WITNESSED"
        } else {
            "CLAIM UNWITNESSED"
        },
        Vec2::new(x, 47.0),
        13.0,
        Color::srgb_u8(92, 96, 88),
        Vec2::new(300.0, 22.0),
        Justify::Center,
        4.0,
    );
    text(
        commands,
        format!("SITE  {site_g} g"),
        Vec2::new(x - 120.0, 14.0),
        13.0,
        Color::srgb_u8(92, 96, 88),
        Vec2::new(220.0, 22.0),
        Justify::Center,
        4.0,
    );
    text(
        commands,
        format!("STACK  {inventory_g} g"),
        Vec2::new(x + 120.0, 14.0),
        13.0,
        Color::srgb_u8(92, 96, 88),
        Vec2::new(220.0, 22.0),
        Justify::Center,
        4.0,
    );
    draw_blocks(commands, x - 120.0, -40.0, site_g, false);
    draw_blocks(commands, x + 120.0, -40.0, inventory_g, true);
}

fn draw_blocks(commands: &mut Commands, x: f32, y: f32, grams: u64, empty_is_clear: bool) {
    let blocks = usize::try_from(grams / BLOCK_GRAMS)
        .unwrap_or(usize::MAX)
        .min(10);
    for index in 0..10 {
        let column = index % 5;
        let row = index / 5;
        let color = if index < blocks {
            Color::srgb_u8(103, 88, 62)
        } else if empty_is_clear {
            Color::srgba_u8(103, 88, 62, 12)
        } else {
            Color::srgba_u8(103, 88, 62, 38)
        };
        rect(
            commands,
            Vec2::new(x - 72.0 + column as f32 * 36.0, y - row as f32 * 27.0),
            Vec2::new(30.0, 20.0),
            color,
            3.0,
        );
    }
}

fn draw_outcome(commands: &mut Commands, receipt: &Receipt) {
    let (line, color) = match receipt.outcome {
        OutcomeKind::Refused(RefusalReason::InsufficientStamina) => (
            "WHAT HAPPENED: REFUSED — not enough strength — nothing spent — no peat moved",
            Color::srgb_u8(151, 74, 61),
        ),
        OutcomeKind::Accepted => (
            "WHAT HAPPENED: ACCEPTED — 15 strength spent — 600 g peat moved",
            Color::srgb_u8(69, 110, 79),
        ),
        _ => (
            "WHAT HAPPENED: unexpected outcome",
            Color::srgb_u8(57, 62, 56),
        ),
    };
    rect(
        commands,
        Vec2::new(0.0, -192.0),
        Vec2::new(1080.0, 46.0),
        color,
        5.0,
    );
    text(
        commands,
        line,
        Vec2::new(0.0, -192.0),
        16.0,
        Color::WHITE,
        Vec2::new(1040.0, 32.0),
        Justify::Center,
        6.0,
    );
}

fn draw_aftermath(commands: &mut Commands) {
    rect(
        commands,
        Vec2::new(0.0, -192.0),
        Vec2::new(1080.0, 72.0),
        Color::srgb_u8(62, 69, 63),
        5.0,
    );
    text(
        commands,
        "HRAFN: believed yes / action failed     EGIL: believed yes / action succeeded\nThe same world answered both; belief did not decide.",
        Vec2::new(0.0, -192.0),
        15.0,
        Color::WHITE,
        Vec2::new(1030.0, 58.0),
        Justify::Center,
        6.0,
    );
}

fn draw_proof(commands: &mut Commands, beat: &Beat) {
    let receipt = beat
        .receipt
        .as_ref()
        .map_or_else(|| "receipt=-".to_owned(), Receipt::canonical_line);
    rect(
        commands,
        Vec2::new(0.0, -282.0),
        Vec2::new(1120.0, 96.0),
        Color::srgba_u8(255, 255, 255, 244),
        5.0,
    );
    text(
        commands,
        format!(
            "PROOF — Publication revisions={} derived_from=0x{:016x} — belief_from=0x{:016x}\n{}\n{}",
            beat.publication.revisions,
            beat.publication.derived_from,
            beat.belief_source.derived_from,
            receipt,
            POLICY_FOOTER,
        ),
        Vec2::new(0.0, -282.0),
        10.5,
        Color::srgb_u8(39, 43, 38),
        Vec2::new(1080.0, 86.0),
        Justify::Left,
        6.0,
    );
}

fn rect(commands: &mut Commands, position: Vec2, size: Vec2, color: Color, z: f32) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(position.extend(z)),
        RenderedBeat,
    ));
}

#[allow(clippy::too_many_arguments)]
fn text(
    commands: &mut Commands,
    value: impl Into<String>,
    position: Vec2,
    size: f32,
    color: Color,
    bounds: Vec2,
    justify: Justify,
    z: f32,
) {
    commands.spawn((
        Text2d::new(value),
        TextFont::from_font_size(size),
        TextColor(color),
        TextLayout::new(justify, LineBreak::WordBoundary),
        TextBounds::from(bounds),
        Transform::from_translation(position.extend(z)),
        RenderedBeat,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_makes_one_wrong_and_one_matching_belief_legible() {
        let beats = build_trace().expect("E01 trace validates");
        assert_eq!(
            beats.iter().map(|beat| beat.kind).collect::<Vec<_>>(),
            BeatKind::ALL,
        );
        let beliefs = beats[0].beliefs().expect("beliefs are Publication-fed");
        assert!(beliefs.iter().all(|belief| belief.expects_success));
        assert_eq!(beliefs.map(|belief| belief.observed_stamina), [14, 15]);
        assert_eq!(
            beats[1].receipt.as_ref().expect("refusal").outcome,
            OutcomeKind::Refused(RefusalReason::InsufficientStamina),
        );
        assert_eq!(
            beats[2].receipt.as_ref().expect("acceptance").outcome,
            OutcomeKind::Accepted,
        );
    }

    #[test]
    fn belief_reads_cannot_change_receipts_or_publications() {
        fn signature(read_beliefs: bool) -> (Vec<String>, u64, u64, Vec<String>) {
            let mut host = Host::new(fixture);
            let initial = host.publication();
            if read_beliefs {
                let _ = belief_from(&initial, 1).expect("C1 belief");
                let _ = belief_from(&initial, 2).expect("C2 belief");
            }
            let mut latest = initial;
            for command in commands() {
                host.run_trial(std::slice::from_ref(&command));
                latest = host.publication();
                if read_beliefs {
                    let _ = belief_from(&latest, 1).expect("C1 belief");
                    let _ = belief_from(&latest, 2).expect("C2 belief");
                }
            }
            (
                host.receipts(),
                latest.revisions,
                latest.derived_from,
                latest.views,
            )
        }
        assert_eq!(signature(false), signature(true));
    }

    #[test]
    fn primary_path_submits_one_command_per_player_advance() {
        let mut run = InteractiveRun::new(false).expect("E01 fixture is coherent");
        assert!(run.host.receipt_log().is_empty());
        let refused = run.advance().expect("advance").expect("refused beat");
        assert_eq!(refused.kind, BeatKind::HrafnRefused);
        assert_eq!(run.host.receipt_log().len(), 1);
        let accepted = run.advance().expect("advance").expect("accepted beat");
        assert_eq!(accepted.kind, BeatKind::EgilAccepted);
        assert_eq!(run.host.receipt_log().len(), 2);
        let aftermath = run.advance().expect("advance").expect("aftermath");
        assert_eq!(aftermath.kind, BeatKind::Aftermath);
        assert_eq!(run.host.receipt_log().len(), 2);
        assert!(run.advance().expect("completed run is stable").is_none());
    }

    #[test]
    fn fixture_excludes_the_sealed_holdout_shape() {
        let mut host = Host::new(fixture);
        let publication = host.publication();
        let hrafn = publication.character(1).expect("C1");
        let egil = publication.character(2).expect("C2");
        assert_eq!([hrafn.stamina, egil.stamina], [14, 15]);
        assert_eq!(publication.site(1).expect("S1").tier, "established");
        assert_eq!(publication.site(2).expect("S2").tier, "established");
        assert!(publication.character(7).is_none());
        assert!(publication.site(7).is_none());
        assert!(publication.claim(7).is_none());
    }

    #[test]
    fn renderer_has_no_canonical_observation_backdoor() {
        let source = include_str!("e01_taste.rs");
        for forbidden in [
            format!("truth_{}(", "state"),
            format!("truth_{}(", "hash"),
            format!("canonical_{}(", "state"),
        ] {
            assert_eq!(
                source.matches(&forbidden).count(),
                0,
                "E01 must consume Publications, not canonical observations: {forbidden}",
            );
        }
    }

    #[test]
    fn capability_is_off_by_default_and_capture_names_are_stable() {
        let manifest = include_str!("../Cargo.toml");
        assert!(manifest.contains("default = []"));
        assert!(manifest.contains("e01-taste = [\"bevy-render\"]"));
        assert!(!manifest.contains("default = [\"e01-taste\"]"));
        assert_eq!(
            BeatKind::ALL.map(BeatKind::slug),
            [
                "00-two-beliefs",
                "01-belief-wrong",
                "02-belief-matched",
                "03-belief-is-not-truth",
            ],
        );
    }
}
