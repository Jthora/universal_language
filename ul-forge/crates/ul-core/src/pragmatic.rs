//! Pragmatic interface — surface ⟹ intended inference rules.
//!
//! Implements §9 of formal-foundations.md:
//! - SI-1: Scalar implicature ("some" ⟹ "not all")
//! - SI-2: Quantity scale ("warm" ⟹ "not hot") — requires scale registry
//! - SI-3: Disjunction ("A or B" ⟹ "not both A and B")
//! - CI-1: Conventional contrast ("but" ⟹ unexpected contrast)
//! - CI-2: Conventional appositive (parenthetical ⟹ projected assertion)
//! - CI-3: Forceful indirect request (query+ability ⟹ direct request)

use crate::types::gir::Gir;

/// An inference rule identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InferenceRule {
    /// Scalar implicature: "some" ⟹ "not all"
    SI1,
    /// Quantity scale: "warm" ⟹ "not hot"
    SI2,
    /// Disjunction: "A or B" ⟹ "not both"
    SI3,
    /// Conventional contrast: "but" ⟹ unexpected contrast
    CI1,
    /// Conventional appositive: parenthetical ⟹ projected assertion
    CI2,
    /// Forceful indirect request: query+ability ⟹ direct request
    CI3,
}

/// A pragmatic inference linking a surface expression to its intended meaning.
#[derive(Debug, Clone)]
pub struct PragmaticInference {
    /// Which inference rule produced this.
    pub rule: InferenceRule,
    /// The surface-level GIR (what was literally said).
    pub surface: Gir,
    /// The intended-level GIR (what was implicated).
    pub intended: Gir,
}

/// Run all applicable inference rules on a surface GIR.
///
/// Returns a (possibly empty) list of inferences. Each inference contains
/// both the matching surface fragment and the inferred intended meaning.
pub fn infer(surface: &Gir) -> Vec<PragmaticInference> {
    let mut results = Vec::new();

    // SI-1: Detect partial quantification → infer negation of universal
    if let Some(inf) = infer_si1(surface) {
        results.push(inf);
    }

    // SI-3: Detect disjunction → infer not-both
    if let Some(inf) = infer_si3(surface) {
        results.push(inf);
    }

    // CI-3: Detect query force + ability predicate → infer direct request
    if let Some(inf) = infer_ci3(surface) {
        results.push(inf);
    }

    results
}

/// SI-1: Scalar implicature — detect quantify with partial measure.
///
/// If the root is a quantify pattern (enclosure with measure p < 1.0 on a modifier),
/// infer the negation of the universal quantification (p = 1.0).
fn infer_si1(surface: &Gir) -> Option<PragmaticInference> {
    // Look for any angle node (modifier) with measure < 1.0 that looks like a quantifier
    // This is a heuristic: quantify operations produce angle nodes with measures in [0, 1]
    let quantifier_node = surface.nodes.iter().find(|n| {
        n.node_type == crate::types::node::NodeType::Angle
            && n.sort == crate::types::sort::Sort::Modifier
            && n.measure.is_some_and(|m| m > 0.0 && m < 1.0)
    })?;

    let measure = quantifier_node.measure.unwrap();

    // Build intended: negate the universal version (measure = 1.0)
    // For now, create a simple annotation rather than a full GIR transform
    let mut intended = surface.clone();
    // Mark the intended as having the negated universal reading
    if let Some(node) = intended.nodes.iter_mut().find(|n| n.id == quantifier_node.id) {
        node.label = Some(format!("¬∀ (from ∃ p={})", measure));
    }

    Some(PragmaticInference {
        rule: InferenceRule::SI1,
        surface: surface.clone(),
        intended,
    })
}

/// SI-3: Disjunction implicature — detect disjoin pattern.
///
/// If the GIR contains a disjoin pattern (two sub-graphs joined via Adjacent edges
/// inside a labeled "disjoin" enclosure), infer negate(conjoin(a, b)).
fn infer_si3(surface: &Gir) -> Option<PragmaticInference> {
    // Detect a node labeled "disjoin" (produced by composer::disjoin)
    let disjoin_node = surface
        .nodes
        .iter()
        .find(|n| n.label.as_deref() == Some("disjoin"))?;

    let mut intended = surface.clone();
    // Relabel to indicate the not-both inference
    if let Some(node) = intended.nodes.iter_mut().find(|n| n.id == disjoin_node.id) {
        node.label = Some("¬(A∧B) [from A∨B]".to_string());
    }

    Some(PragmaticInference {
        rule: InferenceRule::SI3,
        surface: surface.clone(),
        intended,
    })
}

/// CI-3: Indirect request — detect query force on ability predicate.
///
/// Pattern: query { predicate(hearer, can/able, action) }
/// Infers: direct { predicate(hearer, do, action) }
fn infer_ci3(surface: &Gir) -> Option<PragmaticInference> {
    use crate::types::node::PerformativeForce;

    // Find a node with Query force
    let query_node = surface.nodes.iter().find(|n| {
        n.force == Some(PerformativeForce::Query)
    })?;

    // Check if any child has a label suggesting ability ("can", "able", "ability")
    let children = surface.children_of(&query_node.id);
    let has_ability = children.iter().any(|cid| {
        surface.node(cid).is_some_and(|n| {
            n.label.as_ref().is_some_and(|l| {
                let lower = l.to_lowercase();
                lower.contains("can") || lower.contains("able") || lower.contains("ability")
            })
        })
    });

    if !has_ability {
        return None;
    }

    // Build intended: change force from Query to Direct
    let mut intended = surface.clone();
    if let Some(node) = intended.nodes.iter_mut().find(|n| n.id == query_node.id) {
        node.force = Some(PerformativeForce::Direct);
        if let Some(ref mut label) = node.label {
            label.push_str(" [indirect request → direct]");
        } else {
            node.label = Some("[indirect request → direct]".to_string());
        }
    }

    Some(PragmaticInference {
        rule: InferenceRule::CI3,
        surface: surface.clone(),
        intended,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::edge::Edge;
    use crate::types::gir::Gir;
    use crate::types::node::{EnclosureShape, Node, PerformativeForce};
    use crate::types::sort::Sort;

    #[test]
    fn si1_detects_partial_quantifier() {
        // Build a GIR with a partial quantifier (measure = 0.5 → "some")
        let root = Node::enclosure("q1", EnclosureShape::Circle)
            .with_label("quantify");
        let angle = Node::angle("a1", 0.5); // measure < 1.0
        let entity = Node::point("e1").with_label("things");
        let gir = Gir::new(
            "q1",
            vec![root, angle, entity],
            vec![
                Edge::contains("q1", "a1"),
                Edge::contains("q1", "e1"),
            ],
        );

        let inferences = infer(&gir);
        assert!(
            inferences.iter().any(|i| i.rule == InferenceRule::SI1),
            "Expected SI-1 inference from partial quantifier"
        );
    }

    #[test]
    fn si1_does_not_trigger_on_universal() {
        // measure = 1.0 → no scalar implicature (already universal)
        let root = Node::enclosure("q1", EnclosureShape::Circle);
        let angle = {
            let mut a = Node::angle("a1", 1.0);
            a.sort = Sort::Modifier;
            a
        };
        let gir = Gir::new(
            "q1",
            vec![root, angle],
            vec![Edge::contains("q1", "a1")],
        );

        let inferences = infer(&gir);
        assert!(
            !inferences.iter().any(|i| i.rule == InferenceRule::SI1),
            "Should not trigger SI-1 on universal quantifier"
        );
    }

    #[test]
    fn si3_detects_disjoin() {
        let root = Node::enclosure("d1", EnclosureShape::Circle)
            .with_label("disjoin");
        let a = Node::point("a1");
        let b = Node::point("b1");
        let gir = Gir::new(
            "d1",
            vec![root, a, b],
            vec![
                Edge::contains("d1", "a1"),
                Edge::contains("d1", "b1"),
            ],
        );

        let inferences = infer(&gir);
        assert!(
            inferences.iter().any(|i| i.rule == InferenceRule::SI3),
            "Expected SI-3 inference from disjoin pattern"
        );
    }

    #[test]
    fn ci3_detects_indirect_request() {
        let root = Node::enclosure("q1", EnclosureShape::Circle)
            .with_force(PerformativeForce::Query);
        let ability = Node::point("a1").with_label("can");
        let action = Node::point("e1").with_label("pass the salt");
        let gir = Gir::new(
            "q1",
            vec![root, ability, action],
            vec![
                Edge::contains("q1", "a1"),
                Edge::contains("q1", "e1"),
            ],
        );

        let inferences = infer(&gir);
        assert!(
            inferences.iter().any(|i| i.rule == InferenceRule::CI3),
            "Expected CI-3 inference from query+ability"
        );
        // Verify the intended has Direct force
        let ci3 = inferences.iter().find(|i| i.rule == InferenceRule::CI3).unwrap();
        let intended_root = ci3.intended.node(&ci3.intended.root).unwrap();
        assert_eq!(intended_root.force, Some(PerformativeForce::Direct));
    }

    #[test]
    fn ci3_does_not_trigger_without_ability() {
        let root = Node::enclosure("q1", EnclosureShape::Circle)
            .with_force(PerformativeForce::Query);
        let entity = Node::point("e1").with_label("something");
        let gir = Gir::new(
            "q1",
            vec![root, entity],
            vec![Edge::contains("q1", "e1")],
        );

        let inferences = infer(&gir);
        assert!(
            !inferences.iter().any(|i| i.rule == InferenceRule::CI3),
            "Should not trigger CI-3 without ability keyword"
        );
    }

    #[test]
    fn empty_gir_produces_no_inferences() {
        let root = Node::point("p1");
        let gir = Gir::new("p1", vec![root], vec![]);
        let inferences = infer(&gir);
        assert!(inferences.is_empty());
    }
}
