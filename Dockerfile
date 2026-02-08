# Build stage
FROM rust:latest AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src

# Build release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/envsafe /usr/local/bin/envsafe

# Create shared memory mount point
RUN mkdir -p /dev/shm/envsafe

# Copy entrypoint script
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Set environment variables
ENV ENVSAFE_API_URL=https://www.envsafe.dev
ENV ENVSAFE_WATCH=true

ENTRYPOINT ["/entrypoint.sh"]
CMD ["npm", "start"]
