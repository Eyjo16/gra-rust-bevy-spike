# RS01 — live Bevy publication render

Status: **GREEN IMPLEMENTATION CANDIDATE; human gate and independent
ratification pending**. Author-dispatched 2026-08-14. Base `fca5237`.

## Envelope as run

```text
base_commit:         fca5237
objective:           open a real Bevy 2D window, require the player to submit
                     gather/refuse → witness → gather through the existing
                     boundary/Host seam, and capture five reproducible frames
                     whose facts come from identified live Publications and
                     canonical receipts
authoritative_files: AGENTS.md; docs/README.md; docs/architecture.md;
                     docs/runtime-contract-proposal.md; docs/meaning-gate.md;
                     docs/runtime-target-map.md; docs/development-workflow.md;
                     src/boundary.rs; src/host_bevy.rs; local-compute RS00
                     TRUTH_BEVY_CONTRACT_VERIFIED.md; RS01 hvammur.html only
                     as RS01-VISUAL-REFERENCE
write_scope:         Cargo.toml; Cargo.lock; src/main.rs; src/host_bevy.rs;
                     src/render_bevy.rs; docs/README.md; docs/trial-log.md;
                     docs/dependency-map.md; docs/rs01-live-render-report.md;
                     docs/rs01-visual-fact-map.md
frozen:              registry/schema absent and unchanged; grammar
                     0x530003916889b952; standard fixture
                     0x3805f1e20c001051; closed commands/outcomes/reasons;
                     receipt format; oracle behavior/version; default and
                     bevy-host gates; canonical owner/boundary semantics
red_required:        yes — no live render command existed; the broad default
                     Bevy feature also selected an unavailable Wayland
                     dependency
verification:        formatting; default, bevy-host, and bevy-render strict
                     clippy/tests; standard runtime gates; software-Vulkan
                     player walkthrough; separate default/proof captures;
                     PNG hash/dimension checks; visual inspection
evidence:            capability red; player-input walkthrough; Publication /
                     receipt transcript; ten PNGs; visual fact map; gates;
                     atomic claims and falsifier audit below
limits:              one scene, two actors, two existing verbs, three real
                     submissions, five expression states; no gameplay value,
                     meaning, persistence, registry, schema, or broad-engine
                     policy change
environment_delta:   user-approved WSL runtime install libxkbcommon-x11-0
                     and its libxcb-xkb1 dependency; no source/build headers
escalate_when:       any fact is absent from Publication/receipt; any contract,
                     registry, value, or meaning decision is needed
tested_commit:       final run/RS01 branch tip named in the review handoff;
                     clean-tree gates rerun after commit
```

## Capability red

The pre-existing broad `bevy-full` declaration failed before the binary could
run because its default platform feature selected an unallocated Wayland
dependency:

```text
error: failed to run custom build command for `wayland-sys v0.31.11`
Package 'wayland-client' not found
```

After isolating the X11/2D consumer as `bevy-render`, the dispatched command
was still absent on the base implementation:

```text
Running `.../gra-rust-bevy-spike rs01-render`
unknown command: rs01-render
```

This is a capability red, not a semantic red: the truth trace already ran;
the missing capability was a live renderer consuming its published facts.

## Expression policy (non-authority)

The renderer chooses the aliases Snorri, Thordur, and Hvammur; calls the
material peat; and chooses an autumn-morning frame, palette, layout, pacing,
and equal-block scale. ASCII copy is deliberate because Bevy's bundled font
did not cover the Icelandic glyphs in the first real capture. These choices
are presentation only and are disclosed in the visual fact map and optional
proof overlay.

The renderer does not invent danger, hunger, obligation, emotion, weather
pressure, permanence, or state change absent from the Publication/receipt
chain. The game-facing layer contains no counter, exact written quantity,
grams, IDs, hashes, receipt, Publication/boundary vocabulary, or presentation
policy footer.

## Green implementation

`bevy-render` is an off-by-default capability slice containing the default
app, 2D renderer, winit/X11 window, bundled font, and multithreaded executor.
The pre-existing broad `bevy-full` feature remains intact.

The primary command is:

```text
cargo run --features bevy-render -- rs01-render [--proof]
```

It starts with zero receipts. Space or Enter submits exactly one canonical
command through `Host`: refused gather, witness, then successful gather. A
fourth advance reveals the aftermath without submitting another command.
Pointer/focus events cannot advance the trace.

The automated evidence path is deliberately separate:

```text
cargo run --features bevy-render -- rs01-capture <outdir> [--proof]
```

Both paths render typed character/site/claim facts copied from Bevy ECS
projection entities into an identified `Publication`. Receipts come from the
same Host submissions. The renderer has no canonical-truth observation or
writeback path and never chooses an outcome.

| Beat | Publication | Receipt result | Rendered facts |
| --- | --- | --- | --- |
| initial | revisions `0`, `0xbb13552987500462` | none | C1 60, C2 30, S1 2000 g, K1 unwitnessed |
| refused | revisions `0`, `0xbb13552987500462` | gather refused `claim_not_witnessed`, spent 0, mass 0 | byte-identical facts and identity |
| witnessed | revisions `2`, `0xa8706012a643b54c` | witness accepted, C2 spent 5 | C2 25, K1 witnessed |
| gathered | revisions `4`, `0x01dbfcabdc5f69e0` | gather accepted, C1 spent 12, mass 1200 g | C1 48, stack 1200 g, S1 800 g |
| aftermath | revisions `4`, `0x01dbfcabdc5f69e0` | none | byte-identical to gathered |

## Render evidence

The default and proof-overlay sets were captured from a real 1280 × 800
Bevy/winit X11 window using Mesa llvmpipe (`WGPU_BACKEND=vulkan`, Vulkan ICD
`lvp_icd.json`). Every PNG was opened and inspected. The recorded sets have
no missing glyphs, text collisions, or cropping in the game-facing layer.

| Frame | Game-facing SHA-256 | Proof-overlay SHA-256 |
| --- | --- | --- |
| `00-initial.png` | `86338fa32e20a1b54ff415d00f1660bbdcc64f53231ee7028700a30d07eeff1b` | `cd193dfbec981429203db5ac9c9f84c61471b5c6bb559b64f6843d0f922850a5` |
| `01-refused.png` | `c69b5f76e45a905a41fe3197b5265095f0805dbc721cdfd9ae0dbecfea40df3f` | `8c21efa87dbd574a89e82561cba2d523cbef8b23522b03d518cd7008334612d4` |
| `02-witnessed.png` | `180e84ed0ebd05bdfa44e2e4a3bdc5a61c2d448fc880265fb248026c1aa1d7e1` | `3cf78a938d759156b612459669da9917c83855f2837c644ae53620b337361782` |
| `03-gathered.png` | `2e6dea74eb39a8fe0d6562151f9a4db4a864747cfe632d013eb2897446c52dbe` | `4f3af17b7b287d0425f78a149934c74ed88e886c8d8b76b316a6f6dfaf8c6087` |
| `04-aftermath.png` | `080edf006aed94c12ab7cbde599bd1b020c231879128cb502cd280f539161e1f` | `6ead863396ccd4c85928b515942270fe7e6a91300027241464cd18a3f8764018` |

## Verification

Full gate on rustc/cargo 1.97.1, WSL2:

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | green |
| default clippy (`-D warnings`) | green |
| `bevy-host` clippy (`-D warnings`) | green |
| `bevy-render` clippy (`-D warnings`) | green |
| `cargo test` | 56 passed |
| `cargo test --features bevy-host` | 65 passed |
| `cargo test --features bevy-render` | 73 passed |
| default and `bevy-host` runtime gates | green; frozen envelope `grammar=0x530003916889b952 fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v4` |
| final player walkthrough | green; initial → refused → witnessed → gathered → aftermath; exactly three command receipts |
| default and proof capture runs | green; five files each, all 1280 × 800 |
| visual fact-map/default-copy gates | green; every default visual has one class; no default numerals or proof-ledger/engine terms |

## Claims

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
| --- | --- | --- | --- | --- |
| 1 | The primary path starts with zero receipts and reaches refused gather, accepted witness, then accepted gather through exactly three player-driven Host submissions | tested interactive path and branch tip | derivation + measurement | `primary_path_submits_exactly_one_command_per_player_advance`; final live walkthrough |
| 2 | Factual scene input comes from typed ECS view facts copied into identified Publications plus canonical receipts, not a direct canonical-world read | `src/render_bevy.rs` and RS01-only Publication accessors | derivation | `renderer_has_no_canonical_observation_backdoor`; `Beat::facts` identity checks |
| 3 | A real Bevy/winit window produced both five-frame sets at 1280 × 800 with the recorded hashes | this WSL2/X11/llvmpipe environment | measurement | capture log, PNG headers, SHA-256 table, visual inspection |
| 4 | Refused gather leaves projected facts and Publication identity unchanged; witness and gather expose only canonical post-commit changes | bounded RS01 trace | derivation | trace validation and proof overlays |
| 5 | Existing truth gates and identities remain unchanged | default and `bevy-host` gates | measurement | 56/65 tests and dual runtime envelope |
| 6 | No registry, schema, closed vocabulary, gameplay value, receipt format, oracle behavior, or canonical owner/boundary semantics changed | base `fca5237` to tested tip | exact diff | final diff and frozen envelope |
| 7 | Every meaningful default visual has one documented source class, and default copy excludes exact quantities and proof-ledger/engine terms | default RS01 view | derivation + test | `docs/rs01-visual-fact-map.md`; fact-map/default-copy tests |

## Falsifier audit

| Falsifier | Mechanical verdict | Evidence / remaining limit |
| --- | --- | --- |
| F1 comprehension | unmeasured | requires an unbriefed human viewer and verbatim answers |
| F2 beautiful lie | green | typed Publication/receipt binding and visual fact map |
| F3 frozen reenactment | green | player inputs submit live Host commands; automated capture is separate |
| F4 projection authority leak | green | no canonical observation/writeback path |
| F5 quantitative mismatch | green | stable bars/blocks; proof scale and values match 60→48, 30→25, and 2000→800→1200 g |
| F6 ledger-first default | green | default omits counters, exact values, proof/engine terms, and policy disclosure |
| F7 stale Publication | green | every action reads a fresh identified Publication; R03 stale rejection remains green |
| F8 renderer deletion | green | renderer is off by default; default and `bevy-host` truth envelopes are unchanged |
| F9 replay expression | green | two fresh traces produce identical expression-state signatures |
| F10 no coaching | unmeasured | human gate must use only “Try this short scene” before verbatim questions |

## Limits and handoff

This proves one bounded scene, its three-input path, and its Publication
expression. It does not prove a general gameplay loop, persistence,
hardware-GPU behavior, human comprehension, desire to continue, or any
historical/semantic claim. Question three is a continuation signal only, not
an RS01 pass condition. Screenshots are evidence outputs, never authority.

`bevy-full` remains the broad, separately allocated engine feature and is not
part of the RS01 gate. Independent review must reproduce the exact diff,
tests, interactive path, capture command, dimensions, hashes, and visual
verdict before ratification or integration.
