<?php

declare(strict_types=1);

namespace ExtPhpCopilot;

use ExtPhpCopilot\Exception\AuthenticationException;
use ExtPhpCopilot\Exception\ConfigurationException;

final class Copilot
{
    private \Copilot\Client $client;
    private ?\Copilot\Session $session = null;

    public function __construct(private readonly CopilotConfig $config)
    {
        if (!extension_loaded('ext_php_copilot')) {
            throw new ConfigurationException('The ext_php_copilot PHP extension is not loaded.');
        }

        $this->client = new \Copilot\Client(self::encode($this->clientOptions()));
    }

    /**
     * @param array<string, mixed>|CopilotConfig $config
     */
    public static function fromConfig(array|CopilotConfig $config): self
    {
        return new self($config instanceof CopilotConfig ? $config : CopilotConfig::fromArray($config));
    }

    public static function fromEnvironment(?string $cwd = null, ?string $copilotHome = null): self
    {
        return new self(CopilotConfig::fromEnvironment($cwd, $copilotHome));
    }

    public static function forCliUser(?string $cwd = null, ?string $copilotHome = null): self
    {
        return new self(CopilotConfig::forCliUser($cwd, $copilotHome));
    }

    /**
     * @return array<string, mixed>
     */
    public function authStatus(): array
    {
        return self::decodeObject($this->client->authStatusJson());
    }

    public function assertAuthenticated(): void
    {
        $status = $this->authStatus();

        if (($status['isAuthenticated'] ?? false) !== true) {
            $message = is_string($status['statusMessage'] ?? null)
                ? $status['statusMessage']
                : 'GitHub Copilot authentication failed.';

            throw new AuthenticationException($message);
        }
    }

    /**
     * @param array<string, mixed> $sessionConfig
     */
    public function createSession(array $sessionConfig = []): \Copilot\Session
    {
        $this->session?->disconnect();
        $this->session = $this->client->createSession(self::encode($this->sessionConfig($sessionConfig)));

        return $this->session;
    }

    /**
     * @param array<string, mixed> $messageOptions
     * @param array<string, mixed> $sessionConfig
     * @return array<string, mixed>|null
     */
    public function ask(string $prompt, array $messageOptions = [], array $sessionConfig = []): ?array
    {
        $session = $this->session ?? $this->createSession($sessionConfig);
        $eventJson = $session->sendAndWaitJson(
            $prompt,
            self::encode($messageOptions + ['timeoutSeconds' => $this->config->timeoutSeconds])
        );

        return self::decodeNullableObject($eventJson);
    }

    public function client(): \Copilot\Client
    {
        return $this->client;
    }

    public function session(): ?\Copilot\Session
    {
        return $this->session;
    }

    public function close(): void
    {
        $this->session?->disconnect();
        $this->session = null;
        $this->client->stop();
    }

    public function __destruct()
    {
        try {
            $this->close();
        } catch (\Throwable) {
        }
    }

    /**
     * @return array<string, mixed>
     */
    private function clientOptions(): array
    {
        $options = $this->config->clientOptions;
        $options['useLoggedInUser'] = $this->config->useLoggedInUser;
        $options['copilotHome'] = $this->config->copilotHome;

        if ($this->config->cwd !== null) {
            $options['cwd'] = $this->config->cwd;
        }

        if ($this->config->githubToken !== null) {
            $options['githubToken'] = $this->config->githubToken;
        }

        return $options;
    }

    /**
     * @param array<string, mixed> $overrides
     * @return array<string, mixed>
     */
    private function sessionConfig(array $overrides): array
    {
        $config = $overrides + $this->config->sessionConfig + [
            'permissionPolicy' => $this->config->permissionPolicy,
        ];

        if ($this->config->model !== null && !array_key_exists('model', $config)) {
            $config['model'] = $this->config->model;
        }

        return $config;
    }

    /**
     * @param array<string, mixed> $value
     */
    private static function encode(array $value): string
    {
        return json_encode($value, JSON_THROW_ON_ERROR | JSON_UNESCAPED_SLASHES);
    }

    /**
     * @return array<string, mixed>
     */
    private static function decodeObject(string $json): array
    {
        $value = json_decode($json, true, 512, JSON_THROW_ON_ERROR);

        if (!is_array($value)) {
            throw new \UnexpectedValueException('Expected JSON object or array from ext-php-copilot.');
        }

        return $value;
    }

    /**
     * @return array<string, mixed>|null
     */
    private static function decodeNullableObject(string $json): ?array
    {
        $value = json_decode($json, true, 512, JSON_THROW_ON_ERROR);

        if ($value === null) {
            return null;
        }

        if (!is_array($value)) {
            throw new \UnexpectedValueException('Expected JSON object, array, or null from ext-php-copilot.');
        }

        return $value;
    }
}
