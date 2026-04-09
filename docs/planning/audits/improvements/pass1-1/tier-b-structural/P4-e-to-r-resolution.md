# P4 — The e → r Gap: Resolution

**Finding:** F4 (Missing sort transition)  
**Question:** B3 (Why no e → r?)  
**Dependency:** P1 (independence analysis must be current)  
**Date:** April 2026

---

## 1. Problem Recap

The sort-transition graph of Σ_UL's 11 operations has four missing arrows:

| Missing | Linguistic Name | Example |
|---------|----------------|---------|
| **e → r** | Denominalization | "hammer" → "to hammer" |
| m → r | Manner-to-action | "gentle" → "to gentle" |
| a → r | Proposition-to-action | "X is true" → "to verify X" |
| r → m | Action-to-manner | "to run" → "runningly" |

B3 asks: is the absence of e → r a principled structural constraint or an accidental gap?

---

## 2. Non-Derivability (Confirmed)

B3 already showed three derivation attempts fail. Here's the definitive argument:

**Theorem.** No finite composition of the 11 Σ_UL operations maps sort e to sort r.

**Proof.** Enumerate the output sorts of all 11 operations:

| Output sort | Producing operations |
|---|---|
| e | modify_entity, embed |
| r | modify_relation, compose, invert |
| m | abstract |
| a | predicate, negate, conjoin, disjoin, quantify |

Starting from a single input of sort e, the reachable sorts are:
- e → m (via abstract)
- e → e (via modify_entity, with a modifier)
- m × e → e (via modify_entity) — still e
- m × e → a (via quantify) — reaches a
- m × r → r (via modify_relation) — requires r input

To reach sort r from sort e alone, we need an operation that either:
(a) takes e (or something reachable from e alone) as input and produces r, OR
(b) chains operations from e to eventually reach r.

The only r-producing operations all require r in their input:
- modify_relation: needs (m, **r**)
- compose: needs (**r**, **r**)
- invert: needs (**r**)

Sort r appears only as OUTPUT of r-consuming operations. No operation introduces r from non-r inputs. Therefore, sort r is unreachable from sort e without presupposing an element of sort r. ∎

**Corollary.** Sorts m → r and a → r are also non-derivable, by the same argument: no operation produces r from non-r input.

---

## 3. Is the Absence Principled?

### 3.1 The Geometric Argument FOR the Gap

In the geometric model:
- **Entities** are compact subsets of ℝ² (position, shape, occupation of space)
- **Relations** are directed paths in ℝ² (trajectory, connection between two spatial positions)

A single compact set has **no intrinsic directionality**. It occupies space but doesn't connect anything. Constructing a directed path requires specifying **two distinct spatial positions** (source, target) — information that a single entity does not carry.

This is the geometric analog of a philosophical point: **a thing is not an action**. To say "hammer" means "to hammer" requires importing the USAGE context — how the entity is characteristically deployed in a two-place relation. That context is not intrinsic to the entity.

### 3.2 The Linguistic Argument AGAINST the Gap

Denominalization is one of the most productive morphological processes in natural language:
- English: "hammer" → "to hammer", "Google" → "to google", "elbow" → "to elbow"
- Universal: all documented natural languages have some form of noun-to-verb conversion

If UL claims universality for compositional meaning, the inability to express this productive transformation is a gap.

### 3.3 Reconciliation: Denominalization Is Contextual Predication

The resolution: denominalization **is** expressible in UL, but not as a single e → r operation. It's a **pattern** built from existing operations:

**Claim:** "To hammer B" = "A uses hammer on B" = predicate(e_agent, r_use, predicate(e_hammer, r_instrument, e_patient))

More precisely:

```
"A hammers B" ≡ predicate(A, r_act, B)
    WHERE r_act = modify_relation(abstract(e_hammer), r_generic_action)
```

Breaking this down:
1. `abstract(e_hammer) → m_hammer` — extract the "hammer-quality" as a modifier
2. `modify_relation(m_hammer, r_generic_action) → r_hammering` — apply hammer-quality to a generic action relation
3. `predicate(A, r_hammering, B)` — A hammers B

This requires a **generic action relation** r_generic_action ∈ Gᵣ (a "bare" directed connection meaning roughly "affects/acts-on"). This is a legitimate element of the carrier set Gᵣ — the simplest directed path between two points, unmodified.

### 3.4 Verification: Does This Capture Denominalization?

| Natural language | UL decomposition | Captures meaning? |
|---|---|---|
| "A hammers B" | predicate(A, modify_relation(abstract(hammer), r_act), B) | ✅ A acts on B in a hammer-characterized way |
| "A googles X" | predicate(A, modify_relation(abstract(google), r_search), X) | ✅ A searches X in a Google-characterized way |
| "A elbows B" | predicate(A, modify_relation(abstract(elbow), r_contact), B) | ✅ A contacts B in an elbow-characterized way |
| "A shelves X" | predicate(A, modify_relation(abstract(shelf), r_place), X) | ✅ A places X in a shelf-characterized way |

The pattern works: `abstract` extracts the entity's quality, `modify_relation` applies it to a base relation, and `predicate` completes the assertion.

### 3.5 Why This Is Not a New Operation

The key insight: denominalization is not a single sort-transition e → r. It's a **two-step composition** that routes through sort m:

```
e →[abstract]→ m →[modify_relation(m, r₀)]→ r
```

This path EXISTS in the sort-transition graph — it just requires r₀ (a base relation from the carrier set Gᵣ). The requirement for r₀ is not a defect — it reflects the genuine linguistic fact that denominalization always implies a **specific type of action** (using, striking, contacting, searching, placing...) that is contextually determined, not intrinsic to the entity.

---

## 4. The Other Missing Transitions

### 4.1 m → r (Manner-to-action): Same pattern

"gentle" → "to gentle" = modify_relation(m_gentle, r_generic_action). Already directly available as a single operation application.

### 4.2 a → r (Proposition-to-action): Not needed

"The fact that X" → "to verify X" is NOT a sort transition. It's a new predication: predicate(agent, r_verify, embed(a)). The embedded assertion becomes an entity arguments, and "verify" is a relation from the carrier set.

### 4.3 r → m (Action-to-manner): Genuinely absent but low-impact

"to run" → "running-ly" (the manner of running). This would require extracting a modifier from a relation. No current operation does this.

However: `abstract(embed(predicate(e₁, r_run, e₂))) → m` would extract the quality of the entire "running" assertion, which approximates "running-ly." This is awkward but functional — it routes through a → e → m.

**Approximate derivation:** r → (use in predicate) → a → (embed) → e → (abstract) → m.

This path is long but sort-valid. The r → m transition is **approximately derivable** (with semantic loss: the modifier captures the quality of the entire predication, not just the action).

---

## 5. Verdict

### The e → r gap is NOT a defect — it's correct.

**Reasons:**

1. **Non-derivability is principled**: A spatial region (entity) has no intrinsic directionality. A directed path (relation) requires two endpoints. The type mismatch is geometrically real.

2. **Denominalization IS expressible**: Via the composition `abstract(e) → m`, then `modify_relation(m, r₀) → r`. This decomposes "hammer → to hammer" into "extract hammer-quality" + "apply to base action." The decomposition is semantically correct — it makes explicit the hidden generic-action presupposition.

3. **No operation is missing**: The sort-transition graph as-is supports all productive morphological patterns in natural language, using multi-step compositions plus carrier-set elements. A direct e → r would HIDE the fact that denominalization always involves a contextual base relation.

4. **Adding `verbalize` would be wrong**: A single e → r operation would imply that entities have intrinsic actions — that "hammer" has a unique action meaning without context. But "to hammer" means different things in "hammer a nail" vs. "hammer a point" vs. "hammer a price." The contextual decomposition is more accurate.

### Impact on Documentation

The current `formal-operations.md` §3.3 defense — "a relation without entities is meaningless" — is partially correct but poorly stated. The better defense:

> "There is no e → r operation because relations require two endpoints (source, target), while entities are single spatial configurations. Denominalization — the apparent need for e → r — is expressible via the composition `abstract(e) → m, modify_relation(m, r₀) → r`, which makes explicit the contextual base relation that denominalization always presupposes."

---

## 6. Status Update

**Status:** ✅ RESOLVED — e → r is non-derivable as a single operation (proven) but denominalization IS expressible via existing operations (abstract + modify_relation). The gap is principled, not accidental. No new operation needed.

**Files to update:**
1. `foundations/formal-operations.md` §3.3 — improve defense of the gap
2. `ul-core/CRITIQUE.md` — update gap status
3. `docs/planning/audits/improvements/pass1-1/tier-b-structural/B3-entity-to-relation-gap.md` — mark resolved
