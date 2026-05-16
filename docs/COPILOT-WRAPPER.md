<!-- markdownlint-disable MD013 MD024 -->

# Wrapper API Reference

## Summary

The wrapper API provides a PHP-native facade over the `Copilot\Client` and `Copilot\Session` extension classes. It handles JSON conversion, configuration validation, authentication checks, session reuse, and cleanup.

Use this API for normal application code.

## Namespace

```php
ExtPhpCopilot
```

## Classes

| Class | Description |
| --- | --- |
| `ExtPhpCopilot\CopilotConfig` | Immutable configuration object for the wrapper. |
| `ExtPhpCopilot\Copilot` | High-level client wrapper for authentication, sessions, and prompt execution. |

## Authentication Model

The extension starts the GitHub Copilot CLI through the Rust SDK. The CLI authenticates with GitHub. This package does not perform OAuth and does not grant Copilot access.

| Mode | Use case | Notes |
| --- | --- | --- |
| Server token | Internal tools, admin screens, workers | Store `GITHUB_COPILOT_TOKEN` outside source control. |
| Per-user token | Apps that already manage user tokens | The package accepts the token but does not implement OAuth storage. |
| CLI user | Local development | Requires existing CLI login state; not recommended for PHP-FPM or Apache workers. |

For web apps, keep `useLoggedInUser` disabled, set an app-owned `copilotHome`, and avoid logging `githubToken`.

## Recommended Usage

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

## `ExtPhpCopilot\CopilotConfig`

### Description

Immutable configuration for `ExtPhpCopilot\Copilot`.

### Properties

| Property | Type | Default | Description |
| --- | --- | --- | --- |
| `githubToken` | `?string` | `null` | GitHub token with Copilot access. Required unless `useLoggedInUser` is explicitly enabled. |
| `copilotHome` | `?string` | temporary directory | Writable Copilot CLI state directory. |
| `cwd` | `?string` | current process directory | Working directory for the Copilot client. |
| `useLoggedInUser` | `bool` | `false` | Enables fallback to logged-in CLI state. |
| `permissionPolicy` | `string` | `deny_all` | Default session permission policy. |
| `model` | `?string` | `null` | Optional default model name. |
| `timeoutSeconds` | `int` | `60` | Default wait timeout for `ask()`. |
| `clientOptions` | `array` | `[]` | Extra native client options. |
| `sessionConfig` | `array` | `[]` | Extra native session configuration. |

### `__construct()`

```php
public function __construct(
    ?string $githubToken = null,
    ?string $copilotHome = null,
    ?string $cwd = null,
    bool $useLoggedInUser = false,
    string $permissionPolicy = 'deny_all',
    ?string $model = null,
    int $timeoutSeconds = 60,
    array $clientOptions = [],
    array $sessionConfig = []
)
```

Creates an explicit wrapper configuration.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$githubToken` | `?string` | GitHub token with Copilot access. |
| `$copilotHome` | `?string` | Copilot CLI state directory. |
| `$cwd` | `?string` | Working directory. |
| `$useLoggedInUser` | `bool` | Enables logged-in CLI fallback. |
| `$permissionPolicy` | `string` | Session permission policy. |
| `$model` | `?string` | Optional default model. |
| `$timeoutSeconds` | `int` | Default prompt wait timeout. |
| `$clientOptions` | `array` | Extra native client options. |
| `$sessionConfig` | `array` | Extra session config. |

#### Throws

| Exception | Condition |
| --- | --- |
| `ExtPhpCopilot\Exception\ConfigurationException` | Configuration is invalid or token/CLI-user auth is missing. |

#### Example

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

### `fromArray()`

```php
public static function fromArray(array $config): CopilotConfig
```

Creates configuration from an application array. `token` aliases `githubToken`; `home` aliases `copilotHome`.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$config` | `array` | Configuration map. |

#### Return Value

Returns a validated `CopilotConfig` instance.

#### Example

```php
$config = CopilotConfig::fromArray([
    'token' => getenv('GITHUB_COPILOT_TOKEN'),
    'home' => __DIR__ . '/../var/copilot',
    'cwd' => __DIR__,
    'model' => 'gpt-5',
    'timeoutSeconds' => 60,
]);
```

### `fromEnvironment()`

```php
public static function fromEnvironment(?string $cwd = null, ?string $copilotHome = null): CopilotConfig
```

Reads `GITHUB_COPILOT_TOKEN`, disables logged-in CLI fallback, and creates a token-based config.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$cwd` | `?string` | Working directory. |
| `$copilotHome` | `?string` | Copilot CLI state directory. |

#### Return Value

Returns a validated `CopilotConfig` instance.

#### Example

```php
$config = CopilotConfig::fromEnvironment(
    cwd: getcwd(),
    copilotHome: __DIR__ . '/../var/copilot'
);
```

### `forCliUser()`

```php
public static function forCliUser(?string $cwd = null, ?string $copilotHome = null): CopilotConfig
```

Creates configuration for a locally logged-in Copilot CLI user.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$cwd` | `?string` | Working directory. |
| `$copilotHome` | `?string` | Existing or isolated CLI home directory. |

#### Return Value

Returns a validated `CopilotConfig` instance.

#### Example

```php
$config = CopilotConfig::forCliUser(
    cwd: getcwd(),
    copilotHome: $_SERVER['HOME'] . '/.copilot'
);
```

## `ExtPhpCopilot\Copilot`

### Description

Application wrapper around the native Copilot client and a reusable session.

### `__construct()`

```php
public function __construct(CopilotConfig $config)
```

Starts the native Copilot client.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$config` | `CopilotConfig` | Wrapper configuration. |

#### Throws

| Exception | Condition |
| --- | --- |
| `ExtPhpCopilot\Exception\ConfigurationException` | Native extension is not loaded. |
| `ExtPhpCopilot\Exception\CopilotException` | Native client startup fails. |

#### Example

```php
use ExtPhpCopilot\Copilot;
use ExtPhpCopilot\CopilotConfig;

$copilot = new Copilot(CopilotConfig::fromEnvironment(getcwd(), __DIR__ . '/../var/copilot'));
```

### `fromConfig()`

```php
public static function fromConfig(array|CopilotConfig $config): Copilot
```

Creates a wrapper from a `CopilotConfig` instance or array config.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$config` | `array` or `CopilotConfig` | Wrapper configuration. |

#### Return Value

Returns a started `Copilot` wrapper.

#### Example

```php
$copilot = Copilot::fromConfig([
    'githubToken' => getenv('GITHUB_COPILOT_TOKEN'),
    'copilotHome' => __DIR__ . '/../var/copilot',
    'cwd' => __DIR__,
]);
```

### `fromEnvironment()`

```php
public static function fromEnvironment(?string $cwd = null, ?string $copilotHome = null): Copilot
```

Creates a token-authenticated wrapper from `GITHUB_COPILOT_TOKEN`.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$cwd` | `?string` | Working directory. |
| `$copilotHome` | `?string` | Copilot CLI state directory. |

#### Return Value

Returns a started `Copilot` wrapper.

#### Example

```php
$copilot = Copilot::fromEnvironment(
    cwd: getcwd(),
    copilotHome: __DIR__ . '/../var/copilot'
);
```

### `forCliUser()`

```php
public static function forCliUser(?string $cwd = null, ?string $copilotHome = null): Copilot
```

Creates a wrapper that uses existing logged-in CLI state.

#### Return Value

Returns a started `Copilot` wrapper.

#### Example

```php
$copilot = Copilot::forCliUser(cwd: getcwd());
```

### `authStatus()`

```php
public function authStatus(): array
```

Returns decoded authentication status from the native client.

#### Return Value

Returns an associative array from the Copilot SDK auth status response.

#### Example

```php
$status = $copilot->authStatus();
if (($status['isAuthenticated'] ?? false) !== true) {
    throw new RuntimeException('Copilot is not authenticated.');
}
```

### `assertAuthenticated()`

```php
public function assertAuthenticated(): void
```

Verifies that Copilot authentication is available.

#### Throws

| Exception | Condition |
| --- | --- |
| `ExtPhpCopilot\Exception\AuthenticationException` | Copilot is not authenticated. |

#### Example

```php
$copilot->assertAuthenticated();
```

### `createSession()`

```php
public function createSession(array $sessionConfig = []): Copilot\Session
```

Creates and stores a native session. A previously stored session is disconnected first.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$sessionConfig` | `array` | Session config merged with defaults from `CopilotConfig`. |

#### Return Value

Returns the native `Copilot\Session` instance.

#### Example

```php
$session = $copilot->createSession([
    'model' => 'gpt-5',
    'permissionPolicy' => 'deny_all',
    'clientName' => 'my-php-app',
]);
```

### `ask()`

```php
public function ask(string $prompt, array $messageOptions = [], array $sessionConfig = []): ?array
```

Creates a session when needed, sends a prompt, waits for one response event, and returns the decoded event.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$prompt` | `string` | User prompt to send. |
| `$messageOptions` | `array` | Message options, such as `timeoutSeconds`. |
| `$sessionConfig` | `array` | Optional session config for a new session. |

#### Return Value

Returns a decoded event array or `null` if no event arrives before timeout.

#### Example

```php
$event = $copilot->ask(
    'Summarize this PHP file.',
    ['timeoutSeconds' => 90],
    ['model' => 'gpt-5']
);
```

### `client()`

```php
public function client(): Copilot\Client
```

Returns the native client for lower-level calls.

#### Return Value

Returns `Copilot\Client`.

#### Example

```php
$models = json_decode($copilot->client()->modelsJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `session()`

```php
public function session(): ?Copilot\Session
```

Returns the stored native session, if one has been created.

#### Return Value

Returns `Copilot\Session` or `null`.

#### Example

```php
$session = $copilot->session();
if ($session !== null) {
    echo $session->id();
}
```

### `close()`

```php
public function close(): void
```

Disconnects the stored session and stops the native client.

#### Example

```php
try {
    $event = $copilot->ask('Hello Copilot.');
} finally {
    $copilot->close();
}
```

## See Also

- [COPILOT-NATIVE.md](COPILOT-NATIVE.md)
- [COPILOT-OPTIONS.md](COPILOT-OPTIONS.md)
- [COPILOT-EXAMPLES.md](COPILOT-EXAMPLES.md)
