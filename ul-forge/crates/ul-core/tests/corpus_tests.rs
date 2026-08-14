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
    assert!(!entries.is_empty(), "corpus is empty — the harness has nothing to verify");

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
                assert_eq!(map.genus(), Some(g.as_u64().expect("genus") as u32), "{id}: genus");
            }
            Value::Null => {
                assert_eq!(map.genus(), None, "{id}: genus formula must not apply");
            }
            _ => {}
        }
        if let Some(ds) = exp["degree_sequence"].as_array() {
            let want: Vec<usize> = ds.iter().map(|d| d.as_u64().expect("deg") as usize).collect();
            assert_eq!(map.degree_sequence(), want, "{id}: degree sequence");
        }

        // planar assertions require the entry to carry its nesting — extra structure, by design
        if let Some(nest) = entry.get("nesting") {
            let outer: Vec<usize> = nest["top_level_outer_darts"]
                .as_array()
                .expect("top_level_outer_darts")
                .iter()
                .map(|d| d.as_u64().expect("dart") as usize)
                .collect();
            let nesting = Nesting::all_top_level(outer);
            if let Some(pf) = exp["planar_faces"].as_u64() {
                assert_eq!(map.faces_planar(&nesting).len() as u64, pf, "{id}: planar faces");
            }
            if let Some(chi) = exp["euler_planar"].as_i64() {
                assert_eq!(map.euler_characteristic_planar(&nesting), chi, "{id}: planar Euler");
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
