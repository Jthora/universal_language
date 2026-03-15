# P1 — Extract `@ul-forge/sdk`

> Move the TypeScript SDK from the web app into a standalone, publishable package.

**Blocks:** Type 1 (npm package), Type 3 (web components), Type 4 (TypeScript transceiver)

---

## Current State

The web app at `ul-forge/web/` contains a well-typed WASM bridge:

```
web/src/core/index.ts    # ~150 lines: typed interfaces, initialize(), parse(), validate(),
                         #              render(), deparse(), parseAndRender(), parseValidateRender()
web/src/wasm.d.ts        # WASM function type declarations
```

These mirror the 4 WASM functions from `ul-wasm` with TypeScript types for `Sort`, `NodeType`, `EnclosureShape`, `EdgeType`, `Node`, `Edge`, `Gir`, `ValidationResult`, `RenderOptions`.

### Problem

- `web/` is a Vite app — `private: true`, not publishable
- The SDK is imported via relative paths, can't be consumed externally
- Types are co-located with React app code

---

## Target Structure

```
ul-forge/packages/sdk/
  src/
    index.ts                 # Public entry: re-exports everything
    types.ts                 # All UL type definitions
    wasm-bridge.ts           # WASM init + function wrappers
    utils.ts                 # Convenience functions (parseAndRender, etc.)
  package.json               # @ul-forge/sdk, publishable
  tsconfig.json              # Strict, ESM output
  README.md                  # Usage docs + API reference
```

### package.json

```json
{
  "name": "@ul-forge/sdk",
  "version": "0.1.0",
  "description": "TypeScript SDK for Universal Language Forge — parse, validate, and render UL glyphs",
  "type": "module",
  "main": "dist/index.js",
  "module": "dist/index.js",
  "types": "dist/index.d.ts",
  "exports": {
    ".": {
      "import": "./dist/index.js",
      "types": "./dist/index.d.ts"
    }
  },
  "files": ["dist/", "README.md"],
  "scripts": {
    "build": "tsc",
    "typecheck": "tsc --noEmit"
  },
  "peerDependencies": {
    "ul-wasm": "*"
  },
  "devDependencies": {
    "typescript": "~5.7.0"
  },
  "license": "CC0-1.0",
  "repository": "https://github.com/Jthora/universal_language"
}
```

### types.ts

Extracted from `web/src/core/index.ts`:

```typescript
export enum Sort {
  Entity = 'Entity',
  Relation = 'Relation',
  Modifier = 'Modifier',
  Assertion = 'Assertion',
}

export enum NodeType {
  Point = 'Point',
  Line = 'Line',
  Angle = 'Angle',
  Curve = 'Curve',
  Enclosure = 'Enclosure',
}

export enum EnclosureShape {
  Circle = 'Circle',
  Triangle = 'Triangle',
  Square = 'Square',
  Ellipse = 'Ellipse',
  Polygon = 'Polygon',
  Freeform = 'Freeform',
}

export enum EdgeType {
  Contains = 'Contains',
  ModifiedBy = 'ModifiedBy',
  Adjacent = 'Adjacent',
  Intersects = 'Intersects',
  Connects = 'Connects',
  References = 'References',
}

export interface Node {
  id: string;
  node_type: NodeType;
  sort: Sort;
  label?: string;
  shape?: EnclosureShape;
  direction?: [number, number];
  directed?: boolean;
  measure?: number;
  curvature?: number;
  vertices?: number;
}

export interface Edge {
  source: string;
  target: string;
  edge_type: EdgeType;
}

export interface Gir {
  nodes: Node[];
  edges: Edge[];
  root: string;
  metadata?: Record<string, unknown>;
}

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

export interface RenderOptions {
  format?: 'svg' | 'tikz';
  width?: number;
  height?: number;
  embed_gir?: boolean;
}
```

### wasm-bridge.ts

```typescript
import type { Gir, ValidationResult, RenderOptions } from './types';

let wasmModule: typeof import('ul-wasm') | null = null;

export async function initialize(): Promise<void> {
  if (wasmModule) return;
  wasmModule = await import('ul-wasm');
  await wasmModule.default();
}

function ensureInitialized(): typeof import('ul-wasm') {
  if (!wasmModule) throw new Error('@ul-forge/sdk not initialized. Call initialize() first.');
  return wasmModule;
}

export function parse(input: string): Gir {
  return ensureInitialized().parse(input);
}

export function validate(gir: Gir): ValidationResult {
  return ensureInitialized().validate(gir);
}

export function render(gir: Gir, options?: RenderOptions): string {
  return ensureInitialized().render(gir, options ?? {});
}

export function deparse(gir: Gir): string {
  return ensureInitialized().deparse(gir);
}
```

---

## Migration Steps

1. **Create `packages/sdk/`** with files above
2. **Add npm workspace** to root: `"workspaces": ["packages/*"]` in `ul-forge/package.json` (create if needed)
3. **Update `web/src/core/index.ts`** to re-export from `@ul-forge/sdk` instead of defining types inline
4. **Update `web/package.json`** to depend on `@ul-forge/sdk: "workspace:*"`
5. **Verify `web/` still builds and tests pass**
6. **Build SDK**: `cd packages/sdk && npm run build` produces `dist/`

---

## Consumers

After extraction, the SDK is consumed by 3 distribution types:

| Consumer | How It Uses the SDK |
|----------|-------------------|
| **Type 1: @ul-forge/core** | Bundles SDK + WASM together as a single npm package |
| **Type 3: Web Components** | Imports SDK for WASM bridge, wraps in Shadow DOM custom elements |
| **Type 4: @ul-forge/transceiver** | Imports types (Gir, ValidationResult) for message payload typing |
