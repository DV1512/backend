# Build stage
FROM rust:1.80-slim AS builder

RUN rustup default nightly

# Build arguments with default values
ARG APP_NAME=backend
ARG FEATURES
ARG BUILD_ARGS

# Set the working directory
WORKDIR /usr/src/app

# Copy Cargo files to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build and cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Install build dependencies
RUN apt-get update && apt-get install -y ca-certificates pkg-config libssl-dev clang

# Build dependencies only (handle empty FEATURES and BUILD_ARGS)
RUN cargo build --release ${BUILD_ARGS} $(if [ -n "$FEATURES" ]; then echo "--features $FEATURES"; fi)

# Remove the dummy main.rs
RUN rm -f src/main.rs

# Copy the actual source code
COPY . .

# Build the actual application
RUN cargo build --release ${BUILD_ARGS} $(if [ -n "$FEATURES" ]; then echo "--features $FEATURES"; fi)

# Runtime stage using a minimal base image with ca-certificates
FROM gcr.io/distroless/base

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/${APP_NAME} .

# Copy necessary directories
COPY events events
COPY migrations migrations
COPY schemas schemas

# Expose the port (default to 8080)
ENV PORT=9999
EXPOSE $PORT

# Set the entrypoint to the application binary
CMD ["./${APP_NAME}"]
