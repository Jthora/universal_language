# Phase 2 — Performatives: Illocutionary Force

**Status:** 📋 PLANNING  
**Target:** 5 ❌ cases (9.1–9.5)  
**Dependencies:** Phase 0 (consolidation), Phase 1 (modality — partial)  
**Blocks:** Phase 3 (pragmatics)

---

## Problem Statement

All 5 performative failures exhibit the same pattern:

| Case | Expression | Propositional Content | Illocutionary Force |
|------|-----------|----------------------|---------------------|
| 9.1 | "I hereby pronounce you married" | predicate(e_you, r_married, ...) ✅ | Declarative: utterance CREATES the state ❌ |
| 9.2 | "I promise to return" | predicate(e_I, r_return, ...) ✅ | Commissive: speaker binds self to obligation ❌ |
| 9.3 | "I apologize for the delay" | predicate(e_delay, r_occurred, ...) ✅ | Expressive: social repair act ❌ |
| 9.4 | "You're fired" | negate(predicate(e_you, r_employed, ...)) ✅ | Institutional declarative: authority changes reality ❌ |
| 9.5 | "I bet you five dollars it rains" | predicate(e_rain, r_occurs, ...) ✅ | Contractual: agreement created by utterance ❌ |

**The gap is uniform:** Propositional content is fully expressible. Illocutionary force — the speech-act dimension that determines whether an assertion is a statement, question, command, promise, or declaration — is not.

---

## The Content/Force Distinction

Austin (1962) and Searle (1969) established that every speech act has two components:

1. **Propositional content** — what the utterance is about (fully in Σ_UL)
2. **Illocutionary force** — what the utterance DOES (not in Σ_UL)

Searle's taxonomy of force types:

| Force Type | What It Does | D2 Cases |
|-----------|-------------|----------|
| **Assertive** | Commits speaker to truth of content | (Default in Σ_UL — σ = ⊕) |
| **Directive** | Attempts to get hearer to do something | (Not in D2 but common: "Close the door") |
| **Commissive** | Commits speaker to future action | 9.2 (promise), 9.5 (bet) |
| **Expressive** | Expresses psychological state | 9.3 (apology) |
| **Declarative** | Changes institutional reality | 9.1 (pronounce married), 9.4 (fire) |

UL currently treats ALL assertions as assertive (stating facts). The other four force types have no representation.

---

## Approach: Three Candidate Architectures

### Architecture A: Force as a new sort (5th sort)

```
SORTS: Entity (e), Relation (r), Modifier (m), Assertion (a), Force (f)

NEW OPERATION:
  illocute : f × a → a     # Apply force f to assertion a
```

**Force elements:** f_assert, f_direct, f_commit, f_express, f_declare

**Example:**
```
9.2: illocute(f_commit, predicate(e_I, r_return, ...))
     "I [promise/commit to] return"
```

**Pros:** Clean type system. Force is first-class. Each force type has its own felicity conditions.
**Cons:** Breaks 4-sort minimality. Must derive 5th sort from geometric primitives. What geometric primitive generates Force?

### Architecture B: Force as assertion modification (no new sort)

```
NO NEW SORT. Use modify_assertion with distinguished modifiers:

  modify_assertion(m_commissive, a)    # Promise/commitment
  modify_assertion(m_declarative, a)   # Declaration
  modify_assertion(m_expressive, a)    # Expression
  modify_assertion(m_directive, a)     # Command/request
```

**Example:**
```
9.2: modify_assertion(m_commissive, predicate(e_I, r_return, ...))
     "I [promise to] return"
```

**Pros:** No new sort. No new operation. Maximum parsimony. Uses existing `modify_assertion`.
**Cons:** `modify_assertion` was designed for evidentiality/emphasis/hedging — frame-boundary decoration. Illocutionary force is structurally different: it changes the NATURE of the speech act, not its epistemic profile. Conflating these may be a category error.

### Architecture C: Force as σ-extension (enrich assertional sign)

```
Currently: σ ∈ {⊕, ⊖}   (asserted, denied)
Extended:  σ ∈ {⊕, ⊖, ?, !, ⊢, ♡}

Where:
  ⊕ = assertive (asserted)
  ⊖ = denied (negated)
  ? = interrogative (questioned)
  ! = directive (commanded)
  ⊢ = declarative (declared/instituted)
  ♡ = expressive (felt/apologized)
```

**Example:**
```
9.2: (F, C, ♡_commit)   — commissive force
9.4: (F, C, ⊢_terminate) — declarative force
```

**Pros:** Extends existing structure minimally. The assertional sign already encodes "what the speaker is doing with the content." This is arguably the right place for force.
**Cons:** σ currently has algebraic properties (negate swaps ⊕↔⊖). Extending σ beyond binary requires defining how negate acts on ?, !, ⊢, ♡. Is the negation of a question a question? The negation of a command?

---

## Recommended Investigation Path

**Primary:** Architecture B (modify_assertion) — test whether existing machinery suffices.

**Test case:** Formalize "I promise to return" (9.2) as:
```
modify_assertion(m_commissive, predicate(e_I, r_return, e_implicit))
```

**Questions to answer:**
1. Does this capture that the ACT of promising creates an obligation? Or does it merely DESCRIBE a promise?
2. Can "I hereby pronounce you married" (9.1) be distinguished from "They are married" (simple predication) using only assertion-level modification?
3. How does performative modification interact with negate? Is `negate(modify_assertion(m_commissive, a))` = "I retract my promise that a"?

**If B fails** (cannot distinguish performative force from mere description):
→ Architecture C (σ-extension) — extends the assertional sign while keeping 4 sorts
→ Architecture A (5th sort) — last resort

---

## Key Design Questions

### P1: Is illocutionary force a sort, operation, or parameter?

See Architecture A vs B vs C above. The decision depends on whether force is:
- A **what** (sort — force is a thing that exists)
- A **how** (operation — force is something you DO to content)  
- A **which** (parameter — force is a mode of assertion)

**Linguistic evidence:** Austin treats force as a dimension of the ACT (performative verbs). Searle treats it as a component of the speech act (force + content). This suggests Architecture C (parameter of the assertion) is closest to the linguistic consensus.

### P2: How does force interact with compositionality?

"I promise to help and I'll arrive at 3" — two clauses with DIFFERENT forces:
- Clause 1: commissive force (promise)
- Clause 2: assertive force (prediction)

Under `conjoin`: `conjoin(modify_assertion(m_commissive, a₁), a₂)` — force distributes to the first conjunct only.

**Challenge:** Searle & Vanderveken (1985) showed this is a known hard problem. Force does NOT always distribute over conjunction cleanly.

### P3: What is the geometric realization of force?

Current geometric semantics for assertions: **(F, C, σ)** — frame F, content C, sign σ.

Force could be:
- **Frame shape:** Assertive = rectangular, interrogative = circular, directive = triangular, etc.
- **Frame decoration:** Assertive = solid, commissive = double-lined, declarative = bold, etc.
- **External mark:** A force-glyph placed outside the frame

**Recommendation:** Frame decoration (Architecture B's approach) — consistent with how `modify_assertion` already works.

### P4: How do performatives interact with modality?

"You should promise to help" = deontic(obligation, commissive(help))

Modal operators from Phase 1 wrap performative assertions. This is well-typed if performative modification produces assertions:
```
necessary(r_deontic, modify_assertion(m_commissive, predicate(e_you, r_help, ...)))
```

Type check: modify_assertion : m × a → a, so the inner expression is sort a. necessary takes a → a. ✓

### P5: Does UL need an Agent sort?

Performatives inherently involve speakers, hearers, and institutional roles.

Currently, agents are entities. But "the speaker" has a privileged role in "I promise..." that entity-hood doesn't capture — the speaker is the SOURCE of the force.

**Options:**
- **No new sort:** Speaker/hearer are distinguished entities (like w_current for worlds). Use `predicate(e_speaker, r_forces, embed(modify_assertion(m_commissive, a)))`.
- **Role markers:** Use `modify_entity(m_speaker, e_I)` to mark entity roles in speech acts.
- **Agent sort:** Add sort Agent ≠ Entity — but this is radical and probably unnecessary.

**Recommendation:** Distinguished entities (e_speaker, e_hearer) + role modifiers. Same approach as Phase 1's w_current.

---

## Deliverables

| Deliverable | Document | Description |
|-------------|----------|-------------|
| Architecture decision | Updated design-questions.md | A/B/C resolved with formal justification |
| Force algebra | force-algebra.md | Formal definitions, typing, composition rules |
| D2 rework | (in experiments/) | All 5 Cat. 9 cases re-analyzed |
| Composition analysis | force-algebra.md §3 | Force × conjunction interaction formalized |
| Writing system | (in ul-core/) | Visual conventions for force markers |
| Independence | (if new ops added) | Independence proofs |

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Architecture B insufficient | Must redesign with B or C → scope increase | Fail fast: test B on 9.1 and 9.2 first |
| Force/conjunction non-distribution | Compositional semantics breaks for complex performatives | Accept restricted composition; document limitations |
| Agent sort required | 4-sort → 5-sort; cascading proof updates | Try distinguished entities + role modifiers first |
| Institutional facts (9.1, 9.4) resist algebraization | Some performatives are inherently social/institutional, not logical | Formalize the logical structure; note institutional prerequisites as meta-level conditions |
| Phase 1 dependency deeper than expected | Modal apparatus needed for all commissive cases | Design B/C architectures to be modal-compatible from the start |
