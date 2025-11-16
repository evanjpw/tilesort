# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [0.2.0] - 2025-11-16

### Fixed
- Critical correctness bug where overlapping tiles were not sorted correctly. The algorithm now properly handles tiles
  with overlapping value ranges by using inclusive comparisons (`<=`/`>=`) instead of strict comparisons (`<`/`>`).
- Stack overflow when sorting data with heavily overlapping tiles. Converted recursive tile insertion to iterative
  approach using work queue, eliminating stack depth limits.

### Added
- Test coverage for overlapping tile scenarios to prevent regression
- Zero-cost statistics tracking (min/max/median tile size) during scan phase for heuristic optimization

### Changed
- Benchmark suite: Fixed `realistic_workload` to generate truly realistic data instead of accidentally pathological
  cases. Performance on realistic workloads shows ~10x improvement over std::sort (702µs vs 6.9ms for 1M elements).
  Added separate pathological overlap benchmarks.
- Tile insertion algorithm now uses iterative approach instead of recursion, improving robustness for pathological data

---

## [0.1.0] - 2025-11-08

### Added
- Initial implementation of tilesort algorithm for Rust
- Support for custom key extraction functions (`tilesort_by_key`, etc.)
- Reverse sorting support
- Both in-place (`tilesort`) and copying (`tilesorted`) variants
- Python bindings via PyO3
  - `tilesort.sort()` - in-place sorting
  - `tilesort.sorted()` - returns sorted copy
  - Support for `key` and `reverse` parameters
- Type hints with `.pyi` stub files
- Rust & Python test suites
- Benchmark suite comparing against std::sort across multiple scenarios
- GitHub Actions CI/CD for Rust and Python tests

### Changed

### Fixed

---

## Release History

[Unreleased]: https://github.com/evanjpw/tilesort/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/evanjpw/tilesort/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/evanjpw/tilesort/releases/tag/v0.1.0
