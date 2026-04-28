# :whale: Docker Integration

EnvSafe CLI can be seamlessly integrated into your Docker workflows for secure, automated environment variable management.

## Minimal Integration

Use the EnvSafe CLI to inject variables at runtime without baking them into the image.

**Dockerfile:**

```dockerfile
FROM rust:1.75 as builder
RUN cargo install envsafe-cli

FROM debian:bookworm-slim
# Install dependencies needed for SSL
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/envsafe /usr/local/bin/

# Use envsafe to fetch secrets at runtime
ENTRYPOINT ["envsafe", "run", "--prod", "--"]
CMD ["node", "index.js"]
```

## Docker Compose with Hot Reload

For development with hot reload, mount a shared memory volume so the CLI can update variables in real-time without restarting the container.

**docker-compose.yml:**

```yaml
version: '3.8'

services:
  app:
    build: .
    environment:
      - ENVSAFE_TOKEN=${ENVSAFE_TOKEN} 
      - ENVSAFE_PROJECT_ID=${PROJECT_ID}
      - ENVSAFE_WATCH=true
    volumes:
      # Shared memory for IPC
      - envsafe-memory:/dev/shm
      # Local source code (for hot reload of Node/Python app)
      - .:/app
    command: npm run dev

volumes:
  envsafe-memory:
    driver: local
    driver_opts:
      type: tmpfs
      device: tmpfs
      o: size=10m
```

## Production Best Practices

- **Avoid .env files in images**: Never copy `.env` files into your Docker image.
- **Use Multi-Stage Builds**: Install `envsafe` in a builder stage to keep the final image small.
- **Inject Secrets at Runtime**: Using `envsafe run` ensures that secrets are fetched fresh on container start.
- **Rotate Secrets**: Configure automatic rotation for long-lived containers.

## Healthchecks

You can use `envsafe config --show` as a healthcheck to ensure connectivity to the EnvSafe API.

```yaml
healthcheck:
  test: ["CMD", "envsafe", "config", "--show"]
  interval: 1m
  timeout: 10s
  retries: 3
```
