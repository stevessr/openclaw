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
    },

    /// Run the AI agent
    Agent {
        /// Message to the agent
        #[arg(long)]
        message: String,

        /// Thinking level (low, medium, high)
        #[arg(long, default_value = "medium")]
        thinking: String,
    },

    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gateway {
            host,
            port,
            verbose,
        } => {
            init_tracing(verbose);
            info!("Starting OpenClaw gateway on {}:{}", host, port);
            openclaw_gateway::run(host, port).await?;
        }
        Commands::Message { message, to } => {
            init_tracing(false);
            info!("Sending message: {} to {:?}", message, to);
            println!("Message functionality not yet implemented in Rust");
            println!("Message: {}", message);
            if let Some(recipient) = to {
                println!("To: {}", recipient);
            }
        }
        Commands::Agent { message, thinking } => {
            init_tracing(false);
            info!("Agent message: {} (thinking: {})", message, thinking);
            println!("Agent functionality not yet implemented in Rust");
            println!("Message: {}", message);
            println!("Thinking: {}", thinking);
        }
        Commands::Version => {
            println!("openclaw {}", env!("CARGO_PKG_VERSION"));
            println!("Rust implementation (in progress)");
        }
    }

    Ok(())
}

fn init_tracing(verbose: bool) {
    let level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .init();
}
