# Rusty ^(-_-)^

FROM rust:1.93.0 AS builder

WORKDIR /usr/src/rusty

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release || true

COPY src ./src

RUN touch src/main.rs && cargo build --release --bin rusty

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    wget \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/rusty/target/release/rusty /usr/local/bin/rusty

EXPOSE 3030

HEALTHCHECK --interval=10s --timeout=5s --start-period=10s --retries=3 \
    CMD wget -q -O /dev/null http://127.0.0.1:3030/health/live || exit 1

ENTRYPOINT ["rusty"]
