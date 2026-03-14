# AST → GIR Transformation

> Rules for converting the parser's Abstract Syntax Tree into the graph-with-tree-spine GIR.

---

## Overview

The AST is a tree that mirrors UL-Script syntax. The GIR is a graph that represents geometric structure. The transformation:
1. Assigns a unique node ID to each primitive
2. Creates `contains` edges from brace nesting
3. Creates `modified_by` edges from angle modifiers
4. Creates cross-edges from operators (`|`, `→`, `&`)
5. Determines the root node
6. Validates tree-spine properties

---

## Transformation Rules

### Rule 1: Primitive → Node

Each primitive in the AST becomes a GIR node.

| AST node | GIR node type | Additional fields |
|----------|--------------|-------------------|
| `Point` | `point` | — |
| `Enclosure(Circle)` | `enclosure` | `shape: "circle"` |
| `Enclosure(Triangle)` | `enclosure` | `shape: "triangle"` |
| `Enclosure(Square)` | `enclosure` | `shape: "square"` |
| `Arrow(Right)` | `line` | `direction: [1, 0]` |
| `Arrow(Left)` | `line` | `direction: [-1, 0]` |
| `Arrow(Both)` | `line` | `direction: [1, 0]` (bidirectional flag) |
| `Curve` | `curve` | `curvature: 0.5` (default) |
| `Angle(N)` | `angle` | `measure: N` |

### Rule 2: Content braces → `contains` edges

When a `Mark` has `Content`:
```
Mark { primitive: P, content: Some(Composition) }
```

The mark node becomes a parent, and all top-level terms in the composition become children via `contains` edges.

```
Input:  ○{● | △{●}}
        ├── ○ (n1, enclosure/circle)
        ├── ● (n2, point)
        └── △ (n3, enclosure/triangle)
            └── ● (n4, point)

Edges:
  n1 →contains→ n2
  n1 →contains→ n3
  n3 →contains→ n4
```

### Rule 3: Angle modification → `modified_by` edges

When a `Connection` has an angle:
```
Connection { direction: Right, angle: Some(60.0) }
```

An angle node is created and attached to the connection's line node via `modified_by`.

```
Input:  ● →@60 ●
Nodes:  n1 (point), n2 (line, direction=[0.866, 0.5]), n3 (angle, measure=60), n4 (point)
Edges:  n2 →modified_by→ n3
        n1 →connects→ n2
        n2 →connects→ n4
```

The line's direction vector is computed from the angle: `[cos(60°), sin(60°)] = [0.5, 0.866]`.

### Rule 4: Operators → cross-edges

| AST operator | GIR edge type |
|-------------|---------------|
| `Adjacency (\|)` | `adjacent` |
| `Connection (→)` | `connects` |
| `Intersection (&)` | `intersects` |

Cross-edges connect the **root marks** of the left and right operands:

```
Input:  ○{●} | △{●}
Nodes:  n1 (circle), n2 (point), n3 (triangle), n4 (point)
Edges:  n1 →contains→ n2
        n3 →contains→ n4
        n1 →adjacent→ n3    ← the adjacency connects the enclosures, not the points inside
```

### Rule 5: Root determination

The GIR root is determined by:
1. If the glyph has a single top-level mark → that mark is root
2. If the glyph has multiple top-level terms connected by operators → create an implicit root enclosure containing all terms
3. If the implicit root is not needed (single enclosure at top level) → use that enclosure

```
Input:  ○{●}           → root = n1 (the circle)
Input:  ○{●} | △{●}    → root = implicit enclosure containing both
Input:  ● → ●          → root = implicit enclosure containing the full composition
```

---

## ID Assignment

IDs are assigned deterministically based on AST position:

```
function assign_ids(ast, prefix = "n", counter = &mut 1):
    for each node in ast (DFS pre-order):
        node.id = format!("{}{}", prefix, counter)
        counter += 1
```

This ensures that the same UL-Script always produces the same IDs, enabling meaningful diffs between versions of the same glyph.

For collaborative mode, IDs are UUID v7 (time-ordered unique identifiers).

---

## Sort Assignment

After creating all nodes, infer sorts:

```
for each node in gir.nodes:
    if node.sort is not explicitly set:
        node.sort = infer_sort(node.type)
```

Then validate sort constraints on all edges (basic check — full validation is in `ul-validate`):

```
for each edge in gir.edges:
    match edge.type:
        "contains"    → assert source.type == "enclosure"
        "modified_by" → assert target.sort == "modifier"
        _             → pass  // other edge types have no sort constraint
```

---

## Deparse (GIR → UL-Script)

The reverse transformation for round-tripping:

```
function deparse(gir):
    tree = extract_tree_spine(gir)
    cross_edges = gir.edges - tree.edges
    
    script = deparse_subtree(tree.root, tree, cross_edges)
    return script

function deparse_subtree(node, tree, cross_edges):
    text = deparse_primitive(node)
    
    children = tree.children_of(node)
    if children:
        content = children.map(c => deparse_subtree(c, tree, cross_edges))
        text += "{" + content.join(" | ") + "}"
    
    return text
```

Cross-edges are appended after the tree structure using explicit connection syntax.

---

## Edge Cases

| Input | Handling |
|-------|---------|
| Empty document | Empty GIR (`nodes: [], edges: []`, no root) |
| Single primitive `●` | One node, no edges, that node is root |
| Nested connections `● → ● → ●` | Chain: n1→n2→n3 with `connects` edges |
| Self-reference (not expressible in v1 script) | Only possible via direct GIR authoring |
| ASCII mixed with Unicode `○{*}` | Normalized during tokenization — ` *` becomes `●` |
