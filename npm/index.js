#!/usr/bin/env node

/**
 * Point d'entrée du package npm
 * Redirige vers le binaire Rust
 */

const { spawn } = require('child_process');
const path = require('path');

const platform = process.platform;
const binaryName = platform === 'win32' ? 'envsafe.exe' : 'envsafe';
const binaryPath = path.join(__dirname, 'bin', binaryName);

// Exécuter le binaire avec les arguments
const child = spawn(binaryPath, process.argv.slice(2), {
    stdio: 'inherit'
});

child.on('exit', (code) => {
    process.exit(code);
});
