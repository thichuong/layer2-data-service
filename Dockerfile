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

# Expose gRPC port only
EXPOSE 50051

# Environment variables
ENV RUST_LOG=info \
    RUST_BACKTRACE=1 \
    HOST="0.0.0.0"

# Start the application
CMD ["./layer2_data_service"]
