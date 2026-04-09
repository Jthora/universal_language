# Phase 0 — Consolidation: Stale Reference Sweep

**Status:** 📋 PLANNED  
**Goal:** Eliminate all stale "11 operations" references in active documents before any extensions

---

## Inventory

Pass 1.4 Phase 1 claimed zero stale refs but missed ~14 instances across 8 files. These must be fixed before Pass 2 extensions begin.

### Active Files Requiring Fixes

| # | File | Line(s) | Current | Fix |
|---|------|---------|---------|-----|
| 1 | `foundations/formal-operations.md` | 1 (title) | "11 Σ_UL Operations" | → "13 Σ_UL Operations" |
| 2 | `frontier/expedition-two/foundation-securing.md` | 14 | "11 operation codes" | → "13 operation codes" |
| 3 | `frontier/expedition-two/probability-and-information.md` | 49 | "one of the 11 Σ_UL operations" | → "one of the 13 Σ_UL operations" |
| 4 | `frontier/expedition-two/probability-and-information.md` | 71 | "There are 11 Σ_UL operations... 4-bit code ($2^4 = 16 > 11$)" | → "13 Σ_UL operations... 4-bit code ($2^4 = 16 > 13$)" |
| 5 | `frontier/expedition-two/metaphor-and-projection.md` | 367 | "all 11 Σ_UL operations" | → "all 13 Σ_UL operations" |
| 6 | `frontier/expedition-two/metaphor-formalization.md` | 26 | "all 11 Σ_UL operations" | → "all 13 Σ_UL operations" |
| 7 | `frontier/expedition-two/metaphor-formalization.md` | 651 | "preserving all 11 Σ_UL operations" | → "preserving all 13 Σ_UL operations" |
| 8 | `ul-core/CRITIQUE.md` | 294 | "All 11 operations now have visual forms" | → "All 13 operations now have visual forms" |
| 9 | `ul-core/CRITIQUE.md` | 300 | "3/11 operations match exactly; 4 Σ_UL ops missing" | → Update per actual audit findings |
| 10 | `applications/future-research.md` | 19 | "Is the 11-operation set irredundant?" | → "Is the 13-operation set irredundant?" |
| 11 | `applications/future-research.md` | 58 | "11-operation type checker" | → "13-operation type checker" |
| 12 | `ul-core/uqpl/D3-ul-uqpl-analysis.md` | 78, 80 | "UQPL's 11 operations" | → Context-appropriate update (UQPL comparison file) |
| 13 | `ul-core/uqpl/uqpl-spec.md` | 30 | "UQPL's 11 operations do not exactly match the 13 Σ_UL⁺" | → Already partially correct; verify full sentence |

### Historical Files (No Fix Needed)

These files are historical documents with ⚠ HISTORICAL DOCUMENT banners:
- `whitepaper/UL_WhitePaper-Part1.md`
- `whitepaper/UL_WhitePaper-Part2.md`
- `whitepaper/UL_WhitePaper-Part3.md`
- `proto-analysis-papers/UL_WhitePaper-Part1.md`
- `proto-analysis-papers/UL_WhitePaper-Part2.md`
- `proto-analysis-papers/UL_WhitePaper-Part3.md`
- `docs/planning/audits/improvements/pass1-4/` (audit logs)

### Audit Planning Files (No Fix Needed)

These mention "11 operations" in the context of describing what was fixed:
- Various pass1/ documents

---

## Execution Plan

1. Fix items 1–11 (straightforward text replacements)
2. Review items 12–13 (UQPL files — may need more nuanced update since UQPL itself has a different operation count)
3. Run verification grep: `grep -rn "11 operation" --include="*.md" | grep -v whitepaper | grep -v proto-analysis | grep -v pass1`
4. Confirm zero active stale refs

---

## Verification Checklist

- [ ] `foundations/formal-operations.md` title updated
- [ ] `frontier/expedition-two/foundation-securing.md` updated
- [ ] `frontier/expedition-two/probability-and-information.md` updated (2 instances)
- [ ] `frontier/expedition-two/metaphor-and-projection.md` updated
- [ ] `frontier/expedition-two/metaphor-formalization.md` updated (2 instances)
- [ ] `ul-core/CRITIQUE.md` resolution log updated (2 instances)
- [ ] `applications/future-research.md` updated (2 instances)
- [ ] `ul-core/uqpl/D3-ul-uqpl-analysis.md` reviewed and updated
- [ ] `ul-core/uqpl/uqpl-spec.md` reviewed and updated
- [ ] Verification grep shows zero active stale refs (excluding historical docs)
