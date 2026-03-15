//! Snapshot tests using insta.
//!
//! These capture the exact GIR and SVG output for the 12 Tier-1 canonical glyphs
//! so regressions are caught as diffs.

use insta::assert_json_snapshot;
use insta::assert_snapshot;
use ul_core::parser;
use ul_core::renderer::{self, RenderOptions};

fn parse_snapshot(name: &str, input: &str) {
    let gir = parser::parse(input).unwrap_or_else(|e| panic!("parse failed for {name}: {e}"));
    assert_json_snapshot!(format!("gir_{name}"), gir);
}

fn svg_snapshot(name: &str, input: &str) {
    let gir = parser::parse(input).unwrap_or_else(|e| panic!("parse failed for {name}: {e}"));
    let opts = RenderOptions {
        embed_gir: false,
        ..RenderOptions::default()
    };
    let svg = renderer::render(&gir, &opts).unwrap_or_else(|e| panic!("render failed for {name}: {e}"));
    assert_snapshot!(format!("svg_{name}"), svg);
}

// ── GIR snapshots for 12 Tier-1 glyphs ──

#[test]
fn gir_existence()    { parse_snapshot("existence",    "○{●}"); }
#[test]
fn gir_relation()     { parse_snapshot("relation",     "→"); }
#[test]
fn gir_quality()      { parse_snapshot("quality",      "∠60"); }
#[test]
fn gir_process()      { parse_snapshot("process",      "~"); }
#[test]
fn gir_concept()      { parse_snapshot("concept",      "○"); }
#[test]
fn gir_structure()    { parse_snapshot("structure",    "△"); }
#[test]
fn gir_foundation()   { parse_snapshot("foundation",   "□"); }
#[test]
fn gir_point()        { parse_snapshot("point",        "●"); }
#[test]
fn gir_connection()   { parse_snapshot("connection",   "● → ●"); }
#[test]
fn gir_boundary()     { parse_snapshot("boundary",     "○ | ○"); }
#[test]
fn gir_intersection() { parse_snapshot("intersection", "○ & ○"); }
#[test]
fn gir_containment()  { parse_snapshot("containment",  "○{○}"); }

// ── SVG snapshots for 12 Tier-1 glyphs ──

#[test]
fn svg_existence()    { svg_snapshot("existence",    "○{●}"); }
#[test]
fn svg_relation()     { svg_snapshot("relation",     "→"); }
#[test]
fn svg_quality()      { svg_snapshot("quality",      "∠60"); }
#[test]
fn svg_process()      { svg_snapshot("process",      "~"); }
#[test]
fn svg_concept()      { svg_snapshot("concept",      "○"); }
#[test]
fn svg_structure()    { svg_snapshot("structure",    "△"); }
#[test]
fn svg_foundation()   { svg_snapshot("foundation",   "□"); }
#[test]
fn svg_point()        { svg_snapshot("point",        "●"); }
#[test]
fn svg_connection()   { svg_snapshot("connection",   "● → ●"); }
#[test]
fn svg_boundary()     { svg_snapshot("boundary",     "○ | ○"); }
#[test]
fn svg_intersection() { svg_snapshot("intersection", "○ & ○"); }
#[test]
fn svg_containment()  { svg_snapshot("containment",  "○{○}"); }
