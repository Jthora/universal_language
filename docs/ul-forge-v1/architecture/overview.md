# System Architecture Overview

> UL Forge is a pipeline: **parse → represent → layout → render**. Each stage has a single responsibility, a well-defined input/output format, and can be used independently.

---

## Component Map

```
                        ┌──────────────────────────────────┐
                        │         UL FORGE v1               │
                        └──────────────────────────────────┘
                                       │
          ┌────────────────────────────┼────────────────────────────┐
          │                            │                            │
    ┌─────▼──────┐              ┌──────▼──────┐              ┌─────▼──────┐
    │  CORE      │              │  API        │              │ FRONTENDS  │
    │            │              │             │              │            │
    │ • Parser   │◄────────────►│ • REST      │◄────────────►│ • Web      │
    │ • Renderer │              │ • WebSocket │              │ • VS Code  │
    │ • Validator│              │             │              │ • Jupyter  │
    │ • CLI      │              └─────────────┘              └────────────┘
    └────────────┘
         │
    Written in Rust
    Compiles to:
    • Native binary (CLI)
    • WASM (browser, API)
```

> **WASM Module Architecture:** For the detailed WASM compilation pipeline, 33 exported
> functions, TypeScript wrapper layer, serialization boundary, and caching system, see
> [wasm/README.md](../wasm/README.md). For integration recipes across different platforms
> (browser, game engine, Node.js, AI agents), see [wasm/integration-patterns.md](../wasm/integration-patterns.md).

## Core Components

### Parser (`ul-parse`)
- **Input:** UL-Script text (Unicode)
- **Output:** UL-GIR (JSON, graph-with-tree-spine)
- **Responsibility:** Tokenize, parse, and transform UL-Script into a validated GIR document
- **Language:** Rust
- **Details:** [components/parser/README.md](../components/parser/README.md)

### Renderer (`ul-render`)
- **Input:** UL-GIR (JSON)
- **Output:** SVG (primary), TikZ, PDF, Canvas commands
- **Responsibility:** Solve layout constraints and produce visual output faithful to geometric semantics
- **Language:** Rust (compiles to WASM for browser use)
- **Details:** [components/renderer/README.md](../components/renderer/README.md)

### Validator (`ul-validate`)
- **Input:** UL-GIR (JSON)
- **Output:** Validation report (errors, warnings, info)
- **Responsibility:** Check structural integrity, sort correctness, and graph invariants
- **Language:** Rust (shares type definitions with parser)
- **Details:** [components/validator/README.md](../components/validator/README.md)

### CLI (`ul-parse`, `ul-render`, `ul-validate`, `ul-convert`)
- **Input:** Files or stdin
- **Output:** Files or stdout
- **Responsibility:** Thin wrappers around core libraries, composable via Unix pipes
- **Language:** Rust
- **Details:** [components/cli/README.md](../components/cli/README.md)

## API Layer

### REST API (`ul-serve`)
- **Purpose:** HTTP endpoints wrapping core tools for web and AI agent consumption
- **Endpoints:** `/parse`, `/render`, `/validate`, `/convert`
- **Details:** [api/README.md](../api/README.md)

### WebSocket
- **Purpose:** Live preview — client sends UL-Script keystrokes, server streams incremental GIR updates and re-rendered SVG
- **Details:** [api/websocket.md](../api/websocket.md)

## Frontend Layer

### Web Editor
- **Purpose:** Split-pane editor: type UL-Script on left, see rendered glyph on right
- **Stack:** React/Vue + Monaco Editor + D3.js (or direct SVG)
- **Details:** [frontends/web-editor/README.md](../frontends/web-editor/README.md)

### VS Code Extension
- **Purpose:** Language server for UL-Script: syntax highlighting, autocomplete, inline errors, live preview panel
- **Details:** [frontends/vscode-extension/README.md](../frontends/vscode-extension/README.md)

### Jupyter Integration
- **Purpose:** `%%ul` magic command renders UL-Script inline in notebook cells
- **Details:** [frontends/jupyter/README.md](../frontends/jupyter/README.md)

---

## Architectural Invariants

These properties hold across all components:

1. **GIR is the single source of truth.** No component stores meaning in any other format. UL-Script and SVG are derived views.

2. **Sort safety is enforced everywhere.** Every function that constructs or transforms GIR nodes checks that the Σ_UL sort constraints are satisfied. An Entity cannot appear where a Relation is expected.

3. **Graph structure is preserved through the pipeline.** The parser produces a graph. The validator checks the graph. The renderer lays out the graph. No stage flattens or lossy-compresses the graph into a tree.

4. **Components are independently usable.** `ul-parse` works without `ul-render`. `ul-validate` works on hand-written JSON-GIR. The API wraps tools but doesn't replace them.

5. **WASM compilation is a first-class target.** The Rust core compiles to WASM so the same parser/renderer runs in the browser without reimplementation in TypeScript. The WASM crate (`ul-wasm`) is established in Phase 0 (Foundation) alongside the native binary. The TypeScript bridge and npm packaging (`@ul-forge/core`) are Phase 2 (Web Editor).

---

## Deployment Topology

```
DEVELOPMENT:
  Developer laptop
  ├── Rust toolchain (cargo build)
  ├── CLI tools (ul-parse, ul-render, ul-validate)
  └── VS Code with UL Forge extension

STANDALONE WEB:
  Browser (no server needed)
  ├── WASM core (parser + renderer)
  ├── Web editor (React/Vue)
  └── All processing happens client-side

API SERVICE:
  Server
  ├── ul-serve (REST + WebSocket)
  ├── Rust binary or WASM (Deno/Bun/Node)
  └── Serves web editor as static files

JUPYTER:
  Python environment
  ├── ul-forge Python package
  ├── Calls core via WASM (PyO3/wasm-pack) or subprocess
  └── Renders SVG inline in notebook output cells
```

---

## Technology Stack Summary

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| Core (parser, renderer, validator) | **Rust** | Sort-safe ADTs, no runtime nulls, WASM compilation, performance for layout solver |
| Browser runtime | **WASM** (from Rust via wasm-pack) | Same code in CLI and browser, no TypeScript reimplementation |
| Web frontend | **TypeScript** (React or Vue) | DOM, Monaco Editor, D3.js — TypeScript is genuinely good here |
| VS Code extension | **TypeScript** | VS Code extension API requires it |
| API service | **Rust** (Actix/Axum) or **TypeScript** (wrapping WASM) | Web framework maturity |
| Python library | **Python** | PyO3 bindings to Rust core, or subprocess calls |
| CI/CD | **GitHub Actions** | Standard, free for open source |

See [development/tech-stack.md](../development/tech-stack.md) for detailed rationale.
