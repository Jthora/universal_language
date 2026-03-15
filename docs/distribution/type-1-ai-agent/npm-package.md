# Type 1 — npm Package: `@ul-forge/core`

> The primary JavaScript/TypeScript distribution of UL Forge.

---

## Package Identity

```json
{
  "name": "@ul-forge/core",
  "version": "0.1.0",
  "description": "Universal Language Forge — parse, validate, and render UL glyphs (WASM-powered)",
  "license": "CC0-1.0"
}
```

---

## What's Inside

```
@ul-forge/core/
  dist/
    index.js               # ESM entry — re-exports SDK + initializer
    index.d.ts             # TypeScript declarations
    ul_wasm_bg.wasm        # WASM binary (~200KB)
    ul_wasm.js             # wasm-bindgen JS glue
    ul_wasm.d.ts           # WASM type declarations
  schemas/
    gir.schema.json        # GIR document JSON Schema
  README.md                # Usage guide + machine-readable metadata
  package.json
```

---

## Usage

### ESM (Node.js / bundlers)

```typescript
import { initialize, parse, validate, render, deparse } from '@ul-forge/core';

await initialize();

const gir = parse('enclosure(circle, point(existence))');
const result = validate(gir);
if (result.valid) {
  const svg = render(gir, { width: 400, height: 400 });
  console.log(svg);
}
```

### CommonJS

Not supported. ESM only — this aligns with wasm-bindgen output and modern Node.js.

### Browser (via bundler)

Works with Vite, Webpack 5+, esbuild, Rollup — any bundler that supports WASM ESM integration.

For zero-bundler browser usage, see Type 3 (Web Components) instead.

---

## Package Composition

The npm package is assembled from 3 sources during the build:

| Source | Provides |
|--------|----------|
| `packages/sdk/dist/` | TypeScript SDK (types, wasm-bridge, utils) |
| `crates/ul-wasm/pkg/` | WASM binary + JS glue + .d.ts |
| `schemas/` | GIR JSON Schema files |

### Build Script

```bash
#!/bin/bash
# scripts/build-npm-package.sh

set -euo pipefail

# 1. Build WASM
cd crates/ul-wasm
wasm-pack build --target bundler --release
cd ../..

# 2. Build SDK
cd packages/sdk
npm run build
cd ../..

# 3. Assemble
mkdir -p dist/npm
cp packages/sdk/dist/* dist/npm/
cp crates/ul-wasm/pkg/ul_wasm_bg.wasm dist/npm/
cp crates/ul-wasm/pkg/ul_wasm.js dist/npm/
cp crates/ul-wasm/pkg/ul_wasm.d.ts dist/npm/
cp -r schemas/ dist/npm/schemas/
cp packages/sdk/README.md dist/npm/
cp packages/sdk/package.json dist/npm/

# 4. Publish
cd dist/npm
npm publish --access public
```

---

## package.json (Full)

```json
{
  "name": "@ul-forge/core",
  "version": "0.1.0",
  "description": "Universal Language Forge — parse, validate, and render UL glyphs (WASM-powered)",
  "type": "module",
  "main": "index.js",
  "module": "index.js",
  "types": "index.d.ts",
  "exports": {
    ".": {
      "import": "./index.js",
      "types": "./index.d.ts"
    },
    "./schemas/gir": "./schemas/gir.schema.json"
  },
  "files": [
    "index.js",
    "index.d.ts",
    "ul_wasm_bg.wasm",
    "ul_wasm.js",
    "ul_wasm.d.ts",
    "schemas/",
    "README.md"
  ],
  "keywords": [
    "universal-language",
    "glyph",
    "symbology",
    "wasm",
    "formal-language",
    "geometric-primitives"
  ],
  "license": "CC0-1.0",
  "repository": {
    "type": "git",
    "url": "https://github.com/Jthora/universal_language",
    "directory": "ul-forge/packages/sdk"
  },
  "engines": {
    "node": ">=18"
  }
}
```

---

## README.md Machine-Readable Section

Include at the top of the README for agent discovery:

```markdown
<!-- AGENT-READABLE -->
<!--
package: @ul-forge/core
operations: [parse, validate, render, deparse, parseAndRender, parseValidateRender]
input_format: UL-Script (text) or GIR (JSON)
output_format: SVG (string), TikZ (string), GIR (JSON)
schema: ./schemas/gir.schema.json
async_init: true (call initialize() before other functions)
wasm_size: ~200KB
-->
```

---

## CI: Publish on Release

```yaml
# .github/workflows/publish-npm.yml
name: Publish npm
on:
  release:
    types: [published]
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: '20', registry-url: 'https://registry.npmjs.org' }
      - run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      - run: bash scripts/build-npm-package.sh
      - run: cd dist/npm && npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```
