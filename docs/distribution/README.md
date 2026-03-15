# UL Forge Distribution — Development Plan

> Master plan for packaging ul-core into 4 distinct products serving 4 different consumers.

**Status:** Planning  
**Created:** 2026-03-14  
**Depends on:** ul-forge v1.1 hardening (S1–S6 complete, S7–S8 in progress)

---

## Problem Statement

The ul-core Rust library contains the complete UL pipeline (parse → validate → render → deparse). Today it ships as a monolithic WASM blob welded to a single React web app. This blocks adoption by 4 fundamentally different consumer archetypes:

1. **AI agents** that need self-describing, zero-config packages they can autonomously discover and deploy
2. **Game engines** that need raw geometry, evaluation rules, and sub-millisecond performance — not SVG strings
3. **Websites** that need a `<script>` tag, not a build system
4. **AI-to-AI systems** that need structured wire protocols, not human-readable rendering

Each requires a different binding layer on top of the same core.

---

## The 4 Distribution Types

| Type | Codename | Consumer | Form Factor | New Artifacts |
|------|----------|----------|-------------|---------------|
| **1** | AI Agent Package | LLM tool-chains, coding agents, MCP clients | npm + PyPI + Docker + CLI + MCP server | `ul-mcp` crate, `@ul-forge/core` npm, OpenAPI endpoints |
| **2** | ProtoFusionGirl Module | Phaser 3 game client | Game-specific WASM with evaluation API | `ul-game` crate, cosmic rule engine, animation bindings |
| **3** | Educational Web Component | Any HTML page | `<ul-symbol>` custom element, CDN script | `@ul-forge/components` package, IIFE bundle |
| **4** | AI Transceiver Protocol | Multi-agent systems | Wire protocol + Rust/TS/Python codecs | `ul-transceiver` crate, `@ul-forge/transceiver`, JSON Schema |

---

## Dependency Graph

```
                    ┌──────────┐
                    │ ul-core  │  (shared foundation — parse, validate, render, deparse)
                    └────┬─────┘
                         │
          ┌──────────────┼──────────────┬─────────────────┐
          │              │              │                  │
     ┌────▼────┐   ┌────▼────┐   ┌────▼─────┐    ┌──────▼───────┐
     │ ul-wasm │   │ ul-game │   │ ul-mcp   │    │ul-transceiver│
     │ (base)  │   │ (Type2) │   │ (Type1)  │    │   (Type 4)   │
     └────┬────┘   └────┬────┘   └────┬─────┘    └──────┬───────┘
          │              │              │                  │
     ┌────▼────────┐    │         ┌────▼────┐            │
     │ @ul-forge/  │    │         │ ul-api  │            │
     │    sdk      │    │         │ + MCP   │            │
     └────┬────────┘    │         └─────────┘            │
          │              │                                │
   ┌──────┼───────┐     │                                │
   │              │     │                                │
┌──▼───┐   ┌─────▼──┐  │        ┌───────────────────────▼──┐
│Type 1│   │ Type 3 │  │        │  @ul-forge/transceiver   │
│ npm  │   │  <ul-  │  │        │  (TS) + ul-transceiver   │
│ pkg  │   │symbol> │  │        │  (Python)                │
└──────┘   └────────┘  │        └──────────────────────────┘
                        │
                   ┌────▼───────────┐
                   │ ProtoFusionGirl│
                   │   game client  │
                   └────────────────┘
```

---

## Shared Prerequisites

Before any distribution type, 3 preparatory tasks must complete:

| ID | Task | Description | Doc |
|----|------|-------------|-----|
| **P1** | Extract `@ul-forge/sdk` | Move TypeScript wrappers from `web/src/core/` into standalone package | [shared/sdk-extraction.md](shared/sdk-extraction.md) |
| **P2** | Expose layout geometry | Make `compute_layout()`, `PositionedGlyph`, `Shape` public in ul-core | [shared/prerequisites.md](shared/prerequisites.md) |
| **P3** | Generate GIR JSON Schema | Add `schemars` to ul-core, produce machine-readable schema files | [shared/schema-generation.md](shared/schema-generation.md) |

Full details: [shared/prerequisites.md](shared/prerequisites.md)

---

## Implementation Waves

### Wave 1 — Shared Prerequisites (P1, P2, P3)

| Step | Task | Depends On | Deliverable |
|------|------|-----------|-------------|
| 1.1 | Extract `@ul-forge/sdk` | — | `ul-forge/packages/sdk/` with types, wasm-bridge, utils |
| 1.2 | Expose layout geometry | — | `pub use` additions in `ul-core/src/lib.rs` |
| 1.3 | Generate GIR JSON Schema | 1.2 | `ul-forge/schemas/gir.schema.json` + 2 more |
| 1.4 | Update `web/` to import from `packages/sdk/` | 1.1 | Web app still works, now uses extracted SDK |

### Wave 2 — Type 1 + Type 3 (share the SDK)

| Step | Task | Depends On | Deliverable |
|------|------|-----------|-------------|
| 2.1 | Publish `@ul-forge/core` npm | 1.1, 1.3 | WASM + SDK + schema on npm |
| 2.2 | Polish PyPI `ul-forge` | — | Existing bindings, published |
| 2.3 | Productize Docker image | — | `ghcr.io/jthora/ul-forge` with OpenAPI |
| 2.4 | Build `<ul-symbol>` web component | 1.1 | Shadow DOM custom element |
| 2.5 | Build `<ul-composer>` web component | 2.4 | Interactive editor element |
| 2.6 | Build `<ul-dictionary>` component | 2.4 | Reference browser element |
| 2.7 | IIFE/ESM bundle + CDN setup | 2.4–2.6 | Single `<script>` tag deployment |

### Wave 3 — Type 1 MCP + Type 2 Game Module

| Step | Task | Depends On | Deliverable |
|------|------|-----------|-------------|
| 3.1 | Build `ul-mcp` crate | — | MCP server with 5 UL tools |
| 3.2 | API discovery endpoints | 1.3 | `/capabilities`, `/schema/gir`, `ai-plugin.json` |
| 3.3 | `ul-game` crate scaffold | 1.2 | Crate structure, types, WASM entry points |
| 3.4 | Cosmic rule engine | 3.3 | 4 elements × 3 modalities × 9 forces |
| 3.5 | Evaluation + scoring | 3.3, 3.4 | `evaluate()`, `scoreComposition()`, graduated failure |
| 3.6 | Animation bindings | 3.3 | `getAnimationSequence()` keyframe generation |
| 3.7 | Jane's learning system | 3.5 | `evaluateJaneAttempt()`, proficiency tracking |
| 3.8 | Modding pipeline | 3.4 | `loadCustomDefinitions()` runtime rule injection |
| 3.9 | Performance optimization | 3.3–3.8 | <500KB WASM, <1ms simple eval, <16ms complex |

### Wave 4 — Type 4 Protocol

| Step | Task | Depends On | Deliverable |
|------|------|-----------|-------------|
| 4.1 | Wire protocol JSON Schema | 1.3 | `protocol/ul-transceiver.schema.json` |
| 4.2 | Rust `ul-transceiver` crate | 4.1 | Encode/decode, 8 message intents |
| 4.3 | TypeScript `@ul-forge/transceiver` | 4.1 | Encode/decode, typed message builders |
| 4.4 | Python `ul-transceiver` | 4.1 | Encode/decode library |
| 4.5 | Integration test: 2-agent exchange | 4.2, 4.3 | Round-trip GIR integrity verified |

---

## Target Workspace Structure

```
ul-forge/
  Cargo.toml                    # workspace: add ul-game, ul-mcp, ul-transceiver
  crates/
    ul-core/                    # ← P2: expose layout geometry
    ul-cli/                     # existing
    ul-wasm/                    # existing (base WASM for Types 1, 3)
    ul-api/                     # ← Type 1: add /capabilities, OpenAPI
    ul-game/                    # ← NEW: Type 2 game crate
    ul-mcp/                     # ← NEW: Type 1 MCP server
    ul-transceiver/             # ← NEW: Type 4 protocol crate
  bindings/
    python/                     # ← Type 1b: polish + publish
  packages/
    sdk/                        # ← P1: extracted TypeScript SDK
    web-components/             # ← Type 3: <ul-symbol>, <ul-composer>
    transceiver/                # ← Type 4: TypeScript codec
  protocol/
    ul-transceiver.schema.json  # ← Type 4: wire format schema
    README.md
  schemas/
    gir.schema.json             # ← P3: machine-readable schema
    validation-result.schema.json
    render-options.schema.json
  web/                          # existing React editor → imports from packages/sdk
  vscode-extension/             # existing
```

---

## Documentation Map

All distribution documentation lives under `docs/distribution/`:

```
docs/distribution/
  README.md                         ← you are here
  shared/
    prerequisites.md                # P1 + P2 + P3 overview
    sdk-extraction.md               # P1 deep-dive: @ul-forge/sdk
    schema-generation.md            # P3 deep-dive: schemars + JSON Schema
  type-1-ai-agent/
    README.md                       # Architecture + distribution channels
    npm-package.md                  # @ul-forge/core package spec
    mcp-server.md                   # ul-mcp crate spec
    api-discovery.md                # OpenAPI + plugin manifest
    python-package.md               # PyPI distribution
    docker-image.md                 # Container + CI/CD
  type-2-protofusiongirl/
    README.md                       # Architecture + performance budget
    game-crate.md                   # ul-game crate structure
    cosmic-rules.md                 # Rule engine: elements × modalities × forces
    evaluation-api.md               # Scoring + graduated failure
    animation-bindings.md           # Keyframe system
    integration-guide.md            # Phaser 3 integration
  type-3-web-components/
    README.md                       # Architecture + bundle strategy
    ul-symbol.md                    # <ul-symbol> spec
    ul-composer.md                  # <ul-composer> spec
    bundle-strategy.md              # IIFE/ESM + CDN deployment
  type-4-transceiver/
    README.md                       # Architecture + message model
    wire-protocol.md                # JSON Schema + message intents
    rust-library.md                 # ul-transceiver crate
    typescript-library.md           # @ul-forge/transceiver
    python-library.md               # Python codec
```

---

## Success Criteria

| Type | Criterion |
|------|-----------|
| **1** | `npm install @ul-forge/core` works; `pip install ul-forge` works; `docker run` starts API; MCP server responds to tool calls |
| **2** | WASM < 500KB; `evaluate()` < 1ms on simple input; Phaser integration test passes; custom rules load at runtime |
| **3** | `<ul-symbol script="point(existence)">` renders in plain HTML; no build tools needed; works in Chrome/Firefox/Safari |
| **4** | Two agents exchange UL messages over WebSocket; both encode/decode; capability advertisement works; round-trip GIR integrity preserved |

---

## Open Questions

1. **Monorepo tooling**: Should we use npm workspaces, Turborepo, or Nx for the `packages/` directory? (Current: no monorepo tooling)
2. **Registry scoping**: `@ul-forge` scope on npm — need to reserve it. Alternative: `ul-forge-*` flat names.
3. **WASM size budget**: Base ul-wasm is ~200KB. Type 2 adds cosmic rules + evaluation. Can we stay under 500KB?
4. **Protocol versioning**: How does the transceiver protocol evolve? Semantic versioning on the schema? Backwards-compatible field additions only?
5. **ProtoFusionGirl coupling**: How tightly should `ul-game` be coupled to ProtoFusionGirl specifically vs. being a generic "game engine UL module"?
