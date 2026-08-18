# V01 repair — pre-registration

Branch: `trial/V01-give` (continuing). Author: Fable 5 (lead).
Base for the repair: `605e32a`. Reviewer: Sol 5.6, verdict **hold** on
2026-08-18, five material findings.

This document is registered **before** the repair is implemented. It
exists because the repair moves canonical language, and a language move
that is not predicted in advance is indistinguishable from a language
move that was noticed afterwards.

## 0. The authority this repair runs under

Author licence, 2026-08-18, in answer to the review's Step 2:

> Licence the give language (verb code `give`, the six new reason codes,
> the receipt format gaining `claim_witnessed` + `transfer_witness`, and
> the extended command encoding), **and** split identity three ways:
> grammar = gameplay semantics and policies; command-encoding =
> canonical command bytes; receipt-format = canonical receipt fields and
> ordering. Each gets its own fingerprint, pre-registered before
> implementation and pinned by a test, and the envelope line carries all
> three.

This licence covers the split and this vocabulary. It is not a registry,
schema, or persistence format; none is introduced.

## 1. Findings and the repair each one gets

| # | Finding | Repair |
|---|---------|--------|
| 1 | The named witness never reaches the receipt: two different witnesses produce identical receipts | `Receipt` gains `transfer_witness: Option<CharacterId>`; the overloaded boolean becomes `claim_witnessed`; the shadow evaluator recomputes and compares the witness *identity* |
| 2 | "Consent by construction" is wrong — attribution is not consent | Claim #2 restated as attribution; every occurrence of "consent"/"voluntary" as a *proof* word removed from code comments, the TS01 projection, the report and the overview. Where the design intent is voluntary transfer, it is labelled intent, not evidence |
| 3 | Receipt/command evolution has no authority identity | The three-way identity split above, each fingerprint pre-registered in §2 and pinned by a test; the envelope carries all three |
| 4 | Oracle 3's transfer clause checks shape, not consent | The count is renamed `unattributed_transfers`; the doc states exactly what it proves — that a mass-moving receipt with no site names a source, a distinct recipient and a kind |
| 5/6 | W01 language and parity | Handled on `trial/W01-winter-crisis`, not here |

## 2. Pre-registered identities

The grammar fingerprint does **not** move: nothing in this repair
changes a policy, a value, a kind or a reason code. Its value stays
`0x7dd8c6706e0b949f`, and the pin test proves it.

Two identities are new. Declared inputs, hashed with the same FNV-1a as
everything else, each field followed by a `0x1f` separator so that
`("ab","c")` and `("a","bc")` cannot collide:

**Command encoding** — each verb code, then each field it contributes to
`Command::canonical_bytes`, in order, with its byte width. An optional
id is width 9: one presence byte plus eight.

```text
gather  : actor 8, claim 8, site 8
witness : witness 8, claim 8
give    : giver 8, recipient 8, kind 1, grams 8, witness 9
```

**Receipt format** — the field names of `Receipt::canonical_line`, in
printed order:

```text
seq, verb, actor, claim, site, to, outcome, reason, claim_witnessed,
transfer_witness, stamina_before, band, tier, kind, spent, mass_g,
grammar, world_before, world
```

| Identity | Predicted value |
|----------|-----------------|
| `command_encoding_fingerprint()` | **`0xfa37eefa3594cfe3`** |
| `receipt_format_fingerprint()` | **`0x7e62152622bb9132`** |
| `grammar_fingerprint()` | `0x7dd8c6706e0b949f` — unmoved |

The receipt-chain digest, the fixture identity and the world hash of the
standard trial move as measured consequences of the receipt lines
changing; they are recorded at green, not predicted.

## 3. New falsifiers

| ID | Falsifier | Failure meaning |
|----|-----------|-----------------|
| R1 | Two gives identical except for the witness's identity produce **different** receipts and **identical** world state | the witness is still not in the ledger |
| R2 | A receipt naming the wrong witness identity is caught by the shadow evaluator, which recomputes it from the fixture | the witness field is decorative |
| R3 | The three identities are independent: a receipt-format change moves only the receipt-format fingerprint, a command-encoding change moves only that one, and neither moves the grammar | the split is cosmetic |
| R4 | Claim witnessing and transfer witnessing are separately named everywhere a receipt is read or printed | the overloaded field returned under a new name |

R3 is proved by construction rather than by mutating constants: each
fingerprint is computed from a disjoint declared input set, and the test
asserts each against its pinned value, so any edit to one input moves
exactly one number and turns exactly one pin red.

## 4. Oracle suite

`ORACLE_SUITE_VERSION` 6 → 7: oracle 3's transfer count is renamed and
its doc narrowed to what it proves; oracle 3 and 9/10 read the renamed
receipt fields. No oracle gains or loses force over the verbs it already
judged — except oracle 9, which gains the witness-identity comparison
and is therefore strictly stronger.
