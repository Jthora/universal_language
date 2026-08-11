# Type 2 — Evaluation API

> Scoring engine with graduated failure and partial credit, grounded in UL's formal system.

---

## Design Principle

Binary pass/fail is bad game design. The evaluation API returns a **graduated score** that tells the player (and Jane) exactly how close they are and what could improve. Even "wrong" attempts earn partial credit for structural correctness.

Scoring dimensions come directly from UL's formal structure:
- **Structural match** — graph topology (node types, edge patterns)
- **Sort correctness** — entity/relation/modifier/assertion assignment
- **Operation correctness** — composition rule satisfaction
- **Sequence order** — construction ordering validity

---

## Evaluation Functions

### `evaluate(ctx_id, gir_json) → EvaluationResult`

The primary evaluation: given a glyph, how does it score against all active composition rules?

```rust
pub fn evaluate(ctx: &GameContext, gir: &Gir) -> EvaluationResult {
    let (matched, violated) = ctx.rule_index.evaluate(gir);
    let correctness = ctx.rule_index.correctness_score(gir);
    let feedback = generate_feedback(&matched, &violated, correctness);

    EvaluationResult {
        score: correctness,
        grade: Grade::from_score(correctness),
        matched_rules: matched,
        violated_rules: violated,
        feedback,
    }
}
```

### `scoreComposition(ctx_id, gir_json, target_json) → ScoreResult`

Targeted scoring: how does this glyph compare to a specific puzzle target?

```rust
pub fn score_composition(ctx: &GameContext, gir: &Gir, target: &PuzzleTarget) -> ScoreResult {
    let expected = Gir::from_json(&target.expected_gir_json)?;

    let structural = structural_similarity(gir, &expected);
    let sort = sort_similarity(gir, &expected);
    let operation = ctx.rule_index.correctness_score(gir);
    let sequence = sequence_score(gir, &target.sequence_constraints);

    let partial = PartialCredit {
        structural_match: structural,
        sort_correctness: sort,
        operation_correctness: operation,
        sequence_order: sequence,
    };
    let composite = partial.composite();

    ScoreResult {
        score: composite,
        grade: Grade::from_score(composite),
        partial_credit: partial,
        feedback: generate_targeted_feedback(structural, sort, operation, sequence),
    }
}
```

### `evaluateJaneAttempt(ctx_id, attempt_json, expected_json) → JaneResult`

Jane's learning system: compares her attempt to the expected answer and updates her proficiency model across the 5 geometric primitives.

```rust
pub fn evaluate_jane_attempt(
    ctx: &mut GameContext,
    attempt: &Gir,
    expected: &Gir,
) -> JaneResult {
    let structural = structural_similarity(attempt, expected);
    let sort = sort_similarity(attempt, expected);
    let operation = ctx.rule_index.correctness_score(attempt);

    let partial = PartialCredit {
        structural_match: structural,
        sort_correctness: sort,
        operation_correctness: operation,
        sequence_order: 1.0, // No sequence constraints for free-form
    };
    let score = partial.composite();

    // Update proficiency per primitive type present in the attempt
    // Uses exponential moving average (α = 0.3)
    let mut proficiency_delta = HashMap::new();
    for prim in extract_primitive_names(attempt) {
        let delta = ctx.update_proficiency(&prim, score);
        proficiency_delta.insert(prim, delta);
    }

    JaneResult {
        score,
        grade: Grade::from_score(score),
        improvements: suggest_improvements(structural, sort),
        proficiency_delta,
    }
}
```

---

## Graduated Scoring

### Grade Thresholds

| Grade | Score Range | Meaning |
|-------|------------|---------|
| `Exact` | 0.95 – 1.0 | Perfect or near-perfect match |
| `Close` | 0.60 – 0.95 | Right idea, minor issues |
| `Partial` | 0.20 – 0.60 | Some structural overlap but incomplete |
| `Unrelated` | 0.0 – 0.20 | Structurally unrelated |

```rust
pub fn from_score(score: f64) -> Grade {
    match score {
        s if s >= 0.95 => Grade::Exact,
        s if s >= 0.60 => Grade::Close,
        s if s >= 0.20 => Grade::Partial,
        _ => Grade::Unrelated,
    }
}
```

### Partial Credit Components

| Component | What It Measures | Weight |
|-----------|-----------------|--------|
| `structural_match` | Graph topology similarity (node-type histogram intersection + edge count) | 0.35 |
| `sort_correctness` | Are sorts assigned to the right primitives? | 0.25 |
| `operation_correctness` | Are the composition rules satisfied? | 0.25 |
| `sequence_order` | Is the construction sequence valid? | 0.15 |

---

## Result Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub score: f64,
    pub grade: Grade,
    pub matched_rules: Vec<String>,
    pub violated_rules: Vec<String>,
    pub feedback: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Grade {
    Exact,
    Close,
    Partial,
    Unrelated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialCredit {
    pub structural_match: f64,
    pub sort_correctness: f64,
    pub operation_correctness: f64,
    pub sequence_order: f64,
}

impl PartialCredit {
    pub fn composite(&self) -> f64 {
        self.structural_match * 0.35
            + self.sort_correctness * 0.25
            + self.operation_correctness * 0.25
            + self.sequence_order * 0.15
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreResult {
    pub score: f64,
    pub grade: Grade,
    pub partial_credit: PartialCredit,
    pub feedback: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JaneResult {
    pub score: f64,
    pub grade: Grade,
    pub improvements: Vec<String>,
    pub proficiency_delta: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PuzzleTarget {
    /// Expected GIR as JSON string.
    pub expected_gir_json: String,
    /// Which Σ_UL operations this puzzle requires (for feedback).
    pub required_operations: Vec<Operation>,
    /// Ordering constraints (optional).
    pub sequence_constraints: Vec<String>,
}
```
    pub rule_name: String,
    pub element: Element,
    pub force: CosmicForce,
    pub match_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub severity: ViolationSeverity,
    pub message: String,
    pub node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Error,
    Warning,
    Info,
}
```

---

## Structural Similarity Algorithm

```rust
/// Graph edit distance–based similarity between two GIR documents.
pub fn structural_similarity(a: &Gir, b: &Gir) -> f64 {
    let node_types_a: Vec<NodeType> = a.nodes.iter().map(|n| n.node_type).collect();
    let node_types_b: Vec<NodeType> = b.nodes.iter().map(|n| n.node_type).collect();

    // Jaccard-like similarity on node type multisets
    let node_sim = multiset_similarity(&node_types_a, &node_types_b);

    let edge_types_a: Vec<EdgeType> = a.edges.iter().map(|e| e.edge_type).collect();
    let edge_types_b: Vec<EdgeType> = b.edges.iter().map(|e| e.edge_type).collect();

    let edge_sim = multiset_similarity(&edge_types_a, &edge_types_b);

    // Weighted combination
    node_sim * 0.6 + edge_sim * 0.4
}
```

---

## Feedback Generation

The feedback string is a one-sentence explanation for Jane's learning system:

```rust
pub fn generate_feedback(total: f64, structural: f64, sort: f64, cosmic: f64) -> String {
    if total >= 0.95 {
        "Perfect composition!".to_string()
    } else if total >= 0.60 {
        if structural < 0.7 {
            "Close! The structure needs adjustment — check the connections between primitives.".to_string()
        } else if sort < 0.7 {
            "Almost there! Some primitives have the wrong sort assignment.".to_string()
        } else {
            "Good structure, but the cosmic alignment could be stronger.".to_string()
        }
    } else if total >= 0.20 {
        "The approach is different from what's expected. Try starting with the core primitive.".to_string()
    } else {
        "This composition channels the opposite force. Consider reversing the primary relationship.".to_string()
    }
}
```
