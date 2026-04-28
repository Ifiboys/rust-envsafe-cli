# :construction: Development Guide

We welcome contributions! This guide will help you get started with building and testing the EnvSafe CLI.

## Prerequisites

- **Rust:** Install Rust using [rustup.rs](https://rustup.rs).
- **Cargo:** Included with Rust.

## Initial Setup

1.  Clone the repository:

    ```bash
    git clone https://github.com/Ifiboys/rust-envsafe-cli.git
    cd rust-envsafe-cli
    ```

2.  Copy the example environment file:

    ```bash
    cp .env.example .env
    ```

3.  (Optional) Populate `.env` with actual credentials for integration tests.

## Running Locally

To run the CLI in development mode:

```bash
cargo run -- --help
```

For specific commands:

```bash
cargo run -- login
cargo run -- pull --dev
```

## Testing

Run unit tests:

```bash
cargo test
```

Run integration tests (requires valid `.env` config if applicable):

```bash
cargo test --test integration
```

## Code Style

We follow standard Rust formatting guidelines.

Check formatting:

```bash
cargo fmt -- --check
```

Check linting (Clippy):

```bash
cargo clippy -- -D warnings
```

## Release Process

We automate releases with GitHub Actions.

1.  Create a new branch for the release (e.g., `release/v0.2.x`).
2.  Update version in `Cargo.toml`.
3.  Update version in `npm/package.json`.
4.  Commit and push your branch.
5.  Open a **Pull Request** to `main`.
6.  Once CI passes and the PR is merged:
    - Switch to `main` and pull the latest changes.
    - Create and push a git tag (e.g., `v0.2.x`).
7.  GitHub Actions will automatically:
    - Build for all platforms.
    - Create a GitHub Release.
    - Publish to npm.

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.
