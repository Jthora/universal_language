# UL Core Writing System Audit

**Date:** 2026-04-08  
**Scope:** `ul-core/` — Writing system, lexicon, symbology, syntax, grammar, thesaurus, composition  
**Goal:** Assess teaching readiness and completeness relative to formal specification

---

## Executive Summary

**Grade: A- (Publication-ready with minor gaps)**

UL Core is substantially complete and well-structured. The Five Siblings architecture (Symbology → Syntax → Grammar → Thesaurus → Lexicon) provides clear separation of concerns. All 13 operations are documented in the Syntax Dictionary. Modal, performative, and pragmatic extensions are covered in the Grammar Book. The Writer's Companion provides excellent worked examples but lacks coverage of advanced constructions.

---

## 1. Component Grades

| Component | File | Grade | Key Finding |
|-----------|------|-------|-------------|
| README.md | `ul-core/README.md` | **A** | Clear entry point, accurate deliverable table |
| NAVIGATION.md | `ul-core/NAVIGATION.md` | **B** | 6 quick-start paths but no prescribed learning sequence |
| SYNTHESIS.md | `ul-core/SYNTHESIS.md` | **A** | Excellent 5-stage pipeline overview |
| CRITIQUE.md | `ul-core/CRITIQUE.md` | **A** | Current, honest, fully updated through Pass 2 |
| Writing System | `writing-system/writing-system.md` | **A** | Complete reading specification with 5-pass procedure |
| Writer's Companion | `writing-system/writers-companion.md` | **B+** | 10 worked examples; missing modal/performative/pragmatic |
| Lexicon | `lexicon/lexicon.md` | **A** | 42 entries, tier-justified, geometrically grounded |
| Symbology | `symbology/symbol-map.md` | **B+** | Complete catalog; "21 markers" count needs clarification |
| Syntax Dictionary | `syntax/syntax-dictionary.md` | **A** | All 13 operations + modal section |
| Grammar Book | `grammar/grammar-book.md` | **A-** | Full VI-B/C/D sections; needs Writer's Companion tie-in |
| Thesaurus | `thesaurus/thesaurus.md` | **A** | Synonymy via Erlangen hierarchy |
| Glyph Composition | `composition/glyph-composition.md` | **A-** | 5 spatial relationships, advanced theory |
| UQPL | `uqpl/uqpl-spec.md` | **B+** | Speculative but honestly labeled |

---

## 2. Writing System Specification

### 2.1 Reading (writing-system.md) — Grade: A

Complete specification for reading ANY UL mark:
- **Axiom (Totality of Drawing):** Every geometrically drawable mark is valid
- **Decomposition Theorem:** Why reading is possible despite the axiom
- **5-pass reading procedure:**
  1. Enclosures (Jordan Curve Theorem)
  2. Connections (paths between regions)
  3. Angles (quality of relationships)
  4. Points (entities)
  5. Curvature (processes)
- Worked example (house drawing)
- Maps to Σ_UL sorts

**For learners:** Excellent. The 5-pass procedure is the key learning algorithm.

### 2.2 Writing (writers-companion.md) — Grade: B+

4-step writing procedure + 10 worked examples.

**Operation coverage in examples:**

| Operation | Covered? | Where |
|-----------|----------|-------|
| predicate | ✅ | Ex 1, 2, 3 |
| modify_entity | ✅ | Ex 1, 6 |
| modify_relation | ✅ | Ex 1, 2 |
| negate | ✅ | §9.3, §7.2 |
| conjoin | ✅ | Ex 5, §9.1 |
| disjoin | ⚠️ | Pattern only |
| embed | ✅ | Ex 3, 4, 5 |
| abstract | ⚠️ | Implicit only |
| compose | ⚠️ | Implied only |
| invert | ❌ | Not in any example |
| quantify | ✅ | Ex 4 |
| bind | ❌ | Not in any example |
| modify_assertion | ⚠️ | §5.1 decision tree only |

**Missing worked examples:**
1. **Modal:** "It's possible that X" using ◇
2. **Performative:** "I assert/query/direct that X" with force φ
3. **Pragmatic:** "Can you pass the salt?" surface ⟹ intended
4. **Explicit `invert`:** Active ↔ passive demonstration
5. **Explicit `bind`:** Co-reference across clauses
6. **Explicit `abstract`+`compose`:** Standalone demonstrations

---

## 3. Lexicon — Grade: A

**42 entries verified:**
- T1 (geometrically forced): 24 entries
- T2 (structurally distinguished): 14 entries
- T3 (conventional): 4 entries

Each entry has:
- Geometric justification
- Constructive level (0–5+)
- Sort assignment
- Cross-references to siblings

**G ≠ Σ_UL distinction (§0.8)** correctly explained — the lexicon describes geometric objects (G), not algebraic operations (Σ_UL).

---

## 4. Syntax Dictionary — Grade: A

**All 13 operations present + modal section (§3.14):**
- Each operation has type signature, geometric realization, worked examples
- Two classification systems (Sort vs. Symmetry) explained
- G ≠ Σ_UL boundary noted
- Caveat about Euclidean assumption included (from CRITIQUE)

This is the most complete reference for operations in the entire repository.

---

## 5. Grammar Book — Grade: A-

**Sections:**
- §I: Grammatical Principle (grammar IS geometry)
- §II: Parts of Speech (from symmetry groups)
- §III: Relationship Taxonomy (4 classes)
- §IV: Erlangen Hierarchy (5 levels)
- §V: Sentence Composition Rules
- **§VI-A: Topological Connectives** (AND, OR, NOT, IF-THEN)
- **§VI-B: Modal Grammar** — world-enclosures, border styles, stacked modals
- **§VI-C: Performative Grammar** — 6 force types, force composition
- **§VI-D: Pragmatic Inference Notation** — surface→intended mapping, SI, CI

**Gap:** §VI-B, VI-C, VI-D define the structures but don't show practical writing guidance. The Writer's Companion should have matching examples.

---

## 6. Navigation & Learning Path

### 6.1 Current State

NAVIGATION.md provides:
- 5 Siblings-at-a-glance table
- 6 quick-start paths (Write, Read, Understand Why, Find Synonyms, Check Validity, Compose)
- Document dependency map (DAG)

### 6.2 What's Missing

**No prescribed learning sequence for beginners.** The quick-start paths are excellent for someone who knows what they want. But a true beginner needs:

```
Suggested order:
Day 1: README + NAVIGATION + SYNTHESIS (30 min)
Day 2: Symbology §II (20 min) — Atomic symbols
Day 3: Syntax §I–III (45 min) — First 5 operations
Day 4: Syntax §III cont. (45 min) — Remaining 8 operations
Day 5: Grammar §I–V (60 min) — Why combinations mean things
Day 6: Lexicon (90 min) — 42 canonical constructions
Day 7: Thesaurus (45 min) — Synonym relationships
Day 8: Writing System (60 min) — Learn to READ any mark
Day 9: Writer's Companion (120 min) — Learn to WRITE
```

### 6.3 Minimum Viable Knowledge

For someone who just wants to use UL, not master it:
- SYNTHESIS.md (understand the pipeline)
- Symbology §II (7 atomic symbols)
- Syntax §III, operations 1–6 (core operations)
- Lexicon §1–2 (Level 0–1 entries)
- Writer's Companion §1–3 (4-step procedure + first 3 examples)

---

## 7. Symbology Count Issue

The symbol-map.md claims "21 grammatical markers" (recently updated from 17). Actual inventory:

| Category | Count | Items |
|----------|-------|-------|
| Atomic symbols | 7 | Void, Point, Hollow Mark, Line, Angle, Curve, Enclosure |
| Directed line variants | 5 | →, ←, ↑, ↓, ↔ |
| Distinguished angles | 6 | 0°, 60°, 90°, 120°, 180°, 360° |
| Enclosure shapes | 5 | △, □, ⬠, ⬡, ○ |
| Frame decorations | 5 | Solid, Dashed, Dotted, Double, Wavy |
| Polarity marks | 2 | ⊕ (positive), ⊖ (negative) |
| Force marks | 6 | ●(assert), ?(query), →(direct), !(commit), ~(express), ⊡(declare) |

**Total by category: 36.** The "21" claim refers specifically to grammatical markers excluding base atoms and semantic marks. This should be clarified in the document.

---

## 8. Cross-File Consistency

### 8.1 Operation Count Alignment

| Document | Claims | Correct? |
|----------|--------|----------|
| formal-foundations.md | 13 operations (12 independent + 1 derived) | ✅ |
| syntax-dictionary.md | 13 operations in §III | ✅ |
| lexicon.md | References Σ_UL | ✅ |
| grammar-book.md | References formal-foundations.md | ✅ |
| CRITIQUE.md | "13 algebraic operations" | ✅ |
| NAVIGATION.md | References 5 siblings | ✅ |

### 8.2 D2 Score Alignment

All files now consistently report 50/50 (100%) after Pass 2 completion.

### 8.3 Extension References

| Document | Modality | Performatives | Pragmatics |
|----------|----------|---------------|------------|
| formal-foundations.md | §7 ✅ | §8 ✅ | §9 ✅ |
| grammar-book.md | §VI-B ✅ | §VI-C ✅ | §VI-D ✅ |
| syntax-dictionary.md | §3.14 ✅ | ❌ missing | ❌ missing |
| writers-companion.md | ❌ no example | ❌ no example | Ex 19 (CI-1 only) |
| thesaurus.md | ❌ no pathways | ❌ no pathways | ✅ §VI pragmatic |
| lexicon.md | ❌ no modal entries | ❌ no force entries | ❌ |

**Finding:** Extensions are formally defined (formal-foundations.md, grammar-book.md) but not yet propagated to all siblings.

---

## 9. Priority Actions for Pass 3

### HIGH Priority

| # | Action | File(s) | Effort |
|---|--------|---------|--------|
| 1 | Add 3 advanced worked examples to Writer's Companion | writers-companion.md | Medium |
| 2 | Create prescribed learning sequence | NAVIGATION.md or new file | Low |
| 3 | Add performative/pragmatic sections to syntax-dictionary.md | syntax-dictionary.md | Medium |
| 4 | Clarify "21 grammatical markers" count | symbol-map.md | Low |

### MEDIUM Priority

| # | Action | File(s) | Effort |
|---|--------|---------|--------|
| 5 | Add modal/force entries to lexicon (T2/T3 tier) | lexicon.md | Medium |
| 6 | Add modal/performative pathways to thesaurus | thesaurus.md | Medium |
| 7 | Create "Minimum Viable Knowledge" guide | New document | Low |

### LOW Priority

| # | Action | File(s) | Effort |
|---|--------|---------|--------|
| 8 | Add explicit `invert`, `bind`, `abstract`, `compose` examples | writers-companion.md | Medium |
| 9 | Update UQPL spec for new operations | uqpl-spec.md | Medium |
