# Pass 2 — Algebraic Extension: Modality, Performatives, Pragmatics

**Status:** ✅ COMPLETE (see [pass2-summary.md](pass2-summary.md))  
**Prerequisite:** Pass 1 complete (Σ_UL⁺: 4 sorts, 13 operations, D2 at 74%)  
**Result:** D2 at 50✅/0⚠️/0❌ (100%). Zero new primitives, sorts, or operations.  
**Scope:** Theoretical and formal work only — no empirical testing in this pass

---

## The Problem

The D2 completeness challenge tested 50 meaning-bearing expressions against Σ_UL⁺. Result: 37✅ / 0⚠️ / 13❌.

All 13 failures fall into three categories, each representing a distinct class of meaning that the current algebra cannot express:

| Category | ❌ Count | What's Missing | Algebraic Gap |
|----------|---------|----------------|---------------|
| **Modality** (Cat. 4) | 5 | Possibility, necessity, obligation, ability, counterfactuals | No modal operators; σ ∈ {⊕, ⊖} has no modal dimension |
| **Performatives** (Cat. 9) | 5 | Promising, commanding, declaring, apologizing, betting | No illocutionary force; UL encodes content but not speech-act type |
| **Pragmatic inference** (Cat. 6) | 3 | Sarcasm, irony, indirect speech acts | Non-compositional meaning; requires agent-theoretic reasoning |

---

## Strategic Architecture

Pass 2 has **four phases**, ordered by dependency:

```
  Phase 0: CONSOLIDATION
  (tighten existing proofs, clean stale refs, close easy gaps)
      │
      ▼
  Phase 1: MODALITY  ←── Most mature groundwork, no dependencies
  (5 ❌ cases: 4.1–4.5)
      │
      ├──────────────────────┐
      ▼                      ▼
  Phase 2: PERFORMATIVES     │  ←── Depends partially on Phase 1
  (5 ❌ cases: 9.1–9.5)      │
      │                      │
      ▼                      │
  Phase 3: PRAGMATICS  ◄────┘  ←── Depends on both Phase 1 and 2
  (3 ❌ cases: 6.1, 6.2, 6.4)
```

### Why This Order

1. **Phase 0 first** — Don't extend on an inconsistent base. ~15 stale "11 operations" refs remain. Several open proofs are closable.
2. **Modality first** — Logic-parametric architecture proven (§6.4 of formal-foundations.md): the 4 sorts survive modal logic. Categorical framework (Lang(Σ_UL)) ready. Strategic roadmap exists (strategic-plan.md Region 5). No dependency on other extensions.
3. **Performatives second** — Several performatives involve modal dimensions (promises → obligation, bets → possibility). The modal apparatus informs how illocutionary force interacts with the assertion sort.
4. **Pragmatics third** — Requires both epistemic modality (to model speaker beliefs) and illocutionary force (to distinguish saying from doing). Case 6.4 ("Can you pass the salt?") is the convergence point requiring all three.

---

## The 13 ❌ Cases in Detail

### Modality — Category 4

| Case | Expression | Modal Type | What's Missing |
|------|-----------|------------|----------------|
| 4.1 | "She might be sleeping" | Epistemic possibility | ◇ operator |
| 4.2 | "You must leave now" | Deontic necessity | □_deontic operator |
| 4.3 | "He could have won if he had tried" | Counterfactual + ability | Possible-world semantics |
| 4.4 | "It is necessarily true that 2+2=4" | Alethic necessity | □_alethic operator |
| 4.5 | "She should have been able to finish earlier" | Compound: deontic + ability + counterfactual | Compositional modal calculus |

**Root cause:** σ ∈ {⊕, ⊖} distinguishes only asserted/denied. No mechanism for possibility, necessity, or obligation.

### Performatives — Category 9

| Case | Expression | Force Type | What's Missing |
|------|-----------|-----------|----------------|
| 9.1 | "I hereby pronounce you married" | Declarative | Utterance creates state of affairs |
| 9.2 | "I promise to return" | Commissive | Act of promising = creating obligation |
| 9.3 | "I apologize for the delay" | Expressive | Social repair act |
| 9.4 | "You're fired" | Institutional declarative | Institutional authority |
| 9.5 | "I bet you five dollars it rains" | Contractual | Contract created by utterance |

**Root cause:** UL encodes propositional content but not illocutionary force. All 5 cases have fully expressible content; the speech-act dimension is missing.

### Pragmatic Inference — Category 6

| Case | Expression | Pragmatic Type | What's Missing |
|------|-----------|---------------|----------------|
| 6.1 | "Oh great, another meeting" (sarcastic) | Sarcasm | Non-compositional: speaker means opposite |
| 6.2 | "Nice weather we're having" (ironic) | Irony | Context-contradiction inference |
| 6.4 | "Can you pass the salt?" (indirect) | Indirect speech act | Modality + force + pragmatic inference |

**Root cause:** Pragmatic meaning is non-compositional — meaning of whole contradicts/diverges from meaning of parts. Requires agent-theoretic reasoning about speaker intentions.

---

## Existing Groundwork

### What's Already Built

| Resource | Relevance | Phase |
|----------|-----------|-------|
| **Logic-parametric architecture** (formal-foundations.md §6.4) | 4 sorts + 9 operations survive modal logic unchanged. Only negate, conjoin, disjoin, quantify need modal instantiation | Phase 1 |
| **Lang(Σ_UL) category** (category-of-languages.md) | Embedding functor from Σ_UL → Σ_UL^modal is the natural framework for extension | Phase 1 |
| **Topos structure gap** (category-of-languages.md §VIII) | Subobject classifier → richer truth values → modal logic foundation | Phase 1 |
| **Gauge bundle E → X** (gauge-bundle-of-meaning.md) | Context space X ready; possible worlds could extend X | Phase 1 |
| **modify_assertion operation** | Assertion-level frame decoration already exists; modal operators could follow same pattern | Phase 1 |
| **bind operation** | Variable binding with scope — needed for "∀w ∈ W..." quantification over worlds | Phase 1 |
| **Structural prior P(m)** (probability-and-information.md) | Graded modality ("probably") could compose P with modal operators | Phase 1 |
| **Fisher metric** (Expedition Two Sprint 4) | "Closest possible world" for counterfactuals = geodesic distance | Phase 1 |
| **D2 prediction P2** (D2-structural-predictions.md) | Pre-identifies probable 5th sort for illocutionary force | Phase 2 |
| **Gauge field sections** (strategic-plan.md Region 1) | Speech acts as "sections of the bundle" — force = fiber direction | Phase 2 |
| **Rational Speech Act framework** (external) | Bayesian pragmatic inference; connects to structural prior | Phase 3 |

### What's Not Built

- No formal definition of possible worlds in the geometric framework
- No accessibility-relation algebra
- No illocutionary force formalization
- No agent-theoretic reasoning layer
- No Gricean maxim formalization

---

## Success Criteria

### Phase 0: Consolidation
- Zero stale operation counts in active documents
- ℚ construction explicitly translated to Σ_UL operations
- Embedding injectivity (Step 4) formalized
- Recursion depth proven

### Phase 1: Modality
- New operations defined with full geometric grounding
- All 5 Cat. 4 expressions decomposable
- Independence proofs for any new operations
- Montague intensional subsumption characterized
- Writing system updated for modal constructions

### Phase 2: Performatives
- Algebraic mechanism for illocutionary force
- All 5 Cat. 9 expressions decomposable
- Force-content compositionality proven or characterized
- Sort system impact assessed (4 sorts → 5?)

### Phase 3: Pragmatics
- Formal characterization of what is/isn't within scope
- Case 6.4 fully resolved (modality + force + inference)
- Cases 6.1, 6.2: either decomposed or principled boundary declared
- Pragmatic inference layer architecture specified

### Overall
- D2 score: target ≥ 45/50 (90%)+ or principled boundary for remaining ❌
- All new operations have independence proofs
- All writing system documents synchronized
- No regression in existing proofs

---

## Phase Overview

| Phase | Directory | Docs | Focus |
|-------|-----------|------|-------|
| 0 | `pass2-0-consolidation/` | stale-refs-sweep.md, open-proofs.md | Tighten base before extending |
| 1 | `pass2-1-modality/` | README.md, design-questions.md, kripke-realization.md, modal-operations.md | Possible-world semantics + modal operators |
| 2 | `pass2-2-performatives/` | README.md, design-questions.md, force-algebra.md | Illocutionary force + content/force composition |
| 3 | `pass2-3-pragmatics/` | README.md, design-questions.md, inference-layer.md | Agent-theoretic pragmatic reasoning |

See `CHECKLIST.md` for all deliverables and `DEPENDENCIES.md` for cross-phase dependencies.
