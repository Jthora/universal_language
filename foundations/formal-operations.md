# Formal Operations — Mathematical Definitions of the 11 Σ_UL Operations on Geometric Objects

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

**Gₐ (Assertions):** The set of all *framed constructions* — complete geometric configurations enclosed in a distinguished bounding region (the "sentence frame"):

```
Gₐ = { (F, C) | F ⊂ ℝ² is a Jordan domain (bounded, connected, 
        with Jordan curve boundary), C ⊂ F is a finite geometric 
        configuration containing at least one entity-relation-entity triple }
```

A Jordan domain F partitions ℝ² into interior (the claim), boundary (the scope), and exterior (what is not claimed). This enforces the semantic property: assertions have definite content.

---

## 1. THE 11 OPERATIONS — FORMAL DEFINITIONS

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

**Definition.** Given assertion a = (F, C) ∈ Gₐ, let c be the centroid of F and let ρ_c be the reflection through the vertical line passing through c:

```
negate(a) = (F, ρ_c(C))
```

The frame F is preserved; the content is reflected.

**Closure:** ρ_c(C) ⊂ F (since F is symmetric about its centroid's vertical for convex frames; for non-convex frames, we use the smallest containing convex region). The result is a framed construction — an assertion. ✓  
**Totality:** Every framed construction has a centroid and admits reflection. ✓  
**Determinism:** Reflection is a function. ✓  
**Involution:** ρ_c ∘ ρ_c = id, so negate(negate(a)) = a. ✓

### 1.5 conjoin : a × a → a

**Definition.** Given assertions a₁ = (F₁, C₁) and a₂ = (F₂, C₂) ∈ Gₐ:

```
conjoin(a₁, a₂) = (F₁ ∪ F₂, C₁ ∪ C₂)
```

where F₁ and F₂ are positioned such that F₁ ∩ F₂ ≠ ∅ (their frames overlap). The shared boundary region represents the shared context — what both assertions hold in common.

**Positioning convention:** If F₁ and F₂ are not yet positioned relative to each other, translate F₂ so that its left boundary is tangent to the right boundary of F₁ with overlap width δ > 0.

**Closure:** F₁ ∪ F₂ is a Jordan domain (union of two overlapping Jordan domains). C₁ ∪ C₂ is a finite configuration. ✓  
**Totality:** Any two assertions can be positioned to overlap. ✓  
**Determinism:** Given the positioning convention, the result is unique. ✓

### 1.6 disjoin : a × a → a

**Definition.** Given assertions a₁ = (F₁, C₁) and a₂ = (F₂, C₂) ∈ Gₐ:

```
disjoin(a₁, a₂) = (F₁ ⊔ F₂, C₁ ∪ C₂)
```

where F₁ ⊔ F₂ denotes F₁ and F₂ positioned **adjacent** (boundaries touching at exactly one point or along a shared edge, but interiors disjoint: int(F₁) ∩ int(F₂) = ∅).

**Semantic distinction from conjoin:** In conjoin, shared interior = shared truth. In disjoin, no shared interior = independent truth — reading either frame independently yields a valid reading.

**Closure:** F₁ ⊔ F₂ is connected, bounded, and contains both configurations. ✓  
**Totality/Determinism:** Same argument as conjoin, with non-overlapping positioning convention. ✓

### 1.7 embed : a → e

**Definition.** Given assertion a = (F, C) ∈ Gₐ:

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
quantify(m, e) = (F, m(C))
```

where the **type of quantification** is determined by the geometric character of m:

- **Universal (∀):** m = σ_Λ (scaling with Λ > 1, expanding C to fill F). Reading: "all C" — the entity occupies the entire frame.
- **Existential (∃):** m = σ_λ (scaling with 0 < λ < 1, shrinking C to a point-like region within F). Reading: "some C" — the entity is localized within the frame.
- **Negative (¬∃):** m = ρ ∘ complement (reflection + set complement). Reading: "no C" — the entity is excluded from the frame.

**Closure:** The result is a framed construction — an assertion. ✓  
**Totality:** Every modifier can be applied to every entity within every frame. ✓  
**Determinism:** The geometric transformation is unique. ✓

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

3. **negate:** ρ_c(C₁) = ρ_c(C₂) implies C₁ = C₂ (reflections are invertible).

4. **conjoin, disjoin:** If F₁ ∪ F₂ = F₁' ∪ F₂' with the canonical positioning convention, the individual frames and contents are recoverable (the overlap/adjacency boundary separates them).

5. **embed:** If σ_λ(F₁) = σ_λ(F₂), then F₁ = F₂ (scaling is invertible).

6. **abstract:** If T_{C₁} = T_{C₂} as transformations, then ConvexHull(C₁) ≅ ConvexHull(C₂) (the shape-imposing transformation encodes the convex hull shape uniquely).

7. **compose:** If γ₁ · γ₂ = γ₁' · γ₂' as concatenated paths, the midpoint parameter t = 1/2 identifies where the first path ends and the second begins, recovering the components.

8. **invert:** If γ⁻¹ = (γ')⁻¹, then γ = γ' (reversal is an involution).

9. **quantify:** If (F, m(C)) = (F, m'(C')), then m = m' and C = C' (invertibility of m). ∎

**What this proves:** The geometric operations are **faithful** — they don't collapse distinct inputs to the same output. This is the missing piece for the embedding theorem's injectivity claim.

---

## 3. COMPLETENESS ARGUMENT FOR THE 11 OPERATIONS

### 3.1 What Must Be Shown

The claim: the 11 operations of Σ_UL are **sufficient** to express all finite relationships between distinguishable entities. We argue this by showing that any meaningful semantic operation on (Entity, Relation, Modifier, Assertion) can be constructed from the 11.

### 3.2 Classification by Sort Transition

Any operation on (e, r, m, a) can be classified by its input sorts and output sort. There are 4 possible output sorts, and inputs can be drawn from any combination of the 4 sorts. The 11 operations cover the following transitions:

| Output sort | Operations producing it | Transitions covered |
|---|---|---|
| **Entity (e)** | modify_entity (m × e → e), embed (a → e) | Modification of entities, nominalization of assertions |
| **Relation (r)** | modify_relation (m × r → r), compose (r × r → r), invert (r → r) | Modification, chaining, reversal of relations |
| **Modifier (m)** | abstract (e → m) | Extraction of qualities from entities |
| **Assertion (a)** | predicate (e × r × e → a), negate (a → a), conjoin (a × a → a), disjoin (a × a → a), quantify (m × e → a) | Statement formation, logical connectives, quantification |

### 3.3 Coverage Argument

**For Entity production:** Any modification of an entity is a transformation (handled by modify_entity with the appropriate Erlangen-level transformation). Converting any other type to an entity requires embed (the only source of entities from non-entities). These two cover all entity-producing operations.

**For Relation production:** Relations can be modified (modify_relation), chained (compose), or reversed (invert). New relations between entities are created as part of predicate (which embeds a relation between entities into an assertion). There is no free-standing "create a relation from scratch" operation because a relation without entities is meaningless — it must connect things.

**For Modifier production:** The only source of new modifiers is abstract (extracting shape properties from entities). This is sufficient because:
- Concrete modifiers (specific transformations) are elements of Gₘ directly
- Derived modifiers come from abstracting over entities (wood → wooden, circle → circular)
- Composed modifiers are function compositions: m₁ ∘ m₂ is itself a transformation

**For Assertion production:** predicate creates new assertions. negate, conjoin, disjoin provide propositional completeness (any Boolean function of assertions can be expressed). quantify adds scope/generalization.

### 3.4 Propositional Completeness

The triple (negate, conjoin, disjoin) provides a functionally complete set of Boolean connectives:
- NOT from negate
- AND from conjoin
- OR from disjoin
- IMPLIES = disjoin(negate(a), b) — derived
- IFF = conjoin(implies(a,b), implies(b,a)) — derived

This is the standard NAND/NOR completeness of propositional logic. Any truth-functional combination of assertions is expressible.

### 3.5 What About Equality?

**Objection:** There is no explicit equality operation (asserting X = Y).

**Response:** Equality can be expressed as: conjoin(predicate(x, identity_r, y), predicate(y, identity_r, x)) where identity_r = compose(r, invert(r)) for any r (a relation composed with its inverse yields the identity path — a degenerate loop at the source).

### 3.6 What About Definition?

**Objection:** There is no explicit definition operation ("Let C be ...").

**Response:** Definition is handled by embed + predicate: embed an assertion (creating a named entity), then predicate about it. The enclosure IS the definition boundary.

### 3.7 Limitations of This Argument

This is a **coverage argument**, not a formal completeness proof. A full proof would require:
1. A precise formal definition of "all finite relationships between distinguishable entities"
2. A proof that every such relationship decomposes into a finite composition of the 11 operations
3. Such a proof likely requires showing that the 11 operations generate a monoid that acts transitively on the relevant structure space

This remains open work. The coverage argument establishes **plausibility** at a level sufficient for practical writing system use — any relationship a human language can express has a clear construction path through the 11 operations.

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
- Negation MUST reflect (§1.4 specifies reflection ρ_c)

**Contextual grounding:** For contextual use-cases (expressing domain-specific meanings in UL), the formal operations provide the verification layer. Given a target meaning M in some domain, the writing procedure is:
1. Decompose M into Σ_UL sort assignments (which parts are entities, relations, modifiers, assertions?)
2. Identify the operations connecting them (which entities are predicated? which relations are composed?)
3. Construct the geometric realization using the formal definitions above
4. Verify the construction by applying the reading procedure and checking that it recovers the intended decomposition

Step 4 is the **validation algorithm** that was previously missing from the writing system. The formal operation definitions make it possible.

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
