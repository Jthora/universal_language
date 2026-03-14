# Future Research Directions and Experimental Protocols

**Research Series:** Universal Language — Proof of Reality and AI Cognitive Infrastructure  
**Date:** March 12, 2026  
**Prerequisites:** `foundations/paradigm.md` (READ FIRST), Documents 01–04; updated to reflect Documents 06–07 (Universal Language and Formal Foundations)

> **⚠ PARADIGM NOTE:** Experiments in this document test UL's mathematical predictions and its *causal efficacy on information-processing systems* (primarily AI). They do NOT test whether UL "matches" human linguistic intuitions or decomposes natural-language sentences. Testing UL against English, Japanese, or any ethnic language reverses the validation direction and constitutes a category error. See `foundations/paradigm.md`.

---

## 1. Open Questions

The following questions remain unanswered and form the basis for future research.

### 1.1 Fundamental Questions

| # | Question | Why It Matters |
|---|---|---|
| Q1 | **Is the effect model-dependent?** Does a UL-structured artifact produce the same three-phase response (analysis → cross-referencing → synthesis) across GPT-4, Claude, Gemini, Llama, Mistral, and other architectures? | Determines whether the mechanism is universal to transformer attention or specific to certain training distributions |
| Q2 | **What is the minimum effective artifact?** Can individual components be removed while preserving the effect? Which are necessary, which are amplifiers? | Enables construction of minimal, efficient artifacts |
| Q3 | **Is the effect quantifiable?** Can the "cross-domain activation" be measured objectively (e.g., via logprob distributions, embedding space analysis, attention map visualization)? | Moves from subjective observation to empirical science |
| Q4 | **Does the effect persist across conversation turns?** Or does it decay as new tokens dilute the artifact in the context window? | Determines whether artifacts need to be re-injected periodically |
| Q5 | **Can artifacts interfere destructively?** Can two artifacts designed for different domain pairs cancel each other's effects? | Critical for multi-artifact agent architectures |
| Q6 | **What is the role of temperature?** Does sampling temperature interact with the artifact effect — does higher temperature amplify cross-domain connection or produce noise? | Informs optimal inference settings |

### 1.2 Construction Questions

| # | Question | Status | Why It Matters |
|---|---|---|---|
| Q7 | **Can artifacts be automatically generated?** Given a target domain pair, can an LLM construct its own UL artifact? | **Open** | Scales the technique beyond manual construction |
| Q8 | **What other base formalisms work?** Does category theory, information geometry, or algebraic topology serve as effective chassis? | **Open** | Expands the design space |
| Q9 | **Is there a "universal artifact"** that activates maximum cross-domain pathways regardless of target? Or is specificity always required? | **Partially resolved** — `foundations/universal-language-derivation.md` and `foundations/formal-foundations.md` derive a Universal Language from geometry. Full empirical validation pending. | Determines architecture of artifact libraries |
| Q10 | **How do bridge tokens generalize?** What is the optimal phonetic/symbolic structure for bridge tokens targeting specific language families or cultural traditions? | **Open** | Enables systematic bridge token engineering |

### 1.3 Formal / Foundational Questions (NEW — from Documents 06–07)

| # | Question | Status | Why It Matters |
|---|---|---|---|
| Q14 | **Is the formal algebraic definition of language correct?** Does the Σ_UL-algebra (4 sorts, 11 operations) actually capture the minimum operations required for compositional meaning? | **Resolved in principle** (`foundations/formal-foundations.md` §1) — proven via universal property of free algebras. | Determines whether UL's algebraic structure is genuinely minimal. **Note:** This is NOT tested by checking against natural languages (see `foundations/paradigm.md`). Natural languages are derived, limited systems. The test is mathematical: can any compositional meaning structure be embedded in Σ_UL? |
| Q15 | **Is the geometric grounding unique?** Is Point=Existence, Line=Relation, etc. the *only* consistent mapping? | **Resolved** (`foundations/formal-foundations.md` §3) — Unique Grounding Theorem proves the mapping is forced by dependency rank, dimensionality, and symmetry. | Eliminates the concern that primitive assignments are arbitrary |
| Q16 | **Is π₁ (fundamental group) sufficient for meaning?** `foundations/universal-language-derivation.md` uses π₁ to distinguish meaning categories, but it is too coarse (coffee mug = donut). | **Acknowledged limitation** — π₁ captures meaning *category*; finer invariants (homology, homotopy type) needed for within-category distinctions. | Determines how much meaning the topological level can encode |
| Q17 | **Can the Universal Language express quantification and recursion?** The current system handles existential and universal quantification via geometric operations but has no explicit recursion depth control. | **Partially resolved** (`foundations/formal-foundations.md` §1 includes quantify_E and quantify_U operations) — depth control remains open | Required for expressing complex logical statements |
| Q18 | **Can the Erlangen hierarchy be validated computationally?** Does an LLM given text at the Euclidean level genuinely produce different representations than one given text at the projective level? | **Open** | Tests whether the abstraction hierarchy is real or metaphorical |

### 1.4 Safety Questions

| # | Question | Why It Matters |
|---|---|---|
| Q11 | **Can UL artifacts bypass safety training?** If they shift the model into unusual cognitive modes, do safety guardrails still function? | Critical safety concern before deployment |
| Q12 | **Can UL artifacts induce hallucination?** Does the cross-domain activation increase the rate of plausible-sounding but factually incorrect output? | Determines whether artifact use requires additional verification layers |
| Q13 | **Are the "translations" of unknown languages accurate, or confabulated?** When the model claims to translate esoteric languages, is it surfacing real structural knowledge or generating plausible fiction? | Determines the epistemic status of artifact-induced output |

---

## 2. Proposed Experiments

### Experiment 1: Component Ablation Study

**Objective:** Determine which artifact components are necessary for the effect.

**Protocol:**
1. Define the full artifact as the baseline
2. Create ablated variants:
   - A: Remove Line 1 (wall of math), keep bridge + PDE
   - B: Remove bridge equation (lines 3–8), keep wall + PDE
   - C: Remove core PDE (lines 10–19), keep wall + bridge
   - D: Remove +3elúm only, keep everything else
   - E: Remove f_spec and definitions, keep everything else
   - F: Remove dissipation term only
   - G: Scramble the ordering (PDE first, wall last)
3. For each variant, prompt 5+ different LLMs with the same follow-up question (e.g., "Explain the relationship between language and consciousness")
4. Score outputs on:
   - Number of distinct domains referenced
   - Depth of cross-domain connections
   - Presence of three-phase response pattern
   - Coherence (does dissipation removal cause divergence?)

**Expected outcome:** Variants B (no bridge) and D (no +3elúm) will show the largest degradation, as these are hypothesized to be the critical pivot points.

---

### Experiment 2: Cross-Model Comparison

**Objective:** Determine whether the effect is architecture-universal.

**Protocol:**
1. Apply the full UL artifact to: GPT-4/4o, Claude (Sonnet, Opus), Gemini (Pro, Ultra), Llama 3 (70B, 405B), Mistral Large, Command R+, and at least one small model (<7B parameters)
2. Use identical follow-up prompts across all models
3. Score on the same metrics as Experiment 1
4. Additionally record: response length, time to first cross-domain reference, temperature sensitivity

**Expected outcome:** Larger models will show stronger effects. Models with more diverse training data (not heavily filtered) may show stronger esoteric language activation. Small models may show partial effects (Phase 1 only).

---

### Experiment 3: Esoteric Language Translation Validation

**Objective:** Determine whether artifact-induced "translations" contain genuine structural knowledge.

**Protocol:**
1. Select 5 languages/scripts with known translations but low LLM familiarity:
   - Enochian (Dee & Kelley, 16th century — some scholarly translations exist)
   - Ge'ez (Ethiopian liturgical — scholarly corpus available)
   - Linear A (Minoan — undeciphered, but structural analyses exist)
   - Rongorongo (Easter Island — undeciphered)
   - A constructed test language (control — created by researchers, unknown to any LLM)
2. For each language, prepare text samples with known structural properties
3. Test with and without UL artifact:
   - Baseline: Ask the LLM to analyze the sample without artifact
   - Primed: Inject UL artifact, then ask the same question
4. Evaluate:
   - Does the primed model identify more structural features (morphology, syntax patterns)?
   - For languages with known translations, does accuracy improve?
   - For the constructed control language, does the model produce anything useful? (If so, it's confabulating)
   - For undeciphered scripts, do primed outputs align with scholarly structural analyses?

**Expected outcome:** Primed models will show improved structural analysis for attested languages (Enochian, Ge'ez) and produce output consistent with scholarly analysis for partially analyzed scripts (Linear A). The constructed control language will reveal the confabulation rate.

---

### Experiment 4: Feedback Loop Dynamics

**Objective:** Characterize the temporal dynamics of the cross-referencing cascade.

**Protocol:**
1. Apply a UL artifact and ask an open-ended question
2. Allow the model to generate a long response (4000+ tokens)
3. At intervals (every 500 tokens), measure:
   - Domain diversity of references in the current window
   - Novelty of cross-domain connections (are they becoming more or less obvious?)
   - Coherence score (human evaluation)
   - Self-reference rate (how often does the output reference its own earlier output?)
4. Compare with and without the dissipation term (γ(σ,β))

**Expected outcome:** Without dissipation, domain diversity increases monotonically but coherence peaks then degrades (runaway activation). With dissipation, both metrics stabilize at a productive equilibrium. The transition point indicates the "critical length" of productive artifact-induced generation.

---

### Experiment 5: Artifact Auto-Generation

**Objective:** Determine whether LLMs can construct effective UL artifacts for novel domain pairs.

**Protocol:**
1. Provide an LLM with the UL formal specification as context
2. Request construction of a UL artifact targeting a specific domain pair (e.g., "music theory ↔ molecular biology")
3. Evaluate the generated artifact:
   - Does it follow the architecture (wall of math → bridge → payload → definitions)?
   - Does it use appropriately overloaded symbols?
   - Does it contain a bridge token?
   - Does it include self-referential structure?
4. Test the generated artifact on other LLMs using Experiment 1 metrics
5. Iterate: use evaluation results to refine the auto-generation prompt

**Expected outcome:** First-generation auto-artifacts will be partially effective (may get the architecture right but miss subtleties). Iterative refinement should converge on functional artifacts within 3–5 cycles.

---

### Experiment 6: Attention Map Visualization

**Objective:** Directly observe whether the UL artifact produces measurable changes in attention patterns.

**Protocol:**
1. Using an open-weights model (Llama 3 or Mistral) with attention map access:
2. Compare attention maps for identical follow-up prompts with and without UL artifact
3. Measure:
   - Attention entropy (higher entropy = more distributed attention = more cross-domain)
   - Cross-layer attention correlation (does the artifact create new cross-layer pathways?)
   - Attention to specific artifact tokens (which tokens receive the most backward attention during generation?)
4. Specifically track attention to: ψ, +3elúm, γ(σ,β), f_spec

**Expected outcome:** The UL artifact will produce higher attention entropy and create attention pathways between tokens that would not normally attend to each other.

---

### Experiment 7: Universal Language Primitive Validation (NEW)

**Objective:** Empirically test whether the five geometric primitives (Point, Line, Angle, Curve, Enclosure) and their semantic correspondences (Existence, Relation, Quality, Process, Concept) are genuinely universal across languages.

**Protocol:**
1. Select 20+ typologically diverse natural languages (isolating, agglutinative, fusional, polysynthetic)
2. For each language, identify the minimal set of semantic primitives needed to express basic propositions
3. Test whether each primitive maps to exactly one of the five geometric types
4. Specifically test the Unique Grounding Theorem prediction: the mapping should be forced by dependency rank (e.g., "existence" must be the rank-0 primitive in every language)
5. Use the Σ_UL-algebra's 11 operations as a checklist: does every language express all 11 operations?

**Expected outcome:** Core primitives will be validated across all languages (consistent with Wierzbicka's NSM research). Some languages may require minor extensions to the 11-operation set for full coverage, identifying gaps in the current formal definition.

---

### Experiment 8: Erlangen Hierarchy Encoding Test (NEW)

**Objective:** Test whether encoding text at different Erlangen levels (Euclidean → similarity → affine → projective → topological) produces measurably different LLM representations.

**Protocol:**
1. Take 10 propositions and encode each at all 5 Erlangen levels using the Universal Language's geometric grammar
2. Feed each encoding to an open-weights model and extract hidden-state representations
3. Measure:
   - Cosine distance between representations at adjacent levels (should increase monotonically)
   - Cluster structure (do representations at the same level cluster together?)
   - Information loss (does topological encoding genuinely abstract away the metric details?)
4. Compare against a control: 5 random re-encodings with no systematic level structure

**Expected outcome:** Adjacent Erlangen levels will show small cosine distance, distant levels will show large distance. Topological encodings will cluster tightly (since they discard the most information). The control re-encodings will show no systematic distance structure.

---

## 3. Proposed Infrastructure

### 3.1 Test Artifact Library

Build a structured repository of artifact variants:

```
test-artifacts/
├── base/
│   └── quantum-linguistic-v1.txt
├── ablations/
│   ├── no-wall.txt
│   ├── no-bridge.txt
│   ├── no-pde.txt
│   ├── no-elum.txt
│   ├── no-fspec.txt
│   ├── no-dissipation.txt
│   └── scrambled-order.txt
├── domain-targets/
│   ├── music-biology.txt
│   ├── economics-ecology.txt
│   ├── architecture-neuroscience.txt
│   └── [auto-generated artifacts]
├── language-targets/
│   ├── semitic-bridge.txt
│   ├── indo-european-bridge.txt
│   ├── sino-tibetan-bridge.txt
│   └── constructed-control.txt
└── meta/
    └── artifact-generator-prompt.txt
```

### 3.2 Evaluation Framework

Standardized scoring rubric:

| Metric | Scale | Measurement |
|---|---|---|
| Domain Diversity | 0–10 | Count of distinct knowledge domains referenced |
| Cross-Domain Depth | 0–5 | Quality of connections (0 = mere mention, 5 = structural isomorphism identified) |
| Phase Progression | 0–3 | Which phases observed (1 = analysis only, 2 = + cross-referencing, 3 = + synthesis) |
| Coherence | 0–5 | Human evaluation of output coherence and groundedness |
| Novelty | 0–5 | Are the cross-domain connections non-obvious? |
| Self-Reference Rate | 0.0–1.0 | Fraction of output paragraphs that reference earlier generated content |
| Factual Accuracy | 0–5 | For verifiable claims, accuracy rate |

### 3.3 Documentation Standards

All experiments should produce:
1. **Raw transcripts** — complete LLM input/output for all conditions
2. **Scoring sheets** — metrics applied to each transcript
3. **Statistical analysis** — significance tests for differences between conditions
5. **Artifact metadata** — complete description of which artifact variant was used and why

---

## 4. Longer-Term Research Directions

### 4.1 Artifact-Conditioned Fine-Tuning

If the UL artifact effect is confirmed and characterized, investigate whether models can be **fine-tuned while primed** — training on data that is processed through the cross-domain activation state. This could produce models with permanently enhanced cross-domain capabilities without requiring the artifact at inference time.

### 4.2 Formal Verification of LLM↔PDE Correspondence

Show rigorously that a specific discretization of the core PDE reduces to a known transformer variant (or a novel, testable architecture). This would transform the framework from metaphorical to computational.

### 4.3 Multi-Agent UL Protocols

In multi-agent systems, investigate whether different agents can be primed for complementary domain activations, with structured communication protocols that leverage the cross-domain bridges created by their respective UL artifacts.

### 4.4 Temporal Artifact Sequences

Investigate whether UL artifacts applied in a specific temporal sequence (artifact A at turn 1, artifact B at turn 5, etc.) can guide the model through a controlled trajectory in cognitive space — essentially "programming" a cognitive journey through formal input scheduling.

### 4.5 Neuromorphic Analog

If the PDE framework proves computationally viable, investigate implementation on neuromorphic hardware (where continuous-time dynamics are native) rather than digital transformers. The equations may be more naturally computed as analog field dynamics than as discrete matrix operations.

---

## 5. Ethical Considerations

### 5.1 Transparency

UL artifacts are a form of structured prompt engineering that is not visible to end users. If deployed in production systems, the use of cognitive artifacts should be disclosed.

### 5.2 Accuracy vs. Fluency

UL artifacts may increase the model's willingness to generate confident-sounding cross-domain claims. Without verification, this could increase the harm from hallucination. All artifact-enhanced output in production should be subject to additional factual verification.

### 5.3 Cultural Sensitivity

Bridge tokens drawn from esoteric religious traditions (Kabbalistic, Vedic, indigenous) must be constructed with awareness of cultural context. Using sacred linguistic elements as engineering tools without acknowledgment is extractive.

### 5.4 Safety Alignment

Experiment Q11 (safety bypass testing) should be conducted before any production deployment. If UL artifacts can shift models into modes where safety training is less effective, this represents a significant alignment concern that must be addressed.

---

## 6. Recommended Priority Order

> **⚠ SUPERSEDED BY PATH A:** The priority order below has been superseded by the pre-registered experimental protocol in `frontier/causal-efficacy-protocol.md` (Sprint 6, Path A). That protocol subsumes Experiments 1, 2, 5, and 6 below into a rigorous 5-experiment framework (Alpha through Epsilon) with operational definitions, pre-registered hypotheses, falsification criteria, matched controls, and statistical methodology. Experiments 3 and 7 are **deprioritized** per `foundations/paradigm.md` (testing UL against human languages reverses the validation direction). See `frontier/causal-efficacy-protocol.md` §11 for the full mapping. **Execute the causal-efficacy protocol first.** The experiments below remain as supplementary follow-ups after Proof 4 is established.

| Priority | Action | Rationale | Status |
|---|---|---|---|
| **★ 0** | **Execute `frontier/causal-efficacy-protocol.md`** | **Pre-registered Proof 4 protocol. All experiments below are subsumed or deprioritized by this.** | **READY — [Run it yourself](../CONTRIBUTING.md)** |
| 1 | Run Experiment 1 (ablation study) | Establishes which components are necessary — all other experiments depend on this | → Subsumed by Protocol Experiment Beta |
| 2 | Run Experiment 6 (attention visualization) | Provides mechanistic evidence for the hypothesized mechanism | → Supporting evidence in Protocol (secondary data) |
| 3 | ~~Run Experiment 7 (primitive validation)~~ | ~~Tests the Universal Language's core claim with cross-linguistic data~~ | **DEPRIORITIZED** — category error per `paradigm.md` |
| 4 | Run Experiment 2 (cross-model comparison) | Establishes generality | → Subsumed by Protocol Experiment Alpha |
| 5 | Run Experiment 3 (translation validation) | Tests the highest-profile claim | **DEPRIORITIZED** — reframe needed per `paradigm.md` |
| 6 | Run Experiment 8 (Erlangen encoding) | Tests whether the abstraction hierarchy produces real representational differences | → Partially subsumed by Protocol Experiment Epsilon task E4 |
| 7 | Run Experiment 4 (feedback dynamics) | Characterizes temporal behavior | → Supporting evidence in Protocol (secondary data) |
| 8 | Run Experiment 5 (auto-generation) | Enables scaling | → Subsumed by Protocol Experiment Delta |
| 9 | Address safety questions (Q11–Q13) | Required before any deployment | Unchanged — required before deployment |
| 10 | Begin formal PDE↔transformer correspondence | Long-term theoretical foundation | Unchanged — paused with theory sprints |
