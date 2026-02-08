# :recycle: Automatic Secret Rotation

EnvSafe CLI can automatically rotate secrets (e.g., database passwords, API keys) at regular intervals or on demand.

## ⚙️ Configuration

**Enable Rotation**

To enable rotation on a project, specify the rotation interval in days:

```bash
envsafe rotate enable
```

Default interval is **30 days**.

**Custom Interval**

To rotate every 7 days:

```bash
envsafe rotate enable --interval 7
```

**Exclude Variables**

Prevent specific variables from being rotated:

```bash
envsafe rotate enable --exclude DATABASE_URL
```
or multiple:
```bash
envsafe rotate enable --exclude API_KEY --exclude JWT_SECRET
```

## :rocket: Force Rotation (On Demand)

To immediately rotate a secret (e.g., after a leak), use `rotate now`:

```bash
envsafe rotate now --vars AWS_ACCESS_KEY
```

This will:
1.  Generate a new value (using cryptographically secure random bytes).
2.  Update the variable in EnvSafe API.
3.  Trigger a hot reload for all connected clients.

## 🐳 Docker Workflow

For Docker containers, you can use the `envsafe monitor` process (via `envsafe run`) which listens for rotation events.

When a secret is rotated:
1.  The EnvSafe backend notifies the CLI via WebSocket.
2.  The CLI updates the environment variable in memory.
3.  If configured, the CLI can execute a reload command or simply expose the new value to the application.

This ensures zero-downtime secret rotation.
