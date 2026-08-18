# Sprint overview — 2026-08-18 · E01 merge, RES01, V01, W01

**Second pass.** The first version of this document was held in review
for repeating V01's consent overclaim and W01's over-strong framing.
Both are corrected below, and the review round itself is now part of the
record (§8). Author: Fable 5 (lead). Instrument: a synthesis over four
bundles that each ran under their own envelope; it introduces no claim of
its own that is not already evidenced in one of them.

**Read the bundles, not this page, when you review**:
`docs/trial-res01-resource-kinds-report.md`,
`docs/trial-v01-give-report.md`,
`docs/trial-w01-winter-crisis-report.md`.

## 1. What was dispatched, and what I decided as lead

The author forwarded Sol 5.6's five-step sequence and asked me to review
it, map my own reading, then sprint. My verdict on the sequence:
**agreed in full, and it matches the lead iteration map of 2026-08-14
(`agent/lead-iteration-map-2026-08-14`) item for item** — I1 RES01,
I2 V01 Give, I4 W01. Two agents converging on the same order from
different documents is worth noting as agreement, not as proof.

One decision I did **not** take as lead. `AGENTS.md` §1/§4/§10 reserve
closed-vocabulary and contract changes to the author, and RES01 moves
the frozen grammar fingerprint. I stopped and asked. The author licensed
**fodder, food, timber — no generic catch-all**, and that licence is
recorded verbatim in the RES01 report §0. Everything downstream runs
under it and nothing else was assumed from it.

Deviations from Sol's proposal, all deliberate:

- **No catch-all kind.** Sol proposed "one generic carried material". I
  recommended against it and the author chose the three-kind list: a
  catch-all absorbs exactly the pressure that should force a named kind
  and a permissioned move, and it makes cross-kind leakage
  unfalsifiable.
- **Give carries an optional named witness**, receipted **by identity**
  and never stateful. (First pass recorded only a boolean; the review
  caught it, and it is fixed.)
- **W01 became a triage, not a scenario.** The scene is built so no plan
  can win, because the interesting output is the list of what the truth
  layer cannot yet say.

## 2. State of the estate

| Branch | Tip | Gate | Terminal state |
|--------|-----|------|----------------|
| `master` | `1f3cbc6` | green (default · bevy-host · bevy-render · e01-taste) | **E01 merged on the author's instruction** |
| `trial/RES01-resource-kinds` | `d4f7ebe` | green | conditions applied; awaiting the merge word |
| `trial/V01-give` | `f83796d` | green | repaired after *hold*; review-ready (second pass) |
| `trial/W01-winter-crisis` | `8656e73` | green (+ `cargo run winter`) | repaired, rebased onto the repaired V01 |
| `agent/sprint-2026-08-18-overview` | this | green | second pass |

Tip gates: RES01 and V01 pass every applicable step (the `cargo run
winter` step exists only on W01 and downstream). W01 and this branch
pass all fourteen.

Nothing is pushed and nothing beyond E01 is merged. The three trials are
stacked because they are genuinely dependent (give needs kinds; the
scene needs both), and each is separately reviewable at its own tip.

Test counts across the sprint: `58 → 87` default, `67 → 97` bevy-host,
`75 → 105` bevy-render, `89 → 111` e01-taste.

## 3. Identity ledger — what moved, and what runs are still comparable

Canonical language is now identified three ways (author licence,
2026-08-18): **grammar** = gameplay semantics and policies;
**cmdfmt** = canonical command bytes; **rcptfmt** = canonical receipt
fields and order. The split exists so that a presentation change and a
gameplay change can never again be the same number.

| Stage | grammar | cmdfmt | rcptfmt | standard fixture | receipts | world | oracles |
|-------|---------|--------|---------|------------------|----------|-------|---------|
| master before E01 (`9a766ca`) | `0x530003916889b952` | — | — | `0x3805f1e20c001051` | `0x6c5b0e011471d985` | `0x36221d3fdb8aed9a` | 10v4 |
| master with E01 (`1f3cbc6`) | unchanged | — | — | unchanged | unchanged | unchanged | 10v4 |
| RES01 (`7c30816`) | **`0xc5d782ec145af0a5`** | — | — | `0x13524a85dd14d068` | `0x392e759fb4238743` | `0x77100bd059984f29` | 10v5 |
| V01 first pass (`15e1bc7`) | **`0x7dd8c6706e0b949f`** | — | — | `0x93afba3f312bd89d` | `0x2d52250d86f0638b` | `0xb500dee0e5d883d8` | 10v6 |
| V01 repaired (`cccbfcd`) | unchanged | **`0xfa37eefa3594cfe3`** | **`0x7e62152622bb9132`** | unchanged | `0xc0b4da51744bcf19` | unchanged | 10v7 |
| W01 repaired (`333ce29`) | unchanged | unchanged | unchanged | unchanged | unchanged | unchanged | 10v7 |

The V01 repair row is the split doing its work: the receipt format moved
and the grammar did not, so the estate can see that nothing about play
changed when the ledger's columns did.

All four identity values — two grammar moves and the two new format
identities — were **predicted before implementation** and hit exactly. The predictor's control stage reproduces the pre-sprint
fingerprint `0x530003916889b952` from its declared inputs, which is what
makes its two predictions checkable rather than decorative. The
fingerprint is now pinned in a test (`LICENSED_GRAMMAR_FINGERPRINT`), so
the next move must be a declared edit of a constant rather than
something noticed in an envelope line.

**Cross-trial comparability**: every measurement taken before `7c30816`
is about a different language and a different fixture. Quoting a
pre-sprint number against a post-sprint envelope is a category error.
E01's own evidence (`857b0e6`) is in the pre-move regime; its scene
still runs and still passes, but its frozen identities belong to the old
grammar.

## 4. Judges: what got stronger, what was re-scoped

- **Oracle 2** now proves conservation per kind as well as in aggregate.
  Strictly stronger, and the test proves the strictness: a staged swap
  of 300 g fodder into 300 g timber leaves the aggregate identical and
  fails the new check.
- **Oracle 3** `witnessed_gate` → `mass_authority_gate`. Its extraction
  clause is unchanged in force; it gained a transfer clause. The rename
  is declared here and in the suite-version comment so earlier evidence
  quoting the old name stays findable.
- **Oracle 4** keys its exhausted gate on site extraction. That is the
  gather verb's policy made explicit, not a weakening: give follows the
  witness verb's policy of a flat cost with no exhausted gate, and the
  standard trial now contains a receipt where an exhausted character
  hands over 550 g (receipt 27) — visible, deliberate, and gated.
- **Oracle 3's transfer clause** is named for what it proves:
  `unattributed_transfers`. It is a ledger-shape check — a mass-moving
  receipt with no site must name a source, a distinct recipient and a
  kind — and it does not prove consent.
- **The shadow evaluator** independently recomputes kinds for every
  receipt, reimplements give end to end, and compares the attester's
  *identity*, so a receipt naming the wrong witness diverges.
- **Host parity** enumerates 900 inputs across 80 000 commands, all
  byte-identical, and now reaches every give refusal including
  `empty_transfer` and `unknown_witness`. Each W01 plan additionally
  replays byte-for-byte in the host — receipts, chain digest, canonical
  state and world hash.

## 5. Findings for the author, ranked by how much they cost later

1. **Nothing is consumed, so nothing is at stake.** W01 can state a
   2 000 g fodder shortfall and nothing happens because of it. This is
   the widest gap between the truth layer and the game — wider than
   time, wider than randomness.
2. **There is no household.** The four people in the winter scene are
   related only by prose and by a projection that sums their holdings. A
   give between kin is exactly as social as a give between strangers.
3. **Nobody can refuse labour.** Consent to *give* is now structural;
   consent to *work* is not modelled at all (`actor_unwilling` belongs
   to O01). In plan A the head spends stamina attesting a claim because
   the command says so.
4. **Consent is not modelled anywhere.** A transfer is *attributed* to a
   named source; nothing shows the source willed it, and any caller may
   submit a command naming any character. This waits on the
   seat/issuer/delegation ruling, and until then no bundle may use the
   word "voluntary" as evidence.
5. **A third party can be named as a witness without consenting.**
   Nothing is spent on their behalf, so nothing is stolen, but the truth
   layer currently lets anyone assert "X attested this" about anyone.
   Meaning question, not a mechanics bug.
6. **Turf and fuel have no names.** RS01's scene says "turf" and the
   nearest licensed kind is `timber`; a winter fire has no kind at all.
   W01 is **not** evidence that three kinds suffice — the scene was
   written to fit the list, so its verdict on the list is inconclusive.
7. **Scene arithmetic is one careless step from becoming a rule.**
   `WINTER_NEED` is a projection today. The moment consumption is
   licensed it must arrive through a trial, not by promoting a number
   that was already printed.

## 6. What I did not do, and why

- **Did not push and did not merge anything beyond E01.** Terminal state
  is a review-ready branch (`AGENTS.md` §1, §5).
- **Did not touch chronology, the player seat, personality, delegation,
  or keyed randomness.** Sol's step 5 and my I0/I3/I6 — they answer
  pressure the slice discovers, and W01's findings are now that
  pressure.
- **Did not implement consumption, a household, or a herd**, though W01
  makes the case for all three. Naming a gap is a finding; filling it
  during the same run is scope creep.
- **Did not run RS01-human.** It is prepared on
  `review/RS01-human-protocol` and needs an unbriefed person, not an
  agent.
- **Did not adjust any value.** Every number touched in this sprint is a
  new fixture constant (`GIVE_COST`, the winter scene) or a kind label;
  no yield, cost or band moved.

## 7. What the author decides next

1. **Merge order.** My recommendation: `trial/RES01-resource-kinds`,
   then `trial/V01-give`, then `trial/W01-winter-crisis`, serially, each
   with a re-gate — they are already stacked in that order, so a merge
   of the W01 tip carries all three, which is faster and loses the
   ability to stop between them.
2. **Second cross-review.** The first round did its job — three of the
   four bundles came back with material findings, and every finding was
   accepted. The repairs now need the same treatment: the two new format
   identities, whether the attribution restatement is exactly the size of
   its evidence, whether oracle 3's clause name matches its check, and
   whether W01's language purge missed a promise.
3. **Consumption.** The W01 finding stack points at one next trial:
   something that eats. That is a contract-shaped decision — it needs a
   licence, a shape, and a pre-registered red, and it is the first thing
   that would make a shortfall *hurt*.
4. **The turf/peat/fuel question** — grow the kind list, or rename the
   scene's material.
5. **Whether `cargo run winter` joins the standard gate.** It is a
   scene, not law; adding it makes scene drift loud, which is why I ran
   it in this sprint's gate.

## 8. Review round one — what it caught, and what it cost

Reviewer: Sol 5.6, 2026-08-18. Verdicts: RES01 *conditional accept*,
V01 *hold*, W01 *accept as pressure evidence, hold integration*, this
overview *hold*. **Every material finding was accepted; none was
contested**, so no disagreement circle was opened.

| # | Finding | Where it landed |
|---|---------|-----------------|
| 1 | The named witness never reached the receipt | `Receipt.transfer_witness`, shadow comparison, R1/R2 falsifiers |
| 2 | "Consent by construction" was an overclaim | Claim restated as attribution; the word purged as evidence from code, projection, reports and this page |
| 3 | Receipt/command evolution had no authority identity | Author licence + the three-way identity split, pre-registered and pinned |
| 4 | Oracle 3's transfer clause was misnamed | `unattributed_transfers`, with a doc that states its limit |
| 5 | W01's output got ahead of truth | Stockpiling language; kind-list verdict downgraded to inconclusive |
| 6 | W01 parity and proof gaps | Per-plan host parity; the tautological test replaced by six pinned identities; "best" scoped to the three registered plans |

What it cost: one repair pass, two new falsifiers, one widened host
domain, and a receipt-format identity move. What it bought: the estate
no longer contains a claim that the boundary does not carry.

The lesson worth keeping, stated plainly because it is mine: the
mechanics in both trials were sound, and the failures were all in the
*sizing of claims* about them — exactly the failure class the meaning
gate exists to catch, caught by review rather than by me.
