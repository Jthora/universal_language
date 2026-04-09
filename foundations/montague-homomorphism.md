# Montague Homomorphism — Formal Mapping from Extensional Montague Grammar to Σ_UL⁺

**Research Series:** Universal Language  
**Date:** April 7, 2026  
**Prerequisites:** `formal-foundations.md`, `formal-operations.md`, Phase 2 (`bind`), Phase 3 (`modify_assertion`)  
**Purpose:** Construct an explicit structure-preserving map from the extensional fragment of Montague's Intensional Logic to Σ_UL⁺ and determine whether UL is a discovery (proper extension), a notation (equivalence), or incomplete (gaps).

---

## 1. SOURCE ALGEBRA: EXTENSIONAL MONTAGUE FRAGMENT

### 1.1 Types

The extensional fragment of Montague's IL uses:

```
e       — individuals (entities)
t       — truth values {T, F}
⟨σ, τ⟩  — functions from type σ to type τ
```

**Key derived types:**
| Type | Notation | Linguistic Role |
|---|---|---|
| e | e | Individuals: "John," "the cat" |
| t | t | Truth values |
| ⟨e, t⟩ | VP/NP-predicate | One-place predicates: "runs," "is tall" |
| ⟨e, ⟨e, t⟩⟩ | TV | Two-place predicates: "loves," "reads" |
| ⟨⟨e, t⟩, t⟩ | GQ | Generalized quantifiers: "every student," "some book" |
| ⟨⟨e, t⟩, ⟨e, t⟩⟩ | ADJ/ADV | Predicate modifiers: "red," "quickly" |
| ⟨t, t⟩ | NEG | Sentential operators: "not" |
| ⟨t, ⟨t, t⟩⟩ | CONN | Sentential connectives: "and," "or" |

### 1.2 Operations

| Operation | Signature | Definition |
|---|---|---|
| **app** | ⟨α, β⟩ × α → β | Function application: f(a) |
| **λ** | (variable x : α, body : β) → ⟨α, β⟩ | Lambda abstraction: λx.φ |
| **¬** | t → t | Boolean negation |
| **∧** | t × t → t | Boolean conjunction |
| **∨** | t × t → t | Boolean disjunction |
| **∀** | ⟨e, t⟩ → t | Universal quantification: ∀x.P(x) |
| **∃** | ⟨e, t⟩ → t | Existential quantification: ∃x.P(x) |

We denote this algebra **M_ext** = (Types_M, Ops_M).

---

## 2. TYPE MAPPING τ : Types_M → Sorts_UL

### 2.1 Base Mapping

```
τ(e)                    = e (Entity)
τ(t)                    = a (Assertion)
τ(⟨e, ⟨e, t⟩⟩)         = r (Relation)
τ(⟨⟨e, t⟩, ⟨e, t⟩⟩)   = m (Modifier)
```

### 2.2 Derived Type Reductions

Montague types beyond the base cases reduce to UL sorts via operational encoding:

| Montague Type | UL Encoding | Justification |
|---|---|---|
| ⟨e, t⟩ (VP) | Partially saturated predicate: bind one slot in `pred(e_x, r, e_y)` | A one-place predicate = a relation with one entity already fixed. In UL: `bind(e_y, pred(e_x, r, e_y))` produces an assertion parameterized by x — equivalent to λx.R(x,y). The "type" is reconstructed via bind + embed, not stored as a sort. |
| ⟨⟨e, t⟩, t⟩ (GQ) | `quantify(m_p, e)` | Generalized quantifiers are not a separate type in UL. They arise from the quantify operation applied to an entity with frame-fill parameter p. "Every student" = `quantify(m₁, e_student)`. |
| ⟨t, t⟩ (NEG) | `negate` | The sentential negation operator. Not a separate UL sort — it's an operation. |
| ⟨t, ⟨t, t⟩⟩ (CONN) | `conjoin`, `disjoin` | Sentential connectives. Operations, not objects. |
| ⟨s, σ⟩ (intension) | **NO COUNTERPART** | Possible worlds not in UL's scope. |

### 2.3 Type Reduction Theorem

**Claim.** Every Montague type used in the extensional semantics of natural language sentences reduces to one of {e, r, m, a} via the following rules:

1. τ(e) = e
2. τ(t) = a
3. τ(⟨e, ⟨e, t⟩⟩) = r
4. τ(⟨⟨e, t⟩, ⟨e, t⟩⟩) = m
5. τ(⟨e, t⟩) = via bind → embedded in sort a
6. τ(⟨⟨e, t⟩, t⟩) = via quantify → embedded in sort a
7. τ(⟨α, β⟩) for higher types: encoded via operation composition (embed, abstract, bind)

**Caveat:** This is NOT a bijection. Multiple distinct Montague types collapse to the same UL sort. In particular, ⟨e, t⟩ and ⟨⟨e, t⟩, t⟩ both produce a-sort results via different operations. The type distinction is captured by the OPERATION used (bind vs. quantify), not by a sort distinction. This is a key structural difference: Montague encodes information in types; UL encodes it in operations + geometry.

---

## 3. OPERATION MAPPING φ : Ops_M → Ops_UL

### 3.1 Core Mapping Table

| Montague Operation | UL Operation(s) | Signature Correspondence |
|---|---|---|
| app(f : ⟨e, ⟨e, t⟩⟩, a : e, b : e) | predicate(a, f, b) | ⟨e, ⟨e, t⟩⟩ × e × e → t  ↦  e × r × e → a |
| λx.φ | bind(e_x, φ) | (var, body) → function  ↦  e × a → a |
| ¬φ | negate(φ) | t → t  ↦  a → a |
| φ ∧ ψ | conjoin(φ, ψ) | t × t → t  ↦  a × a → a |
| φ ∨ ψ | disjoin(φ, ψ) | t × t → t  ↦  a × a → a |
| ∀(P : ⟨e, t⟩) | quantify(m₁, embed(bind(e_x, P(x)))) | ⟨e, t⟩ → t  ↦  m × e → a |
| ∃(P : ⟨e, t⟩) | quantify(m_{0⁺}, embed(bind(e_x, P(x)))) | ⟨e, t⟩ → t  ↦  m × e → a |
| ADJ(P : ⟨e,t⟩) | modify_entity(abstract(e_adj), e) | ⟨⟨e,t⟩, ⟨e,t⟩⟩ × ⟨e,t⟩ → ⟨e,t⟩  ↦  m × e → e |
| APP(f, a) [1-place] | embed + predicate | ⟨e, t⟩ × e → t  ↦  via composition |

### 3.2 Detailed Correspondences

**3.2.1 Function Application → Predicate**

Montague: `app(LOVE, john, mary) = LOVE(john)(mary) : t`

UL: `predicate(e_john, r_love, e_mary) → a₁`

The curried application app(app(LOVE, mary), john) maps to UL's ternary predicate. UL collapses two applications into one operation — this is valid because binary predication is the atomic unit in both systems.

**3.2.2 Lambda Abstraction → Bind**

Montague: `λx. LOVE(x, mary) : ⟨e, t⟩` — a function from entities to truth values.

UL: `bind(e_x, predicate(e_x, r_love, e_mary))` — an assertion with x as a bound variable.

**Key difference:** Montague's λ produces a FUNCTION (a new object of type ⟨e, t⟩). UL's bind produces an ASSERTION (sort a) with a bound variable. The function-hood is NOT preserved as a first-class object in UL.

**Recovery:** When UL needs to "apply" the bound assertion to a specific entity, it uses embed + predicate:
```
bind(e_x, pred(e_x, r_love, e_mary)) → a₁  [assertion with x bound]
# To apply to john: substitute e_john for e_x in the pre-bind form:
pred(e_john, r_love, e_mary) → a₂
```

This is NOT the same as Montague's app(λx.LOVE(x,m), j) semantically, because UL doesn't create an intermediate function object. The substitution happens at the construction level, not the object level.

**Assessment:** For the purpose of natural language semantics, this distinction is immaterial — both systems produce the same final assertion. But UL cannot express arbitrary lambda terms as first-class objects. This is a **genuine structural difference**, not just notation.

**3.2.3 Quantification**

Montague: `∀(λx.P(x)) : t` — applies the universal quantifier to a property (type ⟨e, t⟩ → t).

UL:
```
bind(e_x, P(e_x)) → a₁        # x bound in P
quantify(m₁, embed(a₁)) → a₂   # universally quantify the bound assertion
```

The two-step UL process (bind + quantify) maps faithfully to Montague's one-step ∀(property). The intermediate bind makes co-reference explicit; Montague's λ does the same implicitly.

**3.2.4 Sentential Connectives**

Direct correspondence:
```
φ(¬α)   = negate(φ(α))
φ(α ∧ β) = conjoin(φ(α), φ(β))
φ(α ∨ β) = disjoin(φ(α), φ(β))
```

These are structure-preserving by inspection.

**3.2.5 Predicate Modification → modify_entity/modify_relation**

Montague: `ADJ(P)(x) = ADJ(P)(x)` where ADJ : ⟨⟨e,t⟩, ⟨e,t⟩⟩ — intersective case: `(λP.λx. ADJ(x) ∧ P(x))`

UL: `modify_entity(abstract(e_adj), e_x)` — apply the modifier derived from the adjective-entity to the target entity.

For **intersective** adjectives ("red," "tall"), the Montague treatment (conjunction of properties) and UL treatment (geometric transformation) produce extensionally equivalent results. For **non-intersective** adjectives ("alleged," "former"), both systems require additional machinery — Montague uses intensional types ⟨s, ...⟩, UL uses modify_assertion or meta-level predication.

---

## 4. HOMOMORPHISM VERIFICATION

### 4.1 The Homomorphism Condition

For φ : M_ext → Σ_UL⁺ to be a Σ-homomorphism, for every operation op in M_ext:

```
φ(op_M(x₁, ..., xₙ)) = op_UL(φ(x₁), ..., φ(xₙ))
```

### 4.2 Verification by Operation

**4.2.1 Negation:** φ(¬α) = negate(φ(α)). ✓ (direct structural correspondence)

**4.2.2 Conjunction:** φ(α ∧ β) = conjoin(φ(α), φ(β)). ✓

**4.2.3 Disjunction:** φ(α ∨ β) = disjoin(φ(α), φ(β)). ✓

**4.2.4 Binary predication:** φ(app(app(R, b), a)) = predicate(φ(a), φ(R), φ(b)). ✓
- Note the argument order: Montague's curried application applies object first, then subject. The reordering is structural (fixed by the mapping) and doesn't affect the homomorphism property.

**4.2.5 Universal quantification:**
φ(∀(λx.P(x))) = quantify(m₁, embed(bind(e_x, φ(P(e_x)))))

This is a **composite** mapping — Montague's single ∀ maps to UL's quantify ∘ embed ∘ bind chain. The homomorphism condition requires:

```
φ(∀(α)) = quantify(m₁, embed(bind(e_x, φ(α(e_x)))))
```

For this to be a homomorphism, we must verify that the composition is well-defined and unique for each input. It is: bind produces a unique assertion (determinism), embed produces a unique entity (determinism), quantify produces a unique assertion (determinism). The chain is deterministic. ✓

**But:** The mapping is not a **simple** homomorphism (one-to-one operation correspondence). It is a **derived** homomorphism — Montague's ∀ maps to a UL operation CHAIN, not a single UL operation. This is acceptable in algebra (derived operations are legitimate structure-preserving maps) but it means UL's operation decomposition of quantification is FINER than Montague's.

**4.2.6 Existential quantification:** Same structure with m_{0⁺}. ✓

**4.2.7 Lambda abstraction:**
φ(λx.α) ≈ bind(e_x, φ(α))

**CAUTION:** This is the weakest point. Montague's λx.α produces a FUNCTION OBJECT of type ⟨τ(x), τ(α)⟩. UL's bind produces an ASSERTION. These are different sorts.

The mapping is valid ONLY when λ is immediately consumed by quantification or application (as it always is in the Montague fragment for natural language). For "dangling" lambdas (function objects passed around as first-class values), UL's bind cannot produce the equivalent.

**Formal statement:** φ preserves λ-abstraction for the **β-normal fragment** — every λ that is immediately β-reduced (applied to an argument or consumed by a quantifier). For unreduced λ-terms, the mapping does not extend.

**Assessment:** In natural language semantics, every λ-term in a complete derivation is eventually β-reduced to produce a truth value. There are no "dangling" lambdas in the final semantic representation of a sentence. Therefore, the restriction to the β-normal fragment does not lose any sentences — it only loses intermediate computational steps.

### 4.3 Homomorphism Result

**Theorem (Extensional Montague Embedding).** There exists a structure-preserving map φ : M_ext → Σ_UL⁺ that is:

1. **Well-defined** on all complete Montague derivations of natural language sentences in the extensional fragment.
2. **Compositional:** φ preserves the tree structure of Montague derivations — the UL construction mirrors the Montague derivation step by step (with λ/bind identification).
3. **Correct:** For every sentence S with Montague denotation ⟦S⟧_M ∈ {T, F}, the UL construction φ(S) has assertional sign σ corresponding to ⟦S⟧_M.

**Qualifications:**
- φ maps Montague's λ to UL's bind, which is valid only for the β-normal fragment. This covers all complete sentence derivations but not arbitrary λ-terms as standalone objects.
- φ is a **derived homomorphism** — some single Montague operations map to chains of UL operations (notably ∀ → quantify ∘ embed ∘ bind). This makes UL's decomposition finer, not coarser.
- The intensional fragment (⟨s, σ⟩ types) is entirely outside the map. ∎

---

## 5. INJECTIVITY ANALYSIS

### 5.1 Is φ Injective?

**Question:** Do distinct Montague expressions always map to distinct UL constructions?

**For well-typed complete derivations:** Yes. Two Montague derivations that produce different denotations (different truth conditions) will produce UL constructions with different geometric structure:

1. Different entities → different marks at different positions
2. Different relations → different directed paths with different angle/curvature
3. Different truth values → different assertional signs (⊕ vs. ⊖)
4. Different scope orderings → different bind nesting depths
5. Different quantificational force → different frame-fill proportions (p values)

**For type-distinct but extensionally equivalent expressions:** Montague distinguishes `λx.P(x)` (type ⟨e,t⟩) from `P` (type ⟨e,t⟩) — they're intensionally different but extensionally equivalent. UL does NOT distinguish these (both map to the same construction). This is an INTENDED collapse — UL is extensional.

### 5.2 Injectivity Verdict

φ is **injective on the extensional denotation level** — distinct truth conditions produce distinct UL constructions.

φ is **not injective on the intensional level** — Montague's type-based distinctions between extensionally equivalent expressions collapse.

---

## 6. SURPLUS STRUCTURE IN UL

### 6.1 What UL Has That Montague Doesn't

UL's geometric algebra carries structure with no counterpart in Montague's type-logical system:

| UL Structure | Montague Counterpart | Type of Surplus |
|---|---|---|
| **Metric distance** between entities (d(e₁, e₂) in ℝ²) | None — entities are unstructured atoms | Geometric |
| **Curvature** of relations (path geometry) | None — relations are set-theoretic functions | Geometric |
| **Area** of quantification (frame-fill proportion p) | None — quantifiers are ⟨⟨e,t⟩, t⟩ functions | Geometric |
| **Frame topology** (Jordan domain, boundary shape) | None — truth values have no spatial structure | Topological |
| **Visual concreteness** (every construction is drawable) | Not drawable — abstract type-theoretic trees | Representational |
| **Assertion modification** (frame decoration) | No counterpart — truth values are bare {T, F} | Metalinguistic |
| **Continuous quantification** (p ∈ [0,1]) | Only ∀ and ∃ in standard fragment | Quantificational |
| **compose** (relation chaining without entities) | Requires λ-abstraction + application | Operational |
| **abstract** (entity → modifier) | Requires type-shifting rule | Operational |
| **invert** (relation reversal) | Requires passivization rule | Operational |

### 6.2 What Montague Has That UL Doesn't

| Montague Structure | UL Counterpart | Gap Type |
|---|---|---|
| **Possible worlds** (type s) | None | Modal/Intensional |
| **Intensions** (⟨s, σ⟩) | None | Modal/Intensional |
| **Higher-order functions** as first-class objects | Only via operation composition | Computational |
| **Type polymorphism** (infinite type hierarchy) | 4 fixed sorts | Type-theoretic |
| **Exact cardinality** (via counting in the model) | Only enumeration (O(n)) | Arithmetic |

---

## 7. CLASSIFICATION

### 7.1 Result: Extensional Subsumption (with qualification)

The mapping φ establishes that UL **strictly subsumes** the extensional fragment of Montague grammar:

```
M_ext ↪ Σ_UL⁺ (injective on extensional denotations)
```

with the following qualifications:

1. **λ/bind restriction:** The map identifies Montague's λ-abstraction with UL's bind for β-normal terms only. This covers all complete NL derivations but not arbitrary higher-order computation.

2. **Derived operations:** Some single Montague operations decompose into UL operation chains. This makes φ a derived (not simple) homomorphism.

3. **Surplus is genuine:** UL's geometric structure (distance, curvature, area, topology) has no Montague counterpart. These properties are operational — they support similarity measurement, continuous deformation, and spatial reasoning that pure type logic cannot express.

4. **Intensional gap persists:** UL has no mechanism for possible worlds, counterfactuals, or intensional contexts. The full Montague IL (with type s) is NOT subsumed.

### 7.2 What This Means for UL's Claims

| Claim | Status |
|---|---|
| "UL is expressively complete for compositional relational semantics" | **SUPPORTED** — UL subsumes the extensional fragment, which IS compositional relational semantics |
| "UL is unique up to isomorphism" | **UNCHANGED** — the Unique Grounding Theorem is about Σ_UL-algebras, not about comparison to other formalisms |
| "UL is a new mathematical structure" | **SUPPORTED** — the surplus geometric structure (§6.1) is not derivable from Montague's type system. UL is not merely Montague in geometric clothing. |
| "UL subsumes all of formal semantics" | **NOT SUPPORTED** — the intensional fragment remains outside scope |

### 7.3 Honest Summary

UL is a **proper geometric extension of extensional Montague grammar**. It embeds everything Montague's extensional fragment can express, adds genuine geometric structure (distance, curvature, area, frame topology) with no type-theoretic equivalent, and decomposes Montague's operations into finer algebraic pieces. It does NOT subsume the intensional fragment (possible worlds, counterfactuals). The lambda/bind identification works for all complete natural language derivations but not for arbitrary higher-order computation.

This places UL in the space of **cognitively grounded formal semantics** — it has the rigor of Montague's algebra WITH the additional constraint that every construction must be geometrically realizable. This constraint is simultaneously a limitation (no possible worlds) and a feature (every meaning is drawable, measurable, and continuously deformable).

---

## 8. IMPLICATIONS FOR FUTURE WORK

1. **Intensional extension:** To subsume full Montague IL, UL would need a "world index" mechanism. Possible geometric model: a STACK of planes (each plane = one world), with cross-plane relations encoding accessibility. This would add a 5th geometric primitive or extend the existing frame structure.

2. **Higher-order extension:** To fully capture λ-calculus as a first-class mechanism, UL would need to promote bind's output to an entity-like sort that can be passed as an argument. The current embed operation partially achieves this (assertions become entities), but the FUNCTION abstraction (not just the result) would need reification.

3. **Cardinality:** Both UL and extensional Montague rely on model-level counting for exact numerals. Neither has a built-in arithmetic. This is a shared limitation, not a UL-specific gap.

---

## REFERENCES

- Montague, R. (1973). "The Proper Treatment of Quantification in Ordinary English." In Hintikka et al. (eds.), *Approaches to Natural Language*.
- Dowty, D., Wall, R., & Peters, S. (1981). *Introduction to Montague Semantics*. Springer.
- Partee, B., ter Meulen, A., & Wall, R. (1990). *Mathematical Methods in Linguistics*. Springer.
