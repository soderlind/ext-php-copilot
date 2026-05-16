<!-- markdownlint-disable MD013 MD024 -->

# Copilot PHP API

## Summary

`ext-php-copilot` exposes GitHub Copilot to PHP 8.3 applications through two API layers:

- `ExtPhpCopilot\Copilot`: a Composer-friendly wrapper for application code.
- `Copilot\Client` and `Copilot\Session`: native extension classes for direct JSON-RPC access.

Prefer the wrapper for applications, plugins, workers, and CLI commands. Use the native API when you need direct control over JSON payloads, streaming event polling, or SDK-specific options.

## Package Layout

| Document | Description |
| --- | --- |
| [COPILOT-WRAPPER.md](COPILOT-WRAPPER.md) | Wrapper API reference for `ExtPhpCopilot\CopilotConfig` and `ExtPhpCopilot\Copilot`. |
| [COPILOT-NATIVE.md](COPILOT-NATIVE.md) | Native extension reference for `copilot_sdk_version()`, `Copilot\Client`, and `Copilot\Session`. |
| [COPILOT-OPTIONS.md](COPILOT-OPTIONS.md) | JSON option reference for clients, transports, telemetry, sessions, messages, and model changes. |
| [COPILOT-EXAMPLES.md](COPILOT-EXAMPLES.md) | Runnable examples, acceptance test flow, and common usage recipes. |

## API Selection

| Use case | Recommended API |
| --- | --- |
| Generic PHP app integration | `ExtPhpCopilot\Copilot` |
| Token-based web application auth | `ExtPhpCopilot\Copilot::fromEnvironment()` |
| Local CLI development with existing login state | `ExtPhpCopilot\Copilot::forCliUser()` |
| Model listing, raw status, or SDK method calls | `Copilot\Client` |
| Streaming events or manual session control | `Copilot\Session` |

## Synopsis

### Wrapper API

```php
use ExtPhpCopilot\Copilot;

$copilot = Copilot::fromEnvironment(
    cwd: getcwd(),
    copilotHome: __DIR__ . '/../var/copilot'
);

try {
    $copilot->assertAuthenticated();
    $event = $copilot->ask('Explain this project in one paragraph.');
} finally {
    $copilot->close();
}
```

### Native API

```php
$client = new Copilot\Client(json_encode([
    'githubToken' => getenv('GITHUB_COPILOT_TOKEN'),
    'useLoggedInUser' => false,
    'copilotHome' => __DIR__ . '/../var/copilot',
], JSON_THROW_ON_ERROR));

$session = $client->createSession(json_encode([
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));

$eventJson = $session->sendAndWaitJson('Explain this project in one paragraph.');

$session->disconnect();
$client->stop();
```

## Authentication

For server-side PHP applications, use explicit token auth and isolated Copilot CLI state.

| Requirement | Recommendation |
| --- | --- |
| Token storage | Store `GITHUB_COPILOT_TOKEN` in the environment or a secret manager. |
| CLI user fallback | Keep `useLoggedInUser` disabled for web servers. |
| CLI state | Set an app-owned `copilotHome` outside the web root. |
| Tool permissions | Use `permissionPolicy` set to `deny_all` unless the app intentionally grants tools. |

## See Also

- [COPILOT-WRAPPER.md](COPILOT-WRAPPER.md)
- [COPILOT-NATIVE.md](COPILOT-NATIVE.md)
- [COPILOT-OPTIONS.md](COPILOT-OPTIONS.md)
- [COPILOT-EXAMPLES.md](COPILOT-EXAMPLES.md)
