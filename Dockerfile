FROM rust:1.65.0 as builder
WORKDIR /usr/src/simple-redis-pusher

# Install dependencies and have Docker cache them
RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo fetch

# Copy source code and build
COPY src src
RUN cargo install --path .

# Create a minimal image
FROM gcr.io/distroless/cc-debian10
COPY --from=builder /usr/local/cargo/bin/simple-redis-pusher /usr/local/bin/simple-redis-pusher
CMD ["simple-redis-pusher"]
