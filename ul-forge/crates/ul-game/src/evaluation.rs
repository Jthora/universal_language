//! Core evaluation: assess a GIR against active composition rules.

use ul_core::Gir;

use crate::context::GameContext;
use crate::types::{EvaluationResult, Grade};

/// Evaluate a GIR against all active composition rules in the context.
pub fn evaluate(ctx: &GameContext, gir: &Gir) -> EvaluationResult {
    let (matched, violated) = ctx.rule_index.evaluate(gir);
    let correctness = ctx.rule_index.correctness_score(gir);

    let feedback = generate_feedback(&matched, &violated, correctness);

    EvaluationResult {
        score: correctness,
        grade: Grade::from_score(correctness),
        matched_rules: matched,
        violated_rules: violated,
        feedback,
    }
}

fn generate_feedback(matched: &[String], violated: &[String], score: f64) -> Vec<String> {
    let mut feedback = Vec::new();

    if matched.is_empty() && violated.is_empty() {
        feedback.push("No composition rules are active.".into());
        return feedback;
    }

    let grade = Grade::from_score(score);
    match grade {
        Grade::Exact => {
            feedback.push(format!(
                "Excellent! All {} composition rules satisfied.",
                matched.len()
            ));
        }
        Grade::Close => {
            feedback.push(format!(
                "Good structure. {} rules matched, {} need work.",
                matched.len(),
                violated.len()
            ));
            for v in violated.iter().take(3) {
                feedback.push(format!("Missing: {v}"));
            }
        }
        Grade::Partial => {
            feedback.push(format!(
                "Partial match. {} of {} rules satisfied.",
                matched.len(),
                matched.len() + violated.len()
            ));
            for v in violated.iter().take(3) {
                feedback.push(format!("Missing: {v}"));
            }
        }
        Grade::Unrelated => {
            feedback.push("Structure does not match the required composition.".into());
            for v in violated.iter().take(3) {
                feedback.push(format!("Violated: {v}"));
            }
        }
    }

    feedback
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::GameConfig;

    #[test]
    fn evaluate_empty_rules() {
        let config = GameConfig {
            rules_json: String::new(),
            session_id: "test".into(),
        };
        let ctx = GameContext::from_config(&config).unwrap();
        let gir = ul_core::parser::parse("*").unwrap();
        let result = evaluate(&ctx, &gir);
        assert_eq!(result.score, 1.0);
        assert_eq!(result.grade, Grade::Exact);
    }

    #[test]
    fn evaluate_with_matching_rule() {
        let json = r#"[{
            "name": "has-point",
            "required_pattern": { "nodes": [{"id": "p", "node_type": "point"}], "edges": [] },
            "weight": 1.0,
            "difficulty": 1
        }]"#;
        let config = GameConfig {
            rules_json: json.into(),
            session_id: "test".into(),
        };
        let ctx = GameContext::from_config(&config).unwrap();
        let gir = ul_core::parser::parse("*").unwrap();
        let result = evaluate(&ctx, &gir);
        assert_eq!(result.score, 1.0);
        assert_eq!(result.matched_rules, vec!["has-point"]);
        assert!(result.violated_rules.is_empty());
    }

    #[test]
    fn evaluate_with_violated_rule() {
        let json = r#"[{
            "name": "needs-curve",
            "required_pattern": { "nodes": [{"id": "c", "node_type": "curve"}], "edges": [] },
            "weight": 1.0,
            "difficulty": 1
        }]"#;
        let config = GameConfig {
            rules_json: json.into(),
            session_id: "test".into(),
        };
        let ctx = GameContext::from_config(&config).unwrap();
        let gir = ul_core::parser::parse("*").unwrap();
        let result = evaluate(&ctx, &gir);
        assert_eq!(result.score, 0.0);
        assert!(result.matched_rules.is_empty());
        assert_eq!(result.violated_rules, vec!["needs-curve"]);
    }
}
