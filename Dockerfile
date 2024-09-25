ARG PROFILE=release
ARG BINARY=backend

# Define common build steps
FROM rust:1.80-slim AS builder-base

RUN rustup default nightly

# Set the working directory
WORKDIR /usr/src/app

# Copy Cargo files to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

COPY src ./src

# Install build dependencies
RUN apt-get update && apt-get install -y ca-certificates pkg-config libssl-dev curl

# Build stage for release
FROM builder-base AS builder-release

# Build arguments with default values
ARG BUILD_ARGS
ARG BINARY
ARG FEATURES

RUN cargo build --release ${BUILD_ARGS} $(if [ -n "$FEATURES" ]; then echo "--features $FEATURES"; fi);

RUN cp /usr/src/app/target/release/${BINARY} .

# Build stage for development
FROM builder-base AS builder-dev

ARG BUILD_ARGS
ARG BINARY
ARG FEATURES

RUN cargo build ${BUILD_ARGS} $(if [ -n "$FEATURES" ]; then echo "--features $FEATURES"; fi);

RUN cp /usr/src/app/target/debug/${BINARY} .

# final build stage
FROM builder-${PROFILE} AS builder

# Runtime stage - modify this to fit the application
FROM debian:stable-slim AS runtime

ARG BINARY

# Installs the required OpenSSL shared library file "libssl.so.3"
RUN apt-get update && apt-get install -y libssl3 curl

# Set the working directory
WORKDIR /usr/src/app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/${BINARY} ./app

# Copy necessary directories
COPY events events
COPY migrations migrations
COPY schemas schemas
COPY .env.local .env.local
COPY .env.production .env.production

# Expose the port (default to 8080)
ENV PORT=9999
EXPOSE $PORT

HEALTHCHECK --timeout=10s --retries=5 --start-period=30s \
    CMD curl -sf http://localhost:$PORT/health || exit 1

# Set the entrypoint to the application binary
CMD ["./app"]
