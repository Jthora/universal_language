# Structural Diff and Merge

> How UL Forge computes differences between GIR documents and merges conflicting edits.

---

## Why Structural Diff?

Text diff (line-by-line) is meaningless for GIR. Two GIR documents can be semantically identical but have different JSON serializations (different node ordering, different ID schemes). UL Forge needs **structural** diff that operates on the graph, not the text.

---

## Diff Algorithm

### Definition

A structural diff between GIR documents A and B is a set of **edit operations**:

| Operation | Description |
|-----------|-------------|
| `add_node(node)` | Node exists in B but not A |
| `remove_node(id)` | Node exists in A but not B |
| `modify_node(id, field, old, new)` | Node exists in both, field value changed |
| `add_edge(edge)` | Edge exists in B but not A |
| `remove_edge(source, target, type)` | Edge exists in A but not B |

### Node Matching

Nodes are matched by stable ID. If IDs have been reassigned, fall back to structural matching:
1. Match nodes with identical `(type, sort, label)` triples
2. Match remaining nodes by graph neighborhood similarity
3. Unmatched nodes are added/removed

### Diff Output

```json
{
  "operations": [
    { "op": "add_node", "node": { "id": "n6", "type": "point", "sort": "e" } },
    { "op": "add_edge", "edge": { "type": "adjacent", "source": "n5", "target": "n6" } },
    { "op": "modify_node", "id": "n1", "field": "label", "old": "concept", "new": "idea" }
  ]
}
```

---

## Three-Way Merge

Given `base`, `ours`, `theirs`:

```
         base
        ╱    ╲
    diff_A    diff_B
      ╱          ╲
   ours        theirs
      ╲          ╱
       merge(diff_A, diff_B)
           │
        merged
```

### Merge Rules

| Scenario | Resolution |
|----------|-----------|
| Only one side changed a node | Accept the change |
| Both sides added different nodes | Accept both (no conflict) |
| Both sides removed the same node | Accept removal |
| Both sides modified the same field to the same value | Accept (no conflict) |
| Both sides modified the same field to different values | **Conflict** |
| One side removed a node, the other modified it | **Conflict** |

### Conflict Representation

```json
{
  "conflicts": [
    {
      "type": "field_conflict",
      "node_id": "n1",
      "field": "label",
      "base": "concept",
      "ours": "idea",
      "theirs": "notion"
    }
  ]
}
```

Conflicts are surfaced to the user for manual resolution. The merge tool does not auto-resolve ambiguous cases.

---

## Rust API

```rust
pub fn diff(a: &Gir, b: &Gir) -> Vec<DiffOperation>;
pub fn merge(base: &Gir, ours: &Gir, theirs: &Gir) -> MergeResult;

pub struct MergeResult {
    pub merged: Gir,
    pub conflicts: Vec<Conflict>,
}
```

---

## Git Integration

GIR JSON files (`.ul.json`) can use a custom Git merge driver:

```gitattributes
*.ul.json merge=ul-forge
```

```gitconfig
[merge "ul-forge"]
    name = UL Forge structural merge
    driver = ul merge %O %A %B
```

This invokes `ul merge base ours theirs`, which writes the merged result to `ours` and exits with code 1 if conflicts remain.
