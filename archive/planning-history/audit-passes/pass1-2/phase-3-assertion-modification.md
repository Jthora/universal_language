# Phase 3 — Assertion-Level Modification

**Status:** ✅ COMPLETE (April 7, 2026)  
**Risk:** Low  
**New operations:** +1 (`modify_assertion`)  
**D2 cases affected:** 5.4, 10.3, 6.5  
**Expected case conversions:** 3 (⚠️ → ✅)  
**Dependencies:** None  
**Blocks:** Phase 4 (Montague mapping needs complete operation set)

---

## Problem Statement

The D2 completeness challenge found that **evidentiality, speaker stance, and epistemic hedging** are not expressible as direct operations. The diagnosis from Cases 5.4 ("Apparently she left early") and 10.3 (Turkish evidential _-miş_):

> "Evidentiality AS AN ASSERTION-LEVEL MODIFIER is not directly available. The algebra modifies entities and relations, not assertions. Negate is the only a → a operation, and it only flips the sign."

### The Gap in the Sort-Transition Table

The current sort-transition coverage:

| Target Sort | Operations | Input Combinations |
|---|---|---|
| e (Entity) | modify_entity, embed | m × e → e, a → e |
| r (Relation) | modify_relation, compose, invert | m × r → r, r × r → r, r → r |
| m (Modifier) | abstract | e → m |
| **a (Assertion)** | **predicate, negate, conjoin, disjoin, quantify, bind** | **e × r × e → a, a → a, a × a → a, m × e → a, e × a → a** |

For the Assertion sort: `negate` is the only **unary** a → a operation, and it only flips the assertional sign σ. There is no mechanism to apply a general modifier to an assertion — to transform the FRAME without changing the CONTENT or the SIGN.

### What's Missing

Natural languages systematically modify assertions at the frame level:

| Phenomenon | Example | What it modifies | Current UL Status |
|---|---|---|---|
| Evidentiality | "Apparently she left" | Speaker's epistemic access to the assertion | Requires restructuring (embed + meta-predicate) |
| Hedging | "She sort of left" | Degree of commitment to the assertion | No mechanism |
| Emphasis | "She DEFINITELY left" | Strength of commitment | No mechanism |
| Evidential source | "I heard she left" | Information channel | Requires restructuring |
| Speaker attitude | "Unfortunately she left" | Speaker's evaluation of the assertion | No mechanism |

All of these modify the assertion AS A WHOLE — they don't change what's predicated (content C) or whether it's asserted/denied (sign σ). They change the **manner** or **status** of the assertion.

---

## Proposed Solution: modify_assertion Operation

### New Operation

```
modify_assertion : m × a → a
```

**Semantics:** Apply a modifier (geometric transformation) to the FRAME of an assertion, without changing the content or the assertional sign.

### Formal Definition

Given modifier m ∈ Gₘ (transformation T) and assertion a = (F, C, σ) ∈ Gₐ:

```
modify_assertion(m, a) = (T(F), C, σ)
```

where:
- T(F) is the image of the frame boundary under T
- C (the content) is **unchanged** — it remains inside T(F) by appropriate repositioning
- σ (the assertional sign) is **unchanged**

**What this does:** The transformation acts on the frame's SHAPE, not its content or truth status. Different frame shapes encode different assertion-level properties.

### Frame-Shape Convention Table

| Frame Shape | Gₘ Element | Meaning | Example |
|---|---|---|---|
| Solid boundary `───` | Identity I | Asserted (default σ = ⊕) | "She left." |
| Dashed boundary `- - -` | Existing: negate flips σ | Denied (σ = ⊖) | "She didn't leave." |
| Dotted boundary `···` | m_uncertain: boundary fragmentation | Evidential/uncertain | "Apparently she left." |
| Double boundary `═══` | m_emphatic: boundary thickening | Emphasized/certain | "She definitely left." |
| Wavy boundary `~~~` | m_hedge: boundary oscillation | Hedged/approximate | "She sort of left." |
| Thin boundary | m_tentative: boundary thinning | Tentative/speculative | "She might have left." |

### Geometric Grounding

The frame boundary is a Jordan curve ∂F. Modifying the boundary changes its **visual character** without changing its topological type (it remains a simple closed curve separating interior from exterior):

- **Solid → Dotted:** The continuous curve becomes a sequence of points (sampling). This is a Homeo(ℝ²)-level transformation (topology changes — the boundary ceases to be connected).
  
  **Alternative geometric model:** Rather than breaking the curve, thicken it into a band and reduce opacity. The frame becomes a "fuzzy boundary" — geometrically, a narrow annular region with reduced visual weight. This stays within Sim(2) (scaling the boundary thickness).

- **Solid → Double:** Duplicate the boundary at a parallel offset (equidistant curve). This is Euc(2) (translation of the boundary by ε in the normal direction).

- **Solid → Wavy:** Apply a periodic perturbation to the boundary: ∂F → ∂F + A·sin(nθ)·n̂, where n̂ is the outward normal, A is amplitude, and n is frequency. This is a Homeo(ℝ²) transformation.

### Relationship to Negate

`negate` and `modify_assertion` are **orthogonal:**

| Operation | Changes σ? | Changes ∂F shape? | Changes C? |
|---|---|---|---|
| negate | YES (⊕ ↔ ⊖) | Incidentally (solid ↔ dashed as visual encoding of σ) | NO |
| modify_assertion | NO | YES | NO |

They compose freely:
```
modify_assertion(m_uncertain, negate(a))
```
= a with denied sign AND uncertain frame = "Apparently she didn't leave" (evidential + negation)

**Negate remains a separate operation** because:
1. negate is an involution (negate ∘ negate = id); modify_assertion is NOT generally an involution
2. negate flips a binary sign; modify_assertion applies an arbitrary transformation
3. The solid/dashed distinction for σ is a truth-functional property; frame shape is an assertion-metacommentary

---

## Verification Requirements

### 1. Closure
T(F) must be a Jordan domain. Is the image of a Jordan domain under T ∈ Gₘ still a Jordan domain?
- For T ∈ Euc(2), Sim(2), Aff(2): YES — these groups preserve Jordan curves.
- For T ∈ Proj(2): YES if T doesn't send ∂F through infinity (projective transformations can do this; restrict to the affine patch).
- For T ∈ Homeo(ℝ²): YES — homeomorphisms preserve Jordan curves (Jordan Curve Theorem is topological).
- **Edge case:** The dotted-boundary convention (fragmentation) technically breaks the Jordan property. Resolution: encode "dotted" not as disconnection but as a **texture** property (decoration on the boundary), like the assertional sign σ. This keeps the boundary topologically intact.

**Decision:** Frame modifications are DECORATIONS of the boundary (thickness, pattern, texture), not topological changes. The Jordan domain structure is preserved. T acts on the boundary's **visual properties**, not its topology.

### 2. Totality
Every modifier can be applied to every frame: T(F) is defined for all T ∈ Gₘ and all Jordan domains F. ✓

### 3. Determinism
T is a function; its action on ∂F is unique. ✓

### 4. Injectivity
If modify_assertion(m₁, a) = modify_assertion(m₂, a), then T₁(F) = T₂(F) with same C and σ. Since T₁ and T₂ act on the same F and produce the same result, and both are invertible, T₁ = T₂ (hence m₁ = m₂). ✓

### 5. Independence

`modify_assertion(m, a) → a` has signature m × a → a. No existing operation has this signature:
- `negate`: a → a (unary, no modifier input)
- `modify_entity`: m × e → e (acts on entities, not assertions)
- `modify_relation`: m × r → r (acts on relations, not assertions)

Furthermore, modify_assertion cannot be decomposed into existing operations because:
- embed(a) → e loses the frame to entity conversion; re-framing via predicate creates a NEW frame + content, not a frame modification
- modify_entity + predicate would modify the content entities, not the frame itself
- No sequence of existing operations transforms ∂F without touching C or σ

**Independence sketch:** Construct a Σ_UL-algebra where all frames have identical boundaries (e.g., all unit circles). All 12 existing operations (including bind) are interpretable in this algebra. But modify_assertion requires frames with different boundaries — it's not interpretable. Therefore modify_assertion is independent.

---

## Impact Assessment on D2 Cases

| Case | Current Verdict | After Phase 3 | Reason |
|---|---|---|---|
| 5.4 "Apparently she left early" | ⚠️ | ✅ | `modify_assertion(m_uncertain, pred(...))` — evidential as frame decoration |
| 10.3 Turkish evidential _-miş_ | ⚠️ | ✅ | Same mechanism — evidential morpheme maps to frame modification |
| 6.5 "He's not the sharpest tool" (litotes) | ⚠️ | ⚠️ | The propositional content stays the same; litotic understatement is still pragmatic. modify_assertion could mark the frame as "hedged/understated" but this doesn't capture that the INTENDED meaning is stronger than the stated meaning. |

**Revised:** 2 definitive ⚠️→✅ (5.4, 10.3). Case 6.5 might improve to ✅ if we accept that "understated frame" captures litotes' compositional aspect, but the pragmatic inference remains outside scope.

**Net conversions:** 2–3 cases ⚠️→✅.

---

## Files to Modify

| File | Change |
|---|---|
| `foundations/formal-foundations.md` | §1.5 — add `modify_assertion` operation; update operation count to 14 (12 independent + 1 derived) |
| `foundations/formal-operations.md` | New section — full definition with closure/totality/determinism/injectivity |
| `ul-core/grammar/formal-grammar.md` | Add C13 construction rule — assertion-level modification |
| `ul-core/writing-system/writers-companion.md` | Add examples showing frame shape conventions |
| `experiments/D2-completeness-challenge.md` | Re-score Cases 5.4 and 10.3; update summary |
| `ul-core/CRITIQUE.md` | Document the extension |
| `docs/planning/audits/improvements/pass1-1/tier-b-structural/P1-operation-independence.md` | Add modify_assertion independence proof |

---

## Enriched Assertional Framework

After Phases 0 (P0 negation) and 3, the assertion sort Gₐ has the following structure:

```
Gₐ = { (F, C, σ, δ) }
```

where:
- F ⊂ ℝ² — Jordan domain (the frame)
- C ⊂ F — content (entity-relation-entity configuration)
- σ ∈ {⊕, ⊖} — assertional sign (asserted/denied) — controlled by `negate`
- δ ∈ Decorations — frame decoration (visual properties of ∂F) — controlled by `modify_assertion`

Alternatively, δ can be absorbed into F itself (the frame IS its boundary shape, thickness, pattern). This avoids adding a 4th component by making frame shape part of the Jordan domain's specification.

**Decision for implementation:** Keep the (F, C, σ) triple but define `modify_assertion` as transforming F's boundary properties (thickness, pattern, texture). The σ-vs-δ distinction is: σ is truth-functional (matters for logical connectives), δ is presentational (doesn't affect conjunction/disjunction truth tables).

---

## Open Questions

### Q1: Does modify_assertion interact with conjoin/disjoin?

When conjoining two assertions with different frame decorations:
```
conjoin(modify_assertion(m_uncertain, a₁), modify_assertion(m_emphatic, a₂)) = ?
```

The frame decorations don't have obvious conjunction semantics. An uncertain claim AND an emphatic claim? 

**Proposal:** Frame decorations are **per-assertion** and don't compose under logical connectives. The conjunction inherits the frame shape of the compound frame (which is the union F₁ ∪ F₂), not of either constituent. The individual frame decorations are visible within the compound frame as sub-frames.

### Q2: Is modify_assertion the same as a modal operator?

Partially. It covers the EVIDENTIAL and EPISTEMIC dimensions of modality (speaker stance toward the assertion). It does NOT cover:
- Alethic modality (necessity/possibility as properties of the WORLD, not the speaker's stance)
- Deontic modality (obligation/permission)

These require reasoning about possible worlds, which modify_assertion doesn't provide. Modality as a scope boundary remains out of UL's scope.

### Q3: Should modify_assertion subsume negate?

Technically, negate = modify_assertion(m_flip_σ, a). But keeping them separate preserves:
- negate's clean involution: negate(negate(a)) = a
- negate's role in De Morgan derivations: conjoin(a,b) = negate(disjoin(negate(a), negate(b)))
- The truth-functional/presentational distinction

**Decision:** Keep separate. negate is a logical operation; modify_assertion is a pragmatic/evidential operation.
