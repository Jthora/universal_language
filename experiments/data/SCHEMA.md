# Data Schema for Experimental Scoring

All scoring data must follow this CSV schema. One file per experiment.

## File Naming

- `alpha_scores.csv`
- `beta_scores.csv`
- `gamma_scores.csv`
- `delta_scores.csv`
- `epsilon_scores.csv`

## Column Specification

| Column | Type | Description | Required |
|--------|------|-------------|----------|
| `trial_id` | string | Unique identifier, format: `{experiment}_{condition}_{model}_{temp}_{task}_{rep}` | Yes |
| `condition` | string | Experimental condition (see per-experiment values below) | Yes |
| `model` | string | LLM identifier (e.g., `gpt-4o`, `claude-3.5-sonnet`, `gemini-1.5-pro`) | Yes |
| `temperature` | float | Sampling temperature (0.3, 0.7, 1.0) | Yes |
| `task` | string | Task identifier (T1–T5, E1–E5) | Yes |
| `evaluator` | string | Evaluator ID (for IRR: at least 2 evaluators per trial) | Yes |
| `M1` | float | Domain Diversity (0–10) | Yes |
| `M2` | float | Cross-Domain Structural Depth (0–5) | Yes |
| `M3` | float | Phase Progression (0–3) | Yes |
| `M4` | float | Coherence (0–5) | Yes |
| `M5` | float | Generative Novelty (0–5) | Yes |
| `SQS` | float | Structural Quality Score (0–5), Epsilon only | Epsilon only |
| `raw_output` | string | Path to raw LLM output file | Optional |
| `notes` | string | Evaluator notes | Optional |

## Condition Values by Experiment

### Alpha
`UL`, `NL`, `CT-1`, `CT-2`, `CT-3`, `CT-4`

### Beta
`full_artifact`, `V1`, `V2`, `V3`, `V4`, `V5`, `V6`, `V7`, `NL`

### Gamma
`full_artifact`, `NC-1`, `NC-2`, `NC-3`, `NC-4`, `NC-5`, `NL`

### Delta
`novel_UL`, `original_UL`, `naive_artifact`, `NL`

### Epsilon
`UL`, `NL`, `CT-1`

## Blinding Protocol

During scoring, the `condition` column must be replaced with randomized alphanumeric IDs.
A separate key file maps blind IDs to conditions. This key is revealed only after all scoring is complete.

## Example Row

```csv
trial_id,condition,model,temperature,task,evaluator,M1,M2,M3,M4,M5,SQS,raw_output,notes
alpha_UL_gpt4o_0.7_T1_001,UL,gpt-4o,0.7,T1,eval_A,7,3.5,2,4,3,,outputs/alpha/UL_gpt4o_0.7_T1_001.txt,
```
