FROM rust:1.61 AS builder
COPY . /build
WORKDIR /build
RUN cargo build --release --workspace

FROM debian:11
# RUN apt-get update && apt-get install -y libssl-dev
COPY --from=builder /build/target/release/tyorka-shop /build/target/release/migration /usr/local/bin/

EXPOSE 3001

CMD ["tyorka-shop"]