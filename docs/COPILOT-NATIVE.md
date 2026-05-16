<!-- markdownlint-disable MD013 MD024 -->

# Native Extension API Reference

## Summary

The native API exposes the Rust extension directly to PHP. Complex inputs and outputs are JSON strings so the PHP surface can track the Copilot SDK without introducing many PHP value objects.

Use this API when you need raw status calls, model listing, manual session control, streaming event polling, or arbitrary SDK JSON-RPC calls.

## Namespace

```php
Copilot
```

## Symbols

| Symbol | Type | Description |
| --- | --- | --- |
| `copilot_sdk_version()` | function | Returns the extension crate version. |
| `Copilot\Client` | class | Starts and controls the Copilot CLI server connection. |
| `Copilot\Session` | class | Represents a Copilot conversation session. |

## Synopsis

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

### `copilot_sdk_version()`

```php
function copilot_sdk_version(): string
```

Returns the extension crate version.

#### Return Value

Returns a semantic version string.

#### Example

```php
echo copilot_sdk_version(), PHP_EOL;
```

## `Copilot\Client`

### Description

Starts the Copilot CLI server and opens a client connection.

### `__construct()`

```php
public function __construct(?string $optionsJson = null)
```

Creates a Copilot client from optional JSON-encoded client options.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$optionsJson` | `?string` | JSON object with client options. |

#### Throws

| Exception | Condition |
| --- | --- |
| `Throwable` | JSON is invalid, options are invalid, or the Copilot CLI server cannot start. |

#### Example

```php
$client = new Copilot\Client(json_encode([
    'githubToken' => getenv('GITHUB_COPILOT_TOKEN'),
    'useLoggedInUser' => false,
    'copilotHome' => __DIR__ . '/../var/copilot',
    'cwd' => getcwd(),
], JSON_THROW_ON_ERROR));
```

### `ping()`

```php
public function ping(?string $message = null): string
```

Pings the Copilot server.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$message` | `?string` | Optional ping message. |

#### Return Value

Returns a JSON string.

#### Example

```php
$response = json_decode($client->ping('hello'), true, 512, JSON_THROW_ON_ERROR);
```

### `modelsJson()`

```php
public function modelsJson(): string
```

Returns available model metadata.

#### Return Value

Returns a JSON string containing model metadata.

#### Example

```php
$models = json_decode($client->modelsJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `statusJson()`

```php
public function statusJson(): string
```

Returns general Copilot server status.

#### Return Value

Returns a JSON string.

#### Example

```php
$status = json_decode($client->statusJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `authStatusJson()`

```php
public function authStatusJson(): string
```

Returns authentication status.

#### Return Value

Returns a JSON string.

#### Example

```php
$auth = json_decode($client->authStatusJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `callJson()`

```php
public function callJson(string $method, ?string $paramsJson = null): string
```

Calls an arbitrary Copilot SDK JSON-RPC method.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$method` | `string` | SDK JSON-RPC method name. |
| `$paramsJson` | `?string` | JSON-encoded params object or array. |

#### Return Value

Returns a JSON string.

#### Example

```php
$result = json_decode(
    $client->callJson('status', json_encode([], JSON_THROW_ON_ERROR)),
    true,
    512,
    JSON_THROW_ON_ERROR
);
```

### `createSession()`

```php
public function createSession(?string $configJson = null): Copilot\Session
```

Creates a new Copilot session.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$configJson` | `?string` | JSON-encoded session configuration. |

#### Return Value

Returns a `Copilot\Session` instance.

#### Example

```php
$session = $client->createSession(json_encode([
    'model' => 'gpt-5',
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));
```

### `resumeSession()`

```php
public function resumeSession(string $sessionId, ?string $configJson = null): Copilot\Session
```

Resumes an existing session id.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$sessionId` | `string` | Session id to resume. |
| `$configJson` | `?string` | JSON-encoded session configuration. |

#### Return Value

Returns a `Copilot\Session` instance.

#### Example

```php
$session = $client->resumeSession($sessionId, json_encode([
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));
```

### `stop()`

```php
public function stop(): void
```

Stops the Copilot client/server. Calling it more than once is safe.

#### Example

```php
$client->stop();
```

## `Copilot\Session`

### Description

Represents an active or resumed Copilot conversation session.

### `id()`

```php
public function id(): string
```

Returns the session id.

#### Return Value

Returns the session id string.

#### Example

```php
echo $session->id(), PHP_EOL;
```

### `workspacePath()`

```php
public function workspacePath(): ?string
```

Returns the session workspace path when one is known.

#### Return Value

Returns the workspace path or `null`.

#### Example

```php
$workspacePath = $session->workspacePath();
```

### `remoteUrl()`

```php
public function remoteUrl(): ?string
```

Returns the remote URL when one is known.

#### Return Value

Returns the remote URL or `null`.

#### Example

```php
$remoteUrl = $session->remoteUrl();
```

### `capabilitiesJson()`

```php
public function capabilitiesJson(): string
```

Returns session capabilities.

#### Return Value

Returns a JSON string.

#### Example

```php
$capabilities = json_decode($session->capabilitiesJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `send()`

```php
public function send(string $prompt, ?string $optionsJson = null): string
```

Sends or enqueues a prompt.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$prompt` | `string` | Prompt text. |
| `$optionsJson` | `?string` | JSON-encoded message options. |

#### Return Value

Returns the message id.

#### Example

```php
$messageId = $session->send('List the top-level PHP files.', json_encode([
    'mode' => 'enqueue',
], JSON_THROW_ON_ERROR));
```

### `sendAndWaitJson()`

```php
public function sendAndWaitJson(string $prompt, ?string $optionsJson = null): string
```

Sends a prompt, waits for the next response event, and returns it.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$prompt` | `string` | Prompt text. |
| `$optionsJson` | `?string` | JSON-encoded message options. |

#### Return Value

Returns a JSON string containing the next response event.

#### Example

```php
$event = json_decode($session->sendAndWaitJson('Explain README.md.', json_encode([
    'timeoutSeconds' => 90,
], JSON_THROW_ON_ERROR)), true, 512, JSON_THROW_ON_ERROR);
```

### `messagesJson()`

```php
public function messagesJson(): string
```

Returns session messages.

#### Return Value

Returns a JSON string.

#### Example

```php
$messages = json_decode($session->messagesJson(), true, 512, JSON_THROW_ON_ERROR);
```

### `nextEventJson()`

```php
public function nextEventJson(?int $timeoutMs = null): ?string
```

Returns the next streaming event, or `null` when no event arrives before the timeout.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$timeoutMs` | `?int` | Wait timeout in milliseconds. |

#### Return Value

Returns a JSON event string or `null`.

#### Example

```php
while (($eventJson = $session->nextEventJson(250)) !== null) {
    $event = json_decode($eventJson, true, 512, JSON_THROW_ON_ERROR);
    var_dump($event['type'] ?? null);
}
```

### `abort()`

```php
public function abort(): void
```

Aborts the active Copilot operation for the session.

#### Example

```php
$session->abort();
```

### `setModel()`

```php
public function setModel(string $model, ?string $optionsJson = null): void
```

Changes the session model.

#### Parameters

| Name | Type | Description |
| --- | --- | --- |
| `$model` | `string` | Model name. |
| `$optionsJson` | `?string` | JSON-encoded set-model options. |

#### Example

```php
$session->setModel('gpt-5', json_encode([
    'reasoningEffort' => 'medium',
], JSON_THROW_ON_ERROR));
```

### `disconnect()`

```php
public function disconnect(): void
```

Disconnects the session. Calling it more than once is safe.

#### Example

```php
$session->disconnect();
```

## See Also

- [COPILOT-WRAPPER.md](COPILOT-WRAPPER.md)
- [COPILOT-OPTIONS.md](COPILOT-OPTIONS.md)
- [COPILOT-EXAMPLES.md](COPILOT-EXAMPLES.md)
