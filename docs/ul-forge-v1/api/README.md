# API Service

> HTTP REST API and WebSocket service for parsing, rendering, and validating UL.

---

## Overview

The UL Forge API is an optional HTTP service wrapping the core library. It enables:
- **Web editor** — browser-based glyph composition
- **External integrations** — any HTTP client can parse/render/validate
- **Live preview** — WebSocket connection for real-time rendering as users type

The API server is built with the same Rust core compiled as a native binary (not WASM). For browser-only deployments, the WASM build provides the same functionality without a server.

---

## Architecture

```
Browser / Client
      │
      ├── REST  ──→  POST /parse, POST /render, POST /validate
      │
      └── WS   ──→  ws://host/live  (bidirectional streaming)
      │
API Server (Rust — Axum)
      │
      └── ul-forge-core library
```

---

## Quick Start

```bash
# Start the API server
ul serve --port 3000

# Or with Docker
docker run -p 3000:3000 ulforge/api:v1
```

---

## Endpoints Summary

| Method | Path | Description | See |
|--------|------|-------------|-----|
| POST | `/parse` | UL-Script → GIR JSON | [endpoints.md](endpoints.md) |
| POST | `/render` | GIR JSON → SVG | [endpoints.md](endpoints.md) |
| POST | `/validate` | GIR JSON → validation result | [endpoints.md](endpoints.md) |
| POST | `/convert` | Any format → any format | [endpoints.md](endpoints.md) |
| GET | `/health` | Health check | [endpoints.md](endpoints.md) |
| WS | `/live` | Bidirectional live preview | [websocket.md](websocket.md) |

---

## Authentication

v1 ships with no authentication. The API is intended for local or trusted-network use:
- Local development (localhost only)
- Internal microservice deployment
- Behind a reverse proxy (nginx, Caddy) that handles auth

If exposed publicly, deploy behind an authentication proxy.

---

## CORS

The server sets permissive CORS headers for local development:

```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

In production, restrict `Allow-Origin` to your domain.

---

## Rate Limiting

No built-in rate limiting in v1. Use a reverse proxy for production rate limiting.

---

## Error Format

All error responses follow a consistent JSON format:

```json
{
  "error": {
    "code": "PARSE_ERROR",
    "message": "Unexpected token at position 15",
    "details": {
      "position": 15,
      "line": 1,
      "column": 16,
      "found": "!",
      "expected": ["→", ")", "}"]
    }
  }
}
```

HTTP status codes:
- `200` — success
- `400` — client error (invalid input, parse failure, validation failure)
- `500` — server error (bug)
