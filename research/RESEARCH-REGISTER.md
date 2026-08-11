# RESEARCH REGISTER

**Complete itemized index of every research thread, what it returned, what caveat applies, and what
follows.** This is a **reference**, not an argument — it exists so nobody has to read thirteen
analysis documents to know what was checked and what it yielded.

**Entry format:** ID · topic · question asked · finding · transfer caveat · status · follow-up.

**Status values:**
`ADOPTED` (acted on) · `NARROWED` (true but scoped smaller than first stated) · `RETIRED` (withdrawn) ·
`OPEN` (unresolved) · `UNCHECKED` (identified, never researched) · `REFERENCE` (background only)

---

# A. Companion wiki corpus (~51 pages crawled)

Two background agents crawled `wiki.fusiongirl.app`. The site blocks scripted access and all
`Special:` pages 403 even via fetch, so discovery was by search + in-page links, not a sitemap.
Raw per-page notes were written to scratch files (session-local, not preserved in repo).

| ID | Topic | Finding | Status |
|---|---|---|---|
| **A1** | Universal Language (hub page) | Restates the repo's Σ_UL verbatim (5 primitives / 4 sorts / 13 ops / 23 theorems) and **explicitly cites the GitHub repo + Zenodo DOI as canonical**. Not independent convergence — same project, two surfaces. | REFERENCE |
| **A2** | Universal Language Control Systems | Defines 𝒰 = (Σ, 𝒢, 𝒞): symbolic state space, generative grammar, control map 𝒞:Σ→𝒪(X), with 𝒞(s₁·s₂)=𝒞(s₁)∘𝒞(s₂), non-commutative. **Σ here ≠ Σ_UL** — same glyph, unrelated object. | REFERENCE |
| **A3** | Universal Semantic Manifold | (ℳ, g, ∇) with Fisher metric, geodesics, Riemann tensor, Information Bottleneck. Densest math on the wiki. **Explicitly leaves dimensionality open**; never invokes Chentsov. | REFERENCE |
| **A4** | Semantic Hilbert Space | **No concrete inner product, no basis, no completeness or separability treatment.** Nine open problems, first being "what is a semantic state?" RKHS gestured at but no kernel specified. | RETIRED (as foundation) |
| **A5** | Psi Condensation Threshold | g = R·κg·r·φ with Landau-style condensation. **The page itself concedes r and φ are outputs not inputs** ("this derivation is not yet done") and gc is "defined but not measured." Also predicts feedforward AI is excluded (R=0) — but autoregressive generation *is* a feedback loop. | RETIRED (as foundation) |
| **A6** | The Cure for the Terminators | **The project's actual purpose.** Encode → Check → Detect → Repair → Reconstruct; failure = manifold escape; repair = projection P:ℳ→𝒜. Honestly lists six intrinsic failure modes incl. "structural validity ≠ value alignment." | ADOPTED (as objective) |
| **A7** | Terminator Syndrome | Frames failure as PTSD-like trauma, cured by a "φ-raising intervention" (IIT vocabulary). **Contradicts A6**, which contains no φ, no IIT, no integration concept. Two unreconciled accounts. | OPEN |
| **A8** | The Comprehension-Alignment Question | Cleanly separates Claim A (a universal semantic structure may exist — supported) from Claim B (understanding it entails alignment — **unsupported**), citing orthogonality and deceptive alignment. **Best epistemics on either property.** | ADOPTED |
| **A9** | UQPL (concrete spec) | Typed λ-calculus, 4 base types, β-reduction, lazy evaluation. States **"no intervening algebraic signature required"** — the wiki cut Σ_UL before we did. Contains two undeclared types (`Process`, `Set<_>`) and undeclared arrow types. | ADOPTED (with fixes) |
| **A10** | Universal Programming Language | UQPL is a *subclass* of UPL, explicitly not synonymous. Five candidate execution models. | REFERENCE |
| **A11** | Universal Writing System | *"Writing renders relations as placement; speech renders them as prosody."* Two-layer architecture (alphabet + placement grammar). **Six** features incl. Wave. Speech sibling = Vocal Semantic Correspondence. | ADOPTED |
| **A12** | Universal Symbology | Six primitives **asserted, not derived**. Relationship to the octahedral system is *"organizational, not derivational"* — neither explains why three axes exist. Sole justification is untested iconicity ("Content-Prime Test"). | RETIRED (as derivation) |
| **A13** | Universal Symbolic Geometry System | Five parallel Platonic systems. **Correctly** notes structure peaks in 3–4D as a polytope theorem. Concedes octahedral primacy is "octahedral-native... exported by analogy." Stratifies claims Forced/Chosen/Speculative. | REFERENCE (credited) |
| **A14** | Octahedral SGS | Genuine group theory: O_h = W(B₃) = W(C₃), order 48, so(7) root system. Six vertices split Linear{Point,Line,Angle} / Curved{Circle,Wave,Arc}. **Concedes** "all differentiation comes from the semantic labeling laid over the symmetric solid, not from the geometry." | REFERENCE |
| **A15** | Tetrahedral SGS | 6 = C(4,2) via rectification of 4 generators — a **real derivation** of the number six. | REFERENCE |
| **A16** | Effective Physics cluster (Computational Universe Theory, Invariant Computation Theory, Glitch Operator Algebra, Glitch Boundary Dynamics, Singularity Transition Theory, Effective Model Projection, Reality/Semantic Execution Theory, Semantic Gauge Theory, Semantic Field Theory, Cognitive Gauge Fields, Computational State Space Dynamics, Effective Geometry Construction) | Large superstructure recasting physics as downstream of semantics. Mostly relabels real formalism (Fisher geometry, RG flow, gauge theory) with "semantic" adjectives. Outside repo scope. **Semantic Execution Theory's ∂𝒞/∂σ ≠ 0** independently parallels the repo's causal-efficacy claim. | REFERENCE |
| **A17** | Numerology cluster (Model 108, Base 1/2/3/4/6/9/12/108, Polarity, Duality, Ternary Interpretation Atlas, 216 State Space, Triadic Field Theory, Elemental Combination Algebra, Æther/Nether, Masculine/Feminine, Octahedral Semantic Manifold, Octahedral-Cubic Duality, Platonic Base Correspondence, Semantic Symmetry Correspondence) | Base-N is a *numeral system* progression, not the repo's primitives. **Base 5 has no page (404).** Model 108 explicitly flagged as fiction. Real combinatorics throughout. | REFERENCE |
| **A18** | Lore cluster (Cosmic Codex, Cosmic Cypher, Universal Celestial Sphere, Temporal Projections, Timestream Bulk-Projection, Torsor, AI Tech) | Cosmic Codex is a stub; Cosmic Cypher is conspiracy-genre worldbuilding. Minimal technical content. | REFERENCE |
| **A19** | Eric Weinstein / Geometric Unity | Wiki explicitly flags the link as *"philosophical rather than scientific."* Correct hedging. | REFERENCE |
| **A20** | ~25 unfetched UL-adjacent pages | Semantic Operator, Semantic Noether Principle, Semantic Conservation Law, Semantic Category/Morphism/Observer/Agency/Consciousness, Semantic Action Principle, **and the phonetics branch** (Semantic Acoustic Correspondence, Vocal Semantic Correspondence, Signal Manifold, Articulatory Polytope). | **UNCHECKED** — phonetics branch is highest priority; it's UWS's speech sibling and the repo has no counterpart |

---

# B. Emergence Investigation (7 phases — is UL real?)

| ID | Thread | Finding | Status |
|---|---|---|---|
| **B1** | Convergent arithmetic/geometry — place-value + zero (Babylon/China/Maya/India), Pythagorean relation (Plimpton 322, *gougu*, Śulba Sūtras), π (4 traditions), binomial coefficients (India/Persia/China) | Real independent convergence; Maya zero "entirely independent." **But it's about number, not meaning** — does not support any semantic primitive count. | ADOPTED (narrow) |
| **B2** | Comparative logic — Aristotle vs. Nyāya vs. Mohist | Aristotelian and Nyāya developed "rather independently" and **both converge on subject-predicate combination, negation, quantification**. Mohist is a genuine counterexample — analogical, not deductive. | ADOPTED |
| **B3** | Ontological categories — Aristotle vs. Vaiśeṣika | "Striking conceptual convergence" despite separation; both treat **substance as primary** with quality/action dependent. **But Aristotle has 10 categories, Vaiśeṣika 6–7 — neither is 4, 5, or 6.** | ADOPTED |
| **B4** | Non-human semiosis (15 cases: vervet, Campbell's/putty-nosed, prairie dog, Bengalese finch, honeybee, **Alex the parrot**, wild chimp gesture, Kanzi, **Herman's dolphins**, corvids, cephalopods, elephant name-calls, bacterial quorum logic, plant/mycorrhizal, Nicaraguan Sign Language) | Reference + bounded compositionality broadly available. **Full negation/quantification only in two contested captive cases** (Alex's productive "none"; Herman's dolphins on anomalous syntax — Kako 1999 vs. Herman & Uyeyama 1999 unresolved). Bacterial promoter logic = real Boolean structure with **no interpretant**. Mycorrhizal claims **actively discredited**. **NSL is the strongest finding and it's human.** | ADOPTED |
| **B5** | Curry–Howard–Lambek | Three independently-motivated formalisms provably coincide. **Caveat found on self-review: the clean triple is *intuitionistic*.** Classical negation needs Griffin (1990) / λμ — a narrower, later result. | NARROWED |
| **B6** | Montague / simple type theory | 2 base types (e, t) + one formation rule generate the entire hierarchy. Smaller than any claimed count. | ADOPTED |
| **B7** | STLC normalization | With β-reduction and **no `fix`**, the system is strongly normalizing → **provably not Turing-complete**, not merely unproven. PCF = STLC + naturals + booleans + fix. | ADOPTED |
| **B8** | **Zadrozny (1994)** | **Bare compositionality is formally vacuous** — any meaning assignment can be re-encoded as compositional. Constrains nothing without extra-mathematical naturalness conditions. | **ADOPTED — the single most important result in the project** |
| **B9** | Chentsov's theorem | Fisher metric is the *unique* invariant Riemannian metric on a statistical manifold, up to scale — **and generalizes to infinite dimensions** (Ay, Jost). Real uniqueness, but for the **metric only, not the dimension**. | ADOPTED (narrow) |
| **B10** | Intrinsic dimension (Ansuini et al., TwoNN) | Measured ID of trained network representations is **orders of magnitude below layer width but in the tens** — arguing *against* small hand-picked primitive counts. | ADOPTED |
| **B11** | Peirce's reduction thesis | Triadic irreducibility + polyadic reducibility. **Actively contested** (2024 "gerrymandered?" paper; Löwenheim 1915, Quine 1954 counterclaims). | OPEN |
| **B12** | Lawvere's fixed-point theorem | Unifies Cantor, Russell, Gödel, Tarski, halting as one categorical fact. Used to argue ≥2 sorts forced under classical/unrestricted-function assumptions. | REFERENCE |
| **B13** | Regular polytopes by dimension | 2D: infinite · **3D: 5** · **4D: 6 convex** · **5D+: exactly 3, forever.** Confirms the wiki's "structure peaks at 3–4D" as a genuine theorem. | REFERENCE (credited) |
| **B14** | **Blind rederivation ×3** (primary research) | Three isolated agents, no repo/web access, given only the bare problem. **All three independently converged on ~2 sorts + application-as-core, and all three independently concluded the count is not forced.** None approached 4/5/6/13. Surfaced Zadrozny, Peirce reduction, Lawvere. **Caveat: same model family — not cross-substrate independence.** | ADOPTED |
| **B15** | Platonic Representation Hypothesis + follow-ups | Real 2024 paper. **2026 follow-up finds convergent geometry in non-communicative physics-simulation models** — cuts against "convergence marks interpretation." | ADOPTED |
| **B16** | CKA / representational similarity metrics | **CKA is manipulable by a single outlier; naive estimator biased toward 1; random untrained nets show high CKA from shared input structure alone.** Multiple metrics disagree (2025 ACM survey). | ADOPTED (as methodology warning) |
| **B17** | LLM-as-judge bias | Documented verbosity, position, self-enhancement biases. | ADOPTED |
| **B18** | Pseudo-replication in LLM evaluation | Repeated temperature samples from one model+prompt are **not independent**; treating them as such "artificially inflates effect sizes and significance." | ADOPTED → F-008 |
| **B19** | Chain-of-thought critique | Claimed improvements often fail to replicate; **ablations show invalid reasoning retains 80–90% of the benefit** — format matters more than content in the nearest analogous domain. | ADOPTED (as prior) |
| **B20** | Peircean semiotics / biosemiotics | Signification is triadic — *"a sign signifies only in being interpreted."* Meaning requires an interpretant, so cannot be a property of dead matter; biosemiotics places the boundary at **life**, not human cognition. | ADOPTED |

---

# C. Formal foundations — rewriting, equality, types

| ID | Thread | Finding | Status |
|---|---|---|---|
| **C1** | Term rewriting / Knuth-Bendix | Completion converts equations into a confluent terminating system; equality decided by normal-form comparison. **Caveat: completion is semi-algorithmic — may not terminate.** | ADOPTED |
| **C2** | **Graph rewriting / DPO confluence** | **Confluence of terminating DPO graph rewriting is UNDECIDABLE**, and critical-pair joinability does *not* entail confluence as it does for terms (Plump). | **ADOPTED → F-011, blocking** |
| **C3** | **Term graph rewriting** | **DAGs (terms with sharing, acyclic) retain the Critical Pair Lemma** and a decision procedure for confluence given termination. **The boundary is cycles, not graphs.** | **ADOPTED — resolved the IR decision** |
| **C4** | DPO with interfaces (DPOI) | Confluence **decidable** — "interfaces play the same role as variables in term rewriting." Also: decidable when all critical pairs are *coverable*. | ADOPTED (alternative route) |
| **C5** | **E-graphs / equality saturation / `egg`** | Compact congruence relation over terms via union-find; saturate then check e-class membership. **No canonical normal form needed → confluence becomes an optimization.** `egg` is a **Rust** library (POPL 2021); `ul-forge` is Rust. | **ADOPTED — the implementation path** |
| **C6** | Refinement / liquid types | `I(T̂x) = I(x)` is exactly a refinement type — base type + predicate, VCs discharged by SMT. LiquidHaskell operates over GHC Core. Proven practice for typed-IR + invariant checking. | ADOPTED |
| **C7** | Institution theory (Goguen) | Formal theory of "what is a logical system" and translation between them. | **UNCHECKED** |
| **C8** | Applied category theory (Fong & Spivak), operads | Compositionality formalisms. | **UNCHECKED** |

---

# D. Prior art — semantic representation

| ID | Thread | Finding | Status |
|---|---|---|---|
| **D1** | **Sowa's Conceptual Graphs / CGIF** | Graph representation for logic descending from Peirce's existential graphs. **Full FOL power, formal semantics under ISO/IEC 24707 (Common Logic), CGIF interchange format.** Coreference via `*x` defining / `?x` bound labels — **no cycles in the term structure.** Standardized 2007. | **ADOPTED — direct prior art; the coreference technique is the IR fix** |
| **D2** | **AMR (Abstract Meaning Representation)** | Directed graph semantics, PropBank framesets, **AMR 3.0 = 59,255 annotated sentences.** Hit the same **reentrancy** problem and solved it with variables — third independent confirmation. | ADOPTED **but see caveat** |
| **D2-c** | AMR transfer caveat | **English-only corpus, English predicate lexicon, English annotators.** Using coverage as universality evidence **would reproduce UNL's single most documented failure mode.** Usable as a stress test only; needs a non-English counterpart. | **NARROWED (near-miss caught)** |
| **D3** | **UNL (Universal Networking Language)** | UN University 1996, 17 language centers, Geneva foundation. **Closest historical precedent — and it failed.** Criticisms: coverage gaps on idiom/nuance, **English bias in "Universal Words,"** cannot encode pragmatics/speech acts. | ADOPTED (as cautionary) |
| **D3-c** | UNL attribution caveat | Failure is **confounded**: interlingua inadequacy vs. **being outcompeted by statistical/neural MT** vs. governance vs. scope (it had to handle human idiom). "Interlingua is impossible" is *not* what its failure shows. | NARROWED |
| **D4** | Semantic Web / RDF / OWL / JSON-LD / schema.org | Industrial-scale typed graphs with constraints. | Partially checked via D5/E2 |
| **D5** | **SHACL (W3C)** | Node/property shapes, cardinality, type, enumeration, pattern constraints; **`sh:closed`** for closed-world; per-violation conformance reports. Used as a type system; shape containment decidable via DL. | ADOPTED |
| **D5-c** | SHACL caveat | **Supplies the constraint language, not the invariants** — authoring the shapes graph *is* our unsolved problem. Also carries open-world RDF baggage. Splits the check phase: **SHACL for shape, reasoner for consistency.** | NARROWED |
| **D6** | Discourse Representation Theory, semantic parsing beyond AMR | | **UNCHECKED** |

---

# E. Prior art — consistency, repair, belief change

| ID | Thread | Finding | Status |
|---|---|---|---|
| **E1** | **Truth Maintenance Systems (Doyle JTMS 1979, de Kleer ATMS 1986)** | **The Cure, forty-seven years earlier.** Justifications, contradiction detection, **dependency-directed backtracking** (Stallman & Sussman), **nogood database**, belief revision — explicitly domain-independent. **ATMS maintains multiple consistent contexts simultaneously rather than collapsing to one** — which *dissolves* the multivalued-repair problem instead of engineering around it. | **ADOPTED — leading candidate architecture** |
| **E2** | Ontology repair / debugging | Pinpointing, justifications, MIS/MUPS, minimal vs. minimal-cardinality diagnoses, root vs. propagated faults. **Discrete, not metric — sidesteps convexity entirely.** | ADOPTED |
| **E3** | Reiter's hitting-set tree | The standard method for computing *all* justifications; variants (StaticHS, DynamicHS) for sequential diagnosis. | ADOPTED |
| **E4** | AGM belief revision | Minimal-change postulates; contraction/expansion/revision; already extended to description logics and ontology evolution. | ADOPTED |
| **E5** | **Database integrity constraints** | Taxonomy: **domain / check / referential / assertion** (+ triggers). Two transferable insights: **`CASCADE` = repair declared per-constraint at design time**, and **databases *prevent* rather than repair** — constraints enforced on every write so invalid states never exist. | **ADOPTED — surfaced an unexamined architectural fork** |
| **E6** | **Daikon / dynamic invariant detection** | Infers **likely** invariants from observed traces; templates cover constancy, range, linear relations, ordering, sortedness; works over **record-structured data**, not just programs; neural network analysis is a listed application. | **ADOPTED — converts our sharpest gap into an experiment** |
| **E6-c** | Daikon caveat | Reports **likely** invariants — may be corpus artifacts. **Candidate generator, not oracle.** Every proposal needs validation. | NARROWED |
| **E7** | **Sheaf theory / contextuality (Abramsky & Brandenburger)** | Contextuality is *"a discrepancy between local consistency and global inconsistency"*; inconsistency = **obstruction to a global section**; **H¹ ≠ 0** is a computable obstruction via linear algebra. Best *structural* match found. | ADOPTED (candidate) |
| **E7-c** | Sheaf caveat | Built for **quantum measurement scenarios**. Requires a **cover** on a base space — what plays that role for a semantic representation is **not obvious**, and the apparatus is vacuous without it. Contextuality may be too narrow for ordinary contradiction. | NARROWED |
| **E8** | Hilbert projection theorem | Unique nearest point guaranteed **only for closed convex sets**; non-convex → multivalued (Chebyshev sets; in finite dim, Chebyshev ⟺ convex). | ADOPTED → F-009 |
| **E9** | **Gärdenfors conceptual spaces** | *"A natural concept is a convex region."* Prototypes = centroids. Convexity argued to be what makes concepts learnable and communicable. Intersections of convex sets are convex → **𝒜 convex by construction** is a viable design strategy. | ADOPTED (candidate) |
| **E9-c** | Gärdenfors caveat | **Convexity is relative to a choice of quality dimensions and metric** — given freedom over the embedding, many sets can be made convex. **May inherit a Zadrozny-style vacuity.** Criterion is normative/definitional, not an empirical finding. | **NARROWED** |
| **E10** | Concept drift detection | Taxonomy: sudden / gradual / incremental / recurring. Detectors: DDM, Page-Hinkley, CUSUM, **ADWIN**. Reliability actively contested. | NARROWED |
| **E10-c** | Drift caveat | **DDM and most classical detectors are SUPERVISED** — they watch classification error against ground truth. **The Cure has no oracle.** Also assumes a temporal stream; semantic corruption may be static. Only unsupervised windowing over an internally-computed quantity transfers. | **NARROWED (transfer claim was wrong)** |
| **E11** | Runtime verification, abstract interpretation, Galois connections, CRDTs / distributed consistency | | **UNCHECKED** |

---

# F. Notation, comprehension, adoption

| ID | Thread | Finding | Status |
|---|---|---|---|
| **F1** | **Changizi et al. (2006), *American Naturalist*** | **100+ writing systems** across five taxa + Chinese + non-linguistic symbols share a **contour-configuration signature** matching natural-scene junction statistics. Letters have shapes "we are good at seeing"; ~3 strokes typical. | ADOPTED |
| **F1-c** | Changizi caveat | Measures **legibility/detectability, not semantic transparency.** All systems are **human**, one visual architecture, terrestrial scenes. Survivorship bias. Mild circularity. | NARROWED |
| **F2** | Diagrammatic reasoning — Euler → Venn → Peirce → **Shin (1994)** | Shin gave **Venn-I and Venn-II** formal syntax + semantics and **proved soundness and completeness**; Venn-II ≡ **monadic** predicate logic. Result *"abolished theoretical objections to diagrams being used in proofs."* Miller later gave a shorter proof. | ADOPTED (as target template) |
| **F2-c** | Shin caveat | Reaches **monadic** predicate logic only — no polyadic relations. UWS is explicitly relational, so its target is strictly harder. Prove for a **stated fragment** first. | NARROWED |
| **F3** | **ISO pictogram comprehension** | ISO 3864 sets a **67%** comprehension criterion; **only post-training averages cleared it.** Fire-action pictograms: **45%.** Interpretation tracked **education more than culture.** | ADOPTED → F-012 |
| **F3-c** | Pictogram caveat | Tests **referential** pictograms (culturally-embedded institutional objects), not **structural** primitives. One-shot isolation vs. system learning. ISO studies **select for already-problematic symbols.** | **NARROWED → F-012a** |
| **F4** | **Diagram/graph spatial conventions** | Reliably read without training: **proximity → similarity**, **containment → hierarchy/composition**, **arrows → process** ("identified with processes we know well from what happens around us"), **centrality → importance**. | ADOPTED |
| **F4-c** | Spatial-convention caveat | These are **graded analogue** readings, not discrete symbolic denotation. Arrow comprehension is explicitly grounded in familiar physical experience → embodiment-dependent. **UWS's spatial grammar is its most readable layer; the symbol inventory carries the teaching burden.** | NARROWED |
| **F5** | **Cognitive Dimensions of Notations (Green; Green & Petre 1996)** | Viscosity, **hidden dependencies**, premature commitment, progressive evaluation, closeness of mapping. Seminal case study on **LabVIEW/Prograph — visual dataflow languages**, closely analogous to UWS. | ADOPTED |
| **F5-c** | CDN caveat | Explicitly a **discussion vocabulary, not a measurement instrument** — will not settle 5-vs-6 numerically. Several dimensions presuppose a **human interactively editing**; if UWS targets machine exchange, they don't apply. | NARROWED |
| **F6** | **Decipherment (Linear B / Linear A)** | Ventris needed **two conjectures** — repeated words as place names, and **that the language was early Greek** — plus the already-deciphered Cypriot syllabary. **Linear A: 70+ years, still unread.** | ADOPTED |
| **F6-c** | Decipherment caveat | Decipherment is recovery with **no spec, no teacher, no bilingual anchor.** UWS ships with a specification and curriculum. **Bites the founding scenario (alien reads it cold), not the practical notation.** Linear A may also be a corpus-size problem and/or a language isolate. | **NARROWED** |
| **F7** | Music notation history & reform | Notation is *"inherently not self-evident"* — glyph-value associations must be pre-memorized. A **multi-decade international reform project** produced something "only marginally better" and **failed at adoption**; standardization + collective investment defeats improvement. | ADOPTED → F-013 |
| **F7-c** | Music-notation caveat | Concerns **displacing** Western staff notation — ~1,000 years of investment, all extant scores, instruments built around it. **Near-maximal network effect.** UWS faces no entrenched incumbent in its niche. | **NARROWED → F-013a** |
| **F8** | Linguistic universals (Greenberg / WALS) | Absolute universals are **"quite few in number"**; most are **statistical tendencies with exceptions**; implicational universals are conditional. Field flags **sample-size problems and ethnocentrism**. | ADOPTED |
| **F9** | Blissymbolics / AAC learnability evidence | The closest empirical analogue to UWS's learnability claims. | **UNCHECKED** (flagged 3×, never done) |
| **F10** | SETI / METI message design (Arecibo, Pioneer/Voyager, Dutil-Dumas, astrolinguistics), Lojban/Loglan, Wilkins' *Real Character* | Practitioners who actually attempted substrate-independent messaging; a logical language with a real speaker community. | **UNCHECKED** |

---

# G. Repo code audit (primary research)

| ID | Finding | Status |
|---|---|---|
| **G1** | `negate` implemented as enclosure-wrap + **self-loop `references` edge** as marker. Double negation ⇒ two-frame structure ≠ original. **No normalization/reduction/equivalence machinery anywhere in the crate.** No test asserts the law. | OPEN → F-006 |
| **G2** | The documented F-001 fix (boundary inversion via σ) **was never implemented.** Code uses neither reflection nor σ. | OPEN |
| **G3** | `validator.rs` checks **graph** well-formedness only (duplicate IDs, dangling refs, edge-endpoint sorts). **Zero semantic invariants.** Good Layer 0; the Cure's layer does not exist. | OPEN |
| **G4** | `composer.rs` is a **graph builder** (`Gir → Gir`), not an interpreter. No evaluator, substitution, or reduction relation. UQPL needs an interpreter that doesn't exist. | OPEN |
| **G5** | `Sort` enum gives **Modifier no discriminant** — four distinct function shapes (`e→e`, `r→r`, quantifier, `a→a`) type-check identically. Real type-safety gap. | OPEN |
| **G6** | GIR is **cyclic by deliberate design** (self-reference); the architecture note says so and its example contains a cycle. **This is what put the IR in the undecidable regime** — and `negate` is built from that exact construct. | ADOPTED → F-011 |
| **G7** | Σ_UL threaded through ~20 source files incl. core type docs. Migration cost real but mechanical. | OPEN |
| **G8** | CI runs `fmt`/`clippy`/`test` **only on `ul-forge/**`** — 83k lines of documentation had **zero** automated verification. | ADOPTED → post-mortem FM6 |
| **G9** | Doc-to-code ratio was **5.3:1**; planning-to-spec **7:1**. Now 3.6:1 after pruning. | ADOPTED |

---

# H. Consolidated follow-ups

### Blocking (do before further design)
1. **Implement the IR decision** — acyclic core, coreference as labels. Blocks C5, E1, everything downstream. *(C3, C4, D1)*
2. **One unchecked question**: has anyone applied **TMS/justification-based consistency to learned, sub-symbolic representations**? This is the genuinely novel part of the Cure. *(E1)*

### High value, unblocked
3. **`claims.yaml` CI checker** — makes enforcement real; currently conventions-without-enforcement again. *(G8)*
4. **Toy `semantically_equal` via `egg`** over a term fragment — tests the central bet cheaply. *(C5)*
5. **Daikon-style invariant inference** over a valid-GIR corpus — first empirically-derived invariant. *(E6)*
6. **Decide prevention vs. repair** posture explicitly. *(E5)*
7. **Compare the four repair routes head-to-head**: ATMS multi-context · pinpointing · convex-by-construction · CASCADE-style declared rules. *(E1, E2, E9, E5)*

### Cheap experiments that would settle open questions
8. **Structural-iconicity test** — do UWS primitives read without training? Settles F-012a. *(F3-c, F4)*
9. **AMR stress test** — with English-coverage framing, never universality. *(D2, D2-c)*
10. **Comprehension testing against the ISO 67% bar.** *(F3)*

### Unchecked literature, by priority
11. Blissymbolics/AAC learnability *(F9)* · Wiki phonetics branch *(A20)* · Mechanistic interpretability & representation engineering · CRDTs/distributed consistency *(E11)* · Institution theory *(C7)* · SETI message design *(F10)* · DRT/semantic parsing *(D6)* · Applied category theory *(C8)*

### Documentation debt
12. **`PRIMER.md` is stale** — predates TMS, prevention-vs-repair, invariant inference.
13. **No consolidated current-architecture document** — findings are spread across 13 research files that supersede each other without cross-marking.
14. **`(removed — see git history)` §0 still invalid** — cites retired theorems as "Proven."

---

## Meta-notes on method

- **~55 literature probes run.** Every one changed something. Sampling rate remains far below the
  available literature.
- **Rule adopted:** check the literature before theorizing. **Its failure mode, found the hard way:**
  adopting a finding at the *source's* confidence level rather than the level that survives transfer.
  Roughly **a third of imported findings needed narrowing** on first adversarial review — recorded
  above as `-c` entries.
- **Three independent mature fields** (ontology repair, AGM, TMS) turned out to be solving the Cure's
  core problem, all descending from justification-based reasoning about belief change. **The Cure is
  integration work, not invention.**
- **Zero experiments have ever been run. Zero lines of code written this session.** The strongest
  evidence the project holds (F1) is someone else's paper.
