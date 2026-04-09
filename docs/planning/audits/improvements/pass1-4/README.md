# Pass 1.4 — Internal Consistency & Teachability

**Date:** April 7, 2026  
**Status:** 🔲 NOT STARTED  
**Scope:** Synchronize the writing system with the extended algebra (Σ_UL⁺), fix all stale references across the repo, complete independence proofs, and produce worked examples covering the full operation set  
**Predecessor:** [Pass 1.3](../pass1-3/README.md) (Boundary Formalization — eliminates all ⚠️ cases)  
**Trigger:** The algebra grew from 11 to 13 operations in Pass 1.2, but the writing system documents, metadata files, and prose references have not caught up. A synchronization audit found 7 of 9 writing system documents are stale or partially stale.  
**Goal:** Every document in the repo reflects Σ_UL⁺ (13 operations). The writing system is learnable from documentation alone. The repo is ready for app development.

---

## How Pass 1.4 Differs from Pass 1.3

Pass 1.3 asks: **"Where are UL's boundaries?"** — and formalizes every scope boundary.

Pass 1.4 asks: **"Can someone learn UL from this documentation?"** — and ensures internal consistency, completeness, and teachability.

**Pass 1.3 is classificatory** (sharpen boundaries). **Pass 1.4 is editorial** (synchronize, complete, polish).

---

## The Problem: Fragmented Synchronization

A full audit of writing system documents against the Σ_UL⁺ algebra reveals:

| Document | `bind` | `modify_assertion` | `graduated quantify` | Stale "11 ops" | Status |
|----------|--------|-------------------|---------------------|----------------|--------|
| writing-system.md | ❌ | ❌ | ❌ | ✗ (§4.2) | **STALE** |
| writers-companion.md | ❌ | ❌ | ❌ | ✗ (§5.2 ×2) | **STALE** |
| symbol-map.md | ⚠️ partial | ❌ | ⚠️ unlabeled | — | **NEEDS UPDATE** |
| syntax-dictionary.md | ❌ | ❌ | ❌ | ✗ (§III heading) | **STALE** |
| grammar-book.md | ❌ | ❌ | ⚠️ mentioned | ✗ (§VIII) | **NEEDS UPDATE** |
| formal-grammar.md | ✅ C12 | ✅ C13 | — | — | **OK** (partially updated) |
| lexicon.md | ⚠️ cross-ref | ⚠️ cross-ref | ✅ §6.7 | — | **PARTIAL** |
| thesaurus.md | ❌ | ❌ | ❌ | — | **NEEDS UPDATE** |
| index.json | — | — | — | ✗ (`11`) | **STALE** |

**7 of 9 documents need updates.** The formal algebra (formal-foundations.md, formal-operations.md, formal-grammar.md) is current; the prose writing system is not.

---

## Phase Structure

### Phase 1 — Operation Count Sweep

**Document:** [phase-1-operation-count-sweep.md](phase-1-operation-count-sweep.md)

Find and update every reference to "11 operations" across the entire repo. Also fix stale signature references (Σ_UL → Σ_UL⁺ where appropriate) and operation list enumerations that stop at 11.

**Scope:** All `.md` files, `index.json`, any other files with stale counts.

### Phase 2 — Writing System Synchronization

**Document:** [phase-2-writing-system-sync.md](phase-2-writing-system-sync.md)

The core of Pass 1.4. Update each writing system document to cover all 13 operations:

| Document | What Needs Adding |
|----------|------------------|
| **writing-system.md** | Sections for bind, modify_assertion, graduated quantify; update operation table; update JSON schema; update BNF grammar |
| **syntax-dictionary.md** | Add C12 (bind), C13 (modify_assertion) entries; update §III heading; update reference table |
| **grammar-book.md** | Add sections on co-reference/scope nesting (bind), assertion modification; update reference table |
| **symbol-map.md** | Complete hollow mark (○) entry with binding semantics; add frame decoration marks for modify_assertion |
| **lexicon.md** | Add body sections for bind and modify_assertion entries (currently cross-ref only) |
| **thesaurus.md** | Add semantic pathways for binding, evidentiality, and graduated quantification |

### Phase 3 — Independence Proofs Consolidation

**Document:** [phase-3-independence-proofs.md](phase-3-independence-proofs.md)

The Pass 1.1 document `P1-operation-independence.md` covers the original 10+1 operations. Pass 1.2 added `bind` and `modify_assertion` with independence proof *sketches* in `formal-operations.md` §1.12 and §1.13. This phase:

1. Consolidates all independence proofs into `P1-operation-independence.md`
2. Expands the sketches for bind and modify_assertion into full proofs
3. Resolves the final independence count: is it 12 independent + 1 derived + 1 near-derived, or 12 independent + 1 derived?
4. Updates `formal-foundations.md` minimality claim to match

### Phase 4 — Worked Examples

**Document:** [phase-4-worked-examples.md](phase-4-worked-examples.md)

The writers-companion.md currently has ~6 worked examples, none using the 3 new operations. This phase adds 4–6 new examples covering:

1. **Multi-quantifier scope** — using `bind` to disambiguate "Every student read some book"
2. **Evidential marking** — using `modify_assertion` for "Apparently she left early"
3. **Proportional quantifier** — using graduated `quantify` for "Most birds can fly"
4. **Self-reference** — using `embed` recursion for "The idea that all ideas are expressible"
5. **Ditransitive** — using the polyadic reduction convention for "She gave him the book"
6. **Cross-linguistic** — using the full operation set for a non-English expression

Each example follows the established format: natural language → operation identification → formal decomposition → visual construction → reading back.

### Phase 5 — Gap Analysis & Pass 1 Summary

**Document:** [phase-5-gap-analysis-and-summary.md](phase-5-gap-analysis-and-summary.md)

Final cleanup:

1. Update `frontier/gap-analysis.md` with Montague result and Pass 1.2 extensions
2. Update the "Missing Mathematics" sections — what's been addressed, what remains
3. Produce a "Pass 1 Summary" document: what was found (31 issues), what was fixed (all), what was extended (3 new ops), what was proven (Montague subsumption), what remains (modality, intensionality, performatives, pragmatics)
4. Verify the 10 existing worked examples in writers-companion.md still work with corrected negation (boundary inversion)
5. Final consistency check: every claim in AGENTS.md, llms.txt, README.md matches the formal specification

---

## Success Criteria

- [ ] Zero instances of "11 operations" remaining in the repo (except in historical/resolution-log context)
- [ ] All 9 writing system documents cover all 13 operations
- [ ] Independence proofs for all 13 operations consolidated and complete
- [ ] 12+ worked examples in writers-companion.md covering the full operation set
- [ ] All cross-references between formal and prose documents are consistent
- [ ] gap-analysis.md updated with current status
- [ ] Pass 1 Summary document produced
- [ ] index.json metadata fully current

---

## Risk Assessment

| Risk | Probability | Mitigation |
|------|-------------|------------|
| Writing system updates are extensive (~25k lines across 7 files) | High | Phase 2 focuses on additions, not rewrites. New content is additive. |
| Existing worked examples may break with negation fix | Medium | Phase 5 includes explicit verification of all 10 examples |
| Independence proof for modify_assertion is unclear | Medium | Phase 3 resolves this definitively — may change the count from 14 to 13+1 |
| Scope creep into content improvements | Medium | Strict discipline: only sync existing content, don't rewrite prose |

---

## Dependencies

- **Requires:** Pass 1.3 complete (stable boundary classifications needed before writing system sync)
- **Blocks:** App development, Pass 2 (empirical validation)
- **External references:** None — all work is internal synchronization

---

## Relationship to User Goal

> "We need UL to be very well fleshed out so we can use this repo to produce apps about learning about UL."

Pass 1.4 is the direct bridge to this goal. After Pass 1.4:
- The algebra is correct, complete within scope, and boundary-defined (Passes 1.1–1.3)
- The writing system documentation reflects the algebra (Pass 1.4)
- Worked examples demonstrate every operation (Pass 1.4)
- The repo can serve as a **single source of truth** for any learning app

Pass 2 would be empirical validation (running experiments) or extension (modality, intensionality). But Pass 1 — the foundational audit — would be complete.
