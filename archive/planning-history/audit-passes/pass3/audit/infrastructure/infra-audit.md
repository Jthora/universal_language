# Infrastructure & Distribution Audit

**Date:** 2026-04-08  
**Scope:** Distribution strategy, packaging, deployment, documentation infrastructure  
**Goal:** Assess what's needed to make UL installable, deployable, and usable by developers

---

## Executive Summary

**Grade: C+ (Architecture solid, nothing published or deployable)**

The distribution architecture is well-designed with 4 target consumer types and clear package boundaries. However, nothing is actually published to any registry. A developer wanting to integrate UL into their app today would need to clone the repo and build from source. The gap between "designed" and "deployed" is the primary blocker for app-readiness.

---

## 1. Distribution Strategy (docs/distribution/)

### 1.1 Four Consumer Types

| Type | Target | Package | Status |
|------|--------|---------|--------|
| Type 1: AI Agent | npm, PyPI, Docker, MCP | `@ul-forge/core` | ❌ Not published |
| Type 2: ProtoFusionGirl | Game module (WASM) | `ul-game` crate | ✅ Implemented, not packaged |
| Type 3: Web Components | HTML custom elements | `@ul-forge/components` | ❌ Planning only |
| Type 4: Transceiver | Wire protocol | `@ul-forge/transceiver` | ❌ Planning only |

### 1.2 Prerequisites (P1–P3)

| Prereq | Description | Status |
|--------|-------------|--------|
| P1: SDK extraction | Extract shared TypeScript types | ⚠️ Partial (`packages/sdk/` has README, minimal src) |
| P2: Schema validation | JSON Schema for all wire formats | ⚠️ Partial (GIR schema exists, transceiver missing) |
| P3: Schema generation | Auto-generate from Rust types | ❌ Not done |

---

## 2. Package State

### 2.1 npm Packages (ul-forge/packages/)

| Package | Directory | Has src/ | Buildable | Published |
|---------|-----------|----------|-----------|-----------|
| `@ul-forge/core` | packages/core/ | ✅ | ❓ Needs verification | ❌ |
| `@ul-forge/sdk` | packages/sdk/ | ⚠️ Minimal | ❓ | ❌ |
| `@ul-forge/web-components` | packages/web-components/ | ❌ Planning only | ❌ | ❌ |
| `@ul-forge/transceiver` | packages/transceiver/ | ❌ Planning only | ❌ | ❌ |

### 2.2 Python (PyPI)

| Package | Status | Install |
|---------|--------|---------|
| `ul-forge` (PyO3) | ✅ Builds locally via `maturin develop` | ❌ Not on PyPI |

### 2.3 Docker

| Image | Status | Registry |
|-------|--------|----------|
| ul-api server | ✅ Dockerfile exists (multi-stage) | ❌ Not pushed |

### 2.4 VS Code Extension

| Package | Status | Registry |
|---------|--------|----------|
| `ul-forge-0.1.0.vsix` | ✅ Built (9.14 KB) | ❌ Not on marketplace |

---

## 3. Documentation Infrastructure

### 3.1 ul-forge-v1 Spec (docs/ul-forge-v1/)

Comprehensive architectural spec across 9 subdirectories:

| Directory | Content | Mapped to Code? |
|-----------|---------|-----------------|
| architecture/ | Overview, data flow, graph model | ❌ No explicit mapping |
| specifications/ | UL-Script grammar, GIR format | ⚠️ GIR schema exists |
| components/ | Parser, Renderer, Validator specs | ❌ |
| api/ | REST endpoints, WebSocket | ❌ |
| frontends/ | Web editor, VS Code, Jupyter | ❌ |
| development/ | Roadmap, tech stack, testing | ❌ |
| collaboration/ | Diff/merge, CRDT | ❌ |
| ai-integration/ | LLM, vision, GNN | ❌ |
| appendices/ | Glossary, comparable systems | N/A |

**Critical gap:** The v1 spec describes a complete system but doesn't map to the actual `ul-forge/crates/` implementation. A developer reading the spec has no idea what's done vs. what's planned.

### 3.2 Entry Points

| Document | Audience | Current? |
|----------|----------|----------|
| README.md | Humans | ✅ (March 2026) |
| AGENTS.md | AI agents | ✅ (April 2026) |
| FOR-AI.md | AI philosophical | ✅ (April 2026) |
| llms.txt | LLM context | ✅ (April 2026) |
| index.json | Machine-readable | ✅ (April 2026) |
| CONTRIBUTING.md | Contributors | ⚠️ Experiment-focused only |

### 3.3 Missing Documentation

| Document | Need |
|----------|------|
| **Quick Start for Developers** | "Install UL Forge and render your first glyph in 5 minutes" |
| **API Reference** | Auto-generated from Rust docs (cargo doc) |
| **Package Migration Guide** | How types from v1 spec map to new packages |
| **Deployment Guide** | How to deploy ul-api in production |
| **Integration Examples** | Using UL in React, Python, CLI, Jupyter |

---

## 4. Build System

### 4.1 Rust

```bash
cd ul-forge && cargo test --workspace  # ~157 tests
cd ul-forge && cargo build --release   # All crates
```

**Status:** ✅ Working

### 4.2 WASM

```bash
cd ul-forge && wasm-pack build crates/ul-wasm --target web --out-dir ../../web/wasm-pkg
```

**Status:** ✅ Working

### 4.3 Web Editor

```bash
cd ul-forge/web && npm install && npm run dev   # Development
cd ul-forge/web && npm run build                # Production
```

**Status:** ✅ Working (Vite + React)

### 4.4 Python

```bash
cd ul-forge/bindings/python && maturin develop  # Local install
```

**Status:** ✅ Working (needs .venv)

### 4.5 VS Code Extension

```bash
cd ul-forge/vscode-extension && npm run compile && npx @vscode/vsce package
```

**Status:** ✅ Working

### 4.6 Missing CI/CD

- ❌ No GitHub Actions workflow
- ❌ No automated testing on push/PR
- ❌ No automated publishing pipeline
- ❌ No version management (all packages at 0.1.0)

---

## 5. Schema State

| Schema | File | Status |
|--------|------|--------|
| GIR | `schemas/gir.schema.json` | ✅ Exists (nodes, edges, sorts) |
| Transceiver | `schemas/ul-transceiver.schema.json` | ❌ Referenced but missing |
| API | — | ❌ No OpenAPI spec |
| MCP | — | ❌ No formal schema (inline in code) |

### 5.1 GIR Schema Gaps

Current GIR schema tracks:
- ✅ Nodes (with sort, position, label)
- ✅ Edges (6 types: contains, modified_by, adjacent, intersects, connects, references)
- ✅ Sort validation

Missing:
- ❌ `modal_context` — world frames, accessibility relations
- ❌ `performative_force` — φ parameter on assertions
- ❌ `pragmatic_annotation` — inference type, surface/intended
- ❌ `binding` — variable slots, scope markers
- ❌ `assertion_modifier` — evidentiality, emphasis

---

## 6. Priority Actions for Pass 3

### Phase 1: Packaging (Critical Path)

| # | Action | Effort | Blocked By |
|---|--------|--------|------------|
| 1 | Rebuild dist/ outputs (fix "11 operations") | Low | — |
| 2 | Verify @ul-forge/core builds from packages/core/ | Low | — |
| 3 | Publish @ul-forge/core to npm (beta) | Low | #2 |
| 4 | Publish ul-forge to PyPI (beta) | Low | — |
| 5 | Push Docker image to registry | Low | — |
| 6 | Publish VS Code extension to marketplace | Low | — |
| 7 | Create transceiver schema | Medium | — |

### Phase 1b: Documentation

| # | Action | Effort |
|---|--------|--------|
| 8 | Create "Quick Start for Developers" | Medium |
| 9 | Map ul-forge-v1 spec to actual implementation | Medium |
| 10 | Add code contribution guidelines to CONTRIBUTING.md | Low |
| 11 | Create OpenAPI spec for ul-api | Medium |
| 12 | Set up GitHub Actions CI | Medium |

### Phase 2+: Web Components

| # | Action | Effort |
|---|--------|--------|
| 13 | Implement `<ul-symbol>` web component | High |
| 14 | Implement `<ul-composer>` web component | High |
| 15 | Build demo site with embedded components | Medium |
