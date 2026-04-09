//! Performative extension — illocutionary force parameter φ on assertion nodes.
//!
//! Implements §8.1–8.7 of formal-foundations.md:
//! - 6 force types: assert, query, direct, commit, express, declare
//! - Force composition rules FC1–FC5
//! - Force assignment on assertion Gir roots

use crate::types::gir::Gir;
use crate::types::node::PerformativeForce;
use crate::types::sort::Sort;
use crate::{UlError, UlResult};

/// Assign an illocutionary force to the root assertion of a Gir.
///
/// The root node must be Assertion sort. Returns a cloned Gir with the
/// `force` field set on the root node.
pub fn with_force(gir: &Gir, force: PerformativeForce) -> UlResult<Gir> {
    let root_node = gir.node(&gir.root).ok_or_else(|| UlError::Render {
        message: format!("Root node '{}' not found", gir.root),
    })?;

    if root_node.sort != Sort::Assertion && root_node.sort != Sort::Entity {
        return Err(UlError::Render {
            message: format!(
                "Cannot assign force to non-assertion root (sort={:?})",
                root_node.sort
            ),
        });
    }

    let mut result = gir.clone();
    if let Some(node) = result.nodes.iter_mut().find(|n| n.id == result.root) {
        node.force = Some(force);
    }
    Ok(result)
}

/// Check whether two forces are compatible for conjunction (FC2).
///
/// Compatible pairs: same force, or Assert + anything (Assert is the default/neutral).
pub fn forces_compatible(a: PerformativeForce, b: PerformativeForce) -> bool {
    a == b || a == PerformativeForce::Assert || b == PerformativeForce::Assert
}

/// Resolve the dominant force when conjoining two forced assertions (FC2).
///
/// Assert is neutral — any non-Assert force dominates.
/// If both are non-Assert and different, returns None (incompatible).
pub fn resolve_conjunction_force(
    a: Option<PerformativeForce>,
    b: Option<PerformativeForce>,
) -> Option<PerformativeForce> {
    match (a, b) {
        (None, None) => None,
        (Some(f), None) | (None, Some(f)) => Some(f),
        (Some(fa), Some(fb)) if fa == fb => Some(fa),
        (Some(PerformativeForce::Assert), Some(f)) | (Some(f), Some(PerformativeForce::Assert)) => {
            Some(f)
        }
        _ => None, // incompatible
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::edge::Edge;
    use crate::types::gir::Gir;
    use crate::types::node::{EnclosureShape, Node};

    fn make_assertion_gir() -> Gir {
        let root = Node::enclosure("a1", EnclosureShape::Circle).with_sort(Sort::Assertion);
        let inner = Node::point("p1");
        Gir::new(
            "a1",
            vec![root, inner],
            vec![Edge::contains("a1", "p1")],
        )
    }

    #[test]
    fn with_force_sets_force_on_root() {
        let gir = make_assertion_gir();
        let result = with_force(&gir, PerformativeForce::Query).unwrap();
        let root = result.node(&result.root).unwrap();
        assert_eq!(root.force, Some(PerformativeForce::Query));
    }

    #[test]
    fn with_force_all_six_variants() {
        let gir = make_assertion_gir();
        for force in [
            PerformativeForce::Assert,
            PerformativeForce::Query,
            PerformativeForce::Direct,
            PerformativeForce::Commit,
            PerformativeForce::Express,
            PerformativeForce::Declare,
        ] {
            let result = with_force(&gir, force).unwrap();
            let root = result.node(&result.root).unwrap();
            assert_eq!(root.force, Some(force));
        }
    }

    #[test]
    fn with_force_rejects_non_assertion_root() {
        let root = Node::line("l1", true);
        let gir = Gir::new("l1", vec![root], vec![]);
        let result = with_force(&gir, PerformativeForce::Query);
        assert!(result.is_err());
    }

    #[test]
    fn force_symbols() {
        assert_eq!(PerformativeForce::Assert.symbol(), ".");
        assert_eq!(PerformativeForce::Query.symbol(), "?");
        assert_eq!(PerformativeForce::Direct.symbol(), "!");
        assert_eq!(PerformativeForce::Commit.symbol(), "⊢");
        assert_eq!(PerformativeForce::Express.symbol(), "♡");
        assert_eq!(PerformativeForce::Declare.symbol(), "⊨");
    }

    #[test]
    fn conjunction_force_resolution() {
        // Same force → that force
        assert_eq!(
            resolve_conjunction_force(Some(PerformativeForce::Query), Some(PerformativeForce::Query)),
            Some(PerformativeForce::Query)
        );
        // Assert + X → X
        assert_eq!(
            resolve_conjunction_force(Some(PerformativeForce::Assert), Some(PerformativeForce::Direct)),
            Some(PerformativeForce::Direct)
        );
        // Different non-Assert → None (incompatible)
        assert_eq!(
            resolve_conjunction_force(Some(PerformativeForce::Query), Some(PerformativeForce::Direct)),
            None
        );
        // None + Some → Some
        assert_eq!(
            resolve_conjunction_force(None, Some(PerformativeForce::Commit)),
            Some(PerformativeForce::Commit)
        );
    }

    #[test]
    fn forces_compatible_checks() {
        assert!(forces_compatible(PerformativeForce::Assert, PerformativeForce::Query));
        assert!(forces_compatible(PerformativeForce::Query, PerformativeForce::Query));
        assert!(!forces_compatible(PerformativeForce::Query, PerformativeForce::Direct));
    }
}
