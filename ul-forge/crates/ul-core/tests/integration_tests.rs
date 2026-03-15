//! End-to-end integration tests: UL-Script → parse → validate → render pipeline.

use ul_core::parser::{deparse, parse};
use ul_core::renderer::{render, OutputFormat, RenderOptions};
use ul_core::validator::validate;

fn opts() -> RenderOptions {
    RenderOptions::default()
}

// ── End-to-end: parse → validate → render ──

#[test]
fn e2e_point_glyph() {
    let gir = parse("●").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("ul-point"));
}

#[test]
fn e2e_circle_enclosure() {
    let gir = parse("○").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<circle"));
    assert!(svg.contains("ul-mark"));
}

#[test]
fn e2e_triangle_enclosure() {
    let gir = parse("△").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<polygon"));
}

#[test]
fn e2e_square_enclosure() {
    let gir = parse("□").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<rect"));
}

#[test]
fn e2e_directed_line() {
    let gir = parse("→").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<line"));
    assert!(svg.contains("arrowhead"));
}

#[test]
fn e2e_angle() {
    let gir = parse("∠60").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<path"));
}

#[test]
fn e2e_curve() {
    let gir = parse("~").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<path"));
    assert!(svg.contains("Q")); // Bezier
}

#[test]
fn e2e_existence_glyph() {
    // ○{●} — Existence glyph
    let gir = parse("○{●}").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("ul-mark")); // circle
    assert!(svg.contains("ul-point")); // point
}

#[test]
fn e2e_connection_glyph() {
    // ● → ● — Connection glyph
    let gir = parse("● → ●").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<svg"));
}

#[test]
fn e2e_adjacency_glyph() {
    // ○ | ○ — Boundary glyph
    let gir = parse("○ | ○").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<svg"));
}

#[test]
fn e2e_intersection_glyph() {
    // ○ & ○ — Intersection glyph
    let gir = parse("○ & ○").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<svg"));
}

#[test]
fn e2e_containment_glyph() {
    // ○{○} — Containment glyph
    let gir = parse("○{○}").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    let svg = render(&gir, &opts()).unwrap();
    let circles = svg.matches("<circle").count();
    assert!(
        circles >= 2,
        "Expected 2+ circles for containment, got {circles}"
    );
}

// ── Parse → deparse roundtrip ──

#[test]
fn roundtrip_point() {
    let gir = parse("●").unwrap();
    let out = deparse(&gir).unwrap();
    assert_eq!(out.trim(), "●");
}

#[test]
fn roundtrip_circle() {
    let gir = parse("○").unwrap();
    let out = deparse(&gir).unwrap();
    assert_eq!(out.trim(), "○");
}

#[test]
fn roundtrip_existence() {
    let gir = parse("○{●}").unwrap();
    let out = deparse(&gir).unwrap();
    assert_eq!(out.trim(), "○{●}");
}

// ── GIR JSON roundtrip through pipeline ──

#[test]
fn gir_json_roundtrip_then_render() {
    let gir = parse("○{●}").unwrap();
    let json = gir.to_json_pretty().unwrap();
    let gir2 = ul_core::Gir::from_json(&json).unwrap();
    let svg1 = render(&gir, &opts()).unwrap();
    let svg2 = render(&gir2, &opts()).unwrap();
    // Both should produce valid SVG
    assert!(svg1.contains("<svg"));
    assert!(svg2.contains("<svg"));
}

// ── TikZ end-to-end ──

#[test]
fn e2e_tikz_output() {
    let gir = parse("○{●}").unwrap();
    let tikz = render(
        &gir,
        &RenderOptions {
            format: OutputFormat::TikZ,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(tikz.contains("\\begin{tikzpicture}"));
    assert!(tikz.contains("\\end{tikzpicture}"));
}

// ── Validation catches invalid parse output (manually constructed) ──

#[test]
fn validate_rejects_modified_gir() {
    let mut gir = parse("●").unwrap();
    // Corrupt the GIR: set root to nonexistent node
    gir.root = "nonexistent".to_string();
    let result = validate(&gir, false);
    assert!(!result.valid);
}
