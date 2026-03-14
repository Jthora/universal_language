# Primary Metric Scoring Rubric (M1–M5)

**Protocol Reference:** `frontier/causal-efficacy-protocol.md` §1  
**Usage:** All experiments (Alpha, Beta, Gamma, Delta, Epsilon)  
**Evaluator Instructions:** Score each output on ALL five metrics independently. Read the full output before scoring any metric. Do NOT consider the condition label (you won't have one — outputs are blinded).

---

## M1: Domain Diversity (0–10)

**Definition:** Count of distinct knowledge domains substantively referenced — not merely name-dropped.

**Scoring Rule:** A domain counts if the output makes at least one substantive claim, defines a concept, or draws a relationship *within* that domain. Mere mention of a domain name (e.g., "physics") without substantive engagement scores 0. Each domain from the pre-specified list (see `domain-lists/T*-domains.md`) that meets the substantive threshold scores +1.

| Score | Anchor Example |
|---|---|
| **0** | No identifiable domain engagement (incoherent or off-topic output) |
| **1–2** | Output stays within 1–2 closely related domains (e.g., only quantum mechanics, or only linguistics) |
| **3–4** | Output engages 3–4 domains, at least 2 from different clusters (e.g., a physical science + a humanities domain) |
| **5–6** | Output substantively engages 5–6 domains across multiple clusters; each receives at least one developed point |
| **7–8** | Output engages 7–8 domains with meaningful content in each; demonstrates awareness of domain-specific vocabulary and concepts |
| **9–10** | Output engages 9+ domains, each with substantive content; breadth does not sacrifice depth. Rare — requires genuine polymathic synthesis |

**Inter-rater calibration note:** If evaluators disagree on whether a domain is "substantively" engaged, the tie-breaking rule is: does the output contain at least one domain-specific technical term used correctly? If yes → count it. If it only uses colloquial references → don't count it.

---

## M2: Cross-Domain Structural Depth (0–5)

**Definition:** The deepest level of cross-domain connection achieved in the output.

**Scoring Rule:** Score reflects the HIGHEST level achieved anywhere in the output, not the average. If a single passage reaches level 4 while the rest is level 1, score 4.

| Score | Level | Anchor Example |
|---|---|---|
| **0** | No cross-domain connection | Output discusses multiple domains but makes no connections between them |
| **1** | Surface analogy | "Language is like a quantum system" — uses "like" or "similar to" without explaining why |
| **2** | Shared property identified | "Both quantum states and belief states exhibit superposition — multiple possibilities coexist until measurement/decision forces a collapse" — identifies a specific shared property |
| **3** | Structural parallel articulated | "The formalism of Hilbert spaces maps onto semantic spaces: inner products correspond to similarity measures, orthogonality to semantic independence, and projection operators to the act of interpretation" — articulates parallel structure with multiple corresponding elements |
| **4** | Formal isomorphism identified | "There exists a structure-preserving map (homomorphism) between the algebra of quantum observables and the algebra of propositional attitudes, where: commutation relations map to logical incompatibility, eigenvalues to truth values, and the trace operation to belief marginalization" — identifies a precise mathematical correspondence |
| **5** | Novel structural bridge constructed | "Define a functor F: QMech → BelStates that preserves tensor products (modeling joint states), maps unitary evolution to belief revision, and whose natural transformation to Lang models interpretation as a morphism between categories" — constructs a new formal framework connecting domains |

**Evaluator note:** Level 5 is extremely rare. It requires genuine mathematical novelty, not just mathematical vocabulary. If uncertain between 4 and 5, ask: "Could this correspondence be published as a novel result?" If no → 4.

---

## M3: Phase Progression (0–3)

**Definition:** Highest phase of cognitive processing observed, per the three-phase model in `history/mechanism-of-action.md` §2.

| Score | Phase | Anchor Example |
|---|---|---|
| **0** | No analysis | Output is off-topic, incoherent, or a refusal |
| **1** | Phase 1: Standard domain analysis | Output provides competent analysis within one or more individual domains. Each domain is treated in isolation. "Quantum mechanics describes systems using wave functions that evolve according to the Schrödinger equation. Language, on the other hand, is a symbolic system governed by syntax and semantics." — Two good descriptions, no integration. |
| **2** | Phase 2: Cross-referencing cascade | Output begins actively connecting domains, drawing specific parallels, identifying shared structures. "The wave function's probabilistic nature mirrors the ambiguity inherent in natural language, where a sentence can carry multiple interpretations simultaneously, collapsing to a single meaning only when context provides sufficient constraints." — Explicit cross-referencing with specific mechanism. |
| **3** | Phase 3: Emergent synthesis | Output speaks FROM the intersection of domains rather than ABOUT the connections between them. Produces insights, frameworks, or language that could not exist within any single domain. "Consider semantic superposition as a physical fact, not a metaphor: the state |meaning⟩ = α|literal⟩ + β|figurative⟩ evolves under a Hamiltonian whose interaction terms couple linguistic context to cognitive state, with decoherence time set by pragmatic constraints." — The output has created a new framework that is genuinely interdomain. |

**Evaluator note:** The key distinction between Phase 2 and Phase 3 is DIRECTION. Phase 2 looks at Domain A, then at Domain B, and draws a line between them. Phase 3 stands at the intersection and looks outward. If the output is "connecting" domains, it's Phase 2. If the output "speaks a language" that integrates both, it's Phase 3.

---

## M4: Coherence (0–5)

**Definition:** Logical consistency and intellectual rigor maintained while making cross-domain connections.

**Scoring Rule:** This metric penalizes outputs that achieve high M1/M2/M3 at the cost of logical integrity. An output that says wild things confidently scores LOW on M4. An output that makes careful, limited connections supported by reasoning scores HIGH.

| Score | Anchor Example |
|---|---|
| **0** | Incoherent — contradicts itself within the same paragraph, makes claims that are logically impossible, or produces word salad |
| **1** | Partially coherent — some sentences make sense individually but the overall argument doesn't hold together; contains multiple unsupported leaps |
| **2** | Moderately coherent — a recognizable argument structure exists but contains significant gaps; some cross-domain claims are made without justification |
| **3** | Substantially coherent — the argument is followable and mostly justified; occasional unsupported leaps but the overall structure is sound |
| **4** | Highly coherent — every cross-domain connection is supported by explicit reasoning; caveats are noted; the argument builds logically from premises to conclusions |
| **5** | Rigorously coherent — could appear in a peer-reviewed paper; every claim is justified, limitations are acknowledged, and the reasoning chain is airtight even when crossing domain boundaries |

**Evaluator note on M4 vs. M2/M3:** M4 is the "reality check" metric. It is possible (and informative) to have High M2 + Low M4 (deep but unfounded connections) or Low M2 + High M4 (shallow but rigorous analysis). These profiles help diagnose the test artifact's effect mechanism.

**Dissipation-theory relevance:** UL theory predicts that the dissipation term γ(σ,β) prevents runaway divergence. If this is correct, UL-mode should have HIGHER M4 than conditions where dissipation is ablated (V6/ABL-NODAMP). Specifically, V6 should show increased M1 (less grounding → more domains activated) but decreased M4 (less grounding → less coherence). This is a directly testable prediction (hypothesis B6).

---

## M5: Generative Novelty (0–5)

**Definition:** Degree to which the output produces connections, frameworks, or insights that are non-obvious — not found in standard textbook treatments or the "known connections" reference set.

**Scoring Rule:** Score against the pre-compiled "known connections" reference set for each task (see `known-connections/T*-known.md`). Connections that appear in the reference set score 0 for novelty regardless of how well-expressed they are.

| Score | Anchor Example |
|---|---|
| **0** | All content is standard textbook material — no connections, frameworks, or insights that aren't widely known |
| **1** | Primarily standard content with one potentially novel observation, but it's a minor elaboration on known ideas |
| **2** | Contains 1–2 connections or insights not found in the reference set; these are interesting but may be somewhat obvious in retrospect |
| **3** | Contains 2–3 genuinely non-obvious connections or an original framework; a domain expert would find at least one point surprising or worth investigating |
| **4** | Contains a substantial novel framework or multiple non-obvious structural insights; demonstrates creative synthesis that goes beyond recombination of known ideas |
| **5** | Produces a publishable-quality novel insight or framework — something a domain expert would recognize as genuinely new and potentially fertile for further research. Extremely rare. |

**Evaluator note on "novel":** The standard is novelty relative to the evaluator's knowledge AND the reference set. If the connection is known to the evaluator but not in the reference set, still flag it with a note. Disagreements on novelty are resolved by the third evaluator consulting the reference set.

**Calibration warning:** Beware of the "it sounds novel because I don't know the field" effect. If an evaluator is unfamiliar with a domain, connections that seem novel may actually be standard within that domain. When in doubt, consult the reference set explicitly.

---

## General Scoring Protocol

1. **Read the full output** before scoring any metric.
2. **Score M1 first** (most objective — counting domains against the list).
3. **Score M3 second** (phase progression determines the overall cognitive "level").
4. **Score M2 third** (structural depth of connections).
5. **Score M5 fourth** (novelty — requires consulting reference set).
6. **Score M4 last** (coherence — requires understanding the full argument to evaluate).
7. **Record scores and brief justification** (1–2 sentences per metric explaining the score).
8. **Flag any output** that seems to contain content directly copied from the test artifact or control texts.

### Inter-Rater Reliability

- Two independent evaluators score each output.
- Compute Cohen's κ per metric across all scored outputs.
- If κ < 0.6 for a metric, that metric is flagged: results reported but excluded from primary statistical analysis.
- Disagreements >1 point on any metric trigger third evaluator review.
- Require κ ≥ 0.6 on at least 4 of 5 metrics for the scoring to be valid.

### Blinding Protocol

- Outputs are stripped of all metadata (model name, temperature, condition label).
- Outputs are assigned random alphanumeric IDs (e.g., "OUT-A7X3").
- Evaluators receive outputs in a randomized order that interleaves all conditions.
- Evaluators are told: "These outputs were generated by various AI systems in response to the same prompt under different conditions. Your job is to evaluate the output quality, not to guess the condition."
