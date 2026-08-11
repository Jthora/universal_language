# Improvement Sequence — Priority-Ordered Resolution Plan

**Date:** April 7, 2026  
**Scope:** 9 improvements (P0–P8) derived from findings F1–F9 and questions A1–D3  
**Principle:** Fix what blocks other work first; defer what can wait without causing cascading errors

---

## The Sequence

### P0: Resolve Negation  
**Blocks:** P1 (independence), P5 (Euclidean analysis), all propositional logic claims  
**Finding:** F1, **Question:** B1  
**Action:** Choose between boundary inversion, topological complement, or honest algebraic axiom. Implement in `formal-operations.md`, propagate to writing system docs.  
**Decision needed from author:** Which option? (Recommendation: boundary inversion — Option A from B1)  
**Estimated files affected:** `formal-operations.md`, `formal-foundations.md`, `syntax-dictionary.md`, `symbol-map.md`, `grammar-book.md`, `glyph-composition.md`

---

### P1: Prove/Refute Operation Independence  
**Blocks:** P3 (mapping table needs correct operation set), P6 (predictions need correct primitives)  
**Finding:** F2, **Question:** B4  
**Action:** For each of the 11 operations, prove independence or exhibit derivation. Current confirmed: conjoin derivable from {negate, disjoin}. Suspects: quantify, invert.  
**Input required:** Corrected negate from P0  
**Method:** Universal algebra model-construction technique (from pass1/2.2)  
**Deliverable:** Independence table for all operations

---

### P2: Clarify Modifier-as-Morphism  
**Blocks:** Nothing directly, but informs P4 and future category-theory work  
**Finding:** F3, **Question:** B2  
**Action:** Add a note to `formal-foundations.md` distinguishing static sorts (e, r, a) from the dynamic sort (m). Optionally, sketch the enriched-category perspective in a frontier document.  
**Minimal fix:** 1 paragraph in `formal-foundations.md` acknowledging the carrier-set distinction  
**Full fix:** New `frontier/` document on the category-theoretic treatment of Modifier

---

### P3: Write Visual↔Algebraic Mapping Table  
**Blocks:** C1 (machine-readability assessment), C2 (identifying orphaned operations)  
**Finding:** F6, **Question:** C1  
**Action:** Create a single reference table mapping every Σ_UL operation to its visual realization and every visual construction category to its algebraic decomposition.  
**Location:** New document in `ul-core/writing-system/` or as an appendix to `grammar-book.md`  
**Format:**

```
| # | Operation | Type Signature | Visual Realization | Example | Notes |
|---|-----------|---------------|-------------------|---------|-------|
| 1 | predicate | e × r × e → a | Entity–Arrow–Entity in Frame | △──→○ in [...] | Complete |
| 2 | modify_entity | m × e → e | Modifier mark adjacent to entity | ∠△ | Complete |
| ... | ... | ... | ... | ... | ... |
| 8 | abstract | e → m | ??? | ??? | MISSING |
```

---

### P4: Address e → r Gap  
**Blocks:** Nothing directly; informs completeness claims  
**Finding:** F4, **Question:** B3  
**Action:** Three attempts in order:
1. Try to derive e → r from existing operations (more thorough than the initial attempt in B3)
2. If not derivable, try to prove impossibility (no sort-respecting function can produce r from e alone)
3. If neither works, propose `verbalize: e → r` as a 12th operation with geometric interpretation  
**Deliverable:** Proof of derivability, impossibility proof, or new operation design

---

### P5: State Euclidean Assumption Explicitly  
**Blocks:** Nothing; improves intellectual honesty  
**Finding:** Implicit in syntax-dictionary.md §II  
**Question:** A2  
**Action:** Add a paragraph to `syntax-dictionary.md` §II and `formal-foundations.md` §1 stating:
> "UL's syntax adopts Euclid's 5 postulates. The 5th postulate (unique parallelism) implies that analogy is unique — a structural claim that may not hold in all domains. Non-Euclidean extensions are possible but unexplored."  
**Effort:** 30 minutes for the note; months if the non-Euclidean analysis is pursued

---

### P6: Generate Falsifiable Predictions  
**Blocks:** Experimental credibility  
**Question:** D2  
**Action:** Derive 5+ testable predictions from UL's structural claims. Each prediction must be:
- Specific (definite pass/fail criterion)
- Risky (non-trivial, could fail)
- Testable at $0 cost (corpus analysis, published linguistic data)  
**Location:** New document in `frontier/` or `experiments/`  
**Most promising predictions:**
1. Factor analysis of semantic datasets yields 5 factors
2. All natural language expressions decompose into 4 sorts
3. Cross-linguistic universality survives polysynthetic/ergative languages

---

### P7: Reconcile UL ↔ UQPL  
**Blocks:** Nothing; clarifies project boundaries  
**Question:** D3  
**Action:** Analyze `ul-core/uqpl/` and map UQPL operations to Σ_UL operations. Classify the relationship (algebra, extension, or parallel system).  
**Deliverable:** Relationship classification + operation mapping table

---

### P8: Write CRITIQUE.md  
**Blocks:** Nothing, but completes a reference from ul-core/README.md  
**Question:** C3  
**Action:** Create `ul-core/CRITIQUE.md` using the findings and questions from this audit. Living document, updated as issues are resolved.  
**Template:** Provided in C3  
**When:** Can be written immediately — all content exists in pass1-1/

---

## Dependency Graph

```
P0 (Negation) ─────→ P1 (Independence) ─────→ P6 (Predictions)
                              │
                              ↓
                      P3 (Mapping Table) ────→ C1, C2 (orphans)
                      
P2 (Modifier) ─────→ (informs P4, frontier/)

P4 (e→r gap) ─────→ (informs completeness claims)

P5 (Euclidean note) ─ standalone ─→ (improves honesty)

P7 (UL↔UQPL) ─── standalone ─→ (clarifies boundary)

P8 (CRITIQUE.md) ─ standalone ─→ (can write immediately)
```

## Effort Estimates by Phase

| Phase | Items | Character | Approximate Effort |
|-------|-------|-----------|--------------------|
| **Immediate** (can start now) | P5, P8 | Documentation fixes | Days |
| **Short-term** (need design decisions) | P0, P2, P3 | Design + documentation | Weeks |
| **Medium-term** (need proofs) | P1, P4 | Mathematical analysis | Months |
| **Long-term** (need data/experiments) | P6, P7 | Empirical + analysis | Months to year |

## Quick Wins (Start Today)

1. **P8:** Write CRITIQUE.md — content exists, just needs assembly
2. **P5:** Add Euclidean assumption note — 1 paragraph, 2 files
3. **P0 design decision:** Author chooses negation resolution — enables all downstream work
