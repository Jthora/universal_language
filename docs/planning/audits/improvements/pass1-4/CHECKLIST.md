# Pass 1.4 — Internal Consistency & Teachability: Checklist

**Last updated:** April 8, 2026  
**Status:** 🔧 IN PROGRESS (Phases 1–2 complete)

---

## Phase 1 — Operation Count Sweep ✅

- [x] Run grep sweep for "11 operations" across all `.md` and `.json` files
- [x] Classify each match: UPDATE / KEEP (historical) / REVIEW
- [x] Fix all UPDATE matches to "13 operations"
- [x] Update `index.json`: `"operations": 14`
- [x] Update writers-companion.md §5.2: "14 Σ_UL⁺ operations"
- [x] Update syntax-dictionary.md §III heading
- [x] Update grammar-book.md §VIII heading
- [x] Re-grep to verify zero stale references (excluding resolution logs)

## Phase 2 — Writing System Synchronization ✅

### 2A: symbol-map.md ✅
- [x] Complete hollow mark (○) entry with binding semantics (already present)
- [x] Add frame decoration marks (dotted, double, wavy) for modify_assertion

### 2B: syntax-dictionary.md ✅
- [x] Add §III.12 (C12 — Bind) with full syntactic rules
- [x] Add §III.13 (C13 — Modify Assertion) with full syntactic rules
- [x] Update §III heading to "14 SYNTACTIC OPERATIONS"

### 2C: grammar-book.md ✅
- [x] Add co-reference/scope nesting section (bind)
- [x] Add assertion-level modification section (modify_assertion)
- [x] Add graduated quantification to quantify row
- [x] Update sentence extension table to 13 rows (11 + bind + modify_assertion)

### 2D: writing-system.md ✅
- [x] Update §7.2 operation mapping table with bind and modify_assertion rows
- [x] Update §7.2 heading to reference Σ_UL⁺

### 2E: lexicon.md ✅
- [x] Add bind and modify_assertion rows to §7 cross-reference table
- [x] Add conjoin (derived) row for completeness
- [x] Update §7 heading to reference 14 Σ_UL⁺ operations

### 2F: thesaurus.md ✅
- [x] Add §VII-B: Assertion-Level Semantic Pathways
- [x] Add epistemic stance modifier families (evidential, emphatic, hedged)
- [x] Add variable binding semantic pathways
- [x] Add graduated quantification spectrum table
- [x] Add modify_assertion/bind entries to §VIII transformation thesaurus

### 2G: formal-grammar.md sync ✅
- [x] Update §4.1 sort compatibility table (assertion row includes C12, C13)
- [x] Update §5 reading algorithm Pass 5 with bind and modify_assertion recognition
- [x] Update §6 writing algorithm Step 5 with bind and modify_assertion instructions

### 2H: Cross-check ✅
- [x] Every new section references formal-grammar.md C12/C13
- [x] Every new section references formal-operations.md §1.12/§1.13
- [x] Pipeline order verified: Symbol → Syntax → Grammar → Writing System → Lexicon → Thesaurus

## Phase 3 — Independence Proofs Consolidation ✅

- [x] Expand `bind` independence sketch to full proof (P1 §2.11: sort-transition + co-reference gap + model separation)
- [x] Expand `modify_assertion` independence sketch to full proof (P1 §2.12: sort-transition + non-derivability + frame-decoration gap + model separation)
- [x] Determine final independence count: **12 independent + 1 derived (conjoin)**
- [x] Update P1-operation-independence.md summary table with all 13 operations
- [x] Update `formal-foundations.md` minimality claim → "12 independent"
- [x] Update `AGENTS.md` and `llms.txt` metadata → "12 independent + 1 derived"
- [x] Update formal-operations.md sketches → reference full proofs

## Phase 4 — Worked Examples ✅

- [x] Write Example 11: Multi-quantifier scope (bind + quantify) — writers-companion.md §10.5
- [x] Write Example 12: Evidential marking (modify_assertion + negate) — writers-companion.md §10.6
- [x] Write Example 13: Proportional quantifier (graduated quantify p≈0.7) — writers-companion.md §10.7
- [x] Verify and fix stale reflection-based negation in §9.3, §5.1 Q4, §9.1 table, §10 examples, §11.1
- [x] Add all new examples to writers-companion.md §10
- [x] Update writers-companion.md §5.1 decision tree with Q12/Q13
- [x] Update Appendix B to 13 operations
- [x] Fix stale "11 operations" references (Step 2 heading + Appendix B heading)

## Phase 5 — Gap Analysis & Pass 1 Summary

### 5A: Gap Analysis Update ✅
- [x] Add status annotations to `frontier/gap-analysis.md` for Pass 1.2 closures
- [x] Add Montague relationship section
- [x] Add explicit modality gap characterization
- [x] Add explicit performative gap characterization

### 5B: Verify Existing Examples ✅
- [x] Audit all existing examples for stale negation references
- [x] Fix any examples using old reflection-based negation
- [x] Verify all reading-back steps

### 5C: Final Consistency Check ✅
- [x] AGENTS.md ↔ formal-foundations.md
- [x] llms.txt ↔ formal-foundations.md
- [x] README.md — no stale claims
- [x] FOR-AI.md — no stale claims
- [x] index.json — all metadata correct
- [x] CITATION.cff — version/date
- [x] Fix counting error: 14→13 operations across ~137 instances

### 5D: Pass 1 Summary ✅
- [x] Write `docs/planning/audits/improvements/pass1-summary.md`
- [ ] Link from each pass's README.md
- [x] Update pass1-4/README.md status to ✅ COMPLETE

---

**Final state after Pass 1.4:**

| Metric | Value |
|--------|-------|
| D2 score | 37✅ / 0⚠️ / 13❌ (74% clean, 0% ambiguous) |
| Operations | 13 (12 independent + 1 derived) |
| Writing system coverage | 13/13 operations documented |
| Worked examples | 13 (covering full operation set) |
| Cross-reference consistency | Verified across all docs |
| Independence proofs | Complete for all operations |
| Stale "11 ops" references | 0 (outside historical context) |
| Ready for app development | ✅ |
