# 07 — Algebraic Invariant & Composition Law Tests

> **Goal:** Verify every proven algebraic law from the formal specification. These are the mathematical *theorems* that UL claims hold. If the implementation violates any of them, the implementation is wrong — not the theorem.

---

## Problem Statement

The formal specification proves 5 theorems unconditionally and 8 conditionally. The implementation has never been tested against any of them. The composition laws (involution, associativity, De Morgan, commutativity) are the algebraic backbone of UL — they're what makes it a *language* rather than an ad hoc encoding scheme.

---

## Test Categories

### 7.1 — Involution Laws (Self-Inverse Operations)

The formal specification proves: `negate(negate(a)) = a` and `invert(invert(r)) = r`.

"Equal" here means *structurally isomorphic GIR* — same node types, edge types, topology. Not identical JSON (node IDs may differ).

| Test | Operation Chain | Assertion |
|---|---|---|
| `negate(negate(predicate(●, →, ●)))` | Build assertion → negate → negate | Isomorphic to original assertion |
| `negate(negate(conjoin(a, b)))` | Complex assertion → negate² | Isomorphic to original |
| `negate(negate(disjoin(a, b)))` | Disjunction → negate² | Isomorphic |
| `negate(negate(quantify(m, e)))` | Quantified → negate² | Isomorphic |
| `invert(invert(→))` | Right arrow → invert² | Same direction as original |
| `invert(invert(←))` | Left arrow → invert² | Same direction as original |
| `invert(invert(↔))` | Bidirectional → invert² | Isomorphic |
| `invert(invert(compose(→, →)))` | Composed relation → invert² | Isomorphic |
| `negate ≠ identity` | `negate(a)` must differ from `a` | Structurally distinct |
| `invert ≠ identity` | `invert(→)` must differ from `→` | Direction changed |

**Test count:** ~15 involution tests

### 7.2 — Associativity Laws

`compose` is associative: `compose(r₁, compose(r₂, r₃)) ≅ compose(compose(r₁, r₂), r₃)`

| Test | Left-Grouped | Right-Grouped | Assertion |
|---|---|---|---|
| Three right arrows | `compose(compose(→, →), →)` | `compose(→, compose(→, →))` | Isomorphic |
| Mixed arrows | `compose(compose(→, ←), →)` | `compose(→, compose(←, →))` | Isomorphic |
| Three curves | `compose(compose(~, ~), ~)` | `compose(~, compose(~, ~))` | Isomorphic |
| Four arrows (two groupings) | `((r₁∘r₂)∘r₃)∘r₄` | `r₁∘(r₂∘(r₃∘r₄))` | Isomorphic |

Also for `conjoin` and `disjoin`:
| Test | Left-Grouped | Right-Grouped | Assertion |
|---|---|---|---|
| `conjoin` of 3 assertions | `conjoin(conjoin(a₁, a₂), a₃)` | `conjoin(a₁, conjoin(a₂, a₃))` | Isomorphic |
| `disjoin` of 3 assertions | `disjoin(disjoin(a₁, a₂), a₃)` | `disjoin(a₁, disjoin(a₂, a₃))` | Isomorphic |
| Mixed: 4 assertions | Two different groupings | Same result | Isomorphic |

**Test count:** ~15 associativity tests

### 7.3 — De Morgan Laws

`negate(disjoin(a, b)) ≅ conjoin(negate(a), negate(b))`
`negate(conjoin(a, b)) ≅ disjoin(negate(a), negate(b))`

| Test | LHS | RHS | Assertion |
|---|---|---|---|
| Simple assertions | `¬(a ∨ b)` | `(¬a) ∧ (¬b)` | Isomorphic |
| Complex assertions | `¬(p₁ ∨ p₂)` where p₁, p₂ are full predications | `(¬p₁) ∧ (¬p₂)` | Isomorphic |
| Dual: conjunction | `¬(a ∧ b)` | `(¬a) ∨ (¬b)` | Isomorphic |
| Nested: De Morgan on nested | `¬(a ∨ (b ∧ c))` | `(¬a) ∧ (¬b ∨ ¬c)` | Isomorphic |
| Triple disjunction | `¬(a ∨ b ∨ c)` | `¬a ∧ ¬b ∧ ¬c` | Isomorphic |

**Test count:** ~10 De Morgan tests

### 7.4 — Commutativity Laws

`conjoin(a, b) ≅ conjoin(b, a)` (conjunction is commutative)
`disjoin(a, b) ≅ disjoin(b, a)` (disjunction is commutative)

| Test | Forward | Backward | Assertion |
|---|---|---|---|
| `conjoin(a, b)` vs `conjoin(b, a)` with simple a, b | Order 1 | Order 2 | Isomorphic |
| `conjoin(a, b)` with distinct complex a, b | Order 1 | Order 2 | Isomorphic |
| `disjoin(a, b)` vs `disjoin(b, a)` | Order 1 | Order 2 | Isomorphic |
| `compose(r₁, r₂)` vs `compose(r₂, r₁)` | Order 1 | Order 2 | **NOT isomorphic** (compose is non-commutative!) |

The non-commutativity of `compose` is important: `r₁ ∘ r₂ ≠ r₂ ∘ r₁` in general (direction matters).

**Test count:** ~10 commutativity tests

### 7.5 — Distribution Laws

If the algebra supports distributivity: `conjoin(a, disjoin(b, c)) ≅ disjoin(conjoin(a, b), conjoin(a, c))`

| Test | LHS | RHS | Status |
|---|---|---|---|
| Simple distribution | `a ∧ (b ∨ c)` | `(a ∧ b) ∨ (a ∧ c)` | Check if holds |
| Dual distribution | `a ∨ (b ∧ c)` | `(a ∨ b) ∧ (a ∨ c)` | Check if holds |

Note: the formal spec doesn't explicitly claim distributivity holds at the GIR level. These tests *discover* whether it does — they are exploratory, not regression.

**Test count:** ~5 distribution tests

### 7.6 — Identity & Absorption Laws

Do identity elements exist?

| Law | Test | Expected |
|---|---|---|
| `conjoin(a, TRUE)` | Where TRUE = "trivially true assertion" | ≅ `a`? (if identity exists) |
| `disjoin(a, FALSE)` | Where FALSE = "trivially false assertion" | ≅ `a`? |
| `compose(r, identity_relation)` | If identity relation exists | ≅ `r`? |
| `modify_entity(identity_modifier, e)` | If identity modifier exists | ≅ `e`? |

These are discovery tests — they probe whether the implementation has identity elements, which the formal spec implies via algebraic structure.

**Test count:** ~8 identity tests

### 7.7 — Embedding Coherence Laws

`embed : a → e` and `abstract : e → m` allow sort-crossing. The composition laws between these must be tested:

| Law | Test | Assertion |
|---|---|---|
| `embed` preserves internal structure | `embed(predicate(●, →, ●))` → the entity should contain a predication subgraph | SubgraphOf check |
| `abstract(embed(a))` produces valid modifier | Chain both operations | Output sort = modifier |
| `embed(conjoin(a, b))` vs `conjoin(embed(a), embed(b))` | Does embed distribute over conjoin? | Characterize behavior |
| `predicate(embed(a), r, e)` works | Use nominalized assertion as entity | Valid assertion output |
| `modify_entity(abstract(e₁), e₂)` works | Use abstracted entity as modifier | Valid entity output |
| Deep chain: `embed(negate(predicate(embed(a), →, ●)))` | 4-level nesting | Valid GIR at each step |

**Test count:** ~12 embedding coherence tests

### 7.8 — Modal Operator Laws

Modal operators are defined in terms of existing operations. Verify the definitions hold:

| Law | Definition | Test |
|---|---|---|
| `possible(a) = negate(necessary(negate(a)))` | Duality of □ and ◇ | Build both sides, compare |
| `necessary(a)` implies `possible(a)` | T-axiom | If `necessary(a)` valid, `possible(a)` is also valid |
| `necessary(a)` implies `a` | Reflexivity (S5) | Structural containment |
| Counterfactual: `[]->{a}{b}` | Defined as `necessary(r_closest(a), b)` | Structure matches definition |

**Test count:** ~8 modal tests

---

## Estimated Total: ~83 invariant & law tests

## Implementation Approach

### GIR Isomorphism Checker

The critical infrastructure for this entire test category is a GIR structural isomorphism checker. String equality of JSON is insufficient — node IDs are generated and may differ.

```typescript
function girIsomorphic(gir1: any, gir2: any): boolean {
  // 1. Node count must match
  if (gir1.nodes.length !== gir2.nodes.length) return false;
  // 2. Edge count must match
  if (gir1.edges.length !== gir2.edges.length) return false;
  // 3. Node type distributions must match
  const types1 = nodeTypeDistribution(gir1);
  const types2 = nodeTypeDistribution(gir2);
  if (!deepEqual(types1, types2)) return false;
  // 4. Edge type distributions must match
  const edgeTypes1 = edgeTypeDistribution(gir1);
  const edgeTypes2 = edgeTypeDistribution(gir2);
  if (!deepEqual(edgeTypes1, edgeTypes2)) return false;
  // 5. Topology check: find a node-ID mapping that preserves all edges
  return findIsomorphism(gir1, gir2) !== null;
}
```

This checker is the single most important piece of test infrastructure for the entire rigor pass. Without it, we cannot test any algebraic law.

### Properties of the isomorphism checker itself
The checker should be tested:
- `girIsomorphic(gir, gir)` is always true (reflexivity)
- `girIsomorphic(gir1, gir2) === girIsomorphic(gir2, gir1)` (symmetry)
- Known-different GIRs return false
- GIRs that differ only in node IDs return true
- GIRs that differ in one edge type return false
