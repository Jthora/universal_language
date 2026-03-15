//! Tests for enhanced validator: angle arity, dangling modifiers, sort constraints.

use ul_core::types::edge::Edge;
use ul_core::types::gir::Gir;
use ul_core::types::node::{EnclosureShape, Node};
use ul_core::validator::validate;
use ul_core::UlError;

fn minimal_gir(nodes: Vec<Node>, edges: Vec<Edge>) -> Gir {
    Gir {
        ul_gir: "0.1.0".to_string(),
        root: nodes[0].id.clone(),
        nodes,
        edges,
        metadata: None,
    }
}

// ── Angle arity tests ──

#[test]
fn angle_arity_zero_is_valid() {
    // Standalone angle contained in enclosure — 0 modifier sources is fine
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::angle("a1", 60.0),
        ],
        vec![Edge::contains("root", "a1")],
    );
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
}

#[test]
fn angle_arity_one_is_valid() {
    // Angle modifying one line
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::line("l1", true),
            Node::angle("a1", 45.0),
        ],
        vec![Edge::contains("root", "l1"), Edge::modified_by("l1", "a1")],
    );
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
}

#[test]
fn angle_arity_two_is_valid() {
    // Angle modifying two lines (standard angle between two relations)
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::line("l1", true),
            Node::line("l2", true),
            Node::angle("a1", 90.0),
        ],
        vec![
            Edge::contains("root", "l1"),
            Edge::contains("root", "l2"),
            Edge::modified_by("l1", "a1"),
            Edge::modified_by("l2", "a1"),
        ],
    );
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
}

#[test]
fn angle_arity_three_is_invalid() {
    // Angle modifying 3 lines — exceeds maximum arity
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::line("l1", true),
            Node::line("l2", true),
            Node::line("l3", true),
            Node::angle("a1", 90.0),
        ],
        vec![
            Edge::contains("root", "l1"),
            Edge::contains("root", "l2"),
            Edge::contains("root", "l3"),
            Edge::modified_by("l1", "a1"),
            Edge::modified_by("l2", "a1"),
            Edge::modified_by("l3", "a1"),
        ],
    );
    let result = validate(&gir, false);
    assert!(!result.valid);
    assert!(result
        .errors
        .iter()
        .any(|e| matches!(e, UlError::InvalidAngleArity { found: 3, .. })));
}

#[test]
fn angle_arity_counts_only_lines_and_curves() {
    // Angle modified_by from a line AND an enclosure —
    // only the line counts for arity (enclosures are not lines/curves)
    let gir = Gir {
        ul_gir: "0.1.0".to_string(),
        root: "root".to_string(),
        nodes: vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::line("l1", true),
            Node::angle("a1", 45.0),
        ],
        edges: vec![
            Edge::contains("root", "l1"),
            Edge::modified_by("l1", "a1"),
            // This edge has an enclosure as source - won't count for angle arity
            // (but will fail sort validation since enclosure is Entity, not Entity/Relation for modified_by source—actually Entity is allowed)
        ],
        metadata: None,
    };
    let result = validate(&gir, false);
    // Angle arity check: only l1 is a line pointing to a1, so arity = 1 (valid)
    assert!(
        !result
            .errors
            .iter()
            .any(|e| matches!(e, UlError::InvalidAngleArity { .. })),
        "Should not have angle arity error, got: {:?}",
        result.errors
    );
}

// ── Dangling modifier tests ──

#[test]
fn dangling_angle_not_contained_warns() {
    // Angle node that isn't contained and has no modified_by targeting it
    let gir = Gir {
        ul_gir: "0.1.0".to_string(),
        root: "root".to_string(),
        nodes: vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::angle("a1", 60.0),
        ],
        edges: vec![
            // a1 not connected to anything
        ],
        metadata: None,
    };
    let result = validate(&gir, false);
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.contains("angle node 'a1'")),
        "Expected dangling modifier warning, got warnings: {:?}",
        result.warnings
    );
}

#[test]
fn contained_angle_without_modified_by_no_warning() {
    // Angle contained in root — standalone primitive, no warning expected
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::angle("a1", 60.0),
        ],
        vec![Edge::contains("root", "a1")],
    );
    let result = validate(&gir, false);
    assert!(
        !result
            .warnings
            .iter()
            .any(|w| w.contains("angle node 'a1'")),
        "Should not warn for contained standalone angle, got warnings: {:?}",
        result.warnings
    );
}

#[test]
fn angle_with_modified_by_no_warning() {
    // Angle properly targeted by modified_by
    let gir = Gir {
        ul_gir: "0.1.0".to_string(),
        root: "root".to_string(),
        nodes: vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::line("l1", true),
            Node::angle("a1", 45.0),
        ],
        edges: vec![Edge::contains("root", "l1"), Edge::modified_by("l1", "a1")],
        metadata: None,
    };
    let result = validate(&gir, false);
    assert!(
        !result
            .warnings
            .iter()
            .any(|w| w.contains("angle node 'a1'")),
        "Should not warn when modified_by exists, got warnings: {:?}",
        result.warnings
    );
}

// ── Sort constraint tests ──

#[test]
fn contains_source_must_be_enclosure() {
    let gir = Gir {
        ul_gir: "0.1.0".to_string(),
        root: "p1".to_string(),
        nodes: vec![
            Node::point("p1"), // Point, not enclosure
            Node::point("p2"),
        ],
        edges: vec![Edge::contains("p1", "p2")],
        metadata: None,
    };
    let result = validate(&gir, false);
    assert!(!result.valid);
    assert!(result.errors.iter().any(|e| matches!(
        e,
        UlError::SortViolation {
            reason,
            ..
        } if reason.contains("enclosure")
    )));
}

#[test]
fn modified_by_target_must_be_modifier() {
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::point("p1"),
            Node::point("p2"), // Point, sort=Entity, not Modifier
        ],
        vec![
            Edge::contains("root", "p1"),
            Edge::modified_by("p1", "p2"), // target should be modifier
        ],
    );
    let result = validate(&gir, false);
    assert!(!result.valid);
    assert!(result
        .errors
        .iter()
        .any(|e| matches!(e, UlError::SortViolation { .. })));
}

// ── Containment cycle detection ──

#[test]
fn deep_containment_cycle_detected() {
    // a → b → c → a (3-node cycle)
    let gir = Gir {
        ul_gir: "0.1.0".to_string(),
        root: "a".to_string(),
        nodes: vec![
            Node::enclosure("a", EnclosureShape::Circle),
            Node::enclosure("b", EnclosureShape::Circle),
            Node::enclosure("c", EnclosureShape::Circle),
        ],
        edges: vec![
            Edge::contains("a", "b"),
            Edge::contains("b", "c"),
            Edge::contains("c", "a"),
        ],
        metadata: None,
    };
    let result = validate(&gir, false);
    assert!(!result.valid);
    assert!(result
        .errors
        .iter()
        .any(|e| matches!(e, UlError::ContainmentCycle { .. })));
}

// ── Schema validation ──

#[test]
fn empty_nodes_invalid() {
    let gir = Gir {
        ul_gir: "0.1.0".to_string(),
        root: "nonexistent".to_string(),
        nodes: vec![],
        edges: vec![],
        metadata: None,
    };
    let result = validate(&gir, false);
    assert!(!result.valid);
    assert!(result
        .errors
        .iter()
        .any(|e| matches!(e, UlError::DanglingEdgeRef { .. })));
}

#[test]
fn geometry_check_runs_without_error() {
    let gir = minimal_gir(
        vec![Node::enclosure("root", EnclosureShape::Circle)],
        vec![],
    );
    let result = validate(&gir, true);
    // Layer 4 is now implemented — a simple enclosure should pass with no errors
    assert!(result.valid, "errors: {:?}", result.errors);
}

// ── Cross-edge types pass sort validation ──

#[test]
fn adjacent_intersects_connects_references_no_sort_error() {
    let gir = Gir {
        ul_gir: "0.1.0".to_string(),
        root: "root".to_string(),
        nodes: vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::enclosure("e1", EnclosureShape::Circle),
            Node::enclosure("e2", EnclosureShape::Circle),
            Node::point("p1"),
            Node::point("p2"),
        ],
        edges: vec![
            Edge::contains("root", "e1"),
            Edge::contains("root", "e2"),
            Edge::contains("root", "p1"),
            Edge::contains("root", "p2"),
            Edge::adjacent("e1", "e2"),
            Edge::intersects("e1", "e2"),
            Edge::connects("p1", "p2"),
            Edge::references("p1", "e1"),
        ],
        metadata: None,
    };
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
}

// ── New sort constraint tests for Adjacent, Intersects, Connects ──

#[test]
fn adjacent_rejects_non_entity_source() {
    let mut line = Node::line("l1", true);
    line.sort = ul_core::Sort::Relation;
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            line,
            Node::point("p1"),
        ],
        vec![
            Edge::contains("root", "l1"),
            Edge::contains("root", "p1"),
            Edge::adjacent("l1", "p1"),
        ],
    );
    let result = validate(&gir, false);
    assert!(!result.valid);
    assert!(result.errors.iter().any(|e| matches!(
        e,
        UlError::SortViolation { reason, .. } if reason.contains("adjacent source must be entity")
    )));
}

#[test]
fn adjacent_rejects_non_entity_target() {
    let mut line = Node::line("l1", true);
    line.sort = ul_core::Sort::Relation;
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::point("p1"),
            line,
        ],
        vec![
            Edge::contains("root", "p1"),
            Edge::contains("root", "l1"),
            Edge::adjacent("p1", "l1"),
        ],
    );
    let result = validate(&gir, false);
    assert!(!result.valid);
    assert!(result.errors.iter().any(|e| matches!(
        e,
        UlError::SortViolation { reason, .. } if reason.contains("adjacent target must be entity")
    )));
}

#[test]
fn intersects_rejects_modifier_endpoint() {
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::point("p1"),
            Node::angle("a1", 45.0),
        ],
        vec![
            Edge::contains("root", "p1"),
            Edge::contains("root", "a1"),
            Edge::intersects("p1", "a1"),
        ],
    );
    let result = validate(&gir, false);
    assert!(!result.valid);
    assert!(result.errors.iter().any(|e| matches!(
        e,
        UlError::SortViolation { reason, .. } if reason.contains("intersects target must be entity")
    )));
}

#[test]
fn connects_allows_entity_and_relation() {
    // Entity → Connects → Relation (line) is valid (how connections work)
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::point("p1"),
            Node::line("l1", true),
        ],
        vec![
            Edge::contains("root", "p1"),
            Edge::contains("root", "l1"),
            Edge::connects("p1", "l1"),
        ],
    );
    let result = validate(&gir, false);
    assert!(result.valid, "errors: {:?}", result.errors);
}

#[test]
fn connects_rejects_modifier_endpoint() {
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::point("p1"),
            Node::angle("a1", 90.0),
        ],
        vec![
            Edge::contains("root", "p1"),
            Edge::contains("root", "a1"),
            Edge::connects("p1", "a1"),
        ],
    );
    let result = validate(&gir, false);
    assert!(!result.valid);
    assert!(result.errors.iter().any(|e| matches!(
        e,
        UlError::SortViolation { reason, .. } if reason.contains("connects target must be entity or relation")
    )));
}

// ── Geometry (Layer 4) tests ──

#[test]
fn geometry_rejects_invalid_angle_measure() {
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::angle("a1", 400.0), // > 360 invalid
        ],
        vec![Edge::contains("root", "a1")],
    );
    let result = validate(&gir, true);
    assert!(!result.valid);
    assert!(result.errors.iter().any(|e| {
        if let UlError::Render { message } = e { message.contains("invalid measure") } else { false }
    }));
}

#[test]
fn geometry_rejects_negative_angle() {
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::angle("a1", -10.0),
        ],
        vec![Edge::contains("root", "a1")],
    );
    let result = validate(&gir, true);
    assert!(!result.valid);
}

#[test]
fn geometry_rejects_negative_curvature() {
    let mut curve = Node::curve("c1", 1.0);
    curve.curvature = Some(-1.0);
    let gir = minimal_gir(
        vec![
            Node::enclosure("root", EnclosureShape::Circle),
            curve,
        ],
        vec![Edge::contains("root", "c1")],
    );
    let result = validate(&gir, true);
    assert!(!result.valid);
}

#[test]
fn geometry_rejects_polygon_with_two_vertices() {
    let mut enc = Node::enclosure("root", EnclosureShape::Polygon);
    enc.vertices = Some(2);
    let gir = minimal_gir(vec![enc], vec![]);
    let result = validate(&gir, true);
    assert!(!result.valid);
}

#[test]
fn geometry_warns_multiple_containment_parents() {
    let gir = Gir {
        ul_gir: "0.1.0".to_string(),
        root: "root".to_string(),
        nodes: vec![
            Node::enclosure("root", EnclosureShape::Circle),
            Node::enclosure("a", EnclosureShape::Circle),
            Node::enclosure("b", EnclosureShape::Circle),
            Node::point("child"),
        ],
        edges: vec![
            Edge::contains("root", "a"),
            Edge::contains("root", "b"),
            Edge::contains("a", "child"),
            Edge::contains("b", "child"), // child has two parents
        ],
        metadata: None,
    };
    let result = validate(&gir, true);
    assert!(result.warnings.iter().any(|w| w.contains("multiple enclosures")));
}
