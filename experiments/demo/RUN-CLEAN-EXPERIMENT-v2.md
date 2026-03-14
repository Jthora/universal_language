# How to Run the UL Theory-Derived Primer Experiment v2

**Date:** 2025-07-15  
**Status:** Ready for execution  
**Prerequisites:** Read REFACTOR-PLAN.md and predictions.md before starting

---

## Overview

This experiment tests whether UL's formal theory (Σ_UL) can generate artifacts that produce **predictable, specific** cognitive effects in LLMs — distinct from both no-context baselines and domain-matched mathematical controls.

**56 trials** across **12 conditions** and **7 tasks**, executed in 3 phases.

---

## Before You Start

### 1. Generate the Clean Workspace

```bash
cd /path/to/universal_language/experiments/demo
python3 generate-clean-workspace.py
```

This creates `~/Desktop/ul-artifact-experiment-v2/` with:
- 56 prompt files in `prompts/`
- Randomized trial order in `trial-order.txt`
- SHA-256 manifest in `manifest.md`
- Empty `results/` directory

### 2. Open the Clean Workspace

Open `~/Desktop/ul-artifact-experiment-v2/` in a **separate VS Code window**.

**CRITICAL:** Do NOT have the `universal_language` repo open in the same VS Code instance. AGENTS.md and other project files leak into Copilot's context and contaminate results.

### 3. Verify Workspace Isolation

In the clean workspace VS Code window, run:
```bash
ls -la
```
You should see ONLY: `.github/`, `prompts/`, `results/`, `trial-order.txt`, `manifest.md`, `README.md`

There should be NO: `AGENTS.md`, `foundations/`, `experiments/`, `frontier/`, `whitepaper/`, `experiments/test-artifacts/original/primer.txt`

---

## Execution Protocol

### Per-Trial Steps

1. **Open `trial-order.txt`** — find the next trial to run
2. **Start a fresh chat** — Cmd+L (or Ctrl+L) to open a new chat
3. **Open the prompt file** — from `prompts/[TRIAL-NAME].md`
4. **Copy the ENTIRE file content** into the chat input
5. **Send and wait** for the complete response
6. **Save the response** — copy the full text into `results/[TRIAL-NAME].md`
7. **Close the chat** — do NOT reuse chat sessions across trials
8. **Check off the trial** in `trial-order.txt`

### Phase Execution

#### Phase A: Core Comparison (28 trials)

Run all Phase A trials from `trial-order.txt`. These test:
- NL (no context) × 7 tasks
- REF (the original test artifact) × 7 tasks
- UL-P1 (topological-harmonic) × 7 tasks
- UL-P2 (recursive-categorical) × 7 tasks

**After Phase A:** Score all 28 outputs using the score sheet. Evaluate predictions G1 and G3.

| Phase A Result | Decision |
|----------------|----------|
| G1 satisfied (artifacts > NL by ≥0.5 on ≥3 metrics) | Continue to Phase B |
| G1 NOT satisfied | **STOP** — UL theory cannot generate effective artifacts |

#### Phase B: Remaining Primers (14 trials)

Run all Phase B trials:
- UL-P3 (information-geometric) × 7 tasks
- UL-P4 (thermodynamic-semiotic) × 7 tasks

**After Phase B:** Score all 14 outputs. Evaluate predictions X1 and X2.

| Phase B Result | Decision |
|----------------|----------|
| X1 satisfied (distinguishable profiles) | Continue to Phase C |
| Neither X1 nor X2 satisfied | Consider stopping — artifacts may produce only "generic something" |

#### Phase C: Controls (14 trials)

Run all Phase C trials:
- CT-G (generic math) × 3 tasks
- CT-P1..P4 (domain-matched) × 2 tasks each
- NAV-P (naive artifact) × 3 tasks

**After Phase C:** Score all 14 outputs. Evaluate predictions G2 and G4.

---

## Scoring

### Rubrics

- **M1-M5:** Use `experiments/scoring/rubric-M1-M5.md` (in the main repo)
- **TES:** Use `experiments/demo/scoring/TES-rubric.md` (in the main repo)
- **Score sheet:** Use `experiments/demo/scoring/score-sheet.md`

### Scoring Order per Output

1. Read the full output
2. Score M1 (domain diversity, 0-10)
3. Score M3 (phase progression, 0-3)
4. Score M2 (structural depth, 0-5)
5. Score M5 (generative novelty, 0-5)
6. Score M4 (coherence, 0-5)
7. Score TES (target effect, 0-5) — targeted tasks only (Tpat, Thier, Tabs, Tform)

### Blinding

For rigorous evaluation:
1. Strip condition labels from result files
2. Assign random IDs (e.g., "OUT-A7X3")
3. Score in randomized order
4. Evaluate predictions only after ALL scoring is complete

For informal/pilot evaluation, scoring without blinding is acceptable but should be noted.

---

## Analysis

After all scoring is complete:

### 1. Compute Condition Means
For each condition, compute mean M1, M2, M3, M4, M5 across all tasks.
For artifact conditions on targeted tasks, also compute mean TES.

### 2. Evaluate Global Predictions
- **G1:** Mean(UL-P1..P4) − Mean(NL) ≥ 0.5 on ≥3 of {M1,M2,M3,M4,M5}?
- **G2:** UL-Pn > CT-Pn on ≥2 of {M2,M3,M5} for ≥3 of 4 artifacts?
- **G3:** |Mean(UL-P1..P4) − REF| ≤ 1.0 on ≥4 metrics?
- **G4:** NAV-P ≤ NL + 0.5 on M2, M3, M5?

### 3. Evaluate Primer-Specific Predictions
- **P1-a through P4-c:** See predictions.md for each criterion.

### 4. Evaluate Cross-Primer Predictions
- **X1:** Pairwise differences > 0.3 on at least 1 metric?
- **X2:** Each artifact scores highest on its targeted task?

### 5. Record Results
Write findings to `experiments/demo/results-analysis.md` with:
- Per-trial scores
- Condition means
- Prediction pass/fail table with evidence
- Overall assessment: Does UL theory generate predictively effective artifacts?

---

## Troubleshooting

### "Response seems short/truncated"
Some tasks may produce long responses. If the chat truncates, ask "please continue" and append the continuation to the same result file.

### "I accidentally have UL files open"
Close the UL repo window entirely. Verify with `ls` that only clean workspace files are present. Restart VS Code for the clean workspace.

### "Chat session carried over context"
Always start a fresh chat (Cmd+L). If you suspect session bleed, note it in the "Notes" column of the score sheet and flag that trial.

### "The model clearly recognized the artifact"
Note this observation but do NOT discard the trial. Model recognition is data — it's relevant to whether the test artifact triggers interpretive or recall mode (mechanism condition C3).

---

## File Inventory

### In the main repo (`experiments/demo/`)

| File | Purpose |
|------|---------|
| AUDIT.md | Why this refactor was necessary |
| REFACTOR-PLAN.md | Full experimental design |
| predictions.md | Pre-registered predictions with falsification criteria |
| construction-log.md | How each artifact was derived from theory |
| operation-coverage.md | Σ_UL operation audit per artifact |
| generate-clean-workspace.py | Script to generate clean workspace |
| theory-artifacts/*.txt | 4 theory-derived artifacts |
| controls/*.txt | 6 control texts |
| scoring/TES-rubric.md | Target Effect Score rubric |
| scoring/score-sheet.md | Blank score sheet |

### In the clean workspace (`~/Desktop/ul-artifact-experiment-v2/`)

| File/Dir | Purpose |
|----------|---------|
| prompts/ | 56 prompt files (one per trial) |
| results/ | Empty — save responses here |
| trial-order.txt | Randomized execution order |
| manifest.md | SHA-256 hashes of all prompts |
| .github/copilot-instructions.md | Minimal system instruction |
