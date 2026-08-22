# Trial CON01 — live-law conformance

Status of this section: **pre-registration**, committed before the test
or any document repair exists. Evidence sections are appended later and
say so.

Branch: `claude/w01-winter-crisis-review-4d9i9e` (the session's
designated branch; under local convention this work would be
`trial/CON01-live-law-conformance` — recorded here as workflow note S3
so the reviewer can hold the deviation against the workflow doc).
Author: lead. Dispatch: the author's 2026-08-22 instruction to do the
unblocked work exactly as amended by Sol's architecture review
(CON01 required amendments: this name, `red_required: yes`, full live
drift in scope, historical reports untouched).

Base commit: `5884f27` (master; grammar `0x7dd8c6706e0b949f`,
oracles `10v7`).

## 0. What this trial is for

Master moved through RES01, V01 and W01 under licensed identity moves,
and the live law did not move with it. `AGENTS.md` §4 freezes a grammar
and a fixture identity that no longer exist; `HANDOUT.md` hands a fresh
session a dead envelope and a two-configuration gate; the target map's
"Current position" describes a master four trials old. Three documents
drifted the same way in the same month, which is not three accidents —
it is a missing pin. This trial adds the pin: an executable test that
binds the live documents to the identities the code actually enforces,
so live-document drift becomes a red test instead of a review finding.

The trial changes **no runtime behavior, no identity, no vocabulary,
no value**. Its write scope is live documents plus one test-only
module. Historical reports keep their historical identities untouched:
a dated record is evidence, not law, and the test deliberately cannot
see them.

## 1. Authoring envelope (as run)

```text
base_commit:         5884f27
objective:           Bind the three live documents (AGENTS.md,
                     HANDOUT.md, the target map's Current position) to
                     the executable identities, red-first; repair the
                     live drift the pin exposes, including the stale
                     T01 command count and the stale held-branch table.
                     Stop condition: the new conformance tests red at
                     base, green after repair, and the full
                     four-configuration gate green on a clean tree.
authoritative_files: AGENTS.md; docs/README.md;
                     docs/development-workflow.md; the author's
                     2026-08-22 dispatch and Sol's amendments
write_scope:         src/conformance.rs (new, test-only);
                     src/main.rs (one cfg(test) mod line);
                     AGENTS.md; HANDOUT.md; docs/runtime-target-map.md;
                     docs/trial-con01-live-law-conformance-report.md;
                     docs/trial-log.md; docs/README.md
frozen:              all runtime behavior and types; grammar, command
                     encoding, receipt format, fixture and oracle
                     identities; the standard and winter fixtures;
                     every historical report; registry/schema (none
                     exists and none may be introduced)
red_required:        yes — the conformance tests must fail verbatim
                     against the stale live documents at base_commit
verification:        clean tree; cargo fmt --check; cargo clippy
                     --all-targets -D warnings for default, bevy-host,
                     bevy-render, e01-taste; cargo test for the same
                     four; hosted standard run and hosted winter run,
                     each judged by direct exit code, never a pipe
evidence:            verbatim red per document; green test output;
                     four-configuration gate transcript tail with
                     envelope lines; claims table
limits:              no new dependencies; no CI; no gate script
                     (TOOL01 is serial after this trial's
                     ratification); no toolchain file
escalate_when:       any identity or runtime behavior would need to
                     move; a check cannot be expressed without giving
                     documents authority over code
tested_commit:       2a93bfb
```

## 2. The drift, enumerated at base

| Live document | Stale claim at `5884f27` | Current fact |
|---|---|---|
| `AGENTS.md` §3 | gate = two feature sets | four: default, `bevy-host`, `bevy-render`, `e01-taste`, plus hosted standard and winter runs |
| `AGENTS.md` §4 | freezes grammar `0x530003916889b952`, fixture `0x3805f1e20c001051` | licensed moves (RES01 §4, V01) ended at grammar `0x7dd8c6706e0b949f`, cmdfmt `0xfa37eefa3594cfe3`, rcptfmt `0x7e62152622bb9132`, fixture `0x93afba3f312bd89d` |
| `HANDOUT.md` | envelope `receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v4`; two-config gate; standing state as of 2026-08-14 | envelope `receipts=0xc0b4da51744bcf19 world=0xb500dee0e5d883d8 oracles=10v7`; four-config gate; master `5884f27` includes E01, 013, 014, RES01, V01, W01 |
| `docs/runtime-target-map.md` "Current position" | master = "007–010, R01–R03, D01, TS01, RS01", 56/65/73 tests, `10v4` envelope | adds 013, 014, E01, RES01, V01, W01; 87/97/105/111 tests; `10v7` envelope |
| `docs/runtime-target-map.md` T01 | "the existing 16 commands as sixteen one-command turns" | the standard trial is 27 commands (V01 added 11); Sol's review adds: bridge the three W01 traces as one-command turns as well |
| `docs/runtime-target-map.md` held table | 013 "holdout sealed", 014 "review required" listed as held | 013 merged (`9a766ca`), 014 verified at `5fc8376` and merged (`8c81454`), E01 merged (`1f3cbc6`); held unmerged: 011 (`5b52e81`), 012 (`ab45f40`), each one unique commit behind master |

## 3. The pin, designed

One test-only module, `src/conformance.rs`, compiled only under
`cfg(test)`, reading the three live documents with `include_str!`.
Direction of authority is one-way: the documents are held to the code;
nothing in the crate ever derives behavior from a document. No
registry, no schema, no runtime read.

Three tests, pre-registered by name:

1. `agents_md_freezes_the_identities_the_code_enforces` —
   `AGENTS.md` must contain the formatted current values of
   `grammar_fingerprint()`, `command_encoding_fingerprint()`,
   `receipt_format_fingerprint()`, and the standard fixture identity.
   These are the frozen *inputs* of the law; run outcomes stay out of
   AGENTS.
2. `handout_identity_block_is_the_current_envelope` — `HANDOUT.md`
   must contain all four of the above plus the standard trial's
   receipt-chain digest, final world hash, and the oracle tag
   (`oracles=10v7` today, computed from `ORACLE_COUNT` and
   `ORACLE_SUITE_VERSION`, never hard-coded).
3. `target_map_current_position_quotes_the_current_envelope` — the
   section of `docs/runtime-target-map.md` between `## Current
   position` and the next `## ` heading must contain the same seven
   values.

The fixture identity, receipt digest and world hash are **recomputed**
inside the test by running the standard trial through `submit`, not
hard-coded — so a future licensed identity move turns these tests red
until the live documents move inside the same envelope, which is the
intended coupling. The checks are presence-only: historical values may
appear anywhere prose legitimately narrates history, and historical
reports are not read at all.

## 4. Pre-registered red

Predicted, before the test exists: all three tests fail at `5884f27`.

- Test 1 fails because `AGENTS.md` contains none of the four current
  identity strings.
- Test 2 fails because `HANDOUT.md` quotes the `10v4` envelope of
  2026-08-14.
- Test 3 fails because the Current position section quotes the same
  dead envelope.

If any of the three unexpectedly passes at base, the drift table above
is wrong, and that finding goes in the log before anything is repaired.

---

# Evidence (appended after pre-registration)

## E1. The red, verbatim (against `5884f27` documents)

All three tests failed at the pre-registration state of the live
documents, exactly as predicted in §4, under
`rustc 1.97.1 (8bab26f4f 2026-07-14)` / `cargo 1.97.1`:

```text
AGENTS.md is stale live law: it does not name 0x7dd8c6706e0b949f, 0xfa37eefa3594cfe3, 0x7e62152622bb9132, 0x93afba3f312bd89d
HANDOUT.md is stale live law: it does not name 0x7dd8c6706e0b949f, 0xfa37eefa3594cfe3, 0x7e62152622bb9132, 0x93afba3f312bd89d, 0xc0b4da51744bcf19, 0xb500dee0e5d883d8, oracles=10v7
runtime-target-map.md "Current position" is stale live law: it does not name 0x7dd8c6706e0b949f, 0xfa37eefa3594cfe3, 0x7e62152622bb9132, 0x93afba3f312bd89d, 0xc0b4da51744bcf19, 0xb500dee0e5d883d8, oracles=10v7

failures:
    conformance::agents_md_freezes_the_identities_the_code_enforces
    conformance::handout_identity_block_is_the_current_envelope
    conformance::target_map_current_position_quotes_the_current_envelope

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 87 filtered out; finished in 0.08s
```

Note what the red itself proves: the values the test recomputes through
`submit` are byte-identical to the envelope recorded in the W01 §E10
integration gate (`receipts=0xc0b4da51744bcf19`,
`world=0xb500dee0e5d883d8`) — the recomputation is measuring the same
trial the gate certified, not a private one.

## E2. The green

After repairing exactly the three live documents (AGENTS §3 gate and
§4 frozen identities; HANDOUT identity block, gate paragraph, and
standing state; target-map Current position, T01 command count, and
held/integrated evidence table):

```text
test conformance::agents_md_freezes_the_identities_the_code_enforces ... ok
test conformance::handout_identity_block_is_the_current_envelope ... ok
test conformance::target_map_current_position_quotes_the_current_envelope ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 87 filtered out
```

## E4. Claims table

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
|---|---|---|---|---|
| 1 | At `5884f27` no live document named any current canonical identity | the three live documents only | behavioral-red | E1 verbatim; the three pre-registered test names |
| 2 | After repair, the three live documents name the exact identities the code enforces | presence of the seven strings; not sufficiency of surrounding prose | measurement | the three green tests, E2 |
| 3 | The pinned values are recomputed through `submit` and equal the W01 §E10 integration envelope | standard trial only | derivation | `current_identities()` in `src/conformance.rs`; E1 closing note |
| 4 | No runtime identity or behavior moved in this trial | full four-configuration gate | measurement | E3 gate transcript; envelope lines byte-equal to W01 §E10.3 |
| 5 | Historical reports are untouched and unread by the pin | repo diff against `5884f27` | measurement | `git diff --stat 5884f27..tested_commit`; the three `include_str!` lines are the pin's whole read surface |
| 6 | 013, 014 and E01 are integrated; only 011 and 012 remain unmerged of the held table | branch topology at origin | measurement | merges `9a766ca`, `8c81454` (verification `5fc8376`), `1f3cbc6`; `git merge-base --is-ancestor` negative for `5b52e81`, `ab45f40` |

## E5. Bundle metadata

- Author: lead (this session); dispatch: author 2026-08-22, as amended
  by Sol's review (name, `red_required: yes`, full-drift scope,
  historical reports untouched).
- Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `cargo 1.97.1`,
  `--locked` throughout.
- Shared-assumptions note: the agent that wrote this pin also wrote
  the document repairs it certifies, and the same session verified the
  branch topology claims. The reviewer should re-derive the red on
  `5884f27`, the green at the tested commit, and claim 6's ancestry
  checks independently rather than trusting any pasted output here.

## E3. The gate transcript (tested commit `2a93bfb`)

`tested_commit`: **`2a93bfb`** — the clean-tree tip carrying the pin,
the repaired live documents, the decision packet and the O01 draft.
This section is appended by a documentation-only commit after the gate
and is re-gated at exact tip before merge, per the W01 §E10 precedent.

Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `cargo 1.97.1`,
every cargo step `--locked`. Every step judged by its own exit code;
no gate command ran through a pipe:

```text
TREE CLEAN at 2a93bfb
STEP fmt EXIT=0
STEP clippy-default EXIT=0
STEP clippy-host EXIT=0
STEP clippy-render EXIT=0
STEP clippy-taste EXIT=0
STEP test-default EXIT=0    (90 passed)
STEP test-host EXIT=0       (100 passed)
STEP test-render EXIT=0     (108 passed)
STEP test-taste EXIT=0      (114 passed)
STEP run-standard EXIT=0
STEP run-winter EXIT=0
GATE OVERALL: GREEN
```

Hosted standard run:

```text
bevy_host_parity receipts_match=true state_match=true world_match=true receipts=0xc0b4da51744bcf19 world=0xb500dee0e5d883d8
envelope baseline_commit=2a93bfb grammar=0x7dd8c6706e0b949f cmdfmt=0xfa37eefa3594cfe3 rcptfmt=0x7e62152622bb9132 fixture=0x93afba3f312bd89d receipts=0xc0b4da51744bcf19 world=0xb500dee0e5d883d8 oracles=10v7
```

Hosted winter run:

```text
winter_host_parity plan=A receipts_match=true state_match=true world_match=true attested_transfers=1 receipts=0x260782d648ffef68 world=0x8955528b452a8dde
envelope scene=W01 plan=A baseline_commit=2a93bfb grammar=0x7dd8c6706e0b949f cmdfmt=0xfa37eefa3594cfe3 rcptfmt=0x7e62152622bb9132 fixture=0x288ef6dbfad7e800 receipts=0x260782d648ffef68 world=0x8955528b452a8dde oracles=10v7
winter_host_parity plan=B receipts_match=true state_match=true world_match=true attested_transfers=1 receipts=0xb67361c3ef45ffca world=0xd2b2803a6c1b77d7
envelope scene=W01 plan=B baseline_commit=2a93bfb grammar=0x7dd8c6706e0b949f cmdfmt=0xfa37eefa3594cfe3 rcptfmt=0x7e62152622bb9132 fixture=0x1ac3857f928579d5 receipts=0xb67361c3ef45ffca world=0xd2b2803a6c1b77d7 oracles=10v7
winter_host_parity plan=C receipts_match=true state_match=true world_match=true attested_transfers=1 receipts=0xf6528f6ae509bdb4 world=0xb898728c0ccd0b48
envelope scene=W01 plan=C baseline_commit=2a93bfb grammar=0x7dd8c6706e0b949f cmdfmt=0xfa37eefa3594cfe3 rcptfmt=0x7e62152622bb9132 fixture=0x209b45a394eddc4a receipts=0xf6528f6ae509bdb4 world=0xb898728c0ccd0b48 oracles=10v7
```

Every identity, parity and envelope value is byte-identical to the W01
§E10.3 integration gate except `baseline_commit`, which names this
tip. Nothing moved.
