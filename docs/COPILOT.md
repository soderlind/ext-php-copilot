# Copilot PHP API

This is the entry point for the ext-php-copilot documentation. The extension exposes a low-level native PHP API and a Composer-friendly PHP wrapper.

Use the wrapper for normal PHP applications. Use the native API when you need direct JSON-RPC access or want to work with the Rust SDK surface more closely.

## Documentation Map

- [COPILOT-WRAPPER.md](COPILOT-WRAPPER.md): recommended PHP app integration, authentication model, `ExtPhpCopilot\CopilotConfig`, and `ExtPhpCopilot\Copilot`.
- [COPILOT-NATIVE.md](COPILOT-NATIVE.md): native `copilot_sdk_version()`, `Copilot\Client`, and `Copilot\Session` methods.
- [COPILOT-OPTIONS.md](COPILOT-OPTIONS.md): client, transport, telemetry, session, message, and set-model options.
- [COPILOT-EXAMPLES.md](COPILOT-EXAMPLES.md): bundled scripts, wrapper flow, native flow, model listing, streaming, and live acceptance testing.

## Which API Should I Use?

Use `ExtPhpCopilot\Copilot` when building an app, plugin, CLI command, worker, or framework integration. It handles JSON conversion, safer auth defaults, session cleanup, and a one-call `ask()` workflow.

Use `Copilot\Client` and `Copilot\Session` directly when you need low-level control, streaming event polling, custom JSON-RPC calls, or exact SDK option shapes.

## Minimal Wrapper Example

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

## Minimal Native Example

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

## Authentication Summary

For web apps, prefer explicit token auth:

- Store `GITHUB_COPILOT_TOKEN` in an environment variable or secret manager.
- Keep `useLoggedInUser` disabled for webservers.
- Set an app-owned `copilotHome` outside the web root.
- Use `permissionPolicy: deny_all` unless the app intentionally allows tool permissions.

See [COPILOT-WRAPPER.md](COPILOT-WRAPPER.md) for the full authentication guidance.
