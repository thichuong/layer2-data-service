# Railway-optimized Dockerfile - Single stage build
FROM rust:1.83-bookworm

# Install build dependencies (including protobuf for gRPC)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy project files
COPY . .

# Build the application
RUN cargo build --release

# Copy binary to working directory and make executable
RUN cp ./target/release/layer2_data_service ./layer2_data_service && \
    chmod +x ./layer2_data_service

# Setup runtime user
RUN useradd -ms /bin/bash appuser && \
    chown -R appuser:appuser /app

USER appuser

# Expose single port for gRPC (includes health check)
# Railway auto-maps PORT for public access
EXPOSE 8001

# Environment variables
ENV RUST_LOG=info \
    RUST_BACKTRACE=1 \
    HOST="0.0.0.0"
# Railway automatically sets PORT env var
# Service exposes:
# - grpc.health.v1.Health (gRPC health check protocol)
# - MarketDataService (market data gRPC API)

# Start the application
CMD ["./layer2_data_service"]
