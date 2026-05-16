<!-- markdownlint-disable MD013 MD024 -->

# Examples Reference

## Summary

This page documents the bundled scripts and common usage recipes. Run examples with the debug extension loaded.

## Requirements

| Requirement | Description |
| --- | --- |
| Extension build | Run `cargo build` before executing examples. |
| Extension path | macOS uses `target/debug/libext_php_copilot.dylib`; Linux uses `target/debug/libext_php_copilot.so`; Windows uses a `.dll`. |
| Authentication | Set `GITHUB_COPILOT_TOKEN` or use explicit CLI-user configuration for local development. |

## Bundled Scripts

| Script | Purpose |
| --- | --- |
| `examples/basic.php` | Direct native client/session flow. |
| `examples/models.php` | Model listing. |
| `examples/streaming.php` | Streaming event polling. |
| `examples/generic_app.php` | Recommended wrapper flow for generic PHP apps. |

## Running Examples

### Synopsis

```sh
cargo build
php -d extension=target/debug/libext_php_copilot.dylib examples/basic.php
php -d extension=target/debug/libext_php_copilot.dylib examples/models.php
php -d extension=target/debug/libext_php_copilot.dylib examples/streaming.php
php -d extension=target/debug/libext_php_copilot.dylib examples/generic_app.php
```

### Notes

Replace the extension path with the Linux `.so` or Windows `.dll` path when running on another platform.

## Recipe: Generic Wrapper Flow

### Description

Use `ExtPhpCopilot\Copilot` when integrating Copilot into an application. This flow keeps auth, JSON handling, session lifecycle, and cleanup in one wrapper.

### Example

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

## Recipe: Direct Native Flow

### Description

Use `Copilot\Client` and `Copilot\Session` directly when you need raw JSON options or streaming/session control.

### Example

```php
use Copilot\Client;

$client = new Client(json_encode([
    'cwd' => getcwd(),
    'githubToken' => getenv('GITHUB_COPILOT_TOKEN'),
    'useLoggedInUser' => false,
    'copilotHome' => __DIR__ . '/../var/copilot',
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

## Recipe: Model Listing

### Description

Call `Copilot\Client::modelsJson()` to inspect available model metadata.

### Example

```php
$client = new Copilot\Client(json_encode([
    'githubToken' => getenv('GITHUB_COPILOT_TOKEN'),
    'useLoggedInUser' => false,
    'copilotHome' => __DIR__ . '/../var/copilot',
], JSON_THROW_ON_ERROR));

$models = json_decode($client->modelsJson(), true, 512, JSON_THROW_ON_ERROR);
print_r($models);

$client->stop();
```

## Recipe: Streaming Events

### Description

Create a streaming session, send a prompt with immediate delivery, and poll for events.

### Example

```php
$session = $client->createSession(json_encode([
    'streaming' => true,
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));

$session->send('Write a short haiku about PHP extensions.', json_encode([
    'mode' => 'immediate',
], JSON_THROW_ON_ERROR));

while (($eventJson = $session->nextEventJson(1000)) !== null) {
    $event = json_decode($eventJson, true, 512, JSON_THROW_ON_ERROR);
    echo $event['type'] ?? 'event', PHP_EOL;
}
```

## Recipe: Live Acceptance Test

### Description

The acceptance test loads `.env`, verifies authentication, sends one prompt, and stores local Copilot CLI state under `var/copilot-acceptance`.

### Environment

```dotenv
GITHUB_COPILOT_TOKEN=your_token_here
```

### Command

```sh
cargo build
php -d extension=target/debug/libext_php_copilot.dylib tests/acceptance.php
```

## See Also

- [COPILOT-WRAPPER.md](COPILOT-WRAPPER.md)
- [COPILOT-NATIVE.md](COPILOT-NATIVE.md)
- [COPILOT-OPTIONS.md](COPILOT-OPTIONS.md)
