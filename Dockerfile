# Multi-stage build for Layer2 Data Service (gRPC + HTTP)
# Optimized for Railway deployment

# Stage 1: Build
FROM rust:1.75-slim as builder

# Install build dependencies including protobuf compiler
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# Copy proto files (needed for gRPC)
RUN mkdir -p proto
COPY proto/ ./proto/

# Create dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release || true

# Remove dummy files
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Verify binary exists
RUN ls -lh /app/target/release/layer2_data_service

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/layer2_data_service /app/layer2_data_service

# Make binary executable
RUN chmod +x /app/layer2_data_service

# Railway sets PORT environment variable (we use SERVICE_PORT internally)
# Default to 8001 for HTTP, gRPC will use 50051
ENV SERVICE_PORT=8001
ENV HOST=0.0.0.0
ENV RUST_LOG=info,layer2_data_service=debug

# Expose ports (Railway will map these)
EXPOSE 8001 50051

# Health check on HTTP port
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:${SERVICE_PORT}/health || exit 1

# Run the application
CMD ["/app/layer2_data_service"]
