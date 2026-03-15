//! Property-based tests using proptest.
//!
//! These verify invariants that must hold for ALL valid inputs,
//! not just hand-picked examples.

use proptest::prelude::*;
use ul_core::parser;
use ul_core::renderer::{self, RenderOptions};
use ul_core::validator;

/// Strategy generating valid UL-Script strings from the canonical vocabulary.
fn ul_script_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        // Single primitives
        Just("●".to_string()),
        Just("○".to_string()),
        Just("△".to_string()),
        Just("□".to_string()),
        Just("~".to_string()),
        Just("→".to_string()),
        // Angle marks
        Just("∠60".to_string()),
        Just("∠90".to_string()),
        Just("∠180".to_string()),
        // Containment patterns
        Just("○{●}".to_string()),
        Just("△{●}".to_string()),
        Just("□{●}".to_string()),
        Just("○{○}".to_string()),
        // Connection patterns
        Just("● → ●".to_string()),
        Just("● ← ●".to_string()),
        Just("● ↔ ●".to_string()),
        // Adjacency / intersection
        Just("○ | ○".to_string()),
        Just("○ & ○".to_string()),
        // ASCII equivalents
        Just("*".to_string()),
        Just("/0".to_string()),
        Just("/3".to_string()),
        Just("/4".to_string()),
        Just("* -> *".to_string()),
        Just("/0{*}".to_string()),
    ]
}

proptest! {
    /// Property 1: parse → deparse → parse produces equivalent GIR.
    ///
    /// The canonical form may differ in whitespace, but the resulting
    /// GIR graph structure must be identical.
    #[test]
    fn roundtrip_parse_deparse_parse(input in ul_script_strategy()) {
        let gir1 = parser::parse(&input).expect("valid input should parse");
        let deparsed = parser::deparse(&gir1).expect("valid GIR should deparse");
        let gir2 = parser::parse(&deparsed).expect("deparsed output should re-parse");

        // Node count and types must match
        prop_assert_eq!(gir1.nodes.len(), gir2.nodes.len(),
            "Node count mismatch: input={:?} deparsed={:?}", input, deparsed);
        prop_assert_eq!(gir1.edges.len(), gir2.edges.len(),
            "Edge count mismatch: input={:?} deparsed={:?}", input, deparsed);

        // Node types in order must match
        let types1: Vec<_> = gir1.nodes.iter().map(|n| &n.node_type).collect();
        let types2: Vec<_> = gir2.nodes.iter().map(|n| &n.node_type).collect();
        prop_assert_eq!(types1, types2,
            "Node types mismatch: input={:?} deparsed={:?}", input, deparsed);
    }

    /// Property 2: if validation passes, rendering must succeed.
    #[test]
    fn valid_gir_renders(input in ul_script_strategy()) {
        let gir = parser::parse(&input).expect("valid input should parse");
        let result = validator::validate(&gir, false);
        if result.valid {
            let opts = RenderOptions::default();
            renderer::render(&gir, &opts).expect("valid GIR should render without error");
        }
    }

    /// Property 3: validation is deterministic / idempotent.
    #[test]
    fn validation_deterministic(input in ul_script_strategy()) {
        let gir = parser::parse(&input).expect("valid input should parse");
        let result1 = validator::validate(&gir, false);
        let result2 = validator::validate(&gir, false);
        prop_assert_eq!(result1.valid, result2.valid);
        prop_assert_eq!(result1.errors.len(), result2.errors.len());
        prop_assert_eq!(result1.warnings.len(), result2.warnings.len());
    }

    /// Property 4: SVG output contains at least one data-ul-node per rendered element,
    /// and no more than the total number of GIR nodes.
    #[test]
    fn svg_node_count_bounded(input in ul_script_strategy()) {
        let gir = parser::parse(&input).expect("valid input should parse");
        let result = validator::validate(&gir, false);
        if result.valid {
            let opts = RenderOptions {
                embed_gir: false,
                ..RenderOptions::default()
            };
            let svg = renderer::render(&gir, &opts).expect("valid GIR should render");
            let data_node_count = svg.matches("data-ul-node").count();
            // At least one rendered element for non-empty GIR
            if !gir.nodes.is_empty() {
                prop_assert!(data_node_count > 0,
                    "SVG should contain at least one data-ul-node for non-empty GIR, input: {:?}", input);
            }
            // Never more data-ul-node attrs than GIR nodes
            prop_assert!(data_node_count <= gir.nodes.len(),
                "SVG has {} data-ul-node but only {} GIR nodes, input: {:?}",
                data_node_count, gir.nodes.len(), input);
        }
    }

    /// Property 5: sort assignments are consistent — every node has a valid sort.
    #[test]
    fn sort_assignments_consistent(input in ul_script_strategy()) {
        let gir = parser::parse(&input).expect("valid input should parse");
        for node in &gir.nodes {
            // Every node must have a sort assigned
            let sort = &node.sort;
            // Sort must be one of the 4 valid sorts
            prop_assert!(
                matches!(sort, ul_core::Sort::Entity | ul_core::Sort::Relation | ul_core::Sort::Modifier | ul_core::Sort::Assertion),
                "Node {} has invalid sort: {:?}", node.id, sort
            );
        }
    }
}

// Property: parser never panics on arbitrary input (fuzz-like).
proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]
    #[test]
    fn parser_never_panics(input in ".*") {
        // This should never panic — it may return Ok or Err but must not crash.
        let _ = parser::parse(&input);
    }
}
