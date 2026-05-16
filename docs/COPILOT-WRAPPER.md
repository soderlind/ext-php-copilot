# Copilot Wrapper API

Use `ExtPhpCopilot\Copilot` for normal PHP applications. It keeps credentials, JSON encoding, session cleanup, and safer defaults in PHP while the native extension stays generic.

Complex native inputs and outputs use JSON strings. The wrapper converts PHP arrays to JSON and decodes JSON responses for common app usage.

## Recommended App Integration

```php
<?php

declare(strict_types=1);

require __DIR__ . '/../vendor/autoload.php';

use ExtPhpCopilot\Copilot;

$copilot = Copilot::fromEnvironment(
    cwd: getcwd(),
    copilotHome: __DIR__ . '/../var/copilot'
);

try {
    $copilot->assertAuthenticated();
    $event = $copilot->ask('Explain this project in one paragraph.');
    var_dump($event);
} finally {
    $copilot->close();
}
```

By default, the wrapper reads `GITHUB_COPILOT_TOKEN`, disables logged-in CLI fallback, isolates CLI state with `copilotHome`, and uses `permissionPolicy: deny_all`.

## Authentication

The extension does not perform OAuth and does not grant Copilot access. It starts the GitHub Copilot CLI through the Rust SDK, and the CLI authenticates with GitHub.

Recommended web-app flow:

1. Store a Copilot-entitled GitHub token in an environment variable or secret manager.
2. Pass it as `githubToken` or use `ExtPhpCopilot\Copilot::fromEnvironment()`.
3. Set `useLoggedInUser` to `false` for webservers.
4. Set a writable, app-owned `copilotHome` outside the web root.
5. Call `authStatusJson()` or `assertAuthenticated()` during application startup or health checks.

Supported auth modes:

- Server token: one app-level token, best for internal/admin tools.
- Per-user token: the application stores and supplies each user's token; this package accepts the token but does not implement OAuth storage.
- CLI user: pre-authenticated Copilot CLI state; useful for local development, not recommended for production PHP-FPM or Apache workers.

Never expose `githubToken` in logs, browser responses, WordPress options, or application error pages.

## ExtPhpCopilot\CopilotConfig

`CopilotConfig` is an immutable configuration object for the wrapper.

Constructor properties:

- `githubToken`: GitHub token with Copilot access, or `null` when `useLoggedInUser` is explicitly enabled.
- `copilotHome`: writable directory for Copilot CLI state.
- `cwd`: working directory for the Copilot client.
- `useLoggedInUser`: enables logged-in CLI fallback. Defaults to `false`.
- `permissionPolicy`: default session permission policy. Defaults to `deny_all`.
- `model`: optional default model name.
- `timeoutSeconds`: default wait timeout for `ask()`. Defaults to `60`.
- `clientOptions`: extra native client options passed through to `Copilot\Client`.
- `sessionConfig`: extra native session config passed through to `createSession()`.

### `new CopilotConfig(...)`

Creates an explicit wrapper configuration.

```php
use ExtPhpCopilot\CopilotConfig;

$config = new CopilotConfig(
    githubToken: getenv('GITHUB_COPILOT_TOKEN') ?: null,
    copilotHome: __DIR__ . '/../var/copilot',
    cwd: __DIR__,
    permissionPolicy: 'deny_all',
    timeoutSeconds: 90,
    clientOptions: ['logLevel' => 'info'],
    sessionConfig: ['clientName' => 'my-php-app']
);
```

### `CopilotConfig::fromArray(array $config): CopilotConfig`

Creates a config from an app array. `token` aliases `githubToken`, and `home` aliases `copilotHome`.

```php
$config = CopilotConfig::fromArray([
    'token' => getenv('GITHUB_COPILOT_TOKEN'),
    'home' => __DIR__ . '/../var/copilot',
    'cwd' => __DIR__,
    'model' => 'gpt-5',
    'timeoutSeconds' => 60,
]);
```

### `CopilotConfig::fromEnvironment(?string $cwd = null, ?string $copilotHome = null): CopilotConfig`

Reads `GITHUB_COPILOT_TOKEN`, disables logged-in user fallback, and uses a supplied or temp `copilotHome`.

```php
$config = CopilotConfig::fromEnvironment(
    cwd: getcwd(),
    copilotHome: __DIR__ . '/../var/copilot'
);
```

### `CopilotConfig::forCliUser(?string $cwd = null, ?string $copilotHome = null): CopilotConfig`

Creates a config for a locally logged-in Copilot CLI user.

```php
$config = CopilotConfig::forCliUser(
    cwd: getcwd(),
    copilotHome: $_SERVER['HOME'] . '/.copilot'
);
```

## ExtPhpCopilot\Copilot

`Copilot` is the recommended high-level app wrapper.

### `new Copilot(CopilotConfig $config)`

Starts the native Copilot client with the supplied config.

```php
use ExtPhpCopilot\Copilot;
use ExtPhpCopilot\CopilotConfig;

$copilot = new Copilot(CopilotConfig::fromEnvironment(getcwd(), __DIR__ . '/../var/copilot'));
```

### `Copilot::fromConfig(array|CopilotConfig $config): Copilot`

Creates a wrapper from either a `CopilotConfig` instance or array config.

```php
$copilot = Copilot::fromConfig([
    'githubToken' => getenv('GITHUB_COPILOT_TOKEN'),
    'copilotHome' => __DIR__ . '/../var/copilot',
    'cwd' => __DIR__,
]);
```

### `Copilot::fromEnvironment(?string $cwd = null, ?string $copilotHome = null): Copilot`

Creates a wrapper from `GITHUB_COPILOT_TOKEN`.

```php
$copilot = Copilot::fromEnvironment(
    cwd: getcwd(),
    copilotHome: __DIR__ . '/../var/copilot'
);
```

### `Copilot::forCliUser(?string $cwd = null, ?string $copilotHome = null): Copilot`

Creates a wrapper that uses existing logged-in CLI state.

```php
$copilot = Copilot::forCliUser(cwd: getcwd());
```

### `$copilot->authStatus(): array`

Returns decoded authentication status.

```php
$status = $copilot->authStatus();
if (($status['isAuthenticated'] ?? false) !== true) {
    throw new RuntimeException('Copilot is not authenticated.');
}
```

### `$copilot->assertAuthenticated(): void`

Throws `ExtPhpCopilot\Exception\AuthenticationException` when Copilot auth is unavailable.

```php
$copilot->assertAuthenticated();
```

### `$copilot->createSession(array $sessionConfig = []): Copilot\Session`

Creates and stores a native session. A previous stored session is disconnected first.

```php
$session = $copilot->createSession([
    'model' => 'gpt-5',
    'permissionPolicy' => 'deny_all',
    'clientName' => 'my-php-app',
]);
```

### `$copilot->ask(string $prompt, array $messageOptions = [], array $sessionConfig = []): ?array`

Creates a session when needed, sends a prompt, waits for one response event, and returns the decoded event.

```php
$event = $copilot->ask(
    'Summarize this PHP file.',
    ['timeoutSeconds' => 90],
    ['model' => 'gpt-5']
);
```

### `$copilot->client(): Copilot\Client`

Returns the native client for lower-level calls.

```php
$models = json_decode($copilot->client()->modelsJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `$copilot->session(): ?Copilot\Session`

Returns the stored native session, if one has been created.

```php
$session = $copilot->session();
if ($session !== null) {
    echo $session->id();
}
```

### `$copilot->close(): void`

Disconnects the stored session and stops the native client.

```php
try {
    $event = $copilot->ask('Hello Copilot.');
} finally {
    $copilot->close();
}
```
