//! GIR → UL-Script deparser (canonical form).
//!
//! Converts a GIR document back to a canonical UL-Script text representation.
//! The canonical form ensures `parse(deparse(gir))` produces an equivalent GIR.

use crate::types::edge::EdgeType;
use crate::types::gir::Gir;
use crate::types::node::{AssertionModifierKind, EnclosureShape, NodeType, PerformativeForce};

/// Deparse a GIR document to canonical UL-Script.
pub fn deparse(gir: &Gir) -> String {
    let _root = match gir.node(&gir.root) {
        Some(n) => n,
        None => return String::new(),
    };

    let mut out = String::new();
    deparse_node(gir, &gir.root, &mut out);

    // Append cross-edges that aren't containment or modified_by
    // (connections, adjacency, intersection between top-level terms)
    // These are represented by operators in the composition
    // For now, tree-spine traversal handles most cases

    out
}

fn deparse_node(gir: &Gir, node_id: &str, out: &mut String) {
    let node = match gir.node(node_id) {
        Some(n) => n,
        None => return,
    };

    // Check if this is an implicit root (skip it, deparse children with operators)
    if node.label.as_deref() == Some("_implicit_root") {
        deparse_implicit_root(gir, node_id, out);
        return;
    }

    // Write the primitive symbol
    match node.node_type {
        NodeType::Point => {
            out.push('●');
            // Bound reference: ●_x
            if let Some(ref vid) = node.variable_id {
                out.push('_');
                out.push_str(vid);
            }
        }
        NodeType::Line => {
            if let Some(dir) = &node.direction {
                if dir[0] < 0.0 {
                    out.push('←');
                } else {
                    out.push('→');
                }
            } else if node.directed == Some(false) || node.directed.is_none() {
                out.push('↔');
            } else {
                out.push('→');
            }
        }
        NodeType::Angle => {
            if let Some(m) = node.measure {
                out.push_str(&format!("∠{}", format_number(m)));
            }
        }
        NodeType::Curve => out.push('~'),
        NodeType::Enclosure => {
            // Check for assertion modifier — emit ?{...}, !{...}, ~?{...}
            if let Some(ref mod_kind) = node.assertion_modifier {
                let prefix = match mod_kind {
                    AssertionModifierKind::Evidential => "?",
                    AssertionModifierKind::Emphatic => "!",
                    AssertionModifierKind::Hedged => "~?",
                };
                out.push_str(prefix);
                let children = gir.children_of(node_id);
                out.push('{');
                deparse_children(gir, &children, node_id, out);
                out.push('}');
                return; // content already emitted
            }
            // Check for modal operator patterns (labeled □, ◇, □→)
            if let Some(ref label) = node.label {
                match label.as_str() {
                    "□" => {
                        out.push_str("[]");
                        let children = non_world_children(gir, node_id);
                        out.push('{');
                        deparse_children(gir, &children, node_id, out);
                        out.push('}');
                        return;
                    }
                    "◇" => {
                        out.push_str("<>");
                        let children = non_world_children(gir, node_id);
                        out.push('{');
                        deparse_children(gir, &children, node_id, out);
                        out.push('}');
                        return;
                    }
                    "□→" => {
                        out.push_str("[]->");
                        let children = non_world_children(gir, node_id);
                        // Split into two halves (antecedent, consequent) by connected-to-world pattern
                        // For simplicity, emit all content in first block
                        out.push('{');
                        let mid = children.len() / 2;
                        let (ante, cons) = if mid > 0 && children.len() > 1 {
                            (&children[..mid], &children[mid..])
                        } else {
                            (children.as_slice(), &[] as &[&str])
                        };
                        deparse_children(gir, ante, node_id, out);
                        out.push('}');
                        out.push('{');
                        deparse_children(gir, cons, node_id, out);
                        out.push('}');
                        return;
                    }
                    _ => {}
                }
            }
            // Check for force annotation
            if let Some(ref force) = node.force {
                let token = match force {
                    PerformativeForce::Assert => "assert",
                    PerformativeForce::Query => "query",
                    PerformativeForce::Direct => "direct",
                    PerformativeForce::Commit => "commit",
                    PerformativeForce::Express => "express",
                    PerformativeForce::Declare => "declare",
                };
                out.push_str(token);
                let children = gir.children_of(node_id);
                out.push('{');
                deparse_children(gir, &children, node_id, out);
                out.push('}');
                return;
            }
            match node.shape {
                Some(EnclosureShape::Circle) => out.push('○'),
                Some(EnclosureShape::Triangle) => out.push('△'),
                Some(EnclosureShape::Square) => out.push('□'),
                Some(EnclosureShape::Polygon) => match node.vertices {
                    Some(5) => out.push('⬠'),
                    Some(6) => out.push('⬡'),
                    _ => out.push('○'), // fallback for other polygons
                },
                Some(EnclosureShape::Ellipse) => out.push('○'), // render as circle in text
                Some(EnclosureShape::Freeform) => out.push('○'),
                None => out.push('○'), // default to circle
            }
        }
        NodeType::VariableSlot => {
            // Variable slot: ○ with subscript variable id
            out.push('○');
            if let Some(ref vid) = node.variable_id {
                out.push('_');
                out.push_str(vid);
            }
        }
    }

    // Write content (children via contains edges)
    let children = gir.children_of(node_id);
    if !children.is_empty() {
        out.push('{');
        deparse_children(gir, &children, node_id, out);
        out.push('}');
    }
}

fn deparse_children(gir: &Gir, child_ids: &[&str], parent_id: &str, out: &mut String) {
    // Find adjacency/intersection edges between children
    let mut i = 0;
    while i < child_ids.len() {
        let cid = child_ids[i];

        // Skip line/angle nodes that are part of connection operators
        let cnode = gir.node(cid);
        if let Some(n) = cnode {
            if n.node_type == NodeType::Line || n.node_type == NodeType::Angle {
                i += 1;
                continue;
            }
        }

        if i > 0 {
            // Check if there's an operator edge between previous and current
            let prev = find_prev_non_line(gir, child_ids, i);
            if let Some(prev_id) = prev {
                if let Some(op) = find_operator_between(gir, prev_id, cid, parent_id) {
                    out.push(' ');
                    out.push_str(&op);
                    out.push(' ');
                } else {
                    out.push_str(" | ");
                }
            }
        }

        deparse_node(gir, cid, out);
        i += 1;
    }
}

fn find_prev_non_line<'a>(gir: &Gir, child_ids: &[&'a str], current_idx: usize) -> Option<&'a str> {
    for j in (0..current_idx).rev() {
        if let Some(n) = gir.node(child_ids[j]) {
            if n.node_type != NodeType::Line && n.node_type != NodeType::Angle {
                return Some(child_ids[j]);
            }
        }
    }
    None
}

fn find_operator_between(gir: &Gir, left: &str, right: &str, _parent: &str) -> Option<String> {
    // Check for adjacency
    for edge in &gir.edges {
        if edge.source == left && edge.target == right {
            match edge.edge_type {
                EdgeType::Adjacent => return Some("|".to_string()),
                EdgeType::Intersects => return Some("&".to_string()),
                EdgeType::Connects => {
                    // This is a line node connecting left to right
                    // Find the line node and check for angle modifier
                    return Some("→".to_string());
                }
                _ => {}
            }
        }
        if edge.source == right && edge.target == left {
            match edge.edge_type {
                EdgeType::Adjacent => return Some("|".to_string()),
                EdgeType::Intersects => return Some("&".to_string()),
                _ => {}
            }
        }
    }

    // Check for connection via a line node
    for edge1 in &gir.edges {
        if edge1.source == left && edge1.edge_type == EdgeType::Connects {
            let line_id = &edge1.target;
            for edge2 in &gir.edges {
                if edge2.source == line_id.as_str()
                    && edge2.target == right
                    && edge2.edge_type == EdgeType::Connects
                {
                    // Found: left →connects→ line →connects→ right
                    // Check for angle modifier on the line
                    let angle = gir
                        .edges
                        .iter()
                        .find(|e| e.source == *line_id && e.edge_type == EdgeType::ModifiedBy)
                        .and_then(|e| gir.node(&e.target))
                        .and_then(|n| n.measure);

                    let arrow = match gir.node(line_id) {
                        Some(n) => {
                            if let Some(dir) = &n.direction {
                                if dir[0] < 0.0 {
                                    "←"
                                } else {
                                    "→"
                                }
                            } else {
                                "↔"
                            }
                        }
                        None => "→",
                    };

                    return if let Some(deg) = angle {
                        Some(format!("{}@{}", arrow, format_number(deg)))
                    } else {
                        Some(arrow.to_string())
                    };
                }
            }
        }
    }

    None
}

fn deparse_implicit_root(gir: &Gir, root_id: &str, out: &mut String) {
    let children = gir.children_of(root_id);
    deparse_children(gir, &children, root_id, out);
}

fn format_number(n: f64) -> String {
    if n == n.floor() {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}

/// Get children of a modal enclosure that aren't world entities (w_current, w′, w_closest).
fn non_world_children<'a>(gir: &'a Gir, parent_id: &str) -> Vec<&'a str> {
    let world_labels = ["w_current", "w′", "w_closest"];
    gir.children_of(parent_id)
        .into_iter()
        .filter(|cid| {
            gir.node(cid)
                .and_then(|n| n.label.as_deref())
                .map(|l| !world_labels.contains(&l))
                .unwrap_or(true)
        })
        .collect()
}
