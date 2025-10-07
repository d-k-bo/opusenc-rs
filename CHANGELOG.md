# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- BREAKING: `pkg-config` is now required to build `opusenc-sys`
- Replace `cstr` with `cstr8`

## [0.2.2] - 2024-10-21

### Added

- Support libopusenc's pull/`get_page`-based API ([#3](https://github.com/d-k-bo/opusenc-rs/pull/3))

## [0.2.1] - 2024-01-21

### Changed

- `opusenc` now correctly depends on the latest version of `opusenc-sys`

## [0.2.0] - 2024-01-21  [YANKED]

### Added

- `opusenc-sys` now uses the `links` manifest key

### Changed

- Update dependencies
  - includes an update of the publicly exposed `num_enum` crate which could be considered a breaking change

## [0.1.0] - 2023-07-22

### Added

- Initial release

[Unreleased]: https://github.com/d-k-bo/opusenc-rs/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/d-k-bo/opusenc-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/d-k-bo/opusenc-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/d-k-bo/opusenc-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/d-k-bo/opusenc-rs/releases/tag/v0.1.0
