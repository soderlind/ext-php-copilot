<?php

declare(strict_types=1);

namespace Copilot {
    final class Client
    {
        public function __construct(?string $optionsJson = null) {}

        public function ping(?string $message = null): string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function modelsJson(): string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function statusJson(): string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function authStatusJson(): string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function callJson(string $method, ?string $paramsJson = null): string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function createSession(?string $configJson = null): Session
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function resumeSession(string $sessionId, ?string $configJson = null): Session
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function stop(): void {}
    }

    final class Session
    {
        public function id(): string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function workspacePath(): ?string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function remoteUrl(): ?string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function capabilitiesJson(): string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function send(string $prompt, ?string $optionsJson = null): string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function sendAndWaitJson(string $prompt, ?string $optionsJson = null): string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function messagesJson(): string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function nextEventJson(?int $timeoutMs = null): ?string
        {
            throw new \LogicException('This is an ext-php-copilot stub.');
        }

        public function abort(): void {}

        public function setModel(string $model, ?string $optionsJson = null): void {}

        public function disconnect(): void {}
    }
}

namespace {
    function copilot_sdk_version(): string
    {
        throw new \LogicException('This is an ext-php-copilot stub.');
    }
}
