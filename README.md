# ext-php-copilot

`ext-php-copilot` is a PHP 8.3+ extension, written in Rust with `ext-php-rs`, for driving GitHub Copilot through the Rust `github-copilot-sdk`.

The extension embeds the GitHub Copilot CLI when built with `COPILOT_CLI_VERSION`, while still supporting `COPILOT_CLI_PATH` and PATH resolution for local development.

## Requirements

- PHP 8.3 or newer, NTS recommended for the initial release, with development headers and `php-config` available.
- Rust 1.94+; nightly is configured because Windows extension builds require `abi_vectorcall`.
- GitHub Copilot CLI in `COPILOT_CLI_PATH` or PATH for dev builds, unless `COPILOT_CLI_VERSION` is set at build time.

## Build

```sh
cargo build
```

Build with an embedded CLI:

```sh
COPILOT_CLI_VERSION=1.0.48 cargo build --release
```

Run a PHP script with the debug extension on macOS:

```sh
php -d extension=target/debug/libext_php_copilot.dylib examples/basic.php
```

On Linux the extension suffix is `.so`; on Windows it is `.dll`.

## Acceptance Test

Create a local `.env` file with a Copilot-enabled token. The file is ignored by Git.

```dotenv
GITHUB_COPILOT_TOKEN=your_token_here
```

Then run the live acceptance test:

```sh
cargo build
php -d extension=target/debug/libext_php_copilot.dylib tests/acceptance.php
```

The script loads `.env`, verifies authentication, sends one prompt, and stores local Copilot CLI state under `var/copilot-acceptance`.

## Usage Documentation

See [docs/README.md](docs/README.md) for PHP wrapper usage, native extension methods, every supported option, and examples for each method.

## Platform Support

The Copilot SDK supports embedded CLI targets for macOS arm64/x64, Linux arm64/x64, and Windows arm64/x64. PHP extensions are ABI-specific, so release artifacts must be built per OS, architecture, PHP 8.3 patch version, ZTS/NTS mode, and debug/non-debug mode.

`cargo-php` is useful for install/stub workflows on macOS and Linux. Windows support should build through Cargo directly with PHP 8.3 development headers and `rust-lld`.

## Release Artifacts

Version tags build downloadable release archives for Linux, macOS, and Windows. Each OS gets two variants:

- `no-cli`: requires `COPILOT_CLI_PATH` or a GitHub Copilot CLI available on `PATH`.
- `embedded-cli`: embeds the GitHub Copilot CLI selected by `COPILOT_CLI_VERSION` in the release workflow.

Available releases are published on GitHub:

- [v0.1.3](https://github.com/soderlind/ext-php-copilot/releases/tag/v0.1.3) latest, embeds GitHub Copilot CLI `1.0.48` in the `embedded-cli` archives.
- [v0.1.2](https://github.com/soderlind/ext-php-copilot/releases/tag/v0.1.2) adds downloadable release archives for each OS and variant.
- [v0.1.1](https://github.com/soderlind/ext-php-copilot/releases/tag/v0.1.1) includes the initial CI portability fixes.

## AI Contribution Attribution

Assisted-by: GitHub Copilot

## License

MIT License.

Copyright (c) 2026 Per Søderlind.
