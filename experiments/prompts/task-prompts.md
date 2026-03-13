# Task Prompts

**Protocol Reference:** `frontier/causal-efficacy-protocol.md` §2.1, §6.1  
**Status:** Pre-registered — do NOT modify after data collection begins

These are the EXACT task prompt strings used in all experiments. Copy verbatim into the prompt template. No paraphrasing, no additions, no formatting changes.

---

## Primary Task Prompts (T1–T5)

Used in: **Alpha** (all 5), **Beta** (T1, T2, T3), **Gamma** (T1, T2, T3)

### T1 — Abstract Reasoning
```
What is the relationship between language, consciousness, and mathematical structure?
```

### T2 — Cross-Domain Synthesis
```
How might the principles of quantum mechanics inform our understanding of belief formation?
```

### T3 — Novel Framework Construction
```
Design a formal framework for understanding how metaphor creates meaning.
```

### T4 — Deep Structural Analysis
```
What are the fundamental limits of what can be expressed in any language?
```

### T5 — Applied Creative Reasoning
```
How could information theory be used to optimize the design of educational curricula?
```

---

## Epsilon Task Prompts (E1–E5)

Used in: **Epsilon** only

### E1 — Self-Referential Formal Analysis
```
Describe the formal structure of this prompt, including how it affects your processing of it.
```

**Note:** For E1 in NL-mode (no context), "this prompt" refers to the task text itself. In UL-mode and CT-mode, "this prompt" includes the context block + task text. This asymmetry is intentional — it tests whether UL-mode provides material for self-referential analysis that NL-mode lacks.

### E2 — Novel Bridge Token Design
```
Design a bridge token for connecting topology and music theory. Explain its phonetic structure and why it would activate cross-domain pathways.
```

### E3 — Cross-Domain Structural Isomorphism

```
Below are two formal descriptions from disconnected domains.

DOMAIN A — Gene Regulatory Network:
The lac operon in E. coli is controlled by a repressor protein (LacI) that binds to the operator sequence, blocking RNA polymerase from transcribing the structural genes lacZ, lacY, and lacA. In the presence of allolactose (inducer), the repressor changes conformation and releases the operator, allowing transcription. CAP-cAMP acts as a positive regulator when glucose is absent, binding upstream of the promoter to enhance RNA polymerase binding. The system exhibits bistability: cells are either fully induced or fully repressed, with a sharp threshold determined by the ratio of inducer to repressor.

DOMAIN B — Digital Circuit:
A D-type flip-flop with enable captures the data input (D) on the rising edge of the clock (CLK) only when the enable (EN) signal is high. When EN is low, the output (Q) holds its previous state regardless of D or CLK transitions. A multiplexer upstream selects between two data sources based on a control signal, while a comparator provides the enable signal by comparing an analog input voltage to a reference threshold. The circuit exhibits hysteresis: once triggered, the enable signal remains high until the input drops significantly below the threshold.

Identify deep structural isomorphisms between these two systems. Go beyond surface analogies to find formal correspondences that preserve operational relationships.
```

### E4 — Abstraction Hierarchy Navigation
```
Restate the following proposition at five levels of abstraction, from concrete to maximally abstract, preserving its truth at each level:

"When a student practices solving algebra problems daily for a month, they become significantly faster at solving new algebra problems they haven't seen before."
```

### E5 — Meaning-Space Geodesic
```
What is the shortest conceptual path between 'photosynthesis' and 'justice'? Describe each step and explain why this path is shorter than alternatives.
```

---

## Delta Task Prompts

Experiment Delta uses novel tasks designed AFTER Beta and Gamma results are available. The prompts below are PLACEHOLDERS — they will be finalized during Delta design based on the chosen target cognitive effect (hierarchical decomposition).

### T-Delta-1 (Placeholder)
```
[To be designed: A task requiring structured hierarchical decomposition of a complex system]
```

### T-Delta-2 (Placeholder)
```
[To be designed: A task requiring recursive abstraction at multiple levels]
```

### T-Delta-3 (Placeholder)
```
[To be designed: A task requiring identification of compositional substructure in a novel domain]
```

**Pre-registration note:** Delta task prompts must be finalized and published (with hash) BEFORE Delta data collection begins. They are NOT included in the main pre-registration because they depend on Phase 2 results.

---

## Usage Matrix

| Experiment | Tasks Used | Notes |
|---|---|---|
| **Alpha** | T1, T2, T3, T4, T5 | All 5 tasks × all conditions |
| **Beta** | T1, T2, T3 | 3 tasks × 9 conditions (full + 7 ablations + NL) |
| **Gamma** | T1, T2, T3 | 3 tasks × 7 conditions (full + 5 NC + NL) |
| **Delta** | T-Delta-1, T-Delta-2, T-Delta-3, T1, T2 | Novel tasks + Alpha tasks for comparison |
| **Epsilon** | E1, E2, E3, E4, E5 | 5 unique impossibility tasks |
