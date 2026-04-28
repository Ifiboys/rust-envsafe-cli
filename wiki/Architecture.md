# :building_construction: Architecture & Security

EnvSafe CLI is designed with security and performance as core principles.

## 🔒 Security Model

### Authentication

- API tokens are **never** stored in plain text.
- Tokens are encrypted at rest using platform-specific secure storage (Keychain on macOS, Credential Manager on Windows, Secret Service API on Linux).
- Short-lived access tokens are used for API calls.

### Variable Encryption

- Environment variables downloaded from EnvSafe are encrypted in transit via TLS 1.3.
- When persisted to disk (`.env`), they are plain text (standard behavior).
- Use `envsafe run` to inject variables strictly in memory without writing to disk.

### Shared Memory (IPC)

EnvSafe uses a **Shared Memory Segment (`/dev/shm`)** to store environment variables for ultra-fast access across processes.

- **Isolation:** The memory segment is created with restricted permissions (0600), accessible only by the user running the CLI.
- **Speed:** Reading variables from shared memory takes **nanoseconds**, eliminating file I/O overhead.
- **Hot Reload:** When variables change on the dashboard, the CLI updates the shared memory segment. Every connected process immediately sees the new values.

## ⚡ Performance: Rust vs Node.js

We rewrote the original Node.js CLI in Rust to achieve significant gains:

| Metric | Node.js CLI | **Rust CLI** | Improvement |
|--------|-------------|--------------|-------------|
| **Binary Size** | ~50 MB | **5.1 MB** | **10x smaller** |
| **Startup Time** | 200ms | **~5ms** | **40x faster** |
| **Memory Usage** | 80 MB | **~3 MB** | **27x less** |
| **Pull 100 vars** | 800ms | **50ms** | **16x faster** |
| **Run command** | 150ms | **2ms** | **75x faster** |

### Why Rust?

- **Zero-cost abstractions:** High-level code compiles to efficient machine code.
- **Memory safety:** No garbage collector pauses, predictable latency.
- **Single binary:** Easy distribution, no runtime dependencies (Node.js/Python).
- **Thread safety:** Safe concurrency for parallel downloads and WebSocket handling.

## 🔄 Internal Workflow

1.  **Authentication**: CLI checks for token in secure storage.
2.  **Command Parsing**: `clap` parses arguments.
3.  **API Call**: `reqwest` fetches encrypted data.
4.  **Decryption**: Data is decrypted in memory.
5.  **Action**:
    - `pull`: Write to `.env` file.
    - `run`: Inject into child process environment.
    - `watch`: Upgrades connection to WebSocket (WSS) and listens for events.
