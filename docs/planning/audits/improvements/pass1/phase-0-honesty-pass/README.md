# Phase 0 — The Honesty Pass

**Parent:** [Pass 1 README](../README.md)  
**Duration:** 2–3 weeks  
**New Math Required:** None  
**Goal:** Relabel, restructure, and reframe existing content to accurately represent proof status

---

## Rationale

The foundations contain genuine mathematical work. But the presentation conflates:
- Definitions with theorems (role properties are DEFINED, then "proven" to match)
- Conventions with necessities (reflection=negation is a CHOICE, presented as forced)
- Coverage with completeness (11-op coverage argument labeled as if it were a proof)
- Theory validation with artifact testing (experiments framed as "Proof 4 of UL")

Phase 0 fixes none of the math. It fixes the CLAIMS about the math.

## Tasks

| Task | File(s) Affected | Effort | Dependencies |
|------|-----------------|--------|--------------|
| [0.1](0.1-tier-label-formal-foundations.md) | formal-foundations.md | 2 days | None |
| [0.2](0.2-fix-gap-a-injectivity.md) | formal-foundations.md §3.3 | 2 hours | None |
| [0.3](0.3-completeness-roadmap.md) | formal-operations.md §3 | 1 day | None |
| [0.4](0.4-separate-role-properties.md) | formal-foundations.md §4 | 1 day | 0.1 |
| [0.5](0.5-independent-role-derivation.md) | independent-derivation.md | 3–5 days | 0.4 |
| [0.6](0.6-remove-unfalsifiability.md) | paradigm.md | 1 day | None |
| [0.7](0.7-tier-label-derivation.md) | universal-language-derivation.md | 2 days | 0.1 |
| [0.8](0.8-update-metadata.md) | AGENTS.md, index.json, llms.txt | 0.5 day | 0.1, 0.7 |
| [0.9](0.9-reframe-experiments.md) | experiments/ (protocol, README) | 1–2 days | None |

## Execution Order

**Week 1 (parallel start):**
- 0.2 Fix Gap A (2 hours — quick win)
- 0.3 Completeness roadmap (1 day)
- 0.6 Remove unfalsifiability (1 day)
- 0.9 Reframe experiments (1–2 days)
- 0.1 Tier-label formal-foundations (2 days, starts Day 1)

**Week 2 (sequential, depends on 0.1):**
- 0.4 Separate role properties (1 day)
- 0.7 Tier-label derivation (2 days)
- 0.5 Independent role derivation (start, 3–5 days)

**Week 3 (completion):**
- 0.5 Independent role derivation (finish)
- 0.8 Update metadata (0.5 day, needs final tier counts)

## Acceptance Criteria

- [ ] Every §N.M in formal-foundations.md has a `**Status:**` marker
- [ ] formal-foundations.md §3.3 Step 4 cites formal-operations.md Operation Distinctness
- [ ] formal-operations.md §3 has an explicit "OPEN PROBLEMS" subsection
- [ ] formal-foundations.md §4 clearly separates DEFINITIONS from THEOREM
- [ ] independent-derivation.md has new section on role-property extraction
- [ ] paradigm.md has "What Would Prove UL Wrong?" section with 4+ criteria
- [ ] universal-language-derivation.md has tier labels on every theorem-like statement
- [ ] AGENTS.md `theorems_proven` updated to tiered breakdown
- [ ] experiments/ README no longer says "Proof 4"
