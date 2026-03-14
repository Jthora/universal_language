# Verification Guides — Multiple Paths to Testing UL's Causal Efficacy

> **No API keys? No problem.** This directory provides multiple independent approaches to testing whether UL-structured text (the test artifact) produces measurable effects on LLM cognition. Every approach uses the same tasks, the same metrics, and the same pre-registered predictions — only the interface differs.

---

## Why Multiple Approaches Matter

The original experimental protocol (`frontier/causal-efficacy-protocol.md`) requires API keys for OpenAI, Anthropic, or Google. This is a barrier that excludes most potential replicators. But the core experiment is simple:

1. Give an LLM a reasoning task **with** the test artifact as context (UL-mode)
2. Give the same LLM the same task **without** any context (NL-mode)
3. Give the same LLM the same task with a **matched control text** (CT-mode)
4. Compare the outputs on pre-defined metrics

This can be done through ANY interface that lets you send text to an LLM and read the response. The API is just one such interface.

---

## Available Approaches

| Approach | Requirements | Cost | Automation | Best For |
|----------|-------------|------|------------|----------|
| **[VSCode + GitHub Copilot](vscode-copilot.md)** | VS Code, GitHub Copilot subscription | Included in Copilot subscription | Semi-manual | Developers already using Copilot |
| **[Manual Chat Interface](manual-chat.md)** | Any browser | Free (free tiers of ChatGPT, Claude, Gemini) | Manual | Anyone with internet access |
| **[Local Models (Ollama)](local-models.md)** | Computer with 8+ GB RAM | Free | Scriptable | Privacy-conscious, offline use, bulk trials |
| **[API Script](../analysis/run_trials.py)** | API key(s) | Pay-per-use | Fully automated | Large-scale replication, statistical power |

All approaches use:
- The **same task prompts** (from `experiments/prompts/task-prompts.md`)
- The **same primer text** (from `experiments/test-artifacts/original/primer.txt`)
- The **same control texts** (from `experiments/test-artifacts/controls/`)
- The **same scoring rubrics** (simplified version in `simplified-scoring.md`, full version in `experiments/scoring/`)

---

## Quick-Start Decision Tree

```
Do you have API keys for OpenAI, Anthropic, or Google?
├── YES → Use the API script: python analysis/run_trials.py pilot --model gpt-4o
└── NO
    ├── Do you have VS Code with GitHub Copilot?
    │   └── YES → Follow vscode-copilot.md
    ├── Do you have a computer with 8+ GB RAM?
    │   └── YES → Follow local-models.md (free, offline, scriptable)
    └── Do you have a web browser?
        └── YES → Follow manual-chat.md (free tier ChatGPT/Claude/Gemini)
```

---

## Shared Experimental Design

Regardless of approach, every valid test must include:

### Minimum Viable Experiment (3 trials)
- **1 trial** in UL-mode (artifact + task T1)
- **1 trial** in NL-mode (no context + task T1)
- **1 trial** in CT-mode (control text CT-1 + task T1)

This is the smallest test that can distinguish "the test artifact helps" from "any math helps" and "just asking the question is enough."

### Recommended Quick Test (9 trials)
- **3 conditions** (UL, NL, CT-1) × **3 tasks** (T1, T2, T3)

This shows whether effects are consistent across different reasoning tasks.

### Full Replication (45+ trials)
- **3 conditions** × **5 tasks** × **3 repetitions** = 45 trials minimum

This provides enough data for basic statistical analysis.

---

## Pre-Registered Predictions

These predictions are specified BEFORE any data collection. They are derived from UL theory (`history/mechanism-of-action.md`) and the formal structure of the test artifact (`history/artifact-analysis.md`).

### Primary Predictions

| # | Prediction | Metric | Direction |
|---|-----------|--------|-----------|
| P1 | UL-mode outputs reference more distinct knowledge domains than NL-mode | M1 (Domain Diversity) | UL > NL |
| P2 | UL-mode outputs reference more distinct knowledge domains than CT-mode | M1 (Domain Diversity) | UL > CT |
| P3 | UL-mode outputs achieve deeper cross-domain connections than NL-mode | M2 (Structural Depth) | UL > NL |
| P4 | UL-mode outputs achieve deeper cross-domain connections than CT-mode | M2 (Structural Depth) | UL > CT |
| P5 | UL-mode outputs more frequently reach Phase 3 synthesis than NL-mode | M3 (Phase Progression) | UL > NL |
| P6 | UL-mode coherence is NOT lower than NL-mode (the test artifact doesn't cause incoherence) | M4 (Coherence) | UL ≥ NL |
| P7 | UL-mode outputs contain more novel connections than NL-mode | M5 (Novelty) | UL > NL |

### Structural Predictions (stronger claims)

| # | Prediction | Rationale |
|---|-----------|-----------|
| S1 | CT-1 (dense physics) does NOT produce the UL-mode pattern — high M1 requires cross-domain activation, not just math | CT-1 is single-domain math |
| S2 | CT-2 (cross-domain prose) produces SOME improvement over NL but less than UL — prose activates cross-domain thinking but lacks the formal binding | CT-2 has cross-domain content but not UL structure |
| S3 | The UL-CT gap is largest on M2 (structural depth) and M3 (phase progression), not M1 (domain count) — the test artifact's unique contribution is deep structural binding, not mere topic diversity | UL theory predicts structural, not topical, enhancement |

### Falsification Criteria

The artifact effect is **NOT present** if any of the following hold:
- NL-mode outputs score equal to or higher than UL-mode on 3+ of 5 metrics
- CT-1 outputs score equal to UL-mode on 3+ of 5 metrics (any math works just as well)
- UL-mode M4 (coherence) is substantially lower than NL-mode (artifact causes incoherence rather than integration)

---

## Scoring

See [`simplified-scoring.md`](simplified-scoring.md) for a streamlined rubric suitable for all approaches.

For AI-assisted scoring (having an LLM score blinded outputs), see the scoring prompt template in the same file.

---

## Reporting Results

If you run the experiment, document:
1. **Model used** (name, version, interface)
2. **Date** of trials
3. **Condition order** (what you ran first — order effects matter)
4. **Raw outputs** (full text of each response)
5. **Scores** (per metric, per output)
6. **Who scored** (you, an AI, both)
7. **Any anomalies** (model refusals, errors, unusual behavior)

Share results by opening an issue or PR on the repository, or by any other public channel. Independent replications — positive or negative — are equally valuable.

---

## Integrity Notes

- **Do not modify prompts** after seeing results. Use them exactly as specified.
- **Do not cherry-pick trials.** Report all trials, including ones where UL-mode doesn't look impressive.
- **Do not score your own outputs unblinded.** Use the blinding procedure (label outputs A/B/C, shuffle, score, then reveal).
- **Negative results are real results.** If the test artifact doesn't work, that's important information. Report it.
