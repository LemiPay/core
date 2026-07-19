# Build
FROM rust:1.94-slim AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y \
    libpq-dev pkg-config musl-tools \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

COPY . .
RUN touch src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime (muy liviano)
FROM alpine:3.20
RUN apk add --no-cache ca-certificates libpq
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /usr/local/bin/app

ENV PORT=3000
EXPOSE 3000
CMD ["app"]