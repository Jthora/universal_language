//! Sequence validation: check ordering constraints across a series of GIR structures.
//!
//! Validates that a sequence of UL compositions demonstrates proper construction
//! order: continuity of primitives between steps and increasing complexity.

use ul_core::Gir;

use crate::context::GameContext;
use crate::types::SequenceResult;

/// Validate that a sequence of GIR structures satisfies ordering constraints.
///
/// Each consecutive pair must share at least one common primitive type,
/// and complexity should generally increase (non-decreasing node count).
pub fn validate_sequence(ctx: &GameContext, glyphs: &[Gir]) -> SequenceResult {
    if glyphs.len() < 2 {
        return SequenceResult {
            valid: true,
            errors: vec![],
            pair_scores: vec![],
        };
    }

    let mut errors = Vec::new();
    let mut pair_scores = Vec::new();

    for i in 0..glyphs.len() - 1 {
        let score = pair_ordering_score(&glyphs[i], &glyphs[i + 1], ctx);
        pair_scores.push(score);

        if score < 0.3 {
            errors.push(format!(
                "Weak ordering between glyph {} and {}: score {:.2}",
                i,
                i + 1,
                score
            ));
        }
    }

    SequenceResult {
        valid: errors.is_empty(),
        errors,
        pair_scores,
    }
}

/// Score how well glyph `a` transitions to glyph `b` (0.0–1.0).
///
/// Factors: primitive continuity, complexity progression, rule satisfaction.
fn pair_ordering_score(a: &Gir, b: &Gir, ctx: &GameContext) -> f64 {
    let continuity = primitive_continuity(a, b);
    let progression = complexity_progression(a, b);
    let correctness = ctx.rule_index.correctness_score(b);

    continuity * 0.4 + progression * 0.3 + correctness * 0.3
}

/// Fraction of primitive types in `a` that also appear in `b`.
fn primitive_continuity(a: &Gir, b: &Gir) -> f64 {
    use std::collections::HashSet;
    use ul_core::types::node::NodeType;

    let types_a: HashSet<NodeType> = a.nodes.iter().map(|n| n.node_type).collect();
    let types_b: HashSet<NodeType> = b.nodes.iter().map(|n| n.node_type).collect();

    if types_a.is_empty() {
        return 1.0;
    }

    let shared = types_a.intersection(&types_b).count();
    shared as f64 / types_a.len() as f64
}

/// Score for non-decreasing complexity: 1.0 if b >= a nodes, decays otherwise.
fn complexity_progression(a: &Gir, b: &Gir) -> f64 {
    let ca = a.nodes.len() as f64;
    let cb = b.nodes.len() as f64;

    if cb >= ca {
        1.0
    } else if ca == 0.0 {
        1.0
    } else {
        (cb / ca).max(0.0)
    }
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
    fn single_glyph_always_valid() {
        let gir = ul_core::parser::parse("*").unwrap();
        let result = validate_sequence(&ctx(), &[gir]);
        assert!(result.valid);
        assert!(result.pair_scores.is_empty());
    }

    #[test]
    fn increasing_complexity_valid() {
        let ctx = ctx();
        // Point `*` → enclosure-with-point `/0{*}`: increasing complexity
        let g1 = ul_core::parser::parse("*").unwrap();
        let g2 = ul_core::parser::parse("/0{*}").unwrap();
        let result = validate_sequence(&ctx, &[g1, g2]);
        assert!(result.valid, "errors: {:?}", result.errors);
        assert_eq!(result.pair_scores.len(), 1);
        assert!(result.pair_scores[0] > 0.5);
    }

    #[test]
    fn empty_sequence_valid() {
        let result = validate_sequence(&ctx(), &[]);
        assert!(result.valid);
    }
}
