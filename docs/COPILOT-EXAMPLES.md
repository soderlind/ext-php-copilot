# Copilot Examples

Run examples with the debug extension loaded. On macOS, the extension file is `target/debug/libext_php_copilot.dylib`; on Linux it is usually `target/debug/libext_php_copilot.so`; on Windows it is a `.dll`.

```sh
cargo build
php -d extension=target/debug/libext_php_copilot.dylib examples/basic.php
php -d extension=target/debug/libext_php_copilot.dylib examples/models.php
php -d extension=target/debug/libext_php_copilot.dylib examples/streaming.php
php -d extension=target/debug/libext_php_copilot.dylib examples/generic_app.php
```

## Bundled Scripts

- `examples/basic.php`: direct native client/session flow.
- `examples/models.php`: model listing.
- `examples/streaming.php`: streaming event polling.
- `examples/generic_app.php`: recommended wrapper flow for generic PHP apps.

## Generic Wrapper Flow

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

## Direct Native Flow

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

## Model Listing

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

## Streaming

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

## Live Acceptance Test

Create a local `.env` file with a Copilot-enabled token. The file is ignored by Git.

```dotenv
GITHUB_COPILOT_TOKEN=your_token_here
```

Then run:

```sh
cargo build
php -d extension=target/debug/libext_php_copilot.dylib tests/acceptance.php
```

The acceptance test loads `.env`, verifies authentication, sends one prompt, and stores local Copilot CLI state under `var/copilot-acceptance`.
