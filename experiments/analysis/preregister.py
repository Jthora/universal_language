"""
Pre-Registration Hash Generator
=================================
Protocol: frontier/causal-efficacy-protocol.md §8.3

Generates a SHA-256 hash of ALL pre-registered materials:
  - Protocol document
  - All 16 primer variant texts + original
  - Scoring rubrics
  - Domain lists and known-connections
  - Prompt templates and task prompts
  - Analysis scripts
  - Data schema

This hash is published BEFORE any data collection begins.
Any modification to hashed materials after publication voids the pre-registration.

Usage:
  python preregister.py              — Generate hash and save receipt
  python preregister.py --verify     — Verify current files match stored hash
"""

import argparse
import hashlib
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent  # experiments/
PROJECT_ROOT = ROOT.parent  # linguistic_experiment/

# ─────────────────────────────────────────────────────
# FILES TO HASH (pre-registered materials)
# ─────────────────────────────────────────────────────

REGISTERED_FILES = [
    # Protocol
    PROJECT_ROOT / "frontier" / "causal-efficacy-protocol.md",

    # Original primer
    ROOT / "primer-library" / "original" / "primer.txt",

    # Ablation variants
    ROOT / "primer-library" / "ablations" / "V1_ABL-PROSE.txt",
    ROOT / "primer-library" / "ablations" / "V2_ABL-SYMBOL.txt",
    ROOT / "primer-library" / "ablations" / "V3_ABL-STANDARD.txt",
    ROOT / "primer-library" / "ablations" / "V4_ABL-BRIDGE.txt",
    ROOT / "primer-library" / "ablations" / "V5_ABL-LINEAR.txt",
    ROOT / "primer-library" / "ablations" / "V6_ABL-NODAMP.txt",
    ROOT / "primer-library" / "ablations" / "V7_ABL-REORDER.txt",

    # Control texts
    ROOT / "primer-library" / "controls" / "CT-1_dense-physics.txt",
    ROOT / "primer-library" / "controls" / "CT-2_cross-domain-prose.txt",
    ROOT / "primer-library" / "controls" / "CT-3_scrambled-primer.txt",
    ROOT / "primer-library" / "controls" / "CT-4_nonsense-math.txt",

    # Negative controls
    ROOT / "primer-library" / "negative-controls" / "NC-1_single-domain-math.txt",
    ROOT / "primer-library" / "negative-controls" / "NC-2_primer-as-prose.txt",
    ROOT / "primer-library" / "negative-controls" / "NC-3_standard-physics.txt",
    ROOT / "primer-library" / "negative-controls" / "NC-4_reversed-dissipation.txt",
    ROOT / "primer-library" / "negative-controls" / "NC-5_pseudo-esoteric.txt",

    # Scoring infrastructure
    ROOT / "scoring" / "rubric-M1-M5.md",
    ROOT / "scoring" / "rubric-SQS.md",
    ROOT / "scoring" / "domain-lists" / "T1-domains.md",
    ROOT / "scoring" / "domain-lists" / "T2-domains.md",
    ROOT / "scoring" / "domain-lists" / "T3-domains.md",
    ROOT / "scoring" / "domain-lists" / "T4-domains.md",
    ROOT / "scoring" / "domain-lists" / "T5-domains.md",
    ROOT / "scoring" / "known-connections" / "T1-known.md",
    ROOT / "scoring" / "known-connections" / "T2-known.md",
    ROOT / "scoring" / "known-connections" / "T3-known.md",
    ROOT / "scoring" / "known-connections" / "T4-known.md",
    ROOT / "scoring" / "known-connections" / "T5-known.md",

    # Prompts
    ROOT / "prompts" / "templates.md",
    ROOT / "prompts" / "task-prompts.md",

    # Analysis scripts
    ROOT / "analysis" / "analysis.py",
    ROOT / "analysis" / "run_trials.py",
    ROOT / "analysis" / "blind.py",

    # Data schema
    ROOT / "data" / "SCHEMA.md",

    # QC Audit
    ROOT / "qc-audit-report.md",
]


def hash_file(path: Path) -> str:
    """SHA-256 hash of a single file."""
    h = hashlib.sha256()
    h.update(path.read_bytes())
    return h.hexdigest()


def compute_master_hash() -> tuple[str, list[dict]]:
    """
    Hash each registered file individually, then compute
    a master hash over all individual hashes (sorted by path).
    """
    file_hashes = []
    missing = []

    for fp in sorted(REGISTERED_FILES, key=lambda p: str(p)):
        if not fp.exists():
            missing.append(str(fp))
            continue
        h = hash_file(fp)
        rel = fp.relative_to(PROJECT_ROOT)
        file_hashes.append({"file": str(rel), "sha256": h, "size_bytes": fp.stat().st_size})

    if missing:
        print(f"\n  WARNING: {len(missing)} registered file(s) not found:")
        for m in missing:
            print(f"    - {m}")
        print()

    # Master hash: SHA-256 of concatenated individual hashes
    master = hashlib.sha256()
    for fh in file_hashes:
        master.update(fh["sha256"].encode())
    master_hex = master.hexdigest()

    return master_hex, file_hashes


def generate_receipt():
    """Generate and save the pre-registration receipt."""
    master_hash, file_hashes = compute_master_hash()

    receipt = {
        "pre_registration": {
            "project": "UL Causal Efficacy Protocol",
            "protocol": "frontier/causal-efficacy-protocol.md",
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "master_hash_sha256": master_hash,
            "n_files": len(file_hashes),
            "note": (
                "This hash was generated BEFORE any experimental data collection. "
                "Any modification to the hashed files after this timestamp voids "
                "the pre-registration. Verify with: python preregister.py --verify"
            ),
        },
        "file_hashes": file_hashes,
    }

    receipt_path = ROOT / "data" / "preregistration_receipt.json"
    receipt_path.parent.mkdir(parents=True, exist_ok=True)
    receipt_path.write_text(json.dumps(receipt, indent=2))

    print(f"\n{'='*60}")
    print(f"  PRE-REGISTRATION RECEIPT")
    print(f"{'='*60}")
    print(f"  Master Hash: {master_hash}")
    print(f"  Files:       {len(file_hashes)}")
    print(f"  Timestamp:   {receipt['pre_registration']['timestamp']}")
    print(f"  Receipt:     {receipt_path}")
    print(f"{'='*60}")
    print(f"\n  NEXT STEPS:")
    print(f"  1. Publish the master hash publicly (e.g., tweet, blog, OSF)")
    print(f"  2. Optionally commit receipt to a public git repo with a signed tag")
    print(f"  3. Do NOT modify any hashed files after publication")
    print(f"  4. Begin data collection (Experiment Alpha)\n")

    return master_hash


def verify_receipt():
    """Verify that current files match the stored pre-registration hash."""
    receipt_path = ROOT / "data" / "preregistration_receipt.json"
    if not receipt_path.exists():
        sys.exit("ERROR: No pre-registration receipt found. Run without --verify first.")

    receipt = json.loads(receipt_path.read_text())
    stored_hash = receipt["pre_registration"]["master_hash_sha256"]
    stored_files = {fh["file"]: fh["sha256"] for fh in receipt["file_hashes"]}

    current_hash, current_files = compute_master_hash()
    current_map = {fh["file"]: fh["sha256"] for fh in current_files}

    print(f"\n{'='*60}")
    print(f"  PRE-REGISTRATION VERIFICATION")
    print(f"{'='*60}")
    print(f"  Stored hash:  {stored_hash}")
    print(f"  Current hash: {current_hash}")

    if stored_hash == current_hash:
        print(f"\n  ✓ VERIFIED — all files match pre-registration.")
        print(f"  Pre-registration integrity INTACT.\n")
    else:
        print(f"\n  ✗ MISMATCH — files have been modified since pre-registration!\n")
        # Identify which files changed
        for f, h in stored_files.items():
            ch = current_map.get(f)
            if ch is None:
                print(f"    DELETED: {f}")
            elif ch != h:
                print(f"    CHANGED: {f}")
        for f in current_map:
            if f not in stored_files:
                print(f"    ADDED:   {f}")
        print(f"\n  Pre-registration integrity COMPROMISED.")
        print(f"  Any results must be reported as post-hoc exploratory.\n")
        sys.exit(1)


def main():
    parser = argparse.ArgumentParser(description="Pre-registration hash tool")
    parser.add_argument("--verify", action="store_true", help="Verify against stored hash")
    args = parser.parse_args()

    if args.verify:
        verify_receipt()
    else:
        generate_receipt()


if __name__ == "__main__":
    main()
