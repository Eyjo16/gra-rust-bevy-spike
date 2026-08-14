//! RS01 renderer: publication-to-expression (trial RS01).
//!
//! Two layers, one doctrine. The pure layer derives an
//! `ExpressionScene` — every visible fact of the game moment — from a
//! live `Publication` and its canonical receipt, deterministically and
//! from nothing else. The Bevy layer (feature `rs01-render`) only draws
//! that scene and forwards player input as commands through the real
//! boundary; it holds no truth, decides no legality, and can be deleted
//! without any canonical evidence changing.
//!
//! Expression policy (RS01 envelope): cold atmosphere is mood, not
//! canonical cold pressure. The default view shows no raw IDs, hashes,
//! receipt lines or exact numbers — those live behind the Sönnun
//! overlay. Display names are presentation aliases only.

use crate::boundary::{OutcomeKind, RefusalReason, Verb};
use crate::host_bevy::Publication;
use crate::rs01_fixture::{RS01_HOLDER, RS01_WITNESS, Rs01Beat, TURF_BLOCK_GRAMS};

/// Presentation aliases. Plain personal names: they imply no unmodeled
/// class, office, kinship, duty or relationship.
pub const HOLDER_ALIAS: &str = "Auðun";
pub const WITNESS_ALIAS: &str = "Vigdís";

/// One portrait's visible state. The bar fraction uses one stable scale
/// for the whole trace: points out of 100 (envelope falsifier F5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortraitExpression {
    pub alias: &'static str,
    pub stamina_points: u8,
}

impl PortraitExpression {
    /// Stable-scale bar fill in hundredths (0..=100).
    pub fn bar_hundredths(&self) -> u8 {
        self.stamina_points
    }
}

/// The claim seal is a boolean gate rendered as a boolean seal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SealExpression {
    Unwitnessed,
    Witnessed,
}

/// The beat's emphasis cue, derived only from the receipt's type and
/// typed deltas. Timing downstream is a fixed frame count per cue kind,
/// so emphasis is deterministic (envelope: emphasis and timing derive
/// from receipt type and typed delta).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeatCue {
    /// Beat 0: no command yet.
    Opening,
    /// A refusal: nothing changed. The emphasis target names which part
    /// of the scene the closed reason concerns.
    RefusedHalt { reason: RefusalReason },
    /// A witness acceptance: the seal changed; stamina was spent.
    SealChanged { spent: u8 },
    /// A gather acceptance: equal-unit blocks moved; stamina was spent.
    MassMoved { blocks: u8, spent: u8 },
}

impl BeatCue {
    /// Where the emphasis lands, from the closed reason vocabulary.
    /// Total over the vocabulary, so any refusal renders somewhere
    /// sensible without inventing meaning.
    pub fn emphasis_target(&self) -> EmphasisTarget {
        match self {
            Self::Opening => EmphasisTarget::None,
            Self::SealChanged { .. } => EmphasisTarget::Seal,
            Self::MassMoved { .. } => EmphasisTarget::Site,
            Self::RefusedHalt { reason } => match reason {
                RefusalReason::UnknownClaim
                | RefusalReason::ClaimNotHeldByActor
                | RefusalReason::ClaimSiteMismatch
                | RefusalReason::ClaimNotWitnessed
                | RefusalReason::ClaimAlreadyWitnessed
                | RefusalReason::CannotWitnessOwnClaim => EmphasisTarget::Seal,
                RefusalReason::UnknownActor
                | RefusalReason::ActorExhausted
                | RefusalReason::InsufficientStamina => EmphasisTarget::Portraits,
                RefusalReason::UnknownSite | RefusalReason::SiteEmpty => EmphasisTarget::Site,
            },
        }
    }

    /// Factual Icelandic cue line — a direct rendering of the receipt's
    /// disposition, not atmosphere. Closed mapping over the vocabulary.
    pub fn cue_line(&self) -> &'static str {
        match self {
            Self::Opening => "",
            Self::SealChanged { .. } => "Vitnað: krafan er nú vottuð",
            Self::MassMoved { .. } => "Torf sótt á skikanum og lagt heim í stæðu",
            Self::RefusedHalt { reason } => match reason {
                RefusalReason::ClaimNotWitnessed => "Stöðvað: krafan er óvottuð",
                RefusalReason::ClaimAlreadyWitnessed => "Stöðvað: krafan er þegar vottuð",
                RefusalReason::CannotWitnessOwnClaim => {
                    "Stöðvað: enginn vottar eigin kröfu"
                }
                RefusalReason::ClaimNotHeldByActor => "Stöðvað: krafan er annars manns",
                RefusalReason::ClaimSiteMismatch => "Stöðvað: krafan nær ekki yfir þennan skika",
                RefusalReason::UnknownClaim => "Stöðvað: engin slík krafa",
                RefusalReason::ActorExhausted => "Stöðvað: örmagna",
                RefusalReason::InsufficientStamina => "Stöðvað: þrekið dugar ekki",
                RefusalReason::UnknownActor => "Stöðvað: enginn slíkur maður",
                RefusalReason::UnknownSite => "Stöðvað: enginn slíkur skiki",
                RefusalReason::SiteEmpty => "Stöðvað: skikinn er uppurinn",
            },
        }
    }

    /// Fixed emphasis duration in frames per cue kind — deterministic
    /// timing, derived from receipt type only.
    pub fn emphasis_frames(&self) -> u32 {
        match self {
            Self::Opening => 0,
            Self::RefusedHalt { .. } => 90,
            Self::SealChanged { .. } => 75,
            Self::MassMoved { .. } => 75,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmphasisTarget {
    None,
    Seal,
    Portraits,
    Site,
}

/// Everything the default view is allowed to show for one beat, derived
/// deterministically from the beat's live publication and receipt.
/// Every field is a canonical fact or a deterministic derived
/// expression; the fact map (docs/rs01-fact-map.md) classifies each.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionScene {
    pub beat: usize,
    /// Publication identity the whole scene derives from (Sönnun only).
    pub revisions: u64,
    pub derived_from: u64,
    pub holder: PortraitExpression,
    pub witness: PortraitExpression,
    pub seal: SealExpression,
    /// Equal projection units of 200 g each, exact division enforced.
    pub site_blocks: u8,
    pub home_blocks: u8,
    pub cue: BeatCue,
    /// Concise factual Icelandic aftermath, emitted only when the facts
    /// themselves close the arc (claim witnessed and mass home).
    pub aftermath: Option<&'static str>,
}

fn published_character(publication: &Publication, id: u64) -> (u8, u64) {
    publication
        .facts
        .characters
        .iter()
        .find(|c| c.id == id)
        .map(|c| (c.stamina, c.inventory_g))
        .unwrap_or((0, 0))
}

fn blocks(grams: u64) -> u8 {
    debug_assert_eq!(grams % TURF_BLOCK_GRAMS, 0, "equal-unit blocks require exact division");
    u8::try_from(grams / TURF_BLOCK_GRAMS).unwrap_or(u8::MAX)
}

/// Derive the beat's cue from its receipt alone.
fn derive_cue(beat: &Rs01Beat) -> BeatCue {
    match &beat.receipt {
        None => BeatCue::Opening,
        Some(receipt) => match receipt.outcome {
            OutcomeKind::Refused(reason) => BeatCue::RefusedHalt { reason },
            OutcomeKind::Accepted | OutcomeKind::Partial(_) => match receipt.verb {
                Verb::Witness => BeatCue::SealChanged {
                    spent: receipt.stamina_spent,
                },
                Verb::Gather => BeatCue::MassMoved {
                    blocks: blocks(receipt.mass_moved.grams()),
                    spent: receipt.stamina_spent,
                },
            },
        },
    }
}

/// The one aftermath sentence the facts can currently support. Factual
/// Icelandic, no numerals, no need/danger/duty/emotion (envelope F2).
const AFTERMATH_LINE: &str =
    "Krafan er nú vottuð. Torf var sótt á skikann og lagt heim í stæðu. Verkið kostaði þrek beggja.";

/// Derive the whole visible scene from one beat's live publication and
/// receipt. Pure and deterministic: same publication and receipt, same
/// scene — falsifier F9's expression half.
pub fn derive_scene(beat: &Rs01Beat) -> ExpressionScene {
    let publication = &beat.publication;
    let (holder_stamina, holder_inventory) = published_character(publication, RS01_HOLDER.0);
    let (witness_stamina, _) = published_character(publication, RS01_WITNESS.0);
    let witnessed = publication
        .facts
        .claims
        .first()
        .is_some_and(|c| c.witnessed);
    let site_stock = publication
        .facts
        .sites
        .first()
        .map(|s| s.stock_g)
        .unwrap_or(0);
    let seal = if witnessed {
        SealExpression::Witnessed
    } else {
        SealExpression::Unwitnessed
    };
    let home_blocks = blocks(holder_inventory);
    let aftermath =
        (witnessed && home_blocks > 0).then_some(AFTERMATH_LINE);
    ExpressionScene {
        beat: beat.index,
        revisions: publication.revisions,
        derived_from: publication.derived_from,
        holder: PortraitExpression {
            alias: HOLDER_ALIAS,
            stamina_points: holder_stamina,
        },
        witness: PortraitExpression {
            alias: WITNESS_ALIAS,
            stamina_points: witness_stamina,
        },
        seal,
        site_blocks: blocks(site_stock),
        home_blocks,
        cue: derive_cue(beat),
        aftermath,
    }
}

#[cfg(test)]
mod expression_tests {
    use super::*;
    use crate::rs01_fixture::run_rs01_trace;

    /// F5 — quantitative expression: bars use one stable /100 scale and
    /// blocks are exact 200 g units across the whole corrected trace.
    #[test]
    fn expression_quantities_match_canonical_facts_exactly() {
        let (beats, _host) = run_rs01_trace();
        let scenes: Vec<ExpressionScene> = beats.iter().map(derive_scene).collect();

        // Beat 0: 65/100 bars, 10 site blocks, empty home, open seal.
        assert_eq!(scenes[0].holder.bar_hundredths(), 65);
        assert_eq!(scenes[0].witness.bar_hundredths(), 65);
        assert_eq!(scenes[0].site_blocks, 10);
        assert_eq!(scenes[0].home_blocks, 0);
        assert_eq!(scenes[0].seal, SealExpression::Unwitnessed);
        assert_eq!(scenes[0].aftermath, None);

        // Beat 1: refused — nothing visible may change except the cue.
        assert_eq!(
            scenes[1].cue,
            BeatCue::RefusedHalt {
                reason: crate::boundary::RefusalReason::ClaimNotWitnessed
            }
        );
        assert_eq!(scenes[1].cue.emphasis_target(), EmphasisTarget::Seal);
        assert_eq!(scenes[1].holder, scenes[0].holder);
        assert_eq!(scenes[1].witness, scenes[0].witness);
        assert_eq!(scenes[1].site_blocks, 10);
        assert_eq!(scenes[1].seal, SealExpression::Unwitnessed);

        // Beat 2: witness — seal flips, witness bar 60, no blocks move.
        assert_eq!(scenes[2].cue, BeatCue::SealChanged { spent: 5 });
        assert_eq!(scenes[2].witness.bar_hundredths(), 60);
        assert_eq!(scenes[2].holder.bar_hundredths(), 65);
        assert_eq!(scenes[2].seal, SealExpression::Witnessed);
        assert_eq!(scenes[2].site_blocks, 10);
        assert_eq!(scenes[2].home_blocks, 0);

        // Beat 3: gather — six 200 g blocks move, holder bar 53,
        // 2000→800 g means 10→4 site blocks (60% removed).
        assert_eq!(scenes[3].cue, BeatCue::MassMoved { blocks: 6, spent: 12 });
        assert_eq!(scenes[3].holder.bar_hundredths(), 53);
        assert_eq!(scenes[3].witness.bar_hundredths(), 60);
        assert_eq!(scenes[3].site_blocks, 4);
        assert_eq!(scenes[3].home_blocks, 6);
        assert_eq!(scenes[3].site_blocks + scenes[3].home_blocks, 10);
        assert_eq!(scenes[3].aftermath, Some(AFTERMATH_LINE));
    }

    /// F9 — expression determinism: two independent trace runs derive
    /// byte-identical scenes.
    #[test]
    fn expression_states_are_deterministic_across_runs() {
        let (beats_a, _) = run_rs01_trace();
        let (beats_b, _) = run_rs01_trace();
        let scenes_a: Vec<ExpressionScene> = beats_a.iter().map(derive_scene).collect();
        let scenes_b: Vec<ExpressionScene> = beats_b.iter().map(derive_scene).collect();
        assert_eq!(scenes_a, scenes_b);
    }

    /// The cue vocabulary is total over the closed refusal reasons: every
    /// reason has a cue line and an emphasis target — no refusal can
    /// reach the renderer without a lawful rendering.
    #[test]
    fn cue_mapping_is_total_over_closed_reasons() {
        for reason in crate::boundary::RefusalReason::ALL {
            let cue = BeatCue::RefusedHalt { reason };
            assert!(!cue.cue_line().is_empty());
            let _ = cue.emphasis_target();
            assert!(cue.emphasis_frames() > 0);
        }
    }
}

// ---------------------------------------------------------------------------
// Bevy renderer app (feature rs01-render)
// ---------------------------------------------------------------------------

#[cfg(feature = "rs01-render")]
pub use app::run;

#[cfg(feature = "rs01-render")]
mod app {
    use std::process::ExitCode;

    use bevy::prelude::*;
    use bevy::render::view::screenshot::{Screenshot, save_to_disk};
    use bevy::sprite::Anchor;

    use super::{
        BeatCue, EmphasisTarget, ExpressionScene, SealExpression, derive_scene,
    };
    use crate::boundary::{Command, Receipt};
    use crate::host_bevy::{Host, Publication};
    use crate::rs01_fixture::{Rs01Beat, rs01_commands, rs01_fixture};

    // Cold-atmosphere palette: presentation-only mood, not canonical
    // cold pressure (envelope expression policy).
    const SKY: Color = Color::srgb(0.847, 0.882, 0.910);
    const GROUND: Color = Color::srgb(0.906, 0.925, 0.937);
    const INK: Color = Color::srgb(0.165, 0.204, 0.224);
    const TURF: Color = Color::srgb(0.408, 0.310, 0.220);
    const TURF_EDGE: Color = Color::srgb(0.290, 0.220, 0.157);
    const BAR_BG: Color = Color::srgb(0.322, 0.369, 0.396);
    const BAR_FG: Color = Color::srgb(0.290, 0.490, 0.349);
    const SEAL_OPEN: Color = Color::srgb(0.557, 0.600, 0.624);
    const SEAL_WITNESSED: Color = Color::srgb(0.726, 0.541, 0.180);
    const HALT: Color = Color::srgb(0.706, 0.271, 0.220);
    const PANEL: Color = Color::srgba(0.086, 0.110, 0.125, 0.94);
    const PANEL_TEXT: Color = Color::srgb(0.878, 0.902, 0.914);

    /// The host, the accepted publication, and the live beat log — one
    /// resource, custodied by the render app but mutated only through
    /// `submit_player_command`, which routes through the real boundary.
    #[derive(Resource)]
    struct Rs01State {
        host: Host,
        beats: Vec<Rs01Beat>,
        /// Newest accepted publication ordering (stale rejection, F7).
        last_revisions: u64,
        scene: ExpressionScene,
        scene_dirty: bool,
        emphasis_frames_left: u32,
        overlay_open: bool,
    }

    impl Rs01State {
        fn new() -> Self {
            let mut host = Host::new(rs01_fixture);
            let publication = host.publication();
            let beat = Rs01Beat {
                index: 0,
                command: None,
                receipt: None,
                publication,
            };
            let scene = derive_scene(&beat);
            let last_revisions = beat.publication.revisions;
            Self {
                host,
                beats: vec![beat],
                last_revisions,
                scene,
                scene_dirty: true,
                emphasis_frames_left: 0,
                overlay_open: false,
            }
        }

        /// Stale rejection at the consumer seam (F7): only a publication
        /// at least as new as the newest accepted may replace the scene.
        fn accept(&mut self, publication: &Publication) -> bool {
            if publication.revisions < self.last_revisions {
                return false;
            }
            self.last_revisions = publication.revisions;
            true
        }

        /// The one submission path — the same for player keys and the
        /// capture driver: through the real boundary, then publish,
        /// then accept-or-reject, then rederive the scene.
        fn submit_player_command(&mut self, cmd: Command) {
            self.host.run_trial(std::slice::from_ref(&cmd));
            let receipt = self
                .host
                .receipt_log()
                .last()
                .expect("submitted command produced a receipt")
                .clone();
            let publication = self.host.publication();
            let index = self.beats.len();
            let beat = Rs01Beat {
                index,
                command: Some(cmd),
                receipt: Some(receipt),
                publication,
            };
            if self.accept(&beat.publication) {
                self.scene = derive_scene(&beat);
                self.emphasis_frames_left = self.scene.cue.emphasis_frames();
                self.scene_dirty = true;
            }
            self.beats.push(beat);
        }

        /// Replay from fixture (envelope flow): a fresh host, same
        /// fixture, empty log — canonical truth is rebuilt, the old
        /// scene is discarded in full.
        fn replay(&mut self) {
            *self = Self::new();
        }
    }

    /// Marker: everything that is rebuilt whenever the scene changes.
    #[derive(Component)]
    struct SceneNode;

    /// Marker: the Sönnun overlay root.
    #[derive(Component)]
    struct OverlayNode;

    /// Capture driver (headless evidence runs): drives the same
    /// submission path as the player keys at fixed frames and saves
    /// screenshots. Recorded substitution in docs/rs01-trial-log.md —
    /// the primary test path remains the player's own three
    /// submissions.
    #[derive(Resource)]
    struct Capture {
        dir: String,
        frame: u32,
    }

    pub fn run() -> ExitCode {
        let mut app = App::new();
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Grágás — RS01".to_owned(),
                    resolution: (1280., 720.).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .insert_resource(ClearColor(SKY))
        .insert_resource(Rs01State::new())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (player_input, capture_driver, rebuild_scene, tick_emphasis).chain(),
        );
        if let Ok(dir) = std::env::var("RS01_CAPTURE") {
            app.insert_resource(Capture { dir, frame: 0 });
        }
        app.run();
        ExitCode::SUCCESS
    }

    fn setup(mut commands: Commands) {
        commands.spawn(Camera2d);
        // Static stage: ground plane and key hints (presentation only).
        commands.spawn((
            Sprite::from_color(GROUND, Vec2::new(1280., 260.)),
            Transform::from_xyz(0., -230., -10.),
        ));
        commands.spawn((
            Text2d::new("G safna torfi   ·   V vitna kröfu   ·   Tab sönnun   ·   R endurspila"),
            TextFont {
                font_size: 20.,
                ..Default::default()
            },
            TextColor(INK),
            Transform::from_xyz(0., -330., 10.),
        ));
    }

    fn player_input(
        keys: Res<ButtonInput<KeyCode>>,
        mut state: ResMut<Rs01State>,
        mut exit: EventWriter<AppExit>,
    ) {
        let commands = rs01_commands();
        if keys.just_pressed(KeyCode::KeyG) {
            state.submit_player_command(commands[0]);
        }
        if keys.just_pressed(KeyCode::KeyV) {
            state.submit_player_command(commands[1]);
        }
        if keys.just_pressed(KeyCode::KeyR) {
            state.replay();
        }
        if keys.just_pressed(KeyCode::Tab) {
            state.overlay_open = !state.overlay_open;
            state.scene_dirty = true;
        }
        if keys.just_pressed(KeyCode::Escape) {
            exit.write(AppExit::Success);
        }
    }

    /// Fixed-frame script for headless capture runs; every command goes
    /// through the identical `submit_player_command` path.
    fn capture_driver(
        capture: Option<ResMut<Capture>>,
        mut state: ResMut<Rs01State>,
        mut commands: Commands,
        mut exit: EventWriter<AppExit>,
    ) {
        let Some(mut capture) = capture else {
            return;
        };
        capture.frame += 1;
        let dir = capture.dir.clone();
        let shot = |commands: &mut Commands, name: &str| {
            let path = format!("{dir}/{name}");
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        };
        let cmds = rs01_commands();
        match capture.frame {
            40 => shot(&mut commands, "01-initial.png"),
            60 => state.submit_player_command(cmds[0]),
            100 => shot(&mut commands, "02-refused.png"),
            120 => state.submit_player_command(cmds[1]),
            160 => shot(&mut commands, "03-witnessed.png"),
            180 => state.submit_player_command(cmds[2]),
            220 => shot(&mut commands, "04-gathered.png"),
            // The aftermath is the settled final scene (emphasis faded).
            300 => shot(&mut commands, "05-aftermath.png"),
            320 => {
                state.overlay_open = true;
                state.scene_dirty = true;
            }
            360 => shot(&mut commands, "06-proof-overlay.png"),
            420 => {
                exit.write(AppExit::Success);
            }
            _ => {}
        }
    }

    fn tick_emphasis(mut state: ResMut<Rs01State>) {
        if state.emphasis_frames_left > 0 {
            state.emphasis_frames_left -= 1;
            if state.emphasis_frames_left == 0 {
                state.scene_dirty = true;
            }
        }
    }

    /// Despawn-and-respawn projection of the current `ExpressionScene`.
    /// Disposable by construction: the entities carry copied facts only.
    fn rebuild_scene(
        mut commands: Commands,
        mut state: ResMut<Rs01State>,
        stale: Query<Entity, Or<(With<SceneNode>, With<OverlayNode>)>>,
    ) {
        if !state.scene_dirty {
            return;
        }
        state.scene_dirty = false;
        for entity in &stale {
            commands.entity(entity).despawn();
        }
        let scene = &state.scene;
        let emphasized = state.emphasis_frames_left > 0;

        spawn_site(&mut commands, scene, emphasized);
        spawn_home(&mut commands, scene);
        spawn_portraits(&mut commands, scene, emphasized);
        spawn_seal(&mut commands, scene, emphasized);
        spawn_cue(&mut commands, scene, emphasized);
        if let Some(aftermath) = scene.aftermath {
            commands.spawn((
                SceneNode,
                Text2d::new(aftermath),
                TextFont {
                    font_size: 22.,
                    ..Default::default()
                },
                TextColor(INK),
                Transform::from_xyz(0., -280., 12.),
            ));
        }
        if state.overlay_open {
            spawn_overlay(&mut commands, &state);
        }
    }

    fn block_positions(count: u8, origin: Vec2) -> impl Iterator<Item = Vec2> {
        (0..count).map(move |i| {
            let column = i % 2;
            let row = i / 2;
            origin + Vec2::new(f32::from(column) * 78., f32::from(row) * 40.)
        })
    }

    fn spawn_site(commands: &mut Commands, scene: &ExpressionScene, emphasized: bool) {
        commands.spawn((
            SceneNode,
            Text2d::new("skikinn"),
            TextFont {
                font_size: 20.,
                ..Default::default()
            },
            TextColor(INK),
            Transform::from_xyz(-420., -140., 5.),
        ));
        let highlight = emphasized && scene.cue.emphasis_target() == EmphasisTarget::Site;
        for position in block_positions(scene.site_blocks, Vec2::new(-460., -90.)) {
            commands.spawn((
                SceneNode,
                Sprite::from_color(TURF_EDGE, Vec2::new(74., 38.)),
                Transform::from_xyz(position.x, position.y, 1.),
            ));
            commands.spawn((
                SceneNode,
                Sprite::from_color(
                    if highlight { SEAL_WITNESSED } else { TURF },
                    Vec2::new(66., 30.),
                ),
                Transform::from_xyz(position.x, position.y, 2.),
            ));
        }
    }

    fn spawn_home(commands: &mut Commands, scene: &ExpressionScene) {
        commands.spawn((
            SceneNode,
            Text2d::new("heimastæðan"),
            TextFont {
                font_size: 20.,
                ..Default::default()
            },
            TextColor(INK),
            Transform::from_xyz(420., -140., 5.),
        ));
        commands.spawn((
            SceneNode,
            Sprite::from_color(BAR_BG, Vec2::new(190., 12.)),
            Transform::from_xyz(420., -116., 1.),
        ));
        for position in block_positions(scene.home_blocks, Vec2::new(380., -90.)) {
            commands.spawn((
                SceneNode,
                Sprite::from_color(TURF_EDGE, Vec2::new(74., 38.)),
                Transform::from_xyz(position.x, position.y, 1.),
            ));
            commands.spawn((
                SceneNode,
                Sprite::from_color(TURF, Vec2::new(66., 30.)),
                Transform::from_xyz(position.x, position.y, 2.),
            ));
        }
    }

    fn spawn_portrait(
        commands: &mut Commands,
        alias: &'static str,
        initial: &str,
        bar_hundredths: u8,
        x: f32,
        dimmed: bool,
    ) {
        let tile = if dimmed { BAR_BG } else { INK };
        commands.spawn((
            SceneNode,
            Sprite::from_color(tile, Vec2::new(64., 64.)),
            Transform::from_xyz(x, 40., 3.),
        ));
        commands.spawn((
            SceneNode,
            Text2d::new(initial),
            TextFont {
                font_size: 34.,
                ..Default::default()
            },
            TextColor(SKY),
            Transform::from_xyz(x, 40., 4.),
        ));
        commands.spawn((
            SceneNode,
            Text2d::new(alias),
            TextFont {
                font_size: 20.,
                ..Default::default()
            },
            TextColor(INK),
            Transform::from_xyz(x, -8., 4.),
        ));
        // Unlabeled stamina bar, stable /100 scale for the whole trace.
        commands.spawn((
            SceneNode,
            Sprite::from_color(BAR_BG, Vec2::new(104., 12.)),
            Transform::from_xyz(x, -34., 3.),
        ));
        let width = f32::from(bar_hundredths);
        commands.spawn((
            SceneNode,
            Sprite {
                color: BAR_FG,
                custom_size: Some(Vec2::new(width, 8.)),
                anchor: Anchor::CenterLeft,
                ..Default::default()
            },
            Transform::from_xyz(x - 50., -34., 4.),
        ));
    }

    fn spawn_portraits(commands: &mut Commands, scene: &ExpressionScene, emphasized: bool) {
        let dim = emphasized && scene.cue.emphasis_target() == EmphasisTarget::Portraits;
        spawn_portrait(
            commands,
            scene.holder.alias,
            "A",
            scene.holder.bar_hundredths(),
            -150.,
            dim,
        );
        spawn_portrait(
            commands,
            scene.witness.alias,
            "V",
            scene.witness.bar_hundredths(),
            150.,
            dim,
        );
    }

    fn spawn_seal(commands: &mut Commands, scene: &ExpressionScene, emphasized: bool) {
        let (fill, label) = match scene.seal {
            SealExpression::Unwitnessed => (SEAL_OPEN, "óvottuð krafa"),
            SealExpression::Witnessed => (SEAL_WITNESSED, "vottuð krafa"),
        };
        let halt = emphasized && scene.cue.emphasis_target() == EmphasisTarget::Seal;
        if halt {
            let ring = if matches!(scene.cue, BeatCue::RefusedHalt { .. }) {
                HALT
            } else {
                SEAL_WITNESSED
            };
            commands.spawn((
                SceneNode,
                Sprite::from_color(ring, Vec2::new(92., 92.)),
                Transform::from_xyz(0., 160., 2.)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
            ));
        }
        commands.spawn((
            SceneNode,
            Sprite::from_color(INK, Vec2::new(74., 74.)),
            Transform::from_xyz(0., 160., 3.)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
        ));
        commands.spawn((
            SceneNode,
            Sprite::from_color(fill, Vec2::new(58., 58.)),
            Transform::from_xyz(0., 160., 4.)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
        ));
        commands.spawn((
            SceneNode,
            Text2d::new(label),
            TextFont {
                font_size: 20.,
                ..Default::default()
            },
            TextColor(INK),
            Transform::from_xyz(0., 100., 4.),
        ));
    }

    fn spawn_cue(commands: &mut Commands, scene: &ExpressionScene, emphasized: bool) {
        let line = scene.cue.cue_line();
        if line.is_empty() || !emphasized {
            return;
        }
        let underline = if matches!(scene.cue, BeatCue::RefusedHalt { .. }) {
            HALT
        } else {
            SEAL_WITNESSED
        };
        commands.spawn((
            SceneNode,
            Text2d::new(line),
            TextFont {
                font_size: 26.,
                ..Default::default()
            },
            TextColor(INK),
            Transform::from_xyz(0., 300., 11.),
        ));
        commands.spawn((
            SceneNode,
            Sprite::from_color(underline, Vec2::new(420., 4.)),
            Transform::from_xyz(0., 282., 11.),
        ));
    }

    /// The Sönnun overlay: per beat, the canonical command, publication
    /// identity before/after, disposition, typed befores/afters and the
    /// receipt line. It explains the game view; the game view never
    /// depends on it.
    fn spawn_overlay(commands: &mut Commands, state: &Rs01State) {
        commands.spawn((
            OverlayNode,
            Sprite::from_color(PANEL, Vec2::new(1180., 620.)),
            Transform::from_xyz(0., 0., 50.),
        ));
        let mut lines: Vec<String> = vec![format!(
            "SÖNNUN — grammar og publication identity á hverju beat (beats: {})",
            state.beats.len()
        )];
        let mut previous: Option<&Rs01Beat> = None;
        for beat in &state.beats {
            let publication = &beat.publication;
            let before = previous
                .map(|p| format!("0x{:016x}", p.publication.derived_from))
                .unwrap_or_else(|| "upphaf".to_owned());
            match (&beat.command, &beat.receipt) {
                (None, _) => lines.push(format!(
                    "beat 0  upphafsútgáfa  revisions={} derived=0x{:016x}",
                    publication.revisions, publication.derived_from
                )),
                (Some(_), Some(receipt)) => {
                    lines.push(format!(
                        "beat {}  {} -> 0x{:016x}  revisions={}",
                        beat.index, before, publication.derived_from, publication.revisions
                    ));
                    lines.push(format!("        {}", receipt.canonical_line()));
                }
                (Some(_), None) => {}
            }
            previous = Some(beat);
        }
        let text = lines.join("\n");
        commands.spawn((
            OverlayNode,
            Text2d::new(text),
            TextFont {
                font_size: 13.,
                ..Default::default()
            },
            TextColor(PANEL_TEXT),
            Transform::from_xyz(0., 0., 51.),
        ));
        let _ = Receipt::canonical_line; // overlay uses canonical lines above
    }
}
