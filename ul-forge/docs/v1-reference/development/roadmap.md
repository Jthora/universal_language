# Development Roadmap

> Phase-by-phase plan for building UL Forge v1.

---

## v1.1 Hardening Release

> **Detailed plan:** [v1.1-plan.md](v1.1-plan.md) — 8 sprints covering panic safety, security, documentation fixes, test infrastructure, CI/CD, template completion, frontend hardening, and spec alignment. Must be completed before Phase 4 begins.

---

## Vision

UL Forge v1 is the minimum viable toolchain for composing, validating, and rendering Universal Language glyphs. By the end of v1, a user can:
1. Write UL-Script in a text editor or web app
2. See a live-rendered SVG preview
3. Validate glyph structure against Σ_UL constraints
4. Export publication-quality SVG or TikZ

---

## Phase Overview

| Phase | Name | Deliverables | Dependencies |
|-------|------|-------------|-------------|
| **0** | Foundation | Repo scaffold, CI, core types | None |
| **1** | Core Pipeline | Parser + Validator + SVG Renderer (CLI) | Phase 0 |
| **2** | Web Editor | Browser-based editor with live preview | Phase 1 |
| **3** | Ecosystem | VS Code extension, Jupyter, API server | Phase 1 |
| **4** | Intelligence | AI integration, collaboration, templates | Phase 2–3 |

---

## Phase 0 — Foundation

**Goal:** Working repository with builds, tests, CI, and core data structures.

### Deliverables

- [ ] Repository scaffold: `crates/ul-core/`, `crates/ul-cli/`, `web/`, `bindings/`
- [ ] Rust workspace `Cargo.toml` with workspace members
- [ ] Core types: `Node`, `Edge`, `Gir`, `Sort`, `EdgeType` (Rust structs/enums)
- [ ] GIR serialization: `serde` JSON round-trip for all core types
- [ ] Test harness: `cargo test` runs, property tests with `proptest`
- [ ] CI pipeline: GitHub Actions — build, test, clippy, format check
- [ ] WASM build: `wasm-pack build` produces working `.wasm` binary (TypeScript wrapper deferred to Phase 2 M2.1)
- [ ] Documentation: `CONTRIBUTING.md`, `README.md` for developers

### Exit Criteria
- `cargo build && cargo test` passes
- `wasm-pack build` produces a `.wasm` file
- CI is green on push to main

---

## Phase 1 — Core Pipeline

**Goal:** Working parser, validator, and renderer. End-to-end UL-Script → SVG via CLI.

### Milestone 1.1: Tokenizer + Parser

- [ ] UL-Script tokenizer: Unicode → Token stream
- [ ] ASCII fallback tokenizer: ASCII sequences → Token stream
- [ ] PEG parser: Token stream → AST (using `pest` crate)
- [ ] AST → GIR transformation (5 rules from [ast-to-gir.md](../components/parser/ast-to-gir.md))
- [ ] Error reporting with source positions (line, column, span)
- [ ] Snapshot tests: 20+ UL-Script inputs with expected GIR output

### Milestone 1.2: Validator

- [ ] Schema validation (Layer 1): required fields, ID uniqueness, referential integrity
- [ ] Sort validation (Layer 2): operation signatures match Σ_UL
- [ ] Graph invariant validation (Layer 3): containment DAG, single root, angle arity
- [ ] `ValidationResult` struct with error codes and node/edge references
- [ ] Test suite: 30+ GIR documents (valid and invalid)

### Milestone 1.3: Renderer

- [ ] Template library: implement 10 Tier-1 templates (forced canonical glyphs)
- [ ] Template matching: GIR subgraph → template lookup
- [ ] Hierarchical constraint layout for non-template glyphs
- [ ] SVG generation: positioned nodes → SVG elements
- [ ] GIR metadata embedding in SVG `<metadata>` element
- [ ] Snapshot tests: visual regression with reference SVGs

### Milestone 1.4: CLI

- [ ] `ul parse` command: file/stdin → GIR JSON to stdout
- [ ] `ul render` command: GIR JSON → SVG to stdout/file
- [ ] `ul validate` command: GIR JSON → validation result
- [ ] `ul convert` command: any format → any format
- [ ] Shell completions generation
- [ ] Integration tests: end-to-end UL-Script → SVG

### Exit Criteria
- `echo '◯(•) → △(•)' | ul parse | ul render > test.svg` produces a valid SVG
- All 42 canonical lexicon glyphs parse and render without error
- `ul validate` catches all schema, sort, and invariant violations

---

## Phase 2 — Web Editor

**Goal:** Browser-based editor with dual-pane editing and live preview.

### Milestone 2.1: WASM Bridge

- [ ] TypeScript wrapper around WASM module: `parse()`, `render()`, `validate()`
- [ ] npm package: `@ul-forge/core`
- [ ] Performance benchmarks: parse+render < 50ms for 50-node GIR in browser

### Milestone 2.2: Editor Shell

- [ ] React app scaffold (Vite + React + TypeScript)
- [ ] Monaco Editor integration with UL-Script language definition
- [ ] Syntax highlighting (TextMate grammar)
- [ ] Two-pane layout: editor left, preview right
- [ ] Responsive: works on 1024px+ screens

### Milestone 2.3: Live Preview

- [ ] Debounced parse-on-keystroke (100ms debounce)
- [ ] SVG rendering via WASM in web worker
- [ ] D3-based SVG viewer: zoom, pan, click-to-select
- [ ] Bidirectional linking: click SVG → highlight text, click text → highlight SVG
- [ ] Error display: inline squiggles + error panel

### Milestone 2.4: Visual Builder

- [ ] Primitive palette (5 geometric types)
- [ ] Drag-and-drop placement on canvas
- [ ] Edge drawing (click source, click target)
- [ ] GIR ↔ UL-Script bidirectional sync
- [ ] Undo/redo stack (GIR-level operations)

### Milestone 2.5: Templates and Export

- [ ] Template palette sidebar (42 canonical glyphs)
- [ ] Template insertion (click → insert at cursor)
- [ ] Export: SVG download, TikZ download, PNG export (via canvas), GIR JSON download
- [ ] Static deployment build (GitHub Pages / Netlify)

### Exit Criteria
- User can type UL-Script and see live SVG preview
- User can drag-and-drop primitives to compose a glyph
- 42 canonical glyphs are available as templates
- Exported SVG matches CLI output

---

## Phase 3 — Ecosystem

**Goal:** VS Code extension, Jupyter integration, and HTTP API server.

### Milestone 3.1: VS Code Extension

- [ ] Extension scaffold (Yeoman generator)
- [ ] UL-Script language registration (syntax highlighting, bracket matching)
- [ ] WASM module loading in extension host
- [ ] Preview webview: side-by-side SVG rendering
- [ ] Diagnostics: parse/validation errors as VS Code problems
- [ ] Commands: preview, export, validate, insert glyph
- [ ] VSIX package build

### Milestone 3.2: Jupyter Integration

- [ ] PyO3 bindings: `ul_forge.parse()`, `ul_forge.render()`, `ul_forge.validate()`
- [ ] Maturin build system (`pyproject.toml`)
- [ ] `%%ul` cell magic: UL-Script → inline SVG
- [ ] `%%ul_gir` cell magic: UL-Script → pretty-printed GIR
- [ ] pip-installable package: `pip install ul-forge`

### Milestone 3.3: API Server

- [ ] Axum HTTP server wrapping core library
- [ ] REST endpoints: `/parse`, `/render`, `/validate`, `/convert`, `/health`
- [ ] WebSocket `/live` endpoint with debounced live preview
- [ ] Docker image: `ulforge/api:v1`
- [ ] Integration tests against running server

### Exit Criteria
- VS Code extension installable from VSIX, provides syntax highlighting + preview
- `pip install ul-forge` works; `%%ul` magic renders in Jupyter
- `docker run ulforge/api:v1` serves a working API

---

## Phase 4 — Intelligence

**Goal:** AI assistance, real-time collaboration, and the complete template library.

> **Detailed plan:** [phase-4-plan.md](phase-4-plan.md) — full task breakdown, dependency graph, file deliverables, and exit criteria.

### Milestone 4.1: Complete Template Library

- [ ] Implement all 42 canonical lexicon templates (Tiers 1-3) in `templates.rs`
- [ ] Add anchor points to Template struct for composition attachment
- [ ] Template composition engine: combine templates at named anchor points
- [ ] Template search: find templates by label, pattern, or sort signature
- [ ] Snapshot tests for all 42 templates
- [ ] Update VS Code insert-glyph to full 42 templates

### Milestone 4.2: AI Integration

**M4.2a — LLM Interface (Core):**
- [ ] New `crates/ul-ai/` crate: provider-agnostic LLM client
- [ ] Provider implementations: OpenAI, Anthropic, Ollama
- [ ] System prompt builder (<2000 tokens): Σ_UL spec + 42 templates + sort rules
- [ ] `compose()`: natural language → UL-Script with validation-in-the-loop (max 3 retries)
- [ ] `explain()`: UL-Script → natural language description
- [ ] `suggest()`: partial UL-Script → 3 validated completions
- [ ] CLI commands: `ul ai compose`, `ul ai explain`, `ul ai suggest`
- [ ] API endpoints: `POST /ai/compose`, `/ai/explain`, `/ai/suggest`
- [ ] Integration tests with mock HTTP provider

**M4.2b — AI Panel (Web Editor):**
- [ ] TypeScript AI client mirroring Rust crate (compose, explain, suggest)
- [ ] `AiPanel.tsx`: collapsible side panel with compose input, explain display, suggest ghost text
- [ ] API key management in localStorage (never sent to UL Forge servers)
- [ ] Provider/model selector dropdown
- [ ] Keyboard shortcut: `Ctrl+Shift+A` toggles panel

**M4.2c — GNN Interface (Stretch):**
- [ ] PyTorch Geometric glyph encoder (GIN, 9→64→32)
- [ ] Training on 42 canonical glyphs + augmented compositions
- [ ] `ul_forge.similar()` and `ul_forge.analogy()` Python functions

**M4.2d — Vision Interface (Stretch):**
- [ ] Image → vision LLM → UL-Script pipeline
- [ ] Web editor "Upload glyph" button

**M4.2e — Theorem Prover (Stretch):**
- [ ] Z3/SMT encoding of sort constraints
- [ ] `ul prove <file.ul>` CLI command

### Milestone 4.3: Collaboration

**M4.3a — Structural Diff & Merge:**
- [ ] `diff.rs` module in ul-core: graph-aware diff (add/remove/modify operations)
- [ ] `merge.rs` module: three-way merge with conflict detection
- [ ] CLI commands: `ul diff`, `ul merge`
- [ ] Git merge driver: `*.ul.json merge=ul-forge`

**M4.3b — CRDT Real-Time Collaboration:**
- [ ] Yjs integration in web editor: Y.Map (nodes) + Y.Array (edges)
- [ ] GIR ↔ Yjs bidirectional sync
- [ ] Awareness protocol (collaborator cursors + names)
- [ ] Collaboration server (Node.js y-websocket relay)
- [ ] Room management, offline support (IndexedDB), auto-reconnect
- [ ] Collaboration UI: share button, cursor rendering, connection status

### Milestone 4.4: Advanced Features

- [ ] TikZ output: complete SVG→TikZ mapping, LaTeX preamble, compile tests
- [ ] Batch rendering: `ul render --batch`, parallel with rayon, atlas output
- [ ] GIR query language: Cypher-inspired pattern matching on GIR subgraphs
- [ ] Performance: incremental parsing, render caching, WASM size optimization
- [ ] Benchmark suite: target <50ms for 50-node GIR in browser

### Internal Dependencies

```
M4.1 Templates ──→ M4.2a LLM (needs templates in prompt)
                 ──→ M4.3a Diff (needs templates for composition tests)
M4.2a LLM ──→ M4.2b AI Panel
M4.3a Diff ──→ M4.3b CRDT
M4.4a-d are independent (parallelizable)
```

### Exit Criteria
- All 42 canonical glyphs have validated templates (snapshot tests)
- AI can translate simple natural language prompts to valid UL-Script
- Two users can edit the same glyph simultaneously in the web editor
- TikZ output compiles for all 42 templates
- 126+ existing tests still pass (no regressions)

---

## Milestone Dependency Graph

```
Phase 0 (Foundation)
    │
    ▼
Phase 1 (Core Pipeline)
    │
    ├────────────────┐
    ▼                ▼
Phase 2           Phase 3
(Web Editor)      (Ecosystem)
    │                │
    └───────┬────────┘
            ▼
        Phase 4
    (Intelligence)
```

Phases 2 and 3 can proceed in parallel once Phase 1 is complete.

---

## Success Metrics

| Metric | Target |
|--------|--------|
| All 42 canonical glyphs parse correctly | Phase 1 |
| End-to-end latency (type → see SVG) | < 200ms (Phase 2) |
| Test coverage (Rust core) | > 80% (Phase 1+) |
| Zero known sort-constraint violations in shipped builds | All phases |
| VS Code extension installs from marketplace | Phase 3 |
| API serves 100 concurrent WebSocket connections | Phase 3 |
