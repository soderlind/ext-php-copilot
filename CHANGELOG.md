# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 2026-05-16

### Added

- Added tag-driven release builds for Linux, macOS, and Windows with `no-cli` and `embedded-cli` archives.

### Fixed

- Made release packaging tolerate optional folders when publishing assets for older tags.

## [0.1.1] - 2026-05-16

### Fixed

- Replaced Rust test-binary linking in CI with `cargo check --all-targets --all-features` for PHP extension crates.
- Scoped the dynamic lookup linker flag to macOS so Linux builds use platform-appropriate linker flags.

## [0.1.0] - 2026-05-16

### Added

- Initial Rust PHP extension scaffold using `ext-php-rs`.
- GitHub Copilot SDK integration with embedded CLI support.
- Native PHP API with `Copilot\Client`, `Copilot\Session`, and `copilot_sdk_version()`.
- Generic Composer-friendly PHP wrapper with safer web-app defaults.
- Token, logged-in user, and isolated `copilotHome` authentication flows.
- macOS, Linux, and Windows build configuration.
- PHP stubs, examples, smoke test, unit tests, and live acceptance test.
- CI workflow for Rust formatting, build, test compilation, Clippy, PHP linting, unit tests, and Composer validation.
