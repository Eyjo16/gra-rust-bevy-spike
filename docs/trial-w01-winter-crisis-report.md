# Trial W01 — the winter-crisis vertical slice

Status while this section stands alone: **pre-registration**. Every
number in §4 was computed and committed *before* the scene existed in
Rust.

Branch: `trial/W01-winter-crisis`. Author: Fable 5 (lead).
Base commit: `605e32a` (`trial/V01-give` tip; grammar
`0x7dd8c6706e0b949f`, oracles `10v6`).

## 0. What this trial is for

RES01 gave the world kinds; V01 gave it a social verb. W01 asks the only
question that can judge them: **can one winter crisis in one household
be expressed at all?** The scene is therefore built to fail informatively.
Its output is not a score — it is a pressure map plus an explicit list of
what the truth layer cannot yet say.

This trial changes **no grammar, no vocabulary, no oracle, and no
value**. The standard fixture, its identity and its envelope are
untouched. A scene is a fixture, and a fixture is not law.

## 1. Authoring envelope (as run)

```text
base_commit:         605e32a
objective:           One household, one winter, three defensible plans.
                     Build the scene as a fixture plus three command
                     sequences, run each through the same boundary and
                     the same ten oracles, and record the shortfall
                     against a stated winter need. Stop condition: the
                     full gate green, all three variants oracle-green,
                     and the executed numbers equal to the predictions
                     registered in §4 before implementation.
authoritative_files: AGENTS.md, docs/meaning-gate.md,
                     docs/trial-res01-resource-kinds-report.md,
                     docs/trial-v01-give-report.md
write_scope:         src/winter.rs, src/main.rs,
                     docs/trial-w01-winter-crisis-report.md,
                     docs/trial-log.md, docs/README.md
frozen:              EVERYTHING in the grammar: kinds, verbs, costs,
                     yields, bands, refusal reasons, oracle suite, the
                     standard fixture and its identity. If the scene
                     needs any of them to move, the run stops and asks.
red_required:        no — this is a measurement trial. There is no
                     honest red: the scene asks what the current rules
                     produce, and staging a bug to fail first would
                     measure the bug, not the winter. (Meaning Gate F3.)
verification:        the full gate, plus `cargo run winter` exit 0 with
                     all thirty oracle verdicts green (ten per variant)
evidence:            predicted vs executed table, three envelopes,
                     shortfall table, inexpressibility list
limits:              no new dependencies, no new verbs, no consumption
                     mechanics inside the truth layer
escalate_when:       the scene cannot be expressed without a new verb,
                     kind, or value — that is the finding, and it stops
                     the run rather than licensing a change
tested_commit:       <filled at completion>
```

## 2. The scene

**Vígslóði, the ninth week of winter.** One household, four people, one
hayfield already cut over, a stand of scrub wood, a shore. The cattle
need more fodder than the hayfield still holds. The roof is open. The
food store will not reach spring.

| Character | Stamina | Standing |
|-----------|---------|----------|
| C1 Auðr | 70 | the head |
| C2 Ketill | 60 | grown, strong |
| C3 Gróa | 45 | grown, tiring |
| C4 Hallr | 25 | young, and his claim on the hayfield is **unwitnessed** |

| Site | Tier | Kind | Stock |
|------|------|------|-------|
| S1 hayfield | established | fodder | 4 000 g |
| S2 scrub wood | crude | timber | 2 500 g |
| S3 shore | crude | food | 1 800 g |

Claims: K1–K3 bind Auðr, Ketill and Gróa to the hayfield (witnessed);
**K4 binds Hallr to the hayfield and is not witnessed** — someone must
spend stamina attesting it before he may work at all. K5/K6 bind Auðr
and Ketill to the wood, K7/K8 bind Gróa and Hallr to the shore.

**The winter need** (scene-local, *not* truth-layer mechanics — there is
no consumption in the truth layer, so this is arithmetic in the report
and in the scene projection, never a rule the world enforces):

```text
fodder 6 000 g   the cattle to spring
food   2 500 g   the household to spring
timber 1 200 g   the roof closed
```

Note what the fixture already decides: the whole world holds 4 000 g of
fodder and 1 800 g of food. **Even stripping every site bare cannot meet
the need.** The scene is not a puzzle with a solution; it is a triage,
and the choice is which shortfall to accept.

## 3. The three plans

- **A — feed the cattle.** Every hand to the hayfield, including
  Hallr's, which first costs Auðr the stamina to attest his claim.
- **B — save the roof.** Auðr and Ketill to the wood, Gróa and Hallr to
  the hay; Gróa pays for the attestation this time.
- **C — feed the people.** Gróa and Hallr work the shore, Auðr and
  Ketill the hay.

Each plan ends with one give: the household consolidates a stock in the
hands of whoever will carry it — a witnessed transfer in A and B, and in
C a young man handing his catch to the head.

## 4. Pre-registered predictions

Computed by an independent re-implementation of the grammar written from
the rules as documented (`w01_predict.py`, reproduced in E5 of the
evidence section), not by the crate. If the crate disagrees with these
numbers, one of the two is wrong and the trial has found something.

### Variant A — feed the cattle

| # | verb | actor | outcome | reason | spent | mass | kind |
|---|------|-------|---------|--------|-------|------|------|
| 1 | gather | C1 | accepted | - | 12 | 1200 | fodder |
| 2 | gather | C2 | accepted | - | 12 | 1200 | fodder |
| 3 | gather | C3 | accepted | - | 12 | 1200 | fodder |
| 4 | gather | C4 | refused | claim_not_witnessed | 0 | 0 | - |
| 5 | witness | C1 | accepted | - | 5 | 0 | - |
| 6 | gather | C4 | partial | site_nearly_depleted | 15 | 400 | fodder |
| 7 | gather | C1 | refused | site_empty | 0 | 0 | - |
| 8 | gather | C2 | refused | site_empty | 0 | 0 | - |
| 9 | give | C4 → C1 | accepted | - | 3 | 400 | fodder |

End: stamina C1 53, C2 48, C3 33, C4 7. Holdings C1 1600 fodder,
C2 1200, C3 1200. Sites S1 0, S2 2500, S3 1800.
Household totals fodder 4000, food 0, timber 0.
**Shortfall: fodder 2000, food 2500, timber 1200.**

### Variant B — save the roof

| # | verb | actor | outcome | reason | spent | mass | kind |
|---|------|-------|---------|--------|-------|------|------|
| 1 | gather | C1 | accepted | - | 12 | 800 | timber |
| 2 | gather | C2 | accepted | - | 12 | 800 | timber |
| 3 | gather | C3 | accepted | - | 12 | 1200 | fodder |
| 4 | witness | C3 | accepted | - | 5 | 0 | - |
| 5 | gather | C4 | accepted | - | 15 | 600 | fodder |
| 6 | gather | C1 | accepted | - | 12 | 800 | timber |
| 7 | gather | C3 | accepted | - | 15 | 600 | fodder |
| 8 | give | C1 → C2 | accepted | - | 3 | 1600 | timber |

End: stamina C1 43, C2 48, C3 13, C4 10. Holdings C2 2400 timber,
C3 1800 fodder, C4 600 fodder. Sites S1 1600, S2 100, S3 1800.
Household totals fodder 2400, food 0, timber 2400.
**Shortfall: fodder 3600, food 2500, timber 0.**

### Variant C — feed the people

| # | verb | actor | outcome | reason | spent | mass | kind |
|---|------|-------|---------|--------|-------|------|------|
| 1 | gather | C3 | accepted | - | 12 | 800 | food |
| 2 | gather | C4 | accepted | - | 15 | 400 | food |
| 3 | gather | C1 | accepted | - | 12 | 1200 | fodder |
| 4 | gather | C2 | accepted | - | 12 | 1200 | fodder |
| 5 | gather | C3 | accepted | - | 15 | 400 | food |
| 6 | gather | C4 | refused | insufficient_stamina | 0 | 0 | - |
| 7 | give | C4 → C1 | accepted | - | 3 | 400 | food |

End: stamina C1 58, C2 48, C3 18, C4 7. Holdings C1 1200 fodder +
400 food, C2 1200 fodder, C3 1200 food. Sites S1 1600, S2 2500, S3 200.
Household totals fodder 2400, food 1600, timber 0.
**Shortfall: fodder 3600, food 900, timber 1200.**

## 5. What the scene is expected to falsify

Pre-registered so the answers cannot be adjusted afterwards:

| ID | Question the scene must answer | Pre-registered expectation |
|----|-------------------------------|----------------------------|
| W1 | Can a household's winter triage be expressed with three kinds, three verbs and stamina alone? | **Yes, partially** — the choice between plans is real and costed, and no plan meets every need |
| W2 | Does the kind list survive its first scene? | **Yes** — nothing in the scene needs a fourth kind to be *stated*; fuel and turf are folded into timber, which is a naming compromise, not a mechanical gap |
| W3 | Is the herd-loss shape (`losing the dairy herd threatens the preservation chain and forces painful alternatives, never immediate game-over`) expressible? | **No** — there is no herd, no consumption, and no game-over. The scene can state the shortfall but cannot make it *happen* |
| W4 | Does the social layer earn its place in the scene? | **Yes, minimally** — the unwitnessed claim makes legitimacy cost stamina before work, and give lets a household consolidate a stock |
| W5 | Is anything in the scene forced to lie? | **Expected: no** — but the winter need itself is scene arithmetic, not a rule; if it ever looks like a rule, the projection is lying |

## 6. Non-authority of the scene projection

`cargo run winter` prints a shortfall table. That table is a
**projection**, in exactly the TS01 sense: nothing reads it back,
nothing in the truth layer consumes it, and no oracle depends on it. The
truth layer still has no concept of a need, a cow, a mouth or a roof.
The moment consumption becomes a rule it is a licensed spec evolution
with its own trial — it does not arrive through a report.
