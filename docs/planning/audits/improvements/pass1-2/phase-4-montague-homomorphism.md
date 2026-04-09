# Phase 4 — Montague Homomorphism

**Status:** ✅ COMPLETE (April 7, 2026)  
**Risk:** High (outcome is genuinely unknown)  
**New operations:** 0  
**D2 cases affected:** 0 directly (meta-level credibility)  
**Dependencies:** Phases 1A, 1B, 2, 3 (must map the final Σ_UL⁺)  
**Blocks:** Nothing (terminal phase)

---

## Problem Statement

The deepest open question about UL is whether it's a **discovery** (new mathematical structure with non-trivial properties) or a **notation** (geometric re-encoding of known formal semantics). The answer determines UL's intellectual contribution:

| If UL is... | Then its value is... | Evidence needed |
|---|---|---|
| **A discovery** | A new algebraic structure that adds geometric distance, topology, and visual concreteness to formal semantics — things Montague/predicate logic lack | Show that UL strictly subsumes Montague (injective homomorphism + surplus structure) |
| **A notation** | A geometric visual language for existing formal semantics — pedagogically useful but not mathematically novel | Show bijective correspondence with Montague (isomorphism with no surplus) |
| **Incomplete** | A partial system that can't express everything Montague can | Show that some Montague construction has no UL counterpart |

The way to settle this is to construct (or prove the impossibility of) an explicit **Σ_UL-homomorphism from Montague's type-logical grammar.**

---

## Background: Montague's System

### Montague's Intensional Logic (IL)

Richard Montague's "The Proper Treatment of Quantification in Ordinary English" (1973) defines:

**Types:**
```
e        — individuals (entities)
t        — truth values
⟨σ, τ⟩   — functions from σ to τ
s        — possible worlds (intensions)
```

**Key operations:**
1. **Function application:** f(a) where f : ⟨σ, τ⟩ and a : σ → result : τ
2. **Lambda abstraction:** λx.φ — creates a function from variable x in expression φ
3. **Quantifier raising:** moves quantified NPs to scope positions
4. **Intensionalization:** ˆα lifts an expression to its intension (function from worlds to extensions)
5. **Extensionalization:** ˇα evaluates an intension at the current world

### The Extensional Fragment

If we drop the `s` type (possible worlds), Montague's system reduces to the **extensional fragment:**

```
Types: e, t, ⟨σ, τ⟩
Operations: function application, lambda abstraction, Boolean connectives, quantification
```

This is equivalent to the simply-typed lambda calculus with product types — the standard framework for compositional formal semantics.

---

## Strategy: Map Extensional Montague to UL

### Type Correspondence

| Montague Type | UL Sort | Correspondence |
|---|---|---|
| e | e (Entity) | Direct — both represent individuals/things |
| t | a (Assertion) | Via the assertional sign σ ∈ {⊕, ⊖} ≅ {T, F} |
| ⟨e, t⟩ | — | One-place predicates. In UL: partially applied predicate (entity + relation waiting for second entity) |
| ⟨e, ⟨e, t⟩⟩ | r (Relation) | Two-place predicates = relations. This maps Montague's curried binary predicates to UL's relations |
| ⟨⟨e, t⟩, t⟩ | — | Generalized quantifiers. In UL: quantify(m, e) → a |
| ⟨⟨e, t⟩, ⟨e, t⟩⟩ | m (Modifier) | Property-to-property functions = modifiers |
| ⟨s, σ⟩ | **NO COUNTERPART** | Intensions (functions from worlds). UL has no world index. |

### Operation Correspondence

| Montague Operation | UL Operation(s) | Status |
|---|---|---|
| Function application | predicate(e, r, e) → a | ✅ Direct (for binary predicates) |
| Lambda abstraction | bind(e, a) → a (Phase 2) | ✅ After Phase 2 |
| Boolean NOT | negate(a) → a | ✅ Direct |
| Boolean AND | conjoin(a, a) → a | ✅ Direct |
| Boolean OR | disjoin(a, a) → a | ✅ Direct |
| Universal quantification | quantify(m_∀, e) → a | ✅ Direct |
| Existential quantification | quantify(m_∃, e) → a | ✅ Direct |
| Predicate modification | modify_entity(m, e) → e or modify_relation(m, r) → r | ✅ Direct |
| Relative clause formation | embed(a) → e (nominalization) + predication | ✅ Via embed |
| Passivization | invert(r) → r | ✅ Direct |
| Noun-to-adjective conversion | abstract(e) → m | ✅ Direct |
| Intensionalization (ˆ) | **NO COUNTERPART** | ❌ No world index |
| Extensionalization (ˇ) | **NO COUNTERPART** | ❌ No world index |

### Preliminary Assessment

**Extensional fragment:** All operations appear to have UL counterparts (especially after Phase 2 provides `bind` for lambda abstraction). The correspondence is structural, not merely notational — UL's geometric grounding adds **distance** (between entities), **curvature** (on relations), and **area** (for quantification) that Montague's type system lacks.

**Intensional fragment:** UL has no mechanism for possible worlds, intensions, or world-dependent evaluation. This is consistent with UL's declared scope ("compositional relational semantics," not modal logic).

---

## Formal Construction Plan

### Step 1: Define the Source Algebra

Let M_ext = the extensional fragment of Montague's IL, formalized as a Σ-algebra:

```
Sorts: e_M, t_M, and function types ⟨σ, τ⟩
Operations: app, λ, ∧, ∨, ¬, ∀, ∃
```

### Step 2: Define the Type Mapping

Construct a function τ from Montague types to UL sorts:

```
τ(e_M) = e
τ(t_M) = a
τ(⟨e_M, ⟨e_M, t_M⟩⟩) = r
τ(⟨⟨e_M, t_M⟩, ⟨e_M, t_M⟩⟩) = m
```

**Challenge:** Montague has infinitely many types (⟨σ, τ⟩ for all σ, τ). UL has 4 sorts. The mapping must either:
- (a) Show that all Montague types used in natural language reduce to the 4 UL sorts, or
- (b) Identify Montague types that don't reduce (counterexamples to the mapping)

### Step 3: Define the Operation Mapping

For each Montague operation, exhibit its UL counterpart as a composition of Σ_UL⁺ operations. The table above provides the map; this step makes it formal.

### Step 4: Verify Homomorphism Property

For the mapping φ to be a Σ-homomorphism, it must satisfy:

```
φ(op_M(x₁, ..., xₙ)) = op_UL(φ(x₁), ..., φ(xₙ))
```

for every Montague operation op_M. This is the compositionality condition — doing the Montague operation then mapping must equal mapping then doing the UL operation.

### Step 5: Check Injectivity

If φ is injective (distinct Montague expressions map to distinct UL constructions), then UL **strictly subsumes** Montague's extensional fragment (since UL has additional structure — geometric distance, frame topology — that Montague lacks).

If φ is not injective (some distinct Montague expressions collapse to the same UL construction), then UL is **coarser** than Montague for the affected expressions. The failure points identify what UL can't distinguish.

---

## Known Challenges

### C1: Higher-Order Types

Montague uses types like ⟨⟨e,t⟩, ⟨⟨e,t⟩, t⟩⟩ (determiner meanings). UL has no function-type constructor. The embedding must show these are DERIVABLE from the 4 sorts + operations.

**Expected resolution:** In practice, all Montague types used for natural language semantics reduce to combinations of e, r, m, a via UL operations. The type ⟨⟨e,t⟩, ⟨⟨e,t⟩, t⟩⟩ (determiners like "every," "some") maps to UL's `quantify` operation — the determiner IS the quantification modifier.

### C2: Lambda Abstraction vs. Bind

Montague's λ creates functions (higher-order objects). UL's `bind` creates scoped assertions (first-order). Are these equivalent?

**Expected analysis:** For the fragment of λ used in natural language semantics (primarily scope-taking and predicate abstraction), bind + embed provides equivalent expressive power without introducing function types. But general λ-abstraction (arbitrary function creation) exceeds bind's capabilities.

**Implication:** If the mapping works for the NL fragment but not for full IL, UL subsumes *the linguistically relevant portion* of Montague — which may be the honest claim.

### C3: Generalized Quantifiers

Montague's generalized quantifiers (type ⟨⟨e,t⟩, t⟩) include: "every," "some," "most," "few," "no," "exactly three," "more than half." After Phase 1B (graduated quantification), UL covers proportional quantifiers. Cardinality quantifiers remain a gap.

**Map:**

| Generalized Quantifier | UL Counterpart | Status |
|---|---|---|
| "every" | quantify(m₁, e) | ✅ |
| "some" | quantify(m_{0⁺}, e) | ✅ |
| "no" | negate(quantify(m_{0⁺}, e)) | ✅ |
| "most" | quantify(m_{0.7}, e) (Phase 1B) | ✅ after 1B |
| "few" | quantify(m_{0.1}, e) (Phase 1B) | ✅ after 1B |
| "exactly three" | Enumeration + negation | ⚠️ (no compact form) |
| "more than half" | quantify(m_{0.5+ε}, e) (Phase 1B) | ✅ after 1B |

---

## Possible Outcomes

### Outcome 1: Extensional Subsumption (Discovery)

The homomorphism φ exists, is injective, and UL's geometric algebra contains structure that Montague's type system doesn't have:

- **Distance** between entities (metric structure in ℝ²)
- **Curvature** of relations (path geometry)
- **Area** of quantification (frame-fill proportion)
- **Visual concreteness** (constructions are drawable; Montague's are abstract trees)

If this holds, UL is a **proper extension** of extensional Montague grammar — everything Montague can say, UL can say, PLUS UL has geometric structure that enables non-linguistic operations (similarity measurement, continuous deformation, spatial reasoning).

**Significance:** UL would be mathematically novel — not just a notation but a new kind of formal semantics where meaning has genuine geometric properties.

### Outcome 2: Extensional Equivalence (Notation)

The homomorphism φ exists and is bijective. UL and extensional Montague have the same expressive power, just different notation.

**Significance:** UL would be a pedagogically valuable geometric visualization of formal semantics. The writing system adds practical value (drawability, visual disambiguation) even if the algebra is equivalent.

### Outcome 3: Partial Failure (Informative Gap)

Some Montague constructions don't map. The failure points identify specific gaps in UL's expressiveness.

**Significance:** Directly actionable — each failure point becomes a Phase 5 extension target. This is the most valuable outcome for strengthening UL.

---

## Files to Create/Modify

| File | Change |
|---|---|
| `foundations/montague-homomorphism.md` | **NEW** — formal construction of the mapping |
| `foundations/formal-foundations.md` | Reference to the new document; update claims about universality |
| `ul-core/CRITIQUE.md` | Document the outcome (whichever it is) |
| `experiments/D2-structural-predictions.md` | Connect to structural prediction P3 (11 ops suffice) |

---

## Prerequisites

This phase CANNOT begin until:

1. ✅ Phase 1A (modifier conventions) — the modifier mapping needs concrete assignments
2. ✅ Phase 1B (graduated quantification) — the quantifier mapping needs the continuous parameter
3. ✅ Phase 2 (variable binding) — the lambda/bind correspondence is central to the proof
4. ✅ Phase 3 (assertion modification) — completes the operation inventory

The mapping must be against the **final** Σ_UL⁺ = {predicate, modify_entity, modify_relation, negate, conjoin (derived), disjoin, embed, abstract, compose, invert, quantify (extended), bind, modify_assertion}.

---

## Deliverable

A document `foundations/montague-homomorphism.md` containing:

1. Formal definition of the source algebra (extensional Montague fragment)
2. Type mapping τ : Types_M → Sorts_UL
3. Operation mapping φ : Ops_M → Ops_UL
4. Homomorphism proof (or counterexample)
5. Injectivity analysis
6. Honest assessment of which outcome obtained
7. Implications for UL's universality claims
