#
# Dockerfile for the crates-io-proxy server application
#

### First stage: Build the application itself.
FROM rust:alpine AS builder

WORKDIR /builds/crates-io-proxy

# Copy source data (see .dockerignore for excludes).
COPY . .

# Install the build deps and build the application with cargo.
RUN \
apk add --no-cache musl-dev && \
cargo build --release --features native-certs

### Second stage: Copy the built application into the runtime image.
FROM alpine:latest AS runner

LABEL org.opencontainers.image.title="crates-io-proxy"
LABEL org.opencontainers.image.description="Caching HTTP proxy server for the crates.io registry"
LABEL org.opencontainers.image.source="https://github.com/tsrman/crates-io-proxy"

# Install the compiled executable into the system.
COPY --from=builder /builds/crates-io-proxy/target/release/crates-io-proxy /usr/bin/crates-io-proxy

# Add the proxy service user and create the crate files cache directory writable by it.
RUN \
adduser -SHD -u 777 -h /var/empty -s /sbin/nologin -g "crates.io proxy" cratesioxy && \
mkdir /var/cache/crates-io-proxy && \
chown cratesioxy /var/cache/crates-io-proxy

# Switch to the service user to run the proxy process.
USER cratesioxy
WORKDIR /var/empty

# Expose the default proxy port.
EXPOSE 3080

# Run the proxy server with the default configuration.
CMD ["crates-io-proxy", "--verbose"]
