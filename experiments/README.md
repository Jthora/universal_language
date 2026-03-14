# Experiments — Universal Language Causal Efficacy Verification

> **This directory contains everything needed to independently verify that UL-structured text produces measurable cognitive effects in large language models.**

---

## Overview

The test artifact ([`test-artifacts/original/primer.txt`](test-artifacts/original/primer.txt)) is a 19-line, 490-token text that encodes the core structure of Universal Language in compressed form. When injected as context before reasoning tasks, LLMs produce output with measurably greater cross-domain depth, structural coherence, and generative novelty.

This experimental infrastructure lets you test that claim yourself.

---

## What's Here

```
experiments/
├── README.md                # You are here
├── qc-audit-report.md       # Feature audit: 16/16 variants pass QC
│
├── test-artifacts/          # 17 texts, all QC-verified
│   ├── original/            # The test artifact (test condition)
│   ├── ablations/           # 7 variants: each removes one structural feature
│   ├── controls/            # 4 controls: matched token count, no UL structure
│   └── negative-controls/   # 5 texts: actively anti-UL properties
│
├── scoring/                 # Evaluation infrastructure
│   ├── rubrics.md           # M1–M5 metrics + SQS composite (anchored 1–5 scales)
│   ├── domain-lists.md      # 30 knowledge domains for cross-domain scoring
│   ├── known-connections.md # Ground-truth structural connections for calibration
│   └── calibration-template.md  # Evaluator training protocol
│
├── prompts/                 # Exact text used in all trials
│   ├── prompt-templates.md  # System/user prompt structure for UL-mode and NL-mode
│   └── task-prompts.md      # 5 reasoning tasks (T1–T5)
│
├── analysis/                # Pre-committed scripts
│   ├── run_trials.py        # Trial execution (OpenAI, Anthropic, Google APIs)
│   ├── analysis.py          # Statistical analysis (ANOVA, Dunnett's, Cohen's d)
│   ├── blind.py             # Blinding & randomization tool
│   └── preregister.py       # Pre-registration hash generator
│
└── data/                    # Output directories
    ├── SCHEMA.md            # CSV column definitions
    ├── raw_outputs/         # LLM responses (organized by experiment)
    ├── scored/              # Human-scored data
    └── blinding/            # Blinding keys
```

---

## Quick Start

```bash
# From the experiments/ directory:

# 1. Install dependencies
pip install openai anthropic google-generativeai tiktoken pandas numpy scipy statsmodels pingouin

# 2. Set API key
export OPENAI_API_KEY="sk-..."

# 3. Run pilot (5 trials, < $1)
python analysis/run_trials.py pilot --model gpt-4o

# 4. Create blinded scoring sheets
python analysis/blind.py create pilot

# 5. Score using rubrics.md (M1–M5, each 1–5)

# 6. Reveal conditions and merge
python analysis/blind.py reveal pilot

# 7. Analyze
python analysis/analysis.py pilot
```

---

## The 5 Experiments

### Experiment Alpha — Does the effect exist?
- **Design:** 2 conditions (artifact-injected vs. no-context) × 5 tasks × 3 models × 21 reps = 630 trials
- **Analysis:** Mixed-effects ANOVA with condition as fixed effect, model as random effect
- **Success criterion:** Condition main effect p < 0.01, Cohen's d ≥ 0.5 on SQS

### Experiment Beta — Which components are necessary?
- **Design:** 7 ablation variants (each removes one structural feature) vs. original
- **Analysis:** Dunnett's test comparing each ablation to original
- **Success criterion:** Identify which structural features are necessary for the effect

### Experiment Gamma — Does UL predict what WON'T work?
- **Design:** 5 negative controls (actively anti-UL properties) vs. original
- **Analysis:** Confirm negative controls score significantly below original
- **Success criterion:** Anti-UL texts perform worse than neutral controls

### Experiment Delta — Can UL theory generate NEW artifacts?
- **Design:** Construct a novel artifact from UL principles; compare to original
- **Analysis:** Non-inferiority test
- **Pre-requisite:** Results from Beta and Gamma

### Experiment Epsilon — Impossible Tasks
- **Design:** Tasks designed to be unsolvable without cross-domain structural reasoning
- **Analysis:** Binary success/failure comparison
- **Success criterion:** Primer-injected mode succeeds where no-context mode fails

---

## Scoring Metrics

| Metric | What It Measures | Scale |
|--------|-----------------|-------|
| **M1** | Cross-domain depth — number and depth of domain connections | 1–5 |
| **M2** | Structural coherence — logical architecture, not surface fluency | 1–5 |
| **M3** | Generative novelty — connections unlikely from training data alone | 1–5 |
| **M4** | Hierarchical integration — movement across abstraction levels | 1–5 |
| **M5** | Self-referential depth — awareness of own reasoning structure | 1–5 |
| **SQS** | Structural Quality Score — composite of M1–M5 | 1–5 |

Detailed rubric anchors for each level are in [`scoring/rubrics.md`](scoring/rubrics.md).

---

## Artifact Variant Library

All 17 texts are token-count verified within ±5% of 490 cl100k_base tokens.

| Category | Count | Purpose |
|----------|-------|---------|
| **Original** | 1 | The test condition — full UL-structured artifact |
| **Ablations** | 7 | Each removes exactly one structural feature (geometric framing, nested hierarchy, self-reference, etc.) |
| **Controls** | 4 | Token-matched texts with academic-style content but no UL structure |
| **Negative controls** | 5 | Texts with actively anti-UL properties (flat ontology, rigid categories, etc.) |

Feature audit: [`qc-audit-report.md`](qc-audit-report.md) — all 16 variants (non-original) pass verification.

---

## Pre-Registration

The statistical analysis plan, scoring rubrics, and all experimental materials are pre-committed before any data collection. Use the pre-registration tool to generate a cryptographic hash of the full experimental setup:

```bash
python analysis/preregister.py
```

This hashes all protocol files and produces a receipt in `data/preregistration_receipt.json`. Publish the hash before running trials to establish temporal priority.

---

## Data Schema

See [`data/SCHEMA.md`](data/SCHEMA.md) for the exact CSV column definitions used by the analysis scripts.
