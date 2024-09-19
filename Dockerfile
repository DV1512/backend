# Build stage
FROM rust:1.80-slim AS builder

RUN rustup default nightly

# Build arguments with default values
ARG FEATURES
ARG BUILD_ARGS

# Set the working directory
WORKDIR /usr/src/app

# Copy Cargo files to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Copy the actual source code
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y ca-certificates pkg-config libssl-dev clang

# Build the actual application
RUN cargo build --release ${BUILD_ARGS} $(if [ -n "$FEATURES" ]; then echo "--features $FEATURES"; fi)

# Runtime stage using a minimal base image with ca-certificates
FROM debian:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/backend .

# Copy necessary directories
COPY events events
COPY migrations migrations
COPY schemas schemas
COPY .env.local .env.local
COPY .env.production .env.production

# Expose the port (default to 8080)
ENV PORT=9999
EXPOSE $PORT

# Set the entrypoint to the application binary
CMD ["./backend"]
