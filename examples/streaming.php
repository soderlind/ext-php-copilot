<?php

declare(strict_types=1);

use Copilot\Client;

$client = new Client(json_encode([
    'cwd' => getcwd(),
], JSON_THROW_ON_ERROR));

$session = $client->createSession(json_encode([
    'streaming' => true,
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));

try {
    $session->send('Write a short haiku about PHP extensions.');

    while ($eventJson = $session->nextEventJson(30000)) {
        $event = json_decode($eventJson, true, 512, JSON_THROW_ON_ERROR);
        if (($event['type'] ?? null) === 'assistant.message_delta') {
            echo $event['data']['delta'] ?? '';
        }
        if (($event['type'] ?? null) === 'session.idle') {
            break;
        }
    }

    echo PHP_EOL;
} finally {
    $session->disconnect();
    $client->stop();
}
