# Phase 3 — Extended Theory

**Timeframe:** 12–36 months  
**Prerequisites:** Most of Phase 2 complete (especially 2.1 semantic domain, 2.2 independence, 2.5 self-description)

---

## Purpose

Phase 3 investigates whether dynamic, probabilistic, modal, and differential phenomena are already expressible within Σ_UL (as specializations of the core), or whether they require additional structure beyond the minimal algebraic skeleton. Each extension either:

- **(a)** can be realized as a Σ_UL-algebra with extra structure — meaning probability, modality, etc. are PROJECTIONS of the core, requiring no new primitives
- **(b)** requires genuinely new primitives/axioms — bounding the scope of the minimal core and characterizing what lies beyond it
- **(c)** is provably impossible within any extension of Σ_UL — a negative result that bounds the framework's reach

This addresses the major gaps catalogued in `frontier/gap-analysis.md` and partially explored in `frontier/expedition-one/` and `frontier/expedition-two/`. These are genuine research problems. Some may have negative answers, and that is a valuable outcome — it tells us exactly where the boundary of the minimal core lies.

## Research Problems

| Task | Problem | Builds On | Estimated Effort |
|------|---------|-----------|-----------------|
| 3.1 | Probability and uncertainty | Expedition-two probability theorems | 3–6 months |
| 3.2 | Modal logic (necessity, possibility, knowledge, obligation, time) | New extension | 4–8 months |
| 3.3 | Riemannian context space (continuous meaning geometry) | Expedition-two metric theorems | 6–12 months |
| 3.4 | Instantons and discontinuity (meaning phase transitions) | Expedition-one gauge theory | 6–12 months |
| 3.5 | Pragmatics (speech acts, implicature, context) | New extension | 3–6 months |
| 3.6 | Meaning dynamics (temporal evolution, stability, change) | Gap-analysis §1.7 | 4–8 months |

## Dependency Structure

```
Phase 2.1 ─┬──> 3.1 (Probability)
            ├──> 3.2 (Modal Logic) ───> 3.6 (Dynamics)
            ├──> 3.5 (Pragmatics) ───> 3.6 (Dynamics)
            └──> 3.3 (Riemannian) ─┬──> 3.4 (Instantons)
                                    └──> 3.6 (Dynamics)
```

Task 3.6 is the capstone — it requires results from at least 3.2, 3.3, and 3.5.

## Success Criteria

**For each problem, either:**
1. A theorem extending Σ_UL with the new capability, preserving all existing structure
2. OR a proof that the extension is impossible within Σ_UL (bounding the framework's scope)
3. OR a characterization of what additional primitives/axioms would be needed

**Overall:** A documented map of UL's scope boundary — which phenomena are already projections of the core, which require explicit extension, and which lie fundamentally outside?

## Relationship to Existing Work

Much of Phase 3 has already been explored (sometimes deeply) in the frontier documents. The difference is:
- Expedition-one/two are exploration and proof sketches
- Phase 3 asks: are these phenomena already Σ_UL-algebras with additional structure (i.e., projections of the core)? Or do they require genuinely new primitives? And can the answers be proven at the same level of rigor as the core?

Existing results that feed directly into Phase 3:
- `frontier/expedition-two/probability-and-information.md`: Theorems 1-3 on probability, KL divergence, Fisher information
- `frontier/expedition-two/metric-and-grounding.md`: Theorems 8-11 on semantic distances, geodesics
- `frontier/expedition-one/gauge-bundle-of-meaning.md`: Gauge symmetry, parallel transport, holonomy
- `frontier/expedition-two/metaphor-and-projection.md`: Blending, conceptual integration
- `frontier/expedition-two/metaphor-formalization.md`: Formal polysemy-holonomy connections
