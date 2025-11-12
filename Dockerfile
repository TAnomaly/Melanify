# Stage 1: Build Frontend
FROM node:18-alpine AS frontend-builder

WORKDIR /frontend

# Copy frontend package files
COPY frontend/package*.json ./

# Install frontend dependencies
RUN npm ci --only=production

# Copy frontend source
COPY frontend/ ./

# Build frontend
RUN npm run build

# Stage 2: Build Backend
FROM rust:1.75-slim as backend-builder

WORKDIR /app

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build backend in release mode
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy backend binary from builder
COPY --from=backend-builder /app/target/release/spotify-ai-playlist /app/spotify-ai-playlist

# Copy frontend build from frontend-builder
COPY --from=frontend-builder /frontend/build /app/frontend/build

# Expose port (Railway will set PORT env var)
EXPOSE 8081

# Run the application
CMD ["/app/spotify-ai-playlist"]
