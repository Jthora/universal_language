# Local Model Verification Protocol (Ollama / llama.cpp / LM Studio)

**Interface:** Local model inference via Ollama, llama.cpp, LM Studio, or similar  
**Model:** Any model that fits on your hardware (Llama 3, Mistral, Phi, Gemma, Qwen, etc.)  
**Cost:** Free (after download)  
**Automation level:** Fully scriptable  
**Estimated time:** Setup: 15–30 min. Trials: depends on hardware and model size.

---

## Overview

Running models locally gives you full control over parameters, unlimited trials at zero marginal cost, and complete privacy. It's the best approach for generating large datasets and the most scientifically rigorous option outside the API script — because you control every variable.

If the artifact effect appears in a 7B-parameter model running on a laptop, that arguably provides STRONGER evidence than showing it in GPT-4o — it means the effect doesn't require frontier-scale models.

---

## Prerequisites

- Computer with 8+ GB RAM (for 7B models) or 16+ GB RAM (for 13B+ models)
- ~5–15 GB disk space per model
- Ollama, LM Studio, or llama.cpp installed

---

## Setup: Ollama (Recommended — Simplest)

### Install Ollama

```bash
# macOS
brew install ollama

# Linux
curl -fsSL https://ollama.com/install.sh | sh

# Windows: download from https://ollama.com
```

### Pull a Model

```bash
# Good starting models (ranked by size):
ollama pull phi3:mini          # ~2.3 GB, very fast, good for quick tests
ollama pull llama3.1:8b        # ~4.7 GB, strong general performance
ollama pull mistral:7b         # ~4.1 GB, good instruction following  
ollama pull gemma2:9b          # ~5.4 GB, Google's open model
ollama pull llama3.1:70b       # ~40 GB, frontier-quality (needs 64+ GB RAM)
```

### Verify It Works

```bash
ollama run llama3.1:8b "What is 2+2?"
```

---

## Running the Experiment

### Option A: Manual via Ollama CLI

For each trial, run:

```bash
# UL-mode trial (artifact + task)
cat experiments/verification-guides/ready-prompts/UL-T1.txt | ollama run llama3.1:8b > results/UL_T1_r1.txt

# NL-mode trial (no context + task)
cat experiments/verification-guides/ready-prompts/NL-T1.txt | ollama run llama3.1:8b > results/NL_T1_r1.txt

# CT-mode trial (control text + task)
cat experiments/verification-guides/ready-prompts/CT-T1.txt | ollama run llama3.1:8b > results/CT_T1_r1.txt
```

### Option B: Script via Ollama API

Ollama exposes a local API at `http://localhost:11434`. The script below runs a full quick test:

```python
#!/usr/bin/env python3
"""
Local model experiment runner using Ollama API.
No API keys needed — runs entirely on your machine.

Usage:
    ollama serve  # Start Ollama server (if not already running)
    python run_local_trials.py --model llama3.1:8b --tasks T1,T2,T3
"""

import requests
import json
import os
import time
from pathlib import Path

OLLAMA_URL = "http://localhost:11434/api/chat"

# Paths (adjust if running from a different location)
SCRIPT_DIR = Path(__file__).parent
PROMPTS_DIR = SCRIPT_DIR / "ready-prompts"
RESULTS_DIR = SCRIPT_DIR / "results"


def load_prompt(filepath: Path) -> str:
    """Load a prompt file and return the text."""
    return filepath.read_text().strip()


def call_ollama(model: str, prompt: str, system: str = "You are a helpful AI assistant.") -> str:
    """Send a chat message to Ollama and return the response."""
    payload = {
        "model": model,
        "messages": [
            {"role": "system", "content": system},
            {"role": "user", "content": prompt},
        ],
        "stream": False,
        "options": {
            "temperature": 0.7,
            "num_predict": 4096,
        }
    }
    response = requests.post(OLLAMA_URL, json=payload)
    response.raise_for_status()
    return response.json()["message"]["content"]


def run_experiment(model: str, tasks: list[str], reps: int = 1):
    """Run all conditions for specified tasks."""
    conditions = ["UL", "NL", "CT"]
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    trials = []
    for task in tasks:
        for cond in conditions:
            for rep in range(1, reps + 1):
                trials.append((cond, task, rep))

    # Shuffle to randomize order
    import random
    random.shuffle(trials)

    for i, (cond, task, rep) in enumerate(trials, 1):
        trial_id = f"{cond}_{task}_r{rep}"
        output_path = RESULTS_DIR / f"{trial_id}.txt"

        if output_path.exists():
            print(f"  [{i}/{len(trials)}] SKIP (exists): {trial_id}")
            continue

        prompt_file = PROMPTS_DIR / f"{cond}-{task}.txt"
        if not prompt_file.exists():
            print(f"  [{i}/{len(trials)}] MISSING PROMPT: {prompt_file}")
            continue

        prompt = load_prompt(prompt_file)
        print(f"  [{i}/{len(trials)}] RUNNING: {trial_id} ...", end="", flush=True)

        t0 = time.time()
        try:
            response = call_ollama(model, prompt)
            elapsed = time.time() - t0
            output_path.write_text(response)
            print(f" done ({elapsed:.1f}s, {len(response)} chars)")
        except Exception as e:
            elapsed = time.time() - t0
            print(f" FAILED ({elapsed:.1f}s): {e}")

        time.sleep(0.5)  # Brief pause between trials

    print(f"\nResults saved to: {RESULTS_DIR}/")


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--model", default="llama3.1:8b")
    parser.add_argument("--tasks", default="T1,T2,T3")
    parser.add_argument("--reps", type=int, default=1)
    args = parser.parse_args()

    tasks = [t.strip() for t in args.tasks.split(",")]
    print(f"\nModel: {args.model}")
    print(f"Tasks: {tasks}")
    print(f"Reps:  {args.reps}")
    print(f"Total trials: {len(tasks) * 3 * args.reps}\n")

    run_experiment(args.model, tasks, args.reps)
```

### Option C: LM Studio

LM Studio provides a GUI for downloading and running models, plus an OpenAI-compatible API at `localhost:1234`. The script above can be adapted by changing the URL and payload format.

---

## Multi-Model Sweep

One of the unique advantages of local models: you can test the artifact effect across multiple architectures for free.

```bash
for model in phi3:mini llama3.1:8b mistral:7b gemma2:9b; do
    echo "=== $model ==="
    python run_local_trials.py --model $model --tasks T1,T2,T3
done
```

If the artifact effect appears consistently across Llama, Mistral, Phi, and Gemma, that's strong evidence it's not model-specific but reflects something about the test artifact's structure.

---

## Advantages of This Approach

1. **Unlimited trials at zero cost** — run 1000 reps if you want
2. **Perfect reproducibility** — same model weights, same temperature, same everything
3. **No rate limits or daily caps** — run the full protocol in one session
4. **Smaller models = harder test** — if a 3B model shows the effect, it's not an artifact of scale
5. **Multiple architectures** — test across model families in one afternoon
6. **Complete privacy** — no data leaves your machine

## Limitations

1. **Models are smaller than frontier** — the effect might require model scale (that's an interesting finding if so)
2. **Instruction following varies** — smaller models may produce less structured outputs
3. **Hardware requirements** — 7B models need ~8 GB RAM, 70B models need ~64 GB RAM
