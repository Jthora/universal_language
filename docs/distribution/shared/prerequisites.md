# Shared Prerequisites

> Infrastructure work required before any distribution type can be built.

**Depends on:** ul-forge v1.1 (S1–S6 complete)  
**Blocks:** All 4 distribution types

---

## Overview

Three preparatory tasks extract shared capabilities from the monolithic codebase so that each distribution type builds on clean, reusable foundations instead of duplicating work.

| ID | Task | What It Unlocks |
|----|------|----------------|
| **P1** | [Extract `@ul-forge/sdk`](sdk-extraction.md) | Types 1, 3, 4 get a publishable TypeScript SDK |
| **P2** | Expose layout geometry from ul-core | Type 2 gets raw positioned geometry instead of SVG strings |
| **P3** | [Generate GIR JSON Schema](schema-generation.md) | Types 1, 4 get machine-readable schema for discovery + validation |

---

## P1 — Extract `@ul-forge/sdk`

**Full spec:** [sdk-extraction.md](sdk-extraction.md)

The typed wrappers in `web/src/core/index.ts` are well-designed but trapped inside the Vite web app. This task moves them into a standalone, publishable package.

### Current State

```
web/src/core/index.ts    ← TypeScript SDK (types, initialize, parse, validate, render, deparse)
web/src/wasm.d.ts        ← WASM type declarations
```

### Target State

```
packages/sdk/
  src/
    index.ts             # Re-export everything
    types.ts             # Sort, NodeType, EdgeType, EnclosureShape, Node, Edge, Gir, etc.
    wasm-bridge.ts       # initialize(), parse(), validate(), render(), deparse()
    utils.ts             # parseAndRender(), parseValidateRender()
  package.json           # @ul-forge/sdk, NOT private
  tsconfig.json
  README.md
```

### Steps

1. Create `packages/sdk/` directory with package.json
2. Move types and wasm-bridge code from `web/src/core/index.ts`
3. Update `web/` to import from `../packages/sdk/src` (or use npm workspace link)
4. Verify web app still works
5. Add build script (tsc → dist/)

---

## P2 — Expose Layout Geometry

The renderer's internal `layout.rs` module contains `PositionedGlyph`, `PositionedElement`, `Shape`, and `Connection` — raw 2D geometry that the SVG generator consumes. These types are not currently exported from `ul-core`'s public API.

Type 2 (ProtoFusionGirl) needs this raw geometry to feed into Phaser's rendering system instead of receiving SVG strings.

### Current Public API (renderer)

```rust
// ul-core/src/renderer/mod.rs
pub struct RenderOptions { format, width, height, embed_gir }
pub enum OutputFormat { Svg, TikZ }
pub fn render(gir: &Gir, options: &RenderOptions) -> UlResult<String>  // ← returns String
pub use layout::Layout;  // ← only Layout is re-exported
```

### Target Public API (additions)

```rust
// ul-core/src/lib.rs — add to public exports:
pub use renderer::layout::{
    PositionedElement,
    PositionedGlyph,
    Shape,
    Connection,
    compute_layout,   // new public function
};
```

### Changes Required

1. In `ul-core/src/renderer/layout.rs`:
   - Make `compute_layout` public (it already is within the module — just needs re-export path)
   - Derive `Serialize` on `PositionedGlyph`, `PositionedElement`, `Shape`, `Connection` (for WASM JSON transfer)

2. In `ul-core/src/renderer/mod.rs`:
   - Add `pub use layout::{PositionedElement, PositionedGlyph, Shape, Connection, compute_layout};`

3. In `ul-core/src/lib.rs`:
   - Re-export the new public types

### Non-Breaking

This is purely additive — no existing public API changes. All current consumers work unchanged.

---

## P3 — Generate GIR JSON Schema

**Full spec:** [schema-generation.md](schema-generation.md)

Using the `schemars` crate, derive JSON Schema from the Rust types. This gives machines (AI agents, protocol validators, IDE extensions) a way to validate GIR documents without running Rust code.

### Deliverables

```
schemas/
  gir.schema.json                  # JSON Schema for Gir struct
  validation-result.schema.json    # JSON Schema for ValidationResult
  render-options.schema.json       # JSON Schema for RenderOptions
```

### Steps

1. Add `schemars = { version = "0.8", features = ["derive"] }` to ul-core dependencies
2. Derive `JsonSchema` on `Gir`, `Node`, `Edge`, `Sort`, `NodeType`, `EdgeType`, `EnclosureShape`, `ValidationResult`, `RenderOptions`
3. Create `ul-core/src/schema.rs` with `pub fn generate_schemas() -> HashMap<String, String>`
4. Add CLI subcommand: `ul-forge schema generate --output schemas/`
5. Add CI step to regenerate schemas and fail if they drift from committed versions

---

## Implementation Order

```
P1 (SDK extraction) ──┐
                       ├──→ Wave 2 (Types 1 + 3)
P3 (JSON Schema) ─────┘
                       
P2 (Layout geometry) ────→ Wave 3 (Type 2 game module)

All three ────────────────→ Wave 4 (Type 4 protocol)
```

P1 and P2 are independent of each other and can proceed in parallel. P3 depends on P2 (schema should include the newly-exposed geometry types).

---

## Exit Criteria

| ID | Criterion |
|----|-----------|
| P1 | `packages/sdk/` builds, exports typed WASM wrappers, web app imports from it successfully |
| P2 | `compute_layout()` is callable from external crates; `PositionedGlyph` derives `Serialize` |
| P3 | `schemas/gir.schema.json` validates a known-good GIR JSON document; CI checks schema drift |
