<?php

declare(strict_types=1);

$roots = ['php', 'examples', 'tests', 'stubs'];
$failed = false;

foreach ($roots as $root) {
    $directory = new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS);
    $files = new RecursiveIteratorIterator($directory);

    foreach ($files as $file) {
        if (!$file instanceof SplFileInfo || $file->getExtension() !== 'php') {
            continue;
        }

        $path = $file->getPathname();
        $command = escapeshellarg(PHP_BINARY) . ' -l ' . escapeshellarg($path);
        passthru($command, $exitCode);

        if ($exitCode !== 0) {
            $failed = true;
        }
    }
}

exit($failed ? 1 : 0);
