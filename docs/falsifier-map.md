# Falsifier map — overnight execution plan

Date: 2026-08-09. Baseline: `ba3eaac` (master, after trial/006).
Source of the falsifiers: `docs/falsification-defier-audit.md` (defiers
2, 3, 5; defiers 1 is done — trial/006 — and 4, 6 are contingent, see
bottom). Audience: an automated collaborator (Codex) executing trials
overnight; the author reviews and merges in the morning.

## Standing rules (all trials)

1. Branch per trial from `ba3eaac`: `trial/007-transition-domain`,
   `trial/008-apply-totality`, `trial/009-language-seam`. One worktree
   per branch, outside the primary checkout, each with its own untracked
   `.cargo/config.toml` setting a distinct `build.target-dir`.
2. **Red first.** Capture the falsifier failing against unmodified code
   before any fix; quote the evidence verbatim in `docs/trial-log.md`.
   Where no behavioral red is possible without staging a bug, label the
   capability red honestly (compile error or absent harness), as
   trials 002 and 006 did.
3. **Envelope discipline.** `grammar` and `fixture` must remain
   `0x530003916889b952` / `0x3805f1e20c001051` — none of these trials
   has a value-pressure license. Receipts must stay byte-identical
   unless a spec evolution is declared in the log. Any oracle behavior
   change bumps `ORACLE_SUITE_VERSION` (currently 3).
4. **Zero new dependencies** in the default build. Test-only helpers
   (trace generators, parsers) are written by hand; deterministic seeds
   only, printed with the evidence so every run is reproducible.
5. **Gate before handoff**, both feature sets:
   `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`;
   same with `--features bevy-host`; `cargo test`; `cargo test
   --features bevy-host`; `BASELINE_COMMIT=$(git rev-parse --short
   HEAD) cargo run --features bevy-host` exit 0.
6. **Do not merge, do not push master.** Leave each branch with its
   trial-log entry and evidence for morning review; serial merges are
   the author's call. Judge-affecting work (008 if it changes an
   oracle) merges before host-surface work, as before.

## trial/007-transition-domain (audit defier 2, priority 1)

**Hypothesis.** Trial/002 proved host parity only for the recorded
16-command trace. Two deterministic implementations can agree on every
visited (state, command) pair and differ on the first unvisited one.

**Falsifier.** A deterministic bounded trace harness, feature-gated
behind `bevy-host`, test-only:

- Command space: every `Gather{actor, claim, site}` and
  `Witness{witness, claim}` over actors 1–4 (plus one unknown id 9),
  claims 1–9 (plus 99), sites 1–4 (plus 9) — illegal commands are
  legal inputs; refusals are outcomes.
- Generator: hand-rolled seeded LCG; N ≥ 1,000 traces of depth ≥ 32
  from the standard fixture. Print the seed and parameters with the
  result.
- For each trace: pure run vs `run_hosted`, compare canonical receipt
  lines AND `canonical_state()` (exact, per trial/006 — never hash
  alone).
- On divergence: shrink by command removal to a minimal counterexample
  and record it as the red; the fix is then whatever the counterexample
  demands.

**Done means:** either a minimal divergence recorded red→green, or a
green harness with the claim stated exactly: "parity holds for the
enumerated trace set (seed S, N traces × depth D)" — trace-scoped, not
universal. The claim in README/architecture must not grow beyond that.

## trial/008-apply-totality (audit defier 4-rule / falsifier 3, priority 2)

**Hypothesis.** Preflight guarantees zero mutation on stale plans, but
post-preflight applies are only total if no step can fail for validated
inputs. Suspect list, in order:

1. `MassGrams::saturating_add` in `apply_extract` (inventory) and in
   `total_mass` — saturation at `u64::MAX` is a **silent clamp**, the
   same defect class as the round-1 stamina clamp. A seeded world near
   the bound can lose mass silently, and if `total_mass` saturates
   identically, `mass_conserved` may stay green — a lie the oracle
   cannot see. This red is likely constructible: seed
   `MassGrams::new(u64::MAX)` stock plus a nonzero inventory and
   extract. Capture it.
2. Entity/owner revision `+= 1` overflow (u64) — argue unreachability
   in the log rather than adding machinery.
3. `expect` calls in applies ("fresh token: …") — for each, state in a
   comment or the log WHY the preflighted input cannot reach it, or
   demonstrate a reachable panic as red.

**Fix shape (if red confirms #1):** bound total seeded mass at
coherence-validation time (`validate_world_coherence` returns a new
closed `FixtureFault` on checked-sum overflow) so applies can use
plain checked arithmetic that is provably total under the bound.
That is a fixture-fault vocabulary addition — declare it as spec
evolution; it does not touch receipts or grammar.

**Done means:** every apply step either has a totality argument pinned
by a test or had its red captured and fixed. `mass_conserved` must use
non-saturating summation or carry a bound argument.

## trial/009-language-seam (audit defier 5, priority 3)

**Hypothesis.** A foreign source can lose meaning before the boundary:
units, rounding, ordering, encoding, numeric normalization. Receipt
parity cannot see it because the canonical command is already wrong.

**Falsifier.** Name the observation point and defend it:

- Extract `Command::canonical_bytes()` (the encoding already inside
  `fixture_identity`) as the seam artifact.
- Build a minimal text-line ingestion path (hand-rolled parser, test
  code): `gather actor=1 claim=1 site=1` / `witness witness=3 claim=8`.
- Adversarial fixtures, each expecting **reject-or-byte-identical,
  never silent normalization**: leading `+` (Rust's `u64::from_str`
  accepts it — likely genuine red), leading zeros, whitespace variants,
  unicode digits, u64::MAX and MAX+1, reordered fields, duplicate
  fields, unknown fields, empty values, BOM.
- Parse faults are a closed enum (like `FixtureFault`), never a
  panic and never a coerced value.

**Done means:** the parser refuses every adversarial fixture or
produces bytes identical to the hand-constructed command; the red
(silently accepted `+15` or similar) is captured first; the log states
that receipt parity claims begin only after this observation point.

## Contingent — do NOT run tonight

- **Audit falsifier 4 (write-skew on aggregate invariants):** waits for
  the first real cross-entity invariant. Writing it now would invent
  speculative machinery, which the audit explicitly rejects.
- **Audit falsifier 6 (value holdout):** waits for the first named
  value-pressure target. The holdout must be pre-registered before any
  value branch sees its result; there is nothing to register yet.

## Morning handoff checklist (author)

Per branch: red evidence verbatim in the log? Fix minimal and inside
ownership boundaries? Envelope fields unchanged except declared judge
bumps? Gate green both feature sets? Then merge serially
(judge-affecting first), re-gate master after each, push once.
