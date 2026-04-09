# Pass 1.3 — Boundary Formalization

**Date:** April 7, 2026  
**Status:** ✅ COMPLETE  
**Scope:** Resolve every remaining ⚠️ case in the D2 completeness challenge — either promotes to ✅ or demotes to principled ❌  
**Predecessor:** [Pass 1.2](../pass1-2/README.md) (Expressiveness Extensions — all phases ✅ COMPLETE)  
**Trigger:** D2 score is 26✅ / 11⚠️ / 13❌ (52% clean). The 11 ⚠️ cases cluster into 4 distinct issues, each resolvable by convention, reclassification, or a targeted formal argument.  
**Goal:** Eliminate the ⚠️ category. Every D2 case becomes either clean (✅) or a principled, declared scope boundary (❌). Target: ~37✅ / 0⚠️ / 13❌ (74% clean, 0% ambiguous).

---

## How Pass 1.3 Differs from Pass 1.2

Pass 1.2 asked: **"Can UL express what it claims to express?"** — and extended the algebra with 3 new operations.

Pass 1.3 asks: **"Where exactly does UL's expressiveness end, and is each boundary principled?"** — and resolves every remaining partial case.

**Pass 1.2 was constructive** (add missing operations). **Pass 1.3 is classificatory** (sharpen every boundary).

No new operations are expected. The work is formal arguments, conventions, and reclassifications.

---

## The 11 Remaining ⚠️ Cases

| Case | Expression | Current Issue | Cluster |
|------|-----------|---------------|---------|
| 1.4 | "She gave him the book" (ditransitive) | Binary predicate needs 3 slots | D: Structural Convention |
| 3.3 | "Exactly three students passed" | Cardinality requires arithmetic | A: Cardinality |
| 5.2 | "I have been studying for three hours" | Duration needs cardinality | A: Cardinality |
| 6.3 | "Some students passed" (scalar implicature) | Pragmatic inference, not content | C: Pragmatic Content |
| 6.5 | "He's not the sharpest tool in the shed" | Pragmatic understatement | C: Pragmatic Content |
| 8.1 | "This sentence is false" (Liar) | Semantic undetermination | B: Self-Reference |
| 8.2 | "Set of all sets that don't contain themselves" | Semantic undetermination | B: Self-Reference |
| 8.3 | "I am lying" | Semantic undetermination | B: Self-Reference |
| 8.5 | "The following/preceding sentence" (Yablo) | Mutual reference undetermination | B: Self-Reference |
| 10.1 | Polysynthetic Mohawk word-sentence | Morphological complexity | D: Structural Convention |
| 10.4 | Mandarin classifier "三本书" | Counting needs cardinality | A: Cardinality |

### Cluster Distribution

| Cluster | Cases | Strategy | Expected Outcome |
|---------|-------|----------|------------------|
| A: Cardinality | 3.3, 5.2, 10.4 | Convention: arithmetic is an external domain | ⚠️ → ✅ with convention |
| B: Self-Reference | 8.1, 8.2, 8.3, 8.5 | Formal argument: Gödel-correct behavior | ⚠️ → ✅ (reclassify as feature) |
| C: Pragmatic Content | 6.3, 6.5 | Scope sharpening: literal ✅, pragmatic ❌ | ⚠️ → ✅ (literal) or ⚠️ → ❌ (pragmatic) |
| D: Structural Convention | 1.4, 10.1 | Canonical decomposition rules | ⚠️ → ✅ with convention |

---

## Phase Structure

### Phase 1 — Cardinality Convention (Cases 3.3, 5.2, 10.4)

**Document:** [phase-1-cardinality-convention.md](phase-1-cardinality-convention.md)

Formalize UL's relationship to arithmetic. Cardinality is not a semantic primitive — it is a property of collections that requires counting, which is an external mathematical operation. UL can reference specific cardinalities via `modify_entity` conventions (like specific modifier assignments for "three") but does not internalize natural number arithmetic.

### Phase 2 — Self-Reference Reclassification (Cases 8.1, 8.2, 8.3, 8.5)

**Document:** [phase-2-self-reference-reclassification.md](phase-2-self-reference-reclassification.md)

Construct a formal argument that UL's handling of paradoxes is Gödel-correct: the system *constructs* the paradox (via `embed` recursion), the truth value is correctly undetermined, and any system that resolves the Liar is necessarily either inconsistent or incomplete. Undetermination is the mathematically correct answer. These should be ✅, not ⚠️.

### Phase 3 — Pragmatic Scope Boundary (Cases 6.3, 6.5)

**Document:** [phase-3-pragmatic-scope-boundary.md](phase-3-pragmatic-scope-boundary.md)

Sharpen the semantic/pragmatic boundary. UL encodes *semantic content* (what is said). Pragmatic inference (what is implicated) operates on top of UL-encoded content as a separate layer. Scalar implicature and litotes have fully expressible literal content; the pragmatic inference mechanism is out of scope. Formalize this as a clean architectural boundary.

### Phase 4 — Structural Decomposition Conventions (Cases 1.4, 10.1)

**Document:** [phase-4-structural-conventions.md](phase-4-structural-conventions.md)

Formalize two canonical decomposition conventions:
1. **Polyadic reduction:** n-ary predicates → binary chains (Peirce's reduction thesis). Ditransitive `give(A, B, C)` → `pred(A, give-to, B) ∧ pred(A, give-obj, C)`.
2. **Morphological transparency:** UL decomposes meaning, not surface morphology. Polysynthetic word-sentences receive the same operation-depth decomposition as analytic sentences expressing the same meaning.

---

## Success Criteria

- [ ] All 11 ⚠️ cases reclassified to ✅ or ❌ with formal justification
- [ ] D2 score: 0 ⚠️ cases remaining
- [ ] Each reclassification documented with the specific argument
- [ ] CRITIQUE.md updated with new D2 table and resolution log entries
- [ ] Scope boundary section of CRITIQUE.md updated with cardinality and pragmatic conventions

---

## Risk Assessment

| Risk | Probability | Mitigation |
|------|-------------|------------|
| Cardinality convention feels ad-hoc | Medium | Ground in Peirce/Frege: arithmetic is a separate formal system, not a semantic primitive |
| Self-reference reclassification is self-serving | Low | The argument is standard (Gödel/Tarski). Any formal system that resolves the Liar is inconsistent. |
| Pragmatic boundary is too convenient | Medium | Cross-reference with linguistics: semantics/pragmatics boundary is standard (Grice 1975, Levinson 2000) |
| Ditransitive convention is lossy | Low | Peirce's reduction thesis is proven: all polyadic relations reduce to binary chains without information loss |

---

## Dependencies

- **Requires:** Pass 1.2 complete (all 13 operations defined)
- **Blocks:** Pass 1.4 (writing system sync needs stable boundary classifications)
- **External references:** Gödel's incompleteness theorems, Peirce's reduction thesis, Grice's pragmatic theory
