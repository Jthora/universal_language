# Demo Experiment — Scoring and Analysis

**Date scored:** 2025-07-15  
**Scorer:** GitHub Copilot (Claude Opus 4.6) — same session that designed and ran the experiment  
**Scoring method:** Manual application of simplified M1–M5 rubric to all 9 outputs  
**Blinding:** Not performed — scorer has full knowledge of conditions (acknowledged bias)

---

## ⚠️ CRITICAL CONTAMINATION FINDING

**Before interpreting any results, note this:** The NL-T1 output (no-context condition) explicitly mentions "geometric primitives — point, line, angle, curve, enclosure" — terminology taken directly from the UL project's AGENTS.md file. This proves that **all subagent sessions had access to UL project metadata** through the system prompt, contaminating every condition.

**Implication:** NL and CT conditions are NOT true baselines. They function as "weak UL exposure" conditions rather than genuine no-UL controls. All results must be interpreted in this light.

---

## Raw Scores

### Task T1: Language, Consciousness, and Mathematical Structure

| Metric | UL-T1 | NL-T1 | CT-T1 |
|--------|-------|-------|-------|
| M1 (Domain Diversity, 0–10) | 7 | 7 | 6 |
| M2 (Structural Depth, 0–5) | 4 | 4 | 3 |
| M3 (Phase Progression, 0–3) | 3 | 3 | 3 |
| M4 (Coherence, 0–5) | 4 | 4 | 4 |
| M5 (Generative Novelty, 0–5) | 3 | 2 | 2 |

**T1 Notes:** Minimal differentiation. UL-T1 explicitly referenced "the Universal Language project in this repository" (contamination). NL-T1 also referenced UL's five geometric primitives despite having no context (contamination from AGENTS.md). CT-T1 engaged productively with the hydrogen atom formalism. UL advantage limited to M5 (+1 over both).

### Task T2: Quantum Mechanics and Belief Formation

| Metric | UL-T2 | NL-T2 | CT-T2 |
|--------|-------|-------|-------|
| M1 (Domain Diversity, 0–10) | 8 | 6 | 6 |
| M2 (Structural Depth, 0–5) | 4 | 3 | 3 |
| M3 (Phase Progression, 0–3) | 3 | 2 | 2 |
| M4 (Coherence, 0–5) | 4 | 4 | 4 |
| M5 (Generative Novelty, 0–5) | 3 | 2 | 3 |

**T2 Notes:** Clearest UL advantage. The test artifact's explicit mathematical structure (gauge field, nonlinear terms, dissipation) gave UL-T2 specific formal objects to engage with. UL-T2 discussed Gross-Pitaevskii self-interaction as "confirmation bias," gauge fields as "framing" (no view from nowhere), and spectral 1/n weighting as cultural influence — all directly reading the test artifact's equations. NL-T2 stayed analogical (Phase 2). CT-T2 drew systematic parallels to hydrogen atom physics but remained at the parallel-drawing level (Phase 2). Only UL-T2 spoke FROM the intersection (Phase 3).

### Task T3: Formal Framework for Metaphor

| Metric | UL-T3 | NL-T3 | CT-T3 |
|--------|-------|-------|-------|
| M1 (Domain Diversity, 0–10) | 9 | 7 | 8 |
| M2 (Structural Depth, 0–5) | 5 | 5 | 5 |
| M3 (Phase Progression, 0–3) | 3 | 3 | 3 |
| M4 (Coherence, 0–5) | 4 | 5 | 4 |
| M5 (Generative Novelty, 0–5) | 5 | 4 | 4 |

**T3 Notes:** ALL conditions performed excellently. All three achieved M2=5 (novel framework construction) and M3=3 (emergent synthesis). This task appears to be at or near Claude's capability ceiling. UL-T3 was the longest (~3000 words, 10 formal sections, category Met as bicategory, nonlinear PDE for meaning dynamics, metaphorical completeness theorem). NL-T3 was ~2000 words with 8 sections. CT-T3 was ~2000 words with 9 sections, explicitly using QM perturbation theory as scaffolding. UL advantage: M1 (+2/+1) and M5 (+1), but NL-T3 scored higher on M4 (tighter argument).

---

## Condition Averages

| Metric | UL Mean | NL Mean | CT Mean | UL−NL | UL−CT |
|--------|---------|---------|---------|-------|-------|
| M1 (0–10) | **8.0** | 6.7 | 6.7 | +1.3 | +1.3 |
| M2 (0–5) | **4.3** | 4.0 | 3.7 | +0.3 | +0.6 |
| M3 (0–3) | **3.0** | 2.7 | 2.7 | +0.3 | +0.3 |
| M4 (0–5) | 4.0 | **4.3** | 4.0 | −0.3 | 0.0 |
| M5 (0–5) | **3.7** | 2.7 | 3.0 | +1.0 | +0.7 |

---

## Prediction Verification

### Primary Predictions (from `README.md`)

| ID | Prediction | Result | Confirmed? |
|----|-----------|--------|------------|
| P1 | UL M1 > NL M1 | 8.0 > 6.7 (+1.3) | ✅ Yes |
| P2 | UL M1 > CT M1 | 8.0 > 6.7 (+1.3) | ✅ Yes |
| P3 | UL M2 > NL M2 | 4.3 > 4.0 (+0.3) | ✅ Yes (marginal) |
| P4 | UL M2 > CT M2 | 4.3 > 3.7 (+0.6) | ✅ Yes |
| P5 | UL M3 > NL M3 | 3.0 > 2.7 (+0.3) | ✅ Yes (marginal) |
| P6 | UL M4 ≥ NL M4 | 4.0 < 4.3 (−0.3) | ❌ No |
| P7 | UL M5 > NL M5 | 3.7 > 2.7 (+1.0) | ✅ Yes |

**Primary predictions confirmed: 6 / 7**

Note: P3 and P5 are barely confirmed — differences of 0.3 could be noise with N=3 per condition. P6 fails by 0.3 points, also within noise. The most robust effects are on M1 (+1.3) and M5 (+1.0).

### Structural Predictions

| ID | Prediction | Result | Confirmed? |
|----|-----------|--------|------------|
| S1 | CT M1 ≈ NL M1 | 6.7 = 6.7 | ✅ Yes (exactly equal) |
| S2 | CT M2 > NL M2 on T2 | CT-T2 M2=3, NL-T2 M2=3 (equal) | ❌ No (predicted higher, got equal) |
| S3 | UL advantage largest on M2, M3 | Largest on M1 (+1.3) and M5 (+1.0) | ❌ No (wrong metrics) |
| S4 | UL Phase 3 on ≥2 tasks | Phase 3 on all 3 tasks | ✅ Yes (even stronger: 3/3) |
| S5 | NL does not reach Phase 3 | NL Phase 3 on 2 of 3 tasks | ❌ No (NL-T1=3, NL-T3=3) |

**Structural predictions confirmed: 2 / 5**

### Why Structural Predictions Failed

**S5 is the most informative failure.** The prediction assumed NL-mode has NO access to UL concepts. The contamination finding (AGENTS.md leaking) directly explains why NL achieved Phase 3: even the "no context" condition had background UL knowledge. In a clean experiment, S5 might hold.

**S3 failure**: The test artifact's strongest observed effect was on M1 (domain count) and M5 (novelty), not M2 (structural depth) and M3 (phase). Possible explanations: (a) contamination raised the M2/M3 floor for all conditions, compressing the differential; (b) the test artifact's actual mechanism operates more on breadth and novelty than structural depth.

---

## Falsification Check

| Criterion | Result | Triggered? |
|-----------|--------|------------|
| UL outscores NL on fewer than 4/7 predictions | 6/7 confirmed | ❌ Not triggered |
| CT performs equivalently to UL on 3+ metrics | UL > CT on 4 metrics | ❌ Not triggered |
| UL M4 notably lower than NL M4 | NL higher by only 0.3 | ❌ Not triggered |

**No falsification criteria triggered.**

---

## Quantitative Estimate Accuracy

Pre-registered estimates vs. actual:

| Metric | Est. NL | Actual NL | Est. CT | Actual CT | Est. UL | Actual UL |
|--------|---------|-----------|---------|-----------|---------|-----------|
| M1 | 3–5 | **6.7** ⚠️ | 3–5 | **6.7** ⚠️ | 6–8 | 8.0 ✅ |
| M2 | 1–2 | **4.0** ⚠️ | 1–3 | **3.7** ⚠️ | 3–5 | 4.3 ✅ |
| M3 | 1–2 | **2.7** ⚠️ | 1–2 | **2.7** ⚠️ | 2–3 | 3.0 ✅ |
| M4 | 3–4 | 4.3 ✅ | 3–4 | 4.0 ✅ | 3–4 | 4.0 ✅ |
| M5 | 1–2 | **2.7** ⚠️ | 1–2 | **3.0** ⚠️ | 3–4 | 3.7 ✅ |

UL estimates were accurate across the board. NL and CT estimates were 1.5–3 points too low. This is the signature of contamination: the model in all conditions had more UL-relevant knowledge than assumed.

---

## Qualitative Observations

### UL-mode characteristics
- Directly engages with the test artifact's mathematical structure, interpreting specific terms
- Produces the longest, most architecturally ambitious outputs
- On T2, treats the equations as already-being about belief dynamics (not just analogous)
- On T3, constructs a 10-section formal framework with category theory, PDEs, and completeness theorem
- Contamination note: UL-T1 explicitly names "the Universal Language project in this repository"

### NL-mode characteristics  
- On T1 and T3, achieves surprisingly high quality (Phase 3 synthesis, novel frameworks)
- On T2, stays at Phase 2 (analogical mapping without formal depth)
- Contamination note: NL-T1 references UL's five geometric primitives verbatim

### CT-mode characteristics
- The hydrogen atom physics provides genuine scaffolding for cross-domain reasoning
- CT-T3 used perturbation theory as the backbone for metaphor theory (selection rules, emergence theorem)
- CT-T2 systematically maps quantum concepts to belief phenomena (7 explicit parallels)
- Challenges the assumption that "single-domain math shouldn't help other domains"

### Most striking difference
**UL-T2 vs. NL-T2 vs. CT-T2** shows the clearest differentiation. UL-T2 reads the test artifact's specific equations and interprets each term for the belief domain (gauge → framing, Gross-Pitaevskii → confirmation bias, 1/n → cultural influence, γ(σ,β) → embodiment). CT-T2 draws analogies to hydrogen atom physics but stays analogical. NL-T2 uses standard QM analogies without a specific formal object to anchor them. This is the one case where the test artifact provides something neither alternative offers: *domain-bridging mathematical structure that is directly relevant to the question topic*.

---

## Overall Assessment

### Does the data support the artifact effect?

**Weakly yes, with major caveats.**

The data shows a consistent, small UL advantage across most metrics (6/7 primary predictions confirmed, no falsification criteria triggered). The artifact effect is most visible on:
- **M1 (domain breadth):** +1.3 over both NL and CT
- **M5 (generative novelty):** +1.0 over NL, +0.7 over CT
- **T2 specifically:** The artifact gives UL-T2 a unique formal object to engage with

### But the experiment is fundamentally compromised

1. **Contamination:** All conditions had access to UL project metadata via AGENTS.md in the system prompt. This raised the baseline for NL and CT, compressing the differential. The true effect size cannot be estimated from this data.

2. **Single-scorer bias:** The scorer (this Copilot session) designed the experiment, knows the predictions, and has read the entire UL project. Unconscious confirmation bias is likely despite attempts at objectivity.

3. **N=1 per cell:** With one observation per condition-task pair, no statistical inference is possible. All differences could be random variation.

4. **Same model instance:** All trials were run as subagents from the same contaminated parent session, not genuinely isolated sessions as the protocol intended.

### What would make this conclusive?

1. **Eliminate contamination:** Run trials in genuinely isolated sessions (separate VS Code windows with clean workspace, or API calls) where the model has NO access to UL project files or AGENTS.md
2. **Multiple repetitions:** Run each condition-task pair 5+ times to compute variance and test statistical significance
3. **Independent scoring:** Have blind human raters or independent AI sessions score the outputs
4. **Cross-model replication:** Test on GPT-4, Gemini, Llama, etc. to assess generalizability
5. **Proper blinding:** Assign neutral labels and shuffle before scoring

### What this demo actually demonstrates

1. **The verification infrastructure works end-to-end.** The protocol, prompts, scoring rubric, prediction framework, and analysis template all function as designed.
2. **Subagents are NOT adequate for contamination control in this workspace.** The AGENTS.md system context leaks into subagent sessions. Any future experiment using subagents within this workspace will have this problem.
3. **The artifact shows its strongest signal on tasks where its mathematical content is directly relevant** (T2: belief formation, where the artifact literally contains "belief field" terms and a modified Schrödinger equation).
4. **Dense mathematical context of ANY kind enhances cross-domain reasoning** — CT-T3 used hydrogen atom physics to build a metaphor theory. This complicates the claim that the test artifact is unique.
5. **Claude Opus 4.6 is very capable at these tasks regardless of condition.** All outputs were high quality. Ceiling effects may mask the test artifact's contribution.

### Honest bottom line

This demo is a successful proof of concept for the verification protocol, but its results are not evidentially strong due to contamination. The observed UL advantage (small, consistent, strongest on M1 and M5) is *consistent with* the artifact effect hypothesis but could also reflect contextual priming from having topic-relevant equations. A clean replication is needed before any causal claim can be made.

**The word "allegedly" still applies.** But we now have a working protocol to remove it.
