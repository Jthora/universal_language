# CORPUS — graded expressions with machine-checkable ground truth

**Purpose:** the data half of the School (layer 4, `research/notes/053`): UWS expressions a learner
— human or model — can train and test against, where **ground truth is machine-checkable, never
authorial say-so.**

## Entry format

One YAML file per entry, `corpus/entries/NNN-slug.yaml`:

```yaml
id: 001-triangle
tier: VERIFIED                    # tiers travel with content — TIERS-TRAVEL-WITH-CONTENT
gir: ...                          # the typed-graph form (ul-forge schema)
rotations:                        # the combinatorial map: vertex -> ordered darts
  v0: [0, 5]
  v1: [2, 1]
  v2: [4, 3]
expected:                         # ground truth, verified against ul-core, not asserted
  vertices: 3
  edges: 3
  faces: 2                        # Jordan: a simple closed curve bounds two regions
  genus: 0
  degree_sequence: [2, 2, 2]
provenance: map.rs::tests::triangle_has_two_faces
```

**The harness** *(Phase 1 deliverable)*: a test that loads every entry, reconstructs the map via
`ul-core`, and asserts the `expected` block. **An entry whose ground truth does not machine-verify
does not merge.** Entries whose semantics outrun what the code can check are admitted only at
CONJECTURED with the unchecked fields marked.

## Grading

Entries are graded by the derivation ladder, matching curriculum modules: single closed curves (N)
→ multi-component configurations with nesting (F) → junction-bearing configurations and the
degree table (S) → exchange scenarios with convention ledgers (P) → executable programs (Q, gated
on the engine).

**Status: format defined, zero entries.** First entries fall out of `map.rs`'s existing test
configurations (triangle, theta graph, two-triangles-with-nesting, torus embedding) — the tests
already compute their ground truth, so seeding the corpus is extraction, not authoring.
