# Lead iteration map — 2026-08-14

Author dispatch: review E01, then map the lead's recommendations for the
next iterations — importance, the shapes to form and test against, one
most-load-bearing verb pick, and scaffolding notes from this week. This
document is a **proposal for author + drift-master review**; nothing in it
is ratified, and no value, registry, schema, contract, or closed-vocabulary
entry is created here. Candidate names below are design language only.

Branch: `agent/lead-iteration-map-2026-08-14`. Base: `9a766ca`.
Author: Fable 5 (lead).

## 1. E01 exact-tip review verdict

`trial/E01-belief-actionability-taste@857b0e6` — **lead ACCEPT**.

Independently re-derived on the exact tip: format, all four strict clippy
suites, tests 58 / 67 / 75 / 81, pure and host envelopes byte-frozen
(`grammar=0x530003916889b952 fixture=0x3805f1e20c001051
receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v4`).
Code review findings:

- Authority boundaries hold. Scene facts and belief inputs come only from
  identified `Publication`s with per-fact `derived_from` identity checks;
  outcomes come only from canonical receipts; the overlay has no command,
  mutation, or outcome-selection path. The retained pre-action
  `belief_source` correctly preserves the belief that caused the action.
- The six focused falsification tests cover the right surfaces, including
  the self-referential source scan for canonical-observation backdoors and
  the sealed-holdout exclusion.
- Minor note, no change requested now: `e01-capture` deletes stale PNGs
  instead of refusing (TS02-style). Acceptable for a scratch capture
  directory; if capture sets become formal evidence, adopt fail-closed
  output plus a hash manifest (see scaffolding note S3).
- The fixture-local belief policy (`confidence at >= 14`) is honestly
  labeled. It must not silently become the belief model; a real belief
  owner remains an E-layer decision.

Merge recommendation: E01 sits on current master and is ready to merge as
the next integration step once the author names it. Nothing blocks it.

## 2. Where the estate stands

Master `9a766ca` now carries 014 (order-invariant anticipation proof) and
013 (H-A verdict preparation; holdout still sealed/unrevealed/unexecuted).
E01 proves the belief/actionability edge is *legible to a player*. The
ratified frame is 880–1050 oral law; the two locked foundation rulings
(story emergence from pressure × drive × anticipation × strategy ×
constraint; the world never lies) still need their repo home. The workbook
is the quarry; scenes carry the load.

## 3. Iteration map — ordered, with shapes to form against

Each entry: why it is load-bearing, what gates it, and the falsifiable
shape(s) Codex sprints should build tests against. Shapes are candidates
in the H-A/H-B style: the sprint formalizes them; the author selects.

### I0 — Chronology and player-seat cluster (author decision, not a sprint)

Workbook questions 08 + 11 + 13. This is the single decision that changes
almost every downstream map, economy, autonomy, and law sprint, and it is
answer-page work, not agent work. What sprints CAN prepare against it:

- **Shape T-A (event-ledger time):** world-time advances only through
  explicit typed transitions; every receipt carries duration/tick binding;
  no background clock. Falsifier: any canonical change without a named
  advance, or any render-frame influence on truth.
- **Shape T-B (canonical tick clock):** a tick owner advances time; verbs
  bind to tick spans. Falsifier: same replay/permutation invariants under
  reordered submission within a tick.
- **Seat shape S-A:** the seat is a canonical right bound to a living
  character; succession is a typed transition with witnesses; game-over is
  the absence of any legal continuity path, not a stamina number.

Recommendation: prepare both T-shapes as paper falsifier sets only; no
time code before the author answers 08.

### I1 — RES01: resource kinds (inventory opens now)

E01 closed the condition the author set ("inventory out of scope until
the belief/action loop is legible"). Current truth has one undifferentiated
mass. Every quarry scene (hay, livestock, timber, peat, food preservation)
needs *kinds* — this is the highest-value hardening step and it touches
grammar identity, so it must be one conscious, permissioned move.

- **Shape:** a closed resource-kind vocabulary (start minimal: the winter
  scene's needs only); per-kind conservation proven by the existing
  mass-conservation oracle generalized per kind; gather binds site kind;
  receipts carry kind. Grammar identity moves once, consciously, with the
  precomputed fingerprint named in the pre-registration (013 pattern).
- **Falsifiers:** cross-kind leakage (kind A consumed, kind B appears);
  totals per kind not conserved; any kind admitted outside the registry
  permission path; identity bumped manually.
- **Gate:** explicit author permission for the registry/schema change and
  the first kind list. Historical labels (hay etc.) route through H01
  admission discipline before they claim historicity.

### I2 — V01: the Give verb (the lead's load-bearing verb pick — see §4)

### I3 — R10: keyed per-decision randomness

Unchanged from the earlier lead envelope, restated as the sprint target:
`choice_key = H(campaign_seed, canonical_decision_identity, draw_purpose,
draw_index)`, selecting only among already-legal candidates.

- **Falsifiers:** replay identity; enumeration-order independence (014
  precedent); legality (a draw can never select a guard-refused command);
  no global cursor (refusals consume nothing; consumption serialized);
  host scheduling/frame/worker isolation from the key; out-of-domain
  rejection, never clamping.
- **Author ratifications inside it:** mixer choice, seed owner and
  persistence, decision-identity fields, closed draw-purpose vocabulary,
  index rule.
- Behavior variance anywhere (story, autonomy, combat) waits on this.

### I4 — W01: the winter-crisis scene fixture

The first quarry scene made executable: one household, short hay, cattle,
damaged roof, storage pressure, delegated seasonal work. Needs I1 (+I2 to
be interesting; time from I0 for seasons — a bounded "one winter step"
fixture can precede full world-time).

- **Shape:** pre-registered scene predictions in the 013 style: exact
  consumption/production chains, then the pressure map (what runs out
  first, which alternatives exist). The author's caution is a falsifier,
  not a flavor: losing the dairy herd must *threaten the preservation
  chain and force painful alternatives* — any fixture where herd loss is
  immediate game-over fails the shape.
- This scene is also the honest test of I1's kind list: if the scene
  cannot be expressed, the kind list was wrong, and that is evidence.

### I5 — LW01: Lawcraft capability state

Authorized in principle by the author; needs its identity rebaseline and
a first bounded shape: remembered-law as per-character capability state,
separate from law content and from office/verdict authority.

- **Falsifiers:** lawcraft can never change what IS legal (only what the
  actor knows/can use); no omniscience (capability bounded by the actor's
  observation history once E-layers exist); identity rebaseline explicit.

### I6 — O01: request/order lifecycle with a bounded charter

The delegation pushback becomes the shape: the head authorizes projects,
budgets, priorities inside a **typed charter with explicit bounds and
expiry**; supervisors schedule inside those bounds; individuals execute,
improvise within bounds, or refuse (`actor_unwilling` as auditable
refusal).

- **Falsifiers:** no permanent loophole (an expired or exceeded charter
  refuses); no micromanagement bypass (work proceeds inside a valid
  charter without per-act head commands); refusal leaves zero mutation;
  every consequence has a reconstructable Why-chain naming the charter.
- **Presence-cost binding (author direction):** orders continue to flow
  while the head travels; *new* building/production-tree starts refuse
  without head presence; expedition verbs cost presence plus the named
  army, and people in the army are simultaneously unavailable at home
  (person-conservation, same family as the mass oracle).
- Gates: E-layer owner decisions, R10, H02-order dossier for any
  historical claim.

### I7 — MAP01 line: located opportunity

When MAP01 is pushed and reviewed: claimed land adds **located**
opportunities (grazing, timber, shoreline, bog iron, water, routes) that
still require presence, travel, labor, buildings, and storage — never
un-located resources into a global pool.

- **Falsifiers:** removing location from a site must break its use (no
  "dome" fallback); claim-axis separation (survey capacity, assertion,
  witnesses/credibility, legal recognition, exploitation capacity are
  separately falsifiable — the strong runner must not automatically be
  the strongest landowner).
- The early numbers (yearly claim, 3/4–6/10 squares, +3/−1 quality) stay
  labeled trial hypotheses, per the author's own review.

## 4. The load-bearing verb: `Give`

One verb pick, as dispatched: **`Give` — a witnessed, voluntary transfer
of a named mass between two characters.**

Why it carries the most load in the current form:

1. **It is the smallest social verb the current truth can afford.** The
   economy owner already tracks per-character inventories; social already
   owns witnessing. `Give { from, to, mass }` needs no new owner, no
   epistemic machinery, and no randomness — it is the third verb that
   turns a single-actor economy into a *social* economy.
2. **Every named scene runs through it.** Winter crisis: hay shared,
   lent, or begged. Land dispute: compensation pressure needs a legal
   transfer to exist. Feast/gift politics (H01-C07/C08): a feast is
   materially a structured sequence of gives with witnesses. Favor-debts
   begin as remembered gives.
3. **It creates the first legal/illegal contrast without new law.** The
   illegal shape already exists as refusals (gather against another's
   claim). Give supplies the legal counterpart, so the hay-theft story
   fixture becomes expressible as a *choice between a legal and an
   illegal path* — exactly the story loop's bounded-intention step.
4. **Its falsifier set is short and hard:**
   - conservation: giver decreases exactly what receiver gains, same
     kind (with I1) — the existing oracle family extends;
   - consent: `Give` requires the giver as actor; no verb moves another
     person's stack (theft stays a refusal until law says otherwise);
   - witnessing: witnessed/unwitnessed gives are separately receipted —
     the unwitnessed give is the future rumor/dispute seed, but that
     meaning waits for E-layers;
   - zero-mutation refusals (insufficient stock, unknown recipient,
     self-give) byte-stable;
   - permutation/replay invariance as always.

Runner-up, explicitly second: `Request` (O01's core). It unlocks
delegation but is only honest after E-layer ownership and R10; Give is
honest *today*.

## 5. Scaffolding notes from this week (S1–S7)

- **S1 — Commit the red.** Twice now the red-only probe source was not
  committed (014 original, 014 permutation red) and had to be
  reconstructed byte-exactly to verify. Rule: every red ships either its
  probe source or a written reconstruction recipe in the same bundle.
- **S2 — Rebase reference map.** The 014 rebase left its evidence naming
  vanished history; the appended old→new commit map (`5fc8376`) closed
  it. Adopt as standard: any rebased evidence branch appends a reference
  map before review.
- **S3 — Evidence needs a provenance home.** `docx_work/` and
  `taste-evidence/` live outside version control. The workbook generator
  (with its overwrite refusal + a refusal test, and a lazy `docx` import
  so the refusal is testable without python-docx) and capture manifests
  (SHA-256 tables like E01's, tool-generated) belong compute-side under
  the TS02 provenance pattern.
- **S4 — Merge preflight as a proofer mode (TP01-v4 candidate).** TASTE01
  proved the value of gating the *union tree* before merging. A proofer
  flag that builds the candidate merge tree for named branches and runs
  the full gate on it would mechanize exactly what was done by hand.
- **S5 — Per-trial evidence files.** The shared `trial-log.md` interleaves
  awkwardly across rebases (014's sections now sit mid-file). The 013/E01
  pattern — one report file per trial, trial-log as index — scales;
  recommend making it the default for new trials.
- **S6 — Worktree/branch lifecycle.** Nine-plus worktrees accumulate.
  Convention: after a branch merges, its worktree is removed and the
  branch archived (tag or `archive/` namespace) in the same session, so
  the live worktree list equals the live review surface.
- **S7 — Foundation rulings need their repo home.** The two locked
  rulings (story emergence; the world never lies) plus the presence-cost
  direction are conversation-only. One docs-only branch in the H01/VS01
  author-verdict style records them without runtime values. Ready on the
  author's word.

## 6. The lead's vital picks

In order, if only three sprints run next:

1. **I1 RES01 resource kinds** — unblocks every scene; one conscious
   grammar move.
2. **I2 V01 Give** — the third verb; social economy; both scenes need it.
3. **I3 R10 keyed randomness** — everything with variance waits on it.

With I0 (chronology/seat) answered on the author's pages in parallel —
it is the highest-importance item overall but is a decision, not a
sprint.

## 7. Open author decisions (standing list)

1. Chronology cluster 08 + 11 + 13 (blocks I0-dependent work).
2. E01 merge word.
3. MAP01 push + review dispatch.
4. RES01 registry permission and first kind list (when proposed).
5. Foundation-rulings docs dispatch (S7).
6. RS01-human: rubric confirmation, sample sizes, scorer count.
7. Slavery scope and representation constraints (H01 question 4).
