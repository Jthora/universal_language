# Graph-with-Tree-Spine: The GIR Data Model

> Why UL-GIR uses a graph, why it has a tree spine, and how the two work together.

---

## The Problem

UL glyphs have five spatial relationships: containment, intersection, adjacency, separation, and connection. Only containment is naturally hierarchical (tree-like). The others create cross-links, shared ownership, and even cycles:

- **Intersection:** Two marks overlap. The point in the overlap belongs to *both* marks. In a tree, every node has exactly one parent. Shared ownership breaks this.
- **Connection:** A directed line connects marks in different branches of the hierarchy. This is a cross-reference — an edge that isn't parent→child.
- **Self-reference:** A glyph can refer to itself (the concept of "self-awareness" contains itself). This creates a cycle. Trees are acyclic by definition.

A pure tree (nested JSON objects with `children` arrays) cannot represent these. Flattening them into metadata or annotations creates a second representation that can desynchronize with the tree — the same "lying document" problem we identified for SVG.

**Solution:** The GIR is a graph, serialized cleanly as two arrays (nodes and edges). The tree structure is extracted as a *view* — a spanning tree of containment edges — not the primary representation.

---

## Structure

```json
{
  "ul_gir": "1.0",
  "root": "n1",
  "nodes": [
    { "id": "n1", "type": "enclosure", "shape": "circle", "label": "concept" },
    { "id": "n2", "type": "point", "label": "existence" },
    { "id": "n3", "type": "line", "direction": [1, 0], "label": "relation" },
    { "id": "n4", "type": "angle", "measure": 60, "label": "quality" },
    { "id": "n5", "type": "enclosure", "shape": "circle", "label": "sub-concept" }
  ],
  "edges": [
    { "source": "n1", "target": "n2", "type": "contains" },
    { "source": "n1", "target": "n3", "type": "contains" },
    { "source": "n3", "target": "n4", "type": "modified_by" },
    { "source": "n1", "target": "n5", "type": "contains" },
    { "source": "n2", "target": "n5", "type": "intersects" },
    { "source": "n5", "target": "n1", "type": "references" }
  ]
}
```

---

## Node Schema

Each node represents a geometric primitive or composite structure.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Unique identifier. Format: `n{integer}` for local, UUID for collaborative. |
| `type` | enum | Yes | One of: `point`, `line`, `angle`, `curve`, `enclosure` |
| `label` | string | No | Human-readable label (from Lexicon or user-assigned) |
| `shape` | string | For enclosure | `circle`, `ellipse`, `polygon`, `freeform` |
| `direction` | [number, number] | For line | Unit vector `[dx, dy]` |
| `measure` | number | For angle | Degrees in `[0, 360)` |
| `curvature` | number | For curve | Curvature parameter (0 = straight, higher = more curved) |
| `sort` | enum | No | Σ_UL sort: `entity`, `relation`, `modifier`, `assertion`. Inferred from type if omitted. |
| `metadata` | object | No | Extensible key-value pairs for annotations |

### Default sort inference

| Node type | Default sort | Rationale |
|-----------|-------------|-----------|
| `point` | Entity | Points are existences — atomic entities |
| `line` | Relation | Lines are directed connections between entities |
| `angle` | Modifier | Angles measure the quality between relations |
| `curve` | Relation | Curves are parameterized paths (composed relations) |
| `enclosure` | Entity | Enclosures bound concepts — composite entities |

Sort can be overridden when a node plays a different role via `embed` (Assertion → Entity) or `abstract` (Entity → Modifier).

---

## Edge Schema

Each edge represents a spatial or semantic relationship between nodes.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source` | string | Yes | ID of source node |
| `target` | string | Yes | ID of target node |
| `type` | enum | Yes | Relationship type (see taxonomy below) |
| `metadata` | object | No | Extensible properties |

---

## Edge Type Taxonomy

| Type | Meaning | Tree-spine? | Creates cycles? | Layout effect |
|------|---------|-------------|-----------------|---------------|
| `contains` | Source encloses target | **Yes** | No (must be acyclic) | Target placed inside source's bounding box |
| `modified_by` | Modifier applies to source | **Yes** | No (must be acyclic) | Modifier decorates source visually |
| `adjacent` | Source touches target at boundary | No | No | Marks share a boundary point |
| `intersects` | Source and target overlap | No | No | Marks drawn overlapping; shared region exists |
| `connects` | Directed relation from source to target | No | Possible (A→B→A) | Arrow/line drawn between marks |
| `references` | Semantic cross-reference | No | **Yes** (intended) | Dashed line or invisible link |

### Tree-Spine Edges

The tree spine is the subgraph formed by `contains` and `modified_by` edges. It must satisfy:
1. **Acyclic:** No node is its own ancestor via contains/modified_by
2. **Connected:** Every node is reachable from `root` via tree-spine edges
3. **Single-parent:** Each node has at most one incoming `contains` or `modified_by` edge

The tree spine provides:
- Top-down bounding box allocation for the layout engine
- Deterministic traversal order for serialization
- A natural "outline" of the glyph's structure

### Cross-edges

All edges that are NOT `contains` or `modified_by` are cross-edges. They are:
- Not required to be acyclic (`references` edges can create cycles)
- Rendered as visual overlaps (`intersects`), visual connections (`connects`, `adjacent`), or semantic links (`references`)
- Routed by the layout engine after the tree-spine layout is computed

---

## Tree-Spine Extraction Algorithm

```
function extract_tree(gir):
    spine_edges = [e for e in gir.edges if e.type in {"contains", "modified_by"}]
    
    // Validate tree properties
    parent_count = count incoming spine edges per node
    assert all(count <= 1 for count in parent_count.values())
    assert no_cycles(gir.root, spine_edges)
    assert all_reachable(gir.root, spine_edges, gir.nodes)
    
    return Tree(root=gir.root, edges=spine_edges)
```

---

## Graph Invariants

The validator enforces these invariants on every GIR document:

1. **Root exists:** `gir.root` references a valid node
2. **ID uniqueness:** All node IDs are distinct
3. **Edge validity:** All edge source/target reference existing nodes
4. **Tree-spine validity:** `contains` + `modified_by` edges form a valid rooted tree spanning all nodes
5. **Contains acyclicity:** No cycles through `contains` edges
6. **Modified_by acyclicity:** No cycles through `modified_by` edges
7. **Sort compatibility:** Edges respect sort constraints:
   - `modified_by`: source is Entity or Relation, target is Modifier
   - `contains`: source is Entity (enclosure), target is any sort
8. **Geometry satisfiability:** Angle measures in `[0, 360)`, directions are unit vectors, curvatures are non-negative
9. **Intersection depth:** `intersects` edges connect nodes at the same or adjacent tree depths (you can't intersect a top-level glyph with a deeply nested sub-glyph)

---

## Why JSON (Still)

Despite the graph model, JSON remains the best serialization:

- The two-array format (`nodes` + `edges`) is clean, standard JSON
- Every JSON tool can read it — `jq`, Python `json`, JavaScript `JSON.parse()`
- The graph structure is *explicit* in the edges, not *implicit* in nesting
- Streaming parsers can handle large documents (nodes and edges are top-level arrays)
- Schema validation via JSON Schema covers all structural invariants

Alternatives considered:
- **RDF/JSON-LD:** Better graph model but worse human/AI readability, heavier parser dependency
- **Protobuf:** Better performance but not human-readable at all
- **SQLite:** Better query performance but not a document format

See [implementation-roster.md §2](../../../ul-core/composition/implementation-roster.md) for the full evaluation.

---

## Example: "The concept of knowledge that contains the process of learning"

**UL-Script:**
```
○{ △{●} |→ ~{●} }
```

**GIR:**
```json
{
  "ul_gir": "1.0",
  "root": "n1",
  "nodes": [
    { "id": "n1", "type": "enclosure", "shape": "circle", "label": "concept" },
    { "id": "n2", "type": "enclosure", "shape": "triangle", "label": "structure" },
    { "id": "n3", "type": "point", "label": "existence" },
    { "id": "n4", "type": "line", "direction": [1, 0], "label": "relation" },
    { "id": "n5", "type": "curve", "curvature": 0.5, "label": "process" },
    { "id": "n6", "type": "point", "label": "existence" }
  ],
  "edges": [
    { "source": "n1", "target": "n2", "type": "contains" },
    { "source": "n2", "target": "n3", "type": "contains" },
    { "source": "n1", "target": "n4", "type": "contains" },
    { "source": "n4", "target": "n5", "type": "connects" },
    { "source": "n5", "target": "n6", "type": "contains" }
  ]
}
```

**Tree spine:** n1 → n2 → n3, n1 → n4, n5 → n6  
**Cross-edges:** n4 → n5 (connects)
