# Trial/E01 — Belief and actionability taste

Date: 2026-08-14

Status: **mechanical and human/manual PASS**

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

The pre-registration is commit `6d76bda`; the bounded implementation is
commit `02e573d`. The implementation changes only the off-by-default
`e01-taste` capability, its command wiring, and `src/e01_taste.rs`.

### Red to green

1. Before the feature existed, `cargo test --features e01-taste` failed with
   `package ... does not contain this feature: e01-taste`. This is the
   capability red.
2. The first automated capture panicked after the second beat: the overlay
   recomputed Egil's expectation from the post-action Publication, where his
   stamina was already zero. That erased the belief that had caused the
   action. The fix retains the identified pre-action Publication as
   `belief_source`; current scene facts still come from the current
   Publication, and outcomes still come only from canonical Receipts.
3. Six focused falsification tests then passed: wrong/matching belief,
   belief-read non-interference, one Host submission per player advance,
   sealed-holdout exclusion, no canonical observation backdoor, and
   capability/capture-name stability.

### Exact mechanical result

The same trace presents two fixture-local beliefs derived from Publication
`0xa650191d3c5fe826`. Hrafn's refused action leaves the current Publication at
that identity. Egil's accepted action advances current truth to
`0x4d2c6d5c1cd8f7c8`, while the displayed remembered belief remains explicitly
identified as coming from `0xa650191d3c5fe826`.

- Hrafn, start 14: refused / `insufficient_stamina`; spent 0; moved 0;
  Publication unchanged.
- Egil, start 15: accepted; spent 15; moved 600 g; site 2,000 -> 1,400 g;
  stack 0 -> 600 g.
- The renderer neither submits an outcome nor observes canonical state. The
  only player advances each submit exactly one existing command through
  `Host`.
- The trial/013 start-29/Advanced holdout does not appear in the executable
  fixture.

Final exact-tip gate, after the evidence append:

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| strict clippy, default / `bevy-host` / `bevy-render` / `e01-taste` | PASS |
| tests, default / `bevy-host` / `bevy-render` / `e01-taste` | 58 / 67 / 75 / 81 PASS |
| pure and `bevy-host` runs | PASS; byte-frozen envelope |

Frozen runtime identity:

```text
grammar=0x530003916889b952
fixture=0x3805f1e20c001051
receipts=0x6c5b0e011471d985
world=0x36221d3fdb8aed9a
oracles=10v4
```

### Automated render evidence

Both sets contain four 1280 x 800 PNGs at
`/home/eyjo/taste-evidence/E01-belief-actionability-2026-08-14/`.

| Beat | Default SHA-256 | Proof SHA-256 |
| --- | --- | --- |
| two beliefs | `dc8636507258fa1a4264f9eaac1c1dcf4adaa3680bb51e418d589c1630ce5850` | `26df31c8c16868b4f363b9ef87dffc0ed032d5c4a932cb8964f4aa227939c480` |
| belief wrong | `a66711bda42e33a78d8f7e47408be8700e6a77330d7a64a791dd6210fee154c6` | `3e7c939c46747d8a14c23fd2f0baaf50fdb514cd8ad50f5458f1bb8b2242412d` |
| belief matched | `5472576ec4b5d3da220255997dbda75132867d34f0f297bf32b341f6fed25a80` | `8096fa2981474c3d3053d1521c69eeab76c02ac19d5952a7be45066084e7e614` |
| belief is not truth | `6f147eb71e3fbe638b6650a88c4922ae4f4cf66a0fd81a364e8daa6436edbce6` | `4cfb777dfe09ad0d35c9f5a43ac0e28991becd876fc1295b5b9ea28c8e3f06e8` |

The full-resolution captures were visually checked for clipping, overlap,
fact drift, and proof/default separation. The default view uses player-facing
language; the proof view adds current Publication, remembered-belief identity,
and receipt detail. This is automated capture plus implementation-side visual
QA, not the required player-driven human verdict.

## Bounded verdict

**Mechanically PASS.** The current core can make a wrong belief and a matching
belief legible against one unchanged canonical actionability edge, without
giving belief or presentation authority over the world. Balance meaning,
general character inference, and inventory design are not ratified here. The
manual loop is reported separately below.

## Human/manual verdict

On exact evidence tip `9df3a90`, the author ran:

```text
cargo run --features e01-taste -- e01-render
```

The author advanced the complete four-beat loop and observed the unchanged
Publication on Hrafn's refusal, the new Publication after Egil's accepted
action, and the retained pre-action beliefs in both later beats. The rendered
window used Mesa llvmpipe; its XSETTINGS, software-rendering, and XRandR
messages were environmental warnings, not trial failures.

Author verdict: **PASS** — "the grammar seems to do its sensible behaviour."
This closes the human/manual gate for the bounded E01 trace. It ratifies the
legibility and sensible behavior of this installed grammar edge only; it does
not ratify balance values, generalized character inference, inventory design,
or a registry/schema contract.
