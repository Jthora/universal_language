# Findings — Overview

**Date:** April 7, 2026  
**Purpose:** Document the concrete errors, mismatches, and ambiguities found during the deep theory audit  
**Method:** Systematic reading of all foundational documents, formal operation definitions, writing system specs, expedition documents, and planning/audit docs

---

## Summary

9 findings organized by severity:

| Severity | Count | Findings |
|----------|-------|----------|
| 🔴 CRITICAL (theory is wrong) | 1 | F1: Negation-as-reflection bug |
| 🟠 SIGNIFICANT (theory is incomplete/overstated) | 3 | F2: De Morgan redundancy, F3: Modifier carrier mismatch, F4: Missing e→r path |
| 🟡 MODERATE (specification is ambiguous) | 3 | F5: Enclosure dual-sort, F6: Missing mapping table, F7: 4-vs-5 tension |
| 🟢 MINOR (convention/presentation issues) | 2 | F8: Glyph Space ambiguity, F9: Grammar-Algebra classification conflict |

## Document Map

| Document | Contents |
|----------|----------|
| [critical-errors.md](critical-errors.md) | F1 (Negation bug) and F2 (De Morgan redundancy) — theory bugs that require correction |
| [structural-gaps.md](structural-gaps.md) | F3–F9 — mismatches between formal algebra, geometric model, and writing system |

## Relationship to Existing Audit

These findings extend [pass1/proof-inventory.md](../pass1/proof-inventory.md) — which documented *honesty gaps* (oversold claims) — with *correctness gaps* (claims that are wrong or inconsistent).

| Finding | Closest Pass 1 Item | What's New |
|---------|---------------------|------------|
| F1 | Pass 1 flagged reflection=negation as "conventional" | Pass 1.1 shows it's not just conventional but *semantically incorrect* — it produces converse, not negation |
| F2 | Pass 1 Problem 2.2 suspected conjoin/disjoin redundancy | Pass 1.1 confirms it via explicit De Morgan construction |
| F3 | Not flagged in Pass 1 | New finding: carrier set mismatch between algebra and writing system |
| F4 | Not flagged in Pass 1 | New finding: sort-transition graph has a gap |
| F5–F9 | Partially implicit in Pass 1 Phase 0 | Now explicitly documented with evidence |
