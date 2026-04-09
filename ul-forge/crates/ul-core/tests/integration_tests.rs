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

// ── Variable binding (bind operation) ──

#[test]
fn e2e_variable_slot() {
    let gir = parse("○_x").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    assert!(
        gir.nodes.iter().any(|n| n.node_type == ul_core::NodeType::VariableSlot),
        "Expected a VariableSlot node"
    );
    assert!(
        gir.nodes.iter().any(|n| n.variable_id.as_deref() == Some("x")),
        "Expected variable_id = 'x'"
    );
}

#[test]
fn e2e_bound_reference() {
    let gir = parse("●_x").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    assert!(
        gir.nodes.iter().any(|n| n.node_type == ul_core::NodeType::Point && n.variable_id.as_deref() == Some("x")),
        "Expected a Point node with variable_id = 'x'"
    );
}

#[test]
fn e2e_binding_pair() {
    // ○_y → ●_y — a variable slot connected to its bound reference
    let gir = parse("○_y → ●_y").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    assert!(
        gir.nodes.iter().any(|n| n.node_type == ul_core::NodeType::VariableSlot && n.variable_id.as_deref() == Some("y")),
        "Expected VariableSlot with variable_id = 'y'"
    );
    assert!(
        gir.nodes.iter().any(|n| n.node_type == ul_core::NodeType::Point && n.variable_id.as_deref() == Some("y")),
        "Expected Point with variable_id = 'y'"
    );
}

#[test]
fn roundtrip_variable_slot() {
    let gir = parse("○_x").unwrap();
    let out = deparse(&gir).unwrap();
    assert_eq!(out.trim(), "○_x");
}

#[test]
fn roundtrip_bound_ref() {
    let gir = parse("●_x").unwrap();
    let out = deparse(&gir).unwrap();
    assert_eq!(out.trim(), "●_x");
}

#[test]
fn e2e_variable_slot_ascii() {
    // ASCII alternative: /o_x
    let gir = parse("/o_x").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    assert!(
        gir.nodes.iter().any(|n| n.node_type == ul_core::NodeType::VariableSlot),
        "Expected a VariableSlot node from ASCII syntax"
    );
}

#[test]
fn e2e_bound_ref_ascii() {
    // ASCII alternative: *_x
    let gir = parse("*_x").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    assert!(
        gir.nodes.iter().any(|n| n.node_type == ul_core::NodeType::Point && n.variable_id.as_deref() == Some("x")),
        "Expected a Point with variable_id from ASCII syntax"
    );
}

// ── Assertion modifiers (modify_assertion operation) ──

#[test]
fn e2e_evidential_modifier() {
    let gir = parse("?{● → ●}").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    assert!(
        gir.nodes.iter().any(|n| n.assertion_modifier == Some(ul_core::AssertionModifierKind::Evidential)),
        "Expected an Evidential assertion modifier node"
    );
}

#[test]
fn e2e_emphatic_modifier() {
    let gir = parse("!{● → ●}").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    assert!(
        gir.nodes.iter().any(|n| n.assertion_modifier == Some(ul_core::AssertionModifierKind::Emphatic)),
        "Expected an Emphatic assertion modifier node"
    );
}

#[test]
fn e2e_hedged_modifier() {
    let gir = parse("~?{● → ●}").unwrap();
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
    assert!(
        gir.nodes.iter().any(|n| n.assertion_modifier == Some(ul_core::AssertionModifierKind::Hedged)),
        "Expected a Hedged assertion modifier node"
    );
}

#[test]
fn roundtrip_evidential() {
    let gir = parse("?{●}").unwrap();
    let out = deparse(&gir).unwrap();
    assert_eq!(out.trim(), "?{●}");
}

#[test]
fn roundtrip_emphatic() {
    let gir = parse("!{●}").unwrap();
    let out = deparse(&gir).unwrap();
    assert_eq!(out.trim(), "!{●}");
}

#[test]
fn roundtrip_hedged() {
    let gir = parse("~?{●}").unwrap();
    let out = deparse(&gir).unwrap();
    assert_eq!(out.trim(), "~?{●}");
}

#[test]
fn e2e_assertion_modifier_renders_svg() {
    let gir = parse("?{●}").unwrap();
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("ul-evidential"), "Expected ul-evidential class in SVG: {svg}");
}

// ── Modal operator tests ──

#[test]
fn e2e_parse_necessity() {
    let gir = parse("[]{● → ●}").unwrap();
    // Root should be an enclosure labeled □
    let root = gir.node(&gir.root).unwrap();
    assert_eq!(root.label.as_deref(), Some("□"));
    // Should have AccessibleFrom edge
    assert!(
        gir.edges.iter().any(|e| e.edge_type == ul_core::types::edge::EdgeType::AccessibleFrom),
        "Expected AccessibleFrom edge"
    );
}

#[test]
fn e2e_parse_possibility() {
    let gir = parse("<>{●}").unwrap();
    let root = gir.node(&gir.root).unwrap();
    assert_eq!(root.label.as_deref(), Some("◇"));
    assert!(
        gir.edges.iter().any(|e| e.edge_type == ul_core::types::edge::EdgeType::AccessibleFrom),
        "Expected AccessibleFrom edge"
    );
}

#[test]
fn e2e_parse_counterfactual() {
    let gir = parse("[]->{●}{●}").unwrap();
    let root = gir.node(&gir.root).unwrap();
    assert_eq!(root.label.as_deref(), Some("□→"));
    assert!(
        gir.edges.iter().any(|e| e.edge_type == ul_core::types::edge::EdgeType::AccessibleFrom),
        "Expected AccessibleFrom edge"
    );
}

#[test]
fn e2e_necessity_validates() {
    let gir = parse("[]{● → ●}").unwrap();
    let result = ul_core::validator::validate(&gir, false);
    assert!(
        result.errors.is_empty(),
        "Validation errors: {:?}",
        result.errors
    );
}

#[test]
fn e2e_possibility_validates() {
    let gir = parse("<>{●}").unwrap();
    let result = ul_core::validator::validate(&gir, false);
    assert!(
        result.errors.is_empty(),
        "Validation errors: {:?}",
        result.errors
    );
}

#[test]
fn e2e_counterfactual_validates() {
    let gir = parse("[]->{●}{●}").unwrap();
    let result = ul_core::validator::validate(&gir, false);
    assert!(
        result.errors.is_empty(),
        "Validation errors: {:?}",
        result.errors
    );
}

#[test]
fn e2e_necessity_renders_svg() {
    let gir = parse("[]{● → ●}").unwrap();
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<svg"));
}

#[test]
fn roundtrip_necessity() {
    let gir = parse("[]{●}").unwrap();
    let out = deparse(&gir).unwrap();
    assert!(out.contains("[]"), "Expected [] in output: {out}");
}

#[test]
fn roundtrip_possibility() {
    let gir = parse("<>{●}").unwrap();
    let out = deparse(&gir).unwrap();
    assert!(out.contains("<>"), "Expected <> in output: {out}");
}

#[test]
fn roundtrip_counterfactual() {
    let gir = parse("[]->{●}{●}").unwrap();
    let out = deparse(&gir).unwrap();
    assert!(out.contains("[]->"), "Expected []-> in output: {out}");
}

// ── Force annotation tests ──

#[test]
fn e2e_parse_query_force() {
    let gir = parse("query{●}").unwrap();
    let root = gir.node(&gir.root).unwrap();
    assert_eq!(root.force, Some(ul_core::PerformativeForce::Query));
}

#[test]
fn e2e_parse_direct_force() {
    let gir = parse("direct{● → ●}").unwrap();
    let root = gir.node(&gir.root).unwrap();
    assert_eq!(root.force, Some(ul_core::PerformativeForce::Direct));
}

#[test]
fn e2e_parse_all_six_forces() {
    for (token, expected) in [
        ("assert", ul_core::PerformativeForce::Assert),
        ("query", ul_core::PerformativeForce::Query),
        ("direct", ul_core::PerformativeForce::Direct),
        ("commit", ul_core::PerformativeForce::Commit),
        ("express", ul_core::PerformativeForce::Express),
        ("declare", ul_core::PerformativeForce::Declare),
    ] {
        let input = format!("{token}{{●}}");
        let gir = parse(&input).unwrap();
        let root = gir.node(&gir.root).unwrap();
        assert_eq!(root.force, Some(expected), "Force mismatch for {token}");
    }
}

#[test]
fn e2e_force_validates() {
    let gir = parse("query{●}").unwrap();
    let result = ul_core::validator::validate(&gir, false);
    assert!(
        result.errors.is_empty(),
        "Validation errors: {:?}",
        result.errors
    );
}

#[test]
fn e2e_force_renders_svg() {
    let gir = parse("commit{● → ●}").unwrap();
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<svg"));
}

#[test]
fn roundtrip_query_force() {
    let gir = parse("query{●}").unwrap();
    let out = deparse(&gir).unwrap();
    assert!(out.contains("query"), "Expected query in output: {out}");
}

#[test]
fn roundtrip_declare_force() {
    let gir = parse("declare{●}").unwrap();
    let out = deparse(&gir).unwrap();
    assert!(out.contains("declare"), "Expected declare in output: {out}");
}
