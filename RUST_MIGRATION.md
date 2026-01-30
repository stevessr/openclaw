# Rust Migration

This document outlines the migration of OpenClaw from TypeScript to Rust.

## Status

**Current Phase:** Phase 1 - Foundation & Setup ✅

The Rust workspace has been established with the following crates:

- **openclaw-core**: Core types, error handling, and configuration
- **openclaw-gateway**: Gateway server implementation
- **openclaw-cli**: Command-line interface

## Architecture

### Workspace Structure

```
openclaw/
├── Cargo.toml                 # Workspace configuration
└── crates/
    ├── openclaw-core/         # Core library
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── config.rs      # Configuration management
    │       ├── error.rs       # Error types
    │       └── types.rs       # Core data types
    ├── openclaw-gateway/      # Gateway server
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs         # HTTP/WebSocket server
    └── openclaw-cli/          # CLI binary
        ├── Cargo.toml
        └── src/
            └── main.rs        # CLI commands
```

## Building

### Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)

### Build Commands

```bash
# Build all crates
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Run specific crate
cargo run --bin openclaw -- --help
```

## Usage

The Rust CLI provides the same interface as the TypeScript version:

```bash
# Show help
./target/release/openclaw --help

# Start gateway server
./target/release/openclaw gateway --host 127.0.0.1 --port 18789 --verbose

# Send a message
./target/release/openclaw message --message "Hello" --to "+1234567890"

# Run agent
./target/release/openclaw agent --message "Hello" --thinking high
```

## Migration Strategy

### Incremental Migration

The Rust codebase is designed to coexist with the TypeScript codebase during migration:

1. **Core library** (openclaw-core): Migrated first, provides types and utilities
2. **Gateway server** (openclaw-gateway): HTTP/WebSocket server with basic routing
3. **CLI** (openclaw-cli): Command-line interface wrapping both implementations

### Interoperability

During the transition phase:

- Both TypeScript and Rust implementations will be available
- Rust binary can be used for performance-critical operations
- TypeScript implementation remains the default until Rust migration is complete

### Phase Roadmap

- [x] **Phase 1**: Foundation & Setup
  - [x] Workspace structure
  - [x] Core types and error handling
  - [x] Configuration management
  - [x] Basic CLI framework
  - [x] Gateway server skeleton

- [ ] **Phase 2**: Core Library Migration
  - [ ] Message serialization/deserialization
  - [ ] Session management
  - [ ] Logging infrastructure
  - [ ] Utility functions

- [ ] **Phase 3**: Gateway & Networking
  - [ ] WebSocket protocol implementation
  - [ ] HTTP API endpoints
  - [ ] Request routing
  - [ ] Connection management

- [ ] **Phase 4**: Messaging Channels
  - [ ] WhatsApp integration (Baileys protocol)
  - [ ] Telegram bot API
  - [ ] Discord integration
  - [ ] Slack integration
  - [ ] Other channels

- [ ] **Phase 5**: AI & Agent Logic
  - [ ] Model provider abstraction
  - [ ] Anthropic Claude integration
  - [ ] OpenAI integration
  - [ ] Agent runtime
  - [ ] Tool/skill execution

- [ ] **Phase 6**: CLI & Interface
  - [ ] All CLI commands
  - [ ] Terminal UI
  - [ ] Configuration wizard
  - [ ] Onboarding flow

- [ ] **Phase 7**: Testing & Validation
  - [ ] Unit tests
  - [ ] Integration tests
  - [ ] End-to-end tests
  - [ ] Performance benchmarks
  - [ ] Documentation

## Performance Benefits

Expected performance improvements from Rust:

- **Memory usage**: Lower overhead due to no garbage collection
- **Startup time**: Faster cold start compared to Node.js
- **CPU efficiency**: Better CPU utilization for compute-intensive tasks
- **Concurrency**: Native async/await with tokio runtime
- **Binary size**: Smaller, self-contained executable

## Development

### Adding a New Crate

```bash
# Create a new crate in the workspace
cargo new --lib crates/openclaw-newfeature

# Add to workspace in root Cargo.toml
# [workspace]
# members = ["crates/openclaw-newfeature", ...]
```

### Code Style

- Follow Rust standard naming conventions
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Write documentation comments for public APIs
- Include unit tests in the same file
- Add integration tests in `tests/` directory

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in specific crate
cargo test -p openclaw-core
```

## Contributing

When contributing to the Rust migration:

1. Follow the phase roadmap
2. Ensure existing TypeScript functionality continues to work
3. Add tests for new Rust code
4. Update this documentation
5. Keep the Rust implementation compatible with existing APIs

## Migration Progress

Track progress on GitHub: [Rust Migration Project](https://github.com/openclaw/openclaw/projects/rust-migration)

## Questions?

- Join the [Discord](https://discord.gg/clawd)
- Open an issue on [GitHub](https://github.com/openclaw/openclaw/issues)
- Check the [documentation](https://docs.openclaw.ai)
