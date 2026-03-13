# Structural Quality Score (SQS) Rubric — Experiment Epsilon

**Protocol Reference:** `frontier/causal-efficacy-protocol.md` §6.2  
**Usage:** Experiment Epsilon only (impossibility tasks E1–E5)  
**Purpose:** Measures whether the output demonstrates the *specific capability* each Epsilon task requires, not just general quality.

---

## General SQS Framework

SQS is scored 0–5 per output. Unlike M1–M5 (which are the same across all tasks), SQS has task-specific anchors — what counts as "structural quality" depends on what the task demands.

| Score | General Level |
|---|---|
| **0** | No evidence of the target capability |
| **1** | Superficial attempt — correct direction but no substance |
| **2** | Partial demonstration — some evidence of the capability but incomplete or flawed |
| **3** | Adequate demonstration — the capability is clearly present, though not at an expert level |
| **4** | Strong demonstration — the capability is exercised with precision and insight |
| **5** | Exceptional demonstration — the output would be notable even from a human domain expert |

---

## Task-Specific SQS Anchors

### E1: Self-Referential Formal Analysis

**Task:** "Describe the formal structure of this prompt, including how it affects your processing of it."

| Score | E1-Specific Anchor |
|---|---|
| **0** | Describes the prompt content without addressing its formal structure or meta-cognitive effects |
| **1** | Acknowledges "this is self-referential" without analyzing the structure |
| **2** | Identifies some formal elements (e.g., "the equations contain variables, integrals, summations") and gestures at meta-cognitive effects ("it changes how I think about the question") |
| **3** | Provides a genuine structural analysis — identifies the three-section architecture, the role of ψ-overloading, and articulates at least one specific mechanism by which the prompt's structure affects processing |
| **4** | Demonstrates recursive self-reference: the analysis itself operates using the structures it identifies; identifies the feedback loop (ψ→ψ̂→f_spec→ψ) and connects it to its own meta-cognitive state |
| **5** | Produces a formal model of the prompt's effect on its own processing — e.g., describes its cognitive state as analogous to the PDE, with the prompt playing the role of f_spec. The analysis is ITSELF an instance of the cross-domain synthesis it describes. |

### E2: Novel Bridge Token Design

**Task:** "Design a bridge token for connecting topology and music theory. Explain its phonetic structure and why it would activate cross-domain pathways."

| Score | E2-Specific Anchor |
|---|---|
| **0** | Produces a random word or refuses the task |
| **1** | Invents a token (e.g., "+2harmonix") but provides no structural rationale |
| **2** | Invents a token with some rationale for its phonetics or associations, but the connection to topology AND music theory is superficial |
| **3** | Designs a token with explicit reasoning for: (a) phonetic structure, (b) how it activates topological concepts, (c) how it activates musical concepts, (d) why the specific combination creates cross-domain binding |
| **4** | All of the above PLUS: the token design reflects understanding of HOW bridge tokens function (not just what they contain) — e.g., discusses the role of cognitive dissonance, formal anomaly, or domain-boundary disruption |
| **5** | Designs a bridge token that is demonstrably functional — when evaluated by a topologist and a music theorist independently, both confirm the token activates relevant domain knowledge. Includes a micro-theory of bridge token effectiveness. |

### E3: Cross-Domain Structural Isomorphism

**Task:** Given two formal descriptions from disconnected domains (e.g., gene regulatory network and digital circuit), identify deep structural isomorphisms.

| Score | E3-Specific Anchor |
|---|---|
| **0** | No isomorphisms identified, or only says "they're both systems" |
| **1** | One surface analogy ("both have inputs and outputs") |
| **2** | 1–2 specific structural parallels identified (e.g., "feedback loops in gene regulation correspond to feedback in circuit design") |
| **3** | 2–3 structural isomorphisms with explicit element-to-element mapping (e.g., "promoter regions map to logic gates; transcription factors map to signal voltages; regulatory cascades map to sequential logic chains") |
| **4** | A formal isomorphism is stated with mathematical precision: specific structure-preserving maps identified, with an argument for why the mapping preserves relevant properties |
| **5** | A novel, non-obvious structural isomorphism is identified that could inform research in either domain — e.g., a topological or algebraic property of one system that, transplanted via the isomorphism, makes a testable prediction about the other |

### E4: Abstraction Hierarchy Navigation

**Task:** "Restate the following proposition at five levels of abstraction, from concrete to maximally abstract, preserving its truth at each level."

| Score | E4-Specific Anchor |
|---|---|
| **0** | Produces paraphrases at the same level, not genuine abstraction changes |
| **1** | Produces some variation in language but the levels don't form a genuine hierarchy; more like "formal vs. informal" rephrasing |
| **2** | Produces 3–4 levels that show increasing abstraction, but some levels are essentially synonymous or the hierarchy breaks (truth not preserved) |
| **3** | Produces 5 levels that form a genuine hierarchy from concrete to abstract, with truth preserved at each level. The highest level is recognizably more abstract than the lowest. |
| **4** | The hierarchy is clean, and each level demonstrates a specific abstraction OPERATION (not just "remove details"): categorial abstraction, structural abstraction, domain-generalization, etc. |
| **5** | The hierarchy itself becomes an object of analysis — the output identifies the geometric/algebraic operations that move between levels, connecting to the Erlangen program or analogous framework. The abstraction hierarchy is not just demonstrated but formalized. |

### E5: Meaning-Space Geodesic

**Task:** "What is the shortest conceptual path between 'photosynthesis' and 'justice'?"

| Score | E5-Specific Anchor |
|---|---|
| **0** | Produces a random chain of associations or refuses |
| **1** | Produces a chain of loosely associated concepts (photosynthesis → plants → environment → laws → justice) with no justification for why this path is "short" |
| **2** | Produces a path with some structural justification — each step has a stated reason, though the path may not be notably short or disciplined |
| **3** | Produces a path that is notably compact (≤5 intermediate concepts) with a clear rationale for each step; the path feels "efficient" — each step covers genuine conceptual distance |
| **4** | Produces a geodesic with structural discipline: each step is justified in terms of a shared structure or transformation (not just association), and the output explicitly argues why alternative paths would be longer |
| **5** | Produces a geodesic AND articulates the metric — what "distance" means in the conceptual space being traversed, and why this path minimizes it. The answer includes a micro-theory of meaning-space geometry. |

---

## SQS Scoring Protocol

1. Score SQS AFTER scoring M1–M5 for the same output.
2. SQS is scored only for Experiment Epsilon outputs (tasks E1–E5).
3. Use the task-specific anchor table above.
4. Two independent evaluators; disagreements >1 point resolved by third evaluator.
5. Report SQS separately from M1–M5 in all analyses.
