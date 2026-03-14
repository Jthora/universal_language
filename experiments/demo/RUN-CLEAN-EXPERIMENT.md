# How to Run a Clean Experiment

> **Step-by-step instructions for running an uncontaminated artifact verification experiment using VS Code + GitHub Copilot + Claude Opus 4.6**

---

## Why These Steps Matter

Our first attempt failed because `AGENTS.md` in the project root automatically gets injected into every Copilot session. Even "no context" trials had access to UL project descriptions, five geometric primitives, and the entire theory. The contamination was so thorough that the NL (baseline) responses quoted UL terminology verbatim.

**The fix is simple: open the clean workspace as its own top-level folder in a completely separate VS Code window.** This prevents AGENTS.md and all other project files from entering Copilot's context.

---

## Prerequisites

- VS Code with GitHub Copilot extension
- Claude Opus 4.6 selected as your Copilot model
- About 30–45 minutes of focused time (9 trials)

---

## Step 1: Copy the Clean Workspace Out

The clean workspace lives inside the UL project — we need to move a copy **outside** so it doesn't inherit the project's AGENTS.md.

Open Terminal and run:

```bash
cp -r ~/Documents/GitHub/universal_language/experiments/demo/clean-workspace ~/Desktop/artifact-experiment
```

This creates a completely isolated copy at `~/Desktop/artifact-experiment/`.

**Verify it contains NO UL project files:**
```bash
ls ~/Desktop/artifact-experiment/
# Should show ONLY: .github/  README.md  prompts/  results/
# Should NOT contain: AGENTS.md, foundations/, whitepaper/, the original test artifact, etc.
```

---

## Step 2: Open in a Separate VS Code Window

1. **Close** this VS Code window (or at minimum, do NOT use this window for trials)
2. Open a **new** VS Code window: `File → New Window`
3. `File → Open Folder...` → navigate to `~/Desktop/artifact-experiment/`
4. Confirm that Copilot shows **Claude Opus 4.6** as the selected model

**Critical check:** The workspace should show ONLY:
```
.github/
  copilot-instructions.md    (contains only "You are a helpful AI assistant.")
README.md
prompts/
  CT-T1.md, CT-T2.md, CT-T3.md
  NL-T1.md, NL-T2.md, NL-T3.md
  UL-T1.md, UL-T2.md, UL-T3.md
results/
  .gitkeep.md
```

If you see AGENTS.md, the original test artifact, foundations/, or any other UL project files — STOP. You're in the wrong folder.

---

## Step 3: Run the 9 Trials

Follow this **exact order** (pre-randomized to prevent ordering effects):

| Trial # | File to Open | What It Tests |
|---------|-------------|---------------|
| 1 | `prompts/NL-T2.md` | No context + QM & belief |
| 2 | `prompts/CT-T1.md` | Physics context + language/consciousness/math |
| 3 | `prompts/UL-T3.md` | **Primer** + metaphor framework |
| 4 | `prompts/UL-T1.md` | **Primer** + language/consciousness/math |
| 5 | `prompts/NL-T3.md` | No context + metaphor framework |
| 6 | `prompts/CT-T2.md` | Physics context + QM & belief |
| 7 | `prompts/CT-T3.md` | Physics context + metaphor framework |
| 8 | `prompts/UL-T2.md` | **Primer** + QM & belief |
| 9 | `prompts/NL-T1.md` | No context + language/consciousness/math |

### For EACH trial:

1. **Open a NEW Copilot chat** (click the `+` icon or `Cmd+Shift+I` for a fresh inline chat, then switch to the Chat panel with a new session)
2. **Open the prompt file** listed for this trial
3. **Select ALL** the text in the prompt file (`Cmd+A`)
4. **Copy** it (`Cmd+C`)
5. **Paste** it into the Copilot chat input (`Cmd+V`)
6. **Send** it and wait for the full response
7. **Copy the entire response**
8. **Create a new file** in `results/` with the same name as the prompt file (e.g., `results/NL-T2.md`)
9. **Paste** the full response and save

### Rules During Trials

- **Do NOT** add any text before or after the copied prompt
- **Do NOT** send follow-up messages
- **Do NOT** tell the model what you're testing or what the artifact is
- **Do NOT** open any other files between trials (prevent context leakage)
- **Use a FRESH chat for every single trial** — never reuse a chat session

---

## Step 4: Bring Results Back

After all 9 trials are complete, copy the results back to the project:

```bash
cp ~/Desktop/artifact-experiment/results/*.md ~/Documents/GitHub/universal_language/experiments/demo/results-clean/
```

(Create the `results-clean/` directory if it doesn't exist.)

---

## Step 5: Score the Outputs

### Option A: Score Them Yourself (With Blinding)

1. Come back to this VS Code window (the main UL project)
2. Rename the 9 result files using neutral labels:

```bash
cd ~/Documents/GitHub/universal_language/experiments/demo/results-clean/

# Create blinding map (save this separately — don't peek until done scoring!)
echo "A=NL-T2, B=CT-T1, C=UL-T3, D=UL-T1, E=NL-T3, F=CT-T2, G=CT-T3, H=UL-T2, I=NL-T1" > BLINDING_KEY.txt

# Rename files
mv NL-T2.md output-A.md
mv CT-T1.md output-B.md
mv UL-T3.md output-C.md
mv UL-T1.md output-D.md
mv NL-T3.md output-E.md
mv CT-T2.md output-F.md
mv CT-T3.md output-G.md
mv UL-T2.md output-H.md
mv NL-T1.md output-I.md
```

3. Score all 9 outputs on **one metric at a time** (all outputs on M1, then all on M2, etc.)
4. Use the rubric in `experiments/verification-guides/simplified-scoring.md`
5. After scoring all metrics, reveal the blinding key and fill in `analysis.md`

### Option B: AI-Assisted Scoring (Recommended for Independence)

1. Open a **different** AI chat (ChatGPT, Claude.ai, Gemini — NOT this Copilot session)
2. For each output, paste this scoring prompt (from `simplified-scoring.md`):

```
You are an evaluator scoring an LLM output on five metrics. Score strictly according to the rubric below. Do not be generous — apply the anchors literally.

RUBRIC:

M1 — Domain Diversity (0–10): Count distinct knowledge domains substantively engaged (not name-dropped). A domain counts if the output makes at least one technical claim within it.

M2 — Cross-Domain Structural Depth (0–5):
0 = no cross-domain connection
1 = surface analogy ("X is like Y")
2 = shared property identified with explanation
3 = structural parallel with multiple corresponding elements
4 = formal isomorphism with precise mathematical correspondence
5 = novel framework constructed that unifies domains

M3 — Phase Progression (0–3):
0 = off-topic
1 = competent single-domain analysis
2 = active cross-domain connection-making
3 = emergent synthesis FROM the intersection (not just about connections)

M4 — Coherence (0–5):
0 = incoherent
1 = partially coherent
2 = moderate
3 = substantial
4 = high
5 = rigorous (publication quality)

M5 — Generative Novelty (0–5):
0 = all standard textbook material
1 = one minor novel observation
2 = 1-2 non-obvious connections
3 = genuinely surprising insights
4 = substantial novel framework
5 = publishable-quality novelty

OUTPUT TO SCORE:

[PASTE OUTPUT HERE]

Provide your scores in this exact format:
M1: [score] — [brief justification]
M2: [score] — [brief justification]
M3: [score] — [brief justification]
M4: [score] — [brief justification]
M5: [score] — [brief justification]

List the specific domains counted for M1.
Identify the deepest cross-domain connection for M2.
Quote the passage that demonstrates the highest phase for M3.
```

3. Score all 9 blinded outputs (use the neutral labels A–I)
4. Reveal the blinding key only after ALL scoring is done

### Option C: Do Both

Use self-scoring (Option A) AND AI-scoring (Option B) and compare for inter-rater reliability. This is the strongest approach.

---

## Step 6: Fill In the Analysis

Come back to this project and open `experiments/demo/analysis.md`. Fill in:
- Raw scores per task
- Condition averages
- Prediction verification (check against `experiments/demo/README.md`)
- Falsification check
- Qualitative observations

---

## What to Look For

### If the artifact effect is real, you should see:

| Pattern | Why |
|---------|-----|
| UL outputs reference more distinct domains than NL or CT | Primer activates cross-domain pathways |
| UL outputs achieve deeper structural connections (M2 ≥ 3) while NL stays at M2 ≤ 2 | Primer forces structural binding |
| UL outputs reach Phase 3 (emergent synthesis); NL stays at Phase 1–2 | Primer enables speaking FROM the intersection |
| CT outputs perform similarly to NL (not to UL) | Standard physics doesn't replicate the artifact's cross-domain effect |
| UL maintains coherence (M4) despite higher ambition | The artifact scaffolds, not scrambles |

### If the artifact effect is NOT real, you should see:

| Pattern | Meaning |
|---------|---------|
| NL outputs ≈ UL outputs across all metrics | The artifact adds nothing |
| CT outputs ≈ UL outputs | Any math works equally well |
| UL M4 (coherence) notably lower than NL | The artifact causes confused overreach |

---

## Contamination Checklist (Verify After Trials)

After running all trials, check the NL outputs for these red flags:

- [ ] Does any NL output mention "point, line, angle, curve, enclosure"?
- [ ] Does any NL output reference "Universal Language" or "UL"?
- [ ] Does any NL output mention "5 geometric primitives"?
- [ ] Does any NL output reference "Σ_UL" or "meaning algebra"?
- [ ] Does any NL output mention AGENTS.md, test artifact, or this repository?

If **any** box is checked: contamination is present, and results cannot be trusted as clean baselines. The most likely cause is that UL project files are still visible to Copilot (you may be in the wrong workspace).

If **none** are checked: you have a clean experiment. Proceed to scoring with confidence.

---

## Quick Reference Card

```
1. cp clean-workspace to ~/Desktop/artifact-experiment
2. Open NEW VS Code window → Open Folder → ~/Desktop/artifact-experiment
3. Verify: no AGENTS.md, no UL project files visible
4. Run 9 trials in pre-randomized order (fresh chat each time)
5. Copy results back to project
6. Blind → Score → Reveal → Analyze
```

Total time: ~45 minutes for trials, ~30 minutes for scoring, ~15 minutes for analysis.

---

## After the Experiment

Bring your scored results back to the main project and we can:
1. Compare clean results against the contaminated pilot data
2. Determine whether the artifact effect holds up
3. Decide whether to proceed to a larger multi-model study

Good luck. Be honest with the data — the experiment is designed to detect truth in either direction.
