<?php

declare(strict_types=1);

if (!extension_loaded('ext_php_copilot')) {
    fwrite(STDERR, "ext_php_copilot is not loaded\n");
    exit(1);
}

if (!class_exists(Copilot\Client::class) || !class_exists(Copilot\Session::class)) {
    fwrite(STDERR, "Copilot classes are not registered\n");
    exit(1);
}

if (!function_exists('copilot_sdk_version')) {
    fwrite(STDERR, "copilot_sdk_version is not registered\n");
    exit(1);
}

echo copilot_sdk_version(), PHP_EOL;
