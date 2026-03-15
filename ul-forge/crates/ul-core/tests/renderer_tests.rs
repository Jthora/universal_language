//! Tests for the renderer: template matching, SVG output, TikZ output, hierarchical layout.

use ul_core::renderer::{render, OutputFormat, RenderOptions};
use ul_core::types::edge::Edge;
use ul_core::types::gir::Gir;
use ul_core::types::node::{EnclosureShape, Node};

fn opts() -> RenderOptions {
    RenderOptions::default()
}

fn tikz_opts() -> RenderOptions {
    RenderOptions {
        format: OutputFormat::TikZ,
        ..Default::default()
    }
}

fn minimal_gir(nodes: Vec<Node>, edges: Vec<Edge>) -> Gir {
    Gir {
        ul_gir: "0.1.0".to_string(),
        root: nodes[0].id.clone(),
        nodes,
        edges,
        metadata: None,
    }
}

// ── Single-node template tests (SVG output) ──

#[test]
fn render_single_point() {
    let gir = minimal_gir(vec![Node::point("p1")], vec![]);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
    assert!(svg.contains("ul-point"));
    assert!(svg.contains(r#"data-ul-node="p1""#));
}

#[test]
fn render_single_circle_enclosure() {
    let gir = minimal_gir(vec![Node::enclosure("e1", EnclosureShape::Circle)], vec![]);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<circle"));
    assert!(svg.contains("ul-mark"));
    assert!(svg.contains(r#"data-ul-node="e1""#));
}

#[test]
fn render_single_triangle_enclosure() {
    let gir = minimal_gir(
        vec![Node::enclosure("e1", EnclosureShape::Triangle)],
        vec![],
    );
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<polygon"));
    assert!(svg.contains("ul-mark"));
}

#[test]
fn render_single_square_enclosure() {
    let gir = minimal_gir(vec![Node::enclosure("e1", EnclosureShape::Square)], vec![]);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<rect"));
    assert!(svg.contains("ul-mark"));
}

#[test]
fn render_single_line() {
    let gir = minimal_gir(vec![Node::line("l1", true)], vec![]);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<line"));
    assert!(svg.contains("arrowhead"));
    assert!(svg.contains(r#"data-ul-node="l1""#));
}

#[test]
fn render_single_angle() {
    let gir = minimal_gir(vec![Node::angle("a1", 90.0)], vec![]);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<path"));
    assert!(svg.contains("90°"));
}

#[test]
fn render_single_curve() {
    let gir = minimal_gir(vec![Node::curve("c1", 0.5)], vec![]);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<path"));
    assert!(svg.contains("Q")); // Quadratic Bezier
}

// ── Multi-node template tests ──

#[test]
fn render_existence_template() {
    // Template #1: ○{●} — circle containing point
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::point("p1"),
        ],
        vec![Edge::contains("root", "p1")],
    );
    let svg = render(&gir, &opts()).unwrap();
    // Should have both circle (enclosure) and point
    assert!(svg.contains("ul-mark"));
    assert!(svg.contains("ul-point"));
    assert!(svg.contains(r#"data-ul-node="root""#));
    assert!(svg.contains(r#"data-ul-node="p1""#));
}

#[test]
fn render_connection_template() {
    // Template #9: ● → ● — two points connected by directed line
    let gir = Gir {
        ul_gir: "0.1.0".to_string(),
        root: "_root".to_string(),
        nodes: vec![
            Node::enclosure("_root", EnclosureShape::Circle).with_label("_implicit_root"),
            Node::point("p1"),
            Node::point("p2"),
            Node::line("l1", true),
        ],
        edges: vec![
            Edge::contains("_root", "p1"),
            Edge::contains("_root", "p2"),
            Edge::contains("_root", "l1"),
            Edge::connects("l1", "p1"),
            Edge::connects("l1", "p2"),
        ],
        metadata: None,
    };
    let svg = render(&gir, &opts()).unwrap();
    // Should have two points
    assert!(svg.contains(r#"data-ul-node="p1""#));
    assert!(svg.contains(r#"data-ul-node="p2""#));
}

#[test]
fn render_containment_template() {
    // Template #12: ○{○} — circle in circle
    let gir = minimal_gir(
        vec![
            Node::enclosure("outer", EnclosureShape::Circle),
            Node::enclosure("inner", EnclosureShape::Circle),
        ],
        vec![Edge::contains("outer", "inner")],
    );
    let svg = render(&gir, &opts()).unwrap();
    // Should have two circles
    let circle_count = svg.matches("<circle").count();
    assert!(circle_count >= 2, "Expected 2+ circles, got {circle_count}");
}

// ── SVG structure tests ──

#[test]
fn svg_has_xml_header() {
    let gir = minimal_gir(vec![Node::point("p1")], vec![]);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.starts_with(r#"<?xml version="1.0""#));
}

#[test]
fn svg_has_viewbox() {
    let gir = minimal_gir(vec![Node::point("p1")], vec![]);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains(r#"viewBox="0 0 400 400""#));
}

#[test]
fn svg_embeds_gir_metadata_by_default() {
    let gir = minimal_gir(vec![Node::point("p1")], vec![]);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<metadata>"));
    assert!(svg.contains("CDATA"));
}

#[test]
fn svg_no_metadata_when_disabled() {
    let gir = minimal_gir(vec![Node::point("p1")], vec![]);
    let options = RenderOptions {
        embed_gir: false,
        ..Default::default()
    };
    let svg = render(&gir, &options).unwrap();
    assert!(!svg.contains("<metadata>"));
}

#[test]
fn svg_has_defs_with_styles_and_arrowhead() {
    let gir = minimal_gir(vec![Node::point("p1")], vec![]);
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<defs>"));
    assert!(svg.contains("<style>"));
    assert!(svg.contains("ul-mark"));
    assert!(svg.contains("ul-point"));
    assert!(svg.contains(r#"id="arrowhead""#));
}

#[test]
fn svg_has_glyph_group_with_accessibility() {
    let gir = minimal_gir(
        vec![Node::enclosure("root", EnclosureShape::Circle).with_label("test glyph")],
        vec![],
    );
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains(r#"role="img""#));
    assert!(svg.contains(r#"aria-label="test glyph""#));
    assert!(svg.contains(r#"data-ul-root="root""#));
    assert!(svg.contains("<title>test glyph</title>"));
}

#[test]
fn svg_custom_dimensions() {
    let gir = minimal_gir(vec![Node::point("p1")], vec![]);
    let options = RenderOptions {
        width: 800.0,
        height: 600.0,
        ..Default::default()
    };
    let svg = render(&gir, &options).unwrap();
    assert!(svg.contains(r#"width="800""#));
    assert!(svg.contains(r#"height="600""#));
    assert!(svg.contains(r#"viewBox="0 0 800 600""#));
}

// ── TikZ output tests ──

#[test]
fn tikz_output_format() {
    let gir = minimal_gir(vec![Node::point("p1")], vec![]);
    let tikz = render(&gir, &tikz_opts()).unwrap();
    assert!(tikz.contains("\\begin{tikzpicture}"));
    assert!(tikz.contains("\\end{tikzpicture}"));
}

#[test]
fn tikz_point_renders_as_fill_circle() {
    let gir = minimal_gir(vec![Node::point("p1")], vec![]);
    let tikz = render(&gir, &tikz_opts()).unwrap();
    assert!(tikz.contains("\\fill"));
    assert!(tikz.contains("circle"));
}

#[test]
fn tikz_enclosure_renders_as_draw_circle() {
    let gir = minimal_gir(vec![Node::enclosure("e1", EnclosureShape::Circle)], vec![]);
    let tikz = render(&gir, &tikz_opts()).unwrap();
    assert!(tikz.contains("\\draw"));
    assert!(tikz.contains("circle"));
}

// ── Hierarchical layout tests (compositions that don't match templates) ──

#[test]
fn hierarchical_layout_for_complex_gir() {
    // 3+ children — won't match any template
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::point("p1"),
            Node::point("p2"),
            Node::point("p3"),
        ],
        vec![
            Edge::contains("root", "p1"),
            Edge::contains("root", "p2"),
            Edge::contains("root", "p3"),
        ],
    );
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains(r#"data-ul-node="root""#));
    assert!(svg.contains(r#"data-ul-node="p1""#));
    assert!(svg.contains(r#"data-ul-node="p2""#));
    assert!(svg.contains(r#"data-ul-node="p3""#));
}

#[test]
fn hierarchical_layout_nested_enclosures() {
    // Nested: root > e1 > p1 — falls through to hierarchical layout
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::enclosure("e1", EnclosureShape::Triangle),
            Node::point("p1"),
        ],
        vec![Edge::contains("root", "e1"), Edge::contains("e1", "p1")],
    );
    let svg = render(&gir, &opts()).unwrap();
    assert!(svg.contains("<circle")); // root circle
    assert!(svg.contains("<polygon")); // e1 triangle
    assert!(svg.contains("ul-point")); // p1 point
}

#[test]
fn render_with_cross_edges() {
    // Connects and references edges should produce connections in SVG
    let gir = Gir {
        ul_gir: "0.1.0".to_string(),
        root: "root".to_string(),
        nodes: vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::point("p1"),
            Node::point("p2"),
            Node::point("p3"),
        ],
        edges: vec![
            Edge::contains("root", "p1"),
            Edge::contains("root", "p2"),
            Edge::contains("root", "p3"),
            Edge::connects("p1", "p2"),
            Edge::references("p2", "p3"),
        ],
        metadata: None,
    };
    let svg = render(&gir, &opts()).unwrap();
    // Should have connection lines
    assert!(svg.contains("ul-edge"));
}
