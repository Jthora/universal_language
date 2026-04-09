# Phase 2 — Force Algebra Plan

**Goal:** Define the formal mechanism for illocutionary force within Σ_UL⁺.

---

## 1. The σ-φ Extension

### Current assertional sign

Every assertion has a sign σ ∈ {⊕, ⊖}:
- ⊕ = asserted (the content is claimed true)
- ⊖ = denied (the content is claimed false)
- `negate` swaps ⊕ ↔ ⊖

### Extended assertional structure

Add a force parameter φ orthogonal to σ:

```
Assertion = (F, C, σ, φ)

where:
  σ ∈ {⊕, ⊖}         (assertional sign — truth claim)
  φ ∈ Force            (illocutionary force — speech act type)

Force = {assert, query, direct, commit, express, declare}
```

### Force lattice

Forces have a partial order based on "strength of world-change":

```
  declare > commit > direct > express > assert > query

  declare: changes institutional reality
  commit:  creates obligation for speaker
  direct:  creates obligation for hearer
  express: reveals speaker's psychological state
  assert:  claims truth of content
  query:   requests information about content
```

This ordering may or may not be algebraically useful — it's noted for potential exploitation.

---

## 2. Interaction with negate

### The problem

`negate : a → a` currently swaps σ⊕↔⊖. What happens to φ?

### Options

| Approach | Definition | Example |
|----------|-----------|---------|
| **φ-preserving** | negate only affects σ; φ unchanged | negate("I promise to return") = "I promise NOT to return" |
| **φ-negating** | negate switches force to its "opposite" | negate("I promise to return") = "I retract my promise to return" |
| **φ-collapsing** | negate always produces assertive force | negate("I promise to return") = "It is not the case that I promise to return" |

### Analysis

**Natural language evidence:**

"I don't promise to return" is ambiguous:
- **Content negation:** "I promise to NOT return" (φ preserved, σ flipped on content)
- **Force negation:** "I do NOT promise to return" (φ negated/collapsed, σ flipped)

This is the classic scope ambiguity: does negation scope over force or under force?

### Recommendation

**φ-preserving** as default: `negate` affects σ only, not φ.

Force negation (retracting a promise, withdrawing a declaration) is a DIFFERENT operation — a meta-level speech act. It can be expressed via:
```
modify_assertion(m_retracted, φ_commit(a))  — "I retract my promise that a"
```

This keeps `negate` clean (operates only on σ) while allowing force-negation via `modify_assertion`.

---

## 3. Force Composition Rules

### Rule FC1: Force under conjoin

```
conjoin(a₁[φ₁], a₂[φ₂]) → a_conj[φ_compound]
```

**Case 1: Same force** — φ₁ = φ₂ = φ → φ_compound = φ
```
conjoin(commit(a₁), commit(a₂)) → commit(conjoin(a₁, a₂))
"I promise to help and promise to arrive" = "I promise to help and arrive"
```

**Case 2: Mixed force** — φ₁ ≠ φ₂ → φ_compound = {φ₁, φ₂} (compound force)
```
conjoin(commit(a₁), assert(a₂)) → {commit, assert}(conjoin(a₁, a₂))
"I promise to help and I'll arrive at 3" — compound: promise + prediction
```

**Formal representation of compound force:** φ_compound is a set. For n conjuncts with distinct forces, φ_compound ⊆ Force with |φ_compound| ≤ n.

### Rule FC2: Force under disjoin

```
disjoin(a₁[φ₁], a₂[φ₂]) → ?
```

This is linguistically unusual. "I promise to help OR I command you to leave" is grammatical but pragmatically odd. 

**Recommendation:** Disjunction of differently-forced assertions is well-formed but marked. The force is the join in the force lattice.

### Rule FC3: Force under embed

```
embed(a[φ]) → e
```

Embedding nominalizes the assertion (turns it into an entity). The force is preserved in the embedded content:
```
embed(commit(a)) = "the promise that a" — an entity
embed(query(a)) = "the question whether a" — an entity
```

**This is the mechanism for TALKING ABOUT speech acts:** `predicate(e_John, r_made, embed(commit(a)))` = "John made a promise that a."

### Rule FC4: Force under modal operators

```
necessary(r_R, a[φ]) → a[φ']
```

**Question:** Does the modal operator affect the force?

```
necessary(r_deontic, commit(a)) → commit(necessary(r_deontic, a))?
```

**Recommendation:** Modal operators are force-transparent — they pass through force unchanged. The modal quantification ranges over worlds; in each world, the speech act retains its force.

---

## 4. Formalization of the 5 Cases

### 9.1: "I hereby pronounce you married"

```
modify_assertion(m_institutional,
  (F, predicate(e_you, r_married, e_implicit), ⊕, declare)
)
```

Where:
- φ = declare (the utterance changes reality)
- m_institutional = modifier marking institutional authority requirement
- The predication `predicate(e_you, r_married, ...)` is the content that becomes true BY the declaration

**Felicity condition (meta-level):** The speaker must have institutional authority. This is NOT representable in the algebra — it is a constraint on the context in which the expression is felicitous.

### 9.2: "I promise to return"

```
(F, predicate(e_I, r_return, e_implicit), ⊕, commit)
```

Simpler: the content is the commitment, the force is commissive.

### 9.3: "I apologize for the delay"

```
(F, predicate(e_delay, r_caused_inconvenience, e_hearer), ⊕, express)
```

The expressive force marks that the utterance performs the social act of apology.

### 9.4: "You're fired"

```
modify_assertion(m_institutional,
  (F, negate(predicate(e_you, r_employed, e_here)), ⊕, declare)
)
```

Declarative force + institutional modification. Content: you are no longer employed.

### 9.5: "I bet you five dollars it rains"

```
(F, predicate(e_rain, r_occurs, e_tomorrow), ⊕, commit)
```

With additional structure:
```
bind(e_bet,
  conjoin(
    modify_assertion(m_conditional,
      (F₁, predicate(e_rain, r_occurs, ...), ⊕, commit)),
    predicate(e_I, r_owes, modify_entity(m_five_dollars, e_you))
  )
)
```

The bet has a conditional structure: the commitment activates contingent on outcome.

---

## 5. Impact Assessment

### What changes in Σ_UL⁺

| Element | Change | Impact |
|---------|--------|--------|
| Assertion tuple | (F, C, σ) → (F, C, σ, φ) | All assertion-producing operations must specify φ |
| `predicate` output | Default φ = assert | Backward compatible |
| `negate` | Preserves φ, flips σ | No change in behavior for existing assertions |
| `conjoin`/`disjoin` | Must handle compound φ | New composition rule needed |
| `embed` | Preserves φ in embedded content | Allows reference to speech acts |
| `modify_assertion` | Orthogonal to φ | No change |
| Writing system | Frame decorations for each φ | New visual conventions |

### What does NOT change
- Sort system (4 sorts)
- Operation count (13)
- Independence proofs (no new operations)
- Montague homomorphism (extends naturally: Montague's moods map to φ)

### Backward compatibility

**Default force:** Every existing assertion has implicit φ = assert. This means all prior formalizations remain valid — they are assertive speech acts, which is what they were always intended to be.
