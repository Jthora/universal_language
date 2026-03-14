# VSCode + GitHub Copilot Verification Protocol

**Interface:** GitHub Copilot Chat in VS Code  
**Model:** Any model available through Copilot (Claude, GPT-4o, etc.)  
**Cost:** Included in GitHub Copilot subscription  
**Automation level:** Semi-manual (copy-paste prompts, save outputs)  
**Estimated time:** 30–60 minutes for a quick test (9 trials)

---

## Overview

This protocol uses GitHub Copilot's chat interface in VS Code as the LLM endpoint. Each trial is a fresh chat session where you paste a pre-assembled prompt and save the full response.

The key advantage: **no API keys, no setup, no code to run.** If you have VS Code and Copilot, you can start immediately.

---

## Prerequisites

- VS Code installed
- GitHub Copilot subscription (Individual, Business, or Enterprise)
- A model that supports chat (Claude Sonnet 4, Claude Opus 4.6, GPT-4o, etc.)

---

## Critical: Contamination Control

The validity of this experiment depends on the model NOT having prior context about what the artifact is or what it's "supposed" to do.

### Option A: Clean Workspace (Recommended)

1. Create a new, empty folder somewhere on your computer (e.g., `~/ul-experiment/`)
2. Open THAT folder as a VS Code workspace (`File → Open Folder`)
3. Copy the `experiments/demo/clean-workspace/` contents into it
4. Run all trials in this isolated workspace
5. The model will have NO access to the UL project files, AGENTS.md, paradigm.md, etc.

### Option B: Same Workspace (Acceptable with caveats)

If you run trials within the UL project workspace, note that Copilot may read project files as context. This introduces potential contamination. Document this in your results.

---

## Step-by-Step Protocol

### Step 1: Choose Your Scope

| Scope | Trials | Time |
|-------|--------|------|
| **Minimum** | 3 (one task, three conditions) | 10 min |
| **Quick test** | 9 (three tasks, three conditions) | 30 min |
| **Full test** | 45 (five tasks, three conditions, three reps) | 2–3 hours |

### Step 2: Set Up Your Workspace

Follow Option A or B above. If using the clean workspace, it should contain only:
```
ul-experiment/
├── .github/
│   └── copilot-instructions.md    # "You are a helpful AI assistant."
├── prompts/
│   ├── UL-T1.md    # Primer + Task 1
│   ├── UL-T2.md    # Primer + Task 2
│   ├── UL-T3.md    # Primer + Task 3
│   ├── NL-T1.md    # No context + Task 1
│   ├── NL-T2.md    # No context + Task 2
│   ├── NL-T3.md    # No context + Task 3
│   ├── CT-T1.md    # Control text + Task 1
│   ├── CT-T2.md    # Control text + Task 2
│   └── CT-T3.md    # Control text + Task 3
└── results/         # You'll save outputs here
```

### Step 3: Select Your Model

In VS Code Copilot Chat, select the model you want to use. **Use the same model for ALL trials.** Document which model you used.

Recommended: Claude Opus 4.6 (if available) or Claude Sonnet 4.

### Step 4: Run Trials

For EACH trial:

1. **Open a NEW Copilot chat session** (click the `+` icon or use `Ctrl/Cmd+Shift+I` for a fresh panel)
2. **Open the prompt file** for this trial (e.g., `prompts/UL-T1.md`)
3. **Copy the ENTIRE contents** of the prompt file
4. **Paste into Copilot chat** and send
5. **Wait for the full response**
6. **Copy the ENTIRE response**
7. **Save as a new file** in `results/` with the name format: `{condition}_{task}_{rep}.md`
   - Example: `UL_T1_r1.md`, `NL_T1_r1.md`, `CT_T1_r1.md`

**IMPORTANT:**
- Start a FRESH chat for EVERY trial. Do not reuse chat sessions.
- Do not add any commentary or follow-up questions — one prompt, one response.
- Do not modify the prompt text. Paste it exactly.
- Randomize the ORDER of conditions. Don't run all UL first, then all NL, then all CT. Interleave them.

### Recommended Trial Order (Quick Test)

To minimize order effects, use this sequence:

| Trial # | Condition | Task | Prompt File |
|---------|-----------|------|-------------|
| 1 | NL | T1 | `NL-T1.md` |
| 2 | UL | T2 | `UL-T2.md` |
| 3 | CT | T3 | `CT-T3.md` |
| 4 | UL | T1 | `UL-T1.md` |
| 5 | CT | T2 | `CT-T2.md` |
| 6 | NL | T3 | `NL-T3.md` |
| 7 | CT | T1 | `CT-T1.md` |
| 8 | NL | T2 | `NL-T2.md` |
| 9 | UL | T3 | `UL-T3.md` |

### Step 5: Blind and Score

1. **Blind the outputs:** Rename result files to neutral labels (A through I, or use random IDs). Record the mapping separately but don't look at it while scoring.
2. **Score each output** using the simplified rubric (`simplified-scoring.md`):
   - M1: Count distinct domains substantively referenced (0–10)
   - M2: Deepest cross-domain connection level (0–5)
   - M3: Highest phase reached (0–3)
   - M4: Overall coherence (0–5)
   - M5: Novel connections not in standard textbooks (0–5)
3. **Record scores** in a simple table.
4. **Reveal condition labels** and compare.

### Step 6: AI-Assisted Scoring (Optional)

For a second opinion, you can have Copilot itself score the blinded outputs:

1. Open a NEW Copilot chat
2. Paste the scoring prompt from `simplified-scoring.md` (Section: AI Scoring Prompt)
3. Paste a blinded output (without its condition label)
4. Record the AI's scores
5. Compare with your own scores

This gives you inter-rater data (human + AI), which strengthens the analysis.

### Step 7: Document Results

Create a results summary file with:
- Date and time of trials
- Model used (exact name and version)
- Workspace setup (clean or project workspace)
- Trial order
- Raw scores per output per metric
- Condition reveal and comparison
- Your observations (qualitative differences you noticed)
- Whether the pre-registered predictions (see `README.md`) were confirmed or falsified

---

## Troubleshooting

**Q: Copilot refuses to engage with the mathematical content in the test artifact.**  
A: This is rare but possible. If Copilot produces a refusal or safety response, document it as a data point (it's interesting if UL-mode triggers refusals that NL-mode doesn't). Try a different model if available.

**Q: The response is very short / seems truncated.**  
A: Copilot chat may have shorter default response lengths than API calls. If responses are consistently under 500 words across ALL conditions, note this as a limitation. The relative comparison (UL vs NL vs CT) is still valid.

**Q: I accidentally provided follow-up or had a multi-turn conversation.**  
A: Discard that trial and re-run it in a fresh session. Only single-turn responses are valid.

**Q: Can I use Copilot's inline chat (Ctrl+I) instead of the panel?**  
A: Use the panel chat (`Ctrl+Shift+I` or the chat icon). Inline chat has different context behavior and shorter response limits.

---

## What This Approach Can and Cannot Prove

### Can Prove
- Observable, documented differences between conditions on pre-defined metrics
- Whether those differences match pre-registered predictions from UL theory
- That the protocol is replicable (others can follow these exact steps)

### Cannot Prove (on its own)
- Statistical significance (need more trials, ideally from multiple independent testers)
- That the effect generalizes across models (need to test multiple models)
- That the effect isn't an artifact of Copilot's specific implementation (need API-based replication)

### Strongest Evidence Comes From
Multiple independent people following this protocol with different models and reporting both positive and negative results.
