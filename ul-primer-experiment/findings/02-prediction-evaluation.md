# Prediction Evaluation

All 18 pre-registered predictions from `experiments/demo/predictions.md` evaluated against the experimental data.

---

## Global Predictions

### G1: Existence of Effect
> **Prediction:** Theory-derived primers produce effects exceeding the NL baseline.  
> **Threshold:** ≥0.5 point increase on ≥3 of 5 metrics.

**VERDICT: FAILED**

Mean M1–M5 across UL-P1..P4 vs. NL (general tasks):
- M1: Primers ≈ 5.8, NL ≈ 6.0 → Δ ≈ −0.2
- M2: Primers ≈ 3.7, NL ≈ 3.7 → Δ ≈ 0.0
- M3: Artifacts ≈ 2.1, NL ≈ 2.0 → Δ ≈ 0.1
- M4: Artifacts ≈ 4.2, NL ≈ 4.2 → Δ ≈ 0.0
- M5: Primers ≈ 3.2, NL ≈ 3.2 → Δ ≈ 0.0

No metric reaches the ≥0.5 threshold. Zero of 5 metrics pass. **Falsified.**

---

### G2: UL-Specificity (Primers > Domain-Matched Controls)
> **Threshold:** For ≥3 of 4 primer-control pairs, UL-Pn > CT-Pn on ≥2 of {M2, M3, M5}.

**VERDICT: FAILED**

| Pair | M2 Δ | M5 Δ | TES Δ | Metrics > 0 |
|------|-------|-------|-------|--------------|
| UL-P1 vs CT-P1 (Tpat) | +0.5 | +0.5 | +1 | 2 |
| UL-P2 vs CT-P2 (Thier) | +0.5 | +0.5 | +1 | 2 |
| UL-P3 vs CT-P3 (Tabs) | 0.0 | 0.0 | 0 | 0 |
| UL-P4 vs CT-P4 (Tform) | +0.5 | +0.5 | +1 | 2 |

Three pairs show edges of +0.5 on M2 and M5 — but the threshold requires ≥2 of {M2, M3, M5} clear, and M3 was not separately scored on targeted tasks. On the strict rubric, **3 of 4 pairs show ≥2 metrics with +0.5**, which is at the exact boundary. If the threshold is "strictly greater" rather than "≥0.5 edge," this fails. Given scoring uncertainty of ±0.5, these differences are within noise. **Conservatively: FAILED.**

---

### G3: Comparability to Reference
> **Threshold:** Within ±1.0 points on ≥4 of 5 metrics.

**VERDICT: PASSED (trivially)**

All conditions are within ±0.5 of each other on all metrics. The artifacts are comparable to REF — but so is NL, CT-G, and NAV-P. Everything is comparable to everything. The prediction passes its letter but reveals nothing.

---

### G4: Naive Control Insufficiency
> **Threshold:** NAV-P ≤ NL + 0.5 on M2, M3, M5.

**VERDICT: FAILED**

NAV-P means vs. NL means (general tasks):
- M2: 4.2 vs 3.7 → NAV-P exceeds by 0.5 (at threshold)
- M3: 2.5 vs 2.0 → NAV-P exceeds by 0.5 (at threshold)
- M5: 3.7 vs 3.2 → NAV-P exceeds by 0.5 (at threshold)

NAV-P meets or exceeds the threshold on all 3 metrics. Falsification criterion requires exceeding on ≥2 of {M2, M3, M5}. **Falsified.**

This is the most damaging result for the structural-necessity thesis.

---

## Primer-Specific Predictions

### P1-a: UL-P1 highest on Tpat
**FAILED (narrowly)**  
UL-P1 Tpat TES=5. But UL-P2, UL-P4, and REF also score TES=5. Four-way tie at top.

### P1-b: UL-P1 M2 ≥ 4.0 on ≥2 general tasks
**FAILED**  
T1: M2=4.0, T2: M2=3.0, T5: M2=3.0. Only 1 of 3 meets threshold.

### P1-c: UL-P1 more isomorphism language than NL
**PARTIALLY SUPPORTED**  
UL-P1 uses more "correspondence," "maps to," "structurally identical" language. But NL uses similar language naturally. Difference is present but modest.

### P2-a: UL-P2 highest on Thier
**PARTIALLY SUPPORTED**  
UL-P2-Thier uses categorical fixed-point notation (Φ_democracy ≅ F(Φ_democracy)), which is structurally novel. Slight edge over REF-Thier, but not ≥1.0 higher.

### P2-b: UL-P2 recursive tree structure
**SUPPORTED**  
UL-P2 outputs consistently use recursive/tree structures: colimit chains in Thier, F⁰(∅) → F¹(∅) staging in Tform, "fixed point of its own generating functor" in Tabs. Structurally distinct from NL and REF outputs. This is the **strongest positive finding in the experiment.**

### P2-c: UL-P2 lower than REF on T2/M2
**FAILED**  
UL-P2-T2 M2 ≈ 4.0, REF-T2 M2 ≈ 3.5. UL-P2 matches or exceeds REF. The specificity prediction fails — UL-P2 performs well on both hierarchy and cross-domain synthesis.

### P3-a: UL-P3 highest on Tabs
**FAILED**  
All conditions score TES=4 on Tabs. The task's explicit structure (5 levels requested) drives convergence. No condition distinguishes itself.

### P3-b: UL-P3 genuine abstraction ≥ 3.0
**PASSED (trivially)**  
All conditions show genuine abstraction. This is a task property, not a primer property.

### P3-c: UL-P3 level-separated sections
**PASSED (trivially)**  
All conditions produce level-separated sections because the task explicitly requests them.

### P4-a: UL-P4 highest on Tform
**FAILED (narrowly)**  
UL-P4-Tform TES=5. But UL-P1, UL-P2, UL-P3, and REF also score TES=5.

### P4-b: UL-P4 dynamical process description
**SUPPORTED**  
UL-P4-Tform describes meaning formation as "phase transition in a nonlinear field" with solitonic stability, explicit stage-driving by equation terms. However, all primer Tform outputs are similarly dynamical.

### P4-c: UL-P4 highest M4
**FAILED**  
M4 scores are uniform across conditions (range 3.5–4.5). No condition dominates.

---

## Cross-Primer Differentiation

### X1: Distinguishable M1–M5 profiles
**PARTIALLY SUPPORTED**  
Qualitative differences exist: UL-P2 is strongest overall, UL-P1 triggers the most primer-dismissal. But quantitative differences are mostly within ±0.5 — at the boundary of the falsification criterion (<0.3 on all 5 metrics).

### X2: Each primer tops its targeted task
**FAILED**  
At most 1 of 4 artifacts (UL-P2 on Thier, arguable) clearly scores highest on its targeted task. Others are in multi-way ties.

---

## Summary Table

| ID | Prediction | Verdict |
|----|-----------|---------|
| G1 | Artifacts > NL by ≥0.5 | **FAILED** |
| G2 | Artifacts > domain controls | **FAILED** |
| G3 | Artifacts ≈ REF | **PASSED (trivial)** |
| G4 | NAV-P ≤ NL + 0.5 | **FAILED** |
| P1-a | P1 highest on Tpat | **FAILED** |
| P1-b | P1 M2 ≥ 4.0 on 2+ tasks | **FAILED** |
| P1-c | P1 more isomorphism language | **PARTIAL** |
| P2-a | P2 highest on Thier | **PARTIAL** |
| P2-b | P2 recursive tree structure | **SUPPORTED** |
| P2-c | P2 lower than REF on T2 | **FAILED** |
| P3-a | P3 highest on Tabs | **FAILED** |
| P3-b | P3 genuine abstraction ≥ 3.0 | **PASSED (trivial)** |
| P3-c | P3 level-separated structure | **PASSED (trivial)** |
| P4-a | P4 highest on Tform | **FAILED** |
| P4-b | P4 dynamical process | **SUPPORTED** |
| P4-c | P4 highest M4 | **FAILED** |
| X1 | Distinguishable profiles | **PARTIAL** |
| X2 | Each primer tops its task | **FAILED** |

**Overall: 2 supported, 3 trivially passed, 3 partial, 10 failed.**
