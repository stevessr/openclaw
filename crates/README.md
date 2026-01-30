# OpenClaw Rust Crates

This directory contains the Rust implementation of OpenClaw.

## Crates

### openclaw-core

Core library providing:
- Type definitions (MessageId, SessionId, Message)
- Error types and result handling
- Configuration management
- Utility functions

### openclaw-gateway

Gateway server providing:
- HTTP/WebSocket server (using Axum)
- REST API endpoints
- WebSocket protocol handling
- Connection management

### openclaw-cli

Command-line interface providing:
- Gateway server management
- Message sending
- Agent interaction
- Configuration commands

## Quick Start

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run the CLI
cargo run --bin openclaw -- --help

# Build release version
cargo build --release
./target/release/openclaw --version
```

## Documentation

See [RUST_MIGRATION.md](../RUST_MIGRATION.md) for detailed migration information.
