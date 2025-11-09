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

## [0.1.0] - 2025-01-08

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

[Unreleased]: https://github.com/evanjpw/tilesort/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/evanjpw/tilesort/releases/tag/v0.1.0
