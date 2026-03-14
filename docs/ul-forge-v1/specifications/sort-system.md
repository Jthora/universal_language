# Sort System: Σ_UL Constraints in Code

> How the 4 sorts (Entity, Relation, Modifier, Assertion) are enforced at every layer of UL Forge.

---

## The Four Sorts

From the formal specification (Σ_UL):

| Sort | Symbol | Geometric grounding | Role |
|------|--------|-------------------|------|
| **Entity** | `e` | Point, Enclosure | Things that exist — subjects and objects |
| **Relation** | `r` | Line, Curve | Directed connections between entities |
| **Modifier** | `m` | Angle | Qualities that modify entities or relations |
| **Assertion** | `a` | (composite) | Complete statements — subject + relation + object |

---

## The 11 Operations and Their Sort Signatures

| Operation | Signature | Description |
|-----------|-----------|-------------|
| `predicate` | `e × r × e → a` | Combine subject, relation, object into statement |
| `modify_entity` | `m × e → e` | Apply modifier to an entity |
| `modify_relation` | `m × r → r` | Apply modifier to a relation |
| `negate` | `a → a` | Negate a statement |
| `conjoin` | `a × a → a` | Combine statements with AND |
| `disjoin` | `a × a → a` | Combine statements with OR |
| `embed` | `a → e` | Turn statement into entity (nominalization) |
| `abstract` | `e → m` | Turn entity into modifier (adjectivalization) |
| `compose` | `r × r → r` | Chain two relations |
| `invert` | `r → r` | Reverse a relation |
| `quantify` | `m × e → a` | Apply quantifier-modifier to entity |

---

## Sort Enforcement in UL Forge

### At parse time (UL-Script → GIR)

The parser infers sorts from the geometric primitive type:

```rust
fn infer_sort(node_type: &NodeType) -> Sort {
    match node_type {
        NodeType::Point     => Sort::Entity,
        NodeType::Line      => Sort::Relation,
        NodeType::Angle     => Sort::Modifier,
        NodeType::Curve     => Sort::Relation,
        NodeType::Enclosure => Sort::Entity,
    }
}
```

When the parser encounters an operation (e.g., containment, connection), it checks that the operand sorts are compatible:

```
△{●}       ← contains: Entity(triangle) contains Entity(point). ✓ Valid.
●{△}       ← contains: Entity(point) contains Entity(triangle). ✗ Invalid: point is not an enclosure.
● →@60 ●   ← connects with modifier: Entity → (Relation modified by Modifier) → Entity. ✓ Valid.
@60{●}     ← contains: Modifier(angle) contains Entity(point). ✗ Invalid: modifier is not an enclosure.
```

### At validation time (GIR → report)

The validator re-checks all sort constraints on arbitrary GIR (which may come from AI generation or hand-authoring):

```
RULE: contains_sort
  For each edge with type "contains":
    assert source.sort == Entity
    assert source.type == "enclosure"

RULE: modified_by_sort
  For each edge with type "modified_by":
    assert target.sort == Modifier

RULE: predicate_sort (implicit in connects edges with angle modification)
  A "connects" edge between two Entities, modified by a Modifier,
  produces an implicit Assertion.
```

### At GIR construction time (programmatic API)

The Rust API enforces sorts at compile time using Rust's type system:

```rust
// Sort-safe node construction
enum Entity { Point(Point), Enclosure(Enclosure) }
enum Relation { Line(Line), Curve(Curve) }
enum Modifier { Angle(Angle) }

// Type-safe operations — won't compile with wrong sorts
fn predicate(subject: Entity, relation: Relation, object: Entity) -> Assertion;
fn modify_entity(modifier: Modifier, entity: Entity) -> Entity;
fn embed(assertion: Assertion) -> Entity;
fn abstract_entity(entity: Entity) -> Modifier;
```

This means sort errors in the core library are **compile-time errors**, not runtime errors.

### At transformation time

When transforming GIR (e.g., applying `embed` or `abstract`), the sort of the affected node changes:

```
embed: Takes a node (or subgraph) with sort Assertion → wraps in new Entity node
  - Creates new enclosure node with sort Entity
  - Original subgraph becomes contained within it
  - The enclosure carries sort Entity; the assertion is preserved inside

abstract: Takes an Entity → creates a Modifier
  - Creates new angle/modifier node
  - The entity's properties become the modifier's quality
```

---

## Sort Override Rules

A node's sort may differ from its type's default when it plays a transformed role:

| Scenario | Default sort | Override sort | How |
|----------|-------------|---------------|-----|
| Embedded assertion | assertion | entity | `embed(a) → e` wraps in enclosure |
| Abstracted entity | entity | modifier | `abstract(e) → m` extracts quality |
| Nominalized relation | relation | entity | Treat relation as a "thing" (e.g., "the connection") |

When sort is overridden, the node's `sort` field must be explicitly set in the GIR (not relying on default inference).

---

## Common Sort Errors

| Error | Cause | Message |
|-------|-------|---------|
| `contains` with non-enclosure source | Point or line used as container | "Node n3 (type: point) cannot contain other nodes — only enclosures can" |
| `modified_by` with non-modifier target | Entity or Relation used as modifier | "Node n5 (type: line, sort: relation) cannot modify — expected sort: modifier" |
| Sort mismatch in transformation | Applying `embed` to non-assertion | "embed requires assertion, got entity at node n2" |
| Unresolvable sort | Ambiguous context | "Cannot infer sort for node n4 — explicit sort annotation required" |

---

## Relationship to Formal Foundations

This sort system is a direct implementation of the Σ_UL signature defined in [foundations/formal-foundations.md](../../../foundations/formal-foundations.md). The 4 sorts and 11 operations are not design choices — they are mathematically derived from the 5 geometric primitives and proven unique up to isomorphism (Theorem: Unique Grounding Theorem).

UL Forge's sort enforcement is the *computational realization* of that mathematical proof. Every sort error caught by the validator is a violation of the formal system's axioms.
