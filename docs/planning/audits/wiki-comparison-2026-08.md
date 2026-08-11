# Repo vs. Wiki Comparison — Universal Language, August 2026

**Date:** 2026-08-01
**Scope:** Compare the GitHub repo `universal_language` (formal Σ_UL system) against the UL-adjacent
content on `wiki.fusiongirl.app` (the companion "Fusion Girl" lore wiki), both owned by the same
author (Jordan Traña / Jono Tho'ra). Produced at the user's request as a standalone reference
document — no repo content has been edited as part of this audit.

**Status:** FINAL — repo-side inventory and both wiki clusters (69 pages total across the two
crawls: 32 in the core-formalism cluster, 37 in the octahedral/Platonic/Base-N cluster) complete.

---

## 1. Executive Summary

- **The wiki is not a rival or independent research effort — it is the same project, cross-published.**
  The wiki's own `Universal_Language` hub page states the identical Σ_UL signature (4 sorts, 13
  operations, 23 theorems, 42 lexicon entries, the same 5-primitive table) and explicitly links
  `github.com/jthora/universal_language` and its Zenodo DOI as the canonical source. "The research
  efforts of this project have been countered by the wiki" is better read as: *the wiki has kept
  elaborating extensions the repo hasn't absorbed back into itself yet*, not as a competing result.
- The repo's Σ_UL core (5 primitives, 4 sorts, 13 operations) is theoretically mature — 100%
  D2-completeness per the repo's own Pass 2 audit. Its implementation (`ul-forge/`) and
  distribution lag the spec (Pass 3 graded it C+ overall), though commits since that audit have
  closed a real portion of the gap (npm/transceiver/MCP packages scaffolded) — packages are
  still unpublished to any registry as of this writing.
- **The direct 5-primitive match is exact and lives on the wiki's own `Universal_Language` page**,
  not on the octahedral pages: Point/Line/Angle/Curve/Enclosure, same names, same geometric roles.
  The one real divergence at that level: the wiki maps each primitive to one of **5 semantic
  categories** (Existence/Relation/Quality/Process/Concept) where the repo has **4 sorts**
  (entity/relation/modifier/assertion) — a genuine, minor structural mismatch worth resolving.
- **Adjacent to that exact match, the wiki has built a large, separate 6-symbol system** (Octahedral
  Symbolic Geometry System / Semantic Primitive Theory: Core·/Void○/Order—/Chaos∿/Alpha∠/Omega⌒)
  with real, checkable finite-group mathematics (O_h = W(B₃), order 48, tied to the so(7) root
  system) and a full parallel family across all 5 Platonic solids (Tetrahedral/Base4, Cubic,
  Icosahedral/Dodecahedral/Base12). This is **not** the repo's 5-primitive set in disguise — it
  shares 3 of 6 symbols (Point, Line, Angle) but is organized around 3 binary polarity axes, not
  5 freestanding primitives, and none of it has a repo-side counterpart.
- The wiki independently (in its own notation) restates the repo's headline "Causal Efficacy"
  claim via Semantic Execution Theory's `∂𝒞/∂σ ≠ 0` — the clearest case of substantive, not just
  terminological, elaboration.
- The wiki maintains a real rigor-tiering discipline on most technical pages (Forced/Derived/
  Convergent/Working-Hypothesis/Open, or Forced/Conventional/Speculative) that mirrors the repo's
  own PROVEN/CONJECTURED/FRAMEWORK/ANALOGY and Geometrically-Forced/Structurally-Distinguished/
  Conventional tiers almost exactly in spirit — but it is applied inconsistently page-to-page, and
  symbol reuse (`Σ` means three different things across pages; `φ_U`'s domain/codomain flips
  between two pages) shows the corpus was written incrementally without a global notation pass.

---

## 2. Repo State (Track A)

### 2.1 Current public-facing claims
Per `README.md` / `AGENTS.md` / `FOR-AI.md`:
- Σ_UL⁺: 4 sorts (entity, relation, modifier, assertion), 13 operations (12 independent + 1 derived),
  5 geometric primitives (Point/Line/Angle/Curve/Enclosure), 23 theorems across PROVEN/CONJECTURED/
  FRAMEWORK/ANALOGY tiers.
- Proof status table: Uniqueness (PROVEN, conditional on role-property defs), Natural Emergence
  (PROVEN), Generative Power / embedding theorem (PROVEN, faithfulness gap open), Causal Efficacy
  (PROTOCOL READY — experiments designed, not yet run at scale).
- Writing system: 5 siblings (symbology/syntax/grammar/thesaurus/lexicon), 42 lexicon entries,
  19 worked examples.

### 2.2 Repo's own self-audit trail (already exists — don't re-derive)
Three self-audit passes live under `docs/planning/audits/improvements/`:
- **Pass 1 (1-1 → 1-4):** Formal-foundations hardening. D2-completeness 37/50 → 37/50 (74%),
  established the 13-operation count, added `bind`/`modify_assertion` to the *spec*.
- **Pass 2:** Modal (□/◇/□→), performative (φ), pragmatic (SI/CI inference) extensions — zero new
  primitives, D2 completeness 37/50 → 50/50 (100%).
- **Pass 3 (2026-04-08):** "Learning & App-Readiness" audit — graded the project **C+ overall**:
  - UL Forge implementation: **C** (11/13 ops, 0/3 extensions implemented in code, unpublished packages)
  - UL Core writing system: **A-** (excellent specs, minor example gaps)
  - Infrastructure & distribution: **C+** (architecture sound, nothing published/deployable)
  - Learning & teaching: **C+** (good reference docs, no tutorial path)
  - 48 gaps catalogued in `audit/gaps/gap-inventory.md` (10 critical, 16 high, 15 medium, 7 low)
  - A separate `DISTRIBUTION-EASE-OF-USE-CRITIQUE.md` found 49 additional usability issues across
    4 distribution channel types (AI-agent package, ProtoFusionGirl module, web components, AI
    transceiver protocol) — 7 critical, including broken README examples and stale TS types.

### 2.3 What's since changed (commits since Pass 3, through 2026-07)
Recent commits show real progress against the Pass 3 gap list:
- `packages/core`, `packages/transceiver`, `packages/web-components` scaffolded (npm packages with
  tests, wasm-bridge, README) — addresses part of Category C (Packaging).
- `ul-mcp` crate added (MCP server) — addresses part of Category A/D.
- `ul-transceiver` crate + `protocol/ul-transceiver.schema.json` added — addresses Type 4 gaps.
- "6 spec gaps closed — symmetry, Erlangen, validation layers, curvature, schema" (commit `2c17c11`)
  — touches parser/renderer/game-analysis, GIR schema.
- WASM test suite expanded to 147 (later 219) tests across 23 entry points.

**However:** all packages remain at version `0.1.0`; only `.github/workflows/ci.yml` exists (test/
build) — **no publish workflow** to npm/PyPI/Docker registries was found. So Pass 3's core
complaint ("nothing published or deployable") is very likely still true even though the packaging
scaffolding now exists. This should be verified directly (`npm view @ul-forge/core`) before citing
as settled either way.

### 2.4 Repo State takeaway
The repo is **not "antiquated" at the theory layer** — Σ_UL is complete and internally audited to
100% D2. It *is* behind at the implementation/distribution layer, and it was behind at the
narrative/exploratory layer relative to the wiki, which has kept generating extensions (control
theory, information geometry, octahedral group structure) that the repo's `foundations/` and
`ul-core/` have not absorbed.

---

## 3. Wiki UL Atlas (Track B)

Two background crawls covered 69 page-fetches total: a **core-formalism cluster** (32 pages —
ULCS, information geometry, glitch/computational-universe stack) and an **octahedral/Platonic/
Base-N cluster** (37 successful fetches, 1 confirmed 404 — the full Platonic-solid symbolic
geometry family, Base 1–12, and supporting pages). Discovery was entirely via WebSearch +
following in-page links, since `Special:AllPages` and all `Special:` namespace pages return
HTTP 403 even through WebFetch — the wiki has no crawlable index.

### 3.1 Provenance
The wiki's `Universal_Language` page explicitly names the GitHub repo (`github.com/jthora/
universal_language`) and its Zenodo DOI as canonical, and restates the repo's exact signature
(5 primitives / 4 sorts / 13 operations / 23 theorems) verbatim. **This is the same project's
expanded gloss, not an independently-arrived-at parallel result.** The wiki layers additional
numerology on top not present in the repo (Base 9/12/27, "Model 108" = Base 12 × Base 9).

### 3.2 The control-theoretic layer (new relative to repo)
`Universal Language Control Systems` (ULCS) defines 𝒰 = (Σ, 𝒢, 𝒞): symbolic state space Σ,
generative grammar 𝒢, control map 𝒞: Σ → 𝒪(X), with `𝒞(s₁·s₂) = 𝒞(s₁)∘𝒞(s₂)` (non-commutative).
A tower of pages builds on this: Semantic Control Fields (continuous relaxation, `dx/dt = Φ(x,σ)`),
Glitch Operator Algebra (non-invertible/discontinuous operators Γ), Semantic State Encoding Theory
(encode/decode operators S/D between physical and symbolic state spaces).

**Important caveat:** this page's `Σ` ("symbolic state space") is a different formal object from
the repo's `Σ_UL` (the algebraic signature) — same glyph, unrelated construction. Any comparison
must not conflate them.

### 3.3 The information-geometric layer (new relative to repo)
`Information Geometry` restates standard, real math (Fisher metric, KL divergence, natural
gradient) as the backbone for `Universal Semantic Manifold` (ℳ, g, ∇) — the wiki's most
sophisticated formalization attempt, building actual Riemannian-manifold machinery (Riemann
tensor, geodesics, Information Bottleneck objective) around "semantic distance/curvature."
`Universal Semantic Space` (Z, projections Rᵢ = Tᵢ(Z)) is the more abstract hypothesis this
manifold is a proposed concrete realization of.

### 3.4 Primitive correspondence — resolved: two distinct wiki systems, not one
There are **two different wiki structures** that both look like "the repo's 5 primitives" at
first glance, and it matters which one is being compared:

1. **The wiki's own `Universal_Language` hub page carries an exact match**: a Five Primitives
   table — Point (0-D/Existence/Symbology), Line (1-D/Relation/Syntax), Angle (Quality/Grammar),
   Curve (Process/Thesaurus), Enclosure (2-D+/Concept/Lexicon) — identical names and geometric
   roles to the repo's Σ_UL primitives, each further mapped to one of **5 semantic categories**
   (Existence/Relation/Quality/Process/Concept). This is the repo's own primitive set, restated.
   The one wrinkle: the repo groups these under **4 sorts** (entity/relation/modifier/assertion),
   not 5 semantic categories — a real, small structural divergence between the two presentations
   of what is otherwise the same object.

2. **A separate 6-symbol system** — `Semantic Primitive Theory` / `Octahedral Symbolic Geometry
   System` — gives Core(·)/Void(○)/Order(—)/Chaos(∿)/Alpha(∠)/Omega(⌒), organized as 3 binary
   polarity axes (Existence Scale, Change Type, Structural Mode) rather than 5 freestanding
   primitives, geometrically realized as vertices of a regular octahedron with a checkable finite
   symmetry group (Oₕ = W(B₃), order 48, tied to the so(7) root system — "an exact identity, not
   an analogy," per the wiki's own text). This splits into a Linear/Straight family (Point ·,
   Line —, Angle ∠) and Non-Linear/Curved family (Circle ○, Wave ∿, Arc ⌒).

**Only system 2 has partial, not exact, overlap with the repo**: Point, Line, Angle match by name
and geometric role; Wave (∿) has no repo counterpart at all; and the repo's Curve/Enclosure don't
cleanly biject onto Arc/Circle (Arc↔Curve is plausible; Circle/Void's "containment/totality" role
is a plausible but unconfirmed match for Enclosure). System 1 is not "partial" — it's the same
5-primitive set the repo already has, just narrativized.

### 3.5 A large physics-reinterpretation superstructure (no repo counterpart)
Computational Universe Theory, Invariant Computation Theory, Effective Model Projection Theory,
Computational State Space Dynamics, Effective Geometry Construction, Glitch Boundary Dynamics,
Singularity Transition Theory — recast physics itself as downstream of/coupled to the semantic
layer via projection operators Π, coarse-graining, renormalization flow, and glitch operators Γ.
Mostly recycles real formalism (Fisher information geometry, RG flow, gauge theory, information
bottleneck) relabeled with "semantic" adjectives. Entirely outside the repo's stated scope.

### 3.6 A genuine independent parallel: causal efficacy of meaning
The repo's headline theorem #4 ("Causal Efficacy" — meaning has measurable causal effects, tested
via experiments Alpha–Epsilon) has a wiki-side echo in `Semantic Execution Theory`:
`∂𝒞/∂σ ≠ 0` ("meaning actively participates in determining state transitions"). Same substantive
claim, independently elaborated in different notation — the strongest example of the wiki
building out a parallel sub-theory rather than just restating the repo.

### 3.7 Rigor and consistency notes
- The main UL page uses a 4-tier rigor label system (PROVEN/CONJECTURED/FRAMEWORK/ANALOGY),
  matching the repo's convention exactly — but this labeling is **not applied consistently** across
  the ~30 other UL-adjacent pages, which instead use looser academic hedges ("proposed,"
  "theoretical," "open questions") or, in a few cases (Temporal Projections, Computational
  Universe Theory), an explicit lore/speculative disclaimer.
- Symbol reuse causes real notational collisions: `Σ` means the repo's signature on one page, a
  "symbolic state space" on another, a "semantic encoding field" on a third. `φ_U` is defined with
  reversed domain/codomain across two different pages. The wiki was evidently authored
  incrementally, page by page, without a global notation pass — this matters if any of it is
  absorbed into the repo's more rigor-disciplined docs.
- The "Three Anchors" (Universal Language / Cosmic Codex / Cosmic Cypher) framing is largely
  aspirational: Cosmic Codex is a stub, and Cosmic Cypher is pure conspiracy-lore with no formal
  content — neither contributes to the technical comparison despite top billing.

### 3.8 Unexplored frontier (candidates for a follow-up pass)
Semantic Operator, Semantic Symmetry, Semantic Noether Principle, Semantic Conservation Law,
Semantic Category, Semantic Morphism, Semantic Observer, Semantic Agency, Semantic Consciousness,
Semantic Hilbert Space, Semantic Action Principle/Hamiltonian, UQPL / Universal Quantum
[Process|Programming] Language, Universal Symbology/Syntax/Grammar/Writing System, and the
per-sort pages (Existence/Relation/Quality/Process/Concept) that likely map onto the repo's 4
sorts. Not fetched in this pass; flagged rather than guessed at. A further adjacent branch was
surfaced but not pursued: a phonetics/linguistics mapping (Semantic Acoustic Correspondence,
Vocal Semantic Correspondence, Signal Manifold, Articulatory Polytope) applying the same
polytope/group machinery to speech articulation.

### 3.9 The Platonic-solid family and Base-N system (cluster 2)
The octahedral system from §3.4 turns out to be one of **five parallel systems**, unified under a
`Universal Symbolic Geometry System` (USGS) hub page, one per Platonic solid:

| Solid | System | Base | Symmetry group | Lie algebra | Crystallographic? |
|---|---|---|---|---|---|
| Tetrahedron (self-dual) | Tetrahedral SGS | 4 | S₄ = W(A₃) | su(4) = so(6) | Yes |
| Octahedron | Octahedral SGS | 6 | O_h = W(B₃) = W(C₃) | so(7) | Yes |
| Cube (dual of octahedron) | Cubic SGS | 6/8 | O_h = W(B₃) | so(7) | Yes |
| Icosahedron | Icosahedral SGS | 12 | I_h = H₃ | none | No |
| Dodecahedron (dual of icosahedron) | Dodecahedral SGS | 12 | I_h = H₃ | none | No |

The octahedron is the load-bearing, most-developed member (full weight/root-system treatment,
216-state combinatorics via `216 State Space`, `Ternary Interpretation Atlas`, `Triadic Field
Theory`). A 4D extension is asserted but flagged speculative (24-cell / (ℤ₂)⁴ / H₄→E₈ folding).

**Base-N is a numeral-system progression, not a primitive count.** `Base 1` through `Base 6`
(Base 5 has no page — confirmed 404, and a purported "Base 5 → Agency" in an Existence/Polarity/
Mediation/Structure/Agency/Resonance sequence could not be verified on any live page — treat that
specific labeling as unconfirmed) walk unary→binary→ternary→quaternary→(missing)→senary numeral
systems, each paired with a geometric realization (point→line segment→triangle→tetrahedron→...→
octahedron) and a group-theoretic note (ℤ₂, C₃, S₄=W(A₃), O_h=W(B₃)). This is a genuinely
different axis of organization from the repo's 5-primitive/4-sort signature — it's about which
numeral base and which Platonic solid, not about entity/relation/modifier/assertion.

**Rigor discipline is real and consistent across this cluster.** Nearly every page in this
family — `Platonic Base Correspondence`, `Semantic Symmetry Correspondence`, `Triadic Field
Theory`, `Æther and Nether` — explicitly tags claims as Forced (standard math, e.g. vertex/edge/
face counts, the O_h=W(B₃) identity) vs. Conventional (labeling choices) vs. Speculative/
Suggestive (e.g. a named "Sheaf Conjecture" explicitly marked "stated for building, not
established"; an E₈ extension explicitly marked not established). This is the wiki's most
mathematically disciplined cluster of the two crawled.

**One page directly grounds the wiki's ambitions in real external research**: `The Platonic
Representation Hypothesis` presents the actual 2024 Huh/Cheung/Wang/Isola paper (arXiv:2405.07987)
and formalizes the wiki's "Meaning = Geometry" claim as an attempt to align with it — this is the
strongest evidence that the wiki intends its speculative apparatus to be answerable to real
external research, not just internally self-consistent lore.

---

## 4. Comparison Matrix & Recommendations (Track C)

| Axis | Repo (Σ_UL) | Wiki | Verdict |
|------|-------------|------|---------|
| 5 primitives | Point/Line/Angle/Curve/Enclosure → 4 sorts (entity/relation/modifier/assertion) | Same 5 primitives, verbatim, on the `Universal_Language` hub page → **5** semantic categories (Existence/Relation/Quality/Process/Concept) | Same object; 4-sort vs. 5-category grouping is a real, small mismatch to reconcile |
| 6-symbol octahedral system | No counterpart | Core/Void/Order/Chaos/Alpha/Omega (·/○/—/∿/∠/⌒), 3 polarity axes, Oₕ=W(B₃) group theory | Wiki-only; partial symbol overlap (Point/Line/Angle) but a genuinely different, separate system |
| Platonic-solid family | No counterpart | 5 parallel systems (Tetrahedral/Octahedral/Cubic/Icosahedral/Dodecahedral SGS), Base 4/6/12 | Wiki-only extension, internally consistent, real finite-group math |
| Core algebra (Σ_UL⁺) | 4 sorts, 13 ops, 23 theorems, proofs | Restates repo's numbers verbatim on the hub page; not independently re-derived | Same object, not a rival result |
| Dynamics/control | Not modeled (repo is static/compositional) | ULCS (Σ,𝒢,𝒞), Semantic Control Fields, Glitch Operator Algebra | Wiki-only extension |
| Geometric/manifold grounding | 5 primitives named, not embedded in a manifold | Universal Semantic Manifold (ℳ,g,∇), full Riemannian + Fisher-information apparatus | Wiki-only extension; the most mathematically substantive wiki content |
| Causal efficacy | Theorem #4, protocol-ready, not yet run at scale | Semantic Execution Theory: ∂𝒞/∂σ≠0, asserted not tested | Independent parallel claim, neither side empirically closed |
| Rigor labeling | Consistent PROVEN/CONJECTURED/FRAMEWORK/ANALOGY across all docs | Real tiering (Forced/Conventional/Speculative etc.) but applied page-by-page, inconsistent notation (Σ, φ_U collide across pages) | Repo more disciplined; wiki's octahedral cluster is its most disciplined sub-area |
| Grounding in outside research | Cites Frege/Montague/Jackendoff/Langacker/Wierzbicka | Cites the real 2024 Platonic Representation Hypothesis paper (arXiv:2405.07987) as a target to align with | Both anchor to real external literature |
| Implementation | ul-forge: 11–13/13 ops in code (improving), unpublished packages | No code implementation anywhere | Repo-only, and still catching up to its own spec |

### Recommendations (not executed — for the user to decide on)
1. **Reconcile the 4-sorts-vs-5-categories mismatch first.** This is the smallest, highest-value
   fix: the repo's own primitive table and the wiki's hub-page primitive table are the same
   object, but grouped differently (4 sorts vs. 5 semantic categories). Worth a one-paragraph note
   in `foundations/formal-foundations.md` either reconciling or explicitly scoping the difference.
2. **Don't conflate the octahedral 6-symbol system with the repo's 5 primitives when writing about
   either publicly.** They're related (3-of-6 symbol overlap) but distinct systems; presenting
   them as the same thing (or as contradictory) would misrepresent both. A short cross-reference
   note in `foundations/` stating "see wiki's Octahedral Symbolic Geometry System for a related but
   distinct 6-symbol/3-axis system" would prevent confusion for anyone reading both.
3. **The Universal Semantic Manifold / Information Geometry material is the strongest candidate
   for real absorption into `frontier/`.** It's the most mathematically substantive wiki content,
   uses real machinery (Fisher metric, geodesics, Information Bottleneck), and plausibly extends
   `frontier/expedition-two/probability-and-information.md` rather than duplicating it.
4. **The octahedral/Platonic-solid group-theory family (Oₕ=W(B₃), so(7) root system, the 5-solid
   family) is the second-strongest candidate** — it's checkable, finite, and self-consistent group
   theory, distinct from the more speculative "Effective Physics" stack.
5. **Do not import the "Effective Physics" superstructure** (Computational Universe Theory, Glitch
   Operator Algebra, Singularity Transition Theory, etc.) as-is — it's explicitly speculative even
   by the wiki's own hedging and sits well outside the repo's stated compositional-semantics scope.
6. **Resolve notation collisions before merging anything.** `Σ` means three different things across
   wiki pages (repo signature / symbolic state space / semantic encoding field); `φ_U`'s domain and
   codomain flip between two pages. Any absorption work should happen after — not during — a
   notation-consistency pass on the source wiki material being pulled in.
7. **On "antiquated":** the theory core is not behind — it's the same object as the wiki's, at the
   same level of completeness (both cite D2-completeness/23 theorems identically). What's genuinely
   behind is (a) the repo not having absorbed the wiki's manifold/control-theoretic *extensions*
   yet, and (b) `ul-forge`'s implementation/distribution lagging even the repo's own spec. Those are
   two different, independently addressable problems — worth treating as such rather than one
   "everything is antiquated" verdict.

---

## Appendix: full wiki page inventories
Full page-by-page extraction notes (technical claims, exact notation, links) for all 69 pages
crawled are preserved at:
- `/private/tmp/claude-501/-Users-jono-Documents-GitHub-universal-language/c5a94882-b949-46d2-944d-7b2a44d535d3/scratchpad/wiki-atlas-cluster1.md` (core UL formalism, 32 pages)
- `/private/tmp/claude-501/-Users-jono-Documents-GitHub-universal-language/c5a94882-b949-46d2-944d-7b2a44d535d3/scratchpad/wiki-atlas-cluster2.md` (octahedral/Platonic/Base-N, 37 pages)

These are session-scratch files, not part of the repo — copy anything worth keeping long-term
into this report or into the repo proper.

---

## 5. Foundational Coherence Check (added 2026-08-01, following user challenge)

The user pushed back on the initial comparison: does "Σ_UL signature (5 primitives, 4 sorts, 13
operations, 23 theorems)" actually hold together, or is something inconsistent across the repo and
wiki? This section reports a direct read of the proofs themselves (`foundations/formal-foundations.md`,
`foundations/universal-language-derivation.md`, `foundations/independent-derivation.md`) plus the
repo's own internal audit trail, rather than trusting the summary labels.

### 5.1 The "5 primitives" claim is internally inconsistent within the repo — not just vs. the wiki

- `foundations/independent-derivation.md` §2.5 explicitly concludes: **"Process is NOT a fifth
  independent sort. It is a property of relations... a structurally distinguished sub-category of
  Relation."** By this document's own reasoning, there are 4 real primitives (Entity/Relation/
  Modifier/Assertion), and Curve/Process is not their structural peer.
- Yet `foundations/formal-foundations.md` §4.3–4.5 (the "Unique Grounding Theorem") treats all 5
  geometric primitives (Point/Line/Angle/Curve/Enclosure) and all 5 "semantic primitives"
  (Existence/Relation/Quality/Process/Concept) as **structural peers** in a single proof — directly
  contradicting the other document's demotion of Process to a non-independent sub-case.
- **This exact tension is already catalogued by the repo's own April 2026 audit**: finding **F7,
  "4-Sort Algebra vs. 5-Primitive Geometry Tension"**
  (`docs/planning/audits/improvements/pass1-1/findings/structural-gaps.md`), logged status "CLEAR
  RESOLUTION — needs documentation additions." The prescribed fix (a reconciling note in
  `formal-foundations.md`, `AGENTS.md`, and `symbol-map.md`) was **never actually added** — verified
  by grep across `README.md`, `AGENTS.md`, `FOR-AI.md`, `foundations/formal-foundations.md`, and
  `ul-core/symbology/symbol-map.md`: zero matches for any reconciling language. The fix was designed
  and approved in the audit trail but never propagated to the live docs.

### 5.2 The "Unique Grounding Theorem" is closer to a matched definition than an independent derivation

The proof works by defining 5 "semantic primitives" (S1–S5, `formal-foundations.md` §4.3) with role
properties written to mirror the 5 geometric primitives (G1–G5, §4.4) property-for-property, then
proving a bijection exists between two hand-matched 5-element lists. Given how S1–S5 were
constructed, the "uniqueness" result is close to definitional, not a forcing argument from
independent first principles. The repo is self-aware of this weakness — `independent-derivation.md`
opens by naming exactly this as "a circular confirmation bias" identified by an earlier critique —
but the fix it offers (re-deriving semantic primitives from Frege/Montague/Jackendoff/Langacker/
Wierzbicka without reference to geometry) arrives at **4** sorts, which undercuts rather than
rescues the "5 primitives" framing used on the public-facing pages.

### 5.3 The "100% D2 completeness" figure was reached by iterative self-patching, not external validation

`ul-core/CRITIQUE.md`'s resolution log shows the internal completeness self-test climbing
32%→38%→40%→48%→52%→74%→84%→94%→100% across nine sequential patches (Pass 1.2–1.3, Pass 2), each
one adding a new operation, distinguished element, or reinterpretation specifically to fix a case
that had just failed. This is the same team designing the test and repeatedly amending the theory
until it passes — internally consistent by construction, but not independent confirmation. "100%
completeness" should be read as "we closed every gap we found in our own checklist," not as an
external validation of the theory.

### 5.4 A real math bug did occur and was fixed — the project is not just loosely argued, it has had actual errors
Finding **F1** (`docs/planning/audits/improvements/pass1-1/findings/critical-errors.md`): negation
was implemented as geometric reflection, which produces logical *converse* ("B is acted upon by A"),
not *negation* ("A does NOT act on B") — a genuine correctness bug that broke the De Morgan/
propositional-completeness proofs relying on it. Fixed 2026-04-07 by switching to boundary inversion.
Documented for balance: this shows the project has had — and caught and fixed — real formal errors,
not only interpretive looseness.

### 5.5 Verdict: neither the repo nor the wiki is "wrong" in isolation — the repo contains two confidence levels of the same claim, and the wiki inherited the less careful one

The repo's public-facing top layer (`README.md`, `AGENTS.md`, `universal-language-derivation.md`)
states "5 primitives" flatly, with no hedge. The repo's own internal audit layer
(`independent-derivation.md`, `CRITIQUE.md`, `pass1-1/findings/`) already knows this is overstated
and downgrades it to "4 sorts + 1 dependent, distinguished sub-case" — but that correction never
propagated to the public docs. The wiki's `Universal_Language` hub page quotes the repo's public
layer near-verbatim (same "5 primitives" table, same 4-sorts/13-ops/23-theorems figures) — it is
reproducing the repo's own unfixed overstatement, not introducing an independent error.

### 5.6 Recommendation
Actually close finding F7: add the reconciling note the repo's own 2026-04 audit already drafted
language for, to `foundations/formal-foundations.md` (near §4.5), `AGENTS.md` (formal spec block),
and `ul-core/symbology/symbol-map.md` (near the Curve entry) — and consider softening the "Unique
Grounding Theorem" framing on public-facing pages to match the more honest framing already present
in `independent-derivation.md` and `CRITIQUE.md`. This is a documentation-propagation fix, not new
research — the honest version of the claim already exists in the repo, it just isn't where readers
(or the wiki) encounter it first.
