# Trial W01 — the winter-crisis vertical slice

**Bundle status: REPAIRED — review-ready (second pass).** Sol 5.6:
*accept as pressure evidence, hold integration*, with findings 5 and 6.
Both are answered in §E7. `tested_commit`: **`333ce29`** (code); later
commits touch `docs/` only.

Rebase reference map (workflow note S2): this branch was rebased onto the
repaired `trial/V01-give` tip `f83796d`. Old → new:
`fa28712 → 1ff6dea` (pre-registration), `0672405 → 0725af5` (scene),
`a173383 → 6d8f6bd` (evidence). The rebase changed no content; the
`docs/trial-log.md` conflict was resolved by keeping both the V01 repair
entry and the W01 entry.

Status of the original text below: **pre-registration**, as first
committed, left as written with corrections marked.

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

*(Plan names corrected after review — the originals were "feed the
cattle", "save the roof", "feed the people", which name consequences the
truth layer cannot enact. What the plans do is stockpile.)*

- **A — stockpile fodder.** Every hand to the hayfield, including
  Hallr's, which first costs Auðr the stamina to attest his claim.
- **B — stockpile building material.** Auðr and Ketill to the wood, Gróa
  and Hallr to the hay; Gróa pays for the attestation this time.
- **C — stockpile food.** Gróa and Hallr work the shore, Auðr and
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
| W2 | Does the kind list survive its first scene? | **Yes** — nothing in the scene needs a fourth kind to be *stated*; fuel and turf are folded into timber, which is a naming compromise, not a mechanical gap. *(This pre-registered expectation was answered **inconclusive** — see §E3 and §E7 finding 5b: the scene was written to fit the list, so it cannot be evidence that the list fits scenes.)* |
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

---

# Evidence

Author: Fable 5 (lead). Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`,
`cargo 1.97.1 (c980f4866 2026-06-30)`. Base commit `605e32a`;
`tested_commit` `0672405` (clean tree).

## E1. Predicted vs executed

Every predicted number was met exactly. The comparison is machine-made,
not eyeballed: `every_plan_matches_its_pre_registered_end_state` holds
the §4 table as a constant and fails on any drift.

| Plan | Stamina | Household totals | Shortfall | Site stocks |
|------|---------|------------------|-----------|-------------|
| A | 53 / 48 / 33 / 7 ✓ | fodder 4000, food 0, timber 0 ✓ | 2000 / 2500 / 1200 ✓ | 0 / 2500 / 1800 ✓ |
| B | 43 / 48 / 13 / 10 ✓ | fodder 2400, food 0, timber 2400 ✓ | 3600 / 2500 / 0 ✓ | 1600 / 100 / 1800 ✓ |
| C | 58 / 48 / 18 / 7 ✓ | fodder 2400, food 1600, timber 0 ✓ | 3600 / 900 / 1200 ✓ | 1600 / 2500 / 200 ✓ |

Two independent implementations of the grammar — the crate, and the
prediction script written from the documented rules — produce identical
receipts and identical end states for all three plans. That is a real
cross-check, and it is bounded: the script was written by the same
author who wrote the trials, on the same reading of the same documents,
so it catches transcription and arithmetic errors, not a shared
misreading of the law. It is not an independent audit and must not be
quoted as one.

## E2. Gate and oracles

```text
winter_shortfall plan=A kind=fodder held=4000g need=6000g short=2000g
winter_shortfall plan=A kind=food   held=0g    need=2500g short=2500g
winter_shortfall plan=A kind=timber held=0g    need=1200g short=1200g
winter_shortfall plan=B kind=fodder held=2400g need=6000g short=3600g
winter_shortfall plan=B kind=food   held=0g    need=2500g short=2500g
winter_shortfall plan=B kind=timber held=2400g need=1200g short=0g
winter_shortfall plan=C kind=fodder held=2400g need=6000g short=3600g
winter_shortfall plan=C kind=food   held=1600g need=2500g short=900g
winter_shortfall plan=C kind=timber held=0g    need=1200g short=1200g
envelope scene=W01 plan=A ... fixture=0x288ef6dbfad7e800 receipts=0xf7226e43a85603ab world=0x8955528b452a8dde oracles=10v6
envelope scene=W01 plan=B ... fixture=0x1ac3857f928579d5 receipts=0x87e56f06f1bd4870 world=0xd2b2803a6c1b77d7 oracles=10v6
envelope scene=W01 plan=C ... fixture=0x209b45a394eddc4a receipts=0x62e99e5a4817d370 world=0xb898728c0ccd0b48 oracles=10v6
```

Thirty oracle verdicts, ten per plan, all PASS; `cargo run winter` exits
0. Test counts: `default 81` · `bevy-host 90` · `bevy-render 98` ·
`e01-taste 104`.

**The standard trial's envelope is byte-identical to V01's**
(`grammar=0x7dd8c6706e0b949f fixture=0x93afba3f312bd89d
receipts=0x2d52250d86f0638b world=0xb500dee0e5d883d8`), which is the
measurable form of "this trial changed no law".

## E3. The pre-registered questions, answered

| ID | Answer | Evidence |
|----|--------|----------|
| W1 | **Yes, partially.** The triage is real and costed: each plan is defensible, none is right, and the price of each is visible in the ledger | the three shortfall rows; `no_plan_can_meet_every_need` |
| W2 | **INCONCLUSIVE** (corrected after review; the first pass answered "yes"). Three kinds expressed this scene — but the scene was authored against the three-kind list, so its success is evidence about the author, not about the list. Turf and fuel were folded into timber by decision, not by discovery | the scene runs with `ResourceKind::ALL` unchanged, which shows expressibility and nothing about sufficiency |
| W3 | **No — as pre-registered.** The herd-loss shape is not expressible. There is no herd, no consumption, no spring, and no game-over; the scene can *state* a 2 000 g fodder shortfall but nothing happens because of it | `WINTER_NEED` is arithmetic in a projection; no oracle and no rule reads it |
| W4 | **Yes, minimally.** The unwitnessed claim makes legitimacy cost stamina before any work is possible, and it costs a *different* person in plan A than in plan B; give lets the household consolidate a stock in one pair of hands | plan A receipts 4–6 (refusal, attestation, partial); plan B receipt 4; the closing give in each plan |
| W5 | **No lie found, one risk named.** The shortfall table is the only statement in the scene that sounds like a rule and is not | E4 finding 1 |

## E4. What the scene proves the truth layer cannot yet say

This is the trial's real output. Each item is a *finding*, not a
licence: none of them is implemented here.

1. **Nothing is consumed, so nothing is at stake.** The shortfall is a
   sentence in a report, not a fate. Until something eats, burns, rots
   or shelters, "the cattle need 6 000 g" is the scene's opinion. This
   is the single largest gap between the current truth layer and the
   game the author is building.
2. **There is no winter.** No time, no season, no ordering beyond the
   command sequence (Runtime Contract A1: immediate and sequential).
   "The ninth week" is prose. Every plan happens in an instant.
3. **There are no cattle, and no household.** The four characters are
   related only by the fixture's prose and by the scene projection that
   sums their holdings. The truth layer has no household, so "the
   household's stock" is arithmetic over four separate people; a give
   between them is exactly as social as a give between strangers.
4. **A plan is not a thing.** The three plans exist as Rust functions.
   Nothing in truth can hold "what we intend to do this week", which is
   what O01's charter is for.
5. **Nobody can refuse.** Auðr's attestation in plan A is unconditional:
   the head spends 5 points because the command says so. `actor_unwilling`
   does not exist, so consent to *labour* is not modelled. Give records an
   attributed transfer, but whether the giver consented remains unproven.
6. **Site stock is the only scarcity with texture.** The hayfield running
   out mid-work (partial, then `site_empty`) is the most alive moment in
   the scene, and it comes entirely from the existing gather verb. The
   4×4 cell earns its keep here.
7. **Turf and fuel are still missing names.** Plan B calls the scrub
   wood `timber`; a real winter roof is turf, and a real winter fire is
   peat. The kind list survived because the scene was written to fit it —
   an honest bias, worth stating: W2's "yes" is weaker than it looks.

## E5. Reconstruction recipe

The prediction script (`w01_predict.py`) is reproduced here rather than
committed to the crate, per the same rule as RES01's fingerprint
predictor: nothing in the crate may read a projection back. It
re-implements bands, costs, yields, the three verbs' gates and the
zero-normalization of holdings, then prints the end state of each plan.
Its structure is the same as the RES01 predictor's — plain integers,
its own literals, no import from the crate — and running it reproduces
the §4 tables exactly.

Verification commands for this trial:

```sh
cargo test winter
BASELINE_COMMIT=$(git rev-parse --short HEAD) cargo run winter
```

## E6. Claims table

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
|---|--------------|-------|---------------|--------------------|
| 1 | All three plans execute with every oracle green | the W01 fixture and the three named command sequences | oracle | thirty `oracle PASS plan=…` lines; `every_plan_passes_all_ten_oracles` |
| 2 | Every executed number equals the number predicted before the scene existed | stamina, holdings, site stocks and shortfalls of all three plans | measurement | §4 table committed at `fa28712`; `every_plan_matches_its_pre_registered_end_state` |
| 3 | No **registered** plan meets every winter need, and the best result per kind comes from a different one of the three | **the three plans as written — not the reachable strategy space, which is unbounded and unsearched** (scope corrected after review) | derivation | `no_registered_plan_can_meet_every_need` pins the best shortfall per kind among these three at `[2000, 900, 0]` |
| 4 | The scene changes no law: the standard trial's grammar, command encoding, receipt format, fixture identity, receipt digest and world hash are all unmoved by this branch | the whole branch | measurement | `the_scene_moves_no_identity_of_the_standard_trial` pins all six. *(The first pass cited `the_scene_adds_no_rule`, which compared a function to itself and proved nothing — the review was right; the test is replaced, not re-worded.)* |
| 5 | Legitimacy is a real cost before work: an unwitnessed claim stops the boy until someone spends stamina attesting it | plan A receipts 4–6, plan B receipt 4 | behavioral | `the_scene_beats_are_the_ones_the_scene_claims` |
| 6 | The winter need is not a rule and is read back by nothing | the whole crate | derivation | `WINTER_NEED` is used only by `household_totals`/`shortfall`/`run`, none of which any oracle or owner calls |
| 7 | The herd-loss shape from the lead iteration map is not expressible today | the current truth layer | derivation | E4 findings 1–3: no consumption, no time, no household |
| 8 | Every plan replays byte-for-byte inside the Bevy host — receipts, chain digest, exact canonical state and world hash — including the attester's identity on its transfer (added in the repair) | the three plans | parity | `every_plan_replays_identically_inside_the_host`; three `winter_host_parity … receipts_match=true state_match=true world_match=true attested_transfers=1` lines |

What this trial does **not** claim: that these numbers are balance, that
three plans exhaust the strategy space, that the scene is fun, or that a
player would understand it — the last is RS01-human's question, and this
scene has no rendering.

---

# E7. Repair pass (review response)

Reviewer: Sol 5.6, 2026-08-18: **accept as pressure evidence, hold
integration.** Repair `tested_commit`: **`333ce29`**, clean tree, gate
green at `87 / 97 / 105 / 111` tests plus `cargo run winter` exit 0.
Base: the repaired `trial/V01-give` (`f83796d`), so this scene now runs
on attributed transfers with attester identities in the ledger.

| Finding | Verdict | What changed |
|---------|---------|--------------|
| 5a — the output gets ahead of truth ("feed the cattle", "the roof closes", "the household eats") | **Accepted.** The terminal was making promises the layer cannot keep | Plans renamed to *stockpile fodder / building material / food*; every consequence clause replaced with what actually happens, including the explicit negatives ("Nothing is repaired: no structure exists", "Nobody eats: no consumption exists"). A module-level note records the language discipline so it is not quietly reintroduced |
| 5b — W01 did not prove the three-kind list sufficient | **Accepted.** The scene was written to fit the list | W2's answer is now **inconclusive**, in both the question table and the claims table, with the bias stated in the same sentence as the result |
| 6a — no exact pure/Bevy comparison for the winter traces | **Accepted.** The host feature ran the plans through the pure runner only | `winter::host_parity` compares receipts, chain digest, exact canonical state and world hash per plan; it runs inside `cargo run --features bevy-host winter` and as a test. Each plan is asserted to carry an attested transfer first, so the check cannot silently become weaker than claimed |
| 6b — `the_scene_adds_no_rule` is tautological | **Accepted; it proved nothing** | Replaced by `the_scene_moves_no_identity_of_the_standard_trial`, which pins all six standard identities |
| 6c — "best achievable" overreaches | **Accepted** | Renamed `no_registered_plan_can_meet_every_need`; scope stated in the test, the assertion message and claim 3 |

## E8. Repair evidence

```text
plan A stockpile fodder — every hand to the hayfield, including the boy's — which first costs the head the stamina to attest his claim. Whether the cattle live is not a fact this layer holds
winter_host_parity plan=A receipts_match=true state_match=true world_match=true attested_transfers=1 receipts=0x260782d648ffef68 world=0x8955528b452a8dde
envelope scene=W01 plan=A baseline_commit=333ce29 grammar=0x7dd8c6706e0b949f cmdfmt=0xfa37eefa3594cfe3 rcptfmt=0x7e62152622bb9132 fixture=0x288ef6dbfad7e800 receipts=0x260782d648ffef68 world=0x8955528b452a8dde oracles=10v7
winter_host_parity plan=B receipts_match=true state_match=true world_match=true attested_transfers=1 receipts=0xb67361c3ef45ffca world=0xd2b2803a6c1b77d7
winter_host_parity plan=C receipts_match=true state_match=true world_match=true attested_transfers=1 receipts=0xf6528f6ae509bdb4 world=0xb898728c0ccd0b48
```

The three plans' *world* hashes are unchanged from the first pass
(`0x8955528b452a8dde`, `0xd2b2803a6c1b77d7`, `0xb898728c0ccd0b48`): the
V01 repair changed how receipts are written, not what happens. Their
receipt digests moved with the receipt format, as they must.

The pre-registered §4 predictions still hold exactly on the repaired
base — stamina, holdings, site stocks and shortfalls are unchanged, and
`every_plan_matches_its_pre_registered_end_state` is unmodified.

## E9. What this bundle is, after the repair

**Pressure evidence, and nothing more.** It shows that a household's
winter triage is *expressible* as a choice between costed plans, and it
enumerates what the truth layer cannot say — consumption, time,
household, plans, refusal of labour, and names for turf and fuel. It
does not show that the kind list is sufficient, that these numbers are
balance, that the strategy space was searched, or that anything is at
stake.
