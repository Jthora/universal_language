//! Modal operator implementations for Universal Language.
//!
//! Implements □ (necessity), ◇ (possibility), and □→ (counterfactual)
//! as composed GIR patterns using distinguished elements from formal-foundations.md §7.
//!
//! These operators use 0 new primitives — they compose from existing Σ_UL operations
//! with 7 distinguished elements (world entities + accessibility relations).

use crate::distinguished::DistinguishedRegistry;
use crate::types::edge::Edge;
use crate::types::gir::{Gir, ModalContext};
use crate::types::node::{EnclosureShape, Node};
use crate::types::sort::Sort;
use crate::{UlError, UlResult};

/// Build □ (necessity): "In all accessible worlds, assertion holds."
///
/// Structure: ∀w′ (R(w_current, w′) → satisfies(w′, a))
/// Encodes as: outer Enclosure(Assertion) containing:
///   - w_current (Entity, world)
///   - w_prime (Entity, world)
///   - AccessibleFrom edge: w_current → w_prime via `relation`
///   - inner assertion with satisfy-link from w_prime
pub fn necessity(
    registry: &DistinguishedRegistry,
    assertion: &Gir,
    relation: &str,
) -> UlResult<Gir> {
    let _ = registry
        .gir(relation)
        .ok_or_else(|| UlError::Render { message: format!("Unknown accessibility relation: {relation}") })?;

    // Build world entities
    let w_current = Node::point("w_current")
        .with_label("w_current")
        .with_sort(Sort::Entity);
    let w_prime = Node::point("w_prime")
        .with_label("w′")
        .with_sort(Sort::Entity);

    // Build the inner assertion content (remapped)
    let remap_prefix = "nec_inner_";
    let inner_nodes: Vec<Node> = assertion
        .nodes
        .iter()
        .map(|n| {
            let mut remapped = n.clone();
            remapped.id = format!("{remap_prefix}{}", n.id);
            remapped
        })
        .collect();
    let inner_edges: Vec<Edge> = assertion
        .edges
        .iter()
        .map(|e| Edge::new(
            format!("{remap_prefix}{}", e.source),
            format!("{remap_prefix}{}", e.target),
            e.edge_type,
        ))
        .collect();
    let inner_root = format!("{remap_prefix}{}", assertion.root);

    // Build satisfy link: w_prime --satisfies--> inner_assertion_root
    let satisfy_edge = Edge::connects("w_prime", &inner_root);

    // Accessibility edge: w_current --> w_prime
    let access_edge = Edge::accessible_from("w_current", "w_prime");

    // Outer enclosure wrapping everything (sort=Assertion for modal claim)
    let outer = Node::enclosure("nec_root", EnclosureShape::Square)
        .with_sort(Sort::Entity)
        .with_label(format!("□_{relation}"));

    let mut all_nodes = vec![outer, w_current, w_prime];
    all_nodes.extend(inner_nodes);

    let mut all_edges = vec![
        Edge::contains("nec_root", "w_current"),
        Edge::contains("nec_root", "w_prime"),
        Edge::contains("nec_root", &inner_root),
        access_edge,
        satisfy_edge,
    ];
    all_edges.extend(inner_edges);

    let access_idx = 3; // index of access_edge in all_edges

    let gir = Gir::new("nec_root", all_nodes, all_edges)
        .with_modal_context(ModalContext {
            world_nodes: vec!["w_current".into(), "w_prime".into()],
            accessibility_edges: vec![access_idx],
        });

    Ok(gir)
}

/// Build ◇ (possibility): "In some accessible world, assertion holds."
///
/// Structure: ∃w′ (R(w_current, w′) ∧ satisfies(w′, a))
pub fn possibility(
    registry: &DistinguishedRegistry,
    assertion: &Gir,
    relation: &str,
) -> UlResult<Gir> {
    let _ = registry
        .gir(relation)
        .ok_or_else(|| UlError::Render { message: format!("Unknown accessibility relation: {relation}") })?;

    let w_current = Node::point("w_current")
        .with_label("w_current")
        .with_sort(Sort::Entity);
    let w_prime = Node::point("w_prime")
        .with_label("w′")
        .with_sort(Sort::Entity);

    let remap_prefix = "pos_inner_";
    let inner_nodes: Vec<Node> = assertion
        .nodes
        .iter()
        .map(|n| {
            let mut remapped = n.clone();
            remapped.id = format!("{remap_prefix}{}", n.id);
            remapped
        })
        .collect();
    let inner_edges: Vec<Edge> = assertion
        .edges
        .iter()
        .map(|e| Edge::new(
            format!("{remap_prefix}{}", e.source),
            format!("{remap_prefix}{}", e.target),
            e.edge_type,
        ))
        .collect();
    let inner_root = format!("{remap_prefix}{}", assertion.root);

    let satisfy_edge = Edge::connects("w_prime", &inner_root);
    let access_edge = Edge::accessible_from("w_current", "w_prime");

    let outer = Node::enclosure("pos_root", EnclosureShape::Square)
        .with_sort(Sort::Entity)
        .with_label(format!("◇_{relation}"));

    let mut all_nodes = vec![outer, w_current, w_prime];
    all_nodes.extend(inner_nodes);

    let mut all_edges = vec![
        Edge::contains("pos_root", "w_current"),
        Edge::contains("pos_root", "w_prime"),
        Edge::contains("pos_root", &inner_root),
        access_edge,
        satisfy_edge,
    ];
    all_edges.extend(inner_edges);

    let access_idx = 3;

    let gir = Gir::new("pos_root", all_nodes, all_edges)
        .with_modal_context(ModalContext {
            world_nodes: vec!["w_current".into(), "w_prime".into()],
            accessibility_edges: vec![access_idx],
        });

    Ok(gir)
}

/// Build □→ (counterfactual): "In all closest antecedent-worlds, consequent holds."
///
/// Lewis semantics: φ □→ ψ iff in all closest φ-worlds, ψ holds.
/// Uses r_closeness for world ordering.
pub fn counterfactual(
    registry: &DistinguishedRegistry,
    antecedent: &Gir,
    consequent: &Gir,
) -> UlResult<Gir> {
    let _ = registry
        .gir("r_closeness")
        .ok_or_else(|| UlError::Render { message: "Missing r_closeness in registry".into() })?;

    let w_current = Node::point("w_current")
        .with_label("w_current")
        .with_sort(Sort::Entity);
    let w_closest = Node::point("w_closest")
        .with_label("w_closest")
        .with_sort(Sort::Entity);

    // Remap antecedent
    let ante_prefix = "cf_ante_";
    let ante_nodes: Vec<Node> = antecedent
        .nodes
        .iter()
        .map(|n| {
            let mut r = n.clone();
            r.id = format!("{ante_prefix}{}", n.id);
            r
        })
        .collect();
    let ante_edges: Vec<Edge> = antecedent
        .edges
        .iter()
        .map(|e| Edge::new(
            format!("{ante_prefix}{}", e.source),
            format!("{ante_prefix}{}", e.target),
            e.edge_type,
        ))
        .collect();
    let ante_root = format!("{ante_prefix}{}", antecedent.root);

    // Remap consequent
    let cons_prefix = "cf_cons_";
    let cons_nodes: Vec<Node> = consequent
        .nodes
        .iter()
        .map(|n| {
            let mut r = n.clone();
            r.id = format!("{cons_prefix}{}", n.id);
            r
        })
        .collect();
    let cons_edges: Vec<Edge> = consequent
        .edges
        .iter()
        .map(|e| Edge::new(
            format!("{cons_prefix}{}", e.source),
            format!("{cons_prefix}{}", e.target),
            e.edge_type,
        ))
        .collect();
    let cons_root = format!("{cons_prefix}{}", consequent.root);

    // Closeness edge: w_current --closeness--> w_closest
    let closeness_edge = Edge::accessible_from("w_current", "w_closest");
    // w_closest satisfies antecedent
    let ante_satisfy = Edge::connects("w_closest", &ante_root);
    // w_closest satisfies consequent
    let cons_satisfy = Edge::connects("w_closest", &cons_root);

    let outer = Node::enclosure("cf_root", EnclosureShape::Square)
        .with_sort(Sort::Entity)
        .with_label("□→");

    let mut all_nodes = vec![outer, w_current, w_closest];
    all_nodes.extend(ante_nodes);
    all_nodes.extend(cons_nodes);

    let mut all_edges = vec![
        Edge::contains("cf_root", "w_current"),
        Edge::contains("cf_root", "w_closest"),
        Edge::contains("cf_root", &ante_root),
        Edge::contains("cf_root", &cons_root),
        closeness_edge,
        ante_satisfy,
        cons_satisfy,
    ];
    all_edges.extend(ante_edges);
    all_edges.extend(cons_edges);

    let closeness_idx = 4; // index of closeness_edge in all_edges

    let gir = Gir::new("cf_root", all_nodes, all_edges)
        .with_modal_context(ModalContext {
            world_nodes: vec!["w_current".into(), "w_closest".into()],
            accessibility_edges: vec![closeness_idx],
        });

    Ok(gir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distinguished::default_registry;
    use crate::parser;
    use crate::types::edge::EdgeType;
    use crate::validator;

    fn simple_assertion() -> Gir {
        parser::parse("□{● → ●}").unwrap()
    }

    #[test]
    fn necessity_creates_valid_gir() {
        let reg = default_registry();
        let assertion = simple_assertion();
        let result = necessity(&reg, &assertion, "r_alethic").unwrap();

        assert_eq!(result.root, "nec_root");
        assert!(result.modal_context.is_some());
        let mc = result.modal_context.unwrap();
        assert_eq!(mc.world_nodes.len(), 2);
        assert!(!mc.accessibility_edges.is_empty());

        // Has AccessibleFrom edge
        assert!(result.edges.iter().any(|e| e.edge_type == EdgeType::AccessibleFrom));
    }

    #[test]
    fn possibility_creates_valid_gir() {
        let reg = default_registry();
        let assertion = simple_assertion();
        let result = possibility(&reg, &assertion, "r_K_agent").unwrap();

        assert_eq!(result.root, "pos_root");
        assert!(result.modal_context.is_some());
        assert!(result.edges.iter().any(|e| e.edge_type == EdgeType::AccessibleFrom));
    }

    #[test]
    fn counterfactual_creates_valid_gir() {
        let reg = default_registry();
        let ante = simple_assertion();
        let cons = parser::parse("□{● → ●}").unwrap();
        let result = counterfactual(&reg, &ante, &cons).unwrap();

        assert_eq!(result.root, "cf_root");
        assert!(result.modal_context.is_some());
        let mc = result.modal_context.unwrap();
        assert_eq!(mc.world_nodes.len(), 2);
        assert!(result.edges.iter().any(|e| e.edge_type == EdgeType::AccessibleFrom));
    }

    #[test]
    fn necessity_unknown_relation_errors() {
        let reg = default_registry();
        let assertion = simple_assertion();
        let result = necessity(&reg, &assertion, "r_nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn necessity_gir_validates() {
        let reg = default_registry();
        let assertion = simple_assertion();
        let result = necessity(&reg, &assertion, "r_alethic").unwrap();
        let validation = validator::validate(&result, false);
        // Should validate (may have warnings but no errors)
        assert!(
            validation.errors.is_empty(),
            "Validation errors: {:?}",
            validation.errors
        );
    }

    #[test]
    fn possibility_gir_validates() {
        let reg = default_registry();
        let assertion = simple_assertion();
        let result = possibility(&reg, &assertion, "r_alethic").unwrap();
        let validation = validator::validate(&result, false);
        assert!(
            validation.errors.is_empty(),
            "Validation errors: {:?}",
            validation.errors
        );
    }

    #[test]
    fn counterfactual_gir_validates() {
        let reg = default_registry();
        let ante = simple_assertion();
        let cons = parser::parse("●").unwrap();
        let result = counterfactual(&reg, &ante, &cons).unwrap();
        let validation = validator::validate(&result, false);
        assert!(
            validation.errors.is_empty(),
            "Validation errors: {:?}",
            validation.errors
        );
    }

    #[test]
    fn modal_gir_renders_svg() {
        let reg = default_registry();
        let assertion = simple_assertion();
        let result = necessity(&reg, &assertion, "r_alethic").unwrap();
        let opts = crate::renderer::RenderOptions::default();
        let svg = crate::renderer::render(&result, &opts);
        assert!(svg.is_ok(), "SVG render failed: {:?}", svg.err());
        assert!(svg.unwrap().contains("<svg"));
    }
}
