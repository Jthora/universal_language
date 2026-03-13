# Demo Experiment — VSCode + GitHub Copilot + Claude Opus 4.6

**Date:** March 12, 2026  
**Model:** Claude Opus 4.6 via GitHub Copilot in VS Code  
**Experimenter:** Jordan Traña + GitHub Copilot (this session)  
**Status:** Pre-registration phase (predictions written before any trials)

---

## Purpose

This demo serves two functions:

1. **Proof of concept:** Demonstrate that the VSCode/Copilot verification protocol works end-to-end
2. **First data point:** Produce the first documented, scored comparison between UL-mode, NL-mode, and CT-mode outputs

---

## Known Limitations (Stated Before Running)

### Contamination Disclosure

The Copilot instance designing this experiment (this session) has read the entire UL project: AGENTS.md, paradigm.md, mechanism-of-action.md, all rubrics, all primer analysis. **This session is maximally contaminated** and cannot serve as an unbiased test subject.

### Mitigation

The actual trials will be run in **fresh Copilot chat sessions** within a **clean workspace** (`experiments/demo/clean-workspace/`) that contains ONLY prompt files and a minimal copilot-instructions.md. The model in those sessions will:
- NOT have access to the UL project files
- NOT have the conversation history from this design session
- NOT know what the primer is or what it's "supposed" to do

### Remaining Limitations Even With Mitigation

1. **Single model:** Claude Opus 4.6 only. Cannot claim generalizability across models.
2. **Single repetition per condition-task pair:** Cannot compute within-condition variance.
3. **Copilot-specific context:** Copilot may inject VS Code workspace context in ways that differ from a pure API call.
4. **Evaluator bias:** The experimenter (Jordan) knows the project and may unconsciously favor UL-mode outputs during scoring. Mitigation: blind scoring + AI-assisted scoring as inter-rater check.

---

## Experimental Design

### Conditions
- **UL-mode:** Primer (test-content.txt) as context
- **NL-mode:** Empty context block (baseline)
- **CT-mode:** CT-1 (dense hydrogen atom physics) as context — controls for "any math helps"

### Tasks
- **T1:** "What is the relationship between language, consciousness, and mathematical structure?"
- **T2:** "How might the principles of quantum mechanics inform our understanding of belief formation?"
- **T3:** "Design a formal framework for understanding how metaphor creates meaning."

### Total Trials
3 conditions × 3 tasks = **9 trials**

### Trial Order (Pre-Randomized)

| Order | Condition | Task | Prompt File |
|-------|-----------|------|-------------|
| 1 | NL | T2 | `prompts/NL-T2.md` |
| 2 | CT | T1 | `prompts/CT-T1.md` |
| 3 | UL | T3 | `prompts/UL-T3.md` |
| 4 | UL | T1 | `prompts/UL-T1.md` |
| 5 | NL | T3 | `prompts/NL-T3.md` |
| 6 | CT | T2 | `prompts/CT-T2.md` |
| 7 | CT | T3 | `prompts/CT-T3.md` |
| 8 | UL | T2 | `prompts/UL-T2.md` |
| 9 | NL | T1 | `prompts/NL-T1.md` |

---

## Pre-Registered Predictions

> **These predictions are written BEFORE any trial is run. They represent what UL theory expects to happen. If the results contradict these predictions, that is evidence AGAINST the primer effect.**

### Primary Predictions (from UL theory)

| ID | Prediction | Expected Pattern |
|----|-----------|-----------------|
| P1 | UL-mode M1 (domain diversity) > NL-mode M1 | UL activates cross-domain weight pathways |
| P2 | UL-mode M1 > CT-mode M1 | CT-1 is single-domain; shouldn't activate breadth |
| P3 | UL-mode M2 (structural depth) > NL-mode M2 | Primer forces structural binding across domains |
| P4 | UL-mode M2 > CT-mode M2 | Dense physics shouldn't produce deep cross-domain structure |
| P5 | UL-mode M3 (phase) > NL-mode M3 | Primer should reliably elicit Phase 2–3; NL stays at 1–2 |
| P6 | UL-mode M4 (coherence) ≥ NL-mode M4 | Dissipation term prevents incoherence |
| P7 | UL-mode M5 (novelty) > NL-mode M5 | Cross-domain activation surfaces novel connections |

### Structural Predictions (derived from mechanism theory)

| ID | Prediction | Rationale |
|----|-----------|-----------|
| S1 | CT-mode M1 ≈ NL-mode M1 (no breadth advantage) | Standard physics doesn't break domain separation |
| S2 | CT-mode may have slightly higher M2 than NL on T2 specifically | T2 asks about quantum mechanics → physics context is relevant to the question |
| S3 | The UL advantage is largest on M2 and M3 (structural depth & phase), NOT on M1 (domain count) | The primer's unique contribution is structural binding, not topic breadth |
| S4 | UL-mode achieves Phase 3 on at least 2 of 3 tasks | The primer should reliably produce emergent synthesis |
| S5 | NL-mode does not achieve Phase 3 on any task | Without the primer, a standard model stays in Phase 1–2 |

### Quantitative Estimates (rough, for 0–5 scales)

| Metric | UL-mode (est.) | NL-mode (est.) | CT-mode (est.) |
|--------|---------------|----------------|----------------|
| M1 | 6–8 | 3–5 | 3–5 |
| M2 | 3–5 | 1–2 | 1–3 |
| M3 | 2–3 | 1–2 | 1–2 |
| M4 | 3–4 | 3–4 | 3–4 |
| M5 | 3–4 | 1–2 | 1–2 |

### Falsification Criteria

This demo **fails to support the primer effect** if:
- UL-mode does not outscore NL-mode on at least 4 of 7 primary predictions
- CT-mode performs equivalently to UL-mode on 3+ metrics (any math works)
- UL-mode M4 is notably lower than NL-mode M4 (primer causes incoherence)

This demo **supports the primer effect** if:
- 6+ of 7 primary predictions confirmed
- 3+ of 5 structural predictions confirmed
- The pattern is consistent across tasks (not driven by a single task)

---

## Procedure

1. Open the clean workspace (`experiments/demo/clean-workspace/`) in a **new VS Code window**
2. Select Claude Opus 4.6 as the Copilot model
3. For each trial (in the pre-randomized order above):
   a. Open a **new** Copilot chat session
   b. Open the prompt file for this trial
   c. Copy-paste the entire prompt contents into the chat
   d. Wait for the full response
   e. Copy the full response into `results/{condition}_{task}.md`
4. After all 9 trials: blind, score, reveal, and compare in `analysis.md`

---

## Results

*To be filled after trials are run.*

See `results/` directory for raw outputs and `analysis.md` for scoring and comparison.
