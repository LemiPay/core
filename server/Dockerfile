FROM rust:latest

WORKDIR /server

COPY . .

RUN cargo build --release

EXPOSE 3000

CMD ["./target/release/server"]