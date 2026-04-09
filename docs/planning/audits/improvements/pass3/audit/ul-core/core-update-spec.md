# UL-Core Update Specification — Pass 3

**Status:** 📋 PLANNED  
**Scope:** All changes to `ul-core/` documentation (writing system, lexicon, symbology, syntax, grammar, thesaurus, composition, UQPL)  
**Reference:** [CHECKLIST.md](../CHECKLIST.md) Phase 2

---

## 1. Current State (Verified 2026-04-08)

### 1.1 What's Already Excellent

UL-Core earned an **A-** grade. The following are complete and accurate:

| Component | Lines | Grade | Notes |
|-----------|-------|-------|-------|
| Writing System (reading spec) | ~1,100 | **A** | 5-pass procedure, 2 worked examples |
| Writer's Companion (writing guide) | ~2,000 | **A** | 19 worked examples (6 simple + 7 compound + 3 modal + 2 performative + 1 pragmatic) |
| Lexicon | ~1,800 | **A** | 42 entries verified, tier-justified |
| Symbology | ~1,050 | **B+** | ~105 symbols; "21 markers" count ambiguous |
| Syntax Dictionary | ~1,500 | **A** | All 13 operations specified |
| Grammar Book | ~1,200 | **A-** | §VI-B (modal), §VI-C (performative), §VI-D (pragmatic) present |
| Thesaurus | ~900 | **A** | 6 semantic families, Erlangen hierarchy |
| Composition | ~2,200 | **A-** | Formal algebra + tech evaluation |
| UQPL | ~1,000 | **B+** | Honest about Σ_UL divergence |
| Meta files | ~1,500 | **A** | CRITIQUE, NAVIGATION, SYNTHESIS, README accurate |

**Total: ~15,750+ lines of specification. All 13 operations documented. Modal/performative/pragmatic specified in grammar-book.md.**

### 1.2 What Needs Updating

The theory is solid. Most extension examples already exist (Pass 2 added Examples 11-19). The remaining gaps are **small pedagogical holes** and **consistency** issues:

| Gap | Impact | Effort |
|-----|--------|--------|
| Writer's Companion: No standalone `invert` example | Passive voice demo only implicit (§5.1 decision tree, §11.1 table) | Low |
| Writer's Companion: No standalone `abstract` example | Operation only implicit in Ex 16's counterfactual expansion | Low |
| Writer's Companion: No standalone `compose` example | Partially in Ex 5 but embedded in conjunction, not standalone | Low |
| Symbology: "21 markers" count ambiguous | Confusing to readers | Low |
| Symbology: No modal/force symbols cataloged | Missing from inventory | Medium |
| Syntax: No performative/pragmatic sections | Only modal (§3.14) present | Medium |
| Thesaurus: No modal/force meaning families | Core families only | Medium |
| Lexicon: No modal/force entries | 42 entries are pre-extension | Decision needed |
| UQPL: bind/modify_assertion not reflected | Stale divergence count | Low |
| NAVIGATION: No prescribed learning sequence | Learner doesn't know where to start | Low |
| Operation Visual Map: Need full verification | May need bind/modify_assertion visuals | Low |

---

## 2. Writer's Companion Updates

### 2.1 Already Complete (Pass 2 — no work needed)

The following examples were added during Pass 2 and already exist in the Writer's Companion:

| # | Sentence | Operations | Status |
|---|----------|------------|--------|
| 11 | "Every student read some book" | `bind`, `quantify` (∀ and ∃) | ✅ Exists |
| 12 | "Apparently she didn't leave" | `modify_assertion` (evidential), `negate` | ✅ Exists |
| 13 | "Most birds can fly" | `quantify` (graduated, p≈0.7) | ✅ Exists |
| 14 | "She might be sleeping" | Modal ◇ epistemic | ✅ Exists |
| 15 | "It is necessarily true that 2+2=4" | Modal □ alethic | ✅ Exists |
| 16 | "He could have won if he had tried" | Counterfactual □→ | ✅ Exists |
| 17 | "I promise to return" | Performative φ=commit | ✅ Exists |
| 18 | "I pronounce you married" | Performative φ=declare | ✅ Exists |
| 19 | "Can you pass the salt?" | Pragmatic CI-1 indirect | ✅ Exists |

### 2.2 New Worked Examples Needed (3 operations without standalone demos)

Only three operations lack a dedicated standalone example. `bind` is covered (Ex 11), `compose` is partially covered (Ex 5), but the following need explicit demonstrations:

**Example 20: Explicit `invert`**
> "The ball was kicked by the child" vs. "The child kicked the ball"

- Active: predicate(e_child, r_kick, e_ball)
- Passive: predicate(e_ball, invert(r_kick), e_child)
- Drawing: Show r_kick as → and invert(r_kick) as ←, same entities swapped positions.
- Note: Currently only referenced in §5.1 decision tree (Q10) and §11.1 summary table.

**Example 21: Explicit `abstract`**
> "The tall..." (extracting tallness as a modifier)

- Entity: e_tall_person (● inside specific enclosure)
- Abstract: abstract(e_tall_person) → m_tall (outline-only of the enclosure, detached)
- Drawing: Before = filled enclosure; After = empty outline (boundary without fill)
- Note: Currently only implicit in Ex 16's counterfactual expansion.

**Example 22: Explicit `compose` (standalone)**
> "grandfather" = father of father

- r_father: → (directed line)
- compose(r_father, r_father): →→ (two directed lines sharing a midpoint)
- Drawing: Two arrows head-to-tail, shared intermediate point marks the composition.
- Note: Ex 5 uses compose but embedded in a conjunction. Needs standalone demo.

---

## 3. Symbology Updates

### 3.1 Marker Count Clarification

Current text says "21 grammatical markers." This needs a **precise enumeration** and methodology note:

Proposed clarification:
```
§VII counts 21 grammatical markers:
  6 polarity/logic markers (⊕, ⊖, ∧, ∨, ⊗, →)
  6 force markers (assert●, query?, direct→, commit←, express~, declare⊡)
  3 tense markers (past←curve, present·, future→curve)
  2 emphasis markers (double-border, underline)
  2 scope markers (open-bracket, close-bracket)
  2 evidential markers (dotted-border, wavy-border)
  = 21 total
```

### 3.2 New Symbols to Add

**Modal symbols (add to §VII or new §VII-B):**
| Symbol | Meaning | Visual |
|--------|---------|--------|
| Bold border (3px) | □ Necessity | `━━━` |
| Dashed border | ◇ Possibility | `╌╌╌` |
| Dash-dot border | □→ Counterfactual | `╌·╌` |
| Double thin border | Epistemic K | `──══──` |
| Dotted thick border | Deontic O | `···(thick)` |
| World-enclosure | Possible world | Double-bordered enclosure ╔═╗ |
| Accessibility arrow | r_access | Labeled directed line between worlds |

**Force symbols (add to force markers section):**
Already partially present; verify all 6 are explicitly listed with geometric justification.

### 3.3 Pragmatic Symbol

| Symbol | Meaning | Visual |
|--------|---------|--------|
| ⟹ | Pragmatic inference arrow | Double-shafted arrow between frames |

---

## 4. Syntax Dictionary Updates

### 4.1 §3.15: Performative Syntax (NEW)

Structure:
- Type signature: `φ(a) → a_φ` where φ ∈ {assert, query, direct, commit, express, declare}
- Geometric realization: frame border decoration determines force
- Composition: FC1–FC5 rules with examples
- Worked examples: 2 per force type (12 mini-examples)

### 4.2 §3.16: Pragmatic Notation (NEW)

Structure:
- Type: `infer(a_surface) → a_intended` with inference chain
- Geometric realization: paired frames with ⟹ arrow
- Six inference rules (SI-1–SI-3, CI-1–CI-3) with examples
- Notation for confidence/defeasibility

### 4.3 §3.12/§3.13 Verification

Verify that bind (C12) and modify_assertion (C13) are fully specified with:
- Type signature ✓
- Geometric realization (before/after) ✓
- 2+ examples at different complexity levels ✓
- Integration with other operations ✓

---

## 5. Thesaurus Updates

### 5.1 Modal Meaning Families

Add to existing family structure:

**§VII-B: Modal Meaning Families**
- Necessity ↔ Possibility ↔ Contingency (alethic axis)
- Knowledge ↔ Belief ↔ Suspicion ↔ Ignorance (epistemic axis)
- Obligation ↔ Permission ↔ Prohibition (deontic axis)
- Ability ↔ Inability ↔ Capacity (dynamic axis)
- Would-be ↔ Might-have-been (counterfactual axis)

Each family: geometric realization, Erlangen level, transformation to adjacent families.

### 5.2 Force Meaning Families

**§VII-C: Illocutionary Force Families**
- Assert ↔ Retract (statement axis)
- Query ↔ Answer (information axis)
- Direct ↔ Comply/Refuse (action axis)
- Commit ↔ Release (obligation axis)
- Express ↔ Suppress (emotion axis)
- Declare ↔ Revoke (institutional axis)

### 5.3 Pragmatic Inference Pathways

**§VII-D: Surface→Intended Transformation Families**
- Scalar pathways: some→not-all, warm→not-hot, good→not-excellent
- Conventional: but→contrast, even→unexpected, still→despite
- Forceful: can-you→please-do, would-you→I-want

---

## 6. Lexicon Evaluation

### 6.1 Should Modal/Force Terms Enter the Lexicon?

Current lexicon has 42 entries at constructive levels 0–5+. The question: do □, ◇, □→, and the 6 forces deserve entries?

**Argument FOR:**
- They are canonical, frequently used, and semantically significant
- Distinguished elements have geometric justification (world-enclosures are enclosures; accessibility relations are lines)
- The 7 modal distinguished elements are tier T2 (structurally distinguished, not forced)

**Argument AGAINST:**
- They are *composed patterns*, not independent constructions
- Adding them risks inflating the lexicon beyond the 42-entry canonical set
- They belong in the Grammar Book (where they already live)

**Recommendation:** Add a **§9: Distinguished Elements** section to the lexicon that cross-references (not duplicates) the Grammar Book entries. Mark as T2. This maintains the 42 canonical entries while acknowledging the distinguished elements exist.

---

## 7. UQPL Updates

### 7.1 Divergence Analysis Sync

Current UQPL spec documents divergence with 11 operations. Needs update:

**Before:** "4 Σ_UL operations missing from UQPL: predicate, modify_entity, modify_relation, embed"  
**After:** "6 Σ_UL operations missing from UQPL primitives: predicate, modify_entity, modify_relation, embed, **bind, modify_assertion**"

Add note: "The divergence grows with Pass 1.2 additions but the principle is unchanged: UQPL is a programming language, not a strict Σ_UL-algebra."

### 7.2 Extension Awareness

Add brief section noting modal/performative/pragmatic exist in the formal spec but are not in UQPL scope (UQPL handles core computation, not speech-act semantics).

---

## 8. Navigation & Meta Updates

### 8.1 NAVIGATION.md: Prescribed Learning Sequence

Add new section:

```markdown
## Prescribed Learning Sequence (9 Days)

For someone learning UL from zero:

| Day | Focus | Time | Document |
|-----|-------|------|----------|
| 1 | Overview | 30 min | README → NAVIGATION → SYNTHESIS |
| 2 | Atomic symbols | 20 min | Symbology §II (7 atoms) |
| 3 | First 6 operations | 45 min | Syntax §III C1–C6 |
| 4 | Remaining 7 operations | 45 min | Syntax §III C7–C13 |
| 5 | Why combinations mean | 60 min | Grammar §I–V |
| 6 | Canonical constructions | 90 min | Lexicon §1–§7 |
| 7 | Synonym navigation | 45 min | Thesaurus §I–§VIII |
| 8 | Reading any mark | 60 min | Writing System (5-pass procedure) |
| 9 | Writing practice | 120 min | Writer's Companion (18+ examples) |

**Minimum Viable Knowledge** (2 hours total):
SYNTHESIS → Symbology §II → Syntax §III C1–C6 → Lexicon §1–2 → Writer's Companion §1–3
```

### 8.2 Operation Visual Map Verification

Confirm all 13 operations have visual conventions. From the audit: ✅ all 13 present. But verify bind (C12) and modify_assertion (C13) have been added (these were Pass 1.2 additions).

### 8.3 Formal Grammar Verification

Verify C12 (bind) and C13 (modify_assertion) have full construction rule entries in formal-grammar.md with:
- Input/output sort constraints
- Geometric construction procedure
- Well-formedness conditions
- At least 1 example

---

## 9. Files Changed Summary

| File | Changes | Priority |
|------|---------|----------|
| `writers-companion.md` | +3 new examples: `invert`, `abstract`, `compose` (~400 lines) | HIGH |
| `symbology/symbol-map.md` | +modal/force symbols, clarify count (~200 lines) | MEDIUM |
| `syntax/syntax-dictionary.md` | +§3.15, §3.16 (~500 lines) | MEDIUM |
| `thesaurus/thesaurus.md` | +§VII-B,C,D (~400 lines) | MEDIUM |
| `lexicon/lexicon.md` | +§9 cross-ref (~100 lines) | LOW |
| `uqpl/uqpl-spec.md` | Update divergence (~50 lines) | LOW |
| `NAVIGATION.md` | +learning sequence (~50 lines) | LOW |
| `writing-system/operation-visual-map.md` | Verify completeness | LOW |
| `grammar/formal-grammar.md` | Verify C12/C13 | LOW |
| **Estimated total** | **~1,700 lines across 9 files** (down from ~2,800 — modal/perf/pragmatic examples already exist) | |
