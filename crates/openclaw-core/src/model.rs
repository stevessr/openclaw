/// Model provider abstraction for AI backends
use crate::{Error, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Model provider type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelProvider {
    Anthropic,
    OpenAI,
    Local,
    Custom(String),
}

impl std::fmt::Display for ModelProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelProvider::Anthropic => write!(f, "anthropic"),
            ModelProvider::OpenAI => write!(f, "openai"),
            ModelProvider::Local => write!(f, "local"),
            ModelProvider::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: ModelProvider,
    pub model_name: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub api_key: Option<String>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            provider: ModelProvider::Anthropic,
            model_name: "claude-opus-4.5".to_string(),
            temperature: 0.7,
            max_tokens: 4096,
            api_key: None,
        }
    }
}

/// Chat message role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    User,
    Assistant,
    System,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

/// Model response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub content: String,
    pub model: String,
    pub tokens_used: usize,
    pub finish_reason: String,
}

/// Model provider trait
#[async_trait]
pub trait Model: Send + Sync {
    /// Get provider type
    fn provider(&self) -> ModelProvider;

    /// Generate completion
    async fn complete(&self, messages: Vec<ChatMessage>) -> Result<ModelResponse>;

    /// Stream completion (returns async stream)
    async fn stream_complete(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<String>>>;

    /// Get available models
    async fn list_models(&self) -> Result<Vec<String>>;
}

/// Mock model for testing
pub struct MockModel {
    provider: ModelProvider,
}

impl MockModel {
    pub fn new(provider: ModelProvider) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl Model for MockModel {
    fn provider(&self) -> ModelProvider {
        self.provider.clone()
    }

    async fn complete(&self, messages: Vec<ChatMessage>) -> Result<ModelResponse> {
        let last_message = messages.last().ok_or_else(|| Error::Unknown("No messages".to_string()))?;

        Ok(ModelResponse {
            content: format!("Echo: {}", last_message.content),
            model: "mock-model".to_string(),
            tokens_used: 10,
            finish_reason: "stop".to_string(),
        })
    }

    async fn stream_complete(
        &self,
        _messages: Vec<ChatMessage>,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<String>>> {
        let (tx, rx) = tokio::sync::mpsc::channel(10);
        tokio::spawn(async move {
            let _ = tx.send(Ok("Streaming ".to_string())).await;
            let _ = tx.send(Ok("response".to_string())).await;
        });
        Ok(rx)
    }

    async fn list_models(&self) -> Result<Vec<String>> {
        Ok(vec!["mock-model-1".to_string(), "mock-model-2".to_string()])
    }
}

/// Agent runtime for executing tasks
pub struct Agent {
    model: Box<dyn Model>,
    system_prompt: String,
}

impl Agent {
    pub fn new(model: Box<dyn Model>, system_prompt: String) -> Self {
        Self {
            model,
            system_prompt,
        }
    }

    pub async fn execute(&self, user_message: &str) -> Result<String> {
        let messages = vec![
            ChatMessage {
                role: Role::System,
                content: self.system_prompt.clone(),
            },
            ChatMessage {
                role: Role::User,
                content: user_message.to_string(),
            },
        ];

        let response = self.model.complete(messages).await?;
        Ok(response.content)
    }

    pub fn set_system_prompt(&mut self, prompt: String) {
        self.system_prompt = prompt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_provider_display() {
        assert_eq!(ModelProvider::Anthropic.to_string(), "anthropic");
        assert_eq!(ModelProvider::OpenAI.to_string(), "openai");
    }

    #[test]
    fn test_model_config_default() {
        let config = ModelConfig::default();
        assert_eq!(config.provider, ModelProvider::Anthropic);
        assert_eq!(config.temperature, 0.7);
    }

    #[tokio::test]
    async fn test_mock_model() {
        let model = MockModel::new(ModelProvider::Anthropic);
        let messages = vec![ChatMessage {
            role: Role::User,
            content: "Hello".to_string(),
        }];

        let response = model.complete(messages).await.unwrap();
        assert!(response.content.contains("Echo"));
    }

    #[tokio::test]
    async fn test_agent() {
        let model = Box::new(MockModel::new(ModelProvider::Anthropic));
        let agent = Agent::new(model, "You are a helpful assistant".to_string());

        let response = agent.execute("Hello").await.unwrap();
        assert!(!response.is_empty());
    }
}
