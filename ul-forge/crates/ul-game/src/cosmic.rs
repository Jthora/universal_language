//! Composition rule engine: pattern matching against GIR structures.
//!
//! Rules check for correct use of UL primitives, sorts, and operations.
//! Pattern matching uses backtracking search over node/edge constraints.

use std::collections::HashMap;
use ul_core::Gir;

use crate::types::{CompositionRule, GirPattern, PatternEdge, PatternNode};

/// Index of composition rules for fast lookup.
#[derive(Debug, Clone, Default)]
pub struct RuleIndex {
    rules: Vec<CompositionRule>,
}

impl RuleIndex {
    pub fn new(rules: Vec<CompositionRule>) -> Self {
        Self { rules }
    }

    /// Load rules from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let rules: Vec<CompositionRule> = serde_json::from_str(json)?;
        Ok(Self::new(rules))
    }

    /// Add rules at runtime (modding support).
    pub fn add_rules(&mut self, rules: Vec<CompositionRule>) {
        self.rules.extend(rules);
    }

    /// All rules in the index.
    pub fn rules(&self) -> &[CompositionRule] {
        &self.rules
    }

    /// Evaluate all rules against a GIR, returning (matched, violated) rule names.
    pub fn evaluate(&self, gir: &Gir) -> (Vec<String>, Vec<String>) {
        let mut matched = Vec::new();
        let mut violated = Vec::new();

        for rule in &self.rules {
            let required_match = pattern_matches(&rule.required_pattern, gir);
            let forbidden_match = rule
                .forbidden_pattern
                .as_ref()
                .is_some_and(|fp| pattern_matches(fp, gir));

            if required_match && !forbidden_match {
                matched.push(rule.name.clone());
            } else {
                violated.push(rule.name.clone());
            }
        }

        (matched, violated)
    }

    /// Compute the composition correctness score (0.0–1.0) for a GIR.
    pub fn correctness_score(&self, gir: &Gir) -> f64 {
        if self.rules.is_empty() {
            return 1.0;
        }

        let mut total_weight = 0.0;
        let mut earned_weight = 0.0;

        for rule in &self.rules {
            total_weight += rule.weight;

            let required_match = pattern_matches(&rule.required_pattern, gir);
            let forbidden_match = rule
                .forbidden_pattern
                .as_ref()
                .is_some_and(|fp| pattern_matches(fp, gir));

            if required_match && !forbidden_match {
                earned_weight += rule.weight;
            }
        }

        if total_weight == 0.0 {
            1.0
        } else {
            earned_weight / total_weight
        }
    }
}

/// Check if a GirPattern matches anywhere in the GIR.
///
/// Uses backtracking search: try all possible assignments of pattern nodes
/// to GIR nodes, checking type/label constraints, then verify edge constraints.
pub fn pattern_matches(pattern: &GirPattern, gir: &Gir) -> bool {
    if pattern.nodes.is_empty() {
        return true;
    }

    let gir_node_ids: Vec<&str> = gir.nodes.iter().map(|n| n.id.as_str()).collect();

    // Try to find an assignment: pattern node id → gir node id
    let mut assignment: HashMap<String, String> = HashMap::new();
    backtrack(pattern, gir, &gir_node_ids, 0, &mut assignment)
}

fn backtrack(
    pattern: &GirPattern,
    gir: &Gir,
    gir_node_ids: &[&str],
    depth: usize,
    assignment: &mut HashMap<String, String>,
) -> bool {
    if depth == pattern.nodes.len() {
        // All pattern nodes assigned — check edge constraints
        return edges_match(&pattern.edges, gir, assignment);
    }

    let pnode = &pattern.nodes[depth];

    for &gir_id in gir_node_ids {
        // Skip if already assigned to another pattern node
        if assignment.values().any(|v| v == gir_id) {
            continue;
        }

        // Check node constraints
        if !node_constraint_matches(pnode, gir, gir_id) {
            continue;
        }

        assignment.insert(pnode.id.clone(), gir_id.to_string());
        if backtrack(pattern, gir, gir_node_ids, depth + 1, assignment) {
            return true;
        }
        assignment.remove(&pnode.id);
    }

    false
}

fn node_constraint_matches(pnode: &PatternNode, gir: &Gir, gir_id: &str) -> bool {
    let node = match gir.node(gir_id) {
        Some(n) => n,
        None => return false,
    };

    // Check node_type constraint
    if let Some(ref required_type) = pnode.node_type {
        let actual_type = format!("{:?}", node.node_type).to_lowercase();
        if actual_type != required_type.to_lowercase() {
            return false;
        }
    }

    // Check sort constraint
    if let Some(ref required_sort) = pnode.sort {
        let actual_sort = format!("{:?}", node.sort).to_lowercase();
        if actual_sort != required_sort.to_lowercase() {
            return false;
        }
    }

    // Check label constraint
    if let Some(ref required_label) = pnode.label {
        match &node.label {
            Some(actual) if actual.eq_ignore_ascii_case(required_label) => {}
            _ => return false,
        }
    }

    true
}

fn edges_match(
    pattern_edges: &[PatternEdge],
    gir: &Gir,
    assignment: &HashMap<String, String>,
) -> bool {
    for pe in pattern_edges {
        let source_gir = match assignment.get(&pe.source) {
            Some(id) => id,
            None => return false,
        };
        let target_gir = match assignment.get(&pe.target) {
            Some(id) => id,
            None => return false,
        };

        // Find a matching edge in the GIR
        let found = gir.edges.iter().any(|e| {
            e.source == *source_gir
                && e.target == *target_gir
                && pe.edge_type.as_ref().map_or(true, |required| {
                    let actual = format!("{:?}", e.edge_type).to_lowercase();
                    actual == required.to_lowercase()
                })
        });

        if !found {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Tier;

    /// Single point: `*` parses to a GIR with one Point node.
    fn point_gir() -> Gir {
        ul_core::parser::parse("*").unwrap()
    }

    /// Circle enclosure containing a point: `/0{*}`
    fn enclosure_gir() -> Gir {
        ul_core::parser::parse("/0{*}").unwrap()
    }

    #[test]
    fn empty_pattern_matches_anything() {
        let pattern = GirPattern {
            nodes: vec![],
            edges: vec![],
        };
        assert!(pattern_matches(&pattern, &point_gir()));
    }

    #[test]
    fn single_node_type_match() {
        let pattern = GirPattern {
            nodes: vec![PatternNode {
                id: "p1".into(),
                node_type: Some("point".into()),
                sort: None,
                label: None,
            }],
            edges: vec![],
        };
        assert!(pattern_matches(&pattern, &point_gir()));
    }

    #[test]
    fn single_node_type_mismatch() {
        let pattern = GirPattern {
            nodes: vec![PatternNode {
                id: "p1".into(),
                node_type: Some("curve".into()),
                sort: None,
                label: None,
            }],
            edges: vec![],
        };
        assert!(!pattern_matches(&pattern, &point_gir()));
    }

    #[test]
    fn sort_constraint_match() {
        let pattern = GirPattern {
            nodes: vec![PatternNode {
                id: "p1".into(),
                node_type: Some("point".into()),
                sort: Some("entity".into()),
                label: None,
            }],
            edges: vec![],
        };
        assert!(pattern_matches(&pattern, &point_gir()));
    }

    #[test]
    fn sort_constraint_mismatch() {
        let pattern = GirPattern {
            nodes: vec![PatternNode {
                id: "p1".into(),
                node_type: Some("point".into()),
                sort: Some("relation".into()),
                label: None,
            }],
            edges: vec![],
        };
        assert!(!pattern_matches(&pattern, &point_gir()));
    }

    #[test]
    fn edge_pattern_match() {
        let pattern = GirPattern {
            nodes: vec![
                PatternNode {
                    id: "enc".into(),
                    node_type: Some("enclosure".into()),
                    sort: None,
                    label: None,
                },
                PatternNode {
                    id: "pt".into(),
                    node_type: Some("point".into()),
                    sort: None,
                    label: None,
                },
            ],
            edges: vec![PatternEdge {
                source: "enc".into(),
                target: "pt".into(),
                edge_type: Some("contains".into()),
            }],
        };
        assert!(pattern_matches(&pattern, &enclosure_gir()));
    }

    #[test]
    fn rule_index_evaluate() {
        let rule = CompositionRule {
            name: "has-point".into(),
            operation: None,
            tier: Tier::Forced,
            required_pattern: GirPattern {
                nodes: vec![PatternNode {
                    id: "p".into(),
                    node_type: Some("point".into()),
                    sort: None,
                    label: None,
                }],
                edges: vec![],
            },
            forbidden_pattern: None,
            weight: 1.0,
            difficulty: 1,
        };

        let index = RuleIndex::new(vec![rule]);
        let (matched, violated) = index.evaluate(&point_gir());
        assert_eq!(matched, vec!["has-point"]);
        assert!(violated.is_empty());
    }
}
