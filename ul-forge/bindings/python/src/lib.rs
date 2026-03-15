//! PyO3 bindings for UL Forge.
//!
//! Exposes `parse()`, `render()`, `validate()`, and `deparse()` as Python functions.
//! Also provides `%%ul` and `%%ul_gir` Jupyter cell magics via the `load_ipython_extension` hook.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Parse UL-Script text into a GIR dict.
///
/// Returns a Python dict with `nodes` and `edges` keys.
///
/// Example:
///     gir = ul_forge.parse("● → ●")
///     print(gir["nodes"])
#[pyfunction]
fn parse(script: &str) -> PyResult<PyObject> {
    let gir = ul_core::parser::parse(script)
        .map_err(|e| PyValueError::new_err(format!("Parse error: {e}")))?;
    let json_str = gir
        .to_json()
        .map_err(|e| PyValueError::new_err(format!("Serialization error: {e}")))?;
    Python::with_gil(|py| {
        let json_mod = py.import("json")?;
        let obj = json_mod.call_method1("loads", (json_str,))?;
        Ok(obj.into())
    })
}

/// Render a GIR dict to an SVG string.
///
/// Accepts the same dict structure returned by `parse()`.
///
/// Options (keyword args):
///     width (float): Canvas width in SVG units (default 400)
///     height (float): Canvas height in SVG units (default 400)
///     embed_gir (bool): Embed GIR JSON in SVG metadata (default True)
///
/// Example:
///     svg = ul_forge.render(gir)
#[pyfunction]
#[pyo3(signature = (gir, width=400.0, height=400.0, embed_gir=true))]
fn render(gir: &Bound<'_, PyAny>, width: f64, height: f64, embed_gir: bool) -> PyResult<String> {
    let gir_obj = gir_from_pyobj(gir)?;
    let opts = ul_core::renderer::RenderOptions {
        format: ul_core::renderer::OutputFormat::Svg,
        width,
        height,
        embed_gir,
    };
    ul_core::renderer::render(&gir_obj, &opts)
        .map_err(|e| PyValueError::new_err(format!("Render error: {e}")))
}

/// Validate a GIR dict.
///
/// Returns a dict with `valid` (bool), `errors` (list[str]), `warnings` (list[str]).
///
/// Options:
///     check_geometry (bool): Run geometric satisfiability checks (default False)
///
/// Example:
///     result = ul_forge.validate(gir)
///     assert result["valid"]
#[pyfunction]
#[pyo3(signature = (gir, check_geometry=false))]
fn validate(gir: &Bound<'_, PyAny>, check_geometry: bool) -> PyResult<PyObject> {
    let gir_obj = gir_from_pyobj(gir)?;
    let result = ul_core::validator::validate(&gir_obj, check_geometry);
    Python::with_gil(|py| {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("valid", result.valid)?;
        let errors: Vec<String> = result.errors.iter().map(|e| e.to_string()).collect();
        dict.set_item("errors", errors)?;
        dict.set_item("warnings", result.warnings.clone())?;
        Ok(dict.into())
    })
}

/// Convert a GIR dict back to UL-Script text.
///
/// Example:
///     text = ul_forge.deparse(gir)
#[pyfunction]
fn deparse(gir: &Bound<'_, PyAny>) -> PyResult<String> {
    let gir_obj = gir_from_pyobj(gir)?;
    ul_core::parser::deparse(&gir_obj)
        .map_err(|e| PyValueError::new_err(format!("Deparse error: {e}")))
}

/// Helper: convert a Python dict to a Rust Gir via JSON roundtrip.
fn gir_from_pyobj(obj: &Bound<'_, PyAny>) -> PyResult<ul_core::Gir> {
    let py = obj.py();
    let json_mod = py.import("json")?;
    let json_str: String = json_mod.call_method1("dumps", (obj,))?.extract()?;
    let gir: ul_core::Gir = serde_json::from_str(&json_str)
        .map_err(|e| PyValueError::new_err(format!("Invalid GIR: {e}")))?;
    Ok(gir)
}

/// Module definition.
#[pymodule]
fn ul_forge(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(render, m)?)?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_function(wrap_pyfunction!(deparse, m)?)?;

    // Register IPython magic loader
    m.add_function(wrap_pyfunction!(load_ipython_extension, m)?)?;

    Ok(())
}

/// IPython extension entry point. Called by `%load_ext ul_forge`.
#[pyfunction]
fn load_ipython_extension(ipython: &Bound<'_, PyAny>) -> PyResult<()> {
    let py = ipython.py();

    // Define the magic class in Python — simplest approach to register cell magics
    let code = c"
from IPython.core.magic import register_cell_magic
import ul_forge as _ul

@register_cell_magic
def ul(line, cell):
    \"\"\"Render UL-Script as inline SVG.\"\"\"
    from IPython.display import SVG, display
    gir = _ul.parse(cell)
    svg = _ul.render(gir)
    display(SVG(data=svg))

@register_cell_magic
def ul_gir(line, cell):
    \"\"\"Show GIR JSON for UL-Script.\"\"\"
    import json
    from IPython.display import display, JSON
    gir = _ul.parse(cell)
    display(JSON(gir))
";
    py.run(code, None, None)?;
    Ok(())
}
