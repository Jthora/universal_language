# @ul-forge/core

**Universal Language (UL) — parse, validate, render, and compose geometric meaning structures.**

WASM-powered, zero native dependencies. Works in Node.js 18+ and modern browsers.

## Quick Start

```typescript
import { initialize, parse, render, validate } from '@ul-forge/core';

// Initialize WASM (call once)
await initialize();

// Parse UL-Script to GIR (Graph Intermediate Representation)
const gir = parse('point(existence)');

// Validate against Σ_UL constraints
const result = validate(gir);
console.log(result.valid); // true

// Render to SVG
const svg = render(gir, 200, 200);
```

## What This Package Contains

| Component | Description |
|-----------|-------------|
| **23 typed functions** | Full Σ_UL pipeline: parse → validate → render → deparse → compose |
| **WASM binary** | Compiled from Rust (ul-core + ul-game), ~600KB |
| **GIR JSON Schema** | Machine-readable schema at `@ul-forge/core/schemas/gir` |
| **Type definitions** | Complete TypeScript types for all UL structures |

## API Overview

### Core Pipeline
- `parse(script)` — UL-Script → GIR
- `validate(gir)` — Check Σ_UL constraints (4-layer: schema, sort, invariant, geometry)
- `render(gir, w, h)` — GIR → SVG string
- `renderPreview(gir)` — GIR → 64×64 SVG preview
- `deparse(gir)` — GIR → canonical UL-Script
- `parseAndRender(script)` — UL-Script → SVG (shortcut)
- `parseValidateRender(script)` — Full pipeline with validation

### Algebraic Composer (Σ_UL Operations)
- `applyOperation(op, operands)` — Apply any of the 11 Σ_UL operations
- `composeGir(a, b, op)` — Binary operation shortcut
- `detectOperations(gir)` — Detect operations expressed in a GIR

### Analysis
- `analyzeStructure(gir)` — Node/edge counts, symmetry group, part of speech, complexity
- `compareGir(a, b, level)` — Erlangen equivalence at 5 hierarchy levels
- `layout(gir, w, h)` — Raw positioned geometry (for game engines, not SVG)

### Teaching & Game
- `createContext(config)` — Create evaluation context
- `evaluate(ctx, gir)` — Score against composition rules
- `scoreComposition(ctx, gir, target)` — Score against puzzle target
- `getHints(attempt, target)` — Contextual learning hints
- `getNextPuzzle(proficiency)` — Adaptive difficulty

### Lexicon
- `queryLexicon(query)` — Search 42 canonical UL definitions
- `lookupLexiconEntry(name)` — Exact name lookup

## For AI Agents

This package is designed for autonomous AI consumption:

```yaml
# Machine-readable capability summary
package: "@ul-forge/core"
formal_system: Σ_UL
primitives: [point, line, angle, curve, enclosure]
sorts: [entity, relation, modifier, assertion]
operations: 11
theorems_proven: 23
uniqueness: proven
gir_schema: "@ul-forge/core/schemas/gir"
```

See [AGENTS.md](https://github.com/Jthora/universal_language/blob/main/AGENTS.md) for full integration guide.

## License

CC0-1.0 — Public Domain
