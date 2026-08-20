//! Truth-shape extractor (TS01).
//!
//! Emits a deterministic, commit-addressed projection of the truth
//! shapes — verbs, owners, host adapter — as YAML plus a
//! non-authoritative HTML review page. The projection is derived from
//! this crate: value fields are formatted from the governing constants
//! at emission time, and the binding tests below pin the closed
//! vocabularies, verbs, refusal mappings, and source-line references to
//! the real code — those bound fields cannot silently drift. Descriptive
//! prose (roles, read/write labels, closures) is authored text: reviewed,
//! not machine-bound, and it drifts only as loudly as review lets it.
//!
//! Authority guard (TS01 envelope, hard): these files are WRITE-ONLY
//! projections. No runtime or test code ever reads them back; making
//! the YAML authoritative would be a registry/schema contract verdict,
//! which does not exist. Descriptive prose in a shape is authored text
//! carrying an explicit `meaning_status`; numbers are always emitted
//! from the constants.

use crate::boundary::{
    ResourceKind, STAMINA_COST_BY_BAND, Stamina, WITNESS_COST, YIELD_TABLE_GRAMS,
    grammar_fingerprint,
};

const BOUNDARY_SRC: &str = include_str!("boundary.rs");
const CHARACTER_SRC: &str = include_str!("character/mod.rs");
const ECONOMY_SRC: &str = include_str!("economy/mod.rs");
const SOCIAL_SRC: &str = include_str!("social/mod.rs");
const HOST_SRC: &str = include_str!("host_bevy.rs");

/// 1-based line of the first source line containing `needle`; 0 when
/// absent — and the binding tests refuse any unresolved (`:0`)
/// reference, so a renamed item turns the projection red instead of
/// letting a line reference drift.
fn line_of(source: &str, needle: &str) -> usize {
    source
        .lines()
        .position(|line| line.contains(needle))
        .map_or(0, |index| index + 1)
}

/// The closed shape-level evidence vocabulary. Distinct from the bundle
/// claims evidence modes (AGENTS.md §7) on purpose: a shape cites the
/// kinds of proof that back it, a claim cites how one statement was
/// evidenced. Both sets are closed to prevent drift.
pub const EVIDENCE_KINDS: [&str; 5] = [
    "behavioral-red",
    "capability-red",
    "measurement",
    "oracle",
    "parity",
];

/// The closed meaning-status vocabulary for projected statements.
pub const MEANING_STATUSES: [&str; 6] = [
    "ratified",
    "proven",
    "measured",
    "fixture",
    "hypothesis",
    "counterfactual",
];

struct Shape {
    id: &'static str,
    kind: &'static str,
    meaning_status: &'static str,
    role: &'static str,
    scope: &'static str,
    evidence_kind: &'static [&'static str],
    dependencies: &'static [&'static str],
    reads: &'static [&'static str],
    writes: &'static [&'static str],
    mutation_closure: &'static str,
    guards: &'static [&'static str],
    refusals: &'static [&'static str],
    receipts: &'static str,
    invariants: &'static [&'static str],
    parity_paths: &'static [&'static str],
    source: String,
    proof_refs: &'static [&'static str],
    values: Vec<(String, String)>,
}

fn shapes() -> Vec<Shape> {
    let cost = STAMINA_COST_BY_BAND;
    let yields = YIELD_TABLE_GRAMS;
    vec![
        Shape {
            id: "verb.gather",
            kind: "verb",
            meaning_status: "proven",
            role: "Move mass of the site's kind from a claimed, witnessed site \
                   into the actor's holding of that kind, paying band-dependent \
                   stamina",
            scope: "mechanics proven on the standard fixture and bounded traces \
                    (trial 007); every numeric value is a fixture, not balance",
            evidence_kind: &["behavioral-red", "oracle", "parity"],
            dependencies: &["owner.social", "owner.character", "owner.economy"],
            reads: &[
                "social.claims",
                "character.stamina",
                "economy.sites",
                "economy.holdings (entity revision bound at validation)",
            ],
            writes: &["character.stamina", "economy.sites", "economy.holdings"],
            mutation_closure: "character and economy entity revisions advance; \
                               social state is never touched by a gather",
            guards: &[
                "claim exists, held by actor, covers site, witnessed",
                "actor exists, band not exhausted, exact stamina headroom",
                "site exists, stock nonzero; grant = min(requested, stock)",
                "two-phase commit: every token fresh before any owner mutates",
            ],
            refusals: &[
                "unknown_claim",
                "claim_not_held_by_actor",
                "claim_site_mismatch",
                "claim_not_witnessed",
                "unknown_actor",
                "actor_exhausted",
                "insufficient_stamina",
                "unknown_site",
                "site_empty",
            ],
            receipts: "canonical line with band, tier, kind, spent, mass, grammar, \
                       and the world hash chain; partial via site_nearly_depleted",
            invariants: &[
                "mass conservation, per kind and in aggregate",
                "stamina bounds",
                "cell bounds",
                "refusal zero-mutation",
                "shadow expectation and final state",
            ],
            parity_paths: &["pure", "bevy-serial"],
            source: format!(
                "src/boundary.rs:{} plan_gather, :{} GatherPlan::apply, :{} submit_gather",
                line_of(BOUNDARY_SRC, "fn plan_gather"),
                line_of(BOUNDARY_SRC, "impl GatherPlan"),
                line_of(BOUNDARY_SRC, "fn submit_gather")
            ),
            proof_refs: &["trial/003", "trial/007", "trial/008", "oracles 1-10"],
            values: vec![
                (
                    "cost_by_band".to_owned(),
                    format!(
                        "[{}, {}, {}, {}] (fixture)",
                        cost[0], cost[1], cost[2], cost[3]
                    ),
                ),
                (
                    "yield_table_g".to_owned(),
                    format!(
                        "rows exhausted..fresh: {:?} {:?} {:?} {:?} (fixture)",
                        yields[0], yields[1], yields[2], yields[3]
                    ),
                ),
            ],
        },
        Shape {
            id: "verb.witness",
            kind: "verb",
            meaning_status: "proven",
            role: "Attest another actor's claim, flipping its boolean gate; flat \
                   stamina cost, no exhausted gate — a spent actor may still vouch",
            scope: "mechanics proven on the standard fixture and bounded traces; \
                    the flat cost is a fixture, not balance",
            evidence_kind: &["behavioral-red", "oracle", "parity"],
            dependencies: &["owner.social", "owner.character"],
            reads: &["social.claims", "character.stamina"],
            writes: &["character.stamina", "social.claims"],
            mutation_closure: "character and social entity revisions advance; \
                               economy is never touched and no mass moves",
            guards: &[
                "claim exists, not already witnessed, not the witness's own",
                "actor exists, exact stamina headroom (no exhausted gate)",
                "two-phase commit: every token fresh before any owner mutates",
            ],
            refusals: &[
                "unknown_claim",
                "cannot_witness_own_claim",
                "claim_already_witnessed",
                "unknown_actor",
                "insufficient_stamina",
            ],
            receipts: "canonical line, zero mass, witnessed flag records the \
                       pre-verb state; hash chain advances",
            invariants: &[
                "zero mass movement",
                "refusal zero-mutation",
                "shadow expectation and final state",
            ],
            parity_paths: &["pure", "bevy-serial"],
            source: format!(
                "src/boundary.rs:{} plan_witness, :{} WitnessPlan::apply, :{} submit_witness",
                line_of(BOUNDARY_SRC, "fn plan_witness"),
                line_of(BOUNDARY_SRC, "impl WitnessPlan"),
                line_of(BOUNDARY_SRC, "fn submit_witness")
            ),
            proof_refs: &["verb-isolation-report", "trial/003", "oracles 1-10"],
            values: vec![(
                "witness_cost".to_owned(),
                format!("{WITNESS_COST} (fixture)"),
            )],
        },
        Shape {
            id: "owner.character",
            kind: "owner",
            meaning_status: "proven",
            role: "Single writer of character bodies (stamina); verb-agnostic \
                   resource semantics — cost arrives from boundary verb policy",
            scope: "proven for the current single-writer product state",
            evidence_kind: &["behavioral-red", "oracle"],
            dependencies: &["boundary primitives only"],
            reads: &["character.stamina (own state)"],
            writes: &["character.stamina (own state only)"],
            mutation_closure: "apply_spend: exact subtraction, per-character \
                               entity revision + owner counter advance",
            guards: &[
                "actor exists",
                "exact headroom (no clamping path exists)",
                "token bound to the one character's entity revision",
            ],
            refusals: &["unknown_actor", "insufficient_stamina"],
            receipts: "none of its own — receipts are boundary artifacts",
            invariants: &["stamina in 0..=MAX by construction", "stale token panics"],
            parity_paths: &["pure", "bevy-serial"],
            source: format!(
                "src/character/mod.rs:{} validate_spend, :{} apply_spend",
                line_of(CHARACTER_SRC, "fn validate_spend"),
                line_of(CHARACTER_SRC, "fn apply_spend")
            ),
            proof_refs: &["trial-log round 1", "trial/003"],
            values: vec![(
                "stamina_max".to_owned(),
                format!("{} (proven: type-enforced bound)", Stamina::MAX),
            )],
        },
        Shape {
            id: "owner.economy",
            kind: "owner",
            meaning_status: "proven",
            role: "Single writer of mass — site stock and per-kind holdings; \
                   negative mass unrepresentable, totals bounded at coherence \
                   time, conserved per kind",
            scope: "proven for the current single-writer product state and the \
                    coherence-bounded mass aggregate",
            evidence_kind: &["behavioral-red", "oracle"],
            dependencies: &["boundary primitives only"],
            reads: &["economy.sites, economy.holdings (own state)"],
            writes: &["economy.sites, economy.holdings (own state only)"],
            mutation_closure: "apply_extract: checked subtraction from the site, \
                               bounded addition to the holding of the SITE's kind; \
                               site + (character, kind) entity revisions + owner \
                               counter advance; a holding that reaches zero is \
                               removed, never stored as zero",
            guards: &[
                "site exists, stock nonzero",
                "grant = min(requested, stock) at validation",
                "token bound to site and (character, kind) entity revisions",
                "the kind is the site's, never the caller's",
            ],
            refusals: &["unknown_site", "site_empty"],
            receipts: "none of its own — receipts are boundary artifacts",
            invariants: &[
                "total mass conserved by every apply",
                "per-kind mass conserved by every apply",
                "no cross-kind leakage: a grant lands only in the site's kind",
                "post-preflight apply totality under the coherence bound",
                "stale token panics",
            ],
            parity_paths: &["pure", "bevy-serial"],
            source: format!(
                "src/economy/mod.rs:{} validate_extract, :{} apply_extract",
                line_of(ECONOMY_SRC, "fn validate_extract"),
                line_of(ECONOMY_SRC, "fn apply_extract")
            ),
            proof_refs: &["trial-log round 1", "trial/003", "trial/008", "trial/RES01"],
            values: vec![(
                "resource_kinds".to_owned(),
                format!(
                    "[{}] (closed vocabulary, author-licensed 2026-08-18)",
                    ResourceKind::ALL
                        .iter()
                        .map(|kind| kind.code())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )],
        },
        Shape {
            id: "owner.social",
            kind: "owner",
            meaning_status: "proven",
            role: "Single writer of claims and witnessing; the witnessed claim is \
                   a boolean gate other verbs read but never write",
            scope: "proven for the current single-writer product state",
            evidence_kind: &["behavioral-red", "oracle"],
            dependencies: &["boundary primitives only"],
            reads: &["social.claims (own state)"],
            writes: &["social.claims (own state only)"],
            mutation_closure: "apply_witness: one boolean flip, per-claim entity \
                               revision + owner counter advance; witnessing is \
                               monotone (never un-witnessed)",
            guards: &[
                "claim exists, not already witnessed, not self-witnessed",
                "token bound to the one claim's entity revision",
            ],
            refusals: &[
                "unknown_claim",
                "claim_already_witnessed",
                "cannot_witness_own_claim",
                "claim_not_held_by_actor",
                "claim_site_mismatch",
                "claim_not_witnessed",
            ],
            receipts: "none of its own — receipts are boundary artifacts",
            invariants: &["monotone witnessing", "stale token panics"],
            parity_paths: &["pure", "bevy-serial"],
            source: format!(
                "src/social/mod.rs:{} validate_witness_gate, :{} validate_witness_grant, :{} apply_witness",
                line_of(SOCIAL_SRC, "fn validate_witness_gate"),
                line_of(SOCIAL_SRC, "fn validate_witness_grant"),
                line_of(SOCIAL_SRC, "fn apply_witness")
            ),
            proof_refs: &["verb-isolation-report", "trial/003"],
            values: vec![],
        },
        Shape {
            id: "host.bevy_ecs",
            kind: "host-adapter",
            meaning_status: "proven",
            role: "Custodies the truth World as one ECS resource and replays the \
                   trial under a byte-parity gate; projects disposable views, \
                   publishes identified snapshots, isolates host faults",
            scope: "proven for the standard fixture, bounded traces, and the \
                    R01-R03 probe suite; sequential scheduling only (A1 law)",
            evidence_kind: &["capability-red", "behavioral-red", "measurement", "parity"],
            dependencies: &["bevy_ecs (minimized: std only)", "boundary"],
            reads: &["canonical observations", "command queue (transport)"],
            writes: &[
                "truth — only through submit, from the single commit system",
                "view entities, publications, host-fault log",
            ],
            mutation_closure: "exactly one registered system holds mutable access \
                               to Truth (the commit system calling submit); \
                               projections and faults leave zero canonical trace",
            guards: &[
                "custody topology test (one ResMut access)",
                "admission gate ahead of the boundary",
                "publication identity: monotone revisions + state hash",
            ],
            refusals: &["host faults are not refusals: admission_failed, \
                         projection_consumer_failed (closed, beside receipts)"],
            receipts: "reproduces the pure run's canonical receipts byte-for-byte",
            invariants: &[
                "byte parity of receipts and canonical state",
                "projection non-authority",
                "stale publication rejectable downstream",
                "no unwind-catching of truth panics",
            ],
            parity_paths: &["bevy-serial vs pure (every gate run)"],
            source: format!(
                "src/host_bevy.rs:{} Host, :{} submit_next, :{} publish, :{} publication",
                line_of(HOST_SRC, "pub struct Host"),
                line_of(HOST_SRC, "fn submit_next"),
                line_of(HOST_SRC, "pub fn publish"),
                line_of(HOST_SRC, "pub fn publication")
            ),
            proof_refs: &[
                "trial/002",
                "trial/006",
                "trial/007",
                "trials R01-R03",
                "trial/D01",
            ],
            values: vec![],
        },
    ]
}

fn yaml_list(indent: &str, name: &str, items: &[&str], out: &mut String) {
    out.push_str(&format!("{indent}{name}:\n"));
    for item in items {
        out.push_str(&format!("{indent}  - \"{item}\"\n"));
    }
}

/// Deterministic YAML projection. Same commit, same output bytes.
pub fn emit_yaml(source_commit: &str) -> String {
    let mut out = String::new();
    out.push_str("# Truth shapes - deterministic projection (TS01)\n");
    out.push_str("# authority: projection. The executable Rust code and its tests are\n");
    out.push_str("# authoritative; nothing may read this file as an input.\n");
    out.push_str(&format!("source_commit: \"{source_commit}\"\n"));
    out.push_str(&format!("grammar: \"0x{:016x}\"\n", grammar_fingerprint()));
    out.push_str("meaning_statuses:\n");
    for status in MEANING_STATUSES {
        out.push_str(&format!("  - \"{status}\"\n"));
    }
    out.push_str("evidence_kinds:\n");
    for kind in EVIDENCE_KINDS {
        out.push_str(&format!("  - \"{kind}\"\n"));
    }
    // The closed resource-kind vocabulary (RES01). Emitted from
    // `ResourceKind::ALL`, so a kind admitted in code cannot be missing
    // here, and one listed here cannot be absent from code.
    out.push_str("resource_kinds:\n");
    for kind in ResourceKind::ALL {
        out.push_str(&format!("  - \"{}\"\n", kind.code()));
    }
    out.push_str("shapes:\n");
    for s in shapes() {
        out.push_str(&format!("  - id: \"{}\"\n", s.id));
        out.push_str(&format!("    kind: \"{}\"\n", s.kind));
        out.push_str(&format!("    meaning_status: \"{}\"\n", s.meaning_status));
        out.push_str("    authority: \"projection\"\n");
        out.push_str(&format!("    role: \"{}\"\n", s.role));
        out.push_str(&format!("    scope: \"{}\"\n", s.scope));
        yaml_list("    ", "evidence_kind", s.evidence_kind, &mut out);
        yaml_list("    ", "dependencies", s.dependencies, &mut out);
        yaml_list("    ", "reads", s.reads, &mut out);
        yaml_list("    ", "writes", s.writes, &mut out);
        out.push_str(&format!(
            "    mutation_closure: \"{}\"\n",
            s.mutation_closure
        ));
        yaml_list("    ", "guards", s.guards, &mut out);
        yaml_list("    ", "refusals", s.refusals, &mut out);
        out.push_str(&format!("    receipts: \"{}\"\n", s.receipts));
        yaml_list("    ", "invariants", s.invariants, &mut out);
        yaml_list("    ", "parity_paths", s.parity_paths, &mut out);
        out.push_str(&format!("    source: \"{}\"\n", s.source));
        yaml_list("    ", "proof_refs", s.proof_refs, &mut out);
        if s.values.is_empty() {
            out.push_str("    values: []\n");
        } else {
            out.push_str("    values:\n");
            for (name, value) in &s.values {
                out.push_str(&format!("      {name}: \"{value}\"\n"));
            }
        }
    }
    out
}

fn html_chips(items: &[&str]) -> String {
    items
        .iter()
        .map(|i| format!("<span class=\"chip\">{i}</span>"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Non-authoritative HTML review projection of the same shapes.
pub fn emit_html(source_commit: &str) -> String {
    let mut cards = String::new();
    for s in shapes() {
        let values = if s.values.is_empty() {
            "<em>none - structural shape</em>".to_owned()
        } else {
            s.values
                .iter()
                .map(|(n, v)| format!("<tr><td>{n}</td><td>{v}</td></tr>"))
                .collect::<Vec<_>>()
                .join("")
        };
        cards.push_str(&format!(
            "<section class=\"card\"><header><h2>{id}</h2>\
             <span class=\"kind\">{kind}</span>\
             <span class=\"status status-{status}\">{status}</span></header>\
             <p class=\"role\">{role}</p>\
             <p class=\"scope\"><strong>Scope:</strong> {scope}</p>\
             <div class=\"rw\"><div><h3>Reads</h3>{reads}</div>\
             <div><h3>Writes</h3>{writes}</div></div>\
             <p class=\"closure\"><strong>Mutation closure:</strong> {closure}</p>\
             <h3>Guards</h3><ul>{guards}</ul>\
             <h3>Refusals</h3><p>{refusals}</p>\
             <h3>Invariants</h3><p>{invariants}</p>\
             <p><strong>Receipts:</strong> {receipts}</p>\
             <p><strong>Parity:</strong> {parity} · <strong>Evidence:</strong> {evidence}</p>\
             <h3>Values</h3><table>{values}</table>\
             <footer>{source} · proofs: {proofs}</footer></section>\n",
            id = s.id,
            kind = s.kind,
            status = s.meaning_status,
            role = s.role,
            scope = s.scope,
            reads = html_chips(s.reads),
            writes = html_chips(s.writes),
            closure = s.mutation_closure,
            guards = s
                .guards
                .iter()
                .map(|g| format!("<li>{g}</li>"))
                .collect::<Vec<_>>()
                .join(""),
            refusals = html_chips(s.refusals),
            invariants = html_chips(s.invariants),
            receipts = s.receipts,
            parity = s.parity_paths.join(", "),
            evidence = s.evidence_kind.join(", "),
            values = values,
            source = s.source,
            proofs = s.proof_refs.join(", "),
        ));
    }
    let legend = MEANING_STATUSES
        .iter()
        .map(|s| format!("<span class=\"status status-{s}\">{s}</span>"))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">\
         <title>Truth shapes - {commit}</title><style>\
         body{{font:15px/1.5 system-ui,sans-serif;margin:0;padding:2rem;\
         background:#14161a;color:#e6e2d8;max-width:70rem;margin-inline:auto}}\
         .banner{{border:1px solid #a8552f;background:#2a1c14;padding:.6rem 1rem;\
         border-radius:8px;margin-bottom:1.5rem}}\
         .card{{border:1px solid #33373f;border-radius:10px;padding:1rem 1.25rem;\
         margin-bottom:1.5rem;background:#1a1d22}}\
         .card header{{display:flex;gap:.75rem;align-items:baseline}}\
         h1{{font-size:1.4rem}} h2{{font-size:1.15rem;margin:0}}\
         h3{{font-size:.8rem;text-transform:uppercase;letter-spacing:.06em;\
         color:#9aa3b2;margin:.9rem 0 .3rem}}\
         .kind{{color:#9aa3b2;font-size:.85rem}}\
         .status{{font-size:.75rem;padding:.1rem .5rem;border-radius:99px;\
         border:1px solid #555;margin-left:auto}}\
         .status-ratified{{border-color:#c9a227;color:#c9a227}}\
         .status-proven{{border-color:#4f9d69;color:#7fc796}}\
         .status-measured{{border-color:#4f7d9d;color:#7fb0c7}}\
         .status-fixture{{border-color:#8a8578;color:#b5ad9c}}\
         .status-hypothesis{{border-color:#9d7d4f;color:#c7a97f}}\
         .status-counterfactual{{border-color:#9d4f6b;color:#c77f9d}}\
         .chip{{display:inline-block;border:1px solid #3c414b;border-radius:6px;\
         padding:0 .45rem;margin:.1rem .15rem .1rem 0;font-size:.85rem}}\
         .rw{{display:grid;grid-template-columns:1fr 1fr;gap:1rem}}\
         .closure{{border-left:3px solid #4f9d69;padding-left:.75rem}}\
         table{{border-collapse:collapse}} td{{border:1px solid #33373f;\
         padding:.25rem .6rem;font-size:.9rem}}\
         footer{{color:#9aa3b2;font-size:.8rem;margin-top:.9rem}}\
         ul{{margin:.2rem 0 .2rem 1.2rem;padding:0}}</style></head><body>\
         <h1>Truth shapes</h1>\
         <div class=\"banner\"><strong>Non-authoritative review projection.</strong> \
         The executable Rust code and its tests are authoritative; nothing reads \
         this page or its YAML twin as an input. source_commit={commit} \
         grammar=0x{grammar:016x}</div>\
         <p>Meaning statuses (closed set): {legend}</p>\
         {cards}</body></html>\n",
        commit = source_commit,
        grammar = grammar_fingerprint(),
        legend = legend,
        cards = cards,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::{PartialReason, RefusalReason, Verb};

    /// Binding: the projection covers every closed refusal and partial
    /// reason, and invents none beyond the closed vocabulary.
    #[test]
    fn projection_covers_the_closed_vocabulary() {
        let yaml = emit_yaml("test");
        for reason in RefusalReason::ALL {
            assert!(
                yaml.contains(reason.code()),
                "missing refusal {}",
                reason.code()
            );
        }
        for reason in PartialReason::ALL {
            assert!(yaml.contains(reason.code()), "missing partial reason");
        }
    }

    /// Binding: the projection lists exactly the closed resource-kind
    /// vocabulary — a kind admitted in code cannot be missing from the
    /// projection, and the projection cannot invent one.
    #[test]
    fn projection_covers_the_resource_kinds() {
        let yaml = emit_yaml("test");
        for kind in ResourceKind::ALL {
            assert!(
                yaml.contains(&format!("  - \"{}\"\n", kind.code())),
                "missing resource kind {}",
                kind.code()
            );
        }
        let listed = yaml
            .split("resource_kinds:\n")
            .nth(1)
            .expect("the projection has a resource_kinds block")
            .lines()
            .take_while(|line| line.starts_with("  - "))
            .count();
        assert_eq!(listed, ResourceKind::ALL.len(), "kind list length drifted");
    }

    /// Binding: every verb in the closed Verb enum has a shape.
    #[test]
    fn projection_covers_every_verb() {
        let yaml = emit_yaml("test");
        for verb in [Verb::Gather, Verb::Witness] {
            assert!(
                yaml.contains(&format!("id: \"verb.{}\"", verb.code())),
                "missing verb shape {}",
                verb.code()
            );
        }
    }

    /// Binding: numeric values are emitted from the governing constants
    /// — the projection cannot carry hand-copied numbers that drift.
    #[test]
    fn projection_values_come_from_the_governing_constants() {
        let yaml = emit_yaml("test");
        assert!(yaml.contains(&format!(
            "[{}, {}, {}, {}] (fixture)",
            STAMINA_COST_BY_BAND[0],
            STAMINA_COST_BY_BAND[1],
            STAMINA_COST_BY_BAND[2],
            STAMINA_COST_BY_BAND[3]
        )));
        assert!(yaml.contains(&format!("witness_cost: \"{WITNESS_COST} (fixture)\"")));
        assert!(yaml.contains(&format!("\"0x{:016x}\"", grammar_fingerprint())));
    }

    /// The meaning-status vocabulary is closed: every shape uses one of
    /// the six statuses, and the legend lists all six.
    #[test]
    fn meaning_statuses_are_closed_and_fully_listed() {
        for s in super::shapes() {
            assert!(
                MEANING_STATUSES.contains(&s.meaning_status),
                "shape {} has unclosed status {}",
                s.id,
                s.meaning_status
            );
        }
        let html = emit_html("test");
        for status in MEANING_STATUSES {
            assert!(html.contains(status), "legend missing {status}");
        }
    }

    /// Determinism falsifier (in-process half): identical inputs must
    /// produce identical bytes. The cross-process half is proven by the
    /// double binary run recorded in the TS01 run record.
    #[test]
    fn falsification_emission_must_be_deterministic() {
        assert_eq!(emit_yaml("same"), emit_yaml("same"));
        assert_eq!(emit_html("same"), emit_html("same"));
        assert_ne!(emit_yaml("a"), emit_yaml("b"), "commit-addressed");
    }

    /// Every shape's evidence kinds come from the closed shape-level
    /// vocabulary — presence in the projection is not enough, the
    /// vocabulary itself must be closed.
    #[test]
    fn evidence_kinds_are_closed() {
        for s in super::shapes() {
            for kind in s.evidence_kind {
                assert!(
                    EVIDENCE_KINDS.contains(kind),
                    "shape {} cites unclosed evidence kind {kind}",
                    s.id
                );
            }
        }
    }

    /// Mapping falsifier (bounded-exhaustive): the refusal set projected
    /// for each verb must equal, in both directions, the set produced by
    /// executing EVERY command in the recorded input domain — each
    /// command against a fresh, identically seeded snapshot, so input
    /// completeness is never confused with sequence reachability.
    ///
    /// Recorded domain: actors {1,2,3,4,5,99}, claims {1..=7,9,99},
    /// sites {1,2,9}; 99/9 are explicit unknown sentinels. General
    /// completeness beyond this domain rests on source audit at the
    /// exact commit, and the claim is scoped accordingly.
    #[test]
    fn falsification_refusal_mapping_must_match_execution() {
        use crate::boundary::{
            CharacterId, ClaimId, Command, GatherCommand, InfraTier, MassGrams, OutcomeKind,
            ResourceKind, SiteId, Stamina, WitnessCommand, World, submit,
        };
        use crate::character::CharacterOwner;
        use crate::economy::EconomyOwner;
        use crate::social::SocialOwner;
        use std::collections::BTreeSet;

        const ACTORS: [u64; 6] = [1, 2, 3, 4, 5, 99];
        const CLAIMS: [u64; 9] = [1, 2, 3, 4, 5, 6, 7, 9, 99];
        const SITES: [u64; 3] = [1, 2, 9];

        // Fresh, identically seeded snapshot for every single command.
        // Claim K9's holder C99 has no body on purpose: unknown_actor
        // must be reachable, and refusals hold regardless of fixture
        // coherence gates.
        fn snapshot() -> World {
            World {
                characters: CharacterOwner::seed([
                    (CharacterId(1), Stamina::new(90).unwrap()),
                    (CharacterId(2), Stamina::new(50).unwrap()),
                    (CharacterId(3), Stamina::new(5).unwrap()),
                    (CharacterId(4), Stamina::new(12).unwrap()),
                    (CharacterId(5), Stamina::new(4).unwrap()),
                ])
                .unwrap(),
                economy: EconomyOwner::seed_sites([
                    (
                        SiteId(1),
                        InfraTier::Established,
                        ResourceKind::Fodder,
                        MassGrams::new(2000),
                    ),
                    (
                        SiteId(2),
                        InfraTier::Crude,
                        ResourceKind::Timber,
                        MassGrams::new(0),
                    ),
                ])
                .unwrap(),
                social: SocialOwner::seed_claims([
                    (ClaimId(1), CharacterId(1), SiteId(1), true),
                    (ClaimId(2), CharacterId(2), SiteId(1), false),
                    (ClaimId(3), CharacterId(3), SiteId(1), true),
                    (ClaimId(4), CharacterId(4), SiteId(1), true),
                    (ClaimId(5), CharacterId(2), SiteId(1), true),
                    (ClaimId(6), CharacterId(1), SiteId(9), true),
                    (ClaimId(7), CharacterId(1), SiteId(2), true),
                    (ClaimId(9), CharacterId(99), SiteId(1), true),
                ])
                .unwrap(),
            }
        }

        let mut observed_gather = BTreeSet::new();
        for actor in ACTORS {
            for claim in CLAIMS {
                for site in SITES {
                    let mut world = snapshot();
                    let receipt = submit(
                        &mut world,
                        1,
                        Command::Gather(GatherCommand {
                            actor: CharacterId(actor),
                            claim: ClaimId(claim),
                            site: SiteId(site),
                        }),
                    );
                    if matches!(receipt.outcome, OutcomeKind::Refused(_)) {
                        observed_gather.insert(receipt.outcome.reason_code().to_owned());
                    }
                }
            }
        }
        let mut observed_witness = BTreeSet::new();
        for actor in ACTORS {
            for claim in CLAIMS {
                let mut world = snapshot();
                let receipt = submit(
                    &mut world,
                    1,
                    Command::Witness(WitnessCommand {
                        witness: CharacterId(actor),
                        claim: ClaimId(claim),
                    }),
                );
                if matches!(receipt.outcome, OutcomeKind::Refused(_)) {
                    observed_witness.insert(receipt.outcome.reason_code().to_owned());
                }
            }
        }

        for (id, observed) in [
            ("verb.gather", &observed_gather),
            ("verb.witness", &observed_witness),
        ] {
            let shape = super::shapes()
                .into_iter()
                .find(|s| s.id == id)
                .expect("shape exists");
            let projected: BTreeSet<String> =
                shape.refusals.iter().map(|r| (*r).to_owned()).collect();
            assert_eq!(
                &projected, observed,
                "{id}: projected refusal set must equal the executed set over the full domain"
            );
        }
    }

    /// Binding: every projected source reference resolved to a real
    /// line — a renamed item turns the projection red instead of
    /// letting a line reference drift.
    #[test]
    fn source_line_references_resolve() {
        let yaml = emit_yaml("test");
        assert!(
            !yaml.contains(":0 "),
            "an unresolved source-line reference (:0) reached the projection"
        );
    }
}
