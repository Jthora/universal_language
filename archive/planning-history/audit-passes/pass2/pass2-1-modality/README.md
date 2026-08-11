# Phase 1 — Modal Semantics

**Status:** 📋 PLANNING  
**Target:** 5 ❌ cases (4.1–4.5)  
**Dependencies:** Phase 0 (consolidation) complete  
**Blocks:** Phase 2 (performatives), Phase 3 (pragmatics)

---

## Problem Statement

The assertional sign σ ∈ {⊕, ⊖} distinguishes only asserted from denied. Natural language routinely distinguishes five modal dimensions:

| Modal Type | Example | What's Needed |
|-----------|---------|---------------|
| **Alethic** | "It is necessarily true that 2+2=4" | Truth in ALL possible worlds |
| **Epistemic** | "She might be sleeping" | Truth in SOME worlds compatible with knowledge |
| **Deontic** | "You must leave now" | Truth in ALL worlds the agent considers ideal |
| **Ability/Dynamic** | "He could have won" | Truth in SOME worlds accessible via agent's capacities |
| **Counterfactual** | "If he had tried, he would have won" | Truth in the CLOSEST world where the antecedent holds |

UL has no mechanism for any of these. The formal-foundations.md §6.4 logic-independence analysis proves the 4-sort system survives modal logic — the sorts are ready, the operations are not.

---

## Approach: Three Sub-Phases

### 1A: Possible-World Foundation

**Goal:** Define what a "possible world" is in the geometric framework.

**Key question (M1):** What is the sort of a possible world?

Three candidate answers:

| Option | Definition | Pros | Cons |
|--------|-----------|------|------|
| **A: World = Entity** | A possible world w is an entity in Gₑ | Reuses existing sort; worlds are "things" | Entities are compact subsets of ℝ²; worlds are global configurations |
| **B: World = Assertion-collection** | A world w is a maximal consistent set of assertions | Semantically natural (a world is everything that's true in it) | No existing operation produces "maximal consistent sets" |
| **C: World = New sort** | Add a 5th sort W to Σ_UL | Clean separation; worlds are first-class | Breaks 4-sort minimality; must re-prove sort necessity |

**Recommended investigation:** Start with **Option B** (assertion-collection via `embed`). A world is an entity obtained by embedding a conjunction of assertions: `w = embed(conjoin(a₁, conjoin(a₂, ...)))`. This keeps the sort system at 4. If this proves insufficient, Option C becomes the fallback.

**Accessibility relations:** Between worlds, accessibility is a relation r_acc ∈ Gᵣ. Different modal flavors = different accessibility relations:
- Alethic: r_acc_alethic (unrestricted — all worlds)
- Epistemic: r_acc_K (worlds compatible with agent's knowledge)
- Deontic: r_acc_O (worlds the agent considers ideal)

These are just distinguished elements of the existing Relation sort.

### 1B: Modal Operations

**Goal:** Define the algebraic operations for □ (necessity) and ◇ (possibility).

**Key question (M5):** New operations, or enrichment of existing ones?

Two candidate approaches:

| Approach | Definition | Fits Existing Pattern? |
|----------|-----------|----------------------|
| **Quantification over worlds** | `□_R(a) = bind(w, quantify(m_∀, w, a_in_w))` where w ranges over R-accessible worlds | Yes — reuses `bind`, `quantify`. □ becomes a defined pattern, not a new operation |
| **New operations** | `necessary_R : a → a` and `possible_R : a → a` as primitive operations | No — adds 2+ operations per modal flavor |

**Recommended investigation:** Start with the **quantification approach**. If □ can be expressed as universal quantification over accessible worlds using existing operations, no new operations are needed — only distinguished accessibility relations. This would be a major parsimony result.

**The test:** Can "She might be sleeping" (case 4.1) be decomposed as:
```
possible_epistemic(predicate(e_she, r_sleeping, e_implicit))
= bind(w, quantify(m_∃, w, predicate_in(w, e_she, r_sleeping, e_implicit)))
```
where `predicate_in(w, ...)` means "this predication holds in world w"?

**Sub-question:** What does "in world w" mean geometrically? This is the core formalization challenge.

### 1C: Geometric Realization

**Goal:** Define how modal constructions look in the writing system.

**Existing notational hints** (from universal-language-derivation.md §7.3):
- Bold lines = necessity
- Dashed lines = possibility

**Proposed geometric semantics:**
- A possible world = an enclosure (a bounded region of ℝ²) — worlds are literally "spaces"
- Accessibility = a directed path (line) between world-enclosures
- □a = a holds in every world-enclosure connected by r_acc
- ◇a = a holds in some world-enclosure connected by r_acc

**Visual:**
```
  ┌═══════════════════┐   Bold enclosure = NECESSARY
  ║  ┌──────┐         ║   (a holds in ALL connected worlds)
  ║  │ a₁   │ ←─r_acc─╫── accessibility from other worlds
  ║  └──────┘         ║
  └═══════════════════┘

  ┌┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┐   Dashed enclosure = POSSIBLE
  ┆  ┌──────┐         ┆   (a holds in SOME connected worlds)
  ┆  │ a₁   │ ←─r_acc─┤── accessibility from other worlds
  ┆  └──────┘         ┆
  └┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┘
```

---

## Worked Targets — The 5 Cases

### 4.1: "She might be sleeping"
- Modal type: Epistemic possibility (◇_K)
- Target: `possible_K(predicate(e_she, r_sleeping, ...))`
- Decomposition test: Can this be expressed via quantification over K-accessible worlds?

### 4.2: "You must leave now"
- Modal type: Deontic necessity (□_O)
- Target: `necessary_O(predicate(e_you, r_leave, e_now))`
- Challenge: "now" is also deictic (temporal reference)

### 4.3: "He could have won if he had tried"
- Modal type: Counterfactual + ability
- Target: `counterfactual(a_tried, a_won)` = "in the closest world where he tried, he won"
- Challenge: Requires metric on world-space (for "closest")
- Leverage: Fisher metric from Expedition Two

### 4.4: "It is necessarily true that 2+2=4"
- Modal type: Alethic necessity (□)
- Target: `necessary_alethic(predicate(e_2+2, r_equals, e_4))`
- Simplest case — should be the first fully formalized

### 4.5: "She should have been able to finish earlier"
- Modal type: Compound (deontic + ability + counterfactual + temporal comparative)
- Target: Stack of modal operators on a base predication
- Challenge: How do modal operators compose? What is the algebraic structure of compound modals?
- This is the hardest case — should be the last formalized

---

## Key Design Questions

See `design-questions.md` for the full list with analysis.

**The gating question:** Can modality be expressed via quantification over embedded worlds using existing operations (`bind`, `quantify`, `embed`), or does it require genuinely new operations?

- If YES (quantification suffices): Phase 1 adds only distinguished accessibility relations to Gᵣ — no new operations, no sort changes, maximum parsimony
- If NO (new operations needed): Phase 1 must define new operations, prove independence, and update all downstream documents (replicating Pass 1.4 Phase 1–2 work)

**Resolution method:** Attempt the "quantification over worlds" formalization first. If it succeeds for all 5 cases, adopt it. If it fails for any case, identify what's missing and define the minimal additional operation.

---

## Deliverables

| Deliverable | Document | Description |
|-------------|----------|-------------|
| Design resolution | `design-questions.md` | All 7 questions (M1–M7) answered with formal justification |
| Kripke realization | `kripke-realization.md` | Formal definition of possible worlds, accessibility, and modal truth in Σ_UL |
| Modal operations | `modal-operations.md` | Either: new operations with proofs, or: quantification reduction with proof of expressibility |
| D2 rework | (in experiments/) | All 5 Cat. 4 cases re-analyzed with modal apparatus |
| Writing system | (in ul-core/) | Visual conventions for modal constructions |
| Independence | (in pass1-1/) | If new operations: independence proofs. If not: proof of definability |
| Montague intensional | (in foundations/) | Characterize relationship to intensional Montague grammar |

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Quantification approach fails | Must design new operations → larger scope | Design both approaches in parallel; commit to one after 4.1/4.4 test |
| Sort system needs expansion (5th sort) | Breaks minimality proofs | Delay until evidence is overwhelming; prefer enriching sorts |
| Compound modals (4.5) resist decomposition | Scope creep | Accept 4.5 as "requires further work" if necessary |
| Writing system becomes visually complex | Teachability regression | Keep visual conventions minimal; bold/dashed already exist |
| Counterfactual metric (4.3) requires full Fisher metric | Expedition Two dependency | Can define counterfactuals abstractly first, instantiate metric later |
