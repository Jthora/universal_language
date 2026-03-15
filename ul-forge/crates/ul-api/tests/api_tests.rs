//! Integration tests for the UL Forge API server.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn app() -> axum::Router {
    ul_api::create_app()
}

// ── GET /health ──

#[tokio::test]
async fn health_returns_ok() {
    let resp = app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
    assert!(json["version"].is_string());
    assert!(json["uptime_seconds"].is_number());
}

// ── POST /parse ──

#[tokio::test]
async fn parse_valid_script() {
    let resp = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/parse")
                .body(Body::from("○{●}"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["gir"].is_object());
    assert!(json["warnings"].is_array());
}

#[tokio::test]
async fn parse_invalid_script_returns_400() {
    let resp = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/parse")
                .body(Body::from("!!!invalid!!!"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["error"]["code"].is_string());
}

// ── POST /render ──

#[tokio::test]
async fn render_valid_gir() {
    // First parse to get valid GIR
    let parse_resp = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/parse")
                .body(Body::from("○{●}"))
                .unwrap(),
        )
        .await
        .unwrap();
    let parse_body = parse_resp.into_body().collect().await.unwrap().to_bytes();
    let parsed: serde_json::Value = serde_json::from_slice(&parse_body).unwrap();

    let render_req = serde_json::json!({
        "gir": parsed["gir"],
    });

    let resp = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/render")
                .header("content-type", "application/json")
                .body(Body::from(render_req.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let svg = String::from_utf8(body.to_vec()).unwrap();
    assert!(svg.contains("<svg"));
}

// ── POST /validate ──

#[tokio::test]
async fn validate_valid_gir() {
    let parse_resp = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/parse")
                .body(Body::from("● → ●"))
                .unwrap(),
        )
        .await
        .unwrap();
    let parse_body = parse_resp.into_body().collect().await.unwrap().to_bytes();
    let parsed: serde_json::Value = serde_json::from_slice(&parse_body).unwrap();

    let validate_req = serde_json::json!({
        "gir": parsed["gir"],
    });

    let resp = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/validate")
                .header("content-type", "application/json")
                .body(Body::from(validate_req.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["valid"], true);
}

// ── POST /convert ──

#[tokio::test]
async fn convert_ul_script_to_svg() {
    let req = serde_json::json!({
        "input": "○{●}",
        "input_format": "ul-script",
        "output_format": "svg",
    });

    let resp = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/convert")
                .header("content-type", "application/json")
                .body(Body::from(req.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let svg = String::from_utf8(body.to_vec()).unwrap();
    assert!(svg.contains("<svg"));
}

#[tokio::test]
async fn convert_ul_script_to_gir_json() {
    let req = serde_json::json!({
        "input": "●",
        "input_format": "ul-script",
        "output_format": "gir-json",
    });

    let resp = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/convert")
                .header("content-type", "application/json")
                .body(Body::from(req.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let text = String::from_utf8(body.to_vec()).unwrap();
    // Should be valid JSON
    let gir: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert!(gir["nodes"].is_array());
}

#[tokio::test]
async fn convert_unsupported_format_returns_400() {
    let req = serde_json::json!({
        "input": "●",
        "input_format": "xml",
        "output_format": "svg",
    });

    let resp = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/convert")
                .header("content-type", "application/json")
                .body(Body::from(req.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ── CORS ──

#[tokio::test]
async fn cors_allows_any_origin() {
    let resp = app()
        .oneshot(
            Request::builder()
                .method("OPTIONS")
                .uri("/parse")
                .header("origin", "http://example.com")
                .header("access-control-request-method", "POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let headers = resp.headers();
    assert!(headers.contains_key("access-control-allow-origin"));
}
