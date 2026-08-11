"""
Blinding & Randomization Tool
===============================
Protocol: frontier/causal-efficacy-protocol.md, scoring/rubric-M1-M5.md

Generates blind IDs for trial outputs so evaluators score without
knowing which condition produced which output.

Usage:
  python blind.py create <experiment>   — Generate blind key + shuffled scoring sheets
  python blind.py reveal <experiment>   — After scoring, merge blind scores with conditions
"""

import argparse
import csv
import hashlib
import json
import random
import string
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA_DIR = ROOT / "data"
OUTPUT_DIR = DATA_DIR / "raw_outputs"
BLIND_DIR = DATA_DIR / "blinding"
SCORING_DIR = DATA_DIR / "scoring_sheets"


def generate_blind_id(length: int = 8) -> str:
    """Generate a random alphanumeric ID for blinding."""
    return "".join(random.choices(string.ascii_uppercase + string.digits, k=length))


def create_blind_key(experiment: str, seed: int = None):
    """
    Scan raw outputs for an experiment, assign blind IDs,
    save the key (HIDDEN from evaluators) and scoring sheets.
    """
    exp_dir = OUTPUT_DIR / experiment
    if not exp_dir.exists():
        sys.exit(f"ERROR: No outputs found at {exp_dir}. Run trials first.")

    # Collect all trial output files
    output_files = sorted(exp_dir.glob("*.txt"))
    meta_files = {f.stem.replace(".meta", ""): f for f in exp_dir.glob("*.meta.json")}

    if not output_files:
        sys.exit(f"ERROR: No .txt output files in {exp_dir}")

    # Set random seed for reproducibility (but keep secret)
    if seed is None:
        seed = int.from_bytes(hashlib.sha256(
            f"{experiment}_{datetime.now(timezone.utc).isoformat()}".encode()
        ).digest()[:4], "big")
    random.seed(seed)

    # Build mapping: blind_id -> trial_id + metadata
    trials = []
    for f in output_files:
        trial_id = f.stem
        meta_path = meta_files.get(trial_id)
        meta = json.loads(meta_path.read_text()) if meta_path and meta_path.exists() else {}
        trials.append({
            "trial_id": trial_id,
            "blind_id": generate_blind_id(),
            "output_file": str(f),
            "condition": meta.get("condition", "UNKNOWN"),
            "model": meta.get("model", "UNKNOWN"),
            "temperature": meta.get("temperature", "UNKNOWN"),
            "task": meta.get("task", "UNKNOWN"),
        })

    # Shuffle for presentation order
    random.shuffle(trials)

    # Save blind key (CONFIDENTIAL — do NOT share with evaluators)
    BLIND_DIR.mkdir(parents=True, exist_ok=True)
    key_path = BLIND_DIR / f"{experiment}_blind_key.json"
    key_data = {
        "experiment": experiment,
        "generated": datetime.now(timezone.utc).isoformat(),
        "seed_hash": hashlib.sha256(str(seed).encode()).hexdigest(),  # hash, not raw seed
        "n_trials": len(trials),
        "mapping": [
            {"blind_id": t["blind_id"], "trial_id": t["trial_id"],
             "condition": t["condition"], "model": t["model"],
             "temperature": t["temperature"], "task": t["task"]}
            for t in trials
        ],
    }
    key_path.write_text(json.dumps(key_data, indent=2))
    print(f"  Blind key saved: {key_path}")
    print(f"  *** KEEP THIS FILE HIDDEN FROM EVALUATORS ***\n")

    # Generate scoring sheet (what evaluators see)
    SCORING_DIR.mkdir(parents=True, exist_ok=True)
    sheet_path = SCORING_DIR / f"{experiment}_scoring_sheet.csv"
    with open(sheet_path, "w", newline="") as f:
        writer = csv.writer(f)
        header = ["blind_id", "task", "M1", "M2", "M3", "M4", "M5"]
        if experiment == "epsilon":
            header.append("SQS")
        header.append("notes")
        writer.writerow(header)
        for t in trials:
            row = [t["blind_id"], t["task"]] + [""] * (len(header) - 2)
            writer.writerow(row)
    print(f"  Scoring sheet: {sheet_path}")
    print(f"  (Give this to evaluators along with the output texts renamed by blind_id)\n")

    # Copy outputs with blind filenames
    blind_output_dir = SCORING_DIR / f"{experiment}_blind_outputs"
    blind_output_dir.mkdir(parents=True, exist_ok=True)
    for t in trials:
        src = Path(t["output_file"])
        dst = blind_output_dir / f"{t['blind_id']}.txt"
        if src.exists():
            dst.write_text(src.read_text())
    print(f"  Blind outputs: {blind_output_dir}/")
    print(f"  Total: {len(trials)} trials blinded and shuffled.\n")


def reveal_and_merge(experiment: str):
    """
    After scoring is complete, merge blind scores with condition labels.
    Produces the final scored CSV ready for analysis.py.
    """
    key_path = BLIND_DIR / f"{experiment}_blind_key.json"
    if not key_path.exists():
        sys.exit(f"ERROR: No blind key found at {key_path}. Run 'create' first.")

    key_data = json.loads(key_path.read_text())
    blind_map = {m["blind_id"]: m for m in key_data["mapping"]}

    # Find all scoring sheets (may have multiple evaluators)
    score_files = sorted(SCORING_DIR.glob(f"{experiment}_scores_eval_*.csv"))
    if not score_files:
        # Try single sheet
        single = SCORING_DIR / f"{experiment}_scoring_sheet_completed.csv"
        if single.exists():
            score_files = [single]
        else:
            sys.exit(
                f"ERROR: No completed scoring files found.\n"
                f"  Expected: {SCORING_DIR}/{experiment}_scores_eval_*.csv\n"
                f"  Or: {SCORING_DIR}/{experiment}_scoring_sheet_completed.csv"
            )

    # Merge
    merged_rows = []
    for sf in score_files:
        evaluator = sf.stem.split("_eval_")[-1] if "_eval_" in sf.stem else "eval_A"
        with open(sf) as f:
            reader = csv.DictReader(f)
            for row in reader:
                blind_id = row["blind_id"]
                if blind_id not in blind_map:
                    print(f"  WARNING: Unknown blind_id '{blind_id}', skipping.")
                    continue
                meta = blind_map[blind_id]
                merged_rows.append({
                    "trial_id": meta["trial_id"],
                    "condition": meta["condition"],
                    "model": meta["model"],
                    "temperature": meta["temperature"],
                    "task": meta["task"],
                    "evaluator": evaluator,
                    "M1": row.get("M1", ""),
                    "M2": row.get("M2", ""),
                    "M3": row.get("M3", ""),
                    "M4": row.get("M4", ""),
                    "M5": row.get("M5", ""),
                    "SQS": row.get("SQS", ""),
                    "notes": row.get("notes", ""),
                })

    # Write final scored CSV (input to analysis.py)
    out_path = DATA_DIR / f"{experiment}_scores.csv"
    fields = ["trial_id", "condition", "model", "temperature", "task",
              "evaluator", "M1", "M2", "M3", "M4", "M5", "SQS", "notes"]
    with open(out_path, "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        writer.writerows(merged_rows)

    print(f"\n  Merged scores: {out_path}")
    print(f"  Total rows: {len(merged_rows)} ({len(score_files)} evaluator(s))")
    print(f"  Conditions revealed. Ready for analysis.py.\n")


def main():
    parser = argparse.ArgumentParser(description="Blinding & randomization tool")
    parser.add_argument("action", choices=["create", "reveal"])
    parser.add_argument("experiment", help="Experiment name (pilot, alpha, beta, ...)")
    parser.add_argument("--seed", type=int, default=None, help="Random seed (for reproducibility)")
    args = parser.parse_args()

    if args.action == "create":
        create_blind_key(args.experiment, seed=args.seed)
    elif args.action == "reveal":
        reveal_and_merge(args.experiment)


if __name__ == "__main__":
    main()
