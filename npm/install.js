#!/usr/bin/env node

/**
 * Script d'installation du binaire Rust via npm
 * Télécharge le binaire pré-compilé correspondant à la plateforme
 */

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const GITHUB_REPO = 'Ifiboys/rust-envsafe-cli';
const VERSION = require('./package.json').version;

// Déterminer la plateforme et l'architecture
const platform = process.platform;
const arch = process.arch;

// Mapping des plateformes
const PLATFORM_MAP = {
    'darwin': 'macos',
    'linux': 'linux',
    'win32': 'windows'
};

const ARCH_MAP = {
    'x64': 'x86_64',
    'arm64': 'aarch64'
};

const platformName = PLATFORM_MAP[platform];
const archName = ARCH_MAP[arch];

if (!platformName || !archName) {
    console.error(`❌ Plateforme non supportée: ${platform}-${arch}`);
    console.error('Plateformes supportées: macOS (x64, arm64), Linux (x64, arm64), Windows (x64)');
    process.exit(1);
}

// URL du binaire
const binaryName = platform === 'win32' ? 'envsafe-bin.exe' : 'envsafe-bin';
const downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/envsafe-${platformName}-${archName}${platform === 'win32' ? '.exe' : ''}`;

// Dossier de destination
const binDir = path.join(__dirname, 'bin');
const binaryPath = path.join(binDir, binaryName);

console.log('📦 Installation d\'EnvSafe CLI (Rust)...');
console.log(`   Plateforme: ${platformName}-${archName}`);
console.log(`   Version: ${VERSION}`);
console.log('');

// Créer le dossier bin
if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
}

// Télécharger le binaire
console.log(`⬇️  Téléchargement depuis GitHub...`);
console.log(`   ${downloadUrl}`);

const file = fs.createWriteStream(binaryPath);

https.get(downloadUrl, (response) => {
    if (response.statusCode === 302 || response.statusCode === 301) {
        // Suivre la redirection
        https.get(response.headers.location, (redirectResponse) => {
            redirectResponse.pipe(file);

            file.on('finish', () => {
                file.close(() => {
                    // Rendre le binaire exécutable
                    if (platform !== 'win32') {
                        fs.chmodSync(binaryPath, 0o755);
                    }

                    console.log('✅ Installation réussie !');
                    console.log('');
                    console.log('Utilisation :');
                    console.log('  envsafe --help');
                    console.log('  envsafe login');
                    console.log('  envsafe pull --dev');
                    console.log('');
                });
            });
        });
    } else if (response.statusCode === 200) {
        response.pipe(file);

        file.on('finish', () => {
            file.close(() => {
                if (platform !== 'win32') {
                    fs.chmodSync(binaryPath, 0o755);
                }

                console.log('✅ Installation réussie !');
                console.log('');
                console.log('Utilisation :');
                console.log('  envsafe --help');
                console.log('  envsafe login');
                console.log('');
            });
        });
    } else if (response.statusCode === 404) {
        console.error('❌ Binaire non trouvé pour cette plateforme');
        console.error('');
        console.error('Solutions :');
        console.error('  1. Installer avec Cargo: cargo install envsafe-cli');
        console.error('  2. Compiler depuis les sources');
        console.error('  3. Attendre que les binaires soient disponibles');
        process.exit(1);
    } else {
        console.error(`❌ Erreur de téléchargement: ${response.statusCode}`);
        process.exit(1);
    }
}).on('error', (err) => {
    fs.unlink(binaryPath, () => { });
    console.error('❌ Erreur de téléchargement:', err.message);
    console.error('');
    console.error('Solution alternative :');
    console.error('  cargo install envsafe-cli');
    process.exit(1);
});
