# Pass 1.2 — Expressiveness Extensions

**Date:** April 7, 2026  
**Status:** ✅ COMPLETE (April 7, 2026)  
**Scope:** Close the within-scope gaps identified by the D2 completeness challenge (50-case boundary test from Pass 1.1)  
**Predecessor:** [Pass 1.1](../pass1-1/README.md) (Deep Theory Audit — all items ✅ RESOLVED)  
**Trigger:** D2-completeness-challenge.md scored 16✅ / 21⚠️ / 13❌. The 13 failures are in declared-out-of-scope categories (modality, performatives, pragmatics). The 21 partial cases concentrate in 3 addressable gaps — all within declared scope.  
**Result:** D2 score moved from 32% → 52% clean (26✅ / 11⚠️ / 13❌). Σ_UL extended to Σ_UL⁺ with 13 operations. Extensional Montague subsumption proven.

---

## How Pass 1.2 Differs from Pass 1.1

Pass 1.1 asked: **"Are the claims correct?"** — and found negation was wrong, independence was overstated, modifiers were under-specified, and the scope boundary was fuzzy.

Pass 1.2 asks: **"Can UL express what it claims to express?"** — and extends the algebra to cover the gaps that the D2 completeness challenge exposed within UL's declared scope.

**Pass 1.1 was corrective** (fix errors). **Pass 1.2 is constructive** (add missing pieces).

---

## What D2 Found (Summary)

| Category | ✅ | ⚠️ | ❌ | Key Issue |
|----------|---|---|---|-----------|
| 1. Simple compositional | 3 | 2 | 0 | Binary predicate forces ditransitive decomposition |
| 2. Deep nesting | 5 | 0 | 0 | Clean — embed handles recursion |
| 3. Quantifier scope | 0 | 5 | 0 | **No variable binding; no graduated quantifiers** |
| 4. Modality | 0 | 0 | 5 | Out of scope (acknowledged) |
| 5. Tense/aspect | 0 | 5 | 0 | **Modifier conventions undefined** |
| 6. Irony/pragmatics | 0 | 2 | 3 | Out of scope (acknowledged) |
| 7. Metaphor | 5 | 0 | 0 | UL's strongest category |
| 8. Self-reference | 1 | 4 | 0 | Correctly Gödel-limited |
| 9. Performatives | 0 | 0 | 5 | Out of scope (acknowledged) |
| 10. Cross-linguistic | 2 | 3 | 0 | **Evidentiality needs assertion-level modification** |
| **Total** | **16** | **21** | **13** | |

### Three Addressable Gaps (Within Scope)

1. **Variable binding + scope ordering** — Cases 3.1, 3.4, 3.5 (and downstream effects on 1.5, 10.1). `quantify(m, e) → a` is monadic; polyadic quantification requires formal variable binding.
2. **Graduated quantification** — Cases 3.2, 3.3. Binary ∀/∃ is too coarse for "most," "few," "many."
3. **Assertion-level modification** — Cases 5.4, 10.3, 6.5. The only a → a operation is `negate`. Evidentiality, speaker stance, and epistemic hedging need `modify_assertion(m, a) → a`.

### One Convention Gap

4. **Modifier convention table** — Cases 5.1–5.3, 5.5, 10.1, 10.4. The carrier set Gₘ contains tense/aspect transformations, but no formal specification says WHICH element IS the past tense.

### One Theoretical Question

5. **Montague homomorphism** — Does UL subsume, equal, or fall short of Montague's extensional semantics? The answer determines whether UL is a discovery (new mathematical structure) or a notation (geometric re-encoding of known formal semantics).

---

## Phase Structure

| Phase | Name | Type | New Ops | Cases Fixed | Risk | Depends On |
|-------|------|------|---------|-------------|------|------------|
| [1A](phase-1a-modifier-conventions.md) | Modifier Conventions | Convention table | 0 | ~7 ⚠️→✅ | None | Nothing |
| [1B](phase-1b-graduated-quantification.md) | Graduated Quantification | Parameter extension | 0 | 2 | Low | Nothing |
| [2](phase-2-variable-binding.md) | Variable Binding | New operation | +1 (`bind`) | 3–5 | Medium | Nothing (but benefits from 1A/1B context) |
| [3](phase-3-assertion-modification.md) | Assertion-Level Modification | New operation | +1 (`modify_assertion`) | 3 | Low | Nothing |
| [4](phase-4-montague-homomorphism.md) | Montague Homomorphism | Theoretical proof | 0 | 0 (credibility) | High | 1A + 1B + 2 + 3 |

**Projected final score:** ~31✅ / 6⚠️ / 13❌ (62% clean, failures only in declared-out-of-scope categories)

**Projected final operation count:** 13 operations (12 independent + 1 derived `conjoin`)

---

## Dependency Graph

```
Phase 1A: Modifier Conventions ─────────────┐
                                             │
Phase 1B: Graduated Quantification ──────────┤
                                             ├──→ Phase 4: Montague Homomorphism
Phase 2:  Variable Binding ──────────────────┤
                                             │
Phase 3:  Assertion-Level Modification ──────┘
```

Phases 1A and 1B are independent of each other and can be done in parallel.  
Phases 2 and 3 are independent of each other but both benefit from 1A/1B being done first.  
Phase 4 must be last — it maps the **final** Σ_UL.

---

## Ordering Rationale

### Phase 1A First: Quick Win, Highest Case Conversion

- Zero algebraic risk (no new operations; naming elements already in Gₘ)
- Highest case-conversion per unit effort (~7 cases)
- Most directly useful for anyone learning UL
- Establishes tense/aspect conventions needed by later phases as examples

### Phase 1B Second: Small Extension, Completes Quantification

- One parameter addition (scale proportion p ∈ [0,1])
- Only 2 cases fixed, but closes the "most/few/many" gap entirely
- Low risk: geometrically natural, injectivity preserved

### Phase 2 Third: Hardest but Most Important

- This is the **only gap within declared scope** causing systematic failures
- Requires a genuine design decision (labeled slots vs. lambda-style vs. scope frames)
- Doing easy wins first builds momentum and clarifies the problem space
- 3–5 cases fixed, plus structural credibility for the completeness claim

### Phase 3 Fourth: Clean Extension

- Straightforward pattern-follow from existing operations
- Separate from negate (preserves negate's involution)
- 3 cases fixed

### Phase 4 Last: Theoretical Capstone

- Must map the **final** Σ_UL (all previous phases must settle the operation set)
- Highest-risk item — outcome is genuinely unknown
- Resolves the deepest credibility question (discovery vs. notation)

---

## Relationship to Pass 1.1 Items

| Pass 1.1 Item | Pass 1.2 Connection |
|---|---|
| P0 (Negation) ✅ | Phase 3 extends the assertional framework P0 established |
| P1 (Independence) ✅ | Phases 2 and 3 add operations → need new independence proofs |
| F3 (Modifier carrier gap) ✅ | Phase 1A directly operationalizes the F3 resolution |
| D2 (Falsifiable predictions) ✅ | The D2 challenge IS the trigger for Pass 1.2 |
| B2 (Modifier sort) ✅ | Phase 1A makes modifier semantics concrete |

---

## Success Criteria

Pass 1.2 is complete when:

1. ✅ Modifier convention table exists with geometric justification for each assignment
2. ✅ `quantify` handles proportional quantifiers (p ∈ [0,1])
3. ✅ Formal variable-binding mechanism exists with closure/totality/determinism/independence proofs
4. ✅ `modify_assertion` exists with closure/totality/determinism proof
5. ✅ D2-completeness-challenge.md is re-scored with updated verdicts
6. ✅ Montague extensional fragment is formally mapped (or failure points identified)
7. ✅ CRITIQUE.md is updated to reflect extensions
8. ✅ Operation count, sort system, formal-foundations.md, formal-operations.md all updated

---

## File Index

```
pass1-2/
├── README.md                                    ← You are here
├── phase-1a-modifier-conventions.md             ← Convention table design
├── phase-1b-graduated-quantification.md         ← Proportional quantifier extension
├── phase-2-variable-binding.md                  ← New bind operation design
├── phase-3-assertion-modification.md            ← New modify_assertion operation design
└── phase-4-montague-homomorphism.md             ← Extensional fragment mapping
```
