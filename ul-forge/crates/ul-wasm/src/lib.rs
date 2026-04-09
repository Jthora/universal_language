use std::panic::{catch_unwind, AssertUnwindSafe};
use wasm_bindgen::prelude::*;

// Force-link ul-game so wasm-bindgen discovers all its #[wasm_bindgen] exports.
// ul-game provides: createContext, evaluate, scoreComposition, evaluateJaneAttempt,
// validateSequence, getAnimationSequence, layout, loadCustomDefinitions,
// parseUlScript, deparseGir, validateGir, applyOperation, composeGir,
// detectOperations, analyzeStructure, compareGir, renderSvg, renderGlyphPreview,
// getHints, analyzeHints, getNextPuzzle, queryLexicon, lookupLexiconEntry,
// set_force, infer_pragmatics
extern crate ul_game;

/// Parse UL-Script text into GIR JSON.
#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsError> {
    let input = input.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::parser::parse(&input).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(serde_wasm_bindgen::to_value(&gir)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal parser panic")))
}

/// Validate GIR JSON. Returns a JS object with { valid, errors, warnings }.
#[wasm_bindgen]
pub fn validate(gir_json: &str, check_geometry: bool) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let result = ul_core::validator::validate(&gir, check_geometry);

        let obj = serde_json::json!({
            "valid": result.valid,
            "errors": result.errors.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
            "warnings": result.warnings,
        });

        Ok(serde_wasm_bindgen::to_value(&obj)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal validator panic")))
}

/// Render GIR JSON to SVG.
#[wasm_bindgen]
pub fn render(gir_json: &str, width: f64, height: f64) -> Result<String, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let options = ul_core::renderer::RenderOptions {
            width,
            height,
            ..ul_core::renderer::RenderOptions::default()
        };
        ul_core::renderer::render(&gir, &options).map_err(|e| JsError::new(&e.to_string()))
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal renderer panic")))
}

/// Deparse GIR JSON back to canonical UL-Script text.
#[wasm_bindgen]
pub fn deparse(gir_json: &str) -> Result<String, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        ul_core::parser::deparse(&gir).map_err(|e| JsError::new(&e.to_string()))
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal deparser panic")))
}

/// Compose a Σ_UL operation on UL-Script operands.
///
/// `operation`: one of the 13 Σ_UL operations (e.g. "negate", "predicate", "conjoin")
/// `operands_json`: JSON array of UL-Script strings (1–3 depending on operation)
///
/// Returns GIR JSON of the composed result.
#[wasm_bindgen]
pub fn compose(operation: &str, operands_json: &str) -> Result<JsValue, JsError> {
    let operation = operation.to_owned();
    let operands_json = operands_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let operands: Vec<String> = serde_json::from_str(&operands_json)
            .map_err(|e| JsError::new(&format!("invalid operands JSON: {e}")))?;
        let girs: Vec<ul_core::Gir> = operands
            .iter()
            .map(|s| ul_core::parser::parse(s).map_err(|e| JsError::new(&format!("parse error on '{s}': {e}"))))
            .collect::<Result<Vec<_>, _>>()?;

        let result = match operation.as_str() {
            "negate" => ul_core::composer::negate(girs.first().ok_or_else(|| JsError::new("negate requires 1 operand"))?),
            "embed" => ul_core::composer::embed(girs.first().ok_or_else(|| JsError::new("embed requires 1 operand"))?),
            "abstract" => ul_core::composer::abstract_op(girs.first().ok_or_else(|| JsError::new("abstract requires 1 operand"))?),
            "invert" => ul_core::composer::invert(girs.first().ok_or_else(|| JsError::new("invert requires 1 operand"))?),
            "predicate" if girs.len() >= 3 => ul_core::composer::predicate(&girs[0], &girs[1], &girs[2]),
            "modify_entity" if girs.len() >= 2 => ul_core::composer::modify_entity(&girs[0], &girs[1]),
            "modify_relation" if girs.len() >= 2 => ul_core::composer::modify_relation(&girs[0], &girs[1]),
            "conjoin" if girs.len() >= 2 => ul_core::composer::conjoin(&girs[0], &girs[1]),
            "disjoin" if girs.len() >= 2 => ul_core::composer::disjoin(&girs[0], &girs[1]),
            "compose" if girs.len() >= 2 => ul_core::composer::compose(&girs[0], &girs[1]),
            "quantify" if girs.len() >= 2 => ul_core::composer::quantify(&girs[0], &girs[1]),
            "bind" if girs.len() >= 2 => ul_core::composer::bind(&girs[0], &girs[1]),
            "modify_assertion" if girs.len() >= 2 => ul_core::composer::modify_assertion(&girs[0], &girs[1]),
            _ => return Err(JsError::new(&format!("unknown operation or insufficient operands: {operation}"))),
        }.map_err(|e| JsError::new(&e.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&result)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal composer panic")))
}

/// Analyze a GIR to detect which Σ_UL operations are present.
///
/// Returns a JSON object with { operations: [...], node_count, edge_count, root }.
#[wasm_bindgen]
pub fn analyze(gir_json: &str) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let ops = ul_core::composer::detect_operations(&gir);
        let op_names: Vec<&str> = ops.iter().map(|o| o.operation).collect();
        let result = serde_json::json!({
            "operations": op_names,
            "node_count": gir.nodes.len(),
            "edge_count": gir.edges.len(),
            "root": gir.root,
        });
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal analyzer panic")))
}

// set_force and infer_pragmatics are provided by ul-game (linked via extern crate).
