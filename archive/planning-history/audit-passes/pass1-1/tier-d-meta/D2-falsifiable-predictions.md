# D2 — Does UL Make Falsifiable Predictions?

**Tier:** D (Meta)  
**Targets:** `frontier/causal-efficacy-protocol.md`, `experiments/`, `foundations/formal-foundations.md`  
**Dependencies:** B4 (predictions require the correct operation set)  
**Effort:** 2–4 weeks for prediction generation; months for testing  
**Status:** ✅ RESOLVED (April 2026) — 50-case completeness challenge completed (`experiments/D2-completeness-challenge.md`). Five structural predictions formalized (`experiments/D2-structural-predictions.md`). Results integrated into CRITIQUE.md §4.6. Utility testing (Workstream C) remains as future work.

---

## The Question

A formal system can be mathematically consistent but empirically vacuous. UL claims to describe how ALL meaning works. If it can't be wrong about anything specific, it explains everything and predicts nothing — which makes it unfalsifiable.

### What Falsifiability Requires

A falsifiable prediction has the form: "If UL is correct, then X should be observed. If not-X is observed, UL is wrong (or requires revision)."

The prediction must be:
1. **Specific** — X is a definite observable outcome
2. **Risky** — X is not trivially true or true by definition
3. **Unique** — X is predicted by UL but NOT by competing theories (otherwise, observing X doesn't distinguish UL from alternatives)

## Current State

### The Causal Efficacy Protocol

`frontier/causal-efficacy-protocol.md` describes 5 experiments testing whether UL-structured prompts produce measurably different outputs than controls. This is a test of UL's *utility*, not its *truth* — it asks "does using UL help?" rather than "is UL correct?"

Even if UL-structured prompts produce better outputs, this doesn't prove UL is the CORRECT theory of meaning — it only proves UL is a USEFUL framework. Many useful frameworks are not uniquely correct.

### What's Missing: Structural Predictions

UL's distinctive claims are structural:
- **5 primitives, not 4 or 6** — there should be exactly 5 irreducible semantic dimensions
- **4 sorts, not 3 or 5** — meaning-bearing expressions decompose into exactly 4 types
- **11 operations** (or fewer per B4) — these are ALL the ways meanings combine
- **Geometric grounding** — meaning structure is isomorphic to plane geometry

Each of these makes a falsifiable prediction.

## Proposed Predictions

### Prediction 1: The 5-Primitive Claim
**Prediction:** Any corpus of natural language expressions, when analyzed for irreducible semantic dimensions, will yield exactly 5 independent factors corresponding to Entity, Relation, Modifier, Process, and Concept.

**Test:** Factor analysis on large semantic datasets. If factor analysis consistently yields 4 or 6 factors instead of 5, the primitive count is wrong.

**Risk level:** HIGH — factor analysis of existing semantic datasets (WordNet, FrameNet, ConceptNet) may contradict the 5-factor prediction.

### Prediction 2: The 4-Sort Claim
**Prediction:** Every well-formed meaning-bearing expression in any natural language can be decomposed into exactly 4 types of components (entities, relations, modifiers, assertions) with no residue.

**Test:** Take a diverse corpus of expressions from multiple languages. Attempt to parse each into the 4-sort framework. Any expression that resists decomposition is a counterexample.

**Risk level:** MEDIUM — the 4-sort decomposition is similar to existing linguistic type systems (noun, verb, adjective, proposition), which have known exceptions (interjections, discourse particles, evidentials).

### Prediction 3: The Completeness Claim
**Prediction:** The 11 (or N-independent) operations are sufficient to construct ANY meaning-bearing expression from primitives.

**Test:** Generate complex semantic constructions (irony, metaphor within metaphor, self-referential statements, pragmatic implicature) and attempt to express each using only the Σ_UL operations. Any inexpressible construction is a counterexample.

**Risk level:** HIGH — irony, sarcasm, and pragmatic effects are notoriously difficult to formalize. If these require operations beyond Σ_UL, the completeness claim fails.

### Prediction 4: The Geometric Grounding Claim
**Prediction:** Every Σ_UL operation has a geometric interpretation that faithfully preserves the algebraic structure. Specifically: the semantic distance between two meanings equals (or is monotonically related to) the geometric distance between their representations.

**Test:** Define a semantic similarity metric (e.g., human similarity judgments). Define a geometric distance in the UL representation space. Test whether the correlation is significantly above chance.

**Risk level:** MEDIUM — semantic similarity judgments are noisy, but if the geometric distance is ANTI-correlated with semantic similarity, the grounding claim is falsified.

### Prediction 5: The Universality Claim
**Prediction:** UL's 4-sort decomposition applies equally to all languages, including those with radically different grammatical structures (polysynthetic, ergative-absolutive, languages without adjectives).

**Test:** Analyze languages known to challenge universalist claims:
- Adyghe (polysynthetic, minimal nouns)
- Lao (no tense marking, no obligatory number)
- Pirahã (disputed: limited recursion, no embedded clauses)
- Sign languages (spatial grammar, simultaneous morphology)

If any language's meaning system resists UL decomposition, universality fails.

**Risk level:** VERY HIGH — linguistic typology has repeatedly shown that "universal" claims based on European languages fail on diverse samples.

## Evaluation Criteria

For UL to be considered falsifiable, it needs at least 3 predictions that are:
- Testable without API keys ($0 cost — corpus analysis, linguistic fieldwork reports)
- Specific enough to produce a definite pass/fail
- Documented in advance (pre-registered, not post-hoc justified)

## Deliverable

A document with:
1. 5+ falsifiable predictions derived from UL's structural claims
2. For each: specific test procedure, expected outcome, falsification condition
3. Risk assessment: which predictions are most likely to fail?
4. A pre-registration template for each prediction

## Status

**Status:** ❌ OPEN — UL has a utility protocol (causal efficacy) but no structural falsification framework. The predictions above are proposed, not yet tested.
