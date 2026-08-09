# Sprint 007–012 — pressure overview

Date: 2026-08-09

Shared baseline: `f5728d6`

Status: six isolated local branches, reviewed but not merged or pushed by
this sprint.

This overview separates pressure on runtime truth from pressure on gameplay
values. A broken measurement path cannot vote on balance: invariant loss,
host drift, or pre-boundary normalization must be resolved before an outcome
distribution is allowed to name a value as wrong.

## Trial ledger

| Trial | Branch / commit | Red class | Result | Pressure landed on |
| --- | --- | --- | --- | --- |
| 007 transition domain | `trial/007-transition-domain` / `653f71e` | Honest capability red: bounded parity harness absent | 1,000 traces × 32 transitions; all 300 command forms visited; exact receipts/state/hash matched | Host-coverage confidence and value reachability |
| 008 apply totality | `trial/008-apply-totality` / `3c07df1` | Confirmed behavioral red | `u64::MAX + 1` silently destroyed one gram and saturating aggregation could false-green | Representable world bound, exact arithmetic, oracle 2 |
| 009 language seam | `trial/009-language-seam` / `bfd141d` | Confirmed behavioral red in the introduced seam harness | Leading `+` was silently normalized before canonical command bytes; strict parser rejects ambiguity | Source representation and normalization |
| 010 active-cell reachability | `trial/010-active-cell-reachability` / `e5e6f0d` | Honest capability red: no all-cell harness | All 12 active cells reached through exact-full, one-gram-short, and empty stock: 36 cases | Systematic value reachability, not value correctness |
| 011 threshold-edge chains | `trial/011-threshold-edge-chains` / `a455288` | Honest capability red: no chain collector | Dead interval and two dominance cliffs measured through real transitions | Threshold/cost/yield candidate hypotheses |
| 012 confluence shape | `trial/012-confluence-shape` / `42c2f40` | Exact mathematical red | All 18 active `2×2` minors are zero; active table is rank one | Meaning of confluence and efficiency compounding |

Per-trial evidence lives in `docs/transition-domain-report.md`,
`docs/trial-008-apply-totality-report.md`,
`docs/trial-009-language-seam-report.md`,
`docs/active-cell-reachability-report.md`,
`docs/trial-011-threshold-edge-chains-report.md`, and
`docs/trial-012-confluence-shape-report.md` on their respective branches.

## Why the weights are a vector

A single pressure score would allow abundant weak evidence to outweigh one
hard contradiction. This sprint therefore records four independent axes:

```text
P = (truth, reachability, sensitivity, holdout)
```

- **Truth** asks whether the evidence or transition path can lie. A confirmed
  truth defect is a veto: repair it before interpreting gameplay output.
- **Reachability** asks whether a value was actually consulted, rather than
  merely appearing in a receipt or source table.
- **Sensitivity** asks how outcomes change when one named value moves while
  the rest of the hypothesis stays fixed. This sprint did not move values.
- **Holdout** asks whether a pre-registered directional prediction survives an
  untouched fixture. No candidate value change was pre-registered, so no
  holdout was opened.

| Trial | Truth weight | Reachability weight | Sensitivity | Holdout | Balance permission |
| --- | --- | --- | --- | --- | --- |
| 007 | No divergence in the bounded host comparison | Strong for command forms; sparse for table cells | Not tested | Not opened | None |
| 008 | Critical confirmed invariant/oracle lie | Constructible through real owner validation/apply | Not a balance parameter | Not applicable | None |
| 009 | Confirmed ambiguity at a proposed source seam | One concrete normalization plus adversarial spellings | Not a balance parameter | Not applicable | None |
| 010 | No semantic contradiction | Systematic: all 12 active cells, three stock boundaries each | Not tested | Not opened | None; every active cell is now eligible for a later hypothesis |
| 011 | Declared verb policies remained coherent | Strong at eight named stamina edges | Natural edge deltas measured; no production perturbation | Not opened | Candidate surfaces named; no value selected |
| 012 | Interaction hypothesis falsified exactly | Whole active matrix | Rank-one shape and 4.5× efficiency spread measured | Required before any fit | Shape earned pressure, but direction remains unchosen |

The sprint therefore moved no yield, cost, threshold, or fixture value. That
is not absence of progress: two measurement defects were named, every active
cell became executable under targeted pressure, two threshold cliffs were
measured, and one claimed interaction shape was falsified exactly.

## Value reachability map

The number in parentheses is how many trial/007 receipts proved that the
yield lookup actually consulted that cell. Trial/010 then reached every
active cell systematically through exact-full, one-gram-short, and empty
stock. This is coverage, not evidence that a value is fair, fun, historical,
or correctly scaled.

| Stamina band \ infrastructure | None | Crude | Established | Advanced |
| --- | ---: | ---: | ---: | ---: |
| Exhausted | `0` (gated) | `0` (gated) | `0` (gated) | `0` (gated) |
| Low | `250` (116) | `400` (103) | `600` (17) | `900` (0) |
| Steady | `500` (0) | `800` (0) | `1200` (1) | `1800` (0) |
| Fresh | `750` (0) | `1200` (0) | `1800` (101) | `2700` (115) |

The random standard-fixture run consulted only 6 of 16 cells; four exhausted
cells are structurally unreachable and six active cells received zero random
consultations. Trial/010 removed the active reachability gap with 36 targeted
cases: 12 exact-full Accepted, 12 one-gram-short Partial, and 12 empty
SiteEmpty no-ops. Full and partial receipts expose each numeric yield; empty
cases prove the refusal boundary but cannot independently reveal a selected
zero-mass value.

### Cost and threshold exposure

| Value or shape | Current expression | Trial/007 exposure | What is and is not known |
| --- | --- | ---: | --- |
| Exhausted gather cost | `0` | 0 consultations | Gather rejects before cost lookup; this is a sentinel, not an exercised price |
| Low gather cost | `15` | 359 consultations | Host parity exercised it; balance direction was not perturbed |
| Steady gather cost | `12` | 1 consultation | Effectively no comparative pressure |
| Fresh gather cost | `10` | 216 consultations | Host parity exercised it; balance direction was not perturbed |
| Witness cost | `5` | 1,079 consultations | Seam and host behavior are stable; opportunity cost is unmeasured |
| Band thresholds | `0–9 / 10–39 / 40–79 / 80–100` | All bands observed in receipts | Observation is not an edge test; no threshold earned movement |

Trial/011 then exercised named threshold starts `9, 10, 14, 15, 39, 40,
79, 80` through complete gather and witness chains. This upgrades edge
reachability and reveals discontinuities; it still does not decide whether
those discontinuities are desirable.

One structural reachability result matters: `unknown_site` is not reachable
through coherent claim-first `gather`. A command site not matching the claim
fails earlier as `claim_site_mismatch`; a matching coherent claim already
names a seeded site. That closed reason remains vocabulary evidence, not a
reachable outcome of this verb under the current world rules.

## Mathematical pressure already visible in the values

These are exact results from trials 011–012 and the current mechanical
numbers, not balance verdicts.

### 1. The active yield cell is multiplicatively separable

Ignoring the gated exhausted row, the table is exactly

```text
Y(band, tier) = band_multiplier × tier_base
band_multiplier = [1, 2, 3]
tier_base       = [250, 400, 600, 900]
```

Trial/012 checked all 18 active `2×2` minors with exact `u128`
cross-products. Every minor is zero. The active 3×4 matrix has rank one and
six degrees of freedom rather than twelve: every positive row is a scalar
multiple of every other row. Infrastructure has the same relative effect at
every stamina band, and stamina has the same relative effect at every tier.
The explicit hypothesis that the cell already expresses non-separable
confluence was falsified. If separability is intentional, the table is a
presentation of a simpler independent-multiplier rule; if interaction is
intended, its direction still requires semantic authority and a holdout.

### 2. Yield and cost compound the same advantage

At a fixed tier, moving from Low to Steady doubles yield while cost falls
from 15 to 12; yield per stamina becomes `2.5×` the Low rate. Moving from Low
to Fresh triples yield while cost falls from 15 to 10; yield per stamina
becomes `4.5×` the Low rate. This is a strong positive-feedback shape, not
automatically a defect. Pressure must decide whether stamina is meant to
reward momentum, restore parity, or express exhaustion cost.

### 3. The Low band contains a gather dead zone

Actors at 10–14 stamina are classified Low but cannot cover the Low gather
cost of 15. Trial/011 confirmed starts 10 and 14 refuse as
`insufficient_stamina`, while start 15 gathers once. Actors at 5–9 are
classified Exhausted yet may witness when they can cover the flat cost of 5;
the witness chains equal `floor(stamina / 5)`. This is coherent under
verb-specific policy. It does mean band names alone do not express
actionability; UI, AI, or balance analysis must ask about the verb and exact
headroom.

### 4. Infrastructure has accelerating absolute increments

The tier base moves `250 → 400 → 600 → 900`, increments of `+150`, `+200`,
and `+300`. Because band multipliers preserve this shape, later tiers have
increasing absolute returns at every active stamina band. Whether that is
earned compounding or runaway advantage needs a progression hypothesis and a
holdout, not a table-aesthetic judgment.

### 5. Thresholds compound classification, yield, and cost

Trial/011 measured two one-point dominance cliffs on an Established site:

| Edge | Chain length | Total yield | Total spent | First-step change |
| --- | ---: | ---: | ---: | --- |
| `39 → 40` | `2 → 2` | `1200 → 1800` (`+50%`) | `30 → 27` | Low `15/600` becomes Steady `12/1200` |
| `79 → 80` | `6 → 6` | `6000 → 6600` (`+10%`) | `78 → 76` | Steady `12/1200` becomes Fresh `10/1800` |

One extra stamina point changes band, reduces cost, and raises yield at once,
without increasing the number of accepted gathers in either comparison. This
is real dominance pressure, but the intended magnitude cannot be derived from
runtime coherence alone.

## Candidate value pressure, ordered

This is a priority for the next hypothesis, not a ranking of what is
“incorrect.” Magnitude, scope, and missing semantic authority stay visible
instead of being collapsed into one score.

| Candidate surface | Evidence now | Magnitude / scope | Missing before a value may move | Priority |
| --- | --- | --- | --- | --- |
| Meaning of the 3×4 yield interaction | Exact rank-one falsification across all 18 minors | Global table shape; six interaction degrees absent | Choose independent, synergistic, or diminishing meaning; seal one chain holdout | First if confluence is intended to be non-separable |
| Low→Steady edge (`39→40`) | Real complete chains | Same two actions; total yield `+50%`; total spend `−10%` | State intended threshold reward and which axis may change | First behavioral pressure |
| Low actionability (`10–14`) | Real refusals at 10/14 and acceptance at 15 | Five-point non-exhausted gather dead interval | Decide whether band labels imply verb availability | Second |
| Steady→Fresh edge (`79→80`) | Real complete chains | Same six actions; total yield `+10%`; total spend `−2.6%`; first yield `+50%` | State intended late-band reward | Second |
| Infrastructure increments | Exact table shape | Base increments accelerate `+150,+200,+300` and scale with stamina | Progression/time-to-depletion target | Third |
| Witness cost `5` | 1,079 random consultations plus exact edge chains | Linear chain `floor(stamina/5)`; no contradiction | Downstream value for claims unlocked | Hold unless another system supplies pressure |

## Pressure cards: completed and next

These are ordered so coverage precedes tuning. None grants itself permission
to change a production value.

### 010 — active-cell reachability — completed

All 12 active cells reached the actual yield lookup through exact-full,
one-gram-short, and empty-site cases. The four exhausted cells remain
structurally gated sentinels. Values stayed frozen.

### 011 — threshold-edge chains — completed

Named gather and witness chains from stamina `9, 10, 14, 15, 39, 40, 79,
80` recorded actionability, cost, yield, transitions, and stop reasons. The
declared policies were coherent; the dead interval and dominance cliffs now
exist as candidate hypotheses rather than intuition.

### 012 — confluence shape — hypothesis falsified

The claim that the active table already contains non-separable interaction
failed exactly: all active minors are zero. Before touching the table,
pre-register one semantic direction:

- **separable:** stamina and infrastructure are independent multipliers; or
- **interactive:** at least one band/tier pairing has meaning not recoverable
  from independent multipliers.

If interaction is chosen, state the expected cross-product direction and its
behavioral effect in a sealed scenario before proposing any cell change. The
intentional red probe was removed so no test now freezes either rank-one or
rank-greater-than-one grammar.

### 013 — value sensitivity and sealed holdout

Only after 010–012 name a value, perturb that one value in an experiment and
measure at least:

- actions until the actor can no longer perform the verb;
- total mass moved per initial stamina;
- site depletion time and partial-outcome count;
- witness opportunity cost and downstream claims unlocked;
- directional effect on one untouched fixture revealed once.

A value earns a production move only if the same named hypothesis survives
the measurement path, the targeted fixture, the local sensitivity check, and
the sealed holdout. A failure at an earlier axis returns pressure to that axis;
it is not permission to tune until the receipts become quiet.

## Integration order

No branch was merged here. The evidence implies this review order:

1. `trial/008-apply-totality` first because it changes the judge from `10v3`
   to `10v4` and closes a false-green path.
2. Rebase and re-run 009 on that judge, then review the named command-byte
   seam.
3. Rebase and re-run 010 and 011 serially; both add boundary test harnesses
   and their evidence must survive the stronger judge and named seam.
4. Integrate 012's falsified-hypothesis record without turning rank one or
   non-rank-one into a contract.
5. Rebase and re-run 007 last so the broad host sample evaluates the combined
   arithmetic, seam, and pressure-harness baseline.
6. Merge this overview only after all six referenced trial reports exist on
   integrated truth.

After every merge, the remaining branches must rebase and run both feature
gates. The frozen standard evidence remains grammar
`0x530003916889b952`, fixture `0x3805f1e20c001051`, receipts
`0x6c5b0e011471d985`, and world checksum `0x36221d3fdb8aed9a`; only trial/008
declares the judge transition to `10v4`.
