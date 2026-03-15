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
Each entry has a deterministic visual form derived from its geometric structure.

> **Cross-reference:** Lexicon entries at `ul-core/lexicon/lexicon.md`.
> Sort signatures from Σ_UL in `foundations/formal-foundations.md`.

| # | Label | UL-Script | Sort | GIR Pattern | Visual Description | Anchors |
|---|-------|-----------|------|-------------|--------------------|---------| 
| 13 | Efficiency | `∠120` | Modifier | `n1(angle, measure=120)` | Arc spanning 120°; supplement of harmony (60°). Interior angle of regular hexagon. | N: vertex, E: arm-1 tip, W: arm-2 tip |
| 14 | Spiral | `𝒮` | Relation | `n1(curve, curvature=increasing)` | Curve with monotonically increasing radius; rendered as Archimedean spiral (2 turns). | N: outer end, S: center origin |
| 15 | Sine Wave | `~~` | Relation | `n1(curve, curvature=alternating)` | Symmetric S-curve with alternating curvature; single period rendered as cubic Bézier. | W: start, E: end |
| 16 | Atom | `△{●}` | Entity | `n1(triangle), n2(point); n1→contains→n2` | Point centered in equilateral triangle. Minimal polygon enclosure + simplest entity. | N: apex, SE: right vertex, SW: left vertex |
| 17 | Constraint | `□{●}` | Entity | `n1(square), n2(point); n1→contains→n2` | Point centered in square. Systematic enclosure bounding existence. | N: top-center, S: bottom-center, E: right-center, W: left-center |
| 18 | Potential | `●→` | Entity | `n1(point), n2(line, direction=right, open=true)` | Point with outgoing ray, no target. Open-ended directed relation. | W: point, E: arrowhead (open) |
| 19 | Duality | `● ∠180 ●` | Assertion | `n1(point), n2(point), n3(line), n4(angle, measure=180); n3→modified_by→n4, n1→connects→n3, n3→connects→n2` | Two points on opposite ends of a line. Maximal opposition between two entities. | W: left point, E: right point |
| 20 | Trinity | `△{●,●,●}` | Assertion | `n1(triangle), n2-n4(point×3); n1→contains→n2,n3,n4; all pairwise angles=60°` | Three points at triangle vertices, each pairwise related at 60°. Perfect balance among minimum plurality. | N: apex point, SE: right point, SW: left point |
| 21 | Quantification | `[m]{e}` | Assertion | `n1(modifier, scale=s), n2(entity); quantify(n1, n2)→a` | Entity scaled within a reference frame. Large fills frame (universal); small centered (particular). | N: top frame, S: bottom frame |
| 22 | Negation | `¬(a)` | Assertion | `n1(assertion); negate(n1)→a'` | Reflected/inverted assertion frame. Horizontal mirror of contained glyph with strike-through bar. | N: top-center, S: bottom-center |
| 23 | Embedding | `⟦a⟧` | Entity | `n1(assertion), n2(circle); embed(n1)→n2; n2→contains→n1` | Assertion frame shrunk into circle enclosure. Nominalization: statement → thing. | N/S/E/W: circle boundary |
| 24 | Abstraction | `⟨e⟩` | Modifier | `n1(entity); abstract(n1)→m` | Entity's shape-properties extracted as modifier. Boundary drawn as dashed outline. | N: modifier attachment, S: source entity |
| 25 | Composition | `●→●→●` | Relation | `n1-n3(point×3), e1-e2(line×2); compose(e1,e2)→e3` | Chain of two directed connections sharing a middle point. Transitive sequential relation. | W: start, center: midpoint, E: end |
| 26 | Inversion | `●←●` | Relation | `n1(point), n2(point), n3(line, direction=left); invert(r)→r'` | Arrow reversed — same pair, opposite direction. Labeled with inversion marker (⟲). | W: target, E: source |
| 27 | Modify Entity | `∠θ ● ` | Entity | `n1(angle, measure=θ), n2(point); modify_entity(n1, n2)→n2'` | Modifier arc applied to entity mark. Arc drawn adjacent to the modified entity. | N: modifier vertex, S: entity center |

### Tier 3: Conventional (15 entries)

Compositions chosen by convention from multiple valid alternatives.
The visual forms are standard by agreement; geometric forcing does not uniquely determine them.

| # | Label | UL-Script | Sort | GIR Pattern | Visual Description | Anchors |
|---|-------|-----------|------|-------------|--------------------|---------| 
| 28 | Void | `Ø` | — | Empty GIR (no nodes, no edges) | Blank glyph space or slashed circle (`∅` symbol). Pre-linguistic null. | none |
| 29 | Angle (generic) | `∠θ` | Modifier | `n1(angle, measure=θ)` | Two rays sharing a vertex with arc at measure θ. Generic quality modifier. | N: vertex, E: arm-1, W: arm-2 |
| 30 | Empty Enclosure | `○{}` | Entity | `n1(circle, children=0)` | Circle with visibly empty interior. Concept without instantiation. | N/S/E/W: circle boundary |
| 31 | Identity | `∠0` | Modifier | `n1(angle, measure=0)` | Two coincident rays (single direction). Zero inclination rendered as parallel lines. | N: vertex, E: direction |
| 32 | Independence | `∠90` | Modifier | `n1(angle, measure=90)` | Right angle with small square corner mark (standard geometric notation). | N: vertex, E: arm-1, N: arm-2 |
| 33 | Opposition | `∠180` | Modifier | `n1(angle, measure=180)` | Straight line through vertex (supplementary angle). Involution marker. | W: arm-1, E: arm-2 |
| 34 | Completion | `∠360` | Modifier | `n1(angle, measure=360)` | Full rotation arc around vertex (circular arrow). Topologically distinct from 0°. | N: vertex, arc around |
| 35 | Constructed Absence | `○{Ø}` | Entity | `n1(circle), interior=void` | Circle with explicit void interior marker (small `∅` or dashed center). Not same as empty enclosure. | N/S/E/W: circle boundary |
| 36 | Cycle | `○` (process) | Relation | `n1(circle, sort=Relation)` | Circle read as closed process (constant curvature, looping). Arrow on circumference. | N: top, clockwise arrow |
| 37 | Coincident Points | `●═●` | Assertion | `n1(point), n2(point), n3(line, length=0)` | Two concentric filled dots (double ring) — two entities at same position. | center: shared position |
| 38 | Modify Relation | `∠θ →` | Relation | `n1(angle), n2(line); modify_relation(n1,n2)→n2'` | Arc drawn at the line's source, tilting line direction by θ. | N: angle vertex, E: line endpoint |
| 39 | Pentagon | `⬠` | Entity | `n1(pentagon)` | Regular 5-sided polygon. Golden-ratio diagonals. Organic/living convention. | Vertices at 5 compass points |
| 40 | Hexagon | `⬡` | Entity | `n1(hexagon)` | Regular 6-sided polygon. Optimal circle-packing. Network/communal convention. | Vertices at 6 compass points |
| 41 | Aspiration | `●→○` | Assertion | `n1(point), n2(line), n3(circle); n1→connects→n2, n2→connects→n3` | Point directing relation toward empty concept. Entity striving toward undefined goal. | W: point, E: circle |
| 42 | Analogy | `○──○` | Assertion | `n1(circle), n2(circle), n3(line, undirected); n1→connects→n3, n3→connects→n2` | Two circles connected by undirected line. Concept-relates-to-concept. | W: circle-1, E: circle-2 |

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
