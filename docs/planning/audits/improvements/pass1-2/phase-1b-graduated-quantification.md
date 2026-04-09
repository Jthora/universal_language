# Phase 1B — Graduated Quantification

**Status:** ✅ COMPLETE (April 7, 2026)  
**Risk:** Low  
**New operations:** 0 (parameter extension of existing `quantify`)  
**D2 cases affected:** 3.2, 3.3  
**Expected case conversions:** 2 (3.2 ❌→✅, 3.3 ⚠️→✅)  
**Dependencies:** None  
**Blocks:** Phase 4 (Montague mapping needs final quantify semantics)

---

## Problem Statement

The current `quantify` operation (formal-operations.md §1.11) supports three modes:

| Mode | Modifier | Geometric Realization | Natural Language |
|---|---|---|---|
| Universal (∀) | σ_Λ (Λ > 1, expand to fill frame) | Entity fills entire frame | "all", "every", "each" |
| Existential (∃) | σ_λ (0 < λ < 1, shrink in frame) | Entity is small within frame | "some", "a", "there exists" |
| Negative (¬∃) | negate(quantify(σ_λ, e)) | Small entity in dashed frame | "no", "none", "there is no" |

This is a **binary** quantification system: ∀ or ∃, with ¬∃ derived via negation.

### The Gap

Natural languages routinely use **proportional/graduated quantifiers** that sit between ∀ and ∃:

| Quantifier | Approximate Meaning | Cannot Express in Current UL |
|---|---|---|
| "most" | > 50% but < 100% | ❌ |
| "many" | contextually large proportion | ❌ |
| "few" | contextually small proportion | ❌ |
| "several" | more than two, less than many | ❌ |
| "about half" | ≈ 50% | ❌ |
| "exactly three" | cardinality = 3 | ❌ (requires arithmetic) |
| "at least five" | cardinality ≥ 5 | ❌ (requires arithmetic) |

D2 Case 3.2 ("Most dogs chase some cat") was scored ❌ because "most" is inexpressible.  
D2 Case 3.3 ("Exactly three students passed") was scored ⚠️ because cardinality requires enumeration + negation at O(n) cost.

---

## Proposed Solution: Continuous Frame-Fill Parameter

### Core Idea

Replace the binary ∀/∃ modes with a continuous parameter p ∈ [0, 1] representing the **proportion of the frame that the entity fills.**

This is geometrically natural: "how much of the available scope does this entity occupy?" The answer is a proportion, directly visible as a spatial fraction.

### Formal Definition (Amendment to §1.11)

Given modifier m_p ∈ Gₘ parameterized by p ∈ [0, 1], and entity e = (C, D) ∈ Gₑ, let F be the ambient sentence frame:

```
quantify(m_p, e) = (F, σ_p(C), ⊕)
```

where σ_p is the scaling that makes the entity occupy proportion p of the frame area:

```
Area(σ_p(C)) / Area(F) = p
```

Concretely:
- σ_p = σ_{√(p · Area(F) / Area(C))} — scale the entity so its area is p × frame area

### Quantifier Spectrum

| p Value | Quantifier | Geometric State | Visual |
|---|---|---|---|
| p = 1 | Universal (∀) | Entity fills entire frame | Entity expanded to frame boundary |
| p ∈ (0.5, 1) | "most" | Entity fills majority | Entity larger than half the frame |
| p = 0.5 | "about half" | Entity fills half | Entity occupies exactly half |
| p ∈ (0, 0.5) | "few" | Entity fills minority | Entity small but present |
| p → 0⁺ | Existential (∃) | Entity is point-like | Entity minimal in frame |
| p = 0 + negate | Negative (¬∃) | Entity absent (denied) | Empty dashed frame |

### Compatibility with Existing System

The current three modes are special cases:
- Universal: p = 1
- Existential: p → 0⁺ (shrink to point-like)
- Negative: negate(quantify(m_{0⁺}, e))

**No existing constructions break.** Every current use of `quantify` continues to work with p = 1 or p → 0⁺. The extension only adds intermediate values.

---

## Verification Requirements

### 1. Closure
σ_p(C) is a compact subset of ℝ² (continuous image of compact set under scaling). The result (F, σ_p(C), ⊕) is an element of Gₐ. ✓

### 2. Totality
For any p ∈ [0, 1], any entity e with Area(C) > 0, and any frame F with Area(F) > 0, the scaling factor √(p · Area(F)/Area(C)) is well-defined and positive.  
**Edge case:** p = 0 produces σ₀(C) = {centroid(C)}, a single point. The resulting assertion "there is a point-entity in the frame" is the minimal existential. ✓

### 3. Determinism
The scaling factor is uniquely determined by p, Area(C), and Area(F). ✓

### 4. Injectivity
Different values of p produce entities with different area proportions. If quantify(m_p, e) = quantify(m_q, e) for the same e and F, then Area(σ_p(C))/Area(F) = Area(σ_q(C))/Area(F), which implies p = q. ✓

### 5. No New Sort / No New Operation

The parameter p is carried by the modifier m_p ∈ Gₘ. Specifically, m_p = σ_{√(p · Area(F)/Area(C))} is a scaling transformation — already an element of Sim(2) ⊂ Gₘ. **No new sort or operation is needed.** The extension is purely a refinement of how `quantify` interprets its modifier input.

---

## Design Decisions

### Q1: Is p a property of the modifier or of the quantification context?

**Decision:** p is a property of the modifier. The modifier m_p ∈ Gₘ IS a specific scaling, and different p values correspond to different modifiers. This keeps the operation signature unchanged: `quantify(m, e) → a`.

### Q2: What about cardinality ("exactly three")?

Graduated quantification addresses PROPORTION, not COUNT. "Exactly three" still requires:
- Either: enumeration + negation (current workaround, O(n))
- Or: a number system integrated into UL (out of scope for Phase 1B; documented in gap-analysis.md §1.5)

**Decision:** Phase 1B addresses proportional quantifiers only. Cardinality quantification remains a known limitation.

### Q3: How precise is p?

Natural language quantifiers are vague ("most" ≈ 60–95%, "few" ≈ 5–30%). The mapping from words to p-values is a convention, not a mathematical fact.

**Decision:** Define ranges rather than exact values:

| Quantifier | p Range |
|---|---|
| "all" / "every" | p = 1 |
| "almost all" | p ∈ [0.9, 1) |
| "most" | p ∈ (0.5, 0.9) |
| "about half" | p ∈ [0.4, 0.6] |
| "some" / "several" | p ∈ (0.1, 0.4) |
| "few" | p ∈ (0, 0.1) |
| "a" / "some (existential)" | p → 0⁺ |

These ranges are conventional, not algebraic. Different contexts may calibrate differently (in a room of 5 people, "most" = p ≈ 0.6 means 3 people; in a nation of millions, "most" ≈ same proportion but different absolute count).

---

## Impact Assessment on D2 Cases

| Case | Current Verdict | After Phase 1B | Reason |
|---|---|---|---|
| 3.2 "Most dogs chase some cat" | ❌ | ✅ | "most" expressible as p ∈ (0.5, 0.9) |
| 3.3 "Exactly three students passed" | ⚠️ | ⚠️ | Proportional quantification helps ("a few") but exact cardinality still requires enumeration |

**Net conversions:** 1 case ❌→✅ (3.2), 0 change on 3.3.

**Revised total with Phase 1A:** 3.2 moves from ❌ to ✅, reducing ❌ count from 13 to 12 and increasing ✅ from ~19 (after 1A) to ~20.

---

## Files to Modify

| File | Change |
|---|---|
| `foundations/formal-operations.md` | Update §1.11 — extend quantify definition with continuous parameter; add area-proportion formula |
| `foundations/formal-foundations.md` | Update §1.5 — note that quantify accepts a continuous parameter p ∈ [0,1] |
| `ul-core/grammar/formal-grammar.md` | Update C11 construction rule — graduated quantification visual conventions |
| `ul-core/writing-system/writers-companion.md` | Add example showing "most" quantification |
| `experiments/D2-completeness-challenge.md` | Re-score Case 3.2 as ✅; update summary |
| `ul-core/CRITIQUE.md` | Update to reflect extended quantification |

---

## Open Questions

1. **Area vs. linear measure:** Should p be based on Area(σ_p(C))/Area(F) or on some linear measure (diameter ratio, boundary-length ratio)? Area is most intuitive for "how much of the frame is occupied."
2. **Zero-area entities:** If the base entity is a point (Area = 0), the area-ratio formula degenerates. Convention: for point entities, p controls the radius of an enclosing disk around the point. p = 1 means the disk fills the frame.
3. **Interaction with bind (Phase 2):** When variable binding is introduced, does the frame-fill proportion interact with scope? Likely yes: the proportion is relative to the frame in which the binding occurs, not the outermost frame.
