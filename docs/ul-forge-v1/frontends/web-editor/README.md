# Web Editor

> Browser-based glyph composition environment for Universal Language.

---

## Overview

The web editor is the primary user-facing application of UL Forge. It provides:
- **Dual-pane editing** — UL-Script text on the left, live SVG preview on the right
- **Visual glyph builder** — drag-and-drop composition for non-programmers
- **Template palette** — 42 canonical glyphs as starting points
- **Real-time validation** — sort and structure errors highlighted as you type
- **Export** — SVG, TikZ, PNG, GIR JSON download

---

## Technology Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| Framework | React 18+ | Component model, ecosystem, TypeScript support |
| Text editor | Monaco Editor | VS Code's editor — syntax highlighting, autocomplete, error squiggles |
| SVG canvas | D3.js (selection/zoom/pan) | Mature SVG manipulation, force-directed layout optional |
| GIR operations | ul-forge-core (WASM) | Same Rust library used by CLI and API, compiled to WebAssembly |
| State management | Zustand | Lightweight, TypeScript-native store |
| Styling | Tailwind CSS | Utility-first, minimal CSS maintenance |

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Web Editor                        │
│                                                     │
│  ┌──────────────┐          ┌──────────────────┐    │
│  │  Monaco       │  edit   │  SVG Preview      │    │
│  │  (UL-Script)  │────────→│  (D3 + rendered   │    │
│  │               │         │   SVG from WASM)  │    │
│  └──────────────┘          └──────────────────┘    │
│         │                           │               │
│         ▼                           ▼               │
│  ┌──────────────┐          ┌──────────────────┐    │
│  │  WASM Bridge  │         │  Visual Builder   │    │
│  │  (parse,      │←────────│  (drag-drop       │    │
│  │   validate,   │  GIR    │   mark placement) │    │
│  │   render)     │         │                   │    │
│  └──────────────┘          └──────────────────┘    │
│         │                                           │
│         ▼                                           │
│  ┌──────────────────────────────────────────┐      │
│  │  GIR Store (Zustand)                      │      │
│  │  Single source of truth for glyph state   │      │
│  └──────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────┘
```

---

## Key Components

### Monaco Integration

Custom language definition for UL-Script:
- **Syntax highlighting** — Unicode marks in one colour, operators in another, brackets in a third
- **Autocomplete** — suggest Unicode glyphs from ASCII input (typing `circle` suggests `◯`)
- **Error squiggles** — underline parse and validation errors via WASM bridge
- **Hover info** — show sort and geometric type of the mark under cursor

### SVG Preview Pane

- Rendered by calling `render()` on the WASM module with the current GIR
- D3 provides zoom, pan, and click-to-select
- Clicking an SVG element highlights the corresponding UL-Script text (and vice versa) via `data-ul-node` attributes
- Re-renders on every debounced edit (~100ms)

### Visual Glyph Builder

For users who prefer visual composition over text:
- Palette of 5 geometric primitives (point, line, angle, curve, enclosure)
- Drag primitives onto canvas
- Draw edges by connecting marks
- Each visual action generates a GIR operation → updates GIR store → updates both text and SVG views

### Template Palette

- Shows all 42 canonical lexicon glyphs as clickable thumbnails
- Clicking inserts the glyph's UL-Script at cursor position
- Templates grouped by sort: Entity, Relation, Modifier, Assertion

---

## Bidirectional Sync

The GIR store is the single source of truth. Both the text editor and visual builder read from and write to the store:

```
Text Edit → parse(text) → GIR Store → render(gir) → SVG
Visual Edit → modify(gir) → GIR Store → deparse(gir) → Text
```

This ensures text and visual views are always consistent.

---

## Deployment Options

### Static Build (Recommended for v1)

```bash
npm run build    # produces dist/ with HTML, JS, WASM
```

Deploy to any static hosting (GitHub Pages, Netlify, Vercel). No server required — all processing happens in the browser via WASM.

### With API Server

```bash
ul serve --port 3000 &       # start API server
npm run dev                   # start web editor with API backend
```

In this mode, the editor can offload parsing/rendering to the server instead of WASM (useful for very large glyphs or when WASM performance is insufficient).

---

## User Workflows

See [workflows.md](workflows.md) for the 5 primary composer workflows.
