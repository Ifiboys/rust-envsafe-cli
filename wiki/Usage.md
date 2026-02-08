# :book: Detailed Usage Guide

EnvSafe CLI provides a rich set of commands for managing your project's environment variables. This guide covers all available commands and options.

## 🔑 Authentication

**Login with API Token**

You must retrieve your API token from the [EnvSafe Dashboard](https://www.envsafe.dev/tokens).

```bash
envsafe login
```

If the automated browser login doesn't work, manually provide the token:

```bash
envsafe login --token <YOUR_API_TOKEN>
```

**Verify Login**

```bash
envsafe whoami
```

**Logout**

```bash
envsafe logout
```

## :rocket: Project Workflow

**Initialize a Project**

Run `init` in your project root to interactively select a project:

```bash
envsafe init
```

This creates a local `.envsafe` config file that links the directory to the project.

**Link a Workspace (Optional)**

If you work with multiple workspaces, you can link the current directory to a specific workspace:

```bash
envsafe link <workspace_id>
```

**List Projects**

```bash
envsafe list
```

## :inbox_tray: Pull Variables (Download)

**Development Environment**

```bash
envsafe pull --dev
# or
envsafe pull --env development
```

**Production Environment**

```bash
envsafe pull --prod
# or
envsafe pull --env production
```

**Staging Environment**

```bash
envsafe pull --staging
# or
envsafe pull --env staging
```

**Options:**

- `--output .env.local`: Specify output file.
- `--format json`: Output variables in JSON format.

## :outbox_tray: Push Variables (Upload)

Upload local variables to EnvSafe.

**Development Environment**

```bash
envsafe push --dev --file .env
```

**Production Environment**

```bash
envsafe push --prod --file .env.prod
```

## :zap: Run Commands (No .env file needed)

Inject environment variables directly into a command without creating a `.env` file!

```bash
# Run node script with dev variables
envsafe run --dev -- node starting-script.js

# Run tests with staging variables
envsafe run --staging -- npm test
```

This is ideal for CI/CD pipelines where you don't want to persist secrets on disk.

## :fire: Hot Reload (Watch Mode)

Start real-time monitoring of environment variables. When a variable is changed in the EnvSafe dashboard, your local process receives the update instantly via WebSocket.

```bash
envsafe watch --dev
```

This command:
1.  Connects to EnvSafe WebSocket.
2.  Updates the local `.env` file on change.
3.  Updates the shared memory segment for ultra-fast access.

## :repeat: Secret Rotation

Configure automatic rotation for sensitive secrets (API keys, JWT secrets).

**Enable Rotation**

```bash
envsafe rotate enable --interval 30 --exclude DATABASE_URL
```

**Force Immediate Rotation**

```bash
envsafe rotate now --vars AWS_ACCESS_KEY
```

## :gear: Configuration

**Show Current Configuration**

```bash
envsafe config --show
```

**Change Language**

```bash
envsafe lang <en|fr>
```
