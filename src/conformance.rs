//! CON01: the live-law conformance pin.
//!
//! Three documents are live law or live context: `AGENTS.md` (the
//! agent-instruction entrypoint), `HANDOUT.md` (the derived context
//! loader), and the "Current position" section of
//! `docs/runtime-target-map.md` (the active work map). Each must name
//! the identities the code actually enforces; through RES01, V01 and
//! W01 all three silently kept the identities of an older master, and
//! the drift was found by review instead of by a test. This module is
//! that test.
//!
//! Direction of authority is one-way: the documents are held to the
//! code, and nothing in the crate derives behavior from a document.
//! The checks are presence-only, so prose may narrate historical
//! values freely, and historical reports are never read at all — a
//! dated record is evidence, not law.

use crate::boundary::{
    Receipt, command_encoding_fingerprint, fixture_identity, grammar_fingerprint,
    receipt_chain_digest, receipt_format_fingerprint, submit,
};
use crate::oracles::{ORACLE_COUNT, ORACLE_SUITE_VERSION};

const AGENTS: &str = include_str!("../AGENTS.md");
const HANDOUT: &str = include_str!("../HANDOUT.md");
const TARGET_MAP: &str = include_str!("../docs/runtime-target-map.md");

fn hex(value: u64) -> String {
    format!("0x{value:016x}")
}

/// The standard trial's identity septet, recomputed through `submit`
/// rather than hard-coded, so a licensed identity move turns these
/// tests red until the live documents move inside the same envelope.
fn current_identities() -> [String; 7] {
    let mut world = crate::fixture();
    let fixture_hash = world.hash();
    let cmds = crate::commands();
    let log: Vec<Receipt> = cmds
        .iter()
        .enumerate()
        .map(|(i, cmd)| submit(&mut world, i as u64 + 1, *cmd))
        .collect();
    [
        hex(grammar_fingerprint()),
        hex(command_encoding_fingerprint()),
        hex(receipt_format_fingerprint()),
        hex(fixture_identity(fixture_hash, &cmds)),
        hex(receipt_chain_digest(&log)),
        hex(world.hash()),
        format!("oracles={ORACLE_COUNT}v{ORACLE_SUITE_VERSION}"),
    ]
}

fn assert_contains(doc_name: &str, doc: &str, wanted: &[String]) {
    let missing: Vec<&str> = wanted
        .iter()
        .filter(|value| !doc.contains(value.as_str()))
        .map(String::as_str)
        .collect();
    assert!(
        missing.is_empty(),
        "{doc_name} is stale live law: it does not name {}",
        missing.join(", ")
    );
}

#[test]
fn agents_md_freezes_the_identities_the_code_enforces() {
    // AGENTS freezes the language and fixture inputs, not run outcomes.
    let ids = current_identities();
    assert_contains("AGENTS.md", AGENTS, &ids[..4]);
}

#[test]
fn handout_identity_block_is_the_current_envelope() {
    let ids = current_identities();
    assert_contains("HANDOUT.md", HANDOUT, &ids);
}

#[test]
fn target_map_current_position_quotes_the_current_envelope() {
    let start = TARGET_MAP
        .find("## Current position")
        .expect("docs/runtime-target-map.md lost its Current position section");
    let body = &TARGET_MAP[start + "## Current position".len()..];
    let end = body.find("\n## ").map_or(body.len(), |i| i);
    let ids = current_identities();
    assert_contains(
        "runtime-target-map.md \"Current position\"",
        &body[..end],
        &ids,
    );
}
