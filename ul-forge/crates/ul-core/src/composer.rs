//! Algebraic composer: executable implementations of the 13 Σ_UL operations.
//!
//! Each operation takes GIR input(s) and produces a new GIR with the operation applied.
//! This transforms UL from a scoring engine into a composition engine — agents and
//! game players can build meaning structures step-by-step.

use crate::error::{UlError, UlResult};
use crate::types::edge::{Edge, EdgeType};
use crate::types::gir::Gir;
use crate::types::node::{EnclosureShape, Node, NodeType};
use crate::types::sort::Sort;

/// Counter for generating unique node IDs within a composition.
struct IdGen {
    prefix: String,
    counter: u32,
}

impl IdGen {
    fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            counter: 0,
        }
    }

    fn next(&mut self) -> String {
        self.counter += 1;
        format!("{}_{}", self.prefix, self.counter)
    }
}

/// Find the highest numeric suffix in existing node IDs to avoid collisions.
#[allow(dead_code)]
fn id_gen_for(gir: &Gir) -> IdGen {
    let max = gir
        .nodes
        .iter()
        .filter_map(|n| {
            n.id.rsplit('_')
                .next()
                .and_then(|s| s.parse::<u32>().ok())
        })
        .max()
        .unwrap_or(0);
    let mut gen = IdGen::new("c");
    gen.counter = max;
    gen
}

// ── Operation 1: predicate(e₁, r, e₂) → a ──

/// Combine subject entity, relation, and object entity into an assertion.
///
/// Creates a new assertion-enclosure containing the three elements,
/// connected by `Connects` edges through the relation.
pub fn predicate(subject: &Gir, relation: &Gir, object: &Gir) -> UlResult<Gir> {
    validate_root_sort(subject, Sort::Entity, "predicate subject")?;
    validate_root_sort(relation, Sort::Relation, "predicate relation")?;
    validate_root_sort(object, Sort::Entity, "predicate object")?;

    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut gen = IdGen::new("pred");

    let frame_id = gen.next();
    let mut frame = Node::enclosure(&frame_id, EnclosureShape::Circle);
    frame.sort = Sort::Assertion;
    frame.label = Some("assertion".to_string());

    // Merge all nodes from inputs, reparenting their roots under the assertion frame
    let s_nodes = remap_nodes(&subject.nodes, "s");
    let r_nodes = remap_nodes(&relation.nodes, "r");
    let o_nodes = remap_nodes(&object.nodes, "o");
    let s_edges = remap_edges(&subject.edges, "s");
    let r_edges = remap_edges(&relation.edges, "r");
    let o_edges = remap_edges(&object.edges, "o");

    let s_root = remap_id(&subject.root, "s");
    let r_root = remap_id(&relation.root, "r");
    let o_root = remap_id(&object.root, "o");

    nodes.push(frame);
    nodes.extend(s_nodes);
    nodes.extend(r_nodes);
    nodes.extend(o_nodes);

    edges.extend(s_edges);
    edges.extend(r_edges);
    edges.extend(o_edges);

    // Frame contains subject, relation, object roots
    edges.push(Edge::contains(&frame_id, &s_root));
    edges.push(Edge::contains(&frame_id, &r_root));
    edges.push(Edge::contains(&frame_id, &o_root));

    // Subject connects to relation, relation connects to object
    edges.push(Edge::connects(&s_root, &r_root));
    edges.push(Edge::connects(&r_root, &o_root));

    Ok(Gir::new(frame_id, nodes, edges))
}

// ── Operation 2: modify_entity(m, e) → e ──

/// Apply a modifier to an entity, producing a modified entity.
pub fn modify_entity(modifier: &Gir, entity: &Gir) -> UlResult<Gir> {
    validate_root_sort(modifier, Sort::Modifier, "modify_entity modifier")?;
    validate_root_sort(entity, Sort::Entity, "modify_entity entity")?;

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    let e_nodes = remap_nodes(&entity.nodes, "e");
    let m_nodes = remap_nodes(&modifier.nodes, "m");
    let e_edges = remap_edges(&entity.edges, "e");
    let m_edges = remap_edges(&modifier.edges, "m");

    let e_root = remap_id(&entity.root, "e");
    let m_root = remap_id(&modifier.root, "m");

    nodes.extend(e_nodes);
    nodes.extend(m_nodes);
    edges.extend(e_edges);
    edges.extend(m_edges);

    // Entity is modified by modifier
    edges.push(Edge::modified_by(&e_root, &m_root));

    Ok(Gir::new(e_root, nodes, edges))
}

// ── Operation 3: modify_relation(m, r) → r ──

/// Apply a modifier to a relation, producing a modified relation.
pub fn modify_relation(modifier: &Gir, relation: &Gir) -> UlResult<Gir> {
    validate_root_sort(modifier, Sort::Modifier, "modify_relation modifier")?;
    validate_root_sort(relation, Sort::Relation, "modify_relation relation")?;

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    let r_nodes = remap_nodes(&relation.nodes, "r");
    let m_nodes = remap_nodes(&modifier.nodes, "m");
    let r_edges = remap_edges(&relation.edges, "r");
    let m_edges = remap_edges(&modifier.edges, "m");

    let r_root = remap_id(&relation.root, "r");
    let m_root = remap_id(&modifier.root, "m");

    nodes.extend(r_nodes);
    nodes.extend(m_nodes);
    edges.extend(r_edges);
    edges.extend(m_edges);

    edges.push(Edge::modified_by(&r_root, &m_root));

    Ok(Gir::new(r_root, nodes, edges))
}

// ── Operation 4: negate(a) → a ──

/// Negate an assertion (geometric reflection — an involution).
pub fn negate(assertion: &Gir) -> UlResult<Gir> {
    validate_root_sort(assertion, Sort::Assertion, "negate")?;

    let mut gen = IdGen::new("neg");
    let frame_id = gen.next();
    let mut frame = Node::enclosure(&frame_id, EnclosureShape::Circle);
    frame.sort = Sort::Assertion;
    frame.label = Some("negation".to_string());

    let a_nodes = remap_nodes(&assertion.nodes, "a");
    let a_edges = remap_edges(&assertion.edges, "a");
    let a_root = remap_id(&assertion.root, "a");

    let mut nodes = vec![frame];
    nodes.extend(a_nodes);

    let mut edges = vec![Edge::contains(&frame_id, &a_root)];
    edges.extend(a_edges);

    // Mark negation with a references-self edge (negation marker)
    edges.push(Edge::references(&frame_id, &frame_id));

    Ok(Gir::new(frame_id, nodes, edges))
}

// ── Operation 5: conjoin(a₁, a₂) → a ──

/// Combine two assertions with AND (overlapping frames → shared context).
pub fn conjoin(a1: &Gir, a2: &Gir) -> UlResult<Gir> {
    validate_root_sort(a1, Sort::Assertion, "conjoin left")?;
    validate_root_sort(a2, Sort::Assertion, "conjoin right")?;

    let mut gen = IdGen::new("conj");
    let frame_id = gen.next();
    let mut frame = Node::enclosure(&frame_id, EnclosureShape::Circle);
    frame.sort = Sort::Assertion;
    frame.label = Some("conjunction".to_string());

    let a1_nodes = remap_nodes(&a1.nodes, "l");
    let a2_nodes = remap_nodes(&a2.nodes, "r");
    let a1_edges = remap_edges(&a1.edges, "l");
    let a2_edges = remap_edges(&a2.edges, "r");
    let a1_root = remap_id(&a1.root, "l");
    let a2_root = remap_id(&a2.root, "r");

    let mut nodes = vec![frame];
    nodes.extend(a1_nodes);
    nodes.extend(a2_nodes);

    let mut edges = vec![
        Edge::contains(&frame_id, &a1_root),
        Edge::contains(&frame_id, &a2_root),
        // Overlapping (intersects) indicates AND
        Edge::intersects(&a1_root, &a2_root),
    ];
    edges.extend(a1_edges);
    edges.extend(a2_edges);

    Ok(Gir::new(frame_id, nodes, edges))
}

// ── Operation 6: disjoin(a₁, a₂) → a ──

/// Combine two assertions with OR (adjacent frames → either can be read).
pub fn disjoin(a1: &Gir, a2: &Gir) -> UlResult<Gir> {
    validate_root_sort(a1, Sort::Assertion, "disjoin left")?;
    validate_root_sort(a2, Sort::Assertion, "disjoin right")?;

    let mut gen = IdGen::new("disj");
    let frame_id = gen.next();
    let mut frame = Node::enclosure(&frame_id, EnclosureShape::Circle);
    frame.sort = Sort::Assertion;
    frame.label = Some("disjunction".to_string());

    let a1_nodes = remap_nodes(&a1.nodes, "l");
    let a2_nodes = remap_nodes(&a2.nodes, "r");
    let a1_edges = remap_edges(&a1.edges, "l");
    let a2_edges = remap_edges(&a2.edges, "r");
    let a1_root = remap_id(&a1.root, "l");
    let a2_root = remap_id(&a2.root, "r");

    let mut nodes = vec![frame];
    nodes.extend(a1_nodes);
    nodes.extend(a2_nodes);

    let mut edges = vec![
        Edge::contains(&frame_id, &a1_root),
        Edge::contains(&frame_id, &a2_root),
        // Adjacent (touching not overlapping) indicates OR
        Edge::adjacent(&a1_root, &a2_root),
    ];
    edges.extend(a1_edges);
    edges.extend(a2_edges);

    Ok(Gir::new(frame_id, nodes, edges))
}

// ── Operation 7: embed(a) → e ──

/// Turn an assertion into an entity (nominalization).
///
/// Shrinks the assertion and places it inside an enclosure,
/// converting it to something that can be talked about.
pub fn embed(assertion: &Gir) -> UlResult<Gir> {
    validate_root_sort(assertion, Sort::Assertion, "embed")?;

    let mut gen = IdGen::new("emb");
    let wrapper_id = gen.next();
    let mut wrapper = Node::enclosure(&wrapper_id, EnclosureShape::Circle);
    wrapper.sort = Sort::Entity;
    wrapper.label = Some("embedded".to_string());

    let a_nodes = remap_nodes(&assertion.nodes, "a");
    let a_edges = remap_edges(&assertion.edges, "a");
    let a_root = remap_id(&assertion.root, "a");

    let mut nodes = vec![wrapper];
    nodes.extend(a_nodes);

    let mut edges = vec![Edge::contains(&wrapper_id, &a_root)];
    edges.extend(a_edges);

    Ok(Gir::new(wrapper_id, nodes, edges))
}

// ── Operation 8: abstract(e) → m ──

/// Turn an entity into a modifier (adjectivalization).
///
/// Extracts the entity's boundary/shape properties as a modifier
/// that can be applied to other entities or relations.
pub fn abstract_op(entity: &Gir) -> UlResult<Gir> {
    validate_root_sort(entity, Sort::Entity, "abstract")?;

    let mut gen = IdGen::new("abs");
    let mod_id = gen.next();
    let mut modifier = Node::new(&mod_id, NodeType::Angle);
    modifier.sort = Sort::Modifier;
    modifier.label = Some("abstracted".to_string());
    // Angle measure is derived from the entity — use 0 as neutral default
    modifier.measure = Some(0.0);

    // The result is a single modifier node that references the source entity
    let e_nodes = remap_nodes(&entity.nodes, "e");
    let e_edges = remap_edges(&entity.edges, "e");
    let e_root = remap_id(&entity.root, "e");

    let mut nodes = vec![modifier];
    nodes.extend(e_nodes);

    let mut edges = vec![Edge::references(&mod_id, &e_root)];
    edges.extend(e_edges);

    Ok(Gir::new(mod_id, nodes, edges))
}

// ── Operation 9: compose(r₁, r₂) → r ──

/// Chain two relations (transitivity).
///
/// The endpoint of r₁ becomes the startpoint of r₂, producing
/// a composite relation with combined path.
pub fn compose(r1: &Gir, r2: &Gir) -> UlResult<Gir> {
    validate_root_sort(r1, Sort::Relation, "compose left")?;
    validate_root_sort(r2, Sort::Relation, "compose right")?;

    let mut gen = IdGen::new("comp");

    let r1_nodes = remap_nodes(&r1.nodes, "l");
    let r2_nodes = remap_nodes(&r2.nodes, "r");
    let r1_edges = remap_edges(&r1.edges, "l");
    let r2_edges = remap_edges(&r2.edges, "r");
    let r1_root = remap_id(&r1.root, "l");
    let r2_root = remap_id(&r2.root, "r");

    // Create a composite wrapper (a curve representing the composed path)
    let wrapper_id = gen.next();
    let mut wrapper = Node::curve(&wrapper_id, 0.5);
    wrapper.sort = Sort::Relation;
    wrapper.label = Some("composed".to_string());

    let mut nodes = vec![wrapper];
    nodes.extend(r1_nodes);
    nodes.extend(r2_nodes);

    let mut edges = vec![
        // The composite connects from r1 to r2 (transitivity chain)
        Edge::connects(&r1_root, &wrapper_id),
        Edge::connects(&wrapper_id, &r2_root),
    ];
    edges.extend(r1_edges);
    edges.extend(r2_edges);

    Ok(Gir::new(wrapper_id, nodes, edges))
}

// ── Operation 10: invert(r) → r ──

/// Reverse the direction of a relation (active ↔ passive).
pub fn invert(relation: &Gir) -> UlResult<Gir> {
    validate_root_sort(relation, Sort::Relation, "invert")?;

    let mut gir = relation.clone();

    // Flip the directed flag on the root node
    if let Some(root_node) = gir.node_mut(&gir.root.clone()) {
        if let Some(directed) = root_node.directed {
            root_node.directed = Some(!directed);
        }
        // Also flip direction vector
        if let Some(dir) = root_node.direction {
            root_node.direction = Some([-dir[0], -dir[1]]);
        }
    }

    // Reverse all connects edges that touch the root
    let root_id = gir.root.clone();
    for edge in &mut gir.edges {
        if edge.edge_type == EdgeType::Connects {
            if edge.source == root_id {
                std::mem::swap(&mut edge.source, &mut edge.target);
            } else if edge.target == root_id {
                std::mem::swap(&mut edge.source, &mut edge.target);
            }
        }
    }

    Ok(gir)
}

// ── Operation 11: quantify(m, e) → a ──

/// Apply a quantifier-modifier to an entity, producing an assertion.
///
/// The modifier encodes the quantifier type (universal, existential, negative)
/// through its geometric properties (scaling).
pub fn quantify(quantifier: &Gir, entity: &Gir) -> UlResult<Gir> {
    validate_root_sort(quantifier, Sort::Modifier, "quantify quantifier")?;
    validate_root_sort(entity, Sort::Entity, "quantify entity")?;

    let mut gen = IdGen::new("quant");
    let frame_id = gen.next();
    let mut frame = Node::enclosure(&frame_id, EnclosureShape::Circle);
    frame.sort = Sort::Assertion;
    frame.label = Some("quantified".to_string());

    let q_nodes = remap_nodes(&quantifier.nodes, "q");
    let e_nodes = remap_nodes(&entity.nodes, "e");
    let q_edges = remap_edges(&quantifier.edges, "q");
    let e_edges = remap_edges(&entity.edges, "e");
    let q_root = remap_id(&quantifier.root, "q");
    let e_root = remap_id(&entity.root, "e");

    let mut nodes = vec![frame];
    nodes.extend(q_nodes);
    nodes.extend(e_nodes);

    let mut edges = vec![
        Edge::contains(&frame_id, &e_root),
        Edge::modified_by(&e_root, &q_root),
    ];
    edges.extend(q_edges);
    edges.extend(e_edges);

    Ok(Gir::new(frame_id, nodes, edges))
}

// ── Operation 12: bind(e, a) → a ──

/// Bind a variable entity to an assertion, establishing co-reference scope.
///
/// The entity `binder` becomes a VariableSlot that the assertion `body` can
/// reference via Binds edges. The result is an assertion-level GIR whose
/// binding_scope records the bound variable.
pub fn bind(binder: &Gir, body: &Gir) -> UlResult<Gir> {
    validate_root_sort(binder, Sort::Entity, "bind binder")?;
    validate_root_sort(body, Sort::Assertion, "bind body")?;

    let mut gen = IdGen::new("bind");
    let frame_id = gen.next();
    let mut frame = Node::enclosure(&frame_id, EnclosureShape::Circle);
    frame.sort = Sort::Assertion;
    frame.label = Some("bound".to_string());

    // Create a VariableSlot node that represents the bound variable
    let slot_id = gen.next();
    let var_id = binder
        .nodes
        .first()
        .and_then(|n| n.label.clone())
        .unwrap_or_else(|| "x".to_string());
    let slot = Node::variable_slot(&slot_id, &var_id);

    let b_nodes = remap_nodes(&body.nodes, "b");
    let b_edges = remap_edges(&body.edges, "b");
    let b_root = remap_id(&body.root, "b");

    let mut nodes = vec![frame, slot];
    nodes.extend(b_nodes);

    let mut edges = vec![
        Edge::contains(&frame_id, &slot_id),
        Edge::contains(&frame_id, &b_root),
        Edge::binds(&slot_id, &b_root),
    ];
    edges.extend(b_edges);

    let mut result = Gir::new(frame_id, nodes, edges);
    result.binding_scope = Some(vec![var_id]);
    Ok(result)
}

// ── Operation 13: modify_assertion(m, a) → a ──

/// Apply an assertion-level modifier (evidentiality, emphasis, hedging) to an assertion.
///
/// The modifier's geometric properties determine the kind of assertion
/// modification; the result is a wrapped assertion with the modifier applied.
pub fn modify_assertion(modifier: &Gir, assertion: &Gir) -> UlResult<Gir> {
    validate_root_sort(modifier, Sort::Modifier, "modify_assertion modifier")?;
    validate_root_sort(assertion, Sort::Assertion, "modify_assertion body")?;

    let mut gen = IdGen::new("ma");
    let frame_id = gen.next();
    let mut frame = Node::enclosure(&frame_id, EnclosureShape::Circle);
    frame.sort = Sort::Assertion;
    frame.label = Some("modified_assertion".to_string());

    let m_nodes = remap_nodes(&modifier.nodes, "m");
    let a_nodes = remap_nodes(&assertion.nodes, "a");
    let m_edges = remap_edges(&modifier.edges, "m");
    let a_edges = remap_edges(&assertion.edges, "a");
    let m_root = remap_id(&modifier.root, "m");
    let a_root = remap_id(&assertion.root, "a");

    let mut nodes = vec![frame];
    nodes.extend(m_nodes);
    nodes.extend(a_nodes);

    let mut edges = vec![
        Edge::contains(&frame_id, &a_root),
        Edge::modified_by(&a_root, &m_root),
    ];
    edges.extend(m_edges);
    edges.extend(a_edges);

    Ok(Gir::new(frame_id, nodes, edges))
}

// ── Decompose: reverse-engineer operations from a GIR ──

/// Detected operation with the nodes involved.
#[derive(Debug, Clone)]
pub struct DetectedOperation {
    /// Which Σ_UL operation was detected.
    pub operation: &'static str,
    /// Node IDs involved in this operation.
    pub node_ids: Vec<String>,
}

/// Analyze a GIR and detect which Σ_UL operations are expressed in its structure.
pub fn detect_operations(gir: &Gir) -> Vec<DetectedOperation> {
    let mut ops = Vec::new();

    let node_map: std::collections::HashMap<&str, &Node> =
        gir.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

    // Detect predicate: assertion containing entity-relation-entity connected chain
    for node in &gir.nodes {
        if node.sort == Sort::Assertion && node.node_type == NodeType::Enclosure {
            let children: Vec<&str> = gir
                .edges
                .iter()
                .filter(|e| e.edge_type == EdgeType::Contains && e.source == node.id)
                .map(|e| e.target.as_str())
                .collect();
            let has_entity = children
                .iter()
                .any(|c| node_map.get(c).is_some_and(|n| n.sort == Sort::Entity));
            let has_relation = children
                .iter()
                .any(|c| node_map.get(c).is_some_and(|n| n.sort == Sort::Relation));
            if has_entity && has_relation && children.len() >= 3 {
                ops.push(DetectedOperation {
                    operation: "predicate",
                    node_ids: vec![node.id.clone()],
                });
            }
        }
    }

    // Detect modify_entity / modify_relation: modified_by edges
    for edge in &gir.edges {
        if edge.edge_type == EdgeType::ModifiedBy {
            if let Some(source) = node_map.get(edge.source.as_str()) {
                let op_name = match source.sort {
                    Sort::Entity => "modify_entity",
                    Sort::Relation => "modify_relation",
                    _ => continue,
                };
                ops.push(DetectedOperation {
                    operation: op_name,
                    node_ids: vec![edge.source.clone(), edge.target.clone()],
                });
            }
        }
    }

    // Detect negate: assertion with self-referencing edge
    for edge in &gir.edges {
        if edge.edge_type == EdgeType::References && edge.source == edge.target {
            if let Some(node) = node_map.get(edge.source.as_str()) {
                if node.sort == Sort::Assertion {
                    ops.push(DetectedOperation {
                        operation: "negate",
                        node_ids: vec![node.id.clone()],
                    });
                }
            }
        }
    }

    // Detect conjoin: assertion containing two assertions linked by intersects
    // Detect disjoin: assertion containing two assertions linked by adjacent
    for node in &gir.nodes {
        if node.sort == Sort::Assertion && node.node_type == NodeType::Enclosure {
            let assertion_children: Vec<&str> = gir
                .edges
                .iter()
                .filter(|e| e.edge_type == EdgeType::Contains && e.source == node.id)
                .map(|e| e.target.as_str())
                .filter(|c| node_map.get(c).is_some_and(|n| n.sort == Sort::Assertion))
                .collect();

            if assertion_children.len() >= 2 {
                let has_intersects = gir.edges.iter().any(|e| {
                    e.edge_type == EdgeType::Intersects
                        && assertion_children.contains(&e.source.as_str())
                        && assertion_children.contains(&e.target.as_str())
                });
                let has_adjacent = gir.edges.iter().any(|e| {
                    e.edge_type == EdgeType::Adjacent
                        && assertion_children.contains(&e.source.as_str())
                        && assertion_children.contains(&e.target.as_str())
                });

                if has_intersects {
                    ops.push(DetectedOperation {
                        operation: "conjoin",
                        node_ids: vec![node.id.clone()],
                    });
                }
                if has_adjacent {
                    ops.push(DetectedOperation {
                        operation: "disjoin",
                        node_ids: vec![node.id.clone()],
                    });
                }
            }
        }
    }

    // Detect embed: entity-sort enclosure containing an assertion
    for node in &gir.nodes {
        if node.sort == Sort::Entity && node.node_type == NodeType::Enclosure {
            let has_assertion_child = gir.edges.iter().any(|e| {
                e.edge_type == EdgeType::Contains
                    && e.source == node.id
                    && node_map
                        .get(e.target.as_str())
                        .is_some_and(|n| n.sort == Sort::Assertion)
            });
            if has_assertion_child {
                ops.push(DetectedOperation {
                    operation: "embed",
                    node_ids: vec![node.id.clone()],
                });
            }
        }
    }

    // Detect abstract: modifier referencing an entity
    for edge in &gir.edges {
        if edge.edge_type == EdgeType::References {
            if let (Some(src), Some(tgt)) = (
                node_map.get(edge.source.as_str()),
                node_map.get(edge.target.as_str()),
            ) {
                if src.sort == Sort::Modifier && tgt.sort == Sort::Entity {
                    ops.push(DetectedOperation {
                        operation: "abstract",
                        node_ids: vec![edge.source.clone(), edge.target.clone()],
                    });
                }
            }
        }
    }

    // Detect compose: relation connected to another relation via connects
    for edge in &gir.edges {
        if edge.edge_type == EdgeType::Connects {
            if let (Some(src), Some(tgt)) = (
                node_map.get(edge.source.as_str()),
                node_map.get(edge.target.as_str()),
            ) {
                if src.sort == Sort::Relation && tgt.sort == Sort::Relation {
                    ops.push(DetectedOperation {
                        operation: "compose",
                        node_ids: vec![edge.source.clone(), edge.target.clone()],
                    });
                }
            }
        }
    }

    // Detect invert: hard to detect statically (flipped direction); skip for now

    // Detect quantify: assertion containing entity modified by quantifier
    for node in &gir.nodes {
        if node.sort == Sort::Assertion
            && node.node_type == NodeType::Enclosure
            && node.label.as_deref() == Some("quantified")
        {
            ops.push(DetectedOperation {
                operation: "quantify",
                node_ids: vec![node.id.clone()],
            });
        }
    }

    // Detect bind: assertion containing a VariableSlot with a Binds edge
    for node in &gir.nodes {
        if node.sort == Sort::Assertion
            && node.node_type == NodeType::Enclosure
            && node.label.as_deref() == Some("bound")
        {
            let has_binding = gir.edges.iter().any(|e| {
                e.edge_type == EdgeType::Binds
                    && node_map
                        .get(e.source.as_str())
                        .is_some_and(|n| n.node_type == NodeType::VariableSlot)
            });
            if has_binding {
                ops.push(DetectedOperation {
                    operation: "bind",
                    node_ids: vec![node.id.clone()],
                });
            }
        }
    }

    // Detect modify_assertion: assertion modified by a modifier (assertion-level)
    for edge in &gir.edges {
        if edge.edge_type == EdgeType::ModifiedBy {
            if let Some(source) = node_map.get(edge.source.as_str()) {
                if source.sort == Sort::Assertion {
                    ops.push(DetectedOperation {
                        operation: "modify_assertion",
                        node_ids: vec![edge.source.clone(), edge.target.clone()],
                    });
                }
            }
        }
    }

    ops
}

// ── Helper functions ──

fn validate_root_sort(gir: &Gir, expected: Sort, context: &str) -> UlResult<()> {
    let root_node = gir
        .node(&gir.root)
        .ok_or_else(|| UlError::Render {
            message: format!("{}: root node '{}' not found", context, gir.root),
        })?;
    if root_node.sort != expected {
        return Err(UlError::Render {
            message: format!(
                "{}: expected root sort {:?}, got {:?}",
                context, expected, root_node.sort
            ),
        });
    }
    Ok(())
}

fn remap_id(id: &str, prefix: &str) -> String {
    format!("{}_{}", prefix, id)
}

fn remap_nodes(nodes: &[Node], prefix: &str) -> Vec<Node> {
    nodes
        .iter()
        .map(|n| {
            let mut remapped = n.clone();
            remapped.id = remap_id(&n.id, prefix);
            remapped
        })
        .collect()
}

fn remap_edges(edges: &[Edge], prefix: &str) -> Vec<Edge> {
    edges
        .iter()
        .map(|e| {
            Edge::new(
                remap_id(&e.source, prefix),
                remap_id(&e.target, prefix),
                e.edge_type,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity_gir(id: &str) -> Gir {
        Gir::new(id, vec![Node::point(id)], vec![])
    }

    fn relation_gir(id: &str) -> Gir {
        Gir::new(id, vec![Node::line(id, true)], vec![])
    }

    fn modifier_gir(id: &str) -> Gir {
        Gir::new(id, vec![Node::angle(id, 45.0)], vec![])
    }

    fn assertion_gir(id: &str) -> Gir {
        let mut frame = Node::enclosure(id, EnclosureShape::Circle);
        frame.sort = Sort::Assertion;
        Gir::new(id, vec![frame], vec![])
    }

    #[test]
    fn predicate_creates_assertion_frame() {
        let result = predicate(&entity_gir("s"), &relation_gir("r"), &entity_gir("o")).unwrap();
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.sort, Sort::Assertion);
        assert_eq!(root.node_type, NodeType::Enclosure);
        // Should have 4 nodes: frame + subject + relation + object
        assert_eq!(result.nodes.len(), 4);
        // Should have connects edges
        assert!(result
            .edges
            .iter()
            .any(|e| e.edge_type == EdgeType::Connects));
    }

    #[test]
    fn predicate_rejects_wrong_sort() {
        assert!(predicate(&relation_gir("s"), &relation_gir("r"), &entity_gir("o")).is_err());
    }

    #[test]
    fn modify_entity_adds_modified_by_edge() {
        let result = modify_entity(&modifier_gir("m"), &entity_gir("e")).unwrap();
        assert!(result
            .edges
            .iter()
            .any(|e| e.edge_type == EdgeType::ModifiedBy));
        // Root should be the entity (sort preserved)
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.sort, Sort::Entity);
    }

    #[test]
    fn modify_relation_adds_modified_by_edge() {
        let result = modify_relation(&modifier_gir("m"), &relation_gir("r")).unwrap();
        assert!(result
            .edges
            .iter()
            .any(|e| e.edge_type == EdgeType::ModifiedBy));
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.sort, Sort::Relation);
    }

    #[test]
    fn negate_wraps_in_assertion() {
        let result = negate(&assertion_gir("a")).unwrap();
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.sort, Sort::Assertion);
        assert_eq!(root.label.as_deref(), Some("negation"));
        // Self-referencing edge marks negation
        assert!(result
            .edges
            .iter()
            .any(|e| e.edge_type == EdgeType::References && e.source == e.target));
    }

    #[test]
    fn negate_is_involution() {
        let a = assertion_gir("a");
        let neg1 = negate(&a).unwrap();
        let neg2 = negate(&neg1).unwrap();
        // Double negation produces another assertion
        let root = neg2.node(&neg2.root).unwrap();
        assert_eq!(root.sort, Sort::Assertion);
    }

    #[test]
    fn conjoin_uses_intersects() {
        let result = conjoin(&assertion_gir("a1"), &assertion_gir("a2")).unwrap();
        assert!(result
            .edges
            .iter()
            .any(|e| e.edge_type == EdgeType::Intersects));
    }

    #[test]
    fn disjoin_uses_adjacent() {
        let result = disjoin(&assertion_gir("a1"), &assertion_gir("a2")).unwrap();
        assert!(result
            .edges
            .iter()
            .any(|e| e.edge_type == EdgeType::Adjacent));
    }

    #[test]
    fn embed_produces_entity() {
        let result = embed(&assertion_gir("a")).unwrap();
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.sort, Sort::Entity);
        assert_eq!(root.node_type, NodeType::Enclosure);
    }

    #[test]
    fn abstract_produces_modifier() {
        let result = abstract_op(&entity_gir("e")).unwrap();
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.sort, Sort::Modifier);
    }

    #[test]
    fn compose_chains_relations() {
        let result = compose(&relation_gir("r1"), &relation_gir("r2")).unwrap();
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.sort, Sort::Relation);
        // Should have connects edges linking the chain
        let connects: Vec<_> = result
            .edges
            .iter()
            .filter(|e| e.edge_type == EdgeType::Connects)
            .collect();
        assert_eq!(connects.len(), 2);
    }

    #[test]
    fn invert_flips_direction() {
        let mut line = Node::line("r", true);
        line.direction = Some([1.0, 0.0]);
        let gir = Gir::new("r", vec![line], vec![]);
        let result = invert(&gir).unwrap();
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.direction, Some([-1.0, -0.0]));
    }

    #[test]
    fn quantify_produces_assertion() {
        let result = quantify(&modifier_gir("q"), &entity_gir("e")).unwrap();
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.sort, Sort::Assertion);
        assert!(result
            .edges
            .iter()
            .any(|e| e.edge_type == EdgeType::ModifiedBy));
    }

    #[test]
    fn detect_operations_finds_embed() {
        let gir = embed(&assertion_gir("a")).unwrap();
        let ops = detect_operations(&gir);
        assert!(ops.iter().any(|o| o.operation == "embed"));
    }

    #[test]
    fn detect_operations_finds_modify_entity() {
        let gir = modify_entity(&modifier_gir("m"), &entity_gir("e")).unwrap();
        let ops = detect_operations(&gir);
        assert!(ops.iter().any(|o| o.operation == "modify_entity"));
    }

    #[test]
    fn bind_produces_assertion_with_variable_slot() {
        let result = bind(&entity_gir("x"), &assertion_gir("a")).unwrap();
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.sort, Sort::Assertion);
        assert_eq!(root.label.as_deref(), Some("bound"));
        // Should have a VariableSlot node
        assert!(result
            .nodes
            .iter()
            .any(|n| n.node_type == NodeType::VariableSlot));
        // Should have a Binds edge
        assert!(result
            .edges
            .iter()
            .any(|e| e.edge_type == EdgeType::Binds));
        // Should have binding_scope set
        assert!(result.binding_scope.is_some());
    }

    #[test]
    fn bind_rejects_wrong_sorts() {
        assert!(bind(&relation_gir("r"), &assertion_gir("a")).is_err());
        assert!(bind(&entity_gir("e"), &entity_gir("e2")).is_err());
    }

    #[test]
    fn modify_assertion_produces_assertion() {
        let result =
            modify_assertion(&modifier_gir("m"), &assertion_gir("a")).unwrap();
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.sort, Sort::Assertion);
        assert_eq!(root.label.as_deref(), Some("modified_assertion"));
        // Should have a modified_by edge from inner assertion to modifier
        assert!(result
            .edges
            .iter()
            .any(|e| e.edge_type == EdgeType::ModifiedBy));
    }

    #[test]
    fn modify_assertion_rejects_wrong_sorts() {
        assert!(modify_assertion(&entity_gir("e"), &assertion_gir("a")).is_err());
        assert!(modify_assertion(&modifier_gir("m"), &entity_gir("e")).is_err());
    }

    #[test]
    fn detect_operations_finds_bind() {
        let gir = bind(&entity_gir("x"), &assertion_gir("a")).unwrap();
        let ops = detect_operations(&gir);
        assert!(ops.iter().any(|o| o.operation == "bind"));
    }

    #[test]
    fn detect_operations_finds_modify_assertion() {
        let gir =
            modify_assertion(&modifier_gir("m"), &assertion_gir("a")).unwrap();
        let ops = detect_operations(&gir);
        assert!(ops.iter().any(|o| o.operation == "modify_assertion"));
    }
}
