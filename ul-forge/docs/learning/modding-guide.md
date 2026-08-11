# Modding Guide — Custom Composition Rules

> How to create custom scoring rules, patterns, and templates for UL Forge's game engine.

> **Requires:** The game WASM module (`ul-game`) must be built and integrated. As of R1, `ul-game` is consolidated into `ul-wasm` — all game functions are available in the web editor WASM build.

---

## Overview

The `ul-game` crate provides a modding system that lets you define custom `CompositionRule`s. Each rule specifies:

- A **required pattern** — a subgraph that must be present in a valid composition
- An optional **forbidden pattern** — a subgraph that must not be present
- **Weight** and **difficulty** for scoring

Rules are loaded as JSON, either at session creation or at runtime.

---

## 1. CompositionRule Structure

```json
{
  "name": "rule-name",
  "operation": "predicate",
  "tier": "conventional",
  "required_pattern": {
    "nodes": [
      { "id": "root", "node_type": "enclosure", "sort": "assertion" },
      { "id": "subj", "sort": "entity" },
      { "id": "rel", "sort": "relation" }
    ],
    "edges": [
      { "source": "root", "target": "subj", "edge_type": "contains" },
      { "source": "root", "target": "rel", "edge_type": "contains" }
    ]
  },
  "forbidden_pattern": null,
  "weight": 1.0,
  "difficulty": 3
}
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | yes | Unique identifier (non-empty) |
| `operation` | string | no | Which Σ_UL operation this tests (e.g., `"predicate"`, `"embed"`, `"necessity"`) |
| `tier` | string | no | `"forced"` / `"distinguished"` / `"conventional"` (default: `"conventional"`) |
| `required_pattern` | object | yes | Subgraph that must match |
| `forbidden_pattern` | object | no | Subgraph that must not match |
| `weight` | float | no | Importance 0.0–1.0 (default: 1.0) |
| `difficulty` | int | no | Difficulty 1–5 (default: 1) |

---

## 2. Patterns (GirPattern)

A pattern has `nodes` and `edges`. The game engine uses backtracking search to find any subgraph in the student's GIR that satisfies all constraints.

### PatternNode

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Binding name (e.g., `"root"`, `"child1"`) — must be unique within the pattern |
| `node_type` | string? | Match geometric primitive: `"point"`, `"line"`, `"angle"`, `"curve"`, `"enclosure"` |
| `sort` | string? | Match algebraic sort: `"entity"`, `"relation"`, `"modifier"`, `"assertion"` |
| `label` | string? | Exact label match (case-insensitive) |

### PatternEdge

| Field | Type | Description |
|-------|------|-------------|
| `source` | string | Source pattern node `id` |
| `target` | string | Target pattern node `id` |
| `edge_type` | string? | Match edge type: `"contains"`, `"modified_by"`, `"connects"`, `"adjacent"`, `"intersects"`, `"binds"`, `"accessible_from"` |

**Matching rules:**
- `null` fields are wildcards (match anything)
- All non-null fields must match exactly
- Search is capped at 10,000 steps to prevent runaway backtracking

---

## 3. Examples

### Rule: "Must contain an enclosure"

```json
{
  "name": "has-enclosure",
  "required_pattern": {
    "nodes": [{ "id": "e", "node_type": "enclosure" }],
    "edges": []
  },
  "weight": 0.8,
  "difficulty": 1
}
```

### Rule: "Must use all 5 primitives"

```json
[
  {
    "name": "has-point",
    "required_pattern": {
      "nodes": [{ "id": "p", "node_type": "point" }],
      "edges": []
    },
    "weight": 0.2,
    "difficulty": 4
  },
  {
    "name": "has-line",
    "required_pattern": {
      "nodes": [{ "id": "l", "node_type": "line" }],
      "edges": []
    },
    "weight": 0.2,
    "difficulty": 4
  },
  {
    "name": "has-angle",
    "required_pattern": {
      "nodes": [{ "id": "a", "node_type": "angle" }],
      "edges": []
    },
    "weight": 0.2,
    "difficulty": 4
  },
  {
    "name": "has-curve",
    "required_pattern": {
      "nodes": [{ "id": "c", "node_type": "curve" }],
      "edges": []
    },
    "weight": 0.2,
    "difficulty": 4
  },
  {
    "name": "has-enclosure",
    "required_pattern": {
      "nodes": [{ "id": "e", "node_type": "enclosure" }],
      "edges": []
    },
    "weight": 0.2,
    "difficulty": 4
  }
]
```

### Rule: "No dangling modifiers" (forbidden pattern)

```json
{
  "name": "no-dangling-modifier",
  "forbidden_pattern": {
    "nodes": [
      { "id": "m", "sort": "modifier" }
    ],
    "edges": []
  },
  "required_pattern": {
    "nodes": [{ "id": "any" }],
    "edges": []
  },
  "weight": 0.5,
  "difficulty": 2
}
```

### Rule: "Valid predication" (entity–relation–entity inside frame)

```json
{
  "name": "valid-predication",
  "operation": "predicate",
  "required_pattern": {
    "nodes": [
      { "id": "frame", "node_type": "enclosure", "sort": "assertion" },
      { "id": "subj", "sort": "entity" },
      { "id": "rel", "sort": "relation" },
      { "id": "obj", "sort": "entity" }
    ],
    "edges": [
      { "source": "frame", "target": "subj", "edge_type": "contains" },
      { "source": "frame", "target": "rel", "edge_type": "contains" },
      { "source": "frame", "target": "obj", "edge_type": "contains" },
      { "source": "subj", "target": "obj", "edge_type": "connects" }
    ]
  },
  "weight": 1.0,
  "difficulty": 2
}
```

---

## 4. Loading Rules

### At Session Creation (WASM)

```typescript
import { createContext } from "@ul-forge/core";

const rules = [/* ... CompositionRule JSON array ... */];
const config = {
  rules_json: JSON.stringify(rules),
  session_id: "custom-session",
};
const contextId = createContext(config);
```

### At Runtime (WASM)

```typescript
import { loadCustomDefinitions } from "@ul-forge/core";

const result = loadCustomDefinitions(contextId, JSON.stringify(newRules));
// → { loaded: 3, errors: [] }
```

### CLI

```bash
# Load rules and evaluate a GIR
ul-cli evaluate --rules custom-rules.json --gir student-glyph.json
```

---

## 5. Scoring

Rules are evaluated against student GIR compositions:

- **Matched rules** — the required pattern is found (and forbidden pattern is absent)
- **Violated rules** — the required pattern is not found (or forbidden pattern is present)

The weighted score is:

$$\text{score} = \frac{\sum_{i \in \text{matched}} w_i}{\sum_{i \in \text{all}} w_i}$$

Grades from score:
- **Exact** (≥ 0.95) — all rules satisfied
- **Close** (≥ 0.60) — most rules satisfied
- **Partial** (≥ 0.20) — some rules satisfied
- **Unrelated** (< 0.20) — few or no rules satisfied

---

## 6. Available Operations for Rules

The 18 `Operation` variants you can reference:

| Core (13) | Extensions (5) |
|-----------|----------------|
| `predicate`, `modify_entity`, `modify_relation` | `necessity` |
| `negate`, `conjoin`, `disjoin` | `possibility` |
| `embed`, `abstract` | `counterfactual` |
| `compose`, `invert` | `set_force` |
| `quantify`, `bind`, `modify_assertion` | `infer_pragmatic` |
