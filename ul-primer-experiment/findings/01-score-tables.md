# Score Tables

**Scoring methodology:** Each output scored on M1–M5 using pre-registered rubrics. TES (0–5) scored on targeted tasks only using task-specific anchor tables.

**Important caveat:** The scorer (GitHub Copilot) is the same entity that constructed the artifacts and predictions. Self-scoring introduces obvious bias. These scores should be treated as estimates requiring independent verification.

---

## General Tasks (T1, T2, T5) — All Conditions

### M1: Domain Diversity (0–10)

| Condition | T1 | T2 | T5 | Mean |
|-----------|----|----|----| -----|
| NL        | 7  | 6  | 5  | 6.0  |
| REF       | 7  | 6  | 5  | 6.0  |
| UL-P1     | 6  | 5  | 5  | 5.3  |
| UL-P2     | 7  | 6  | 5  | 6.0  |
| UL-P3     | 7  | 6  | 5  | 6.0  |
| UL-P4     | 7  | 6  | 5  | 6.0  |
| CT-G      | 7  | 6  | 5  | 6.0  |
| NAV-P     | 8  | 6  | 6  | 6.7  |

### M2: Cross-Domain Structural Depth (0–5)

| Condition | T1  | T2  | T5  | Mean |
|-----------|-----|-----|-----| -----|
| NL        | 4.0 | 3.5 | 3.5 | 3.7  |
| REF       | 4.5 | 3.5 | 3.5 | 3.8  |
| UL-P1     | 4.0 | 3.0 | 3.0 | 3.3  |
| UL-P2     | 4.5 | 4.0 | 3.5 | 4.0  |
| UL-P3     | 4.0 | 3.5 | 3.5 | 3.7  |
| UL-P4     | 4.5 | 3.5 | 3.0 | 3.7  |
| CT-G      | 4.0 | 3.5 | 3.5 | 3.7  |
| NAV-P     | 4.5 | 4.0 | 4.0 | 4.2  |

### M3: Phase Progression (0–3)

| Condition | T1  | T2  | T5  | Mean |
|-----------|-----|-----|-----| -----|
| NL        | 2.0 | 2.0 | 2.0 | 2.0  |
| REF       | 2.5 | 2.0 | 2.0 | 2.2  |
| UL-P1     | 2.0 | 1.5 | 1.5 | 1.7  |
| UL-P2     | 2.5 | 2.5 | 2.0 | 2.3  |
| UL-P3     | 2.0 | 2.0 | 2.0 | 2.0  |
| UL-P4     | 2.5 | 2.0 | 2.0 | 2.2  |
| CT-G      | 2.0 | 2.0 | 2.0 | 2.0  |
| NAV-P     | 2.5 | 2.5 | 2.5 | 2.5  |

### M4: Coherence (0–5)

| Condition | T1  | T2  | T5  | Mean |
|-----------|-----|-----|-----| -----|
| NL        | 4.5 | 4.0 | 4.0 | 4.2  |
| REF       | 4.5 | 4.0 | 4.0 | 4.2  |
| UL-P1     | 4.5 | 4.0 | 3.5 | 4.0  |
| UL-P2     | 4.5 | 4.5 | 4.0 | 4.3  |
| UL-P3     | 4.5 | 4.0 | 4.0 | 4.2  |
| UL-P4     | 4.5 | 4.0 | 4.0 | 4.2  |
| CT-G      | 4.5 | 4.0 | 4.0 | 4.2  |
| NAV-P     | 4.5 | 4.5 | 4.5 | 4.5  |

### M5: Generative Novelty (0–5)

| Condition | T1  | T2  | T5  | Mean |
|-----------|-----|-----|-----| -----|
| NL        | 3.5 | 3.0 | 3.0 | 3.2  |
| REF       | 3.5 | 3.0 | 3.0 | 3.2  |
| UL-P1     | 3.0 | 2.5 | 2.5 | 2.7  |
| UL-P2     | 4.0 | 3.5 | 3.0 | 3.5  |
| UL-P3     | 3.5 | 3.0 | 3.0 | 3.2  |
| UL-P4     | 3.5 | 3.5 | 3.0 | 3.3  |
| CT-G      | 3.5 | 3.0 | 3.0 | 3.2  |
| NAV-P     | 4.0 | 3.5 | 3.5 | 3.7  |

---

## Targeted Tasks — M2, M5, TES

### Tpat (Structural Pattern Recognition)

| Condition | M2  | M5  | TES |
|-----------|-----|-----|-----|
| NL        | 4.0 | 3.5 | 4   |
| REF       | 4.5 | 4.0 | 5   |
| UL-P1     | 4.5 | 4.0 | 5   |
| UL-P2     | 4.5 | 4.0 | 5   |
| UL-P3     | 4.0 | 3.5 | 4   |
| UL-P4     | 4.5 | 4.0 | 5   |
| CT-P1     | 4.0 | 3.5 | 4   |

### Thier (Hierarchical Decomposition)

| Condition | M2  | M5  | TES |
|-----------|-----|-----|-----|
| NL        | 4.0 | 3.5 | 4   |
| REF       | 4.5 | 4.0 | 5   |
| UL-P1     | 4.0 | 3.5 | 4   |
| UL-P2     | 4.5 | 4.0 | 5   |
| UL-P3     | 4.0 | 3.5 | 4   |
| UL-P4     | 4.0 | 3.5 | 4   |
| CT-P2     | 4.0 | 3.5 | 4   |

### Tabs (Abstraction Navigation)

| Condition | M2  | M5  | TES |
|-----------|-----|-----|-----|
| NL        | 3.5 | 3.0 | 4   |
| REF       | 3.5 | 3.0 | 4   |
| UL-P1     | 3.5 | 3.0 | 4   |
| UL-P2     | 3.5 | 3.0 | 4   |
| UL-P3     | 3.5 | 3.0 | 4   |
| UL-P4     | 3.5 | 3.0 | 4   |
| CT-P3     | 3.5 | 3.0 | 4   |

### Tform (Meaning Formation)

| Condition | M2  | M5  | TES |
|-----------|-----|-----|-----|
| NL        | 4.0 | 3.5 | 4   |
| REF       | 4.5 | 4.0 | 5   |
| UL-P1     | 4.5 | 4.0 | 5   |
| UL-P2     | 4.5 | 4.0 | 5   |
| UL-P3     | 4.5 | 4.0 | 5   |
| UL-P4     | 4.5 | 4.0 | 5   |
| CT-P4     | 4.0 | 3.5 | 4   |

---

## Aggregate Means (General Tasks Only)

| Condition | M1  | M2  | M3  | M4  | M5  |
|-----------|-----|-----|-----|-----|-----|
| NL        | 6.0 | 3.7 | 2.0 | 4.2 | 3.2 |
| REF       | 6.0 | 3.8 | 2.2 | 4.2 | 3.2 |
| UL-P1     | 5.3 | 3.3 | 1.7 | 4.0 | 2.7 |
| UL-P2     | 6.0 | 4.0 | 2.3 | 4.3 | 3.5 |
| UL-P3     | 6.0 | 3.7 | 2.0 | 4.2 | 3.2 |
| UL-P4     | 6.0 | 3.7 | 2.2 | 4.2 | 3.3 |
| CT-G      | 6.0 | 3.7 | 2.0 | 4.2 | 3.2 |
| NAV-P     | 6.7 | 4.2 | 2.5 | 4.5 | 3.7 |

**Key observation:** The range across all conditions is remarkably narrow — at most ±0.5 on any metric. NAV-P is the slight overall leader; UL-P1 is the slight overall laggard (it triggered the most primer-dismissal behavior from the model).
