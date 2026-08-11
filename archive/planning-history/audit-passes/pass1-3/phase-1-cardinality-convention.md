# Phase 1 — Cardinality Convention

**Status:** ✅ COMPLETE
**New operations:** 0  
**D2 cases affected:** 3.3, 5.2, 10.4  
**Expected case conversions:** 3 (⚠️ → ✅ with convention)  
**Dependencies:** None  
**Blocks:** None

---

## Problem Statement

Three D2 cases share the same root cause: UL has no arithmetic.

| Case | Expression | What needs cardinality |
|------|-----------|----------------------|
| 3.3 | "Exactly three students passed" | Exact count: \|{x : Student(x) ∧ Passed(x)}\| = 3 |
| 5.2 | "I have been studying for three hours" | Duration measure: t = 3 hours |
| 10.4 | Mandarin "三本书" (three + CLF + book) | Classifier + numeral: 3 instances of book |

In each case, the *structural meaning* (quantification, temporal aspect, classification) is expressible via existing operations. The sticking point is the **number 3** itself.

---

## Design Options

### Option A: Add a Cardinality Operation

```
count : ℕ × e → a    # "exactly n entities satisfying..."
```

**Geometric grounding:** n copies of a mark inside a frame.

**Pros:** Compact, handles all three cases directly.

**Cons:**
- Imports ℕ into the sort system — a fundamental type expansion
- Arithmetic is not geometric (ℕ has algebraic structure that doesn't arise from ℝ² geometry)
- Opens the door to "why not ℝ?" (continuous measurement) and "why not ℤ?" (negative counts)
- Violates the principle that UL's sorts are geometrically grounded

**Verdict:** Clean solution but wrong layer. Cardinality belongs to arithmetic, not semantics.

### Option B: Declare Arithmetic as External Domain (RECOMMENDED)

**Mechanism:** Document that natural number arithmetic is an **external formal system** that UL interfaces with but does not internalize. UL can *reference* specific cardinalities via `modify_entity` with conventional modifiers, just as it references specific colors or speeds.

**Convention:**

```
m_n = modifier (Sim(2) scaling by factor n)    # "n-fold" as entity scaling
```

For "exactly three students passed":
```
e_student = entity "student"
r_pass    = relation "passed"
m_3       = modifier (3-fold scaling — conventional cardinality marker)

bind(○_x, pred(me(m_3, ○_x), r_pass, e_exam)) where ○_x ← e_student
```

The modifier `m_3` is a **conventional assignment** (T3 tier in the lexicon), just like `m_red` for the color red. The writing system represents it as 3 marks or a numeral glyph inside the modifier position.

**Pros:**
- No new operations, sorts, or types
- Consistent with how UL handles other domain-specific knowledge (colors, shapes, speeds)
- Preserves geometric grounding (scaling by integer factor IS a valid Sim(2) element)
- Maps cleanly to the T3 (conventional) tier of the lexicon

**Cons:**
- "Exactly 3 and no more" requires conjunction with negation: `conj(a_3_passed, neg(a_4th_passed))`
- Verbose for large numbers
- Doesn't generalize to continuous measurement without additional conventions

**Verdict:** Most principled. Arithmetic is not a semantic primitive — it's a domain that UL can reference.

### Option C: Expressibility via Finite Enumeration

**Mechanism:** Document that cardinality IS expressible (albeit verbosely) via existing operations:

"Exactly 3 students passed" =
```
bind(○_1, bind(○_2, bind(○_3,
  conj(conj(
    pred(○_1, r_pass, e_exam),
    pred(○_2, r_pass, e_exam)),
    conj(
      pred(○_3, r_pass, e_exam),
      neg(bind(○_4, pred(○_4, r_pass, e_exam)))  # "no 4th"
    ))
))) where ○_i ← e_student, all distinct
```

**Pros:** No conventions needed — pure algebraic expressibility
**Cons:** O(n) complexity, impractical for large n, doesn't handle continuous quantities

**Verdict:** Proves expressibility but is not a practical representation.

---

## Recommended Approach

**Use Option B (external domain) as the primary convention, with Option C as the expressibility proof.**

### Formal Statement

> **Cardinality Convention:** Natural number arithmetic is an external formal system. UL references specific cardinalities via conventional modifiers in Gₘ (T3 tier). The modifier `m_n` for cardinality n is geometrically grounded as n-fold scaling in Sim(2). Exact cardinality ("exactly n, no more") is expressible via finite conjunction + negation (Option C), establishing that cardinality is within UL's expressible scope — the convention is for compactness, not capability.

### What This Means for Each Case

| Case | Resolution |
|------|-----------|
| 3.3 "Exactly three students passed" | ✅ — `m_3` conventional modifier + bind + conjunction + negation |
| 5.2 "three hours" | ✅ — `m_3` as duration modifier (temporal domain scaling: 3 × hour-unit) |
| 10.4 "三本书" | ✅ — `m_3` as cardinality modifier + classifier via `abstract` (flat-object shape extraction) |

---

## Deliverables

- [ ] Add §6 "Cardinality Convention" to `foundations/formal-operations.md` documenting the external-domain interface
- [ ] Add cardinality modifier examples to `ul-core/lexicon/lexicon.md` (T3 tier)
- [ ] Re-score D2 cases 3.3, 5.2, 10.4 in `experiments/D2-completeness-challenge.md`
- [ ] Update `ul-core/CRITIQUE.md` with cardinality resolution
- [ ] Add visual convention for numeric modifiers to `ul-core/grammar/formal-grammar.md` (optional)

---

## Precedent and Justification

This approach mirrors how formal logic handles arithmetic:

- **First-order logic** can express "exactly 3" via finite sentences but has no native number sort
- **Montague grammar** treats numerals as generalized quantifiers (determiners), not as a separate formal layer
- **Peano arithmetic** is a separate axiom system that gets *combined* with first-order logic, not embedded in it

UL follows the same architecture: the semantic primitives are geometric, and arithmetic interfaces as an external specification — just as it does in Montague's PTQ, where numerals are handled by the lexicon, not the type system.
