# Phase 3 — Pragmatics: Design Questions

**Status:** OPEN

---

## R1: Is pragmatic inference formalizable within the algebra?

### Short answer
Partially. Three categories:

| Category | Formalizable? | Mechanism |
|----------|--------------|-----------|
| Compositional pragmatics (scalar implicature, presupposition) | YES | Inference rules on Σ_UL expressions |
| Conventionalized patterns (indirect speech acts with fixed forms) | YES | Pattern-matching rules at the Σ_UL interface |
| Open pragmatic inference (sarcasm, irony, Gricean reasoning) | NO | Requires Bayesian agent models outside the algebra |

### Formal criterion

A pragmatic phenomenon is **Σ_UL-formalizable** iff the intended meaning can be computed from the literal meaning + a finite, fixed set of inference rules, without requiring:
- Probabilistic reasoning about speaker intentions
- Real-world knowledge not present in the expression
- Context-dependent abductive inference

By this criterion: scalar implicature (YES), conventional indirection (YES), sarcasm (NO), irony (NO).

---

## R2: Can sarcasm reduce to modal + negation?

### Structural representation

With Phase 1 (modality) and Phase 2 (performatives), sarcasm has a clean representation:

```
SARCASM(a):
  Surface: illocute(φ_assert, a)                    — speaker asserts a
  Belief:  necessary(r_epistemic, negate(a))          — speaker believes ¬a
  Intent:  illocute(φ_express, negate(a))             — speaker communicates ¬a
```

The sarcasm IS the conjunction of these three components:
```
sarcastic(a) ≡ conjoin(
  illocute(φ_assert, a),                             — surface assertion
  necessary(r_K_speaker, negate(a))                   — speaker knows it's false
)
```

### What's missing

**Detection:** How does the hearer know the speaker is being sarcastic rather than lying? In formal terms: how to distinguish `sarcastic(a)` from `deceitful(a)`, given that both involve `assert(a) ∧ believe(¬a)`?

The distinction requires:
1. Mutual knowledge that a is obviously false in context
2. Recognition that the speaker INTENDS the hearer to recognize the falsity
3. Inference that the purpose is expressive (humor, criticism) not deceptive

None of these are algebraic operations.

### Verdict

Sarcasm is **representable** (its structure can be encoded) but not **derivable** (the system cannot determine that an expression IS sarcastic). The representation is useful — it gives a formal target for pragmatic inference layers.

---

## R3: What formalizes Gricean maxims?

### The four maxims

| Maxim | Content | Formal analogue |
|-------|---------|-----------------|
| **Quantity** | Say enough, but not too much | Informativeness optimization |
| **Quality** | Say what you believe true | Sincerity condition (φ_assert → speaker believes content) |
| **Relation** | Be relevant | Discourse coherence (relation to prior context) |
| **Manner** | Be clear, orderly, brief | DC_UL complexity minimization |

### Bayesian formalization (RSA framework)

Goodman & Frank (2016) formalize Gricean reasoning as:

```
P_speaker(utterance | meaning) ∝ exp(λ · log P_listener(meaning | utterance) - cost(utterance))
P_listener(meaning | utterance) ∝ P_speaker(utterance | meaning) · P(meaning)
```

This connects to Expedition Two:
- **P(meaning)** = structural prior P(m) from probability-and-information.md
- **cost(utterance)** = description complexity DC_UL
- **λ** = rationality parameter (how "rational" the speaker is)

### Does this belong in Σ_UL?

No — it operates ON Σ_UL expressions. The algebra provides the meaning space; the RSA framework provides the pragmatic reasoning over that space. This is the correct architectural separation.

### Recommendation

Define the RSA-Σ_UL interface as a Phase 3 deliverable. The structural prior and complexity measure from Expedition Two are the bridge.

---

## R4: How does scalar implicature interact with quantify?

### The scale

UL's graduated quantification uses p ∈ [0,1]:
- p = 0⁺ → "some" (existential)
- p = 0.5 → "half"
- p = 1 → "all" (universal)

This creates a natural **quantifier scale**: {some, many, most, all} maps to a linearly ordered subset of [0,1].

### The implicature rule

```
RULE (Scalar Implicature):
  If assert(quantify(p₁, e)) and p₁ < 1,
  then implicate(negate(quantify(p₂, e))) for all p₂ > p₁
  
  "Asserting 'some' (p₁ < 1) implicates 'not all' (p₂ = 1)"
```

### Is this algebraic?

Almost. The rule is:
- **Structural:** It depends only on the quantification parameter p
- **Finite:** One rule covers all scalar cases
- **Derivable from the scale ordering:** The ordering p₁ < p₂ is built into [0,1]

**But:** The implicature is **cancellable** — "Some students passed, in fact all of them did" is valid. This means the inference is defeasible, which is fundamentally different from algebraic derivation.

### Recommendation

Formalize as a **default inference rule** — it applies unless explicitly cancelled:
```
assert(quantify(p₁, e)) ⟹_default negate(quantify(1, e))  for p₁ < 1
```

The cancellation mechanism:
```
conjoin(quantify(p₁, e), quantify(1, e))  — explicitly asserting both cancels the implicature
```

This is compatible with the algebra (conjoin is an operation) and makes the pragmatic inference explicit.

---

## R5: Is the semantic/pragmatic boundary the right cut point?

### Current boundary

Σ_UL handles: compositional truth-conditional semantics
External to Σ_UL: everything else (pragmatics, discourse, social meaning)

### Proposed refined boundary

| Layer | What's Here | Formal Status |
|-------|------------|---------------|
| **Σ_UL core** | 4 sorts, 13 operations, truth-conditional compositional semantics | Algebraic — closed under operations |
| **Σ_UL extended** (Phase 1+2) | Modal semantics (defined operators) + illocutionary force (φ extension) | Algebraic + parameterized — closed under operations |
| **Inference interface** | Scalar implicature, presupposition, conventional indirection | Rule-based — finite rule set over Σ_UL expressions |
| **Pragmatic layer** | Gricean reasoning, sarcasm/irony detection, discourse coherence | Probabilistic — RSA/Bayesian framework over Σ_UL meaning space |
| **Social layer** | Politeness, face-saving, register, power dynamics | Not formalized — beyond current scope |

### Is this the right cut?

**Argument for:** The boundary aligns with a natural mathematical boundary — algebraic (compositional, finitary) vs. probabilistic (inferential, statistical). This is a deep distinction in formal semantics.

**Argument against:** Some "pragmatic" phenomena (presupposition projection, focus semantics) are treated as semantic by formal linguists. The boundary has been debated for 50 years.

### Recommendation

Adopt the refined boundary above. Acknowledge that it may need adjustment. The key principle: **Σ_UL encodes compositional structure; inference rules derive conventional implicatures; probabilistic models handle open-ended pragmatics.**

This gives a clean separation of concerns and allows each layer to use its natural mathematical formalism.
