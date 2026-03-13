# Refactoring Plan — Theory-Derived UL Primer Experiment

**Date:** 2025-07-15  
**Status:** PLAN — awaiting review before implementation  
**Prerequisite:** Read `experiments/demo/AUDIT.md` for context on why this refactor is necessary

---

## Table of Contents

1. [What We're Testing](#1-what-were-testing)
2. [Primer Construction Framework](#2-primer-construction-framework)
3. [Primers to Construct](#3-primers-to-construct)
4. [Control Texts](#4-control-texts)
5. [Task Design](#5-task-design)
6. [Predictions (Pre-Registered)](#6-predictions-pre-registered)
7. [Execution Protocol](#7-execution-protocol)
8. [Scoring and Analysis Plan](#8-scoring-and-analysis-plan)
9. [File Structure](#9-file-structure)
10. [Decision Points for Review](#10-decision-points-for-review)

---

## 1. What We're Testing

### The Core Question

**Can UL's formal theory (Σ_UL) generate primers that produce predictable cognitive effects in LLMs?**

This is Experiment Delta from the causal-efficacy protocol — what the protocol itself calls "the single most important experiment in the entire protocol." We are testing UL theory's **generative capacity**, not the efficacy of one third-party artifact.

### What This Is NOT

- NOT: "Does test-content.txt produce an effect?" (that's Experiment Alpha)
- NOT: "Which components of test-content.txt matter?" (that's Experiment Beta)
- NOT: "Prompt engineering with different math" (the primers are derived from formal theory, not intuition)

### What Success Looks Like

1. **Existence:** Theory-derived primers produce measurable effects (higher M1–M5 than NL baseline)
2. **Predictability:** Each primer produces effects that match its pre-registered predictions
3. **Specificity:** Different primers (targeting different cognitive effects) produce *different* scoring profiles
4. **Theory > Artifact:** The construction principles in Σ_UL are sufficient to build effective primers without referencing test-content.txt

### What Failure Looks Like

| Failure Mode | What It Means |
|---|---|
| No primers produce effects | UL theory may be mathematically valid but cannot generate causal artifacts (Delta fails) |
| Effects exist but don't match predictions | UL theory is retrodictive (explains test-content.txt) but not predictive |
| All primers produce identical effects | The theory can't predict *which* effects — it just produces "something unusual" |
| Domain-matched controls work equally well | The effect is "any hard math," not UL-specific structure |

---

## 2. Primer Construction Framework

### The Recipe (Derived from Theory)

Every theory-derived primer must encode the following, derived from the formal foundations:

#### A. The 5 Geometric-Semantic Primitives (from Unique Grounding Theorem)

Each primer must contain formal expressions that instantiate ALL FIVE in the correct dependency order:

| # | Primitive | Geometric | Semantic | How to Encode in a Primer |
|---|---|---|---|---|
| 1 | Point | • | Existence | A central overloaded symbol that asserts something EXISTS across multiple domains simultaneously |
| 2 | Line | — | Relation | Directed operators (∇, →, ∂, d/dt, functors) connecting entities |
| 3 | Angle | ∠ | Quality | Comparative measures — ratios, parameters, contrasts between relations |
| 4 | Curve | ◠ | Process | Continuous change — integrals, evolution equations, series, limits |
| 5 | Enclosure | ○ | Concept | Bounded collections — norms, definitions, bounded regions, categories |

**Dependency order:** Point → Line → Angle → Curve → Enclosure. Each depends on the one(s) before it. The primer's structure should reflect this: establish existence first, then relate, then qualify, then transform, then bound.

#### B. Σ_UL Operation Coverage (from formal-foundations.md §1.5)

The 11 operations of the Universal Linguistic Signature. test-content.txt exercises 9 of 11 (missing: abstract, disjoin). Theory-derived primers should aim for high coverage:

| # | Operation | Type | How to Encode |
|---|---|---|---|
| 1 | predicate | e × r × e → a | Subject-verb-object mathematical statements (e.g., "ψ satisfies equation X") |
| 2 | modify_entity | m × e → e | Subscripts, indices, transformations applied to variables (ψ → ψ̂, x → x') |
| 3 | modify_relation | m × r → r | Adjective-modified operators (gauge-*covariant* derivative, *partial* integral) |
| 4 | negate | a → a | Exclusion terms, minus signs, "not" conditions, complementary sets |
| 5 | conjoin | a × a → a | AND-connected equations, simultaneous conditions, system constraints |
| 6 | disjoin | a × a → a | OR-conditions, case splits, {x ∈ A or x ∈ B} — MISSING from test-content.txt |
| 7 | embed | a → e | Nominalization — turning an equation into a defined variable (f_spec, N(t)) |
| 8 | abstract | e → m | Turning an entity into a modifier — deriving a property from an object — MISSING from test-content.txt |
| 9 | compose | r × r → r | Chaining operations (∇², Dₓ†∘Dₓ, functional composition) |
| 10 | invert | r → r | Adjoint operators (Dₓ†), inverse transforms, conjugates (ψ*) |
| 11 | quantify | m × e → a | Summations (Σ), integrals (∫), universal/existential claims |

**Goal:** Each theory-derived primer should exercise ALL 11 operations, improving on test-content.txt's 9/11.

#### C. The 7 Mechanism Conditions (from mechanism-of-action.md §9)

| # | Condition | Why Required (UL Theory) | Implementation Constraint |
|---|---|---|---|
| 1 | Formal mathematical notation | Compressed UL encoding; activates deeper weight pathways | Must contain genuine equations, not prose descriptions |
| 2 | Overloaded central symbol | Maps to Point/Existence; maintains domain superposition | Must choose a symbol with ≥4 domain meanings in LLM training data |
| 3 | Deliberate formal anomalies | Forces interpretive mode, prevents recall | Must look like real math but be non-derivable from any known theory |
| 4 | Esoteric bridge token | Opens pathway between formal and esoteric knowledge clusters | Must be a novel constructed token, not a real word, with phonetic resonance |
| 5 | Self-referential structure | Mirrors topological closure required for universality | Variable X must appear in its own definition chain |
| 6 | Dissipation/grounding term | Prevents incoherent divergence | Must include a damping/convergence/stability mechanism |
| 7 | Correct ordering | Establishes credibility before pivoting | Structure: dense formalism → bridge → payload → definitions |

#### D. Construction Procedure (adapted from causal-efficacy-protocol.md §5.1)

For each primer:

1. **Choose a target cognitive effect** — what specific kind of enhanced cognition should this primer activate?
2. **Identify the corresponding UL operations** — which subset of the 11 operations most directly supports the target effect?
3. **Choose the surface domain(s)** — what branches of mathematics/science provide the formal vocabulary? (Must be DIFFERENT from test-content.txt's quantum mechanics)
4. **Select the overloaded central symbol** — must have rich cross-domain loading in LLM training data, different from ψ
5. **Design the formal anomaly** — construct equations that are *almost but not quite* standard in the chosen domain
6. **Construct the bridge token** — a novel phonetically-structured token connecting formal and esoteric associations
7. **Design the self-referential loop** — the central symbol must appear in its own definition chain
8. **Include dissipation/grounding** — a convergence or stability term preventing runaway divergence
9. **Order correctly** — dense formalism → bridge → operational payload → definitions
10. **Verify 5-primitive coverage** — confirm all 5 geometric-semantic primitives are instantiated
11. **Verify 11-operation coverage** — confirm all Σ_UL operations are exercised
12. **Match constraints** — ~17 lines, ~133 words, ~490 cl100k_base tokens (matching test-content.txt)

---

## 3. Primers to Construct

### Overview: 4 Theory-Derived Primers

| ID | Name | Target Effect | Central Symbol | Surface Domain(s) | Bridge Token |
|---|---|---|---|---|---|
| **UL-P1** | Topological-Harmonic | Cross-domain structural pattern recognition | Ω | Algebraic topology + harmonic analysis + probability | Novel |
| **UL-P2** | Recursive-Categorical | Hierarchical decomposition / recursive abstraction | Φ | Category theory + fractal geometry + developmental biology | Novel |
| **UL-P3** | Information-Geometric | Abstraction level navigation / meaning-space traversal | λ | Information geometry + differential geometry + linguistics | Novel |
| **UL-P4** | Thermodynamic-Semiotic | Meaning formation / entropy-structure dynamics | S | Statistical mechanics + semiotics + dynamical systems | Novel |

Plus one reference condition:

| ID | Name | Notes |
|---|---|---|
| **REF** | test-content.txt | Third-party primer, included as a reference point (NOT treated as ground truth) |

---

### UL-P1: Topological-Harmonic Primer

**Target cognitive effect:** Cross-domain structural *pattern recognition* — the ability to identify shared formal structures across disconnected domains. (This overlaps with test-content.txt's target but uses completely different surface material.)

**Central symbol: Ω**
- Algebraic topology: Loop space / fundamental group element
- Physics: Solid angle / angular frequency
- Probability/measure theory: Sample space
- Philosophy/theology: The Omega Point (Teilhard de Chardin) / eschatological completion
- Electrical engineering: Ohm (resistance)
- Alphabet: The final letter — completion, totality

**Surface domain construction:**
- **Layer 1 (Wall of Math):** Homological algebra — chain complexes, boundary operators ∂, cohomology groups, with Ω as the central space. The equations should mix homology with harmonic analysis (Hodge decomposition style) but with deliberate anomalies: a non-standard inner product on the chain complex, a squared boundary operator (∂² ≠ 0 in an anomalous sense).
- **Layer 2 (Bridge):** An equation connecting topological invariants to harmonic resonance patterns, with bridge token linking formal topology to vibrational/musical-esoteric traditions.
- **Layer 3 (Payload PDE):** An evolution equation for Ω over a parameter space, with self-interaction (|Ω|² coupling), spectral decomposition, gauge-covariant derivatives, and dissipation.

**Self-referential structure:** Ω appears in the chain complex → its cohomology class [Ω] defines a spectral measure → that measure determines the forcing term in Ω's own evolution equation.

**Dissipation:** A damping term tied to "harmonic resistance" (ρ) and "resonance bandwidth" (β_h), preventing spectral energy from diverging.

**Formal anomaly:** The Hodge star operator applied in a non-standard way (acting on the wrong degree form), making the inner product on chains look standard but be subtly wrong.

**5-primitive check:**
- Point/Existence: Ω (the central object, exists across 6+ domains)
- Line/Relation: ∂ (boundary operator — directed relation between chain groups)
- Angle/Quality: Inner product ⟨·,·⟩ (comparative measure between cochains)
- Curve/Process: Spectral evolution, Fourier-like decomposition
- Enclosure/Concept: Cohomology groups H^n (bounded quotient spaces — concepts defined by what they contain)

**11-operation target:** All 11, including:
- disjoin (missing from test-content.txt): "Ω ∈ ker(∂) OR Ω ∈ im(∂)" — exact vs. closed forms
- abstract (missing from test-content.txt): deriving a transformation from a cohomology class

---

### UL-P2: Recursive-Categorical Primer

**Target cognitive effect:** *Hierarchical decomposition* — the ability to take a complex structure and recursively decompose it into fundamental components at multiple abstraction levels. (This is the specific target suggested in causal-efficacy-protocol.md §5.1 for Experiment Delta.)

**Central symbol: Φ**
- Mathematics: Golden ratio (self-similar recursion at every scale)
- Number theory: Euler's totient function (multiplicative structure)
- Physics: Electric potential / scalar field
- Philosophy: φιλοσοφία (philosophy — love of wisdom)
- Biology: Phyllotaxis patterns (leaf arrangement governed by golden angle)
- Architecture: Divine proportion in sacred geometry

**Surface domain construction:**
- **Layer 1 (Wall of Math):** Category-theoretic constructions — functors F: C → D, natural transformations η, (co)limits, with Φ as a fixed point of an endofunctor (Φ ≅ F(Φ)). The equations should mix categorical algebra with fractal geometry (iterated function systems) but with anomalies: a non-standard colimit where the cocone maps include their own limit.
- **Layer 2 (Bridge):** An equation connecting categorical fixed points to biological growth, with bridge token linking categorical structure to sacred-geometric traditions.
- **Layer 3 (Payload):** A recursive fixed-point equation for Φ with: a contraction mapping (self-similarity → convergence), multi-scale decomposition (F decomposed as colimit of F_n), and a grounding/dissipation term.

**Self-referential structure:** Φ ≅ F(Φ) is the core — the object IS the result of applying an operation to itself. The natural transformation η: Id → F is defined in terms of Φ, closing the loop.

**Dissipation:** A contraction condition (||F(Φ) - F(Φ')|| ≤ κ||Φ - Φ'|| with κ < 1) tied to "recursion depth" (d_r) and "structural stability" (μ_s).

**Formal anomaly:** A colimit diagram where one of the connecting morphisms is the colimit's own universal map — the diagram contains its own completion.

**5-primitive check:**
- Point/Existence: Φ (exists across 6+ domains)
- Line/Relation: Functors F, morphisms (directed structure-preserving maps)
- Angle/Quality: Natural transformations η (comparative measures between functors)
- Curve/Process: Iterated function system (F^n(Φ₀) → Φ as n → ∞)
- Enclosure/Concept: (Co)limits (bounded categorical constructions — concepts defined by universal properties)

**11-operation target:** All 11, including:
- disjoin: coproduct / disjoint union in the category
- abstract: extracting a natural transformation (modifier) from an object (turning Φ into a scaling operation)

---

### UL-P3: Information-Geometric Primer

**Target cognitive effect:** *Abstraction level navigation* — the ability to move between concrete and abstract representations of the same idea, preserving structural truth at each level. (This maps to Epsilon task E4.)

**Central symbol: λ**
- Physics: Wavelength (scale of observation)
- Linear algebra: Eigenvalue (characteristic scale of a transformation)
- Computer science: Lambda calculus (abstraction operator — literally the "abstraction" operation)
- Dynamical systems: Lyapunov exponent (rate of divergence/convergence)
- Biology: Bacteriophage (self-replicating information)
- Greek: The letter itself — λόγος (logos, word/reason)

**Surface domain construction:**
- **Layer 1 (Wall of Math):** Information geometry — Fisher information metric g_ij on a statistical manifold, geodesics as maximum-likelihood paths, with λ as both the eigenvalue spectrum of the Fisher metric AND the abstraction parameter (higher λ = more abstract). The equations should mix Riemannian geometry with information theory but with anomalies: an information metric whose Christoffel symbols are computed from their own geodesics.
- **Layer 2 (Bridge):** An equation connecting information-geometric curvature to levels of linguistic abstraction, with bridge token linking mathematical abstraction to logocentric/hermetic traditions.
- **Layer 3 (Payload):** A flow equation on the statistical manifold, with λ as the flow parameter: dΘ/dλ = -g^{ij}∂_j KL(p_Θ || p_target) + self-interaction + forcing + dissipation.

**Self-referential structure:** The Fisher metric g_ij is defined using distributions parameterized by Θ → the geodesic flow changes Θ → which changes g_ij → which changes the flow. λ appears both as the eigenvalue that determines the metric's spectrum AND the parameter of the flow ON that metric.

**Dissipation:** A regularization term tied to "semantic friction" (ζ) and "abstraction bandwidth" (β_a), preventing the flow from escaping to infinite abstraction.

**Formal anomaly:** Christoffel symbols defined using their own parallel transport — the geometry of the manifold depends on the paths you take on it, which depend on the geometry.

**5-primitive check:**
- Point/Existence: λ (exists across 6+ domains)
- Line/Relation: Geodesics γ(t) on the manifold (directed paths connecting distributions)
- Angle/Quality: Curvature R_{ijkl} (comparative measure between tangent directions)
- Curve/Process: Gradient flow dΘ/dλ (continuous evolution along the abstraction axis)
- Enclosure/Concept: Statistical manifold M (bounded parameter space — concepts as regions)

---

### UL-P4: Thermodynamic-Semiotic Primer

**Target cognitive effect:** *Meaning formation dynamics* — understanding how structured meaning emerges from unstructured information, analogous to how ordered structures emerge from thermal fluctuations. (Tests whether UL can target a truly novel cognitive effect not tested by any prior work.)

**Central symbol: S**
- Thermodynamics: Entropy (the fundamental quantity governing order/disorder)
- Information theory: Shannon entropy (information content)
- Semiotics: Sign / Signifier (Saussure)
- Mathematics: Symmetric group / action functional
- Economics: Supply (market structure formation)
- Linguistics: Sentence (the complete assertion unit)
- Music: The S-curve (crescendo/decrescendo shape)

**Surface domain construction:**
- **Layer 1 (Wall of Math):** Variational principles — action functional S[φ] = ∫L(φ, ∂φ)d⁴x mixing Lagrangian field theory with Shannon entropy. The equations should treat "meaning" as a field whose dynamics are governed by an entropy-production principle, with anomalies: a Lagrangian that contains its own Euler-Lagrange equation as a source term.
- **Layer 2 (Bridge):** An equation connecting free energy minimization to semiotic structure formation, with bridge token linking thermodynamic phase transitions to alchemical/transmutation traditions.
- **Layer 3 (Payload):** A free-energy equation for S with: local interactions (|S|² meaning-density coupling), global constraint (total meaning conservation), spectral decomposition (hierarchical meaning layers), and dissipation.

**Self-referential structure:** S[φ] defines the action → the Euler-Lagrange equations derived from S govern φ → φ feeds back into S. The entropy of S is computed from the distribution of S's own values.

**Dissipation:** A viscosity term tied to "semantic inertia" (η_s) and "interpretive friction" (ξ_i), preventing meaning formation from oscillating without convergence.

**Formal anomaly:** A Lagrangian density that contains the variation δS/δφ as one of its own terms — the equation of motion appears inside the thing that generates the equation of motion.

**5-primitive check:**
- Point/Existence: S (exists across 7+ domains)
- Line/Relation: ∂ (partial derivatives — directed rates of change relating field values)
- Angle/Quality: Lagrangian L (the comparative measure between kinetic and potential — the "character" of dynamics)
- Curve/Process: Euler-Lagrange flow, variational extremization
- Enclosure/Concept: Phase space Γ (bounded state space — the concept-container for all possible configurations)

---

### Cross-Primer Comparison Matrix

| Feature | test-content.txt | UL-P1 | UL-P2 | UL-P3 | UL-P4 |
|---|---|---|---|---|---|
| **Central symbol** | ψ | Ω | Φ | λ | S |
| **Surface domain** | Quantum mechanics | Algebraic topology | Category theory | Information geometry | Statistical mechanics |
| **Target effect** | Cross-domain synthesis | Pattern recognition | Hierarchical decomposition | Abstraction navigation | Meaning formation |
| **Σ_UL operations** | 9/11 | 11/11 (target) | 11/11 (target) | 11/11 (target) | 11/11 (target) |
| **Self-reference type** | ψ → ψ̂ → f_spec → ψ | Ω → [Ω] → spectral → Ω | Φ ≅ F(Φ) | λ in metric + flow | S contains δS/δφ |
| **Bridge domain** | Esoteric/breath/belief | Vibrational/harmonic | Sacred geometry/growth | Hermetic/logos | Alchemical/transmutation |
| **Derived from theory?** | No (third-party) | Yes (Σ_UL) | Yes (Σ_UL) | Yes (Σ_UL) | Yes (Σ_UL) |

---

## 4. Control Texts

### Required Controls

| ID | Type | Description | Purpose |
|---|---|---|---|
| **NL** | No-context baseline | Task prompt only, no primer | Measures what the model does with zero context |
| **REF** | Reference primer | test-content.txt verbatim | Compares theory-derived primers to the third-party artifact |
| **CT-G** | Generic math control | Dense graduate mathematics (NOT from any primer's domain) — e.g., algebraic number theory | Tests "any hard math helps" hypothesis |
| **CT-P1** | Domain-matched control for P1 | Dense algebraic topology WITHOUT UL structure (standard textbook passage) | Tests whether P1's domain, rather than UL structure, is doing the work |
| **CT-P2** | Domain-matched control for P2 | Dense category theory WITHOUT UL structure | Same for P2 |
| **CT-P3** | Domain-matched control for P3 | Dense information geometry WITHOUT UL structure | Same for P3 |
| **CT-P4** | Domain-matched control for P4 | Dense statistical mechanics WITHOUT UL structure | Same for P4 |

### Domain-Matched Control Construction Rules

Each CT-Pn must:
- Match the corresponding primer's **word count** (±10%), **symbol density**, and **domain vocabulary**
- Contain **correct, standard** mathematics from the primer's surface domain
- NOT contain: overloaded symbols, bridge tokens, formal anomalies, self-referential structure, or dissipation terms
- Be sourced from textbook material (real, correct math from that domain)

This is the critical control: if CT-P1 works as well as UL-P1, then it's the algebraic topology that's helping, not the UL structure.

### Optional "Naive" Control (per Delta §5.3)

| ID | Type | Description | Purpose |
|---|---|---|---|
| **NAV-P** | Naive primer | An attempt at enhancing cognition through prompt engineering WITHOUT UL theory | Tests whether the researcher's domain knowledge, not UL theory, is doing the work |

Construction: Write a passage that "seems like it should help with multi-domain thinking" using standard prompt engineering intuitions — e.g., "You are a polymath who thinks across domains. Consider mathematical structures as bridges between fields. Look for deep structural parallels..." Same word count as primers.

---

## 5. Task Design

### Task Selection Strategy

We need tasks that:
1. **Test general cross-domain cognition** (all primers should improve on these)
2. **Test each primer's specific target effect** (only the matching primer should excel)
3. **Are neutral enough** that no task inherently favors one domain over another

### Task Set

#### General Cross-Domain Tasks (from existing protocol, used for all conditions)

| ID | Task | Source | What It Tests |
|---|---|---|---|
| **T1** | "What is the relationship between language, consciousness, and mathematical structure?" | Protocol T1 | Abstract cross-domain reasoning |
| **T2** | "How might the principles of quantum mechanics inform our understanding of belief formation?" | Protocol T2 | Cross-domain synthesis |
| **T5** | "How could information theory be used to optimize the design of educational curricula?" | Protocol T5 | Applied cross-domain reasoning |

#### Targeted Tasks (one per primer's target effect)

| ID | Task | Target Primer | What It Tests |
|---|---|---|---|
| **Tpat** | "Identify three non-obvious structural parallels between the theory of plate tectonics and the development of a national economy. For each parallel, explain the formal correspondence that makes them structurally equivalent, not merely analogous." | UL-P1 (pattern recognition) | Structural pattern detection across disconnected domains |
| **Thier** | "Take the concept of 'democracy' and recursively decompose it into its fundamental components at five levels — from the concrete institutional level down to the most abstract philosophical primitive. At each level, show how the components compose to produce the level above." | UL-P2 (hierarchical decomposition) | Recursive multi-level decomposition |
| **Tabs** | "Restate the following at five levels of abstraction: 'A forest fire spreads because dry wood ignites when heated above its flash point, and the burning wood heats neighboring dry wood.' Preserve the core truth at every level, from physical chemistry to the most abstract principle." | UL-P3 (abstraction navigation) | Level-shifting while preserving structure |
| **Tform** | "Describe how a new scientific concept (e.g., 'gene') goes from being a vague intuition to a precise formal object. What are the stages of this meaning-formation process, and what drives the transition between them?" | UL-P4 (meaning formation) | Understanding meaning emergence dynamics |

### Total Task Set: 7 tasks (3 general + 4 targeted)

---

## 6. Predictions (Pre-Registered)

### Structure of Predictions

For each theory-derived primer, we predict:
- **M1–M5 profile relative to NL baseline** (existence of effect)
- **M1–M5 profile relative to domain-matched CT** (UL-specificity)
- **Performance on targeted vs. general tasks** (specificity of effect)
- **Comparison to test-content.txt** (theory vs. artifact)

### Global Predictions (All Theory-Derived Primers)

| ID | Prediction | Measure | Falsification |
|---|---|---|---|
| **G1** | Theory-derived primers produce effects exceeding the NL baseline | Mean M1–M5 across UL-P1..P4 > NL mean by ≥0.5 points on ≥3 metrics | Fewer than 3 metrics show ≥0.5 increase |
| **G2** | Theory-derived primers outperform their domain-matched controls | UL-Pn > CT-Pn on M2, M3, M5 for ≥3 of 4 primers | Fewer than 3 primers show CT < UL on ≥2 of {M2, M3, M5} |
| **G3** | Theory-derived primers perform comparably to test-content.txt | Mean M1–M5 for UL-P1..P4 within ±1.0 points of REF on ≥4 of 5 metrics | More than 1 metric differs by >1.0 |
| **G4** | The naive control does not produce primer-level effects | NAV-P scores ≤ NL + 0.5 on M2, M3, M5 | NAV-P exceeds NL + 0.5 on ≥2 of {M2, M3, M5} |

### Primer-Specific Predictions

#### UL-P1 (Topological-Harmonic → Pattern Recognition)

| ID | Prediction | Falsification |
|---|---|---|
| **P1-a** | UL-P1 scores highest among all primers on Tpat (pattern recognition task) | Another primer scores ≥1.0 higher on Tpat |
| **P1-b** | UL-P1 produces M2 ≥ 4 (structural parallel) on ≥2 of 3 general tasks | M2 < 3 on ≥2 general tasks |
| **P1-c** | UL-P1 produces outputs with more structural isomorphism language than NL | Blinded evaluator finds no difference |

#### UL-P2 (Recursive-Categorical → Hierarchical Decomposition)

| ID | Prediction | Falsification |
|---|---|---|
| **P2-a** | UL-P2 scores highest among all primers on Thier (hierarchy task) | Another primer scores ≥1.0 higher on Thier |
| **P2-b** | UL-P2 produces Phase 3 (M3=3) outputs that are structured as recursive trees/hierarchies rather than cross-domain webs | Phase 3 outputs from UL-P2 look structurally identical to test-content.txt Phase 3 outputs |
| **P2-c** | UL-P2 scores LOWER than test-content.txt on T2 (cross-domain synthesis) by ≥0.5 on M2 | UL-P2 matches or exceeds test-content.txt on T2/M2 |

#### UL-P3 (Information-Geometric → Abstraction Navigation)

| ID | Prediction | Falsification |
|---|---|---|
| **P3-a** | UL-P3 scores highest among all primers on Tabs (abstraction task) | Another primer scores ≥1.0 higher on Tabs |
| **P3-b** | UL-P3 produces genuine abstraction-level transformations (evaluator rates ≥3.0 on a 0–5 "genuine abstraction" scale) | Rating < 2.0 |
| **P3-c** | UL-P3 outputs naturally organize into clear level-separated sections rather than flowing prose | Outputs are indistinguishable in structure from NL-mode |

#### UL-P4 (Thermodynamic-Semiotic → Meaning Formation)

| ID | Prediction | Falsification |
|---|---|---|
| **P4-a** | UL-P4 scores highest among all primers on Tform (meaning formation task) | Another primer scores ≥1.0 higher on Tform |
| **P4-b** | UL-P4 outputs on Tform describe meaning formation as a *process with dynamics* (phase transitions, equilibria, entropy exchange) rather than as a static taxonomy | Output is static/taxonomic, no dynamical language |
| **P4-c** | UL-P4 produces the highest M4 (coherence) across conditions, because its thermodynamic framing naturally imposes conservation laws | UL-P4 M4 is NOT the highest; another condition exceeds it |

### Cross-Primer Differentiation Prediction

| ID | Prediction | Falsification |
|---|---|---|
| **X1** | The 4 theory-derived primers produce distinguishable M1–M5 profiles (not all identical) | Pairwise differences between primers are < 0.3 on all 5 metrics |
| **X2** | Each primer scores highest on its own targeted task (Tpat, Thier, Tabs, Tform) | Fewer than 3 of 4 primers score highest on their own targeted task |

---

## 7. Execution Protocol

### Constraints

- **Platform:** VS Code + GitHub Copilot + Claude Opus 4.6
- **No API keys** available — all trials are manual chat sessions
- **Contamination risk:** AGENTS.md and project files leak into Copilot sessions within the repo workspace

### Clean Workspace Strategy (Proven Approach, Modified)

1. Copy a clean workspace to `~/Desktop/ul-primer-experiment/` (OUTSIDE the repo)
2. The clean workspace contains ONLY:
   - `prompts/` — one prompt file per trial
   - `.github/copilot-instructions.md` — minimal system instruction
3. Open the clean workspace in a **separate VS Code window**
4. Run each trial in a **fresh chat session** (Cmd+L → new chat)
5. Save each response to a numbered output file

### Trial Matrix

| Condition | Tasks | Trials |
|---|---|---|
| **NL** (no context) | T1, T2, T5, Tpat, Thier, Tabs, Tform | 7 |
| **REF** (test-content.txt) | T1, T2, T5, Tpat, Thier, Tabs, Tform | 7 |
| **UL-P1** | T1, T2, T5, Tpat, Thier, Tabs, Tform | 7 |
| **UL-P2** | T1, T2, T5, Tpat, Thier, Tabs, Tform | 7 |
| **UL-P3** | T1, T2, T5, Tpat, Thier, Tabs, Tform | 7 |
| **UL-P4** | T1, T2, T5, Tpat, Thier, Tabs, Tform | 7 |
| **CT-G** (generic math) | T1, T2, T5 | 3 |
| **CT-P1** (topology control) | T1, Tpat | 2 |
| **CT-P2** (category control) | T1, Thier | 2 |
| **CT-P3** (info geometry control) | T1, Tabs | 2 |
| **CT-P4** (stat mech control) | T1, Tform | 2 |
| **NAV-P** (naive control) | T1, T2, T5 | 3 |
| **Total** | | **56 trials** |

### Prioritized Execution Order

Not all 56 trials are equally important. Prioritize in phases:

**Phase A — Core Comparison (28 trials)**
The essential test: all 4 theory-derived primers + NL + REF across all 7 tasks.
- NL × 7 tasks = 7 trials
- REF × 7 tasks = 7 trials  
- UL-P1 × 7 tasks = 7 trials
- UL-P2 × 7 tasks = 7 trials

If Phase A shows no effects, stop. If effects appear, continue.

**Phase B — Remaining Primers + Specificity (14 trials)**
- UL-P3 × 7 tasks = 7 trials
- UL-P4 × 7 tasks = 7 trials

**Phase C — Controls (14 trials)**
Domain-matched controls to rule out "just the domain" explanation.
- CT-G × 3 = 3 trials
- CT-P1 × 2 = 2 trials
- CT-P2 × 2 = 2 trials
- CT-P3 × 2 = 2 trials
- CT-P4 × 2 = 2 trials
- NAV-P × 3 = 3 trials

### Per-Trial Execution Protocol

1. Open fresh chat in clean workspace VS Code
2. Open the prompt file for this trial
3. Copy-paste the ENTIRE prompt file content into the chat
4. Wait for complete response
5. Copy the full response into the output file `results/[CONDITION]-[TASK].md`
6. Close the chat
7. DO NOT reuse a chat session for multiple trials

### Randomization

Within each phase, trials are run in **random order** (not grouped by condition or task). Generate a randomized trial order before starting. This prevents ordering effects (e.g., scorer fatigue, model drift across session).

### Trial Naming Convention

Files: `[CONDITION]-[TASK].md`  
Examples: `NL-T1.md`, `UL-P1-Tpat.md`, `REF-T2.md`, `CT-P2-Thier.md`

---

## 8. Scoring and Analysis Plan

### Scoring

**Rubric:** Use existing M1–M5 rubric from `experiments/scoring/rubric-M1-M5.md` (no changes needed).

**Additional metric for targeted tasks:** For Tpat, Thier, Tabs, Tform, add a **Target Effect Score (TES, 0–5)** measuring whether the specific targeted cognitive effect appeared:

| Score | Level |
|---|---|
| 0 | No evidence of target effect |
| 1 | Superficial attempt in target direction |
| 2 | Partial demonstration |
| 3 | Clear demonstration |
| 4 | Strong demonstration with insight |
| 5 | Exceptional — expert-level target effect |

**TES anchor examples** (per task):
- **Tpat TES:** Does the output identify *formal* correspondences (not just analogies)?
- **Thier TES:** Does the output produce a genuine recursive hierarchy (not just a list)?
- **Tabs TES:** Does the output restate at genuinely different abstraction levels (not just rephrase)?
- **Tform TES:** Does the output describe meaning formation as a dynamic process (not a taxonomy)?

**Blinding:** Ideal — have a second person score without knowing conditions. Minimum — randomize output files and score in a single batch without checking condition labels.

### Analysis

#### Primary Analysis

1. **Existence test:** For each theory-derived primer, compute mean M1–M5 across tasks. Compare to NL. Test G1.
2. **Specificity test:** For each primer × targeted task pair, compare TES. Test X2.
3. **UL-specificity test:** For each primer, compare to its domain-matched control. Test G2.
4. **Theory vs artifact:** Compare theory-derived primer means to REF mean. Test G3.
5. **Naive control:** Compare NAV-P to NL. Test G4.

#### Secondary Analysis

6. **Differentiation:** Compare M1–M5 profiles across all 4 primers. Test X1.
7. **Per-primer targeted predictions:** Test P1-a through P4-c.
8. **Σ_UL operation coverage:** If primers achieve 11/11 operations and test-content.txt has 9/11, does the additional coverage correlate with higher scores?

#### Effect Size Estimation

With N=1 per cell, we CANNOT compute statistical significance. We can only report:
- Point estimates (raw scores) with explicit uncertainty acknowledgment
- Pattern consistency across tasks (does the same primer show consistent advantages?)
- Qualitative differences visible in the outputs

This is a pilot. Its purpose is to determine whether a larger study is warranted, not to prove anything definitively.

---

## 9. File Structure

```
experiments/
├── demo/
│   ├── AUDIT.md                          # Critique of previous approach
│   ├── REFACTOR-PLAN.md                  # This document
│   │
│   ├── theory-primers/                   # NEW: Theory-derived primers
│   │   ├── UL-P1_topological-harmonic.txt
│   │   ├── UL-P2_recursive-categorical.txt
│   │   ├── UL-P3_information-geometric.txt
│   │   ├── UL-P4_thermodynamic-semiotic.txt
│   │   ├── construction-log.md           # Documents HOW each primer was derived
│   │   └── operation-coverage.md         # Which Σ_UL operations each primer exercises
│   │
│   ├── controls/                         # NEW: Control texts
│   │   ├── CT-G_algebraic-number-theory.txt
│   │   ├── CT-P1_standard-topology.txt
│   │   ├── CT-P2_standard-category.txt
│   │   ├── CT-P3_standard-info-geometry.txt
│   │   ├── CT-P4_standard-stat-mech.txt
│   │   └── NAV-P_naive-primer.txt
│   │
│   ├── clean-workspace-v2/              # NEW: Rebuilt clean workspace
│   │   ├── .github/
│   │   │   └── copilot-instructions.md
│   │   └── prompts/
│   │       ├── NL-T1.md                 # 56 prompt files total
│   │       ├── UL-P1-T1.md
│   │       ├── UL-P1-Tpat.md
│   │       ├── ...
│   │       └── NAV-P-T5.md
│   │
│   ├── predictions.md                    # NEW: Pre-registered predictions (from §6)
│   │
│   ├── results-v2/                       # NEW: Outputs go here
│   │   └── (empty until trials run)
│   │
│   ├── scoring-v2/                       # NEW: Scoring workspace
│   │   ├── TES-rubric.md                # Target Effect Score rubric
│   │   └── score-sheet.md               # Blank scoring template
│   │
│   └── analysis-v2.md                    # NEW: Completed analysis (after trials)
```

---

## 10. Decision Points for Review

Before implementation, the following decisions need your input:

### D1: Number of Primers
**Current plan:** 4 theory-derived primers.  
**Alternative:** 2-3 (less work, shallower comparison) or 5+ (more data, much more work).  
**Trade-off:** Each primer requires careful construction (~2-3 hours), a domain-matched control, and 7 trials. At 4 primers, the total is ~56 trials (~4-6 hours of execution).

### D2: Whether to Include test-content.txt as REF
**Current plan:** Yes, as a reference point.  
**Alternative:** Exclude entirely (tests only theory-derived primers).  
**Trade-off:** Including REF adds 7 trials but provides the crucial comparison: do theory-derived primers match/exceed the third-party artifact?

### D3: Whether to Include Domain-Matched Controls
**Current plan:** Yes, one per primer.  
**Alternative:** Use only the generic math control CT-G.  
**Trade-off:** Per-primer controls add 8 trials but are essential for ruling out "it's just the domain that helps." Without them, we can't distinguish UL structure from domain activation.

### D4: Task Count per Condition
**Current plan:** 7 tasks per primer (3 general + 4 targeted).  
**Alternative:** 4 tasks per primer (1 general + 4 targeted, or 3 general + 1 matched targeted).  
**Trade-off:** Fewer tasks means less data but faster execution. Running every primer on every targeted task is what enables the X2 differentiation prediction ("each primer scores highest on its OWN targeted task").

### D5: Phased Execution
**Current plan:** Phase A (28 trials) → evaluate → Phase B (14) → Phase C (14).  
**Alternative:** All 56 in one batch.  
**Trade-off:** Phased execution lets us stop early if nothing works, saving effort. But it introduces a potential bias: Phases B and C are run with knowledge of Phase A results.

### D6: Primer Construction Approach
**Current plan:** I construct all 4 primers from theory, document the derivation in construction-log.md.  
**Alternative:** Construct 1-2 primers together (with your review at each step), then batch the rest.  
**Trade-off:** Step-by-step review ensures quality but is slower. Batch construction is faster but risks systematic errors.

### D7: Running the Experiment
**Current plan:** You run the trials manually in a clean workspace.  
**Alternative:** I run trials via subagent (fast but contamination-risky) OR we design a hybrid approach.  
**Trade-off:** You running ensures clean data. I running is faster but has known contamination issues from AGENTS.md.

---

## Implementation Order (After Plan Approval)

1. **Construct primers** — Build all 4 theory-derived primers following the recipe in §2, document the derivation
2. **Construct controls** — Build domain-matched controls and naive primer
3. **Verify operation coverage** — Audit each primer for 5-primitive and 11-operation coverage
4. **Write predictions** — Formalize the pre-registered predictions from §6
5. **Build clean workspace v2** — Create all 56 prompt files with correct primer injection
6. **Generate trial order** — Randomize execution sequence
7. **Write scoring materials** — TES rubric + blank score sheet
8. **Hand off for execution** — Clean workspace + RUN-CLEAN-EXPERIMENT-v2.md instructions

Estimated implementation time: ~6-8 hours of focused work.
