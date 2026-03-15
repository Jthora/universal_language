//! AST → GIR transformation.
//!
//! Implements the 5 transformation rules from ast-to-gir.md:
//! 1. Primitive → Node
//! 2. Content braces → contains edges
//! 3. Angle modification → modified_by edges
//! 4. Operators → cross-edges
//! 5. Root determination

use crate::error::UlResult;
use crate::types::edge::{Edge, EdgeType};
use crate::types::gir::Gir;
use crate::types::node::{EnclosureShape, Node, NodeType};
use crate::types::sort::Sort;

use super::ast::*;

/// Counter for deterministic ID assignment.
struct IdGen {
    counter: usize,
}

impl IdGen {
    fn new() -> Self {
        IdGen { counter: 0 }
    }

    fn next(&mut self) -> String {
        self.counter += 1;
        format!("n{}", self.counter)
    }
}

/// Transform a parsed AST document into a GIR.
/// For now, only the first glyph is transformed (single-glyph mode).
pub fn ast_to_gir(doc: &AstDocument) -> UlResult<Gir> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut id_gen = IdGen::new();

    // Find first real composition (skip comments)
    let composition = doc.glyphs.iter().find_map(|g| match g {
        AstGlyph::Composition(c) => Some(c),
        AstGlyph::Comment(_) => None,
    });

    let root_id = match composition {
        Some(comp) => transform_composition(comp, &mut nodes, &mut edges, &mut id_gen)?,
        None => {
            // Empty document — create a minimal root
            let id = id_gen.next();
            nodes.push(Node::point(&id));
            id
        }
    };

    Ok(Gir {
        ul_gir: "0.1.0".to_string(),
        root: root_id,
        nodes,
        edges,
        metadata: None,
    })
}

/// Transform a composition. Returns the ID of the "representative" node for this composition.
/// If the composition has multiple top-level terms, an implicit root enclosure is created.
fn transform_composition(
    comp: &AstComposition,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Edge>,
    id_gen: &mut IdGen,
) -> UlResult<String> {
    // Transform the head term
    let head_id = transform_term(&comp.head, nodes, edges, id_gen)?;

    if comp.tail.is_empty() {
        // Single term — it is its own representative
        return Ok(head_id);
    }

    // Multiple terms connected by operators.
    // Collect all term IDs, then create cross-edges.
    let mut term_ids = vec![head_id.clone()];

    for (_, term) in &comp.tail {
        let term_id = transform_term(term, nodes, edges, id_gen)?;
        term_ids.push(term_id);
    }

    // Create cross-edges between consecutive terms based on operators
    for (i, (op, _)) in comp.tail.iter().enumerate() {
        let left = &term_ids[i];
        let right = &term_ids[i + 1];

        match op {
            AstOperator::Adjacency => {
                edges.push(Edge::new(left, right, EdgeType::Adjacent));
            }
            AstOperator::Intersection => {
                edges.push(Edge::new(left, right, EdgeType::Intersects));
            }
            AstOperator::Connection { direction, angle } => {
                // Create a line node for the connection
                let line_id = id_gen.next();
                let directed = matches!(direction, AstDirection::Right | AstDirection::Left);

                let dir_vec = match direction {
                    AstDirection::Right => Some([1.0, 0.0]),
                    AstDirection::Left => Some([-1.0, 0.0]),
                    AstDirection::Both => None,
                };

                let mut line = Node::line(&line_id, directed);
                if let Some(d) = dir_vec {
                    line.direction = Some(d);
                }

                nodes.push(line);

                // connects: left → line → right
                edges.push(Edge::new(left, &line_id, EdgeType::Connects));
                edges.push(Edge::new(&line_id, right, EdgeType::Connects));

                // If there's an angle modifier, create an angle node
                if let Some(degrees) = angle {
                    let angle_id = id_gen.next();
                    nodes.push(Node::angle(&angle_id, *degrees));
                    edges.push(Edge::new(&line_id, &angle_id, EdgeType::ModifiedBy));
                }
            }
        }
    }

    // Rule 5: Multiple top-level terms → create implicit root enclosure
    let root_id = id_gen.next();
    let root = Node::enclosure(&root_id, EnclosureShape::Circle)
        .with_label("_implicit_root")
        .with_sort(Sort::Entity);
    nodes.push(root);

    for tid in &term_ids {
        edges.push(Edge::contains(&root_id, tid));
    }

    // Also contain all line nodes that were created for connections
    // (they're already in the nodes vec but need containment)
    // The line nodes are implicitly within the root composition context
    // We find them by checking which nodes are lines not already contained
    let contained: std::collections::HashSet<&str> = edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Contains)
        .map(|e| e.target.as_str())
        .collect();

    let line_ids: Vec<String> = nodes
        .iter()
        .filter(|n| {
            n.node_type == NodeType::Line && !contained.contains(n.id.as_str()) && n.id != root_id
        })
        .map(|n| n.id.clone())
        .collect();

    for lid in line_ids {
        edges.push(Edge::contains(&root_id, &lid));
    }

    // Contain angle modifier nodes too
    let contained2: std::collections::HashSet<&str> = edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Contains)
        .map(|e| e.target.as_str())
        .collect();

    let angle_ids: Vec<String> = nodes
        .iter()
        .filter(|n| {
            n.node_type == NodeType::Angle && !contained2.contains(n.id.as_str()) && n.id != root_id
        })
        .map(|n| n.id.clone())
        .collect();

    for aid in angle_ids {
        edges.push(Edge::contains(&root_id, &aid));
    }

    Ok(root_id)
}

/// Transform a single term. Returns the ID of its representative node.
fn transform_term(
    term: &AstTerm,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Edge>,
    id_gen: &mut IdGen,
) -> UlResult<String> {
    match term {
        AstTerm::Mark(mark) => transform_mark(mark, nodes, edges, id_gen),
        AstTerm::Group(comp) => transform_composition(comp, nodes, edges, id_gen),
    }
}

/// Transform a mark (primitive + optional content).
fn transform_mark(
    mark: &AstMark,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Edge>,
    id_gen: &mut IdGen,
) -> UlResult<String> {
    let id = id_gen.next();

    // Rule 1: Primitive → Node
    let node = match mark.primitive {
        AstPrimitive::Point => Node::point(&id),
        AstPrimitive::Circle => Node::enclosure(&id, EnclosureShape::Circle),
        AstPrimitive::Triangle => Node::enclosure(&id, EnclosureShape::Triangle),
        AstPrimitive::Square => Node::enclosure(&id, EnclosureShape::Square),
        AstPrimitive::RightArrow => {
            let mut n = Node::line(&id, true);
            n.direction = Some([1.0, 0.0]);
            n
        }
        AstPrimitive::LeftArrow => {
            let mut n = Node::line(&id, true);
            n.direction = Some([-1.0, 0.0]);
            n
        }
        AstPrimitive::BiArrow => Node::line(&id, false),
        AstPrimitive::Curve => Node::curve(&id, 0.5),
        AstPrimitive::Angle(deg) => Node::angle(&id, deg),
    };

    nodes.push(node);

    // Rule 2: Content braces → contains edges
    if let Some(content) = &mark.content {
        // The content is a composition; each top-level term becomes a child
        let child_ids = transform_content_children(content, nodes, edges, id_gen)?;
        for child_id in child_ids {
            edges.push(Edge::contains(&id, &child_id));
        }
    }

    Ok(id)
}

/// Transform composition children for containment. Returns IDs of all direct children.
fn transform_content_children(
    comp: &AstComposition,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Edge>,
    id_gen: &mut IdGen,
) -> UlResult<Vec<String>> {
    let head_id = transform_term(&comp.head, nodes, edges, id_gen)?;
    let mut child_ids = vec![head_id.clone()];

    if comp.tail.is_empty() {
        return Ok(child_ids);
    }

    // Multiple children with operators between them
    for (i, (op, term)) in comp.tail.iter().enumerate() {
        let term_id = transform_term(term, nodes, edges, id_gen)?;
        child_ids.push(term_id.clone());

        // Clone to avoid borrowing child_ids across push
        let left = if i == 0 {
            head_id.clone()
        } else {
            child_ids[i].clone()
        };
        let right = term_id;

        match op {
            AstOperator::Adjacency => {
                edges.push(Edge::new(&left, &right, EdgeType::Adjacent));
            }
            AstOperator::Intersection => {
                edges.push(Edge::new(&left, &right, EdgeType::Intersects));
            }
            AstOperator::Connection { direction, angle } => {
                let line_id = id_gen.next();
                let directed = matches!(direction, AstDirection::Right | AstDirection::Left);
                let dir_vec = match direction {
                    AstDirection::Right => Some([1.0, 0.0]),
                    AstDirection::Left => Some([-1.0, 0.0]),
                    AstDirection::Both => None,
                };
                let mut line = Node::line(&line_id, directed);
                if let Some(d) = dir_vec {
                    line.direction = Some(d);
                }
                nodes.push(line);
                child_ids.push(line_id.clone());

                edges.push(Edge::new(&left, &line_id, EdgeType::Connects));
                edges.push(Edge::new(&line_id, &right, EdgeType::Connects));

                if let Some(degrees) = angle {
                    let angle_id = id_gen.next();
                    nodes.push(Node::angle(&angle_id, *degrees));
                    edges.push(Edge::new(&line_id, &angle_id, EdgeType::ModifiedBy));
                    child_ids.push(angle_id);
                }
            }
        }
    }

    Ok(child_ids)
}
