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

# Run benchmarks and save results with version info
bench-save VERSION:
    #!/usr/bin/env bash
    set -euo pipefail

    COMMIT=$(git rev-parse --short HEAD)
    TIMESTAMP=$(date +%Y-%m-%d_%H:%M:%S)
    OUTPUT_FILE="benchmark_data/v{{VERSION}}.txt"

    echo "Running benchmarks for version {{VERSION}}..."
    echo "Commit: $COMMIT"
    echo "Timestamp: $TIMESTAMP"
    echo ""

    {
        echo "# Benchmark Results for v{{VERSION}}"
        echo "# Date: $TIMESTAMP"
        echo "# Commit: $COMMIT"
        echo ""
        cargo bench --bench sort_benchmark 2>&1
    } | tee "$OUTPUT_FILE"

    echo ""
    echo "✓ Results saved to $OUTPUT_FILE"

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

# Run pre-release checklist (automates as much as possible)
release-checklist:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "========================================="
    echo "  PRE-RELEASE CHECKLIST"
    echo "========================================="
    echo ""

    # Extract version from Cargo.toml
    VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    echo "📦 Version: $VERSION"
    echo ""

    # CODE QUALITY CHECKS
    echo "=== CODE QUALITY ==="

    echo "✓ Running tests..."
    if ! cargo test 2>&1 | tee /tmp/test_output.txt | tail -20; then
        echo "FAILED - see output above"
        exit 1
    fi
    if ! grep -q "test result: ok" /tmp/test_output.txt; then
        echo "FAILED - tests did not pass"
        exit 1
    fi
    echo "  → PASSED"
    echo ""

    echo "✓ Building release (checking warnings)..."
    if BUILD_OUTPUT=$(cargo build --release 2>&1); then
        if echo "$BUILD_OUTPUT" | grep -q "warning:"; then
            echo "  → WARNINGS FOUND:"
            echo "$BUILD_OUTPUT" | grep "warning:"
            exit 1
        else
            echo "  → PASSED"
            echo ""
        fi
    else
        echo "  → BUILD FAILED:"
        echo "$BUILD_OUTPUT"
        exit 1
    fi

    echo "✓ Running clippy..."
    if CLIPPY_OUTPUT=$(cargo clippy --all-targets --all-features 2>&1); then
        if echo "$CLIPPY_OUTPUT" | grep -q "warning:"; then
            echo "  → WARNINGS FOUND:"
            echo "$CLIPPY_OUTPUT" | grep "warning:"
            exit 1
        else
            echo "  → PASSED"
            echo ""
        fi
    else
        echo "  → CLIPPY FAILED:"
        echo "$CLIPPY_OUTPUT"
        exit 1
    fi

    echo "✓ Running examples..."
    for example in examples/*.rs; do
        name=$(basename "$example" .rs)
        if ! OUTPUT=$(cargo run --example "$name" --quiet 2>&1); then
            echo "  → FAILED: $name"
            echo "$OUTPUT"
            exit 1
        fi
    done
    echo "  → PASSED"
    echo ""

    echo "✓ Running Python tests..."
    if ! uv run --group dev pytest python/tests/ -v 2>&1 | tee /tmp/pytest_output.txt | tail -20; then
        echo "  → FAILED - see output above"
        exit 1
    fi
    echo "  → PASSED"
    echo ""

    echo ""
    echo "=== VERSION VERIFICATION ==="

    # Check CHANGELOG has the version and today's date
    TODAY=$(date +%Y-%m-%d)
    if grep -q "## \[$VERSION\] - $TODAY" CHANGELOG.md; then
        echo "✓ CHANGELOG.md has version $VERSION with today's date ($TODAY)"
    else
        echo "⚠ WARNING: CHANGELOG.md should have '## [$VERSION] - $TODAY'"
        echo "  Current CHANGELOG dates:"
        grep "^## \[" CHANGELOG.md | head -3
    fi

    echo ""
    echo "=== GIT STATUS ==="

    # Check for untracked files (excluding known directories)
    UNTRACKED=$(git status --porcelain | grep '^??' | grep -v 'target/' | grep -v '.pyc' | grep -v '__pycache__' || true)
    if [ -n "$UNTRACKED" ]; then
        echo "⚠ WARNING: Untracked files found:"
        echo "$UNTRACKED"
    else
        echo "✓ No unexpected untracked files"
    fi

    # Check for uncommitted changes
    MODIFIED=$(git status --porcelain | grep '^[MADRC]' || true)
    if [ -n "$MODIFIED" ]; then
        echo "⚠ WARNING: Uncommitted changes:"
        echo "$MODIFIED"
    else
        echo "✓ No uncommitted changes"
    fi

    echo ""
    echo "========================================="
    echo "  MANUAL REVIEW REQUIRED"
    echo "========================================="
    echo ""
    echo "Please manually verify:"
    echo ""
    echo "  [ ] Review 'git diff' - all changes intentional?"
    echo "  [ ] CHANGELOG.md has ALL significant changes documented?"
    echo "  [ ] README.md updated if API changed?"
    echo "  [ ] No performance regressions (check benchmark output)?"
    echo "  [ ] No temporary debugging code left in?"
    echo "  [ ] Commits squashed and cleaned up?"
    echo "  [ ] No blocking TODO/FIXME comments?"
    echo "  [ ] Breaking changes documented (if any)?"
    echo "  [ ] Tag message prepared?"
    echo ""
    echo "If all checks pass, run:"
    echo "  just bench-save $VERSION    # Save benchmark results for this version"
    echo "  git add benchmark_data/v$VERSION.txt"
    echo "  git commit -m 'Add benchmark results for v$VERSION'"
    echo "  git tag -a v$VERSION -m 'Release v$VERSION'"
    echo "  git push origin main --tags"
    echo ""
