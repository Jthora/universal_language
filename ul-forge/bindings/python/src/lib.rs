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

/// Compose a Σ_UL operation on UL-Script operands.
///
/// Args:
///     operation (str): One of the 13 Σ_UL operation names.
///     operands (list[str]): UL-Script strings for the operands.
///
/// Returns a Python dict with `gir` and `ul_script` keys.
///
/// Example:
///     result = ul_forge.compose("invert", ["→"])
///     print(result["ul_script"])
#[pyfunction]
fn compose(operation: &str, operands: Vec<String>) -> PyResult<PyObject> {
    let girs: Vec<ul_core::Gir> = operands
        .iter()
        .map(|s| ul_core::parser::parse(s)
            .map_err(|e| PyValueError::new_err(format!("Parse error on '{s}': {e}"))))
        .collect::<Result<Vec<_>, _>>()?;

    let result = match operation {
        "negate" => ul_core::composer::negate(girs.first().ok_or_else(|| arity_err("negate", 1))?),
        "embed" => ul_core::composer::embed(girs.first().ok_or_else(|| arity_err("embed", 1))?),
        "abstract" => ul_core::composer::abstract_op(girs.first().ok_or_else(|| arity_err("abstract", 1))?),
        "invert" => ul_core::composer::invert(girs.first().ok_or_else(|| arity_err("invert", 1))?),
        "predicate" => {
            let (a, b, c) = get_3(&girs, "predicate")?;
            ul_core::composer::predicate(a, b, c)
        }
        "modify_entity" => { let (a, b) = get_2(&girs, "modify_entity")?; ul_core::composer::modify_entity(a, b) }
        "modify_relation" => { let (a, b) = get_2(&girs, "modify_relation")?; ul_core::composer::modify_relation(a, b) }
        "conjoin" => { let (a, b) = get_2(&girs, "conjoin")?; ul_core::composer::conjoin(a, b) }
        "disjoin" => { let (a, b) = get_2(&girs, "disjoin")?; ul_core::composer::disjoin(a, b) }
        "compose" => { let (a, b) = get_2(&girs, "compose")?; ul_core::composer::compose(a, b) }
        "quantify" => { let (a, b) = get_2(&girs, "quantify")?; ul_core::composer::quantify(a, b) }
        "bind" => { let (a, b) = get_2(&girs, "bind")?; ul_core::composer::bind(a, b) }
        "modify_assertion" => { let (a, b) = get_2(&girs, "modify_assertion")?; ul_core::composer::modify_assertion(a, b) }
        _ => return Err(PyValueError::new_err(format!("Unknown operation: {operation}"))),
    }.map_err(|e| PyValueError::new_err(format!("Compose error: {e}")))?;

    let gir_json = result.to_json()
        .map_err(|e| PyValueError::new_err(format!("Serialization error: {e}")))?;
    let ul_script = ul_core::parser::deparse(&result).unwrap_or_default();

    Python::with_gil(|py| {
        let json_mod = py.import("json")?;
        let gir_obj = json_mod.call_method1("loads", (gir_json,))?;
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("gir", gir_obj)?;
        dict.set_item("ul_script", ul_script)?;
        Ok(dict.into())
    })
}

/// Analyze a GIR and detect which Σ_UL operations are expressed.
///
/// Returns a dict with `operations`, `node_count`, `edge_count`.
///
/// Example:
///     info = ul_forge.analyze(gir)
///     print(info["operations"])
#[pyfunction]
fn analyze(gir: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let gir_obj = gir_from_pyobj(gir)?;
    let ops = ul_core::composer::detect_operations(&gir_obj);
    Python::with_gil(|py| {
        let dict = pyo3::types::PyDict::new(py);
        let op_names: Vec<&str> = ops.iter().map(|o| o.operation).collect();
        dict.set_item("operations", op_names)?;
        dict.set_item("node_count", gir_obj.nodes.len())?;
        dict.set_item("edge_count", gir_obj.edges.len())?;
        Ok(dict.into())
    })
}

fn arity_err(op: &str, n: usize) -> PyErr {
    PyValueError::new_err(format!("{op} requires {n} operand(s)"))
}

fn get_2<'a>(girs: &'a [ul_core::Gir], op: &str) -> PyResult<(&'a ul_core::Gir, &'a ul_core::Gir)> {
    if girs.len() < 2 {
        return Err(arity_err(op, 2));
    }
    Ok((&girs[0], &girs[1]))
}

fn get_3<'a>(girs: &'a [ul_core::Gir], op: &str) -> PyResult<(&'a ul_core::Gir, &'a ul_core::Gir, &'a ul_core::Gir)> {
    if girs.len() < 3 {
        return Err(arity_err(op, 3));
    }
    Ok((&girs[0], &girs[1], &girs[2]))
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

/// Set performative force on a GIR assertion.
///
/// Args:
///     gir (dict): GIR dict (must have Entity or Assertion root).
///     force (str): One of "assert", "query", "direct", "commit", "express", "declare".
///
/// Returns a new GIR dict with the force applied.
#[pyfunction]
fn set_force(gir: &Bound<'_, PyAny>, force: &str) -> PyResult<PyObject> {
    let gir_obj = gir_from_pyobj(gir)?;
    let perf_force = match force {
        "assert" => ul_core::PerformativeForce::Assert,
        "query" => ul_core::PerformativeForce::Query,
        "direct" => ul_core::PerformativeForce::Direct,
        "commit" => ul_core::PerformativeForce::Commit,
        "express" => ul_core::PerformativeForce::Express,
        "declare" => ul_core::PerformativeForce::Declare,
        other => return Err(PyValueError::new_err(format!("Unknown force: {other}"))),
    };
    let result = ul_core::performative::with_force(&gir_obj, perf_force)
        .map_err(|e| PyValueError::new_err(format!("Force error: {e}")))?;
    let json_str = result.to_json()
        .map_err(|e| PyValueError::new_err(format!("Serialization error: {e}")))?;
    Python::with_gil(|py| {
        let json_mod = py.import("json")?;
        let obj = json_mod.call_method1("loads", (json_str,))?;
        Ok(obj.into())
    })
}

/// Run pragmatic inference on a GIR.
///
/// Returns a dict with `inferences` (list of {rule, surface, intended}) and `count`.
#[pyfunction]
fn infer_pragmatics(gir: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let gir_obj = gir_from_pyobj(gir)?;
    let inferences = ul_core::pragmatic::infer(&gir_obj);

    Python::with_gil(|py| {
        let json_mod = py.import("json")?;
        let results = pyo3::types::PyList::empty(py);
        for inf in &inferences {
            let dict = pyo3::types::PyDict::new(py);
            dict.set_item("rule", format!("{:?}", inf.rule))?;
            let surface_json = serde_json::to_string(&inf.surface)
                .map_err(|e| PyValueError::new_err(format!("JSON error: {e}")))?;
            dict.set_item("surface", json_mod.call_method1("loads", (surface_json,))?)?;
            let intended_json = serde_json::to_string(&inf.intended)
                .map_err(|e| PyValueError::new_err(format!("JSON error: {e}")))?;
            dict.set_item("intended", json_mod.call_method1("loads", (intended_json,))?)?;
            results.append(dict)?;
        }
        let out = pyo3::types::PyDict::new(py);
        out.set_item("inferences", results)?;
        out.set_item("count", inferences.len())?;
        Ok(out.into())
    })
}

/// Module definition.
#[pymodule]
fn ul_forge(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(render, m)?)?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_function(wrap_pyfunction!(deparse, m)?)?;
    m.add_function(wrap_pyfunction!(compose, m)?)?;
    m.add_function(wrap_pyfunction!(analyze, m)?)?;
    m.add_function(wrap_pyfunction!(set_force, m)?)?;
    m.add_function(wrap_pyfunction!(infer_pragmatics, m)?)?;

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
