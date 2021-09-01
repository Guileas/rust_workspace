FROM rust:1.54-alpine

RUN apk --no-cache --update add bash curl musl-dev musl openssl-dev openssl libgcc

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN  curl -y --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

WORKDIR /rust
