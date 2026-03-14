# Edge Type Reference

> Complete semantics, constraints, and layout effects for each GIR edge type.

---

## Summary Table

| Type | Spatial relationship | Tree-spine | Cycles allowed | Sort constraint on source | Sort constraint on target |
|------|---------------------|------------|----------------|--------------------------|--------------------------|
| `contains` | Containment (A inside B) | Yes | No | Entity (enclosure) | Any |
| `modified_by` | Modification (M applied to X) | Yes | No | Entity or Relation | Modifier |
| `adjacent` | Adjacency (A touches B) | No | No | Any | Any |
| `intersects` | Intersection (A overlaps B) | No | No | Any | Any |
| `connects` | Connection (A directed to B) | No | Possible | Any | Any |
| `references` | Cross-reference (A refers to B) | No | Yes | Any | Any |

---

## `contains`

**Spatial meaning:** Source enclosure completely surrounds target. Target is "inside" source.

**Σ_UL mapping:** Nesting of entities. `embed(a) → e` produces containment when an assertion becomes an entity within a concept.

**Constraints:**
- Source must be an enclosure node (`type: "enclosure"`)
- Source sort must be `entity`
- Must be acyclic: no node can contain itself (directly or transitively)
- Each target has at most one `contains` parent (tree property)

**Layout effect:** Target is rendered inside source's bounding box. The renderer allocates space within the enclosure and places children according to the constraint solver.

**Example:** "Point inside circle" = existence within concept
```json
{ "source": "n1", "target": "n2", "type": "contains" }
```

---

## `modified_by`

**Spatial meaning:** Target modifier applies to source, changing its quality or measure.

**Σ_UL mapping:** `modify_entity(m, e) → e` or `modify_relation(m, r) → r`.

**Constraints:**
- Target must have sort `modifier` (typically `angle` nodes)
- Source must have sort `entity` or `relation`
- Must be acyclic
- Each target has at most one `modified_by` parent (tree property)

**Layout effect:** Modifier is rendered as a decoration on the source — e.g., an angle arc between two lines, or a quality mark on an enclosure.

**Example:** "Line modified by 60° angle"
```json
{ "source": "n2", "target": "n3", "type": "modified_by" }
```

---

## `adjacent`

**Spatial meaning:** Source and target share a boundary point but do not overlap. They are "next to" each other.

**Σ_UL mapping:** `disjoin(a₁, a₂)` — logical OR, or sequential composition.

**Constraints:**
- Source and target should be at the same tree depth (siblings) or in separate branches
- No sort restriction

**Layout effect:** Marks are rendered sharing a boundary point or line segment. The shared boundary is visually apparent.

**Example:** "Circle adjacent to triangle" = concept-or-structure
```json
{ "source": "n1", "target": "n5", "type": "adjacent" }
```

---

## `intersects`

**Spatial meaning:** Source and target partially overlap. There is a shared region that belongs to both.

**Σ_UL mapping:** `conjoin(a₁, a₂)` — logical AND. The overlap is the conjunction.

**Constraints:**
- Source and target should be at the same or adjacent tree depths
- Not meaningful between a point and an enclosure (point is either inside or outside — use `contains` instead)
- No sort restriction

**Layout effect:** Marks are rendered overlapping. The shared region is visually identifiable. Marks inside the overlap belong to both parent enclosures simultaneously.

**Example:** "Circle overlaps triangle" = shared meaning between concept and structure
```json
{ "source": "n1", "target": "n5", "type": "intersects" }
```

---

## `connects`

**Spatial meaning:** A directed relation from source to target. Visually, a line or arrow is drawn between them.

**Σ_UL mapping:** `predicate(e₁, r, e₂) → a` — the relation `r` connecting subject `e₁` to object `e₂`.

**Constraints:**
- No sort restriction (any node can relate to any other)
- Cycles are possible (A connects to B connects to A) — represents mutual or circular relation
- Direction matters: source→target is distinct from target→source

**Layout effect:** An arrow or line is routed between the bounding boxes of source and target. The route avoids crossing other marks when possible.

**Example:** "Concept connects to structure" = concept-relates-to-structure
```json
{ "source": "n1", "target": "n5", "type": "connects" }
```

---

## `references`

**Spatial meaning:** A semantic cross-reference — source "refers to" or "is about" target. May or may not have a visual representation.

**Σ_UL mapping:** Used for `embed` (assertion nominalized into entity that references its assertion) and self-reference.

**Constraints:**
- **Cycles are explicitly allowed.** This is how self-referential glyphs work (a concept that references itself).
- No sort restriction

**Layout effect:** Typically rendered as a dashed line, or not rendered at all (semantic-only link). The web editor may show references on hover.

**Example:** "Self-aware concept" = enclosure references itself
```json
{ "source": "n1", "target": "n1", "type": "references" }
```

---

## Edge Ordering in Serialization

Edges are serialized in this order:
1. Tree-spine edges (`contains`, `modified_by`) in parent→child DFS order
2. Cross-edges (`adjacent`, `intersects`, `connects`, `references`) in lexicographic order of `(source_id, target_id)`

This deterministic ordering ensures identical serialization for identical graphs.
