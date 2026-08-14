# Sprint and dependency map — 2026-08-14

Status: **DERIVED REVIEW MAP — non-authoritative.** This document consolidates
the reachable evidence estate into a lead-reviewable work order. It changes no
runtime contract, registry/schema, closed vocabulary, value, authority identity,
holdout, or gameplay meaning. Authoritative status remains in
runtime-target-map.md; semantic promotion remains governed by meaning-gate.md.

## Authoring envelope

~~~text
base_commit:         2dd4db5db6f52b287ebf4f6b8a3d259bf30ba028
objective:           map every reachable open or historical lane to a disposition
                     and dependency order that lets the lead refine the next sprints
authoritative_files: AGENTS.md; docs/README.md; docs/architecture.md;
                     docs/runtime-contract-proposal.md; docs/runtime-target-map.md;
                     docs/meaning-gate.md; docs/development-workflow.md
write_scope:         docs/sprint-dependency-map-2026-08-14.md; docs/README.md
frozen:              runtime code; contracts; registry/schema; closed vocabularies;
                     values; authority identities; sealed holdouts; A1 runtime law
red_required:        no — this is documentation/measurement consolidation; its
                     falsifier is a contradictory reachable ref, diff, or gate result
verification:        clean-tree full default, bevy-host, and bevy-render gates;
                     host runtime probes; structural diff and reference review
evidence:            exact ref snapshot, reproduced gate matrix, dispositions,
                     ordered sprint graph, claim table
limits:              no branch merges, holdout reveal, value promotion, or historical
                     claim promotion
escalate_when:       meaning, RNG authority, contract/schema change, new canonical
                     type, closed-vocabulary evolution, or merge authority is needed
tested_commit:       final pushed branch tip named in the review handoff;
                     clean-tree gates rerun after commit
~~~

## Cold estate snapshot

Snapshot date: 2026-08-14. Reachability here means reachable from the named ref
in this audit's object store; it is not a claim about every clone.

| Surface | Verified identity | Reading |
| --- | --- | --- |
| Truth master / origin/master | 2dd4db5 | Clean canonical baseline; includes TS01, bounded live RS01, and integration consolidation |
| Canonical live RS01 | e666cb6, ancestor of cab61be | Verified reachable; renderer remains downstream of Host/Publication |
| Compute master / origin/master | a508276 | All named TS01/MB01/DOS01/NM00/RS00/RS01/TP01 run tips are ancestors |
| Truth proofer | initial 986c5cc; current 246dd25 | --render reproduced 37 PASS / 0 FAIL; current provenance scope is local refs/heads/run/*, not all remote run refs |
| Trial 014 review response | afbae24390be55820ebb52c5a68dd6376d71e553 | Clean, pushed, exact-tip independent re-review pending |
| Cold evidence branch | 4661ef4 | Archive evidence only; its coordination map is stale |
| Local bundle | SHA-256 71a4b3634c1310846c01dcc6c86441c5124726b4ddd26d80fa9a2ff4afee82e6 | Valid complete bundle ending at old f5728d6; contains neither RS01 candidate |
| Reported d43927c | unverified (source: prior handoff) | Never call it dead or refuted without a scope-complete search; it is not an integration input |

Identity discipline has exactly three useful states: verified-reachable(ref),
unverified(source named), and refuted(search scope named). “Not reachable here
now” is only the second state.

## Reproduced proof matrix

All branch proofs below used isolated detached worktrees and separate build
caches. Standard gates did not execute or reveal the trial/013 holdout.

| Tip | Structural result | Default tests | bevy-host tests | bevy-render tests | Runtime/evidence result |
| --- | --- | ---: | ---: | ---: | --- |
| master@2dd4db5 | clean | 56 | 65 | 73 | Ten oracles, host parity, frozen envelope; TP01 render estate proof green |
| trial/011@b2648a6 | clean | 48 | 50 | not claimed | Runtime probes/envelope green; threshold-chain falsifier reproduced |
| trial/012@24c5524 | **red**: six committed conflict markers in docs/trial-log.md | 47 | 49 | not claimed | Code/runtime green, branch as a merge unit invalid; standalone derivation remains readable |
| trial/013@d7fdd1b | clean | 48 | 50 | not claimed | Disclosed training trace green; verdict inconclusive; holdout sealed/unexecuted |
| trial/014@afbae24 | clean | 49 | 58 | not claimed | Ten oracles and host parity green; exact-tip re-review still required |

Lower counts on older trial branches reflect older branch points, not
regressions against current master. Every integration candidate must be
rebased serially and re-gated at the new tip.

## Disposition of every open lane

| Branch/artifact | Weight retained | Disposition before any merge | Blocking dependency |
| --- | --- | --- | --- |
| trial/014@afbae24 | Order-invariant test-only ranking over two already-legal intents; reviewer findings answered with reproducible red/green | **RE-REVIEW EXACT TIP**, then author may name it for merge | Independent verdict on afbae24; final author Meaning Gate ruling |
| trial/011@b2648a6 | Exact Low dead interval and threshold/dominance-chain measurements | **REBASE + CROSS-REVIEW**; eligible as test/evidence only if approved | Post-014 baseline and independent review |
| trial/012@24c5524 | Clean standalone proof that the active yield matrix is rank one | **SALVAGE REPORT ONLY** into a fresh branch; never merge the conflicted branch | Author chooses what question the falsified shape opens; fresh review |
| trial/013@d7fdd1b | Training evidence separating exhaustion, affordability, and accepted yield | **HOLD**; no holdout execution or integration | Author meaning ruling and explicit holdout dispatch |
| agent/turn-contract@2717ffb | Parallel planning ≠ parallel mutation; composite preflight; scheduler independence; possible degenerate bridge | **ARCHIVE AS CANDIDATE SPEC** | A2 contradicts ratified A1; contention, Preempted, and RNG lack authority |
| agent/sprint-007-012-overview@6f1cd7b | Useful pressure vector (truth, reachability, sensitivity, holdout) and historical measurements | **SUPERSEDE**, retaining cited measurements | Tips and integration order are stale; this map replaces coordination claims |
| claude/truth-layer-scaffold-verify-2fhvhp@4661ef4 | Cold-reproduction evidence and corrected identity-state lesson | **ARCHIVE**, do not whole-merge | Stale coordination and superseded RS01 reading |
| compute remote run/* refs | Provenance and review history | **RETAIN AS EVIDENCE REFS**; code already integrated into compute master | TP01 must state whether its universe is local or remote refs |
| old bundle at f5728d6 | Portable historical baseline | **ARCHIVE BY HASH** | Predates current estate; neither live RS01 candidate is inside |

Discarding a merge unit does not discard its evidence. Salvage means extracting
only an independently reviewable report or test onto a fresh, current branch,
with the original branch retained as provenance.

## Load-bearing results already available

### Trial 011 — threshold topology

The reproduced focused test shows Low gathering refused at stamina 10–14,
accepted at 15 for mass 600, and that the 39→40 and 79→80 boundaries change
transition-chain behavior. This is evidence of cliffs and a dead interval; it
does not choose whether those cliffs are desirable.

### Trial 012 — confluence shape falsified

The active yield matrix is exactly:

~~~text
[1, 2, 3]^T × [250, 400, 600, 900]
~~~

All eighteen two-by-two minors are zero. The attempted independent
character/site interaction hypothesis was falsified: the current surface is
separable rank one. Whether to preserve legibility or introduce interaction is
a Meaning Gate question, not a mathematical consequence.

### Trial 013 — meaning is still open

The disclosed trace separates actor exhaustion from insufficient stamina and
shows the first accepted Low gather at 15. It did not distinguish “Low is a
descriptive band” from “Low should afford an action.” The holdout remains
sealed, unrevealed, and unexecuted.

### Trial 014 — a reusable non-authority shape

The response tip derives intent kind from strict cost-and-yield dominance,
rejects ties and crossed trade-offs, hashes sorted commitment records, refuses
caps outside ±1, and produces an identical entire evaluated plan under input
reversal. It remains test-only. If accepted, it is precedent for deterministic
ranking of legal candidates, not authority for an AI planner or RNG.

## Two lanes, one author gate

~~~mermaid
flowchart LR
    M["Current truth master\n2dd4db5"] --> V14["014 exact-tip re-review"]
    V14 --> I14["Author-named 014 integration"]
    I14 --> V11["011 rebase + review"]
    V11 --> E11["Threshold evidence"]
    M --> S12["012 clean report salvage"]
    M --> H13["013 hold: meaning + holdout"]

    H0["Choose time and place"] --> H1["Source ledger"]
    H1 --> H2["Kin, honor, law, economy dossiers"]
    H2 --> H3["Triangulated mechanic hypotheses"]

    E11 --> G["Author Meaning Gate"]
    S12 --> G
    H13 --> G
    H3 --> G
    G --> X["One bounded cross-system trial"]
    X --> T["Truth implementation"]
    T --> P["Publication / renderer / TS02 views"]
~~~

The two lanes may gather evidence in parallel. They converge only when the
author defines competing meanings and a falsifier. Historical research cannot
silently select mechanics; code shape cannot silently select history.

## Ordered sprint map

### Phase 0 — close the reachable estate

1. **014-R:** independently re-derive the red on bd2f8ca and green on exact
   response tip afbae24, with verdict per claim. If accepted and the author
   explicitly names the branch, merge and run all three feature gates.
2. **011-R:** rebase onto the resulting master, retain test/measurement scope,
   cross-review red and green, then request author integration.
3. **012-S:** create a fresh docs-only salvage branch from current master;
   import only the standalone rank-one report, correct current refs, and
   cross-review. Do not copy the conflicted trial-log history.
4. **013-G:** author states the exact meaning question, competing hypotheses,
   directional prediction, and whether the sealed holdout is to be run. No
   automated queue may cross this gate.
5. **A-01:** label the turn-contract, stale overview, cold evidence branch, and
   old bundle as retained archive inputs, not active sprints.

### Phase 1 — safe parallel preparation

- **TP01-v3:** make proof scope explicit (local-run-refs, remote-run-refs, or
  both) and make render coverage unmistakable (--render required or a clearly
  named quick mode). This changes proof tooling, not truth.
- **TS02-shape:** derive YAML first and HTML second from authoritative truth
  observations. Both outputs are disposable views, contain provenance, and
  acquire no schema/registry authority.
- **RS01-human:** perform F1/F10 human visual checks on the bounded renderer;
  record presentation findings without changing truth.
- **H0/H1:** decide historical scope and build the claim ledger below.
  Research can proceed without inventing gameplay values.

### Phase 2 — author-selected first vertical slice

Choose one small scenario only after Phase 0 and H0/H1. A good first slice has
one existing legal economic command, one explicit kin/social observation, one
honor/reputation hypothesis, and one competing deterministic baseline. It must
state which owner validates each predicate and which receipt/publication
expresses the result. If a new canonical family or honor type is required,
stop for an explicit contract/registry decision before coding it.

### Phase 3 — randomness only after replay authority

Do not add RNG as flavor plumbing. First define and falsify its authority:

1. owner of seed and draw position;
2. canonical identity and serialized replay representation;
3. whether refused, retried, or invalid commands consume a draw;
4. scheduler-independent draw assignment;
5. recovery behavior around commit and publication;
6. rule that a draw may choose only among already-legal candidates.

Under current A1 law, the smallest candidate is a per-command sealed random
scope. That is a proposal for a future trial, not current law. The A2 turn
draft must not be smuggled in to obtain randomness.

## Deterministic cross-system interaction shape

Keep selection, truth, and expression separate:

1. **Observe:** a planner reads a versioned Publication; it never receives a
   mutable truth handle.
2. **Form candidates:** domain owners expose typed, read-only affordances or
   the planner constructs typed commands from public facts.
3. **Rank:** deterministic weights may compare already-legal candidates. Input
   order and scheduler order are non-authoritative; trial/014 is the current
   test-only model for this property.
4. **Submit:** exactly one typed Command crosses the boundary under A1.
5. **Validate complete read-set:** every economic, social, personal, kin, and
   honor predicate affecting legality belongs to a named owner and proof.
6. **Commit atomically:** only the boundary mutates canonical state. A ranking
   score cannot waive a refusal or manufacture a new outcome.
7. **Publish:** receipts and the next Publication are facts; UI, narrative,
   animation, and analytics remain replaceable expression.

Minimum falsifiers for any future weighted interaction:

- same canonical observation and candidate set produce byte-identical ranking
  and selection;
- permutation, serial/parallel planning, and host scheduler changes cannot
  alter the result;
- every weight names its source fact, normalization, unit, bound, and owner;
- illegal candidates stay illegal at every random draw;
- all cross-owner reads are included in freshness/preflight evidence;
- changing one declared fact affects only named predicates and score terms;
- replay reproduces both draw assignment and canonical receipts.

## Historical research lane

“Real Norse” must be narrowed before it becomes a constraint. The first author
choice is time and place: for example, Icelandic Commonwealth society
(c. 930–1262), settlement/Viking Age Iceland, or a deliberately broader Norse
comparison. Those scopes have different evidence and must not be flattened.

Create a source ledger with these fields:

~~~text
claim_id, claim_text, time_scope, geography, source_type,
event_or_material_date, manuscript_or_publication_date,
confidence, contradictions, mechanic_hypothesis, forbidden_inference
~~~

Then prepare five bounded dossiers:

1. kinship, household, fosterage, inheritance, and obligation;
2. honor, reputation, witnessing, feud, settlement, and compensation;
3. law, assemblies, chieftaincy, enforcement, and dispute procedure;
4. land, livestock, seasonal movement, fishing, craft, exchange, and scarcity;
5. feasting, gifts, hospitality, alliance, and chiefly/social power.

Triangulate source classes. Grágás is indispensable legal evidence, while its
main manuscripts are thirteenth-century witnesses to earlier law. The
Íslendingasögur are mostly thirteenth-century compositions preserved in later
copies, so they are evidence for narrated norms and social imagination, not
timestamped telemetry of every tenth-century act. Archaeology can constrain
farm economy, livestock, seasonal use, craft, food, and exchange, but cannot by
itself supply an actor's motive. Every mechanic hypothesis should state which
parts come from law, saga/literature, archaeology, or modern synthesis and what
would falsify the combination.

Starting references for the ledger:

- Árni Magnússon Institute, [Grágás](https://www.arnastofnun.is/is/greinar/gragas)
- Árni Magnússon Institute, [Möðruvallabók](https://arnastofnun.is/is/flettibok-modruvallabok)
- Cambridge, [The Cambridge History of the Viking World — excerpt](https://assets.cambridge.org/97811084/86811/excerpt/9781108486811_excerpt.pdf)
- University of Iceland, [Feasting in Viking Age Iceland](https://iris.hi.is/en/publications/feasting-in-viking-age-iceland-sustaining-a-chiefly-political-eco/)
- University of Iceland, [Pálstóftir: a Viking Age shieling](https://iris.hi.is/is/publications/palstoftir-a-viking-age-shieling-in-iceland/)
- University of Iceland, [Two Valleys — farm midden investigation](https://twovalleys.hi.is/wp-2-farm-midden-investigation/)

## Decisions requested from lead and author

1. Does exact afbae24 satisfy the trial/014 review, claim by claim, or is one
   further response required?
2. Should trial/011 integrate as test/evidence after rebase, with no value or
   meaning promotion?
3. For trial/012, is separability desirable legibility, or should an
   interaction hypothesis enter the Meaning Gate?
4. What exact competing meanings and stop condition govern trial/013, and may
   its sealed holdout be executed?
5. What is the initial historical time/place boundary?
6. Which one micro-scenario should be the first cross-system vertical slice?
7. Is future RNG canonical truth input/state, or an external choice that must
   be materialized as a typed command before submit?
8. May TP01-v3, TS02-shape, RS01-human, and H0/H1 proceed in parallel after
   their individual envelopes are reviewed?

## Overnight-ready queue policy

No standing merge or semantic authority is implied. After lead refinement,
the author may dispatch safe independent Phase 1 preparation and any explicitly
unblocked Phase 0 review/salvage item. Each gets its own branch, envelope,
build cache, cross-review, and terminal review bundle. Automatic work stops at
any Meaning Gate, holdout, contract/registry, closed-vocabulary, RNG, or
per-item merge decision.

## Claims table

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
| ---: | --- | --- | --- | --- |
| 1 | Truth master passed 56/65/73 tests with the frozen envelope | 2dd4db5 in this environment | measurement | TP01 --render; runtime-target-map.md current position |
| 2 | Trial/011's standard gates and threshold test reproduce | b2648a6 only | measurement | isolated worktree gate; falsification_threshold_edges_need_transition_chain_evidence |
| 3 | Trial/012 is not merge-clean because its committed trial log contains six conflict markers | 24c5524 only | measurement | git diff --check; docs/trial-log.md lines 93, 103, 120, 128, 185, 186 on that tip |
| 4 | The trial/012 active yield matrix is rank one | values on 24c5524; no meaning verdict | derivation | docs/trial-012-confluence-shape-report.md; eighteen zero minors |
| 5 | Trial/013 standard gates pass without executing its holdout | disclosed suite on d7fdd1b | measurement | isolated standard gate; report states sealed/unrevealed/unexecuted |
| 6 | Trial/014 response gates pass and full evaluated plans are invariant under input reversal | test-only afbae24 | behavioral-red | old-tip permutation red plus response-tip reversal tests |
| 7 | Current runtime law is A1 immediate/sequential and defines no RNG | ratified v0.1 | derivation | runtime-contract-proposal.md §§1–4 |
| 8 | Historical source classes have different date and inference limits | starting ledger only; no mechanic claim | derivation | linked institute, Cambridge, and archaeology references |
| 9 | This map changes no authority surface | diff against 2dd4db5 | measurement | write scope and final git diff --check |
