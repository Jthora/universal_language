# SVG Output Generation

> How positioned GIR nodes are converted to SVG elements.

---

## SVG Structure

```xml
<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg"
     xmlns:ul="urn:ul:gir:1.0"
     viewBox="0 0 {width} {height}"
     width="{width}" height="{height}">
  
  <!-- GIR metadata for structure recovery -->
  <metadata>
    <ul:gir version="1.0" hash="{sha256_of_gir}">
      <![CDATA[ {gir_json} ]]>
    </ul:gir>
  </metadata>
  
  <!-- Render hash for sync detection -->
  <desc class="ul-render-hash">{sha256_of_gir}</desc>
  
  <!-- Style definitions -->
  <defs>
    <style>
      .ul-mark { stroke: black; stroke-width: 2; fill: none; }
      .ul-point { fill: black; }
      .ul-edge { stroke: black; stroke-width: 1.5; }
      .ul-edge-dashed { stroke-dasharray: 5,3; }
      .ul-arrowhead { fill: black; }
    </style>
    <marker id="arrowhead" ...> ... </marker>
  </defs>
  
  <!-- Glyph rendering -->
  <g class="ul-glyph" data-ul-root="{root_id}">
    <!-- Elements ordered by z-index (enclosures first, points last) -->
    ...
  </g>
</svg>
```

---

## Node → SVG Element Mapping

| GIR node type | SVG element | Attributes |
|--------------|-------------|------------|
| `point` | `<circle>` | `cx`, `cy`, `r=3`, `class="ul-point"` |
| `line` | `<line>` + `<marker>` | `x1`, `y1`, `x2`, `y2`, arrowhead marker |
| `angle` | `<path>` (arc) | `d` (SVG arc path), degree label |
| `curve` | `<path>` (Bézier) | `d` (cubic Bézier) |
| `enclosure` (circle) | `<circle>` | `cx`, `cy`, `r`, `class="ul-mark"` |
| `enclosure` (triangle) | `<polygon>` | `points` (3 vertices), `class="ul-mark"` |
| `enclosure` (square) | `<rect>` | `x`, `y`, `width`, `height`, `class="ul-mark"` |

Each SVG element carries `data-ul-node="{node_id}"` for interactive linking.

---

## Z-Ordering

Elements are rendered in this order (back to front):
1. Cross-edge paths (connections, references) — drawn behind everything
2. Enclosures — largest (outermost) first
3. Lines and curves
4. Angles (arcs)
5. Points — drawn on top

This ensures points are always visible (not hidden behind enclosures) and connections appear behind the marks they connect.

---

## Edge → SVG Element Mapping

| GIR edge type | SVG rendering |
|--------------|---------------|
| `contains` | No visual element (containment is spatial) |
| `modified_by` | No visual element (modifier decorates parent) |
| `adjacent` | No visual element (proximity is spatial) |
| `intersects` | No visual element (overlap is spatial) |
| `connects` | `<path>` or `<line>` with optional arrowhead |
| `references` | `<path class="ul-edge-dashed">` (dashed line) |

Only `connects` and `references` edges produce visible SVG elements. The other edge types are expressed through spatial positioning of the nodes.

---

## Scale and Units

- Default canvas: 400×400 SVG units
- 1 SVG unit = 1 CSS pixel at default zoom
- Stroke width: 2 units (marks), 1.5 units (connections)
- Point radius: 3 units
- Minimum mark size: 6 units (smallest enclosure radius)
- ViewBox adjusts to content: glyphs smaller than canvas are centered, glyphs larger are scaled to fit

---

## Coordinate System

SVG uses top-left origin (y increases downward). UL Forge's internal coordinate system also uses top-left origin to avoid coordinate transformation errors.

```
(0,0) ──────→ x
  │
  │    glyph
  │
  ▼
  y
```

---

## Accessibility

Each rendered glyph includes minimal accessibility attributes:

```xml
<g class="ul-glyph" role="img" aria-label="{glyph_label}">
  <title>{glyph_label}</title>
  <!-- visual elements -->
</g>
```

Where `glyph_label` is derived from the root node's label, or "UL glyph" if no label is set.

Full accessibility mapping (screen readers, tactile displays) is out of scope for v1. See [architecture/decision-log.md](../../architecture/decision-log.md).

---

## TikZ Output

For academic papers, the renderer can produce TikZ commands:

```latex
\begin{tikzpicture}
  \draw (2,2) circle (1.6);              % n1: concept enclosure
  \fill (2,2) circle (0.06);             % n2: existence point
  \draw[->] (3.6,2) -- (5,2);           % n3: connection line
  \draw (5,2) -- ++(60:1) -- ++(180:1) -- cycle;  % n4: structure triangle
  \fill (5,2.33) circle (0.06);         % n5: point inside triangle
\end{tikzpicture}
```

TikZ output follows the same layout computation — only the output format differs.
