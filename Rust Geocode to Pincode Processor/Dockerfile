# Stage 1: Build the application
FROM rust:1.89 as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libclang-dev \
    clang \
    zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo.toml first to cache dependencies
COPY Cargo.toml ./

# Create dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy actual source code
COPY . .

# Build final binary
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

# Install SSL certificates and other runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/geocode-pincode /app/

# Create directories with proper permissions
RUN mkdir -p input output && \
    chmod 777 input output

# Make the binary executable
RUN chmod +x /app/geocode-pincode