# Trial log — truth-layer slice 001

## 2026-08-14 — RS01 live Bevy publication render (red→green candidate)

Author dispatch: implement RS01 against `fca5237`; treat the HTML scene only
as `RS01-VISUAL-REFERENCE`; the proof itself must consume live Bevy
publications from a real boundary trace. The full envelope, capability-red,
expression-policy restraint, evidence, and eventual claims table live in
`docs/rs01-live-render-report.md`.

Red captured before renderer implementation: broad default Bevy features
failed on the unallocated `wayland-client` system dependency; after selecting
the named X11/2D capability slice, the binary built and then rejected the
dispatched command with `unknown command: rs01-render`. No registry, schema,
closed vocabulary, value, receipt, fixture identity, or oracle behavior is in
scope.

Green candidate: added the off-by-default `bevy-render` X11/2D capability,
executed the bounded two-actor trace through `Host`, and rendered only typed
facts copied from identified Bevy publication-view entities plus the canonical
receipts returned by those submissions. A real 1280 × 800 Bevy/winit window
under Mesa llvmpipe produced five game-facing frames and five optional exact-
proof overlays: initial, refused, witnessed, gathered, aftermath. The first
capture exposed unsupported Icelandic glyphs in Bevy's bundled font; the one
allowed clarity pass changed only non-authoritative display copy and the final
sets were visually inspected clean.

Pre-commit gate: formatting and strict default/`bevy-host`/`bevy-render`
clippy green; 56 default, 65 `bevy-host`, and 68 `bevy-render` tests pass;
dual runtime envelope remains `grammar=0x530003916889b952`,
`fixture=0x3805f1e20c001051`, `receipts=0x6c5b0e011471d985`,
`world=0x36221d3fdb8aed9a`, `oracles=10v4`. One user-approved WSL runtime
package (`libxkbcommon-x11-0`, plus `libxcb-xkb1`) was required to open the
X11 window. Full frame hashes, bounded claims, limits, and reproduction command
are in `docs/rs01-live-render-report.md`. Independent ratification remains
pending; no registry, schema, vocabulary, value, receipt, oracle, or canonical
truth contract changed.

## 2026-08-12 — bundle: TS01 truth-shape extractor (red→green)

Instrument: the dispatched envelope
`gragas-local-compute/queue/TS01-truth-shape-extractor.md`, executed as
written (base `91dcd94`; write scope `src/shapes.rs`, `src/main.rs`
shapes mode, this entry; artifacts to the local-compute repo).

Red: capability red, captured verbatim against base `91dcd94`:

```
error[E0583]: file not found for module `shapes`
```

No shapes module existed; the projection could not be expressed.

Green: `src/shapes.rs` — six shapes (verb.gather, verb.witness,
owner.character, owner.economy, owner.social, host.bevy_ecs), each with
role, dependencies, read/write sets, mutation closure, guards,
refusals, receipts, invariants, parity paths, source references, proof
references, and `authority`/`evidence_kind`/`scope`/`meaning_status`
from the closed six-status set. Values are formatted from the governing
constants at emission time; `cargo run -- shapes <dir>` writes
commit-addressed YAML + a non-authoritative HTML review page.
Write-only by construction — nothing reads the files back; the binding
tests verify the emitted *string* against the code (direction
code→projection, never projection→behavior).

Claims (claim-ids `trial/ts01-truth-shape-extractor#N`):

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
|---|---|---|---|---|
| 1 | Two independent process runs at the artifact commit produce byte-identical YAML and HTML | this toolchain, locked commit | measurement | sha256 pairs in the committed provenance file (commit-addressed); `cmp` clean |
| 2 | The projection covers every closed refusal and partial reason and every verb | vocabulary binding | derivation | tests `projection_covers_the_closed_vocabulary`, `projection_covers_every_verb` |
| 3 | Numeric values are emitted from the governing constants, not hand-copied | value binding | derivation | test `projection_values_come_from_the_governing_constants` |
| 4 | Every shape's meaning_status is in the closed six-status set, and the HTML lists all six | status closure | derivation | test `meaning_statuses_are_closed_and_fully_listed` |
| 5 | No runtime or test path reads the generated files | authority guard | derivation | `shapes` mode is fs::write-only; no fs reads exist in the crate |
| 6 | Full dual gate green on tested_commit | repo | measurement | 56 default / 65 bevy-host tests; envelope `oracles=10v4`, frozen identities |

Amendment pass (per cross-review): the host shape's writes field was
inaccurate — the host DOES write truth, through `submit` from the
single commit system; it now says exactly that. The shape-level
evidence vocabulary is closed as `EVIDENCE_KINDS` (distinct from the
§7 claim modes on purpose, both emitted in the YAML), the
`(contract bound)` value label was replaced with closed-vocabulary
wording, and claim #2 was upgraded from presence to mapping:

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
|---|---|---|---|---|
| 2b | Each verb's projected refusal set equals, in both directions, the set produced by executing every command in the recorded bounded domain (actors {1,2,3,4,5,99} x claims {1..7,9,99} x sites {1,2,9}), each against a fresh identically seeded snapshot | bounded input domain at this commit; general completeness rests on source audit at the exact commit | derivation | test `falsification_refusal_mapping_must_match_execution` |
| 7 | Shape evidence kinds come from a closed, emitted vocabulary | vocabulary closure | derivation | test `evidence_kinds_are_closed`; `evidence_kinds` block in the YAML |
| 8 | Every projected source reference carries a resolved line number computed from the source at build time | source references | derivation | `line_of` over `include_str!` sources; test `source_line_references_resolve` refuses `:0` |
| 9 | The gather read-set includes the inventory entity revision bound at validation | read-set accuracy (write-skew doctrine) | derivation | `Extraction` binds `from_inventory_revision` in `validate_extract`; shape reads field |

Bundle metadata: author Fable 5; rustc/cargo 1.97.1, WSL2; base
`91dcd94`; tested_commit = branch tip (recorded in the review request);
shared assumptions: same clone and gate commands as the reviewer.
Artifacts and run record: `gragas-local-compute` branch `run/TS01`.

## 2026-08-12 — Evidence Factory Protocol v0.1 ratified

Author verdict, recorded: ratified as proven on
`agent/evidence-factory-laws` at `6017d5b` — exact-diff review by
Sol 5.6 (base `4d5f439`, tested `6017d5b`, rustc/cargo 1.97.1, WSL2)
confirmed claims #1-#7, no scope creep, no registry/schema/runtime
change, no vocabulary drift, full dual gate green, frozen identities
unchanged. The ratification commit changes the draft classification
only; the protocol's evolution history (v1 red on its own laws → v2 →
v3 → v4 mechanical pass) stands in the entries below as evidence that
the cross-review circle works on its own governance.

The author also confirmed the provisional joint agreement of Fable 5 +
Sol 5.6: executable truth, proof history, and the truth-shape extractor
stay in `gra-rust-bevy-spike`; local orchestration, queues, runs,
rendering, and generated assets go to the separate
`gragas-local-compute` repository; the Grágás vision gets its own
document, kept apart from proven truth.

Unblocked by this ratification: the local-compute scaffold and the
first truth-shape envelope.

## 2026-08-12 — bundle: AGENTS.md draft v4 (v3 + mechanical finding pass)

v3, same envelope and write scope, three re-review points plus the
author's new laws: (1) the docs/README row now reads "Draft workflow
proposal" until the ratification commit flips the classification; (2)
§8.8 records the author's disagreement rule — one review→pushback→
re-review circle, then both readings go verbatim to the author,
dependent work is marked `BLOCKED(disagreement:<claim-id>)` and stops,
independent work continues — with the workflow circle drawn as a
diagram; (3) new §9: suggestion notes (non-binding, identity-carrying,
read daily) and provisional joint decisions (Fable 5 + Sol 5.6 may
provisionally decide architecture principles/invariants when both
explicitly agree and note it; binding only at the author's ratifying
confirmation).

v4, mechanical pass over the author-confirmed finding set. Claims for
v3+v4 content (claim-ids `agent/evidence-factory-laws#N`):

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
|---|---|---|---|---|
| 1 | §2 states the index row exists now and ratification flips only its classification | AGENTS.md §2 vs docs/README.md row | derivation | AGENTS.md §2; docs/README.md AGENTS row |
| 2 | §6 subordinates rendered-line counts to crate counts scoped to a locked feature set, toolchain, and target | AGENTS.md §6 | derivation | AGENTS.md §6 wording |
| 3 | §8.8 defines the claim-id form used by BLOCKED markers | AGENTS.md §8.8 | derivation | claim-id definition sentence |
| 4 | §9 permits envelope-free notes only outside the repo; committed notes require an envelope | AGENTS.md §9 | derivation | §9 first paragraph |
| 5 | §9 limits provisional joint decisions to author-dispatched reversible work, excluding contract/registry changes, block-lifts, and merges | AGENTS.md §9 | derivation | §9 second paragraph |
| 6 | The branch touches only AGENTS.md, CLAUDE.md, docs/README.md, docs/trial-log.md | this branch vs master | measurement | `git diff --stat master...HEAD` |
| 7 | Full dual gate is green on the branch tip | repo at tested_commit | measurement | gate transcript in review request |

## 2026-08-12 — bundle: AGENTS.md draft v2 (amended per adversarial review)

Instrument (authoring envelope, executed):

```text
base_commit:         325f5e1
objective:           agent-instruction entrypoint + cross-review protocol,
                     amended per review findings 1-8, review-ready
authoritative_files: docs/README.md, runtime-contract-proposal.md,
                     meaning-gate.md, development-workflow.md,
                     falsification-defier-audit.md
write_scope:         AGENTS.md, CLAUDE.md, docs/README.md (one index row),
                     docs/trial-log.md (this entry)
frozen:              all §4 identities; no runtime code
red_required:        no — governance/documentation; no honest red exists
                     (Meaning Gate F3); the adversarial review itself is
                     the falsifier for this class of work
verification:        full dual gate (§3), clean tree
evidence:            this entry; claims table below
tested_commit:       tip of agent/evidence-factory-laws at review request
                     (self-reference inside the commit is excluded by
                     construction; recorded in the review request)
```

Findings 1-8 from the first adversarial review, each addressed: (1)
review mandate defined as a second instrument — reviewers record
UNVERIFIABLE and continue, no deadlock on the protocol's own bundles;
(2) "single instruction source / zero drift" corrected to "single
agent-instruction entrypoint" with honest drift limits, and the index
row added; (3) objectives are author-dispatched or queue-policy, never
self-selected; (4) base_commit/tested_commit split with a clean-tree
requirement inside the gate; (5) red_required yes|no with reviewable
justification; reviewers reproduce both colors when yes; (6) claims are
atomic with scope and evidence reference; `assertion` removed as an
evidence mode; (7) reviewer is "an agent other than the author",
independence stated as procedural with identity/toolchain/assumptions
recorded; "re-derive; never merely re-read"; (8) isolation claim
reduced to detect-and-reduce, aligned with the defier audit.

Claims:

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
|---|---|---|---|---|
| 1 | Each of findings 1-8 maps to a concrete v2 change | AGENTS.md text vs review | derivation | the mapping paragraph above; diff vs `4a2f66d` |
| 2 | The branch touches only its declared write scope | this branch | measurement | `git diff --stat master...HEAD` |
| 3 | Full dual gate green on the branch tip | repo at tested_commit | measurement | gate transcript in review request |
| 4 | Frozen identities and judge unchanged | envelope line | measurement | `envelope ... grammar=0x5300... oracles=10v4` |

Author: Claude (lead programmer agent). Toolchain: rustc 1.97.1,
cargo 1.97.1, WSL2. Shared assumptions: same repo clone and gate
commands as the reviewer; measurement of rendered `cargo tree` line
counts is environment-sensitive (see the D01 correction entry) and is
subordinate to unique-crate counts.
## 2026-08-12 — correction: D01 tree line counts are environment-sensitive

Cross-review of the D01 entry found the rendered `cargo tree` line
counts (recorded below as 128 → 88) reproduce as 126 → 86 in the
reviewer's environment; the author's environment reproduces 88
(stderr-free, including 24 `(*)` dedup display lines). Both
measurements are honest; rendered line counts are display artifacts of
the toolchain/environment and are hereby subordinated to the stabler
evidence both parties confirm **for the locked feature set, toolchain,
and target**: 65 → 52 unique crates, a delta of 40 rendered lines, and
the removed-crate list (the bevy_reflect stack, the async_executor
stack, and serde entirely). One precision the review also caught:
`backtrace` is a *disabled `bevy_ecs` feature*, not a crate in the
removed list — the original entry below overstates it as a removed
crate. The original D01 entry below stands
unedited as dated evidence; the active target map now leads with crate
counts. Recorded per the disagreement rule: both readings, verbatim,
no silent reconciliation.

## 2026-08-12 — trial/D01 ECS slice audit: features minimized, green

Hypothesis (target D01): the host adapter uses only ECS fundamentals;
every other default `bevy_ecs` capability is unallocated and must not
ride along shaping the runtime. Execution baseline: `4de06a2`.

Falsifier: feature minimization. `bevy_ecs` was reduced to
`default-features = false, features = ["std"]` and the complete gates
re-run. A failure would have named the capability the adapter actually
depends on; instead everything stayed green — 48 default tests, 57
`bevy-host` tests including R01–R03, all four probe lines, envelope
byte-identical (`10v4`, frozen fingerprints).

Measured effect: local `cargo tree --features bevy-host` shrank from
128 lines / 65 unique crates to 88 lines / 52. Removed as unallocated:
`bevy_reflect` (+ derive, erased-serde, typeid, assert_type_match,
downcast-rs), `async_executor` (+ async-executor, futures-io, fastrand,
parking, slab), `backtrace`, and — notably — **serde/serde_core left
the build entirely**: no dependency in the host build is now even
positioned to supply an accidental persistence format before the R11
schema/recovery ruling. The default build remains one `cargo tree`
line with zero external dependencies.

Verdict: the extra capabilities stay disabled until a named consumer
and gate justify each (D02+ per the sweep order). No registry, schema,
gameplay contract, canonical format, receipt, or judge changed.

## 2026-08-12 — trial/R03 publication identity: red→green

Hypothesis (Runtime Contract R4, target R03): every projection snapshot
names the exact canonical state it derives from, and a stale view can be
detected and rejected without becoming truth.

Red: capability red — `Publication`, `ViewConsumer`,
`Host::publication`, and `Host::truth_revisions` did not exist
(`E0433`/`E0599`); views carried `derived_from` per entity (R01) but
publications had no ordered identity a consumer could compare.

Green, in `src/host_bevy.rs`:

- `Publication { revisions, derived_from, views }`: identity is two
  existing canonical observations — the monotone sum of the three
  owners' apply counters, and the canonical state hash. No new registry
  or schema ID was invented, per the target's own constraint.
- `ViewConsumer::accept`: rejects a delivery older than the newest seen,
  by identity alone; idempotent on re-delivery of the current one.
  Rejection is the consumer's whole power — it replaces nothing
  upstream and cannot touch canonical truth.
- The falsifier reverses delivery order (newer publication first, the
  delayed one afterwards) and proves: the stale one is rejected, the
  consumer keeps the newer projection, canonical state and receipts are
  byte-identical through both the reorder and the rejection, and the
  trial simply continues with the next publication superseding cleanly.
- The `bevy-host` runtime gate now runs the trial in two segments so a
  genuinely older publication exists each run, and prints a
  `bevy_publication` probe line folded into the exit code
  (`revisions=14` on the standard fixture — 7+5+2).

Output evolution (declared): one new `bevy_publication` line in the
`bevy-host` run. Receipts, grammar, fixture, world hashes, and the judge
(`10v4`) are all byte-identical. R01–R03 are now all part of every
`bevy-host` gate run.

## 2026-08-12 — trial/R02 host failure boundary: red→green

Hypothesis (Runtime Contract R5, target R02): failures outside `submit`
can become game outcomes or silently change truth — unless the host
separates transport faults from dispositions in a closed vocabulary.

Red: capability red — `HostFault`, `fail_next_admission`, `host_faults`,
and `publish_to` did not exist (`E0433`/`E0599`); the R5 host rows were
ratified as classification but had no executable path.

Green, in `src/host_bevy.rs`:

- Closed `HostFault` vocabulary (`admission_failed`,
  `projection_consumer_failed`), reported beside canonical receipts,
  never inside them.
- Admission gate ahead of the boundary: an injected transport failure
  leaves canonical state, hash, and receipts byte-identical, produces no
  receipt, and consumes no canonical sequence — the same intention
  re-admitted afterwards is byte-identical to a fault-free pure
  reference.
- `publish_to`: a projection consumer failing downstream of a committed
  transition cannot invalidate the commit; the fault is recorded
  host-locally and the next publish serves the projection in full.
- Topology pin: the host source contains no unwind-catching — a
  truth-layer panic (stale token, impossible apply) stays loud and is
  never translated into a disposition.
- The `bevy-host` runtime gate gained a `bevy_host_faults` probe line
  (injected admission + consumer fault each run) folded into the exit
  code.

Contract effect: the two proposed R5 host rows are promoted to tested
claims; the durability row (crash between commit and durable publish)
remains the only unproven class, held for R11. Output evolution
(declared): one new `bevy_host_faults` line in the `bevy-host` run.
Receipts, grammar, fixture, world hashes, and the judge (`10v4`) are all
byte-identical.

## 2026-08-12 — trial/R01 projection non-authority: red→green

Hypothesis (Runtime Contract R4, target R01): Bevy can project canonical
state into view components without creating a second truth owner; the
projection can be corrupted or lost without canonical truth noticing,
and every publish replaces it in full.

Red: capability red `E0433` — no `Host`, no projection, no publish; the
falsifier could not be expressed against the trial/002 adapter.

Green, in `src/host_bevy.rs`:

- The command queue moved out of the `Truth` resource into its own
  `CommandQueue` resource — transport is not truth, so loading a trial
  no longer requires mutable canonical access.
- Custody topology pinned by test: exactly one registered system holds
  `ResMut` access to `Truth` (the commit system calling `submit`); the
  pattern is built at runtime so the test's own source cannot satisfy
  the count.
- Disposable projections `CharacterView` / `SiteView` / `ClaimView`:
  plain copied facts plus `derived_from` (the canonical state hash) —
  no `World`, no owner storage, no proof tokens, no mutable handle back.
- `Host::publish` despawns every view and respawns from canonical truth.
- The behavioral falsifier corrupts a character view out of band (255
  stamina — out of canonical bounds on purpose) and despawns a claim
  view, then proves: canonical state, hash, and receipts byte-identical;
  the next publish rebuilds the projection exactly; and a subsequent
  submit is byte-identical to a pure reference that never had a
  projection at all.
- The `bevy-host` runtime gate gained a `bevy_projection` line
  (`views_match` + `derived_from`) folded into the exit code; publish
  reads truth immutably.

Output evolution (declared): the `bevy-host` run prints one new
`bevy_projection` line. Receipts, grammar, fixture, world hashes, and
the judge (`10v4`) are all byte-identical. `run_hosted` is now the
test-side helper; the runtime gate drives `Host` directly.

## 2026-08-12 — R00 passed: Runtime Contract v0.1 ratified

R1–R7 ratified by the author with both review amendments incorporated:
the R1 custody amendment (custody of canonical `World` inside a host
container grants no semantic authority; exactly one registered host
system may hold mutable access — the commit system that calls `submit`)
and the R5 evidence-status note (the host-failure-before-submit row is
promised, not proven, until R02 exercises it).

Rulings: **A1 immediate/sequential execution is law**; A2 sealed turns
stays unratified until a gameplay need opens T01, and
`agent/turn-contract` remains an unmerged candidate spec. Sequence
ownership is headed to the boundary through the R10 ruling. The next
trial is **R01 projection non-authority**. `bevy-full` remains future
convenience until capability sweeps justify it.

Full contract: `docs/runtime-contract-proposal.md`. Target order:
`docs/runtime-target-map.md`. No runtime code changed by ratification.

## 2026-08-09 — trial/010-active-cell-reachability: 12/12 PASS

Hypothesis: random or fixed-fixture traffic can leave table cells
untouched; before balance pressure can target a value honestly, every
non-exhausted stamina-band × infrastructure-tier cell must be reachable
through a coherent gather and its stock boundaries.

Red evidence against unmodified `f5728d6` was an honest capability red:

```text
error[E0425]: cannot find function `assert_all_active_cells_reachable` in this scope
   --> src/boundary.rs:950:9
    |
950 |         assert_all_active_cells_reachable();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `gra-rust-bevy-spike` (bin "gra-rust-bevy-spike" test) due to 1 previous error
```

Green: test-only coherent single-actor/site/claim worlds forced all 12
active cells through real `submit(Gather)` execution. Each cell passed
three derived stock cases: exact requested stock was Accepted, one gram
short was Partial with `site_nearly_depleted`, and empty stock was
Refused with `site_empty` and exact zero mutation.

```text
active_cell_reachability cells=12/12 cases=36 full=12 partial=12 empty=12
```

Structural result: only the four exhausted-row cells are unreachable,
deliberately, because the actor-exhausted gate returns before cost or
yield lookup. Their zero values are sentinels, not active balance
choices. The empty cases traverse yield selection but cannot expose its
numeric value in a zero-mass receipt; full and partial cases provide
that observable evidence.

Review neutralization: the original harness's `expected_yield > 0`
assertion and unconditional `yield - 1` were removed as an unauthorized
balance floor. Full-path reachability now supports zero yield against a
nonempty site; the one-short Partial case is conditional on positive
remaining stock. Current values still produce 36 cases, but zero or one
no longer fail merely because a lower Partial boundary does not exist.
The neutralized branch replay passed 48/48 default and 50/50 `bevy-host`
tests under judge `10v4`, with exact hosted parity and all non-judge
fingerprints unchanged.

Pressure verdict: **no value earned permission to move.** This pass
proves all 12 active yields and all three active gather costs can receive
future purpose-built pressure. It supplies no independent expectation
about which value is true or balanced.

Full matrix, evidence boundary, and overview:
`docs/active-cell-reachability-report.md`. No registry, schema,
contract, grammar, standard fixture, value, receipt, runtime,
dependency, oracle, or judge version changed. The standard envelope
remains `grammar=0x530003916889b952 fixture=0x3805f1e20c001051
oracles=10v4` (judge advanced by trials 007-009; unchanged by this
trial).

## 2026-08-09 — trial/008-apply-totality: behavioral red → exact bound

Hypothesis (audit defier 4): the trial/003 freshness barrier prevents stale
partial commits, but post-preflight apply is total only if validated inputs
cannot overflow or discover a new guard. Full evidence and totality table:
`docs/trial-008-apply-totality-report.md`.

The suspected red was real and reachable through the public economy owner
token API. Seed S1=`u64::MAX`, S2=1; transfer S1 into C1's inventory, then
transfer S2's gram into the same inventory. Against unmodified runtime:

```text
running 1 test
test economy::tests::falsification_overfull_inventory_must_not_silently_clamp ... FAILED

thread 'economy::tests::falsification_overfull_inventory_must_not_silently_clamp' panicked at src/economy/mod.rs:324:9:
u64::MAX + 1 inventory transfer silently clamped instead of failing
```

The old apply silently destroyed the last gram, while old `total_mass`
saturated to the same `u64::MAX` before and after; oracle 2 could therefore
false-green the loss.

Green: `FixtureFault::TotalMassOverflow` makes the overfull world invalid at
coherence validation; all mass aggregation and inventory addition use checked
arithmetic; `apply_extract` computes both arithmetic results before its first
write. Focused result: two falsifiers passed, with the bypass case panicking
loudly before mutation. This is a declared closed fixture-fault vocabulary
evolution and oracle 2 judge strengthening, so `ORACLE_SUITE_VERSION` is
3 → 4. Receipts, grammar, fixture values, registry, and schema are unchanged.

Pressure verdict: **runtime-bound / contract-vocabulary pressure, not balance
pressure**. No value moved. Revision overflow remains physically unreachable
in one process (2^64 successful applies; >584 years even at one per
nanosecond), though that is an execution-bound argument rather than a
finite-state proof. Standard allocator failure also remains outside the game
transition model. The valid-world apply path has no reachable post-preflight
guard left under exclusive serial `&mut World`.

Full gate: fmt and strict clippy clean in both feature sets; 44/44 default
tests and 45/45 `bevy-host` tests passed; both runs exited 0; all ten oracles
passed; hosted parity reported `receipts_match=true state_match=true
world_match=true`. Evidence envelope:

```text
envelope baseline_commit=f5728d6 grammar=0x530003916889b952 fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v4
```

## 2026-08-09 — trial/009-language-seam: red→green

Hypothesis (audit defier 5): a foreign source can lose meaning before a
typed `Command` exists, so receipt and host parity can both stay green after
a lossy or ambiguous source normalization.

Observation point: `Command::canonical_bytes()`. This is the exact byte
encoding already embedded inside `fixture_identity`, extracted without
changing it. Language/host parity claims begin only after an adapter has
produced these bytes; they do not prove that source text reached them
without loss.

Behavioral red, captured with the minimal hand-written text parser still
using Rust's `u64::from_str` behavior:

```text
running 1 test
test boundary::tests::falsification_text_seam_rejects_leading_plus ... FAILED

thread 'boundary::tests::falsification_text_seam_rejects_leading_plus' panicked:
leading plus silently normalized before canonical command bytes

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 42 filtered out
```

Green: test-only `parse_text_command` now accepts only ASCII, canonical
decimal integers (no sign, no leading zero except `0`) and the exact field
order/spacing of the two named command forms. Its closed `TextCommandFault`
vocabulary rejects non-ASCII/BOM input, non-canonical whitespace and
integers, overflow, missing/extra/reordered/duplicate/unknown fields, and
empty values. Canonical gather/witness lines and `u64::MAX` must produce
bytes identical to hand-constructed commands. The three seam tests pass.

Proof envelope after green (baseline `f5728d6`):

```text
envelope baseline_commit=f5728d6 grammar=0x530003916889b952
  fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985
  world=0x36221d3fdb8aed9a oracles=10v3
```

No registry/schema, grammar, receipt, fixture, oracle, dependency, or
balance-value change. The pressure verdict is **representation, not
balance**: the red occurred before any game value entered transition
semantics, so moving a yield, cost, band, or fixture number would conceal
the defect rather than answer it. Full evidence and limits:
`docs/trial-009-language-seam-report.md`.

## 2026-08-09 — trial/007-transition-domain: bounded parity PASS

Hypothesis (audit defier 2): trial/002's agreement on one recorded
16-command history does not establish universal transition-function
equivalence. A host difference can hide at the first unvisited
state/command pair.
Red evidence against unmodified `f5728d6` was an honest capability red:

```text
error[E0425]: cannot find function `bounded_transition_domain_parity` in this scope
  --> src/host_bevy.rs:93:9
   |
93 |         bounded_transition_domain_parity();
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
For more information about this error, try `rustc --explain E0425`.
error: could not compile `gra-rust-bevy-spike` (bin "gra-rust-bevy-spike" test) due to 1 previous error
```

Green: the feature-gated harness generated 1,000 deterministic traces
of depth 32 from seed `0x007007006d617065`. It visited every one of the
300 enumerated command forms over actors 1–4 plus unknown 9, claims 1–9
plus unknown 99, and sites 1–4 plus unknown 9. All 32,000 pure/Bevy
transitions produced exact canonical receipt chains and exact final
canonical states (`receipts_match=true state_match=true
world_match=true`). A divergence path is armed to shrink by command
removal and print a one-minimal counterexample; none appeared.

Pressure verdict: **no value earned permission to move.** Six of sixteen
yield cells were actually consulted; all four bands and tiers merely
appeared in receipt context. The steady cost row was consulted once,
and `unknown_site` was not boundary-reachable under the coherent
claim-first gate. This maps where future purpose-built fixtures can
apply balance pressure; it is not evidence that the reached values are
balanced.

Exact coverage counts, the claim boundary, risks, and the complete
pressure map are in `docs/transition-domain-report.md`. No registry,
schema, contract, grammar, fixture, value, receipt, runtime, dependency,
oracle, or judge version changed. The standard envelope remains
`grammar=0x530003916889b952 fixture=0x3805f1e20c001051
oracles=10v3`.

Cross-elimination replay before integration: rebased after trials 008 and
009, the same harness passed all 32,000 transitions and 300 command forms
under judge `10v4`; default tests were 47/47 and `bevy-host` tests 49/49.
Grammar, fixture, receipts, and world fingerprints stayed byte-identical.

## 2026-08-09 — falsifier map armed for overnight execution

`docs/falsifier-map.md` turns the audit's open falsifiers into three
executable trials for an automated collaborator: 007 transition-domain
pressure (bounded trace parity beyond the fixture), 008 apply-totality
(the `saturating_add` silent-clamp suspect leads), 009 language-seam
(pre-boundary normalization, reject-or-byte-identical). Falsifier #1
was closed by trial/006 this session; #4 and #6 stay contingent by the
audit's own rule against speculative machinery. Standing rules pin the
envelope (grammar and fixture frozen, judge bumps declared), red-first
evidence, zero new dependencies, and no merges before morning review.

## 2026-08-09 — trial/006-exact-final-state: defier audit falsifier #1

Hypothesis (audit defier 1): hash equality is checksum evidence, not
state equality — FNV-1a is not injective, and both host parity and the
replay oracle compared final worlds by hash alone.

Red evidence: the falsifier
`falsification_canonical_state_must_see_every_domain_mutation` demands a
canonical serialization that is stable across identical histories and
sees every truth-domain mutation; against the unmodified code it is a
capability red (compile error `E0599`: no method `canonical_state`) — no
behavioral red exists without manufacturing an FNV collision, and that
is exactly the audit's point: the comparison could not even express
exact equality.

Green: `World::canonical_state()` — one line per fact, deterministic
owner/key order, the same lines the pure host prints (the end-of-run
summary now IS the serialization, printed rather than paraphrased).
Exact-equality claims upgraded to compare it:

- Bevy parity gate: `state_match` compares serializations; the hash
  remains as checksum address. First hosted run: `receipts_match=true
  state_match=true world_match=true`.
- Oracle 7 `replay_determinism`: now `states_match` + `hashes_match` +
  `receipts_match`.

Spec evolutions (conscious): `ORACLE_SUITE_VERSION` 2 → 3 (oracle 7
behavior strengthened); run-summary site lines now carry `tier=` since
tier is truth state (output evolution — receipts, grammar, fixture, and
world hashes all byte-identical, envelope confirms `oracles=10v3` as the
only change). `EconomyOwner::stock` removed — the serialization reads
through `sites_iter`, and a dead accessor is a trap, not an API.

## 2026-08-09 — adversarial defier audit: proof boundaries recorded

An external hostile review attacked the mathematical claims behind the
four-trial sprint with model-preserving counterexamples. Full record:
`docs/falsification-defier-audit.md`.

Result: no new runtime defect and no contract, registry, schema, grammar,
oracle, fixture, or value change. Trials 003–004 had already closed the
concrete owner-wide false conflict, stale-token partial commit, shared
run/replay bug, and receipt-only final-state gap.

The surviving defiers bound what may honestly be claimed: equality on one
fixture is not universal transition equivalence; FNV equality is checksum
evidence rather than state equality; future cross-entity invariants require
complete read-set or predicate-version binding; rollback-free preflight also
requires post-check apply totality; language seams can lose meaning before
the canonical command exists; and red-first value changes still need a sealed
holdout to resist overfitting. The branch workflow cross-eliminates
implementation hypotheses, but its shared fixture and judge mean the four
trials are not four logically independent proofs.

`docs/development-workflow.md` now carries those review questions and the
strong-versus-checksum reading of the proof envelope. Current receipt and
world hashes remain byte-identical to baseline `08db100`.

## 2026-08-09 — trial/005-value-pressure: null result, recorded on purpose

Hypothesis: an incoherent result under trials 002–004 is pressure to
*move a value* under a logged, red-first hypothesis rather than to lock
it. This branch held the only license to change the grammar fingerprint
and was merged last by design.

Result: **no value moved, because no incoherence surfaced.** 004 exposed
an oracle capability gap, 003 exposed conflict-granularity semantics,
002 passed parity clean — none of them implicated a table value, band
threshold, cost, or fixture number. Every value therefore remains a
mechanical example, and the grammar fingerprint
`0x530003916889b952` has now survived the entire four-trial sprint
unchanged — which is itself the protocol working, not a formality
skipped.

The discipline this null result pins down: a value may move only when a
falsified expectation names it — state why the old value is wrong, write
the red test the old value fails, then move it and update the shadow
evaluator's own literals consciously. A value moved without that chain
is tuning until the oracles go quiet, and it is forbidden even on this
branch. When a future trial surfaces a concrete incoherence, a fresh
`trial/00N-value-pressure` spawns with that target named in its first
log line.

## 2026-08-09 — trial/002-bevy-host-parity: parity PASS

Hypothesis: Bevy can host the existing truth and reproduce identical
receipts and hashes without adding gameplay semantics.

This is a pass/fail parity experiment, not a red→green fix: no
behavioral red exists before the host exists, so the falsifier is the
comparison itself — `src/host_bevy.rs` hosts the truth `World` as an ECS
resource, submits the identical fixture and 16-command sequence through
the identical boundary (one command per schedule tick, every write still
through `submit`), and the run exits non-zero on any divergence.

Evidence (baseline `5e844fa`):

```
bevy_host_parity receipts_match=true world_match=true
  receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a
envelope ... grammar=0x530003916889b952 fixture=0x3805f1e20c001051
  receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v2
```

The hosted digests equal the pure run's envelope values exactly, on the
same grammar and fixture — the host contributed scheduling and nothing
else. Conscious dependency decision: `bevy-host` now pulls `bevy_ecs`
only (the parity proof needs scheduling, not a renderer); new
`bevy-full` feature layers the whole engine on top for later rendering
work. The default gate remains zero-dependency.

Known frontier deliberately NOT claimed by this trial: truly parallel
plan+commit inside Bevy systems. Entity-revision tokens and the
all-or-nothing commit (trial/003) make disjoint plans safe to commit
from one snapshot, but canonical receipts chain `world_before` →
`world` sequentially — a parallel commit model needs its own receipt
semantics before any host may schedule submissions concurrently. That
is the next falsifiable hypothesis for this seam.

## 2026-08-09 — trial/003-parallel-plan: red→green

Hypothesis: owner-wide revision binding false-conflicts independent
plans, and the boundary's commit sequence can leave a partial commit
when only a later owner's token has gone stale.

Preparation (behavior-neutral, before any red): the commit phase was
extracted into `GatherPlan::apply` / `WitnessPlan::apply` — the one
place a multi-owner plan mutates the world — so the falsifiers could
exercise the real commit path with planning separated from application.
Receipts and hashes unchanged by the refactor.

Red evidence (captured against baseline `466f272`, three failures):

```
character::falsification_independent_spends_must_not_false_conflict
  panicked: stale proof token (character) — boundary bug
    left: 0 / right: 1        <- C2's spend killed by C1's apply

boundary::falsification_independent_plans_against_one_snapshot_must_both_commit
  panicked: stale proof token (character) — boundary bug
                              <- two fully disjoint plans false-conflict

boundary::falsification_stale_later_token_must_not_leave_partial_commit
  first panic:  stale proof token (economy) — boundary bug
  second panic: stale plan committed partially: the character spend
                landed without the extraction
    left:  5778436795119231595   <- world hash CHANGED across a refused
    right: 4302712438088730884      commit: partial commit
```

Green, two mechanisms (both required — neither alone passes all three):

1. **Entity-revision binding.** Tokens now bind to the entities they
   touch: per character (`StaminaSpend`), per site + per inventory
   (`Extraction`), per claim (`WitnessGrant`). Disjoint plans validated
   against the same snapshot are independent and both commit; same-entity
   replay still panics (all prior `should_panic` tests unchanged).
2. **Two-phase commit.** `GatherPlan::apply` / `WitnessPlan::apply`
   check every token fresh BEFORE any owner mutates; a stale plan panics
   with zero mutation — all-or-nothing, verified by hash equality across
   the refused commit.

Entity revisions are derived bookkeeping, excluded from the world hash;
owner-wide apply counters remain hashed, so **grammar, fixture,
receipts, and world hashes are all byte-identical** to the baseline —
the envelope proves this trial changed conflict semantics without
touching game semantics. No spec evolution: no new oracles, reasons, or
receipt fields.

## 2026-08-09 — trial/004-shadow-final-state: red→green

Hypothesis: final-world truth rested on oracle 7 alone, which replays
through the same implementation it audits — a bug shared by run and
replay presents a divergent final world no independent oracle sees.

Red evidence (captured against baseline `a91023d` before the fix): the
falsifier applies one extra gather to the world after the logged trial —
mass conserved, log untouched, so oracles 1–6 and 8–9 stay green — and
demands a failure from any oracle other than `replay_determinism`:

```
falsification_divergent_final_world_must_fail_an_independent_oracle FAILED
  panicked: divergent final world was visible only to the
  self-trusting replay oracle
```

Green: new oracle 10 `shadow_final_state` — the shadow evaluator (already
independent for receipts) now steps every command and compares its final
state against the actual world across all four truth domains: stamina,
inventories, site stocks, claim gates. It reads the world only through
read-only owner iterators; zero trust in the implementation, receipts, or
replay. The falsifier and the full gate are green.

Spec evolution (conscious): oracles **nine → ten** (count still
type-enforced), `ORACLE_SUITE_VERSION` **1 → 2**. Envelope before/after
the merge proves the protocol's promise: `grammar`, `fixture`,
`receipts`, and `world` byte-identical, only the judge changed
(`oracles=9v1` → `oracles=10v2`).

## 2026-08-09 — sprint armed: four concurrent trials, cross-elimination protocol

Baseline for the whole sprint: `3ce9efc` (master). Protocol recorded in
`docs/development-workflow.md` § Cross-trial comparison protocol; every
run now emits a proof envelope
(`baseline_commit / grammar / fixture / receipts / world / oracles`).

The four trials, each one falsifiable hypothesis, with its named falsifier:

1. `trial/004-shadow-final-state` — **hypothesis:** final-world truth
   currently rests on oracle 7 alone, which replays through the same
   implementation it audits; an implementation whose run and replay share
   a bug can present a divergent final world that no independent oracle
   sees. **Falsifier:** a divergent final world paired with an
   internally consistent log must fail at least one oracle *other than*
   `replay_determinism`. Red today; green when the shadow evaluator's
   final state is compared against the actual world.
2. `trial/002-bevy-host-parity` — **hypothesis:** Bevy can host the
   existing truth and reproduce identical receipts and hashes without
   adding gameplay semantics. **Falsifier:** envelope comparison — the
   Bevy-hosted run must reproduce `receipts` and `world` exactly, on the
   same `grammar` and `fixture`, or the host has acquired semantics.
3. `trial/003-parallel-plan` — **hypothesis:** owner-wide revisions
   false-conflict independent plans and can leave a partial commit.
   **Falsifiers (both red today):** two plans for independent characters
   validated against the same snapshot must both apply without a stale
   panic; a stale second-owner token must never leave the first owner's
   apply committed. Green requires finer conflict granularity and an
   all-tokens-fresh check before any apply.
4. `trial/005-value-pressure` — **hypothesis:** an incoherent result
   under the trials above is pressure to *move a value* under a logged,
   red-first hypothesis rather than to lock it. This branch is the only
   one allowed to change the grammar fingerprint, and it merges last.
   Its concrete target is chosen from whatever incoherence the other
   three surface — deliberately not predetermined.

Merge order: 004 (strengthen the judge) → 002 and 003 (either order,
first-green-first) → 005 (re-baselines everyone). After each merge the
remaining branches rebase and re-run red→green — the cross-elimination
moment. Envelope fields make any silent divergence visible immediately.

Spec evolution this round (conscious): `cargo run` output gained the
envelope line; `ORACLE_SUITE_VERSION` (v1) added to name the judge.
Receipts, grammar, and world hashes are unchanged.

## 2026-08-09 — architecture decision: Bevy host and worktree loop

- The Bevy HOLD is lifted. The round-2 condition was satisfied: the
  adversarial second verb landed without leaking policy into established
  owners, and the hidden single-verb assumptions surfaced in the oracles.
- Bevy is assigned a narrower role than the truth layer: execution,
  scheduling, ECS projection, interaction, rendering, and additional test
  surfaces. It must submit commands through the same boundary and reproduce
  the pure host's canonical receipts and final hash.
- The pure-Rust truth layer remains the semantic authority below every host.
  Registry/schema contracts remain higher authority when present; this
  decision changes no registry or schema.
- Multi-language work is allowed only across explicit, versioned seams. A
  language or tool may express a form where it is clearest, but it may not
  acquire a second mutation path into canonical truth.
- `docs/architecture.md` records the refined layer model and host acceptance
  criteria. `docs/development-workflow.md` records the repeatable branch and
  worktree cycle for hypothesis, red-first falsification, implementation,
  review, integration, and cleanup.
- No runtime semantics changed in this decision round.

## 2026-08-08 — falsification round 2: second verb (`witness`) vs isolation

Full standalone record: `docs/verb-isolation-report.md`. Summary:

- Second verb `witness` added, deliberately inverting every gather
  pattern: flat cost 5, no exhausted gate, Social+Character only,
  Economy untouched, zero mass, and the social owner's first mutation
  path (revision-bound `WitnessGrant`, by-value, stale panics).
- **Isolation claim held:** `git diff` on `src/character/mod.rs` and
  `src/economy/mod.rs` is empty — zero lines changed for the new verb.
  All verb policy lives in the boundary (`Command` dispatch,
  `plan_witness`, `WITNESS_COST`).
- **Falsification found and fixed:** the first full run turned oracles 3
  and 4 red on a legal trial — both had encoded "Accepted ⇒ mass moved",
  a single-verb assumption. Corrected to key on actual mass movement,
  which is verb-agnostic and stronger.
- Closed vocabulary grew consciously: RefusalReason 9 → 11
  (`claim_already_witnessed`, `cannot_witness_own_claim`), new closed
  `Verb` enum in every receipt, `WITNESS_COST` folded into the grammar
  fingerprint.
- Gate: fmt clean, clippy -D warnings clean, 35/35 tests, run exit 0,
  all nine oracles PASS across the 16-command two-verb trial.
- Round-1 HOLD condition (second verb + isolation survives) is met;
  lifting HOLD remains the author's call.

## 2026-08-08 — falsification round 1: red first, then green

External review (Codex + author) identified false-green risks in the
scaffold. Discipline followed: falsification tests were written and run
**red against the unmodified code first**, then the fixes were applied,
then the tests went green for the right reasons.

### Red evidence (captured before any fix)

```
economy::tests::falsification_token_replay_must_not_create_mass
  assertion `left == right` failed: token replay created mass
    left: MassGrams(3900)
   right: MassGrams(2300)      <- one &Extraction applied twice minted 1600 g

character::tests::falsification_low_headroom_must_be_refused
  panicked: 12-point actor approved for a 15-point spend
```

### Fixes, in the review's priority order

1. **Token replay / stale token.** Proof tokens (`StaminaSpend`,
   `Extraction`) are now consumed **by value** — reusing one token is a
   compile error. Each token carries the owner revision it was minted
   against; owners bump their revision on every apply, and applying a
   stale token (two tokens minted against the same revision, or a token
   applied after unrelated state change) panics with
   `stale proof token — boundary bug`. Loud, never silent.
   `#[should_panic]` tests pin both owners.
2. **Exact stamina headroom.** `saturating_spend` (the silent clamp) is
   deleted. Validation requires exact headroom and refuses with the new
   closed reason `insufficient_stamina`; apply performs an exact
   subtraction that cannot clamp. A 12-point actor asked for a 15-point
   spend is now receipt line `outcome=refused reason=insufficient_stamina`
   (fixture seq=10 demonstrates it).
3. **Refusal = byte-identical state.** Receipts now record
   `world_before` and `world` (after). New oracle 8
   `refusal_zero_mutation` walks the hash chain: each receipt's before
   must equal the previous after (starting from the fixture hash),
   refusals must not change the hash, yields must change it.
4. **Paired state+receipt lie.** New oracle 9 `shadow_expectation`: an
   independent evaluator that rebuilds initial state from the fixture and
   recomputes every expected outcome with its own state tracking and its
   own band-threshold literals — it never reads a receipt field. A test
   doctors a receipt into an internally consistent lie (band=low,
   mass=600, spent=15) and shows oracles 3–6 all accept it while the
   shadow refuses it.
5. **Fixture coherence.** Owner seeding now rejects duplicate IDs
   (`FixtureFault::Duplicate*` — no silent last-write-wins) and
   `validate_world_coherence` rejects claims referencing an unknown
   holder or site.
6. **Grammar fingerprint.** Every receipt carries `grammar=0x…`, an
   FNV-1a hash of the yield table, cost table, realized band mapping over
   0–100, and all closed reason codes — a trial record now names the
   grammar version that produced it.
7. **Second-verb isolation (preparation).** The gather cost table moved
   out of `CharacterOwner` into boundary verb policy; the owner API is
   now `validate_spend(id, cost)` — verb-agnostic resource semantics
   only. The actual second verb remains future work, by design.

### Spec evolutions this round (conscious changes)

- Oracles: exactly **seven → nine** (count still type-enforced).
- Refusal reasons: **eight → nine** (`insufficient_stamina` added).
- "Infallible apply" restated honestly: an apply never produces a wrong
  game outcome; it panics on a stale token because that is a boundary
  bug, not a game outcome.
- Owner revisions are part of the hashed state, so the world hash also
  advances with every apply.

### Gate after the round

| Check | Result |
| --- | --- |
| `cargo fmt --check` | clean |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `cargo test` | 32 passed, 0 failed (incl. 4 falsification tests) |
| `cargo run` | all 9 oracles PASS, exit 0 |

Bevy host remains ON HOLD. The first four falsification items are done
red→green; the standing recommendation is to keep HOLD until the second
verb lands and the isolation claim survives it.


## 2026-08-08 — scaffold rebuilt and verified in remote session

- The earlier local scaffold branch (`codex/truth-layer-slice-001`) never
  reached the remote: the GitHub App push was refused (403) and the git
  bundle only exists on the author's machine, unreachable from this
  environment. The slice was therefore rebuilt from its written spec on
  `claude/truth-layer-scaffold-verify-2fhvhp`.
- Spec honored: three single-writer owners (Character, Economy, Social);
  typed IDs, bounded stamina, `MassGrams` with negative mass
  unrepresentable; witnessed claim as boolean gate; one active 4×4 cell
  (Stamina × GatheringInfrastructure); Accepted/Partial/Refused with closed
  reasons; validate-everything-before-infallible-apply; canonical receipts
  and deterministic world hashes; exactly seven bounded oracles.
- All fixture and table numbers are mechanical examples — not balance, not
  historical truth.
- Bevy host decision: still ON HOLD. `bevy`/`bevy_ecs` stay pinned but move
  behind the off-by-default `bevy-host` feature so the default build is the
  pure boundary and the compiler gate stays fast and deterministic.

### Compiler gate results (this environment, rustc 1.94.1)

| Check | Result |
| --- | --- |
| `cargo fmt --check` | clean |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `cargo test` | 21 passed, 0 failed |
| `cargo run` | all 7 oracles PASS, exit 0 |

### The seven oracles

1. `stamina_in_bounds` — every stamina stays in `0..=100`
2. `mass_conserved` — total mass equals the fixture baseline
3. `witnessed_gate` — no unwitnessed claim ever moves mass
4. `exhausted_gate` — no exhausted actor ever yields
5. `closed_reasons` — every reason code round-trips the closed enums
6. `cell_bounds` — every yield stays inside the active 4×4 cell
7. `replay_determinism` — replaying the fixture reproduces receipts and hash

## 2026-08-14 — RS01 player-driven live Publication renderer

- Added the off-by-default `bevy-render` capability and a real Bevy/winit X11
  window consuming typed facts copied from identified Publications plus the
  canonical receipts returned by `Host`.
- Corrected the first implementation gap found against the recovered dispatch:
  `rs01-render` now begins with zero receipts and requires one deliberate
  Space/Enter submission for each of gather/refuse, witness, and gather. The
  automated five-frame evidence path is separately named `rs01-capture` and
  cannot stand in for the primary player path.
- Added `docs/rs01-visual-fact-map.md`, deterministic replay and deletion/
  isolation checks, a one-command-per-advance test, and a default-copy gate.
  Exact quantities, grams, IDs, hashes, receipts, engine vocabulary, and
  presentation-policy disclosure are proof-only; the default view uses stable
  bars and equal blocks without ledger copy.
- Final live walkthrough reached initial → refused → witnessed → gathered →
  aftermath through exactly three Host receipts. Separate default and proof
  capture sets each produced five visually inspected 1280 × 800 PNGs.
- Registry, schema, closed vocabulary, gameplay values, receipt format,
  oracles, and canonical owner/boundary semantics were not changed.
- Mechanical verdict is green. F1/F10 remain unmeasured until an unbriefed
  human receives only “Try this short scene” and answers the dispatched
  questions verbatim. Question three is continuation evidence only.
