FROM rust:latest as builder

RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /rust/gravity
COPY . .
RUN cargo test

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl


FROM alpine:latest as release
COPY --from=builder /rust/gravity/target/x86_64-unknown-linux-musl/release/rust-playground /rust/gravity/rust-playground
WORKDIR /rust/gravity

ENTRYPOINT ["./rust-playground"]