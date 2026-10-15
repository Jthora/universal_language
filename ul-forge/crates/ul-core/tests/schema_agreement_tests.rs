//! The published schema and the reference implementation must agree.
//!
//! WHY THIS FILE EXISTS
//!   A cold reader doing a usability pass found that `schemas/gir.schema.json` required
//!   `["root","nodes","edges"]` and never mentioned `ul_gir`, which the Rust deserializer
//!   requires. So a document valid against the PUBLISHED schema was rejected by the
//!   implementation that publishes it. For anything claiming to be an interchange format that is
//!   disqualifying on its own: there was no stable contract to code against.
//!
//!   Worse, the version was PATH-DEPENDENT. `Gir::new` wrote "1.0", the parser wrote "0.1.0" and
//!   ul-game's fixtures wrote "0.2", so the version a consumer received depended on which
//!   operation produced the document — `parse` and `invert` gave one answer, `predicate` and
//!   `conjoin` another. A consumer cannot branch on that.
//!
//!   These tests check agreement in BOTH directions, so neither side can drift silently again.

use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use ul_core::parser;
use ul_core::types::gir::GIR_VERSION;

fn schema() -> Value {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../schemas/gir.schema.json");
    let text = fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
    serde_json::from_str(&text).expect("schema is valid JSON")
}

fn fixtures() -> Vec<String> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/canonical");
    let mut out: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .filter_map(|e| {
            let p = e.ok()?.path();
            if p.extension()? == "ul" {
                fs::read_to_string(&p).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    out.sort();
    assert!(!out.is_empty(), "no canonical fixtures found");
    out
}

#[test]
fn the_schema_declares_the_version_the_code_emits() {
    let s = schema();
    let declared = s["properties"]["ul_gir"]["const"]
        .as_str()
        .expect("schema must declare a const for ul_gir");
    assert_eq!(
        declared, GIR_VERSION,
        "schemas/gir.schema.json declares ul_gir const `{declared}` but the code emits `{GIR_VERSION}`"
    );
}

#[test]
fn every_key_the_schema_requires_is_emitted() {
    let s = schema();
    let required: Vec<&str> = s["required"]
        .as_array()
        .expect("schema has `required`")
        .iter()
        .map(|v| v.as_str().expect("required entries are strings"))
        .collect();
    assert!(required.contains(&"ul_gir"), "ul_gir must be required (F-038)");

    for src in fixtures() {
        let gir = parser::parse(&src).unwrap_or_else(|e| panic!("`{src}` must parse: {e}"));
        let json: Value = serde_json::to_value(&gir).expect("serializes");
        let obj = json.as_object().expect("GIR serializes to an object");
        for key in &required {
            assert!(
                obj.contains_key(*key),
                "`{src}` emitted GIR without required key `{key}`"
            );
        }
    }
}

#[test]
fn every_key_the_code_emits_is_declared_in_the_schema() {
    // The other direction: an emitter that starts writing a new top-level field without
    // declaring it leaves consumers unable to validate. `additionalProperties` is unset in the
    // schema, so JSON Schema itself would NOT catch this — only this test does.
    let s = schema();
    let declared = s["properties"].as_object().expect("schema has `properties`");
    for src in fixtures() {
        let gir = parser::parse(&src).unwrap_or_else(|e| panic!("`{src}` must parse: {e}"));
        let json: Value = serde_json::to_value(&gir).expect("serializes");
        for key in json.as_object().expect("object").keys() {
            assert!(
                declared.contains_key(key),
                "emitter writes top-level `{key}` (from `{src}`) which schemas/gir.schema.json \
                 does not declare"
            );
        }
    }
}

#[test]
fn the_version_is_uniform_across_every_emitter() {
    // The path-dependence bug: parse and the composer disagreed. Check a parsed document and
    // every composer product that a fresh frame is built for.
    use ul_core::composer;

    let parsed = parser::parse("○{● → △}").expect("parses");
    assert_eq!(parsed.ul_gir, GIR_VERSION, "parser");

    let subject = parser::parse("*").expect("parses");
    let relation = parser::parse("->").expect("parses");
    let object = parser::parse("/3").expect("parses");
    let pred = composer::predicate(&subject, &relation, &object).expect("predicate");
    assert_eq!(pred.ul_gir, GIR_VERSION, "composer::predicate builds a fresh frame");

    let a = parser::parse("assert{*}").expect("parses");
    let b = parser::parse("assert{/3}").expect("parses");
    for (name, g) in [
        ("negate", composer::negate(&a).expect("negate")),
        ("conjoin", composer::conjoin(&a, &b).expect("conjoin")),
        ("disjoin", composer::disjoin(&a, &b).expect("disjoin")),
        ("invert", composer::invert(&relation).expect("invert")),
    ] {
        assert_eq!(g.ul_gir, GIR_VERSION, "composer::{name}");
    }
}
