/// WebSocket handler for real-time communication
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use openclaw_core::session::SessionStore;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

use crate::AppState;

pub type SharedSessionStore = Arc<RwLock<SessionStore>>;

/// Handle WebSocket upgrade
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.sessions))
}

/// Handle WebSocket connection
async fn handle_socket(socket: WebSocket, store: SharedSessionStore) {
    let (mut sender, mut receiver) = socket.split();

    info!("New WebSocket connection established");

    // Create a new session for this connection
    let session_id = openclaw_core::types::SessionId::new();
    {
        let mut store = store.write().await;
        store.create_session(session_id);
    }
    info!("Created session: {}", session_id);

    // Send welcome message
    if sender
        .send(Message::Text(format!(
            "{{\"type\":\"welcome\",\"session_id\":\"{}\"}}",
            session_id
        )))
        .await
        .is_err()
    {
        error!("Failed to send welcome message");
        return;
    }

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                debug!("Received text message: {}", text);

                // Parse and handle message
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                    let response = handle_message(parsed, session_id, &store).await;

                    if let Ok(response_text) = serde_json::to_string(&response) {
                        if sender.send(Message::Text(response_text)).await.is_err() {
                            error!("Failed to send response");
                            break;
                        }
                    }
                }
            }
            Ok(Message::Binary(data)) => {
                debug!("Received binary message: {} bytes", data.len());
                // Echo binary data for now
                if sender.send(Message::Binary(data)).await.is_err() {
                    break;
                }
            }
            Ok(Message::Ping(data)) => {
                if sender.send(Message::Pong(data)).await.is_err() {
                    break;
                }
            }
            Ok(Message::Pong(_)) => {
                // Ignore pong messages
            }
            Ok(Message::Close(_)) => {
                info!("Client closed connection");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    // Clean up session
    {
        let mut store = store.write().await;
        store.delete_session(&session_id);
    }
    info!("Cleaned up session: {}", session_id);
}

/// Handle incoming WebSocket message
async fn handle_message(
    msg: serde_json::Value,
    session_id: openclaw_core::types::SessionId,
    store: &SharedSessionStore,
) -> serde_json::Value {
    let msg_type = msg.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");

    match msg_type {
        "ping" => {
            serde_json::json!({
                "type": "pong",
                "timestamp": chrono::Utc::now().to_rfc3339()
            })
        }
        "message" => {
            let content = msg.get("content").and_then(|v| v.as_str()).unwrap_or("");

            // Update session
            {
                let mut store = store.write().await;
                if let Some(session) = store.get_session_mut(&session_id) {
                    let message = openclaw_core::types::Message::new(session_id, content.to_string());
                    session.add_message(&message);
                }
            }

            serde_json::json!({
                "type": "ack",
                "status": "received",
                "session_id": session_id.to_string()
            })
        }
        "get_session" => {
            let store = store.read().await;
            if let Some(session) = store.get_session(&session_id) {
                serde_json::json!({
                    "type": "session",
                    "session_id": session.id.to_string(),
                    "message_count": session.message_count,
                    "created_at": session.created_at.to_rfc3339(),
                    "updated_at": session.updated_at.to_rfc3339()
                })
            } else {
                serde_json::json!({
                    "type": "error",
                    "error": "Session not found"
                })
            }
        }
        _ => serde_json::json!({
            "type": "error",
            "error": format!("Unknown message type: {}", msg_type)
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_store_creation() {
        let store = Arc::new(RwLock::new(SessionStore::new()));
        assert!(store.try_read().is_ok());
    }
}
