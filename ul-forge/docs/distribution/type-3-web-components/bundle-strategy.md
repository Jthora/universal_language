# Type 3 — Bundle Strategy

> Build tooling, WASM inlining, and distribution for zero-dependency web components.

---

## Goals

1. Ship a **single JavaScript file** that works with a `<script>` tag — no npm, no build step required
2. Also ship an **ESM module** for bundler-aware consumers
3. Inline the WASM binary so consumers never need to deal with `.wasm` file hosting
4. Keep total bundle size under **150 KB gzipped**

---

## Build Architecture

```
packages/web-components/
├── src/
│   ├── wasm-loader.ts          # Shared WASM init (inline base64)
│   ├── styles.ts               # Shared CSS-in-JS for Shadow DOM
│   ├── ul-symbol.ts            # <ul-symbol> component class
│   ├── ul-composer.ts          # <ul-composer> component class
│   ├── ul-dictionary.ts        # <ul-dictionary> component class
│   └── index.ts                # Barrel: register all 3 components
├── vite.config.ts
├── package.json
├── tsconfig.json
└── dist/
    ├── ul-components.iife.js   # Drop-in <script> bundle
    ├── ul-components.iife.js.gz
    ├── ul-components.es.js     # ESM import
    └── ul-components.d.ts      # Type declarations
```

---

## WASM Inlining

The WASM binary must be embedded in the JavaScript bundle so that consumers don't need to host a separate `.wasm` file.

### Strategy: Base64 Inline

```typescript
// wasm-loader.ts
// At build time, Vite transforms `?inline` imports to base64 strings
import wasmBase64 from '../../../crates/ul-wasm/pkg/ul_wasm_bg.wasm?inline';
import init, { parse, validate, render, deparse } from '../../../crates/ul-wasm/pkg/ul_wasm';

let wasmReady: Promise<void> | null = null;

export function ensureWasmLoaded(): Promise<void> {
  if (!wasmReady) {
    wasmReady = (async () => {
      const bytes = Uint8Array.from(atob(wasmBase64), c => c.charCodeAt(0));
      await init(bytes);
    })();
  }
  return wasmReady;
}

export function getWasm() {
  return { parse, validate, render, deparse };
}
```

### Why not `fetch()`?

`fetch()` requires the WASM file to be served from a URL, which breaks the zero-dependency promise. Inlining trades ~33% size overhead (base64 encoding) for zero configuration.

### Size Budget

| Component | Estimated Size |
|-----------|---------------|
| WASM binary (pre-gzip) | ~80 KB |
| WASM base64 overhead (+33%) | ~107 KB |
| JS components + loader | ~15 KB |
| **Total pre-gzip** | **~122 KB** |
| **Total gzipped** | **~60-80 KB** |

WASM compresses very well (typically 50-70% ratio), so gzip brings the total well under the 150 KB budget.

---

## Vite Configuration

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import { resolve } from 'path';
import dts from 'vite-plugin-dts';

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, 'src/index.ts'),
      name: 'ULComponents',
      formats: ['iife', 'es'],
      fileName: (format) => `ul-components.${format}.js`,
    },
    rollupOptions: {
      // No externals — everything is self-contained
    },
    target: 'es2020',
    minify: 'terser',
    terserOptions: {
      compress: {
        pure_getters: true,
        passes: 2,
      },
    },
  },
  plugins: [
    dts({ rollupTypes: true }),
  ],
});
```

### Key Decisions

| Decision | Choice | Reason |
|----------|--------|--------|
| Bundler | Vite | Already used by web editor; `?inline` for WASM |
| Formats | IIFE + ESM | Zero-build users get IIFE; bundler users get ESM |
| Target | ES2020 | Custom Elements v1 requires modern browsers anyway |
| Minifier | Terser | Better minification than esbuild for production |
| Types | vite-plugin-dts | Single `.d.ts` rollup for ESM consumers |

---

## IIFE Bundle (Zero-Build)

The IIFE bundle auto-registers all three custom elements on load:

```html
<script src="https://cdn.jsdelivr.net/npm/@ul-forge/web-components/dist/ul-components.iife.js"></script>

<!-- Components are ready immediately -->
<ul-symbol script="point(existence)"></ul-symbol>
<ul-composer></ul-composer>
```

The IIFE wrapper:

```javascript
(function(global) {
  "use strict";
  // ... bundled code ...
  // Auto-registers: customElements.define('ul-symbol', ULSymbol)
  //                 customElements.define('ul-composer', ULComposer)
  //                 customElements.define('ul-dictionary', ULDictionary)
  global.ULComponents = { ULSymbol, ULComposer, ULDictionary };
})(typeof globalThis !== 'undefined' ? globalThis : self);
```

---

## ESM Bundle (Bundler-Aware)

```typescript
// Consumer usage
import '@ul-forge/web-components';
// or selective:
import { ULSymbol, ULComposer } from '@ul-forge/web-components';
```

### index.ts (barrel)

```typescript
export { ULSymbol } from './ul-symbol';
export { ULComposer } from './ul-composer';
export { ULDictionary } from './ul-dictionary';

// Auto-register if not already defined
if (typeof customElements !== 'undefined') {
  if (!customElements.get('ul-symbol')) {
    const { ULSymbol } = await import('./ul-symbol');
    customElements.define('ul-symbol', ULSymbol);
  }
  if (!customElements.get('ul-composer')) {
    const { ULComposer } = await import('./ul-composer');
    customElements.define('ul-composer', ULComposer);
  }
  if (!customElements.get('ul-dictionary')) {
    const { ULDictionary } = await import('./ul-dictionary');
    customElements.define('ul-dictionary', ULDictionary);
  }
}
```

---

## package.json

```json
{
  "name": "@ul-forge/web-components",
  "version": "0.1.0",
  "type": "module",
  "license": "CC0-1.0",
  "description": "UL-Script rendering and editing as drop-in web components",
  "main": "./dist/ul-components.iife.js",
  "module": "./dist/ul-components.es.js",
  "types": "./dist/ul-components.d.ts",
  "exports": {
    ".": {
      "import": "./dist/ul-components.es.js",
      "require": "./dist/ul-components.iife.js",
      "types": "./dist/ul-components.d.ts"
    }
  },
  "files": [
    "dist/",
    "README.md"
  ],
  "keywords": ["universal-language", "web-components", "wasm", "custom-elements"],
  "scripts": {
    "build": "vite build",
    "dev": "vite",
    "preview": "vite preview",
    "type-check": "tsc --noEmit",
    "size": "gzip -c dist/ul-components.iife.js | wc -c"
  },
  "devDependencies": {
    "typescript": "^5.7.0",
    "vite": "^6.0.0",
    "vite-plugin-dts": "^4.0.0",
    "terser": "^5.30.0"
  }
}
```

---

## CDN Deployment

### jsDelivr (primary)

Auto-available after npm publish:
```
https://cdn.jsdelivr.net/npm/@ul-forge/web-components@latest/dist/ul-components.iife.js
```

### unpkg (fallback)
```
https://unpkg.com/@ul-forge/web-components@latest/dist/ul-components.iife.js
```

### Self-hosted

Download `dist/ul-components.iife.js` and serve from any static file server.

---

## Build Pipeline

```
wasm-pack build crates/ul-wasm --target web
        │
        ▼
  pkg/ul_wasm_bg.wasm  +  pkg/ul_wasm.js
        │
        ▼
  vite build (inlines .wasm as base64)
        │
        ├──► dist/ul-components.iife.js    (self-registering, zero-dep)
        ├──► dist/ul-components.es.js      (tree-shakeable ESM)
        └──► dist/ul-components.d.ts       (TypeScript declarations)
```

### CI Step (GitHub Actions)

```yaml
  build-web-components:
    runs-on: ubuntu-latest
    needs: [build-wasm]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: '20' }
      - uses: actions/download-artifact@v4
        with: { name: wasm-pkg, path: crates/ul-wasm/pkg }

      - name: Install deps
        working-directory: packages/web-components
        run: npm ci

      - name: Build
        working-directory: packages/web-components
        run: npm run build

      - name: Size check
        working-directory: packages/web-components
        run: |
          SIZE=$(gzip -c dist/ul-components.iife.js | wc -c)
          echo "Gzipped size: $SIZE bytes"
          if [ "$SIZE" -gt 153600 ]; then
            echo "::error::Bundle exceeds 150 KB gzipped"
            exit 1
          fi

      - name: Publish (on tag)
        if: startsWith(github.ref, 'refs/tags/web-components-v')
        run: npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

---

## Testing

### Unit Tests (Vitest)

```typescript
import { describe, it, expect, beforeAll } from 'vitest';
import { ULSymbol } from '../src/ul-symbol';

// register components
beforeAll(() => {
  customElements.define('ul-symbol', ULSymbol);
});

describe('ul-symbol', () => {
  it('renders SVG from script attribute', async () => {
    const el = document.createElement('ul-symbol');
    el.setAttribute('script', 'point(existence)');
    document.body.appendChild(el);

    await el.updateComplete; // wait for WASM init + render
    const svg = el.shadowRoot?.querySelector('svg');
    expect(svg).toBeTruthy();
  });

  it('emits ul-render event', async () => {
    const el = document.createElement('ul-symbol');
    const rendered = new Promise(resolve => {
      el.addEventListener('ul-render', resolve, { once: true });
    });
    el.setAttribute('script', 'point(existence)');
    document.body.appendChild(el);
    const event = await rendered;
    expect(event).toBeTruthy();
  });
});
```

### Browser Tests

Use Playwright to verify actual rendering in Chrome, Firefox, Safari:

```typescript
test('ul-symbol renders in real browser', async ({ page }) => {
  await page.setContent(`
    <script src="/dist/ul-components.iife.js"></script>
    <ul-symbol script="point(existence)"></ul-symbol>
  `);
  const svg = await page.locator('ul-symbol').evaluateHandle(
    el => el.shadowRoot?.querySelector('svg')
  );
  expect(svg).toBeTruthy();
});
```
