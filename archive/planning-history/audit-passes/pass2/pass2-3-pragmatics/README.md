# Phase 3 — Pragmatic Inference

**Status:** 📋 PLANNING  
**Target:** 3 ❌ cases (6.1, 6.2, 6.4)  
**Dependencies:** Phase 0 (consolidation), Phase 1 (modality), Phase 2 (performatives)  
**Blocks:** Nothing — this is the terminal phase

---

## Problem Statement

The three pragmatic failures represent the hardest category — meaning that is **non-compositional**, where the intended meaning diverges from or contradicts the literal compositional meaning.

| Case | Expression | Type | Why It Fails |
|------|-----------|------|-------------|
| 6.1 | "Oh great, another meeting" (sarcastic) | Sarcasm | Speaker means OPPOSITE of literal content |
| 6.2 | "Nice weather we're having" (ironic, during storm) | Irony | Literal content contradicts context |
| 6.4 | "Can you pass the salt?" (indirect request) | Indirect speech act | Surface: question about ability. Intended: request |

**Root cause:** These expressions require reasoning about **speaker intentions**, **context**, and **cooperative principles** — none of which are compositional algebraic operations.

---

## The Architectural Question

The deepest question for Phase 3 is not "what operations to add" but whether pragmatic inference belongs inside or outside Σ_UL.

### Option I: Pragmatics as an external layer

UL encodes **semantic content** (what is said). Pragmatic inference operates **on top of** UL expressions, using:
- Speaker/hearer models (beliefs, intentions, goals)
- Contextual information (physical environment, conversational history)
- Cooperative principles (Gricean maxims: quantity, quality, relation, manner)

```
                    ┌─────────────────────────┐
                    │   PRAGMATIC INFERENCE    │  ← Agent-theoretic reasoning
                    │   (speaker model,        │     Not algebraic
                    │    context, maxims)       │
                    ├─────────────────────────┤
                    │   ILLOCUTIONARY FORCE    │  ← Phase 2
                    │   (φ ∈ Force)            │
                    ├─────────────────────────┤
                    │   MODAL SEMANTICS        │  ← Phase 1
                    │   (□, ◇, worlds)         │
                    ├─────────────────────────┤
                    │   Σ_UL⁺ CORE            │  ← Passes 1.0–1.4
                    │   (4 sorts, 13 ops)      │
                    └─────────────────────────┘
```

**Implication:** Cases 6.1 and 6.2 (sarcasm, irony) are PERMANENTLY outside Σ_UL's scope. They are principled scope boundaries. Case 6.4 resolves partially (modality + force give "can you" = query about ability; the pragmatic inference from ability-question to request is external).

### Option II: Pragmatics internalized (partial)

Some pragmatic phenomena can be brought inside the algebra:

| Phenomenon | Internalization Path | Feasibility |
|-----------|---------------------|-------------|
| **Scalar implicature** | Quantifier scale p ∈ [0,1]; "some" ⟹ "not all" derivable from scale ordering | HIGH — already have graduated quantify |
| **Presupposition** | Modify_assertion with m_presupposed; content marked as taken-for-granted | MEDIUM — modify_assertion exists |
| **Conventional implicature** | "He is English and therefore brave" — "therefore" is a discourse relation | MEDIUM — compose(r_therefore, ...) |
| **Sarcasm** | Speaker believes ¬a but asserts a — requires epistemic modal + performative force mismatch | LOW — requires agent reasoning |
| **Indirect speech acts** | Surface force ≠ intended force — requires pragmatic inference | LOW — metalinguistic |

**Implication:** Some pragmatic phenomena (scalar implicature, presupposition, conventional implicature) ARE compositional and could be internalized. Non-compositional phenomena (sarcasm, irony, indirect speech acts) cannot.

### Recommendation

**Hybrid approach (Option I + partial Option II):**

1. **Internalize** what's compositional: scalar implicature, presupposition, conventional implicature
2. **Specify the interface** between Σ_UL and the pragmatic layer
3. **Declare principled boundaries** for sarcasm, irony, and indirect speech acts
4. **Provide decomposition tools** that make pragmatic inference easier even when it's external

---

## Case-by-Case Analysis

### 6.1: "Oh great, another meeting" (sarcasm)

**Literal:** `modify_assertion(m_positive, predicate(e_meeting, r_is_another, e_implicit))` ✅ expressible  
**Intended:** The speaker means the OPPOSITE — the meeting is unwelcome.

**Decomposition with Phases 1+2:**
```
Literal content:   a = predicate(e_meeting, r_is, modify_entity(m_great, e_implicit))
Speaker belief:    believes(e_speaker, negate(a))     [Phase 1: epistemic modal]
Speaker assertion: illocute(φ_assert, a)              [Phase 2: assertive force]
Sarcasm detection: believes(e_speaker, negate(a)) ∧ illocute(φ_assert, a)
                   → mismatch between belief and assertion → sarcasm
```

**What this gives us:** With modality + performatives, we can REPRESENT the structure of sarcasm (belief/assertion mismatch). We cannot DETECT it — detection requires:
1. Access to speaker's belief state (pragmatic context)
2. Recognition that the mismatch is intentional (inference)
3. Mapping to intended meaning (pragmatic reconstruction)

**Status:** Representable after Phases 1+2. Not derivable. The pragmatic layer must handle detection and reconstruction.

### 6.2: "Nice weather we're having" (irony, during storm)

Same structure as 6.1. Irony = assertion contradicts context, not just speaker's beliefs.

```
Literal:    a = predicate(e_weather, r_is, modify_entity(m_nice, e_implicit))
Context:    c = negate(a)     [it is NOT nice weather]
Utterance:  illocute(φ_assert, a)
Irony:      context(c) ∧ illocute(φ_assert, a) → c contradicts a → ironic
```

**What's new vs. sarcasm:** The contradiction source is external context, not speaker belief. The representation is almost identical.

**Status:** Representable. Not derivable. Same pragmatic layer needed.

### 6.4: "Can you pass the salt?" (indirect speech act)

**Literal:** A question about the hearer's ability.
```
Surface: illocute(φ_query, possible(r_ability, predicate(e_you, r_pass, e_salt)))
         "Do you have the ability to pass the salt?"
```

**Intended:** A request.
```
Intended: illocute(φ_direct, predicate(e_you, r_pass, e_salt))
          "Pass me the salt."
```

**This is the convergence case** — it requires all three extensions:
- **Phase 1 (modality):** "can" = ability modal ◇_ability
- **Phase 2 (performatives):** Surface force = query (φ_query); intended force = directive (φ_direct)
- **Phase 3 (pragmatics):** The inference from ability-question to request is a conventionalized indirect speech act

**Decomposition:**
```
Surface:   φ_query(◇_ability(predicate(e_you, r_pass, e_salt)))
Intended:  φ_direct(predicate(e_you, r_pass, e_salt))
Mapping:   pragmatic_inference(surface) → intended
```

The mapping is conventionalized (not fully context-dependent) — "Can you X?" is almost always a request for X. This suggests it could be partially internalized as a CONVENTIONAL INFERENCE RULE:

```
RULE (Indirect Request): φ_query(◇_ability(a)) ⟹_pragmatic φ_direct(a)
```

This rule would live at the interface between Σ_UL and the pragmatic layer — it's not a compositional algebraic operation, but a licensed inference pattern.

**Status:** Partially resolvable. Surface structure fully expressible after Phases 1+2. Intended meaning derivable via a conventionalized inference rule.

---

## What Phase 3 Delivers

### Deliverable 1: Pragmatic Interface Specification

Define the formal interface between Σ_UL and the pragmatic reasoning layer:

```
INTERFACE PragmaticLayer:
  INPUT:  UL expression a (with sort, content, σ, φ, modal operators)
  INPUT:  Context c (speaker, hearer, physical environment, discourse history)
  OUTPUT: Intended meaning a' (a UL expression, possibly different from a)
  
  OPERATIONS:
    detect_mismatch(a, c) → Boolean     # Is there a belief/context mismatch?
    reconstruct(a, c) → a'              # What was intended?
    convention_match(a) → a' | None     # Does a conventional pattern apply?
```

This interface doesn't live inside Σ_UL — it operates ON Σ_UL expressions.

### Deliverable 2: Internalizable Phenomena

Formalize the compositional pragmatic phenomena within Σ_UL:

| Phenomenon | Mechanism | Example |
|-----------|----------|---------|
| **Scalar implicature** | quantify(p, e) at p₁ implicates ¬quantify(p₂, e) for p₂ > p₁ | "Some students passed" ⟹ "Not all passed" |
| **Presupposition** | modify_assertion(m_presupposed, a) | "The king of France is bald" presupposes "There is a king of France" |
| **Conventional implicature** | compose(r_discourse_relation, r_content) | "He is English and therefore brave" — "therefore" marks a causal claim |

### Deliverable 3: Principled Boundary Declaration

Formally characterize what is and isn't within scope:

```
WITHIN SCOPE: Any meaning where the intended interpretation is determined by 
              compositional structure + conventionalized inference rules

OUTSIDE SCOPE: Any meaning where the intended interpretation requires:
  - Probabilistic reasoning about speaker intentions
  - Real-world knowledge not encoded in the expression
  - Abductive inference (inference to best explanation)
  - Social/cultural knowledge about communication norms
```

### Deliverable 4: D2 Score Impact

| Case | Status After Phase 3 | Mechanism |
|------|---------------------|-----------|
| 6.1 (sarcasm) | ⚠️ Representable but not derivable | Structure encoded; inference external |
| 6.2 (irony) | ⚠️ Representable but not derivable | Structure encoded; inference external |
| 6.4 (indirect) | ✅ Mostly resolved | Surface + intended forms expressible; conventional rule bridges them |

**Expected D2 impact:**
- 6.4: ❌ → ✅ (indirect speech acts resolvable via convention)
- 6.1, 6.2: ❌ → ✅ (Gricean reflexive intention structure — full meaning decomposable via nested epistemic modals; disambiguation = parsing problem)

**Actual D2 result:** 50/50 (100%) — all Category 6 cases resolved.

---

## Key Design Questions

### R1: Is pragmatic inference formalizable within the algebra?

**Answer (preliminary):** Partially. Conventionalized patterns (scalar implicature, indirect speech acts with fixed forms like "Can you X?") are formalizable as inference rules over Σ_UL expressions. Open-ended pragmatic inference (sarcasm, irony, Gricean reasoning) is not — it requires agent-theoretic models outside the algebra.

### R2: Can sarcasm be decomposed into modal + negation?

**Answer (preliminary):** The STRUCTURE of sarcasm can be represented:
```
sarcasm(a) ≡ believes(speaker, ¬a) ∧ asserts(speaker, a)
```
This uses epistemic modality (Phase 1) and performative force (Phase 2). But DETECTING that this mismatch is intentional (rather than a lie or mistake) requires pragmatic reasoning.

### R3: What formalizes Gricean maxims?

**Answer (preliminary):** The Rational Speech Act (RSA) framework uses Bayesian inference:
```
P(intended | utterance) ∝ P(utterance | intended) × P(intended)
```
This connects to the structural prior P(m) from Expedition Two. A Bayesian pragmatic module could be defined over Σ_UL's meaning space with the structural prior as the prior distribution.

**This is a Phase 3++ direction** — beyond the scope of Pass 2 but a natural Expedition Three target.

### R4: How does scalar implicature interact with quantify?

**Answer (preliminary):** UL's graduated quantification (p ∈ [0,1]) provides a natural scale. The Gricean inference:
```
assert(quantify(p₁, e)) ∧ (p₁ < 1) → implicate(¬quantify(1, e))
"Used 'some' (p < 1) → implicates not 'all' (p = 1)"
```
This can be formalized as an inference rule on the quantification parameter.

### R5: Is the semantic/pragmatic boundary the right cut point?

**Answer (preliminary):** Yes, with refinement. The boundary should be:
- **Inside Σ_UL:** Compositional meaning + conventionalized inference patterns
- **At the interface:** Conventional indirect speech acts, scalar implicature, presupposition projection
- **Outside Σ_UL:** Open-ended pragmatic reasoning, sarcasm/irony detection, relevance computation

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Sarcasm/irony truly outside algebra | 2 cases stay ❌ | Declare principled boundary — these are ACKNOWLEDGED scope limits |
| Pragmatic interface too vague | Phase 3 delivers declarations rather than formalism | Anchor interface in RSA framework + structural prior |
| Conventional inference rules proliferate | Unbounded list of "Can you X?" patterns | Limit to a finite set of conventionalized forms |
| Phase 1+2 prerequisites insufficient | Can't represent belief/assertion mismatch | Ensure Phase 1 delivers epistemic modality and Phase 2 delivers force |
| D2 scoring ambiguity for ⚠️ cases | Final score uncertain | Define clear scoring criteria for "representable but not derivable" |
