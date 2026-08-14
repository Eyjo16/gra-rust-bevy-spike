# Trial/E01 — Belief and actionability taste

Date: 2026-08-14

Status: **pre-registered; implementation not yet written**

Branch baseline: `9a766ca`

Author direction: taste the current core before the next hardening. Inventory
is explicitly out of scope until the shorter belief/action loop is legible.

This is a bounded presentation trial, not a registry, schema, balance, or
historical-law decision. `E01` is a local trial label; it creates no contract
identifier.

## Question

Can one player-driven render trace make the epistemic distinction legible:

> Canonical truth never lies; a character's belief may be wrong.

The renderer must show the installed H-A actionability edge without letting a
belief, overlay, or renderer choose the outcome.

## Fixed fixture and prediction

One coherent world contains two independent witnessed claims over two
Established sites:

- C1 starts at exactly 14 stamina and holds K1 over S1;
- C2 starts at exactly 15 stamina and holds K2 over S2;
- S1 and S2 each start with 2,000 g;
- both characters carry the same fixture-local belief: they expect they can
  manage one last gather;
- submit `Gather(C1,K1,S1)`, then `Gather(C2,K2,S2)`, each through `Host`.

Exact installed-H-A prediction:

| Actor | Start | Character belief | Canonical outcome | Spent | Moved | Post |
| --- | ---: | --- | --- | ---: | ---: | ---: |
| C1 | 14 | expects success | refused / `insufficient_stamina` | 0 | 0 g | 14 |
| C2 | 15 | expects success | accepted | 15 | 600 g | 0 |

The first refusal must preserve the publication identity and every projected
fact. The second command must advance the publication, reduce S2 by exactly
600 g, and place the same 600 g with C2. These are mechanical fixture values,
not a proposed balance ratification.

The trial/013 sealed start-29/Advanced holdout is forbidden. It must not be
constructed, submitted, projected, rendered, or mentioned by the executable
fixture.

## Ownership and capability envelope

- New capability is off by default and implies the existing `bevy-render`
  capability; default, `bevy-host`, and `bevy-render` behavior stay unchanged.
- Both gathers cross the existing `Host` and canonical boundary. The renderer
  and belief overlay have no command-result selection path.
- Scene facts and belief inputs are copied only from identified
  `Publication`s. The E01 module may not call `truth_state`, `truth_hash`, or
  `canonical_state`.
- Receipts are the sole source for actual outcome, reason, stamina spent, and
  mass moved.
- The belief overlay is explicitly perspective-bearing presentation. It may
  be wrong, but it may not claim a cost, rewrite a Publication, submit a
  command, or be serialized as canonical state.
- No registry, schema contract, closed reason vocabulary, command, receipt,
  table, threshold, yield, or oracle changes are in scope.

## Falsifiers

The trial fails if any of these occur:

1. start 14 mutates canonical state, or does not refuse with
   `insufficient_stamina`;
2. start 15 does not spend 15 and move 600 g exactly;
3. the refused Publication changes identity or projected facts;
4. any belief or renderer code can select, alter, or apply an outcome;
5. any scene fact or belief input bypasses `Publication` to observe canonical
   truth;
6. removing or changing the overlay changes receipts or Publications;
7. the start-29/Advanced holdout is touched;
8. the feature is enabled by default or changes the frozen default/host/render
   proof envelope;
9. an automated capture is reported as the human/manual verdict.

## Planned evidence

1. Capture the capability red before adding `e01-taste`.
2. Add the smallest Publication-fed trace and its renderer.
3. Run focused falsification tests, then strict default, `bevy-host`,
   `bevy-render`, and `e01-taste` gates.
4. Run pure and host envelopes and prove the four inherited identity fields
   remain frozen.
5. Produce an automated capture set as mechanical evidence. Human taste stays
   a separate verdict.

## Execution ledger

Implementation, red/green evidence, exact commits, gate counts, capture
identity, and the bounded verdict will be appended without rewriting this
pre-registration.
