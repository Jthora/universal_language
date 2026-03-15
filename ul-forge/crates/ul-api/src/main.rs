//! UL Forge API Server — Axum-based REST + WebSocket service.
//!
//! Endpoints:
//!   POST /parse      — UL-Script text → GIR JSON
//!   POST /render     — GIR JSON → SVG
//!   POST /validate   — GIR JSON → validation result
//!   POST /convert    — Any format → any format
//!   GET  /health     — Health check
//!   WS   /live       — WebSocket live preview

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "ul-serve", about = "UL Forge API server")]
struct Cli {
    /// Port to listen on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// Bind address
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let cli = Cli::parse();
    let app = ul_api::create_app();
    let addr = format!("{}:{}", cli.host, cli.port);

    tracing::info!("UL Forge API listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");

    axum::serve(listener, app).await.expect("server error");
}
