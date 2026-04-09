//! Scoring: puzzle-specific scoring and Jane's learning assessment.
//!
//! Scores compositions against the real UL formal system:
//! structural topology, sort correctness, operation usage, and sequence order.

use std::collections::HashMap;

use ul_core::types::node::NodeType;
use ul_core::Gir;

use crate::context::GameContext;
use crate::types::{Grade, JaneResult, PartialCredit, PuzzleTarget, ScoreResult};

/// Score a composition against a specific puzzle target.
pub fn score_composition(ctx: &GameContext, gir: &Gir, target: &PuzzleTarget) -> ScoreResult {
    let expected = match ul_core::Gir::from_json(&target.expected_gir_json) {
        Ok(g) => g,
        Err(_) => {
            return ScoreResult {
                score: 0.0,
                grade: Grade::Unrelated,
                partial_credit: PartialCredit {
                    structural_match: 0.0,
                    sort_correctness: 0.0,
                    operation_correctness: 0.0,
                    sequence_order: 0.0,
                },
                feedback: vec!["Internal error: invalid target GIR.".into()],
            };
        }
    };

    let structural = structural_similarity(gir, &expected);
    let sort = sort_similarity(gir, &expected);
    let operation = ctx.rule_index.correctness_score(gir);
    let sequence = sequence_score(gir, &target.sequence_constraints);

    let partial = PartialCredit {
        structural_match: structural,
        sort_correctness: sort,
        operation_correctness: operation,
        sequence_order: sequence,
    };
    let composite = partial.composite();

    let mut feedback = Vec::new();
    if structural < 0.5 {
        feedback.push("Structure differs significantly from target.".into());
    }
    if sort < 0.5 {
        feedback.push("Sort assignments need correction (entity/relation/modifier/assertion).".into());
    }
    if operation < 0.5 {
        feedback.push("Composition rules are not satisfied.".into());
    }
    if sequence < 0.5 {
        feedback.push("Construction ordering needs work.".into());
    }
    if feedback.is_empty() {
        feedback.push("Good composition!".into());
    }

    ScoreResult {
        score: composite,
        grade: Grade::from_score(composite),
        partial_credit: partial,
        feedback,
    }
}

/// Evaluate Jane's learning attempt: score + proficiency delta.
pub fn evaluate_jane_attempt(
    ctx: &mut GameContext,
    attempt: &Gir,
    expected: &Gir,
) -> JaneResult {
    let structural = structural_similarity(attempt, expected);
    let sort = sort_similarity(attempt, expected);
    let operation = ctx.rule_index.correctness_score(attempt);
    let sequence = 1.0; // No sequence constraints for Jane's free-form attempts

    let partial = PartialCredit {
        structural_match: structural,
        sort_correctness: sort,
        operation_correctness: operation,
        sequence_order: sequence,
    };
    let score = partial.composite();

    // Update proficiency per primitive type found in the attempt
    let mut proficiency_delta = HashMap::new();
    let primitives_in_attempt = extract_primitive_names(attempt);
    for prim in &primitives_in_attempt {
        let delta = ctx.update_proficiency(prim, score);
        proficiency_delta.insert(prim.clone(), delta);
    }

    let mut improvements = Vec::new();
    if structural < 0.8 {
        improvements.push("Try matching the geometric structure more closely.".into());
    }
    if sort < 0.8 {
        improvements.push("Check that each node has the correct sort (entity/relation/modifier/assertion).".into());
    }

    JaneResult {
        score,
        grade: Grade::from_score(score),
        improvements,
        proficiency_delta,
    }
}

/// Structural similarity: compare node-type histograms and edge counts.
fn structural_similarity(a: &Gir, b: &Gir) -> f64 {
    let hist_a = node_type_histogram(a);
    let hist_b = node_type_histogram(b);

    // Histogram intersection normalized by max
    let mut intersection = 0u32;
    let mut union = 0u32;
    let all_types = [
        NodeType::Point,
        NodeType::Line,
        NodeType::Angle,
        NodeType::Curve,
        NodeType::Enclosure,
        NodeType::VariableSlot,
    ];
    for nt in &all_types {
        let ca = hist_a.get(nt).copied().unwrap_or(0);
        let cb = hist_b.get(nt).copied().unwrap_or(0);
        intersection += ca.min(cb);
        union += ca.max(cb);
    }

    let shape_score = if union == 0 {
        1.0
    } else {
        intersection as f64 / union as f64
    };

    // Edge count similarity
    let ea = a.edges.len() as f64;
    let eb = b.edges.len() as f64;
    let edge_score = if ea == 0.0 && eb == 0.0 {
        1.0
    } else {
        1.0 - (ea - eb).abs() / ea.max(eb)
    };

    (shape_score + edge_score) / 2.0
}

/// Sort similarity: fraction of nodes that have matching sorts in the expected GIR.
fn sort_similarity(actual: &Gir, expected: &Gir) -> f64 {
    if expected.nodes.is_empty() {
        return 1.0;
    }

    let expected_sorts: HashMap<NodeType, ul_core::Sort> = expected
        .nodes
        .iter()
        .map(|n| (n.node_type, n.sort))
        .collect();

    let mut matches = 0usize;
    let total = actual.nodes.len();
    for node in &actual.nodes {
        if let Some(&expected_sort) = expected_sorts.get(&node.node_type) {
            if node.sort == expected_sort {
                matches += 1;
            }
        }
    }

    if total == 0 {
        1.0
    } else {
        matches as f64 / total as f64
    }
}

/// Score how well node ordering matches sequence constraints.
fn sequence_score(gir: &Gir, constraints: &[Vec<String>]) -> f64 {
    if constraints.is_empty() {
        return 1.0;
    }

    let pos: HashMap<&str, usize> = gir
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.id.as_str(), i))
        .collect();

    let mut satisfied = 0usize;
    let mut total = 0usize;

    for constraint in constraints {
        for pair in constraint.windows(2) {
            total += 1;
            let p0 = pos.get(pair[0].as_str());
            let p1 = pos.get(pair[1].as_str());
            if let (Some(&a), Some(&b)) = (p0, p1) {
                if a < b {
                    satisfied += 1;
                }
            }
        }
    }

    if total == 0 {
        1.0
    } else {
        satisfied as f64 / total as f64
    }
}

fn node_type_histogram(gir: &Gir) -> HashMap<NodeType, u32> {
    let mut hist = HashMap::new();
    for node in &gir.nodes {
        *hist.entry(node.node_type).or_insert(0) += 1;
    }
    hist
}

fn extract_primitive_names(gir: &Gir) -> Vec<String> {
    let mut names: Vec<String> = gir
        .nodes
        .iter()
        .map(|n| format!("{:?}", n.node_type).to_lowercase())
        .collect();
    names.sort();
    names.dedup();
    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::GameConfig;

    fn ctx() -> GameContext {
        GameContext::from_config(&GameConfig {
            rules_json: String::new(),
            session_id: "test".into(),
        })
        .unwrap()
    }

    #[test]
    fn identical_girs_score_perfect() {
        // Parse a point `*`, then score it against itself
        let gir = ul_core::parser::parse("*").unwrap();
        let expected_json = gir.to_json().unwrap();
        let target = PuzzleTarget {
            expected_gir_json: expected_json,
            required_operations: vec![],
            sequence_constraints: vec![],
            weights: None,
        };
        let result = score_composition(&ctx(), &gir, &target);
        assert!(result.score >= 0.95, "score was {}", result.score);
        assert_eq!(result.grade, Grade::Exact);
    }

    #[test]
    fn different_girs_score_lower() {
        // Compare a point `*` to an enclosure-with-point `/0{*}`
        let actual = ul_core::parser::parse("*").unwrap();
        let expected = ul_core::parser::parse("/0{*}").unwrap();
        let expected_json = expected.to_json().unwrap();
        let target = PuzzleTarget {
            expected_gir_json: expected_json,
            required_operations: vec![],
            sequence_constraints: vec![],
            weights: None,
        };
        let result = score_composition(&ctx(), &actual, &target);
        assert!(result.score < 0.95, "score was {}", result.score);
    }

    #[test]
    fn jane_attempt_updates_proficiency() {
        let mut ctx = ctx();
        let attempt = ul_core::parser::parse("*").unwrap();
        let expected = ul_core::parser::parse("*").unwrap();
        let initial = ctx.proficiency_for("point");
        let result = evaluate_jane_attempt(&mut ctx, &attempt, &expected);
        assert!(result.score > 0.0);
        let new = ctx.proficiency_for("point");
        assert_ne!(initial, new);
    }
}
