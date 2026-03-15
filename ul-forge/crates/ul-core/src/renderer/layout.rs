//! Layout computation: position GIR nodes on a 2D canvas.
//!
//! Three-level approach:
//! 1. Template lookup for known canonical glyphs
//! 2. Hierarchical constraint layout for novel compositions
//! 3. (Future) Aesthetic refinement

use crate::types::edge::EdgeType;
use crate::types::gir::Gir;
use crate::types::node::{EnclosureShape, NodeType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::templates;

/// A positioned element ready for SVG generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionedElement {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
    pub shape: Shape,
}

/// Visual shape for a positioned element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Point {
        radius: f64,
    },
    Circle {
        radius: f64,
    },
    Triangle {
        size: f64,
    },
    Square {
        size: f64,
    },
    Pentagon {
        size: f64,
    },
    Hexagon {
        size: f64,
    },
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        directed: bool,
    },
    Angle {
        radius: f64,
        degrees: f64,
    },
    Curve {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        curvature: f64,
    },
}

/// A connection (edge) to draw between positioned elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub edge_id: String,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub directed: bool,
    pub dashed: bool,
}

/// Fully positioned glyph ready for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionedGlyph {
    pub elements: Vec<PositionedElement>,
    pub connections: Vec<Connection>,
    pub width: f64,
    pub height: f64,
}

/// Saved layout state (for incremental updates in Phase 2+).
#[derive(Debug, Clone)]
pub struct Layout {
    pub glyph: PositionedGlyph,
}

/// Compute layout for a GIR document.
pub fn compute_layout(gir: &Gir, width: f64, height: f64) -> PositionedGlyph {
    // Level 1: Try template matching
    if let Some(positioned) = templates::match_template(gir, width, height) {
        return positioned;
    }

    // Level 2: Hierarchical constraint layout
    hierarchical_layout(gir, width, height)
}

/// Hierarchical constraint layout for novel compositions.
fn hierarchical_layout(gir: &Gir, width: f64, height: f64) -> PositionedGlyph {
    let mut elements = Vec::new();
    let mut connections = Vec::new();
    let mut positions: HashMap<String, (f64, f64)> = HashMap::new();

    // Phase A: Extract tree spine, allocate bounding boxes
    let bbox = BBox {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    allocate_tree(gir, &gir.root, &bbox, &mut elements, &mut positions);

    // Phase B: Route cross-edges (connects, references)
    for edge in &gir.edges {
        match edge.edge_type {
            EdgeType::Connects => {
                if let (Some(&(x1, y1)), Some(&(x2, y2))) =
                    (positions.get(&edge.source), positions.get(&edge.target))
                {
                    let line_node = gir.node(&edge.source);
                    let directed = line_node.and_then(|n| n.directed).unwrap_or(true);
                    connections.push(Connection {
                        edge_id: format!("{}-{}", edge.source, edge.target),
                        x1,
                        y1,
                        x2,
                        y2,
                        directed,
                        dashed: false,
                    });
                }
            }
            EdgeType::References => {
                if let (Some(&(x1, y1)), Some(&(x2, y2))) =
                    (positions.get(&edge.source), positions.get(&edge.target))
                {
                    connections.push(Connection {
                        edge_id: format!("{}-{}", edge.source, edge.target),
                        x1,
                        y1,
                        x2,
                        y2,
                        directed: false,
                        dashed: true,
                    });
                }
            }
            _ => {}
        }
    }

    PositionedGlyph {
        elements,
        connections,
        width,
        height,
    }
}

#[derive(Debug, Clone)]
struct BBox {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl BBox {
    fn cx(&self) -> f64 {
        self.x + self.w / 2.0
    }
    fn cy(&self) -> f64 {
        self.y + self.h / 2.0
    }
}

fn allocate_tree(
    gir: &Gir,
    node_id: &str,
    bbox: &BBox,
    elements: &mut Vec<PositionedElement>,
    positions: &mut HashMap<String, (f64, f64)>,
) {
    let node = match gir.node(node_id) {
        Some(n) => n,
        None => return,
    };

    let cx = bbox.cx();
    let cy = bbox.cy();
    positions.insert(node_id.to_string(), (cx, cy));

    // Skip implicit root from visual rendering (it's structural only)
    let is_implicit = node.label.as_deref() == Some("_implicit_root");

    if !is_implicit {
        let shape = node_to_shape(node, bbox);
        elements.push(PositionedElement {
            node_id: node_id.to_string(),
            x: cx,
            y: cy,
            shape,
        });
    }

    // Get contained children (via contains edges)
    let children = gir.children_of(node_id);
    // Filter to non-line/non-angle children for spatial layout
    let spatial_children: Vec<&str> = children
        .iter()
        .filter(|cid| {
            gir.node(cid)
                .is_some_and(|n| n.node_type != NodeType::Line && n.node_type != NodeType::Angle)
        })
        .copied()
        .collect();

    // Also collect line/angle children for separate positioning
    let line_children: Vec<&str> = children
        .iter()
        .filter(|cid| gir.node(cid).is_some_and(|n| n.node_type == NodeType::Line))
        .copied()
        .collect();

    let angle_children: Vec<&str> = children
        .iter()
        .filter(|cid| {
            gir.node(cid)
                .is_some_and(|n| n.node_type == NodeType::Angle)
        })
        .copied()
        .collect();

    if !spatial_children.is_empty() {
        // Partition bounding box among children
        let padding = bbox.w.min(bbox.h) * 0.1;
        let inner = BBox {
            x: bbox.x + padding,
            y: bbox.y + padding,
            w: bbox.w - 2.0 * padding,
            h: bbox.h - 2.0 * padding,
        };

        let n = spatial_children.len();
        if n == 1 {
            // Single child centered
            allocate_tree(gir, spatial_children[0], &inner, elements, positions);
        } else {
            // Horizontal strip layout
            let child_w = inner.w / n as f64;
            for (i, child_id) in spatial_children.iter().enumerate() {
                let child_bbox = BBox {
                    x: inner.x + i as f64 * child_w,
                    y: inner.y,
                    w: child_w,
                    h: inner.h,
                };
                allocate_tree(gir, child_id, &child_bbox, elements, positions);
            }
        }
    }

    // Position line children near parent center
    for line_id in line_children {
        let pos = (cx, cy);
        positions.insert(line_id.to_string(), pos);
        // Lines are drawn as connections, not shapes — skip element creation
    }

    // Position angle children near parent center
    for angle_id in angle_children {
        let pos = (cx, cy);
        positions.insert(angle_id.to_string(), pos);
        if let Some(n) = gir.node(angle_id) {
            elements.push(PositionedElement {
                node_id: angle_id.to_string(),
                x: cx,
                y: cy,
                shape: Shape::Angle {
                    radius: bbox.w.min(bbox.h) * 0.15,
                    degrees: n.measure.unwrap_or(60.0),
                },
            });
        }
    }
}

fn node_to_shape(node: &crate::types::node::Node, bbox: &BBox) -> Shape {
    let size = bbox.w.min(bbox.h);
    match node.node_type {
        NodeType::Point => Shape::Point { radius: 3.0 },
        NodeType::Enclosure => match node.shape {
            Some(EnclosureShape::Circle) => Shape::Circle { radius: size * 0.4 },
            Some(EnclosureShape::Triangle) => Shape::Triangle { size: size * 0.7 },
            Some(EnclosureShape::Square) => Shape::Square { size: size * 0.7 },
            Some(EnclosureShape::Polygon) => match node.vertices {
                Some(5) => Shape::Pentagon { size: size * 0.7 },
                Some(6) => Shape::Hexagon { size: size * 0.7 },
                _ => Shape::Circle { radius: size * 0.4 },
            },
            Some(EnclosureShape::Ellipse) => Shape::Circle { radius: size * 0.4 },
            Some(EnclosureShape::Freeform) => Shape::Circle { radius: size * 0.4 },
            None => Shape::Circle { radius: size * 0.4 },
        },
        NodeType::Line => {
            let half = size * 0.35;
            let cx = bbox.cx();
            let cy = bbox.cy();
            Shape::Line {
                x1: cx - half,
                y1: cy,
                x2: cx + half,
                y2: cy,
                directed: node.directed.unwrap_or(true),
            }
        }
        NodeType::Angle => Shape::Angle {
            radius: size * 0.25,
            degrees: node.measure.unwrap_or(60.0),
        },
        NodeType::Curve => {
            let half = size * 0.35;
            let cx = bbox.cx();
            let cy = bbox.cy();
            Shape::Curve {
                x1: cx - half,
                y1: cy,
                x2: cx + half,
                y2: cy,
                curvature: node.curvature.unwrap_or(0.5),
            }
        }
    }
}
