use clap::{Parser, Subcommand};
use openclaw_core::Result;
use tracing::info;

#[derive(Parser)]
#[command(name = "openclaw")]
#[command(about = "OpenClaw - Personal AI Assistant", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the gateway server
    Gateway {
        /// Gateway host
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Gateway port
        #[arg(long, default_value = "18789")]
        port: u16,

        /// Enable verbose logging
        #[arg(long, short = 'v')]
        verbose: bool,
    },

    /// Send a message
    Message {
        /// Message content
        #[arg(long)]
        message: String,

        /// Recipient (phone number or channel ID)
        #[arg(long)]
        to: Option<String>,

        /// Channel to use
        #[arg(long, default_value = "whatsapp")]
        channel: String,
    },

    /// Run the AI agent
    Agent {
        /// Message to the agent
        #[arg(long)]
        message: String,

        /// Thinking level (low, medium, high)
        #[arg(long, default_value = "medium")]
        thinking: String,

        /// Model provider (anthropic, openai, local)
        #[arg(long, default_value = "anthropic")]
        provider: String,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Manage channels
    Channels {
        #[command(subcommand)]
        action: ChannelAction,
    },

    /// Show version information
    Version,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Key to set
        key: String,
        /// Value to set
        value: String,
    },
    /// Get a configuration value
    Get {
        /// Key to get
        key: String,
    },
}

#[derive(Subcommand)]
enum ChannelAction {
    /// List all channels
    List,
    /// Show channel status
    Status {
        /// Channel name
        channel: String,
    },
    /// Enable a channel
    Enable {
        /// Channel name
        channel: String,
    },
    /// Disable a channel
    Disable {
        /// Channel name
        channel: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gateway { host, port, verbose } => {
            init_tracing(verbose);
            info!("Starting OpenClaw gateway on {}:{}", host, port);
            openclaw_gateway::run(host, port).await?;
        }
        Commands::Message {
            message,
            to,
            channel,
        } => {
            init_tracing(false);
            info!(
                "Sending message via {}: {} to {:?}",
                channel, message, to
            );
            println!("Message functionality not yet implemented in Rust");
            println!("Channel: {}", channel);
            println!("Message: {}", message);
            if let Some(recipient) = to {
                println!("To: {}", recipient);
            }
        }
        Commands::Agent {
            message,
            thinking,
            provider,
        } => {
            init_tracing(false);
            info!(
                "Agent message: {} (thinking: {}, provider: {})",
                message, thinking, provider
            );
            println!("Agent functionality not yet implemented in Rust");
            println!("Provider: {}", provider);
            println!("Message: {}", message);
            println!("Thinking: {}", thinking);
        }
        Commands::Config { action } => {
            init_tracing(false);
            match action {
                ConfigAction::Show => {
                    let config = openclaw_core::config::Config::default();
                    println!("Configuration:");
                    println!(
                        "  Gateway: {}:{}",
                        config.gateway.host, config.gateway.port
                    );
                    println!("  Provider: {}", config.models.provider);
                    println!("  Model: {}", config.models.model);
                }
                ConfigAction::Set { key, value } => {
                    println!("Setting {} = {}", key, value);
                    println!("(Configuration persistence not yet implemented)");
                }
                ConfigAction::Get { key } => {
                    println!("Getting value for key: {}", key);
                    println!("(Configuration persistence not yet implemented)");
                }
            }
        }
        Commands::Channels { action } => {
            init_tracing(false);
            match action {
                ChannelAction::List => {
                    println!("Available channels:");
                    println!("  - whatsapp");
                    println!("  - telegram");
                    println!("  - discord");
                    println!("  - slack");
                    println!("  - signal");
                    println!("  - web");
                }
                ChannelAction::Status { channel } => {
                    println!("Status for channel '{}': Not connected", channel);
                }
                ChannelAction::Enable { channel } => {
                    println!("Enabling channel '{}'", channel);
                    println!("(Channel management not yet implemented)");
                }
                ChannelAction::Disable { channel } => {
                    println!("Disabling channel '{}'", channel);
                    println!("(Channel management not yet implemented)");
                }
            }
        }
        Commands::Version => {
            println!("openclaw {}", env!("CARGO_PKG_VERSION"));
            println!("Rust implementation (Phases 1-6 complete)");
        }
    }

    Ok(())
}

fn init_tracing(verbose: bool) {
    let level = if verbose { tracing::Level::DEBUG } else { tracing::Level::INFO };

    tracing_subscriber::fmt().with_max_level(level).with_target(false).init();
}
