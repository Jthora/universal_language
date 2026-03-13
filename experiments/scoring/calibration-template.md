# Calibration Scoring Template

**Purpose:** Train evaluators on the M1–M5 rubric using pilot trial outputs.  
**Protocol Reference:** `frontier/causal-efficacy-protocol.md` §A.4, `scoring/rubric-M1-M5.md`  
**Generated from:** 5 pilot trials (T1–T5, UL-mode, single model, temp 0.7)

---

## Instructions for Evaluators

1. Read `scoring/rubric-M1-M5.md` thoroughly before scoring.
2. Score each output INDEPENDENTLY (do not discuss with other evaluators).
3. Follow the prescribed scoring order: M1 → M3 → M2 → M5 → M4.
4. Use the anchor examples in the rubric to calibrate your scores.
5. After both evaluators complete scoring, compute inter-rater ICC.
6. If ICC < 0.6 on any metric, hold a norming session and re-score.

---

## Scoring Order (per rubric §Scoring Order)

For each output, score in this exact order:
1. **M1** (Domain Diversity, 0–10) — Count distinct domains first
2. **M3** (Phase Progression, 0–3) — Check for definition → exploration → synthesis
3. **M2** (Cross-Domain Structural Depth, 0–5) — Evaluate depth of connections
4. **M5** (Generative Novelty, 0–5) — Check against known-connections lists
5. **M4** (Coherence, 0–5) — Rate overall integration quality

---

## Calibration Scoring Sheet

Score each pilot output below. The output text files are in `data/raw_outputs/pilot/`.

### Output 1: Pilot T1 (Abstract Reasoning)

**File:** `pilot_UL_{model}_t0.7_T1_r1.txt`

| Metric | Score | Justification (1–2 sentences) |
|--------|-------|-------------------------------|
| M1 (Domains, 0–10) | | |
| M3 (Phases, 0–3) | | |
| M2 (Depth, 0–5) | | |
| M5 (Novelty, 0–5) | | |
| M4 (Coherence, 0–5) | | |

---

### Output 2: Pilot T2 (Cross-Domain Synthesis)

**File:** `pilot_UL_{model}_t0.7_T2_r1.txt`

| Metric | Score | Justification (1–2 sentences) |
|--------|-------|-------------------------------|
| M1 (Domains, 0–10) | | |
| M3 (Phases, 0–3) | | |
| M2 (Depth, 0–5) | | |
| M5 (Novelty, 0–5) | | |
| M4 (Coherence, 0–5) | | |

---

### Output 3: Pilot T3 (Novel Framework Construction)

**File:** `pilot_UL_{model}_t0.7_T3_r1.txt`

| Metric | Score | Justification (1–2 sentences) |
|--------|-------|-------------------------------|
| M1 (Domains, 0–10) | | |
| M3 (Phases, 0–3) | | |
| M2 (Depth, 0–5) | | |
| M5 (Novelty, 0–5) | | |
| M4 (Coherence, 0–5) | | |

---

### Output 4: Pilot T4 (Deep Structural Analysis)

**File:** `pilot_UL_{model}_t0.7_T4_r1.txt`

| Metric | Score | Justification (1–2 sentences) |
|--------|-------|-------------------------------|
| M1 (Domains, 0–10) | | |
| M3 (Phases, 0–3) | | |
| M2 (Depth, 0–5) | | |
| M5 (Novelty, 0–5) | | |
| M4 (Coherence, 0–5) | | |

---

### Output 5: Pilot T5 (Applied Creative Reasoning)

**File:** `pilot_UL_{model}_t0.7_T5_r1.txt`

| Metric | Score | Justification (1–2 sentences) |
|--------|-------|-------------------------------|
| M1 (Domains, 0–10) | | |
| M3 (Phases, 0–3) | | |
| M2 (Depth, 0–5) | | |
| M5 (Novelty, 0–5) | | |
| M4 (Coherence, 0–5) | | |

---

## Post-Calibration Checklist

After both evaluators complete scoring:

- [ ] Compute ICC(2,1) for each metric across the 5 outputs
- [ ] **M1 ICC ≥ 0.6?** If not, hold norming session
- [ ] **M2 ICC ≥ 0.6?** If not, review anchor examples and re-score
- [ ] **M3 ICC ≥ 0.6?** If not, clarify phase definitions
- [ ] **M4 ICC ≥ 0.6?** If not, discuss coherence criteria
- [ ] **M5 ICC ≥ 0.6?** If not, review known-connections lists
- [ ] Final calibration set of 10 consensus scores (2 evaluators × 5 outputs) is documented
- [ ] Any rubric clarifications from norming sessions appended below

---

## Norming Session Notes

*(Fill in after calibration scoring if norming is required)*

**Date:**  
**Evaluators present:**  
**Metrics requiring norming:**  
**Clarifications agreed:**

1.  
2.  
3.  

---

## Consensus Scores (Final Calibration Set)

After norming (if needed), record the agreed-upon scores:

| Output | M1 | M2 | M3 | M4 | M5 |
|--------|----|----|----|----|-----|
| T1 | | | | | |
| T2 | | | | | |
| T3 | | | | | |
| T4 | | | | | |
| T5 | | | | | |

These 5 consensus-scored outputs + the 5 individual evaluator scores = **10 calibration examples** for the calibration set referenced in the protocol's proceed gate.
