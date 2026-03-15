//! Fixture-driven tests.
//!
//! Iterates over `.ul` fixture files and asserts the expected behavior:
//! - `canonical/` files must parse + validate + render successfully
//! - `edge-cases/` files must parse without panicking
//! - `invalid/` `.ul` files must fail to parse
//! - `invalid/` `.gir.json` files must fail validation

use std::fs;
use std::path::Path;
use ul_core::parser;
use ul_core::renderer::{self, RenderOptions};
use ul_core::validator;

fn fixture_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures"))
}

#[test]
fn canonical_fixtures_parse_validate_render() {
    let dir = fixture_dir().join("canonical");
    let mut count = 0;
    for entry in fs::read_dir(&dir).expect("canonical fixture dir must exist") {
        let entry = entry.expect("readable dir entry");
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "ul") {
            let input = fs::read_to_string(&path).expect("readable fixture file");
            let input = input.trim();
            let name = path.file_stem().unwrap().to_string_lossy();

            let gir = parser::parse(input)
                .unwrap_or_else(|e| panic!("[{name}] parse failed: {e}"));

            let result = validator::validate(&gir, false);
            assert!(result.valid, "[{name}] validation failed: {:?}", result.errors);

            let opts = RenderOptions::default();
            renderer::render(&gir, &opts)
                .unwrap_or_else(|e| panic!("[{name}] render failed: {e}"));

            count += 1;
        }
    }
    assert_eq!(count, 12, "Expected 12 canonical fixtures, found {count}");
}

#[test]
fn edge_case_fixtures_do_not_panic() {
    let dir = fixture_dir().join("edge-cases");
    let mut count = 0;
    for entry in fs::read_dir(&dir).expect("edge-cases fixture dir must exist") {
        let entry = entry.expect("readable dir entry");
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "ul") {
            let input = fs::read_to_string(&path).expect("readable fixture file");
            let input = input.trim();

            // Must not panic — either Ok or Err is fine
            let _ = parser::parse(input);
            count += 1;
        }
    }
    assert!(count >= 4, "Expected at least 4 edge-case fixtures, found {count}");
}

#[test]
fn invalid_ul_fixtures_fail_to_parse() {
    let dir = fixture_dir().join("invalid");
    let mut count = 0;
    for entry in fs::read_dir(&dir).expect("invalid fixture dir must exist") {
        let entry = entry.expect("readable dir entry");
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "ul") {
            let input = fs::read_to_string(&path).expect("readable fixture file");
            let input = input.trim();
            let name = path.file_stem().unwrap().to_string_lossy();

            let result = parser::parse(input);
            assert!(result.is_err(), "[{name}] expected parse error but got Ok");
            count += 1;
        }
    }
    assert!(count >= 3, "Expected at least 3 invalid .ul fixtures, found {count}");
}

#[test]
fn invalid_gir_fixtures_fail_validation() {
    let dir = fixture_dir().join("invalid");
    let mut count = 0;
    for entry in fs::read_dir(&dir).expect("invalid fixture dir must exist") {
        let entry = entry.expect("readable dir entry");
        let path = entry.path();
        if path.to_string_lossy().ends_with(".gir.json") {
            let json = fs::read_to_string(&path).expect("readable fixture file");
            let name = path.file_name().unwrap().to_string_lossy();

            if let Ok(gir) = serde_json::from_str::<ul_core::Gir>(&json) {
                let result = validator::validate(&gir, true);
                assert!(!result.valid, "[{name}] expected validation failure but got valid");
            }
            // If deserialization fails, that's also a valid "rejection"
            count += 1;
        }
    }
    assert!(count >= 2, "Expected at least 2 invalid .gir.json fixtures, found {count}");
}
