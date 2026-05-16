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
COPILOT_CLI_VERSION=1.0.15 cargo build --release
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

## PHP API

Complex inputs and outputs use JSON strings in the initial API. This avoids lossy PHP array conversion while the Copilot SDK is still in technical preview.

For normal PHP applications, prefer the Composer wrapper in `ExtPhpCopilot\Copilot`. It keeps credentials, JSON encoding, session cleanup, and safer defaults in PHP while the native extension stays generic.

```php
use ExtPhpCopilot\Copilot;

$copilot = Copilot::fromEnvironment(
    cwd: getcwd(),
    copilotHome: __DIR__ . '/../var/copilot'
);

$copilot->assertAuthenticated();

$event = $copilot->ask('Explain this project in one paragraph.');

$copilot->close();
```

By default, the wrapper reads `GITHUB_COPILOT_TOKEN`, disables logged-in CLI fallback, isolates CLI state with `copilotHome`, and uses `permissionPolicy: deny_all`.

Use CLI-user auth only for local development or CLI scripts:

```php
$copilot = Copilot::forCliUser(cwd: getcwd());
```

For direct native-extension usage, instantiate `Copilot\Client` yourself:

```php
use Copilot\Client;

$client = new Client(json_encode([
    'cwd' => getcwd(),
    'logLevel' => 'info',
], JSON_THROW_ON_ERROR));

$session = $client->createSession(json_encode([
    'model' => 'gpt-5',
    'streaming' => true,
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));

$event = json_decode(
    $session->sendAndWaitJson('Explain this project in one paragraph.'),
    true,
    512,
    JSON_THROW_ON_ERROR
);

$session->disconnect();
$client->stop();
```

### Client Options

Available via `new Copilot\Client($optionsJson)`:

- `programPath`: explicit Copilot CLI path.
- `cwd`: working directory.
- `env`: child-process environment map.
- `envRemove`: environment variable names to remove.
- `prefixArgs`: arguments before the CLI server flags.
- `extraArgs`: extra CLI flags after transport flags.
- `githubToken`: token passed through the SDK auth-token environment flow.
- `useLoggedInUser`: allow or disable logged-in user fallback.
- `logLevel`: `none`, `error`, `warning`, `info`, `debug`, `all`.
- `sessionIdleTimeoutSeconds`: CLI server session idle timeout.
- `copilotHome`: isolated Copilot state directory.
- `tcpConnectionToken`: token for TCP/external transport.
- `remote`: enable remote session support.
- `transport`: `{ "type": "stdio" }`, `{ "type": "tcp", "port": 0 }`, or `{ "type": "external", "host": "127.0.0.1", "port": 4141 }`.
- `telemetry`: `otlpEndpoint`, `filePath`, `exporterType`, `sourceName`, `captureContent`.

### Authentication

The native extension does not perform OAuth and does not grant Copilot access. It starts the GitHub Copilot CLI through the Rust SDK, and the CLI authenticates with GitHub.

Recommended web-app flow:

1. Store a Copilot-entitled GitHub token in an environment variable or secret manager.
2. Pass it as `githubToken` or use `ExtPhpCopilot\Copilot::fromEnvironment()`.
3. Set `useLoggedInUser` to `false` for webservers.
4. Set a writable, app-owned `copilotHome` outside the web root.
5. Call `authStatusJson()` or `assertAuthenticated()` during application startup/health checks.

Supported auth modes:

- Server token: one app-level token, best for internal/admin tools.
- Per-user token: the application stores and supplies each user's token; this wrapper accepts the token but does not implement OAuth storage.
- CLI user: pre-authenticated Copilot CLI state; useful for local development, not recommended for production PHP-FPM/Apache workers.

Never expose `githubToken` in logs, browser responses, WordPress options, or application error pages.

### Session Options

Available via `createSession($configJson)` and `resumeSession($sessionId, $configJson)` using the SDK's camelCase JSON shape:

- `sessionId`, `model`, `clientName`, `reasoningEffort`, `streaming`.
- `systemMessage`, `availableTools`, `excludedTools`, `mcpServers`.
- `enableConfigDiscovery`, `requestUserInput`, `requestPermission`, `requestElicitation`, `requestExitPlanMode`, `requestAutoModeSwitch`.
- `skillDirectories`, `instructionDirectories`, `disabledSkills`.
- `customAgents`, `defaultAgent`, `agent`.
- `infiniteSessions`, `provider`, `enableSessionTelemetry`.
- `configDir`, `workingDirectory`, `gitHubToken`, `includeSubAgentStreamingEvents`.
- `permissionPolicy`: extension-level helper, currently `deny_all` or `approve_all`.

### Message Options

Available via `send($prompt, $optionsJson)` and `sendAndWaitJson($prompt, $optionsJson)`:

- `mode`: `enqueue` or `immediate`.
- `timeoutSeconds` or `timeoutMs` for `sendAndWaitJson`.
- `attachments`: SDK attachment JSON array.
- `requestHeaders`, `traceparent`, `tracestate`.

## Platform Support

The Copilot SDK supports embedded CLI targets for macOS arm64/x64, Linux arm64/x64, and Windows arm64/x64. PHP extensions are ABI-specific, so release artifacts must be built per OS, architecture, PHP 8.3 patch version, ZTS/NTS mode, and debug/non-debug mode.

`cargo-php` is useful for install/stub workflows on macOS and Linux. Windows support should build through Cargo directly with PHP 8.3 development headers and `rust-lld`.

## AI Contribution Attribution

Assisted-by: GitHub Copilot
