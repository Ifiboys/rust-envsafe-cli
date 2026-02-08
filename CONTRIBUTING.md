# Contributing to EnvSafe CLI

First off, thanks for taking the time to contribute! 🎉

The following is a set of guidelines for contributing to EnvSafe CLI (Rust). These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## Code of Conduct

This project and everyone participating in it is governed by a Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to [info@envsafe.dev](mailto:info@envsafe.dev).

## How Can I Contribute?

### Reporting Bugs

This section guides you through submitting a bug report. Following these guidelines helps maintainers and the community understand your report, reproduce the behavior, and find related reports.

- **Use a clear and descriptive title** for the issue to identify the problem.
- **Describe the exact steps to reproduce the problem** in as much detail as possible.
- **Provide specific examples** to demonstrate the steps.
- **Describe the behavior you observed** after following the steps and point out what exactly is the problem with that behavior.
- **Explain which behavior you expected to see instead and why.**

### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion, including completely new features and minor improvements to existing functionality.

- **Use a clear and descriptive title** for the issue to identify the suggestion.
- **Provide a step-by-step description of the suggested enhancement** in as much detail as possible.
- **Explain why this enhancement would be useful** to most EnvSafe users.

### Pull Requests

The process described here has several goals:

- Maintain EnvSafe's quality
- Fix problems that are important to users
- Engage the community in working toward the best possible EnvSafe
- Enable a sustainable system for EnvSafe's maintainers to review contributions

Please follow these steps:

1.  **Fork** the repository on GitHub.
2.  **Clone** your fork locally.
3.  **Create a branch** for your feature or bugfix (`git checkout -b feature/amazing-feature`).
4.  **Make your changes**.
5.  **Run tests** to ensure no regressions (`cargo test`).
6.  **Format your code** using `cargo fmt` and `cargo clippy`.
7.  **Commit** your changes (`git commit -m 'Add amazing feature'`).
8.  **Push** to your branch (`git push origin feature/amazing-feature`).
9.  **Open a Pull Request**.

## Styleguides

### Git Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

### Rust Styleguide

- Use `cargo fmt` to format your code.
- Use `cargo clippy` to check for common mistakes and improve your code.
- Write documentation for public APIs using `///` comments.
- Write tests for new features and bug fixes.

## Development Setup

```bash
# Clone repository
git clone https://github.com/Ifiboys/envsafe-cli-rust.git
cd envsafe-cli-rust

# Build project
cargo build

# Run tests
cargo test
```

## Need Help?

If you have any questions, feel free to reach out to us at [info@envsafe.dev](mailto:info@envsafe.dev) or open an issue on GitHub.
