//! Structural analysis: inspect GIR to extract primitives, sorts, operations, complexity.

use crate::types::{
    EquivalenceDimensions, EquivalenceResult, ErlangenLevel, Operation, PartOfSpeech,
    StructureReport, SymmetryGroup,
};
use ul_core::composer;
use ul_core::types::edge::EdgeType;
use ul_core::types::gir::Gir;
use ul_core::types::node::{EnclosureShape, Node, NodeType};
use ul_core::types::sort::Sort;

/// Analyze a GIR structure and produce a comprehensive report.
pub fn analyze_structure(gir: &Gir) -> StructureReport {
    let mut primitive_counts = std::collections::HashMap::new();
    let mut sort_distribution = std::collections::HashMap::new();
    let mut node_symmetries = std::collections::HashMap::new();

    for node in &gir.nodes {
        *primitive_counts
            .entry(format!("{}", node.node_type))
            .or_insert(0) += 1u32;
        *sort_distribution
            .entry(format!("{}", node.sort))
            .or_insert(0) += 1u32;
        node_symmetries.insert(node.id.clone(), classify_symmetry(node));
    }

    let detected = composer::detect_operations(gir);
    let detected_operations: Vec<String> = detected.iter().map(|d| d.operation.to_string()).collect();

    let depth = compute_depth(gir);
    let breadth = compute_breadth(gir);
    let complexity = compute_complexity(gir);

    // Root node symmetry determines the overall classification
    let root_symmetry = gir
        .nodes
        .iter()
        .find(|n| n.id == gir.root)
        .map(|n| classify_symmetry(n))
        .unwrap_or(SymmetryGroup::None);

    StructureReport {
        node_count: gir.nodes.len() as u32,
        edge_count: gir.edges.len() as u32,
        primitive_counts,
        sort_distribution,
        detected_operations,
        depth,
        breadth,
        complexity_score: complexity,
        symmetry_group: root_symmetry,
        part_of_speech: symmetry_to_part_of_speech(root_symmetry),
        node_symmetries,
    }
}

/// Classify the symmetry group of a single node based on its geometric type.
///
/// Follows the grammar-book specification (Erlangen Program):
/// - Circle → SO(2), Triangle → D₃, Square → D₄, Pentagon → D₅, Hexagon → D₆
/// - Undirected line / angle / ellipse → Bilateral
/// - Directed line / curve / point / freeform → None
pub fn classify_symmetry(node: &Node) -> SymmetryGroup {
    match node.node_type {
        NodeType::Point => SymmetryGroup::None,
        NodeType::Line => {
            if node.directed == Some(true) {
                SymmetryGroup::None
            } else {
                SymmetryGroup::Bilateral
            }
        }
        NodeType::Angle => SymmetryGroup::Bilateral,
        NodeType::Curve => SymmetryGroup::None,
        NodeType::Enclosure => match node.shape {
            Some(EnclosureShape::Circle) => SymmetryGroup::So2,
            Some(EnclosureShape::Triangle) => SymmetryGroup::D3,
            Some(EnclosureShape::Square) => SymmetryGroup::D4,
            Some(EnclosureShape::Polygon) => match node.vertices {
                Some(5) => SymmetryGroup::D5,
                Some(6) => SymmetryGroup::D6,
                Some(n) if n >= 3 => SymmetryGroup::Bilateral,
                _ => SymmetryGroup::None,
            },
            Some(EnclosureShape::Ellipse) => SymmetryGroup::Bilateral,
            Some(EnclosureShape::Freeform) => SymmetryGroup::None,
            None => SymmetryGroup::So2, // default enclosure is circle
        },
    }
}

/// Map a symmetry group to its grammatical part of speech.
///
/// From grammar-book.md §II:
/// - SO(2) → Determiner/Article (universally applicable)
/// - D₃–D₆ → Noun (context-independent concept)
/// - Bilateral → Adjective/Adverb (one axis of comparison)
/// - None → Verb (directed/contextual) or ProperNoun (unique)
pub fn symmetry_to_part_of_speech(sym: SymmetryGroup) -> PartOfSpeech {
    match sym {
        SymmetryGroup::So2 => PartOfSpeech::Determiner,
        SymmetryGroup::D3 | SymmetryGroup::D4 | SymmetryGroup::D5 | SymmetryGroup::D6 => {
            PartOfSpeech::Noun
        }
        SymmetryGroup::Bilateral => PartOfSpeech::Adjective,
        SymmetryGroup::None => PartOfSpeech::Verb,
    }
}

// ── Erlangen equivalence testing ──────────────────────────────

/// Compare two GIR structures at a given Erlangen level.
///
/// Levels (strictest → loosest):
/// 1. Euclidean: identical types, shapes, angles, sizes
/// 2. Similarity: ignore absolute scale
/// 3. Affine: ignore proportions, keep containment/parallel structure
/// 4. Projective: ignore parallel relationships, keep incidence
/// 5. Topological: graph isomorphism of types only
pub fn compare_gir(a: &Gir, b: &Gir, level: ErlangenLevel) -> EquivalenceResult {
    let topology = compare_topology(a, b);
    let types = compare_types(a, b);
    let sorts = compare_sorts(a, b);
    let shapes = compare_shapes(a, b);
    let metrics = compare_metrics(a, b);

    // Weight dimensions based on Erlangen level:
    // stricter levels weight more dimensions; looser levels ignore fine details
    let score = match level {
        ErlangenLevel::Euclidean => {
            topology * 0.20 + types * 0.20 + sorts * 0.15 + shapes * 0.20 + metrics * 0.25
        }
        ErlangenLevel::Similarity => {
            // Ignore absolute scale (metrics weight reduced, shapes more important)
            topology * 0.25 + types * 0.25 + sorts * 0.15 + shapes * 0.25 + metrics * 0.10
        }
        ErlangenLevel::Affine => {
            // Ignore proportions, keep structure
            topology * 0.30 + types * 0.30 + sorts * 0.20 + shapes * 0.20
        }
        ErlangenLevel::Projective => {
            // Just incidence structure and types
            topology * 0.35 + types * 0.35 + sorts * 0.30
        }
        ErlangenLevel::Topological => {
            // Pure connectivity
            topology * 0.50 + types * 0.50
        }
    };

    EquivalenceResult {
        level,
        score,
        equivalent: score >= 0.95,
        dimensions: EquivalenceDimensions {
            topology,
            types,
            sorts,
            shapes,
            metrics,
        },
    }
}

/// Compare graph topology: node count, edge count, depth, breadth.
fn compare_topology(a: &Gir, b: &Gir) -> f64 {
    let na = a.nodes.len() as f64;
    let nb = b.nodes.len() as f64;
    let ea = a.edges.len() as f64;
    let eb = b.edges.len() as f64;

    if na == 0.0 && nb == 0.0 {
        return 1.0;
    }
    if na == 0.0 || nb == 0.0 {
        return 0.0;
    }

    let node_sim = 1.0 - (na - nb).abs() / na.max(nb);
    let edge_sim = if ea.max(eb) == 0.0 {
        1.0
    } else {
        1.0 - (ea - eb).abs() / ea.max(eb)
    };

    let da = compute_depth(a) as f64;
    let db = compute_depth(b) as f64;
    let depth_sim = if da.max(db) == 0.0 {
        1.0
    } else {
        1.0 - (da - db).abs() / da.max(db)
    };

    (node_sim + edge_sim + depth_sim) / 3.0
}

/// Compare node type distributions.
fn compare_types(a: &Gir, b: &Gir) -> f64 {
    compare_distributions(
        &count_by(a, |n| format!("{}", n.node_type)),
        &count_by(b, |n| format!("{}", n.node_type)),
    )
}

/// Compare sort distributions.
fn compare_sorts(a: &Gir, b: &Gir) -> f64 {
    compare_distributions(
        &count_by(a, |n| format!("{}", n.sort)),
        &count_by(b, |n| format!("{}", n.sort)),
    )
}

/// Compare enclosure shape distributions.
fn compare_shapes(a: &Gir, b: &Gir) -> f64 {
    let shape_a: Vec<String> = a
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Enclosure)
        .map(|n| format!("{:?}", n.shape))
        .collect();
    let shape_b: Vec<String> = b
        .nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Enclosure)
        .map(|n| format!("{:?}", n.shape))
        .collect();

    if shape_a.is_empty() && shape_b.is_empty() {
        return 1.0;
    }
    if shape_a.is_empty() || shape_b.is_empty() {
        return 0.0;
    }

    let mut counts_a: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    let mut counts_b: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for s in &shape_a {
        *counts_a.entry(s.as_str()).or_insert(0) += 1;
    }
    for s in &shape_b {
        *counts_b.entry(s.as_str()).or_insert(0) += 1;
    }

    let all_keys: std::collections::HashSet<&str> =
        counts_a.keys().chain(counts_b.keys()).copied().collect();
    let mut intersection = 0usize;
    let mut union = 0usize;
    for k in all_keys {
        let ca = counts_a.get(k).copied().unwrap_or(0);
        let cb = counts_b.get(k).copied().unwrap_or(0);
        intersection += ca.min(cb);
        union += ca.max(cb);
    }

    if union == 0 {
        1.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Compare metric properties (angles, curvatures).
fn compare_metrics(a: &Gir, b: &Gir) -> f64 {
    let mut angles_a: Vec<f64> = a
        .nodes
        .iter()
        .filter_map(|n| n.measure)
        .collect();
    let mut angles_b: Vec<f64> = b
        .nodes
        .iter()
        .filter_map(|n| n.measure)
        .collect();
    angles_a.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    angles_b.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));

    let angle_sim = compare_sorted_f64(&angles_a, &angles_b, 360.0);

    let mut curv_a: Vec<f64> = a
        .nodes
        .iter()
        .filter_map(|n| n.curvature)
        .collect();
    let mut curv_b: Vec<f64> = b
        .nodes
        .iter()
        .filter_map(|n| n.curvature)
        .collect();
    curv_a.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    curv_b.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));

    let curv_sim = compare_sorted_f64(&curv_a, &curv_b, 10.0);

    (angle_sim + curv_sim) / 2.0
}

/// Helper: count nodes by a key function.
fn count_by(gir: &Gir, f: impl Fn(&Node) -> String) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for node in &gir.nodes {
        *counts.entry(f(node)).or_insert(0) += 1;
    }
    counts
}

/// Helper: compare two string-keyed distributions using Jaccard-like similarity.
fn compare_distributions(
    a: &std::collections::HashMap<String, usize>,
    b: &std::collections::HashMap<String, usize>,
) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let all_keys: std::collections::HashSet<&String> = a.keys().chain(b.keys()).collect();
    let mut intersection = 0usize;
    let mut union = 0usize;
    for k in all_keys {
        let ca = a.get(k).copied().unwrap_or(0);
        let cb = b.get(k).copied().unwrap_or(0);
        intersection += ca.min(cb);
        union += ca.max(cb);
    }
    if union == 0 {
        1.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Helper: compare two sorted f64 vectors (padded with zeros, normalized by max_range).
fn compare_sorted_f64(a: &[f64], b: &[f64], max_range: f64) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let len = a.len().max(b.len());
    if len == 0 {
        return 1.0;
    }
    let mut sum = 0.0;
    for i in 0..len {
        let va = if i < a.len() { a[i] } else { 0.0 };
        let vb = if i < b.len() { b[i] } else { 0.0 };
        sum += 1.0 - (va - vb).abs() / max_range;
    }
    (sum / len as f64).max(0.0)
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
        assert_eq!(report.symmetry_group, SymmetryGroup::None);
        assert_eq!(report.part_of_speech, PartOfSpeech::Verb);
    }

    #[test]
    fn symmetry_circle_is_so2() {
        let node = Node::enclosure("c", EnclosureShape::Circle);
        assert_eq!(classify_symmetry(&node), SymmetryGroup::So2);
        assert_eq!(symmetry_to_part_of_speech(SymmetryGroup::So2), PartOfSpeech::Determiner);
    }

    #[test]
    fn symmetry_triangle_is_d3() {
        let node = Node::enclosure("t", EnclosureShape::Triangle);
        assert_eq!(classify_symmetry(&node), SymmetryGroup::D3);
        assert_eq!(symmetry_to_part_of_speech(SymmetryGroup::D3), PartOfSpeech::Noun);
    }

    #[test]
    fn symmetry_square_is_d4() {
        let node = Node::enclosure("s", EnclosureShape::Square);
        assert_eq!(classify_symmetry(&node), SymmetryGroup::D4);
    }

    #[test]
    fn symmetry_polygon5_is_d5() {
        let mut node = Node::enclosure("p", EnclosureShape::Polygon);
        node.vertices = Some(5);
        assert_eq!(classify_symmetry(&node), SymmetryGroup::D5);
    }

    #[test]
    fn symmetry_polygon6_is_d6() {
        let mut node = Node::enclosure("h", EnclosureShape::Polygon);
        node.vertices = Some(6);
        assert_eq!(classify_symmetry(&node), SymmetryGroup::D6);
    }

    #[test]
    fn symmetry_directed_line_is_none() {
        let node = Node::line("l", true);
        assert_eq!(classify_symmetry(&node), SymmetryGroup::None);
        assert_eq!(symmetry_to_part_of_speech(SymmetryGroup::None), PartOfSpeech::Verb);
    }

    #[test]
    fn symmetry_undirected_line_is_bilateral() {
        let node = Node::line("l", false);
        assert_eq!(classify_symmetry(&node), SymmetryGroup::Bilateral);
        assert_eq!(symmetry_to_part_of_speech(SymmetryGroup::Bilateral), PartOfSpeech::Adjective);
    }

    #[test]
    fn symmetry_angle_is_bilateral() {
        let node = Node::angle("a", 45.0);
        assert_eq!(classify_symmetry(&node), SymmetryGroup::Bilateral);
    }

    #[test]
    fn symmetry_ellipse_is_bilateral() {
        let node = Node::enclosure("e", EnclosureShape::Ellipse);
        assert_eq!(classify_symmetry(&node), SymmetryGroup::Bilateral);
    }

    #[test]
    fn symmetry_freeform_is_none() {
        let node = Node::enclosure("f", EnclosureShape::Freeform);
        assert_eq!(classify_symmetry(&node), SymmetryGroup::None);
    }

    #[test]
    fn report_includes_node_symmetries() {
        let gir = Gir::new(
            "root",
            vec![
                Node::enclosure("root", EnclosureShape::Circle),
                Node::point("p1"),
                Node::line("l1", true),
            ],
            vec![
                Edge::contains("root", "p1"),
                Edge::contains("root", "l1"),
            ],
        );
        let report = analyze_structure(&gir);
        assert_eq!(report.symmetry_group, SymmetryGroup::So2);
        assert_eq!(report.part_of_speech, PartOfSpeech::Determiner);
        assert_eq!(report.node_symmetries.get("root"), Some(&SymmetryGroup::So2));
        assert_eq!(report.node_symmetries.get("p1"), Some(&SymmetryGroup::None));
        assert_eq!(report.node_symmetries.get("l1"), Some(&SymmetryGroup::None));
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

    #[test]
    fn erlangen_identical_structures_score_1() {
        let gir = Gir::new(
            "root",
            vec![
                Node::enclosure("root", EnclosureShape::Circle),
                Node::point("p1"),
            ],
            vec![Edge::contains("root", "p1")],
        );
        let result = compare_gir(&gir, &gir, ErlangenLevel::Euclidean);
        assert!((result.score - 1.0).abs() < 0.001);
        assert!(result.equivalent);
    }

    #[test]
    fn erlangen_different_structures_low_score() {
        let gir_a = Gir::new("p", vec![Node::point("p")], vec![]);
        let gir_b = Gir::new(
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
        let result = compare_gir(&gir_a, &gir_b, ErlangenLevel::Topological);
        assert!(result.score < 0.5);
        assert!(!result.equivalent);
    }

    #[test]
    fn erlangen_topological_looser_than_euclidean() {
        let gir_a = Gir::new(
            "root",
            vec![
                Node::enclosure("root", EnclosureShape::Circle),
                Node::angle("a", 45.0),
            ],
            vec![Edge::contains("root", "a")],
        );
        let gir_b = Gir::new(
            "root",
            vec![
                Node::enclosure("root", EnclosureShape::Circle),
                Node::angle("a", 90.0),
            ],
            vec![Edge::contains("root", "a")],
        );
        let euclidean = compare_gir(&gir_a, &gir_b, ErlangenLevel::Euclidean);
        let topological = compare_gir(&gir_a, &gir_b, ErlangenLevel::Topological);
        // Same topology but different metrics, so topological should score >= euclidean
        assert!(topological.score >= euclidean.score);
    }
}
