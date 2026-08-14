# RS01 — live Bevy publication render

Status: **GREEN IMPLEMENTATION CANDIDATE; independent ratification pending**.
Author-dispatched 2026-08-14. Base `fca5237`.

## Envelope as run

```text
base_commit:         fca5237
objective:           open a real Bevy 2D window, execute the Hvammur
                     gather/refuse → witness → gather trace through the
                     existing boundary/Host seam, and capture five frames
                     whose factual content comes from identified live
                     Publications and canonical receipts
authoritative_files: AGENTS.md; docs/README.md; docs/architecture.md;
                     docs/runtime-contract-proposal.md; docs/meaning-gate.md;
                     docs/runtime-target-map.md; docs/development-workflow.md;
                     src/boundary.rs; src/host_bevy.rs; local-compute RS00
                     TRUTH_BEVY_CONTRACT_VERIFIED.md; RS01 hvammur.html only
                     as RS01-VISUAL-REFERENCE
write_scope:         Cargo.toml; Cargo.lock; src/main.rs; src/host_bevy.rs;
                     src/render_bevy.rs; docs/README.md; docs/trial-log.md;
                     docs/dependency-map.md; docs/rs01-live-render-report.md
frozen:              registry/schema absent and unchanged; grammar
                     0x530003916889b952; standard fixture
                     0x3805f1e20c001051; closed commands/outcomes/reasons;
                     receipt format; oracle behavior/version; default and
                     bevy-host gates; canonical owner/boundary semantics
red_required:        yes — capability red: no live render command existed;
                     the broad default Bevy feature set also demanded an
                     unavailable, unallocated Wayland system dependency
verification:        cargo fmt --check; both clippy gates from AGENTS.md;
                     cargo test; cargo test --features bevy-host;
                     cargo test/clippy --features bevy-render; both standard
                     cargo-run gates;
                     software-Vulkan rs01-render; five PNG existence/hash/
                     dimension checks; visual inspection
evidence:            red transcript below; live publication/receipt console
                     transcript; initial/refused/witnessed/gathered/aftermath
                     PNGs; gate tail; numbered claims table
limits:              one scene, two actors, two existing verbs, three real
                     submissions, five frames, one clarity pass; no gameplay
                     value change, new meaning, persistence, registry, schema,
                     or broad-engine policy
environment_delta:   user-approved WSL runtime install libxkbcommon-x11-0
                     (and its libxcb-xkb1 dependency); no source/build headers
escalate_when:       any required fact is absent from Publication/receipt;
                     any contract/registry/value/meaning decision is needed;
                     real rendering cannot be reproduced with existing X11 +
                     lavapipe capabilities
tested_commit:       branch tip named in the review request; the clean-tree
                     gate is rerun after the implementation commit
```

## Capability red (verbatim excerpts)

The pre-existing broad `bevy-full` declaration failed before the binary could
run because its default platform feature selected an unallocated Wayland
dependency:

```text
error: failed to run custom build command for `wayland-sys v0.31.11`
Package 'wayland-client' not found
```

After isolating the named X11/2D consumer as `bevy-render`, the
dispatched command was still absent on the base implementation:

```text
Running `.../gra-rust-bevy-spike rs01-render`
unknown command: rs01-render
```

This is a capability red, not a semantic red: the existing truth trace was
already executable; the missing capability was a live renderer consuming its
published facts.

## Expression policy (non-authority)

The renderer chooses the display names Snorri, Thordur, and Hvammur; calls the
material peat; and chooses an autumn-morning frame, palette, layout, pacing,
and a visual scale of 200 grams per block. Those choices are printed on every
frame. ASCII display copy is deliberate because Bevy's bundled default font
did not cover the Icelandic glyphs in the first real capture. These choices
are presentation only. The renderer does not invent danger, hunger,
obligation, emotion, weather pressure, or any state change absent from the
Publication/receipt chain. Exact IDs, quantities, hashes, and receipt lines
belong to the optional proof overlay; the default capture remains game-first.

## Green implementation

`bevy-render` is a named, off-by-default capability slice: the default app,
2D renderer, winit/X11 window, bundled font, and multithreaded executor. It
leaves the pre-existing broad `bevy-full` boundary intact. The command is:

```text
cargo run --features bevy-render -- rs01-render <outdir> [--proof]
```

The command creates its fixture, submits the three commands through `Host`,
and renders typed character/site/claim facts copied from Bevy ECS projection
entities into an identified `Publication`. Receipts come from the same host
submissions. The presentation module has no canonical-truth observation path.

The reproduced live chain was:

| Beat | Publication | Receipt result | Rendered facts |
| --- | --- | --- | --- |
| initial | revisions `0`, `0xbb13552987500462` | none | C1 60, C2 30, S1 2000 g, K1 unwitnessed |
| refused | revisions `0`, `0xbb13552987500462` | gather refused `claim_not_witnessed`, spent 0, mass 0 | byte-identical facts and identity |
| witnessed | revisions `2`, `0xa8706012a643b54c` | witness accepted, C2 spent 5 | C2 25, K1 witnessed |
| gathered | revisions `4`, `0x01dbfcabdc5f69e0` | gather accepted, C1 spent 12, mass 1200 g | C1 48, stack 1200 g, S1 800 g |
| aftermath | revisions `4`, `0x01dbfcabdc5f69e0` | none | byte-identical to gathered |

## Render evidence

Both the default game-facing set and the optional proof-overlay set were
captured from a real 1280 × 800 Bevy/winit X11 window using Mesa llvmpipe
(`WGPU_BACKEND=vulkan`, Vulkan ICD `lvp_icd.json`). Every PNG was opened and
visually inspected after capture. The first glyph-defective draft was rejected
and replaced; the recorded set has no missing glyphs, text collisions, or
cropping in the game-facing layer.

| Frame | Game-facing SHA-256 | Proof-overlay SHA-256 |
| --- | --- | --- |
| `00-initial.png` | `cd1b11ff0151bbf6276bbd7a6ac3dde46bc6e1d2b98834af3184fe174ddfd79a` | `fda124f96a5a73c899f2423ff6761f0f32ed344de409ae0d1535695dbeeaa29a` |
| `01-refused.png` | `b4814a704a14f6e53290377838c39d24deb8192446ab61a64564aedfddab29aa` | `db88e9b3b0e3b5f4feb159d010379c5a2da0b398eb3c8e6e39f3515cabe10349` |
| `02-witnessed.png` | `8e505bd70ca1f993cbc2be99064e03b5ece6e4a2dff52fba59524a379bacfb77` | `90ebabeadc4d878ae6f8ba40cd6037fd792367a9b2c43476d438ef852f82f021` |
| `03-gathered.png` | `6d7eb4ba8589f754804486591ae9e09b0d19b01e6f9b54a9698b20c9e234431e` | `1adc2928ad3498d570219b20c093518d85af55d3c81db4877fd75827ed763c79` |
| `04-aftermath.png` | `91740a87cc3c7c2931c0d268b15cbd522396ad1f7a4612d7d7b19824c3da6224` | `21c2a245aa92f27ea7ecfff4dcf45d272dd2906097f68ce6d2919a9d048cbf94` |

## Verification

Pre-commit full gate on rustc/cargo 1.97.1, WSL2:

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | green |
| default clippy (`-D warnings`) | green |
| `bevy-host` clippy (`-D warnings`) | green |
| `bevy-render` clippy (`-D warnings`) | green |
| `cargo test` | 56 passed |
| `cargo test --features bevy-host` | 65 passed |
| `cargo test --features bevy-render` | 68 passed |
| default and `bevy-host` runtime gates | green; frozen envelope `grammar=0x530003916889b952 fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v4` |
| game-facing and proof render runs | green; five files each, all 1280 × 800 |

## Claims

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
| --- | --- | --- | --- | --- |
| 1 | The RS01 fixture reaches refused gather, accepted witness, then accepted gather through the real Host boundary, with the publication/receipt identities and values listed above | the three submitted commands at the tested branch tip | derivation + measurement | `rs01_trace_is_publication_identified_and_receipt_derived`; live console transcript |
| 2 | The renderer's factual scene input comes from typed ECS view facts copied into identified Publications plus canonical receipts, not from a direct canonical-world read | `src/render_bevy.rs` and RS01-only Publication accessors | derivation | `renderer_has_no_canonical_observation_backdoor`; identity checks in `Beat::facts` |
| 3 | A real Bevy/winit window produced both five-frame sets at 1280 × 800 with the recorded hashes | this WSL2/X11/llvmpipe environment | measurement | capture log, PNG headers, SHA-256 table, visual inspection |
| 4 | Refused gather leaves the initial projected facts and publication identity unchanged; witness and gather expose only their canonical post-commit changes | bounded RS01 trace | derivation | trace validation test plus the five proof overlays |
| 5 | Existing truth gates and identities remain unchanged | default and `bevy-host` gates at the tested branch tip | measurement | 56/65 tests, dual runtime envelope above |
| 6 | No registry, schema, closed vocabulary, gameplay value, receipt format, oracle behavior, or canonical owner/boundary semantics changed | base `fca5237` to tested branch tip | exact diff | final diff review and frozen runtime envelope |

## Limits and handoff

This proves one bounded scene and its publication path; it does not prove a
general gameplay UI, input loop, persistence, hardware-GPU behavior, or any
historical/semantic claim. Screenshots are evidence outputs, never authority.
`bevy-full` remains the broad, separately allocated engine feature and is not
part of the RS01 gate. Independent review must reproduce the exact diff,
tests, render command, dimensions, hashes, and visual verdict before
ratification or integration.
