# Dockerfile
FROM rust:1.81 as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy ALL source files at once (not in layers)
COPY backend/ ./

# Clean any existing build artifacts
RUN cargo clean

# Build the application in one step
RUN cargo build --release

# Verify the binary exists
RUN ls -la target/release/backend && file target/release/backend

# Production stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/backend ./backend

# Copy migrations
COPY backend/migrations ./migrations/

# Make sure binary is executable
RUN chmod +x ./backend && ls -la ./backend

# Expose port
EXPOSE 8000

# Run the application
CMD ["./backend"]