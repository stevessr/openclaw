/// Channel abstraction for messaging platforms
use crate::{types::Message, Error, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Channel identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    WhatsApp,
    Telegram,
    Discord,
    Slack,
    Signal,
    Web,
    Custom(String),
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelType::WhatsApp => write!(f, "whatsapp"),
            ChannelType::Telegram => write!(f, "telegram"),
            ChannelType::Discord => write!(f, "discord"),
            ChannelType::Slack => write!(f, "slack"),
            ChannelType::Signal => write!(f, "signal"),
            ChannelType::Web => write!(f, "web"),
            ChannelType::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Channel message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMessage {
    pub id: String,
    pub channel: ChannelType,
    pub from: String,
    pub to: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: serde_json::Value,
}

/// Channel status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelStatus {
    Connected,
    Connecting,
    Disconnected,
    Error(String),
}

/// Channel trait for messaging platform integration
#[async_trait]
pub trait Channel: Send + Sync {
    /// Get channel type
    fn channel_type(&self) -> ChannelType;

    /// Initialize the channel
    async fn initialize(&mut self) -> Result<()>;

    /// Send a message
    async fn send_message(&self, to: &str, content: &str) -> Result<String>;

    /// Receive messages (polling or webhook-based)
    async fn receive_messages(&self) -> Result<Vec<ChannelMessage>>;

    /// Get channel status
    async fn status(&self) -> Result<ChannelStatus>;

    /// Disconnect from channel
    async fn disconnect(&mut self) -> Result<()>;
}

/// Channel registry for managing multiple channels
pub struct ChannelRegistry {
    channels: std::collections::HashMap<ChannelType, Box<dyn Channel>>,
}

impl ChannelRegistry {
    pub fn new() -> Self {
        Self {
            channels: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, channel: Box<dyn Channel>) {
        let channel_type = channel.channel_type();
        self.channels.insert(channel_type, channel);
    }

    pub fn get(&self, channel_type: &ChannelType) -> Option<&Box<dyn Channel>> {
        self.channels.get(channel_type)
    }

    pub fn list(&self) -> Vec<ChannelType> {
        self.channels.keys().cloned().collect()
    }
}

impl Default for ChannelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock channel for testing
pub struct MockChannel {
    channel_type: ChannelType,
    status: ChannelStatus,
}

impl MockChannel {
    pub fn new(channel_type: ChannelType) -> Self {
        Self {
            channel_type,
            status: ChannelStatus::Disconnected,
        }
    }
}

#[async_trait]
impl Channel for MockChannel {
    fn channel_type(&self) -> ChannelType {
        self.channel_type.clone()
    }

    async fn initialize(&mut self) -> Result<()> {
        self.status = ChannelStatus::Connected;
        Ok(())
    }

    async fn send_message(&self, _to: &str, _content: &str) -> Result<String> {
        Ok(crate::utils::generate_id())
    }

    async fn receive_messages(&self) -> Result<Vec<ChannelMessage>> {
        Ok(vec![])
    }

    async fn status(&self) -> Result<ChannelStatus> {
        Ok(self.status.clone())
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.status = ChannelStatus::Disconnected;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_type_display() {
        assert_eq!(ChannelType::WhatsApp.to_string(), "whatsapp");
        assert_eq!(ChannelType::Telegram.to_string(), "telegram");
        assert_eq!(ChannelType::Custom("test".to_string()).to_string(), "test");
    }

    #[tokio::test]
    async fn test_mock_channel() {
        let mut channel = MockChannel::new(ChannelType::WhatsApp);
        assert!(channel.initialize().await.is_ok());

        let message_id = channel.send_message("+1234567890", "test").await.unwrap();
        assert!(!message_id.is_empty());

        let status = channel.status().await.unwrap();
        matches!(status, ChannelStatus::Connected);
    }

    #[test]
    fn test_channel_registry() {
        let mut registry = ChannelRegistry::new();
        let channel = Box::new(MockChannel::new(ChannelType::WhatsApp));
        registry.register(channel);

        assert_eq!(registry.list().len(), 1);
        assert!(registry.get(&ChannelType::WhatsApp).is_some());
    }
}
