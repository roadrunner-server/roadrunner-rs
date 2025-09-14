# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust implementation of RoadRunner, consisting of a workspace with three main crates:
- **core**: Main application entry point
- **sdk**: Worker process management using goridge-rs for IPC with PHP workers
- **plugins**: Plugin trait system for extensibility

## Key Architecture

### Worker Process Management
The SDK crate implements worker process management through:
- `WorkerProcess` (sdk/src/worker.rs:10): Manages PHP worker processes using pipes for communication
- `Payload` (sdk/src/payload.rs:13): Message structure for worker communication with codec, context, and body
- `WorkerState` (sdk/src/state.rs:14): Tracks worker state (currently stub implementation)
- Uses goridge-rs library for frame-based IPC protocol with protobuf codec support

### Communication Protocol
- Frame-based protocol with version, flags, options, and CRC validation
- Protobuf codec for message serialization
- Bidirectional pipe communication between Rust process and PHP workers
- Test worker available at tests/worker.php

## Development Commands

### Building and Checking
```bash
# Check compilation without building
cargo check

# Build all workspace crates
cargo build

# Build release version
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p sdk
cargo test -p core
cargo test -p plugins

# Run specific test
cargo test test_init_worker
```

### Code Quality
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run clippy linter
cargo clippy

# Run clippy with all targets
cargo clippy --all-targets
```

### Running
```bash
# Run the SDK binary
cargo run -p sdk

# Run the core binary
cargo run -p core
```

## Dependencies

The SDK crate depends on:
- goridge-rs (local path dependency at ../../../../spiral/goridge-rs)
- tokio (async runtime with full features)
- clap, structopt (CLI parsing)
- anyhow (error handling)
- chrono, log, env_logger (utilities)

## Testing Infrastructure

The SDK includes integration tests that:
- Spawn PHP worker processes from tests/worker.php
- Send/receive payloads through goridge protocol
- Handle stderr output from workers
- Measure performance metrics

Test PHP workers require composer dependencies installed in tests/ directory.