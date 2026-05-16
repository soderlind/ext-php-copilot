<!-- markdownlint-disable MD013 MD024 -->

# Options Reference

## Summary

Native API methods accept JSON-encoded option objects. Wrapper methods accept PHP arrays and encode them before calling the native extension.

Most option names use camelCase. Many extension-specific keys also accept snake_case aliases.

## Option Groups

| Group | Used by |
| --- | --- |
| Client options | `new Copilot\Client($optionsJson)`, `CopilotConfig::$clientOptions` |
| Transport options | `clientOptions['transport']` |
| Telemetry options | `clientOptions['telemetry']` |
| Session options | `createSession()`, `resumeSession()`, `CopilotConfig::$sessionConfig`, `Copilot::ask()` |
| Message options | `send()`, `sendAndWaitJson()`, `Copilot::ask()` |
| Set-model options | `Copilot\Session::setModel()` |

## Client Options

### Description

Client options control Copilot CLI process startup, authentication, transport, and telemetry.

### Reference

| Option | Type | Alias | Description |
| --- | --- | --- | --- |
| `programPath` | `string` | `program_path` | Explicit Copilot CLI path. |
| `cwd` | `string` | none | Working directory for the CLI server. |
| `env` | `array<string,string>` | none | Extra child-process environment map. |
| `envRemove` | `string[]` | `env_remove` | Environment variable names to remove. |
| `prefixArgs` | `string[]` | `prefix_args` | Arguments inserted before CLI server flags. |
| `extraArgs` | `string[]` | `extra_args` | Extra CLI arguments after transport flags. |
| `githubToken` | `string` | `github_token` | Token passed through the SDK auth-token flow. |
| `useLoggedInUser` | `bool` | `use_logged_in_user` | Allows existing logged-in CLI credentials. |
| `logLevel` | `string` | `log_level` | `none`, `error`, `warning`, `warn`, `info`, `debug`, `all`, or `trace`. |
| `sessionIdleTimeoutSeconds` | `int` | `session_idle_timeout_seconds` | CLI server idle timeout in seconds. |
| `copilotHome` | `string` | `copilot_home` | Isolated Copilot state directory. |
| `tcpConnectionToken` | `string` | `tcp_connection_token` | Token for TCP/external transport. |
| `remote` | `bool` | none | Enables remote session support. |
| `transport` | `array` | none | Transport config. See [Transport Options](#transport-options). |
| `telemetry` | `array` | none | Telemetry config. See [Telemetry Options](#telemetry-options). |

### Example

```php
$client = new Copilot\Client(json_encode([
    'cwd' => getcwd(),
    'githubToken' => getenv('GITHUB_COPILOT_TOKEN'),
    'useLoggedInUser' => false,
    'copilotHome' => __DIR__ . '/../var/copilot',
    'logLevel' => 'info',
], JSON_THROW_ON_ERROR));
```

## Transport Options

### Description

`transport.type` controls how the extension talks to the Copilot CLI server.

### Reference

| Type | Options | Example |
| --- | --- | --- |
| `stdio` | none | `['transport' => ['type' => 'stdio']]` |
| `tcp` | `port`, defaults to `0` | `['transport' => ['type' => 'tcp', 'port' => 4141]]` |
| `external` | `host`, `port`; host defaults to `127.0.0.1`, port defaults to `0` | `['transport' => ['type' => 'external', 'host' => '127.0.0.1', 'port' => 4141]]` |

## Telemetry Options

### Description

Telemetry options configure SDK telemetry export. Avoid `captureContent` unless your application is allowed to persist prompt and response content.

### Reference

| Option | Type | Alias | Description |
| --- | --- | --- | --- |
| `otlpEndpoint` | `string` | `otlp_endpoint` | OTLP HTTP endpoint. |
| `filePath` | `string` | `file_path` | Telemetry output file. |
| `sourceName` | `string` | `source_name` | Telemetry source name. |
| `captureContent` | `bool` | `capture_content` | Include prompt/response content in telemetry. |
| `exporterType` | `string` | `exporter_type` | `otlp-http`, `otlpHttp`, or `file`. |

### Example

```php
$client = new Copilot\Client(json_encode([
    'telemetry' => [
        'exporterType' => 'file',
        'filePath' => __DIR__ . '/../var/copilot-telemetry.jsonl',
        'sourceName' => 'my-php-app',
        'captureContent' => false,
    ],
], JSON_THROW_ON_ERROR));
```

## Session Options

### Description

Session options configure a Copilot conversation session. The wrapper merges `CopilotConfig::$sessionConfig` with per-call session config.

### Reference

| Option | Type | Description |
| --- | --- | --- |
| `sessionId` | `string` | Resume target. Injected by `resumeSession()`. |
| `model` | `string` | Model name. |
| `clientName` | `string` | App/client display name. |
| `reasoningEffort` | `string` | Requested reasoning effort. |
| `streaming` | `bool` | Enables streaming events. |
| `systemMessage` | `string` | Session system instructions. |
| `availableTools` | `array` | Tool allow-list passed to the SDK. |
| `excludedTools` | `array` | Tool deny-list passed to the SDK. |
| `mcpServers` | `array` | MCP server configuration passed to the SDK. |
| `enableConfigDiscovery` | `bool` | Allows SDK config discovery. |
| `requestUserInput` | `mixed` | SDK user-input callback/config field. |
| `requestPermission` | `mixed` | SDK permission callback/config field. |
| `requestElicitation` | `mixed` | SDK elicitation callback/config field. |
| `requestExitPlanMode` | `mixed` | SDK exit-plan callback/config field. |
| `requestAutoModeSwitch` | `mixed` | SDK auto-mode callback/config field. |
| `skillDirectories` | `string[]` | Skill directories. |
| `instructionDirectories` | `string[]` | Instruction directories. |
| `disabledSkills` | `string[]` | Skills to disable. |
| `customAgents` | `array` | Custom agent definitions. |
| `defaultAgent` | `string` | Default agent name. |
| `agent` | `string` | Agent for the session. |
| `infiniteSessions` | `bool` | Enables infinite sessions when supported by SDK. |
| `provider` | `string` | Provider override. |
| `enableSessionTelemetry` | `bool` | Enables session telemetry. |
| `configDir` | `string` | Config directory. |
| `workingDirectory` | `string` | Session working directory. |
| `gitHubToken` | `string` | Session-level GitHub token field accepted by the SDK. |
| `includeSubAgentStreamingEvents` | `bool` | Includes sub-agent streaming events. |
| `permissionPolicy` | `string` | Extension helper: `deny_all`, `denyAll`, `approve_all`, or `approveAll`. Defaults to `deny_all`. |

### Example

```php
$session = $client->createSession(json_encode([
    'model' => 'gpt-5',
    'clientName' => 'my-php-app',
    'streaming' => true,
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));
```

## Message Options

### Description

Message options configure prompt delivery and wait behavior.

### Reference

| Option | Type | Alias | Description |
| --- | --- | --- | --- |
| `mode` | `string` | none | Delivery mode, commonly `enqueue` or `immediate`. |
| `timeoutSeconds` | `int` | `timeout_seconds` | Wait timeout in seconds for `sendAndWaitJson()` or `ask()`. |
| `timeoutMs` | `int` | `timeout_ms` | Wait timeout in milliseconds. |
| `attachments` | `array` | none | SDK attachment JSON array. |
| `requestHeaders` | `array<string,string>` | `request_headers` | Request header map. |
| `traceparent` | `string` | none | W3C traceparent value. |
| `tracestate` | `string` | none | W3C tracestate value. |

### Example

```php
$event = json_decode($session->sendAndWaitJson('Summarize this class.', json_encode([
    'mode' => 'immediate',
    'timeoutSeconds' => 90,
    'requestHeaders' => ['x-request-id' => bin2hex(random_bytes(8))],
], JSON_THROW_ON_ERROR)), true, 512, JSON_THROW_ON_ERROR);
```

## Set-Model Options

### Description

Set-model options configure a session model change.

### Reference

| Option | Type | Alias | Description |
| --- | --- | --- | --- |
| `reasoningEffort` | `string` | `reasoning_effort` | Requested reasoning effort for the new model. |

### Example

```php
$session->setModel('gpt-5', json_encode([
    'reasoningEffort' => 'high',
], JSON_THROW_ON_ERROR));
```

## See Also

- [COPILOT-WRAPPER.md](COPILOT-WRAPPER.md)
- [COPILOT-NATIVE.md](COPILOT-NATIVE.md)
- [COPILOT-EXAMPLES.md](COPILOT-EXAMPLES.md)
