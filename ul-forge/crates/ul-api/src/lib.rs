//! UL Forge API — library interface for testing and embedding.

mod routes;

use axum::Router;
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};

/// Shared state accessible to all handlers.
#[derive(Clone)]
pub struct AppState {
    pub start_time: Instant,
}

/// Create the API router with all routes and middleware.
pub fn create_app() -> Router {
    let state = AppState {
        start_time: Instant::now(),
    };

    let cors = if cfg!(debug_assertions) {
        // Permissive CORS for development
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        // Restrictive CORS for production — configure via UL_FORGE_CORS_ORIGINS
        let origins = std::env::var("UL_FORGE_CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173".to_string());
        let allowed: Vec<axum::http::HeaderValue> = origins
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(allowed)
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
            .allow_headers([axum::http::header::CONTENT_TYPE])
    };

    Router::new()
        .merge(routes::api_routes())
        .layer(cors)
        .with_state(state)
}
