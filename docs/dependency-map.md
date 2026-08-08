# Dependency and tool-tree map — truth-layer slice 001

Six views: the cargo tree, the module graph, the ownership tree, the flow
of one command, the oracle input map, and — for tomorrow's hypothesis
work — exactly where the tunable values live.

## 1. Cargo dependency tree

The default build is the pure boundary and has **zero external
dependencies** — `cargo tree` is one line:

```
gra-rust-bevy-spike v0.1.0
```

The `bevy-host` feature (OFF by default, host on HOLD) pulls in the full
engine — about 1,480 nodes in `cargo tree`. Depth 1–2:

```
gra-rust-bevy-spike v0.1.0
├── bevy v0.19.0
│   └── bevy_internal v0.19.0        (→ renderer, windowing, assets, …)
└── bevy_ecs v0.19.0
    ├── bevy_reflect, bevy_tasks, bevy_platform, bevy_ptr, bevy_utils
    ├── serde, thiserror, smallvec, indexmap, fixedbitset, slotmap
    └── … (proc-macros and support crates)
```

This is the concrete cost of lifting the HOLD: compile time and surface
area jump from zero to the whole engine. The truth layer itself never
references Bevy, so the boundary stays testable at zero-dependency speed
regardless.

## 2. Module graph

`boundary` defines the shared primitives (IDs, `Stamina`, `MassGrams`,
reasons, receipts, hashing) and orchestrates; each owner module depends
only on those primitives — never on another owner:

```mermaid
flowchart TD
    main["main.rs<br/>fixture + trial + gate"]
    oracles["oracles.rs<br/>7 bounded checks"]
    boundary["boundary.rs<br/>primitives + orchestrator"]
    character["character/<br/>stamina owner"]
    economy["economy/<br/>mass owner"]
    social["social/<br/>claims owner"]

    main --> boundary
    main --> oracles
    oracles --> boundary
    boundary -->|"validate / apply"| character
    boundary -->|"validate / apply"| economy
    boundary -->|"validate / apply"| social
    character -.->|"primitives only"| boundary
    economy -.->|"primitives only"| boundary
    social -.->|"primitives only"| boundary
```

The dotted edges are the module cycle Rust allows within a crate: owners
import *types* from `boundary`, but only the orchestrator calls *into*
owners. Owners never import each other — cross-system effects can only
travel through the boundary.

## 3. Ownership tree (who may write what)

```mermaid
flowchart TD
    world["World"]
    chr["CharacterOwner"]
    eco["EconomyOwner"]
    soc["SocialOwner"]
    stamina["stamina: BTreeMap&lt;CharacterId, Stamina&gt;"]
    sites["sites: BTreeMap&lt;SiteId, {tier, stock}&gt;"]
    inv["inventories: BTreeMap&lt;CharacterId, MassGrams&gt;"]
    claims["claims: BTreeMap&lt;ClaimId, {holder, site, witnessed}&gt;"]

    world --> chr --> stamina
    world --> eco --> sites
    eco --> inv
    world --> soc --> claims
```

Every leaf is private to its module. The only mutation paths are
`apply_spend` (character) and `apply_extract` (economy) — both take a
proof token with private fields, so only a validation inside the same
owner can mint the right to write. Social has no apply in this slice.

## 4. Flow of one command (validate everything, then apply)

```mermaid
flowchart TD
    cmd["GatherCommand<br/>{actor, claim, site}"]
    g1{"1. social gate<br/>claim exists · held by actor<br/>covers site · witnessed"}
    g2{"2. character gate<br/>actor exists · not exhausted"}
    g3{"3. economy gate<br/>site exists · stock &gt; 0"}
    cell["4x4 cell lookup<br/>YIELD_TABLE_GRAMS[band][tier]"]
    plan["GatherPlan<br/>WitnessPass + StaminaSpend + Extraction"]
    apply["infallible applies<br/>spend stamina · move mass"]
    receipt["canonical Receipt + world hash"]
    refused["Refused(reason)<br/>state untouched, hash unchanged"]

    cmd --> g1
    g1 -->|pass| g2
    g2 -->|pass| cell --> g3
    g3 -->|"granted == requested"| plan
    g3 -->|"granted &lt; requested → Partial"| plan
    plan --> apply --> receipt
    g1 -->|fail| refused
    g2 -->|fail| refused
    g3 -->|fail| refused
```

Nothing mutates until all three gates pass — a refusal at any gate leaves
the world hash byte-identical.

The second verb, `witness`, follows the same shape with a different
policy: social gate (claim exists, not own claim, not already witnessed)
→ character gate (flat `WITNESS_COST`, no exhausted gate) → apply
(spend stamina, flip the claim's boolean gate). Economy is untouched and
no mass moves. See `docs/verb-isolation-report.md` for the isolation
proof.

## 5. Oracle input map

```mermaid
flowchart LR
    world["World (current state)"]
    log["Receipt log"]
    base["baseline mass"]
    fix["fixture + commands<br/>(replayable)"]

    o1["1 stamina_in_bounds"]
    o2["2 mass_conserved"]
    o3["3 witnessed_gate"]
    o4["4 exhausted_gate"]
    o5["5 closed_reasons"]
    o6["6 cell_bounds"]
    o7["7 replay_determinism"]
    o8["8 refusal_zero_mutation"]
    o9["9 shadow_expectation"]

    world --> o1
    world --> o2
    base --> o2
    log --> o3
    log --> o4
    log --> o5
    log --> o6
    world --> o7
    log --> o7
    fix --> o7
    log --> o8
    fix --> o8
    log --> o9
    fix --> o9
```

Oracles 1–2 audit the state, 3–6 audit the receipt log, 7 replays the
whole trial through the real implementation, 8 walks the receipt hash
chain (refusals must be byte-identical no-ops, yields must change the
hash), and 9 is an **independent shadow evaluator**: it recomputes every
expected outcome from the immutable fixture with its own state tracking
and its own band thresholds, never reading a receipt field. The layers
catch different lie classes: forged state trips 1/2/7, forged receipts
trip 3–6/7/8, and an implementation that lies *consistently* — receipts
and replay agreeing on wrong semantics — trips 9.

## 6. Where the values live (for hypothesis work)

Everything tunable sits in exactly four places — change a number, run the
gate, and the receipts + oracles show the spread immediately:

| What | Where | Currently |
| --- | --- | --- |
| Yield per band × tier | `YIELD_TABLE_GRAMS` in `src/boundary.rs` | 4×4 table, 0–2700 g |
| Stamina cost per band (gather) | `STAMINA_COST_BY_BAND` in `src/boundary.rs` | `[0, 15, 12, 10]` |
| Witness cost (flat) | `WITNESS_COST` in `src/boundary.rs` | `5` |
| Band thresholds | `Stamina::band` in `src/boundary.rs` | 0–9 / 10–39 / 40–79 / 80–100 |
| Fixture (actors, sites, claims, commands) | `fixture()` + `commands()` in `src/main.rs` | 4 actors, 4 sites, 9 claims, 16 commands (two verbs) |

Every receipt now carries a `grammar=0x…` fingerprint hashed from the
yield table, the cost table, the realized band mapping over 0–100, and
the closed reason codes — so a trial record always identifies which
grammar version produced it. One caveat when experimenting: the shadow
evaluator (oracle 9) carries its **own** band-threshold literals on
purpose. Changing thresholds in `Stamina::band` alone turns oracle 9 red
until the shadow is updated to match — a threshold change must be made
consciously in both places, which is the point.

Suggested loop for a data-spread hypothesis: edit one table or the
fixture, run `cargo run`, read the canonical receipt lines as the
experiment record, and let the nine oracles veto any spread that breaks
an invariant. The oracle test fixture in `src/oracles.rs` is intentionally
separate and smaller, so `cargo test` keeps guarding the logic while
`main.rs` becomes the playground.

All numbers above remain mechanical examples — not balance, not
historical truth — until a hypothesis promotes them.
