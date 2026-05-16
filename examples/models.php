<?php

declare(strict_types=1);

use Copilot\Client;

$client = new Client(json_encode([
    'cwd' => getcwd(),
], JSON_THROW_ON_ERROR));

try {
    echo $client->modelsJson(), PHP_EOL;
} finally {
    $client->stop();
}
