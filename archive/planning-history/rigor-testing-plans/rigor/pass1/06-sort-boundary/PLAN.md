# 06 — Sort Boundary & Error Handling Tests

> **Goal:** Verify that the system enforces type safety at every operation boundary. When an agent passes a relation where an entity is expected, the system must produce a clear error — not silently produce nonsense GIR.

---

## Problem Statement

No existing test verifies what happens when operations receive wrong-sort inputs. The formal specification says `predicate : e × r × e → a` — entity, relation, entity. But what does the implementation do when you pass `predicate(→, →, →)` (three relations)? Does it:
1. Reject with a clear error? (correct)
2. Silently produce a malformed GIR? (dangerous)
3. Crash? (unacceptable)

This is the adversarial *semantic* test surface — not malformed syntax (parser handles that), but well-formed GIR with wrong sort assignments.

---

## Test Categories

### 6.1 — Every Operation × Every Wrong-Sort Combination

For each of the 13 operations, enumerate every possible sort-mismatch input and verify clean error.

#### predicate(e × r × e → a) — 3 arguments, 4 sorts each
| Arg1 (expects e) | Arg2 (expects r) | Arg3 (expects e) | Expected |
|---|---|---|---|
| entity ✓ | relation ✓ | entity ✓ | Success |
| **relation** | relation | entity | Error: arg1 must be entity |
| **modifier** | relation | entity | Error: arg1 must be entity |
| **assertion** | relation | entity | Error: arg1 must be entity |
| entity | **entity** | entity | Error: arg2 must be relation |
| entity | **modifier** | entity | Error: arg2 must be relation |
| entity | **assertion** | entity | Error: arg2 must be relation |
| entity | relation | **relation** | Error: arg3 must be entity |
| entity | relation | **modifier** | Error: arg3 must be entity |
| entity | relation | **assertion** | Error: arg3 must be entity |

**For predicate alone: 9 error cases**

#### Unary operations (negate, embed, abstract, invert) — 1 argument, 3 wrong sorts each
| Operation | Expects | Wrong Inputs | Error Cases |
|---|---|---|---|
| `negate(a → a)` | assertion | entity, relation, modifier | 3 |
| `embed(a → e)` | assertion | entity, relation, modifier | 3 |
| `abstract(e → m)` | entity | relation, modifier, assertion | 3 |
| `invert(r → r)` | relation | entity, modifier, assertion | 3 |

**12 error cases**

#### Binary operations — 2 arguments, various wrong combinations
| Operation | Expects | Error Cases |
|---|---|---|
| `modify_entity(m × e → e)` | modifier, entity | ~6 |
| `modify_relation(m × r → r)` | modifier, relation | ~6 |
| `conjoin(a × a → a)` | assertion, assertion | ~6 |
| `disjoin(a × a → a)` | assertion, assertion | ~6 |
| `compose(r × r → r)` | relation, relation | ~6 |
| `quantify(m × e → a)` | modifier, entity | ~6 |
| `bind(e × a → a)` | entity, assertion | ~6 |
| `modify_assertion(m × a → a)` | modifier, assertion | ~6 |

**~48 error cases**

#### Arity violations (wrong number of arguments)
| Operation | Expected Arity | Test Inputs | Error Cases |
|---|---|---|---|
| `predicate` | 3 | 0, 1, 2, 4 args | 4 |
| `negate` | 1 | 0, 2, 3 args | 3 |
| `embed` | 1 | 0, 2 args | 2 |
| `abstract` | 1 | 0, 2 args | 2 |
| `invert` | 1 | 0, 2 args | 2 |
| `conjoin` | 2 | 0, 1, 3 args | 3 |
| `compose` | 2 | 0, 1, 3 args | 3 |
| All 13 operations | N | 0 args | 13 |

**~32 arity error cases**

### Total for 6.1: ~101 sort/arity error tests

### 6.2 — Error Message Quality

Beyond "does it error?" — are the error messages useful to an agent?

| Test | Assertion |
|---|---|
| Every sort error | Message mentions the expected sort name |
| Every sort error | Message mentions which argument position is wrong |
| Every arity error | Message mentions expected count and received count |
| Unknown operation | Message names the unknown operation |
| Error messages | Never contain internal implementation details (no Rust panic traces) |
| Error messages | Are parseable by another agent (structured enough to react to) |

**~15 error quality tests**

### 6.3 — Edge Cases at Sort Boundaries

Some GIR nodes have ambiguous sorts depending on context:

| Test | Edge Case | Expected |
|---|---|---|
| Curve as entity vs relation | `~` defaults to Relation sort, but can be entity? | Operation should check actual sort |
| VariableSlot as entity | `○_x` is entity sort — can it be used in entity-expecting operations? | Should work |
| Point with variable_id as entity | `●_x` is entity sort | Should work in entity-expecting ops |
| Angle as modifier | `∠45` is modifier sort | Should work in modifier-expecting ops |
| Angle used as entity | Passing `∠45` where entity expected | Should error |
| Enclosure without content | `○` alone — entity or assertion? | Depends on context — test both |
| Assertion used as entity (without embed) | Passing assertion directly to entity-expecting op | Error — must embed first |
| Entity used as assertion (without predicate) | Passing entity directly to assertion-expecting op | Error — must predicate first |

**~15 boundary tests**

### 6.4 — GIR Validator Tests

If a GIR validator exists (or should exist), test that it catches structural violations:

| Violation | Expected |
|---|---|
| Cycle in `Contains` edges | Error or warning |
| Node with two `Contains` parents | Error (tree, not DAG) |
| `Connects` edge: point→point (missing line) | Error |
| `Connects` edge: line→line (invalid) | Error |
| Missing root node | Error |
| Root node ID not in nodes list | Error |
| Edge referencing non-existent node ID | Error |
| Node with out-of-range angle measure (e.g., ∠-1, ∠361) | Error or clamp |
| Node with sort field contradicting node_type | Warning |
| Empty nodes array | Error or valid empty document |

**~15 validator tests**

---

## Estimated Total: ~146 sort-boundary & error tests

## Implementation Approach

Each sort-error test follows:
```typescript
it("predicate rejects relation as first arg", () => {
  const relation = wasm.parseUlScript("→");
  const entity = wasm.parseUlScript("●");
  expect(() => wasm.applyOperation("predicate", JSON.stringify([relation, relation, entity])))
    .toThrow(/entity|sort|type/i);
});
```

Note: Tests use regex matching on error messages to verify quality without coupling to exact wording.

**Key discovery potential:** Some of these tests may reveal that the implementation does NOT enforce sort constraints at the WASM boundary — it might silently produce GIR with wrong sorts. This is an important finding either way:
- If sorts are enforced → tests pass, system is type-safe
- If sorts are NOT enforced → tests document the gap, and we know the type system is decorative not operational
