//! Parser integration tests — 25+ test cases covering:
//! - Single primitives (Unicode + ASCII)
//! - Content (containment)
//! - Operators (connection, adjacency, intersection)
//! - Complex compositions
//! - Comments
//! - Error cases
//! - Roundtrip (parse → deparse → parse)

use ul_core::parser;
use ul_core::types::edge::EdgeType;
use ul_core::types::node::NodeType;

// ═══════════════════════════════════════════════════════
// AST-level tests — verify grammar → AST conversion
// ═══════════════════════════════════════════════════════

#[test]
fn parse_single_point_unicode() {
    let result = parser::parse("●").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Point);
    assert_eq!(result.root, "n1");
}

#[test]
fn parse_single_point_ascii() {
    let result = parser::parse("*").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Point);
}

#[test]
fn parse_circle_unicode() {
    let result = parser::parse("○").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Enclosure);
}

#[test]
fn parse_circle_ascii() {
    let result = parser::parse("/0").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Enclosure);
}

#[test]
fn parse_triangle_unicode() {
    let result = parser::parse("△").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Enclosure);
}

#[test]
fn parse_triangle_ascii() {
    let result = parser::parse("/3").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Enclosure);
}

#[test]
fn parse_square_unicode() {
    let result = parser::parse("□").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Enclosure);
}

#[test]
fn parse_square_ascii() {
    let result = parser::parse("/4").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Enclosure);
}

#[test]
fn parse_right_arrow() {
    let result = parser::parse("→").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Line);
    assert_eq!(result.nodes[0].directed, Some(true));
}

#[test]
fn parse_right_arrow_ascii() {
    let result = parser::parse("->").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Line);
}

#[test]
fn parse_left_arrow() {
    let result = parser::parse("←").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Line);
}

#[test]
fn parse_bi_arrow() {
    let result = parser::parse("↔").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Line);
}

#[test]
fn parse_bi_arrow_ascii() {
    let result = parser::parse("<->").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Line);
}

#[test]
fn parse_curve() {
    let result = parser::parse("~").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Curve);
}

#[test]
fn parse_angle() {
    let result = parser::parse("∠60").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Angle);
    assert_eq!(result.nodes[0].measure, Some(60.0));
}

#[test]
fn parse_angle_ascii() {
    let result = parser::parse("@90").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Angle);
    assert_eq!(result.nodes[0].measure, Some(90.0));
}

// ═══════════════════════════════════════════════════════
// Content (containment) tests
// ═══════════════════════════════════════════════════════

#[test]
fn parse_circle_contains_point() {
    let result = parser::parse("○{●}").unwrap();
    // ○ (enclosure) containing ● (point)
    assert_eq!(result.nodes.len(), 2);

    let enclosure = result
        .nodes
        .iter()
        .find(|n| n.node_type == NodeType::Enclosure)
        .unwrap();
    let point = result
        .nodes
        .iter()
        .find(|n| n.node_type == NodeType::Point)
        .unwrap();

    // Should have a contains edge from enclosure to point
    let contains = result
        .edges
        .iter()
        .find(|e| e.edge_type == EdgeType::Contains)
        .unwrap();
    assert_eq!(contains.source, enclosure.id);
    assert_eq!(contains.target, point.id);

    // Root should be the enclosure
    assert_eq!(result.root, enclosure.id);
}

#[test]
fn parse_nested_containment() {
    // △{○{●}}
    let result = parser::parse("△{○{●}}").unwrap();
    assert_eq!(result.nodes.len(), 3);

    let triangle = result
        .nodes
        .iter()
        .find(|n| {
            n.node_type == NodeType::Enclosure && n.shape == Some(ul_core::EnclosureShape::Triangle)
        })
        .unwrap();
    let circle = result
        .nodes
        .iter()
        .find(|n| {
            n.node_type == NodeType::Enclosure && n.shape == Some(ul_core::EnclosureShape::Circle)
        })
        .unwrap();
    let point = result
        .nodes
        .iter()
        .find(|n| n.node_type == NodeType::Point)
        .unwrap();

    // △ contains ○
    assert!(result.edges.iter().any(|e| e.source == triangle.id
        && e.target == circle.id
        && e.edge_type == EdgeType::Contains));
    // ○ contains ●
    assert!(result.edges.iter().any(|e| e.source == circle.id
        && e.target == point.id
        && e.edge_type == EdgeType::Contains));

    assert_eq!(result.root, triangle.id);
}

#[test]
fn parse_multiple_children() {
    // □{● | ●}
    let result = parser::parse("□{● | ●}").unwrap();

    let square = result
        .nodes
        .iter()
        .find(|n| n.node_type == NodeType::Enclosure)
        .unwrap();
    let points: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Point)
        .collect();
    assert_eq!(points.len(), 2);

    // Square contains both points
    let contains: Vec<_> = result
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Contains)
        .collect();
    assert!(contains.len() >= 2);
    assert_eq!(result.root, square.id);
}

// ═══════════════════════════════════════════════════════
// Operator tests
// ═══════════════════════════════════════════════════════

#[test]
fn parse_connection() {
    // ● → ●
    let result = parser::parse("● → ●").unwrap();

    let points: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Point)
        .collect();
    assert_eq!(points.len(), 2);

    // Should have a line node and connects edges
    let lines: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Line)
        .collect();
    assert_eq!(lines.len(), 1);

    let connects: Vec<_> = result
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Connects)
        .collect();
    assert_eq!(connects.len(), 2);
}

#[test]
fn parse_connection_ascii() {
    let result = parser::parse("* -> *").unwrap();
    let points: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Point)
        .collect();
    assert_eq!(points.len(), 2);
    let lines: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Line)
        .collect();
    assert_eq!(lines.len(), 1);
}

#[test]
fn parse_adjacency() {
    // ○ | △
    let result = parser::parse("○ | △").unwrap();

    let adjacent: Vec<_> = result
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Adjacent)
        .collect();
    assert_eq!(adjacent.len(), 1);
}

#[test]
fn parse_intersection() {
    // ○ & △
    let result = parser::parse("○ & △").unwrap();

    let intersects: Vec<_> = result
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Intersects)
        .collect();
    assert_eq!(intersects.len(), 1);
}

#[test]
fn parse_connection_with_angle() {
    // ● →@60 ●  (using ASCII syntax)
    let result = parser::parse("● →@60 ●").unwrap();

    // Should have points, a line, and an angle modifier
    let angles: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Angle)
        .collect();
    assert_eq!(angles.len(), 1);
    assert_eq!(angles[0].measure, Some(60.0));

    let mod_edges: Vec<_> = result
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::ModifiedBy)
        .collect();
    assert_eq!(mod_edges.len(), 1);
}

// ═══════════════════════════════════════════════════════
// Complex compositions
// ═══════════════════════════════════════════════════════

#[test]
fn parse_complex_connection_with_content() {
    // ○{●} → △{●}
    let result = parser::parse("○{●} → △{●}").unwrap();

    // 2 explicit enclosures + 1 implicit root = 3 enclosures, 2 points, 1 line
    let enclosures: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Enclosure)
        .collect();
    assert_eq!(enclosures.len(), 3); // ○, △, implicit root

    let points: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Point)
        .collect();
    assert_eq!(points.len(), 2);

    let lines: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Line)
        .collect();
    assert_eq!(lines.len(), 1);

    // Contains edges: ○→●, △→●, root→○, root→△, root→line
    let contains: Vec<_> = result
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Contains)
        .collect();
    assert!(contains.len() >= 4);
}

#[test]
fn parse_grouped_expression() {
    // ● → (○ | △)
    let result = parser::parse("● → (○ | △)").unwrap();

    let points: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Point)
        .collect();
    assert_eq!(points.len(), 1);

    // ○, △, + implicit root from group, + implicit root from top-level
    let enclosures: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Enclosure)
        .collect();
    assert!(enclosures.len() >= 2); // at least the explicit ○ and △
}

#[test]
fn parse_ascii_complex() {
    // /0{*} -> /3{*}
    let result = parser::parse("/0{*} -> /3{*}").unwrap();

    // 2 explicit enclosures + 1 implicit root = 3
    let enclosures: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Enclosure)
        .collect();
    assert_eq!(enclosures.len(), 3);

    let points: Vec<_> = result
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Point)
        .collect();
    assert_eq!(points.len(), 2);
}

// ═══════════════════════════════════════════════════════
// Comments
// ═══════════════════════════════════════════════════════

#[test]
fn parse_comment() {
    let result = parser::parse("# This is a comment\n●").unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].node_type, NodeType::Point);
}

// ═══════════════════════════════════════════════════════
// Error cases
// ═══════════════════════════════════════════════════════

#[test]
fn parse_empty_gives_empty_gir() {
    // Empty input (no glyphs)
    let result = parser::parse("");
    // Should succeed with a minimal GIR (single point root)
    assert!(result.is_ok());
}

#[test]
fn parse_unclosed_brace_is_error() {
    let result = parser::parse("○{●");
    assert!(result.is_err());
}

#[test]
fn parse_unknown_char_is_error() {
    let result = parser::parse("X");
    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════
// Roundtrip tests — parse → deparse → parse
// ═══════════════════════════════════════════════════════

#[test]
fn roundtrip_single_point() {
    let gir1 = parser::parse("●").unwrap();
    let text = parser::deparse(&gir1).unwrap();
    let gir2 = parser::parse(&text).unwrap();
    assert_eq!(gir1.nodes.len(), gir2.nodes.len());
    assert_eq!(gir1.edges.len(), gir2.edges.len());
}

#[test]
fn roundtrip_circle_contains_point() {
    let gir1 = parser::parse("○{●}").unwrap();
    let text = parser::deparse(&gir1).unwrap();
    let gir2 = parser::parse(&text).unwrap();
    assert_eq!(gir1.nodes.len(), gir2.nodes.len());
    assert_eq!(gir1.edges.len(), gir2.edges.len());
}
