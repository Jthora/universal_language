//! Structural analysis: inspect GIR to extract primitives, sorts, operations, complexity.

use crate::types::{Operation, StructureReport};
use ul_core::composer;
use ul_core::types::edge::EdgeType;
use ul_core::types::gir::Gir;
use ul_core::types::node::NodeType;
use ul_core::types::sort::Sort;

/// Analyze a GIR structure and produce a comprehensive report.
pub fn analyze_structure(gir: &Gir) -> StructureReport {
    let mut primitive_counts = std::collections::HashMap::new();
    let mut sort_distribution = std::collections::HashMap::new();

    for node in &gir.nodes {
        *primitive_counts
            .entry(format!("{}", node.node_type))
            .or_insert(0) += 1u32;
        *sort_distribution
            .entry(format!("{}", node.sort))
            .or_insert(0) += 1u32;
    }

    let detected = composer::detect_operations(gir);
    let detected_operations: Vec<String> = detected.iter().map(|d| d.operation.to_string()).collect();

    let depth = compute_depth(gir);
    let breadth = compute_breadth(gir);
    let complexity = compute_complexity(gir);

    StructureReport {
        node_count: gir.nodes.len() as u32,
        edge_count: gir.edges.len() as u32,
        primitive_counts,
        sort_distribution,
        detected_operations,
        depth,
        breadth,
        complexity_score: complexity,
    }
}

/// Detect which Σ_UL operations are expressed in a GIR.
pub fn detect_operations_list(gir: &Gir) -> Vec<Operation> {
    let detected = composer::detect_operations(gir);
    detected
        .iter()
        .filter_map(|d| match d.operation {
            "predicate" => Some(Operation::Predicate),
            "modify_entity" => Some(Operation::ModifyEntity),
            "modify_relation" => Some(Operation::ModifyRelation),
            "negate" => Some(Operation::Negate),
            "conjoin" => Some(Operation::Conjoin),
            "disjoin" => Some(Operation::Disjoin),
            "embed" => Some(Operation::Embed),
            "abstract" => Some(Operation::Abstract),
            "compose" => Some(Operation::Compose),
            "invert" => Some(Operation::Invert),
            "quantify" => Some(Operation::Quantify),
            _ => None,
        })
        .collect()
}

/// Compute the maximum containment depth.
fn compute_depth(gir: &Gir) -> u32 {
    let mut children: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
    for edge in &gir.edges {
        if edge.edge_type == EdgeType::Contains {
            children
                .entry(edge.source.as_str())
                .or_default()
                .push(edge.target.as_str());
        }
    }

    fn dfs_depth(node: &str, children: &std::collections::HashMap<&str, Vec<&str>>) -> u32 {
        match children.get(node) {
            None => 1,
            Some(kids) => 1 + kids.iter().map(|k| dfs_depth(k, children)).max().unwrap_or(0),
        }
    }

    dfs_depth(&gir.root, &children)
}

/// Compute the maximum breadth (most children at any level).
fn compute_breadth(gir: &Gir) -> u32 {
    let mut children_count: std::collections::HashMap<&str, u32> =
        std::collections::HashMap::new();
    for edge in &gir.edges {
        if edge.edge_type == EdgeType::Contains {
            *children_count.entry(edge.source.as_str()).or_insert(0) += 1;
        }
    }
    children_count.values().copied().max().unwrap_or(0)
}

/// Compute a complexity score (0.0–1.0) based on structural features.
fn compute_complexity(gir: &Gir) -> f64 {
    let n = gir.nodes.len() as f64;
    let e = gir.edges.len() as f64;
    if n == 0.0 {
        return 0.0;
    }

    // Unique sorts used
    let sorts: std::collections::HashSet<Sort> = gir.nodes.iter().map(|n| n.sort).collect();
    let sort_variety = sorts.len() as f64 / 4.0; // max 4 sorts

    // Unique node types used
    let types: std::collections::HashSet<NodeType> =
        gir.nodes.iter().map(|n| n.node_type).collect();
    let type_variety = types.len() as f64 / 5.0; // max 5 primitives

    // Edge density
    let max_edges = n * (n - 1.0);
    let density = if max_edges > 0.0 { e / max_edges } else { 0.0 };

    // Operations detected
    let ops = composer::detect_operations(gir);
    let op_variety = (ops.len() as f64 / 11.0).min(1.0);

    // Weighted composite
    (sort_variety * 0.25 + type_variety * 0.25 + density * 0.2 + op_variety * 0.3).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ul_core::types::edge::Edge;
    use ul_core::types::node::{EnclosureShape, Node};

    #[test]
    fn analyze_single_point() {
        let gir = Gir::new("p", vec![Node::point("p")], vec![]);
        let report = analyze_structure(&gir);
        assert_eq!(report.node_count, 1);
        assert_eq!(report.edge_count, 0);
        assert_eq!(report.depth, 1);
        assert_eq!(report.breadth, 0);
    }

    #[test]
    fn analyze_nested_structure() {
        let gir = Gir::new(
            "root",
            vec![
                Node::enclosure("root", EnclosureShape::Circle),
                Node::point("p1"),
                Node::point("p2"),
                Node::line("l1", true),
            ],
            vec![
                Edge::contains("root", "p1"),
                Edge::contains("root", "p2"),
                Edge::contains("root", "l1"),
            ],
        );
        let report = analyze_structure(&gir);
        assert_eq!(report.node_count, 4);
        assert_eq!(report.depth, 2);
        assert_eq!(report.breadth, 3);
        assert!(report.complexity_score > 0.0);
    }

    #[test]
    fn detect_operations_finds_modify() {
        let gir = ul_core::composer::modify_entity(
            &Gir::new("m", vec![Node::angle("m", 45.0)], vec![]),
            &Gir::new("e", vec![Node::point("e")], vec![]),
        )
        .unwrap();
        let ops = detect_operations_list(&gir);
        assert!(ops.contains(&Operation::ModifyEntity));
    }
}
