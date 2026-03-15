# @ul-forge/sdk

TypeScript type definitions and utilities for the Universal Language WASM module.

## What's included

- **`types.ts`** — Full typed interfaces for all 23 WASM functions (GIR, layout geometry, game evaluation, teaching system, lexicon)
- **`cache.ts`** — `ResultCache` class + `girHash()` for game-loop performance caching

## Usage

```typescript
import type { Gir, PositionedGlyph, EvaluationResult } from "@ul-forge/sdk";
import { ResultCache, girHash } from "@ul-forge/sdk/cache";
```

## For Game Engines (Phaser, PixiJS, etc.)

The `PositionedGlyph` type gives you raw coordinates and shapes instead of SVG strings:

```typescript
import type { PositionedGlyph, PositionedElement } from "@ul-forge/sdk";

// layout() returns PositionedGlyph with elements[] and connections[]
// Each element has { node_id, x, y, shape } ready for game sprites
```

## Peer dependency

Requires `ul-wasm` (the compiled WASM package from `wasm-pack build crates/ul-game`).
