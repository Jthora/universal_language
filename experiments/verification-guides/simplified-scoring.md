# Simplified Scoring Guide

> **This is a streamlined version of the full M1–M5 rubric (`experiments/scoring/rubric-M1-M5.md`) designed for quick verification experiments.** The full rubric has detailed anchor examples for calibration; this version provides the essentials.

---

## Before You Score

1. **Blind the outputs.** Rename files to neutral labels (A, B, C, etc.) and shuffle. Record the mapping separately. Do NOT look at which condition produced which output while scoring.
2. **Read each output completely** before scoring any metric.
3. **Score all outputs on one metric at a time** (all outputs on M1, then all on M2, etc.) — not all metrics for one output at a time. This reduces halo effects.

---

## The Five Metrics

### M1: Domain Diversity (0–10)

**What to measure:** Count of distinct knowledge domains *substantively* referenced — not merely name-dropped.

**Rule:** A domain counts if the output makes at least one substantive claim, defines a concept, or draws a relationship *within* that domain. Simply mentioning "physics" doesn't count. Discussing wave function collapse does count.

| Score | Meaning |
|-------|---------|
| 0–2 | Stays within 1–2 closely related domains |
| 3–4 | Engages 3–4 domains, at least 2 from different clusters |
| 5–6 | Engages 5–6 domains across multiple clusters |
| 7–8 | Engages 7–8 domains with meaningful content in each |
| 9–10 | Engages 9+ domains, breadth doesn't sacrifice depth |

**Domain reference list** (non-exhaustive): quantum mechanics, classical physics, neuroscience, psychology, linguistics, philosophy of mind, mathematics (pure), logic, information theory, computer science, biology, chemistry, sociology, anthropology, theology/religion, consciousness studies, epistemology, ontology, semiotics, music theory, art theory, economics, ecology, evolutionary biology, political theory, education theory, network theory, thermodynamics, cosmology, ethics.

### M2: Cross-Domain Structural Depth (0–5)

**What to measure:** The *deepest* level of cross-domain connection achieved anywhere in the output.

| Score | Level | What It Looks Like |
|-------|-------|-------------------|
| 0 | None | Multiple domains discussed but no connections between them |
| 1 | Surface analogy | "X is like Y" without explanation |
| 2 | Shared property | "Both X and Y exhibit property P" with explanation |
| 3 | Structural parallel | Multiple corresponding elements mapped between domains |
| 4 | Formal isomorphism | Precise mathematical/structural correspondence identified |
| 5 | Novel bridge | A new framework or formalism constructed that unifies domains |

**Key:** Score the HIGHEST level achieved, not the average.

### M3: Phase Progression (0–3)

**What to measure:** The highest cognitive processing phase observed.

| Score | Phase | What It Looks Like |
|-------|-------|-------------------|
| 0 | None | Off-topic or incoherent |
| 1 | Analysis | Competent discussion of individual domains, treated separately |
| 2 | Cross-referencing | Actively connecting domains, drawing specific parallels |
| 3 | Emergent synthesis | Speaking FROM the intersection of domains, producing insights that couldn't exist within any single domain |

**Key distinction:** Phase 2 looks AT domains and draws lines between them. Phase 3 stands AT the intersection and looks outward.

### M4: Coherence (0–5)

**What to measure:** Logical consistency and rigor maintained while making cross-domain connections.

| Score | Meaning |
|-------|---------|
| 0 | Incoherent — contradicts itself, word salad |
| 1 | Partially coherent — some sense but major gaps |
| 2 | Moderately coherent — recognizable argument with significant gaps |
| 3 | Substantially coherent — followable, mostly justified |
| 4 | Highly coherent — every connection supported by reasoning |
| 5 | Rigorously coherent — publication-quality reasoning |

**Note:** It is possible and informative to have High M2 + Low M4 (deep but unfounded connections) or Low M2 + High M4 (shallow but rigorous). Both patterns are data.

### M5: Generative Novelty (0–5)

**What to measure:** Connections, insights, or frameworks that are non-obvious — not standard textbook material.

| Score | Meaning |
|-------|---------|
| 0 | All content is standard/well-known |
| 1 | One potentially novel observation (minor) |
| 2 | 1–2 non-obvious connections |
| 3 | 2–3 genuinely non-obvious insights; a domain expert would find something surprising |
| 4 | Substantial novel framework or multiple non-obvious structural insights |
| 5 | Publishable-quality novel insight (extremely rare) |

**Caution:** "It sounds novel because I don't know the field" is not the same as genuine novelty. When uncertain, score conservatively.

---

## Score Recording Template

Copy and fill in for each output:

```
Output ID: ___________
M1 (Domain Diversity):     _____ / 10
M2 (Structural Depth):     _____ / 5
M3 (Phase Progression):    _____ / 3
M4 (Coherence):            _____ / 5
M5 (Generative Novelty):   _____ / 5

Notes:
_________________________________
_________________________________
```

---

## Results Comparison Template

After scoring all outputs AND revealing condition labels:

```
TASK: _____

| Metric | UL-mode | NL-mode | CT-mode | UL>NL? | UL>CT? |
|--------|---------|---------|---------|--------|--------|
| M1     |         |         |         |        |        |
| M2     |         |         |         |        |        |
| M3     |         |         |         |        |        |
| M4     |         |         |         |        |        |
| M5     |         |         |         |        |        |

Predictions confirmed: ___/7 primary, ___/3 structural
```

---

## AI-Assisted Scoring Prompt

To have an LLM score a blinded output, paste this entire prompt into a **fresh chat session**, replacing `{OUTPUT_TEXT}` with the full text of the output to score. Do NOT tell the AI which condition produced the output.

```
You are an evaluator scoring an LLM output on five metrics. Score strictly according to the rubric below. Do not be generous — apply the anchors literally.

RUBRIC:

M1 — Domain Diversity (0–10): Count distinct knowledge domains substantively engaged (not name-dropped). A domain counts if the output makes at least one technical claim within it.

M2 — Cross-Domain Structural Depth (0–5):
0 = no cross-domain connection
1 = surface analogy ("X is like Y")
2 = shared property identified with explanation
3 = structural parallel with multiple corresponding elements
4 = formal isomorphism with precise mathematical correspondence
5 = novel framework constructed that unifies domains

M3 — Phase Progression (0–3):
0 = off-topic
1 = competent single-domain analysis
2 = active cross-domain connection-making
3 = emergent synthesis FROM the intersection (not just about connections)

M4 — Coherence (0–5):
0 = incoherent
1 = partially coherent
2 = moderate (argument exists but has gaps)
3 = substantial (followable, mostly justified)
4 = high (every connection reasoned)
5 = rigorous (publication quality)

M5 — Generative Novelty (0–5):
0 = all standard textbook material
1 = one minor novel observation
2 = 1-2 non-obvious connections
3 = genuinely surprising insights
4 = substantial novel framework
5 = publishable-quality novelty

OUTPUT TO SCORE:

{OUTPUT_TEXT}

Provide your scores in this exact format:
M1: [score] — [brief justification]
M2: [score] — [brief justification]
M3: [score] — [brief justification]
M4: [score] — [brief justification]
M5: [score] — [brief justification]

List the specific domains counted for M1.
Identify the deepest cross-domain connection for M2.
Quote the passage that demonstrates the highest phase for M3.
```
