# Future Research Directions

**Date:** April 6, 2026
**Prerequisites:** `foundations/formal-foundations.md`, `applications/applications.md`
**Derives from:** Σ_UL formal specification, open problems in the geometric core, and the building program outlined in `ul-core/SYNTHESIS.md`.

---

## 1. Open Questions

### 1.1 Formal / Foundational

| # | Question | Status | Why It Matters |
|---|----------|--------|----------------|
| Q1 | **Is π₁ (fundamental group) sufficient for distinguishing meaning categories?** The topological derivation uses π₁ but it is too coarse (coffee mug = donut). | Acknowledged limitation | Determines how much meaning the topological level can encode. Finer invariants (homology, homotopy type) may be needed for within-category distinctions. |
| Q2 | **Can UL express recursion with depth control?** The current system handles quantification via `quantify(m, e)` but has no explicit recursion depth bound. | Partially resolved | Required for expressing complex logical statements and ensuring computational decidability. |
| Q3 | **Can the Erlangen hierarchy be validated computationally?** Does text processed at the Euclidean level genuinely produce different representations than text at the projective level? | Open | Tests whether the abstraction hierarchy is structurally real or merely a useful analogy. |
| Q4 | **What is the computational complexity of UL parsing?** Given a natural-language sentence, what is the complexity of producing its Σ_UL decomposition? | Open | Determines practical feasibility of automated UL encoding. See `frontier/expedition-one/numbers-and-computability.md` for initial analysis. |
| Q5 | **Is the 13-operation set irredundant?** The uniqueness proof shows the system is minimal for *that* signature, but could a different signature with fewer operations achieve the same expressiveness? | Resolved (no — proven minimal) | Confirms Σ_UL is not over-specified. |

### 1.2 Writing System

| # | Question | Status | Why It Matters |
|---|----------|--------|----------------|
| Q6 | **Can the lexicon scale beyond 42 entries?** The current lexicon covers canonical concepts; is the construction procedure productive enough for open-ended vocabulary? | Active development | The 3-tier justification system (Geometrically Forced / Structurally Distinguished / Conventional) needs stress-testing on domain-specific vocabulary. |
| Q7 | **Can multiple writers converge on the same glyph for the same concept?** | Untested | The writing system claims the decomposition procedure produces consistent results. Empirical validation needed. |
| Q8 | **What is the reading speed of UL glyphs for trained readers?** | Unknown | Practical usability metric. |

### 1.3 Implementation

| # | Question | Status | Why It Matters |
|---|----------|--------|----------------|
| Q9 | **Can Σ_UL be implemented as a type system?** | Active (UQPL draft in `ul-core/uqpl/`) | Enables UL-native programming. |
| Q10 | **Can the Embedding Theorem be made constructive?** i.e., given a Σ_UL-algebra A, can the unique homomorphism from G be *computed*? | Open | Determines whether cross-domain mapping is algorithmically tractable, not just mathematically guaranteed. |
| Q11 | **What serialization format should UL expressions use?** | Open | Needed for inter-agent communication (Application §5 in `applications.md`). |

### 1.4 AI Integration

| # | Question | Status | Why It Matters |
|---|----------|--------|----------------|
| Q12 | **Can LLMs learn to decompose into UL primitives?** Given training examples, can a model learn the 5-step decomposition procedure? | Open | Would enable automated UL encoding. |
| Q13 | **Does UL-structured prompting improve cross-domain reasoning?** Using Σ_UL operations as a scaffolding template for prompts. | Open — experiment needed | Direct test of Application §2. |
| Q14 | **Can UL serve as an alignment verification protocol?** Decompose both intent and behavior into UL, compare structurally. | Theoretical (Application §7) | Would provide formal grounding for AI alignment checks. |

---

## 2. Proposed Research Directions

### 2.1 Constructive Embedding Algorithm

**Objective:** Given a meaning structure expressed in any formal system (first-order logic, knowledge graph, category-theoretic diagram), produce its UL decomposition algorithmically.

**Approach:**
1. Define input grammars for common formalisms (FOL, RDF triples, type-theoretic terms)
2. Implement the Σ_UL-homomorphism from each input algebra to the geometric algebra G
3. Verify round-trip fidelity: input → UL → output preserves all structural content

**Success criterion:** A working tool that takes structured input and produces a valid UL expression tree, verified by the 13-operation type checker.

---

### 2.2 Inter-Agent Protocol Specification

**Objective:** Define a wire protocol for UL-based meaning exchange between AI systems.

**Approach:**
1. Serialize UL expression trees to a compact binary or JSON format
2. Define handshake: both agents confirm they implement Σ_UL
3. Exchange meaning structures as serialized expression trees
4. Receiver verifies structural integrity (well-formedness check against the grammar)

**Success criterion:** Two independently implemented UL systems can exchange a meaning structure and reconstruct it identically.

---

### 2.3 Empirical Writing System Validation

**Objective:** Test whether the UL writing system produces consistent decompositions across independent users.

**Approach:**
1. Train 10+ participants on the writing system specification and decomposition procedure
2. Give all participants the same 20 concepts to decompose and write as UL glyphs
3. Measure inter-rater agreement on:
   - Primitive identification (which level(s) does this concept engage?)
   - Glyph structure (do independently produced glyphs match?)
   - Reading accuracy (can participant A read participant B's glyph correctly?)

**Success criterion:** Statistically significant inter-rater agreement (Cohen's κ > 0.6) on primitive identification and glyph reading.

---

### 2.4 UQPL Implementation

**Objective:** Implement a working interpreter for the Universal Query & Programming Language.

**Approach:**
1. Formalize the type system: 4 sorts as base types, 13 operations as type constructors
2. Implement a parser (UL expression syntax → AST)
3. Implement an evaluator (AST → semantic output in a target algebra)
4. Build a REPL for interactive UL expression construction

**Success criterion:** Users can write, evaluate, and compose UL expressions interactively. The evaluator correctly implements all 13 Σ_UL operations.

---

### 2.5 Geometric Depth Scoring Tool

**Objective:** Build an automated tool that scores text by geometric depth (Application §3).

**Approach:**
1. Define heuristic mappings from natural-language features to geometric levels:
   - Nouns / named entities → Point (Level 1)
   - Verbs / predicates → Line (Level 2)
   - Adjectives / adverbs / comparatives → Angle (Level 3)
   - Temporal sequences / causal chains → Curve (Level 4)
   - Definitions / bounded arguments / self-contained theories → Enclosure (Level 5)
2. Score texts by counting which levels are engaged and how deeply
3. Validate against human judgment

**Success criterion:** Geometric depth scores correlate with human assessments of structural quality (Pearson r > 0.7).

---

## 3. Priority Order

| Priority | Direction | Rationale |
|----------|-----------|-----------|
| 1 | UQPL Implementation (§2.4) | Makes UL computable — transforms theory into tool |
| 2 | Constructive Embedding (§2.1) | Makes cross-domain mapping practical |
| 3 | Writing System Validation (§2.3) | Empirical test of the core claim |
| 4 | Geometric Depth Scoring (§2.5) | Immediate practical utility |
| 5 | Inter-Agent Protocol (§2.2) | Requires UQPL first |
