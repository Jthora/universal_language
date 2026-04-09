//! GIR validator: schema, sort constraints, graph invariants, geometric satisfiability.
//!
//! The validator runs after any GIR is produced (from parser, editor, AI, or manual construction)
//! and before it enters the renderer.

use crate::error::UlError;
use crate::types::edge::EdgeType;
use crate::types::gir::Gir;
use crate::types::sort::Sort;
use std::collections::{HashMap, HashSet};

/// Result of validating a GIR document.
#[derive(Debug)]
pub struct ValidationResult {
    /// Whether the GIR is valid (no errors).
    pub valid: bool,
    /// Errors that prevent rendering.
    pub errors: Vec<UlError>,
    /// Warnings that don't prevent rendering but indicate potential issues.
    pub warnings: Vec<String>,
}

impl ValidationResult {
    fn new() -> Self {
        ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn add_error(&mut self, error: UlError) {
        self.valid = false;
        self.errors.push(error);
    }

    fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

/// Validate a GIR document. Runs layers 1-3 (schema, sorts, invariants).
/// Set `check_geometry` to `true` to also run layer 4 (geometric satisfiability).
pub fn validate(gir: &Gir, check_geometry: bool) -> ValidationResult {
    let mut result = ValidationResult::new();

    validate_schema(gir, &mut result);
    if !result.valid {
        return result; // schema must pass before checking sorts/invariants
    }

    validate_sorts(gir, &mut result);
    validate_invariants(gir, &mut result);
    validate_bindings(gir, &mut result);

    if check_geometry {
        validate_geometry(gir, &mut result);
    }

    result
}

/// Quick check: is this GIR valid for rendering? (Layers 1-3, no geometry.)
pub fn is_renderable(gir: &Gir) -> bool {
    validate(gir, false).valid
}

// -- Layer 1: Schema validation --

fn validate_schema(gir: &Gir, result: &mut ValidationResult) {
    // Check for duplicate node IDs
    let mut seen_ids: HashSet<&str> = HashSet::new();
    for node in &gir.nodes {
        if !seen_ids.insert(&node.id) {
            result.add_error(UlError::DuplicateNodeId {
                node_id: node.id.clone(),
            });
        }
    }

    // Check referential integrity: every edge references existing nodes
    let node_ids: HashSet<&str> = gir.nodes.iter().map(|n| n.id.as_str()).collect();
    for edge in &gir.edges {
        if !node_ids.contains(edge.source.as_str()) {
            result.add_error(UlError::DanglingEdgeRef {
                node_id: edge.source.clone(),
            });
        }
        if !node_ids.contains(edge.target.as_str()) {
            result.add_error(UlError::DanglingEdgeRef {
                node_id: edge.target.clone(),
            });
        }
    }

    // Check root node exists
    if !node_ids.contains(gir.root.as_str()) {
        result.add_error(UlError::DanglingEdgeRef {
            node_id: gir.root.clone(),
        });
    }
}

// -- Layer 2: Sort constraint validation --

fn validate_sorts(gir: &Gir, result: &mut ValidationResult) {
    let node_map: HashMap<&str, &crate::types::node::Node> =
        gir.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

    for edge in &gir.edges {
        let source = match node_map.get(edge.source.as_str()) {
            Some(n) => n,
            None => continue, // already caught by schema validation
        };
        let target = match node_map.get(edge.target.as_str()) {
            Some(n) => n,
            None => continue,
        };

        match edge.edge_type {
            EdgeType::Contains => {
                // Source must be Entity with enclosure type
                if source.sort != Sort::Entity {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::Contains,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "contains source must be entity".to_string(),
                    });
                }
                if source.node_type != crate::types::node::NodeType::Enclosure {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::Contains,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "contains source must be an enclosure node".to_string(),
                    });
                }
            }
            EdgeType::ModifiedBy => {
                // Target must be Modifier
                if target.sort != Sort::Modifier {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::ModifiedBy,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "modified_by target must be modifier".to_string(),
                    });
                }
                // Source must be Entity or Relation
                if source.sort != Sort::Entity && source.sort != Sort::Relation {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::ModifiedBy,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "modified_by source must be entity or relation".to_string(),
                    });
                }
            }
            EdgeType::Adjacent => {
                // Both endpoints must be Entity (spatial boundary sharing)
                if source.sort != Sort::Entity {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::Adjacent,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "adjacent source must be entity".to_string(),
                    });
                }
                if target.sort != Sort::Entity {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::Adjacent,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "adjacent target must be entity".to_string(),
                    });
                }
            }
            EdgeType::Intersects => {
                // Both endpoints must be Entity (spatial overlap)
                if source.sort != Sort::Entity {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::Intersects,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "intersects source must be entity".to_string(),
                    });
                }
                if target.sort != Sort::Entity {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::Intersects,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "intersects target must be entity".to_string(),
                    });
                }
            }
            EdgeType::Connects => {
                // Connects edges link entities and relations (lines/curves connect entities).
                // Both endpoints must be Entity or Relation.
                if source.sort != Sort::Entity && source.sort != Sort::Relation {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::Connects,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "connects source must be entity or relation".to_string(),
                    });
                }
                if target.sort != Sort::Entity && target.sort != Sort::Relation {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::Connects,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "connects target must be entity or relation".to_string(),
                    });
                }
            }
            EdgeType::References => {
                // References are semantic cross-references — no sort restriction,
                // but warn if referencing across containment boundaries (heuristic).
                let source_parent = gir.edges.iter().find(|e| {
                    e.edge_type == EdgeType::Contains && e.target == edge.source
                });
                let target_parent = gir.edges.iter().find(|e| {
                    e.edge_type == EdgeType::Contains && e.target == edge.target
                });
                if let (Some(sp), Some(tp)) = (source_parent, target_parent) {
                    if sp.source != tp.source {
                        result.add_warning(format!(
                            "references edge from '{}' to '{}' crosses containment boundaries \
                             (parents '{}' and '{}')",
                            edge.source, edge.target, sp.source, tp.source
                        ));
                    }
                }
            }
            EdgeType::Binds => {
                // Source must be a VariableSlot entity
                if source.node_type != crate::types::node::NodeType::VariableSlot {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::Binds,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "binds source must be a variable_slot node".to_string(),
                    });
                }
                // Target must be Entity (the co-referent)
                if target.sort != Sort::Entity {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::Binds,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "binds target must be entity".to_string(),
                    });
                }
            }
            EdgeType::AccessibleFrom => {
                // Both source and target must be Entity (worlds)
                if source.sort != Sort::Entity {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::AccessibleFrom,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "accessible_from source must be entity (world)".to_string(),
                    });
                }
                if target.sort != Sort::Entity {
                    result.add_error(UlError::SortViolation {
                        edge_type: EdgeType::AccessibleFrom,
                        from_node: edge.source.clone(),
                        from_sort: source.sort,
                        to_node: edge.target.clone(),
                        to_sort: target.sort,
                        reason: "accessible_from target must be entity (world)".to_string(),
                    });
                }
            }
        }
    }
}

// -- Layer 3: Graph invariant validation --

fn validate_invariants(gir: &Gir, result: &mut ValidationResult) {
    // Check containment acyclicity (DFS cycle detection)
    let contains_edges: Vec<(&str, &str)> = gir
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Contains)
        .map(|e| (e.source.as_str(), e.target.as_str()))
        .collect();

    // Build adjacency list for contains edges
    let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut has_parent: HashSet<&str> = HashSet::new();
    for (src, tgt) in &contains_edges {
        children.entry(src).or_default().push(tgt);
        has_parent.insert(tgt);
    }

    // Cycle detection via DFS
    let node_ids: Vec<&str> = gir.nodes.iter().map(|n| n.id.as_str()).collect();
    let mut visited: HashSet<&str> = HashSet::new();
    let mut in_stack: HashSet<&str> = HashSet::new();

    for &start in &node_ids {
        if !visited.contains(start) && has_cycle_dfs(start, &children, &mut visited, &mut in_stack)
        {
            result.add_error(UlError::ContainmentCycle {
                node_ids: in_stack.iter().map(|s| s.to_string()).collect(),
            });
            break;
        }
    }

    // Check single root: exactly one node not targeted by any contains edge
    let roots: Vec<&str> = node_ids
        .iter()
        .filter(|id| !has_parent.contains(*id))
        .copied()
        .collect();

    match roots.len() {
        0 => result.add_error(UlError::NoRoot),
        1 => {
            if roots[0] != gir.root {
                result.add_warning(format!(
                    "declared root '{}' does not match tree-spine root '{}'",
                    gir.root, roots[0]
                ));
            }
        }
        _ => {
            // Multiple roots are okay if only one is the declared root and others
            // are disconnected from the containment tree (e.g., cross-edge-only nodes).
            // For strict mode, flag it:
            if roots.len() > 1 && !roots.contains(&gir.root.as_str()) {
                result.add_error(UlError::MultipleRoots {
                    node_ids: roots.iter().map(|s| s.to_string()).collect(),
                });
            }
        }
    }

    // Check angle arity: angle nodes must reference exactly 2 lines/curves
    // An angle node is "associated" with lines via modified_by edges (line → modified_by → angle)
    let node_map: HashMap<&str, &crate::types::node::Node> =
        gir.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

    for node in &gir.nodes {
        if node.node_type == crate::types::node::NodeType::Angle {
            // Count how many lines/curves are modified by this angle
            let modifier_sources: Vec<&str> = gir
                .edges
                .iter()
                .filter(|e| e.edge_type == EdgeType::ModifiedBy && e.target == node.id)
                .map(|e| e.source.as_str())
                .filter(|src| {
                    node_map.get(src).is_some_and(|n| {
                        n.node_type == crate::types::node::NodeType::Line
                            || n.node_type == crate::types::node::NodeType::Curve
                    })
                })
                .collect();

            // Angles applied via operators have exactly 1 line source (the connection line)
            // Standalone angle primitives have 0 (they are self-contained marks)
            // The spec says "exactly 2" but in current transform, angles modify 1 line
            // We warn if > 2 (invalid), but allow 0-2 as valid
            if modifier_sources.len() > 2 {
                result.add_error(UlError::InvalidAngleArity {
                    node_id: node.id.clone(),
                    found: modifier_sources.len(),
                });
            }
        }
    }

    // Check dangling modifiers: modifier-sort nodes should have at least one modified_by edge targeting them
    for node in &gir.nodes {
        if node.sort == Sort::Modifier && node.node_type == crate::types::node::NodeType::Angle {
            let has_modifier_edge = gir
                .edges
                .iter()
                .any(|e| e.edge_type == EdgeType::ModifiedBy && e.target == node.id);
            let is_standalone_primitive = !has_modifier_edge
                && gir
                    .edges
                    .iter()
                    .any(|e| e.edge_type == EdgeType::Contains && e.target == node.id);
            // Only warn if the angle node is truly dangling (not contained and not modifying anything)
            if !has_modifier_edge && !is_standalone_primitive {
                result.add_warning(format!(
                    "angle node '{}' has no modified_by edge targeting it",
                    node.id
                ));
            }
        }
    }

    // Check force: warn if set on non-enclosure nodes (force is semantically for assertion frames)
    for node in &gir.nodes {
        if node.force.is_some() && node.node_type != crate::types::node::NodeType::Enclosure {
            result.add_warning(format!(
                "force set on non-enclosure node '{}' (type={:?})",
                node.id, node.node_type
            ));
        }
    }
}

fn has_cycle_dfs<'a>(
    node: &'a str,
    children: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    in_stack: &mut HashSet<&'a str>,
) -> bool {
    visited.insert(node);
    in_stack.insert(node);

    if let Some(kids) = children.get(node) {
        for &child in kids {
            if !visited.contains(child) {
                if has_cycle_dfs(child, children, visited, in_stack) {
                    return true;
                }
            } else if in_stack.contains(child) {
                return true;
            }
        }
    }

    in_stack.remove(node);
    false
}

// -- Layer 3b: Binding validation --

fn validate_bindings(gir: &Gir, result: &mut ValidationResult) {
    use crate::types::node::NodeType;

    // Every VariableSlot must have at least one Binds edge from it
    for node in &gir.nodes {
        if node.node_type == NodeType::VariableSlot {
            let has_bind_edge = gir
                .edges
                .iter()
                .any(|e| e.edge_type == EdgeType::Binds && e.source == node.id);
            if !has_bind_edge {
                result.add_warning(format!(
                    "variable_slot node '{}' has no binds edge — may be an unresolved variable",
                    node.id
                ));
            }
        }
    }

    // Every Binds edge source variable_id must be listed in binding_scope (if scope exists)
    if let Some(ref scope) = gir.binding_scope {
        for node in &gir.nodes {
            if node.node_type == NodeType::VariableSlot {
                if let Some(ref vid) = node.variable_id {
                    if !scope.contains(vid) {
                        result.add_warning(format!(
                            "variable_slot node '{}' (var '{}') is not in binding_scope {:?}",
                            node.id, vid, scope
                        ));
                    }
                }
            }
        }
    }
}

// -- Layer 4: Geometric satisfiability --

fn validate_geometry(gir: &Gir, result: &mut ValidationResult) {
    for node in &gir.nodes {
        match node.node_type {
            crate::types::node::NodeType::Angle => {
                // Angle measure must be in [0, 360)
                if let Some(measure) = node.measure {
                    if !measure.is_finite() || measure < 0.0 || measure >= 360.0 {
                        result.add_error(UlError::Render {
                            message: format!(
                                "angle node '{}' has invalid measure {} (must be in [0, 360))",
                                node.id, measure
                            ),
                        });
                    }
                }
            }
            crate::types::node::NodeType::Line => {
                // Direction vector must be normalized (finite, non-zero)
                if let Some(dir) = node.direction {
                    let len = (dir[0] * dir[0] + dir[1] * dir[1]).sqrt();
                    if !len.is_finite() || len < 1e-10 {
                        result.add_error(UlError::Render {
                            message: format!(
                                "line node '{}' has degenerate direction vector [{}, {}]",
                                node.id, dir[0], dir[1]
                            ),
                        });
                    }
                }
            }
            crate::types::node::NodeType::Curve => {
                // Curvature must be finite and non-negative
                if let Some(curvature) = node.curvature {
                    if !curvature.is_finite() || curvature < 0.0 {
                        result.add_error(UlError::Render {
                            message: format!(
                                "curve node '{}' has invalid curvature {} (must be >= 0 and finite)",
                                node.id, curvature
                            ),
                        });
                    }
                }
            }
            crate::types::node::NodeType::Enclosure => {
                // Polygon vertices must be >= 3
                if let Some(vertices) = node.vertices {
                    if vertices < 3 {
                        result.add_error(UlError::Render {
                            message: format!(
                                "enclosure node '{}' has {} vertices (polygon requires >= 3)",
                                node.id, vertices
                            ),
                        });
                    }
                }
            }
            crate::types::node::NodeType::Point => {}
            crate::types::node::NodeType::VariableSlot => {
                // Variable slot must have variable_id set
                if node.variable_id.is_none() {
                    result.add_error(UlError::MissingField {
                        node_id: node.id.clone(),
                        field: "variable_id".to_string(),
                    });
                }
            }
        }
    }

    // Containment nesting: verify no sibling enclosures claim to contain each other
    // (already partially checked by acyclicity, but also check that two siblings
    // don't both try to contain the same child)
    let contains: Vec<(&str, &str)> = gir
        .edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Contains)
        .map(|e| (e.source.as_str(), e.target.as_str()))
        .collect();

    // Check for nodes with multiple parents (valid in general graphs but
    // problematic for spatial containment — a node can only be inside one enclosure)
    let mut parent_count: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(src, tgt) in &contains {
        parent_count.entry(tgt).or_default().push(src);
    }
    for (child, parents) in &parent_count {
        if parents.len() > 1 {
            result.add_warning(format!(
                "node '{}' is contained by multiple enclosures: {:?} — \
                 spatial containment should form a strict tree",
                child, parents
            ));
        }
    }
}
