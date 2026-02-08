<div align="center">

# 🔐 EnvSafe CLI (Rust)

**Blazing fast, secure environment variable manager with real-time hot reload**

[![Build Status](https://github.com/Ifiboys/rust-envsafe-cli/workflows/Build%20and%20Test/badge.svg)](https://github.com/Ifiboys/rust-envsafe-cli/actions)
[![npm](https://img.shields.io/npm/v/@envsafes-org/cli.svg)](https://www.npmjs.com/package/@envsafes-org/cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Wiki Documentation](https://github.com/Ifiboys/rust-envsafe-cli/wiki)

</div>

---

## 📖 Documentation

Full documentation is available in our **[GitHub Wiki](https://github.com/Ifiboys/rust-envsafe-cli/wiki)**:

- **[Installation Guide](https://github.com/Ifiboys/rust-envsafe-cli/wiki/Installation)**
- **[Detailed Usage](https://github.com/Ifiboys/rust-envsafe-cli/wiki/Usage)**
- **[Docker Integration](https://github.com/Ifiboys/rust-envsafe-cli/wiki/Docker-Integration)**
- **[Architecture & Security](https://github.com/Ifiboys/rust-envsafe-cli/wiki/Architecture)**

---

## ✨ Features

- **🔄 Real-time hot reload** via WebSocket
- **💾 Shared memory IPC** for sub-millisecond access
- **♻️ Automatic secret rotation**
- **⚡ Blazing performance** (Rust-based)
- **🔒 Secure encryption** for all variables

## 🚀 Why Rust?

| Metric | Node.js CLI | **Rust CLI** | Improved |
|--------|-------------|--------------|----------|
| **Startup** | 200ms | **~5ms** | **40x** |
| **Pull 100 vars** | 800ms | **50ms** | **16x** |

## 📦 Installation

**Via npm (Recommended)**
```bash
npm install -g @envsafes-org/cli
```

**Binary**
Download from [Releases](https://github.com/Ifiboys/rust-envsafe-cli/releases).

## 🏃 Quick Start

1.  **Login**
    ```bash
    envsafe login
    ```

2.  **Initialize Project**
    ```bash
    envsafe init
    ```

3.  **Pull Variables**
    ```bash
    envsafe pull --dev
    ```

4.  **Run with Secrets**
    ```bash
    envsafe run --dev -- npm start
    ```

## 🤝 Contributing

See [Development Guide](https://github.com/Ifiboys/rust-envsafe-cli/wiki/Development).

## 📄 License

MIT License - see [LICENSE](LICENSE) file.
