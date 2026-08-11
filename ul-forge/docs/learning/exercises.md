# UL Practice Exercises

> 10 exercises across 3 levels. Answers at the bottom.

---

## Level 1: Recognition (4 exercises)

Identify the structure of existing UL expressions.

> **Common Mistakes at Level 1:**
> - Confusing `○` (enclosure/assertion) with `●` (point/entity) — enclosure has braces, point does not
> - Forgetting that `∠` (angle) is the modifier sort, not a relation
> - Counting labels as separate entities — `● "cat"` is one entity, not two

### Exercise 1.1: Sort Identification

For each element in the expression below, state its sort (Entity, Relation, Modifier, or Assertion):

```
○{ ● "cat" → ● "mat" }
```

| Element | Your Answer |
|---------|-------------|
| `● "cat"` | |
| `→` | |
| `● "mat"` | |
| `○{ ... }` | |

---

### Exercise 1.2: Operation Identification

What operation does each expression demonstrate?

| # | Expression | Operation |
|---|-----------|-----------|
| a | `○{ ● "dog" → ● "loyal" }` | |
| b | `○{ ○{ ● → ● } ∠ ○{ ● → ● } }` | |
| c | `∠ "big" ● "house"` | |
| d | `← "kicked"` (given `→ "kicked"`) | |

---

### Exercise 1.3: English Translation

Translate each UL-Script expression into natural English:

| # | Expression | English |
|---|-----------|---------|
| a | `○{ ● "water" ~ "freezes" }` | |
| b | `○{ ∠ "slowly" ● "tortoise" → ● "finish line" }` | |
| c | `○{ ○{ ● "sun" → ● "hot" } ∠ ○{ ● "ice" → ● "cold" } }` | |

---

### Exercise 1.4: Entity Counting

How many entities (sort=Entity) are in each expression?

| # | Expression | Count |
|---|-----------|-------|
| a | `● → ●` | |
| b | `○{ ● → ○{ ∠ ● } }` | |
| c | `○{ ○{ ● → ● } ∠ ○{ ● → ● } → ● }` | |

---

## Level 2: Writing (3 exercises)

Convert English sentences into UL-Script.

> **Common Mistakes at Level 2:**
> - Forgetting the enclosure `○{ }` around a complete assertion
> - Using `●` (entity) where `∠` (modifier) is needed — adjectives are modifiers, not entities
> - Missing the relation `→` between subject and object — every predication needs a directed connection

### Exercise 2.1: Simple Predication

Encode: **"Fire is hot."**

Steps:
1. Decompose — list each element and its sort:
   | Element | Sort |
   |---------|------|
   | | |
   | | |
   | | |

2. Select operations — which operation(s) do you need?

3. Write the UL-Script expression:
   ```
   (your answer)
   ```

---

### Exercise 2.2: Modified Entity

Encode: **"The ancient tree stands tall."**

Steps:
1. Decompose:
   | Element | Sort |
   |---------|------|
   | | |
   | | |
   | | |
   | | |

2. Select operations (you'll need at least 2):

3. Write:
   ```
   (your answer)
   ```

---

### Exercise 2.3: Compound Assertion

Encode: **"The sun rises AND the birds sing."**

Steps:
1. Decompose each clause separately:
   - Clause 1: "The sun rises" → 
   - Clause 2: "The birds sing" → 

2. Combine — which operation joins them?

3. Write:
   ```
   (your answer)
   ```

---

## Level 3: Composition (3 exercises)

Combine existing expressions using advanced operations.

> **Common Mistakes at Level 3:**
> - Embedding: forgetting that `embed(a)` converts assertion→entity — the result is a `●`, not an `○`
> - Binding: using the same slot name for different variables in nested scopes
> - Modal: confusing necessity `□` (all worlds) with possibility `◇` (some world) — "must" vs "might"
> - Force: applying force to a non-assertion sort — only `○{ }` can have force set

### Exercise 3.1: Embedding

Given these two GIRs:
- A: `○{ ● "rain" ~ "falls" }` ("Rain falls")
- B: `○{ ● "farmers" → ● "happy" }` ("Farmers are happy")

Combine them to express: **"The fact that rain falls makes farmers happy."**

Hint: You need `embed` to turn A into an entity, then use it as a subject.

```
(your answer)
```

---

### Exercise 3.2: Binding

Encode: **"Every dog has a bone."**

You need:
- `bind` for the universal variable ("every dog")
- `quantify` for the quantifier
- `predicate` for the core relation

```
(your answer)
```

---

### Exercise 3.3: Modal + Force

Encode: **"Can you close the window?"** (an indirect request)

Consider:
- Surface meaning: a *query* about ability
- Intended meaning: a *directive* to close the window
- What force annotation and modal operator are involved?

Surface expression:
```
(your answer — surface)
```

Intended expression:
```
(your answer — intended)
```

---

## Answer Key

### Level 1 Answers

**1.1:**
| Element | Sort |
|---------|------|
| `● "cat"` | Entity |
| `→` | Relation |
| `● "mat"` | Entity |
| `○{ ... }` | Assertion |

**1.2:**
| # | Operation |
|---|-----------|
| a | `predicate(e, r, e) → a` |
| b | `conjoin(a, a) → a` (two assertion frames combined) |
| c | `modify_entity(m, e) → e` |
| d | `invert(r) → r` |

**1.3:**
| # | English |
|---|---------|
| a | "Water freezes" (entity undergoing a process) |
| b | "The slow tortoise reaches the finish line" (modified entity + relation) |
| c | "The sun is hot AND the ice is cold" (conjunction of two assertions) |

**1.4:**
| # | Count |
|---|-------|
| a | 2 (two Points) |
| b | 3 (outer Point + inner Angle-entity + inner Point) |
| c | 5 (two pairs of Points in the sub-assertions + one standalone) |

---

### Level 2 Answers

**2.1:** "Fire is hot"
- Decompose: fire=Entity, is=Relation, hot=Entity (property)
- Operation: `predicate(e, r, e) → a`
- UL-Script: `○{ ● "fire" → ● "hot" }`

> **Verify:** `ul-cli check '○{ ● "fire" → ● "hot" }' --expect predicate`

**2.2:** "The ancient tree stands tall"
- Decompose: ancient=Modifier, tree=Entity, stands=Relation, tall=Modifier
- Operations: `modify_entity(m, e) → e` then `predicate(e, r, e) → a` + `modify_relation(m, r) → r`
- UL-Script: `○{ ∠ "ancient" ● "tree" ∠ "tall" → ● "standing" }`
  (Or: `○{ ∠ "ancient" ● "tree" → ∠ "tall" ● "standing" }` — multiple valid representations)

> **Verify:** `ul-cli check '○{ ∠ "ancient" ● "tree" ∠ "tall" → ● "standing" }' --expect modify_entity,predicate`

**2.3:** "The sun rises AND the birds sing"
- Clause 1: `○{ ● "sun" ~ "rises" }` — process
- Clause 2: `○{ ● "birds" ~ "sing" }` — process  
- Operation: `conjoin(a, a) → a`
- UL-Script: `○{ ○{ ● "sun" ~ "rises" } ○{ ● "birds" ~ "sing" } }`

> **Verify:** `ul-cli check '○{ ○{ ● "sun" ~ "rises" } ○{ ● "birds" ~ "sing" } }' --expect conjoin`

---

### Level 3 Answers

**3.1:** "The fact that rain falls makes farmers happy"
1. Embed A: `embed(○{ ● "rain" ~ "falls" })` → entity
2. Combine: `predicate(embedded_A, "makes", modify_entity("happy", "farmers"))`
3. UL-Script: `○{ ○{ ● "rain" ~ "falls" } → ∠ "happy" ● "farmers" }`

> **Verify:** `ul-cli check '○{ ○{ ● "rain" ~ "falls" } → ∠ "happy" ● "farmers" }' --expect embed,predicate,modify_entity`

**3.2:** "Every dog has a bone"
1. Create variable slot: `○_x` (the dog variable)
2. Quantify: `quantify(∠1.0, ○_x)` — universal (p=1.0 = "every")
3. Core: `predicate(●_x, "has", ● "bone")`
4. Bind: `bind(○_x, predicate_assertion)`
5. UL-Script: `○{ ∠1.0 ○_x ●_x → ● "bone" }`

> **Verify:** `ul-cli check '○{ ∠1.0 ○_x ●_x → ● "bone" }' --expect quantify,bind,predicate`

**3.3:** "Can you close the window?" (indirect request)

Surface (query about ability):
```
query{ <>{○{ ● "you" → ● "window" ~ "close" }} }
```

Intended (directive):
```
direct{ ○{ ● "you" → ● "window" ~ "close" } }
```

The pragmatic inference rule CI-3 maps: Query + ability → Direct.

---

## Cross-References

Each exercise maps to Writer's Companion worked examples:
- 1.1–1.4 → Examples 1–3 (basic predication)
- 2.1 → Example 1 ("Knowledge is structured truth")
- 2.2 → Example 6 ("Love is patient" — property encoding)
- 2.3 → Example 7 ("Love is patient AND love is strong")
- 3.1 → Example 10 ("Peace emerges when harmony overcomes conflict")
- 3.2 → Example 11 ("Every student read some book")
- 3.3 → Example 19 ("Can you pass the salt?")
