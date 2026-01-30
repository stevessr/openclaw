/// OpenClaw Gateway Server
///
/// This crate provides the gateway server implementation that handles
/// communication between channels, agents, and the AI backend.
mod websocket;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use openclaw_core::{session::SessionStore, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::info;
use websocket::ws_handler;

pub use websocket::SharedSessionStore;

#[derive(Clone)]
pub struct AppState {
    pub sessions: SharedSessionStore,
}

/// Run the gateway server
pub async fn run(host: String, port: u16) -> Result<()> {
    let sessions = Arc::new(RwLock::new(SessionStore::new()));
    let state = AppState {
        sessions: sessions.clone(),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/status", get(status))
        .route("/message", post(send_message))
        .route("/ws", get(ws_handler))
        .route("/sessions", get(list_sessions))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| Error::Network(format!("Failed to bind to {}: {}", addr, e)))?;

    info!("Gateway listening on {}", addr);
    info!("WebSocket endpoint: ws://{}/ws", addr);
    axum::serve(listener, app).await.map_err(|e| Error::Network(format!("Server error: {}", e)))?;

    Ok(())
}

async fn root() -> &'static str {
    "OpenClaw Gateway (Rust)"
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

async fn status(State(state): State<AppState>) -> impl IntoResponse {
    let sessions = state.sessions.read().await;
    let session_count = sessions.list_sessions().len();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "gateway": "running",
            "version": env!("CARGO_PKG_VERSION"),
            "implementation": "rust",
            "sessions": session_count
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
    State(state): State<AppState>,
    Json(payload): Json<MessageRequest>,
) -> impl IntoResponse {
    info!("Sending message to {}: {}", payload.to, payload.message);

    // Create a session and message
    let session_id = openclaw_core::types::SessionId::new();
    let message = openclaw_core::types::Message::new(session_id, payload.message.clone());

    {
        let mut sessions = state.sessions.write().await;
        let _session = sessions.create_session(session_id);
        sessions
            .get_session_mut(&session_id)
            .unwrap()
            .add_message(&message);
    }

    (
        StatusCode::OK,
        Json(MessageResponse {
            success: true,
            message: format!("Message queued for {} (session: {})", payload.to, session_id),
        }),
    )
}

async fn list_sessions(State(state): State<AppState>) -> impl IntoResponse {
    let sessions = state.sessions.read().await;
    let session_list: Vec<_> = sessions
        .list_sessions()
        .iter()
        .map(|s| {
            serde_json::json!({
                "id": s.id.to_string(),
                "message_count": s.message_count,
                "created_at": s.created_at.to_rfc3339(),
                "updated_at": s.updated_at.to_rfc3339()
            })
        })
        .collect();

    (StatusCode::OK, Json(serde_json::json!({ "sessions": session_list })))
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
