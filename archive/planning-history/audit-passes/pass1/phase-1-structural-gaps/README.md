# Phase 1 — Structural Gap Closure

**Parent:** [Pass 1 README](../README.md)  
**Duration:** 2–3 months  
**New Math Required:** Minor (construction exercises, not new theory)  
**Goal:** Wire existing proofs together, complete missing constructions, close gaps where the machinery already exists

---

## Rationale

Phase 0 relabeled claims. Phase 1 FIXES the easy ones — gaps where:
- The proof exists in one document but isn't cited in another (Gap A)
- A construction is described but not formally carried out (ℚ reduction)
- A computation is claimed but not shown (Robinson Q)
- A result is proven but its significance isn't stated as a theorem (weak terminality)

None of these require inventing new mathematics. They require applying known techniques.

**Core thesis driving this phase:** UL does not compete with or replace other frameworks. It reveals their shared algebraic skeleton. First-order logic, type theory, natural language semantics, and other formal systems are all valid Σ_UL-algebras specialized for different domains. Phase 1 wires together the existing proofs that:
1. The 4-sort structure is not imposed by UL — meanings REQUIRE it (independent derivation)
2. Known formal systems are already Σ_UL-algebras (their operations map into the 11 core operations)
3. G is the initial/terminal object among all such algebras (weak terminality = canonicity)

## Tasks

| Task | Problem | Approach | Effort | Dependencies |
|------|---------|----------|--------|--------------|
| [1.1](1.1-close-gap-a-permanently.md) | Injectivity gap in embedding | Formal connecting argument via Operation Distinctness | 1–2 days | 0.2 |
| [1.2](1.2-reduce-rationals-to-sigma.md) | ℚ construction not reduced to Σ_UL | Thales → compose/invert/modify_relation decomposition | 3–5 days | None |
| [1.3](1.3-robinson-q-verification.md) | Robinson Q claimed but computations missing | Either find in foundation-securing.md or produce | 2–4 days | None |
| [1.4](1.4-adjoint-functors-f3-f4.md) | F₃, F₄ conjectured, AFT hypotheses unchecked | Verify or refute AFT hypotheses | 2–4 weeks | Category theory |
| [1.5](1.5-weak-terminality-specialness.md) | G's specialness not stated as theorem | Formalize weak terminality → canonicity argument | 1–2 weeks | 0.1 |

## Execution Order

**Month 1:**
- 1.1 (2 days) — quick win, already drafted in 0.2
- 1.2 (5 days) — independent, can start immediately
- 1.3 (4 days) — independent, start after checking foundation-securing.md
- 1.5 (start) — needs 0.1 complete

**Month 2:**
- 1.4 (4 weeks) — hardest task, full month
- 1.5 (complete)

**Month 3 (buffer):**
- Review, integration, cross-reference updates

## Acceptance Criteria

- [ ] formal-foundations.md §3.3 injectivity proven without "visual distinctness"
- [ ] numbers-and-computability.md ℚ section shows explicit Σ_UL operation decomposition
- [ ] Robinson Q5, Q7 computations are written out (either cited or produced)
- [ ] F₃, F₄ status upgraded from CONJECTURED to PROVEN or explicitly documented as BLOCKED with specific missing hypotheses
- [ ] New theorem: "G is the canonical geometric realization of Σ_UL" with precise statement and proof
