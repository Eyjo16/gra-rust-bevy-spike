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

---

# Evidence

Author: Fable 5 (lead). Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`,
`cargo 1.97.1 (c980f4866 2026-06-30)`. Base commit `4f2443c`;
`tested_commit` `15e1bc7` (clean tree).

## E1. Red, verbatim

Falsifiers first, against `4f2443c` (`c192b75`, falsifiers only) — a
capability red: the verb, its refusal reasons, its receipt field and its
owner path all did not exist.

```text
      1 error: could not compile `gra-rust-bevy-spike` (bin "gra-rust-bevy-spike" test) due to 24 previous errors
      3 error[E0422]: cannot find struct, variant or union type `GiveCommand` in this scope
      2 error[E0599]: no method named `apply_transfer` found for struct `economy::EconomyOwner` in the current scope
      4 error[E0599]: no method named `validate_transfer` found for struct `economy::EconomyOwner` in the current scope
      1 error[E0599]: no variant, associated function, or constant named `CannotGiveToSelf` found for enum `boundary::RefusalReason` in the current scope
      1 error[E0599]: no variant, associated function, or constant named `EmptyTransfer` found for enum `boundary::RefusalReason` in the current scope
      3 error[E0599]: no variant, associated function, or constant named `Give` found for enum `boundary::Command` in the current scope
      1 error[E0599]: no variant, associated function, or constant named `Give` found for enum `boundary::Verb` in the current scope
      4 error[E0599]: no variant, associated function, or constant named `InsufficientHolding` found for enum `boundary::RefusalReason` in the current scope
      1 error[E0599]: no variant, associated function, or constant named `UnknownKind` found for enum `boundary::TextCommandFault` in the current scope
      1 error[E0599]: no variant, associated function, or constant named `UnknownRecipient` found for enum `boundary::RefusalReason` in the current scope
      1 error[E0599]: no variant, associated function, or constant named `UnknownWitness` found for enum `boundary::RefusalReason` in the current scope
      1 error[E0599]: no variant, associated function, or constant named `WitnessIsParty` found for enum `boundary::RefusalReason` in the current scope
      1 error[E0609]: no field `recipient` on type `boundary::Receipt`
```

## E2. Green, verbatim gate tail

```text
oracle PASS stamina_in_bounds (all stamina within 0..=100)
oracle PASS mass_conserved (fodder=2000g/2000g food=5000g/5000g timber=1300g/1300g total=8300g/8300g)
oracle PASS mass_authority_gate (0 unwitnessed extractions, 0 unconsented transfers)
oracle PASS exhausted_gate (0 exhausted or band-less receipts drained a site)
oracle PASS closed_reasons (0 receipts with unclosed reason codes)
oracle PASS cell_bounds (0 receipts outside the 4x4 cell)
oracle PASS replay_determinism (states_match=true hashes_match=true receipts_match=true)
oracle PASS refusal_zero_mutation (0 hash-chain or mutation violations)
oracle PASS shadow_expectation (0 receipts diverge from the shadow evaluator)
oracle PASS shadow_final_state (0 truth domains diverge from the shadow final state)
bevy_host_parity receipts_match=true state_match=true world_match=true receipts=0x2d52250d86f0638b world=0xb500dee0e5d883d8
bevy_projection views_match=true derived_from=0xb500dee0e5d883d8
bevy_publication revisions=20 derived_from=0xb500dee0e5d883d8 stale_rejected=true
bevy_host_faults admission_zero_mutation=true projection_isolated=true faults=admission_failed,projection_consumer_failed
envelope baseline_commit=15e1bc7 grammar=0x7dd8c6706e0b949f fixture=0x93afba3f312bd89d receipts=0x2d52250d86f0638b world=0xb500dee0e5d883d8 oracles=10v6
```

Bounded transition-domain parity, now covering the third verb — the
enumerated command space grew from 300 to 375 inputs, all visited, all
byte-identical between the pure and hosted runs:

```text
transition_domain_parity seed=0x007007006d617065 traces=1000 depth=32 commands=32000 command_space=375 unique_commands=375 receipts_match=true state_match=true world_match=true
transition_domain_verbs {"gather": 21429, "give": 6390, "witness": 4181}
transition_domain_outcomes {"accepted:-": 905, "partial:site_nearly_depleted": 69, "refused:actor_exhausted": 79, "refused:cannot_give_to_self": 1269, "refused:cannot_witness_own_claim": 742, "refused:claim_already_witnessed": 2205, "refused:claim_not_held_by_actor": 15444, "refused:claim_not_witnessed": 237, "refused:claim_site_mismatch": 3076, "refused:insufficient_holding": 2277, "refused:insufficient_stamina": 131, "refused:site_empty": 3, "refused:unknown_actor": 918, "refused:unknown_claim": 2570, "refused:unknown_recipient": 1097, "refused:witness_is_party": 978}
transition_domain_cost_cells gather={"fresh": 163, "low": 258, "steady": 2} witness_flat_uses=834 give_flat_uses=3046
```

Scope note, stated rather than glossed: the enumerated host space fixes
the transfer mass at 500 g and draws witnesses from existing actors, so
`empty_transfer` and `unknown_witness` do **not** appear in the host
trace. Both are exercised in the pure suite and in the standard trial
(receipts 21 and 23) — host parity for those two refusals is therefore
argued from the shared `submit` path, not measured.

Test counts, all four feature sets green:
`default 76` · `bevy-host 85` · `bevy-render 93` · `e01-taste 99`
(before: 67 / 76 / 84 / 90).

## E3. Identity movement

| Identity | Before (`4f2443c`) | After (`15e1bc7`) | Status |
|----------|--------------------|-------------------|--------|
| grammar | `0xc5d782ec145af0a5` | `0x7dd8c6706e0b949f` | **matches the value pre-registered in RES01 §4** |
| standard fixture | `0x13524a85dd14d068` | `0x93afba3f312bd89d` | measured; the command sequence grew eleven gives |
| receipts | `0x392e759fb4238743` | `0x2d52250d86f0638b` | measured; every receipt line gained `to=` and `claim=` became optional |
| world | `0x77100bd059984f29` | `0xb500dee0e5d883d8` | measured; the trial now ends after two transfers |
| oracle suite | `10v5` | `10v6` | count type-enforced at ten |

## E4. The standard trial's give receipts

Every new refusal is reachable in the canonical trial, and the last
accepted give empties a holding completely (`C2`'s timber, 550 g → 0):

```text
seq=17 verb=give actor=C1 claim=- site=- to=C2 outcome=accepted reason=- witnessed=true stamina_before=53 band=steady tier=- kind=fodder spent=3 mass_g=500 ...
seq=18 verb=give actor=C1 claim=- site=- to=C2 outcome=accepted reason=- witnessed=false ...
seq=19 ... reason=cannot_give_to_self ... seq=20 ... reason=unknown_recipient
seq=21 ... reason=unknown_witness ...   seq=22 ... reason=witness_is_party
seq=23 ... reason=empty_transfer ...    seq=24 ... reason=insufficient_holding kind=timber
seq=25 actor=C9 ... reason=unknown_actor ... seq=26 actor=C3 ... reason=insufficient_stamina
seq=27 verb=give actor=C2 ... to=C1 outcome=accepted ... band=exhausted ... kind=timber spent=3 mass_g=550
```

Receipt 27 is the declared policy visible in the ledger: an exhausted
character may still hand over what they hold. Oracle 4 permits it
because no site was drained; oracle 3 accepts it because the transfer
named a distinct counterparty.

## E5. Claims table

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
|---|--------------|-------|---------------|--------------------|
| 1 | A give moves exactly the named mass of exactly the named kind from giver to recipient, and nothing else changes | the owner path and the standard trial | behavioral-red | `falsification_transfer_conserves_the_kind_and_touches_nothing_else`, `falsification_give_conserves_the_transferred_kind`; oracle 2 per-kind line |
| 2 | Consent is structural: no command shape moves a character's holding without that character being the actor | the whole command vocabulary as typed | derivation | `Command::Give` names `giver` as the only source; `Transfer.from` is private and set from the giver; `falsification_give_never_moves_a_third_partys_holding` |
| 3 | A named witness pays nothing — no stamina, no mass, no owner state | the give path | behavioral-red | `falsification_give_never_moves_a_third_partys_holding` (C3's stamina and holding are unchanged after witnessing) |
| 4 | Witnessed and unwitnessed gives are distinguishable in the receipt and identical in world state | the give path | behavioral-red | `falsification_witnessing_a_give_is_receipted_but_never_stateful` (canonical lines differ, canonical states and hashes are equal) |
| 5 | A giver short of the named mass is refused, never partially satisfied, and the refusal mutates nothing | all eight give refusals | behavioral-red | `falsification_short_transfer_is_refused_not_clamped`, `falsification_give_refusals_are_closed_and_byte_stable` (world hash identical across every refusal) |
| 6 | Giving away all of a kind leaves the giver indistinguishable from someone who never held it, in state, text and hash | the owner and boundary paths | behavioral-red | `falsification_giving_everything_leaves_no_zero_entry` (equal hashes at equal apply counts), `falsification_giving_everything_erases_the_holding_completely`; standard trial receipt 27 |
| 7 | The give verb is audited by a judge that trusts no receipt field | the standard trial and the 32 000-command host trace | oracle | `ShadowState::step_give` is an independent reimplementation; oracles 9 and 10 green |
| 8 | The hosted run reproduces every give byte-for-byte, across 375 enumerated inputs including refusals | the enumerated space named in E2 — *excluding* `empty_transfer` and `unknown_witness` | parity | `transition_domain_parity ... unique_commands=375 receipts_match=true`; the exclusion is stated in E2 |
| 9 | The grammar moved exactly once, to the value pre-registered before either trial was implemented | this branch's history | measurement | prediction at `fc5e431` (RES01 §4); envelope at `15e1bc7`; `grammar_fingerprint_matches_the_licensed_value` |
| 10 | Oracle 3's extraction clause is unchanged in force by the rename and re-scope | the extraction half of the oracle | derivation | `mass_authority_gate` still counts `site.is_some() && !witnessed && mass != 0`; `mass_authority_gate_oracle_catches_a_doctored_receipt` is the v5 test, unchanged except for the name |

What this trial does **not** claim: that a give is *remembered* by
anyone (no E-layer), that an unwitnessed give has any consequence (it
does not, yet), that refusing to give is modelled (`actor_unwilling`
belongs to O01), or that the flat cost of 3 is balance.

## E6. Findings for the author

1. **`witnessed` now carries two verb-local meanings** — "the claim was
   witnessed" on a gather, "this transfer had a named witness" on a
   give. The receipt disambiguates structurally (`claim=-` and `site=-`
   on a transfer), but if a third reading ever appears, the field should
   split. Recorded now so the drift is visible early.
2. **A witness who is named without consenting.** Today a giver may name
   any third party as witness and that party cannot refuse, because
   refusal needs an E-layer and a will. Nothing is spent on their
   behalf, so nothing is stolen — but "was named as a witness" is a
   social fact the truth layer currently allows anyone to assert about
   anyone. That is a meaning question, not a mechanics bug.
3. **No debt, no memory, no obligation.** A give leaves no trace outside
   the receipt ledger. Feast politics, favour-debt, and compensation all
   need something to remember the give — which is the first thing after
   the E-layers, not before.
