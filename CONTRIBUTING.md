# Contributing to reqsh

## Getting started

1. Fork the repository
2. Clone your fork
3. Build the project:

```bash
cargo build
```

## Development

### Run tests

```bash
cargo test
```

### Lint

```bash
cargo clippy
```

### Format

```bash
cargo fmt
```

### Run

```bash
cargo run
```

## Pull request process

1. Make sure your code compiles without warnings
2. Run `cargo test` to confirm all tests pass
3. Run `cargo clippy` and `cargo fmt` before submitting
4. Keep changes focused and describe what they do

## Code style

- Follow standard Rust formatting (`cargo fmt`)
- Write unit tests for new functionality
- Keep functions small and focused on one thing
