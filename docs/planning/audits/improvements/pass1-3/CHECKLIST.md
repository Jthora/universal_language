# Pass 1.3 — Boundary Formalization: Checklist

**Last updated:** April 7, 2026  
**Status:** ✅ COMPLETE

---

## Phase 1 — Cardinality Convention (Cases 3.3, 5.2, 10.4)

- [x] Draft cardinality convention statement (arithmetic as external domain)
- [x] Add §6 "Cardinality Convention" to `foundations/formal-operations.md`
- [ ] Add cardinality modifier examples to `ul-core/lexicon/lexicon.md` (T3 tier) — deferred to Pass 1.4 Phase 2
- [x] Re-score D2 case 3.3 (⚠️ → ✅)
- [x] Re-score D2 case 5.2 (⚠️ → ✅)
- [x] Re-score D2 case 10.4 (⚠️ → ✅)
- [x] Update CRITIQUE.md D2 table (Categories 3, 5, 10)
- [x] Resolution log entry: Phase 1

## Phase 2 — Self-Reference Reclassification (Cases 8.1, 8.2, 8.3, 8.5)

- [x] Write formal note: `embed` recursion ↔ Gödel diagonal lemma
- [x] Add preamble to D2-completeness-challenge.md §Category 8
- [x] Re-score D2 case 8.1 (⚠️ → ✅)
- [x] Re-score D2 case 8.2 (⚠️ → ✅)
- [x] Re-score D2 case 8.3 (⚠️ → ✅)
- [x] Re-score D2 case 8.5 (⚠️ → ✅)
- [x] Update CRITIQUE.md D2 table (Category 8: 1/4/0 → 5/0/0)
- [x] Resolution log entry: Phase 2
- [ ] Optional: Add §7 "Self-Reference and Incompleteness" to `foundations/formal-foundations.md`

## Phase 3 — Pragmatic Scope Boundary (Cases 6.3, 6.5)

- [x] Document semantic/pragmatic architectural boundary
- [x] Add "Semantic/Pragmatic Architecture" section to CRITIQUE.md §5
- [x] Re-score D2 case 6.3 (⚠️ → ✅)
- [x] Re-score D2 case 6.5 (⚠️ → ✅)
- [x] Update CRITIQUE.md D2 table (Category 6: 0/2/3 → 2/0/3)
- [x] Resolution log entry: Phase 3

## Phase 4 — Structural Decomposition Conventions (Cases 1.4, 10.1)

- [x] Add §7 "Structural Decomposition Conventions" to `foundations/formal-operations.md`
  - [x] §7.1 Polyadic Reduction (Peirce's reduction thesis + canonical ditransitive form)
  - [x] §7.2 Morphological Transparency (principle + polysynthetic example)
- [x] Re-score D2 case 1.4 (⚠️ → ✅)
- [x] Re-score D2 case 10.1 (⚠️ → ✅)
- [x] Update CRITIQUE.md D2 table (Categories 1, 10)
- [x] Resolution log entry: Phase 4

## Cross-Cutting

- [x] Verify final D2 totals: target ~37✅ / 0⚠️ / 13❌
- [x] Update CRITIQUE.md §5.2 scope table with cardinality and pragmatic entries
- [x] Update pass1-3/README.md status to ✅ COMPLETE
- [x] Verify no new ⚠️ cases were introduced

---

**Projected final D2 table:**

| Category | ✅ | ⚠️ | ❌ | Change from Pass 1.2 |
|----------|---|---|---|----------------------|
| 1. Simple compositional | 5 | 0 | 0 | +1✅, −1⚠️ |
| 2. Deep nesting | 5 | 0 | 0 | no change |
| 3. Quantifier scope | 5 | 0 | 0 | +1✅, −1⚠️ |
| 4. Modality | 0 | 0 | 5 | no change |
| 5. Tense/aspect | 5 | 0 | 0 | +1✅, −1⚠️ |
| 6. Irony/pragmatics | 2 | 0 | 3 | +2✅, −2⚠️ |
| 7. Metaphor | 5 | 0 | 0 | no change |
| 8. Self-reference | 5 | 0 | 0 | +4✅, −4⚠️ |
| 9. Performatives | 0 | 0 | 5 | no change |
| 10. Cross-linguistic | 5 | 0 | 0 | +2✅, −2⚠️ |
| **TOTAL** | **37** | **0** | **13** | +11✅, −11⚠️ |

**Score: 74% clean, 0% ambiguous, 26% principled scope boundaries.**
