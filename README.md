# ZK Proof System

[![CI](https://github.com/amitduabits/zk-proof-system/workflows/CI/badge.svg)](https://github.com/amitduabits/zk-proof-system/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

A high-performance zero-knowledge proof system built with Halo2, providing efficient proof generation and verification capabilities.

## Features

- **Modular Architecture**: Separated into core, commitments, verifier, and bindings modules
- **Halo2 Integration**: Built on top of the battle-tested Halo2 proving system
- **Performance Optimized**: Leveraging Rust's zero-cost abstractions
- **FFI Bindings**: C/C++ and WebAssembly bindings for cross-language support
- **Comprehensive Testing**: Unit tests, integration tests, and benchmarks
- **CI/CD Pipeline**: Automated testing and deployment via GitHub Actions

## Quick Start

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Installation

```bash
# Clone the repository
git clone https://github.com/amitduabits/zk-proof-system.git
cd zk-proof-system

# Build the project
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Project Structure

```
zk-proof-system/
â”œâ”€â”€ core/               # Core functionality and abstractions
â”œâ”€â”€ commitments/        # Commitment schemes implementation
â”œâ”€â”€ verifier/          # Proof verification logic
â”œâ”€â”€ bindings/          # FFI and WASM bindings
â”œâ”€â”€ Cargo.toml         # Workspace configuration
â”œâ”€â”€ rustfmt.toml       # Code formatting rules
â”œâ”€â”€ .clippy.toml       # Linting configuration
â””â”€â”€ .github/           # CI/CD workflows
```

## License

This project is dual-licensed under either:

- Apache License, Version 2.0
- MIT license

at your option.
