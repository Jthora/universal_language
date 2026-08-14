//! The corpus harness: every entry in `corpus/entries/` is reconstructed through the real
//! implementation and its `expected` block asserted. Ground truth is machine-verified, never
//! authorial say-so (corpus/README.md; claims.yaml#TIERS-TRAVEL-WITH-CONTENT).
//!
//! An entry whose ground truth does not verify does not merge.

use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use ul_core::map::{CombinatorialMap, Nesting};

fn corpus_dir() -> PathBuf {
    // crates/ul-core -> ul-forge -> repo root
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../corpus/entries")
}

fn load_map(entry: &Value) -> CombinatorialMap {
    let n_darts = entry["n_darts"].as_u64().expect("n_darts") as usize;
    let mut rotations: Vec<(String, Vec<usize>)> = entry["rotations"]
        .as_object()
        .expect("rotations object")
        .iter()
        .map(|(v, darts)| {
            let darts = darts
                .as_array()
                .expect("dart list")
                .iter()
                .map(|d| d.as_u64().expect("dart") as usize)
                .collect();
            (v.clone(), darts)
        })
        .collect();
    // deterministic order regardless of JSON object iteration
    rotations.sort_by(|a, b| a.0.cmp(&b.0));
    CombinatorialMap::from_rotations(rotations, n_darts)
}

#[test]
fn every_corpus_entry_verifies_against_the_implementation() {
    let dir = corpus_dir();
    let mut entries: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("corpus dir missing: {}", dir.display()))
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    entries.sort();
    assert!(
        !entries.is_empty(),
        "corpus is empty — the harness has nothing to verify"
    );

    for path in entries {
        let text = fs::read_to_string(&path).expect("read entry");
        let entry: Value = serde_json::from_str(&text)
            .unwrap_or_else(|e| panic!("{}: invalid JSON: {e}", path.display()));
        let id = entry["id"].as_str().expect("id");
        let map = load_map(&entry);
        let exp = &entry["expected"];

        if let Some(v) = exp["vertices"].as_u64() {
            assert_eq!(map.vertices().len() as u64, v, "{id}: vertices");
        }
        if let Some(e) = exp["edges"].as_u64() {
            assert_eq!(map.edge_count() as u64, e, "{id}: edges");
        }
        if let Some(f) = exp["faces"].as_u64() {
            assert_eq!(map.faces().len() as u64, f, "{id}: faces (raw orbit count)");
        }
        match &exp["genus"] {
            Value::Number(g) => {
                assert_eq!(
                    map.genus(),
                    Some(g.as_u64().expect("genus") as u32),
                    "{id}: genus"
                );
            }
            Value::Null => {
                assert_eq!(map.genus(), None, "{id}: genus formula must not apply");
            }
            _ => {}
        }
        if let Some(ds) = exp["degree_sequence"].as_array() {
            let want: Vec<usize> = ds
                .iter()
                .map(|d| d.as_u64().expect("deg") as usize)
                .collect();
            assert_eq!(map.degree_sequence(), want, "{id}: degree sequence");
        }

        if let Some(c) = exp["components"].as_u64() {
            assert_eq!(map.components() as u64, c, "{id}: components");
        }

        // The face set used for regional (lexicon) assertions: the planar reading when the entry
        // carries its nesting, the raw orbit faces otherwise (sufficient for connected entries).
        let region_faces: Vec<Vec<usize>> = if let Some(nest) = entry.get("nesting") {
            let outer: Vec<usize> = nest["top_level_outer_darts"]
                .as_array()
                .expect("top_level_outer_darts")
                .iter()
                .map(|d| d.as_u64().expect("dart") as usize)
                .collect();
            let mut nesting = Nesting::all_top_level(outer);
            if let Some(places) = nest["placements"].as_array() {
                for p in places {
                    let pair = p.as_array().expect("placement pair");
                    nesting = nesting.place(
                        pair[0].as_u64().expect("outer dart") as usize,
                        pair[1].as_u64().expect("container dart") as usize,
                    );
                }
            }
            if let Some(pf) = exp["planar_faces"].as_u64() {
                assert_eq!(
                    map.faces_planar(&nesting).len() as u64,
                    pf,
                    "{id}: planar faces"
                );
            }
            if let Some(chi) = exp["euler_planar"].as_i64() {
                assert_eq!(
                    map.euler_characteristic_planar(&nesting),
                    chi,
                    "{id}: planar Euler"
                );
            }
            map.faces_planar(&nesting)
        } else {
            map.faces()
        };

        // Lexicon-grade regional semantics: co-facality is the machine-checkable form of
        // containment/separation/adjacency (spec/grammar-core.md).
        let cofacial = |a: usize, b: usize| {
            region_faces
                .iter()
                .any(|f| f.contains(&a) && f.contains(&b))
        };
        if let Some(pairs) = exp["same_face"].as_array() {
            for p in pairs {
                let (a, b) = (
                    p[0].as_u64().unwrap() as usize,
                    p[1].as_u64().unwrap() as usize,
                );
                assert!(cofacial(a, b), "{id}: darts {a},{b} must share a region");
            }
        }
        if let Some(pairs) = exp["different_face"].as_array() {
            for p in pairs {
                let (a, b) = (
                    p[0].as_u64().unwrap() as usize,
                    p[1].as_u64().unwrap() as usize,
                );
                assert!(
                    !cofacial(a, b),
                    "{id}: darts {a},{b} must lie in different regions"
                );
            }
        }

        // tiers travel with content: an entry without a tier does not verify
        let tier = entry["tier"].as_str().unwrap_or("");
        assert!(
            ["VERIFIED", "ARGUED", "CONJECTURED"].contains(&tier),
            "{id}: missing or invalid tier — TIERS-TRAVEL-WITH-CONTENT"
        );
    }
}

/// Every corpus entry is also reading-invariant over the 𝔽₀ generators: mirror, relabeling,
/// subdivision (spec/reading-invariance-v1.md). The lexicon's ground truth does not depend on
/// how the reader numbers, orients, or samples the drawing.
#[test]
fn every_corpus_entry_is_reading_invariant() {
    let dir = corpus_dir();
    let mut entries: Vec<_> = fs::read_dir(&dir)
        .expect("corpus dir")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    entries.sort();

    for path in entries {
        let entry: Value =
            serde_json::from_str(&fs::read_to_string(&path).expect("read")).expect("json");
        let id = entry["id"].as_str().expect("id");
        let map = load_map(&entry);
        let base = map.essential_invariants();

        // mirror
        assert_eq!(map.mirror().essential_invariants(), base, "{id}: mirror");

        // an edge-reversing relabeling
        let e = map.edge_count();
        let perm: Vec<usize> = (0..e).rev().collect();
        let flip: Vec<bool> = (0..e).map(|i| i % 2 == 0).collect();
        assert_eq!(
            map.relabel(&perm, &flip).essential_invariants(),
            base,
            "{id}: relabel"
        );

        // a double subdivision
        let sub = map.subdivide(0).subdivide(1);
        assert_eq!(sub.essential_invariants(), base, "{id}: subdivision");
    }
}
