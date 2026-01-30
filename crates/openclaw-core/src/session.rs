/// Session management
use crate::types::{Message, SessionId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub message_count: usize,
}

impl Session {
    pub fn new(id: SessionId) -> Self {
        let now = Utc::now();
        Self {
            id,
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
            message_count: 0,
        }
    }

    pub fn add_message(&mut self, _message: &Message) {
        self.message_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn set_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
        self.updated_at = Utc::now();
    }
}

/// Session store for managing multiple sessions
#[derive(Debug, Default)]
pub struct SessionStore {
    sessions: HashMap<SessionId, Session>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, id: SessionId) -> &Session {
        let session = Session::new(id);
        self.sessions.insert(id, session);
        self.sessions.get(&id).unwrap()
    }

    pub fn get_session(&self, id: &SessionId) -> Option<&Session> {
        self.sessions.get(id)
    }

    pub fn get_session_mut(&mut self, id: &SessionId) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    }

    pub fn list_sessions(&self) -> Vec<&Session> {
        self.sessions.values().collect()
    }

    pub fn delete_session(&mut self, id: &SessionId) -> bool {
        self.sessions.remove(id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let id = SessionId::new();
        let session = Session::new(id);
        assert_eq!(session.id, id);
        assert_eq!(session.message_count, 0);
    }

    #[test]
    fn test_session_store() {
        let mut store = SessionStore::new();
        let id = SessionId::new();

        store.create_session(id);
        assert!(store.get_session(&id).is_some());

        let deleted = store.delete_session(&id);
        assert!(deleted);
        assert!(store.get_session(&id).is_none());
    }

    #[test]
    fn test_session_metadata() {
        let mut session = Session::new(SessionId::new());
        session.set_metadata("key".to_string(), serde_json::json!("value"));
        assert_eq!(session.metadata.get("key").unwrap(), "value");
    }
}
