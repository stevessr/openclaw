/// Message serialization and deserialization
use crate::types::Message;
use crate::Result;
use serde::{Deserialize, Serialize};

/// Message envelope for wire protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope {
    pub version: u8,
    pub message_type: String,
    pub payload: serde_json::Value,
    pub timestamp: String,
}

impl MessageEnvelope {
    pub fn new(message_type: String, payload: serde_json::Value) -> Self {
        Self {
            version: 1,
            message_type,
            payload,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn from_message(message: &Message) -> Self {
        Self::new(
            "message".to_string(),
            serde_json::json!({
                "id": message.id.to_string(),
                "session_id": message.session_id.to_string(),
                "content": message.content,
                "timestamp": message.timestamp.to_rfc3339(),
            }),
        )
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

/// Serialize message to bytes
pub fn serialize_message(message: &Message) -> Result<Vec<u8>> {
    let envelope = MessageEnvelope::from_message(message);
    let json = envelope.to_json()?;
    Ok(json.into_bytes())
}

/// Deserialize message from bytes
pub fn deserialize_message(bytes: &[u8]) -> Result<MessageEnvelope> {
    let json = std::str::from_utf8(bytes)
        .map_err(|e| crate::Error::Unknown(format!("Invalid UTF-8: {}", e)))?;
    MessageEnvelope::from_json(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Message, SessionId};

    #[test]
    fn test_message_envelope_creation() {
        let envelope = MessageEnvelope::new("test".to_string(), serde_json::json!({"key": "value"}));
        assert_eq!(envelope.version, 1);
        assert_eq!(envelope.message_type, "test");
    }

    #[test]
    fn test_message_serialization() {
        let session_id = SessionId::new();
        let message = Message::new(session_id, "test content".to_string());

        let bytes = serialize_message(&message).unwrap();
        let envelope = deserialize_message(&bytes).unwrap();

        assert_eq!(envelope.message_type, "message");
        assert_eq!(envelope.version, 1);
    }

    #[test]
    fn test_envelope_roundtrip() {
        let original = MessageEnvelope::new("test".to_string(), serde_json::json!({"data": 123}));

        let json = original.to_json().unwrap();
        let parsed = MessageEnvelope::from_json(&json).unwrap();

        assert_eq!(original.version, parsed.version);
        assert_eq!(original.message_type, parsed.message_type);
    }
}
