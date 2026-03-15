//! Template library: pre-computed visual forms for the 12 Tier-1 canonical glyphs.
//!
//! Template matching checks if a GIR subgraph matches a known canonical glyph
//! and returns pre-computed element positions if so.

use crate::types::edge::EdgeType;
use crate::types::gir::Gir;
use crate::types::node::{EnclosureShape, NodeType};

use super::layout::{PositionedElement, PositionedGlyph, Shape};

/// Attempt to match the GIR against a known Tier-1 template.
/// Returns a fully positioned glyph if matched.
pub fn match_template(gir: &Gir, width: f64, height: f64) -> Option<PositionedGlyph> {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let scale = width.min(height) / 2.5;

    // Analyze the structure
    let root = gir.node(&gir.root)?;
    let children = gir.children_of(&gir.root);

    // Single-node templates
    if gir.nodes.len() == 1 {
        return match_single_node(gir, cx, cy, scale);
    }

    // Template #1: Existence — ○{●} (circle containing point)
    if root.node_type == NodeType::Enclosure
        && root.shape == Some(EnclosureShape::Circle)
        && children.len() == 1
    {
        let child = gir.node(children[0])?;
        if child.node_type == NodeType::Point && gir.nodes.len() == 2 {
            return Some(existence_template(gir, cx, cy, scale));
        }
    }

    // Template #9: Connection — ● → ● (arrow between points)
    if is_connection_pattern(gir) {
        return Some(connection_template(gir, cx, cy, scale));
    }

    // Template #10: Boundary — ○ | ○ (adjacent circles)
    if is_adjacency_pattern(gir, NodeType::Enclosure) {
        return Some(adjacency_template(gir, cx, cy, scale));
    }

    // Template #11: Intersection — ○ & ○ (overlapping circles)
    if is_intersection_pattern(gir, NodeType::Enclosure) {
        return Some(intersection_template(gir, cx, cy, scale));
    }

    // Template #12: Containment — ○{○} (circle in circle)
    if root.node_type == NodeType::Enclosure
        && root.shape == Some(EnclosureShape::Circle)
        && children.len() == 1
    {
        let child = gir.node(children[0])?;
        if child.node_type == NodeType::Enclosure
            && child.shape == Some(EnclosureShape::Circle)
            && gir.nodes.len() == 2
        {
            return Some(containment_template(gir, cx, cy, scale));
        }
    }

    None
}

fn match_single_node(gir: &Gir, cx: f64, cy: f64, scale: f64) -> Option<PositionedGlyph> {
    let node = &gir.nodes[0];
    let elem = match node.node_type {
        // #8: Point
        NodeType::Point => PositionedElement {
            node_id: node.id.clone(),
            x: cx,
            y: cy,
            shape: Shape::Point { radius: 3.0 },
        },
        // #5: Concept (circle) / #6: Structure (triangle) / #7: Foundation (square)
        NodeType::Enclosure => {
            let shape = match node.shape {
                Some(EnclosureShape::Circle) => Shape::Circle {
                    radius: scale * 0.6,
                },
                Some(EnclosureShape::Triangle) => Shape::Triangle { size: scale * 1.0 },
                Some(EnclosureShape::Square) => Shape::Square { size: scale * 1.0 },
                _ => Shape::Circle {
                    radius: scale * 0.6,
                },
            };
            PositionedElement {
                node_id: node.id.clone(),
                x: cx,
                y: cy,
                shape,
            }
        }
        // #2: Relation (arrow)
        NodeType::Line => {
            let half = scale * 0.5;
            PositionedElement {
                node_id: node.id.clone(),
                x: cx,
                y: cy,
                shape: Shape::Line {
                    x1: cx - half,
                    y1: cy,
                    x2: cx + half,
                    y2: cy,
                    directed: node.directed.unwrap_or(true),
                },
            }
        }
        // #3: Quality (angle)
        NodeType::Angle => {
            let m = node.measure.unwrap_or(60.0);
            PositionedElement {
                node_id: node.id.clone(),
                x: cx,
                y: cy,
                shape: Shape::Angle {
                    radius: scale * 0.4,
                    degrees: m,
                },
            }
        }
        // #4: Process (curve)
        NodeType::Curve => {
            let half = scale * 0.5;
            PositionedElement {
                node_id: node.id.clone(),
                x: cx,
                y: cy,
                shape: Shape::Curve {
                    x1: cx - half,
                    y1: cy,
                    x2: cx + half,
                    y2: cy,
                    curvature: node.curvature.unwrap_or(0.5),
                },
            }
        }
    };

    Some(PositionedGlyph {
        elements: vec![elem],
        connections: Vec::new(),
        width: cx * 2.0,
        height: cy * 2.0,
    })
}

fn existence_template(gir: &Gir, cx: f64, cy: f64, scale: f64) -> PositionedGlyph {
    let root = gir
        .nodes
        .iter()
        .find(|n| n.node_type == NodeType::Enclosure);
    let point = gir
        .nodes
        .iter()
        .find(|n| n.node_type == NodeType::Point);
    let r = scale * 0.6;
    PositionedGlyph {
        elements: vec![
            PositionedElement {
                node_id: root.map_or_else(|| "enclosure".into(), |n| n.id.clone()),
                x: cx,
                y: cy,
                shape: Shape::Circle { radius: r },
            },
            PositionedElement {
                node_id: point.map_or_else(|| "point".into(), |n| n.id.clone()),
                x: cx,
                y: cy,
                shape: Shape::Point { radius: 3.0 },
            },
        ],
        connections: Vec::new(),
        width: cx * 2.0,
        height: cy * 2.0,
    }
}

fn connection_template(gir: &Gir, cx: f64, cy: f64, scale: f64) -> PositionedGlyph {
    let points: Vec<_> = gir
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Point)
        .collect();
    let line = gir
        .nodes
        .iter()
        .find(|n| n.node_type == NodeType::Line);
    let half = scale * 0.7;
    let mut elems = Vec::new();

    if let Some(p1) = points.first() {
        elems.push(PositionedElement {
            node_id: p1.id.clone(),
            x: cx - half,
            y: cy,
            shape: Shape::Point { radius: 3.0 },
        });
    }
    if let Some(p2) = points.get(1) {
        elems.push(PositionedElement {
            node_id: p2.id.clone(),
            x: cx + half,
            y: cy,
            shape: Shape::Point { radius: 3.0 },
        });
    }

    let conn = super::layout::Connection {
        edge_id: line.map_or_else(|| "conn".into(), |l| format!("{}-conn", l.id)),
        x1: cx - half,
        y1: cy,
        x2: cx + half,
        y2: cy,
        directed: line.and_then(|l| l.directed).unwrap_or(true),
        dashed: false,
    };

    PositionedGlyph {
        elements: elems,
        connections: vec![conn],
        width: cx * 2.0,
        height: cy * 2.0,
    }
}

fn adjacency_template(gir: &Gir, cx: f64, cy: f64, scale: f64) -> PositionedGlyph {
    let enclosures: Vec<_> = gir
        .nodes
        .iter()
        .filter(|n| {
            n.node_type == NodeType::Enclosure && n.label.as_deref() != Some("_implicit_root")
        })
        .collect();
    let r = scale * 0.4;
    let mut elems = Vec::new();

    if let Some(e1) = enclosures.first() {
        elems.push(PositionedElement {
            node_id: e1.id.clone(),
            x: cx - r,
            y: cy,
            shape: Shape::Circle { radius: r },
        });
    }
    if let Some(e2) = enclosures.get(1) {
        elems.push(PositionedElement {
            node_id: e2.id.clone(),
            x: cx + r,
            y: cy,
            shape: Shape::Circle { radius: r },
        });
    }

    PositionedGlyph {
        elements: elems,
        connections: Vec::new(),
        width: cx * 2.0,
        height: cy * 2.0,
    }
}

fn intersection_template(gir: &Gir, cx: f64, cy: f64, scale: f64) -> PositionedGlyph {
    let enclosures: Vec<_> = gir
        .nodes
        .iter()
        .filter(|n| {
            n.node_type == NodeType::Enclosure && n.label.as_deref() != Some("_implicit_root")
        })
        .collect();
    let r = scale * 0.45;
    let overlap = r * 0.5;
    let mut elems = Vec::new();

    if let Some(e1) = enclosures.first() {
        elems.push(PositionedElement {
            node_id: e1.id.clone(),
            x: cx - overlap,
            y: cy,
            shape: Shape::Circle { radius: r },
        });
    }
    if let Some(e2) = enclosures.get(1) {
        elems.push(PositionedElement {
            node_id: e2.id.clone(),
            x: cx + overlap,
            y: cy,
            shape: Shape::Circle { radius: r },
        });
    }

    PositionedGlyph {
        elements: elems,
        connections: Vec::new(),
        width: cx * 2.0,
        height: cy * 2.0,
    }
}

fn containment_template(gir: &Gir, cx: f64, cy: f64, scale: f64) -> PositionedGlyph {
    let outer = gir.nodes.iter().find(|n| n.id == gir.root);
    let inner = gir.nodes.iter().find(|n| n.id != gir.root);
    let r_outer = scale * 0.7;
    let r_inner = scale * 0.35;

    PositionedGlyph {
        elements: vec![
            PositionedElement {
                node_id: outer.map_or_else(|| "outer".into(), |n| n.id.clone()),
                x: cx,
                y: cy,
                shape: Shape::Circle { radius: r_outer },
            },
            PositionedElement {
                node_id: inner.map_or_else(|| "inner".into(), |n| n.id.clone()),
                x: cx,
                y: cy,
                shape: Shape::Circle { radius: r_inner },
            },
        ],
        connections: Vec::new(),
        width: cx * 2.0,
        height: cy * 2.0,
    }
}

// --- Pattern detection helpers ---

fn is_connection_pattern(gir: &Gir) -> bool {
    let points = gir
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Point)
        .count();
    let lines = gir
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Line)
        .count();
    let connects = gir
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Connects)
        .count();
    // ● → ● pattern: 2 points, 1 line, 2 connects edges + implicit root
    points == 2 && lines == 1 && connects == 2
}

fn is_adjacency_pattern(gir: &Gir, node_type: NodeType) -> bool {
    let matching = gir
        .nodes
        .iter()
        .filter(|n| n.node_type == node_type && n.label.as_deref() != Some("_implicit_root"))
        .count();
    let adjacent = gir
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Adjacent)
        .count();
    matching == 2 && adjacent == 1 && gir.nodes.len() <= 3
}

fn is_intersection_pattern(gir: &Gir, node_type: NodeType) -> bool {
    let matching = gir
        .nodes
        .iter()
        .filter(|n| n.node_type == node_type && n.label.as_deref() != Some("_implicit_root"))
        .count();
    let intersects = gir
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Intersects)
        .count();
    matching == 2 && intersects == 1 && gir.nodes.len() <= 3
}
