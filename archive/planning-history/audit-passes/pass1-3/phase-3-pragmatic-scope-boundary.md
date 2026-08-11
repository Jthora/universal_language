# Phase 3 — Pragmatic Scope Boundary

**Status:** ✅ COMPLETE
**New operations:** 0  
**D2 cases affected:** 6.3, 6.5  
**Expected case conversions:** 2 (⚠️ → split verdict: literal ✅, pragmatic inference declared out-of-scope)  
**Dependencies:** None  
**Blocks:** None

---

## Problem Statement

Two D2 cases in Category 6 (Irony/Pragmatics) are scored ⚠️ because their *literal semantic content* is expressible but their *pragmatic inference* is not.

| Case | Expression | Literal Content | Pragmatic Inference |
|------|-----------|----------------|-------------------|
| 6.3 | "Some students passed" | ∃x. Student(x) ∧ Passed(x) — **expressible ✅** | Scalar implicature: "not all" — **not expressible** |
| 6.5 | "He's not the sharpest tool" | ¬(superlative(sharpness, he)) — **expressible ✅** | Litotic understatement: "he's actually quite dull" — **not expressible** |

In both cases:
- The compositional semantic content decomposes cleanly via Σ_UL⁺
- The pragmatic layer (what the speaker *implies* beyond what is *said*) is outside scope

The question is: should these be scored by their semantic content (→ ✅) or penalized for missing pragmatic inference (→ ⚠️)?

---

## The Semantics/Pragmatics Architecture

### Standard Linguistic Framework

The semantics/pragmatics distinction is foundational in formal linguistics (Grice 1975, Levinson 1983, 2000):

| Layer | What It Covers | Mechanism |
|-------|---------------|-----------|
| **Semantics** | What is said — truth-conditional content, compositional meaning | Formal composition (Montague, type theory, Σ_UL) |
| **Pragmatics** | What is implicated — context-dependent inferences beyond literal content | Gricean maxims, relevance theory, speech act theory |

Key properties:
- Semantics is **compositional** — meaning of the whole = function of meanings of parts
- Pragmatics is **non-compositional** — meaning depends on context, speaker intention, social convention
- UL is explicitly scoped to **compositional relational semantics** (see CRITIQUE.md §5.1)

### UL's Position

UL encodes the **semantic layer**. Pragmatic inference operates *on top of* semantically encoded content. This is an architectural boundary, not a limitation:

```
┌─────────────────────────────────────────┐
│  PRAGMATIC LAYER (outside UL scope)     │
│  - Scalar implicature                   │
│  - Ironic inversion                     │
│  - Indirect speech acts                 │
│  - Conversational maxims                │
│  INPUT: UL-encoded semantic content     │
│  OUTPUT: enriched/inverted meaning      │
├─────────────────────────────────────────┤
│  SEMANTIC LAYER (UL scope)              │
│  - Compositional meaning                │
│  - Predication, modification, negation  │
│  - Quantification, binding, embedding   │
│  OUTPUT: truth-conditional content      │
└─────────────────────────────────────────┘
```

The pragmatic layer *requires* a semantic layer as input. UL provides that input. A future pragmatic extension could operate over UL-encoded content — but the core algebra is not the right place for context-dependent inference.

---

## Case-by-Case Analysis

### 6.3 "Some students passed" (Scalar Implicature)

**Literal content:**
```
bind(○_x, pred(○_x, r_pass, e_exam)) where ○_x ← me(m_∃, e_student)
```
= "There exists a student who passed" — **fully expressible ✅**

**Pragmatic inference:** "...and not ALL students passed" (Gricean: speaker used "some" instead of "all," so "all" must not hold — maxim of quantity).

**The implicature's CONTENT is also expressible:**
```
conj(
  bind(○_x, pred(○_x, r_pass, e_exam)),       # some passed
  neg(bind(○_y, pred(○_y, r_pass, e_exam)))    # not all passed
) where ○_x ← me(m_∃, e_student), ○_y ← me(m_∀, e_student)
```

The issue is NOT that UL can't express "some but not all." It CAN. The issue is that UL has no mechanism for **deriving** "not all" from the utterance of "some" — the Gricean inference process requires context, speaker model, and conversational maxims.

**Resolution:** ✅ — The semantic content is clean. The pragmatic derivation process is out of scope (and was always declared so). The fact that the implicature's content is ALSO expressible when stated explicitly demonstrates that the limitation is at the inference layer, not the representation layer.

### 6.5 "He's not the sharpest tool in the shed" (Litotes + Metaphor)

**Literal content:**
```
a₁ = pred(e_he, r_is, me(m_superlative, abs(e_tool_sharpness)))  # "he is the sharpest"
neg(a₁)  # "he is NOT the sharpest"
```

The metaphorical mapping (tool → person, sharpness → intelligence) is handled via `abstract`:
```
m_sharp = abs(e_sharpness)    # extract sharp-quality
m_tool  = abs(e_tool)         # extract tool-quality (→ intelligence metaphor)
```

**Expressible ✅** as literal negation of superlative with metaphorical mapping.

**Pragmatic inference:** The litotic understatement — "not the sharpest" in context means "actually quite dull." This requires recognizing that negating a positive attribute in certain social contexts implies the strong negative. That's pragmatic inference (face-saving, politeness conventions, scalar reasoning).

**Resolution:** ✅ — Literal compositional content (negation + metaphor) is clean. The pragmatic intensification ("not sharp" → "dull") is a contextual inference process, not a compositional operation.

---

## Scoring Decision

**Option A: Score as ✅ (literal content is clean)**

Both cases have fully clean Σ_UL⁺ decompositions for their *semantic content*. The missing piece is a pragmatic inference engine, which:
- Is non-compositional
- Is context-dependent
- Is explicitly outside UL's declared scope
- Would be a separate layer operating on UL output

**Option B: Score as split (literal ✅, mark pragmatic gap)**

Keep the ✅ score but add a note: "Pragmatic inference layer not modeled. UL provides the semantic input; pragmatic enrichment is architecturally separate."

**Recommendation:** Option A for the D2 score (these ARE clean decompositions). Option B for the documentation (acknowledge the architectural boundary explicitly).

---

## Deliverables

- [ ] Re-score D2 cases 6.3, 6.5 from ⚠️ to ✅ with updated decompositions and pragmatic-boundary notes
- [ ] Add a "Semantic/Pragmatic Architecture" section to CRITIQUE.md §5 (Scope Limitations)
- [ ] Update CRITIQUE.md D2 table: Category 6 from 0/2/3 to 2/0/3
- [ ] Resolution log entry for Phase 3
- [ ] Optional: Add architectural diagram (semantic layer → pragmatic layer) to gap-analysis.md or CRITIQUE.md

---

## Relationship to Existing ❌ Cases

The 3 existing ❌ cases in Category 6 (6.1 sarcasm, 6.2 verbal irony, 6.4 indirect speech act) are already scored as failures *because* pragmatic inference is out of scope. The Phase 3 resolution is consistent with that existing classification — it just clarifies that 6.3 and 6.5 are cases where the literal content IS clean (unlike 6.1/6.2 where the literal content *contradicts* the intended meaning, or 6.4 where the literal form is interrogative but the intent is imperative).

The gradient:
- **6.3, 6.5:** Literal content = intended content + additional implicature → ✅ (content expressible)
- **6.1, 6.2:** Literal content ≠ intended content (inversion) → ❌ (non-compositional meaning)
- **6.4:** Literal form ≠ intended force (question → request) → ❌ (illocutionary, not semantic)
