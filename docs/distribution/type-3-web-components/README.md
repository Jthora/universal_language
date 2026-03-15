# Type 3 — Educational Web Components

> Framework-agnostic custom elements for embedding UL glyphs in any HTML page.

**Status:** Planning  
**Prerequisites:** P1 (SDK extraction)  
**Wave:** 2

---

## Goal

Any website can embed Universal Language glyphs with a single `<script>` tag. No npm, no build tools, no React. Works in plain HTML. Designed for:

- Educational websites teaching UL
- Documentation sites (this repo's own docs)
- Blog posts and interactive tutorials
- Any HTML page that wants to display or compose UL glyphs

---

## Components

| Element | Purpose | Complexity |
|---------|---------|------------|
| `<ul-symbol>` | Display a static rendered glyph from UL-Script | Low |
| `<ul-composer>` | Interactive editor with live preview | Medium |
| `<ul-dictionary>` | Browsable reference of canonical glyphs | Medium |

See individual specs:
- [ul-symbol.md](ul-symbol.md)
- [ul-composer.md](ul-composer.md)

---

## Architecture

```
┌──────────────────────────────────┐
│  Any HTML page                    │
│                                   │
│  <script src="ul-components.js"> │
│                                   │
│  ┌─────────────────────────────┐ │
│  │ Shadow DOM                   │ │
│  │  ┌────────┐  ┌───────────┐  │ │
│  │  │ WASM   │  │ SVG       │  │ │
│  │  │ Bridge │→ │ Renderer  │  │ │
│  │  └────────┘  └───────────┘  │ │
│  │  Styles (encapsulated)       │ │
│  └─────────────────────────────┘ │
│                                   │
│  <ul-symbol script="...">        │
│  <ul-composer>                    │
│  <ul-dictionary>                  │
└──────────────────────────────────┘
```

Each component uses Shadow DOM — styles don't leak in or out. The WASM module is shared across all instances (loaded once, lazily).

---

## Package Structure

```
ul-forge/packages/web-components/
  src/
    index.ts                # Custom element registration + auto-init
    ul-symbol.ts            # <ul-symbol> element
    ul-composer.ts          # <ul-composer> element
    ul-dictionary.ts        # <ul-dictionary> element
    wasm-loader.ts          # Lazy WASM initialization (shared singleton)
    styles.ts               # Shadow DOM CSS templates
  vite.config.ts            # IIFE + ESM build config
  package.json              # @ul-forge/components
  tsconfig.json
  README.md
```

### Bundle Strategy

See [bundle-strategy.md](bundle-strategy.md) for full details.

Two output formats:

| Format | File | Use Case |
|--------|------|----------|
| **IIFE** | `ul-components.iife.js` | `<script>` tag, CDN | 
| **ESM** | `ul-components.esm.js` | Import maps, bundlers |

The IIFE bundle includes WASM inlined as base64 — truly single-file, no external fetches.

---

## Usage

### Minimal (zero build tools)

```html
<!DOCTYPE html>
<html>
<head>
  <script src="https://cdn.jsdelivr.net/npm/@ul-forge/components/ul-components.iife.js"></script>
</head>
<body>
  <!-- Static glyph -->
  <ul-symbol script="point(existence)" width="200" height="200"></ul-symbol>

  <!-- Interactive composer -->
  <ul-composer width="400" height="300"></ul-composer>

  <!-- Dictionary reference -->
  <ul-dictionary columns="3"></ul-dictionary>
</body>
</html>
```

### ESM (with import map)

```html
<script type="importmap">
{
  "imports": {
    "@ul-forge/components": "https://cdn.jsdelivr.net/npm/@ul-forge/components/ul-components.esm.js"
  }
}
</script>
<script type="module">
  import '@ul-forge/components';
</script>
```

### npm (in a build tool project)

```bash
npm install @ul-forge/components
```

```typescript
import '@ul-forge/components';
// Custom elements are now registered and usable in HTML
```

---

## Shared WASM Loader

All components share a single WASM instance:

```typescript
// wasm-loader.ts
let wasmReady: Promise<void> | null = null;
let wasmModule: typeof import('ul-wasm') | null = null;

export async function ensureWasmLoaded(): Promise<void> {
  if (wasmReady) return wasmReady;
  wasmReady = loadWasm();
  return wasmReady;
}

async function loadWasm() {
  // In IIFE bundle: WASM is inlined as base64
  // In ESM bundle: WASM is fetched from sibling URL
  wasmModule = await import('ul-wasm');
  await wasmModule.default();
}

export function getWasm() {
  if (!wasmModule) throw new Error('WASM not loaded yet');
  return wasmModule;
}
```

---

## Theming

Components support a `theme` attribute for light/dark mode:

```html
<ul-symbol script="point(existence)" theme="dark"></ul-symbol>
<ul-symbol script="point(existence)" theme="light"></ul-symbol>
```

Custom CSS properties for advanced theming:

```css
ul-symbol {
  --ul-stroke-color: #333;
  --ul-fill-color: transparent;
  --ul-background: white;
  --ul-font-family: 'Inter', sans-serif;
  --ul-error-color: #cc0000;
}
```

---

## Files to Create

| File | Lines (est.) | Purpose |
|------|-------------|---------|
| `src/index.ts` | ~20 | Registration + auto-init |
| `src/ul-symbol.ts` | ~80 | Static glyph element |
| `src/ul-composer.ts` | ~200 | Interactive editor |
| `src/ul-dictionary.ts` | ~150 | Reference browser |
| `src/wasm-loader.ts` | ~30 | Shared WASM singleton |
| `src/styles.ts` | ~100 | Shadow DOM CSS |
| `vite.config.ts` | ~40 | IIFE/ESM build |
| `package.json` | ~30 | Package manifest |

---

## Success Criteria

1. `<ul-symbol script="point(existence)">` renders in a plain HTML file opened in Chrome, Firefox, and Safari
2. No build tools, no npm, no framework required — just a `<script>` tag
3. Multiple `<ul-symbol>` elements on one page share a single WASM instance
4. `<ul-composer>` provides live preview as the user types
5. Shadow DOM prevents style leakage in both directions
6. IIFE bundle size < 400KB (WASM + JS + CSS)
