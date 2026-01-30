/// OpenClaw Gateway Server
///
/// This crate provides the gateway server implementation that handles
/// communication between channels, agents, and the AI backend.
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use openclaw_core::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Clone)]
struct AppState {
    // Gateway state will be added here
}

/// Run the gateway server
pub async fn run(host: String, port: u16) -> Result<()> {
    let state = Arc::new(AppState {});

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/status", get(status))
        .route("/message", post(send_message))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| Error::Network(format!("Failed to bind to {}: {}", addr, e)))?;

    info!("Gateway listening on {}", addr);
    axum::serve(listener, app).await.map_err(|e| Error::Network(format!("Server error: {}", e)))?;

    Ok(())
}

async fn root() -> &'static str {
    "OpenClaw Gateway (Rust)"
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

async fn status(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "gateway": "running",
            "version": env!("CARGO_PKG_VERSION"),
            "implementation": "rust"
        })),
    )
}

#[derive(Deserialize)]
struct MessageRequest {
    to: String,
    message: String,
}

#[derive(Serialize)]
struct MessageResponse {
    success: bool,
    message: String,
}

async fn send_message(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<MessageRequest>,
) -> impl IntoResponse {
    info!("Sending message to {}: {}", payload.to, payload.message);

    // TODO: Implement actual message sending
    (
        StatusCode::OK,
        Json(MessageResponse {
            success: true,
            message: format!("Message queued for {}", payload.to),
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_endpoint() {
        let _response = health().await;
        // Basic test to ensure health endpoint compiles and returns successfully
    }
}
