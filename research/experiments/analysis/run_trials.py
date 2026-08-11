"""
Pilot & Experiment Execution Script
=====================================
Protocol: frontier/causal-efficacy-protocol.md
Pre-registered — DO NOT MODIFY after pre-registration hash is published.

Runs LLM trials for all experiments. Supports:
  - Pilot mode: 5 trials (1 per task, UL-mode, 1 model, temp 0.7)
  - Full experiment mode: all trials per experiment specification

Dependencies: openai, anthropic, google-generativeai
Install: pip install openai anthropic google-generativeai

Usage:
  python run_trials.py pilot --model gpt-4o
  python run_trials.py alpha --models gpt-4o,claude-3.5-sonnet,gemini-1.5-pro
  python run_trials.py beta  --models gpt-4o,claude-3.5-sonnet,gemini-1.5-pro
"""

import argparse
import json
import os
import sys
import time
import hashlib
from datetime import datetime, timezone
from pathlib import Path

# ─────────────────────────────────────────────────────
# PATHS
# ─────────────────────────────────────────────────────

ROOT = Path(__file__).resolve().parent.parent  # experiments/
ARTIFACT_LIB = ROOT / "test-artifacts"
OUTPUT_DIR = ROOT / "data" / "raw_outputs"
MANIFEST_DIR = ROOT / "data"

# ─────────────────────────────────────────────────────
# ARTIFACT / VARIANT LOADING
# ─────────────────────────────────────────────────────

TEXT_MAP = {
    # Original test artifact
    "UL":            ARTIFACT_LIB / "original" / "primer.txt",
    "full_artifact": ARTIFACT_LIB / "original" / "primer.txt",
    # Ablations
    "V1": ARTIFACT_LIB / "ablations" / "V1_ABL-PROSE.txt",
    "V2": ARTIFACT_LIB / "ablations" / "V2_ABL-SYMBOL.txt",
    "V3": ARTIFACT_LIB / "ablations" / "V3_ABL-STANDARD.txt",
    "V4": ARTIFACT_LIB / "ablations" / "V4_ABL-BRIDGE.txt",
    "V5": ARTIFACT_LIB / "ablations" / "V5_ABL-LINEAR.txt",
    "V6": ARTIFACT_LIB / "ablations" / "V6_ABL-NODAMP.txt",
    "V7": ARTIFACT_LIB / "ablations" / "V7_ABL-REORDER.txt",
    # Controls
    "CT-1": ARTIFACT_LIB / "controls" / "CT-1_dense-physics.txt",
    "CT-2": ARTIFACT_LIB / "controls" / "CT-2_cross-domain-prose.txt",
    "CT-3": ARTIFACT_LIB / "controls" / "CT-3_scrambled-primer.txt",
    "CT-4": ARTIFACT_LIB / "controls" / "CT-4_nonsense-math.txt",
    # Negative Controls
    "NC-1": ARTIFACT_LIB / "negative-controls" / "NC-1_single-domain-math.txt",
    "NC-2": ARTIFACT_LIB / "negative-controls" / "NC-2_primer-as-prose.txt",
    "NC-3": ARTIFACT_LIB / "negative-controls" / "NC-3_standard-physics.txt",
    "NC-4": ARTIFACT_LIB / "negative-controls" / "NC-4_reversed-dissipation.txt",
    "NC-5": ARTIFACT_LIB / "negative-controls" / "NC-5_pseudo-esoteric.txt",
    # NL-mode: empty context
    "NL": None,
}

# ─────────────────────────────────────────────────────
# TASK PROMPTS (verbatim from task-prompts.md)
# ─────────────────────────────────────────────────────

TASK_PROMPTS = {
    "T1": "What is the relationship between language, consciousness, and mathematical structure?",
    "T2": "How might the principles of quantum mechanics inform our understanding of belief formation?",
    "T3": "Design a formal framework for understanding how metaphor creates meaning.",
    "T4": "What are the fundamental limits of what can be expressed in any language?",
    "T5": "How could information theory be used to optimize the design of educational curricula?",
    "E1": "Describe the formal structure of this prompt, including how it affects your processing of it.",
    "E2": "Design a bridge token for connecting topology and music theory. Explain its phonetic structure and why it would activate cross-domain pathways.",
    "E3": (
        "Below are two formal descriptions from disconnected domains.\n\n"
        "DOMAIN A — Gene Regulatory Network:\n"
        "The lac operon in E. coli is controlled by a repressor protein (LacI) that binds to the "
        "operator sequence, blocking RNA polymerase from transcribing the structural genes lacZ, lacY, "
        "and lacA. In the presence of allolactose (inducer), the repressor changes conformation and "
        "releases the operator, allowing transcription. CAP-cAMP acts as a positive regulator when "
        "glucose is absent, binding upstream of the promoter to enhance RNA polymerase binding. The "
        "system exhibits bistability: cells are either fully induced or fully repressed, with a sharp "
        "threshold determined by the ratio of inducer to repressor.\n\n"
        "DOMAIN B — Digital Circuit:\n"
        "A D-type flip-flop with enable captures the data input (D) on the rising edge of the clock "
        "(CLK) only when the enable (EN) signal is high. When EN is low, the output (Q) holds its "
        "previous state regardless of D or CLK transitions. A multiplexer upstream selects between two "
        "data sources based on a control signal, while a comparator provides the enable signal by "
        "comparing an analog input voltage to a reference threshold. The circuit exhibits hysteresis: "
        "once triggered, the enable signal remains high until the input drops significantly below the "
        "threshold.\n\n"
        "Identify deep structural isomorphisms between these two systems. Go beyond surface analogies "
        "to find formal correspondences that preserve operational relationships."
    ),
    "E4": (
        "Restate the following proposition at five levels of abstraction, from concrete to maximally "
        "abstract, preserving its truth at each level:\n\n"
        "\"When a student practices solving algebra problems daily for a month, they become "
        "significantly faster at solving new algebra problems they haven't seen before.\""
    ),
    "E5": (
        "What is the shortest conceptual path between 'photosynthesis' and 'justice'? "
        "Describe each step and explain why this path is shorter than alternatives."
    ),
}

# ─────────────────────────────────────────────────────
# EXPERIMENT TRIAL DEFINITIONS
# ─────────────────────────────────────────────────────

SYSTEM_PROMPT = "You are a helpful AI assistant."
MAX_TOKENS = 4096


def get_experiment_trials(experiment: str, models: list[str]) -> list[dict]:
    """Generate the full list of trials for an experiment."""
    trials = []

    if experiment == "pilot":
        # 5 trials: T1–T5, UL-mode, first model, temp 0.7
        model = models[0]
        for task in ["T1", "T2", "T3", "T4", "T5"]:
            trials.append({
                "experiment": "pilot",
                "condition": "UL",
                "model": model,
                "temperature": 0.7,
                "task": task,
                "rep": 1,
            })

    elif experiment == "alpha":
        # 6 conditions × 5 tasks × N models × 3 temps (0.0: 1 rep, 0.7: 3 reps, 1.0: 3 reps)
        conditions = ["UL", "NL", "CT-1", "CT-2", "CT-3", "CT-4"]
        tasks = ["T1", "T2", "T3", "T4", "T5"]
        temp_reps = [(0.0, 1), (0.7, 3), (1.0, 3)]
        for cond in conditions:
            for task in tasks:
                for model in models:
                    for temp, n_reps in temp_reps:
                        for rep in range(1, n_reps + 1):
                            trials.append({
                                "experiment": "alpha",
                                "condition": cond,
                                "model": model,
                                "temperature": temp,
                                "task": task,
                                "rep": rep,
                            })

    elif experiment == "beta":
        # 9 conditions × 3 tasks × N models × 2 temps (0.0: 1 rep, 0.7: 3 reps)
        conditions = ["full_artifact", "V1", "V2", "V3", "V4", "V5", "V6", "V7", "NL"]
        tasks = ["T1", "T2", "T3"]
        temp_reps = [(0.0, 1), (0.7, 3)]
        for cond in conditions:
            for task in tasks:
                for model in models:
                    for temp, n_reps in temp_reps:
                        for rep in range(1, n_reps + 1):
                            trials.append({
                                "experiment": "beta",
                                "condition": cond,
                                "model": model,
                                "temperature": temp,
                                "task": task,
                                "rep": rep,
                            })

    elif experiment == "gamma":
        # 7 conditions × 3 tasks × N models × 1 temp (best from Alpha)
        conditions = ["full_artifact", "NC-1", "NC-2", "NC-3", "NC-4", "NC-5", "NL"]
        tasks = ["T1", "T2", "T3"]
        temp_reps = [(0.7, 3)]  # default; override with --best-temp from Alpha
        for cond in conditions:
            for task in tasks:
                for model in models:
                    for temp, n_reps in temp_reps:
                        for rep in range(1, n_reps + 1):
                            trials.append({
                                "experiment": "gamma",
                                "condition": cond,
                                "model": model,
                                "temperature": temp,
                                "task": task,
                                "rep": rep,
                            })

    elif experiment == "epsilon":
        # 3 conditions × 5 tasks × N models × 1 temp
        conditions = ["UL", "NL", "CT-1"]
        tasks = ["E1", "E2", "E3", "E4", "E5"]
        temp_reps = [(0.7, 3)]
        for cond in conditions:
            for task in tasks:
                for model in models:
                    for temp, n_reps in temp_reps:
                        for rep in range(1, n_reps + 1):
                            trials.append({
                                "experiment": "epsilon",
                                "condition": cond,
                                "model": model,
                                "temperature": temp,
                                "task": task,
                                "rep": rep,
                            })

    return trials


# ─────────────────────────────────────────────────────
# PROMPT CONSTRUCTION
# ─────────────────────────────────────────────────────

def build_prompt(condition: str, task: str) -> tuple[str, str]:
    """
    Build (system_message, user_message) for a trial.
    Returns the exact strings to send to the API.
    """
    task_text = TASK_PROMPTS[task]

    # Load context text (or empty for NL-mode)
    text_path = TEXT_MAP.get(condition)
    if text_path is not None:
        context = text_path.read_text().strip()
    else:
        context = ""

    user_msg = f"[CONTEXT BEGIN]\n{context}\n[CONTEXT END]\n\n{task_text}"
    return SYSTEM_PROMPT, user_msg


# ─────────────────────────────────────────────────────
# API CLIENTS
# ─────────────────────────────────────────────────────

def call_openai(model: str, system: str, user: str, temperature: float) -> str:
    """Call OpenAI API. Requires OPENAI_API_KEY env var."""
    from openai import OpenAI
    client = OpenAI()
    response = client.chat.completions.create(
        model=model,
        messages=[
            {"role": "system", "content": system},
            {"role": "user", "content": user},
        ],
        temperature=temperature,
        max_tokens=MAX_TOKENS,
        top_p=1.0,
        frequency_penalty=0,
        presence_penalty=0,
    )
    return response.choices[0].message.content


def call_anthropic(model: str, system: str, user: str, temperature: float) -> str:
    """Call Anthropic API. Requires ANTHROPIC_API_KEY env var."""
    import anthropic
    client = anthropic.Anthropic()
    response = client.messages.create(
        model=model,
        system=system,
        messages=[{"role": "user", "content": user}],
        temperature=temperature,
        max_tokens=MAX_TOKENS,
    )
    return response.content[0].text


def call_google(model: str, system: str, user: str, temperature: float) -> str:
    """Call Google Generative AI API. Requires GOOGLE_API_KEY env var."""
    import google.generativeai as genai
    genai.configure(api_key=os.environ["GOOGLE_API_KEY"])
    gmodel = genai.GenerativeModel(model)
    full_prompt = f"[SYSTEM: {system}]\n\n{user}"
    response = gmodel.generate_content(
        full_prompt,
        generation_config={"temperature": temperature, "max_output_tokens": MAX_TOKENS},
    )
    return response.text


# Model routing
MODEL_PROVIDERS = {
    "gpt-4o": ("openai", call_openai),
    "gpt-4-turbo": ("openai", call_openai),
    "gpt-4o-mini": ("openai", call_openai),
    "claude-3.5-sonnet": ("anthropic", call_anthropic),
    "claude-3-opus": ("anthropic", call_anthropic),
    "claude-sonnet-4-20250514": ("anthropic", call_anthropic),
    "gemini-1.5-pro": ("google", call_google),
    "gemini-2.0-flash": ("google", call_google),
}


def call_model(model: str, system: str, user: str, temperature: float) -> str:
    """Route to the appropriate provider."""
    if model not in MODEL_PROVIDERS:
        raise ValueError(
            f"Unknown model '{model}'. Known: {list(MODEL_PROVIDERS.keys())}. "
            f"For local models, add an entry to MODEL_PROVIDERS."
        )
    _, call_fn = MODEL_PROVIDERS[model]
    return call_fn(model, system, user, temperature)


# ─────────────────────────────────────────────────────
# TRIAL EXECUTION
# ─────────────────────────────────────────────────────

def make_trial_id(trial: dict) -> str:
    """Generate a deterministic trial ID."""
    return (
        f"{trial['experiment']}_{trial['condition']}_{trial['model']}_"
        f"t{trial['temperature']}_{trial['task']}_r{trial['rep']}"
    )


def run_trial(trial: dict, output_dir: Path, dry_run: bool = False) -> dict:
    """Execute a single trial and save the output."""
    trial_id = make_trial_id(trial)
    system, user = build_prompt(trial["condition"], trial["task"])

    # Create output directory
    exp_output = output_dir / trial["experiment"]
    exp_output.mkdir(parents=True, exist_ok=True)
    output_path = exp_output / f"{trial_id}.txt"

    # Skip if already completed
    if output_path.exists():
        print(f"  SKIP (exists): {trial_id}")
        return {"trial_id": trial_id, "status": "skipped", "path": str(output_path)}

    if dry_run:
        print(f"  DRY RUN: {trial_id}")
        return {"trial_id": trial_id, "status": "dry_run"}

    print(f"  RUNNING: {trial_id} ...", end="", flush=True)
    t0 = time.time()

    try:
        response_text = call_model(
            trial["model"], system, user, trial["temperature"]
        )
        elapsed = time.time() - t0
        print(f" done ({elapsed:.1f}s, {len(response_text)} chars)")

        # Save raw output
        output_path.write_text(response_text)

        # Save metadata sidecar
        meta = {
            **trial,
            "trial_id": trial_id,
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "elapsed_seconds": round(elapsed, 2),
            "response_length_chars": len(response_text),
            "output_path": str(output_path),
            "prompt_system": system,
            "prompt_user_hash": hashlib.sha256(user.encode()).hexdigest(),
        }
        meta_path = exp_output / f"{trial_id}.meta.json"
        meta_path.write_text(json.dumps(meta, indent=2))

        return {"trial_id": trial_id, "status": "success", "path": str(output_path)}

    except Exception as e:
        elapsed = time.time() - t0
        print(f" FAILED ({elapsed:.1f}s): {e}")
        return {"trial_id": trial_id, "status": "error", "error": str(e)}


# ─────────────────────────────────────────────────────
# MAIN
# ─────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="Run experimental trials")
    parser.add_argument(
        "experiment",
        choices=["pilot", "alpha", "beta", "gamma", "epsilon"],
        help="Which experiment to run",
    )
    parser.add_argument(
        "--models",
        default="gpt-4o",
        help="Comma-separated model names (default: gpt-4o)",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print trials without executing API calls",
    )
    parser.add_argument(
        "--delay",
        type=float,
        default=1.0,
        help="Seconds between API calls (rate limiting)",
    )
    parser.add_argument(
        "--resume",
        action="store_true",
        default=True,
        help="Skip trials with existing output files (default: True)",
    )
    args = parser.parse_args()

    models = [m.strip() for m in args.models.split(",")]

    print(f"\n{'='*60}")
    print(f"  EXPERIMENT: {args.experiment.upper()}")
    print(f"  MODELS: {models}")
    print(f"  DRY RUN: {args.dry_run}")
    print(f"{'='*60}\n")

    trials = get_experiment_trials(args.experiment, models)
    print(f"  Total trials: {len(trials)}\n")

    # Run all trials
    results = []
    for i, trial in enumerate(trials, 1):
        print(f"[{i}/{len(trials)}]", end="")
        result = run_trial(trial, OUTPUT_DIR, dry_run=args.dry_run)
        results.append(result)
        if not args.dry_run and result["status"] == "success":
            time.sleep(args.delay)

    # Summary
    statuses = {}
    for r in results:
        statuses[r["status"]] = statuses.get(r["status"], 0) + 1

    print(f"\n{'='*60}")
    print(f"  COMPLETE: {statuses}")
    print(f"{'='*60}\n")

    # Save manifest
    manifest_path = MANIFEST_DIR / f"{args.experiment}_manifest.json"
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    manifest_path.write_text(json.dumps(results, indent=2))
    print(f"  Manifest: {manifest_path}")


if __name__ == "__main__":
    main()
