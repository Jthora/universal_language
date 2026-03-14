# UL-GIR Specification v1.0

> The complete specification of the Universal Language Geometric Intermediate Representation.

---

## Overview

UL-GIR is a JSON document format that represents UL glyphs as typed graphs with a tree spine. It is the canonical representation — all other formats (UL-Script, SVG, TikZ) are derived from it.

**Design goals:**
- Faithful representation of all 5 spatial relationships (containment, intersection, adjacency, separation, connection)
- Sort-safe: Σ_UL type constraints are expressible and validatable
- Supports cycles (self-referential glyphs via `references` edges)
- Human-inspectable with standard JSON tools
- Schema-validatable via JSON Schema

---

## Top-Level Structure

### Single-glyph document

```json
{
  "ul_gir": "1.0",
  "root": "n1",
  "nodes": [ ... ],
  "edges": [ ... ],
  "metadata": { ... }
}
```

### Multi-glyph document

```json
{
  "ul_gir": "1.0",
  "glyphs": [
    {
      "id": "g1",
      "root": "n1",
      "nodes": [ ... ],
      "edges": [ ... ]
    },
    {
      "id": "g2",
      "root": "n10",
      "nodes": [ ... ],
      "edges": [ ... ]
    }
  ],
  "metadata": { ... }
}
```

---

## Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `ul_gir` | string | Version string. Format: `"MAJOR.MINOR"`. Current: `"1.0"` |
| `root` | string | ID of the root node (single-glyph) |
| `nodes` | array | Array of node objects |
| `edges` | array | Array of edge objects |

## Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `glyphs` | array | Array of glyph objects (multi-glyph; replaces `root`/`nodes`/`edges`) |
| `metadata` | object | Provenance, generation info, annotations |
| `metadata.source` | string | Source file that produced this GIR |
| `metadata.generated_by` | string | Tool name and version |
| `metadata.timestamp` | string | ISO 8601 timestamp |

---

## Node Object

```json
{
  "id": "n1",
  "type": "enclosure",
  "sort": "entity",
  "shape": "circle",
  "label": "concept",
  "direction": null,
  "measure": null,
  "curvature": null,
  "metadata": {}
}
```

### Required node fields

| Field | Type | Constraints |
|-------|------|-------------|
| `id` | string | Unique within document. Pattern: `n[0-9]+` (local) or UUID (collaborative) |
| `type` | string | One of: `"point"`, `"line"`, `"angle"`, `"curve"`, `"enclosure"` |

### Optional node fields

| Field | Type | Applicable types | Description |
|-------|------|-----------------|-------------|
| `sort` | string | All | `"entity"`, `"relation"`, `"modifier"`, `"assertion"`. Default: inferred from type |
| `label` | string | All | Human-readable label |
| `shape` | string | `enclosure` | `"circle"`, `"triangle"`, `"square"`, `"ellipse"`, `"polygon"`, `"freeform"` |
| `direction` | [number, number] | `line` | Unit vector `[dx, dy]` where dx²+dy²=1 |
| `measure` | number | `angle` | Degrees in `[0, 360)` |
| `curvature` | number | `curve` | Non-negative real. 0 = straight, higher = more curved |
| `vertices` | number | `enclosure` (polygon) | Number of sides for polygon enclosure |
| `metadata` | object | All | Extensible key-value pairs |

### Sort inference

If `sort` is omitted, it defaults based on `type`:

| `type` | Default `sort` | Geometric rationale |
|--------|---------------|-------------------|
| `point` | `entity` | Point = existence = atomic entity |
| `line` | `relation` | Line = directed connection = relation |
| `angle` | `modifier` | Angle = measure between = modifier |
| `curve` | `relation` | Curve = parameterized path = process (composed relation) |
| `enclosure` | `entity` | Enclosure = bounded concept = composite entity |

Sort may be overridden when a node plays a transformed role (e.g., an embedded assertion becoming an entity).

---

## Edge Object

```json
{
  "source": "n1",
  "target": "n2",
  "type": "contains",
  "metadata": {}
}
```

### Required edge fields

| Field | Type | Constraints |
|-------|------|-------------|
| `source` | string | Must reference an existing node ID |
| `target` | string | Must reference an existing node ID |
| `type` | string | One of the 6 edge types (see below) |

### Optional edge fields

| Field | Type | Description |
|-------|------|-------------|
| `metadata` | object | Extensible key-value pairs |

---

## Edge Types

| Type | Tree-spine? | May cycle? | Sort constraint | Layout effect |
|------|-------------|-----------|-----------------|---------------|
| `contains` | Yes | No | source.sort ∈ {entity} | Target inside source bbox |
| `modified_by` | Yes | No | target.sort ∈ {modifier} | Target decorates source |
| `adjacent` | No | No | — | Marks share boundary |
| `intersects` | No | No | — | Marks overlap visually |
| `connects` | No | Possible | — | Arrow/line between marks |
| `references` | No | Yes | — | Semantic link (may be invisible) |

---

## Serialization Order

Nodes and edges are serialized in a deterministic order for diffability:

**Nodes:** Depth-first traversal of the tree spine, starting from root. Children ordered by their first appearance in the edge list.

**Edges:** Tree-spine edges first (in parent→child DFS order), then cross-edges (in source-id, target-id lexicographic order).

This ensures that two GIR documents representing the same glyph produce identical JSON (byte-for-byte), enabling meaningful `git diff` and hash comparison.

---

## JSON Schema

A JSON Schema for validation is provided at [ul-gir-schema.json](ul-gir-schema.json) (to be generated from this spec during build).

**Validation levels:**
1. **Schema validation:** Structure matches JSON Schema (field types, required fields)
2. **Graph validation:** References are valid, tree spine is acyclic, all nodes reachable
3. **Sort validation:** Edges respect Σ_UL sort constraints
4. **Geometry validation:** Measures in range, directions are unit vectors

Level 1 is handled by JSON Schema. Levels 2-4 require `ul-validate`.

---

## Examples

### Minimal: a point

```json
{
  "ul_gir": "1.0",
  "root": "n1",
  "nodes": [{ "id": "n1", "type": "point" }],
  "edges": []
}
```

### Existence: point inside circle

```json
{
  "ul_gir": "1.0",
  "root": "n1",
  "nodes": [
    { "id": "n1", "type": "enclosure", "shape": "circle" },
    { "id": "n2", "type": "point" }
  ],
  "edges": [
    { "source": "n1", "target": "n2", "type": "contains" }
  ]
}
```

### Connection with angle: directed relation at 60°

```json
{
  "ul_gir": "1.0",
  "root": "n1",
  "nodes": [
    { "id": "n1", "type": "point" },
    { "id": "n2", "type": "line", "direction": [0.866, 0.5] },
    { "id": "n3", "type": "angle", "measure": 60 },
    { "id": "n4", "type": "point" }
  ],
  "edges": [
    { "source": "n2", "target": "n3", "type": "modified_by" },
    { "source": "n1", "target": "n2", "type": "connects" },
    { "source": "n2", "target": "n4", "type": "connects" }
  ]
}
```

### Self-referential: concept references itself

```json
{
  "ul_gir": "1.0",
  "root": "n1",
  "nodes": [
    { "id": "n1", "type": "enclosure", "shape": "circle", "label": "self-awareness" },
    { "id": "n2", "type": "point" }
  ],
  "edges": [
    { "source": "n1", "target": "n2", "type": "contains" },
    { "source": "n1", "target": "n1", "type": "references" }
  ]
}
```

Note the `references` self-loop on n1. This is valid — `references` edges may create cycles.
