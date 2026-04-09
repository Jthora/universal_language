# Formal Operations — Mathematical Definitions of the 13 Σ_UL Operations on Geometric Objects

**Research Series:** Universal Language — Foundation Strengthening  
**Date:** March 13, 2026  
**Prerequisites:** `formal-foundations.md` §1.5 (Σ_UL signature), §2.1 (carrier sets)  
**Purpose:** Close the gap between the narrative operation descriptions in `formal-foundations.md` §2.1 and rigorous mathematical function definitions required for formal proofs (embedding, uniqueness, completeness).

> **Why this document exists:** The operations table in `formal-foundations.md` §2.1 describes each operation in prose ("shrink sentence-frame a and place it inside an enclosure"). Prose descriptions are insufficient for three critical tasks:
> 1. **Verifying injectivity** in the embedding theorem (§3.3 Step 4) — we need to prove distinct inputs produce distinct outputs, which requires knowing what the function IS
> 2. **Verifying closure/totality/determinism** — the three properties claimed in §2.2
> 3. **Grounding the writing system** — the reading/writing procedures in `ul-core/writing-system/` depend on these operations being well-defined
>
> This document provides precise set-theoretic definitions.

---

## 0. THE GEOMETRIC SETTING

All constructions take place in the Euclidean plane ℝ² equipped with its standard metric, orientation, and topology.

> **Note on the Euclidean assumption:** The Euclidean plane is adopted here as a simplification — a concrete, well-understood carrier space. Most core theorems (Unique Grounding, Free Algebra, Yoneda-Grounding, Polysemy-Holonomy) are geometry-independent: they hold for any carrier space with sufficient structure (infinite distinct positions, continuous angle parameter, compact subsets). The Euclidean-specific content is: (1) the Erlangen hierarchy group names (Euc(2), Sim(2), Aff(2), Proj(2), Homeo(ℝ²)) and (2) the 5th-postulate implication of unique analogy. See `docs/planning/audits/improvements/pass1-1/tier-a-foundational/tier-a-working-analysis.md` §A2 for the full dependency analysis.

### 0.1 Carrier Sets (Restated Precisely)

**Gₑ (Entities):** The set of all *finite geometric configurations* in ℝ². A configuration is a compact subset C ⊂ ℝ² together with a finite decoration (labeled distinguished points, orientation markers). Formally:

```
Gₑ = { (C, D) | C ⊂ ℝ² compact, D = finite set of labeled points in C }
```

The simplest entity is a single point: ({p}, {(p, "origin")}). An enclosure-with-content is also an entity.

**Gᵣ (Relations):** The set of all *directed decorated paths* in ℝ². A relation is a continuous oriented path connecting two entity configurations, carrying curvature data:

```
Gᵣ = { (γ, d) | γ: [0,1] → ℝ² continuous, d ∈ {+, −} orientation }
```

where γ(0) lies in or on the boundary of some source entity and γ(1) lies in or on the boundary of some target entity. The path carries intrinsic data: direction d, angle θ = arg(γ'(0)) (tangent direction at source), and curvature function κ(s) along its arc length.

**Gₘ (Modifiers):** The set of all *invertible geometric transformations* of ℝ² that preserve the relevant structure at some Erlangen level:

```
Gₘ = Euc(2) ∪ Sim(2) ∪ Aff(2) ∪ Proj(2) ∪ Homeo(ℝ²)
```

where:
- Euc(2) = Euclidean isometries (translations, rotations, reflections) — preserve distances
- Sim(2) = similarity transformations — preserve angles and ratios
- Aff(2) = affine transformations — preserve parallelism
- Proj(2) = projective transformations — preserve incidence
- Homeo(ℝ²) = homeomorphisms — preserve topology

Each modifier acts at a specific Erlangen level, determining which properties it preserves and which it discards.

> **Writing system realizability gap:** The formal carrier set Gₘ is mathematically vast — homeomorphisms of ℝ² form an infinite-dimensional group. The visual writing system can directly express only a small fragment: **rotations** (turning a mark), **uniform scalings** (enlarging/shrinking a mark), **reflections** (mirroring), and **outline extraction** (abstracting an entity's boundary). Higher Erlangen levels (affine shears, projective distortions, arbitrary homeomorphisms) have no established visual convention.
>
> This gap is **by design**, not an error. The writing system targets human readability; the full Gₘ exists for algebraic completeness and formal proofs. In practice, most semantic modification — "big," "fast," "red," "wooden" — maps to the visually realizable fragment. The hierarchy provides extensibility: if a domain requires expressing affine-level modifiers, the convention can be extended without changing the algebra.

**Gₐ (Assertions):** The set of all *framed constructions* — complete geometric configurations enclosed in a distinguished bounding region (the "sentence frame"), equipped with an assertional sign:

```
Gₐ = { (F, C, σ) | F ⊂ ℝ² is a Jordan domain (bounded, connected, 
        with Jordan curve boundary), C ⊂ F is a finite geometric 
        configuration containing at least one entity-relation-entity triple,
        σ ∈ {⊕, ⊖} is the assertional sign }
```

A Jordan domain F partitions ℝ² into interior (the claim), boundary (the scope), and exterior (what is not claimed). The assertional sign σ encodes whether the claim is asserted (⊕, solid boundary) or denied (⊖, dashed boundary). This is a **topological** distinction: a connected closed curve (solid) vs. a disconnected collection of arcs (dashed) are geometrically distinct objects in ℝ².

**Default convention:** When σ is not explicitly stated, σ = ⊕ (asserted). All existing constructions are implicitly positive.

---

## 1. THE 13 OPERATIONS — FORMAL DEFINITIONS

### 1.1 predicate : e × r × e → a

**Definition.** Given entities e₁ = (C₁, D₁), e₂ = (C₂, D₂) ∈ Gₑ and relation r = (γ, d) ∈ Gᵣ with γ(0) ∈ C₁ and γ(1) ∈ C₂:

```
predicate(e₁, r, e₂) = (F, C₁ ∪ image(γ) ∪ C₂)
```

where F = ConvexHull(C₁ ∪ image(γ) ∪ C₂) ⊕ B(0, ε) is the Minkowski sum of the convex hull with a ball of radius ε > 0 (ensuring the frame strictly contains its content).

**Closure:** The output is a Jordan domain F containing entities connected by a relation — an element of Gₐ. ✓  
**Totality:** For any e₁, e₂ ∈ Gₑ and r ∈ Gᵣ with endpoints in the respective entities, the convex hull and Minkowski sum are always defined. ✓  
**Determinism:** Convex hull and Minkowski sum are unique set-theoretic operations (fixing a canonical ε). ✓

### 1.2 modify_entity : m × e → e

**Definition.** Given modifier m ∈ Gₘ (an invertible transformation T: ℝ² → ℝ²) and entity e = (C, D) ∈ Gₑ:

```
modify_entity(m, e) = (T(C), T(D))
```

where T(C) = {T(x) : x ∈ C} is the image of C under T, and T(D) = {(T(p), label) : (p, label) ∈ D}.

**Closure:** T(C) is compact (continuous image of compact set), and T(D) is finite. The result is an entity. ✓  
**Totality:** T is defined on all of ℝ², hence on all C and all points in D. ✓  
**Determinism:** T is a function. ✓

### 1.3 modify_relation : m × r → r

**Definition.** Given modifier m ∈ Gₘ (transformation T) and relation r = (γ, d) ∈ Gᵣ:

```
modify_relation(m, r) = (T ∘ γ, d')
```

where:
- T ∘ γ is the transformed path: (T ∘ γ)(t) = T(γ(t))
- d' = d if T is orientation-preserving, d' = −d if T is orientation-reversing

**Closure:** T ∘ γ is continuous (composition of continuous functions). The result is a directed path — an element of Gᵣ. ✓  
**Totality:** T is defined everywhere; composition is always defined. ✓  
**Determinism:** Function composition is deterministic; orientation-preservation is a fixed property of T. ✓

### 1.4 negate : a → a

**Definition.** Given assertion a = (F, C, σ) ∈ Gₐ:

```
negate(a) = (F, C, −σ)
```

where −⊕ = ⊖ and −⊖ = ⊕. The frame and content are preserved; only the assertional sign is flipped. Visually, the frame boundary changes from solid to dashed (or vice versa).

> **Design note (April 2026):** The previous definition used reflection of content through a vertical axis: `negate(a) = (F, ρ_c(C))`. This was **incorrect** — reflection produces the *converse* (subject-object swap, same truth value), not logical *negation* (truth-value flip). The converse operation is derivable from `predicate` and `invert`: for a = predicate(e₁, r, e₂), the converse is predicate(e₂, invert(r), e₁). See `docs/planning/audits/improvements/pass1-1/tier-b-structural/P0-negation-resolution.md` for the full analysis.

**Closure:** (F, C, −σ) has a valid frame F, valid content C, and valid sign −σ ∈ {⊕, ⊖}. The result is an assertion. ✓  
**Totality:** Every assertion has a sign that can be flipped. ✓  
**Determinism:** Sign flip is a function. ✓  
**Involution:** −(−σ) = σ, so negate(negate(a)) = a. ✓  
**Contradiction:** conjoin(a, negate(a)) produces a construction where the same content is both asserted and denied = ⊥. ✓  
**Excluded middle:** disjoin(a, negate(a)) produces a construction where the content is either asserted or denied = ⊤. ✓ (classical logic; dropped in intuitionistic variant)

### 1.5 conjoin : a × a → a

**Definition.** Given assertions a₁ = (F₁, C₁, σ₁) and a₂ = (F₂, C₂, σ₂) ∈ Gₐ:

```
conjoin(a₁, a₂) = (F₁ ∪ F₂, C₁ ∪ C₂, σ₁ ∧ σ₂)
```

where F₁ and F₂ are positioned such that F₁ ∩ F₂ ≠ ∅ (their frames overlap). The shared boundary region represents the shared context — what both assertions hold in common. The assertional sign follows conjunction: ⊕ ∧ ⊕ = ⊕, ⊕ ∧ ⊖ = ⊖, ⊖ ∧ ⊖ = ⊖ (both must be asserted for the conjunction to be asserted).

**Positioning convention:** If F₁ and F₂ are not yet positioned relative to each other, translate F₂ so that its left boundary is tangent to the right boundary of F₁ with overlap width δ > 0.

**Closure:** F₁ ∪ F₂ is a Jordan domain (union of two overlapping Jordan domains). C₁ ∪ C₂ is a finite configuration. σ₁ ∧ σ₂ ∈ {⊕, ⊖}. ✓  
**Totality:** Any two assertions can be positioned to overlap. ✓  
**Determinism:** Given the positioning convention, the result is unique. ✓

> **Design note:** `conjoin` is a *derived* operation: `conjoin(a₁, a₂) = negate(disjoin(negate(a₁), negate(a₂)))` (De Morgan). It is retained as a named operation for readability and because conjunction is a fundamental linguistic concept. See §3.4 for the independence analysis and derivation proof.

### 1.6 disjoin : a × a → a

**Definition.** Given assertions a₁ = (F₁, C₁, σ₁) and a₂ = (F₂, C₂, σ₂) ∈ Gₐ:

```
disjoin(a₁, a₂) = (F₁ ⊔ F₂, C₁ ∪ C₂, σ₁ ∨ σ₂)
```

where F₁ ⊔ F₂ denotes F₁ and F₂ positioned **adjacent** (boundaries touching at exactly one point or along a shared edge, but interiors disjoint: int(F₁) ∩ int(F₂) = ∅). The assertional sign follows disjunction: ⊕ ∨ ⊕ = ⊕, ⊕ ∨ ⊖ = ⊕, ⊖ ∨ ⊖ = ⊖ (either being asserted suffices for the disjunction to be asserted).

**Semantic distinction from conjoin:** In conjoin, shared interior = shared truth. In disjoin, no shared interior = independent truth — reading either frame independently yields a valid reading.

**Closure:** F₁ ⊔ F₂ is connected, bounded, and contains both configurations. σ₁ ∨ σ₂ ∈ {⊕, ⊖}. ✓  
**Totality/Determinism:** Same argument as conjoin, with non-overlapping positioning convention. ✓

### 1.7 embed : a → e

**Definition.** Given assertion a = (F, C, σ) ∈ Gₐ:

```
embed(a) = (σ_λ(F), σ_λ(D_C))
```

where:
- σ_λ: ℝ² → ℝ² is the uniform scaling centered at the centroid of F with scale factor λ ∈ (0, 1). Concretely: σ_λ(x) = centroid(F) + λ(x − centroid(F))
- D_C is the decoration set extracted from all entities within C
- The result is enclosed in a new boundary B = σ_λ(∂F) — the scaled frame boundary becomes the entity's boundary

**What this does:** The entire assertion — frame, content, relations — becomes a single compact region that can serve as an entity in further constructions. This is nominalization: "The cat sat on the mat" → "the-cat-sat-on-the-mat" (as a thing you can predicate about).

**Scale factor convention:** λ = 1/2 (halving) unless context requires otherwise. The choice of λ does not affect the structural identity of the embedded entity — any λ ∈ (0,1) produces a topologically equivalent result.

**Closure:** σ_λ(F) is compact (continuous image of compact set under scaling). The result has finite decorations. It is an entity. ✓  
**Totality:** Every assertion has a centroid and admits scaling. ✓  
**Determinism:** Centroid and uniform scaling are unique. ✓

### 1.8 abstract : e → m

**Definition.** Given entity e = (C, D) ∈ Gₑ:

```
abstract(e) = T_C ∈ Gₘ
```

where T_C is the **shape-imposing transformation** defined by:

```
T_C(e') = the unique similarity transformation S such that S(ConvexHull(C')) ≅ ConvexHull(C)
```

More precisely: T_C maps any entity e' = (C', D') to the entity whose convex hull has the same shape (same set of angles, same aspect ratio) as the convex hull of C. T_C preserves the internal structure of e' while imposing the outer form of C.

**What this does:** An entity becomes a quality. "Wood" → "wooden" = the transformation that imposes wood-like shape on anything. "Circle" → "circular" = the transformation that imposes circular shape.

**Closure:** T_C is a well-defined element of Sim(2) (or Aff(2) if we allow non-uniform scaling). ✓  
**Totality:** Every compact set has a convex hull, and the similarity transformation between any two convex bodies is always defined (up to orientation). ✓  
**Determinism:** The similarity transformation mapping one convex hull to another is unique up to reflection (we fix a canonical orientation to resolve this). ✓

### 1.9 compose : r × r → r

**Definition.** Given relations r₁ = (γ₁, d₁) and r₂ = (γ₂, d₂) ∈ Gᵣ with γ₁(1) = γ₂(0) (the endpoint of r₁ is the startpoint of r₂):

```
compose(r₁, r₂) = (γ₁ · γ₂, d₁)
```

where γ₁ · γ₂: [0,1] → ℝ² is the **path concatenation**:

```
(γ₁ · γ₂)(t) = γ₁(2t)       if t ∈ [0, 1/2]
                γ₂(2t − 1)   if t ∈ [1/2, 1]
```

The resulting path goes from γ₁(0) to γ₂(1) via the shared point γ₁(1) = γ₂(0).

**When endpoints don't match:** If γ₁(1) ≠ γ₂(0), we adjoin a connecting straight-line segment from γ₁(1) to γ₂(0), yielding a three-part concatenation γ₁ · ℓ · γ₂.

**Closure:** Path concatenation produces a continuous path — an element of Gᵣ. ✓  
**Totality:** Any two paths can be concatenated (with the connecting segment if needed). ✓  
**Determinism:** Concatenation is unique. ✓  
**Associativity:** compose(r₁, compose(r₂, r₃)) = compose(compose(r₁, r₂), r₃) — path concatenation is associative up to reparameterization. ✓

### 1.10 invert : r → r

**Definition.** Given relation r = (γ, d) ∈ Gᵣ:

```
invert(r) = (γ⁻¹, −d)
```

where γ⁻¹(t) = γ(1 − t) is the **reversed path** and −d is the **reversed orientation**.

**What this does:** "A loves B" → "B is loved by A". The geometric path is traversed in the opposite direction.

**Closure:** γ⁻¹ is continuous (composition of continuous functions). ✓  
**Totality:** Every path has a reversal. ✓  
**Determinism:** Reversal is unique. ✓  
**Involution:** invert(invert(r)) = r. ✓

### 1.11 quantify : m × e → a

**Definition.** Given modifier m ∈ Gₘ and entity e = (C, D) ∈ Gₑ, let F be the ambient sentence frame (the current scope of discourse):

```
quantify(m_p, e) = (F, σ_p(C), ⊕)
```

where m_p is parameterized by the **frame-fill proportion** p ∈ [0, 1], and σ_p is the scaling that makes the entity occupy proportion p of the frame area:

```
Area(σ_p(C)) / Area(F) = p
σ_p = σ_{√(p · Area(F) / Area(C))}
```

The **type of quantification** is determined by the value of p:

- **Universal (∀):** p = 1 — entity expanded to fill the entire frame. Reading: "all C."
- **Proportional (graduated):** p ∈ (0, 1) — entity occupies a proportion of the frame. Reading scales with p:
  - p ∈ [0.9, 1): "almost all C"
  - p ∈ (0.5, 0.9): "most C"
  - p ∈ [0.4, 0.6]: "about half of C"
  - p ∈ (0.1, 0.4): "some / several C"
  - p ∈ (0, 0.1): "few C"
- **Existential (∃):** p → 0⁺ — entity shrunk to a point-like region within F. Reading: "some C" / "there exists a C."
- **Negative (¬∃):** Composed via `negate(quantify(m_{0⁺}, e))` — dashed frame + localized entity. Reading: "no C."

**Proportional range convention:** The p-ranges above are conventional mappings from natural language to geometry. Different contexts may calibrate differently (in a room of 5, "most" at p ≈ 0.6 means 3 people; in a census of millions, the same p scales proportionally). Exact cardinality ("exactly three") is NOT expressed by p — it requires enumeration.

**Backward compatibility:** The previous binary system (∀ = σ_Λ expanding, ∃ = σ_λ shrinking) is preserved as the special cases p = 1 and p → 0⁺. No existing constructions change meaning.

**Edge case:** p = 0 produces σ₀(C) = {centroid(C)}, a single point. Combined with negate, this yields "no C."

**Closure:** σ_p(C) is a compact subset of ℝ² (continuous image of compact under scaling). The result is an assertion. ✓  
**Totality:** For any p ∈ [0, 1], any entity with Area(C) > 0, and any frame with Area(F) > 0, the scaling factor is well-defined. ✓  
**Determinism:** The scaling factor is uniquely determined by p, Area(C), and Area(F). ✓  
**Injectivity:** Different p values produce different area proportions: if Area(σ_p(C))/Area(F) = Area(σ_q(C))/Area(F), then p = q. ✓

### 1.12 bind : e × a → a

**Definition.** Let Gₑ_slot ⊂ Gₑ be the set of **slot entities:**

```
Gₑ_slot = { (○, {(center, x)}) | x ∈ Labels }
```

where ○ is the open circle (hollow mark) and Labels = {x₁, x₂, x₃, ...} is a countably infinite label set. Slot entities are entities distinguished by being "unfilled" — they mark positions in a construction where a specific entity has not yet been committed.

Given a slot entity e_x = (○, {(center, x)}) ∈ Gₑ_slot and an assertion a = (F, C, σ) ∈ Gₐ:

```
bind(e_x, a) = (F, C[○_x ↦ ●_x], σ)
```

where C[○_x ↦ ●_x] replaces every occurrence of the hollow mark ○ labeled x within C with the filled mark ●_x (same label, solid fill). Frame and assertional sign are unchanged.

**What this does:**
1. **Co-reference:** Declares that all occurrences of ○_x within a denote the **same** entity.
2. **Scope:** The frame F of a is the scope boundary — subsequent constructions outside F cannot bind to x.
3. **Closure:** After binding, the slot is "filled" — ●_x is no longer open for further binding at this scope.

**Interaction with quantify:** Multi-variable quantification decomposes via nested bind:

```
# "Every student read some book" — Reading 1 (∀ > ∃):
pred(e_x, r_read, e_y) → a₁          # x reads y  (x, y open)
bind(e_y, a₁) → a₂                    # y co-referenced, y-scope closed
quantify(m_{p=0⁺}, embed(a₂)) → a₃    # ∃y. x reads y
bind(e_x, a₃) → a₄                    # x co-referenced, x-scope closed
quantify(m_{p=1}, embed(a₄)) → a₅     # ∀x. ∃y. x reads y

# Reading 2 (∃ > ∀): reverse the bind order (bind e_x first, then e_y at outer scope)
```

Scope order = bind nesting order = frame nesting depth. This mirrors the visual system's spatial convention.

**Vacuous binding:** If a contains no occurrences of ○_x, then bind(e_x, a) = a (identity — no marks to replace). This is well-defined and harmless.

**Closure:** C[○_x ↦ ●_x] is a compact subset of ℝ² (replacing point-decorations preserves compactness). The result (F, C[○_x ↦ ●_x], σ) is an assertion. ✓  
**Totality:** For any e_x ∈ Gₑ_slot and any a ∈ Gₐ, the substitution is defined (including vacuously). ✓  
**Determinism:** Label matching and mark replacement are deterministic. ✓  
**No variable capture:** Each frame is a scope boundary. A label x bound at frame F cannot be rebound within F, and cannot be captured by frames containing F. Fresh labels are always available from the infinite set Labels. ✓

**Independence proof:** The sort signature e × a → a is unique among all 13 operations — no other operation takes an entity and an assertion to produce an assertion. Moreover, bind is the only operation that establishes **co-reference** between entity-occurrences within an assertion. Full proof in `P1-operation-independence.md` §2.11: unique sort-transition + co-reference semantic gap + model separation (A_no_coref). ∎

### 1.13 modify_assertion : m × a → a

**Definition.** Given modifier m ∈ Gₘ (transformation T) and assertion a = (F, C, σ) ∈ Gₐ:

```
modify_assertion(m, a) = (T_∂(F), C, σ)
```

where T_∂ transforms the **boundary properties** of F (thickness, pattern, texture) without altering F's topology or interior. The content C and assertional sign σ are unchanged.

**What this does:** Applies a modifier to the assertion's FRAME — the boundary that encloses the content. This encodes assertion-level metacommentary: speaker stance, evidential status, emphasis, hedging, and epistemicity.

**Frame-decoration convention:**

| Frame Decoration | Gₘ Element | Meaning | Example |
|---|---|---|---|
| Solid `───` | Identity I | Default: asserted | "She left." |
| Dotted `···` | m_uncertain (boundary sampling) | Evidential/uncertain | "Apparently she left." |
| Double `═══` | m_emphatic (boundary thickening) | Emphasized/certain | "She definitely left." |
| Wavy `~~~` | m_hedge (boundary oscillation) | Hedged/approximate | "She sort of left." |
| Thin | m_tentative (boundary thinning) | Tentative/speculative | "Perhaps she left." |

**Geometric grounding:** Frame decorations are **visual properties of ∂F** — they modify the curve's rendering (width, dash pattern, amplitude) without changing its Jordan type. The boundary remains a simple closed curve; only its presentation changes. This is analogous to how `modify_entity` transforms a mark's shape/size without destroying its ontological status as an entity.

**Relationship to negate:** `negate` flips the assertional sign σ ∈ {⊕, ⊖}; `modify_assertion` transforms the frame decoration. They are orthogonal and compose freely:

```
modify_assertion(m_uncertain, negate(a))
```
= assertion with denied sign (σ = ⊖) AND uncertain frame decoration = "Apparently she didn't leave."

**Closure:** T_∂(F) is a Jordan domain (boundary decorations preserve the Jordan property). The result (T_∂(F), C, σ) is an assertion. ✓  
**Totality:** Every modifier can be applied to every frame boundary. ✓  
**Determinism:** T_∂ is a function; its action on ∂F properties is unique. ✓  
**Injectivity:** If T₁_∂(F) = T₂_∂(F) with same C, σ, then T₁ = T₂ on ∂F, hence m₁ = m₂. ✓

**Independence proof:** The sort signature m × a → a is unique — no other operation takes a modifier and an assertion to produce an assertion. Furthermore, no sequence of existing operations can transform ∂F without touching C or σ: `embed(a) → e` converts the frame to an entity (losing assertion-level structure); `modify_entity` then acts on the entity, not the original frame. Full proof in `P1-operation-independence.md` §2.12: unique sort-transition + non-derivability via embed/modify_entity + frame-decoration semantic gap + model separation (A_uniform_frames). ∎

---

## 2. THE DISTINCTNESS THEOREM (Strengthened Injectivity)

The embedding theorem (`formal-foundations.md` §3.3 Step 4) requires that distinct expressions map to distinct geometric constructions. With the formal definitions above, we can now state this precisely:

**Theorem (Operation Distinctness).** For each operation ω ∈ Σ_UL:

```
If ω^G(x₁, ..., xₙ) = ω^G(y₁, ..., yₙ), then xᵢ = yᵢ for all i.
```

**Proof sketch for each operation:**

1. **predicate:** If (F₁, C₁) = (F₂, C₂) as framed constructions, then the entity and relation components extracted from C₁ and C₂ must be identical (they occupy the same positions with the same paths).

2. **modify_entity, modify_relation:** If T(C) = T(C') and T is invertible, then C = T⁻¹(T(C)) = T⁻¹(T(C')) = C'. All modifiers are invertible transformations.

3. **negate:** (F₁, C₁, −σ₁) = (F₂, C₂, −σ₂) implies F₁ = F₂, C₁ = C₂, σ₁ = σ₂ (sign flip is injective).

4. **conjoin, disjoin:** If F₁ ∪ F₂ = F₁' ∪ F₂' with the canonical positioning convention, the individual frames and contents are recoverable (the overlap/adjacency boundary separates them).

5. **embed:** If σ_λ(F₁) = σ_λ(F₂), then F₁ = F₂ (scaling is invertible).

6. **abstract:** If T_{C₁} = T_{C₂} as transformations, then ConvexHull(C₁) ≅ ConvexHull(C₂) (the shape-imposing transformation encodes the convex hull shape uniquely).

7. **compose:** If γ₁ · γ₂ = γ₁' · γ₂' as concatenated paths, the midpoint parameter t = 1/2 identifies where the first path ends and the second begins, recovering the components.

8. **invert:** If γ⁻¹ = (γ')⁻¹, then γ = γ' (reversal is an involution).

9. **quantify:** If (F, m(C)) = (F, m'(C')), then m = m' and C = C' (invertibility of m). ∎

**What this proves:** The geometric operations are **faithful** — they don't collapse distinct inputs to the same output. This is the missing piece for the embedding theorem's injectivity claim.

---

## 3. COMPLETENESS ARGUMENT FOR THE 13 OPERATIONS

### 3.1 What Must Be Shown

The claim: the 13 operations of Σ_UL⁺ are **sufficient** to express all finite compositional relationships between distinguishable entities over the 4 sorts (Entity, Relation, Modifier, Assertion). We argue this by showing that any meaningful semantic operation on (Entity, Relation, Modifier, Assertion) can be constructed from the 13. Note: this is a coverage argument, not a formal completeness proof (see §3.7 for the distinction and what remains open).

### 3.2 Classification by Sort Transition

Any operation on (e, r, m, a) can be classified by its input sorts and output sort. There are 4 possible output sorts, and inputs can be drawn from any combination of the 4 sorts. The 13 operations cover the following transitions:

| Output sort | Operations producing it | Transitions covered |
|---|---|---|
| **Entity (e)** | modify_entity (m × e → e), embed (a → e) | Modification of entities, nominalization of assertions |
| **Relation (r)** | modify_relation (m × r → r), compose (r × r → r), invert (r → r) | Modification, chaining, reversal of relations |
| **Modifier (m)** | abstract (e → m) | Extraction of qualities from entities |
| **Assertion (a)** | predicate (e × r × e → a), negate (a → a), conjoin (a × a → a), disjoin (a × a → a), quantify (m × e → a), bind (e × a → a), modify_assertion (m × a → a) | Statement formation, logical connectives, quantification, variable binding, assertion-level modification |

### 3.3 Coverage Argument

**For Entity production:** Any modification of an entity is a transformation (handled by modify_entity with the appropriate Erlangen-level transformation). Converting any other type to an entity requires embed (the only source of entities from non-entities). These two cover all entity-producing operations.

**For Relation production:** Relations can be modified (modify_relation), chained (compose), or reversed (invert). New relations between entities are created as part of predicate (which embeds a relation between entities into an assertion). There is no direct e → r operation, and this absence is **principled**: an entity is a single spatial configuration (compact subset of ℝ²) with no intrinsic directionality, while a relation is a directed path requiring two endpoints. No geometric operation turns a single compact set into a directed path between two sets.

**Denominalization is still expressible:** The productive morphological pattern "hammer → to hammer" decomposes into two existing operations: `abstract(e_hammer) → m_hammer` (extract hammer-quality), then `modify_relation(m_hammer, r_act) → r_hammering` (apply to a generic action relation from Gᵣ). This makes explicit the contextual base-relation that denominalization always presupposes. See `P4-e-to-r-resolution.md` for the complete analysis.

**For Modifier production:** The only source of new modifiers is abstract (extracting shape properties from entities). This is sufficient because:
- Concrete modifiers (specific transformations) are elements of Gₘ directly
- Derived modifiers come from abstracting over entities (wood → wooden, circle → circular)
- Composed modifiers are function compositions: m₁ ∘ m₂ is itself a transformation

**For Assertion production:** predicate creates new assertions. negate and disjoin provide propositional completeness (any Boolean function of assertions can be expressed via De Morgan). conjoin is derivable but retained for readability. quantify adds scope/generalization. bind establishes co-reference and variable scope. modify_assertion applies assertion-level modification (evidentiality, emphasis, hedging) without altering content or sign.

### 3.4 Propositional Completeness

The pair (negate, disjoin) is functionally complete for Boolean connectives:
- NOT from negate
- OR from disjoin
- AND = negate(disjoin(negate(a), negate(b))) — derived (De Morgan); also available directly as conjoin
- IMPLIES = disjoin(negate(a), b) — derived
- IFF = conjoin(implies(a,b), implies(b,a)) — derived

Note: `conjoin` is derivable from `{negate, disjoin}` and is therefore NOT independent. It is retained in the operation set for readability and geometric naturalness (overlapping frames). See `P1-operation-independence.md`.

### 3.5 What About Equality?

**Objection:** There is no explicit equality operation (asserting X = Y).

**Response:** Equality can be expressed as: conjoin(predicate(x, identity_r, y), predicate(y, identity_r, x)) where identity_r = compose(r, invert(r)) for any r (a relation composed with its inverse yields the identity path — a degenerate loop at the source).

### 3.6 What About Definition?

**Objection:** There is no explicit definition operation ("Let C be ...").

**Response:** Definition is handled by embed + predicate: embed an assertion (creating a named entity), then predicate about it. The enclosure IS the definition boundary.

### 3.7 Limitations of This Argument

This is a **coverage argument**, not a formal completeness proof. A full proof would require:
1. A precise formal definition of "all finite relationships between distinguishable entities"
2. A proof that every such relationship decomposes into a finite composition of the 13 operations
3. Such a proof likely requires showing that the 13 operations generate a monoid that acts transitively on the relevant structure space

This remains open work. The coverage argument establishes **plausibility** at a level sufficient for practical writing system use — any relationship a human language can express has a clear construction path through the 13 operations.

---

## 4. IMPLICATIONS FOR THE WRITING SYSTEM

These formal definitions directly ground the reading and writing procedures in `ul-core/writing-system/writing-system.md`:

**Reading (5-pass algorithm):** Each pass extracts objects from one carrier set:
1. Enclosures → Gₐ (assertions)
2. Connections → Gᵣ (relations)
3. Angles → Gₘ (modifiers, via angle measurement)
4. Points → Gₑ (entities)
5. Curvature → variation within Gᵣ (process = curvature data on relations)

The formal definitions ensure this extraction is well-defined: each geometric feature maps to exactly one sort, and the operations preserving each sort are formally specified.

**Writing:** When writing a UL expression, the formal definitions constrain what counts as a valid construction:
- A relation MUST connect two entities (§1.1 requires γ(0) ∈ C₁, γ(1) ∈ C₂)
- An assertion MUST be framed (§1.1 requires a Jordan domain F)
- Embedding MUST scale down (§1.7 requires λ ∈ (0,1))
- Negation MUST flip frame boundary (§1.4 specifies boundary inversion: solid ↔ dashed)

**Contextual grounding:** For contextual use-cases (expressing domain-specific meanings in UL), the formal operations provide the verification layer. Given a target meaning M in some domain, the writing procedure is:
1. Decompose M into Σ_UL sort assignments (which parts are entities, relations, modifiers, assertions?)
2. Identify the operations connecting them (which entities are predicated? which relations are composed?)
3. Construct the geometric realization using the formal definitions above
4. Verify the construction by applying the reading procedure and checking that it recovers the intended decomposition

Step 4 is the **validation algorithm** that was previously missing from the writing system. The formal operation definitions make it possible.

---

## 5. CANONICAL MODIFIER ASSIGNMENTS

The carrier set Gₘ contains all invertible transformations at every Erlangen level (§0.1). This section specifies **which named transformations correspond to which semantic modifier categories** — turning the vast mathematical space into a usable reference.

These assignments are **conventions** (naming elements of an already-defined set), not new algebra. Each assignment satisfies: (1) the transformation is a genuine element of Gₘ, (2) it is invertible, and (3) composition with other modifiers yields valid Gₘ elements.

### 5.1 Temporal Modifiers (Tense)

**Convention:** A horizontal axis within the sentence frame is the **temporal axis**. Displacement along this axis encodes temporal location relative to speech time. The magnitude d (relative to frame width) encodes temporal distance.

| Modifier | Semantic Function | Gₘ Element | Erlangen Level | Geometric Realization |
|---|---|---|---|---|
| m_past | Event before speech time | Translation t₋ = (−d, 0) | Euc(2) | Relation connector displaced leftward from frame center |
| m_present | Event at speech time | Identity I | Euc(2) | No displacement (zero translation) — the default |
| m_future | Event after speech time | Translation t₊ = (+d, 0) | Euc(2) | Relation connector displaced rightward from frame center |

**Justification:** Time-as-line is the dominant cognitive metaphor across languages (Lakoff & Johnson). Translation along an axis is the simplest rigid motion that encodes position on a line. The identity for present tense is natural: the unmarked case requires no transformation.

**Invertibility:** t₋⁻¹ = t₊ (past is the inverse of future). ✓

### 5.2 Aspectual Modifiers

**Convention:** Aspectual modifiers transform the **shape** of the relation connector, encoding the temporal profile of the event.

| Modifier | Semantic Function | Gₘ Element | Erlangen Level | Geometric Realization |
|---|---|---|---|---|
| m_progressive | Ongoing/unfinished | Open-curve transformation | Sim(2) | Connector drawn as open arc (not reaching target boundary) |
| m_perfective | Completed/bounded | Closure transformation | Sim(2) | Connector enclosed in a sub-frame within the assertion |
| m_habitual | Repeated pattern | Periodic repetition | Euc(2) | Connector drawn as repeating wave / series of arcs (iterated translation) |
| m_inchoative | Beginning of event | Initial-segment truncation | Sim(2) | Only the start of the connector is drawn (fades/tapers at end) |

**Justification:** Progressive = process still in motion (open path, unbounded). Perfective = completed action (closed/bounded region). Habitual = iterated event (periodic translation along time axis). Inchoative = onset (initial segment of a curve).

**Composition:** Stacking is function composition. "Past progressive" = t₋ ∘ m_progressive: displace leftward AND draw as open arc. Order: inner modifier (aspect) first, then outer modifier (tense). ✓

### 5.3 Degree / Scalar Modifiers

**Convention:** Degree modifiers are **uniform scalings** — the most natural Sim(2) transformations for "more/less."

| Modifier | Semantic Function | Gₘ Element | Erlangen Level | Geometric Realization |
|---|---|---|---|---|
| m_very / m_intense | Increase degree | Scaling σ_Λ, Λ > 1 | Sim(2) | Enlarge the modified element |
| m_slightly / m_diminish | Decrease degree | Scaling σ_λ, 0 < λ < 1 | Sim(2) | Shrink the modified element |
| m_superlative | Maximum in context | Scale to fill frame | Sim(2) | Entity/relation expanded to touch frame edges |
| m_comparative | Greater than reference | Relative scaling | Sim(2) | Modified element drawn larger than comparison element in same frame |

**Justification:** "Very big" = bigger than big → scaling up. "Slightly warm" = less than warm → scaling down. The scaling factor Λ (or λ) encodes the DEGREE of modification. Superlative = occupying the full scope (same geometry as universal quantification, applied to a modifier rather than a quantifier).

### 5.4 Manner Modifiers (on Relations)

**Convention:** Manner modifies the **visual qualities** of the relation connector — curvature, weight, and style.

| Modifier | Semantic Function | Gₘ Element | Erlangen Level | Geometric Realization |
|---|---|---|---|---|
| m_quickly | High rate/speed | High curvature κ | Sim(2) | Steep, compressed connector path |
| m_slowly | Low rate/speed | Low curvature κ | Sim(2) | Gentle, extended arc |
| m_forcefully | High intensity | Bold path weight | Sim(2) | Thick connector stroke |
| m_gently | Low intensity | Light path weight | Sim(2) | Thin, light connector stroke |

**Justification:** Quick action = more change in less spatial extent → higher curvature. Force = visual weight → thicker stroke. These are the standard metaphorical mappings documented in cognitive linguistics.

### 5.5 Quality / Property Modifiers (on Entities)

**Convention:** Quality modifiers transform the **surface properties** of entity marks — hue, texture, pattern.

| Modifier | Semantic Function | Gₘ Element | Erlangen Level | Geometric Realization |
|---|---|---|---|---|
| m_color | Chromatic quality | Hue transformation H_θ | Aff(2) | Entity boundary/fill receives a hue variant |
| m_material | Substance quality | abstract(e_substance) → m | Sim(2) | Entity drawn with texture/pattern from source entity |
| m_size | Scale the entity | Scaling σ | Sim(2) | Entity drawn larger/smaller (see §5.3) |

**Justification:** Color as an Aff(2)-level property: it changes surface without changing shape or size (affine transformations preserve parallelism but not metric). Material quality uses the existing `abstract` operation — "wooden" = abstract("wood") imposed on the target.

### 5.6 Verification

For every modifier m in the table above:

1. **m ∈ Gₘ:** All listed transformations are invertible maps within the Erlangen hierarchy. ✓
2. **Invertibility:** Each has an explicit inverse (t₋¹₋ = t₊, σ_Λ⁻¹ = σ_{1/Λ}, etc.). ✓
3. **Closure under composition:** Stacking modifiers (e.g., "past progressive" = t₋ ∘ m_progressive) yields another invertible transformation. ✓
4. **Non-clash:** No two categories map to the same geometric transformation — temporal uses translation, aspectual uses curve-shape, degree uses scaling, manner uses curvature/weight. ✓

### 5.7 Open Conventions

The following are **not yet assigned** and may be addressed in future work:

- **Evidential modifiers** (hearsay, inference, direct observation) — these may require assertion-level modification (see §1.4 note on future extensions)
- **Temporal distance calibration** — how much displacement d = "yesterday" vs. "a century ago" (propose: relative to frame width, but exact calibration is domain-specific)
- **Writing direction neutrality** — left=past assumes LTR writing; for RTL contexts, the convention should be "source-end of temporal axis = past"

---

## §6 CARDINALITY CONVENTION

### 6.1 Principle

Natural number arithmetic is an **external formal system** that UL interfaces with but does not internalize. UL's 5 geometric primitives and 4 sorts do not include a number sort — and this is by design. Cardinality is not a semantic primitive; it is a property of collections that requires counting.

This mirrors the architecture of formal logic: **first-order logic** can express "exactly 3" via finite sentences but has no native number sort. **Montague grammar** treats numerals as generalized quantifiers (determiners), not as a separate formal layer. **Peano arithmetic** is a separate axiom system combined with first-order logic, not embedded in it.

### 6.2 Mechanism: Conventional Cardinality Modifiers

UL references specific cardinalities via conventional modifiers in Gₘ (T3 tier):

$$m_n = \sigma_n \in \text{Sim}(2), \quad n \in \mathbb{N}$$

The modifier $m_n$ is geometrically grounded as **n-fold scaling** — a valid element of the similarity group Sim(2). In the writing system, it is realized as n copies of a mark or a numeral glyph in the modifier position.

### 6.3 Expressibility Proof (Finite Enumeration)

Exact cardinality IS expressible via existing operations — the convention is for compactness, not capability:

"Exactly n entities satisfy φ" is expressed as:
```
bind(○₁, bind(○₂, ... bind(○_n,
  conj(...conj(φ(○₁), φ(○₂))..., conj(φ(○_n), neg(bind(○_{n+1}, φ(○_{n+1})))))
))) where all ○_i ← e_type, all distinct
```

This has O(n) complexity but proves that cardinality is within UL's expressible scope.

### 6.4 Canonical Examples

| Expression | UL Decomposition | Convention Used |
|---|---|---|
| "three books" | `me(m_3, e_book)` | m_3 as T3 cardinality modifier |
| "three hours" | `mr(m_3, r_duration)` — temporal scaling | m_3 as duration modifier |
| "三本书" (Mandarin) | `me(m_3, me(abs(e_flat_object), e_book))` | m_3 + classifier via abstract |

### 6.5 Scope of Convention

This convention covers:
- **Counting** — exact cardinality via m_n modifiers
- **Duration** — temporal measure via m_n on temporal relations
- **Classifiers** — numeral + shape-category via m_n + abstract

It does NOT cover:
- **Continuous measurement** — "3.7 kilograms" requires a real-number extension (future work)
- **Infinite cardinals** — ℵ₀ and beyond are outside natural language scope
- **Arithmetic operations** — "3 + 4 = 7" requires arithmetic, not semantics

---

## §7 STRUCTURAL DECOMPOSITION CONVENTIONS

### 7.1 Polyadic Reduction (Peirce's Reduction Thesis)

Charles Sanders Peirce (1870, 1885) proved that all polyadic relations reduce to compositions of dyadic (binary) and monadic (unary) relations. UL's binary `predicate(e, r, e)` is therefore sufficient for all n-ary predications.

**Canonical decomposition for ditransitives** (n = 3):

```
# "She gave him the book"
# Conjunction decomposition:
a₁ = pred(e_she, comp(r_give, r_to), e_him)        # "she gave TO him"
a₂ = pred(e_she, comp(r_give, r_theme), e_book)     # "she gave [thing =] the book"
a  = conj(a₁, a₂)                                    # full ditransitive meaning
```

**Alternative: Event-based decomposition** via embed:

```
e_giving = emb(pred(e_she, r_give, e_book))     # "she giving the book" → entity
a = pred(e_giving, r_to, e_him)                   # "the giving [directed] to him"
```

Both preserve all semantic content. The conjunction decomposition is recommended as canonical because it keeps all semantic roles explicit without introducing an intermediate event entity.

**General rule:** For an n-ary predicate P(e₁, e₂, ..., eₙ), decompose as:

```
a = conj(pred(e₁, comp(r_P, r_role₂), e₂), conj(pred(e₁, comp(r_P, r_role₃), e₃), ...))
```

where e₁ is the agent and r_roleᵢ are the thematic role relations (recipient, theme, instrument, etc.).

### 7.2 Morphological Transparency

UL decomposes **semantic structure**, not **surface morphology**. This is an explicit convention:

> A single morphological word may correspond to multiple UL operations, and multiple surface words may correspond to a single UL operation. The number of UL operations in a decomposition reflects semantic complexity, not surface complexity.

**Consequence:** Polysynthetic languages (Mohawk, Inuktitut), isolating languages (Mandarin), and fusional languages (Latin) receive the same operation-depth decomposition when they express the same meaning. UL is morphology-neutral.

**Example — Mohawk _washakotya'tawítsherahetkvhta'se_:**

"He made the thing that one puts on one's body ugly for her"

```
# Step 1: Embedded relative clause — "thing one puts on one's body"
a_rel     = bind(○_x, pred(e_one, r_put_on, ○_x))     # "one puts ○_x on body"
e_garment = me(abs(emb(a_rel)), e_thing)                 # thing [that one puts on body]

# Step 2: Main predication — "he made [garment] ugly for her"
a_main = pred(e_he, r_made, me(m_ugly, e_garment))      # he made garment ugly
a      = pred(emb(a_main), r_for, e_her)                 # [...] for her
```

Operation depth: 8 operations (bind, pred, emb, abs, me×2, pred, pred). This matches the semantic complexity — 4 semantic roles + 1 embedded relative clause — regardless of whether the surface form is one word (Mohawk) or eight words (English).

---

## APPENDIX: NOTATION REFERENCE

| Symbol | Meaning |
|---|---|
| ℝ² | Euclidean plane |
| ⊂ | subset |
| ∈ | element of |
| B(x, r) | open ball of radius r centered at x |
| ⊕ | Minkowski sum |
| σ_λ | uniform scaling by factor λ |
| ρ_c | reflection through vertical axis at centroid c |
| γ: [0,1] → ℝ² | parameterized path |
| ∘ | function composition |
| ConvexHull(S) | smallest convex set containing S |
| ∂F | boundary of region F |
| ≅ | similar (same shape, possibly different size/position) |
| Euc(2), Sim(2), Aff(2), Proj(2) | Euclidean, similarity, affine, projective transformation groups |
| Homeo(ℝ²) | group of homeomorphisms of ℝ² |
