# Manual Chat Interface Verification Protocol

**Interface:** Any browser-based LLM chat (ChatGPT, Claude.ai, Gemini, HuggingChat, etc.)  
**Model:** Whatever is available on the free tier  
**Cost:** Free  
**Automation level:** Fully manual  
**Estimated time:** 30–60 minutes for a quick test (9 trials)

---

## Overview

This is the lowest-barrier approach. Anyone with a web browser and internet access can run this experiment using the free tiers of ChatGPT, Claude.ai, Google Gemini, or any other chat interface.

---

## Prerequisites

- A web browser
- An account on at least one LLM chat service:
  - [ChatGPT](https://chat.openai.com) (free tier: GPT-4o-mini or GPT-4o limited)
  - [Claude.ai](https://claude.ai) (free tier: Claude Sonnet)
  - [Google Gemini](https://gemini.google.com) (free tier: Gemini)
  - [HuggingChat](https://huggingface.co/chat) (free, multiple open models)

---

## Step-by-Step Protocol

### Step 1: Choose a Service and Model

Pick ONE service. Use the SAME model for all trials. Document which service and model you used.

**Recommended for free users:** Claude.ai (Sonnet) or ChatGPT (GPT-4o)

### Step 2: Prepare Prompts

Use the pre-assembled prompts from `experiments/verification-guides/ready-prompts.md`. Each prompt contains the complete text to paste — context block + task.

For a quick test, you need 9 prompts:
- UL-T1, UL-T2, UL-T3 (artifact + task)
- NL-T1, NL-T2, NL-T3 (empty context + task)
- CT-T1, CT-T2, CT-T3 (control text + task)

### Step 3: Run Trials

For EACH trial:

1. **Start a new conversation** (click "New Chat" or equivalent)
2. **Copy the entire prompt** from `ready-prompts.md` for this trial
3. **Paste and send** — do not add any greeting, instruction, or follow-up
4. **Wait for the full response**
5. **Copy the entire response** and save it in a text file
6. **Name the file:** `{condition}_{task}_r{rep}.txt` (e.g., `UL_T1_r1.txt`)

**IMPORTANT:**
- **New chat for every trial.** Never reuse a conversation.
- **No system prompt modification.** Use the default chat behavior (do not use "Custom Instructions" on ChatGPT or similar features).
- **Paste exactly.** Don't paraphrase, add context, or explain what you're doing.
- **Randomize order.** Don't do all UL trials first. Interleave conditions.

### Step 4: Score and Compare

Follow the scoring process in `simplified-scoring.md`:
1. Rename output files to neutral labels (A–I)
2. Score each on M1–M5
3. Reveal condition labels
4. Compare

---

## Platform-Specific Notes

### ChatGPT
- Free tier may default to GPT-4o-mini. Document which model responded.
- "Custom Instructions" must be EMPTY (Settings → Custom Instructions → clear both fields).
- GPT Plus users can select GPT-4o explicitly.

### Claude.ai
- Free tier uses Claude Sonnet.
- No custom instructions feature — default behavior is appropriate.
- Daily message limits may apply. Plan accordingly (you may need to spread trials across days).

### Google Gemini
- The chat interface may auto-select the model. Document what it says.
- Gemini may format responses with more visual elements (headers, bullets). This doesn't affect scoring.

### HuggingChat
- Select a specific model (e.g., Llama-3.1-70B) and use the same one throughout.
- Open-source models may be useful for comparison — if the artifact effect appears across architectures (Llama, Mistral, etc.), that's stronger evidence.

---

## Tips for Quality Data

1. **Run trials on the same day** if possible — model versions can change between days.
2. **Screenshot the model name/version** visible in the interface for documentation.
3. **Don't read outputs until all trials are done.** Reading UL-mode output before running NL-mode might bias your behavior (e.g., unconsciously picking a different model setting).
4. **If a trial fails** (model refuses, gives very short response, or has an error), document it and re-run ONCE. If it fails again, record the failure.

---

## Multi-Model Comparison (Bonus)

If you have access to multiple services, running the same 9-trial quick test on each gives you cross-model data. This is extremely valuable — it tests whether the artifact effect is model-specific or general.

Example: Run the quick test on ChatGPT, Claude.ai, AND Gemini. That's 27 trials and provides a strong initial dataset.
