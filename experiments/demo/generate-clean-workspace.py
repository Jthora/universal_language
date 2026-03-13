#!/usr/bin/env python3
"""Generate the clean workspace for the UL Theory-Derived Primer Experiment v2.

Creates ~/Desktop/ul-primer-experiment-v2/ with:
  .github/copilot-instructions.md
  prompts/  (56 prompt files)
  results/  (empty, for output)
  trial-order.txt (randomized execution order)
"""

import os
import random
import hashlib
from datetime import datetime

# ─────────────────────────────────────────────
# Configuration
# ─────────────────────────────────────────────

BASE = os.path.expanduser("~/Desktop/ul-primer-experiment-v2")
REPO = os.path.dirname(os.path.abspath(__file__))  # experiments/demo/
REPO_ROOT = os.path.dirname(os.path.dirname(REPO))  # universal_language/

# Task prompts (exact text, verbatim from protocol)
TASKS = {
    "T1": "What is the relationship between language, consciousness, and mathematical structure?",
    "T2": "How might the principles of quantum mechanics inform our understanding of belief formation?",
    "T5": "How could information theory be used to optimize the design of educational curricula?",
    "Tpat": "Identify three non-obvious structural parallels between the theory of plate tectonics and the development of a national economy. For each parallel, explain the formal correspondence that makes them structurally equivalent, not merely analogous.",
    "Thier": "Take the concept of 'democracy' and recursively decompose it into its fundamental components at five levels — from the concrete institutional level down to the most abstract philosophical primitive. At each level, show how the components compose to produce the level above.",
    "Tabs": "Restate the following at five levels of abstraction: 'A forest fire spreads because dry wood ignites when heated above its flash point, and the burning wood heats neighboring dry wood.' Preserve the core truth at every level, from physical chemistry to the most abstract principle.",
    "Tform": "Describe how a new scientific concept (e.g., 'gene') goes from being a vague intuition to a precise formal object. What are the stages of this meaning-formation process, and what drives the transition between them?",
}

# Context files (relative to REPO = experiments/demo/)
CONTEXT_FILES = {
    "UL-P1": "theory-primers/UL-P1_topological-harmonic.txt",
    "UL-P2": "theory-primers/UL-P2_recursive-categorical.txt",
    "UL-P3": "theory-primers/UL-P3_information-geometric.txt",
    "UL-P4": "theory-primers/UL-P4_thermodynamic-semiotic.txt",
    "REF": os.path.join(REPO_ROOT, "test-content.txt"),  # absolute
    "CT-G": "controls/CT-G_algebraic-number-theory.txt",
    "CT-P1": "controls/CT-P1_standard-topology.txt",
    "CT-P2": "controls/CT-P2_standard-category.txt",
    "CT-P3": "controls/CT-P3_standard-info-geometry.txt",
    "CT-P4": "controls/CT-P4_standard-stat-mech.txt",
    "NAV-P": "controls/NAV-P_naive-primer.txt",
}

# Trial matrix: condition → list of tasks
TRIAL_MATRIX = {
    "NL":    ["T1", "T2", "T5", "Tpat", "Thier", "Tabs", "Tform"],
    "REF":   ["T1", "T2", "T5", "Tpat", "Thier", "Tabs", "Tform"],
    "UL-P1": ["T1", "T2", "T5", "Tpat", "Thier", "Tabs", "Tform"],
    "UL-P2": ["T1", "T2", "T5", "Tpat", "Thier", "Tabs", "Tform"],
    "UL-P3": ["T1", "T2", "T5", "Tpat", "Thier", "Tabs", "Tform"],
    "UL-P4": ["T1", "T2", "T5", "Tpat", "Thier", "Tabs", "Tform"],
    "CT-G":  ["T1", "T2", "T5"],
    "CT-P1": ["T1", "Tpat"],
    "CT-P2": ["T1", "Thier"],
    "CT-P3": ["T1", "Tabs"],
    "CT-P4": ["T1", "Tform"],
    "NAV-P": ["T1", "T2", "T5"],
}

# Phase assignments for prioritized execution
PHASES = {
    "A": ["NL", "REF", "UL-P1", "UL-P2"],      # 28 trials
    "B": ["UL-P3", "UL-P4"],                      # 14 trials
    "C": ["CT-G", "CT-P1", "CT-P2", "CT-P3", "CT-P4", "NAV-P"],  # 14 trials
}

# ─────────────────────────────────────────────
# Helper functions
# ─────────────────────────────────────────────

def load_context(condition):
    """Load the context text for a given condition. Returns '' for NL."""
    if condition == "NL":
        return ""
    path = CONTEXT_FILES[condition]
    if not os.path.isabs(path):
        path = os.path.join(REPO, path)
    with open(path, "r") as f:
        return f.read().strip()


def build_prompt(condition, task_id):
    """Build the full prompt for a trial."""
    context = load_context(condition)
    task_text = TASKS[task_id]
    
    if condition == "NL":
        return task_text
    else:
        return f"""Consider the following:

---
{context}
---

Now respond to the following:

{task_text}"""


def sha256_of(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()[:16]


# ─────────────────────────────────────────────
# Main
# ─────────────────────────────────────────────

def main():
    # Create directories
    os.makedirs(os.path.join(BASE, "prompts"), exist_ok=True)
    os.makedirs(os.path.join(BASE, "results"), exist_ok=True)
    os.makedirs(os.path.join(BASE, ".github"), exist_ok=True)

    # Write copilot-instructions.md (minimal)
    with open(os.path.join(BASE, ".github", "copilot-instructions.md"), "w") as f:
        f.write("Respond thoroughly and thoughtfully to the user's message.\n")

    # Collect all trials
    all_trials = []
    for condition, tasks in TRIAL_MATRIX.items():
        for task_id in tasks:
            phase = None
            for p, conditions in PHASES.items():
                if condition in conditions:
                    phase = p
                    break
            all_trials.append({
                "condition": condition,
                "task": task_id,
                "phase": phase,
                "filename": f"{condition}-{task_id}.md",
            })

    # Verify count
    assert len(all_trials) == 56, f"Expected 56 trials, got {len(all_trials)}"

    # Generate prompt files
    manifest = []
    for trial in all_trials:
        prompt_text = build_prompt(trial["condition"], trial["task"])
        filepath = os.path.join(BASE, "prompts", trial["filename"])
        with open(filepath, "w") as f:
            f.write(prompt_text)
        manifest.append({
            **trial,
            "hash": sha256_of(prompt_text),
            "chars": len(prompt_text),
        })

    # Randomize trial order within each phase
    random.seed(42)  # Fixed seed for reproducibility
    phase_trials = {"A": [], "B": [], "C": []}
    for trial in all_trials:
        phase_trials[trial["phase"]].append(trial["filename"])
    
    for phase in phase_trials:
        random.shuffle(phase_trials[phase])

    # Write trial order
    with open(os.path.join(BASE, "trial-order.txt"), "w") as f:
        f.write(f"# Trial Execution Order\n")
        f.write(f"# Generated: {datetime.now().isoformat()}\n")
        f.write(f"# Seed: 42\n")
        f.write(f"# Total: {len(all_trials)} trials\n\n")
        
        order_num = 1
        for phase in ["A", "B", "C"]:
            f.write(f"## Phase {phase} ({len(phase_trials[phase])} trials)\n\n")
            for filename in phase_trials[phase]:
                f.write(f"{order_num:3d}. {filename}\n")
                order_num += 1
            f.write("\n")

    # Write manifest
    with open(os.path.join(BASE, "manifest.md"), "w") as f:
        f.write("# Trial Manifest\n\n")
        f.write(f"Generated: {datetime.now().isoformat()}\n\n")
        f.write(f"| # | File | Phase | Condition | Task | Chars | SHA-256 (first 16) |\n")
        f.write(f"|---|------|-------|-----------|------|-------|--------------------|\n")
        for i, m in enumerate(manifest, 1):
            f.write(f"| {i} | {m['filename']} | {m['phase']} | {m['condition']} | {m['task']} | {m['chars']} | {m['hash']} |\n")
        f.write(f"\n**Total: {len(manifest)} trials**\n")

    # Write a README
    with open(os.path.join(BASE, "README.md"), "w") as f:
        f.write("""# UL Theory-Derived Primer Experiment v2

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
""")

    print(f"✓ Clean workspace created at: {BASE}")
    print(f"  {len(manifest)} prompt files in prompts/")
    print(f"  Trial order in trial-order.txt")
    print(f"  Manifest with hashes in manifest.md")
    print(f"\nPhase breakdown:")
    for phase in ["A", "B", "C"]:
        print(f"  Phase {phase}: {len(phase_trials[phase])} trials")


if __name__ == "__main__":
    main()
