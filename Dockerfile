# Layer2 Data Service Dockerfile
# Multi-stage build for optimal image size

# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install required dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 app

# Copy binary from builder
COPY --from=builder /app/target/release/layer2_data_service /usr/local/bin/layer2_data_service

# Set user
USER app

# Expose port
EXPOSE 8001

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/layer2_data_service"]
