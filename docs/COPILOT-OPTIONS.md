# Copilot Options Reference

Most option names accept camelCase. Many extension-specific keys also accept snake_case aliases.

## Client Options

Available through `new Copilot\Client($optionsJson)` or `CopilotConfig::$clientOptions`.

| Option | Type | Alias | Description | Example |
| --- | --- | --- | --- | --- |
| `programPath` | string | `program_path` | Explicit Copilot CLI path. | `['programPath' => '/usr/local/bin/copilot']` |
| `cwd` | string | none | Working directory for the CLI server. | `['cwd' => getcwd()]` |
| `env` | object | none | Extra child-process environment map. | `['env' => ['APP_ENV' => 'local']]` |
| `envRemove` | string[] | `env_remove` | Environment variable names to remove. | `['envRemove' => ['GITHUB_TOKEN']]` |
| `prefixArgs` | string[] | `prefix_args` | Arguments inserted before CLI server flags. | `['prefixArgs' => ['--verbose']]` |
| `extraArgs` | string[] | `extra_args` | Extra CLI arguments after transport flags. | `['extraArgs' => ['--log-level', 'debug']]` |
| `githubToken` | string | `github_token` | Token passed through the SDK auth-token flow. | `['githubToken' => getenv('GITHUB_COPILOT_TOKEN')]` |
| `useLoggedInUser` | bool | `use_logged_in_user` | Allows existing logged-in CLI credentials. | `['useLoggedInUser' => false]` |
| `logLevel` | string | `log_level` | `none`, `error`, `warning`, `warn`, `info`, `debug`, `all`, or `trace`. | `['logLevel' => 'info']` |
| `sessionIdleTimeoutSeconds` | int | `session_idle_timeout_seconds` | CLI server idle timeout in seconds. | `['sessionIdleTimeoutSeconds' => 300]` |
| `copilotHome` | string | `copilot_home` | Isolated Copilot state directory. | `['copilotHome' => __DIR__ . '/../var/copilot']` |
| `tcpConnectionToken` | string | `tcp_connection_token` | Token for TCP/external transport. | `['tcpConnectionToken' => bin2hex(random_bytes(16))]` |
| `remote` | bool | none | Enables remote session support. | `['remote' => true]` |
| `transport` | object | none | Transport config; see transport options. | `['transport' => ['type' => 'stdio']]` |
| `telemetry` | object | none | Telemetry config; see telemetry options. | `['telemetry' => ['exporterType' => 'file']]` |

Client option example:

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

`transport.type` controls how the extension talks to the Copilot CLI server.

| Type | Options | Example |
| --- | --- | --- |
| `stdio` | none | `['transport' => ['type' => 'stdio']]` |
| `tcp` | `port`, defaults to `0` | `['transport' => ['type' => 'tcp', 'port' => 4141]]` |
| `external` | `host`, `port`; host defaults to `127.0.0.1`, port defaults to `0` | `['transport' => ['type' => 'external', 'host' => '127.0.0.1', 'port' => 4141]]` |

## Telemetry Options

Available inside the `telemetry` client option.

| Option | Type | Alias | Description | Example |
| --- | --- | --- | --- | --- |
| `otlpEndpoint` | string | `otlp_endpoint` | OTLP HTTP endpoint. | `['otlpEndpoint' => 'http://127.0.0.1:4318']` |
| `filePath` | string | `file_path` | Telemetry output file. | `['filePath' => __DIR__ . '/../var/copilot-telemetry.jsonl']` |
| `sourceName` | string | `source_name` | Telemetry source name. | `['sourceName' => 'my-php-app']` |
| `captureContent` | bool | `capture_content` | Include prompt/response content in telemetry. | `['captureContent' => false]` |
| `exporterType` | string | `exporter_type` | `otlp-http`, `otlpHttp`, or `file`. | `['exporterType' => 'file']` |

Telemetry example:

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

Available through `createSession($configJson)`, `resumeSession($sessionId, $configJson)`, `CopilotConfig::$sessionConfig`, or the third argument to `Copilot::ask()`.

| Option | Type | Description | Example |
| --- | --- | --- | --- |
| `sessionId` | string | Resume target. Injected by `resumeSession()`. | `['sessionId' => 'session-123']` |
| `model` | string | Model name. | `['model' => 'gpt-5']` |
| `clientName` | string | App/client display name. | `['clientName' => 'my-php-app']` |
| `reasoningEffort` | string | Requested reasoning effort. | `['reasoningEffort' => 'medium']` |
| `streaming` | bool | Enables streaming events. | `['streaming' => true]` |
| `systemMessage` | string | Session system instructions. | `['systemMessage' => 'Answer tersely.']` |
| `availableTools` | array | Tool allow-list passed to the SDK. | `['availableTools' => ['read_file']]` |
| `excludedTools` | array | Tool deny-list passed to the SDK. | `['excludedTools' => ['run_shell_command']]` |
| `mcpServers` | object | MCP server configuration passed to the SDK. | `['mcpServers' => ['local' => ['command' => 'php']]]` |
| `enableConfigDiscovery` | bool | Allows SDK config discovery. | `['enableConfigDiscovery' => false]` |
| `requestUserInput` | mixed | SDK user-input callback/config field. | `['requestUserInput' => false]` |
| `requestPermission` | mixed | SDK permission callback/config field. | `['requestPermission' => false]` |
| `requestElicitation` | mixed | SDK elicitation callback/config field. | `['requestElicitation' => false]` |
| `requestExitPlanMode` | mixed | SDK exit-plan callback/config field. | `['requestExitPlanMode' => false]` |
| `requestAutoModeSwitch` | mixed | SDK auto-mode callback/config field. | `['requestAutoModeSwitch' => false]` |
| `skillDirectories` | string[] | Skill directories. | `['skillDirectories' => [__DIR__ . '/skills']]` |
| `instructionDirectories` | string[] | Instruction directories. | `['instructionDirectories' => [__DIR__ . '/instructions']]` |
| `disabledSkills` | string[] | Skills to disable. | `['disabledSkills' => ['experimental']]` |
| `customAgents` | array | Custom agent definitions. | `['customAgents' => [['name' => 'reviewer']]]` |
| `defaultAgent` | string | Default agent name. | `['defaultAgent' => 'reviewer']` |
| `agent` | string | Agent for the session. | `['agent' => 'reviewer']` |
| `infiniteSessions` | bool | Enables infinite sessions when supported by SDK. | `['infiniteSessions' => false]` |
| `provider` | string | Provider override. | `['provider' => 'github-copilot']` |
| `enableSessionTelemetry` | bool | Enables session telemetry. | `['enableSessionTelemetry' => true]` |
| `configDir` | string | Config directory. | `['configDir' => __DIR__ . '/../config/copilot']` |
| `workingDirectory` | string | Session working directory. | `['workingDirectory' => getcwd()]` |
| `gitHubToken` | string | Session-level GitHub token field accepted by the SDK. | `['gitHubToken' => getenv('GITHUB_COPILOT_TOKEN')]` |
| `includeSubAgentStreamingEvents` | bool | Includes sub-agent streaming events. | `['includeSubAgentStreamingEvents' => true]` |
| `permissionPolicy` | string | Extension helper: `deny_all`, `denyAll`, `approve_all`, or `approveAll`. Defaults to `deny_all`. | `['permissionPolicy' => 'deny_all']` |

Session option example:

```php
$session = $client->createSession(json_encode([
    'model' => 'gpt-5',
    'clientName' => 'my-php-app',
    'streaming' => true,
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));
```

## Message Options

Available through `send($prompt, $optionsJson)`, `sendAndWaitJson($prompt, $optionsJson)`, or the second argument to `Copilot::ask()`.

| Option | Type | Alias | Description | Example |
| --- | --- | --- | --- | --- |
| `mode` | string | none | Delivery mode, commonly `enqueue` or `immediate`. | `['mode' => 'immediate']` |
| `timeoutSeconds` | int | `timeout_seconds` | Wait timeout in seconds for `sendAndWaitJson()` or `ask()`. | `['timeoutSeconds' => 90]` |
| `timeoutMs` | int | `timeout_ms` | Wait timeout in milliseconds. | `['timeoutMs' => 5000]` |
| `attachments` | array | none | SDK attachment JSON array. | `['attachments' => [['type' => 'text', 'content' => 'Context']]]` |
| `requestHeaders` | object | `request_headers` | Request header map. | `['requestHeaders' => ['x-trace-id' => 'abc123']]` |
| `traceparent` | string | none | W3C traceparent value. | `['traceparent' => '00-00000000000000000000000000000000-0000000000000000-01']` |
| `tracestate` | string | none | W3C tracestate value. | `['tracestate' => 'vendor=value']` |

Message option example:

```php
$event = json_decode($session->sendAndWaitJson('Summarize this class.', json_encode([
    'mode' => 'immediate',
    'timeoutSeconds' => 90,
    'requestHeaders' => ['x-request-id' => bin2hex(random_bytes(8))],
], JSON_THROW_ON_ERROR)), true, 512, JSON_THROW_ON_ERROR);
```

## Set Model Options

Available through `$session->setModel($model, $optionsJson)`.

| Option | Type | Alias | Description | Example |
| --- | --- | --- | --- | --- |
| `reasoningEffort` | string | `reasoning_effort` | Requested reasoning effort for the new model. | `['reasoningEffort' => 'medium']` |

Set model option example:

```php
$session->setModel('gpt-5', json_encode([
    'reasoningEffort' => 'high',
], JSON_THROW_ON_ERROR));
```
