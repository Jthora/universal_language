# Proof Inventory — Every Claim, Its Status, Its Location

**Parent:** [Pass 1 README](README.md)  
**Purpose:** Single source of truth for every theorem-like statement in the project.  
**Convention:** Status labels follow the three-tier system established in Phase 0.

---

## Tier Definitions

| Tier | Label | Meaning |
|------|-------|---------|
| **PROVEN** | ✅ | Complete rigorous proof with no logical gaps. Every step follows from axioms, definitions, or previously proven results. |
| **CONDITIONAL** | ⚠️ | Proof is complete IF explicitly stated assumptions hold. The assumptions are clearly labeled and their status is documented. |
| **OPEN** | ❌ | No proof exists. May have a roadmap, evidence, or plausibility argument, but the claim is not established. |

---

## Foundations

### formal-foundations.md

| § | Claim | Current Label | Actual Status | Notes |
|---|-------|--------------|---------------|-------|
| 1.1–1.4 | Language = Σ-homomorphism (E, M, ⟦·⟧) | Definition | ✅ Definition | Standard universal algebra. Not original — correctly applied. |
| 1.5 | Σ_UL is the universal linguistic signature (4 sorts, 11 ops) | PROVEN (minimality) | ⚠️ CONDITIONAL | Sort-necessity arguments are obvious. Operation-necessity needs formal independence proof (see Phase 2.2). |
| 2.1 | G is a Σ_UL-algebra (carrier sets, 11 operations defined) | PROVEN | ⚠️ CONDITIONAL | Geometric operations well-defined. Semantic interpretation of operations (reflection=negation, etc.) is CONVENTIONAL, not forced. |
| 2.2 | Closure, totality, determinism of operations | PROVEN | ✅ PROVEN | Verified rigorously in formal-operations.md. |
| 3.1–3.2 | Embedding theorem setup | — | ✅ Setup | Definitions and construction steps. |
| 3.3 | Geometric Universality Theorem (embedding) | PROVEN | ⚠️ CONDITIONAL | Step 4 (injectivity) has "visual distinctness → set-theoretic distinctness" gap. Fix: cite Operation Distinctness from formal-operations.md. Embedding is ADEQUACY, not UNIVERSALITY — any large enough algebra would work. |
| 3.4 | Embedding is injective | PROVEN | ⚠️ CONDITIONAL | Depends on Gap A (visual distinctness). Fixable via formal-operations.md. |
| 4.1–4.2 | 5 geometric role properties defined | — | Definition | These are the load-bearing assumptions. Not theorems. |
| 4.3–4.4 | 5 semantic role properties defined | — | Definition | Crafted to match geometric properties. The question is whether this match is discovered or imposed. |
| 4.5 | Unique Grounding Theorem | PROVEN | ⚠️ CONDITIONAL | IF the role-property characterizations are accepted, the exhaustive elimination is correct. But the definitions are designed to force the match. See Phase 2.4. |
| 5.1(1) | Σ_UL is minimal | PROVEN | ⚠️ CONDITIONAL | Sort minimality is clear. Operation minimality needs algebraic independence proof. |
| 5.1(2) | G admits embedding of any language | PROVEN | ⚠️ CONDITIONAL | Same as §3.3. Rename to Adequacy. |
| 5.1(3) | G can express its own structure | PROVEN | ❌ OPEN | Hand-waving. No encoding constructed, no fixed-point theorem. See Phase 2.5. |

### formal-operations.md

| § | Claim | Current Label | Actual Status | Notes |
|---|-------|--------------|---------------|-------|
| 1.1–1.11 | 11 operations: set-theoretic definitions | Definition | ✅ Definition | Precise, rigorous. The strongest document in the project. |
| 2 | Operation Distinctness (each preserves input distinctness) | PROVEN | ✅ PROVEN | Invertibility + structural recovery analysis. No gaps. |
| 2 | Closure, totality, determinism | PROVEN | ✅ PROVEN | Verified per-operation. |
| 3 | 11-operation completeness | Coverage argument | ❌ OPEN | Document explicitly states "not a formal completeness proof" (§3.7). Requires: semantic domain axiomatization, decomposition proof, transitivity proof. |

### independent-derivation.md

| § | Claim | Current Label | Actual Status | Notes |
|---|-------|--------------|---------------|-------|
| 2.1–2.6 | 5 traditions converge on 4-sort structure | Evidence | ✅ Genuine evidence | Not a theorem — an empirical observation about independent intellectual traditions. Strongest non-formal result. |
| 3.1–3.4 | Geometric correspondence (same 4 sorts) | Established | ⚠️ CONDITIONAL | The correspondence is real. Whether it's discovered or imposed is the open question. |
| 3.5 | Honest caveat: doesn't prove geometry is unique | — | ✅ Honest | The document itself says this. Preserve. |
| 4.1 | Entity is irreducible | PROVEN | ✅ PROVEN | Clean proof by contradiction. |
| 4.2 | Relation is irreducible | PROVEN | ✅ PROVEN | Clean proof by contradiction. |
| 4.3 | Modifier is irreducible | PROVEN | ✅ PROVEN | Clean proof by contradiction. |
| 4.4 | Assertion is irreducible | PROVEN | ✅ PROVEN | Clean proof by contradiction. |
| 4.5 | 4 sorts are sufficient | Coverage argument | ❌ OPEN | No proof that a 5th sort is impossible. |

### paradigm.md

| § | Claim | Current Label | Actual Status | Notes |
|---|-------|--------------|---------------|-------|
| — | UL is not a model of human language | Framing | ✅ Legitimate framing | Core insight worth keeping. |
| — | Human languages are "distorted, incomplete instantiations" | Framing | ❌ UNFALSIFIABLE | Should be: "Natural languages are valid Σ_UL-algebras with additional historical/social structure — not failed attempts at UL, but specializations of it." |
| — | Validating against natural languages is a "category error" | Framing | ⚠️ Overstated | Should be: "not the primary validation target" rather than "invalid in principle." |

### universal-language-derivation.md

| § | Claim | Current Label | Actual Status | Notes |
|---|-------|--------------|---------------|-------|
| Parts I–IV | 5 primitives (Point, Line, Angle, Curve, Enclosure) | Derivation | ✅ Definition | The geometric objects are well-defined. |
| Part V | Erlangen Program as linguistic architecture | Established | ⚠️ CONDITIONAL | The hierarchy is mathematically correct. Its interpretation as linguistic levels is a modeling choice. |
| Part VI | Links to formal-foundations.md proofs | Reference | — | Pointers, not proofs. |
| Various | ~27 theorem-like statements | Mixed | Mostly ⚠️/❌ | Need individual tier labels. See Phase 0.7. |

---

## Frontier — Expedition One

### category-of-languages.md

| Claim | Status | Notes |
|-------|--------|-------|
| Lang(Σ_UL) is a category | ✅ PROVEN | Trivial category axiom verification. |
| Interpretation is functorial | ✅ PROVEN | Compositionality = functoriality. |
| Erlangen levels as forgetful functors | ✅ PROVEN | Quotient map composition. |
| Left adjoints F₁⊣U₁, F₂⊣U₂ | ✅ PROVEN | Explicit carrier construction. |
| Left adjoints F₃⊣U₃, F₄⊣U₄ | ❌ OPEN | AFT hypotheses not verified. |
| Weak terminality of G | ✅ PROVEN | Up to automorphisms. |

### numbers-and-computability.md

| Claim | Status | Notes |
|-------|--------|-------|
| ℕ constructible as Σ_UL terms | ✅ PROVEN | n-fold translation of unit segment. |
| ℤ constructible | ✅ PROVEN | Invert direction. |
| ℚ constructible | ⚠️ CONDITIONAL | Thales construction correct but not reduced to Σ_UL operations. |
| Robinson Q axioms verified | ⚠️ CONDITIONAL | Document claims verification; computations for Q5, Q7 not shown in cited section. foundation-securing.md may contain them. |
| Gödel incompleteness follows | ⚠️ CONDITIONAL | Standard result IF Robinson Q is verified (depends on above). |
| Parsing O(n log n) | ✅ PROVEN | For unambiguous expressions. |
| Ambiguous parsing decidability | ❌ OPEN | Explicitly identified as open problem. |

### gauge-bundle-of-meaning.md

| Claim | Status | Notes |
|-------|--------|-------|
| Path-connectivity of context space X | ✅ PROVEN | Manifold product topology. |
| Meaning bundle as fiber bundle | ⚠️ FRAMEWORK | Definitions given; not all properties proven. |
| Connection components | ⚠️ FRAMEWORK | Definitions given; explicit computation deferred to Expedition Two. |

---

## Frontier — Expedition Two

### foundation-securing.md

| Claim | Status | Notes |
|-------|--------|-------|
| DC_UL multiplicatively invariant (Theorems 1–5) | ✅ PROVEN | Important correction: additive invariance does NOT hold. |
| F₁⊣U₁, F₂⊣U₂ left adjoints (Theorems 6–7) | ✅ PROVEN | Explicit carrier construction. |
| BANK polysemy toy model | ⚠️ FRAMEWORK | Illustrative, not a general proof. |

### metric-and-grounding.md

| Claim | Status | Notes |
|-------|--------|-------|
| Prim Hom-sets: column profiles pairwise distinct (Theorem 8) | ✅ PROVEN | 5×5 matrix computed. |
| Yoneda ⇔ Unique Grounding on Prim (Theorem 9) | ✅ PROVEN | Tight result. |
| Fisher metric on X positive semi-definite (Theorems 10–11) | ✅ PROVEN | Full positive-definiteness deferred. |

### probability-and-information.md

| Claim | Status | Notes |
|-------|--------|-------|
| Prefix-free encoding of Σ_UL constructions (Theorem 1) | ✅ PROVEN | |
| Structural prior is well-defined semimeasure (Theorem 2) | ✅ PROVEN | |
| Normalizable to proper probability (Theorem 3) | ✅ PROVEN | |
| Stochastic extension D3 | ⚠️ FRAMEWORK | Random variables over meaning space. |

### metaphor-and-projection.md

| Claim | Status | Notes |
|-------|--------|-------|
| μ: T → Γ(E) is section on meaning bundle (Theorem 1) | ✅ PROVEN | Under Hypothesis S. |
| Polysemy ⇔ non-trivial holonomy (Theorems 2–3) | ✅ PROVEN | Under Hypothesis S + expressiveness. |

### metaphor-formalization.md

| Claim | Status | Notes |
|-------|--------|-------|
| G_QM, G_ling, G_cog are valid Σ_UL-subalgebras (Theorem 12) | ✅ PROVEN | 13, 11, 13 generators. |
| G_QM ∩ G_cog ⊇ 3 independent elements (Theorem 13) | ✅ PROVEN | |
| φ_cross is Σ_UL-homomorphism (Theorem 14) | ✅ PROVEN | |
| Connection component A₂, curvature F₁₂ (Theorem 15) | ✅ PROVEN | On extended toy model. |

---

## Summary Counts

| Tier | Count | Percentage |
|------|-------|------------|
| ✅ PROVEN | ~45 core theorems | ~52% |
| ⚠️ CONDITIONAL / FRAMEWORK | ~18 | ~21% |
| ❌ OPEN | ~23 | ~27% |

**Note:** These counts cover distinctly stated theorems/propositions across all documents. The ~105 total from the raw audit includes redundant restatements and definitions.
