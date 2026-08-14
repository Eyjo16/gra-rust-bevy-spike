# RS01 visual fact map

Status: implementation evidence for independent review. This map describes
the default Bevy view; it is not a registry, schema, gameplay contract, or
source of canonical state.

Every meaningful default-view element has exactly one classification:

- `PUBLICATION_FACT` — value read from typed facts copied from identified
  Bevy publication-view entities.
- `RECEIPT_FACT` — value read from the canonical receipt returned by the
  player-submitted command.
- `DETERMINISTIC_DERIVATION` — fixed presentation mapping from named
  Publication/receipt inputs; no writeback and no independent state.
- `EXPRESSION_POLICY` — authored alias, material, atmosphere, palette, or
  layout that claims no additional world meaning.
- `INTERACTION_POLICY` — local input guidance; it chooses when to submit an
  already-canonical command and never decides legality or outcome.

## Default view

| Visual ID | Exactly one class | Input and mapping | Authority limit |
| --- | --- | --- | --- |
| `frame.background` | `EXPRESSION_POLICY` | pale cold-morning frame | atmosphere only; no cold pressure, danger, need, or historical claim |
| `frame.beat_heading` | `DETERMINISTIC_DERIVATION` | current beat kind maps to a word-only state label | labels display state only; the game-facing layer shows no numeric counter |
| `narrative.initial` | `DETERMINISTIC_DERIVATION` | K1 unwitnessed Publication fact + authorized cold-morning mood | “closed” means gather is gated; no household need or winter danger |
| `narrative.refusal` | `DETERMINISTIC_DERIVATION` | receipt outcome `refused/claim_not_witnessed` and equal before/after identity | does not add motive, emotion, law, or politics |
| `narrative.witness` | `DETERMINISTIC_DERIVATION` | accepted witness receipt; C2 stamina delta; K1 witnessed Publication fact | alias and prose add no relationship, honor, or permanent future claim |
| `narrative.gather` | `DETERMINISTIC_DERIVATION` | accepted gather receipt; C1 stamina, inventory, and S1 stock Publication deltas | “peat” and “stack” are presentation aliases |
| `narrative.aftermath` | `DETERMINISTIC_DERIVATION` | final Publication equals gathered Publication | states current cost/gain only; claims no continuation |
| `actor.aliases` | `EXPRESSION_POLICY` | C1→Snorri; C2→Thordur | no class, office, kinship, duty, or relationship implied |
| `actor.silhouettes` | `EXPRESSION_POLICY` | abstract block figures | no body, mood, age, status, or historical claim |
| `layout.positions` | `EXPRESSION_POLICY` | fixed scene composition and actor movement | spatial staging is not canonical location or distance |
| `actor.stamina_bars` | `PUBLICATION_FACT` | C1/C2 stamina on one stable 0–100 scale | color/layout are policy; fill is the published value |
| `site.alias_and_material` | `EXPRESSION_POLICY` | S1→“Peat bog at Hvammur”; stock→equal peat blocks | does not assert historical correctness or household need |
| `site.witness_seal` | `PUBLICATION_FACT` | K1 witnessed boolean | label/palette express only false/true gate state |
| `site.turf_blocks` | `DETERMINISTIC_DERIVATION` | `floor(S1 stock_g / 200 g)`, capped at the fixture's ten equal units | the default shows equal blocks but no number or gram label; the proof overlay exposes that 2,000→800 renders 10→4 |
| `inventory.turf_blocks` | `DETERMINISTIC_DERIVATION` | `floor(C1 inventory_g / 200 g)` | the default shows equal blocks but no number; proof exposes that 1,200 g renders six blocks |
| `outcome.banner` | `RECEIPT_FACT` | receipt outcome/reason/verb mapped to refused, witness accepted, or gather accepted copy | banner cannot decide or replace the canonical outcome |
| `aftermath.cost` | `DETERMINISTIC_DERIVATION` | receipt deltas: C1 60→48; C2 30→25; S1 2,000→800 g | default copy says only that stamina and bog stock fell; stable bars/blocks carry the magnitude, while exact deltas remain proof-only |
| `aftermath.gain` | `DETERMINISTIC_DERIVATION` | final K1 witnessed; C1 inventory 1,200 g; mass conservation | default copy states only current gain and conservation, with no exact quantity or continuation claim |
| `interaction.prompt` | `INTERACTION_POLICY` | trace position maps Space/Enter to gather, witness, gather, then aftermath | each of the three actions is submitted through `Host`; renderer cannot choose result; pointer focus cannot advance the trace |
| `palette.state_colors` | `EXPRESSION_POLICY` | red/gold/green and neutral turf/stamina colors | emphasis only; no moral, legal, emotional, or political meaning |

## Proof overlay (not required to understand the default view)

| Element | Source | Limit |
| --- | --- | --- |
| Publication revisions and `derived_from` | identified `Publication` | identity/explanation only |
| exact block scale | expression policy plus the renderer's fixed unit mapping | proof-only declaration that each equal block represents 200 g |
| C1/C2 stamina, C1 inventory, S1 stock, K1 witnessed | the same typed Publication facts used by the game view | exact values never feed back into truth |
| canonical receipt line | receipt returned by the same submitted command | display only; not reparsed and not an authority source |
| presentation-policy footer | authored aliases, material, atmosphere, palette, and layout | proof-only disclosure; never treated as world state |

The default layer contains no numeric counter, gram label, exact written
quantity, receipt, revision, hash, ID, engine-boundary vocabulary, or
presentation-policy disclosure. Exact values and expression disclosures
remain available in this map and the optional proof overlay for F5 audit
without making the game-facing scene ledger-first.

## Mechanical bindings

- `primary_path_submits_exactly_one_command_per_player_advance` proves the
  interactive path starts with zero receipts and adds one receipt for each of
  the three player advances.
- `rs01_trace_is_publication_identified_and_receipt_derived` binds the five
  expression beats to the required live trace.
- `replay_expression_states_are_deterministic` compares beat kind,
  Publication identity, typed scene facts, and canonical receipt lines across
  two fresh executions.
- `every_default_visual_has_exactly_one_fact_map_row` refuses missing or
  duplicate rows for the default-view IDs above.
- `default_copy_omits_ledger_and_exact_quantities` rejects numerals,
  proof-ledger terms, engine vocabulary, and policy disclosure in every
  default narrative, heading, prompt, and aftermath line.
- Default and `bevy-host` gates plus the off-by-default feature manifest prove
  deletion/isolation; R01–R03 tests retain projection non-authority and stale
  Publication rejection.

Human F1/F10 comprehension remains deliberately unmeasured until an
unbriefed viewer runs the scene. Question three is a continuation signal, not
an RS01 pass requirement and not permission to add a verb or system.
