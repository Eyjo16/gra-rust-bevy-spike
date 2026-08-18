# Trial V01 — the `Give` verb

Status while this section stands alone: **pre-registration**.

Branch: `trial/V01-give`. Author: Fable 5 (lead).
Base commit: `4f2443c` (`trial/RES01-resource-kinds` tip; grammar
`0xc5d782ec145af0a5`, oracles `10v5`).

## 0. Why this verb, and what it is not

`Give` is a **witnessed-or-unwitnessed, voluntary transfer of a named
mass of one kind between two characters**. It is the smallest verb that
turns a single-actor economy into a social one: the economy owner
already tracks per-kind holdings, and no new owner, no epistemic layer,
and no randomness are required.

What it is not, and must not become in this trial: not a request (that
is O01, and it needs E-layer ownership first), not theft (taking
another's holding stays unrepresentable — there is no command shape for
it), not a contract or debt (a remembered give is E-layer work), and not
a legal transfer of a claim (claims stay social-owner state).

Deliberately absent: any randomness, any time or expiry, any
consumption. A give is instantaneous under Runtime Contract A1, like
every other verb today.

## 1. Authoring envelope (as run)

```text
base_commit:         4f2443c
objective:           Add `give` as the third verb: giver -> recipient,
                     one named kind, exact mass, flat stamina cost,
                     optional named witness recorded on the receipt.
                     Conservation, consent, refusal zero-mutation and
                     replay invariance must hold. Stop condition: full
                     gate green on a clean tree at all feature sets,
                     grammar equal to the value pre-registered in
                     RES01 §4 for the V01 tip.
authoritative_files: AGENTS.md, docs/runtime-contract-proposal.md,
                     docs/meaning-gate.md, docs/development-workflow.md,
                     docs/trial-res01-resource-kinds-report.md
write_scope:         src/**, docs/trial-v01-give-report.md,
                     docs/trial-log.md, docs/README.md
frozen:              The RES01 kind list; yield/cost tables; band
                     thresholds; the gather and witness verbs' gates,
                     costs and refusal reasons; the three-owner split;
                     the oracle count of ten.
red_required:        yes (behavioral where reachable, capability where
                     the verb does not exist yet)
verification:        the §3 gate of AGENTS.md extended with the
                     bevy-render and e01-taste feature sets
evidence:            red transcript verbatim, gate tail, identity table,
                     numbered claims table
limits:              no new dependencies, no new owner, no time model,
                     no randomness, one grammar move
escalate_when:       consent cannot be enforced by construction; a
                     transfer needs social-owner state to be honest; an
                     oracle would have to be weakened rather than
                     re-scoped with a compensating clause
tested_commit:       <filled at completion>
```

## 2. The shape under test

**S-V01.** `give giver=C_a to=C_b kind=K g=N [witness=C_c]`

- **Consent by construction.** The command names the giver as the
  actor. There is no command shape in which one character moves
  another's holding, so theft is not "refused" — it is unrepresentable,
  the same way negative mass is.
- **Exactness.** A giver who holds less than `N` of kind `K` is
  *refused*, never partially satisfied. Partial exists for gather
  because a site's remaining stock is a fact about the world the actor
  cannot fully know; a giver's own store is not.
- **Cost.** A flat `GIVE_COST` (mechanical example: 3), with no
  exhausted gate — the same policy family as `witness`, decided by the
  boundary, not by an owner. An exhausted person may still hand over
  what they hold; a person with no stamina at all cannot.
- **Witnessing is receipted, not stateful.** A named witness must exist
  and must not be either party. The witness pays nothing and no owner
  state changes: a third party cannot be made to spend by someone
  else's action. Witnessed and unwitnessed gives are therefore
  *separately receipted but identical in world state* — the unwitnessed
  give is the future rumour/dispute seed, and that meaning waits for
  the E-layers. This is the one place where this trial deliberately
  records something it does not yet mechanise; the receipt is the
  ledger, so recording it there costs no unratified mechanics.

## 3. Falsifiers

| ID | Falsifier | Failure meaning |
|----|-----------|-----------------|
| G1 | Conservation: the giver's holding of `K` decreases by exactly what the recipient's holding of `K` gains; no other holding, kind, or character changes | a transfer can mint or destroy mass |
| G2 | Consent: no command sequence makes a character's holding decrease without that character being the receipt's actor | theft by verb |
| G3 | Exactness: a giver short of the named mass is refused with `insufficient_holding`, and the world hash is byte-identical before and after | silent clamping, the defect trial/008 caught in the economy owner |
| G4 | Closed refusals, each reachable: `cannot_give_to_self`, `unknown_recipient`, `unknown_witness`, `witness_is_party`, `empty_transfer`, `insufficient_holding`, plus the shared `unknown_actor` / `insufficient_stamina` | an unreachable reason code is decoration, not law |
| G5 | Witness recording: a witnessed and an unwitnessed give differ in their receipts and are identical in world state; a named witness who is a party or does not exist refuses the whole command | witnessing became a hidden mutation, or a decorative field |
| G6 | Give-to-zero: after giving everything of a kind, the giver's state, hash and canonical text are identical to a world where they never held that kind (the reachable half of RES01's F5) | the world hash stops being a function of visible truth |
| G7 | The independent shadow evaluator recomputes every give from the fixture and catches an internally consistent receipt lie | the new verb is audited only by the code that implements it |

## 4. Pre-registered identity move

Predicted V01 grammar fingerprint: **`0x7dd8c6706e0b949f`** — registered
in `docs/trial-res01-resource-kinds-report.md` §4 (commit `fc5e431`),
before either trial was implemented, from these declared inputs:
`GIVE_COST = 3` hashed after the witness cost, and six new refusal codes
appended to the closed list in this order: `unknown_recipient`,
`cannot_give_to_self`, `insufficient_holding`, `empty_transfer`,
`unknown_witness`, `witness_is_party`.

The standard fixture identity moves again, this time because the command
sequence itself grows the give commands that make every new refusal
reachable. Recorded as measured.

## 5. Oracle-suite consequence, declared in advance

Two oracles must learn the difference between **extracting from a site**
and **transferring between characters**. Neither loses strength on the
verbs it already judged:

- Oracle 3 `witnessed_gate` → renamed `mass_authority_gate`, suite
  v5 → v6. Old clause, unchanged in force: no receipt moves mass *out of
  a site* without a witnessed claim. New clause: a receipt that moves
  mass with no site must name a counterparty distinct from the actor and
  a kind. The rename is declared here so that earlier evidence quoting
  `witnessed_gate` stays findable — same oracle position, wider audit.
- Oracle 4 `exhausted_gate` keys on site extraction rather than on any
  mass movement. This is verb policy made explicit, not a weakening:
  the exhausted gate has always been the gather verb's, and `witness`
  already forced the log to be keyed by effect rather than by outcome.
  Give follows the witness policy: no exhausted gate, flat cost.

The oracle count stays ten and stays type-enforced. Consent and transfer
conservation are audited by oracles 2, 9 and 10 — the independent
recomputation path — not by a receipt-trusting check.
