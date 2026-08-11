# Type 1 — Docker Image: `ghcr.io/jthora/ul-forge`

> Containerized REST API server with self-describing endpoints.

---

## Current State

A `Dockerfile` already exists at `ul-forge/Dockerfile`. It builds the ul-api server. Gaps:

1. **Not published** to any container registry
2. **No OpenAPI / discovery endpoints** (added by api-discovery.md)
3. **No MCP server** included (added by mcp-server.md)
4. **Non-configurable** — hardcoded port, no health check

---

## Target

```bash
# Pull and run
docker run -p 8080:8080 ghcr.io/jthora/ul-forge

# Verify
curl http://localhost:8080/health              # → {"status":"ok"}
curl http://localhost:8080/capabilities         # → OpenAPI 3.1 JSON
curl http://localhost:8080/schema/gir           # → GIR JSON Schema
```

---

## Dockerfile Updates

```dockerfile
FROM rust:1.82-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p ul-api -p ul-mcp -p ul-cli

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ul-api /usr/local/bin/
COPY --from=builder /app/target/release/ul-mcp /usr/local/bin/
COPY --from=builder /app/target/release/ul-cli /usr/local/bin/ul-forge
COPY schemas/ /usr/local/share/ul-forge/schemas/

ENV UL_BIND_ADDR=0.0.0.0:8080
ENV UL_SCHEMAS_DIR=/usr/local/share/ul-forge/schemas

EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s CMD curl -f http://localhost:8080/health || exit 1

# Default: REST API. Override with `ul-mcp` for MCP mode.
CMD ["ul-api"]
```

### Multi-mode Usage

```bash
# REST API (default)
docker run -p 8080:8080 ghcr.io/jthora/ul-forge

# MCP server (stdio)
docker run -i ghcr.io/jthora/ul-forge ul-mcp

# CLI one-shot
docker run ghcr.io/jthora/ul-forge ul-forge parse "point(existence)"
```

---

## CI: Build and Push

```yaml
# .github/workflows/publish-docker.yml
name: Publish Docker
on:
  release:
    types: [published]
jobs:
  docker:
    runs-on: ubuntu-latest
    permissions:
      packages: write
    steps:
      - uses: actions/checkout@v4
      - uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
      - uses: docker/build-push-action@v5
        with:
          context: ul-forge/
          push: true
          tags: |
            ghcr.io/jthora/ul-forge:latest
            ghcr.io/jthora/ul-forge:${{ github.event.release.tag_name }}
```

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `UL_BIND_ADDR` | `0.0.0.0:8080` | Address and port for the REST API |
| `UL_CORS_ORIGIN` | `*` | Allowed CORS origin |
| `UL_SCHEMAS_DIR` | `/usr/local/share/ul-forge/schemas` | Path to JSON Schema files |
| `UL_MAX_BODY_SIZE` | `1048576` (1MB) | Maximum request body size |
