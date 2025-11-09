# https://just.systems

export RUST_LOG := "debug"

# List available commands
default:
    @just --list

# Run all Rust tests
test-rust:
    cargo test

# Run all Rust tests with logging output
test-rust-log:
    cargo test -- --nocapture

# Run all Python tests (including type tests)
test-python:
    uv run --group dev pytest python/tests/ -v

# Run all tests (Rust + Python)
test: test-rust test-python

# Run mypy type checking
typecheck:
    uv run --group dev mypy python/

# Run ruff linter
lint:
    uv run --group dev ruff check .

# Run ruff formatter
format:
    uv run --group dev ruff format .

# Build Python package with maturin
build:
    maturin develop --features python

# Build release version
build-release:
    maturin build --release --features python

# Run benchmarks
bench:
    cargo bench

# Run all checks (tests + typecheck + lint)
check: test typecheck lint

# Clean build artifacts
clean:
    cargo clean
    rm -rf target/
    rm -rf python/tilesort/__pycache__
    rm -rf python/tests/__pycache__
    rm -rf .pytest_cache
    find . -type d -name "*.egg-info" -exec rm -rf {} +
    find . -type f -name "*.pyc" -delete
    find . -type f -name "*.so" -delete
