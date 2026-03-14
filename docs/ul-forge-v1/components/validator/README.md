# Validator Component

> Enforces GIR structural constraints, sort rules, and geometric invariants.

---

## Role in Pipeline

```
UL-Script ──→ Parser ──→ GIR ──→ Validator ──→ Renderer
                                  ▲
                                  │
                         also validates:
                         - editor glyph-builder output
                         - LLM-generated GIR
                         - manual JSON construction
                         - CRDT merge results
```

The validator runs **after** any GIR is produced and **before** it enters the renderer. It is a gate — invalid GIR never reaches rendering.

---

## Validation Layers

### Layer 1: Schema Validation

Structural JSON validity against the GIR schema.

| Check | Rule |
|-------|------|
| Required fields | `version`, `nodes`, `edges` present |
| Node required fields | `id`, `type`, `sort` on every node |
| Edge required fields | `type`, `source`, `target` on every edge |
| ID uniqueness | No duplicate node IDs |
| Referential integrity | Every edge `source` and `target` refers to an existing node ID |
| Type enums | `node.type` ∈ {point, line, angle, curve, enclosure}; `edge.type` ∈ {contains, modified_by, adjacent, intersects, connects, references} |

### Layer 2: Sort Constraints

Every operation in Σ_UL has a type signature. The validator enforces these.

| Rule | Constraint |
|------|-----------|
| `contains` edges | Source must be Entity (sort `e`); target can be any sort |
| `modified_by` edges | Source is Entity or Relation; target must be Modifier (sort `m`) |
| `predicate` structure | Requires Entity × Relation × Entity → Assertion |
| `quantify` structure | Requires Modifier × Entity → Assertion |
| `embed` result | An Assertion-bearing subgraph enclosed in an Entity node |
| Sort assignment | Every node has a valid sort based on its geometric type and context |

Sort inference rules (see [specifications/sort-system.md](../../specifications/sort-system.md)):
- `point` → Entity
- `line` → Relation
- `angle` → Modifier
- `curve` → Relation (process-encoded)
- `enclosure` → Entity (default) or Assertion (if contains a predicate structure)

### Layer 3: Graph Invariants

Structural properties the GIR must satisfy.

| Invariant | Rule |
|-----------|------|
| Containment acyclicity | `contains` edges form a DAG (no circular containment) |
| Single root | Exactly one node with no incoming `contains` edge (the root) |
| No dangling modifiers | Every `modified_by` target (modifier) has exactly one `modified_by` source |
| Angle validity | `angle` nodes have exactly 2 associated line/curve references |
| Line endpoints | `line` nodes connect (at most) 2 point nodes |

### Layer 4: Geometric Satisfiability (Optional)

Checks whether the described geometry can be realized in 2D. This is expensive and optional.

| Check | Description |
|-------|-------------|
| Intersection consistency | If nodes A and B have an `intersects` edge, their geometric shapes can overlap |
| Containment geometry | If A `contains` B, B's bounding box fits inside A |
| Adjacency feasibility | If A and B are `adjacent`, they can be placed touching without overlap with other adjacent marks |

---

## Rust API

```rust
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

pub struct ValidationError {
    pub code: ErrorCode,
    pub message: String,
    pub node_ids: Vec<NodeId>,
    pub edge_ids: Vec<EdgeId>,
}

pub enum ErrorCode {
    // Layer 1: Schema
    MissingRequiredField,
    DuplicateNodeId,
    DanglingEdgeReference,
    InvalidTypeEnum,
    // Layer 2: Sort
    SortViolation,
    InvalidOperationSignature,
    // Layer 3: Invariant
    ContainmentCycle,
    NoRoot,
    MultipleRoots,
    DanglingModifier,
    InvalidAngleArity,
    // Layer 4: Geometry
    UnrealizableIntersection,
    UnrealizableContainment,
}

/// Validate a GIR document. Layers 1-3 always run. Layer 4 runs if `check_geometry` is true.
pub fn validate(gir: &Gir, check_geometry: bool) -> ValidationResult;

/// Quick check: is this GIR valid for rendering? (Layers 1-3 only)
pub fn is_renderable(gir: &Gir) -> bool;
```

---

## WASM API

```typescript
interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
  warnings: ValidationWarning[];
}

function validate(girJson: string, checkGeometry?: boolean): ValidationResult;
function isRenderable(girJson: string): boolean;
```

---

## Error Recovery

The validator reports errors but does not fix them. Repair strategies are the caller's responsibility:

- **Parser-generated GIR**: Errors indicate parser bugs (should not happen in normal use)
- **Editor-built GIR**: Errors are shown to the user in real-time as they compose
- **LLM-generated GIR**: Errors are fed back to the LLM for correction
- **CRDT-merged GIR**: Errors trigger conflict resolution UI

---

## Performance

- Layers 1-3: O(V + E) — linear in graph size
- Layer 4 (geometric satisfiability): O(V²) worst case — disabled by default
- Target: validate a 100-node GIR in < 1ms (Layers 1-3)
