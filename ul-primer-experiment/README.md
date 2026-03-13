# UL Theory-Derived Primer Experiment v2

## Clean Workspace

This workspace contains NO Universal Language source material, theory documents, or AGENTS.md.
It is a clean execution environment for the experiment.

## Structure

- `prompts/` — 56 prompt files, one per trial. Each contains the full text to paste into a fresh chat.
- `results/` — Save each trial's response here with the same filename as the prompt.
- `trial-order.txt` — Randomized execution order. Follow this sequence exactly.
- `manifest.md` — SHA-256 hashes of all prompt files for integrity verification.

## Execution Protocol

1. Open this folder in a **separate VS Code window** (not the UL repo)
2. Open `trial-order.txt` and follow the order
3. For each trial:
   a. Start a **fresh chat** (Cmd+L → new)
   b. Open the prompt file from `prompts/`
   c. Copy-paste the ENTIRE content into the chat
   d. Wait for complete response
   e. Copy the response into `results/[same-filename]`
   f. **Close the chat** — do NOT reuse
4. Run all Phase A trials first. Evaluate. Then Phase B. Then Phase C.

## Important

- Do NOT open any UL project files while running trials
- Do NOT modify prompt files after generation
- Do NOT skip the randomized order
- Each chat session = one trial only
