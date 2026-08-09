# Verb-isolation report — second verb: `witness`

Standalone record of falsification item 7: add a second verb with a
different stamina policy and verify that the grammar/ownership separation
survives it. Date: 2026-08-08, on top of the round-1 hardening.

## The claim under test

After round 1 moved the gather cost table out of `CharacterOwner`, the
architecture claimed: *a new verb with a different policy never has to
touch an owner that isn't gaining new resource semantics.* If a second
verb forces action-specific logic into `CharacterOwner` or
`EconomyOwner`, the separation is not real.

## The verb chosen, and why it is adversarial

`witness(witness: CharacterId, claim: ClaimId)` — a character attests
another character's claim, flipping its boolean gate false → true. It
inverts every pattern the gather verb established:

| Dimension | gather | witness |
| --- | --- | --- |
| Stamina policy | band-based cost table `[_, 15, 12, 10]` | flat `WITNESS_COST = 5` |
| Exhausted gate | refused (`actor_exhausted`) | **none** — the exhausted may attest |
| Owners touched | Social (read) + Character + Economy | Social (**write**) + Character; Economy untouched |
| Mass moved | yes (the 4×4 cell) | never |
| Social state | read-only | first mutation path (`WitnessGrant` token) |

New closed refusal reasons: `claim_already_witnessed`,
`cannot_witness_own_claim` (RefusalReason: 9 → 11). New closed `Verb`
enum in every receipt. Grammar fingerprint now also covers
`WITNESS_COST`.

## Isolation result: the claim held

```
$ git diff --stat HEAD -- src/character/mod.rs src/economy/mod.rs
(empty — zero lines changed)
```

- **`CharacterOwner`: 0 lines changed.** The witness verb's entire
  stamina policy (flat cost, no exhausted gate) lives in
  `plan_witness` in the boundary; the owner's `validate_spend(id, cost)`
  served both verbs unchanged.
- **`EconomyOwner`: 0 lines changed.** The verb never references it.
- **`SocialOwner`: grew its own resource semantics only** — a
  revision-bound, by-value `WitnessGrant` token and `apply_witness`,
  identical in doctrine to the other two owners. That is an owner gaining
  a capability, not a verb leaking policy into an owner.
- **Boundary** holds all verb dispatch (`Command` enum), both policies,
  and both plans. **Shadow evaluator** (oracle 9) reimplements the
  witness verb independently.

## Falsification found: the second verb broke two oracles — correctly

First full run: `witnessed_gate` and `exhausted_gate` went **red** on a
legal trial. Both had operationalized their invariant as
`outcome.yields_mass()` — i.e. *"Accepted ⇒ mass moved"* — which was
true only while the grammar had one verb. A witness receipt is Accepted
with zero mass, so the hidden assumption surfaced immediately.

Both oracles were corrected to key on **actual mass movement**
(`!mass_moved.is_zero()`), which is verb-agnostic and strictly stronger:
any future mass-moving verb is gated automatically, and zero-mass verbs
are exempt by fact rather than by verb-specific exception.

This is exactly the class of finding the exercise existed to force: not
a code bug, but a grammar assumption that one verb could never have
falsified.

## Trial evidence (fixture run, mechanical example numbers)

The 16-command sequence now tells a cross-verb story the receipts fully
record:

- seq=3: gather via unwitnessed K3 → `refused claim_not_witnessed`
- seq=11: C1 witnesses K3 → `accepted`, flat spent=5
- seq=12: repeat → `refused claim_already_witnessed`
- seq=13: holder C2 tries → `refused cannot_witness_own_claim`
- seq=14: gather via K3 retried — the social gate now **passes**, but the
  gatherer is exhausted → `refused actor_exhausted` (`witnessed=true` in
  the receipt: two gates, independently visible)
- seq=15: C3 at 5 points (exhausted band) witnesses K8 → `accepted` —
  the divergent stamina policy in action
- seq=16: C3 at 0 points → `refused insufficient_stamina`

Final revisions: character=7, economy=5, social=2. Mass conserved at
8300 g. All nine oracles PASS, including the independent shadow
evaluator across both verbs.

## Gate

| Check | Result |
| --- | --- |
| `cargo fmt --check` | clean |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `cargo test` | 35 passed, 0 failed |
| `cargo run` | all 9 oracles PASS, exit 0 |
| `git diff -- src/character src/economy` | empty |

## Verdict

The isolation claim survived its strongest available falsification: a
verb chosen to invert every existing pattern required zero changes in
the two established owners, and the one thing it did break — two
oracles' hidden single-verb assumption — is precisely what the exercise
was designed to surface, and is now fixed at the invariant level.

The round-1 HOLD condition ("keep Bevy on HOLD until the second verb
lands and the isolation claim survives it") is now met. Lifting HOLD is
a deliberate decision left to the author, not taken here.
