<?php

declare(strict_types=1);

namespace ExtPhpCopilot;

use ExtPhpCopilot\Exception\ConfigurationException;

final readonly class CopilotConfig
{
    /**
     * @param array<string, mixed> $clientOptions
     * @param array<string, mixed> $sessionConfig
     */
    public function __construct(
        public ?string $githubToken,
        public string $copilotHome,
        public ?string $cwd = null,
        public bool $useLoggedInUser = false,
        public string $permissionPolicy = 'deny_all',
        public ?string $model = null,
        public int $timeoutSeconds = 60,
        public array $clientOptions = [],
        public array $sessionConfig = [],
    ) {
        if ($this->githubToken === null && !$this->useLoggedInUser) {
            throw new ConfigurationException('Provide githubToken or explicitly enable useLoggedInUser.');
        }

        if ($this->timeoutSeconds < 1) {
            throw new ConfigurationException('timeoutSeconds must be greater than zero.');
        }

        if ($this->permissionPolicy === '') {
            throw new ConfigurationException('permissionPolicy must not be empty.');
        }
    }

    /**
     * @param array<string, mixed> $config
     */
    public static function fromArray(array $config): self
    {
        $githubToken = self::nullableString($config['githubToken'] ?? $config['token'] ?? null);
        $useLoggedInUser = (bool) ($config['useLoggedInUser'] ?? false);
        $copilotHome = self::nullableString($config['copilotHome'] ?? $config['home'] ?? null)
            ?? self::defaultCopilotHome();

        return new self(
            githubToken: $githubToken,
            copilotHome: $copilotHome,
            cwd: self::nullableString($config['cwd'] ?? null),
            useLoggedInUser: $useLoggedInUser,
            permissionPolicy: (string) ($config['permissionPolicy'] ?? 'deny_all'),
            model: self::nullableString($config['model'] ?? null),
            timeoutSeconds: (int) ($config['timeoutSeconds'] ?? 60),
            clientOptions: self::arrayValue($config['clientOptions'] ?? []),
            sessionConfig: self::arrayValue($config['sessionConfig'] ?? []),
        );
    }

    public static function fromEnvironment(?string $cwd = null, ?string $copilotHome = null): self
    {
        $token = getenv('GITHUB_COPILOT_TOKEN');

        return new self(
            githubToken: $token === false || $token === '' ? null : $token,
            copilotHome: $copilotHome ?? self::defaultCopilotHome(),
            cwd: $cwd,
            useLoggedInUser: false,
        );
    }

    public static function forCliUser(?string $cwd = null, ?string $copilotHome = null): self
    {
        return new self(
            githubToken: null,
            copilotHome: $copilotHome ?? self::defaultCopilotHome(),
            cwd: $cwd,
            useLoggedInUser: true,
        );
    }

    private static function defaultCopilotHome(): string
    {
        return rtrim(sys_get_temp_dir(), DIRECTORY_SEPARATOR) . DIRECTORY_SEPARATOR . 'ext-php-copilot';
    }

    private static function nullableString(mixed $value): ?string
    {
        if ($value === null || $value === '') {
            return null;
        }

        if (!is_scalar($value)) {
            throw new ConfigurationException('Expected a string-compatible configuration value.');
        }

        return (string) $value;
    }

    /**
     * @return array<string, mixed>
     */
    private static function arrayValue(mixed $value): array
    {
        if (!is_array($value)) {
            throw new ConfigurationException('Expected configuration value to be an array.');
        }

        return $value;
    }
}
