//! # ul-mcp — MCP Server for Universal Language
//!
//! Model Context Protocol server exposing UL operations as AI-agent tools.
//! Communicates over stdio using JSON-RPC 2.0 (MCP standard transport).
//!
//! ## Tools Exposed
//!
//! - `ul_parse`            — Parse UL-Script text into GIR JSON
//! - `ul_validate`         — Validate a GIR document against Σ_UL constraints
//! - `ul_render`           — Render a GIR to SVG or TikZ
//! - `ul_deparse`          — Convert GIR back to canonical UL-Script
//! - `ul_parse_and_render` — Parse + validate + render in one call
//! - `ul_lexicon`          — Search the 42-entry UL lexicon

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

use ul_core::parser;
use ul_core::renderer::{self, OutputFormat, RenderOptions};
use ul_core::types::gir::Gir;
use ul_core::validator;
use ul_core::composer;

// ── JSON-RPC 2.0 types ──

#[derive(Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Serialize)]
struct JsonRpcError {
    code: i64,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl JsonRpcResponse {
    fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: Some(result),
            error: None,
        }
    }

    fn error(id: Value, code: i64, message: String) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message,
                data: None,
            }),
        }
    }
}

// ── MCP protocol constants ──

const MCP_VERSION: &str = "2024-11-05";
const SERVER_NAME: &str = "ul-mcp";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

// ── Tool definitions ──

fn tool_definitions() -> Value {
    json!([
        {
            "name": "ul_parse",
            "description": "Parse UL-Script text into a GIR (Graph Intermediate Representation) JSON document. UL-Script uses geometric primitives: point(label), line(label), angle(label), curve(label), enclosure(shape, children...).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "ul_script": {
                        "type": "string",
                        "description": "UL-Script text to parse (e.g., 'point(existence)' or 'enclosure(circle, point(existence))')"
                    }
                },
                "required": ["ul_script"]
            }
        },
        {
            "name": "ul_validate",
            "description": "Validate a GIR document against Σ_UL formal constraints. Returns errors classified into 4 layers: schema, sort, invariant, and geometry.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "gir": {
                        "type": "object",
                        "description": "GIR JSON document to validate"
                    },
                    "check_geometry": {
                        "type": "boolean",
                        "description": "Whether to run geometric constraint checks (default: false)",
                        "default": false
                    }
                },
                "required": ["gir"]
            }
        },
        {
            "name": "ul_render",
            "description": "Render a GIR document to SVG or TikZ format.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "gir": {
                        "type": "object",
                        "description": "GIR JSON document to render"
                    },
                    "width": {
                        "type": "number",
                        "description": "Output width in pixels (default: 256)",
                        "default": 256
                    },
                    "height": {
                        "type": "number",
                        "description": "Output height in pixels (default: 256)",
                        "default": 256
                    },
                    "format": {
                        "type": "string",
                        "enum": ["svg", "tikz"],
                        "description": "Output format (default: svg)",
                        "default": "svg"
                    }
                },
                "required": ["gir"]
            }
        },
        {
            "name": "ul_deparse",
            "description": "Convert a GIR JSON document back to canonical UL-Script text.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "gir": {
                        "type": "object",
                        "description": "GIR JSON document to convert"
                    }
                },
                "required": ["gir"]
            }
        },
        {
            "name": "ul_parse_and_render",
            "description": "Parse UL-Script text and render to SVG in one step. Returns the GIR, validation result, and rendered output.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "ul_script": {
                        "type": "string",
                        "description": "UL-Script text to parse and render"
                    },
                    "width": {
                        "type": "number",
                        "description": "Output width (default: 256)",
                        "default": 256
                    },
                    "height": {
                        "type": "number",
                        "description": "Output height (default: 256)",
                        "default": 256
                    }
                },
                "required": ["ul_script"]
            }
        },
        {
            "name": "ul_lexicon",
            "description": "Search the 42-entry canonical UL lexicon. Each entry maps a concept to its geometric UL-Script representation with Σ_UL typing.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query (name, label, or empty for all entries)"
                    }
                },
                "required": ["query"]
            }
        },
        {
            "name": "ul_compose",
            "description": "Apply a Σ_UL operation to compose two GIR documents. Operations: predicate, modify_entity, modify_relation, negate, conjoin, disjoin, embed, abstract, compose, invert, quantify, bind, modify_assertion.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "description": "One of the 13 Σ_UL operations to apply",
                        "enum": ["predicate", "modify_entity", "modify_relation", "negate", "conjoin", "disjoin", "embed", "abstract", "compose", "invert", "quantify", "bind", "modify_assertion"]
                    },
                    "operands": {
                        "type": "array",
                        "description": "Array of UL-Script strings to use as operands (1 or 2 depending on operation)",
                        "items": { "type": "string" }
                    }
                },
                "required": ["operation", "operands"]
            }
        },
        {
            "name": "ul_analyze",
            "description": "Analyze a GIR document to detect which Σ_UL operations were used in its construction. Returns the detected operations and structural metrics.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "gir": {
                        "type": "object",
                        "description": "GIR JSON document to analyze"
                    }
                },
                "required": ["gir"]
            }
        },
        {
            "name": "ul_set_force",
            "description": "Set the performative force on a GIR assertion. Forces: assert (neutral statement), query (question), direct (command), commit (promise), express (feeling), declare (institutional act).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "gir": {
                        "type": "object",
                        "description": "GIR JSON document (must have Entity or Assertion root)"
                    },
                    "force": {
                        "type": "string",
                        "enum": ["assert", "query", "direct", "commit", "express", "declare"],
                        "description": "Performative force to apply"
                    }
                },
                "required": ["gir", "force"]
            }
        },
        {
            "name": "ul_infer_pragmatics",
            "description": "Run pragmatic inference rules on a surface GIR. Detects scalar implicatures (SI-1: 'some' ⟹ 'not all'), disjunction implicatures (SI-3: 'A or B' ⟹ 'not both'), and indirect requests (CI-3: 'can you X?' ⟹ 'do X!').",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "gir": {
                        "type": "object",
                        "description": "GIR JSON document to analyze for pragmatic inferences"
                    }
                },
                "required": ["gir"]
            }
        }
    ])
}

// ── Tool handlers ──

fn handle_ul_parse(params: &Value) -> Result<Value, String> {
    let ul_script = params
        .get("ul_script")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: ul_script")?;

    let gir = parser::parse(ul_script).map_err(|e| format!("Parse error: {e}"))?;
    let gir_json: Value =
        serde_json::from_str(&gir.to_json_pretty().map_err(|e| format!("Serialize error: {e}"))?)
            .map_err(|e| format!("JSON error: {e}"))?;

    Ok(json!({ "gir": gir_json }))
}

fn handle_ul_validate(params: &Value) -> Result<Value, String> {
    let gir_value = params.get("gir").ok_or("Missing required parameter: gir")?;
    let check_geometry = params
        .get("check_geometry")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let gir_json = serde_json::to_string(gir_value).map_err(|e| format!("JSON error: {e}"))?;
    let gir = Gir::from_json(&gir_json).map_err(|e| format!("Invalid GIR: {e}"))?;

    let result = validator::validate(&gir, check_geometry);
    let errors: Vec<String> = result.errors.iter().map(|e| e.to_string()).collect();

    Ok(json!({
        "valid": result.valid,
        "errors": errors,
        "warnings": result.warnings,
    }))
}

fn handle_ul_render(params: &Value) -> Result<Value, String> {
    let gir_value = params.get("gir").ok_or("Missing required parameter: gir")?;
    let width = params
        .get("width")
        .and_then(|v| v.as_f64())
        .unwrap_or(256.0);
    let height = params
        .get("height")
        .and_then(|v| v.as_f64())
        .unwrap_or(256.0);
    let format_str = params
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("svg");

    let format = match format_str {
        "tikz" => OutputFormat::TikZ,
        _ => OutputFormat::Svg,
    };

    let gir_json = serde_json::to_string(gir_value).map_err(|e| format!("JSON error: {e}"))?;
    let gir = Gir::from_json(&gir_json).map_err(|e| format!("Invalid GIR: {e}"))?;

    let options = RenderOptions {
        format,
        width,
        height,
        embed_gir: false,
    };

    let output = renderer::render(&gir, &options).map_err(|e| format!("Render error: {e}"))?;

    Ok(json!({ "output": output, "format": format_str }))
}

fn handle_ul_deparse(params: &Value) -> Result<Value, String> {
    let gir_value = params.get("gir").ok_or("Missing required parameter: gir")?;

    let gir_json = serde_json::to_string(gir_value).map_err(|e| format!("JSON error: {e}"))?;
    let gir = Gir::from_json(&gir_json).map_err(|e| format!("Invalid GIR: {e}"))?;

    let ul_script = parser::deparse(&gir).map_err(|e| format!("Deparse error: {e}"))?;

    Ok(json!({ "ul_script": ul_script }))
}

fn handle_ul_parse_and_render(params: &Value) -> Result<Value, String> {
    let ul_script = params
        .get("ul_script")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: ul_script")?;
    let width = params
        .get("width")
        .and_then(|v| v.as_f64())
        .unwrap_or(256.0);
    let height = params
        .get("height")
        .and_then(|v| v.as_f64())
        .unwrap_or(256.0);

    let gir = parser::parse(ul_script).map_err(|e| format!("Parse error: {e}"))?;
    let gir_json: Value = serde_json::from_str(
        &gir.to_json_pretty().map_err(|e| format!("Serialize error: {e}"))?,
    )
    .map_err(|e| format!("JSON error: {e}"))?;

    let validation = validator::validate(&gir, false);
    let errors: Vec<String> = validation.errors.iter().map(|e| e.to_string()).collect();

    let svg = if validation.valid {
        let options = RenderOptions {
            format: OutputFormat::Svg,
            width,
            height,
            embed_gir: false,
        };
        Some(renderer::render(&gir, &options).map_err(|e| format!("Render error: {e}"))?)
    } else {
        None
    };

    Ok(json!({
        "gir": gir_json,
        "validation": {
            "valid": validation.valid,
            "errors": errors,
            "warnings": validation.warnings,
        },
        "svg": svg,
    }))
}

fn handle_ul_lexicon(params: &Value) -> Result<Value, String> {
    let query = params
        .get("query")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let entries = if query.is_empty() {
        ul_core::lexicon::all_entries().to_vec()
    } else {
        ul_core::lexicon::search(query).into_iter().cloned().collect()
    };

    let entries_json: Vec<Value> = entries
        .iter()
        .map(|e| {
            json!({
                "id": e.id,
                "level": e.level,
                "name": e.name,
                "tier": e.tier,
                "symbol": e.symbol,
                "sigma_ul": e.sigma_ul,
                "labels": e.labels,
            })
        })
        .collect();

    Ok(json!({ "entries": entries_json, "count": entries_json.len() }))
}

fn handle_ul_compose(params: &Value) -> Result<Value, String> {
    let operation = params
        .get("operation")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: operation")?;
    let operands = params
        .get("operands")
        .and_then(|v| v.as_array())
        .ok_or("Missing required parameter: operands (array of UL-Script strings)")?;

    let operand_strs: Vec<&str> = operands.iter().filter_map(|v| v.as_str()).collect();

    // Parse operands
    let girs: Vec<Gir> = operand_strs
        .iter()
        .map(|s| parser::parse(s).map_err(|e| format!("Parse error on '{}': {}", s, e)))
        .collect::<Result<Vec<_>, _>>()?;

    let result = match operation {
        "negate" => {
            ensure_operands(&girs, 1, "negate")?;
            composer::negate(&girs[0]).map_err(|e| format!("Compose error: {e}"))
        }
        "embed" => {
            ensure_operands(&girs, 1, "embed")?;
            composer::embed(&girs[0]).map_err(|e| format!("Compose error: {e}"))
        }
        "abstract" => {
            ensure_operands(&girs, 1, "abstract")?;
            composer::abstract_op(&girs[0]).map_err(|e| format!("Compose error: {e}"))
        }
        "invert" => {
            ensure_operands(&girs, 1, "invert")?;
            composer::invert(&girs[0]).map_err(|e| format!("Compose error: {e}"))
        }
        "predicate" => {
            ensure_operands(&girs, 3, "predicate")?;
            composer::predicate(&girs[0], &girs[1], &girs[2])
                .map_err(|e| format!("Compose error: {e}"))
        }
        "modify_entity" => {
            ensure_operands(&girs, 2, "modify_entity")?;
            composer::modify_entity(&girs[0], &girs[1])
                .map_err(|e| format!("Compose error: {e}"))
        }
        "modify_relation" => {
            ensure_operands(&girs, 2, "modify_relation")?;
            composer::modify_relation(&girs[0], &girs[1])
                .map_err(|e| format!("Compose error: {e}"))
        }
        "conjoin" => {
            ensure_operands(&girs, 2, "conjoin")?;
            composer::conjoin(&girs[0], &girs[1]).map_err(|e| format!("Compose error: {e}"))
        }
        "disjoin" => {
            ensure_operands(&girs, 2, "disjoin")?;
            composer::disjoin(&girs[0], &girs[1]).map_err(|e| format!("Compose error: {e}"))
        }
        "compose" => {
            ensure_operands(&girs, 2, "compose")?;
            composer::compose(&girs[0], &girs[1]).map_err(|e| format!("Compose error: {e}"))
        }
        "quantify" => {
            ensure_operands(&girs, 2, "quantify")?;
            composer::quantify(&girs[0], &girs[1]).map_err(|e| format!("Compose error: {e}"))
        }
        "bind" => {
            ensure_operands(&girs, 2, "bind")?;
            composer::bind(&girs[0], &girs[1]).map_err(|e| format!("Compose error: {e}"))
        }
        "modify_assertion" => {
            ensure_operands(&girs, 2, "modify_assertion")?;
            composer::modify_assertion(&girs[0], &girs[1])
                .map_err(|e| format!("Compose error: {e}"))
        }
        _ => Err(format!("Unknown operation: {operation}")),
    }?;

    let gir_json: Value =
        serde_json::from_str(&result.to_json_pretty().map_err(|e| format!("Serialize error: {e}"))?)
            .map_err(|e| format!("JSON error: {e}"))?;
    let ul_script = parser::deparse(&result).unwrap_or_default();

    Ok(json!({
        "gir": gir_json,
        "ul_script": ul_script
    }))
}

fn ensure_operands(girs: &[Gir], expected: usize, op: &str) -> Result<(), String> {
    if girs.len() < expected {
        Err(format!("{op} requires {expected} operand(s), got {}", girs.len()))
    } else {
        Ok(())
    }
}

fn handle_ul_analyze(params: &Value) -> Result<Value, String> {
    let gir_value = params.get("gir").ok_or("Missing required parameter: gir")?;
    let gir_json = serde_json::to_string(gir_value).map_err(|e| format!("JSON error: {e}"))?;
    let gir = Gir::from_json(&gir_json).map_err(|e| format!("Invalid GIR: {e}"))?;

    let ops = composer::detect_operations(&gir);
    let op_names: Vec<&str> = ops.iter().map(|o| o.operation).collect();

    Ok(json!({
        "operations": op_names,
        "node_count": gir.nodes.len(),
        "edge_count": gir.edges.len(),
        "root": gir.root,
    }))
}

fn handle_ul_set_force(params: &Value) -> Result<Value, String> {
    let gir_value = params.get("gir").ok_or("Missing required parameter: gir")?;
    let force_str = params
        .get("force")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: force")?;

    let gir_json = serde_json::to_string(gir_value).map_err(|e| format!("JSON error: {e}"))?;
    let gir = Gir::from_json(&gir_json).map_err(|e| format!("Invalid GIR: {e}"))?;

    let force = match force_str {
        "assert" => ul_core::PerformativeForce::Assert,
        "query" => ul_core::PerformativeForce::Query,
        "direct" => ul_core::PerformativeForce::Direct,
        "commit" => ul_core::PerformativeForce::Commit,
        "express" => ul_core::PerformativeForce::Express,
        "declare" => ul_core::PerformativeForce::Declare,
        other => return Err(format!("Unknown force: {other}")),
    };

    let result = ul_core::performative::with_force(&gir, force)
        .map_err(|e| format!("Force error: {e}"))?;

    let result_json: Value =
        serde_json::from_str(&result.to_json_pretty().map_err(|e| format!("Serialize error: {e}"))?)
            .map_err(|e| format!("JSON error: {e}"))?;
    let ul_script = parser::deparse(&result).unwrap_or_default();

    Ok(json!({
        "gir": result_json,
        "ul_script": ul_script,
        "force": force_str
    }))
}

fn handle_ul_infer_pragmatics(params: &Value) -> Result<Value, String> {
    let gir_value = params.get("gir").ok_or("Missing required parameter: gir")?;

    let gir_json = serde_json::to_string(gir_value).map_err(|e| format!("JSON error: {e}"))?;
    let gir = Gir::from_json(&gir_json).map_err(|e| format!("Invalid GIR: {e}"))?;

    let inferences = ul_core::pragmatic::infer(&gir);
    let results: Vec<Value> = inferences
        .iter()
        .map(|inf| {
            json!({
                "rule": format!("{:?}", inf.rule),
                "surface": serde_json::to_value(&inf.surface).unwrap_or_default(),
                "intended": serde_json::to_value(&inf.intended).unwrap_or_default(),
            })
        })
        .collect();

    Ok(json!({
        "inferences": results,
        "count": results.len()
    }))
}

// ── MCP request dispatch ──

fn handle_request(req: &JsonRpcRequest) -> JsonRpcResponse {
    let id = req.id.clone().unwrap_or(Value::Null);

    match req.method.as_str() {
        // MCP lifecycle
        "initialize" => JsonRpcResponse::success(
            id,
            json!({
                "protocolVersion": MCP_VERSION,
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": SERVER_NAME,
                    "version": SERVER_VERSION,
                }
            }),
        ),

        "notifications/initialized" => {
            // Acknowledgment — no response needed for notifications
            return JsonRpcResponse::success(id, json!(null));
        }

        // Tool discovery
        "tools/list" => JsonRpcResponse::success(
            id,
            json!({ "tools": tool_definitions() }),
        ),

        // Tool execution
        "tools/call" => {
            let tool_name = req
                .params
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let arguments = req
                .params
                .get("arguments")
                .cloned()
                .unwrap_or(json!({}));

            let result = match tool_name {
                "ul_parse" => handle_ul_parse(&arguments),
                "ul_validate" => handle_ul_validate(&arguments),
                "ul_render" => handle_ul_render(&arguments),
                "ul_deparse" => handle_ul_deparse(&arguments),
                "ul_parse_and_render" => handle_ul_parse_and_render(&arguments),
                "ul_lexicon" => handle_ul_lexicon(&arguments),
                "ul_compose" => handle_ul_compose(&arguments),
                "ul_analyze" => handle_ul_analyze(&arguments),
                "ul_set_force" => handle_ul_set_force(&arguments),
                "ul_infer_pragmatics" => handle_ul_infer_pragmatics(&arguments),
                _ => Err(format!("Unknown tool: {tool_name}")),
            };

            match result {
                Ok(value) => JsonRpcResponse::success(
                    id,
                    json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&value).unwrap_or_default()
                        }]
                    }),
                ),
                Err(msg) => JsonRpcResponse::success(
                    id,
                    json!({
                        "content": [{
                            "type": "text",
                            "text": msg
                        }],
                        "isError": true
                    }),
                ),
            }
        }

        // Ping
        "ping" => JsonRpcResponse::success(id, json!({})),

        _ => JsonRpcResponse::error(id, -32601, format!("Method not found: {}", req.method)),
    }
}

// ── Main loop ──

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    eprintln!("{SERVER_NAME} v{SERVER_VERSION} — MCP server for Universal Language");
    eprintln!("Listening on stdio (JSON-RPC 2.0)...");

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("stdin read error: {e}");
                break;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let request: JsonRpcRequest = match serde_json::from_str(trimmed) {
            Ok(r) => r,
            Err(e) => {
                let err_resp = JsonRpcResponse::error(
                    Value::Null,
                    -32700,
                    format!("Parse error: {e}"),
                );
                let _ = writeln!(stdout.lock(), "{}", serde_json::to_string(&err_resp).unwrap());
                continue;
            }
        };

        // Notifications (no id) don't get a response
        if request.id.is_none() && request.method.starts_with("notifications/") {
            continue;
        }

        let response = handle_request(&request);
        let json_out = serde_json::to_string(&response).unwrap();
        let _ = writeln!(stdout.lock(), "{json_out}");
        let _ = stdout.lock().flush();
    }
}

// ── Tests ──

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ── ul_parse ──

    #[test]
    fn parse_valid_point() {
        let params = json!({ "ul_script": "●" });
        let result = handle_ul_parse(&params).unwrap();
        assert!(result.get("gir").is_some());
        let gir = result.get("gir").unwrap();
        assert!(gir.get("nodes").unwrap().as_array().unwrap().len() > 0);
    }

    #[test]
    fn parse_invalid_input() {
        let params = json!({ "ul_script": "(((" });
        let result = handle_ul_parse(&params);
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_param() {
        let params = json!({});
        let result = handle_ul_parse(&params);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing required parameter"));
    }

    // ── ul_validate ──

    #[test]
    fn validate_parsed_gir() {
        let parse_result = handle_ul_parse(&json!({ "ul_script": "●" })).unwrap();
        let gir = parse_result.get("gir").unwrap();
        let params = json!({ "gir": gir });
        let result = handle_ul_validate(&params).unwrap();
        assert_eq!(result.get("valid").unwrap().as_bool().unwrap(), true);
    }

    #[test]
    fn validate_missing_gir() {
        let params = json!({});
        let result = handle_ul_validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn validate_with_geometry_flag() {
        let parse_result = handle_ul_parse(&json!({ "ul_script": "●" })).unwrap();
        let gir = parse_result.get("gir").unwrap();
        let params = json!({ "gir": gir, "check_geometry": true });
        let result = handle_ul_validate(&params).unwrap();
        assert!(result.get("valid").is_some());
    }

    // ── ul_render ──

    #[test]
    fn render_produces_svg() {
        let parse_result = handle_ul_parse(&json!({ "ul_script": "●" })).unwrap();
        let gir = parse_result.get("gir").unwrap();
        let params = json!({ "gir": gir, "width": 128, "height": 128 });
        let result = handle_ul_render(&params).unwrap();
        let output = result.get("output").unwrap().as_str().unwrap();
        assert!(output.contains("<svg"));
        assert_eq!(result.get("format").unwrap().as_str().unwrap(), "svg");
    }

    #[test]
    fn render_tikz_format() {
        let parse_result = handle_ul_parse(&json!({ "ul_script": "●" })).unwrap();
        let gir = parse_result.get("gir").unwrap();
        let params = json!({ "gir": gir, "format": "tikz" });
        let result = handle_ul_render(&params).unwrap();
        assert_eq!(result.get("format").unwrap().as_str().unwrap(), "tikz");
    }

    // ── ul_deparse ──

    #[test]
    fn deparse_roundtrip() {
        let parse_result = handle_ul_parse(&json!({ "ul_script": "●" })).unwrap();
        let gir = parse_result.get("gir").unwrap();
        let params = json!({ "gir": gir });
        let result = handle_ul_deparse(&params).unwrap();
        assert!(result.get("ul_script").unwrap().as_str().unwrap().len() > 0);
    }

    // ── ul_parse_and_render ──

    #[test]
    fn parse_and_render_valid() {
        let params = json!({ "ul_script": "●", "width": 200, "height": 200 });
        let result = handle_ul_parse_and_render(&params).unwrap();
        assert!(result.get("gir").is_some());
        assert_eq!(
            result
                .get("validation")
                .unwrap()
                .get("valid")
                .unwrap()
                .as_bool()
                .unwrap(),
            true
        );
        assert!(result.get("svg").unwrap().as_str().unwrap().contains("<svg"));
    }

    #[test]
    fn parse_and_render_invalid() {
        let params = json!({ "ul_script": "(((" });
        let result = handle_ul_parse_and_render(&params);
        assert!(result.is_err());
    }

    // ── ul_lexicon ──

    #[test]
    fn lexicon_all_entries() {
        let params = json!({ "query": "" });
        let result = handle_ul_lexicon(&params).unwrap();
        let count = result.get("count").unwrap().as_u64().unwrap();
        assert!(count > 0, "Lexicon should have entries");
        assert_eq!(
            result.get("entries").unwrap().as_array().unwrap().len() as u64,
            count
        );
    }

    #[test]
    fn lexicon_search_query() {
        let params = json!({ "query": "point" });
        let result = handle_ul_lexicon(&params).unwrap();
        let count = result.get("count").unwrap().as_u64().unwrap();
        assert!(count > 0, "Search for 'point' should return results");
    }

    #[test]
    fn lexicon_nonexistent() {
        let params = json!({ "query": "xyzzy_nonexistent_42" });
        let result = handle_ul_lexicon(&params).unwrap();
        let count = result.get("count").unwrap().as_u64().unwrap();
        assert_eq!(count, 0);
    }

    // ── MCP protocol dispatch ──

    #[test]
    fn mcp_initialize() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(json!(1)),
            method: "initialize".into(),
            params: json!({}),
        };
        let resp = handle_request(&req);
        let result = resp.result.unwrap();
        assert_eq!(result.get("protocolVersion").unwrap().as_str().unwrap(), MCP_VERSION);
    }

    #[test]
    fn mcp_tools_list() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(json!(2)),
            method: "tools/list".into(),
            params: json!({}),
        };
        let resp = handle_request(&req);
        let result = resp.result.unwrap();
        let tools = result.get("tools").unwrap().as_array().unwrap();
        assert_eq!(tools.len(), 10);
    }

    #[test]
    fn mcp_unknown_method() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(json!(99)),
            method: "nonexistent/method".into(),
            params: json!({}),
        };
        let resp = handle_request(&req);
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, -32601);
    }

    #[test]
    fn mcp_tools_call_dispatch() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(json!(3)),
            method: "tools/call".into(),
            params: json!({
                "name": "ul_parse",
                "arguments": { "ul_script": "●" }
            }),
        };
        let resp = handle_request(&req);
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn mcp_tools_call_unknown_tool() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(json!(4)),
            method: "tools/call".into(),
            params: json!({
                "name": "nonexistent_tool",
                "arguments": {}
            }),
        };
        let resp = handle_request(&req);
        // Unknown tools return a success with isError: true (MCP convention)
        let result = resp.result.unwrap();
        assert_eq!(result.get("isError").unwrap().as_bool().unwrap(), true);
    }

    #[test]
    fn mcp_ping() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(json!(5)),
            method: "ping".into(),
            params: json!({}),
        };
        let resp = handle_request(&req);
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    // ── ul_compose ──

    #[test]
    fn compose_invert() {
        let params = json!({
            "operation": "invert",
            "operands": ["→"]
        });
        let result = handle_ul_compose(&params).unwrap();
        assert!(result.get("gir").is_some());
        assert!(result.get("ul_script").is_some());
    }

    #[test]
    fn compose_modify_entity() {
        let params = json!({
            "operation": "modify_entity",
            "operands": ["∠60", "●"]
        });
        let result = handle_ul_compose(&params).unwrap();
        assert!(result.get("gir").is_some());
    }

    #[test]
    fn compose_missing_operands() {
        let params = json!({
            "operation": "negate",
            "operands": []
        });
        let result = handle_ul_compose(&params);
        assert!(result.is_err());
    }

    #[test]
    fn compose_unknown_operation() {
        let params = json!({
            "operation": "nonexistent",
            "operands": ["●"]
        });
        let result = handle_ul_compose(&params);
        assert!(result.is_err());
    }

    // ── ul_analyze ──

    #[test]
    fn analyze_simple_gir() {
        let parse_result = handle_ul_parse(&json!({ "ul_script": "○{●}" })).unwrap();
        let gir = parse_result.get("gir").unwrap();
        let params = json!({ "gir": gir });
        let result = handle_ul_analyze(&params).unwrap();
        assert!(result.get("operations").is_some());
        assert!(result.get("node_count").is_some());
        assert!(result.get("edge_count").is_some());
    }

    #[test]
    fn analyze_missing_gir() {
        let params = json!({});
        let result = handle_ul_analyze(&params);
        assert!(result.is_err());
    }
}
