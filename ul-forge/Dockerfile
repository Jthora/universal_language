# UL Forge API Server — Multi-stage Docker build
FROM rust:1.85-slim AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

RUN cargo build --release -p ul-api

# ── Runtime image ──
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/ul-serve /usr/local/bin/ul-serve

EXPOSE 3000

ENTRYPOINT ["ul-serve"]
CMD ["--port", "3000"]
