# :package: Installation Guide

EnvSafe CLI can be installed in several ways. We recommend using **Cargo** for Rust developers or downloading pre-compiled binaries for CI/CD environments.

## Via Cargo (Recommended)

If you have Rust installed, this is the easiest method:

```bash
cargo install envsafe-cli
```

To update to the latest version:

```bash
cargo install --force envsafe-cli
```

## Via NPM (Wrapper)

For Node.js projects, you can use our npm wrapper which downloads the appropriate binary:

```bash
npm install -g @envsafes-org/cli
```

## Binary Download

Download the latest release for your platform from our [Releases Page](https://github.com/Ifiboys/rust-envsafe-cli/releases).

| Platform | Architecture | File |
|----------|--------------|------|
| **macOS** | Intel (x64) | `envsafe-macos-x86_64` |
| **macOS** | Apple Silicon (M1/M2) | `envsafe-macos-aarch64` |
| **Linux** | x64 (Standard) | `envsafe-linux-x86_64` |
| **Windows** | x64 | `envsafe-windows-x86_64.exe` |

Make the binary executable (Linux/macOS):

```bash
chmod +x envsafe
sudo mv envsafe /usr/local/bin/
```

## From Source

Build directly from the repository:

```bash
git clone https://github.com/Ifiboys/rust-envsafe-cli.git
cd rust-envsafe-cli
cargo build --release
```

The binary will be located at `target/release/envsafe`.

## Docker Image

We provide a minimal Docker image for CI/CD pipelines:

```bash
docker pull ghcr.io/ifiboys/envsafe-cli:latest
```

Example usage:

```bash
docker run --rm -it ghcr.io/ifiboys/envsafe-cli --version
```
