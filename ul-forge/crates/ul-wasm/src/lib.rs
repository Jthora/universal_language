use std::panic::{catch_unwind, AssertUnwindSafe};
use wasm_bindgen::prelude::*;

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
pub fn validate(gir_json: &str) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let result = ul_core::validator::validate(&gir, false);

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
pub fn render(gir_json: &str) -> Result<String, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let options = ul_core::renderer::RenderOptions::default();
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
