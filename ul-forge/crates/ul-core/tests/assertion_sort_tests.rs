//! Regression tests for F-038: the Assertion sort was unreachable from text.
//!
//! WHY THIS FILE EXISTS
//!   `transform_force_annotation`'s own doc comment said "Creates an assertion-sort enclosure",
//!   but the code called `Node::enclosure(..)` without `.with_sort(Sort::Assertion)`. Consequence:
//!   NO string the parser accepted ever produced an Assertion-sorted root, so `negate`, `embed`,
//!   `conjoin`, `disjoin`, `bind` and `modify_assertion` were unreachable from UL-Script, and
//!   README REQ-1's "expressive adequacy" was false in code rather than merely unproven — a
//!   negation could not be written.
//!
//!   The 380-test suite passed throughout, because nothing asserted the sort of a parsed force
//!   annotation. That is the gap these tests close.

use ul_core::composer;
use ul_core::parser;
use ul_core::types::sort::Sort;

fn root_sort(src: &str) -> Sort {
    let gir = parser::parse(src).expect("parses");
    gir.node(&gir.root).expect("root node").sort
}

#[test]
fn every_force_annotation_parses_to_an_assertion() {
    // The whole point of F-038: these are assertions, not decorated entities.
    for src in [
        "assert{*}",
        "query{*}",
        "direct{*}",
        "commit{*}",
        "express{*}",
        "declare{*}",
        "assert{* -> /3}",
    ] {
        assert_eq!(root_sort(src), Sort::Assertion, "root sort of `{src}`");
    }
}

#[test]
fn a_bare_primitive_is_still_an_entity() {
    // Negative control: the fix must not promote everything to Assertion.
    for src in ["*", "/0", "/3", "/0{*}", "/0{* -> /3}"] {
        assert_eq!(root_sort(src), Sort::Entity, "root sort of `{src}`");
    }
    assert_eq!(root_sort("->"), Sort::Relation);
}

#[test]
fn negation_conjunction_and_disjunction_are_writable() {
    // Before F-038 every one of these failed with
    // "expected root sort Assertion, got Entity" — a negation could not be expressed at all.
    let a = parser::parse("assert{* -> /3}").expect("parses");
    let b = parser::parse("assert{/4}").expect("parses");

    let neg = composer::negate(&a).expect("negate an assertion");
    assert_eq!(neg.node(&neg.root).unwrap().sort, Sort::Assertion);

    let conj = composer::conjoin(&a, &b).expect("conjoin two assertions");
    assert_eq!(conj.node(&conj.root).unwrap().sort, Sort::Assertion);

    let disj = composer::disjoin(&a, &b).expect("disjoin two assertions");
    assert_eq!(disj.node(&disj.root).unwrap().sort, Sort::Assertion);

    // `embed` is NOMINALIZATION — it deliberately takes an assertion OUT of the assertion sort and
    // makes it an entity you can talk about ("that it rains" as a noun phrase). It was unreachable
    // before F-038 for the same reason as the others: its INPUT must be an Assertion.
    let emb = composer::embed(&a).expect("embed an assertion");
    assert_eq!(emb.node(&emb.root).unwrap().sort, Sort::Entity);
}

#[test]
fn an_entity_is_still_rejected_where_an_assertion_is_required() {
    // The sort discipline must still bite; the fix widens what EXISTS, not what is ACCEPTED.
    let e = parser::parse("/0{*}").expect("parses");
    assert!(composer::negate(&e).is_err(), "negate must reject an entity");
    assert!(composer::conjoin(&e, &e).is_err(), "conjoin must reject entities");
}

#[test]
fn an_assertion_frame_is_a_valid_container() {
    // The other half of F-038: validator.rs required a `contains` source to be Sort::Entity, so
    // the composer emitted assertion frames that this very validator rejected. `predicate` is the
    // most basic operation there is and its output did not validate.
    let subject = parser::parse("*").expect("parses");
    let relation = parser::parse("->").expect("parses");
    let object = parser::parse("/3").expect("parses");
    let pred = composer::predicate(&subject, &relation, &object).expect("predicate composes");
    assert_eq!(pred.node(&pred.root).unwrap().sort, Sort::Assertion);

    let report = ul_core::validator::validate(&pred, false);
    assert!(
        report.valid,
        "predicate output must validate; errors: {:?}",
        report.errors
    );
}

#[test]
fn a_relation_or_modifier_is_still_not_a_container() {
    // Guard the widened rule: Entity and Assertion may contain; Relation and Modifier may not.
    use ul_core::types::edge::Edge;
    use ul_core::types::gir::Gir;
    use ul_core::types::node::{EnclosureShape, Node};

    for bad in [Sort::Relation, Sort::Modifier] {
        let container = Node::enclosure("c", EnclosureShape::Circle).with_sort(bad);
        let child = Node::point("p");
        let gir = Gir::new("c", vec![container, child], vec![Edge::contains("c", "p")]);
        let report = ul_core::validator::validate(&gir, false);
        assert!(
            !report.valid,
            "a {bad:?}-sorted node must not be a valid container"
        );
    }
}

#[test]
fn a_force_annotation_survives_a_round_trip() {
    // The sort was previously dropped on deparse, so even a composed assertion could not be
    // serialized and read back as one.
    for src in ["assert{* -> /3}", "query{*}", "direct{/4 & /3}"] {
        let gir = parser::parse(src).expect("parses");
        let text = parser::deparse(&gir).expect("deparses");
        let again = parser::parse(&text).unwrap_or_else(|e| panic!("`{text}` must re-parse: {e}"));
        assert_eq!(again.node(&again.root).unwrap().sort, Sort::Assertion, "{src}");
        assert_eq!(gir, again, "round-trip must be stable for `{src}`");
    }
}
