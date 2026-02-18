# Contributing to Tauri Bridge

Thank you for your interest in contributing to Tauri Bridge! This document provides guidelines and information for contributors.

## Code of Conduct

Please be respectful and constructive in all interactions. We're building something together, and a welcoming environment helps everyone contribute their best work.

## How to Contribute

### Reporting Bugs

1. **Check existing issues** — Search the issue tracker to see if the bug has already been reported.
2. **Create a minimal reproduction** — If possible, create a minimal example that demonstrates the bug.
3. **Open an issue** — Use the bug report template and include:
   - A clear description of the bug
   - Steps to reproduce
   - Expected vs actual behavior
   - Rust version, OS, and Tauri version
   - Any relevant code snippets or error messages

### Suggesting Features

1. **Check existing issues** — Your idea may already be under discussion.
2. **Open an issue** — Describe the feature, its use case, and why it would be valuable.
3. **Be open to feedback** — Features often evolve through discussion.

### Submitting Pull Requests

1. **Fork the repository** and create your branch from `main`.
2. **Write tests** for any new functionality.
3. **Ensure tests pass** — Run `cargo test --features backend`.
4. **Format your code** — Run `cargo fmt`.
5. **Run clippy** — Run `cargo clippy --all-features`.
6. **Update documentation** if needed.
7. **Write a clear PR description** explaining the changes.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
# Clone your fork
git clone https://github.com/your-username/tauri-bridge.git
cd tauri-bridge

# Build the project
cargo build

# Run tests
cargo test

# Run tests with all features
cargo test --features backend
```

### Running Examples

```bash
# Basic usage example
cargo run --example basic_usage

# Complex types example (requires backend feature)
cargo run --example complex_types --features backend

# Async commands example
cargo run --example async_commands
```

### Project Structure

```
tauri-bridge/
├── src/
│   ├── lib.rs          # Main entry point, macro definition
│   ├── backend.rs      # Backend code generation
│   ├── client.rs       # WASM client code generation
│   ├── types.rs        # Type analysis utilities
│   └── tests.rs        # Unit tests
├── examples/           # Example code
├── tests/              # Integration tests
└── README.md
```

## Testing

We have several levels of testing:

### Unit Tests

Located in `src/tests.rs`, these test the code generation logic:

```bash
cargo test
```

### Integration Tests

Located in `tests/`, these test the macro in real usage scenarios:

```bash
cargo test --test macro_expansion
cargo test --test tauri_integration --features backend
```

### Writing Tests

When adding new functionality:

1. Add unit tests in `src/tests.rs` for code generation logic
2. Add integration tests in `tests/` for end-to-end behavior
3. Ensure tests cover both success and error cases

## Code Style

- Follow Rust conventions and idioms
- Use `rustfmt` for formatting
- Address all `clippy` warnings
- Write documentation for public APIs
- Use meaningful variable and function names

## Documentation

- Update the README if you change user-facing behavior
- Add doc comments (`///`) to public functions and types
- Include examples in doc comments where helpful

## Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Reference issues when relevant ("Fix #123")
- Keep the first line under 72 characters

## Release Process

Releases are managed by maintainers. The process is:

1. Update version in `Cargo.toml`
2. Update CHANGELOG (if present)
3. Create a git tag
4. Publish to crates.io

## Questions?

If you have questions about contributing, feel free to:

- Open a discussion or issue
- Ask in the PR if it's related to specific changes

Thank you for contributing! 🎉