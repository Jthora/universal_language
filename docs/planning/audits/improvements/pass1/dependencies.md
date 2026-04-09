# Dependencies — Inter-Task Dependency Graph

**Parent:** [Pass 1 README](README.md)

---

## Critical Path

The longest dependency chain determines the minimum calendar time:

```
Phase 0.1 (tier labels) ──→ Phase 1.5 (weak terminality) ──→ Phase 2.3 (assignment uniqueness)
     │                                                              │
     └──→ Phase 0.4 (separate role props) ──→ Phase 2.4 (non-circular grounding)
                                                    │
Phase 1.3 (Robinson Q) ──────────────────→ Phase 2.5 (self-description)
                                                    │
Phase 2.1 (semantic domain) ────────────→ Phase 2.2 (algebraic independence)
```

**Critical path duration:** Phase 0 (3 weeks) → Phase 1 (3 months) → Phase 2 (12 months) = ~16 months minimum

---

## Phase 0 Internal Dependencies

```
0.1 Tier-label formal-foundations ──→ 0.4 Separate role-properties ──→ 0.5 Independent role derivation
                                  └──→ 0.7 Tier-label derivation
                                  └──→ 0.8 Update metadata (needs final tier counts)

0.2 Fix Gap A ─────────── (no dependencies, can start immediately)
0.3 Completeness roadmap ─ (no dependencies, can start immediately)
0.6 Remove unfalsifiability (no dependencies, can start immediately)
0.9 Reframe experiments ── (no dependencies, can start immediately)
```

**Parallelization:** Tasks 0.2, 0.3, 0.6, 0.9 can all start Day 1.  
Task 0.1 must complete before 0.4, 0.7, 0.8.  
Task 0.4 must complete before 0.5.

---

## Phase 0 → Phase 1 Dependencies

| Phase 1 Task | Depends On |
|--------------|------------|
| 1.1 Close Gap A permanently | 0.2 (draft fix) |
| 1.2 Reduce ℚ to Σ_UL | None (independent) |
| 1.3 Robinson Q verification | None (independent, needs foundation-securing.md review) |
| 1.4 F₃, F₄ adjoints | None (independent, needs category theory) |
| 1.5 Weak terminality → specialness | 0.1 (embedding renamed to adequacy) |

**Parallelization:** Tasks 1.2, 1.3, 1.4 can all start as soon as Phase 0 begins. Only 1.1 and 1.5 require specific Phase 0 outputs.

---

## Phase 1 → Phase 2 Dependencies

| Phase 2 Problem | Depends On |
|-----------------|------------|
| 2.1 Semantic domain axiomatization | Phase 0 complete (honest status needed to define scope) |
| 2.2 Algebraic independence | 2.1 (need domain definition to test against) |
| 2.3 Semantic assignment uniqueness | 1.5 (specialness result informs what "unique" means categorically) |
| 2.4 Role-property grounding | 0.5 (independent derivation attempt) |
| 2.5 Self-description encoding | 1.3 (Robinson Q must be verified for Route B) |

**Parallelization:** Problems 2.1 and 2.4 can start simultaneously. 2.3 can start once 1.5 is done. 2.2 waits for 2.1. 2.5 waits for 1.3.

---

## Phase 2 → Phase 3 Dependencies

| Phase 3 Research | Depends On |
|------------------|------------|
| 3.1 Probability/uncertainty | 2.1 (semantic domain needed for measure spaces) |
| 3.2 Modal logic | 2.3 (assignment uniqueness determines modal operator form) |
| 3.3 Riemannian context space | Expedition Two metric results (already done) |
| 3.4 Instantons | 3.3 (need full Riemannian geometry first) |
| 3.5 Pragmatics | 3.1 + 3.2 (need probability AND modality) |
| 3.6 Meaning dynamics | 3.3 (need metric for flow equations) |

**Parallelization:** 3.1 and 3.3 can start simultaneously. 3.2 waits for 2.3. Everything else waits for multiple Phase 3 predecessors.

---

## Phase 4 Dependencies

Implementation tasks are triggered by the mathematical results they implement:

| Phase 4 Task | Triggered By |
|--------------|-------------|
| 4.1 GIR schema tier labels | Phase 0 complete |
| 4.2 Operation independence tests | 2.2 complete |
| 4.3 Probability extension | 3.1 complete |
| 4.4 Modal operators | 3.2 complete |

---

## Parallelization Summary

### Maximum parallelism at each stage:

| Time Window | Concurrent Tasks | Workers Needed |
|-------------|-----------------|----------------|
| Weeks 1–3 | 0.2, 0.3, 0.6, 0.9, 0.1 (in parallel), then 0.4, 0.5, 0.7, 0.8 | 2–3 |
| Months 1–3 | 1.1, 1.2, 1.3, 1.4, 1.5 | 3–4 |
| Months 4–12 | 2.1 + 2.4 (parallel), then 2.2 + 2.3 + 2.5 | 2–3 |
| Months 12–36 | 3.1 + 3.3 (parallel), then 3.2, 3.4, 3.5, 3.6 | 2–4 |

### Solo worker timeline:
- Phase 0: 3 weeks
- Phase 1: 4 months (sequential)
- Phase 2: 18 months (sequential)  
- Phase 3: 30 months (sequential)
- **Total solo: ~4.5 years**

### Two-worker timeline:
- Phase 0: 2 weeks
- Phase 1: 2.5 months
- Phase 2: 12 months
- Phase 3: 18 months
- **Total with 2 workers: ~2.7 years**
