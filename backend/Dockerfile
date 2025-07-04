# Build stage
FROM rust:1.80 as builder

WORKDIR /app

# Copy manifests first for better caching
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy actual source code
COPY src ./src
COPY migrations ./migrations

# Build the application (skip SQLx compile-time checks)
ENV SQLX_OFFLINE=true
RUN touch sqlx-data.json && echo '{"db":"PostgreSQL","queries":[]}' > sqlx-data.json
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/spotify-clone-backend /app/spotify-clone-backend

# Copy migrations
COPY --from=builder /app/migrations /app/migrations

# Create uploads directory
RUN mkdir -p /app/uploads

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./spotify-clone-backend"]
