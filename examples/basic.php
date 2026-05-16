<?php

declare(strict_types=1);

use Copilot\Client;

$client = new Client(json_encode([
    'cwd' => getcwd(),
    'logLevel' => 'info',
], JSON_THROW_ON_ERROR));

$session = $client->createSession(json_encode([
    'streaming' => false,
    'permissionPolicy' => 'deny_all',
], JSON_THROW_ON_ERROR));

try {
    echo $session->sendAndWaitJson('Say hello from ext-php-copilot in one sentence.', json_encode([
        'timeoutSeconds' => 60,
    ], JSON_THROW_ON_ERROR));
    echo PHP_EOL;
} finally {
    $session->disconnect();
    $client->stop();
}
