//! The placement channel must agree with the register it was derived from.
//!
//! WHY THIS FILE EXISTS
//!   `placement.rs` carries the six-mark symmetry table as a const. A hardcoded table that
//!   nothing compares to its source is a number that goes stale silently, which is exactly what
//!   `COUNTS-ARE-GENERATED` is about and what `F-039` cost. So the table is checked against the
//!   vendored `enantiomorphs` sheet in BOTH directions, and the handedness function is checked
//!   against the vendored `chirality-source` column cell by cell.
//!
//!   The column check is the load-bearing one. It is allowed to disagree on exactly four cells —
//!   the four that note 076 located as a hole in the imported column and that a third sheet
//!   independently adjudicates in the rule's favour. Any FIFTH disagreement, or the loss of one of
//!   those four, fails here. That is the difference between a known defect and silent drift.

use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use ul_core::placement::{Handedness, Mark, MirrorAxes, Placement};

fn register(rel: &str) -> Value {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../uws/register/codex")
        .join(rel);
    let text = fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{} is not valid JSON: {e}", p.display()))
}

/// The four cells where this rule contradicts the imported handedness column, and where the
/// per-orbit records (`mirror_axes: 0`, hence chiral) uphold the rule. Note 076 section 4.
const KNOWN_HOLE: [(&str, &str, &str); 4] = [
    ("concatenate", "Angle", "Curve"),
    ("concatenate", "Curve", "Angle"),
    ("superpose_displaced", "Angle", "Curve"),
    ("superpose_displaced", "Curve", "Angle"),
];

#[test]
fn the_mark_table_agrees_with_the_enantiomorphs_sheet() {
    let sheet = register("charts/enantiomorphs.json");
    let rows = sheet["rows"].as_array().expect("rows");
    let threshold = sheet["threshold"].as_f64().expect("threshold");

    let mut seen = BTreeSet::new();
    for row in rows {
        let name = row["mark"].as_str().expect("mark");
        let mark = Mark::from_name(name).unwrap_or_else(|| panic!("register mark {name} unknown"));
        seen.insert(name.to_string());

        assert_eq!(mark.group(), row["group"].as_str().expect("group"), "{name}: group");
        assert_eq!(
            mark.is_chiral(),
            row["chiral"].as_bool().expect("chiral"),
            "{name}: chirality from the group column vs the sheet's own flag"
        );
        // and against the sheet's INDEPENDENT measurement, not just its flag
        let measured = row["mirror_distance"].as_f64().expect("mirror_distance") > threshold;
        assert_eq!(
            mark.is_chiral(),
            measured,
            "{name}: mirror_distance {:?} against threshold {threshold} disagrees with group {}",
            row["mirror_distance"],
            mark.group()
        );
    }

    // the other direction: no mark in the implementation is missing from the register
    for m in Mark::ALL {
        assert!(
            seen.contains(m.name()),
            "{} is in the implementation but not in the register sheet",
            m.name()
        );
    }
    assert_eq!(seen.len(), Mark::ALL.len(), "mark count");
}

#[test]
fn wave_is_the_unique_chiral_letter() {
    let chiral: Vec<_> = Mark::ALL.into_iter().filter(|m| m.is_chiral()).collect();
    assert_eq!(chiral, vec![Mark::Wave], "exactly one cyclic group among the six");
    // and it is the SMOOTH C2 one: the Line is also C2-rotational but D2, so its half-turn is
    // realised by a reflection too.
    assert_eq!(Mark::Line.mirror_axes(), MirrorAxes::Finite(2));
    assert_eq!(Mark::Wave.mirror_axes(), MirrorAxes::Finite(0));
}

#[test]
fn handedness_reproduces_the_attested_column_except_the_known_hole() {
    let cells = register("charts/chirality-source.cells.json");
    let cells = cells["cells"].as_array().expect("cells");
    assert!(!cells.is_empty(), "no handedness cells");

    let mut agree = 0usize;
    let mut disputed: BTreeSet<(String, String, String)> = BTreeSet::new();

    for c in cells {
        let op = c["operator"].as_str().expect("operator");
        let a = c["a"].as_str().expect("a");
        let b = c["b"].as_str().expect("b");
        let attested = c["source"].as_str().expect("source");

        let p = Placement::new(
            op,
            Mark::from_name(a).unwrap_or_else(|| panic!("mark {a}")),
            Mark::from_name(b).unwrap_or_else(|| panic!("mark {b}")),
        );
        let got = p
            .handedness()
            .unwrap_or_else(|| panic!("{op} is not an attested binary operator"));

        if got.as_str() == attested {
            agree += 1;
        } else {
            disputed.insert((op.to_string(), a.to_string(), b.to_string()));
        }

        // the sheet's two columns must not contradict each other
        assert_eq!(
            c["chiral"].as_str().expect("chiral") == "true",
            attested != "achiral",
            "{op}({a},{b}): the sheet's chiral flag and source disagree"
        );
    }

    let want: BTreeSet<(String, String, String)> = KNOWN_HOLE
        .iter()
        .map(|(o, a, b)| (o.to_string(), a.to_string(), b.to_string()))
        .collect();

    assert_eq!(
        disputed, want,
        "the disagreement set moved. Extra entries are NEW drift and must be explained before \
         this test is changed; missing entries mean the imported column was edited, which it must \
         not be — the register is vendored verbatim and hashed."
    );
    assert_eq!(
        agree,
        cells.len() - KNOWN_HOLE.len(),
        "agreement count: {agree} of {}",
        cells.len()
    );
}

#[test]
fn inherited_handedness_is_exactly_the_wave() {
    let cells = register("charts/chirality-source.cells.json");
    for c in cells["cells"].as_array().expect("cells") {
        let (op, a, b) = (
            c["operator"].as_str().unwrap(),
            c["a"].as_str().unwrap(),
            c["b"].as_str().unwrap(),
        );
        let has_wave = a == "Wave" || b == "Wave";
        let inherited = c["source"].as_str().unwrap() == "inherited";
        // one biconditional per cell, both directions — NOT a chain of ||, which cannot fail
        assert_eq!(inherited, has_wave, "{op}({a},{b})");
    }
}

#[test]
fn reflecting_an_operand_does_not_change_where_handedness_comes_from() {
    // reflect_b selects WHICH enantiomorph, not whether the figure has enantiomorphs.
    for a in Mark::ALL {
        for b in Mark::ALL {
            for op in ["nest", "superpose_concentric", "cross", "concatenate", "superpose_displaced"] {
                let p = Placement::new(op, a, b);
                assert_eq!(
                    p.handedness(),
                    p.clone().mirrored().handedness(),
                    "{op}({}, {})",
                    a.name(),
                    b.name()
                );
            }
        }
    }
}

#[test]
fn survival_under_reflection_tracks_mirror_axis_count() {
    // The predicate measured over the register's reflect-B pairs, as a table.
    for b in Mark::ALL {
        let p = Placement::new("cross", Mark::Circle, b);
        assert_eq!(
            p.survives_reflection(),
            b.mirror_axes().at_least(2),
            "{}: survival should track having two or more mirror axes",
            b.name()
        );
    }
    // the two sides are both populated, or the predicate is vacuous
    assert!(Mark::ALL.into_iter().any(|m| Placement::new("cross", Mark::Circle, m).survives_reflection()));
    assert!(Mark::ALL.into_iter().any(|m| !Placement::new("cross", Mark::Circle, m).survives_reflection()));
}

#[test]
fn unary_operators_get_no_answer_rather_than_a_guess() {
    for op in ["close", "reflect_pair", "", "superpose"] {
        assert_eq!(
            Placement::new(op, Mark::Wave, Mark::Wave).handedness(),
            None,
            "{op} is not an attested binary operator and must not be answered"
        );
    }
}

#[test]
fn the_arrangement_branch_is_reachable_and_is_not_the_wave() {
    // If this ever became unreachable the Displacing/Crossing rules would be dead code.
    let p = Placement::new("superpose_displaced", Mark::Angle, Mark::Circle);
    assert_eq!(p.handedness(), Some(Handedness::Arrangement));
    assert!(!Mark::Angle.is_chiral() && !Mark::Circle.is_chiral());
}
