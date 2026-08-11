# Type 1 — AI Agent Self-Service Package

> A self-contained, zero-config package that an AI agent can find, download, and deploy autonomously.

**Status:** Planning  
**Prerequisites:** P1 (SDK extraction), P3 (JSON Schema)  
**Wave:** 2 (npm/PyPI/Docker) + 3 (MCP server)

---

## Goal

An AI agent — whether an LLM tool-chain, autonomous coding agent, or MCP-compatible client — should be able to:

1. **Discover** UL Forge through repository metadata (AGENTS.md, llms.txt, npm registry)
2. **Install** via its preferred channel (npm, pip, docker, binary download)
3. **Use** the full UL pipeline (parse, validate, render, deparse) immediately
4. **Self-describe** — the package tells the agent what it can do, what inputs it expects, what outputs it produces

No human assistance required at any step.

---

## Distribution Channels

| Channel | Package Name | Spec Doc | Contents |
|---------|-------------|----------|----------|
| **npm** | `@ul-forge/core` | [npm-package.md](npm-package.md) | WASM + TypeScript SDK + GIR Schema |
| **PyPI** | `ul-forge` | [python-package.md](python-package.md) | PyO3 native extension |
| **Docker** | `ghcr.io/jthora/ul-forge` | [docker-image.md](docker-image.md) | REST API server with OpenAPI |
| **Binary** | GitHub Releases | (CI artifact) | Standalone CLI for macOS/Linux/Windows |
| **MCP** | `ul-mcp` | [mcp-server.md](mcp-server.md) | Model Context Protocol tool server |

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│                  AI Agent                         │
│                                                   │
│  Discovery:                                       │
│    AGENTS.md → links to @ul-forge/core            │
│    llms.txt  → machine-readable capability list   │
│    npm info  → package metadata + README          │
│                                                   │
│  Integration (pick one):                          │
│    npm install @ul-forge/core   → WASM in-process │
│    pip install ul-forge         → Python native   │
│    docker run ul-forge          → REST API        │
│    ./ul-forge-cli parse <file>  → CLI subprocess  │
│    MCP server                   → tool protocol   │
│                                                   │
│  Self-description:                                │
│    GET /capabilities            → OpenAPI 3.1     │
│    GET /schema/gir              → GIR JSON Schema │
│    GET /.well-known/ai-plugin   → Plugin manifest │
└─────────────────────────────────────────────────┘
```

---

## Agent Discovery Enhancements

### AGENTS.md Updates

Add a section to the repository's AGENTS.md linking to distribution channels:

```yaml
distribution:
  npm: "@ul-forge/core"
  pypi: "ul-forge"
  docker: "ghcr.io/jthora/ul-forge"
  mcp: "ul-mcp"
  cli: "https://github.com/Jthora/universal_language/releases"
```

### llms.txt Updates

Add machine-readable capability advertisements:

```
# UL Forge — Available Operations
parse: UL-Script text → GIR (Graph Intermediate Representation)
validate: GIR → {valid, errors, warnings}
render: GIR → SVG or TikZ string
deparse: GIR → UL-Script text
schema: GIR JSON Schema for document validation
```

---

## API Discovery Endpoints

See [api-discovery.md](api-discovery.md) for full specification.

The existing ul-api REST server gets 3 new discovery endpoints:

| Endpoint | Returns | Used By |
|----------|---------|---------|
| `GET /capabilities` | OpenAPI 3.1 spec (auto-generated) | Any HTTP-aware agent |
| `GET /schema/gir` | GIR JSON Schema | Schema validators, code generators |
| `GET /.well-known/ai-plugin.json` | OpenAI plugin manifest format | ChatGPT plugins, compatible agents |

---

## New Crate: `ul-mcp`

See [mcp-server.md](mcp-server.md) for full specification.

Model Context Protocol server exposing 5 UL tools:
- `ul_parse` — Parse UL-Script to GIR
- `ul_validate` — Validate GIR against Σ_UL
- `ul_render` — Render GIR to SVG/TikZ
- `ul_deparse` — Convert GIR back to UL-Script
- `ul_parse_and_render` — Full pipeline in one call

---

## Files to Create / Modify

| File | Action | Purpose |
|------|--------|---------|
| `crates/ul-mcp/Cargo.toml` | Create | MCP server crate manifest |
| `crates/ul-mcp/src/main.rs` | Create | MCP tool handler implementation |
| `crates/ul-api/src/openapi.rs` | Create | OpenAPI spec generation |
| `crates/ul-api/src/routes.rs` | Modify | Add /capabilities, /schema, ai-plugin endpoints |
| `packages/sdk/package.json` | Create | Standalone TypeScript SDK |
| `packages/sdk/src/*` | Create | SDK source files |
| `schemas/gir.schema.json` | Create | Machine-readable GIR schema |
| `AGENTS.md` | Modify | Add distribution links |
| `Cargo.toml` | Modify | Add ul-mcp to workspace members |

---

## Build & Publish Pipeline

```bash
# 1. npm — SDK + WASM + schemas
cd packages/sdk && npm run build
cd crates/ul-wasm && wasm-pack build --target bundler
# package together and: npm publish @ul-forge/core --access public

# 2. PyPI — native Python extension
cd bindings/python && maturin publish

# 3. Docker — REST API server
docker build -t ghcr.io/jthora/ul-forge .
docker push ghcr.io/jthora/ul-forge

# 4. Binary — CLI
cargo build --release -p ul-cli
# attach to GitHub Release

# 5. MCP — standalone or bundled
cargo install ul-mcp
# also included in Docker image
```

---

## Success Criteria

1. `npm install @ul-forge/core` installs cleanly, `import { parse } from '@ul-forge/core'` works
2. `pip install ul-forge` installs cleanly, `from ul_forge import parse` works
3. `docker run -p 8080:8080 ghcr.io/jthora/ul-forge` starts server, `curl /capabilities` returns OpenAPI spec
4. MCP server responds to `tools/list` with 5 UL tools
5. An LLM with function calling can parse, validate, and render a UL glyph using only the package metadata for guidance
