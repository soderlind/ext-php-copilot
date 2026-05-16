<?php

declare(strict_types=1);

use ExtPhpCopilot\Copilot;

require __DIR__ . '/../php/Exception/CopilotException.php';
require __DIR__ . '/../php/Exception/AuthenticationException.php';
require __DIR__ . '/../php/Exception/ConfigurationException.php';
require __DIR__ . '/../php/CopilotConfig.php';
require __DIR__ . '/../php/Copilot.php';

load_env(dirname(__DIR__) . '/.env');

if (getenv('GITHUB_COPILOT_TOKEN') === false || getenv('GITHUB_COPILOT_TOKEN') === '') {
    fwrite(STDERR, 'SKIP GITHUB_COPILOT_TOKEN is not set.' . PHP_EOL);
    exit(77);
}

$projectRoot = dirname(__DIR__);
$copilotHome = $projectRoot . '/var/copilot-acceptance';

if (!is_dir($copilotHome) && !mkdir($copilotHome, 0700, true) && !is_dir($copilotHome)) {
    fwrite(STDERR, 'FAIL could not create local Copilot home.' . PHP_EOL);
    exit(1);
}

$copilot = Copilot::fromEnvironment(
    cwd: $projectRoot,
    copilotHome: $copilotHome,
);

try {
    $copilot->assertAuthenticated();
    fwrite(STDOUT, 'PASS authenticated with GitHub Copilot.' . PHP_EOL);

    $event = $copilot->ask(
        'Reply with exactly this text and no punctuation: ext php copilot acceptance ok',
        ['timeoutSeconds' => 90],
    );

    if ($event === null) {
        fwrite(STDERR, 'FAIL Copilot returned no event.' . PHP_EOL);
        exit(1);
    }

    fwrite(STDOUT, 'PASS received Copilot response event.' . PHP_EOL);
    fwrite(STDOUT, 'Event keys: ' . implode(', ', array_keys($event)) . PHP_EOL);
} finally {
    $copilot->close();
}

/**
 * @return void
 */
function load_env(string $path): void
{
    if (!is_file($path)) {
        return;
    }

    $lines = file($path, FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES);
    if ($lines === false) {
        return;
    }

    foreach ($lines as $line) {
        $line = trim($line);

        if ($line === '' || str_starts_with($line, '#') || !str_contains($line, '=')) {
            continue;
        }

        [$name, $value] = explode('=', $line, 2);
        $name = trim($name);
        $value = trim($value);

        if (str_starts_with($name, 'export ')) {
            $name = trim(substr($name, strlen('export ')));
        }

        if ($name === '' || getenv($name) !== false) {
            continue;
        }

        if (
            strlen($value) >= 2
            && (($value[0] === '"' && $value[strlen($value) - 1] === '"')
                || ($value[0] === "'" && $value[strlen($value) - 1] === "'"))
        ) {
            $value = substr($value, 1, -1);
        }

        putenv($name . '=' . $value);
        $_ENV[$name] = $value;
        $_SERVER[$name] = $value;
    }
}
