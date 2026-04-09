# WASM Modules — Architecture, Build, and Integration Guide

> Complete reference for the UL Forge WebAssembly system: Rust crate structure,
> `wasm-pack` build pipeline, TypeScript wrapper layer, initialization patterns,
> serialization boundaries, caching, testing, and deployment.

---

## Table of Contents

1. [System Overview](#1-system-overview)
2. [Crate Architecture](#2-crate-architecture)
3. [Build Pipeline](#3-build-pipeline)
4. [Initialization & Lifecycle](#4-initialization--lifecycle)
5. [Serialization Boundary](#5-serialization-boundary)
6. [The 33 WASM Exports](#6-the-33-wasm-exports)
7. [TypeScript Wrapper Layer](#7-typescript-wrapper-layer)
8. [Caching System](#8-caching-system)
9. [Testing WASM Modules](#9-testing-wasm-modules)
10. [Deployment Targets](#10-deployment-targets)
11. [Troubleshooting](#11-troubleshooting)

**See also:**
- [TypeScript API Reference](typescript-api-reference.md) — complete typed function signatures
- [Integration Patterns](integration-patterns.md) — recipes for game engines, web apps, Node.js, agents
- [Known Issues & Edge Cases](known-issues.md) — validated behaviors and gotchas

---

## 1. System Overview

The UL Forge WASM system compiles the entire Universal Language formal engine
(parser, validator, renderer, 13 Σ_UL operations, game engine, teaching system,
lexicon) into a single WebAssembly binary consumable from any JavaScript runtime.

```
┌─────────────────────────────────────────────────────────────────┐
│                        Rust Workspace                           │
│                                                                 │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌─────────────┐ │
│  │ ul-core  │   │ ul-game  │   │ ul-wasm  │   │ ul-mcp, etc │ │
│  │ parser   │──▶│ game ctx │──▶│ wasm_    │   │ (native     │ │
│  │ validator│   │ scoring  │   │ bindgen  │   │  only)      │ │
│  │ renderer │   │ puzzles  │   │ exports  │   │             │ │
│  │ composer │   │ teaching │   │          │   │             │ │
│  │ analyzer │   │ lexicon  │   │          │   │             │ │
│  └──────────┘   └──────────┘   └──────────┘   └─────────────┘ │
│                                     │                           │
└─────────────────────────────────────┼───────────────────────────┘
                                      │ wasm-pack build
                                      ▼
                           ┌──────────────────────┐
                           │  wasm-pkg/           │
                           │  ├── ul_wasm_bg.wasm │  ← compiled binary
                           │  ├── ul_wasm.js      │  ← JS glue (wasm-bindgen)
                           │  ├── ul_wasm.d.ts    │  ← TS declarations
                           │  └── package.json    │  ← npm metadata
                           └──────────────────────┘
                                      │
                                      ▼
                           ┌──────────────────────┐
                           │  web/src/core/        │
                           │    index.ts           │  ← typed wrapper (25+ fns)
                           │    (caches, types,    │
                           │     error handling)   │
                           └──────────────────────┘
                                      │
                                      ▼
                           ┌──────────────────────┐
                           │  Application Layer    │
                           │  React App / Game /   │
                           │  Node.js / Agent      │
                           └──────────────────────┘
```

**Key design decisions:**

- **ul-game owns the WASM surface.** The `ul-wasm` crate provides 8 direct exports.
  The `ul-game` crate provides 25 more via `#[wasm_bindgen]`. Both get linked into
  a single `.wasm` binary because `ul-wasm` has `extern crate ul_game;`.
- **JSON is the wire format.** All complex types cross the WASM boundary as JSON
  strings, serialized by `serde-wasm-bindgen` (Rust → JS `Map`/`Object`) or
  `serde_json` (Rust → JSON string → `JSON.parse` in TS).
- **TypeScript wrapper hides serialization.** Consumers import from `@ul-forge/core`
  and work with typed interfaces. They never see raw JSON or WASM memory.

---

## 2. Crate Architecture

### 2.1 ul-core (foundation)

The core library with no WASM dependency. Pure Rust.

| Module | Purpose |
|--------|---------|
| `parser` | PEG parser (pest grammar) for UL-Script → GIR |
| `parser::deparse` | GIR → canonical UL-Script text |
| `validator` | 4-layer validation pipeline (schema, sort, invariant, geometry) |
| `renderer` | GIR → SVG rendering with constraint solver + layout engine |
| `composer` | All 13 Σ_UL operations (predicate, negate, embed, etc.) |
| `Gir` | Core data structure: nodes, edges, root, metadata |

### 2.2 ul-game (game engine + WASM surface)

Depends on `ul-core`. Provides the game evaluation engine and **25 WASM exports**.

| Module | Purpose |
|--------|---------|
| `context` | Game session lifecycle (create, evaluate, score) |
| `evaluation` | Rule-based GIR evaluation |
| `scoring` | Graduated scoring (exact/close/partial/unrelated) |
| `animation` | Construction-order keyframe generation |
| `hints` | Teaching hint generation (contextual + standalone) |
| `difficulty` | Adaptive puzzle difficulty via proficiency tracking |
| `cosmic` | Cosmic rule engine (9 forces × 3 modalities × 4 elements) |
| `analysis` | Structural analysis: symmetry groups, Erlangen equivalence |
| `modding` | Runtime custom rule injection |
| `types` | Shared types (GameConfig, EvaluationResult, etc.) |

### 2.3 ul-wasm (thin entry crate)

The `cdylib` crate that `wasm-pack` compiles. It:

1. Re-exports 8 core functions with `catch_unwind` panic safety
2. Force-links `ul-game` via `extern crate ul_game;` to pull in its 25 exports
3. Uses `serde-wasm-bindgen` for type-safe JS ↔ Rust conversion

```toml
# crates/ul-wasm/Cargo.toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
ul-core = { path = "../ul-core" }
ul-game = { path = "../ul-game" }
wasm-bindgen = "=0.2.99"
serde-wasm-bindgen = "0.6"
```

**Why `=0.2.99`?** wasm-bindgen requires exact version parity between the Rust
crate and the CLI tool. Pin it to avoid build failures.

### 2.4 Dependency Graph

```
ul-core ─────────────┐
   │                  │
   ▼                  ▼
ul-game ──────▶ ul-wasm (cdylib)
   │
   ▼
ul-api, ul-cli, ul-mcp (native-only crates)
```

Only `ul-wasm` compiles to `wasm32-unknown-unknown`. The other crates use
platform-specific dependencies (tokio, clap, etc.) that don't compile to WASM.

---

## 3. Build Pipeline

### 3.1 Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | 1.75+ | `rustup update stable` |
| wasm-pack | 0.12+ | `cargo install wasm-pack` |
| wasm32 target | — | `rustup target add wasm32-unknown-unknown` |
| Node.js | 20+ | — |
| npm | 10+ | — |

### 3.2 Building the WASM Module

```bash
cd ul-forge

# Development build (faster, larger binary, debug info)
wasm-pack build --target web --dev crates/ul-wasm

# Production build (optimized — LTO, strip, opt-level 3)
wasm-pack build --target web --release crates/ul-wasm
```

**Output directory:** `ul-forge/crates/ul-wasm/pkg/`

Copy to web project:

```bash
# The web/ directory expects the package at web/wasm-pkg/
cp -r crates/ul-wasm/pkg/* web/wasm-pkg/
```

Or build directly into the web project:

```bash
wasm-pack build --target web crates/ul-wasm --out-dir ../../web/wasm-pkg
```

### 3.3 Build Targets

`wasm-pack` supports multiple output targets. Choose based on your consumer:

| Target | `--target` flag | Use Case | Module System |
|--------|----------------|----------|---------------|
| **Web** | `web` | Browser apps, Vite/Webpack | ESM with `init()` async loader |
| **Bundler** | `bundler` | npm package distribution | ESM, bundler handles WASM loading |
| **Node.js** | `nodejs` | Server-side, testing | CommonJS with sync `require()` |
| **No modules** | `no-modules` | Legacy `<script>` tag | Global variable |

**For the UL Forge web editor, use `--target web`.**

### 3.4 Build Optimization

The workspace `Cargo.toml` configures release optimizations:

```toml
[profile.release]
opt-level = 3       # Maximum optimization
lto = true          # Link-Time Optimization — smaller binary
codegen-units = 1   # Single codegen unit — better optimization
strip = true        # Strip debug symbols
```

**Resulting binary size:** ~800KB–1.2MB (compressed with WASM streaming: ~300–400KB).

**Note:** `panic = "unwind"` is required (not `"abort"`) because `ul-game` uses
`catch_unwind` for WASM safety. Every WASM export is wrapped in a panic boundary
so that a Rust panic doesn't crash the entire JS runtime.

### 3.5 CI Pipeline

```yaml
# .github/workflows/ci.yml — WASM build step
wasm:
  runs-on: ubuntu-latest
  steps:
    - uses: dtolnay/rust-toolchain@stable
      with:
        targets: wasm32-unknown-unknown
    - run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    - run: wasm-pack build --target web crates/ul-wasm
```

### 3.6 Verifying the Build

After building, confirm the output:

```bash
ls -la web/wasm-pkg/
# ul_wasm_bg.wasm   — the compiled WASM binary
# ul_wasm.js        — JS glue code (wasm-bindgen generated)
# ul_wasm.d.ts      — TypeScript type declarations
# package.json      — npm package metadata

# Check exported functions
wasm-objdump -x web/wasm-pkg/ul_wasm_bg.wasm | grep "export" | head -40

# Check binary size
du -sh web/wasm-pkg/ul_wasm_bg.wasm
```

---

## 4. Initialization & Lifecycle

### 4.1 Browser Initialization (async)

The WASM module must be initialized before calling any exported function.
In the browser, initialization is asynchronous (fetches and compiles the `.wasm` file):

```typescript
import wasmInit, { init as wasmModuleInit } from "ul-wasm";

// Step 1: Load and compile the WASM binary
await wasmInit();             // fetches ul_wasm_bg.wasm, compiles it

// Step 2: Initialize Rust-side state (panic hook, etc.)
wasmModuleInit();             // calls the Rust `init()` function

// Now all 33 functions are available
```

The TypeScript wrapper encapsulates this:

```typescript
import * as core from "@ul-forge/core";

await core.initialize();  // Safe to call multiple times (idempotent)
const gir = core.parse("●");
```

### 4.2 Node.js Initialization (synchronous)

For testing and server-side usage, use `initSync`:

```typescript
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const wasmModule = await import("./wasm-pkg/ul_wasm.js");
const wasmBuffer = readFileSync(resolve("./wasm-pkg/ul_wasm_bg.wasm"));

// Synchronous initialization — no async needed
wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
wasmModule.init();

// Now ready
const girJson = wasmModule.parseUlScript("●");
```

### 4.3 Guard Pattern

Every function in the TypeScript wrapper checks initialization status:

```typescript
let initialized = false;

function ensureInit(): void {
  if (!initialized) {
    throw new Error("WASM not initialized. Call initialize() first.");
  }
}

export function parse(input: string): Gir {
  ensureInit();  // ← throws if not initialized
  // ...
}
```

**If you see `"WASM not initialized"` errors**, ensure `core.initialize()` (or the
raw `wasmInit()` + `wasmModuleInit()`) has completed before calling any other function.

### 4.4 Lifecycle Summary

```
Application Start
    │
    ▼
core.initialize()          ← async, resolves when WASM is ready
    │
    ▼
core.parse("●")            ← all functions now available
core.render(gir)
core.createContext()
core.evaluate(ctx, gir)
    │
    ▼
core.clearCaches()         ← optional: free cached results
    │
    ▼
(Application continues — WASM stays loaded for the page lifetime)
```

There is no explicit shutdown. WASM memory is freed when the page unloads.
Game contexts created via `createContext()` persist in thread-local storage
and are never explicitly freed (designed for single-session game patterns).

---

## 5. Serialization Boundary

This is the most important section for understanding WASM integration behavior.
The WASM boundary is a serialization boundary — every value that crosses between
Rust and JavaScript must be serialized.

### 5.1 Two Serialization Paths

The WASM module uses two distinct serialization strategies:

| Path | Mechanism | Used By | JS Type |
|------|-----------|---------|---------|
| **JSON string** | `serde_json::to_string()` in Rust → `JSON.parse()` in TS | `parseUlScript`, `deparseGir`, `applyOperation`, `composeGir` | `string` |
| **serde-wasm-bindgen** | `serde_wasm_bindgen::to_value()` → direct JS object | `validate`, `evaluate`, `analyzeStructure`, `compareGir`, `layout` | `Object` / `Map` |

**Why two paths?** Historical evolution. The `ul-wasm` crate (8 functions) uses
`serde-wasm-bindgen` for direct JS values. The `ul-game` crate (25 functions) uses
JSON strings for some functions and `serde-wasm-bindgen` for others.

### 5.2 JSON String Path

Functions that return JSON strings require `JSON.parse()` on the TypeScript side:

```typescript
// Raw WASM call:
const jsonStr: string = wasmParseUlScript("●");
const gir: Gir = JSON.parse(jsonStr);

// Wrapper handles this:
export function parse(input: string): Gir {
  const json = wasmParseUlScript(input) as string;
  return JSON.parse(json) as Gir;
}
```

### 5.3 serde-wasm-bindgen Path

Functions that use `serde_wasm_bindgen` return JS objects directly — no `JSON.parse`
needed. **However**, Rust `HashMap` values become JavaScript `Map` objects (not plain
objects):

```typescript
const report = wasmAnalyzeStructure(girJson);

// report.primitive_counts is a Map, not an Object!
report.primitive_counts instanceof Map;           // true
report.primitive_counts.get("point");             // 1

// The TypeScript wrapper interface says Record<string, number>,
// but the actual runtime value is a Map.
```

**This is a critical gotcha.** If you access `report.primitive_counts.point`
(object property syntax), you'll get `undefined`. You must use `.get("point")`.

**Affected fields in `StructureReport`:**
- `primitive_counts` — `Map<string, number>` at runtime
- `sort_distribution` — `Map<string, number>` at runtime
- `node_symmetries` — `Map<string, SymmetryGroup>` at runtime

### 5.4 Dual-Path Return Handling

Some functions may return either a string or an object depending on the WASM
serialization path used. The wrapper handles both:

```typescript
export function setForce(gir: Gir, force: ForceName): Gir {
  const result = wasmSetForce(JSON.stringify(gir), force);
  return (typeof result === "string" ? JSON.parse(result) : result) as Gir;
}
```

### 5.5 Input Serialization

All functions that accept a GIR must serialize it to JSON before passing to WASM:

```typescript
export function render(gir: Gir, width = 256, height = 256): string {
  const json = JSON.stringify(gir);    // ← serialize
  return wasmRenderSvg(json, width, height) as string;
}
```

For functions accepting arrays of GIRs (e.g., `applyOperation`, `validateSequence`),
each GIR is individually serialized, then the array is serialized:

```typescript
export function applyOperation(operation: OperationName, operands: Gir[]): Gir {
  const jsons = operands.map((g) => JSON.stringify(g));
  const result = wasmApplyOperation(operation, JSON.stringify(jsons));
  return JSON.parse(result) as Gir;
}
```

**Performance note:** Double-serialization (stringify each GIR, then stringify the array)
is intentional — the Rust side expects `Vec<String>` where each string is a GIR JSON.

---

## 6. The 33 WASM Exports

The compiled WASM binary exports 33 functions across two source crates.

### 6.1 From ul-wasm (8 functions)

These are the core UL functions with `catch_unwind` panic safety:

| Function | Signature (TS) | Returns | Description |
|----------|---------------|---------|-------------|
| `parse` | `(input: string) → any` | JS Object (GIR) | Parse UL-Script → GIR via serde-wasm-bindgen |
| `validate` | `(gir_json: string, check_geometry: boolean) → any` | JS Object | 4-layer validation result |
| `render` | `(gir_json: string, width: number, height: number) → string` | SVG string | Render GIR to SVG |
| `deparse` | `(gir_json: string) → string` | UL-Script text | Deparse GIR to canonical text |
| `compose` | `(operation: string, operands_json: string) → any` | JS Object (GIR) | Apply Σ_UL operation |
| `analyze` | `(gir_json: string) → any` | JS Object | Detect operations in a GIR |
| `init` | `() → void` | — | Initialize panic hook |
| `initSync` | built-in | — | Synchronous WASM module loading |

### 6.2 From ul-game (25 functions)

| Function | Signature (TS) | Returns | Description |
|----------|---------------|---------|-------------|
| `parseUlScript` | `(input: string) → any` | JSON string | Parse UL-Script → GIR JSON |
| `deparseGir` | `(gir_json: string) → string` | UL-Script text | GIR → canonical text |
| `validateGir` | `(gir_json: string, check_geo: boolean) → any` | JS Object | 4-layer validation |
| `applyOperation` | `(op: string, operands_json: string) → any` | JSON string | Apply Σ_UL operation |
| `composeGir` | `(a_json: string, b_json: string, op: string) → any` | JSON string | Binary composition |
| `detectOperations` | `(gir_json: string) → any` | JS Array | Detect operations |
| `analyzeStructure` | `(gir_json: string) → any` | JS Object + Maps | Structural analysis report |
| `compareGir` | `(a_json: string, b_json: string, level: string) → any` | JS Object | Erlangen equivalence |
| `renderSvg` | `(gir_json: string, width: number, height: number) → string` | SVG string | Render GIR to SVG |
| `renderGlyphPreview` | `(gir_json: string) → string` | SVG string | Compact 64×64 preview |
| `createContext` | `(config_json: string) → number` | Context ID (u32) | Create game session |
| `evaluate` | `(ctx_id: number, gir_json: string) → any` | JS Object | Evaluate against rules |
| `scoreComposition` | `(ctx_id: number, gir_json: string, target_json: string) → any` | JS Object | Score against puzzle |
| `evaluateJaneAttempt` | `(ctx_id: number, attempt_json: string, expected_json: string) → any` | JS Object | Learning assessment |
| `validateSequence` | `(ctx_id: number, glyphs_json: string) → any` | JS Object | Sequence validation |
| `getAnimationSequence` | `(gir_json: string, width: number, height: number) → any` | JS Object | Animation keyframes |
| `layout` | `(gir_json: string, width: number, height: number) → any` | JS Object | Positioned geometry |
| `loadCustomDefinitions` | `(ctx_id: number, rules_json: string) → any` | JS Object | Custom rules |
| `getHints` | `(attempt_json: string, target_json: string) → any` | JS Array | Contextual hints |
| `analyzeHints` | `(gir_json: string) → any` | JS Array | Standalone hints |
| `getNextPuzzle` | `(proficiency_json: string) → any` | JS Object | Adaptive puzzle |
| `queryLexicon` | `(query: string) → any` | JS Array | Search lexicon |
| `lookupLexiconEntry` | `(name: string) → any` | JS Object or null | Lookup by name |
| `set_force` | `(gir_json: string, force: string) → any` | JS Object (GIR) | Set illocutionary force |
| `infer_pragmatics` | `(gir_json: string) → any` | JS Array | Pragmatic inference |

### 6.3 Duplicate Functions

**Note:** Some functions exist in both `ul-wasm` and `ul-game` with different names
and slightly different serialization:

| Purpose | ul-wasm export | ul-game export | Used by wrapper |
|---------|---------------|----------------|-----------------|
| Parse | `parse` | `parseUlScript` | `parseUlScript` |
| Validate | `validate` | `validateGir` | `validateGir` |
| Render | `render` | `renderSvg` | `renderSvg` |
| Deparse | `deparse` | `deparseGir` | `deparseGir` |
| Compose | `compose` | `applyOperation` | `applyOperation` |
| Analyze | `analyze` | `detectOperations` | `detectOperations` |

The TypeScript wrapper (`core/index.ts`) uses the `ul-game` versions because they
are the ones re-exported from wasm-pkg. The `ul-wasm` versions are also available
but not imported by the wrapper. If you use raw WASM directly, prefer the `ul-game`
versions for consistency.

---

## 7. TypeScript Wrapper Layer

The typed wrapper in `web/src/core/index.ts` is the recommended API surface for all
TypeScript/JavaScript consumers. It provides:

### 7.1 What the Wrapper Does

1. **Type safety** — All inputs and outputs use TypeScript interfaces (not `any`)
2. **Serialization management** — Handles `JSON.stringify` / `JSON.parse` at the boundary
3. **Initialization guard** — Every function checks `ensureInit()` before calling WASM
4. **Result caching** — 4 LRU caches for render, layout, evaluation, structure analysis
5. **Convenience functions** — `parseAndRender()`, `parseValidateRender()`, `extractGirFromSvg()`
6. **Dual-path handling** — Some WASM functions return strings, others objects; wrapper normalizes

### 7.2 Import Pattern

```typescript
// Recommended: import all from the wrapper
import * as core from "@ul-forge/core";

// Or import specific functions
import { parse, render, validate, applyOperation } from "@ul-forge/core";

// Or import types only
import type { Gir, Sort, NodeType, OperationName } from "@ul-forge/core";
```

The `@ul-forge/core` alias is configured in:
- **Vite:** `vite.config.ts` → `resolve.alias`
- **TypeScript:** `tsconfig.json` → `compilerOptions.paths`

### 7.3 Type Hierarchy

```
Gir ──────────────── The core data structure
├── Node[]           ├── id: string
│                    ├── type: NodeType (point|line|angle|curve|enclosure)
│                    ├── sort: Sort (entity|relation|modifier|assertion)
│                    ├── label?: string
│                    ├── shape?: EnclosureShape
│                    ├── directed?: boolean (for lines)
│                    ├── measure?: number (for angles)
│                    └── curvature?: number (for curves)
├── Edge[]           ├── source: string (node ID)
│                    ├── target: string (node ID)
│                    └── type: EdgeType
├── root: string     (ID of the root node)
├── ul_gir: string   (version — "0.1.0" from parser, "1.0" from some operations)
└── metadata?        (source, generated_by, timestamp)
```

### 7.4 Sort / NodeType / EdgeType Enums

```typescript
type Sort = "entity" | "relation" | "modifier" | "assertion";

type NodeType = "point" | "line" | "angle" | "curve" | "enclosure";

type EdgeType =
  | "contains"       // Enclosure contains child node
  | "modified_by"    // Node is modified by a modifier
  | "adjacent"       // Spatial adjacency
  | "intersects"     // Geometric intersection
  | "connects"       // Directed/undirected connection
  | "references"     // Variable binding reference
  | "binds"          // Variable binder
  | "accessible_from"; // Modal accessibility relation
```

See [TypeScript API Reference](typescript-api-reference.md) for the complete type catalog.

---

## 8. Caching System

The wrapper implements 4 LRU caches to avoid redundant WASM calls in game loops
and interactive editing:

| Cache | Key | Value | Use Case |
|-------|-----|-------|----------|
| `renderCache` | `girHash:WxH` | SVG string | Avoid re-rendering unchanged glyphs |
| `layoutCache` | `girHash:WxH` | PositionedGlyph | Avoid re-computing geometry |
| `evaluationCache` | `ctxId:girHash` | EvaluationResult | Avoid re-evaluating same GIR |
| `structureCache` | `girHash` | StructureReport | Avoid re-analyzing structure |

### 8.1 Cache Key: `girHash()`

```typescript
function girHash(gir: Gir): string {
  return JSON.stringify({ r: gir.root, n: gir.nodes.length, e: gir.edges.length });
}
```

**Warning: This hash is weak.** Two GIRs with the same root ID, same node count,
and same edge count will hash identically even if their structure differs.
This can cause cache collisions in rare cases (e.g., `○{●}` and `○{∠90}` if they
happen to have the same counts).

**Mitigation:** Call `clearCaches()` when switching between unrelated GIRs.
The wrapper does not automatically detect collisions.

### 8.2 LRU Eviction

Each cache holds at most 256 entries. When full, the oldest entry (first inserted)
is evicted. This is a simple FIFO eviction, not true LRU (recently-accessed entries
are not promoted).

### 8.3 Manual Cache Control

```typescript
import { clearCaches } from "@ul-forge/core";

// Clear all 4 caches
clearCaches();
```

Call `clearCaches()`:
- When the user starts a new document
- When switching game contexts
- When cache staleness is suspected
- After `loadCustomDefinitions()` (rules change invalidates evaluation cache)

---

## 9. Testing WASM Modules

### 9.1 Test Environments

The project uses **Vitest** with per-file environment matching:

```typescript
// vitest.config.ts
environmentMatchGlobs: [
  ["src/__tests__/wasm-integration.test.ts", "node"],      // raw WASM
  ["src/__tests__/semantic-structural.test.ts", "node"],    // raw WASM
  ["src/__tests__/core-integration.test.ts", "node"],       // wrapper layer
  // ... more WASM tests use "node" environment
  ["src/__tests__/App.test.tsx", "jsdom"],                  // React components
]
```

WASM tests run in `node` environment because they need filesystem access to
load the `.wasm` binary via `readFileSync`.

### 9.2 Raw WASM Test Pattern

```typescript
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

let wasm: any;

beforeAll(async () => {
  const wasmPath = resolve(__dirname, "../../wasm-pkg/ul_wasm_bg.wasm");
  const wasmModule = await import("../../wasm-pkg/ul_wasm.js");
  const wasmBuffer = readFileSync(wasmPath);
  wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
  wasmModule.init();
  wasm = wasmModule;
});

it("parses a point", () => {
  const json = wasm.parseUlScript("●");
  const gir = JSON.parse(json);
  expect(gir.nodes.length).toBeGreaterThanOrEqual(1);
});
```

### 9.3 Wrapper Test Pattern

```typescript
import * as core from "../../core/index.ts";
import wasmInit, { init as wasmModuleInit, initSync } from "ul-wasm";

beforeAll(async () => {
  const wasmPath = resolve(__dirname, "../../wasm-pkg/ul_wasm_bg.wasm");
  const wasmBuffer = readFileSync(wasmPath);
  initSync({ module: new WebAssembly.Module(wasmBuffer) });
  wasmModuleInit();
  await core.initialize();
});

it("parse returns typed Gir", () => {
  const gir = core.parse("●");
  expect(gir.ul_gir).toBeTruthy();
  expect(gir.nodes[0].sort).toBe("entity");
});
```

### 9.4 Test Coverage (1,358 tests)

| Test File | Tests | Layer | Focus |
|-----------|-------|-------|-------|
| `wasm-integration.test.ts` | 33 | Raw WASM | Basic function calls |
| `semantic-structural.test.ts` | 77 | Raw WASM | Edge types, sort safety |
| `algebraic-depth.test.ts` | 73 | Raw WASM | Σ_UL operation algebra |
| `deparser-characterization.test.ts` | 62 | Raw WASM | Deparse canonical forms |
| `sort-boundary.test.ts` | 107 | Raw WASM | Sort constraint verification |
| `invariant-laws.test.ts` | 42 | Raw WASM | Algebraic invariant laws |
| `cross-operation.test.ts` | 54 | Raw WASM | Multi-operation combinations |
| `edge-adversarial.test.ts` | 143 | Raw WASM | Adversarial inputs |
| `roundtrip-stress.test.ts` | 172 | Raw WASM | Parse-deparse-reparse cycles |
| `agent-pipeline.test.ts` | 27 | Raw WASM | Agent workflow simulation |
| `scifi-scenarios.test.ts` | 44 | Raw WASM | Complex domain scenarios |
| `agent-scenarios.test.ts` | 99 | Raw WASM | Agent-to-agent protocol |
| **`core-integration.test.ts`** | **236** | **Wrapper** | **All 25+ wrapper functions** |
| `core.test.ts` | 35 | Mocked | Wrapper argument passing |
| `perf.test.ts` | 14 | Raw WASM | Performance benchmarks |
| Other (UI, store, etc.) | ~40 | jsdom | React components |

---

## 10. Deployment Targets

### 10.1 Static Site (Vite)

The standard deployment for the web editor:

```bash
cd ul-forge/web
npm run build    # outputs to dist/
```

Serve `dist/` from any static hosting (GitHub Pages, Netlify, Vercel, S3).
The WASM binary is served alongside JS and loaded asynchronously.

**Vite configuration for WASM:**

```typescript
// vite.config.ts
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [react(), wasm()],
  resolve: {
    alias: {
      "ul-wasm": path.resolve(__dirname, "wasm-pkg/ul_wasm.js"),
    },
  },
  optimizeDeps: {
    exclude: ["ul-wasm"],  // Don't pre-bundle the WASM glue
  },
});
```

### 10.2 npm Package

For publishing to npm as `@ul-forge/core`:

```bash
wasm-pack build --target bundler crates/ul-wasm --out-dir ../../dist/npm
cd dist/npm
npm publish --access public
```

Consumers install:

```bash
npm install @ul-forge/core
```

### 10.3 CDN (Web Components)

Zero-build distribution via `<script>` tag:

```html
<script src="https://cdn.jsdelivr.net/npm/@ul-forge/components/dist/ul-components.iife.js"></script>
<ul-symbol script="● → ●" width="200" height="200"></ul-symbol>
```

The web components bundle inlines the WASM binary as base64 for zero-fetch deployment.

### 10.4 Docker (API Server)

The REST API server runs as a native binary (not WASM), but shares the same
`ul-core` library:

```bash
docker pull ghcr.io/jthora/ul-forge:latest
docker run -p 4200:4200 ghcr.io/jthora/ul-forge:latest
```

### 10.5 Node.js Runtime

For server-side Node.js usage (e.g., build scripts, SSR):

```typescript
import { readFileSync } from "node:fs";
import * as wasmModule from "./wasm-pkg/ul_wasm.js";

const wasmBuffer = readFileSync("./wasm-pkg/ul_wasm_bg.wasm");
wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
wasmModule.init();

// Use raw WASM functions directly
const girJson = wasmModule.parseUlScript("○{● -> ●}");
const svg = wasmModule.renderSvg(girJson, 512, 512);
```

---

## 11. Troubleshooting

### 11.1 Common Build Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `error: crate type cdylib not available` | Missing WASM target | `rustup target add wasm32-unknown-unknown` |
| `wasm-bindgen version mismatch` | CLI version ≠ crate version | Pin `wasm-bindgen = "=0.2.99"` in Cargo.toml |
| `import.meta.url is not defined` | Wrong `--target` for environment | Use `--target web` for browser, `--target nodejs` for Node |
| `TypeError: wasmInit is not a function` | Importing wrong init function | Use `import wasmInit from "ul-wasm"` (default export) |
| `RuntimeError: unreachable` | Panic in Rust code | Check Rust code for panics; ensure `catch_unwind` wraps all exports |

### 11.2 Common Runtime Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `WASM not initialized` | Called function before `initialize()` | Await `core.initialize()` before calling any function |
| `parse error at line 1, column 2: expected Number` | Bare `∠` without angle value | Use `∠90`, `∠45`, etc. — the angle symbol requires a numeric argument |
| `parse error: expected EOI, Operator, or Content` | Invalid UL-Script syntax | Check syntax — `●──●` (em-dash) is not valid; use `● -> ●` |
| `missing field expected_gir_json` | Wrong `scoreComposition` target format | Target must be `JSON.stringify({ expected_gir_json: girJsonString })` |
| `No context with id N` | Using invalid context ID | Create context first with `createContext()` |
| `primitive_counts.point is undefined` | Accessing Map as object | Use `.get("point")` — serde-wasm-bindgen returns `Map` for Rust `HashMap` |

### 11.3 Version Mismatches

The `ul_gir` field in parsed GIR varies depending on the code path:

| Source | `ul_gir` Value | Notes |
|--------|---------------|-------|
| `parseUlScript("●")` | `"0.1.0"` | Standard parser output |
| `set_force(gir, "assert")` | `"1.0"` | Force-tagged GIR uses different version |

Do not hardcode `ul_gir` checks to a specific version string. Use `gir.ul_gir != null`
or `typeof gir.ul_gir === "string"`.

---

*Last updated: 2026-04-09*
