# 03 — Deparser Bug Characterization Tests

> **Goal:** Map the exact failure surface of every known deparser bug. Don't work around bugs — *surround them with tests* that document exactly where they break and where they don't. This makes the bug fixable and prevents regressions.

---

## Problem Statement

Two bugs were discovered during the aggressive test pass:
1. **Operator loss inside enclosure content:** `○{● → ● | ● → ●}` deparses to `○{● → ● | ● | ●}` — the second `→` is lost.
2. **Left-arrow in mixed-operator chains inside enclosures:** `○{● → ● | ● ← ●}` produces a malformed GIR edge.

Both were worked around (expressions were rewritten to avoid the bug). This plan characterizes the exact boundaries so the bugs can be fixed in the deparser.

---

## Bug 1: Operator Loss in Enclosure Content

### Root Cause Hypothesis

`deparser.rs:find_operator_between()` scans for line nodes connecting two children. When operators (`→`, `|`) are mixed inside `{}`, the deparser's for-loop finds a line connecting *distal* children and emits it between *adjacent* children, or fails to find the correct operator and falls through to adjacency (`|`).

### Characterization Test Matrix

Each cell tests: does `deparse(parse(X)) == X`?

#### Varying the second operator after `|`

| Expression | Expected | Passes? |
|---|---|---|
| `○{● \| ●}` | `○{● \| ●}` | Baseline — should pass |
| `○{● → ●}` | `○{● → ●}` | Single operator — should pass |
| `○{● → ● \| ●}` | `○{● → ● \| ●}` | → then \| — should pass |
| `○{● \| ● → ●}` | `○{● \| ● → ●}` | \| then → — **characterize** |
| `○{● → ● \| ● → ●}` | `○{● → ● \| ● → ●}` | → \| → — **known broken** |
| `○{● ← ● \| ● ← ●}` | `○{● ← ● \| ● ← ●}` | ← \| ← — **characterize** |
| `○{● ↔ ● \| ● ↔ ●}` | `○{● ↔ ● \| ● ↔ ●}` | ↔ \| ↔ — **characterize** |
| `○{● → ● \| ● ← ●}` | `○{● → ● \| ● ← ●}` | → \| ← — **characterize** |
| `○{● → ● → ● \| ●}` | `○{● → ● → ● \| ●}` | Chain then \| — **characterize** |
| `○{● \| ● → ● → ●}` | `○{● \| ● → ● → ●}` | \| then chain — **characterize** |
| `○{● → ● & ● → ●}` | `○{● → ● & ● → ●}` | → & → — **characterize** |
| `○{● & ● → ●}` | `○{● & ● → ●}` | & then → — **characterize** |

#### Varying the enclosure type

| Enclosure | Expression | Expected |
|---|---|---|
| `○` (circle) | `○{● → ● \| ● → ●}` | **known broken** |
| `△` (triangle) | `△{● → ● \| ● → ●}` | **characterize** |
| `□` (square) | `□{● → ● \| ● → ●}` | **characterize** |
| `⬠` (pentagon) | `⬠{● → ● \| ● → ●}` | **characterize** |
| `⬡` (hexagon) | `⬡{● → ● \| ● → ●}` | **characterize** |

#### Varying the nesting wrapper

| Context | Expression | Expected |
|---|---|---|
| Top-level | `● → ● \| ● → ●` | ✅ (known passing) |
| Inside `○{}` | `○{● → ● \| ● → ●}` | ❌ (known broken) |
| Inside `declare{}` | `declare{● → ● \| ● → ●}` | ❌ (known broken) |
| Inside `assert{}` | `assert{● → ● \| ● → ●}` | **characterize** |
| Inside `[]{}` | `[]{● → ● \| ● → ●}` | **characterize** |
| Inside `<>{}` | `<>{● → ● \| ● → ●}` | **characterize** |
| Inside `?{}` | `?{● → ● \| ● → ●}` | **characterize** |
| Inside `(...)` | `(● → ●) \| (● → ●)` | ✅ (known passing — parens strip to flat) |
| Double-nested | `○{○{● → ● \| ● → ●}}` | **characterize** |

#### Varying the number of mixed operators

| Expression | Operator Count | Expected |
|---|---|---|
| `○{● → ●}` | 1 op | ✅ |
| `○{● → ● \| ●}` | 2 ops | ✅ |
| `○{● \| ● → ●}` | 2 ops | **characterize** |
| `○{● → ● \| ● → ●}` | 3 ops | ❌ |
| `○{● → ● \| ● → ● \| ●}` | 4 ops | **characterize** |
| `○{● → ● → ● \| ● → ●}` | 4 ops (chain + mixed) | **characterize** |
| `○{● → ● \| ● \| ● → ●}` | 4 ops (two \| + two →) | **characterize** |

### Diagnostic Tests

Beyond pass/fail, these tests dump the GIR and deparser output for manual inspection:

1. Parse `○{● → ● | ● → ●}` → dump full GIR node list and edge list
2. Show which node IDs map to which operator positions
3. Show the deparser's `find_operator_between()` call sequence and results
4. Compare GIR from top-level `● → ● | ● → ●` vs enclosed version — they should be structurally identical except for the enclosure wrapper

**Test count for Bug 1:** ~45 characterization tests + ~5 diagnostic tests

---

## Bug 2: Left-Arrow in Mixed-Operator Chains Inside Enclosures

### Root Cause Hypothesis

When `←` appears after `|` inside enclosure content, the parser transform creates a `Connects` edge from `line → line` (wrong) instead of `point → line` (correct). The parser correctly builds the AST, but the AST→GIR transform misassigns the connection source.

### Characterization Test Matrix

| Expression | Expected Behavior |
|---|---|
| `● ← ●` | ✅ (bare works) |
| `○{● ← ●}` | **characterize** — single ← inside enclosure |
| `● → ● \| ● ← ●` | ✅ at top level |
| `○{● → ● \| ● ← ●}` | ❌ (known broken) |
| `○{● ← ● \| ● → ●}` | **characterize** — ← before \| |
| `○{● ← ● \| ● ← ●}` | **characterize** — both ← |
| `○{● ← ● ← ●}` | **characterize** — chain of ← |
| `○{● \| ● ← ●}` | **characterize** — \| then ← |
| `○{● ← ● & ●}` | **characterize** — ← with & |
| `△{● → ● \| ● ← ●}` | **characterize** — triangle wrapper |
| `[]{● → ● \| ● ← ●}` | **characterize** — modal wrapper |
| `assert{● → ● \| ● ← ●}` | **characterize** — force wrapper |

### GIR Edge Validation

For every expression above, check:
- Every `Connects` edge has source/target alternating between point and line types
- No `Connects` edge has `line → line`
- No `Connects` edge has `point → point`
- Direction vectors are correct for the arrow type (`←` should have `[-1, 0]`)

**Test count for Bug 2:** ~20 characterization + ~10 GIR edge validation

---

## Bug 3 (Potential): Deparser Modal vs Assertion Modifier Overlap

### Hypothesis (from code review)

`deparser.rs` lines 48–68 check for assertion modifiers and modal operators using overlapping detection logic. Both use node labels/patterns on enclosure nodes. If an enclosure has BOTH a modal label and an assertion modifier, the deparser may emit only one.

### Characterization Tests

| Expression | Expected Output |
|---|---|
| `!{[]{● → ●}}` | Emphatic wrapping necessity — both preserved? |
| `?{<>{○{●}}}` | Evidential wrapping possibility — both preserved? |
| `~?{[]->{●}{●}}` | Hedged counterfactual — both preserved? |
| `assert{!{● → ●}}` | Force wrapping modifier — both preserved? |
| `!{assert{● → ●}}` | Modifier wrapping force — order preserved? |
| `?{query{● → ●}}` | Evidential query — no conflict? |

**Test count for Bug 3:** ~12 characterization tests

---

## Estimated Total: ~92 deparser characterization tests

## Implementation Approach

Each characterization test follows a standard template:
```typescript
it("characterize: ○{● → ● | ● → ●}", () => {
  const input = "○{● → ● | ● → ●}";
  const girJson = wasm.parseUlScript(input);
  const deparsed = wasm.deparseGir(girJson);
  
  // Record actual behavior (may be a known failure)
  if (deparsed === input) {
    // PASS: mark as "correct behavior confirmed"
  } else {
    // KNOWN BUG: record the actual output for regression tracking
    // Uncomment the next line when the bug is fixed:
    // expect(deparsed).toBe(input);
    expect(deparsed).toBe("○{● → ● | ● | ●}"); // current (buggy) output
  }
});
```

This approach means:
- Tests PASS even when bugs exist (they assert the *actual behavior*)
- When bugs are fixed, change the assertion to the *correct behavior*
- If a bug regresses, the test catches it immediately
- The test file serves as living documentation of the exact bug surface
