# Phase 1 — Kripke Realization Plan

**Goal:** Define possible worlds, accessibility relations, and modal truth conditions within the geometric framework of Σ_UL.

---

## 1. Possible Worlds in Glyph Space

### Definition (World as embedded assertion-collection)

A **possible world** is an entity obtained by embedding a maximally consistent conjunction of assertions:

```
w = embed(conjoin(a₁, conjoin(a₂, conjoin(a₃, ...))))
```

where {a₁, a₂, a₃, ...} is a maximal consistent set of assertions.

**Geometric realization:** A world is an enclosure in glyph space containing a complete specification — every atomic predication is either asserted (⊕) or denied (⊖) within the enclosure.

**Notation:** World-enclosures are drawn with double borders to distinguish them from concept-enclosures.

### Challenges

1. **Maximality** is not first-order: we cannot express "for all atomic a, either a ∈ w or negate(a) ∈ w" using Σ_UL operations alone. This is a meta-level property.
   - **Resolution path:** Accept maximality as a constraint on the semantic model (Kripke frame), not on the algebra. Σ_UL expressions can reference worlds without constructing them axiomatically.

2. **Infinite conjunctions:** A world contains infinitely many assertions. `conjoin` is binary.
   - **Resolution path:** Use `bind` for universal quantification: "for all propositions p, w assigns truth/falsity to p." Alternatively, treat worlds as abstract entities (distinguished elements of Gₑ) without requiring explicit construction.

### Fallback: Abstract worlds

If the constructive approach (worlds via `embed(conjoin(...))`) proves too cumbersome:

Define **abstract world-entities** as distinguished elements of Gₑ with the property that for any assertion a, `predicate(w, r_satisfies, embed(a))` or `predicate(w, r_satisfies, embed(negate(a)))` holds.

This treats worlds as entities we predicate about, rather than entities we construct.

---

## 2. Accessibility Relations

### Definition

An **accessibility relation** is a distinguished relation r_acc ∈ Gᵣ connecting world-entities.

```
predicate(w₁, r_acc, w₂)    # w₂ is accessible from w₁ via r_acc
```

### Modal Flavors as Accessibility Constraints

| Modal Flavor | Accessibility Relation | Algebraic Properties | Frame Condition |
|-------------|----------------------|---------------------|-----------------|
| Alethic | r_alethic | Reflexive, transitive, symmetric | Universal (S5) |
| Epistemic | r_K_agent | Reflexive, transitive | Partitional (S4) |
| Deontic | r_O | Serial | At least one ideal world exists (KD) |
| Ability/Dynamic | r_ability_agent | — | Agent-relative |
| Temporal | r_future / r_past | r_future: transitive, dense | Linear or branching |

Each flavor is a distinguished element of Gᵣ with algebraic constraints expressed as axioms on the Kripke frame.

### Geometric Realization

```
    ╔═══════════╗         ╔═══════════╗
    ║  World w₁ ║──r_K──→║  World w₂ ║
    ║  a holds  ║         ║  a holds  ║
    ╚═══════════╝         ╚═══════════╝
         │                      │
         │ r_K                  │ r_K
         ▼                      ▼
    ╔═══════════╗         ╔═══════════╗
    ║  World w₃ ║         ║  World w₄ ║
    ║  a holds  ║         ║  ¬a holds ║
    ╚═══════════╝         ╚═══════════╝

    □_K(a) = TRUE in w₁   (a holds in w₁, w₂, w₃ — all K-accessible)
    ◇_K(a) = TRUE in w₁   (a holds in at least one K-accessible world)
    □_K(a) = FALSE: w₄ is K-accessible from w₂ and ¬a in w₄
```

---

## 3. Modal Truth (Satisfaction)

### Definition (Truth in a world)

An assertion a is **true in world w** (written `w ⊨ a`) if:

```
predicate(w, r_satisfies, embed(a))     has σ = ⊕
```

This uses:
- `embed(a)` — turns the assertion into an entity (nominalization)
- `r_satisfies` — a distinguished relation "makes true"
- `predicate(...)` — forms the satisfaction assertion

### Definition (Necessity)

```
w ⊨ □_R(a)   iff   ∀w'. predicate(w, r_R, w') → w' ⊨ a
```

In UL operations:
```
necessary(r_R, a) =
  bind(w', quantify(m_∀, w',
    predicate(w_current, r_R, w') → predicate(w', r_satisfies, embed(a))
  ))
```

where `→` is material implication (expressible as `disjoin(negate(antecedent), consequent)`).

### Definition (Possibility)

```
w ⊨ ◇_R(a)   iff   ∃w'. predicate(w, r_R, w') ∧ w' ⊨ a
```

**Derived from necessity:**
```
possible(r_R, a) = negate(necessary(r_R, negate(a)))
```

### Definition (Counterfactual)

```
w ⊨ (a □→ b)   iff   b holds in the closest R-accessible world(s) where a holds
```

**Requires:** A metric d(w₁, w₂) on world-space measuring "closeness."

**In UL terms:**
```
counterfactual(a, b) =
  bind(w_closest,
    conjoin(
      predicate(w_closest, r_closest_satisfying, embed(a)),
      predicate(w_closest, r_satisfies, embed(b))
    )
  )
```

where `r_closest_satisfying` is a distinguished relation incorporating the metric.

**Dependency:** The closeness metric could be the Fisher information metric from Expedition Two, applied to world-space.

---

## 4. Connection to Gauge Bundle

The gauge bundle framework (Expedition One) defines:
- **Base space X** = context space
- **Fiber G** = meaning group
- **Connection** = how meaning transforms along context paths

**Modal extension:** Expand the base space:
```
X_modal = X × W
```
where W is the set of possible worlds. A point in X_modal specifies both a context AND a world. The gauge connection on X_modal determines how meanings transform across worlds.

**Accessibility as connection:** r_acc determines which paths in W are "meaningful" — you can only parallel-transport along accessible paths.

This formalization waits until Phase 1 basic algebra is settled.

---

## 5. Formal Checks Required

Before declaring Phase 1 complete, verify:

1. **Sort closure:** All modal constructions produce well-sorted terms
2. **Negation duality:** `possible(R, a) = negate(necessary(R, negate(a)))` type-checks and has correct semantics
3. **Composition:** Stacked modals (`necessary(R₁, possible(R₂, a))`) are well-formed
4. **Interaction with existing operations:** How does `modify_assertion(m, necessary(R, a))` behave? (e.g., "It is evidently necessary that...")
5. **Independence:** If new operations are added, prove independence from existing 13
6. **Embedding preservation:** The Montague homomorphism extends to modal constructions, or characterize the gap
