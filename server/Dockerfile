# Build
FROM rust:1.94-slim AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y \
    libpq-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo.toml and Cargo.lock first to leverage Docker cache
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN touch src/main.rs
RUN cargo build --release

# Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /usr/local/bin/app

ENV PORT=3000
EXPOSE 3000
CMD ["app"]