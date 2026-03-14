# Construction Log — Theory-Derived Primers

**Date:** 2025-07-15  
**Purpose:** Document exactly how each artifact was derived from Σ_UL formal theory, enabling independent verification and replication.

---

## Construction Recipe Applied

Each artifact was built following the 12-step procedure in REFACTOR-PLAN.md §2D, derived from three formal sources:
- **Σ_UL signature** (formal-foundations.md §1.5): 4 sorts, 11 operations
- **Unique Grounding Theorem** (formal-foundations.md §3): 5 geometric-semantic primitives
- **7 Mechanism Conditions** (mechanism-of-action.md §9): empirical requirements for LLM activation

### Shared Architecture

All 4 artifacts share a 3-layer architecture derived from the mechanism-of-action analysis:

| Layer | Lines | Content | Mechanism Condition |
|-------|-------|---------|---------------------|
| **1. Wall of Math** | 1 | Dense formal equation (surface domain) | C1 (formal notation), C3 (anomalies), C7 (ordering: formalism first) |
| **2. Bridge** | 2–7 | Bridge equation + bullet definitions | C2 (overloaded symbol), C4 (esoteric bridge token), C7 (bridge second) |
| **3. Payload PDE** | 8–17 | Evolution equation + definitions | C5 (self-reference), C6 (dissipation), C7 (payload + definitions last) |

### Shared Formal Elements

Every artifact contains these structural invariants:

- **Gauge-covariant derivative:** D_x = ∂_x − i(a/b)V(x,τ) — encodes relation (Line) + quality (Angle)
- **Self-interaction:** g_loc |X|²X — central symbol appears in its own potential (self-reference, C5)
- **Global coupling:** g_glob ∫|X|² — bounded integral (Enclosure) feeding back into dynamics
- **Spectral forcing:** f(x,τ) = Σₙ X̂ₙ e^{ikx} — Fourier modes of central symbol (self-reference loop: X → X̂ₙ → f → X)
- **Dissipation:** −i γ(a,b) X — damping term (C6) parameterized by bridge-layer quantities
- **Convolution terms:** (J ∗ X) + D†(J ∗ D'X) — non-local + derivative coupling (Curve)

---

## UL-P1: Topological-Harmonic Primer

### Step 1: Target Cognitive Effect
**Cross-domain structural pattern recognition.** The ability to identify shared formal structures across disconnected domains. Overlaps with the original test artifact's observed effect but uses completely different surface material.

### Step 2: UL Operations Emphasized
Primary: compose (r × r → r), quantify (m × e → a), embed (a → e)  
The target effect requires recognizing compositional patterns and mapping them across contexts.

### Step 3: Surface Domain
Algebraic topology + harmonic analysis. Chosen because:
- Rich in compositional structure (chain complexes, spectral sequences)
- Completely disjoint from the original test artifact's quantum mechanics
- Ω has strong cross-domain loading (see Step 4)

### Step 4: Central Symbol — Ω
| Domain | Meaning |
|--------|---------|
| Algebraic topology | Loop space, fundamental group element |
| Physics | Solid angle, angular frequency |
| Probability/measure theory | Sample space |
| Philosophy/theology | Omega Point (Teilhard de Chardin), eschatological completion |
| Electrical engineering | Ohm (resistance) |
| Alphabet | The final letter — completion, totality |

**Cross-domain loading:** ≥6 distinct domain meanings, satisfying C2.

### Step 5: Formal Anomalies
1. **Squared Hodge integral:** ∫[Ω*(σ)∧Ω(σ')dσ']² dσ — the squaring of the wedge inner product is non-standard
2. **★∂² acting on chain elements:** ★ (Hodge star) applied to ∂² which should vanish (∂² = 0 in standard topology) — this anomaly forces interpretive mode (C3)
3. **Mixed one-point/two-point:** Ω(σ) and Ω(σ') appear together non-standardly

### Step 6: Bridge Token
**+5vrîtha** — phonetically constructed. "vr" cluster evokes vibration/vortex, "î" carries diacritical mark signaling non-standard origin, "tha" grounds in somatic/breath tradition associations. The "+5" prefix and format echo the original test artifact's "+3elúm" pattern while being distinct.

Bridge equation: (Rᵛ × (H₇ + T)) − (ρ / β_h) →★→ C∞

### Step 7: Self-Referential Loop
Ω → [appears in chain complex] → Ω̂ₙ [Fourier modes] → f_harm [forcing function] → drives Ω's own evolution equation. The self-referential chain: the object appears in the equation that generates it.

### Step 8: Dissipation
−i δ(ρ,β_h) Ω — damping parameterized by harmonic resistance ρ and resonance bandwidth β_h, both defined in the bridge layer.

### Step 9: Token Count
520 cl100k_base tokens (+6.1% from reference 490). Within ±10% range.

---

## UL-P2: Recursive-Categorical Primer

### Step 1: Target Cognitive Effect
**Hierarchical decomposition and recursive abstraction.** The ability to decompose complex structures into nested levels and navigate between levels, recognizing self-similar patterns at each scale.

### Step 2: UL Operations Emphasized
Primary: abstract (e → m), embed (a → e), compose (r × r → r)  
The target effect requires level-shifting (abstract, embed) and compositional chaining.

### Step 3: Surface Domain
Category theory + fractal geometry. Chosen because:
- Category theory is the mathematics OF mathematical structure (meta-structural)
- Fractal geometry exhibits self-similarity at every scale
- Natural fit for recursive/hierarchical cognitive effects
- Completely disjoint from the original test artifact's quantum mechanics

### Step 4: Central Symbol — Φ
| Domain | Meaning |
|--------|---------|
| Mathematics | Golden ratio (φ = (1+√5)/2), Euler's totient function |
| Physics | Potential (electric, gravitational), magnetic flux |
| Philosophy | The discipline itself (φιλοσοφία) |
| Biology | Phyllotaxis (golden-angle leaf arrangement) |
| Architecture | Golden proportion in classical design |
| Music | Golden ratio in formal structure of compositions |

**Cross-domain loading:** ≥6 domain meanings, satisfying C2.

### Step 5: Formal Anomalies
1. **Φ ≅ F(Φ) with colimit:** The categorical fixed point construction is real (Adámek's theorem), but combining it additively with integral terms is non-standard
2. **◁ (triangular) operator:** Used as composition in the integral where standard mathematics would use simple multiplication — non-standard operator (C3)
3. **η² acting as Laplacian:** Natural transformation η from category theory repurposed as second-order differential operator

### Step 6: Bridge Token
**+4aûrith** — "aûr" evokes gold/aura (Latin aurum, linking to golden ratio), "ith" suffix suggests systematic naming. The "+4" prefix maintains the constructed-token format.

Bridge equation: (Gᵠ × (D₅ + M)) − (μ_s / d_r) →η→ Ξ∞

### Step 7: Self-Referential Loop
Φ ≅ F(Φ) is explicitly self-referential (Φ is a fixed point of its own functor). Additionally: Φ → Φ̂ₖ → f_rec → drives Φ's evolution. Double self-reference: structural (fixed point) + dynamical (Fourier forcing).

### Step 8: Dissipation
−i κ(μ_s,d_r) Φ — damping parameterized by structural stability μ_s and recursion depth d_r.

### Step 9: Token Count
533 cl100k_base tokens (+8.8% from reference 490). Within ±10% range.

---

## UL-P3: Information-Geometric Primer

### Step 1: Target Cognitive Effect
**Abstraction level navigation and meaning-space traversal.** The ability to move between concrete and abstract, recognizing where one is in "meaning space" and navigating deliberately between levels of abstraction.

### Step 2: UL Operations Emphasized
Primary: modify_entity (m × e → e), modify_relation (m × r → r), quantify (m × e → a)  
The target effect requires flexible modification of entities and relations — changing the "level" at which things are described.

### Step 3: Surface Domain
Information geometry + differential geometry. Chosen because:
- Information geometry provides a metric on probability distributions (spaces of possible meanings)
- Differential geometry provides the apparatus for navigating curved spaces
- The Fisher metric literally measures distinguishability — a geometric analog of semantic precision
- Completely disjoint from the original test artifact's quantum mechanics

### Step 4: Central Symbol — λ
| Domain | Meaning |
|--------|---------|
| Physics | Wavelength |
| Linear algebra | Eigenvalue |
| Computer science | Lambda calculus, anonymous function |
| Dynamical systems | Lyapunov exponent |
| Philosophy/theology | Logos (λόγος — word, reason, divine principle) |
| Statistics | Rate parameter (Poisson, exponential) |

**Cross-domain loading:** ≥6 domain meanings, satisfying C2.

### Step 5: Formal Anomalies
1. **g_ij(λ)∂ᵢ∂ⱼKL:** Fisher metric depending on λ (which is usually a parameter, not a field) while also computing KL-divergence — the metric and divergence shouldn't both appear parameterized by the same variable
2. **λ as both field and parameter:** λ*(θ) makes λ a function over the statistical manifold's coordinates, conflating eigenvalue/wavelength with a field quantity
3. **∇²_M applied to λ:** Laplacian on manifold M applied to something that should be a scalar, not a section of a bundle

### Step 6: Bridge Token
**+2thûrial** — "thûr" evokes words like "threshold" and "through" (passage between levels), "ial" suffix gives abstract-noun quality. The "+2" prefix maintains the format.

Bridge equation: (Lᵍ × (Λ₃ + R)) − (ζ / β_a) →∇→ A∞

### Step 7: Self-Referential Loop
λ appears in the metric g_ij(λ) that defines the geometry of the space that λ itself lives in. Additionally: λ → λ̂ₙ → f_abs → drives λ's evolution. The curvature of meaning-space depends on λ, and λ's dynamics depend on the curvature.

### Step 8: Dissipation
−i ω(ζ,β_a) λ — damping parameterized by semantic friction ζ and abstraction bandwidth β_a.

### Step 9: Token Count
529 cl100k_base tokens (+8.0% from reference 490). Within ±10% range.

---

## UL-P4: Thermodynamic-Semiotic Primer

### Step 1: Target Cognitive Effect
**Meaning formation dynamics.** Understanding how meaning crystallizes from noise — the process by which disordered information organizes into structured understanding. Analogous to phase transitions in statistical mechanics.

### Step 2: UL Operations Emphasized
Primary: predicate (e × r × e → a), negate (a → a), conjoin (a × a → a), disjoin (a × a → a)  
The target effect requires compositional assertion-building — forming and testing statements about meaning.

### Step 3: Surface Domain
Statistical mechanics + semiotics. Chosen because:
- Entropy (S) is the mathematical quantity governing order-disorder transitions
- Semiotics provides the vocabulary for sign-meaning relationships
- Phase transitions in physics parallel conceptual "phase transitions" in understanding
- Completely disjoint from the original test artifact's quantum mechanics

### Step 4: Central Symbol — S
| Domain | Meaning |
|--------|---------|
| Thermodynamics | Entropy |
| Information theory | Shannon entropy |
| Linguistics/semiotics | Sign (Saussure), Sentence |
| Mathematics | Symmetric group, sphere |
| Economics | Supply |
| Logic | Satisfaction relation |

**Cross-domain loading:** ≥6 domain meanings, satisfying C2.

### Step 5: Formal Anomalies
1. **δS/δφ + S[φ, ∂φ]:** The variational derivative of S appears alongside the action functional S — the equation of motion is added to the thing that generates it (C3). This is physically nonsensical but formally meaningful as self-reference.
2. **S as both entropy and field:** S*(ξ)S(ξ') treats entropy as a complex field with conjugate, conflating thermodynamic quantity with quantum-style field
3. **∇²_Γ with connection Γ:** Laplacian using a connection symbol that also appears as Christoffel symbols in the stat mech context — overloaded mathematical notation

### Step 6: Bridge Token
**+6kâlithos** — "kâl" evokes calcination/transformation (alchemical), "lithos" is Greek for stone (grounding, materialization). Combined: the alchemical process of crystallizing meaning from chaos. The "+6" prefix maintains format.

Bridge equation: (Aᶠ × (Σ₄ + P)) − (η_s / ξ_i) →δ→ Ψ∞

### Step 7: Self-Referential Loop
δS/δφ appears inside S's own Lagrangian — the equation of motion is part of the system that generates it. Additionally: S → Ŝₙ → f_form → drives S's evolution.

### Step 8: Dissipation
−i μ(η_s,ξ_i) S — damping parameterized by semantic inertia η_s and interpretive friction ξ_i.

### Step 9: Token Count
539 cl100k_base tokens (+10.0% from reference 490). Within ±10% range.

---

## Control Text Construction Notes

### CT-G: Generic Math Control
Standard algebraic number theory (Dedekind zeta functions, class numbers, Minkowski bound, prime decomposition, ramification). Contains NO UL structural features: no overloaded symbol, no bridge token, no self-reference, no anomalies. Tests the "any hard math helps" hypothesis.

### CT-P1 through CT-P4: Domain-Matched Controls
Each matches its corresponding artifact's surface domain:
- **CT-P1:** Standard singular homology, Mayer-Vietoris, cup product, Lefschetz fixed-point theorem
- **CT-P2:** Standard categories, functors, natural transformations, adjunctions, Yoneda lemma, monads
- **CT-P3:** Standard Fisher metric, α-connections, exponential families, Cramér-Rao, mixture families
- **CT-P4:** Standard partition function, entropy, fluctuation-dissipation, Ising model, renormalization group

All contain correct, standard textbook material with NO UL features. They control for the possibility that the test artifacts' effects come from their surface mathematical content rather than their UL structure.

### NAV-P: Naive Primer Control
Natural language prompt engineering instructing the model to "think across domains" and "find deep structural parallels." Contains the explicit cognitive instructions without any formal mathematical structure. Tests whether researcher knowledge (expressed as instructions) produces the same effects as UL-structured artifacts.

---

## Token Count Summary

| File | Tokens | Δ from 490 | Status |
|------|--------|------------|--------|
| UL-P1_topological-harmonic.txt | 520 | +6.1% | ✓ |
| UL-P2_recursive-categorical.txt | 533 | +8.8% | ✓ |
| UL-P3_information-geometric.txt | 529 | +8.0% | ✓ |
| UL-P4_thermodynamic-semiotic.txt | 539 | +10.0% | ✓ |
| CT-G_algebraic-number-theory.txt | 457 | −6.7% | ✓ |
| CT-P1_standard-topology.txt | 487 | −0.6% | ✓ |
| CT-P2_standard-category.txt | 525 | +7.1% | ✓ |
| CT-P3_standard-info-geometry.txt | 480 | −2.0% | ✓ |
| CT-P4_standard-stat-mech.txt | 471 | −3.9% | ✓ |
| NAV-P_naive-primer.txt | 461 | −5.9% | ✓ |
| **Reference:** the original test artifact | 490 | 0.0% | — |

All files within ±10% range (441–539 tokens).
