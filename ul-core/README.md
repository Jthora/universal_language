# UL-Core: Building Universal Language from Geometry

> **Everything here derives from the 5 geometric primitives and Σ_UL. Nothing references test-content.txt.**

---

## Deliverables

| # | Deliverable | File | Status |
|---|------------|------|--------|
| 0 | Project Critique | [CRITIQUE.md](CRITIQUE.md) | Complete — known errors, gaps, open questions, resolution log |
| — | Navigation Guide | [NAVIGATION.md](NAVIGATION.md) | Quick-start paths, sibling cross-references, two-classification-system explanation |
| — | Synthesis | [SYNTHESIS.md](SYNTHESIS.md) | **New** — how the 5-pipeline works, current capabilities, 10 expansion paths, strategic priorities |
| 1 | Universal Symbology Map | [symbology/symbol-map.md](symbology/symbol-map.md) | Complete — cross-sibling refs, G≠Σ_UL note, tier labels on §V |
| 2 | Universal Syntax Dictionary | [syntax/syntax-dictionary.md](syntax/syntax-dictionary.md) | Complete — cross-sibling refs, sort/symmetry note, G≠Σ_UL note |
| 3 | Universal Grammar Book | [grammar/grammar-book.md](grammar/grammar-book.md) | Complete — cross-sibling refs, sort/symmetry note, 3 new worked examples (Negation, Causation, Self-Reference) |
| 4 | Universal Language Thesaurus | [thesaurus/thesaurus.md](thesaurus/thesaurus.md) | Complete — cross-sibling refs, tier labels on §VII |
| 5 | Universal Lexicon | [lexicon/lexicon.md](lexicon/lexicon.md) | v2 — 42 entries, 3-tier justification, cross-sibling refs, normative-status note, semantic index (§10.5) |
| 6 | Universal Writing System | [writing-system/writing-system.md](writing-system/writing-system.md) | **Rewritten v2** — bottom-up from axiom "what can be drawn, counts" |
| 6b | Writer's Companion Guide | [writing-system/writers-companion.md](writing-system/writers-companion.md) | **New** — practical pen-and-paper guide: meaning decomposition, handwriting conventions, 10 worked examples, compound statements, grammar bridge |
| 7 | UQPL Specification | [uqpl/uqpl-spec.md](uqpl/uqpl-spec.md) | Draft — programming language inspired by UL, not a strict Σ_UL-algebra (3/14 ops match exactly). Sort names aligned; relationship characterized in [uqpl/D3-ul-uqpl-analysis.md](uqpl/D3-ul-uqpl-analysis.md) |

---

## Source Dependencies

Every document in this directory derives ONLY from:

| Source | What It Provides |
|--------|-----------------|
| 5 Geometric Primitives | Point, Line, Angle, Curve, Enclosure |
| 5 Semantic Primitives | Existence, Relation, Quality, Process, Concept |
| Unique Grounding Theorem | Point↔Existence, Line↔Relation, Angle↔Quality, Curve↔Process, Enclosure↔Concept |
| Σ_UL⁺ Signature | 4 sorts (Entity, Relation, Modifier, Assertion), 13 operations |
| Euclidean Postulates | 5 axioms → 5 grammatical rules |
| Erlangen Program | Symmetry hierarchy → abstraction levels |
| Jordan Curve Theorem | Closed curves partition space → categorization |
| Free Algebra Universal Property | Atom maps extend uniquely → embedding theorem |

**Explicitly excluded:** test-content.txt, mechanism-of-action.md, primer-analysis.md, reverse-engineering.md, +3elúm, Bᵉ, vrîtha, kâlithos, and all experimental infrastructure built around these.

---

## Design Philosophy

**"If UL is real, build the things that are predicted by UL theory and test whether they work as predicted."**

A successful prediction under controlled conditions would support UL; a failed prediction would tell us UL's scope is narrower than claimed.

These aren't academic analyses. They're engineering specifications. A symbology you can read. A syntax you can parse. A grammar you can compose with. A thesaurus you can navigate. A lexicon you can look up. A writing system you can write in. A programming language you can compute with.

If UL is the minimal algebraic skeleton that all compositional meaning systems share, then this is the engineering manual for that skeleton.

### The Five Siblings

UL has 5 geometric primitives. The core reference has 5 siblings. This is not a coincidence.

| Primitive | Sibling | Concern |
|-----------|---------|---------|
| **Point** (0D) | Symbology | What the atomic marks ARE |
| **Line** (1D) | Syntax | How marks CONNECT |
| **Angle** (hybrid) | Grammar | Rules governing valid CONSTRUCTIONS |
| **Curve** (1D in 2D) | Thesaurus | Paths between RELATED meanings |
| **Enclosure** (2D) | Lexicon | BOUNDED canonical DEFINITIONS |

Each sibling does what its primitive does: Point identifies, Line connects, Angle qualifies, Curve navigates, Enclosure bounds.
