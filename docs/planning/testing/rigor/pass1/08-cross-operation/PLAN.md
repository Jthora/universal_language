# 08 — Cross-Operation Interaction Tests

> **Goal:** Test the interactions *between* operations that the formal specification implies but no test exercises. UL's power is that operations compose freely within sort constraints. The current tests treat each operation as isolated. These tests prove the operations form a coherent algebra.

---

## Problem Statement

The 13 Σ_UL operations are not independent tools — they form an algebra. The formal specification defines them as operations on shared carrier sets. The current tests call each operation once and check node counts. No test explores what happens when you use the *output* of one operation as the *input* to another — which is the entire point of having an algebra.

---

## Test Categories

### 8.1 — Every Pairwise Operation Combination

For each pair (op_A, op_B) where the output sort of op_A matches an input sort of op_B, verify the composition works.

**Sort-compatible chains (output of A feeds input of B):**

| Chain | op_A output sort | op_B input sort | Expression |
|---|---|---|---|
| `embed(predicate(...))` | a → **e** | **e** → ... | Nominalize a predication |
| `predicate(embed(a), r, e)` | **e** from embed | **e** in predicate | Use nominalized assertion as subject |
| `abstract(embed(a))` | a → e → **m** | **m** → ... | Abstracted nominalized assertion |
| `modify_entity(abstract(e), e₂)` | e → **m** | **m** × e → e | Modify entity with abstracted entity |
| `modify_relation(abstract(e), r)` | e → **m** | **m** × r → r | Modify relation with abstracted entity |
| `negate(predicate(...))` | a → **a** | **a** → a | Double operation on assertions |
| `negate(conjoin(a, b))` | a → **a** | **a** → a | Negate a conjunction |
| `negate(disjoin(a, b))` | a → **a** | **a** → a | Negate a disjunction |
| `conjoin(negate(a), negate(b))` | a → **a** × 2 | **a** × **a** → a | De Morgan decomposition |
| `compose(invert(r), r)` | r → **r** | **r** × r → r | Invert then compose |
| `invert(compose(r₁, r₂))` | r → **r** | **r** → r | Compose then invert |
| `quantify(abstract(e), e₂)` | e → **m** | **m** × e → a | Quantify with abstracted entity |
| `bind(e, quantify(m, e₂))` | a from quantify | e × **a** → a | Bind a quantified assertion |
| `modify_assertion(abstract(e), predicate(...))` | e → **m**, a | **m** × **a** → a | Modify assertion with abstracted entity |
| `embed(negate(predicate(...)))` | a → **e** | **e** | Nominalize a negated predication |
| `predicate(e, compose(r₁, r₂), e₂)` | r from compose | e × **r** × e → a | Use composed relation in predication |
| `predicate(e, invert(r), e₂)` | r from invert | e × **r** × e → a | Use inverted relation in predication |
| `conjoin(predicate(a), predicate(b))` | a × 2 | **a** × **a** → a | Conjoin two predications |
| `disjoin(predicate(a), negate(predicate(b)))` | a × 2 | **a** × **a** → a | Disjoin predication with negated predication |
| `modify_entity(m, embed(predicate(...)))` | m × e | **m** × **e** → e | Modify a nominalized assertion-entity |

**Test count:** ~30 pairwise chain tests

### 8.2 — Three-Deep Operation Chains

| Chain (3 operations) | Description |
|---|---|
| `negate(conjoin(predicate(●,→,●), predicate(●,←,●)))` | Negate conjunction of two predications |
| `embed(negate(predicate(●,→,●)))` | Nominalize a negation |
| `predicate(embed(negate(a)), →, ●)` | Use nominalized negation as subject |
| `abstract(embed(predicate(●,→,●)))` | Entity→Modifier via embed+abstract |
| `modify_entity(abstract(embed(a)), ●)` | Apply abstracted-embedded assertion as modifier |
| `conjoin(negate(a), negate(b))` where a, b are predications | De Morgan right-hand side |
| `invert(compose(invert(→), →))` | Invert(compose(invert, identity)) |
| `quantify(abstract(●), embed(predicate(●,→,●)))` | Quantify with abstracted entity over nominalized assertion |
| `bind(○_x, quantify(∠45, ●))` | Bind variable to quantified assertion |
| `modify_assertion(∠45, conjoin(a, b))` | Modify a conjunction |

**Test count:** ~15 three-deep tests

### 8.3 — Five-Deep Operation Chains

Push the algebra to realistic depth.

| Chain (5 operations) | Description |
|---|---|
| `negate(conjoin(embed(predicate(●,→,●)), predicate(●,←,●)))` — wait, sort error: embed produces entity, conjoin expects assertion | This should ERROR — sort mismatch at depth 3 |
| `modify_assertion(abstract(embed(negate(predicate(●,→,●)))), predicate(●,→,●))` | 5-deep: predicate→negate→embed→abstract→modify_assertion |
| `predicate(modify_entity(abstract(embed(predicate(●,→,●))), ●), compose(→,→), ●)` | 5-deep: predicate→embed→abstract→modify_entity→predicate |
| `negate(conjoin(predicate(embed(negate(predicate(●,→,●))), →, ●), predicate(●,←,●)))` | 5-deep with nested nominalization |
| `bind(○_x, quantify(abstract(embed(predicate(●,→,●))), embed(predicate(●,←,●))))` | 5-deep: predicate→embed→abstract→quantify→bind |

**Test count:** ~10 five-deep tests

### 8.4 — Modal + Force + Modifier Orthogonality

The formal specification says force, assertion modifier, and modal operators compose orthogonally. Test all combinations.

| Force | Modal | Modifier | Expression (conceptual) | Test |
|---|---|---|---|---|
| assert | necessary | evidential | `assert{?{[]{● → ●}}}` | Parse → verify all three annotations present in GIR |
| query | possible | emphatic | `query{!{<>{● → ●}}}` | All three present |
| direct | counterfactual | hedged | `direct{~?{[]->{●}{●}}}` | All three present |
| commit | necessary | emphatic | `commit{!{[]{● → ●}}}` | All three present |
| express | possible | evidential | `express{?{<>{●}}}` | All three present |
| declare | necessary | hedged | `declare{~?{[]{● → ●}}}` | All three present |

For each: verify that after round-trip through SVG, ALL three annotations survive independently.

**Additional tests:** What happens when you apply operations to triply-annotated assertions?
| Operation on triply-annotated | Expected |
|---|---|
| `negate(assert{!{[]{●→●}}})` | Negation flips sign, preserves force+modal+modifier |
| `embed(assert{!{[]{●→●}}})` | Nominalization preserves all annotations on embedded structure |
| `conjoin` two triply-annotated | Both annotations preserved in their respective branches |

**Test count:** ~20 orthogonality tests

### 8.5 — Operation Chains Across SVG Boundaries

Combine cross-operation testing with pipeline testing: perform operation A, serialize to SVG, extract, perform operation B.

| Step 1 (Agent A) | SVG Boundary | Step 2 (Agent B) | Verification |
|---|---|---|---|
| `predicate(●, →, ●)` | → SVG → extract | `negate()` on extracted GIR | Negated assertion, correct sort |
| `compose(→, →)` | → SVG → extract | `invert()` on extracted GIR | Inverted composed relation |
| `conjoin(a₁, a₂)` | → SVG → extract | `negate()` on extracted GIR | De Morgan applicable |
| `embed(predicate(...))` | → SVG → extract | `abstract()` on extracted GIR | Modifier sort output |
| `abstract(embed(predicate(...)))` | → SVG → extract | `modify_entity(result, ●)` | Modified entity |
| 3-step relay: A→SVG→B→SVG→C | 2 boundaries | C applies final operation | Full relay works |

**Test count:** ~15 cross-boundary chain tests

---

## Estimated Total: ~90 cross-operation tests

## Implementation Notes

### Discovering sort errors in deep chains

Some chains that *look* valid are actually sort errors at intermediate steps. For example:
```
conjoin(embed(predicate(●,→,●)), predicate(●,←,●))
```
This looks like "conjoin two things." But `embed` returns an *entity*, and `conjoin` expects two *assertions*. The first argument has the wrong sort.

These tests should discover whether the implementation catches such errors or silently produces garbage. If the implementation doesn't check sorts, the test should document this as a finding.

### GIR equivalence at each step

For algebraic law tests, we need to compare GIR at intermediate and final steps. The `girIsomorphic()` function from plan 07 is essential infrastructure here too.

### Test generation strategy

Many of these tests can be partially auto-generated:
1. Enumerate all sort-compatible two-operation chains from the 13 operations
2. For each chain, generate a test case with concrete UL-Script inputs
3. Verify each step produces valid GIR with correct output sort
4. Verify the final result survives SVG round-trip

The enumeration is finite: 13 operations × (average ~4 sort-compatible followers) = ~52 two-step chains.
