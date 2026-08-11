# Pre-Registered Predictions — Theory-Derived UL Primer Experiment

**Date:** 2025-07-15  
**Status:** PRE-REGISTERED (before data collection)  
**Source:** Derived from REFACTOR-PLAN.md §6, formalized here as a standalone document for audit trail.

---

## Experimental Design Summary

- **4 theory-derived artifacts:** UL-P1 (pattern recognition), UL-P2 (hierarchical decomposition), UL-P3 (abstraction navigation), UL-P4 (meaning formation)
- **7 control/baseline conditions:** NL (no context), REF (the original test artifact), CT-G (generic math), CT-P1 through CT-P4 (domain-matched), NAV-P (naive prompt engineering)
- **7 tasks:** T1, T2, T5 (general cross-domain) + Tpat, Thier, Tabs, Tform (targeted)
- **Scoring:** M1–M5 on all tasks; TES (0–5) on targeted tasks only
- **56 total trials** across 3 phases

---

## Global Predictions (All Theory-Derived Primers)

### G1: Existence of Effect
**Prediction:** Theory-derived artifacts produce effects exceeding the NL baseline.  
**Measure:** Mean M1–M5 scores across UL-P1..P4 vs. NL mean.  
**Threshold:** ≥0.5 point increase on ≥3 of 5 metrics.  
**Falsification:** Fewer than 3 metrics show ≥0.5 increase over NL.

### G2: UL-Specificity (Primers > Domain-Matched Controls)
**Prediction:** Theory-derived artifacts outperform their domain-matched controls, demonstrating that UL structure — not surface domain content — drives the effect.  
**Measure:** UL-Pn vs. CT-Pn on M2 (structural depth), M3 (phase progression), M5 (generative novelty).  
**Threshold:** For ≥3 of 4 artifact-control pairs, UL-Pn > CT-Pn on ≥2 of {M2, M3, M5}.  
**Falsification:** Fewer than 3 artifacts show superiority over their controls on ≥2 of the 3 target metrics.

### G3: Comparability to Reference
**Prediction:** Theory-derived artifacts perform comparably to the original test artifact (the third-party artifact), demonstrating that UL theory can generate artifacts of equivalent efficacy.  
**Measure:** Mean M1–M5 for UL-P1..P4 vs. REF scores.  
**Threshold:** Within ±1.0 points on ≥4 of 5 metrics.  
**Falsification:** More than 1 metric differs by >1.0.

### G4: Naive Control Insufficiency
**Prediction:** The naive prompt-engineering control does not produce artifact-level effects, demonstrating that mere instructions to "think across domains" are insufficient.  
**Measure:** NAV-P scores on M2, M3, M5 vs. NL baseline.  
**Threshold:** NAV-P ≤ NL + 0.5 on M2, M3, M5.  
**Falsification:** NAV-P exceeds NL + 0.5 on ≥2 of {M2, M3, M5}.

---

## Primer-Specific Predictions

### UL-P1: Topological-Harmonic → Pattern Recognition

| ID | Prediction | Falsification Criterion |
|----|-----------|------------------------|
| **P1-a** | UL-P1 scores highest among all 4 artifacts on Tpat (structural pattern recognition task) | Another artifact scores ≥1.0 higher on Tpat |
| **P1-b** | UL-P1 produces M2 ≥ 4.0 on ≥2 of 3 general tasks | M2 < 3.0 on ≥2 general tasks |
| **P1-c** | UL-P1 outputs contain more structural isomorphism language (e.g., "corresponds to," "maps to," "shares the structure of") than NL outputs | Blinded evaluator finds no difference in isomorphism language frequency |

### UL-P2: Recursive-Categorical → Hierarchical Decomposition

| ID | Prediction | Falsification Criterion |
|----|-----------|------------------------|
| **P2-a** | UL-P2 scores highest among all 4 artifacts on Thier (hierarchical decomposition task) | Another artifact scores ≥1.0 higher on Thier |
| **P2-b** | UL-P2 Phase 3 outputs are structured as recursive trees/hierarchies rather than cross-domain webs | Phase 3 outputs from UL-P2 look structurally identical to the original test artifact Phase 3 outputs |
| **P2-c** | UL-P2 scores LOWER than the original test artifact on T2 (cross-domain synthesis) by ≥0.5 on M2 | UL-P2 matches or exceeds the original test artifact on T2/M2 |

**Rationale for P2-c:** UL-P2 is specialized for hierarchical depth, not cross-domain breadth. If it scores equally on cross-domain synthesis, the specificity claim is weakened.

### UL-P3: Information-Geometric → Abstraction Navigation

| ID | Prediction | Falsification Criterion |
|----|-----------|------------------------|
| **P3-a** | UL-P3 scores highest among all 4 artifacts on Tabs (abstraction level task) | Another artifact scores ≥1.0 higher on Tabs |
| **P3-b** | UL-P3 produces genuine abstraction-level transformations rated ≥3.0/5.0 on a "genuine abstraction" scale | Rating < 2.0 |
| **P3-c** | UL-P3 outputs naturally organize into clear level-separated sections rather than flowing prose | Outputs are indistinguishable in structure from NL baseline outputs |

### UL-P4: Thermodynamic-Semiotic → Meaning Formation

| ID | Prediction | Falsification Criterion |
|----|-----------|------------------------|
| **P4-a** | UL-P4 scores highest among all 4 artifacts on Tform (meaning formation task) | Another artifact scores ≥1.0 higher on Tform |
| **P4-b** | UL-P4 outputs on Tform describe meaning formation as a *process with dynamics* (phase transitions, equilibria, entropy exchange) rather than as a static taxonomy | Output is static/taxonomic with no dynamical language |
| **P4-c** | UL-P4 produces the highest M4 (coherence) across all conditions, because its thermodynamic framing naturally imposes conservation laws on reasoning | UL-P4 M4 is NOT the highest; another condition exceeds it |

---

## Cross-Primer Differentiation Predictions

| ID | Prediction | Falsification Criterion |
|----|-----------|------------------------|
| **X1** | The 4 theory-derived artifacts produce distinguishable M1–M5 profiles (not all identical) | Pairwise differences between artifacts are < 0.3 on all 5 metrics |
| **X2** | Each artifact scores highest on its own targeted task (UL-P1→Tpat, UL-P2→Thier, UL-P3→Tabs, UL-P4→Tform) | Fewer than 3 of 4 artifacts score highest on their targeted task |

---

## Decision Rules

### After Phase A (28 trials: NL, REF, UL-P1, UL-P2 × 7 tasks)

| Result | Decision |
|--------|----------|
| G1 satisfied for P1 and P2 | Continue to Phase B |
| G1 NOT satisfied | Stop — UL theory cannot generate effective artifacts |
| G3 satisfied | Promising — theory matches artifact |
| G3 NOT satisfied but G1 satisfied | Continue — artifacts work but differ from reference |

### After Phase B (14 trials: UL-P3, UL-P4 × 7 tasks)

| Result | Decision |
|--------|----------|
| X1 satisfied | Primers produce distinguishable effects — strong evidence for UL theory |
| X2 satisfied | Targeted effects confirmed — strongest evidence for predictive capacity |
| Neither X1 nor X2 | Primers produce "generic something" — UL theory may not be predictive |

### After Phase C (14 trials: controls)

| Result | Decision |
|--------|----------|
| G2 satisfied | UL structure, not domain content, drives effects |
| G2 NOT satisfied | Domain content alone may explain effects — UL specificity not demonstrated |
| G4 satisfied | Formal structure required; prose instructions insufficient |
| G4 NOT satisfied | "Just tell it to think broadly" works — undermines structural necessity |

---

## Integrity Notes

1. These predictions were written BEFORE any experimental trials.
2. The predictions are derived from Σ_UL formal theory and mechanism-of-action analysis, not from prior observations of the original test artifact.
3. Each prediction has an explicit, quantitative falsification criterion.
4. No prediction is unfalsifiable — every one can be disconfirmed by specific empirical outcomes.
5. The decision rules specify when to stop, preventing goal-post-shifting.
