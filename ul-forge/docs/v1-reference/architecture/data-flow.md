# Data Flow

> How data moves through UL Forge, from human keystroke or AI generation to rendered glyph and back.

---

## Primary Flow: UL-Script → Rendered Glyph

```
UL-Script (text)
  │
  │  ① TOKENIZE
  │  Split Unicode stream into tokens:
  │  △ { ● } → @ 60 □ { ● }
  │  ↓
  │
  │  ② PARSE
  │  Build Abstract Syntax Tree from token stream:
  │  Composition(
  │    Mark(triangle, content=[Mark(point)]),
  │    Connection(direction=right, angle=60),
  │    Mark(square, content=[Mark(point)])
  │  )
  │  ↓
  │
  │  ③ TRANSFORM (AST → GIR)
  │  Convert AST nodes to GIR nodes + edges:
  │  nodes: [n1:enclosure/triangle, n2:point, n3:line, n4:angle, n5:enclosure/square, n6:point]
  │  edges: [n1→n2:contains, n1→n5:connects, n3→n4:modified_by, n5→n6:contains]
  │  Assign stable IDs. Validate sort constraints.
  │  ↓
  │
UL-GIR (JSON)
  │
  │  ④ VALIDATE
  │  Check graph invariants:
  │  - Every node reachable from root via tree-spine edges
  │  - contains edges form acyclic tree
  │  - Sort constraints satisfied (Entity where Entity expected, etc.)
  │  - Geometric constraints satisfiable (angles in [0°, 360°), etc.)
  │  ↓
  │
  │  ⑤ LAYOUT
  │  Solve placement:
  │  Level 1: Template lookup for known glyphs (lexicon entries)
  │  Level 2: Hierarchical constraint solver for composed glyphs
  │    - Allocate bounding boxes top-down from tree spine
  │    - Solve geometric constraints within each box
  │    - Pack siblings
  │    - Route cross-reference edges
  │  Level 3: Aesthetic refinement (force-nudge, whitespace, collision avoidance)
  │  ↓
  │
  │  ⑥ RENDER
  │  Generate visual output:
  │  - SVG elements from positioned primitives
  │  - Embed GIR as <metadata> (with content hash)
  │  - Apply stroke weights, fill rules, viewBox
  │  ↓
  │
SVG / TikZ / PDF (visual output)
```

---

## Reverse Flow: SVG → UL-GIR (Extraction)

```
.ul.svg file
  │
  │  ① CHECK HASH
  │  Compare embedded GIR hash with render hash in <desc>
  │  Match → GIR is trustworthy
  │  Mismatch → Warn: SVG was edited outside UL Forge
  │  ↓
  │
  │  ② EXTRACT
  │  Pull JSON-GIR from <metadata><ul:gir> element
  │  ↓
  │
UL-GIR (JSON)
  │
  │  ③ DEPARSE (optional)
  │  Convert GIR back to UL-Script for editing
  │  ↓
  │
UL-Script (text)
```

**Note:** Visual-to-GIR extraction from arbitrary SVG (without embedded metadata) is a computer vision problem and is out of scope for v1.

---

## AI Agent Flow

```
AI GENERATION:
  LLM / code agent
    │
    ├─── Option A: Generate UL-Script text
    │    → POST /parse → GIR
    │    → POST /render → SVG
    │
    └─── Option B: Generate JSON-GIR directly
         → POST /validate → report
         → POST /render → SVG

AI CONSUMPTION:
  .ul.json file or API response
    │
    ├─── LLM: Read as JSON text
    ├─── GNN: Convert to node features + adjacency (see ai-integration/gnn-interface.md)
    ├─── Vision model: Consume rendered SVG/PNG
    └─── Theorem prover: Convert to SMT-LIB/Lean4 assertions
```

---

## Data at Each Stage

| Stage | Format | Size (typical) | Human-readable? | Machine-readable? |
|-------|--------|----------------|-----------------|-------------------|
| UL-Script | Unicode text | 10-500 bytes | Yes (primary) | Yes (parseable) |
| AST | In-memory Rust struct | N/A | No (internal) | N/A |
| UL-GIR | JSON | 200 bytes – 50 KB | Tolerable (with jq) | Yes (primary) |
| Layout | In-memory positioned graph | N/A | No (internal) | N/A |
| SVG | XML | 500 bytes – 500 KB | Yes (visual) | Partial (structure in metadata) |
| TikZ | LaTeX | 200 bytes – 20 KB | Yes (for LaTeX users) | Partial |

---

## Incremental Flow (Live Preview)

During interactive editing, the full pipeline runs on every keystroke, but with caching:

```
Keystroke
  → Incremental tokenize (only re-tokenize changed region)
  → Incremental parse (re-parse affected subtree)
  → Incremental GIR update (patch changed nodes/edges)
  → Incremental layout (re-solve only affected bounding boxes)
  → Incremental SVG update (patch changed DOM elements)

WebSocket protocol:
  Client sends: { "type": "edit", "position": 12, "insert": "△" }
  Server sends: { "type": "patch", "svg_ops": [...], "gir_delta": {...} }
```

The incremental path is an optimization for Phase 3+. Initially, the full pipeline re-runs on every change (fast enough for small documents with WASM).

---

## Error Propagation

Errors are caught at the earliest possible stage and propagated with source location:

```
Stage        Error type               Example
─────        ──────────               ───────
Tokenize     LexError                 "Unknown character ¥ at position 7"
Parse        SyntaxError              "Expected closing } at line 1, col 12"
Transform    SortError                "Modifier applied to Assertion (expected Entity) at node n3"
Validate     GraphError               "Orphan node n5 not reachable from root"
Validate     GeometryError            "Angle measure 400° exceeds [0°, 360°) at node n4"
Layout       LayoutError (warning)    "Cannot satisfy all constraints; relaxing angle n4 by 3°"
Render       RenderError (rare)       "SVG viewBox overflow for glyph of depth 12"
```

Every error includes:
- Source position (line:col in UL-Script, or node ID in GIR)
- Error code (machine-parseable)
- Human-readable message
- Suggestion (when possible)
