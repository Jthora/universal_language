# B4 — What Is the True Minimal Operation Set?

**Tier:** B (Structural)  
**Finding:** F2 (De Morgan redundancy — conjoin derivable from {negate, disjoin})  
**Targets:** `foundations/formal-foundations.md` §2 (11 operations), `pass1/phase-2-hard-problems/2.2-algebraic-independence.md`  
**Dependencies:** B1 (negation must be correct before independence can be tested)  
**Effort:** 2–6 months of algebraic analysis

---

## The Problem

`formal-foundations.md` §2.1 claims 11 operations. Pass 1 problem 2.2 identified suspicions. The deep audit confirms at least one redundancy and suspects more.

### Confirmed Redundancies

**F2 — De Morgan redundancy:**
```
conjoin(a, b) = negate(disjoin(negate(a), negate(b)))
```

If negate and disjoin exist, conjoin is derivable. Therefore the operation set is NOT minimal — it has at most 10 independent operations.

### Suspected Redundancies

| Operation | Suspicion | Test |
|-----------|-----------|------|
| `quantify(m, e) → a` | May be derivable from `modify_entity` + `predicate` + `embed` | Can `∀x.P(x)` be constructed from "for-all modifier applied to x, then predicated"? |
| `invert(r) → r` | May be a special case of `modify_relation` | Is there a reflection modifier m_refl such that `modify_relation(m_refl, r) = invert(r)`? |
| `compose(r₁, r₂) → r` | May be derivable from `predicate` + `embed` | Can `r₁ ∘ r₂` be constructed as "the relation such that predicate(e, r₁, embed(predicate(e₂, r₂, e₃))) holds"? |

### Known Independent Operations

Some operations are clearly independent (they produce sort transitions that no other operation does):

| Operation | Unique Sort Signature | Independence Argument |
|-----------|----------------------|----------------------|
| `predicate(e, r, e) → a` | Only e × r × e → a path | Independent (unique output construction) |
| `embed(a) → e` | Only a → e path | Independent (unique sort transition) |
| `abstract(e) → m` | Only e → m path | Independent (unique sort transition) |

## Independence Proof Strategy

### Standard Technique (from Universal Algebra)

From `pass1/phase-2-hard-problems/2.2-algebraic-independence.md`:

> To prove an operation f independent of the others: construct a model (algebra) where all other operations are defined but f is not definable from them. If no finite combination of the other operations can reproduce f's behavior, f is independent.

### Concrete Approach for Each Suspect

**For conjoin:** Already shown derivable from {negate, disjoin}. Status: REDUNDANT.

**For quantify:** Construct a Σ_UL-algebra where:
- modify_entity, predicate, embed are defined normally
- All outputs of compositions of these operations have a specific property P
- quantify produces outputs that violate property P
- Therefore quantify cannot be a composition of the others

**For invert:** Construct a Σ_UL-algebra where:
- modify_relation is defined normally
- Show that for some r, no modifier m exists where modify_relation(m, r) = invert(r)
- This requires checking whether the "direction-reversal" transformation is in the modifier carrier set G_m

**For compose:** Construct a Σ_UL-algebra where:
- predicate and embed are defined normally  
- Show that the "transitive chaining" of two relations cannot be reproduced by predication + embedding
- Key obstacle: `predicate(e₁, r₁, embed(predicate(e₂, r₂, e₃)))` produces an assertion, not a relation. We'd need some way to extract a relation from this assertion. No such operation exists (no a → r path) — this is related to the B3 gap.

## Expected Outcome

### Minimum Independent Set (Conservative Estimate)

```
Independent Core (9 operations):
  predicate       : e × r × e → a    — unique sort signature
  modify_entity   : m × e → e        — needed for entity modification
  modify_relation : m × r → r        — needed for relation modification  
  negate          : a → a             — needed for logical negation (once B1 is fixed)
  disjoin         : a × a → a        — needed for disjunction
  embed           : a → e             — unique a → e path
  abstract        : e → m             — unique e → m path
  compose         : r × r → r        — likely independent (no r output alternatives)
  invert          : r → r             — likely independent (direction-reversal)

Derivable (≥ 1 operation):
  conjoin         : a × a → a        — CONFIRMED derivable from {negate, disjoin}

Uncertain (2 operations):  
  quantify        : m × e → a        — may be derivable, needs proof
  invert          : r → r             — may be special modify_relation, needs proof
```

### Best Case: 8–9 Independent Operations  
### Worst Case: 10 Independent Operations (only conjoin redundant)

## Relationship to Minimality Claims

`formal-foundations.md` currently claims 11 operations. This analysis will produce one of:
1. **≤ 10 independent operations** — the claimed count is too high; needs correction
2. **Exactly 10 independent operations** (only conjoin drops) — minor correction needed
3. **9 or fewer** — significant redundancy; raises questions about the design process

In any case, the correct framing should be:
- "Σ_UL has [N] independent operations and [11-N] derived operations"
- "The full set of 11 is chosen for *convenience and readability*, not minimality"
- Minimality claim removed; completeness claim retained

## Deliverable

A table:
| Operation | Independent? | Proof | Notes |
|-----------|-------------|-------|-------|
| (each of 11) | ✅/❌/❓ | Reference to proof | Derivation if redundant |

## Status

**Status:** ❌ OPEN — confirmed 1 redundancy (conjoin), suspected 1–2 more (quantify, invert). Full independence proof requires B1 resolution first.
