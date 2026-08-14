# RS01 trial log — append-only

## 2026-08-14 — dispatch received, discrepancy recorded BEFORE rendering

Envelope: RS01 Publication-to-Expression Causal Proof, author-dispatched,
base `fca5237`, branch `run/RS01`. The hardened HTML scene is accepted as
RS01-VISUAL-REFERENCE only.

### Discrepancy record (envelope rule: executable truth wins)

The envelope's fixture tracks one stamina line through both verbs:

```text
65 —(witness, −5)→ 60 —(gather, −12)→ 48
```

That arc requires one character to witness a claim and then gather under
the same claim. Executable truth refuses it twice over:

- `validate_witness_grant` refuses the holder with
  `cannot_witness_own_claim` (ratified vocabulary, trial round 2);
- `plan_gather` requires `holder == actor` (`claim_not_held_by_actor`),
  so a non-holder who *could* witness cannot be the gatherer.

No single-character fixture can produce the envelope's beats. This is
not an RS01 implementation choice: the grammar itself forces witnessing
to be a **second person's act**. The moment is social by law.

### Corrected canonical trace (all other envelope numbers survive)

Two characters. C1 "holder/gatherer", C2 "witness", both start at 65
(Steady). Claim K1 held by C1 on S1 (Established, 2,000 g), unwitnessed.

```text
Beat 0  initial:   C1 65 · C2 65 · K1 unwitnessed · S1 2000 g · inv 0 g
Beat 1  Gather(C1,K1,S1) → Refused(claim_not_witnessed), zero mutation
Beat 2  Witness(C2,K1)   → Accepted, C2 65→60 (−5), K1 false→true
Beat 3  Gather(C1,K1,S1) → Accepted, C1 65→53 (−12, steady),
                            S1 2000→800 g, C1 inventory 0→1200 g
Beat 4  aftermath: C1 53 · C2 60 · K1 witnessed · S1 800 g · inv 1200 g
```

Surviving envelope numbers: witness cost 5, gather cost 12 (Steady),
yield 1,200 g (Steady x Established), stock 2,000→800 g, inventory
0→1,200 g, refusal reason, zero-mutation refusal. Changed: the single
65→60→48 line becomes C1 65→53 and C2 65→60 — the two costs land on the
two people the law requires.

A falsification test pins the discrepancy itself: the single-actor
version of beat 2 must refuse with `cannot_witness_own_claim`.

### Environment notes

- Software Vulkan (lavapipe) installed; Xvfb present. Real windowed
  rendering + screenshots will be attempted headlessly. If capture
  fails, that sub-claim is recorded `INCONCLUSIVE_ENVIRONMENT`.
- Capture mode (`RS01_CAPTURE`) drives the same submission code path as
  the interactive keys, auto-issuing the three commands for screenshot
  runs only. Recorded substitution: the primary test path remains the
  player's three submissions; capture mode exists because this
  environment has no human hand.
