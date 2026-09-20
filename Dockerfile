FROM rust:1.85-bookworm AS builder
COPY . /build
WORKDIR /build

RUN apt-get update \ 
  && DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends --assume-yes protobuf-compiler

ENV SQLX_OFFLINE=true

RUN cargo build --release --workspace

FROM debian:trixie-slim
LABEL org.opencontainers.image.source=https://github.com/tyorka-shop/shop
COPY --from=builder /build/target/release/tyorka-shop /usr/local/bin/
COPY --from=builder /build/target/release/migration /usr/local/bin/

EXPOSE 3003

ENTRYPOINT ["/usr/local/bin/tyorka-shop"]