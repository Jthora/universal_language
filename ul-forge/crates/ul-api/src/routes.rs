//! Route handlers for all API endpoints.

use axum::{
    Router,
    extract::{State, ws::{Message, WebSocket, WebSocketUpgrade}},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::AppState;

// ────────────────────────────────────────────
// JSON schemas
// ────────────────────────────────────────────

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
    uptime_seconds: u64,
}

#[derive(Serialize)]
struct ParseResponse {
    gir: serde_json::Value,
    warnings: Vec<String>,
}

#[derive(Deserialize)]
struct RenderRequest {
    gir: serde_json::Value,
    #[serde(default)]
    options: RenderOpts,
}

#[derive(Deserialize, Default)]
struct RenderOpts {
    #[serde(default = "default_format")]
    format: String,
    #[serde(default = "default_dim")]
    width: f64,
    #[serde(default = "default_dim")]
    height: f64,
    #[serde(default = "default_true")]
    embed_gir: bool,
}

fn default_format() -> String {
    "svg".into()
}
fn default_dim() -> f64 {
    400.0
}
fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
struct ValidateRequest {
    gir: serde_json::Value,
    #[serde(default)]
    options: ValidateOpts,
}

#[derive(Deserialize, Default)]
struct ValidateOpts {
    #[serde(default)]
    check_geometry: bool,
}

#[derive(Serialize)]
struct ValidationResponse {
    valid: bool,
    errors: Vec<String>,
    warnings: Vec<String>,
}

#[derive(Deserialize)]
struct ConvertRequest {
    input: String,
    input_format: String,
    output_format: String,
}

#[derive(Deserialize)]
struct ComposeRequest {
    operation: String,
    operands: Vec<String>,
}

#[derive(Serialize)]
struct ComposeResponse {
    gir: serde_json::Value,
    ul_script: String,
}

#[derive(Deserialize)]
struct AnalyzeRequest {
    gir: serde_json::Value,
}

#[derive(Serialize)]
struct AnalyzeResponse {
    operations: Vec<String>,
    node_count: usize,
    edge_count: usize,
}

#[derive(Deserialize)]
struct ForceRequest {
    gir: serde_json::Value,
    force: String,
}

#[derive(Serialize)]
struct ForceResponse {
    gir: serde_json::Value,
    ul_script: String,
    force: String,
}

#[derive(Deserialize)]
struct PragmaticsRequest {
    gir: serde_json::Value,
}

#[derive(Serialize)]
struct PragmaticsResponse {
    inferences: Vec<PragmaticInferenceItem>,
    count: usize,
}

#[derive(Serialize)]
struct PragmaticInferenceItem {
    rule: String,
    surface: serde_json::Value,
    intended: serde_json::Value,
}

#[derive(Serialize)]
struct ApiError {
    error: ApiErrorBody,
}

#[derive(Serialize)]
struct ApiErrorBody {
    code: String,
    message: String,
}

impl ApiError {
    fn new(code: &str, message: impl Into<String>) -> (StatusCode, Json<ApiError>) {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: ApiErrorBody {
                    code: code.to_string(),
                    message: message.into(),
                },
            }),
        )
    }
}

// ────────────────────────────────────────────
// Router
// ────────────────────────────────────────────

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/parse", post(parse))
        .route("/render", post(render))
        .route("/validate", post(validate))
        .route("/convert", post(convert))
        .route("/compose", post(compose))
        .route("/analyze", post(analyze))
        .route("/force", post(set_force))
        .route("/pragmatics", post(infer_pragmatics))
        .route("/live", get(ws_upgrade))
}

// ────────────────────────────────────────────
// GET /health
// ────────────────────────────────────────────

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: state.start_time.elapsed().as_secs(),
    })
}

// ────────────────────────────────────────────
// POST /parse
// ────────────────────────────────────────────

async fn parse(body: String) -> Result<Json<ParseResponse>, (StatusCode, Json<ApiError>)> {
    const MAX_LEN: usize = 100_000;
    if body.len() > MAX_LEN {
        return Err(ApiError::new("PAYLOAD_TOO_LARGE", "Input exceeds 100KB limit"));
    }

    let result = ul_core::parser::parse_with_diagnostics(&body);

    match result {
        Ok(pr) => {
            let gir_json = serde_json::to_value(&pr.gir)
                .map_err(|e| ApiError::new("INTERNAL_ERROR", e.to_string()))?;
            Ok(Json(ParseResponse {
                gir: gir_json,
                warnings: pr.warnings,
            }))
        }
        Err(e) => Err(ApiError::new("PARSE_ERROR", e.to_string())),
    }
}

// ────────────────────────────────────────────
// POST /render
// ────────────────────────────────────────────

async fn render(body: axum::body::Bytes) -> Response {
    const MAX_LEN: usize = 100_000;
    if body.len() > MAX_LEN {
        let (status, json) = ApiError::new("PAYLOAD_TOO_LARGE", "Input exceeds 100KB limit");
        return (status, json).into_response();
    }

    let req: RenderRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => {
            let (status, json) = ApiError::new("INVALID_JSON", e.to_string());
            return (status, json).into_response();
        }
    };

    let gir_str = req.gir.to_string();
    let gir = match ul_core::Gir::from_json(&gir_str) {
        Ok(g) => g,
        Err(e) => {
            let (status, json) = ApiError::new("INVALID_GIR", e.to_string());
            return (status, json).into_response();
        }
    };

    let format = match req.options.format.as_str() {
        "tikz" => ul_core::renderer::OutputFormat::TikZ,
        _ => ul_core::renderer::OutputFormat::Svg,
    };

    let opts = ul_core::renderer::RenderOptions {
        format,
        width: req.options.width,
        height: req.options.height,
        embed_gir: req.options.embed_gir,
    };

    match ul_core::renderer::render(&gir, &opts) {
        Ok(output) => {
            let content_type = match format {
                ul_core::renderer::OutputFormat::Svg => "image/svg+xml",
                ul_core::renderer::OutputFormat::TikZ => "text/plain",
            };
            ([(axum::http::header::CONTENT_TYPE, content_type)], output).into_response()
        }
        Err(e) => {
            let (status, json) = ApiError::new("RENDER_ERROR", e.to_string());
            (status, json).into_response()
        }
    }
}

// ────────────────────────────────────────────
// POST /validate
// ────────────────────────────────────────────

async fn validate(
    body: axum::body::Bytes,
) -> Result<Json<ValidationResponse>, (StatusCode, Json<ApiError>)> {
    const MAX_LEN: usize = 100_000;
    if body.len() > MAX_LEN {
        return Err(ApiError::new("PAYLOAD_TOO_LARGE", "Input exceeds 100KB limit"));
    }

    let req: ValidateRequest = serde_json::from_slice(&body)
        .map_err(|e| ApiError::new("INVALID_JSON", e.to_string()))?;

    let gir_str = req.gir.to_string();
    let gir = ul_core::Gir::from_json(&gir_str)
        .map_err(|e| ApiError::new("INVALID_GIR", e.to_string()))?;

    let result = ul_core::validator::validate(&gir, req.options.check_geometry);

    Ok(Json(ValidationResponse {
        valid: result.valid,
        errors: result.errors.iter().map(|e| e.to_string()).collect(),
        warnings: result.warnings,
    }))
}

// ────────────────────────────────────────────
// POST /convert
// ────────────────────────────────────────────

async fn convert(Json(req): Json<ConvertRequest>) -> Response {
    const MAX_LEN: usize = 100_000;
    if req.input.len() > MAX_LEN {
        let (status, json) = ApiError::new("PAYLOAD_TOO_LARGE", "Input exceeds 100KB limit");
        return (status, json).into_response();
    }

    // Step 1: Parse input to GIR
    let gir = match req.input_format.as_str() {
        "ul-script" => match ul_core::parser::parse(&req.input) {
            Ok(g) => g,
            Err(e) => {
                let (status, json) = ApiError::new("PARSE_ERROR", e.to_string());
                return (status, json).into_response();
            }
        },
        "gir-json" => match ul_core::Gir::from_json(&req.input) {
            Ok(g) => g,
            Err(e) => {
                let (status, json) = ApiError::new("INVALID_GIR", e.to_string());
                return (status, json).into_response();
            }
        },
        other => {
            let (status, json) = ApiError::new(
                "UNSUPPORTED_FORMAT",
                format!("Unsupported input format: {other}"),
            );
            return (status, json).into_response();
        }
    };

    // Step 2: Convert GIR to output format
    match req.output_format.as_str() {
        "ul-script" => match ul_core::parser::deparse(&gir) {
            Ok(text) => ([(axum::http::header::CONTENT_TYPE, "text/plain")], text).into_response(),
            Err(e) => {
                let (status, json) = ApiError::new("DEPARSE_ERROR", e.to_string());
                (status, json).into_response()
            }
        },
        "gir-json" => match gir.to_json() {
            Ok(json) => {
                ([(axum::http::header::CONTENT_TYPE, "application/json")], json).into_response()
            }
            Err(e) => {
                let (status, json) = ApiError::new("INTERNAL_ERROR", e.to_string());
                (status, json).into_response()
            }
        },
        "svg" | "tikz" => {
            let format = if req.output_format == "tikz" {
                ul_core::renderer::OutputFormat::TikZ
            } else {
                ul_core::renderer::OutputFormat::Svg
            };
            let opts = ul_core::renderer::RenderOptions {
                format,
                ..Default::default()
            };
            match ul_core::renderer::render(&gir, &opts) {
                Ok(output) => {
                    let ct = if req.output_format == "tikz" {
                        "text/plain"
                    } else {
                        "image/svg+xml"
                    };
                    ([(axum::http::header::CONTENT_TYPE, ct)], output).into_response()
                }
                Err(e) => {
                    let (status, json) = ApiError::new("RENDER_ERROR", e.to_string());
                    (status, json).into_response()
                }
            }
        }
        other => {
            let (status, json) = ApiError::new(
                "UNSUPPORTED_FORMAT",
                format!("Unsupported output format: {other}"),
            );
            (status, json).into_response()
        }
    }
}

// ────────────────────────────────────────────
// POST /compose
// ────────────────────────────────────────────

async fn compose(
    Json(req): Json<ComposeRequest>,
) -> Result<Json<ComposeResponse>, (StatusCode, Json<ApiError>)> {
    if req.operands.iter().any(|s| s.len() > 100_000) {
        return Err(ApiError::new("PAYLOAD_TOO_LARGE", "Operand exceeds 100KB limit"));
    }

    let girs: Vec<ul_core::Gir> = req
        .operands
        .iter()
        .map(|s| ul_core::parser::parse(s).map_err(|e| {
            ApiError::new("PARSE_ERROR", format!("Parse error on '{}': {e}", s))
        }))
        .collect::<Result<Vec<_>, _>>()?;

    let result = match req.operation.as_str() {
        "negate" => { check_arity(&girs, 1, "negate")?; ul_core::composer::negate(&girs[0]) }
        "embed" => { check_arity(&girs, 1, "embed")?; ul_core::composer::embed(&girs[0]) }
        "abstract" => { check_arity(&girs, 1, "abstract")?; ul_core::composer::abstract_op(&girs[0]) }
        "invert" => { check_arity(&girs, 1, "invert")?; ul_core::composer::invert(&girs[0]) }
        "predicate" => { check_arity(&girs, 3, "predicate")?; ul_core::composer::predicate(&girs[0], &girs[1], &girs[2]) }
        "modify_entity" => { check_arity(&girs, 2, "modify_entity")?; ul_core::composer::modify_entity(&girs[0], &girs[1]) }
        "modify_relation" => { check_arity(&girs, 2, "modify_relation")?; ul_core::composer::modify_relation(&girs[0], &girs[1]) }
        "conjoin" => { check_arity(&girs, 2, "conjoin")?; ul_core::composer::conjoin(&girs[0], &girs[1]) }
        "disjoin" => { check_arity(&girs, 2, "disjoin")?; ul_core::composer::disjoin(&girs[0], &girs[1]) }
        "compose" => { check_arity(&girs, 2, "compose")?; ul_core::composer::compose(&girs[0], &girs[1]) }
        "quantify" => { check_arity(&girs, 2, "quantify")?; ul_core::composer::quantify(&girs[0], &girs[1]) }
        "bind" => { check_arity(&girs, 2, "bind")?; ul_core::composer::bind(&girs[0], &girs[1]) }
        "modify_assertion" => { check_arity(&girs, 2, "modify_assertion")?; ul_core::composer::modify_assertion(&girs[0], &girs[1]) }
        other => return Err(ApiError::new("UNKNOWN_OPERATION", format!("Unknown operation: {other}"))),
    }.map_err(|e| ApiError::new("COMPOSE_ERROR", e.to_string()))?;

    let gir_json = serde_json::to_value(&result)
        .map_err(|e| ApiError::new("INTERNAL_ERROR", e.to_string()))?;
    let ul_script = ul_core::parser::deparse(&result).unwrap_or_default();

    Ok(Json(ComposeResponse { gir: gir_json, ul_script }))
}

fn check_arity(
    girs: &[ul_core::Gir],
    expected: usize,
    op: &str,
) -> Result<(), (StatusCode, Json<ApiError>)> {
    if girs.len() < expected {
        Err(ApiError::new(
            "INVALID_OPERANDS",
            format!("{op} requires {expected} operand(s), got {}", girs.len()),
        ))
    } else {
        Ok(())
    }
}

// ────────────────────────────────────────────
// POST /analyze
// ────────────────────────────────────────────

async fn analyze(
    Json(req): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, (StatusCode, Json<ApiError>)> {
    let gir_str = req.gir.to_string();
    let gir = ul_core::Gir::from_json(&gir_str)
        .map_err(|e| ApiError::new("INVALID_GIR", e.to_string()))?;

    let ops = ul_core::composer::detect_operations(&gir);
    let op_names: Vec<String> = ops.iter().map(|o| o.operation.to_string()).collect();

    Ok(Json(AnalyzeResponse {
        operations: op_names,
        node_count: gir.nodes.len(),
        edge_count: gir.edges.len(),
    }))
}

// ────────────────────────────────────────────
// POST /force
// ────────────────────────────────────────────

async fn set_force(
    Json(req): Json<ForceRequest>,
) -> Result<Json<ForceResponse>, (StatusCode, Json<ApiError>)> {
    let gir_str = req.gir.to_string();
    let gir = ul_core::Gir::from_json(&gir_str)
        .map_err(|e| ApiError::new("INVALID_GIR", e.to_string()))?;

    let force = match req.force.as_str() {
        "assert" => ul_core::PerformativeForce::Assert,
        "query" => ul_core::PerformativeForce::Query,
        "direct" => ul_core::PerformativeForce::Direct,
        "commit" => ul_core::PerformativeForce::Commit,
        "express" => ul_core::PerformativeForce::Express,
        "declare" => ul_core::PerformativeForce::Declare,
        other => return Err(ApiError::new("UNKNOWN_FORCE", format!("Unknown force: {other}"))),
    };

    let result = ul_core::performative::with_force(&gir, force)
        .map_err(|e| ApiError::new("FORCE_ERROR", e.to_string()))?;

    let gir_json = serde_json::to_value(&result)
        .map_err(|e| ApiError::new("INTERNAL_ERROR", e.to_string()))?;
    let ul_script = ul_core::parser::deparse(&result).unwrap_or_default();

    Ok(Json(ForceResponse {
        gir: gir_json,
        ul_script,
        force: req.force,
    }))
}

// ────────────────────────────────────────────
// POST /pragmatics
// ────────────────────────────────────────────

async fn infer_pragmatics(
    Json(req): Json<PragmaticsRequest>,
) -> Result<Json<PragmaticsResponse>, (StatusCode, Json<ApiError>)> {
    let gir_str = req.gir.to_string();
    let gir = ul_core::Gir::from_json(&gir_str)
        .map_err(|e| ApiError::new("INVALID_GIR", e.to_string()))?;

    let inferences = ul_core::pragmatic::infer(&gir);
    let items: Vec<PragmaticInferenceItem> = inferences
        .iter()
        .map(|inf| PragmaticInferenceItem {
            rule: format!("{:?}", inf.rule),
            surface: serde_json::to_value(&inf.surface).unwrap_or_default(),
            intended: serde_json::to_value(&inf.intended).unwrap_or_default(),
        })
        .collect();

    Ok(Json(PragmaticsResponse {
        count: items.len(),
        inferences: items,
    }))
}

// ────────────────────────────────────────────
// WS /live — WebSocket live preview
// ────────────────────────────────────────────

async fn ws_upgrade(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_ws)
}

async fn handle_ws(mut socket: WebSocket) {
    // Debounce: wait for a pause in messages before processing
    let debounce = Duration::from_millis(100);
    let mut pending: Option<String> = None;

    loop {
        // Try to receive with a timeout for debouncing
        let timeout = if pending.is_some() {
            debounce
        } else {
            Duration::from_secs(300) // 5 min idle timeout
        };

        match tokio::time::timeout(timeout, socket.recv()).await {
            // Received a message
            Ok(Some(Ok(Message::Text(text)))) => {
                // Try to parse as JSON
                if let Ok(msg) = serde_json::from_str::<WsClientMessage>(&text) {
                    match msg.msg_type.as_str() {
                        "edit" => {
                            pending = msg.payload.and_then(|p| p.text);
                        }
                        _ => {} // Ignore unknown message types
                    }
                }
            }
            // Timeout → flush pending edit
            Err(_) => {
                if let Some(ref source) = pending {
                    let response = process_live_edit(source);
                    if socket
                        .send(Message::Text(response.into()))
                        .await
                        .is_err()
                    {
                        break; // Client disconnected
                    }
                    pending = None;
                } else {
                    // Idle timeout with no pending work
                    break;
                }
            }
            // Connection closed or error
            Ok(None) | Ok(Some(Err(_))) => break,
            // Binary/ping/pong — ignore
            Ok(Some(Ok(_))) => {}
        }
    }
}

#[derive(Deserialize)]
struct WsClientMessage {
    #[serde(rename = "type")]
    msg_type: String,
    #[allow(dead_code)]
    id: Option<String>,
    payload: Option<WsEditPayload>,
}

#[derive(Deserialize)]
struct WsEditPayload {
    text: Option<String>,
}

#[derive(Serialize)]
struct WsServerMessage {
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    payload: serde_json::Value,
}

fn process_live_edit(source: &str) -> String {
    let result = ul_core::parser::parse(source);

    match result {
        Ok(gir) => {
            let validation = ul_core::validator::validate(&gir, false);
            let svg = if validation.valid {
                ul_core::renderer::render(&gir, &ul_core::renderer::RenderOptions::default()).ok()
            } else {
                None
            };

            let gir_json = serde_json::to_value(&gir).unwrap_or_default();

            serde_json::to_string(&WsServerMessage {
                msg_type: "update".into(),
                id: None,
                payload: serde_json::json!({
                    "gir": gir_json,
                    "svg": svg,
                    "validation": {
                        "valid": validation.valid,
                        "errors": validation.errors.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
                        "warnings": validation.warnings,
                    },
                    "error": null,
                }),
            })
            .unwrap_or_default()
        }
        Err(e) => serde_json::to_string(&WsServerMessage {
            msg_type: "update".into(),
            id: None,
            payload: serde_json::json!({
                "gir": null,
                "svg": null,
                "validation": null,
                "error": e.to_string(),
            }),
        })
        .unwrap_or_default(),
    }
}
