<div align="center">

# 🔐 EnvSafe CLI (Rust)

**Blazing fast, secure environment variable manager with real-time hot reload**

[![Build Status](https://github.com/Ifiboys/envsafe-cli-rust/workflows/CI/badge.svg)](https://github.com/Ifiboys/envsafe-cli-rust/actions)
[![Crates.io](https://img.shields.io/crates/v/envsafe-cli.svg)](https://crates.io/crates/envsafe-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

[Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Contributing](#-contributing) • [Documentation](#-documentation)

</div>

---

## 📝 Table of Contents

- [About](#about)
- [Features](#-features)
- [Why Rust?](#-why-rust)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Usage](#-usage)
- [Docker Integration](#-docker-integration)
- [Development](#-development)
- [Contributing](#-contributing)
- [Architecture](#-architecture)
- [License](#-license)

## 🎯 About

EnvSafe CLI (Rust) is a high-performance command-line tool for managing environment variables securely. It's part of the [EnvSafe](https://www.envsafe.dev) ecosystem and provides:

- **🔄 Real-time hot reload** via WebSocket connectivity
- **💾 Shared memory IPC** for ultra-fast variable access across processes  
- **♻️ Automatic secret rotation** with Docker integration
- **⚡ Blazing performance** - 40x faster startup than Node.js alternative

Built for teams who need secure, centralized environment variable management with modern DevOps workflows.

## ✨ Features

### Core Features
- ✅ **Secure Authentication** - Token-based API authentication
- ✅ **Workspace Management** - Organize projects in workspaces
- ✅ **Multi-Environment** - Development, Staging, Production
- ✅ **Pull/Push Variables** - Sync .env files with EnvSafe backend
- ✅ **Command Injection** - Run commands with environment variables loaded
- ✅ **Multi-Language** - English and French support

### 🌟 Advanced Features

#### Real-Time Hot Reload
```bash
envsafe watch --dev
```
- Watch remote changes via WebSocket
- Automatic local `.env` updates
- Shared memory synchronization
- Zero-downtime variable updates

#### Automatic Secret Rotation
```bash
envsafe rotate enable --interval 30
envsafe rotate now --vars API_KEY,JWT_SECRET
```
- Cryptographically secure secret generation (SHA-256)
- Configurable rotation intervals
- Variable exclusion policies
- Docker-ready rotation workflows

#### Shared Memory IPC
- **Sub-millisecond access** to environment variables
- **Cross-process sharing** - All processes see same data
- **Automatic versioning** - Detect updates instantly
- **System-level isolation** - Secure memory segments

## 🚀 Why Rust?

| Metric | Node.js CLI | **Rust CLI** | Improvement |
|--------|-------------|--------------|-------------|
| **Binary Size** | ~50 MB | **5.1 MB** | **10x smaller** |
| **Startup Time** | 200ms | **~5ms** | **40x faster** |
| **Memory Usage** | 80 MB | **~3 MB** | **27x less** |
| **Pull 100 vars** | 800ms | **50ms** | **16x faster** |
| **Run command** | 150ms | **2ms** | **75x faster** |

The Rust version provides **dramatic performance improvements** while adding powerful new features like shared memory and hot reload.

## 📦 Installation

### Via Cargo (Recommended)

```bash
cargo install envsafe-cli
```

### From Source

```bash
git clone https://github.com/Ifiboys/envsafe-cli-rust.git
cd envsafe-cli-rust
cargo build --release
cargo install --path .
```

### Verify Installation

```bash
envsafe --version
# envsafe 0.1.0
```

## 🏃 Quick Start

### 1. Login

```bash
envsafe login
```

This will open your browser to generate an API token. Alternatively:

```bash
envsafe login --token YOUR_TOKEN
```

### 2. Link a Workspace

```bash
envsafe link
# Or specify workspace ID
envsafe link workspace_123
```

### 3. Initialize Project

```bash
cd your-project
envsafe init
```

Interactive prompts will help you select a project.

### 4. Pull Environment Variables

```bash
# Development environment
envsafe pull --dev

# Production environment
envsafe pull --prod

# Custom environment
envsafe pull --env staging
```

### 5. Run Your Application

```bash
# Inject variables and run
envsafe run --dev -- npm start

# Variables are loaded from shared memory (ultra-fast!)
```

### 6. 🔥 Enable Hot Reload

```bash
envsafe watch --dev
```

Now any changes on the EnvSafe dashboard will **automatically** update your local `.env` and shared memory!

## 📖 Usage

### Commands

| Command | Description |
|---------|-------------|
| `login` | Authenticate with API token |
| `whoami` | Display current user info |
| `link [workspace]` | Link workspace to current directory |
| `init` | Interactive project setup |
| `list` | List projects in current workspace |
| `select <project>` | Select a project |
| `create [name]` | Create new project |
| `projects` | List all accessible projects |
| `pull [project]` | Download environment variables |
| `push [project]` | Upload environment variables |
| `run [project] <command>` | Execute command with injected variables |
| `watch [project]` | Enable real-time hot reload |
| `rotate` | Manage secret rotation |
| `config` | Manage CLI configuration |
| `lang [language]` | Change language (en, fr) |
| `logout` | Logout and clear credentials |

### Environment Shortcuts

```bash
# Instead of --env development
--dev, -d

# Instead of --env staging  
--staging, -s

# Instead of --env production
--prod, -p
```

### Examples

**Team Collaboration with Hot Reload:**
```bash
# Developer A starts watch mode
envsafe watch --dev

# Developer B changes a variable in dashboard
# → Developer A's .env updates automatically!
```

**Docker with Secret Rotation:**
```bash
# Enable rotation every 7 days
envsafe rotate enable --interval 7

# Deploy with auto-updated secrets
docker-compose up
```

See [EXAMPLES.md](EXAMPLES.md) for more detailed scenarios.

## 🐳 Docker Integration

### Basic Usage

```dockerfile
FROM rust:1.75 as builder
RUN cargo install envsafe-cli

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/envsafe /usr/local/bin/
ENTRYPOINT ["envsafe", "run", "--prod", "--"]
CMD ["node", "index.js"]
```

### With Hot Reload & Shared Memory

```yaml
version: '3.8'
services:
  app:
    image: your-app
    environment:
      - ENVSAFE_TOKEN=${ENVSAFE_TOKEN}
      - ENVSAFE_PROJECT_ID=${PROJECT_ID}
      - ENVSAFE_WATCH=true
    volumes:
      - envsafe-memory:/dev/shm

volumes:
  envsafe-memory:
    driver: local
    driver_opts:
      type: tmpfs
      device: tmpfs
      o: size=10m
```

See [docker/](docker/) for complete examples.

## 🛠️ Development

### Prerequisites

- Rust 1.75+ ([Install](https://rustup.rs/))
- Cargo (comes with Rust)

### Setup

```bash
# Clone repository
git clone https://github.com/Ifiboys/envsafe-cli-rust.git
cd envsafe-cli-rust

# Copy environment example
cp .env.example .env
# Edit .env with your test credentials

# Build
cargo build

# Run tests
cargo test

# Run CLI in development
cargo run -- --help
```

### Quick Commands

```bash
# Fast build
./quick.sh build

# Run tests
./quick.sh test

# Lint code
./quick.sh lint

# Format code
./quick.sh format

# All checks before PR
cargo test && cargo clippy -- -D warnings && cargo fmt --check
```

### Project Structure

```
src/
├── main.rs           # CLI entry point
├── config.rs         # Configuration management
├── api.rs            # EnvSafe API client
├── storage.rs        # Shared memory IPC
├── watcher.rs        # Hot reload (WebSocket + file watching)
├── rotation.rs       # Secret rotation logic
├── commands/         # CLI commands (15 modules)
└── utils/            # Utilities (i18n, parsers)
```

## 🤝 Contributing

We love contributions! **EnvSafe CLI is an open source project** and we welcome collaborators from around the world! 🌍

### How to Contribute

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes**
4. **Run tests and linters** (`cargo test && cargo clippy`)
5. **Commit your changes** (`git commit -m 'feat: add amazing feature'`)
6. **Push to branch** (`git push origin feature/amazing-feature`)
7. **Open a Pull Request**

### Contribution Guidelines

We welcome contributions! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on how to report bugs, suggest enhancements, and submit pull requests.

### Development Setup

```bash
# Clone repository
git clone https://github.com/Ifiboys/envsafe-cli-rust.git
cd envsafe-cli-rust

# Build project
cargo build

# Run tests
cargo test
```

## 🏗️ Architecture

Built with modern Rust best practices:

- **Async Runtime**: Tokio for high-performance async operations
- **CLI Framework**: Clap for robust command parsing
- **HTTP Client**: Reqwest for API communications
- **WebSocket**: Tokio-tungstenite for real-time updates
- **IPC**: Shared memory for cross-process communication
- **Security**: AES-256-GCM encryption, SHA-256 hashing



## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [EnvSafe Team](https://www.envsafe.dev) - Original platform
- [Tokio](https://tokio.rs/) - Async runtime
- [Clap](https://github.com/clap-rs/clap) - CLI framework
- All our [contributors](https://github.com/Ifiboys/envsafe-cli-rust/graphs/contributors) 🎉

## 📞 Support

- 📧 **Email**: info@envsafe.dev
- 🐛 **Issues**: [GitHub Issues](https://github.com/Ifiboys/envsafe-cli-rust/issues)
- 📖 **Docs**: [docs.envsafe.dev](https://docs.envsafe.dev)

## 🌟 Show Your Support

If you find this project useful, please consider:
- ⭐ **Starring the repository**
- 📢 **Sharing with your team**
- 🤝 **Contributing**
- 💬 **Reporting bugs or requesting features**

---

<div align="center">

**Made with ❤️ and 🦀 Rust**

[Website](https://www.envsafe.dev) • [GitHub](https://github.com/Ifiboys)

</div>
