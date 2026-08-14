# Coordination map — 2026-08-14, post-RS01 handoff

One page, verified against actual git refs at write time — not memory.
Purpose: kill cross-agent confusion before it costs integration time.
Derived, non-authoritative; when this disagrees with refs, refs win.

## Where every piece of work actually is

| Location | Ref / tip | Content | Status |
| --- | --- | --- | --- |
| Remote `master` | `fca5237` | Trials 001–010, R01–R03, D01, TS01, laws, HANDOUT | Canonical. Untouched by RS01. |
| Remote `claude/truth-layer-scaffold-verify-2fhvhp` | `b384048` | `b2436cb` cold-repro-001 evidence + `b384048` superseded RS01 partial (labeled) | **Evidence archive — never merge as code** (see disposition below) |
| Remote `agent/*` (6), `trial/*` (8) | various | Dispositions already recorded in `docs/runtime-target-map.md` / HANDOUT | Unchanged by RS01 |
| Remote `run/RS01` | **does not exist yet** | — | The one missing push |
| Author's machine, truth repo | `d43927c` in `rs01-reference-d43927c.bundle`, SHA-256 `e8e78d10…21a3cebb` | **The canonical RS01 candidate**: live human-verified walkthrough, 73 renderer tests, fact map, refined default view | Bundle verified locally; **not yet reachable from any remote** |
| Author's machine, companion repo | `0fc3066` (local `run/RS01`) | RS01-VISUAL-REFERENCE alignment + provenance + contact sheet | Authorized for push; push still pending |
| Remote cold container (this session) | clean at `b384048` | Nothing unpushed; local scratch branches die with the container | No dangling state |

## Actual conflict inventory (the honest count)

- **Code conflicts on the remote right now: zero.** Master is untouched,
  `run/RS01` is unclaimed, the claude/ branch is self-contained.
- **One future conflict exists and is hereby defused:** `b384048`
  contains a superseded partial RS01 touching `Cargo.toml`,
  `src/host_bevy.rs`, `src/main.rs`. If it were ever merged after
  `e666cb6` lands, it would collide. Disposition: the claude/ branch is
  an **evidence archive**. Never merge it into master. If the
  cold-repro-001 document (`b2436cb`) is wanted on master, cherry-pick
  that single doc commit; discard `b384048`'s code by simply never
  merging it.
- **Duplicate work already happened** (this session's parallel renderer
  vs. the local session's). Sunk cost, honestly recorded; it left no
  repo-state conflict. Root cause: one dispatch executed by two
  machines without a claim step. Cheap fix going forward: a dispatch
  names its single executor, or the executor claims it by pushing the
  branch name empty before starting.

## Identity correction (2026-08-14, later same day)

The local session originally reported its RS01 tip as `e666cb6`. That
hash was subsequently found in **no** object store — not the author's
local repo, not GitHub, not this environment (which never held it).
The verified artifact is `d43927c` inside
`rs01-reference-d43927c.bundle` (SHA-256
`e8e78d100fa754e5dd532ac71b5b796a1cd79117c541b0aa7dd9212421a3cebb`).
`e666cb6` is a dead label — most likely a misreported or
rewritten-away hash — and appears in earlier records (including the
first revision of this map) only as history. Lesson, doctrine-grade:
an agent-reported commit identity enters coordination records only
after `git cat-file`-level verification against a reachable ref.

## Open decisions, in order (everything else is noise)

1. **Make `d43927c` reachable**: push `run/RS01` at `d43927c` to the
   truth remote (already authorized), and the companion branch to its
   remote. The bundle route also works but committing bundles into git
   is the wart already flagged — a pushed ref is cleaner.
2. **Cold-verify `d43927c`** in the remote environment (dispatched;
   blocked only on step 1; verification will also check the bundle
   SHA-256 above if the bundle route is used). Environment is ready:
   rustc 1.97.1, lavapipe, Xvfb, windowing libs, bevy 0.19 cached.
3. **Review check on `d43927c`:** does it formally carry the envelope
   discrepancy record (single-actor arc 65→60→48 unrepresentable:
   `cannot_witness_own_claim` + holder-only gather) and a test pinning
   it? If not, lift both from `b384048`'s `docs/rs01-trial-log.md` and
   `src/rs01_fixture.rs` — that is the one artifact of the superseded
   partial worth carrying forward.
4. **Claude/ branch disposition** as above (archive; optional
   cherry-pick of `b2436cb`).
5. Then the queue that predates RS01, unchanged: Meaning Gate verdicts
   on 011/012/013 (still the critical path), 014, MB01, turn-contract.

## What this map is not

It assigns no meaning, changes no law, and moves no value. It is a
snapshot for the author and both agent lanes so that "who has what"
never again needs reconstructing from three memories.
