/// Logging infrastructure
use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize logging with default configuration
pub fn init_logging() {
    init_logging_with_level("info")
}

/// Initialize logging with specific level
pub fn init_logging_with_level(level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(level))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(false).with_thread_ids(true))
        .init();
}

/// Initialize logging with JSON format
pub fn init_logging_json() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().json().with_target(true))
        .init();
}

/// Log level from string
pub fn level_from_str(s: &str) -> Level {
    match s.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_from_str() {
        assert_eq!(level_from_str("debug"), Level::DEBUG);
        assert_eq!(level_from_str("INFO"), Level::INFO);
        assert_eq!(level_from_str("warn"), Level::WARN);
        assert_eq!(level_from_str("unknown"), Level::INFO);
    }
}
