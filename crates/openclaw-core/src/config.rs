/// Configuration management
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Gateway configuration
    #[serde(default)]
    pub gateway: GatewayConfig,

    /// Model configuration
    #[serde(default)]
    pub models: ModelConfig,

    /// Channel configurations
    #[serde(default)]
    pub channels: ChannelConfig,
}

/// Gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Gateway host
    pub host: String,

    /// Gateway port
    pub port: u16,

    /// Gateway mode (local, remote)
    pub mode: String,

    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 18789,
            mode: "local".to_string(),
            verbose: false,
        }
    }
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Primary model provider
    pub provider: String,

    /// Model name
    pub model: String,

    /// API key (optional, can use OAuth)
    pub api_key: Option<String>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            provider: "anthropic".to_string(),
            model: "claude-opus-4.5".to_string(),
            api_key: None,
        }
    }
}

/// Channel configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChannelConfig {
    /// WhatsApp enabled
    #[serde(default)]
    pub whatsapp: bool,

    /// Telegram enabled
    #[serde(default)]
    pub telegram: bool,

    /// Discord enabled
    #[serde(default)]
    pub discord: bool,

    /// Slack enabled
    #[serde(default)]
    pub slack: bool,
}

impl Config {
    /// Load configuration from a file
    pub fn from_file(path: &PathBuf) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn to_file(&self, path: &PathBuf) -> crate::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.gateway.port, 18789);
        assert_eq!(config.gateway.host, "127.0.0.1");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.gateway.port, config.gateway.port);
    }
}
