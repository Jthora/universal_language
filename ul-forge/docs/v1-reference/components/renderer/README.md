# Renderer Component: `ul-render`

> Converts UL-GIR into visual output (SVG, TikZ, PDF) with geometrically faithful layout.

---

## Responsibility

The renderer takes a validated UL-GIR document and produces visual output where every geometric relationship is visually apparent and metrically correct. A 60° angle looks like 60°. A point inside a circle is drawn inside the circle. Intersecting marks visually overlap.

**Input:** UL-GIR (JSON, `.ul.json`)  
**Output:** SVG (primary), TikZ, PDF, Canvas commands

---

## The Central Problem: Layout

The renderer's hardest job is **layout** — deciding where to place each mark on a 2D surface so that:
1. Containment is visible (children inside parents)
2. Intersections overlap correctly
3. Angles are metrically accurate
4. The result is readable and aesthetically coherent

This is not a simple coordinate lookup. It's a constraint satisfaction problem with geometric fidelity requirements. See [layout-algorithm.md](layout-algorithm.md) for the full approach.

---

## Pipeline

```
UL-GIR (JSON)
  │
  ├── 1. Template lookup      → Known glyph? Return pre-computed SVG
  ├── 2. Tree-spine extraction → Containment hierarchy
  ├── 3. Bounding box allocation → Top-down space assignment
  ├── 4. Constraint solving    → Position marks within boxes
  ├── 5. Cross-edge routing    → Route non-tree edges
  ├── 6. Aesthetic refinement  → Nudge for visual balance
  └── 7. Output generation     → SVG / TikZ / etc.
```

### Stage 1: Template Lookup

The 42 canonical lexicon entries have pre-computed visual forms. If the input GIR matches a known template, skip layout entirely and return the template SVG.

See [template-library.md](template-library.md).

### Stage 2-5: Layout Computation

See [layout-algorithm.md](layout-algorithm.md) and [constraint-solver.md](constraint-solver.md).

### Stage 6: Aesthetic Refinement

Post-layout polish:
- Force-directed nudging for visual balance
- Whitespace equalization between siblings
- Collision detection between non-intersecting marks
- Label placement in available whitespace

### Stage 7: Output Generation

See [svg-generation.md](svg-generation.md).

Convert positioned graph to output format:
- **SVG:** Geometric primitives → SVG elements with exact coordinates. Embed GIR as metadata.
- **TikZ:** Positioned graph → TikZ commands for LaTeX rendering.
- **PDF:** Via SVG → PDF conversion (Phase 4 ecosystem).

---

## API Surface (Rust)

```rust
pub struct RenderOptions {
    pub format: OutputFormat,     // Svg, TikZ, Pdf
    pub width: f64,               // Canvas width in points
    pub height: f64,              // Canvas height in points
    pub stroke_width: f64,        // Default stroke width
    pub embed_gir: bool,          // Embed GIR metadata in SVG
    pub template_fallback: bool,  // Use templates when available
}

pub enum OutputFormat {
    Svg,
    TikZ,
    Pdf,
}

pub struct RenderResult {
    pub output: String,           // SVG/TikZ text, or PDF bytes
    pub layout: Layout,           // Computed positions (for incremental update)
    pub warnings: Vec<Diagnostic>,
}

/// Render GIR to visual output
pub fn render(gir: &Gir, options: &RenderOptions) -> RenderResult;

/// Render with incremental update from previous layout
pub fn render_incremental(
    gir: &Gir,
    previous_layout: &Layout,
    changed_nodes: &[NodeId],
    options: &RenderOptions,
) -> RenderResult;
```

---

## WASM API

```typescript
import { render } from 'ul-forge-core';

const svg = render(gir, {
    format: 'svg',
    width: 400,
    height: 400,
    strokeWidth: 2,
    embedGir: true,
});
// svg.output: string (SVG markup)
```

---

## Visual Conventions

| Primitive | Default visual | Customizable? |
|-----------|---------------|---------------|
| Point | Filled circle, radius 3px | Radius |
| Line | Straight segment with arrowhead | Stroke width, arrowhead style |
| Angle | Arc between two lines | Arc radius, degree label |
| Curve | Bézier path | Stroke width, curvature |
| Enclosure (circle) | Circle, unfilled | Stroke width, radius |
| Enclosure (triangle) | Equilateral triangle, unfilled | Stroke width, size |
| Enclosure (square) | Square, unfilled | Stroke width, size |

Colors are monochrome (black on white) by default, matching the pen-and-paper aesthetic of the Writer's Companion.

---

## Rendering Levels

| Level | What it handles | Complexity | Phase |
|-------|----------------|-----------|-------|
| **Level 1: Templates** | 42 canonical lexicon entries | Low (lookup table) | Phase 1 |
| **Level 2: Hierarchical constraints** | Novel compositions | Medium-High (constraint solver) | Phase 1-2 |
| **Level 3: Aesthetic optimization** | Polish and readability | Medium (force nudging) | Phase 2-3 |

Build Level 1 first — it delivers immediate value. Level 2 is the core engineering. Level 3 is iterative polish.

---

## Related Documents

- [layout-algorithm.md](layout-algorithm.md) — The layout problem and solution
- [template-library.md](template-library.md) — Pre-computed templates for known glyphs
- [constraint-solver.md](constraint-solver.md) — Hierarchical constraint solver
- [svg-generation.md](svg-generation.md) — SVG output format
