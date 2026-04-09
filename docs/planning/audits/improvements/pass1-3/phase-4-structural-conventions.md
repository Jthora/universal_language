# Phase 4 — Structural Decomposition Conventions

**Status:** ✅ COMPLETE
**New operations:** 0  
**D2 cases affected:** 1.4, 10.1  
**Expected case conversions:** 2 (⚠️ → ✅ with convention)  
**Dependencies:** None  
**Blocks:** None

---

## Problem Statement

Two D2 cases require decomposition conventions rather than new operations:

| Case | Expression | Issue |
|------|-----------|-------|
| 1.4 | "She gave him the book" (ditransitive) | Binary `predicate(e, r, e)` needs 3 entity slots |
| 10.1 | Mohawk polysynthetic word-sentence | Single morphological word encodes multiple semantic operations |

Both are **within** UL's expressible scope — the meaning decomposes cleanly. The issue is that the *canonical* decomposition isn't documented.

---

## Phase 4A: Polyadic Reduction Convention (Case 1.4)

### The Problem

"She gave him the book" involves three entities: **she** (agent), **him** (recipient), **the book** (theme). UL's `predicate(e, r, e)` accepts exactly two entity slots.

### The Solution: Peirce's Reduction Thesis

Charles Sanders Peirce (1870, 1885) proved that all polyadic relations reduce to compositions of dyadic (binary) and monadic (unary) relations. This is not a UL-specific convention — it is a theorem of relation algebra.

**Canonical decomposition for ditransitives:**

```
# "She gave him the book"
e_she  = entity "she"
e_him  = entity "him"
e_book = entity "the book"
r_give = relation "give"

# Decompose into two binary predications sharing the agent:
a₁ = pred(e_she, comp(r_give, r_to), e_him)       # "she gave TO him"
a₂ = pred(e_she, comp(r_give, r_theme), e_book)    # "she gave [the thing =] the book"
a  = conj(a₁, a₂)                                   # full ditransitive meaning
```

Alternative decomposition (event-based):
```
# Treat the giving as an entity (event nominalization):
e_giving = emb(pred(e_she, r_give, e_book))     # "she giving the book" → entity
a = pred(e_giving, r_to, e_him)                   # "the giving [was directed] to him"
```

Both decompositions preserve all semantic content. The choice between them is a convention — UL should document both and recommend one as canonical.

### Recommended Convention

> **Polyadic Reduction Convention:** n-ary predicates are decomposed into n−1 binary predications sharing the agent entity, using `compose` to create sub-relations (e.g., `comp(r_give, r_to)` for the recipient relation, `comp(r_give, r_theme)` for the theme relation). The predications are joined by `conjoin`. Event-based decomposition via `embed` is an alternative when the event itself is predicated upon.

### Visual Representation

```
Conjunction decomposition:         Event decomposition:

  ┌──────────────────┐             ┌──────────────────┐
  │  she ──give→── him│             │  [she──give→book]│
  │   │                │             │      │           │
  │   └──give→── book │             │      ──to→── him │
  └──────────────────┘             └──────────────────┘
```

---

## Phase 4B: Morphological Transparency Convention (Case 10.1)

### The Problem

The Mohawk word _washakotya'tawítsherahetkvhta'se_ ("he made the thing that one puts on one's body ugly for her") is a single morphological word that encodes:

- Agent: he
- Patient: her
- Theme: "thing one puts on one's body" (garment) — an embedded restrictive relative
- Verb: made ugly

UL's decomposition requires multiple operations (embed, bind, modify_entity, predicate), producing a deep operation tree from a single surface word.

### The Solution: UL Decomposes Meaning, Not Morphology

This is already implicit in UL's design but needs to be stated as an explicit convention:

> **Morphological Transparency Convention:** UL decomposes *semantic structure*, not *surface morphology*. A single morphological word may correspond to multiple UL operations, and multiple surface words may correspond to a single UL operation. The number of UL operations in a decomposition reflects semantic complexity, not surface complexity.

This is analogous to how:
- Chemistry decomposes molecules into atoms regardless of whether the compound name is one word or three
- Music theory analyzes harmonic structure regardless of whether the passage is played by one instrument or ten
- Programming language semantics analyzes operations regardless of syntactic sugar

### Decomposition of Case 10.1

```
# "He made the thing that one puts on one's body ugly for her"

# Step 1: The embedded relative clause — "thing one puts on one's body"
e_body    = entity "body"
r_put_on  = relation "puts on"
e_thing   = entity "thing"

# Restrictive relative via bind + embed:
a_rel     = bind(○_x, pred(e_one, r_put_on, ○_x))   # "one puts ○_x on body"
e_garment = me(abs(emb(a_rel)), e_thing)               # thing [that one puts on body]

# Step 2: The main predication — "he made [garment] ugly for her"
e_he      = entity "he"
e_her     = entity "her"
m_ugly    = modifier (quality: ugly)
r_made    = relation "made"

a_main    = pred(e_he, r_made, me(m_ugly, e_garment))  # he made garment ugly
a_for     = pred(emb(a_main), r_for, e_her)             # [...] for her
```

**Operation depth:** 8 operations (bind, pred, emb, abs, me×2, pred, pred). This is appropriate for a sentence with 4 semantic roles and an embedded relative clause — the same depth as an English paraphrase with 8+ words.

### What This Proves

Polysynthetic languages are not a UL limitation — they are evidence that morphological packaging varies across languages while semantic structure remains constant. UL captures the semantic structure; morphological packaging is a surface phenomenon outside UL's scope (just as pragmatic inference is outside scope).

---

## Deliverables

- [ ] Add §7 "Structural Decomposition Conventions" to `foundations/formal-operations.md`
  - §7.1 Polyadic Reduction — Peirce's reduction, canonical decomposition for ditransitives, two alternative forms
  - §7.2 Morphological Transparency — principle statement, polysynthetic example
- [ ] Re-score D2 cases 1.4, 10.1 from ⚠️ to ✅ with canonical decompositions
- [ ] Update CRITIQUE.md D2 table and resolution log
- [ ] Optional: Add worked examples to writers-companion.md (ditransitive, polysynthetic)

---

## Precedent

Both conventions are standard in formal semantics:

- **Polyadic reduction:** Davidson (1967) event semantics decomposes all predicates into binary relations via events. Neo-Davidsonian semantics (Parsons 1990) is the standard approach in formal semantics for handling ditransitives and beyond.
- **Morphological transparency:** Compositional semantics (Montague 1970, Heim & Kratzer 1998) routinely assigns different semantic structures to surface-similar expressions and identical semantic structures to surface-different expressions. The syntax-semantics interface is many-to-many.
