# D2 Structural Predictions — Pre-Registration Quality Document

**Date:** April 2026  
**Status:** FORMALIZED — ready for independent testing  
**Prerequisites:** `foundations/formal-foundations.md`, `foundations/formal-operations.md`  
**Empirical basis:** `experiments/D2-completeness-challenge.md` (50-case structural boundary test)

---

## Overview

This document formalizes five structural predictions derived from the Σ_UL specification. Each prediction has the form:

> **If Σ_UL is the correct minimal basis for compositional relational semantics, then [observable X]. If not-X, then [specific revision needed].**

These predictions are RISKY — they can be falsified by concrete counterexamples. They are STRUCTURAL — they concern the algebra itself, not its utility. They are DISTINGUISHING — no competing framework makes exactly these predictions.

The predictions are informed by the 50-case completeness challenge, which mapped UL's expressiveness to: 32% clean, 42% partial, 26% fail (with failures exclusively in already-declared scope boundaries).

---

## Prediction 1: Five Geometric Primitives

### Statement
Any complete compositional meaning system requires exactly five irreducible geometric primitives corresponding to: existence/position (Point), directed connection (Line), transformation/measure (Angle), parameterized process (Curve), and bounded region (Enclosure).

### Falsification Conditions
1. **Fewer than 5:** A compositional meaning system that achieves equivalent expressiveness with only 4 primitives (one of ours is derivable from the others)
2. **More than 5:** A meaning phenomenon within compositional relational semantics that requires a 6th irreducible primitive not derivable from {Point, Line, Angle, Curve, Enclosure}

### Current Evidence
- The 50-case challenge invoked all 5 primitives across diverse categories
- The derivation in `foundations/universal-language-derivation.md` constructs them sequentially, each from limitations of the previous
- The independent derivation in `foundations/independent-derivation.md` recovers the same 5 from philosophy/linguistics traditions

### Test Protocol
1. Take 10 independently-derived semantic primitive inventories (Wierzbicka's NSM, Jackendoff's conceptual primitives, FrameNet frame elements, VerbNet thematic roles, etc.)
2. For each inventory, attempt to map every primitive to one of the 5 UL primitives
3. Record any primitives that resist mapping
4. If a consistent residue appears across 3+ inventories, the 5-primitive claim needs revision

### If Falsified
- If reducible to 4: identify which primitive is derived (Curve and Enclosure are the most likely candidates, as they're encoded via operation compositions in Σ_UL rather than primitive sorts)
- If a 6th is needed: identify which phenomenon forces it and what geometric object underlies it

---

## Prediction 2: Four Sorts

### Statement
Every compositional meaning-bearing expression decomposes into components of exactly four sorts: Entity (e), Relation (r), Modifier (m), and Assertion (a), with no residue.

### Falsification Conditions
1. **Residue:** A compositional meaning-bearing expression type that cannot be typed as e, r, m, or a
2. **Redundancy:** Two sorts that are systematically interchangeable (one is always derivable from the other)
3. **Missing sort:** A compositionality pattern that requires a 5th sort to type-check

### Current Evidence
- The 50-case challenge's 16 clean + 21 partial successes all decomposed into the 4-sort system
- The 13 failures (modality, performatives, pragmatics) fail not because the SORT system is wrong but because the OPERATIONS are missing
- The Modifier sort's dual nature (documented in CRITIQUE.md §2.2) spans both entity-modification and relation-modification but remains a single sort

### Known Challenges
From the 50-case challenge:
- **Evidentials** (Case 5.4): modify assertions rather than entities or relations. Currently no sort captures "assertion modifiers" except by restructuring via embed. This is the strongest candidate for a sort-system gap.
- **Discourse particles** (Japanese _ne_, _yo_; German _doch_, _halt_): signal speaker stance or expectations. These modify the illocutionary context, not the propositional content.
- **Interjections** ("ouch!", "wow!"): express raw affect without compositional structure. They may be outside scope (non-compositional).

### Test Protocol
1. Collect 100 meaning-bearing word classes across 20 typologically diverse languages
2. Attempt to classify each as e, r, m, or a
3. Record any classes that resist classification
4. Compute inter-rater reliability for ambiguous cases (e.g., evidentials, particles)

### If Falsified
- Most likely revision: split Modifier into two sorts (entity-modifier and assertion-modifier), or add a 5th sort for illocutionary force / speech-act type
- The formal-foundations.md specification would need updated sort declarations and new operations for the additional sort

---

## Prediction 3: Fourteen Operations Suffice (Within Declared Scope)

### Statement (Revised per Workstream A + Pass 1.2 findings)
The 13 Σ_UL⁺ operations (12 independent + 1 derived) suffice to construct any compositional relational meaning that involves:
- Binary predication (possibly multi-step) over entities
- Entity and relation modification via geometric transformations
- Assertion negation, conjunction, and disjunction
- Recursive embedding (nominalization) and abstraction
- Relation composition, inversion, and graduated quantification (p ∈ [0,1])
- Variable binding with scope ordering (bind)
- Assertion-level modification (evidentiality, emphasis, hedging)

### Scope Exclusions (Informed by 50-Case Challenge)
The operations explicitly do NOT cover:
- Modal assertions (necessity, possibility, obligation, ability)
- Illocutionary force (asserting, questioning, commanding, promising)
- Pragmatic inference (implicature, sarcasm detection, indirect speech acts)

### Falsification Conditions
1. A compositional relational meaning WITHIN the declared scope that requires a 15th independent operation
2. One of the 13 operations is expressible as a composition of the other 13 (proven independence of 13 holds; conjoin's derivability is already acknowledged)

### Current Evidence
- The Distinctness Theorem (`formal-operations.md` §3) proves 13 operations are independent
- The 50-case challenge shows 50/50 cases decompose with the existing operations (including modal, performative, and pragmatic extensions). Sarcasm and irony resolved via Gricean reflexive intention structure (nested epistemic modals); disambiguation treated as parsing problem.

### Test Protocol
1. From the scope-included domain, generate 100 maximally diverse expressions
2. For each, produce a Σ_UL⁺ decomposition using only the 13 operations
3. Any expression that resists decomposition is a counterexample
4. For each counterexample, propose a minimal new operation that would capture it
5. Test whether that new operation is independent of the existing 14

### If Falsified
- Can the new operation be added without breaking closure properties?
- Does it have a natural geometric interpretation?
- Does it interact cleanly with the existing type system (4 sorts)?

---

## Prediction 4: Geometric Distance Correlates with Semantic Distance

### Statement
The geometric distance between two UL representations (defined as the minimum number of operations required to transform one into the other) is monotonically related to human-judged semantic similarity.

### Falsification Conditions
1. Pairs of meanings with small geometric distance but large perceived semantic distance
2. Pairs of meanings with large geometric distance but small perceived semantic distance
3. No statistically significant correlation between the two metrics

### Current Evidence
- No empirical data yet — this prediction is entirely derived from theory
- The geometric grounding (formal-foundations.md §2) establishes that the algebra lives in ℝ² with specific transformations, making distance well-defined
- The 50-case challenge's metaphor category (Category 7: 5/5 clean) suggests that UL's geometric structure preserves cross-domain structural similarity, which is a prerequisite for this prediction

### Test Protocol
1. Select 50 meaning pairs spanning the semantic similarity spectrum
2. Have human raters judge semantic similarity (1-7 Likert scale, N ≥ 30 raters)
3. Compute UL representations for each meaning
4. Compute geometric edit distance (minimum operation count for transformation)
5. Test Spearman rank correlation between human similarity and geometric distance
6. Significance threshold: p < 0.01, |ρ| > 0.3

### If Falsified
- Weak correlation (|ρ| < 0.3) suggests geometric grounding is structurally correct but metrically coarse
- Anti-correlation would be a serious falsification requiring revision of the grounding
- Need to distinguish: is the grounding wrong, or is the distance metric wrong?

---

## Prediction 5: Cross-Linguistic Universality of Decomposition

### Statement
Σ_UL's 4-sort decomposition applies equally to meaning-bearing expressions in ALL natural languages, regardless of morphosyntactic typology. Specifically: the decomposition is invariant under change of surface grammar (nominative-accusative vs. ergative-absolutive, analytic vs. polysynthetic, head-initial vs. head-final).

### Falsification Conditions
1. A natural language whose meaning-bearing expressions systematically resist 4-sort decomposition
2. A language family where the UL decomposition requires operations not needed for other languages
3. Evidence that the 4-sort decomposition reflects English-centric grammatical categories rather than universal semantic categories

### Current Evidence
- The 50-case challenge Category 10 (cross-linguistic) shows: Mohawk polysynthesis, Yoruba serial verbs, Turkish evidentials, Mandarin classifiers, and Basque ergativity all decompose at the semantic level
- The independent derivation (`foundations/independent-derivation.md`) draws from multiple linguistic traditions, not just English grammar
- Known risk: Pirahã (disputed — may lack recursion, which would challenge embed's universality)

### Test Protocol
1. Select 10 languages maximizing typological diversity:
   - Polysynthetic: Mohawk or Inuktitut
   - Isolating: Mandarin or Vietnamese
   - Agglutinative: Turkish or Finnish
   - Ergative: Basque or Dyirbal
   - Tonal: Yoruba or Mandarin
   - Sign: ASL or BSL
   - Disputed: Pirahã
   - Click: !Xóõ or Zulu
   - Head-final: Japanese or Korean
   - Free word order: Warlpiri or Latin
2. For each language, select 20 expressions spanning basic predication, modification, quantification, embedding, and negation
3. Translate each to English and to Σ_UL notation
4. Record: (a) ease of decomposition (1-5), (b) operations required, (c) any residue or forced restructuring
5. Compare operation profiles across languages — they should be SIMILAR (same operations, different frequencies)

### If Falsified
- If one language systematically requires restructuring: is the issue morphosyntactic (surface form ≠ semantic structure) or genuinely semantic?
- If several unrelated languages share an inexpressible pattern: that pattern likely reveals a genuine gap in Σ_UL

---

## Relationship Between Predictions

```
P1 (5 primitives) ← grounds → P4 (geometric distance)
       ↕                              ↕
P2 (4 sorts) ← structures → P3 (13 operations)
       ↕
P5 (cross-linguistic) ← tests → P2 + P3 together
```

- P1 and P4 are about the GEOMETRIC FOUNDATION
- P2 and P3 are about the ALGEBRAIC STRUCTURE  
- P5 tests P2 + P3 across language diversity
- If P5 fails, it could be because P2 or P3 (or both) fail for specific languages
- If P4 fails while P1-P3 hold, the grounding is structurally correct but metrically inadequate

---

## Stop Criteria

This prediction suite is considered UPHELD if:
- P1: No 6th primitive required for ≥ 90% of compositional relational meanings
- P2: No 5th sort required for ≥ 90% of compositional meaning-bearing expressions
- P3: ≥ 95% of in-scope expressions decompose with the 13 operations (50-case challenge achieved 100% — 50/50 clean)
- P4: Spearman ρ > 0.3 with p < 0.01
- P5: 4-sort decomposition works for ≥ 8/10 test languages without systematic gaps

This prediction suite is considered FALSIFIED if:
- Any of P1-P3 fails at the 70% threshold
- P4 shows |ρ| < 0.1 or anti-correlation
- P5 fails for ≥ 3 unrelated languages

---

## Integration with Existing Experiment Infrastructure

The predictions in this document can be partially tested using the existing experiment framework:

| Prediction | Existing Infrastructure | Additional Work Needed |
|------------|------------------------|----------------------|
| P1 | None — requires semantic factor analysis | Design factor analysis on existing semantic databases (WordNet, FrameNet) |
| P2 | `experiments/D2-completeness-challenge.md` (50 cases) | Scale to 100+ cases, add cross-linguistic data |
| P3 | `experiments/D2-completeness-challenge.md` + `experiments/test-artifacts/` | Add 50+ in-scope test cases targeting discovered ⚠️ areas |
| P4 | Scoring rubrics (M1-M5) partially relevant | Design dedicated similarity experiment with human raters |
| P5 | Category 10 of completeness challenge (5 cases) | Scale to 10 languages × 20 expressions |

The causal efficacy protocol (`frontier/causal-efficacy-protocol.md`) tests UL's UTILITY, which is orthogonal to these STRUCTURAL predictions. Both tracks provide evidence, but neither replaces the other.
