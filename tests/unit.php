<?php

declare(strict_types=1);

use ExtPhpCopilot\CopilotConfig;
use ExtPhpCopilot\Exception\ConfigurationException;

require __DIR__ . '/../php/Exception/CopilotException.php';
require __DIR__ . '/../php/Exception/ConfigurationException.php';
require __DIR__ . '/../php/CopilotConfig.php';

$tests = [];

$test = static function (string $name, callable $callback) use (&$tests): void {
    $tests[$name] = $callback;
};

function assert_same(mixed $expected, mixed $actual, string $message = ''): void
{
    if ($expected !== $actual) {
        throw new RuntimeException($message !== '' ? $message : sprintf(
            'Expected %s, got %s.',
            var_export($expected, true),
            var_export($actual, true)
        ));
    }
}

function assert_true(bool $actual, string $message = ''): void
{
    if (!$actual) {
        throw new RuntimeException($message !== '' ? $message : 'Expected true.');
    }
}

function assert_throws(string $className, callable $callback): Throwable
{
    try {
        $callback();
    } catch (Throwable $throwable) {
        if ($throwable instanceof $className) {
            return $throwable;
        }

        throw new RuntimeException(sprintf(
            'Expected %s, got %s.',
            $className,
            $throwable::class
        ), previous: $throwable);
    }

    throw new RuntimeException(sprintf('Expected %s to be thrown.', $className));
}

$test('fromArray accepts explicit token and safe defaults', static function (): void {
    $config = CopilotConfig::fromArray([
        'githubToken' => 'ghu_test',
        'copilotHome' => '/tmp/copilot-home',
    ]);

    assert_same('ghu_test', $config->githubToken);
    assert_same('/tmp/copilot-home', $config->copilotHome);
    assert_same(false, $config->useLoggedInUser);
    assert_same('deny_all', $config->permissionPolicy);
    assert_same(60, $config->timeoutSeconds);
});

$test('fromArray supports token and home aliases', static function (): void {
    $config = CopilotConfig::fromArray([
        'token' => 'alias-token',
        'home' => '/tmp/alias-home',
        'cwd' => '/tmp/project',
        'model' => 'gpt-test',
        'timeoutSeconds' => 7,
    ]);

    assert_same('alias-token', $config->githubToken);
    assert_same('/tmp/alias-home', $config->copilotHome);
    assert_same('/tmp/project', $config->cwd);
    assert_same('gpt-test', $config->model);
    assert_same(7, $config->timeoutSeconds);
});

$test('fromArray requires token unless CLI-user auth is explicit', static function (): void {
    $throwable = assert_throws(ConfigurationException::class, static function (): void {
        CopilotConfig::fromArray([]);
    });

    assert_same('Provide githubToken or explicitly enable useLoggedInUser.', $throwable->getMessage());

    $config = CopilotConfig::fromArray(['useLoggedInUser' => true]);
    assert_same(null, $config->githubToken);
    assert_same(true, $config->useLoggedInUser);
});

$test('fromEnvironment reads GITHUB_COPILOT_TOKEN and disables CLI fallback', static function (): void {
    $previous = getenv('GITHUB_COPILOT_TOKEN');
    putenv('GITHUB_COPILOT_TOKEN=env-token');

    try {
        $config = CopilotConfig::fromEnvironment('/tmp/app', '/tmp/env-home');
    } finally {
        if ($previous === false) {
            putenv('GITHUB_COPILOT_TOKEN');
        } else {
            putenv('GITHUB_COPILOT_TOKEN=' . $previous);
        }
    }

    assert_same('env-token', $config->githubToken);
    assert_same('/tmp/app', $config->cwd);
    assert_same('/tmp/env-home', $config->copilotHome);
    assert_same(false, $config->useLoggedInUser);
});

$test('forCliUser enables logged-in user mode without a token', static function (): void {
    $config = CopilotConfig::forCliUser('/tmp/app', '/tmp/cli-home');

    assert_same(null, $config->githubToken);
    assert_same(true, $config->useLoggedInUser);
    assert_same('/tmp/app', $config->cwd);
    assert_same('/tmp/cli-home', $config->copilotHome);
});

$test('invalid scalar and array settings throw configuration exceptions', static function (): void {
    assert_throws(ConfigurationException::class, static function (): void {
        CopilotConfig::fromArray([
            'githubToken' => ['not-a-string'],
            'copilotHome' => '/tmp/copilot-home',
        ]);
    });

    assert_throws(ConfigurationException::class, static function (): void {
        CopilotConfig::fromArray([
            'githubToken' => 'token',
            'copilotHome' => '/tmp/copilot-home',
            'clientOptions' => 'not-an-array',
        ]);
    });
});

$test('timeout and permission policy are validated', static function (): void {
    assert_throws(ConfigurationException::class, static function (): void {
        CopilotConfig::fromArray([
            'githubToken' => 'token',
            'timeoutSeconds' => 0,
        ]);
    });

    assert_throws(ConfigurationException::class, static function (): void {
        CopilotConfig::fromArray([
            'githubToken' => 'token',
            'permissionPolicy' => '',
        ]);
    });
});

$failed = 0;

foreach ($tests as $name => $callback) {
    try {
        $callback();
        fwrite(STDOUT, "PASS {$name}" . PHP_EOL);
    } catch (Throwable $throwable) {
        ++$failed;
        fwrite(STDERR, "FAIL {$name}: {$throwable->getMessage()}" . PHP_EOL);
    }
}

if ($failed > 0) {
    fwrite(STDERR, sprintf('%d unit test(s) failed.' . PHP_EOL, $failed));
    exit(1);
}

fwrite(STDOUT, sprintf('%d unit test(s) passed.' . PHP_EOL, count($tests)));
