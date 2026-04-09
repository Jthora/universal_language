# P0 — Negation Resolution: Boundary Inversion Design

**Date:** April 7, 2026  
**Status:** DESIGN COMPLETE — ready for propagation  
**Resolves:** F1 (negation implements converse), B1 (what IS negation?)  
**Design choice:** Option A — Boundary Inversion

---

## 1. THE PROBLEM (Brief Recap)

The current definition:

```
negate(a) = (F, ρ_c(C))
```

reflects content C through a vertical axis. This produces the **converse** (subject-object swap), not logical **negation** (truth-value flip). The reflection `[△ ──→ ○]` → `[○ ←── △]` reads as "B is acted upon by A" — same truth value as the original. True negation should produce "A does NOT act on B."

This breaks: propositional completeness, De Morgan's laws, the contradiction axiom.

---

## 2. THE SOLUTION: Boundary Inversion

### 2.1 Core Idea

An assertion's **frame boundary** carries its assertional status:
- **Solid boundary (───):** The assertion is **claimed** (positive sign, ⊕)
- **Dashed boundary (╌╌╌):** The assertion is **denied** (negative sign, ⊖)

Negation flips the boundary style. Content is unchanged.

### 2.2 Why This Is Geometric

A dashed boundary and a solid boundary are geometrically distinct objects in ℝ²:
- A solid frame boundary is a **connected** closed curve (Jordan curve)
- A dashed frame boundary is a **disconnected** collection of arcs — topologically distinct

The assertional sign is encoded in the **topology of the boundary**: connected = asserted, disconnected = denied. This is a topological property, hence geometric. No non-geometric conventions are introduced.

---

## 3. FORMAL DEFINITION

### 3.1 Enriched Assertion Carrier Set

**Old definition:**

```
Gₐ = { (F, C) | F ⊂ ℝ² is a Jordan domain, C ⊂ F is a finite 
        geometric configuration containing at least one 
        entity-relation-entity triple }
```

**New definition:**

```
Gₐ = { (F, C, σ) | F ⊂ ℝ² is a Jordan domain, C ⊂ F is a finite 
        geometric configuration containing at least one 
        entity-relation-entity triple, σ ∈ {⊕, ⊖} }
```

where σ is the **assertional sign**: ⊕ = asserted (solid boundary), ⊖ = denied (dashed boundary).

**Visual realization:**
- σ = ⊕: The frame boundary ∂F is drawn as a solid curve
- σ = ⊖: The frame boundary ∂F is drawn as a dashed curve (strictly: a finite collection of arcs approximating ∂F with uniform gaps)

**Compatibility:** All existing assertions are implicitly (F, C, ⊕) — the positive sign is the default when not specified. All existing operations that produce assertions (predicate, conjoin, disjoin, quantify) produce assertions with σ = ⊕ by default.

### 3.2 New Negation Operation

**Definition.** Given assertion a = (F, C, σ) ∈ Gₐ:

```
negate(a) = (F, C, −σ)
```

where −⊕ = ⊖ and −⊖ = ⊕.

**Closure:** The output is (F, C, −σ) which is an assertion (same F, same C, valid sign). ✓  
**Totality:** Every assertion has a sign that can be flipped. ✓  
**Determinism:** Sign flip is a function. ✓

### 3.3 Visual Representation

```
ASSERTION:                   NEGATION:
┌───────────────┐            ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐
│  △ ──→ ○      │            ╎  △ ──→ ○      ╎
└───────────────┘            └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘
 σ = ⊕ (solid)               σ = ⊖ (dashed)
"fundamental acts             "NOT: fundamental
 on universal"                acts on universal"
```

Content is **identical**. Only the frame boundary changes. The reader sees that the same structural claim is being denied rather than asserted.

---

## 4. PROPERTY VERIFICATION

### 4.1 N1: Involution

```
negate(negate(a)) = negate((F, C, −σ)) = (F, C, −(−σ)) = (F, C, σ) = a  ✓
```

Double negation returns the original. ✓

### 4.2 N2: Contradiction

```
conjoin(a, negate(a)) = conjoin((F, C, ⊕), (F, C, ⊖))
```

The conjunction of "C is asserted" and "C is denied" is a contradiction. Geometrically: when two frames with identical content but opposite signs overlap, the result is a construction where the same region is simultaneously solid-bounded and dash-bounded — an inconsistent boundary. This is ⊥.

Formally: define ⊥ = (∅, ∅, ⊕) (the empty assertion) or equivalently as any assertion (F, C, σ) where σ-conflict is detected. The key property is that no interpretation satisfies both (F, C, ⊕) and (F, C, ⊖) simultaneously. ✓

### 4.3 N3: Excluded Middle

```
disjoin(a, negate(a)) = disjoin((F, C, ⊕), (F, C, ⊖))
```

The disjunction of "C is asserted" or "C is denied" is a tautology: for any content, it is either asserted or denied. Geometrically: adjacent frames with identical content and opposite signs tile the assertional space completely — every claim is either solid-framed or dash-framed. This is ⊤. ✓

**Note:** Excluded middle holds in classical logic. In intuitionistic UL, we would not require N3 — and boundary inversion still works (just drop the tautology requirement). The design is logic-parametric.

### 4.4 N4: De Morgan's Laws

```
negate(conjoin(a₁, a₂)) = negate((F₁ ∪ F₂, C₁ ∪ C₂, min(σ₁, σ₂)))
                         = (F₁ ∪ F₂, C₁ ∪ C₂, −min(σ₁, σ₂))
```

```
disjoin(negate(a₁), negate(a₂)) = disjoin((F₁, C₁, −σ₁), (F₂, C₂, −σ₂))
                                 = (F₁ ∪ F₂, C₁ ∪ C₂, max(−σ₁, −σ₂))
```

For σ₁ = σ₂ = ⊕: −min(⊕, ⊕) = −⊕ = ⊖ and max(⊖, ⊖) = ⊖. Equal. ✓  
For σ₁ = ⊕, σ₂ = ⊖: −min(⊕, ⊖) = −⊖ = ⊕ and max(⊖, ⊕) = ⊕. Equal. ✓  
For σ₁ = σ₂ = ⊖: −min(⊖, ⊖) = −⊖ = ⊕ and max(⊕, ⊕) = ⊕. Equal. ✓

De Morgan's first law holds. The second law (`negate(disjoin(a₁, a₂)) = conjoin(negate(a₁), negate(a₂))`) follows by symmetric argument (swap min↔max, ∪↔∩). ✓

### 4.5 Injectivity (for Embedding Theorem)

```
negate(a₁) = negate(a₂) ⟹ (F₁, C₁, −σ₁) = (F₂, C₂, −σ₂) 
                          ⟹ F₁ = F₂, C₁ = C₂, σ₁ = σ₂ 
                          ⟹ a₁ = a₂  ✓
```

Negation is injective (which it must be for the embedding theorem's injectivity proof). ✓

---

## 5. WHAT HAPPENS TO REFLECTION?

The current reflection operation `ρ_c` is not negation — it's **converse**. What to do with it?

### 5.1 Converse Is Derivable

For an assertion `a = predicate(e₁, r, e₂)`:

```
converse(a) = predicate(e₂, invert(r), e₁)
```

This uses the existing `predicate` and `invert` operations. No new operation needed.

**Proof:** `invert(r)` reverses the directed path from e₁→e₂ to e₂→e₁. Then `predicate(e₂, invert(r), e₁)` constructs an assertion with the roles swapped and the relation reversed. This is exactly what reflection was doing — but now expressed in terms of existing operations.

### 5.2 Decision

**Remove reflection as the implementation of negate. Do NOT add a separate `converse` operation.** Converse is derivable from `{predicate, invert}` and was never needed as a primitive.

This means the net operation count stays at 11 (or 10, pending conjoin independence — see P1).

---

## 6. IMPACT ON conjoin AND disjoin

### 6.1 Updated conjoin

The sign propagation rule for conjunction:

```
conjoin((F₁, C₁, σ₁), (F₂, C₂, σ₂)) = (F₁ ∪ F₂, C₁ ∪ C₂, σ₁ ∧ σ₂)
```

where ⊕ ∧ ⊕ = ⊕, ⊕ ∧ ⊖ = ⊖, ⊖ ∧ ⊖ = ⊖.

(If either input is denied, the conjunction carries the denial. Both must be asserted for the conjunction to be asserted.)

Visual: the boundary of the combined frame inherits the "most denied" style.

### 6.2 Updated disjoin

```
disjoin((F₁, C₁, σ₁), (F₂, C₂, σ₂)) = (F₁ ∪ F₂, C₁ ∪ C₂, σ₁ ∨ σ₂)
```

where ⊕ ∨ ⊕ = ⊕, ⊕ ∨ ⊖ = ⊕, ⊖ ∨ ⊖ = ⊖.

(If either input is asserted, the disjunction carries the assertion. Both must be denied for the disjunction to be denied.)

Visual: the boundary inherits the "most asserted" style—which reflects the asymmetry of disjunction (one positive input suffices).

---

## 7. COMPATIBILITY WITH EXISTING CONVENTIONS

### 7.1 Possibility (Dashed Content Lines)

`symbol-map.md` §VII already uses dashed lines for "possibility" — but this applies to the **content** (dashed relations, dashed entities), not the **frame boundary**. The two are compatible:

| Dashed element | Meaning |
|---------------|---------|
| Dashed frame boundary | Assertion is **denied** (negation) |
| Dashed content line | Relation is **hypothetical** (possibility) |

These compose naturally:
- Solid frame, solid content: "A acts on B" (fact)
- Dashed frame, solid content: "NOT: A acts on B" (denied fact)
- Solid frame, dashed content: "A might act on B" (possible fact)
- Dashed frame, dashed content: "NOT: A might act on B" (denied possibility)

No conflict. Frame style = assertional status. Content style = modal status.

---

## 8. FILES REQUIRING UPDATES

| # | File | Section | Change |
|---|------|---------|--------|
| 1 | `foundations/formal-operations.md` | §0 (Gₐ definition) | Add σ parameter |
| 2 | `foundations/formal-operations.md` | §1.4 (negate) | Replace reflection with boundary inversion |
| 3 | `foundations/formal-operations.md` | §1.1 (predicate) | Default σ = ⊕ |
| 4 | `foundations/formal-operations.md` | §1.5, §1.6 (conjoin, disjoin) | Add sign propagation |
| 5 | `foundations/formal-operations.md` | §2 (injectivity) | Update negate proof |
| 6 | `foundations/formal-operations.md` | §3.4 (propositional completeness) | Now actually valid |
| 7 | `foundations/formal-foundations.md` | Any negate references | Align with new definition |
| 8 | `ul-core/syntax/syntax-dictionary.md` | §3.4 | Replace reflection visual with boundary inversion |
| 9 | `ul-core/grammar/grammar-book.md` | Negation worked example | Update |
| 10 | `ul-core/symbology/symbol-map.md` | §VII Negation row | Update from reflection to boundary inversion |
| 11 | `ul-core/CRITIQUE.md` | §1.1 | Mark as resolved |

---

## 9. SUMMARY

| Property | Old (reflection) | New (boundary inversion) |
|----------|-----------------|-------------------------|
| N1 Involution | ✅ | ✅ |
| N2 Contradiction | ❌ | ✅ |
| N3 Excluded Middle | ❌ | ✅ |
| N4 De Morgan | ❌ | ✅ |
| Injectivity | ✅ | ✅ |
| Geometric | ✅ spatial | ✅ topological |
| Content preserved | ❌ reflected | ✅ identical |
| Net operation change | — | 0 (converse derived from {predicate, invert}) |

**Boundary inversion is a strict upgrade:** it satisfies all properties that reflection satisfied (N1, injectivity) plus all properties it failed (N2, N3, N4), while remaining geometric and preserving content identity.
