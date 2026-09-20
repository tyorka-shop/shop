FROM rust:1.85-bookworm AS builder
COPY . /build
WORKDIR /build
RUN cargo build --release --workspace

FROM debian:trixie-slim
LABEL org.opencontainers.image.source=https://github.com/tyorka-shop/shop
COPY --from=builder /build/target/release/tyorka-shop /build/target/release/migration /usr/local/bin/

EXPOSE 3003

ENTRYPOINT ["/usr/local/bin/tyorka-shop"]