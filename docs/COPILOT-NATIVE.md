# Native Copilot Extension API

Use the native API directly when you need full JSON-RPC access or want to avoid the wrapper. Complex inputs and outputs use JSON strings.

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

## Global Function

### `copilot_sdk_version(): string`

Returns the extension crate version.

```php
echo copilot_sdk_version(), PHP_EOL;
```

## Copilot\Client

### `new Copilot\Client(?string $optionsJson = null)`

Starts the Copilot CLI server and opens a client connection.

```php
$client = new Copilot\Client(json_encode([
    'githubToken' => getenv('GITHUB_COPILOT_TOKEN'),
    'useLoggedInUser' => false,
    'copilotHome' => __DIR__ . '/../var/copilot',
    'cwd' => getcwd(),
], JSON_THROW_ON_ERROR));
```

### `$client->ping(?string $message = null): string`

Pings the Copilot server and returns JSON.

```php
$response = json_decode($client->ping('hello'), true, 512, JSON_THROW_ON_ERROR);
```

### `$client->modelsJson(): string`

Returns available model metadata as JSON.

```php
$models = json_decode($client->modelsJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `$client->statusJson(): string`

Returns general Copilot server status as JSON.

```php
$status = json_decode($client->statusJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `$client->authStatusJson(): string`

Returns authentication status as JSON.

```php
$auth = json_decode($client->authStatusJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `$client->callJson(string $method, ?string $paramsJson = null): string`

Calls an arbitrary Copilot SDK JSON-RPC method and returns JSON.

```php
$result = json_decode(
    $client->callJson('status', json_encode([], JSON_THROW_ON_ERROR)),
    true,
    512,
    JSON_THROW_ON_ERROR
);
```

### `$client->createSession(?string $configJson = null): Copilot\Session`

Creates a new Copilot session.

```php
$session = $client->createSession(json_encode([
    'model' => 'gpt-5',
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));
```

### `$client->resumeSession(string $sessionId, ?string $configJson = null): Copilot\Session`

Resumes an existing session id.

```php
$session = $client->resumeSession($sessionId, json_encode([
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));
```

### `$client->stop(): void`

Stops the Copilot client/server. Calling it more than once is safe.

```php
$client->stop();
```

## Copilot\Session

### `$session->id(): string`

Returns the session id.

```php
echo $session->id(), PHP_EOL;
```

### `$session->workspacePath(): ?string`

Returns the session workspace path when one is known.

```php
$workspacePath = $session->workspacePath();
```

### `$session->remoteUrl(): ?string`

Returns the remote URL when one is known.

```php
$remoteUrl = $session->remoteUrl();
```

### `$session->capabilitiesJson(): string`

Returns session capabilities as JSON.

```php
$capabilities = json_decode($session->capabilitiesJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `$session->send(string $prompt, ?string $optionsJson = null): string`

Enqueues or sends a prompt and returns the message id.

```php
$messageId = $session->send('List the top-level PHP files.', json_encode([
    'mode' => 'enqueue',
], JSON_THROW_ON_ERROR));
```

### `$session->sendAndWaitJson(string $prompt, ?string $optionsJson = null): string`

Sends a prompt, waits for the next response event, and returns JSON.

```php
$event = json_decode($session->sendAndWaitJson('Explain README.md.', json_encode([
    'timeoutSeconds' => 90,
], JSON_THROW_ON_ERROR)), true, 512, JSON_THROW_ON_ERROR);
```

### `$session->messagesJson(): string`

Returns session messages as JSON.

```php
$messages = json_decode($session->messagesJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `$session->nextEventJson(?int $timeoutMs = null): ?string`

Returns the next streaming event as JSON, or `null` when no event arrives before the timeout.

```php
while (($eventJson = $session->nextEventJson(250)) !== null) {
    $event = json_decode($eventJson, true, 512, JSON_THROW_ON_ERROR);
    var_dump($event['type'] ?? null);
}
```

### `$session->abort(): void`

Aborts the active Copilot operation for the session.

```php
$session->abort();
```

### `$session->setModel(string $model, ?string $optionsJson = null): void`

Changes the session model.

```php
$session->setModel('gpt-5', json_encode([
    'reasoningEffort' => 'medium',
], JSON_THROW_ON_ERROR));
```

### `$session->disconnect(): void`

Disconnects the session. Calling it more than once is safe.

```php
$session->disconnect();
```
