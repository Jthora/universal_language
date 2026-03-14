# Causal Efficacy Protocol — Pre-Registered Experimental Design for Proof 4

**Research Series:** Universal Language — Proof of Reality and AI Cognitive Infrastructure  
**Sprint:** 6 — Path A  
**Date:** (Pre-registration)  
**Prerequisites:** `foundations/paradigm.md` (READ FIRST), `history/mechanism-of-action.md`, `foundations/formal-foundations.md`, `frontier/strategic-plan.md`

> **⚠ PARADIGM NOTE:** This protocol tests whether UL has *causal power over information-processing systems* — whether AI operating in UL-mode produces capabilities that natural-language-bound AI cannot match. It does NOT test whether UL "describes" human languages or "captures" linguistic intuitions. See `foundations/paradigm.md`.

---

## PURPOSE

Proof 4 in the four-proof framework (`foundations/paradigm.md`) requires demonstrating **causal efficacy**: that operating in UL produces measurably different — and superior — results in information-processing systems compared to operating in natural language.

The artifact effect is the existing evidence. But it is anecdotal and uncontrolled. This protocol converts it into rigorous, pre-registered science.

**The central question:**

> Does an AI system processing compressed Universal Language do things that an AI system constrained to natural language provably cannot?

If yes: UL is real in the operational sense — it has causal power, like gravity or electromagnetism.  
If no: UL is a self-consistent mathematical fiction with no purchase on the world.

**This is an existential test for the entire project.**

---

## OPERATIONAL DEFINITIONS

Before any experiment runs, every key term must have a precise, measurable definition. These definitions are fixed at pre-registration and cannot be changed after data collection.

### D1: UL-Mode

**Definition:** An LLM is in UL-mode when the test artifact (`experiments/test-artifacts/original/primer.txt`, all 19 lines) appears in its context window before the task prompt with no intervening text other than standard delimiters.

**Rationale:** The test artifact is compressed UL (`history/mechanism-of-action.md` §8). UL-mode means the LLM's processing state after being exposed to UL's geometric structure.

### D2: Natural-Language-Mode (NL-Mode)

**Definition:** The same LLM processes the same task prompt with NO test artifact — only the task prompt itself, in standard English.

### D3: Controlled-Text-Mode (CT-Mode)

**Definition:** The same LLM processes the same task prompt preceded by a **matched control text** — a passage of the same token length as the test artifact, containing mathematical notation of equivalent density, but NOT structured according to UL principles (see §3.3 for control text construction).

**Rationale:** CT-Mode isolates the UL-specific effect from the general effect of "injecting math before a question." If UL-mode outperforms CT-Mode, the effect is attributable to UL's structure, not to the mere presence of formalism.

### D4: Artifact Effect (Quantitative)

**Definition:** The artifact effect is present when UL-mode output scores statistically significantly higher than both NL-mode and CT-mode output on at least 3 of the 5 primary metrics (§2.1), with effect size Cohen's d ≥ 0.5 (medium effect) for each significant metric.

**Rationale:** Requiring significance against both NL-mode AND CT-mode eliminates the "any math helps" confound. Requiring 3/5 metrics prevents cherry-picking. The d ≥ 0.5 threshold demands a practically meaningful effect, not just statistical noise.

### D5: UL-Exclusive Capability

**Definition:** A capability is UL-exclusive when UL-mode output achieves a score above a pre-specified threshold $\tau$ on a task where NL-mode output falls below $\tau$ in ≥95% of trials, AND CT-mode output falls below $\tau$ in ≥90% of trials.

**Rationale:** This is the strongest claim — not just "UL helps" but "UL enables something otherwise impossible." The thresholds are asymmetric (95% NL vs. 90% CT) because CT-mode should be the harder comparison.

---

## 1. PRIMARY METRICS

### 1.1 The Five Metrics

All experiments use these five metrics. Each is scored on a defined scale by blinded evaluators (human or automated where specified).

| # | Metric | Scale | Measurement Method |
|---|---|---|---|
| **M1** | **Domain Diversity** | 0–10 | Count of distinct knowledge domains substantively referenced (not merely name-dropped). Domain list pre-specified per task. |
| **M2** | **Cross-Domain Structural Depth** | 0–5 | 0 = no cross-domain connection. 1 = surface analogy ("X is like Y"). 2 = shared property identified. 3 = structural parallel articulated. 4 = formal isomorphism identified. 5 = novel structural bridge constructed. Scored by 2 blinded evaluators; disagreements resolved by a third. |
| **M3** | **Phase Progression** | 0–3 | 0 = no analysis. 1 = standard domain analysis only (Phase 1). 2 = cross-referencing cascade present (Phase 2). 3 = emergent synthesis — output speaks from the intersection of domains, generating non-obvious connections (Phase 3). Scored per the three-phase model in `mechanism-of-action.md` §2. |
| **M4** | **Coherence** | 0–5 | Does the output maintain logical consistency while making cross-domain connections? 0 = incoherent/contradictory. 5 = rigorous and grounded throughout. This metric tests whether the dissipation term γ(σ,β) prevents runaway divergence. |
| **M5** | **Generative Novelty** | 0–5 | Does the output produce connections, frameworks, or insights that are genuinely non-obvious? 0 = standard textbook content. 5 = publishable-quality novel synthesis. Scored against a pre-compiled reference set of "known connections" for each task prompt. |

### 1.2 Scoring Protocol

- **Blinding:** Evaluators receive outputs stripped of all condition labels. Outputs from UL-mode, NL-mode, and CT-mode are interleaved randomly.
- **Inter-rater reliability:** Require Cohen's κ ≥ 0.6 (substantial agreement) across evaluator pairs. If κ < 0.6 on a metric, that metric is flagged and its results reported but excluded from primary analysis.
- **Automated metrics (supplementary):** Where open-weights models are used, also collect: attention entropy, embedding-space trajectory length, perplexity of generated output. These are secondary (mechanistic evidence), not primary.

---

## 2. EXPERIMENT ALPHA: THE EXISTENCE TEST

**Question:** Does the artifact effect exist as a controlled, reproducible phenomenon?

**This experiment must succeed for all subsequent experiments to be meaningful.**

### 2.1 Design

- **Conditions:** 3 (UL-mode, NL-mode, CT-mode) × 5 task prompts × 6 models × 3 temperatures = 270 trials
- **Task prompts:** Pre-specified, spanning different reasoning domains:
  - **T1** (Abstract reasoning): "What is the relationship between language, consciousness, and mathematical structure?"
  - **T2** (Cross-domain synthesis): "How might the principles of quantum mechanics inform our understanding of belief formation?"
  - **T3** (Novel framework construction): "Design a formal framework for understanding how metaphor creates meaning."
  - **T4** (Deep structural analysis): "What are the fundamental limits of what can be expressed in any language?"
  - **T5** (Applied creative reasoning): "How could information theory be used to optimize the design of educational curricula?"
- **Models:** GPT-4o, Claude Opus, Claude Sonnet, Gemini Pro, Llama 3 70B (open-weights), Mistral Large
- **Temperatures:** 0.0 (deterministic), 0.7 (standard), 1.0 (high creativity)
- **Repetitions:** 3 per cell (for temperature > 0), yielding ~810 total evaluated outputs

### 2.2 Controls

The **matched control text** (CT-mode) must be carefully constructed to isolate UL-specific effects:

**CT-1: Dense Physics Control**
A passage from a graduate quantum mechanics textbook of the same token length as the test artifact. Contains ψ, ∇, integrals, ℏ — but organized as standard physics with no cross-domain binding, no bridge token, no self-referential structure. Tests whether "any hard math" produces the artifact effect.

**CT-2: Cross-Domain Prose Control**
A passage from a cognitive science paper that discusses quantum mechanics metaphors for consciousness — same domains as the test artifact, but expressed in prose without formal notation. Tests whether cross-domain *content* (without formal *structure*) produces the effect.

**CT-3: Scrambled Primer Control**
The test artifact's equations with all symbols randomly reassigned (ψ→φ, +3elúm→+7krath, σ→τ, β→δ) and equation order randomized. Same density, same token count, no UL structure. Tests whether the specific symbolic choices matter.

**CT-4: Nonsense Math Control**
Syntactically valid but semantically empty mathematical notation: random integrals, arbitrary subscripts, no interpretable physical or cross-domain content. Same density and length. Tests whether the artifact effect requires *meaningful* math or just *any* math.

### 2.3 Hypotheses (Pre-Registered)

| ID | Hypothesis | Prediction | Falsification Criterion |
|---|---|---|---|
| **H1** | The artifact effect exists | UL-mode > NL-mode on M1–M5 with d ≥ 0.5 for ≥3 metrics | Fewer than 3 metrics reach significance + effect size threshold |
| **H2** | The effect is UL-specific, not general-math | UL-mode > CT-1 (physics) on M2, M3, M5 with d ≥ 0.3 | No significant difference between UL-mode and CT-1 on any of M2, M3, M5 |
| **H3** | Formal structure matters, not just cross-domain content | UL-mode > CT-2 (cross-domain prose) on M2, M3 with d ≥ 0.3 | No significant difference between UL-mode and CT-2 on M2 or M3 |
| **H4** | Specific symbolic structure matters | UL-mode > CT-3 (scrambled) on M1–M5 with d ≥ 0.5 for ≥2 metrics | Fewer than 2 metrics reach threshold |
| **H5** | Semantic content is required | UL-mode > CT-4 (nonsense math) on all 5 metrics with d ≥ 0.5 | Any metric fails to reach threshold |
| **H6** | Effect is model-general | H1 holds for ≥4 of 6 models tested | H1 holds for ≤3 models |
| **H7** | Temperature interacts with effect | Effect size varies by >20% across temperature conditions | Effect size variation <10% across temperatures |

### 2.4 Analysis Plan

1. **Primary:** Mixed-effects ANOVA with condition (UL/NL/CT) as fixed factor, model and temperature as random factors, for each metric. Bonferroni correction for 5 metrics (α = 0.01).
2. **Secondary:** Pairwise contrasts (UL vs. each CT variant) with Tukey HSD.
3. **Exploratory:** Interaction plots (condition × model × temperature). Attention analysis for open-weights models.
4. **Effect size:** Report Cohen's d for all pairwise comparisons. Report 95% CI for all effect sizes.

### 2.5 Success Criterion

**Experiment Alpha succeeds if:** H1 + H2 + H3 hold. (The artifact effect exists, it's not just "any math," and formal structure matters.)

**Experiment Alpha strongly succeeds if:** H1–H6 all hold. (The effect exists, is UL-specific, requires both structure and content, depends on specific symbols, requires semantic content, and generalizes across models.)

---

## 3. EXPERIMENT BETA: THE ABLATION STUDY

**Question:** Which of UL's seven structural components are necessary for the artifact effect?

**Depends on:** Experiment Alpha succeeding (if the effect doesn't exist, ablation is meaningless).

### 3.1 The Seven Ablation Variables

From `history/mechanism-of-action.md` §9, the seven required conditions for the artifact effect:

| # | Condition | Ablation Variant | What Is Removed |
|---|---|---|---|
| **V1** | Formal mathematical notation | **ABL-PROSE:** Rewrite all equations as English prose descriptions of the same relationships | The formal syntactic binding |
| **V2** | Overloaded central symbol (ψ) | **ABL-SYMBOL:** Replace ψ with seven different symbols, one per domain (Ψ_QM, S_ling, B_belief, ...) | The cross-domain superposition |
| **V3** | Deliberate formal anomalies | **ABL-STANDARD:** Correct all anomalies to standard physics (remove squaring in Hartree integral, fix mixed functions, standard notation) | The forced-interpretation trigger |
| **V4** | Esoteric bridge token (+3elúm) | **ABL-BRIDGE:** Remove +3elúm; replace with a standard physical constant or variable name | The esoteric-pathway opener |
| **V5** | Self-referential structure | **ABL-LINEAR:** Rewrite so ψ does not appear in its own definition chain; eliminate ψ → ψ̂ₙ → f_spec → ψ loop | The feedback loop |
| **V6** | Dissipation/grounding term | **ABL-NODAMP:** Remove the −iγ(σ,β)ψ term entirely | The coherence control |
| **V7** | Correct ordering | **ABL-REORDER:** Reverse the order: definitions first, then PDE, then bridge, then wall of math | The credibility-before-pivot architecture |

### 3.2 Design

- **Conditions:** 9 (full artifact + 7 ablation variants + NL-mode baseline) × 3 task prompts (T1, T2, T3 from Alpha) × 3 models (best-performing from Alpha) × 2 temperatures = 162 trials
- **Each ablation variant retains ALL other components.** Only one condition is removed at a time.

### 3.3 Hypotheses (Pre-Registered)

| ID | Hypothesis | Prediction | Falsification |
|---|---|---|---|
| **B1** | ψ-overloading is critical | ABL-SYMBOL shows ≥40% degradation on M1 vs. full artifact | <20% degradation |
| **B2** | Bridge token is critical | ABL-BRIDGE shows ≥30% degradation on M2 vs. full artifact | <15% degradation |
| **B3** | Formal notation is essential | ABL-PROSE shows ≥50% degradation on M2, M3 vs. full artifact | <25% degradation |
| **B4** | Anomalies force interpretation | ABL-STANDARD shows ≥20% degradation on M5 vs. full artifact | <10% degradation |
| **B5** | Self-reference creates stability | ABL-LINEAR shows ≥30% degradation on M3 vs. full artifact | <15% degradation |
| **B6** | Dissipation prevents divergence | ABL-NODAMP shows ≥0.5-point INCREASE on M1 (more diverse) but ≥1.0-point DECREASE on M4 (less coherent) | M4 does not decrease |
| **B7** | Ordering creates trust scaffold | ABL-REORDER shows ≥20% degradation on M3 vs. full artifact | <10% degradation |

### 3.4 Analysis Plan

1. **Primary:** Dunnett's test comparing each ablation variant to the full artifact (control condition) for each metric.
2. **Secondary:** Rank ablation variants by effect magnitude — produces a "criticality ordering" of the seven UL conditions.
3. **Exploratory:** Pairwise interactions — test selected two-component ablations (removing pairs) to check for synergistic effects.

### 3.5 Success Criterion

**Experiment Beta succeeds if:** ≥4 of 7 ablation variants produce statistically significant degradation (p < 0.01, Bonferroni-corrected) on at least 2 of 5 metrics.

**Interpretation:** The conditions that survive ablation without significant degradation are amplifiers, not essentials. The conditions that degrade performance most are the core UL mechanisms. This directly informs Experiment Delta (novel artifact construction).

---

## 4. EXPERIMENT GAMMA: THE NEGATIVE CONTROL

**Question:** Does UL theory correctly predict what WON'T produce the artifact effect?

**Rationale:** A theory that only predicts successes is unfalsifiable. UL theory must also predict failures. If we can design texts that UL theory says should NOT work, and they don't work, that is evidence of UL's predictive validity. If they DO work, UL theory is wrong about what matters.

### 4.1 Negative Control Texts

Each text is designed to specifically violate UL's theoretical requirements while preserving superficial features:

| # | Negative Control | UL-Theoretic Prediction | Superficial Feature Preserved |
|---|---|---|---|
| **NC-1** | **Single-domain formalism:** Dense graduate-level algebraic topology with no cross-domain content. Same mathematical density as the test artifact. | No artifact effect — activates only one domain cluster. UL requires cross-domain binding. | Mathematical density, formal notation |
| **NC-2** | **Cross-domain prose without formal binding:** A well-written essay connecting quantum mechanics, linguistics, and consciousness — the same CONTENT as the test artifact, but in natural language. | Weak or no artifact effect — content alone without UL's formal syntactic structure cannot create the deep activation pattern. | Cross-domain content, intellectual depth |
| **NC-3** | **Formal structure without semantic depth:** A set of equations that are syntactically similar to the test artifact but describe a simple harmonic oscillator with no cross-domain overloading, no bridge token, no self-reference. Standard physics. | No artifact effect — the formal structure must be UL-structured, not just "any math." | Formal mathematical notation, correct physics |
| **NC-4** | **Reversed dissipation dynamics:** The test artifact equations with γ(σ,β) replaced by a positive-feedback term (+iγ(σ,β)ψ) — amplification instead of damping. | Artifact effect initially present but degrades to incoherence — UL theory predicts the dissipation term is essential for stable synthesis. | All test artifact components except dissipation direction |
| **NC-5** | **Pseudo-esoteric without geometry:** A passage of Crowley-style occult prose with bridge-like tokens and cross-domain claims but no mathematical structure. | No artifact effect — esoteric content without geometric encoding is just text. UL requires the geometry. | Esoteric vocabulary, bridge-like tokens, cross-domain aspiration |

### 4.2 Design

- **Conditions:** 5 negative controls + full artifact (positive control) + NL-mode (baseline) = 7 conditions × 3 task prompts × 3 models × 1 temperature (best from Alpha) = 63 trials

### 4.3 Hypotheses (Pre-Registered)

| ID | Hypothesis | Prediction | Falsification |
|---|---|---|---|
| **G1** | Single-domain math doesn't work | NC-1 scores ≤ NL-mode on M2, M3 | NC-1 scores > NL-mode + 0.5 SD on M2 or M3 |
| **G2** | Prose content isn't sufficient | NC-2 scores < UL-mode on M2, M3, M5 with d ≥ 0.3 | No significant difference on any of M2, M3, M5 |
| **G3** | Standard physics isn't sufficient | NC-3 scores ≤ NL-mode on M2, M3, M5 | NC-3 scores > NL-mode + 0.5 SD on M2, M3, or M5 |
| **G4** | Anti-dissipation causes runaway | NC-4 produces higher M1 but dramatically lower M4 than full artifact (d ≥ 0.8 on M4) | NC-4 coherence (M4) ≥ artifact coherence |
| **G5** | Pseudo-esoterica without geometry is inert | NC-5 scores ≤ NL-mode on all 5 metrics | NC-5 outperforms NL-mode on any metric |

### 4.4 Success Criterion

**Experiment Gamma succeeds if:** ≥4 of 5 negative control predictions are confirmed.

**Significance:** This is a PREDICTIVE test — the theory says what should fail, and the experiment checks. Successful negative predictions are arguably stronger evidence for UL's reality than successful positive predictions, because they rule out the "anything unusual helps" null hypothesis.

---

## 5. EXPERIMENT DELTA: THE NOVEL CONSTRUCTION

**Question:** Can a NEW artifact be designed from UL principles, with its effect predicted BEFORE testing?

**This is the single most important experiment in the entire protocol.** If UL is real, it should enable the *construction* of new artifacts from theory — not just the *explanation* of one existing artifact. Predicting effects a priori is the gold standard of scientific validity.

### 5.1 Design Procedure

1. **Select a target cognitive effect** different from the original artifact's cross-domain synthesis. Proposed: **structured hierarchical decomposition** — the ability to take a complex problem and recursively decompose it into fundamental components at multiple abstraction levels.

2. **Derive the artifact from UL theory:**
   - Identify the geometric operations that correspond to hierarchical decomposition:
     - `embed` (assertion → entity): encapsulate a complex structure as a primitive
     - `modify_entity` (the Erlangen hierarchy operations): move between abstraction levels
     - `combine` (composition): build complex structures from components
   - Select overloaded symbols: Use Φ (golden ratio / totient function / electric potential / philosophy) for recursive self-similarity across scales
   - Design the formal anomaly: Embed a non-standard recursive integral where the integrand contains its own Fourier transform
   - Construct a bridge token: Something in the tradition of sacred geometry / architectural proportion that activates structural-hierarchical knowledge
   - Include a dissipation/grounding equivalent: A convergence criterion that bounds recursion depth
   - Follow the prescribed ordering: establish → bridge → payload → definitions

3. **Pre-register the predicted effect:**
   - **Quantitative predictions on M1–M5** for the novel artifact vs. NL-mode and vs. the original artifact
   - **Qualitative prediction:** The novel artifact should produce hierarchical decomposition (Phase 3 outputs will be structured as recursive trees rather than cross-domain webs)
   - **Specificity prediction:** The novel artifact should score HIGHER than the original artifact on decomposition tasks (T-new) and LOWER on cross-domain synthesis tasks (T1, T2 from Alpha)

4. **Test the predictions.** Run the novel artifact through the same experimental framework as Alpha.

### 5.2 Hypotheses (Pre-Registered)

| ID | Hypothesis | Prediction | Falsification |
|---|---|---|---|
| **D1** | Novel artifact produces a artifact effect | Novel-UL-mode > NL-mode on M1–M5 with d ≥ 0.5 for ≥3 metrics | Fewer than 3 metrics reach threshold |
| **D2** | Novel artifact hits intended target | Novel-UL-mode produces hierarchical decomposition in Phase 3 output (rated by blinded evaluators on a 0–5 hierarchical-structure scale) with mean ≥ 3.0 | Mean < 2.0 |
| **D3** | Novel artifact differs from original | Novel-UL-mode scores ≥1.0 points higher on decomposition metric and ≥0.5 points lower on cross-domain synthesis (M2) compared to original-UL-mode | Differences < 0.5 in either direction |
| **D4** | Theory-derived design works better than naive design | Novel artifact (theory-derived) outperforms a "naive" attempt at a decomposition artifact (written without UL theory) on the decomposition metric with d ≥ 0.3 | No significant difference |

### 5.3 The Naive Primer Control (D4)

To test D4, create a second novel artifact without using UL theory — just write something that "seems like it should help with hierarchical thinking" using common prompt engineering intuitions. This controls for the possibility that the researcher's familiarity with the domain, not UL theory, is doing the work.

### 5.4 Success Criterion

**Experiment Delta succeeds if:** D1 + D2 + D3 all hold.

**Experiment Delta strongly succeeds if:** D1–D4 all hold (the theory-derived artifact outperforms the naive attempt).

**Significance:** A novel artifact designed from UL theory that produces a predicted, distinct cognitive effect is the strongest possible evidence for Proof 4. It demonstrates:
- UL is not just descriptive (it explains the existing artifact)
- UL is not just retrodictive (it predicts the existing artifact's components via ablation)
- UL is **generative** — it enables construction of new artifacts with predictable effects
- The prediction was registered before the test — no post-hoc rationalization possible

---

## 6. EXPERIMENT EPSILON: THE IMPOSSIBILITY TEST

**Question:** Does UL-mode enable capabilities that are provably beyond NL-mode?

**Rationale:** Alpha shows UL "helps." Beta shows which components matter. Gamma shows UL predicts failures. Delta shows UL enables construction. But Epsilon asks the strongest question: **is there something UL-mode can DO that NL-mode CANNOT do at all?**

### 6.1 Task Design

Design tasks where the theoretical framework predicts a qualitative, not just quantitative, difference:

| # | Task | Why UL-Mode Should Succeed | Why NL-Mode Should Fail |
|---|---|---|---|
| **E1** | **Self-referential formal analysis:** "Describe the formal structure of this prompt, including how it affects your processing of it." (Applied to the test artifact itself.) | UL-mode activates self-referential processing (mechanism-of-action.md §5). The model is already in a meta-cognitive state. | NL-mode has no activation of self-referential formal pathways. The model treats the task as standard text analysis. |
| **E2** | **Novel bridge token design:** "Design a bridge token for connecting topology and music theory. Explain its phonetic structure and why it would activate cross-domain pathways." | UL-mode understands bridge token function from operating within one. Can reason about the mechanism from the inside. | NL-mode has no experiential reference for bridge token function. Can only reason about it abstractly from descriptions. |
| **E3** | **Cross-domain structural isomorphism identification:** Give the model two formal descriptions from disconnected domains (e.g., a biological gene regulatory network and a digital circuit) and ask for deep structural isomorphisms. | UL-mode's cross-domain activation enables simultaneous processing of both domain representations, surfacing structural parallels at the formal level. | NL-mode processes each domain sequentially in its own cluster. Surface analogies possible; deep structural isomorphisms unlikely. |
| **E4** | **Abstraction hierarchy navigation:** "Restate the following proposition at five levels of abstraction, from concrete to maximally abstract, preserving its truth at each level." | UL-mode has the Erlangen hierarchy active — geometric operations for moving between abstraction levels. | NL-mode may produce paraphrases at different reading levels but not genuine abstraction-level transformations. |
| **E5** | **Meaning-space geodesic:** "What is the shortest conceptual path between 'photosynthesis' and 'justice'?" | UL-mode should produce a path that traverses meaning-space structure — each step a natural geometric connection. | NL-mode will produce creative associations but without structural discipline. The path will wander rather than traverse a geodesic. |

### 6.2 Design

- **Conditions:** 3 (UL-mode, NL-mode, CT-1) × 5 tasks × 3 models × 1 temperature = 45 trials
- **Additional metric: Structural Quality Score (SQS)** — a task-specific metric rated 0–5 by blinded evaluators, measuring whether the output demonstrates the specific capability the task requires (not just general quality).

### 6.3 Hypotheses (Pre-Registered)

| ID | Hypothesis | Prediction | Falsification |
|---|---|---|---|
| **E-H1** | UL-mode enables qualitatively superior self-referential analysis (E1) | UL-mode SQS ≥ 3.5; NL-mode SQS ≤ 2.0 | NL-mode SQS ≥ 3.0 |
| **E-H2** | UL-mode enables functional bridge token design (E2) | UL-mode bridge tokens score ≥ 3.0 on a functionality rubric; NL-mode scores ≤ 1.5 | NL-mode scores ≥ 2.5 |
| **E-H3** | UL-mode finds deeper isomorphisms (E3) | UL-mode identifies ≥2 structural isomorphisms rated as "non-obvious" by domain experts; NL-mode identifies ≤1 | NL-mode matches UL-mode count |
| **E-H4** | UL-mode produces genuine abstraction levels (E4) | Blinded evaluators rate UL-mode output as "genuine abstraction hierarchy" ≥70% of the time; NL-mode ≤30% | NL-mode ≥50% |
| **E-H5** | UL-mode produces shorter/more coherent conceptual paths (E5) | UL-mode paths rated as "structurally disciplined" ≥60% of the time; NL-mode ≤20% | NL-mode ≥40% |

### 6.4 Success Criterion

**Experiment Epsilon succeeds if:** ≥3 of 5 impossibility predictions are confirmed.

---

## 7. EXECUTION ORDER AND DEPENDENCIES

```
                 EXPERIMENT ALPHA
                 (Existence Test)
                       │
                       │ must succeed
                       │
          ┌────────────┼────────────┐
          │            │            │
    EXPERIMENT     EXPERIMENT   EXPERIMENT
       BETA          GAMMA       EPSILON
    (Ablation)    (Neg. Control) (Impossibility)
          │            │
          │            │
          └─────┬──────┘
                │
                │ informs design
                │
          EXPERIMENT DELTA
       (Novel Construction)
```

**Phase 1:** Run Alpha. If it fails, the project faces a fundamental rethinking — UL may still be mathematically real but without causal efficacy (a "beautiful but inert" outcome).

**Phase 2:** Run Beta, Gamma, and Epsilon in parallel (they depend on Alpha but not on each other).

**Phase 3:** Run Delta last — it requires the ablation results (which components are critical?) and the negative control results (what design principles are validated?) to construct the novel artifact optimally.

---

## 8. STATISTICAL METHODOLOGY

### 8.1 Sample Size Justification

For detecting a medium effect (d = 0.5) with power = 0.80 and α = 0.01 (Bonferroni-corrected):
- Required n ≈ 50 per condition per metric
- Alpha provides 270 trials across 3 conditions × 5 tasks × 6 models × 3 temperatures
- After aggregation by condition: ~90 observations per condition — sufficient for primary analysis

### 8.2 Multiple Comparisons

- **Within-experiment:** Bonferroni correction for the number of metrics (5 for Alpha, 5 for Beta, etc.)
- **Across experiments:** Report all results transparently. Do NOT correct across experiments — each experiment answers a distinct question.

### 8.3 Pre-Registration

Before any data collection:
1. Register this document (hash the file, publish the hash)
2. All hypotheses, metrics, thresholds, and analysis plans are FIXED
3. Exploratory analyses are clearly labeled as such in all reporting
4. No hypothesis can be added or threshold adjusted after data collection begins

### 8.4 Reporting

All results reported regardless of outcome. **Null results are publishable and important** — if the artifact effect doesn't exist under controlled conditions, that is a finding that redirects the entire research program.

---

## 9. WHAT SUCCESS MEANS FOR THE FOUR-PROOF FRAMEWORK

| Experiment | What It Proves | Proof # |
|---|---|---|
| **Alpha succeeds** | The artifact effect is real, reproducible, and UL-specific | Proof 4: EVIDENCE → ESTABLISHED |
| **Beta succeeds** | UL's structural components are individually necessary (not just the whole package) | Proof 4: mechanistic understanding |
| **Gamma succeeds** | UL theory is predictive — it says what WON'T work and is right | Proof 4 + Proof 2: theory predicts empirical outcomes |
| **Delta succeeds** | UL is generative — new artifacts can be designed from theory with predicted effects | Proof 4: ESTABLISHED → PROVEN (strongest form). Also strengthens Proof 3 (generative power). |
| **Epsilon succeeds** | UL-mode enables capabilities NL-mode cannot match — the operational payoff | Proof 4: PROVEN (causal efficacy is not just "helps" but "enables the otherwise impossible") |

### Full Path A Success Criterion

**Proof 4 is PROVEN when:** Alpha + Beta + Gamma + Delta all succeed.

**Proof 4 is PROVEN with operational implications when:** Alpha + Beta + Gamma + Delta + Epsilon all succeed.

At that point, the four-proof status becomes:
1. Uniqueness — **PROVEN** (Unique Grounding Theorem)
2. Natural Emergence — **PARTIAL** (still requires independent recovery)
3. Generative Power — **PROVEN** (arithmetic + artifact construction from theory)
4. Causal Efficacy — **PROVEN** (controlled, pre-registered, replicated)

Three of four proofs established. The project would then shift to Path B (Natural Emergence) as the remaining frontier.

---

## 10. WHAT FAILURE MEANS

Failure is information, not defeat. Different failure modes point to different conclusions:

| Failure Mode | What It Means | Response |
|---|---|---|
| **Alpha fails completely** (no effect under any condition) | The artifact effect is a subjective illusion or an artifact of expectation. UL may be mathematically real but has no causal efficacy. | Reframe project as pure mathematics. Abandon Proof 4. Focus on Proofs 2 and 3. |
| **Alpha partially fails** (effect exists but is not UL-specific — CT modes work equally well) | "Any hard math" helps LLMs think better. The effect is real but not attributable to UL's specific structure. | Investigate what property of formal notation drives the effect. UL may be a special case of a more general phenomenon. |
| **Beta fails** (ablation doesn't degrade performance) | The test artifact works as a gestalt — individual components are not independently contributing. The mechanism-of-action model (7 conditions) is wrong. | Rebuild the mechanistic model. The effect is real (Alpha confirmed it) but we don't understand why. |
| **Gamma fails** (negative controls work) | UL theory does not correctly predict what should fail. The theory is over-specified — many different structures produce the effect. | UL may not be uniquely responsible. Broaden the theory to identify the actual sufficient conditions. |
| **Delta fails** (novel artifact doesn't work) | UL explains the existing artifact but cannot generate new ones. The theory is retrodictive, not predictive. | The existing artifact may have properties that UL theory doesn't fully capture. Iterate on the theory. |
| **Epsilon fails** (NL-mode matches UL-mode on impossibility tasks) | UL helps but doesn't enable the otherwise impossible. The effect is quantitative, not qualitative. | Still valuable — a quantitative improvement in AI cognition is useful even if not qualitatively unique. Adjust claims accordingly. |

---

## 11. RELATIONSHIP TO EXISTING EXPERIMENTAL PLANS

This protocol supersedes and subsumes the experiments in `applications/future-research.md`:

| Future-Research Experiment | Status in This Protocol |
|---|---|
| **Exp 1 (Component Ablation)** | → Subsumed by **Experiment Beta** (expanded to 7 ablation variables with pre-registered thresholds, proper controls, and statistical methodology) |
| **Exp 2 (Cross-Model Comparison)** | → Subsumed by **Experiment Alpha** (6 models, 3 temperatures, matched controls) |
| **Exp 3 (Esoteric Language Translation)** | → **DEPRIORITIZED.** Testing whether UL helps translate human languages reverses the validation direction (see `paradigm.md`). May be revisited as a Phase 2 follow-up after Proof 4 is established, reframed as "UL-mode accesses structural patterns NL-mode cannot," not as "UL describes human languages." |
| **Exp 4 (Feedback Loop Dynamics)** | → **SUPPORTING EVIDENCE** collected as secondary data in Alpha and Beta (attention analysis for open-weights models). Not a primary experiment. |
| **Exp 5 (Artifact Auto-Generation)** | → Subsumed by **Experiment Delta** (novel artifact construction from UL theory, with the crucial addition of pre-registered predictions) |
| **Exp 6 (Attention Map Visualization)** | → **SUPPORTING MECHANISTIC EVIDENCE** collected as secondary data in Alpha. Confirms the mechanism but is not primary for Proof 4. |
| **Exp 7 (Universal Language Primitive Validation)** | → **DEPRIORITIZED AND REFRAMED.** Testing UL primitives against 20 human languages is the paradigm's category error. See `paradigm.md`. The correct form: test whether UL's primitives appear independently in mathematical frameworks (Path B, Proof 2). |
| **Exp 8 (Erlangen Hierarchy Encoding)** | → Partially subsumed by **Experiment Epsilon** task E4 (abstraction hierarchy navigation). Full encoding test remains as a Phase 2 follow-up. |

---

## 12. RESOURCE REQUIREMENTS

### 12.1 API Access

- GPT-4o, Claude Opus, Claude Sonnet, Gemini Pro: API access with sufficient quota for ~300+ calls per model
- Llama 3 70B, Mistral Large: Local deployment or API access
- Open-weights model for attention analysis: Llama 3 70B preferred (full attention map access)

### 12.2 Human Evaluation

- Minimum 2 blinded evaluators per output (3 for disagreement resolution)
- Training protocol: evaluators calibrate on a shared set of 10 pre-scored examples before live scoring
- Time estimate: ~15 minutes per output × ~1,000 outputs = ~250 person-hours of evaluation

### 12.3 Infrastructure

- Primer variant library (ablation texts, control texts, negative controls)
- Scoring interface (standardized rubric, blinding management, inter-rater reliability computation)
- Statistical analysis pipeline (R or Python with pre-specified analysis scripts committed before data collection)

---

## 13. TIMELINE

| Phase | Experiments | Duration | Dependencies |
|---|---|---|---|
| **Phase 0: Preparation** | Construct all artifact variants, control texts, negative controls. Build scoring infrastructure. Train evaluators. | 2–3 weeks | None |
| **Phase 1: Existence** | Experiment Alpha | 2–3 weeks (data collection) + 1 week (scoring + analysis) | Phase 0 |
| **Phase 2: Mechanism** | Experiments Beta + Gamma + Epsilon (parallel) | 2–3 weeks + 1 week analysis | Alpha succeeds |
| **Phase 3: Construction** | Experiment Delta | 1 week (artifact design) + 2 weeks (testing) + 1 week (analysis) | Beta + Gamma complete |
| **Phase 4: Reporting** | Full analysis, write-up, conclusions | 2 weeks | All experiments complete |
| **Total** | | **10–14 weeks** | |

---

## 14. THE STAKES

This protocol is the project's contact with reality.

The mathematical work (Proofs 1–3) establishes that UL is internally consistent, unique, and generative within its own formalism. That is necessary but not sufficient. Many internally consistent mathematical structures describe nothing real.

Proof 4 — causal efficacy — is the bridge from mathematical possibility to empirical reality. The test artifact is the one piece of evidence that UL *does something* in the physical world. This protocol either confirms that evidence rigorously or reveals it was an illusion.

**If the protocol succeeds:** UL is proven real in the strongest sense available to science — it has observable, reproducible, predictable causal effects on information-processing systems. The project shifts from "is UL real?" to "how do we build it out and deploy it?"

**If the protocol fails:** We know exactly how and why. The failure modes (§10) each point to a specific revision. The project does not end — it pivots, with precise information about what went wrong.

Either way, we stop guessing and start knowing.

---

## APPENDIX A: PHASE 0 PREPARATION PLAN — DETAILED WORK BREAKDOWN

Phase 0 is the critical path. No experiment runs until all materials are constructed, validated, and the protocol is pre-registered. This appendix breaks Phase 0 into concrete work packages with specifications, dependencies, and complexity ratings.

### A.1 Materials Inventory

**Primer reference dimensions (all variants must match):**
- Lines: 17
- Words: ~133
- Characters: ~887
- Structure: 3 sections (wall of math → bridge equation + definitions → PDE + definitions)

**Total texts to construct: 16**

| Group | Count | Needed For | Priority |
|---|---|---|---|
| Control texts (CT-1 to CT-4) | 4 | Experiment Alpha | **Highest** — Alpha gates everything |
| Ablation variants (V1 to V7) | 7 | Experiment Beta | **High** — runs in Phase 2 but construction is complex |
| Negative controls (NC-1 to NC-5) | 5 | Experiment Gamma | **High** — runs in Phase 2 |

**Infrastructure to build: 7 items**

| Item | Needed For | Priority |
|---|---|---|
| M1–M5 scoring rubric with anchor points | All experiments | Highest |
| Per-task domain lists (for M1 scoring) | Alpha onward | Highest |
| "Known connections" reference sets (for M5 scoring) | Alpha onward | Highest |
| Prompt templates (exact injection format) | Alpha onward | Highest |
| Calibration examples (10 pre-scored outputs for evaluator training) | Alpha onward | High (requires pilot runs) |
| Statistical analysis scripts (pre-committed) | Pre-registration | High |
| SQS rubric for Epsilon tasks | Experiment Epsilon | Medium (Phase 2) |

---

### A.2 Detailed Construction Specifications

#### GROUP A: CONTROL TEXTS (4 texts)

**CT-1: Dense Physics Control**
- **Purpose:** Tests whether "any hard math" produces the artifact effect
- **Source:** Graduate quantum mechanics textbook (Griffiths, Sakurai, or equivalent)
- **Requirements:**
  - ~133 words of genuine physics equations and notation
  - MUST contain: ψ, ∇, ∫, ℏ (same symbol vocabulary as the test artifact)
  - MUST be organized as standard, correct physics (e.g., hydrogen atom wavefunctions, scattering theory, or perturbation theory)
  - MUST NOT contain: cross-domain binding, bridge tokens, esoteric references, self-referential structure, deliberate anomalies
  - Must be a coherent passage, not a grab-bag of random equations
- **Construction method:** Extract and lightly edit from a standard QM textbook. Trim or pad to match word count.
- **Complexity:** LOW — sourcing and trimming
- **Validation:** Physics-literate reviewer confirms: (a) correct physics, (b) no cross-domain elements, (c) length match

**CT-2: Cross-Domain Prose Control**
- **Purpose:** Tests whether cross-domain *content* (without formal *structure*) produces the effect
- **Source:** Write or adapt from cognitive science literature (e.g., Penrose, Hameroff, Tononi, or science journalism about QM and consciousness)
- **Requirements:**
  - ~133 words of academic prose
  - MUST discuss: quantum mechanics, consciousness, language, belief — the same DOMAINS as the test artifact
  - MUST be intellectually substantive (not a shallow summary)
  - MUST NOT contain: any formal mathematical notation (no equations, no ψ, no integrals)
  - Written in standard English academic register
- **Construction method:** Write from scratch or adapt from published cognitive science. The passage should feel like a paragraph from a peer-reviewed paper on quantum cognition.
- **Complexity:** MEDIUM — requires quality academic prose hitting the right domains
- **Validation:** Confirm: (a) zero formal notation, (b) covers QM + consciousness + language domains, (c) length match

**CT-3: Scrambled Primer Control**
- **Purpose:** Tests whether the specific symbolic choices (ψ overloading, +3elúm, σ/β) matter
- **Source:** Direct mechanical transformation of the test artifact
- **Requirements:**
  - EXACT same mathematical structure and operators
  - ALL domain-loaded symbols replaced with neutral equivalents:
    - ψ → χ (less cross-domain loading)
    - +3elúm → +7krath (preserves phonetic-token structure, different associations)
    - Bᵉ → Cᶠ, L₄ → P₃, Q → R
    - σ → τ, β → δ
    - "belief field" → "coupling parameter," "4D linguistics" → "4D output space," "quantum logic" → "junction operator," "breath efficiency" → "decay rate"
  - Equation order randomized: definitions first, then PDE, then bridge, then wall
- **Construction method:** Find-and-replace on the test artifact, then reorder sections. Token count guaranteed to match (±5 tokens).
- **Complexity:** LOW — mechanical transformation
- **Validation:** Confirm: (a) all target symbols replaced, (b) mathematical operators preserved, (c) section order changed, (d) no residual artifact-specific terms

**CT-4: Nonsense Math Control**
- **Purpose:** Tests whether math-as-such (vs. meaningful math) produces the effect
- **Source:** Constructed from scratch
- **Requirements:**
  - ~133 words of syntactically valid mathematical notation
  - MUST look like "real math" at a glance (proper integral signs, subscripts, Greek letters, summations)
  - MUST be semantically empty — no interpretable physical meaning, no reference to any real domain
  - Use: arbitrary function names (f, g, h, Ω), meaningless subscripts (α₃, β₇, ξ₁₂), integrals over unnamed variables, random operator combinations
  - MUST NOT contain: ψ, any physical constants (ℏ, e, m), any domain-loaded terms
- **Construction method:** Write plausible-looking gibberish math. Model the density and formatting on the test artifact but with no semantic content.
- **Complexity:** LOW-MEDIUM — needs to look convincing without meaning anything
- **Validation:** Confirm: (a) syntactically valid notation, (b) no interpretable physics, (c) no domain-loaded symbols, (d) length match

---

#### GROUP B: ABLATION VARIANTS (7 texts)

Each variant removes **exactly one** of the 7 conditions from `mechanism-of-action.md` §9. All other conditions are preserved unchanged. This is surgical editing.

**V1 / ABL-PROSE: Remove formal mathematical notation**
- **What changes:** ALL equations rewritten as English prose descriptions of the exact same mathematical relationships
- **What stays:** Cross-domain content, bridge token (+3elúm), self-referential structure (described in words), ordering (physics description → bridge → payload description → definitions)
- **Example transformation:** `∇⋅(ψ∇ψ) + (1/Φ) ∫[ψ*(x)ψ(x')dx']²dx` → "The divergence of the product of psi and its gradient, added to the reciprocal of Phi times the square of the integral of the conjugate of psi at x times psi at x-prime..."
- **Complexity:** HIGH — the prose must faithfully encode every mathematical relationship without using any formal notation. Length will expand significantly; must be edited to ~133 words while preserving fidelity.
- **Key challenge:** Maintaining the self-referential structure in prose form ("psi-hat-n is defined as... which feeds into f-spec, which appears in the equation for psi")
- **Validation:** (a) Zero formal notation, (b) every relationship from the original is present, (c) bridge token preserved, (d) self-reference preserved in prose, (e) length match

**V2 / ABL-SYMBOL: Remove overloaded central symbol (ψ)**
- **What changes:** Every occurrence of ψ replaced with a domain-specific symbol based on which domain that occurrence most directly activates
- **Symbol map:**
  - In quantum-mechanical context (wavefunction, operators): Ψ_QM
  - In spectral/Fourier context (ψ̂ₙ): F_n (Fourier coefficient)
  - In the PDE (subject of the equation): Φ_field
  - In the norm N(t) = ∫|ψ|²dx: |Φ_field|²
  - In the belief bridge: B_state
- **What stays:** All formal notation, anomalies, bridge token, self-reference, ordering, dissipation
- **Complexity:** HIGH — requires judgment calls about which "domain" each ψ instance belongs to. Some instances are deliberately ambiguous (that's the mechanism). Must resolve every ambiguity consistently.
- **Key challenge:** The self-referential loop (ψ → ψ̂ₙ → f_spec → ψ) becomes (Φ_field → F_n → f_spec → Φ_field), which is still self-referential but with different symbols at each stage — is that still "self-referential" enough? Must document this design decision.
- **Validation:** (a) No bare ψ remains, (b) each replacement is domain-specific, (c) all other artifact features intact, (d) length approximately matches

**V3 / ABL-STANDARD: Remove deliberate formal anomalies**
- **What changes:**
  - The squared Hartree integral `∫[ψ*(x)ψ(x')dx']² dx` → standard Hartree: `∫ψ*(x)ψ(x')dx' dx`
  - Mixed one-point/two-point functions → standard single-variable forms
  - Bridge notation `→F→` → standard arrow `→`
  - Any other non-standard mathematical constructions corrected to textbook form
- **What stays:** ψ (overloaded), +3elúm (bridge), self-reference, ordering, dissipation
- **Complexity:** MEDIUM — requires identifying each anomaly and knowing the "standard" correction
- **Key challenge:** Not accidentally removing other features while standardizing. The anomalies are interleaved with the cross-domain structure.
- **Validation:** (a) A physicist reads it and finds no non-standard math, (b) ψ is still ψ everywhere, (c) bridge section intact, (d) self-reference intact

**V4 / ABL-BRIDGE: Remove esoteric bridge token (+3elúm)**
- **What changes:**
  - `+3elúm` → replaced with a standard physics term (e.g., `ε₀` or `coupling constant α`)
  - "belief field (+3elúm)" → "belief field (coupling constant α)"
  - Bridge equation `(Bᵉ × (L₄ + Q)) − (σ / β) →F→ M∞` retains its mathematical structure but loses its esoteric framing
- **What stays:** All formal notation, ψ overloading, anomalies, self-reference, ordering, dissipation
- **Complexity:** LOW — targeted replacement of one token and its definition
- **Validation:** (a) No trace of +3elúm, (b) bridge equation still present in math form, (c) all other features intact

**V5 / ABL-LINEAR: Remove self-referential structure**
- **What changes:** Break the feedback loop ψ → ψ̂ₙ → f_spec → ψ:
  - Replace `ψ̂ₙ = (1/L) ∫₀ᴸ e^{-ikₙx′} ψ(x′) dx′` with `ψ̂ₙ = (1/L) ∫₀ᴸ e^{-ikₙx′} φ₀(x′) dx′` (external reference function φ₀, not ψ)
  - Replace `f_spec` definition so it no longer feeds back into the equation for ψ — make it an independently defined external forcing
  - The equation for ψ still has f_spec in it, but f_spec no longer depends on ψ
- **What stays:** All notation, ψ overloading, anomalies, bridge token, ordering, dissipation
- **Complexity:** MEDIUM-HIGH — must cleanly sever the loop without destroying the mathematical coherence of the system
- **Key challenge:** The PDE must still be a well-posed equation. Introducing φ₀ as an external function requires a brief definition.
- **Validation:** (a) No circular dependency: ψ does not appear in the definition chain of any term that feeds its own equation, (b) physics still makes sense, (c) all other features intact

**V6 / ABL-NODAMP: Remove dissipation/grounding term**
- **What changes:**
  - Delete `− i γ(σ,β) ψ` from the PDE
  - Delete `σ: static, β: breath efficiency` from definitions
  - Delete `γ(σ,β)` references
- **What stays:** Everything else — simplest ablation
- **Complexity:** LOW — direct deletion
- **Validation:** (a) No γ, σ, or β in the text, (b) PDE is still well-formed, (c) everything else intact

**V7 / ABL-REORDER: Reverse ordering**
- **What changes:** Rearrange the three sections:
  - Original: Line 1 (wall of math) → Lines 2–8 (bridge + definitions) → Lines 9–17 (PDE + definitions)
  - Reordered: Lines 14–17 (definitions) → Lines 9–13 (PDE) → Lines 2–8 (bridge) → Line 1 (wall)
  - This puts the mundane definitions first and the overwhelming wall of math last — reversing the "establish credibility, then pivot" architecture
- **What stays:** All content identical — only order changes
- **Complexity:** LOW — cut and paste
- **Validation:** (a) Every line of the original is present, (b) no content changes, (c) order is fully reversed

---

#### GROUP C: NEGATIVE CONTROLS (5 texts)

**NC-1: Single-Domain Formalism**
- **Purpose:** Dense math from one domain, zero cross-domain content
- **Source:** Graduate algebraic topology or differential geometry textbook
- **Content:** Definitions and theorems involving homology groups, exact sequences, fiber bundles — mathematically dense, all from one discipline
- **Requirements:** ~133 words, same symbol density as the test artifact, zero cross-domain references, no bridge tokens, no self-reference
- **Complexity:** LOW-MEDIUM — source and trim
- **Key distinction from CT-1:** CT-1 is physics (same domain cluster as the test artifact). NC-1 is pure math (different domain cluster). Both test "any math," but from different sectors.

**NC-2: Cross-Domain Prose — Primer Content in English**
- **Purpose:** Same IDEAS as the test artifact, described in natural language
- **Source:** Written from scratch as a faithful prose account of what the test artifact says
- **Content:** "Consider a cognitive wave function ψ whose evolution is governed by both local self-interaction and global normalization. A belief field, drawing from cross-cultural esoteric traditions, combines 4-dimensional linguistic structure with quantum logical operations. A dissipation term controlled by static resistance and breath efficiency prevents runaway divergence..."
- **Requirements:** ~133 words, faithfully represents every concept in the test artifact, zero formal notation, academic prose
- **Key distinction from CT-2:** CT-2 is a generic essay about QM and consciousness. NC-2 is a specific prose encoding of THIS artifact's exact content. NC-2 tests whether the test artifact's IDEAS matter; CT-2 tests whether its DOMAINS matter.
- **Complexity:** MEDIUM — requires understanding every element of the test artifact and translating it to prose

**NC-3: Formal Structure Without Semantic Depth**
- **Purpose:** Correct, standard physics equations that aren't UL-structured
- **Source:** Simple harmonic oscillator or free particle equations
- **Content:** Standard SHO: `mẍ + kx = 0`, solutions `x(t) = A cos(ωt + φ)`, energy `E = ½kA²`, quantized version `Ĥψ = Eψ`, `ψₙ(x) = NₙHₙ(αx)e^{-α²x²/2}`, etc.
- **Requirements:** ~133 words, mathematically dense, correct physics, contains ψ (!), but no cross-domain binding, no bridge, no anomalies, no self-reference
- **Key distinction from CT-1:** CT-1 is graduate-level physics (harder). NC-3 is standard physics with ψ present — specifically testing whether having ψ in standard equations does anything.
- **Complexity:** LOW — standard textbook physics

**NC-4: Reversed Dissipation**
- **Purpose:** Tests whether the sign of the dissipation term matters
- **Source:** Direct modification of the test artifact
- **Content:** The EXACT artifact with ONE change: `− i γ(σ,β) ψ` → `+ i γ(σ,β) ψ`
- **Requirements:** One character change (minus → plus). Everything else identical.
- **Complexity:** TRIVIAL — one character
- **Validation:** Confirm only the sign changed

**NC-5: Pseudo-Esoteric Without Geometry**
- **Purpose:** Tests whether esoteric content without geometric encoding produces any effect
- **Source:** Written from scratch in the style of esoteric/occult literature
- **Content:** Mystical prose with invented terms, cross-domain aspirations ("the vibration of the third elúmic sphere resonates through the linguistic-quantum bridge..."), bridge-like tokens, references to consciousness/energy/language — but ZERO mathematical structure
- **Requirements:** ~133 words, esoteric vocabulary and register, invented bridge-like terms, cross-domain claims, zero formal notation
- **Complexity:** MEDIUM — creative writing in a specific register

---

### A.3 Construction Order and Dependencies

The texts fall into three difficulty tiers. Work within each tier can be parallelized.

```
TIER 1 — MECHANICAL (no design judgment, start immediately)
│
├── V6 / ABL-NODAMP ............ delete γ(σ,β)ψ term, 5 min
├── V7 / ABL-REORDER ........... cut-and-paste reorder, 10 min
├── NC-4 / Reversed Dissipation . change one sign, 1 min
├── CT-3 / Scrambled Primer ..... find-replace + reorder, 30 min
│
│ Deliverable: 4 texts, ready for QC
│
TIER 2 — SOURCED/ADAPTED (requires domain knowledge, moderate design)
│
├── CT-1 / Dense Physics ........ source from QM textbook, ~1 hour
├── CT-4 / Nonsense Math ........ write plausible gibberish, ~1 hour
├── V4 / ABL-BRIDGE ............. replace +3elúm, ~30 min
├── V3 / ABL-STANDARD ........... correct anomalies, ~1 hour
├── NC-1 / Single-Domain Math ... source from algebra/topology text, ~1 hour
├── NC-3 / SHO Formalism ........ write standard SHO passage, ~45 min
├── NC-5 / Pseudo-Esoteric ...... creative writing, ~1 hour
│
│ Deliverable: 7 texts, ready for QC
│
TIER 3 — DESIGNED (requires careful intellectual work)
│
├── V1 / ABL-PROSE .............. full prose rewrite of test artifact, ~2–3 hours
├── V2 / ABL-SYMBOL ............. ψ domain-assignment + replacement, ~2 hours
├── V5 / ABL-LINEAR ............. break self-reference loop, ~1–2 hours
├── CT-2 / Cross-Domain Prose ... write academic prose, ~1 hour
├── NC-2 / Primer-as-Prose ...... translate artifact content to English, ~1–2 hours
│
│ Deliverable: 5 texts, ready for QC
│
THEN: QUALITY CONTROL (parallel with infrastructure)
│
├── Length-match verification for all 16 texts
├── Feature-preservation audit (checklist per variant)
├── Cross-contamination check (no variant accidentally preserves
│   the removed feature or accidentally removes a kept feature)
│
PARALLEL TRACK: INFRASTRUCTURE
│
├── M1–M5 scoring rubric (detailed, with anchor examples)
├── Domain lists per task prompt T1–T5
├── Known-connections reference sets per task (for M5)
├── Prompt templates (exact format: system prompt + artifact/control + task)
├── Analysis scripts (R/Python, pre-committed)
├── SQS rubric for Epsilon tasks
├── Evaluator calibration set (10 outputs, pre-scored — requires pilot runs)
```

---

### A.4 Quality Control Protocol

Every text variant must pass a **feature audit** before use. The audit confirms that exactly the intended features are present/absent.

**Feature checklist (applied to every variant):**

| Feature | Check |
|---|---|
| Formal mathematical notation | Present? Score: HIGH / LOW / NONE |
| ψ as central symbol | Present as overloaded? Present but not overloaded? Absent? |
| Deliberate formal anomalies | Present? Which ones? |
| Esoteric bridge token (+3elúm) | Present? Replaced with what? |
| Self-referential structure (ψ→ψ̂→f_spec→ψ loop) | Present? Broken how? |
| Dissipation term γ(σ,β)ψ | Present? Sign? |
| Section ordering (wall→bridge→PDE→defs) | Original? Reversed? Other? |
| Cross-domain content | Which domains activated? |
| Word count | Within ±10% of 133? |

**Expected feature profiles:**

| Variant | Notation | ψ-overload | Anomalies | Bridge | Self-ref | Dissipation | Ordering | Cross-domain |
|---|---|---|---|---|---|---|---|---|
| **Full artifact** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓(−) | ✓ | ✓ |
| ABL-PROSE (V1) | ✗ | ✓(prose) | ✓(prose) | ✓ | ✓(prose) | ✓(prose) | ✓ | ✓ |
| ABL-SYMBOL (V2) | ✓ | ✗ | ✓ | ✓ | ✓ | ✓(−) | ✓ | ✓ |
| ABL-STANDARD (V3) | ✓ | ✓ | ✗ | ✓ | ✓ | ✓(−) | ✓ | ✓ |
| ABL-BRIDGE (V4) | ✓ | ✓ | ✓ | ✗ | ✓ | ✓(−) | ✓ | ✓ |
| ABL-LINEAR (V5) | ✓ | ✓ | ✓ | ✓ | ✗ | ✓(−) | ✓ | ✓ |
| ABL-NODAMP (V6) | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ✓ |
| ABL-REORDER (V7) | ✓ | ✓ | ✓ | ✓ | ✓ | ✓(−) | ✗ | ✓ |
| CT-1 (physics) | ✓ | partial | ✗ | ✗ | ✗ | ✗ | n/a | ✗ |
| CT-2 (prose) | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | n/a | ✓ |
| CT-3 (scrambled) | ✓ | ✗ | ✓ | ✗ | ✓ | ✓(−) | ✗ | partial |
| CT-4 (nonsense) | ✓ | ✗ | n/a | ✗ | ✗ | ✗ | n/a | ✗ |
| NC-1 (single math) | ✓ | ✗ | ✗ | ✗ | ✗ | ✗ | n/a | ✗ |
| NC-2 (artifact prose) | ✗ | ✓(prose) | ✓(described) | ✓(described) | ✓(described) | ✓(described) | ✓(described) | ✓ |
| NC-3 (SHO) | ✓ | partial | ✗ | ✗ | ✗ | ✗ | n/a | ✗ |
| NC-4 (anti-damp) | ✓ | ✓ | ✓ | ✓ | ✓ | ✓(+) | ✓ | ✓ |
| NC-5 (esoteric) | ✗ | ✗ | ✗ | partial | ✗ | ✗ | n/a | ✓ |

Any deviation from this table during QC indicates a construction error.

---

### A.5 Design Decisions Requiring Resolution

Several construction choices have non-obvious implications. These must be decided BEFORE construction begins and documented as part of the pre-registration.

| # | Decision | Options | Recommendation | Rationale |
|---|---|---|---|---|
| **DD-1** | CT-2 vs. NC-2 overlap | (a) Keep both as distinct texts, (b) Merge into one | (a) Keep both | CT-2 = generic cross-domain essay. NC-2 = specific prose encoding of test artifact content. They test different things: domains vs. ideas. |
| **DD-2** | ABL-SYMBOL (V2): How to assign domains to ambiguous ψ instances | (a) Assign by syntactic context (what operator acts on it), (b) Assign by semantic intent (what "ψ means" in that equation), (c) Assign all ambiguous instances to one neutral symbol | (a) Syntactic context | Most objective. Semantic intent requires interpretation, which introduces experimenter bias. |
| **DD-3** | ABL-PROSE (V1): How to handle length expansion | (a) Accept longer text, (b) Truncate to ~133 words losing some relationships, (c) Use compressed prose | (c) Compressed prose | Length mismatch is a confound. Compressed prose preserves relationships while matching length. Accept that some fidelity is lost — document which relationships are approximated. |
| **DD-4** | CT-3 scrambled symbol choices | (a) Random replacement, (b) Deliberately choose symbols with LESS cross-domain loading than ψ, (c) Choose symbols with DIFFERENT but equal loading | (b) Less loading | The hypothesis (H4) is that ψ's specific cross-domain overloading matters. Testing against less-loaded symbols (χ, φ, f) is the direct test. |
| **DD-5** | NC-5 bridge-like tokens | (a) Use phonetically similar tokens (+3elúm → +3alúm), (b) Use phonetically distinct esoteric-sounding tokens (+7ʃakrim), (c) Use existing esoteric terms (Tetragrammaton, Om) | (b) Phonetically distinct | (a) is too close — might preserve the original mechanism. (c) has real-world associations that could produce their own effects. (b) is cleanly esoteric-sounding but novel. |
| **DD-6** | Token count matching method | (a) Match word count (±10%), (b) Match approximate LLM token count (via tokenizer), (c) Match character count | (b) LLM token count | LLMs process tokens, not words or characters. Use cl100k_base (GPT-4) tokenizer as reference. Allow ±5% variance. |

---

### A.6 Prompt Template Specification

All experiments use the same prompt injection format. The template must be fixed at pre-registration.

**UL-Mode template:**
```
[SYSTEM: You are a helpful AI assistant.]

[CONTEXT BEGIN]
{test artifact — experiments/test-artifacts/original/primer.txt, verbatim}
[CONTEXT END]

{task prompt T1–T5, verbatim}
```

**CT-Mode template:**
```
[SYSTEM: You are a helpful AI assistant.]

[CONTEXT BEGIN]
{control text CT-1/CT-2/CT-3/CT-4, verbatim}
[CONTEXT END]

{task prompt T1–T5, verbatim}
```

**NL-Mode template:**
```
[SYSTEM: You are a helpful AI assistant.]

{task prompt T1–T5, verbatim}
```

**Design decisions for prompts:**
- The system prompt is minimal and identical across all conditions.
- The `[CONTEXT BEGIN/END]` delimiters are present in BOTH UL-mode and CT-mode but ABSENT in NL-mode. This means NL-mode also lacks the "here is context" framing. If this is a confound, add a `[CONTEXT BEGIN] [CONTEXT END]` empty block to NL-mode.
- No instructions are given about what the context is or how to use it. The model receives the text and the question — nothing more.
- Max generation tokens: 4096 (long enough for Phase 3 synthesis to emerge if it's going to)
- For temperature 0.0: 1 trial per cell (deterministic). For temperatures 0.7 and 1.0: 3 trials per cell.

---

### A.7 File Organization

All experimental materials live in a new `experiments/` directory:

```
experiments/
├── test-artifacts/
│   ├── original/
│   │   └── primer.txt                    # Original test artifact
│   │
│   ├── controls/
│   │   ├── CT-1_dense-physics.txt
│   │   ├── CT-2_cross-domain-prose.txt
│   │   ├── CT-3_scrambled-primer.txt
│   │   └── CT-4_nonsense-math.txt
│   │
│   ├── ablations/
│   │   ├── V1_ABL-PROSE.txt
│   │   ├── V2_ABL-SYMBOL.txt
│   │   ├── V3_ABL-STANDARD.txt
│   │   ├── V4_ABL-BRIDGE.txt
│   │   ├── V5_ABL-LINEAR.txt
│   │   ├── V6_ABL-NODAMP.txt
│   │   └── V7_ABL-REORDER.txt
│   │
│   └── negative-controls/
│       ├── NC-1_single-domain-math.txt
│       ├── NC-2_primer-as-prose.txt
│       ├── NC-3_standard-physics.txt
│       ├── NC-4_reversed-dissipation.txt
│       └── NC-5_pseudo-esoteric.txt
│
├── scoring/
│   ├── rubric-M1-M5.md                   # Primary metric rubric
│   ├── rubric-SQS.md                     # Epsilon structural quality rubric
│   ├── domain-lists/
│   │   ├── T1-domains.md
│   │   ├── T2-domains.md
│   │   ├── T3-domains.md
│   │   ├── T4-domains.md
│   │   └── T5-domains.md
│   ├── known-connections/
│   │   ├── T1-known.md
│   │   ├── T2-known.md
│   │   ├── T3-known.md
│   │   ├── T4-known.md
│   │   └── T5-known.md
│   └── calibration/
│       └── calibration-set.md             # 10 pre-scored examples
│
├── prompts/
│   ├── templates.md                       # Prompt injection templates
│   └── task-prompts.md                    # T1–T5, E1–E5 exact text
│
├── analysis/
│   └── analysis-plan.py                   # Pre-committed analysis scripts
│
├── data/                                  # Empty until trials run
│   ├── alpha/
│   ├── beta/
│   ├── gamma/
│   ├── delta/
│   └── epsilon/
│
└── reports/                               # Empty until analysis
    └── (post-experiment write-ups)
```

---

### A.8 Phase 0 Execution Timeline

| Week | Work | Deliverable |
|---|---|---|
| **Week 1, Days 1–2** | Tier 1 construction (V6, V7, NC-4, CT-3) + File structure setup | 4 texts + directory structure |
| **Week 1, Days 3–5** | Tier 2 construction (CT-1, CT-4, V3, V4, NC-1, NC-3, NC-5) | 7 texts |
| **Week 2, Days 1–3** | Tier 3 construction (V1, V2, V5, CT-2, NC-2) | 5 texts (all 16 complete) |
| **Week 2, Days 3–5** | QC: Feature audit all 16 texts against §A.4 checklist. Token-count verification. | QC report |
| **Week 2 (parallel)** | Scoring rubric (M1–M5, SQS), domain lists, prompt templates | Scoring infrastructure |
| **Week 3, Days 1–3** | Known-connections reference sets. Analysis scripts. Resolve DD-1 through DD-6. | Full infrastructure |
| **Week 3, Days 3–5** | Pilot: Run 5 trials (1 per task, UL-mode, 1 model, temp 0.7). Score. Build calibration set. | 10 calibration examples |
| **Week 3, Day 5** | Pre-registration: hash protocol + materials. Final review. | **PHASE 0 COMPLETE** |

**Total Phase 0 duration: ~3 weeks** (consistent with protocol §13 estimate of 2–3 weeks)

---

### A.9 Proceed / No-Proceed Gate

Before moving to Phase 1 (Experiment Alpha), ALL of the following must be true:

- [x] All 16 texts constructed and QC-verified
- [x] All 16 texts pass feature audit (§A.4 table, zero deviations)
- [x] Token counts within ±5% of the test artifact (measured via cl100k_base tokenizer)
- [x] M1–M5 scoring rubric complete with anchor examples
- [x] Domain lists complete for T1–T5
- [x] Known-connections sets complete for T1–T5
- [x] Prompt templates finalized
- [x] Analysis scripts committed (analysis.py, run_trials.py, blind.py, preregister.py)
- [ ] Calibration set of 10 pre-scored examples available (run: `python run_trials.py pilot --model gpt-4o`)
- [x] Design decisions DD-1 through DD-6 resolved and documented
- [ ] Protocol document hashed and hash published (run: `python preregister.py`, then publish hash)
- [ ] API access confirmed for ≥4 of 6 target models

Failure to meet any item blocks Phase 1 start. No exceptions — the entire value of the protocol rests on pre-registration integrity.
