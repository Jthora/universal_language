# Template Library

> Pre-computed visual forms for canonical UL glyphs, enabling instant rendering without layout computation.

---

## Purpose

The 42 canonical entries in the UL Lexicon have fixed, known visual forms. Rather than computing layout for these glyphs every time, the template library stores pre-computed SVG paths and positions that can be returned immediately.

Templates are the **Level 1** rendering strategy — they handle the majority of commonly-used glyphs with zero layout computation.

---

## Template Structure

Each template contains:

```rust
struct Template {
    id: String,                 // Template identifier (matches Lexicon entry)
    label: String,              // Human-readable name
    gir_pattern: Gir,           // Graph structure to match (IDs are wildcards)
    svg_elements: Vec<SvgElement>, // Pre-computed visual elements
    bounding_box: BoundingBox,  // Total dimensions
    anchor_points: HashMap<String, Point2D>, // Named attachment points for composition
}
```

**Anchor points** are positions where other marks can attach when this template is used as a component of a larger composition. For example, the "existence" glyph (point in circle) has anchor points at the circle's edges (N, S, E, W) for connecting to other glyphs.

---

## Template Matching

Given a GIR fragment, determine if it matches a template:

```
function match_template(gir_fragment):
    for template in template_library:
        if graph_isomorphic(gir_fragment, template.gir_pattern, ignore_ids=true):
            return template
    return None
```

**Isomorphism check** ignores node IDs and compares:
- Node types and shapes
- Edge types
- Graph topology (adjacency structure)

For the lexicon's 42 entries (typically 2-5 nodes each), this is fast (brute-force comparison in microseconds).

---

## The 42 Templates

Mapped from the Lexicon tiers:

### Tier 1: Geometrically Forced (12 entries)

These have unique visual forms derived directly from the geometric primitives.

| # | Label | UL-Script | Structure | Notes |
|---|-------|-----------|-----------|-------|
| 1 | Existence | `○{●}` | Point in circle | Most fundamental |
| 2 | Relation | `→` | Directed line | Arrow with direction |
| 3 | Quality | `∠60` | Angle mark | Arc with degree |
| 4 | Process | `~` | Curve | Bézier path |
| 5 | Concept | `○` | Circle | Empty enclosure |
| 6 | Structure | `△` | Triangle | 3-sided enclosure |
| 7 | Foundation | `□` | Square | 4-sided enclosure |
| 8 | Point | `●` | Filled dot | Atomic mark |
| 9 | Connection | `● → ●` | Arrow between points | Directed link |
| 10 | Boundary | `○ \| ○` | Adjacent circles | Shared edge |
| 11 | Intersection | `○ & ○` | Overlapping circles | Shared region |
| 12 | Containment | `○{○}` | Circle in circle | Nesting |

### Tier 2: Structurally Distinguished (15 entries)

Compositions that are unique among alternatives for their meaning.

*(Template definitions follow the same pattern — each entry has a GIR pattern and pre-computed SVG. Full enumeration to be completed during implementation, referencing the Lexicon.)*

### Tier 3: Conventional (15 entries)

Compositions chosen by convention from multiple valid alternatives.

*(Full enumeration deferred to implementation.)*

---

## Template Composition

When a composition includes templates as sub-components, the renderer uses the template's anchor points to connect them:

```
Input:  ○{●} → △{●}
Match:  Left term = template "existence" (○{●})
        Right term = template "structure-instance" (△{●})
        Operator = connection (→)

Render: Place "existence" template at left position
        Place "structure-instance" template at right position
        Draw arrow from "existence".anchor_E to "structure-instance".anchor_W
```

This enables efficient rendering of composed glyphs without full constraint solving, as long as all sub-components have templates.

---

## Template Generation

Templates are created by:
1. Hand-designing the SVG for each of the 42 Lexicon entries
2. Extracting the GIR pattern from the canonical definitions
3. Defining anchor points at natural attachment positions

The templates are stored as a static library compiled into the renderer binary. They are not user-configurable in v1 (aesthetic consistency matters for a formal system).

---

## Fallback

When no template matches:
1. Check if sub-components of the glyph have templates → compose using anchor points (see above)
2. If no sub-components match → proceed to Level 2 hierarchical constraint solving

The template library is a fast path, not a requirement. The renderer always has the constraint solver as a fallback.
