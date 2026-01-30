//! Node.js FFI bridge for TypeScript plugin compatibility
//!
//! This module provides Node.js bindings for the Rust core,
//! allowing TypeScript plugins to interact with the Rust implementation.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use openclaw_core::types::{Message, MessageId, SessionId};

/// Initialize the FFI bridge
#[napi]
pub fn init() -> Result<String> {
    Ok("OpenClaw Rust FFI initialized".to_string())
}

/// Get version information
#[napi]
pub fn get_version() -> Result<String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

/// Create a new message ID
#[napi]
pub fn create_message_id() -> Result<String> {
    Ok(MessageId::new().to_string())
}

/// Create a new session ID
#[napi]
pub fn create_session_id() -> Result<String> {
    Ok(SessionId::new().to_string())
}

/// Message wrapper for FFI
#[napi(object)]
pub struct JsMessage {
    pub id: String,
    pub session_id: String,
    pub content: String,
    pub timestamp: String,
}

/// Create a new message
#[napi]
pub fn create_message(session_id: String, content: String) -> Result<JsMessage> {
    let sid = SessionId::from_string(&session_id)
        .map_err(|e| Error::from_reason(format!("Invalid session ID: {}", e)))?;

    let message = Message::new(sid, content);

    Ok(JsMessage {
        id: message.id.to_string(),
        session_id: message.session_id.to_string(),
        content: message.content,
        timestamp: message.timestamp.to_rfc3339(),
    })
}

/// Configuration wrapper for FFI
#[napi(object)]
pub struct JsConfig {
    pub gateway_host: String,
    pub gateway_port: u16,
    pub gateway_mode: String,
}

/// Load configuration
#[napi]
pub fn load_config() -> Result<JsConfig> {
    let config = openclaw_core::config::Config::default();

    Ok(JsConfig {
        gateway_host: config.gateway.host,
        gateway_port: config.gateway.port,
        gateway_mode: config.gateway.mode,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = get_version().unwrap();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_create_ids() {
        let msg_id = create_message_id().unwrap();
        let sess_id = create_session_id().unwrap();
        assert!(!msg_id.is_empty());
        assert!(!sess_id.is_empty());
    }
}
