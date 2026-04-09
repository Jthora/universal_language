# 02 — Algebraic Depth Tests

> **Goal:** Verify that the 13 Σ_UL operations satisfy their formal algebraic properties — sort correctness, composition laws, involution identities, injectivity, and multi-step operation chains. The current tests check `nodes.length > 0`. That proves *something happened*. These tests prove *the right thing happened*.

---

## Problem Statement

The formal specification defines 13 operations with precise type signatures, algebraic laws (negate is involution, compose is associative, De Morgan holds), and structural outcomes. No existing test verifies any of these properties. The algebraic tests are weaker than the round-trip tests they sit beside.

---

## Test Categories

### 2.1 — Sort Correctness (Output Type Verification)

Every operation has a declared output sort. The GIR root node of the output must have the correct sort.

| Operation | Signature | Expected Output Sort | Test |
|---|---|---|---|
| `predicate(e, r, e)` | e × r × e → **a** | Root sort = Assertion | Verify `sort == "assertion"` on root |
| `modify_entity(m, e)` | m × e → **e** | Root sort = Entity | Verify root is entity |
| `modify_relation(m, r)` | m × r → **r** | Root sort = Relation | Verify root is relation |
| `negate(a)` | a → **a** | Root sort = Assertion | Verify root is assertion |
| `conjoin(a, a)` | a × a → **a** | Root sort = Assertion | Verify root is assertion |
| `disjoin(a, a)` | a × a → **a** | Root sort = Assertion | Verify root is assertion |
| `embed(a)` | a → **e** | Root sort = Entity | Verify root is entity |
| `abstract(e)` | e → **m** | Root sort = Modifier | Verify root is modifier |
| `compose(r, r)` | r × r → **r** | Root sort = Relation | Verify root is relation |
| `invert(r)` | r → **r** | Root sort = Relation | Verify root is relation |
| `quantify(m, e)` | m × e → **a** | Root sort = Assertion | Verify root is assertion |
| `bind(e, a)` | e × a → **a** | Root sort = Assertion | Verify root is assertion |
| `modify_assertion(m, a)` | m × a → **a** | Root sort = Assertion | Verify root is assertion |

**Test count:** 13 operations × 1–3 input variants = ~30 tests

### 2.2 — Sort Input Rejection (Wrong-Type Inputs)

Operations must reject inputs of the wrong sort. `predicate(r, e, e)` passes a relation as the first argument instead of an entity — this should fail.

| Operation | Wrong Input | Expected Behavior |
|---|---|---|
| `predicate` | `(→, →, →)` — three relations | Error: first arg must be entity |
| `predicate` | `(●, ●, ●)` — three entities | Error: second arg must be relation |
| `negate` | `(●)` — an entity | Error: input must be assertion |
| `embed` | `(→)` — a relation | Error: input must be assertion |
| `abstract` | `(→)` — a relation | Error: input must be entity |
| `compose` | `(●, ●)` — two entities | Error: both must be relations |
| `invert` | `(●)` — an entity | Error: input must be relation |
| `modify_entity` | `(●, ∠45)` — swapped order | Error: first must be modifier |
| `conjoin` | `(●, ●)` — two entities | Error: both must be assertions |
| `quantify` | `(●, →)` — entity, relation | Error: first must be modifier, second entity |
| `bind` | `(→, ●)` — relation, entity | Error: first must be entity, second assertion |

**Test count:** ~25 sort-rejection tests

### 2.3 — Algebraic Laws (Proven Invariants)

These are the composition laws that the formal specification proves hold. Violation of any of these means the implementation contradicts the theory.

#### Involution Laws
| Law | Test Expression | Assertion |
|---|---|---|
| `negate(negate(a)) = a` | Build assertion, negate twice | Output GIR ≅ input GIR |
| `invert(invert(r)) = r` | Parse `→`, invert twice | Output GIR has same direction as input |
| `negate` is distinct from identity | `negate(a) ≠ a` | Negated assertion differs from original |
| `invert` is distinct from identity | `invert(→) ≠ →` | Inverted arrow differs from original |

#### Associativity
| Law | Test Expression | Assertion |
|---|---|---|
| `compose(r₁, compose(r₂, r₃)) ≅ compose(compose(r₁, r₂), r₃)` | Three arrows | Both orderings produce equivalent GIR |
| `conjoin(a₁, conjoin(a₂, a₃)) ≅ conjoin(conjoin(a₁, a₂), a₃)` | Three assertions | Associative |
| `disjoin(a₁, disjoin(a₂, a₃)) ≅ disjoin(disjoin(a₁, a₂), a₃)` | Three assertions | Associative |

#### De Morgan Law
| Law | Test Expression | Assertion |
|---|---|---|
| `negate(disjoin(a, b)) ≅ conjoin(negate(a), negate(b))` | Two assertions | Both sides produce structurally equivalent GIR |
| `negate(conjoin(a, b)) ≅ disjoin(negate(a), negate(b))` | Dual | Both sides equivalent |

#### Commutativity
| Law | Test Expression | Assertion |
|---|---|---|
| `conjoin(a, b) ≅ conjoin(b, a)` | Two distinct assertions | Order doesn't matter (up to isomorphism) |
| `disjoin(a, b) ≅ disjoin(b, a)` | Two distinct assertions | Order doesn't matter |

#### Embedding Coherence
| Law | Test Expression | Assertion |
|---|---|---|
| `embed(predicate(e, r, e))` produces an entity | Chain operations | Result has entity sort |
| `predicate(embed(a), r, e)` is valid | Use embedded assertion as entity | No error; produces assertion |
| `abstract(embed(a))` produces a modifier | Chain embed→abstract | Result has modifier sort |

**Test count:** ~25 law-verification tests

### 2.4 — Multi-Step Operation Chains

The power of Σ_UL is that operations compose freely (within sort constraints). No existing test chains more than one operation. These tests build deep operation stacks.

| Chain | Steps | Expected |
|---|---|---|
| `embed(predicate(●, →, ●))` → use as entity in another `predicate` | 2 ops | Valid assertion containing nominalized assertion |
| `abstract(embed(predicate(●, →, ●)))` → use as modifier in `modify_entity` | 3 ops | Entity modified by abstracted embedded predication |
| `negate(conjoin(predicate(a,r,b), predicate(c,r,d)))` | 3 ops | Negated conjunction |
| `modify_assertion(abstract(●), predicate(●, →, ●))` | 2 ops | Assertion with entity-derived modifier |
| `bind(○_x, quantify(∠45, ●))` | 2 ops | Bound quantified assertion |
| `compose(invert(→), →)` | 2 ops | Relation: reversed then extended |
| `embed(negate(predicate(●, →, ●)))` | 2 ops | Nominalized negation |
| Deep chain: build 5 operations deep | 5 ops | Valid GIR at each intermediate step |
| Full pipeline: every operation type used once | 13+ ops | Valid final GIR |

**Test count:** ~30 chain tests

### 2.5 — Injectivity (Operation Distinctness Theorem)

The formal specification proves that each operation is injective: distinct inputs produce distinct outputs. This means `predicate(a, r, b) ≠ predicate(c, r, d)` when `a ≠ c` or `b ≠ d`.

| Test | Assertion |
|---|---|
| `predicate(●, →, ●)` vs `predicate(○{●}, →, ●)` | Outputs differ |
| `modify_entity(∠45, ●)` vs `modify_entity(∠90, ●)` | Outputs differ |
| `negate(a)` vs `negate(b)` where a ≠ b | Outputs differ |
| `compose(→, →)` vs `compose(←, →)` | Outputs differ |
| For each operation: same inputs → same output (determinism) | Repeated calls yield identical GIR |

**Test count:** ~20 injectivity tests

---

## Estimated Total: ~130 algebraic depth tests

## Implementation Approach

Helper functions needed:
- `getRootSort(gir)` → extract sort of root node
- `girEquivalent(gir1, gir2)` → structural isomorphism check (node/edge counts, types, topology match — NOT string equality)
- `girDistinct(gir1, gir2)` → at least one structural difference
- `chainOps(ops: Array<{op, args}>)` → sequentially apply operations, returning intermediate and final GIRs
- `makeAssertion(...)` → shortcut for `predicate(●, →, ●)`
- Re-use existing `parseGir()`, `applyOp()`, `composeOp()` helpers from agent-scenarios

Key design principle: **GIR equivalence is structural isomorphism, not string equality.** Node IDs may differ. The tests must compare topology (edge type distributions, node type distributions, sort assignments) not serialized JSON strings.
