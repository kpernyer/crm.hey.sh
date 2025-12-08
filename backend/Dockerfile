# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# Copy proto files
COPY ../proto ../proto

# Create dummy main to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source
COPY src ./src
COPY schema ./schema

# Build actual binary
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/crm-server /app/crm-server
COPY --from=builder /app/schema /app/schema

# Set environment
ENV RUST_LOG=info
ENV SERVER_PORT=8080

EXPOSE 8080

CMD ["/app/crm-server"]
