# Type 2 — Composition Rule Engine

> Pattern matching against the Σ_UL formal system — 5 primitives, 4 sorts, 11 operations.

---

## Overview

The composition rule engine evaluates player-composed GIR structures against rules grounded in the real Universal Language formal system. Rules check for correct use of:

- **5 geometric primitives:** Point, Line, Angle, Curve, Enclosure
- **4 sorts:** Entity, Relation, Modifier, Assertion
- **11 Σ_UL operations:** predicate, modify_entity, modify_relation, negate, conjoin, disjoin, embed, abstract, compose, invert, quantify
- **3 lexicon tiers:** T1 (geometrically forced), T2 (structurally distinguished), T3 (conventional)

Each composition rule binds a structural pattern (required + optional forbidden subgraph) to a weight and difficulty level. The rule engine matches player GIR against all active rules using backtracking search, producing a weighted correctness score.

---

## Types

```rust
/// The 11 operations of the Σ_UL algebraic signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Predicate,       // e × r × e → a
    ModifyEntity,    // m × e → e
    ModifyRelation,  // m × r → r
    Negate,          // a → a
    Conjoin,         // a × a → a
    Disjoin,         // a × a → a
    Embed,           // a → e
    Abstract,        // e → m
    Compose,         // r × r → r
    Invert,          // r → r
    Quantify,        // m × e → a
}

/// Tier classification from the UL lexicon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    Forced,        // T1: geometrically forced — exactly one construction
    Distinguished, // T2: structurally distinguished — extremal or maximally symmetric
    Conventional,  // T3: conventional — reasonable but not unique
}
```

---

## Composition Rules

```rust
/// A composition rule grounded in real UL structure.
/// Rules check for correct use of geometric primitives, sorts, and operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionRule {
    pub name: String,
    /// Which Σ_UL operation this rule tests (if specific to one).
    pub operation: Option<Operation>,
    /// Tier classification (T1 forced, T2 distinguished, T3 conventional).
    pub tier: Tier,           // default: Conventional
    /// GIR pattern that must be present for this rule to be satisfied.
    pub required_pattern: GirPattern,
    /// GIR pattern that must NOT be present (optional).
    pub forbidden_pattern: Option<GirPattern>,
    /// Importance weight (0.0–1.0) for scoring.
    pub weight: f64,          // default: 1.0
    /// Difficulty level (1–5), used for proficiency scaling.
    pub difficulty: u8,       // default: 1
}
```

### GIR Pattern Matching

A `GirPattern` is a partial GIR that describes a subgraph to match against:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GirPattern {
    pub nodes: Vec<PatternNode>,
    #[serde(default)]
    pub edges: Vec<PatternEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternNode {
    /// Pattern-local ID for edge references
    pub id: String,
    /// Required node type: "point", "line", "angle", "curve", "enclosure" (None = any)
    pub node_type: Option<String>,
    /// Required sort: "entity", "relation", "modifier", "assertion" (None = any)
    pub sort: Option<String>,
    /// Required label (exact match, case-insensitive) (None = any)
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternEdge {
    pub source: String,
    pub target: String,
    /// Required edge type: "contains", "modified_by", "connects", etc. (None = any)
    pub edge_type: Option<String>,
}
```

### Pattern Matching Algorithm

Backtracking search: assign pattern nodes to GIR nodes, verify all constraints.

```rust
pub fn pattern_matches(pattern: &GirPattern, gir: &Gir) -> bool {
    // 1. Collect all GIR node IDs
    // 2. For each pattern node, try candidate GIR nodes
    //    - Check node_type constraint (case-insensitive)
    //    - Check sort constraint (case-insensitive)
    //    - Check label constraint (case-insensitive exact match)
    // 3. No two pattern nodes may bind to the same GIR node
    // 4. Verify all edge constraints on the current assignment
    // 5. Backtrack on failure
}
```

### Rule Index

```rust
pub struct RuleIndex {
    rules: Vec<CompositionRule>,
}

impl RuleIndex {
    pub fn new(rules: Vec<CompositionRule>) -> Self;
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error>;
    pub fn add_rules(&mut self, rules: Vec<CompositionRule>);
    pub fn rules(&self) -> &[CompositionRule];

    /// Returns (matched_names, violated_names) for all rules.
    pub fn evaluate(&self, gir: &Gir) -> (Vec<String>, Vec<String>);

    /// Weighted correctness score (0.0–1.0).
    pub fn correctness_score(&self, gir: &Gir) -> f64;
}
```

---

## Rule Data Format (JSON)

Rules are loaded from JSON data files, not compiled into the WASM binary. This enables modding and per-level customization.

```json
[
  {
    "name": "predicate-structure",
    "operation": "predicate",
    "tier": "forced",
    "required_pattern": {
      "nodes": [
        { "id": "subj", "node_type": "point", "sort": "entity" },
        { "id": "rel",  "node_type": "line",  "sort": "relation" },
        { "id": "obj",  "node_type": "point", "sort": "entity" }
      ],
      "edges": [
        { "source": "rel", "target": "subj", "edge_type": "connects" },
        { "source": "rel", "target": "obj",  "edge_type": "connects" }
      ]
    },
    "weight": 1.0,
    "difficulty": 1
  },
  {
    "name": "embed-operation",
    "operation": "embed",
    "tier": "forced",
    "required_pattern": {
      "nodes": [
        { "id": "enc", "node_type": "enclosure" },
        { "id": "inner" }
      ],
      "edges": [
        { "source": "enc", "target": "inner", "edge_type": "contains" }
      ]
    },
    "weight": 1.0,
    "difficulty": 2
  }
]
```

---

## Element ↔ Primitive Affinities

Each element has a natural affinity with certain UL primitives:

| Element | Primary Primitive | Secondary | Typical Patterns |
|---------|------------------|-----------|------------------|
| **Fire** | Point | Line (directed) | Creation from a single point, directed force |
| **Water** | Curve | Enclosure | Flowing paths, containment, cycles |
| **Earth** | Enclosure | Point | Solid structures, grounded existence |
| **Air** | Line | Angle | Connections, modifications, relationships |

These affinities affect scoring — a Fire rule expects Point-heavy glyphs, and a Water rule expects Curve-heavy glyphs. Mismatched primitives reduce the score but don't prevent matching entirely.

---

## Rule Index

For performance, the `GameContext` pre-indexes rules at creation time:

```rust
pub struct RuleIndex {
    /// Rules indexed by element
    by_element: HashMap<Element, Vec<usize>>,
    /// Rules indexed by force
    by_force: HashMap<CosmicForce, Vec<usize>>,
    /// Rules indexed by tier
    by_tier: HashMap<u8, Vec<usize>>,
    /// Rules indexed by primary node type in pattern
    by_primary_node: HashMap<NodeType, Vec<usize>>,
}
```

This allows `evaluate()` to quickly narrow down candidate rules based on the glyph's composition before running the full pattern match.
