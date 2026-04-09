//! Layout computation: position GIR nodes on a 2D canvas.
//!
//! Three-level approach:
//! 1. Template lookup for known canonical glyphs
//! 2. Hierarchical constraint layout for novel compositions
//! 3. Adjacency/intersection-aware positioning

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
    /// Optional CSS class override for assertion modifier styling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_class: Option<String>,
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
        /// Piecewise curvature profile — if present, renders as multi-segment path.
        curvature_profile: Option<Vec<f64>>,
    },
    /// Variable binding slot — dashed circle with optional label.
    VariableSlot {
        radius: f64,
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

    // Phase B: Route cross-edges (connects, adjacency, intersection, references, binds)
    for edge in &gir.edges {
        match edge.edge_type {
            EdgeType::Connects => {
                // For connect edges: check if source is a line node connecting two endpoints
                let source_node = gir.node(&edge.source);
                let target_pos = positions.get(&edge.target);
                let source_pos = positions.get(&edge.source);

                if let (Some(s_node), Some(&(x1, y1)), Some(&(x2, y2))) =
                    (source_node, source_pos, target_pos)
                {
                    if s_node.node_type == NodeType::Line {
                        // Line node connecting to an endpoint — draw as arrow
                        let directed = s_node.directed.unwrap_or(true);
                        // Use direction vector from the line node for the connection angle
                        connections.push(Connection {
                            edge_id: format!("{}-{}", edge.source, edge.target),
                            x1,
                            y1,
                            x2,
                            y2,
                            directed,
                            dashed: false,
                        });
                    } else {
                        // Non-line source connecting to target
                        connections.push(Connection {
                            edge_id: format!("{}-{}", edge.source, edge.target),
                            x1,
                            y1,
                            x2,
                            y2,
                            directed: true,
                            dashed: false,
                        });
                    }
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
            EdgeType::Binds => {
                if let (Some(&(x1, y1)), Some(&(x2, y2))) =
                    (positions.get(&edge.source), positions.get(&edge.target))
                {
                    connections.push(Connection {
                        edge_id: format!("bind-{}-{}", edge.source, edge.target),
                        x1,
                        y1,
                        x2,
                        y2,
                        directed: true,
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
        let css_class = node.assertion_modifier.as_ref().map(|m| {
            match m {
                crate::types::node::AssertionModifierKind::Evidential => "ul-evidential".to_string(),
                crate::types::node::AssertionModifierKind::Emphatic => "ul-emphatic".to_string(),
                crate::types::node::AssertionModifierKind::Hedged => "ul-hedged".to_string(),
            }
        });
        elements.push(PositionedElement {
            node_id: node_id.to_string(),
            x: cx,
            y: cy,
            shape,
            css_class,
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

    // Collect line and angle children for connection/modifier positioning
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
        } else if n == 2 {
            // Check if there's an intersection or adjacency edge between them
            let has_intersection = gir.edges.iter().any(|e| {
                e.edge_type == EdgeType::Intersects
                    && ((e.source == spatial_children[0] && e.target == spatial_children[1])
                        || (e.source == spatial_children[1] && e.target == spatial_children[0]))
            });
            let has_adjacency = gir.edges.iter().any(|e| {
                e.edge_type == EdgeType::Adjacent
                    && ((e.source == spatial_children[0] && e.target == spatial_children[1])
                        || (e.source == spatial_children[1] && e.target == spatial_children[0]))
            });

            if has_intersection {
                // Overlap: place side by side with significant overlap
                let child_w = inner.w * 0.6;
                let left = BBox {
                    x: inner.x,
                    y: inner.y,
                    w: child_w,
                    h: inner.h,
                };
                let right = BBox {
                    x: inner.x + inner.w - child_w,
                    y: inner.y,
                    w: child_w,
                    h: inner.h,
                };
                allocate_tree(gir, spatial_children[0], &left, elements, positions);
                allocate_tree(gir, spatial_children[1], &right, elements, positions);
            } else if has_adjacency {
                // Adjacent: place side by side touching
                let gap = inner.w * 0.02;
                let child_w = (inner.w - gap) / 2.0;
                let left = BBox {
                    x: inner.x,
                    y: inner.y,
                    w: child_w,
                    h: inner.h,
                };
                let right = BBox {
                    x: inner.x + child_w + gap,
                    y: inner.y,
                    w: child_w,
                    h: inner.h,
                };
                allocate_tree(gir, spatial_children[0], &left, elements, positions);
                allocate_tree(gir, spatial_children[1], &right, elements, positions);
            } else {
                // Default: horizontal split with spacing
                let child_w = inner.w / 2.0;
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
        } else if n <= 6 {
            // 3-6 children: radial layout around center
            let r = inner.w.min(inner.h) * 0.35;
            let child_size = inner.w.min(inner.h) / (n as f64).sqrt();
            for (i, child_id) in spatial_children.iter().enumerate() {
                let angle = -std::f64::consts::FRAC_PI_2
                    + 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
                let child_cx = inner.cx() + r * angle.cos();
                let child_cy = inner.cy() + r * angle.sin();
                let child_bbox = BBox {
                    x: child_cx - child_size / 2.0,
                    y: child_cy - child_size / 2.0,
                    w: child_size,
                    h: child_size,
                };
                allocate_tree(gir, child_id, &child_bbox, elements, positions);
            }
        } else {
            // 7+ children: grid layout
            let cols = ((n as f64).sqrt().ceil()) as usize;
            let rows = (n + cols - 1) / cols;
            let cell_w = inner.w / cols as f64;
            let cell_h = inner.h / rows as f64;
            for (i, child_id) in spatial_children.iter().enumerate() {
                let col = i % cols;
                let row = i / cols;
                let child_bbox = BBox {
                    x: inner.x + col as f64 * cell_w,
                    y: inner.y + row as f64 * cell_h,
                    w: cell_w,
                    h: cell_h,
                };
                allocate_tree(gir, child_id, &child_bbox, elements, positions);
            }
        }
    }

    // Position line children at midpoint between their connection endpoints
    for line_id in line_children {
        // Find the two nodes this line connects to
        let connected: Vec<&str> = gir
            .edges
            .iter()
            .filter(|e| {
                e.edge_type == EdgeType::Connects
                    && (e.source == line_id || e.target == line_id)
            })
            .map(|e| {
                if e.source == line_id {
                    e.target.as_str()
                } else {
                    e.source.as_str()
                }
            })
            .collect();

        let (lx, ly) = if connected.len() >= 2 {
            // Place at midpoint of the two connected endpoints
            let p1 = positions.get(connected[0]).copied().unwrap_or((cx, cy));
            let p2 = positions.get(connected[1]).copied().unwrap_or((cx, cy));
            ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0)
        } else {
            (cx, cy)
        };
        positions.insert(line_id.to_string(), (lx, ly));
        // Lines are drawn as connections, not shapes — skip element creation
    }

    // Position angle children near their modified line or at parent center
    for angle_id in angle_children {
        // Find if this angle modifies a line (via modified_by edge)
        let mod_target = gir.edges.iter().find(|e| {
            e.edge_type == EdgeType::ModifiedBy && e.target == angle_id
        });
        let (ax, ay) = if let Some(me) = mod_target {
            positions.get(&me.source).copied().unwrap_or((cx, cy))
        } else {
            (cx, cy)
        };
        positions.insert(angle_id.to_string(), (ax, ay));
        if let Some(n) = gir.node(angle_id) {
            elements.push(PositionedElement {
                node_id: angle_id.to_string(),
                x: ax,
                y: ay,
                shape: Shape::Angle {
                    radius: bbox.w.min(bbox.h) * 0.15,
                    degrees: n.measure.unwrap_or(60.0),
                },
                css_class: None,
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
            // Use direction vector if present, otherwise default to horizontal
            let (dx, dy) = node
                .direction
                .map(|d| {
                    let len = (d[0] * d[0] + d[1] * d[1]).sqrt();
                    if len > 0.0 {
                        (d[0] / len, d[1] / len)
                    } else {
                        (1.0, 0.0)
                    }
                })
                .unwrap_or((1.0, 0.0));
            Shape::Line {
                x1: cx - half * dx,
                y1: cy - half * dy,
                x2: cx + half * dx,
                y2: cy + half * dy,
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
                curvature_profile: node.curvature_profile.clone(),
            }
        }
        NodeType::VariableSlot => {
            // Variable slot renders as a dashed circle (visually distinct from filled Point)
            Shape::VariableSlot { radius: size * 0.08 }
        }
    }
}
