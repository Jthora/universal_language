# Phase 3 — Pragmatic Inference Layer Plan

**Goal:** Specify the formal interface between Σ_UL and the pragmatic reasoning layer, internalize what's compositional, and declare principled boundaries for what's not.

---

## 1. Architecture

```
┌───────────────────────────────────────────────────┐
│              PRAGMATIC INFERENCE LAYER             │
│                                                   │
│  RSA Module          Convention Module             │
│  ┌──────────────┐   ┌─────────────────────────┐  │
│  │ P(meaning|u) │   │ "Can you X?" ⟹ "Do X"  │  │
│  │ Bayesian     │   │ Scalar implicature       │  │
│  │ inference    │   │ Presupposition           │  │
│  └──────┬───────┘   └───────────┬─────────────┘  │
│         │                       │                  │
│         └───────────┬───────────┘                  │
│                     │                              │
│              ┌──────▼──────┐                       │
│              │  INTERFACE   │                       │
│              │  detect()    │                       │
│              │  resolve()   │                       │
│              │  convention()│                       │
│              └──────┬──────┘                       │
└─────────────────────┼─────────────────────────────┘
                      │
              ┌───────▼───────┐
              │   Σ_UL⁺       │
              │   + Modality  │
              │   + Force     │
              │   (4 sorts,   │
              │    13 ops)    │
              └───────────────┘
```

---

## 2. Interface Specification

```
INTERFACE PragmaticInterface:

  # Type definitions
  ULExpression  = well-formed Σ_UL⁺ term (with σ, φ, modal operators)
  Context       = {speaker: Entity, hearer: Entity, 
                   environment: Set<Assertion>, 
                   discourse: List<ULExpression>}
  
  # Core operations
  literal_meaning   : ULExpression → ULExpression
    # Identity — the compositional meaning IS the literal meaning
  
  detect_mismatch   : ULExpression × Context → {none, scalar, conventional, pragmatic}
    # Classifies whether the literal meaning needs pragmatic enrichment
    
  resolve_scalar    : ULExpression → ULExpression
    # Applies scalar implicature rules (Section 3)
    
  resolve_convention: ULExpression → ULExpression | None
    # Applies conventionalized inference patterns (Section 4)
    
  resolve_pragmatic : ULExpression × Context → ULExpression
    # Full pragmatic inference (RSA model) — Section 5
    
  # Composition
  interpret : ULExpression × Context → ULExpression
  interpret(u, c) = 
    let m = detect_mismatch(u, c) in
    case m of
      none         → literal_meaning(u)
      scalar       → resolve_scalar(u)
      conventional → resolve_convention(u) ?? literal_meaning(u)
      pragmatic    → resolve_pragmatic(u, c)
```

---

## 3. Scalar Implicature Rules

### Rule Set (finite, fixed)

```
RULE SI-1 (Quantifier Scale):
  assert(quantify(p, e)) where p < 1
  ⟹_default ¬quantify(1, e)
  "Some → not all"
  
  Cancellable by: explicit assertion of stronger quantifier

RULE SI-2 (Disjunction):
  assert(disjoin(a, b))
  ⟹_default ¬conjoin(a, b)
  "A or B → not both A and B"
  
  Cancellable by: explicit assertion of conjunction

RULE SI-3 (Possibility → Not Necessity):
  assert(possible(r, a))
  ⟹_default ¬necessary(r, a)
  "Might → not must"
  
  Cancellable by: explicit assertion of necessity
  NOTE: Requires Phase 1 (modality)
```

### Properties

- **Finite:** 3 rules (+ potential extensions for numeral scales, temporal scales)
- **Defeasible:** Each rule is a default inference, cancellable by explicit assertion
- **Scale-based:** All rules derive from an ordering on expression strength
- **Compositional:** The rules reference only algebraic structure (p values, modal operators, connectives)

---

## 4. Conventionalized Inference Patterns

### Pattern Set (finite, extendable)

```
PATTERN CI-1 (Indirect Request):
  φ_query(◇_ability(a))
  ⟹_convention φ_direct(a)
  "Can you X?" → "Do X"
  
  Conditions: hearer could plausibly do X; context is request-appropriate

PATTERN CI-2 (Indirect Offering):
  φ_query(◇_ability(predicate(e_speaker, r, e_hearer)))
  ⟹_convention φ_commit(predicate(e_speaker, r, e_hearer))
  "Can I help you?" → "I offer to help you"

PATTERN CI-3 (Rhetorical Question):
  φ_query(a) where a is obviously true/false in context
  ⟹_convention φ_assert(a) / φ_assert(¬a)
  "Is the sky blue?" (obviously yes) → assertion that sky IS blue
```

### Properties

- **Pattern-based:** Each pattern matches a structural template in Σ_UL
- **Conventional:** The mapping is culturally established, not derived from first principles
- **Context-gated:** Patterns activate only when contextual conditions are met
- **Finite:** Bounded set of conventionalized forms

---

## 5. RSA Connection (Specification, Not Implementation)

### Architecture

The Rational Speech Act module takes Σ_UL expressions as input and reasons about speaker intentions using Bayesian inference.

```
P_listener(meaning | utterance, context) ∝ 
  P_speaker(utterance | meaning, context) × P(meaning)
```

**Connection points to Σ_UL / Expedition Two:**

| RSA Component | Σ_UL Connection |
|---------------|-----------------|
| **Utterance space** | Well-formed Σ_UL expressions |
| **Meaning space** | Σ_UL meaning space M |
| **P(meaning)** | Structural prior P(m) from probability-and-information.md |
| **cost(utterance)** | Description complexity DC_UL |
| **Rationality λ** | Free parameter (not in Σ_UL) |

### What RSA handles

| Case | RSA Process |
|------|-------------|
| 6.1 (sarcasm) | P(sarcastic | "great" + bad context) >> P(sincere | "great" + bad context) because sincerity violates Quality maxim |
| 6.2 (irony) | P(ironic | "nice weather" + storm) >> P(sincere | "nice weather" + storm) because sincerity contradicts shared knowledge |
| Non-literal metaphor | P(metaphorical | utterance) computed via meaning-space proximity (Fisher metric) |

### Implementation status

**Not implemented in Pass 2.** Pass 2 specifies the interface; implementation is an Expedition Three deliverable. The RSA module requires:
- A working structural prior P(m)
- A working complexity measure DC_UL
- Computational infrastructure for Bayesian inference over Σ_UL expressions

All three prerequisites are partially developed in Expedition Two.

---

## 6. Principled Boundary Declaration

### Formal statement

```
SCOPE BOUNDARY (Pragmatic Inference):

Σ_UL (including modal and force extensions) is a universal notation for
COMPOSITIONAL RELATIONAL SEMANTICS — meaning that is determined by the 
meanings of parts and their mode of combination.

The following phenomena are OUTSIDE this scope:

1. SARCASM / IRONY: Meaning depends on mismatch between literal content
   and speaker belief/context. The STRUCTURE of the mismatch is representable
   in Σ_UL (via epistemic modality + force); the DETECTION and RESOLUTION
   require probabilistic agent-theoretic reasoning.

2. OPEN INDIRECT SPEECH ACTS: Where the intended force cannot be predicted
   from the surface form alone (unlike conventionalized patterns like 
   "Can you X?").

3. RELEVANCE-BASED INFERENCE: Where the intended meaning is computed by
   considering what would be relevant in context (Sperber & Wilson 1986).

4. SOCIAL/CULTURAL MEANING: Politeness strategies, register shifts,
   face-saving moves, power dynamics.

These phenomena are handled by the PRAGMATIC INFERENCE LAYER, which operates
ON Σ_UL expressions, not within them.
```

### D2 impact

| Case | Before | After Phase 3 | Mechanism |
|------|--------|---------------|-----------|
| 6.1 | ❌ | ⚠️ (representable, not derivable) | Structure via modal + force; inference external |
| 6.2 | ❌ | ⚠️ (representable, not derivable) | Structure via modal + force; inference external |
| 6.4 | ❌ | ✅ (resolved) | Conventional inference pattern CI-1 |

---

## 7. Deliverable Summary

| # | Deliverable | Type |
|---|-------------|------|
| 1 | Pragmatic interface specification | Formal definition |
| 2 | Scalar implicature rules (SI-1 through SI-3+) | Inference rules |
| 3 | Conventionalized inference patterns (CI-1 through CI-3+) | Pattern set |
| 4 | RSA-Σ_UL interface specification | Interface definition |
| 5 | Principled boundary declaration | Scope document |
| 6 | D2 re-analysis for Cat. 6 | Updated scoring |
| 7 | Updated gap-analysis.md | Scope boundary refinement |
