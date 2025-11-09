# Tilesort

[![Crates.io](https://img.shields.io/crates/v/tilesort.svg)](https://crates.io/crates/tilesort)
[![PyPI](https://img.shields.io/pypi/v/tilesort.svg)](https://pypi.org/project/tilesort/)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/tilesort)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Python Versions](https://img.shields.io/pypi/pyversions/tilesort.svg)](https://pypi.org/project/tilesort/)
[![Python Version from PEP 621 TOML](https://img.shields.io/python/required-version-toml?tomlFilePath=https%3A%2F%2Fraw.githubusercontent.com%2Fevanjpw%2Ftilesort%2Fmain%2Fpyproject.toml)](pyproject.toml)
[![Crates.io MSRV](https://img.shields.io/crates/msrv/tilesort)](Cargo.toml)
[![Rust Tests](https://github.com/evanjpw/tilesort/actions/workflows/rust-tests.yml/badge.svg)](https://github.com/evanjpw/tilesort/actions/workflows/rust-tests.yml)
[![Python Tests](https://github.com/evanjpw/tilesort/actions/workflows/python-tests.yml/badge.svg)](https://github.com/evanjpw/tilesort/actions/workflows/python-tests.yml)
[![Downloads](https://pepy.tech/badge/tilesort)](https://pepy.tech/project/tilesort)

A sorting algorithm optimized for datasets with pre-sorted contiguous blocks (tiles).

## Overview

**Tilesort** is a specialized sorting algorithm that achieves high performance when your data consists of
non-overlapping, pre-sorted contiguous blocks. Instead of sorting individual elements, tilesort identifies these "tiles"
and arranges them as discrete units.

### When to Use Tilesort

Tilesort is particularly effective when:
- Data arrives in pre-sorted chunks or batches
- Data structures maintain sorted regions
- Distributed systems produce sorted shards that need merging
- Merging sorted log files or event streams
- Processing time-series data with sorted segments
- You have *k* tiles in a dataset of *n* elements where *k* << *n*

### When NOT to Use Tilesort

**Do not use tilesort if:**
- **Your data is randomly shuffled** - Tilesort has O(n²) worst-case complexity when there are no pre-sorted tiles.
  Use standard sort algorithms instead.
- **You don't know if your data has tiles** - If your data doesn't have pre-sorted regions, tilesort will be
  significantly slower than standard sorting.
- **The number of tiles approaches the number of elements (k ≈ n)** - The overhead of tile detection and management
  provides no benefit when most elements are in their own tile.

Tilesort is a specialized algorithm for a specific data pattern. If you're unsure whether your data has pre-sorted
tiles, use a standard sorting algorithm.

### Performance

For a dataset of *n* elements partitioned into *k* tiles:
- **Time Complexity**: O(n + k²) where k << n
- **Space Complexity**: O(n) for the output buffer
- **Best Case**: O(n) when data is already sorted (k = 1)
- **Typical Case**: Significantly faster than O(n log n) when k is small

The performance is primarily determined by *k* (number of tiles) rather than *n* (total elements), making it highly
efficient when the number of tiles is much smaller than the total number of elements.

## Installation

### Python

```bash
pip install tilesort
```

**Requirements**: Python 3.8-3.14

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
tilesort = "0.1.0"
```

## Usage

### Python

The Python API mirrors Python's built-in `list.sort()` and `sorted()` functions:

```python
import tilesort

# Sort a list in place (like list.sort())
data = [3, 4, 5, 1, 2, 6, 7, 8]
tilesort.sort(data)
print(data)  # [1, 2, 3, 4, 5, 6, 7, 8]

# Return a sorted copy (like sorted())
data = [3, 4, 5, 1, 2, 6, 7, 8]
sorted_data = tilesort.sorted(data)
print(sorted_data)  # [1, 2, 3, 4, 5, 6, 7, 8]
print(data)         # [3, 4, 5, 1, 2, 6, 7, 8] (unchanged)

# Sort with a key function
words = ["elephant", "cat", "dog", "a", "bear"]
tilesort.sort(words, key=len)
print(words)  # ["a", "cat", "dog", "bear", "elephant"]

# Sort in reverse order
numbers = [3, 1, 4, 1, 5, 9, 2, 6]
tilesort.sort(numbers, reverse=True)
print(numbers)  # [9, 6, 5, 4, 3, 2, 1, 1]

# Combine key and reverse
data = [-5, -3, -1, 2, 4]
tilesort.sort(data, key=abs, reverse=True)
print(data)  # [-5, 4, -3, 2, -1]

# Sort custom objects
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age

people = [Person("Alice", 30), Person("Bob", 25), Person("Charlie", 35)]
tilesort.sort(people, key=lambda p: p.age)
# Now sorted by age: Bob (25), Alice (30), Charlie (35)
```

### Rust

```rust
use tilesort::{tilesort, tilesorted, tilesort_by_key, tilesort_reverse};

fn main() {
    // Sort in place
    let mut data = vec![3, 4, 5, 1, 2, 6, 7, 8];
    tilesort(&mut data);
    println!("{:?}", data);  // [1, 2, 3, 4, 5, 6, 7, 8]

    // Return a sorted copy
    let data = vec![3, 4, 5, 1, 2, 6, 7, 8];
    let sorted = tilesorted(&data);
    println!("{:?}", sorted);  // [1, 2, 3, 4, 5, 6, 7, 8]
    println!("{:?}", data);    // [3, 4, 5, 1, 2, 6, 7, 8] (unchanged)

    // Sort in reverse
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    tilesort_reverse(&mut data);
    println!("{:?}", data);  // [9, 6, 5, 4, 3, 2, 1, 1]

    // Sort with a key function
    let mut data = vec![-5i32, -3, -1, 2, 4];
    tilesort_by_key(&mut data, |&x| x.abs());
    println!("{:?}", data);  // [-1, 2, -3, 4, -5]

    // Sort strings by length
    let mut words = vec!["elephant", "cat", "dog", "a", "bear"];
    tilesort_by_key(&mut words, |s| s.len());
    println!("{:?}", words);  // ["a", "cat", "dog", "bear", "elephant"]

    // Sort custom structs
    #[derive(Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];

    tilesort_by_key(&mut people, |p| p.age);
    // Now sorted by age: Bob (25), Alice (30), Charlie (35)
}
```

## How It Works

Tilesort operates in two phases:

1. **Scan Phase**: Identifies contiguous sorted blocks (tiles) in the input data
2. **Restructure Phase**: Rearranges tiles in sorted order to produce the final output

The algorithm automatically detects tile boundaries by scanning for order violations. When elements are out of order, a
new tile begins. The tiles are then sorted based on their key ranges and concatenated to produce the final sorted
sequence.

### Example

Given input: `[3, 4, 5, 1, 2, 6, 7, 8]`

1. Scan identifies three tiles:
   - Tile 0: `[3, 4, 5]` (range 3-5)
   - Tile 1: `[1, 2]` (range 1-2)
   - Tile 2: `[6, 7, 8]` (range 6-8)

2. Tiles are sorted by their ranges: Tile 1, Tile 0, Tile 2

3. Output: `[1, 2, 3, 4, 5, 6, 7, 8]`

## API Reference

### Python

- `tilesort.sort(list, *, key=None, reverse=False)` - Sort a list in place
- `tilesort.sorted(list, *, key=None, reverse=False)` - Return a sorted copy

Both functions support:
- `key`: Optional function to extract comparison key from each element
- `reverse`: If `True`, sort in descending order

### Rust

**In-place sorting:**
- `tilesort(data: &mut [T])` - Sort in ascending order
- `tilesort_reverse(data: &mut [T])` - Sort in descending order
- `tilesort_by_key(data: &mut [T], key_fn: F)` - Sort by custom key
- `tilesort_by_key_reverse(data: &mut [T], key_fn: F)` - Sort by custom key, descending

**Copying variants:**
- `tilesorted(data: &[T]) -> Vec<T>` - Return sorted copy
- `tilesorted_reverse(data: &[T]) -> Vec<T>` - Return sorted copy, descending
- `tilesorted_by_key(data: &[T], key_fn: F) -> Vec<T>` - Return sorted copy by key
- `tilesorted_by_key_reverse(data: &[T], key_fn: F) -> Vec<T>` - Return sorted copy by key, descending

All functions work with any type `T` that implements `Ord + Clone`. Key functions must return a type `K` that implements
`Ord`.

## Development

### Building from Source

#### Rust Library

```bash
# Run tests
cargo test

# Build the library
cargo build --release

# Generate documentation
cargo doc --open
```

#### Python Package

Requirements:
- Rust toolchain (1.71.1+)
- Python 3.8-3.14
- [uv](https://github.com/astral-sh/uv) (recommended) or maturin

```bash
# Install development dependencies
uv sync --group dev

# Build and install in development mode
maturin develop --features python

# Run Python tests
just test-python
# or: uv run --group dev pytest python/tests/

# Run type checking
just typecheck
# or: uv run --group dev mypy python/

# Run all tests (Rust + Python)
just test

# Run linter
just lint

# Format code
just format
```

### Development Commands (Just)

This project uses [Just](https://just.systems/) as a command runner:

```bash
just              # List all available commands
just test         # Run all tests (Rust + Python)
just test-rust    # Run Rust tests only
just test-python  # Run Python tests only
just typecheck    # Run mypy type checking
just lint         # Run ruff linter
just format       # Format code with ruff
just build        # Build Python package
just bench        # Run benchmarks
just check        # Run all checks (test + typecheck + lint)
just clean        # Clean build artifacts
```

### Benchmarks

Performance benchmarks compare tilesort against Rust's standard sort across different scenarios:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark group
cargo bench uniform_tiles
cargo bench varied_tiles
cargo bench hybrid_tiles
cargo bench random_data
cargo bench key_function
cargo bench realistic_workload
```

**Benchmark scenarios:**
- **uniform_tiles**: All tiles have the same size (~1K elements)
- **varied_tiles**: Tiles of different sizes (100, 1K, 5K, 10K)
- **hybrid_tiles**: Mix of single elements and large blocks
- **random_data**: Completely random (worst case for tilesort)
- **key_function**: Structured data requiring key extraction
- **realistic_workload**: 1M elements with ~10K element tiles (mirrors real-world usage)

Results are saved to `target/criterion/` with HTML reports.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Areas for Contribution

- Performance benchmarks and optimizations
- Additional language bindings
- Documentation improvements
- Bug reports and fixes

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes in each release.
