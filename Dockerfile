FROM rust:1.61 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/tyorka-shop ./app
EXPOSE 3001
CMD ["app"]