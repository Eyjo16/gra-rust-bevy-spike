# VS01 — order, witness, and discipline vertical-slice pre-registration

Date: 2026-08-14. Branch:
`trial/VS01-order-witness-discipline`. Base:
`2dd4db5db6f52b287ebf4f6b8a3d259bf30ba028`.

Status: **REVIEW-READY PROPOSAL; BLOCKED AT NAMED AUTHORITY GATES.**

This document turns the author's north-star answers into a bounded first
cross-system slice. It creates no runtime type, command, outcome, registry,
schema, value, RNG law, or historical rule. Candidate words below are design
language only, not additions to a closed vocabulary.

## Author direction carried forward

- The player occupies the current household head; play continues through a
  dynasty rather than treating every person as a puppet.
- Other people remain free-willed. An order changes their decision context; it
  does not directly mutate their action.
- Rich behavior should arise from conditional economic, social, personal,
  family, legal, and honor pressures.
- Randomness may vary action while remaining bounded, replayable, and unable to
  legalize an illegal act.
- Causes and consequences should be understandable to the player even when
  exact internal arithmetic is not exposed.
- Real-time-with-pause is an expression layer over discrete deterministic
  canonical decisions.
- Historical constraints require dated sources and cannot be taken from a
  timeless composite “Norse” setting.

These are product directions, not executable law until their owner and proof
surface are ratified.

## The smallest useful story

One household head asks one subordinate to perform one already legal economic
act. One other person can observe the request, performance, and later response.
The subordinate may perform it, decline it, misrepresent what happened, or
leave the relationship only if the final ratified action space makes that
option legal. The head then chooses whether and how to respond. Future behavior
may change because participants remember different facts and interpretations.

The slice is intentionally one household, one request, one economic act, one
observer, and one response. It is large enough to force cross-system ownership
and small enough to falsify.

No candidate social consequence is yet a production mutation. Existing
`Gather`, `Witness`, Host publication, and receipt machinery are the only
current executable seams.

## Five-layer epistemic separation

The lead should require a ruling on these conceptual layers before code:

| Layer | Candidate meaning | Owner question |
|---|---|---|
| fact | canonical event/state that actually occurred | which truth owner writes it? |
| observation | what an actor could perceive from a fact | is it canonical, derived, or publication-only? |
| statement | what one actor communicated | how is speaker, audience, and referent identified? |
| belief | an actor's current model, possibly wrong | canonical character/social state or disposable AI memory? |
| judgment | an actor's evaluative reading such as fair, shameful, dangerous | authored rule, learned policy, or expression? |

A lie must never rewrite a fact. At minimum it is a statement whose content
does not match the speaker's available fact/belief under a declared test. That
definition itself remains a Meaning Gate candidate.

AI and UI may use only the layer available to that consumer. No planner may
read canonical hidden facts through a renderer, debug projection, or global
world handle.

## Candidate interaction pipeline

```text
identified Publication
  -> actor-specific observation
      -> already-legal candidate actions
          -> deterministic feature extraction
              -> bounded score/ranking
                  -> optional keyed choice among legal candidates
                      -> exactly one typed boundary command
                          -> atomic validation/commit
                              -> receipt + next Publication
                                  -> actor-specific memory/expression
```

Selection is not truth. A score, language model, animation, or random draw
cannot waive a guard, create a refusal reason, or choose a canonical outcome.
Only the boundary command and its named owners may mutate truth.

## Weight shape without values

The first scoring trial may compare terms from these provisional families:

- personal capacity, safety, skill, and prior experience;
- household obligation, dependants, reciprocity, and exit pressure;
- material need, seasonal opportunity, tools, and expected yield;
- liking, trust, fear, resentment, and remembered treatment;
- public reputation, witnessed promises, legal exposure, and honor judgment;
- anticipation based only on the actor's available observations and beliefs.

This list is neither a registry nor permission to add statistics. Lead must
decide which terms are distinct, which owner supplies each fact, and which are
historically supportable.

Every admitted term later needs: unit, normalization, range, sign convention,
owner, availability layer, interaction terms, and counterfactual sensitivity
test. No hidden constant or model prior may become an undeclared weight.

Punishment is not allowed a universal “respect up” effect. A bounded hypothesis
may predict increased short-term compliance through fear while decreasing
liking or trust, with reputation depending on observer, proportionality,
standing, and context. The trial must allow that hypothesis to fail.

## Randomness authority proposal, not law

Current runtime law is A1 immediate/sequential and defines no RNG. VS01 depends
on a future R10 verdict. The narrow candidate is keyed per-decision randomness:

```text
choice_key = H(campaign_seed, canonical_decision_identity, draw_purpose, index)
```

A draw selects only among already-legal candidates. Candidate enumeration,
host scheduling, UI frame rate, and worker count must not affect
`canonical_decision_identity` or the draw. Invalid or refused commands do not
silently advance a global cursor; any consumption rule must be explicit in
R10 and serialized for replay.

The formula, fields, hash, seed owner, index rule, and persistence are all
unratified. Trial/014 at `afbae24`, if independently accepted, can contribute
test-only precedent for order-invariant ranking and rejection of ties/crossed
trade-offs. It cannot authorize RNG or an AI planner.

## Dependency gates

| Gate | Required ruling/evidence | Why VS01 cannot cross it silently |
|---|---|---|
| VS01-G0 | exact historical bucket and geography from H01 | social/legal constraints change by era and source |
| VS01-G1 | player-seat and succession contract | “household head” needs identity, death, transfer, and agency boundaries |
| VS01-G2 | epistemic-layer ownership | deception and private belief otherwise leak or overwrite truth |
| VS01-G3 | order lifecycle and legal action space | request, refusal, abandonment, and discipline are not current verbs |
| VS01-G4 | R10 deterministic random authority | replay identity and draw consumption are undefined |
| VS01-G5 | independent exact-tip trial/014 verdict | ranking precedent is not yet accepted evidence |
| VS01-G6 | historical dossiers for order, status, witness, discipline, and household attachment | H01 maps questions but does not establish these mechanics |
| VS01-G7 | author Meaning Gate for directional social consequences | weights cannot choose what respect, fear, trust, or honor mean |
| VS01-G8 | registry/schema/contract permission for every new canonical type | standing project law forbids implicit contract evolution |

## Pre-registered falsifiers

Any implementation bundle must include these before tuning values:

1. **Legality first:** no score or random draw selects a command rejected by
   its complete canonical guards.
2. **Replay:** same canonical observation, actor memory, candidate set, and
   random authority produce byte-identical selected command and receipts.
3. **Permutation:** candidate input order, collection order, and scheduler
   order do not alter feature values, ranking, tie handling, or choice.
4. **Information boundary:** changing a hidden fact that the actor cannot
   observe cannot change that actor's decision.
5. **Declared sensitivity:** changing one visible declared fact affects only
   named features and interactions.
6. **No universal discipline monotone:** at least one declared context can
   separate compliance, fear, liking, trust, and third-party reputation.
7. **False statement isolation:** a statement can change belief or judgment
   without changing the fact it refers to.
8. **Atomicity:** a failed cross-owner preflight leaves facts, observations,
   memories, RNG identity, receipts, and publications unchanged except for any
   separately ratified noncanonical fault log.
9. **Counterfactual explanation:** the system can name the few declared terms
   that changed the selected action without exposing forbidden hidden facts.
10. **Historical guard:** removing or changing a contested historical premise
    disables only mechanics explicitly dependent on that premise.

## Sprint packets after lead refinement

```text
H01 + author era choice
  -> H02-order/status/witness dossier
      -> VS01-GATE authority decisions
          -> E01 epistemic seam
          -> R10 keyed-random identity
              -> O01 request/order lifecycle
                  -> VS01-B test-only behavior model
                      -> VS01-T canonical vertical slice
                          -> RS02 player-facing causal expression
                              -> HUMAN-VS01 meaning/play review
```

E01 and R10 can proceed in parallel only after their separate envelopes are
ratified. O01 requires both. VS01-B may be test-only but still cannot invent
production vocabulary. VS01-T is the first point where new canonical behavior
could enter truth, so it requires branch-named implementation and merge
authority.

## Stop condition for this branch

This proposal is complete when it is pushed, fully gated as docs-only, and the
lead returns:

1. a dependency-tree correction;
2. an exact H01 time/place choice or an explicit counterfactual blend;
3. owner decisions for the five epistemic layers;
4. the smallest legal order/response action space;
5. an R10 envelope;
6. a claim-by-claim trial/014 review dispatch;
7. one directional social-consequence Meaning Gate;
8. explicit permission for any required contract/registry/schema work.

Until then, code sprinting stops here by design.

## Claims table

| # | Atomic claim | Scope | Evidence mode | Evidence |
|---|---|---|---|---|
| 1 | Current truth has only Gather and Witness as canonical verbs | master `2dd4db5` | source audit | boundary grammar and TS01 shapes |
| 2 | Current law is A1 immediate/sequential and has no RNG authority | Runtime Contract v0.1 | derivation | runtime-contract-proposal.md |
| 3 | Trial/014 response demonstrates test-only order-invariant ranking properties but lacks independent exact-tip verdict | `afbae24` | branch evidence state | sprint map and reachable ref |
| 4 | H01 does not establish universal status, spy, weapon-burial, or numeric social rules | H01 ledger only | source-bound review | H01-C10, C13–C16 |
| 5 | This proposal adds no production type, value, registry/schema, contract, closed vocabulary, or executable behavior | branch diff | diff audit | docs-only write scope |

## Verification

Docs-only staged tree: format and all three strict Clippy suites passed;
tests passed 56 default / 65 bevy-host / 73 bevy-render. Both feature-enabled
runtime probes exited 0 with receipts, state, and world parity true; the
frozen `10v4` envelope remained unchanged.
