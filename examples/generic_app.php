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

    $event = $copilot->ask('Explain what this PHP application does in one paragraph.');
    echo json_encode($event, JSON_PRETTY_PRINT | JSON_THROW_ON_ERROR), PHP_EOL;
} finally {
    $copilot->close();
}
