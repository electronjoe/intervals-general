# Changelog
All notable changes to intervals-general will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Optional serde support with feature flag (thanks @misaka10987)
- Re-export `Interval` type at crate root

### Changed
- Updated to Rust 2021 edition
- Updated dependencies:
  - itertools from 0.8.0 to 0.13.0
  - criterion from 0.2 to 0.5
  - quickcheck from 0.8 to 1.0
  - quickcheck_macros from 0.8 to 1.0
- Improved code organization with conditional compilation for serde support
- Enhanced code quality and readability in interval operations
- Improved test code organization and error handling

### Deprecated
- None

### Removed
- None

### Fixed
- Code formatting and style improvements
- More idiomatic pattern matching in complement operation
- Simplified boolean expressions in tests

### Security
- Updated GitHub Actions workflow to use latest action versions

## [0.1.0] - 2020-04-20
### Added
- Initial release
- Support for all real interval types (open, closed, half-open)
- Generic bound data types with PartialOrd trait requirement
- Core interval operations: Union, Intersection, Complement
- Left and Right Partial Compare operations
- No-std support
- Initial test coverage
- Type-enforced empty interval representation
- Type-enforced unbounded interval support

### Changed
- None (initial release)

### Deprecated
- None

### Removed
- None

### Fixed
- None

### Security
- None

[Unreleased]: https://github.com/electronjoe/intervals-general/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/electronjoe/intervals-general/releases/tag/v0.1.0
