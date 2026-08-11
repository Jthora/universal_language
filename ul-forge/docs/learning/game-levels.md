# Game Challenge Levels

> 5 curriculum-aligned levels for learning UL through interactive composition.
> Each level is defined as a set of `CompositionRule` JSON configs for ul-game.

---

## Level 1: Identify the Primitive

**Goal**: Recognize and produce each of the 5 geometric primitives.
**Day**: 1–2 of curriculum

### Tasks

1. Draw a **Point** (●) — simplest entity
2. Draw a **Line** (→) — directed connection
3. Draw an **Angle** (@45) — modifier between relations
4. Draw a **Curve** (~) — parameterized path
5. Draw an **Enclosure** (○{…}) — bounded region

### Rules

```json
[
  {
    "name": "L1-point",
    "required_pattern": {
      "nodes": [{"id": "p", "node_type": "point", "sort": "entity"}],
      "edges": []
    },
    "weight": 1.0,
    "difficulty": 1
  },
  {
    "name": "L1-line",
    "required_pattern": {
      "nodes": [{"id": "l", "node_type": "line", "sort": "relation"}],
      "edges": []
    },
    "weight": 1.0,
    "difficulty": 1
  },
  {
    "name": "L1-angle",
    "required_pattern": {
      "nodes": [{"id": "a", "node_type": "angle", "sort": "modifier"}],
      "edges": []
    },
    "weight": 1.0,
    "difficulty": 1
  },
  {
    "name": "L1-curve",
    "required_pattern": {
      "nodes": [{"id": "c", "node_type": "curve"}],
      "edges": []
    },
    "weight": 1.0,
    "difficulty": 1
  },
  {
    "name": "L1-enclosure",
    "required_pattern": {
      "nodes": [{"id": "e", "node_type": "enclosure"}],
      "edges": []
    },
    "weight": 1.0,
    "difficulty": 1
  }
]
```

---

## Level 2: Sort the Sentence

**Goal**: Map words to the correct algebraic sort (Entity, Relation, Modifier, Assertion).
**Day**: 3–4 of curriculum

### Tasks

Given sentences like "The tall tree grows quickly", produce a GIR where:
- Nouns → Entity nodes (point/enclosure)
- Verbs → Relation nodes (line)
- Adjectives/Adverbs → Modifier nodes (angle)
- Full clause → Assertion enclosure

### Rules

```json
[
  {
    "name": "L2-has-entity",
    "required_pattern": {
      "nodes": [{"id": "e", "sort": "entity"}],
      "edges": []
    },
    "weight": 0.25,
    "difficulty": 2
  },
  {
    "name": "L2-has-relation",
    "required_pattern": {
      "nodes": [{"id": "r", "sort": "relation"}],
      "edges": []
    },
    "weight": 0.25,
    "difficulty": 2
  },
  {
    "name": "L2-has-modifier",
    "required_pattern": {
      "nodes": [{"id": "m", "sort": "modifier"}],
      "edges": []
    },
    "weight": 0.25,
    "difficulty": 2
  },
  {
    "name": "L2-has-assertion",
    "required_pattern": {
      "nodes": [{"id": "a", "sort": "assertion"}],
      "edges": []
    },
    "weight": 0.25,
    "difficulty": 2
  },
  {
    "name": "L2-modifier-connected",
    "operation": "modify_entity",
    "required_pattern": {
      "nodes": [
        {"id": "m", "sort": "modifier"},
        {"id": "e", "sort": "entity"}
      ],
      "edges": [
        {"source": "e", "target": "m", "edge_type": "modified_by"}
      ]
    },
    "weight": 0.5,
    "difficulty": 2
  }
]
```

---

## Level 3: Complete the Glyph

**Goal**: Given a partial glyph with a gap, add the missing element.
**Day**: 5–6 of curriculum

### Tasks

1. Given `● → __`, add the object entity
2. Given `○{ ● __ ● }`, add the relation
3. Given `○{ @__ ● → ● }`, add the modifier value
4. Given `__ { ● → ● }`, add the enclosure

### Rules

```json
[
  {
    "name": "L3-complete-predication",
    "operation": "predicate",
    "required_pattern": {
      "nodes": [
        {"id": "frame", "node_type": "enclosure", "sort": "assertion"},
        {"id": "subj", "sort": "entity"},
        {"id": "rel", "sort": "relation"},
        {"id": "obj", "sort": "entity"}
      ],
      "edges": [
        {"source": "frame", "target": "subj", "edge_type": "contains"},
        {"source": "frame", "target": "rel", "edge_type": "contains"},
        {"source": "frame", "target": "obj", "edge_type": "contains"},
        {"source": "subj", "target": "obj", "edge_type": "connects"}
      ]
    },
    "weight": 1.0,
    "difficulty": 3
  },
  {
    "name": "L3-no-dangling-nodes",
    "forbidden_pattern": {
      "nodes": [{"id": "orphan", "sort": "entity"}],
      "edges": []
    },
    "required_pattern": {
      "nodes": [{"id": "any"}],
      "edges": []
    },
    "weight": 0.5,
    "difficulty": 3
  }
]
```

---

## Level 4: Compose Operations

**Goal**: Combine two sub-glyphs using the correct Σ_UL operation.
**Day**: 7–8 of curriculum

### Tasks

1. Given two assertions, `conjoin` them (AND)
2. Given an assertion, `negate` it
3. Given an assertion, `embed` it (make it an entity)
4. Given an entity, `abstract` it (make it a modifier)
5. Given two relations, `compose` them (chain)

### Rules

```json
[
  {
    "name": "L4-conjoin",
    "operation": "conjoin",
    "required_pattern": {
      "nodes": [
        {"id": "top", "sort": "assertion"},
        {"id": "a1", "sort": "assertion"},
        {"id": "a2", "sort": "assertion"}
      ],
      "edges": [
        {"source": "top", "target": "a1", "edge_type": "contains"},
        {"source": "top", "target": "a2", "edge_type": "contains"}
      ]
    },
    "weight": 1.0,
    "difficulty": 4
  },
  {
    "name": "L4-embed",
    "operation": "embed",
    "required_pattern": {
      "nodes": [
        {"id": "outer", "sort": "entity"},
        {"id": "inner", "sort": "assertion"}
      ],
      "edges": [
        {"source": "outer", "target": "inner", "edge_type": "contains"}
      ]
    },
    "weight": 1.0,
    "difficulty": 4
  },
  {
    "name": "L4-compose-relations",
    "operation": "compose",
    "required_pattern": {
      "nodes": [
        {"id": "r1", "sort": "relation"},
        {"id": "r2", "sort": "relation"},
        {"id": "mid", "sort": "entity"}
      ],
      "edges": [
        {"source": "mid", "target": "r1", "edge_type": "connects"},
        {"source": "mid", "target": "r2", "edge_type": "connects"}
      ]
    },
    "weight": 1.0,
    "difficulty": 4
  }
]
```

---

## Level 5: Free Composition

**Goal**: Open canvas — write a novel glyph that satisfies structural quality criteria.
**Day**: 9 of curriculum

### Tasks

1. Compose a glyph that uses at least 3 different operations
2. Glyph must pass all 4 validation layers
3. Glyph must contain at least 5 nodes
4. Bonus: use a modal operator or force annotation

### Rules

```json
[
  {
    "name": "L5-min-nodes",
    "required_pattern": {
      "nodes": [
        {"id": "n1"}, {"id": "n2"}, {"id": "n3"}, {"id": "n4"}, {"id": "n5"}
      ],
      "edges": []
    },
    "weight": 0.3,
    "difficulty": 5
  },
  {
    "name": "L5-has-enclosure",
    "required_pattern": {
      "nodes": [{"id": "e", "node_type": "enclosure"}],
      "edges": []
    },
    "weight": 0.2,
    "difficulty": 5
  },
  {
    "name": "L5-has-modification",
    "operation": "modify_entity",
    "required_pattern": {
      "nodes": [
        {"id": "m", "sort": "modifier"},
        {"id": "e", "sort": "entity"}
      ],
      "edges": [
        {"source": "e", "target": "m", "edge_type": "modified_by"}
      ]
    },
    "weight": 0.2,
    "difficulty": 5
  },
  {
    "name": "L5-has-predication",
    "operation": "predicate",
    "required_pattern": {
      "nodes": [
        {"id": "s", "sort": "entity"},
        {"id": "r", "sort": "relation"},
        {"id": "o", "sort": "entity"}
      ],
      "edges": [
        {"source": "s", "target": "o", "edge_type": "connects"}
      ]
    },
    "weight": 0.2,
    "difficulty": 5
  },
  {
    "name": "L5-uses-embedding",
    "operation": "embed",
    "required_pattern": {
      "nodes": [
        {"id": "outer", "sort": "entity"},
        {"id": "inner", "sort": "assertion"}
      ],
      "edges": [
        {"source": "outer", "target": "inner", "edge_type": "contains"}
      ]
    },
    "weight": 0.1,
    "difficulty": 5
  }
]
```

---

## Loading Levels

```typescript
import { createContext, evaluate } from "@ul-forge/core";

// Load a level
const level1Rules = await fetch("/levels/level-1.json").then(r => r.json());
const ctx = createContext({ rules_json: JSON.stringify(level1Rules) });

// Evaluate student's composition
const result = evaluate(ctx, studentGirJson);
console.log(`Score: ${result.score}`);
console.log(`Matched: ${result.matched.map(r => r.name).join(", ")}`);
```

```bash
# CLI
ul-cli evaluate --rules levels/level-3.json --gir student-answer.json
```
