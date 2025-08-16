# Async First Steps

A simple Rust project demonstrating basic async/await concepts using Tokio.

## What this project demonstrates

- Basic async/await syntax in Rust
- Sequential vs parallel execution of async functions
- Future patterns similar to Dart
- Using Tokio runtime for async operations

## Prerequisites

- Rust (latest stable version)
- Cargo

## Running the project

```bash
cargo run
```

## Project structure

- `src/main.rs` - Contains examples of different async execution patterns
- `Cargo.toml` - Project dependencies (Tokio with full features)

## Examples included

1. **Sequential execution** - Waiting for one async function to complete before continuing
2. **Parallel execution** - Running multiple async functions simultaneously
3. **Future pattern** - Creating futures without immediate execution

## Dependencies

- `tokio` - Async runtime for Rust
