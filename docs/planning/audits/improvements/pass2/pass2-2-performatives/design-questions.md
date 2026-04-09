# Phase 2 — Performatives: Design Questions

**Status:** OPEN — All questions require formal resolution before implementation

---

## P1: Is illocutionary force a sort, operation, or parameter?

### Analysis

Three framings:

**Force as sort** — Force is a kind of thing. You pick a force and apply it.
```
illocute(f_commissive, a) → a
```
Requires: 5th sort Force, new operation illocute.

**Force as operation** — Forcing is something you DO.
```
promise(a) → a       # Each force type is a separate operation
question(a) → a
command(a) → a
```
Requires: 5+ new operations. Combinatorial explosion.

**Force as parameter** — Force is a mode of the assertional sign.
```
Assertion = (F, C, σ, φ) where φ ∈ {assert, question, command, commit, express, declare}
```
Requires: Extending the assertion tuple. No new sorts or operations.

### Evidence from linguistics

Austin (1962): Force is carried by **performative verbs** — "I promise," "I command," "I declare." These are verbs (actions), suggesting force is an operation.

Searle (1969): Force is a **component** of the speech act, alongside propositional content. Speech act = F(p). This is closest to "force as parameter."

Vanderveken (1990): Force is a **complex structure** with 7 components (illocutionary point, degree of strength, mode of achievement, propositional content conditions, preparatory conditions, sincerity conditions, degree of strength of sincerity). This suggests force is too rich for a single parameter.

### Recommendation

**Start with "force as parameter" (Architecture C / σ-extension)** because:
1. It requires no new sorts or operations
2. It extends an existing mechanism (the assertional sign)
3. Interaction with negate needs careful analysis but is tractable
4. If the parameter proves insufficient (Vanderveken's 7 components), upgrade to "force as sort"

---

## P2: How does force distribute over conjunction?

### The problem

"I promise to help and I'll arrive at 3" has two readings:

**Reading 1:** `conjoin(promise(help), assert(arrive-at-3))` — different forces per conjunct  
**Reading 2:** `promise(conjoin(help, arrive-at-3))` — single force over compound content

These are semantically distinct:
- Reading 1: I'm committed to helping; I'm predicting I'll arrive at 3
- Reading 2: I'm committed to both helping AND arriving at 3

### Current `conjoin` semantics

`conjoin : a × a → a` takes two assertions and produces their conjunction.

If force is a parameter of each assertion, then `conjoin` must either:
- **Preserve separate forces** (φ₁ ∧ᶠ φ₂ — conjunction carries both forces) → vector-valued force
- **Default to assertive** (φ_assert ∧ ... — conjunction force is always assertive) → lossy
- **Require matching forces** (φ₁ = φ₂ for conjunction to be well-formed) → restrictive

### Recommendation

**Separate forces preserved:** `conjoin(a₁, a₂)` where a₁ has force φ₁ and a₂ has force φ₂ produces a compound with both forces. This matches natural language: "I promise to help and predict I'll arrive" is valid.

**Formal mechanism:** The conjunction (F_conj, C_conj, σ_conj, φ_conj) inherits:
- σ_conj = ⊕ if both σ₁ = σ₂ = ⊕; otherwise complex
- φ_conj = {φ₁, φ₂} — set-valued (the conjunction has compound force)

**This needs formal proof that set-valued φ doesn't break existing theorems.**

---

## P3: What is the geometric realization of force?

### Current assertion geometry

An assertion is (F, C, σ) — frame, content, sign:
- **F** = boundary of a region in ℝ² (the frame)
- **C** = content inside the frame (geometric marks)
- **σ** ∈ {⊕, ⊖} — assertional sign (asserted/denied)

### Extension for force

Add φ as a **frame property**:

| Force | φ Value | Visual Convention | Mnemonic |
|-------|---------|-------------------|----------|
| Assertive | φ_assert | Solid frame (default) | Statement |
| Interrogative | φ_query | Frame with opening (gap in boundary) | Question is "open" |
| Directive | φ_direct | Arrow-marked frame (→ on boundary) | Points at hearer |
| Commissive | φ_commit | Inward-arrow frame (← on boundary) | Points at speaker |
| Expressive | φ_express | Wavy frame | Emotional/felt |
| Declarative | φ_declare | Bold frame with institutional mark | Authoritative |

### Consistency check

The `modify_assertion` operation adds frame-boundary decoration. Force markers are also frame-boundary features. Are they the same kind of thing?

**Distinction:** `modify_assertion` modifiers (evidentiality, emphasis, hedging) modify the EPISTEMIC profile of the assertion. Force markers modify the PRAGMATIC function. These are orthogonal dimensions:
- "I confidently promise to return" — evidential modifier (m_confident) + commissive force (φ_commit)
- `modify_assertion(m_confident, force(φ_commit, predicate(e_I, r_return, ...)))`

**They should compose independently.** The frame has both an epistemic decoration AND a force decoration.

---

## P4: How do performatives interact with modality?

### Composition cases

| Expression | Modality | Force | Structure |
|-----------|---------|-------|-----------|
| "You should promise" | Deontic necessity | Commissive | □_O(φ_commit(content)) |
| "Can you close the door?" | Ability | Directive (indirect) | ◇_A(φ_direct(content)) — but this is actually pragmatic |
| "I might apologize" | Epistemic possibility | Expressive | ◇_K(φ_express(content)) |

### Type checking

If force is a parameter of assertions (Architecture C), and modality is defined over assertions:
```
necessary(r_O, a) where a = (F, C, ⊕, φ_commit)
```
The modal operator doesn't interact with φ — it quantifies over worlds where the speech act holds. This is semantically odd but type-correct.

**Semantic question:** What does "in all ideal worlds, I promise to return" mean? Does the obligation apply to:
- The ACT of promising? (I should make the promise)
- The CONTENT of the promise? (I should return)

This ambiguity is real in natural language and cannot be resolved algebraically — it depends on pragmatic context.

### Recommendation

Define modality as operating on the propositional CONTENT, not the force:
```
φ_commit(necessary(r_O, predicate(e_I, r_return, ...)))
= "I promise to necessarily return"
= "I promise to return (obligatorily)"
```

vs.

```
necessary(r_O, φ_commit(predicate(e_I, r_return, ...)))
= "It is obligatory that I promise to return"
= "I should promise to return"
```

**Both are valid expressions** with different meanings. The algebra handles the ambiguity by allowing both word orders.

---

## P5: Does UL need an Agent sort?

### The question

Performatives require speakers, hearers, and sometimes institutional authorities. Are these just entities, or something more?

### Analysis

**What entities can capture:**
- Speaker: e_speaker (distinguished entity)
- Hearer: e_hearer (distinguished entity)
- Authority: predicate(e_speaker, r_has_authority, e_institution)

**What entities cannot capture:**
- **Indexicality:** "I" refers to the speaker, but which entity IS the speaker changes with context. This is a deixis problem, not a sort problem.
- **Agency:** "I promise" requires the speaker to HAVE the capacity to commit. This is a felicity condition on the speech act, not a structural property.

### Verdict

**No Agent sort needed.** Use:
- Distinguished entities: e_speaker, e_hearer (contextual, like w_current)
- Role modifiers: modify_entity(m_speaker_of, e_person) — marks an entity as the speaker
- Felicity conditions: meta-level constraints on when a speech act is valid

This follows the same "distinguished elements, not new sorts" approach as Phase 1's w_current.
