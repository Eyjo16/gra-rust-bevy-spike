# RS01 human F1/F10 review protocol

Date: 2026-08-14. Branch: `review/RS01-human-protocol`. Base:
`2dd4db5db6f52b287ebf4f6b8a3d259bf30ba028`.

Status: **PROTOCOL READY; HUMAN VERDICT NOT RUN OR CLAIMED.**

This is a docs-only protocol for the two mechanical gaps explicitly retained
by the integrated RS01 report. It changes no renderer, truth, presentation
copy, value, contract, registry/schema, or meaning.

## Review object

- executable: truth master at the exact commit recorded by the run
- command: `cargo run --features bevy-render -- rs01-render`
- layer under review: default game-facing view only
- excluded during F1/F10: `--proof`, source code, fact map, receipts, hashes,
  prior explanation, and leading terminology
- exact introductory script: **“Try this short scene.”**
- interaction available on screen: the existing Space/Enter prompt
- trace: initial → refused gather → accepted witness → accepted gather →
  aftermath
- scope: one scene, two existing verbs, three real Host submissions

The proof overlay may be shown only after all default-view answers are locked.

## Why two gates

“F1” asks whether the presentation communicates the immediate action and
obstacle. “F10” asks whether the viewer can reconstruct the causal chain
without coaching. Neither gate asks whether the viewer likes the art, wants a
larger game, or has learned the engine vocabulary.

A single pilot can falsify a strong comprehension claim. A single success
cannot establish population-level comprehension. Lead and author must choose
sample size and acceptance threshold before any broader claim.

## Session controls

1. Use a clean exact truth commit and record full SHA, OS, GPU/backend, window
   size, and whether the run is interactive or capture replay.
2. Start a new process at the initial beat. Do not rehearse or explain the
   scene.
3. Read only the exact introductory script.
4. Do not answer questions about goals, controls beyond the visible prompt,
   terms, colors, or causality until the final response is locked. Record any
   spontaneous question.
5. Preserve the viewer's words verbatim. Do not translate their answer into
   project vocabulary during the session.
6. Record every Space/Enter press and the visible beat reached.
7. Ask the F1 questions immediately after the first refusal.
8. Let the viewer continue using only the visible prompt, then ask F10 after
   aftermath.
9. Ask inference-control and continuation questions last.
10. Only then may the proof overlay and fact map be shown for audit.

## F1 — immediate action and obstacle

Ask verbatim:

1. “What were you trying to do?”
2. “What happened?”
3. “Why do you think it did not work?”

Record an exact answer and a confidence rating chosen by the viewer.

### Pre-registered scoring

| Code | Evidence in the answer | Reading |
|---|---|---|
| F1-A | identifies collecting/taking material from the site | immediate action communicated |
| F1-B | identifies that the first attempt failed/refused | outcome communicated |
| F1-C | identifies missing witness/attestation/closed claim as the obstacle, without requiring project terminology | causal obstacle communicated |
| F1-X | gives a different cause such as hunger, strength, distance, weather, or random failure | false or unsupported inference |
| F1-? | cannot tell | comprehension failure, not viewer error |

Provisional per-viewer F1 pass requires A+B+C and no X. This rubric itself
awaits lead review before a multi-person gate.

## F10 — uncoached causal chain

Ask verbatim after the aftermath:

1. “Tell me the story of what changed, in order.”
2. “What made the later attempt different from the first?”
3. “What did the successful action cost?”
4. “What did it produce or move?”
5. “Which parts are you certain of, and which are guesses?”

Exact numbers are neither requested nor required.

### Pre-registered scoring

| Code | Evidence in the answer |
|---|---|
| F10-A | first collection attempt failed |
| F10-B | another actor witnessed/attested or opened the claim |
| F10-C | a later collection attempt succeeded because that gate changed |
| F10-D | understands the acting character spent bodily capacity/stamina |
| F10-E | understands site stock fell and a carried/owned stack rose |
| F10-F | preserves causal order rather than treating frames as unrelated |
| F10-X | claims unsupported hunger, danger, obligation, kinship, permanence, legal title, emotion, or random outcome as displayed fact |

Provisional per-viewer F10 pass requires A–F and no X. A viewer may use ordinary
language. Engine, receipt, Publication, gram, hash, ID, or exact-delta terms
must not be necessary.

## Inference-control and continuation

After F10 scoring, ask:

- “What else did the scene make you assume about these people or this place?”
- “What do you think might happen next?”
- “Would you choose to continue? Why or why not?”

The first question detects beautiful lies in expression. The last two are
design signals only. They cannot turn F1/F10 red into green and cannot
authorize a new verb, relationship, historical claim, or mechanic.

## Evidence bundle per viewer

```text
viewer_code:
prior_exposure:
exact_truth_commit:
environment:
start/end timestamps:
input sequence:
verbatim spontaneous comments:
F1 verbatim answers:
F1 codes and reviewer rationale:
F10 verbatim answers:
F10 codes and reviewer rationale:
unsupported inferences:
continuation answer:
session deviations:
independent scorer:
disagreements:
```

Use pseudonymous viewer codes and collect no unnecessary personal data.
A scorer who authored the presentation may transcribe but must not be the only
semantic judge. Disagreement follows the project's one-circle rule: one
evidence exchange, then lead/author verdict.

## Mechanical preflight before each human run

- full default, bevy-host, and bevy-render gates green at the exact commit;
- interactive path begins with zero receipts;
- three input advances create exactly three receipts;
- refused beat and initial Publication are byte-identical;
- witness/gather beats bind to the expected Publication and receipt sequence;
- default layer excludes exact quantities and proof/engine terms;
- renderer is off by default and has no canonical observation backdoor;
- window is fully visible and text is not clipped.

A failed preflight invalidates the session; it is not scored as human
comprehension evidence.

## Stop conditions

Stop and report, without editing copy during the same evidence round, if:

- the viewer cannot advance using the visible prompt;
- any question has to be coached;
- capture/window clipping hides material text;
- the trace or identity differs from the recorded mechanical object;
- a proof overlay or prior explanation was exposed;
- the reviewer begins tuning wording against the current viewer before the
  verdict is recorded.

If copy changes later, it is a new presentation candidate and requires a fresh
unbriefed run.

## Review requests

Lead/author must decide before dispatch:

1. whether the provisional A+B+C and A–F rubrics match intended F1/F10;
2. pilot and ratification sample sizes;
3. whether one or two independent scorers are required;
4. which language(s) the default copy must be tested in;
5. whether the current English ASCII aliases remain the object or a localized
   candidate is created on a separate branch.

## Claims table

| # | Atomic claim | Scope | Evidence |
|---|---|---|---|
| 1 | RS01's F1 and F10 remain unmeasured on master | integrated report at `2dd4db5` | rs01-live-render-report.md |
| 2 | The mechanical trace and fact map already define the bounded review object | integrated RS01 only | report, fact map, renderer tests |
| 3 | This protocol separates immediate obstacle comprehension, causal-chain comprehension, unsupported inference, and continuation interest | this docs-only proposal | scoring sections above |
| 4 | No human session or comprehension pass is claimed by this branch | branch state | no completed viewer record |
| 5 | No code, value, contract, registry/schema, vocabulary, or meaning changes | branch diff | docs-only write scope |

## Verification

Docs-only staged tree: format and all three strict Clippy suites passed;
tests passed 56 default / 65 bevy-host / 73 bevy-render. Both feature-enabled
runtime probes exited 0 with receipts, state, and world parity true; the
frozen `10v4` envelope remained unchanged.
