# 01 — Semantic-Structural Verification Tests

> **Goal:** Prove that UL expressions don't just *survive* round-trip — they produce GIR graphs with the correct *structural meaning*. Every comment that says "feedback loop" should correspond to a GIR with a detectable cycle. Every comment that says "containment hierarchy" should produce a tree of `Contains` edges.

---

## Problem Statement

The current test suite's "semantic" tests are syntax round-trips with narrative labels. `expectRoundTrip("○_x{~ → ●_x}")` proves the string survives — it proves nothing about whether the GIR encodes a feedback loop. The UL claim is that *structure IS meaning*. If we don't verify structure, we verify nothing.

---

## Test Categories

### 1.1 — Edge-Type Verification

For every UL expression pattern, verify the GIR contains the exact edge types predicted by the Σ_UL specification.

| Expression | Expected Edge Types | Structural Assertion |
|---|---|---|
| `● → ●` | 2× `Connects` | point → line → point chain |
| `● \| ●` | 1× `Adjacent` | points share boundary |
| `● & ●` | 1× `Intersects` | points overlap |
| `○{●}` | 1× `Contains` | enclosure encloses point |
| `○_x{●_x}` | 1× `Contains` + 1× `Binds` | binding creates co-reference |
| `[]{●}` | `Contains` + `AccessibleFrom` | modal world structure |
| `∠45 → ●` (via `modify_relation`) | `ModifiedBy` | angle modifies line |

**Test count:** ~30 expression patterns × edge-type assertions

### 1.2 — Containment Tree Invariants

The `Contains` edges must form a **tree** (acyclic, single-parent per child, rooted). This is a fundamental invariant of UL's geometric model — enclosures are Jordan domains, and Jordan domains nest but don't share interior without intersection.

| Test | Assertion |
|---|---|
| `○{○{●}}` | Contains edges form path root→outer→inner→point |
| `○{● \| ●}` | Both points are children of same enclosure |
| `○{○{●}} \| ○{●}` | Two independent containment trees under implicit root |
| `○{○{○{○{○{●}}}}}` | Contains chain has length exactly 5 |
| Every parsed expression | No node has >1 `Contains` parent (tree, not DAG) |
| Every parsed expression | No cycle in `Contains` edges |
| Every parsed expression | Root node has no `Contains` edges targeting it |

**Test count:** ~15 structural invariants × 10 expression variants = ~50 tests

### 1.3 — Connection Chain Topology

`Connects` edges encode directed relations. The invariant is: connections always go `point → line → point` (or reverse for `←`). A line node never connects directly to another line node. A point never connects to a point without an intervening line.

| Test | Assertion |
|---|---|
| `● → ●` | Exactly: point₁ →Connects→ line →Connects→ point₂ |
| `● → ● → ●` | Chain: p₁→l₁→p₂→l₂→p₃ (alternating point/line) |
| `● ← ●` | Reverse direction vector, but same topology |
| `● ↔ ●` | Line has `directed: false` |
| Every parsed connection | Source of Connects edge is always point or line |
| Every parsed connection | Target of Connects edge is always line or point |
| Every parsed expression | No two consecutive Connects edges have same-type source/target |

**Test count:** ~20 tests

### 1.4 — Variable Binding Graph Structure

Variable binding creates co-reference edges. When `○_x` appears and `●_x` appears in the same scope, there must be a `Binds` or `References` edge connecting them by variable ID.

| Test | Assertion |
|---|---|
| `○_x{●_x}` | Node with `variable_id: "x"` appears twice; edge links them |
| `○_x → ●_x` | Cross-structure reference edge exists |
| `○_x{●_x → ●_x}` | Both bound refs link back to the slot |
| `○_x \| ○_y` | Two distinct variable scopes, no cross-edges |
| `○_x{○_y{●_x}}` | Binding crosses containment depth — edge spans levels |

**Test count:** ~15 tests

### 1.5 — Modal World Structure

Modal operators create distinguished world entities and accessibility relations. `[]{a}` asserts `a` in all accessible worlds. The GIR should contain `w_current` entity, accessibility relation `r_alethic`, and quantification over worlds.

| Test | Assertion |
|---|---|
| `[]{●}` | GIR contains node with label `w_current` or equivalent |
| `[]{● → ●}` | Modal context has `world_nodes` populated |
| `<>{●}` | Possibility: same world structure with existential quantification |
| `[]->{A}{B}` | Counterfactual: closeness relation present |
| `[]{<>{●}}` | Nested modals: multiple world-scope levels |

**Test count:** ~15 tests

### 1.6 — Force Annotation Preservation

Force annotations decorate assertion frames. After operations (embed, modify_assertion, negate), the force must be preserved or correctly transformed.

| Test | Assertion |
|---|---|
| `assert{● → ●}` | Root node has `force: Assert` |
| `query{● → ●}` | Root node has `force: Query` |
| All 6 forces | Each produces distinct `force` field value |
| `negate(assert{● → ●})` | Force preserved, sign flipped to ⊖ |
| `embed(assert{● → ●})` | Nominalized assertion retains force metadata |

**Test count:** ~20 tests

### 1.7 — Assertion Modifier Structure

Assertion modifiers (`?`, `!`, `~?`) decorate the frame boundary. These must appear as `assertion_modifier` fields on the enclosure node, not as separate nodes.

| Test | Assertion |
|---|---|
| `?{● → ●}` | Node has `assertion_modifier: Evidential` |
| `!{● → ●}` | Node has `assertion_modifier: Emphatic` |
| `~?{● → ●}` | Node has `assertion_modifier: Hedged` |
| `!{[]{●}}` | Modifier on outer frame, modal on inner |
| `assert{?{● → ●}}` | Force and modifier compose orthogonally |

**Test count:** ~15 tests

---

## Estimated Total: ~165 structural verification tests

## Implementation Approach

Each test should:
1. Parse the UL-Script expression
2. Inspect the GIR JSON directly (not just round-trip the string)
3. Assert specific structural properties (node types, edge types, field values)
4. Be grouped by the structural invariant being verified, not by the expression

Helper functions needed:
- `findEdgesOfType(gir, edgeType)` → filtered edge list
- `findNodesOfType(gir, nodeType)` → filtered node list
- `assertContainmentIsTree(gir)` → validates tree invariant
- `assertConnectionChainValid(gir)` → validates alternating point/line
- `findNodeByVariableId(gir, varId)` → locate bound/slot nodes
- `getForce(gir)` → extract force from root assertion node
- `getAssertionModifier(gir)` → extract modifier from assertion node
