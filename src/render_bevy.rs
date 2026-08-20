//! RS01 live Bevy render proof.
//!
//! The renderer owns no gameplay logic and reads no canonical truth. It
//! executes the three-command Hvammur trace through `Host`, then renders only
//! typed facts copied from identified `Publication`s plus the canonical
//! receipts produced by those submissions. Names, material, palette, layout,
//! weather frame, pacing, and the 200 g-per-block visual scale are explicitly
//! presentation policy rather than truth.

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
        RefusalReason, ResourceKind, SiteId, Stamina, WitnessCommand, World as TruthWorld,
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
/// The RS01 scene's site yields building material. Under the licensed
/// RES01 vocabulary the nearest kind is `timber`; the scene's own visual
/// vocabulary still says "turf", which is a naming tension recorded for
/// the author in the RES01 report, not resolved here — RS01's frozen
/// visual-fact map is evidence and is not edited by a later trial.
const SCENE_KIND: ResourceKind = ResourceKind::Timber;
const WARMUP_FRAMES: u16 = 12;
const CAPTURE_TIMEOUT_FRAMES: u16 = 600;
const POLICY_FOOTER: &str = "Presentation policy: Snorri - Thordur - Hvammur - peat - autumn morning - equal block scale - palette and layout. No danger or emotion is asserted.";
const AFTERMATH_COST: &str =
    "COST\n\n- Snorri spent stamina\n- Thordur spent stamina\n- Peat left the bog";
const AFTERMATH_GAIN: &str = "GAIN\n\n- Peat now stands at Snorri's stack\n- Snorri's claim is witnessed\n- Material moved; none appeared";
#[cfg(test)]
const DEFAULT_VISUAL_FACT_IDS: [&str; 20] = [
    "frame.background",
    "frame.beat_heading",
    "narrative.initial",
    "narrative.refusal",
    "narrative.witness",
    "narrative.gather",
    "narrative.aftermath",
    "actor.aliases",
    "actor.silhouettes",
    "layout.positions",
    "actor.stamina_bars",
    "site.alias_and_material",
    "site.witness_seal",
    "site.turf_blocks",
    "inventory.turf_blocks",
    "outcome.banner",
    "aftermath.cost",
    "aftermath.gain",
    "interaction.prompt",
    "palette.state_colors",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BeatKind {
    Initial,
    Refused,
    Witnessed,
    Gathered,
    Aftermath,
}

impl BeatKind {
    const fn slug(self) -> &'static str {
        match self {
            Self::Initial => "00-initial",
            Self::Refused => "01-refused",
            Self::Witnessed => "02-witnessed",
            Self::Gathered => "03-gathered",
            Self::Aftermath => "04-aftermath",
        }
    }

    const fn heading(self) -> &'static str {
        match self {
            Self::Initial => "INITIAL",
            Self::Refused => "REFUSED",
            Self::Witnessed => "WITNESSED",
            Self::Gathered => "GATHERED",
            Self::Aftermath => "AFTERMATH",
        }
    }
}

#[derive(Clone)]
struct Beat {
    kind: BeatKind,
    narrative: &'static str,
    publication: Publication,
    receipt: Option<Receipt>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SceneFacts {
    snorri_stamina: u8,
    snorri_inventory_g: u64,
    thordur_stamina: u8,
    site_stock_g: u64,
    claim_witnessed: bool,
}

impl Beat {
    fn facts(&self) -> Result<SceneFacts, String> {
        let snorri = self
            .publication
            .character(1)
            .ok_or_else(|| "publication missing character C1".to_owned())?;
        let thordur = self
            .publication
            .character(2)
            .ok_or_else(|| "publication missing character C2".to_owned())?;
        let site = self
            .publication
            .site(1)
            .ok_or_else(|| "publication missing site S1".to_owned())?;
        let claim = self
            .publication
            .claim(1)
            .ok_or_else(|| "publication missing claim K1".to_owned())?;
        if site.tier != "established" {
            return Err(format!("publication site S1 tier drifted: {}", site.tier));
        }
        // RES01: the scene draws one material. If the published site's
        // kind is not the one this scene was built for, the drawing
        // would be true of a different world — refuse instead of
        // rendering a plausible lie.
        if site.kind != SCENE_KIND.code() {
            return Err(format!("publication site S1 kind drifted: {}", site.kind));
        }
        if claim.holder != 1 || claim.site != 1 {
            return Err("publication claim K1 no longer binds C1 to S1".to_owned());
        }
        for identity in [
            snorri.derived_from,
            thordur.derived_from,
            site.derived_from,
            claim.derived_from,
        ] {
            if identity != self.publication.derived_from {
                return Err("typed view fact identity differs from publication".to_owned());
            }
        }
        Ok(SceneFacts {
            snorri_stamina: snorri.stamina,
            snorri_inventory_g: snorri.holding_g(SCENE_KIND),
            thordur_stamina: thordur.stamina,
            site_stock_g: site.stock_g,
            claim_witnessed: claim.witnessed,
        })
    }
}

fn fixture() -> TruthWorld {
    TruthWorld {
        characters: CharacterOwner::seed([
            (CharacterId(1), Stamina::new(60).expect("fixture stamina")),
            (CharacterId(2), Stamina::new(30).expect("fixture stamina")),
        ])
        .expect("unique fixture characters"),
        economy: EconomyOwner::seed_sites([(
            SiteId(1),
            InfraTier::Established,
            SCENE_KIND,
            MassGrams::new(2_000),
        )])
        .expect("unique fixture site"),
        social: SocialOwner::seed_claims([(ClaimId(1), CharacterId(1), SiteId(1), false)])
            .expect("unique fixture claim"),
    }
}

fn commands() -> [Command; 3] {
    [
        Command::Gather(GatherCommand {
            actor: CharacterId(1),
            claim: ClaimId(1),
            site: SiteId(1),
        }),
        Command::Witness(WitnessCommand {
            witness: CharacterId(2),
            claim: ClaimId(1),
        }),
        Command::Gather(GatherCommand {
            actor: CharacterId(1),
            claim: ClaimId(1),
            site: SiteId(1),
        }),
    ]
}

fn action_spec(index: usize) -> Option<(Command, BeatKind, &'static str)> {
    let commands = commands();
    match index {
        0 => Some((
            commands[0],
            BeatKind::Refused,
            "Snorri attempts to gather peat. The attempt is refused: the claim is not witnessed. The world does not change.",
        )),
        1 => Some((
            commands[1],
            BeatKind::Witnessed,
            "Thordur witnesses the claim. Snorri's claim opens, paid for with Thordur's own stamina.",
        )),
        2 => Some((
            commands[2],
            BeatKind::Gathered,
            "Snorri tries again. Peat moves from the bog to his stack, and his stamina falls.",
        )),
        _ => None,
    }
}

fn initial_beat(host: &mut Host) -> Beat {
    Beat {
        kind: BeatKind::Initial,
        narrative: "A cold autumn morning at Hvammur. Snorri's claim on the peat bog is unwitnessed; the bog remains closed.",
        publication: host.publication(),
        receipt: None,
    }
}

fn submit_action(host: &mut Host, index: usize) -> Result<Beat, String> {
    let (command, kind, narrative) =
        action_spec(index).ok_or_else(|| format!("RS01 has no action at trace index {index}"))?;
    host.run_trial(std::slice::from_ref(&command));
    let receipt = host
        .receipt_log()
        .last()
        .cloned()
        .ok_or_else(|| "submitted command produced no receipt".to_owned())?;
    Ok(Beat {
        kind,
        narrative,
        publication: host.publication(),
        receipt: Some(receipt),
    })
}

fn aftermath_beat(publication: Publication) -> Beat {
    Beat {
        kind: BeatKind::Aftermath,
        narrative: "Cost and gain now stand side by side.",
        publication,
        receipt: None,
    }
}

fn build_trace() -> Result<Vec<Beat>, String> {
    let seeded = fixture();
    validate_world_coherence(&seeded).map_err(|fault| format!("fixture fault: {fault:?}"))?;
    let mut host = Host::new(fixture);
    let initial = initial_beat(&mut host);
    let mut action_beats = Vec::with_capacity(3);
    for index in 0..3 {
        action_beats.push(submit_action(&mut host, index)?);
    }
    let gathered = action_beats
        .last()
        .expect("three action beats")
        .publication
        .clone();
    let mut beats = vec![initial];
    beats.extend(action_beats);
    beats.push(aftermath_beat(gathered));
    validate_trace(&beats)?;
    Ok(beats)
}

fn validate_trace(beats: &[Beat]) -> Result<(), String> {
    if beats.len() != 5 {
        return Err(format!("expected five render beats, got {}", beats.len()));
    }
    let facts: Vec<SceneFacts> = beats.iter().map(Beat::facts).collect::<Result<_, _>>()?;
    let initial = SceneFacts {
        snorri_stamina: 60,
        snorri_inventory_g: 0,
        thordur_stamina: 30,
        site_stock_g: 2_000,
        claim_witnessed: false,
    };
    if facts[0] != initial || facts[1] != initial {
        return Err("refused gather did not preserve the initial publication".to_owned());
    }
    if facts[2]
        != (SceneFacts {
            thordur_stamina: 25,
            claim_witnessed: true,
            ..initial
        })
    {
        return Err("witness publication facts differ from the boundary trace".to_owned());
    }
    let gathered = SceneFacts {
        snorri_stamina: 48,
        snorri_inventory_g: 1_200,
        thordur_stamina: 25,
        site_stock_g: 800,
        claim_witnessed: true,
    };
    if facts[3] != gathered || facts[4] != gathered {
        return Err("gathered/aftermath publication facts drifted".to_owned());
    }
    let refused = beats[1].receipt.as_ref().expect("refused receipt");
    if refused.outcome != OutcomeKind::Refused(RefusalReason::ClaimNotWitnessed)
        || refused.stamina_spent != 0
        || refused.mass_moved != MassGrams::ZERO
        || refused.world_hash_before != refused.world_hash_after
    {
        return Err("first receipt is not the required zero-mutation refusal".to_owned());
    }
    let witnessed = beats[2].receipt.as_ref().expect("witness receipt");
    if witnessed.outcome != OutcomeKind::Accepted
        || witnessed.stamina_spent != 5
        || witnessed.mass_moved != MassGrams::ZERO
    {
        return Err("second receipt is not the required witness transition".to_owned());
    }
    let gathered_receipt = beats[3].receipt.as_ref().expect("gather receipt");
    if gathered_receipt.outcome != OutcomeKind::Accepted
        || gathered_receipt.stamina_spent != 12
        || gathered_receipt.mass_moved != MassGrams::new(1_200)
    {
        return Err("third receipt is not the required gather transition".to_owned());
    }
    if beats[0].publication.revisions != beats[1].publication.revisions
        || beats[0].publication.derived_from != beats[1].publication.derived_from
        || beats[2].publication.revisions <= beats[1].publication.revisions
        || beats[3].publication.revisions <= beats[2].publication.revisions
        || beats[4].publication.derived_from != beats[3].publication.derived_from
    {
        return Err(
            "publication identity chain is not refusal-stable and apply-monotone".to_owned(),
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
        validate_world_coherence(&seeded).map_err(|fault| format!("fixture fault: {fault:?}"))?;
        let mut host = Host::new(fixture);
        let initial = initial_beat(&mut host);
        Ok(Self {
            host,
            beats: vec![initial],
            proof,
        })
    }

    fn current(&self) -> &Beat {
        self.beats
            .last()
            .expect("interactive run always has a beat")
    }

    fn advance(&mut self) -> Result<Option<Beat>, String> {
        let next = match self.current().kind {
            BeatKind::Initial => submit_action(&mut self.host, 0)?,
            BeatKind::Refused => submit_action(&mut self.host, 1)?,
            BeatKind::Witnessed => submit_action(&mut self.host, 2)?,
            BeatKind::Gathered => aftermath_beat(self.current().publication.clone()),
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
                    title: "RS01 - Witness at Hvammur".to_owned(),
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

/// Primary RS01 path: the window starts from the initial Publication and
/// submits exactly one canonical command per player advance input. Nothing is
/// pre-submitted; Space or Enter drives the three-command trace.
pub fn run_interactive(proof: bool) -> bool {
    let run = match InteractiveRun::new(proof) {
        Ok(run) => run,
        Err(error) => {
            eprintln!("rs01_trace: {error}");
            return false;
        }
    };
    print_beat(run.current());
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb_u8(232, 237, 241)))
        .insert_resource(run);
    add_render_plugins(&mut app);
    app.add_systems(Startup, setup_interactive)
        .add_systems(Update, advance_interactive);
    app.run().is_success()
}

/// Mechanical evidence path: replays the same live boundary trace, captures
/// each publication beat, and exits. It is intentionally a separate command
/// so automated capture cannot masquerade as the player-driven primary path.
pub fn capture(output_dir: &Path, proof: bool) -> bool {
    let beats = match build_trace() {
        Ok(beats) => beats,
        Err(error) => {
            eprintln!("rs01_trace: {error}");
            return false;
        }
    };
    if let Err(error) = fs::create_dir_all(output_dir) {
        eprintln!(
            "rs01_render: cannot create {}: {error}",
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
                "rs01_render: cannot remove stale {}: {error}",
                path.display()
            );
            return false;
        }
        print_beat(beat);
    }

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb_u8(232, 237, 241)))
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
    println!(
        "rs01_publication beat={} revisions={} derived_from=0x{:016x}",
        beat.kind.slug(),
        beat.publication.revisions,
        beat.publication.derived_from,
    );
    if let Some(receipt) = &beat.receipt {
        println!("rs01_receipt {}", receipt.canonical_line());
    }
}

impl BeatKind {
    const ALL: [Self; 5] = [
        Self::Initial,
        Self::Refused,
        Self::Witnessed,
        Self::Gathered,
        Self::Aftermath,
    ];
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
    let advance = keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter);
    if !advance {
        return;
    }
    let next = match run.advance() {
        Ok(Some(next)) => next,
        Ok(None) => return,
        Err(error) => {
            eprintln!("rs01_interactive: {error}");
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
            let captured = fs::metadata(&path).is_ok_and(|metadata| metadata.len() > 0);
            if captured {
                println!(
                    "rs01_capture beat={} path={}",
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
                eprintln!("rs01_capture: timed out waiting for {}", path.display());
                exit.write(AppExit::error());
            }
        }
    }
}

fn interaction_prompt(kind: BeatKind) -> &'static str {
    match kind {
        BeatKind::Initial => "SPACE / ENTER - attempt to gather peat",
        BeatKind::Refused => "SPACE / ENTER - ask Thordur to witness",
        BeatKind::Witnessed => "SPACE / ENTER - gather again",
        BeatKind::Gathered => "SPACE / ENTER - view the aftermath",
        BeatKind::Aftermath => "ESC / Q - close",
    }
}

fn spawn_beat(commands: &mut Commands, beat: &Beat, proof: bool, prompt: Option<&str>) {
    let facts = beat.facts().expect("trace was validated before app start");
    rect(
        commands,
        Vec2::new(0.0, 65.0),
        Vec2::new(1180.0, 410.0),
        Color::srgb_u8(244, 246, 248),
        0.0,
    );
    rect(
        commands,
        Vec2::new(0.0, -85.0),
        Vec2::new(1180.0, 110.0),
        Color::srgb_u8(221, 227, 231),
        0.5,
    );
    text(
        commands,
        beat.kind.heading(),
        Vec2::new(0.0, 374.0),
        16.0,
        Color::srgb_u8(107, 118, 129),
        Vec2::new(1120.0, 28.0),
        Justify::Center,
        5.0,
    );
    text(
        commands,
        beat.narrative,
        Vec2::new(0.0, 325.0),
        24.0,
        Color::srgb_u8(35, 42, 48),
        Vec2::new(1050.0, 78.0),
        Justify::Center,
        5.0,
    );

    draw_site(commands, facts);
    let snorri_x = if beat.kind == BeatKind::Initial {
        -430.0
    } else {
        -180.0
    };
    let thordur_x = if matches!(
        beat.kind,
        BeatKind::Witnessed | BeatKind::Gathered | BeatKind::Aftermath
    ) {
        -410.0
    } else {
        -90.0
    };
    draw_person(
        commands,
        "Snorri",
        snorri_x,
        facts.snorri_stamina,
        Color::srgb_u8(58, 68, 77),
    );
    draw_person(
        commands,
        "Thordur",
        thordur_x,
        facts.thordur_stamina,
        Color::srgb_u8(79, 89, 96),
    );
    if facts.snorri_inventory_g > 0 {
        draw_inventory(commands, facts.snorri_inventory_g);
    }
    if let Some(receipt) = &beat.receipt {
        draw_outcome(commands, receipt);
    }
    if beat.kind == BeatKind::Aftermath {
        draw_aftermath(commands);
    }
    if proof {
        draw_proof(commands, beat, facts);
        text(
            commands,
            POLICY_FOOTER,
            Vec2::new(0.0, -380.0),
            12.0,
            Color::srgb_u8(107, 118, 129),
            Vec2::new(1160.0, 34.0),
            Justify::Center,
            5.0,
        );
    }
    if let Some(prompt) = prompt {
        text(
            commands,
            prompt,
            Vec2::new(0.0, -348.0),
            14.0,
            Color::srgb_u8(58, 68, 77),
            Vec2::new(1160.0, 24.0),
            Justify::Center,
            5.0,
        );
    }
}

fn draw_site(commands: &mut Commands, facts: SceneFacts) {
    let gate_color = if facts.claim_witnessed {
        Color::srgb_u8(138, 109, 26)
    } else {
        Color::srgb_u8(156, 74, 58)
    };
    rect(
        commands,
        Vec2::new(360.0, 185.0),
        Vec2::new(128.0, 30.0),
        gate_color,
        2.0,
    );
    text(
        commands,
        if facts.claim_witnessed {
            "WITNESSED"
        } else {
            "UNWITNESSED"
        },
        Vec2::new(360.0, 185.0),
        15.0,
        Color::WHITE,
        Vec2::new(120.0, 24.0),
        Justify::Center,
        3.0,
    );
    text(
        commands,
        "Peat bog at Hvammur",
        Vec2::new(360.0, 147.0),
        20.0,
        Color::srgb_u8(35, 42, 48),
        Vec2::new(300.0, 30.0),
        Justify::Center,
        3.0,
    );
    let alive = usize::try_from(facts.site_stock_g / BLOCK_GRAMS)
        .unwrap_or(usize::MAX)
        .min(10);
    for index in 0..10 {
        let column = index % 5;
        let row = index / 5;
        let color = if index < alive {
            Color::srgb_u8(107, 93, 67)
        } else {
            Color::srgba_u8(107, 93, 67, 36)
        };
        rect(
            commands,
            Vec2::new(270.0 + column as f32 * 45.0, 93.0 - row as f32 * 31.0),
            Vec2::new(38.0, 23.0),
            color,
            2.0,
        );
    }
}

fn draw_person(commands: &mut Commands, name: &str, x: f32, stamina: u8, color: Color) {
    rect(
        commands,
        Vec2::new(x, 28.0),
        Vec2::new(42.0, 102.0),
        color,
        2.0,
    );
    circle(commands, Vec2::new(x, 96.0), 32.0, color, 2.0);
    text(
        commands,
        name,
        Vec2::new(x, -39.0),
        19.0,
        Color::srgb_u8(35, 42, 48),
        Vec2::new(150.0, 26.0),
        Justify::Center,
        3.0,
    );
    rect(
        commands,
        Vec2::new(x, -68.0),
        Vec2::new(112.0, 12.0),
        Color::srgb_u8(207, 214, 219),
        2.0,
    );
    let fill = f32::from(stamina) * 1.12;
    rect(
        commands,
        Vec2::new(x - (112.0 - fill) / 2.0, -68.0),
        Vec2::new(fill, 12.0),
        if stamina <= 30 {
            Color::srgb_u8(163, 103, 42)
        } else {
            Color::srgb_u8(77, 107, 77)
        },
        3.0,
    );
    text(
        commands,
        "stamina",
        Vec2::new(x, -89.0),
        13.0,
        Color::srgb_u8(107, 118, 129),
        Vec2::new(100.0, 20.0),
        Justify::Center,
        3.0,
    );
}

fn draw_inventory(commands: &mut Commands, inventory_g: u64) {
    rect(
        commands,
        Vec2::new(75.0, -30.0),
        Vec2::new(205.0, 118.0),
        Color::WHITE,
        1.5,
    );
    text(
        commands,
        "Snorri's stack",
        Vec2::new(75.0, 11.0),
        16.0,
        Color::srgb_u8(35, 42, 48),
        Vec2::new(180.0, 24.0),
        Justify::Center,
        3.0,
    );
    let blocks = usize::try_from(inventory_g / BLOCK_GRAMS)
        .unwrap_or(usize::MAX)
        .min(12);
    for index in 0..blocks {
        let column = index % 3;
        let row = index / 3;
        rect(
            commands,
            Vec2::new(25.0 + column as f32 * 50.0, -24.0 - row as f32 * 24.0),
            Vec2::new(42.0, 17.0),
            Color::srgb_u8(107, 93, 67),
            2.0,
        );
    }
}

fn draw_outcome(commands: &mut Commands, receipt: &Receipt) {
    let (label, color) = match receipt.outcome {
        OutcomeKind::Refused(RefusalReason::ClaimNotWitnessed) => (
            "REFUSED - claim unwitnessed - world unchanged",
            Color::srgb_u8(156, 74, 58),
        ),
        OutcomeKind::Accepted if receipt.verb.code() == "witness" => (
            "ACCEPTED - witness opens the claim",
            Color::srgb_u8(138, 109, 26),
        ),
        OutcomeKind::Accepted => ("ACCEPTED - peat moves", Color::srgb_u8(77, 107, 77)),
        _ => ("KVITTUN", Color::srgb_u8(58, 68, 77)),
    };
    rect(
        commands,
        Vec2::new(0.0, 244.0),
        Vec2::new(520.0, 34.0),
        color,
        2.0,
    );
    text(
        commands,
        label,
        Vec2::new(0.0, 244.0),
        15.0,
        Color::WHITE,
        Vec2::new(500.0, 28.0),
        Justify::Center,
        3.0,
    );
}

fn draw_aftermath(commands: &mut Commands) {
    rect(
        commands,
        Vec2::new(-292.0, -250.0),
        Vec2::new(560.0, 165.0),
        Color::WHITE,
        1.0,
    );
    rect(
        commands,
        Vec2::new(292.0, -250.0),
        Vec2::new(560.0, 165.0),
        Color::WHITE,
        1.0,
    );
    text(
        commands,
        AFTERMATH_COST,
        Vec2::new(-292.0, -250.0),
        16.0,
        Color::srgb_u8(163, 103, 42),
        Vec2::new(510.0, 140.0),
        Justify::Left,
        4.0,
    );
    text(
        commands,
        AFTERMATH_GAIN,
        Vec2::new(292.0, -250.0),
        16.0,
        Color::srgb_u8(77, 107, 77),
        Vec2::new(510.0, 140.0),
        Justify::Left,
        4.0,
    );
}

fn draw_proof(commands: &mut Commands, beat: &Beat, facts: SceneFacts) {
    rect(
        commands,
        Vec2::new(0.0, 255.0),
        Vec2::new(1160.0, 104.0),
        Color::srgba_u8(255, 255, 255, 244),
        20.0,
    );
    let receipt = beat
        .receipt
        .as_ref()
        .map_or_else(|| "receipt=-".to_owned(), Receipt::canonical_line);
    text(
        commands,
        format!(
            "PROOF - Publication revisions={} derived_from=0x{:016x} - block_scale={}g\nC1 stamina={} stack={}g - C2 stamina={} - S1={}g - K1 witnessed={}\n{}",
            beat.publication.revisions,
            beat.publication.derived_from,
            BLOCK_GRAMS,
            facts.snorri_stamina,
            facts.snorri_inventory_g,
            facts.thordur_stamina,
            facts.site_stock_g,
            facts.claim_witnessed,
            receipt,
        ),
        Vec2::new(0.0, 255.0),
        11.0,
        Color::srgb_u8(35, 42, 48),
        Vec2::new(1120.0, 92.0),
        Justify::Left,
        21.0,
    );
}

fn rect(commands: &mut Commands, position: Vec2, size: Vec2, color: Color, z: f32) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(position.extend(z)),
        RenderedBeat,
    ));
}

fn circle(commands: &mut Commands, position: Vec2, diameter: f32, color: Color, z: f32) {
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(diameter)),
            rect: Some(Rect::from_center_size(Vec2::ZERO, Vec2::splat(diameter))),
            ..default()
        },
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
    fn rs01_trace_is_publication_identified_and_receipt_derived() {
        let beats = build_trace().expect("live boundary trace validates");
        assert_eq!(
            beats.iter().map(|beat| beat.kind).collect::<Vec<_>>(),
            BeatKind::ALL
        );
        assert_eq!(beats[0].facts().unwrap(), beats[1].facts().unwrap());
        assert_eq!(beats[3].facts().unwrap(), beats[4].facts().unwrap());
        assert!(beats.iter().all(|beat| !beat.publication.views.is_empty()));
    }

    #[test]
    fn renderer_has_no_canonical_observation_backdoor() {
        let source = include_str!("render_bevy.rs");
        for forbidden in [
            format!("truth_{}(", "state"),
            format!("truth_{}(", "hash"),
            format!("canonical_{}(", "state"),
        ] {
            assert_eq!(
                source.matches(&forbidden).count(),
                0,
                "renderer must consume Publication facts, not canonical observations: {forbidden}"
            );
        }
    }

    #[test]
    fn capture_names_are_stable_and_complete() {
        assert_eq!(
            BeatKind::ALL.map(BeatKind::slug),
            [
                "00-initial",
                "01-refused",
                "02-witnessed",
                "03-gathered",
                "04-aftermath",
            ]
        );
    }

    #[test]
    fn primary_path_submits_exactly_one_command_per_player_advance() {
        let mut run = InteractiveRun::new(false).expect("interactive fixture is coherent");
        assert_eq!(run.current().kind, BeatKind::Initial);
        assert!(run.host.receipt_log().is_empty());

        for (receipt_count, kind) in [
            (1, BeatKind::Refused),
            (2, BeatKind::Witnessed),
            (3, BeatKind::Gathered),
        ] {
            let next = run
                .advance()
                .expect("advance validates")
                .expect("next beat");
            assert_eq!(next.kind, kind);
            assert_eq!(run.host.receipt_log().len(), receipt_count);
        }

        let aftermath = run
            .advance()
            .expect("aftermath validates")
            .expect("aftermath");
        assert_eq!(aftermath.kind, BeatKind::Aftermath);
        assert_eq!(run.host.receipt_log().len(), 3);
        assert!(run.advance().expect("completed run is stable").is_none());
        assert_eq!(run.host.receipt_log().len(), 3);
    }

    #[test]
    fn replay_expression_states_are_deterministic() {
        let signature = |beats: Vec<Beat>| {
            beats
                .into_iter()
                .map(|beat| {
                    (
                        beat.kind,
                        beat.publication.revisions,
                        beat.publication.derived_from,
                        beat.facts().expect("published facts"),
                        beat.receipt.map(|receipt| receipt.canonical_line()),
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            signature(build_trace().expect("first trace")),
            signature(build_trace().expect("second trace")),
        );
    }

    #[test]
    fn renderer_is_deletable_and_off_by_default() {
        let manifest = include_str!("../Cargo.toml");
        assert!(manifest.contains("default = []"));
        assert!(manifest.contains("bevy-host = [\"dep:bevy_ecs\"]"));
        assert!(manifest.contains("bevy-render = [\"bevy-host\", \"dep:bevy\"]"));
        assert!(!manifest.contains("default = [\"bevy-render\"]"));
    }

    #[test]
    fn every_default_visual_has_exactly_one_fact_map_row() {
        let fact_map = include_str!("../docs/rs01-visual-fact-map.md");
        for id in DEFAULT_VISUAL_FACT_IDS {
            let marker = format!("| `{id}` |");
            assert_eq!(
                fact_map.matches(&marker).count(),
                1,
                "default visual must have exactly one fact-map row: {id}"
            );
        }
    }

    #[test]
    fn default_copy_omits_ledger_and_exact_quantities() {
        let beats = build_trace().expect("live boundary trace validates");
        let mut copy = beats
            .iter()
            .flat_map(|beat| {
                [
                    beat.kind.heading(),
                    beat.narrative,
                    interaction_prompt(beat.kind),
                ]
            })
            .collect::<Vec<_>>();
        copy.extend([AFTERMATH_COST, AFTERMATH_GAIN]);

        for text in copy {
            assert!(
                !text.chars().any(|character| character.is_ascii_digit()),
                "default copy must not display exact numbers: {text}"
            );
            let lower = text.to_ascii_lowercase();
            for ledger_term in [
                " gram",
                "receipt",
                "revision",
                "derived_from",
                "hash=",
                "publication",
                "boundary",
                "presentation policy",
            ] {
                assert!(
                    !lower.contains(ledger_term),
                    "default copy must not display proof-ledger terms: {text}"
                );
            }
        }
    }
}
