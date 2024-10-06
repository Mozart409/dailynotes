FROM rust:1.74-slim AS builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/assets/static /usr/app/assets/static
COPY --from=builder /usr/src/assets/static/404.html /usr/app/assets/static/404.html
COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/dailynotes-cli /usr/app/dailynotes-cli

ENTRYPOINT ["/usr/app/dailynotes-cli"]
