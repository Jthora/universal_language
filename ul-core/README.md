# UL-Core: Building Universal Language from Geometry

> **Everything here derives from the 5 geometric primitives and Σ_UL. Nothing references test-content.txt.**

---

## Deliverables

| # | Deliverable | File | Status |
|---|------------|------|--------|
| 0 | Project Critique | [CRITIQUE.md](CRITIQUE.md) | Complete |
| 1 | Universal Symbology Map | [symbology/symbol-map.md](symbology/symbol-map.md) | Complete |
| 2 | Universal Syntax Dictionary | [syntax/syntax-dictionary.md](syntax/syntax-dictionary.md) | Complete |
| 3 | Universal Grammar Book | [grammar/grammar-book.md](grammar/grammar-book.md) | Complete |
| 4 | Universal Language Thesaurus | [thesaurus/thesaurus.md](thesaurus/thesaurus.md) | Complete |
| 5 | Universal Lexicon | [lexicon/lexicon.md](lexicon/lexicon.md) | **New v1** — 40 entries, bottom-up from geometry, 3-tier justification (T1/T2/T3) |
| 6 | Universal Writing System | [writing-system/writing-system.md](writing-system/writing-system.md) | **Rewritten v2** — bottom-up from axiom "what can be drawn, counts" |
| 7 | UQPL Specification | [uqpl/uqpl-spec.md](uqpl/uqpl-spec.md) | Draft — known operation/sort mismatches with Σ_UL (deferred) |

---

## Source Dependencies

Every document in this directory derives ONLY from:

| Source | What It Provides |
|--------|-----------------|
| 5 Geometric Primitives | Point, Line, Angle, Curve, Enclosure |
| 5 Semantic Primitives | Existence, Relation, Quality, Process, Concept |
| Unique Grounding Theorem | Point↔Existence, Line↔Relation, Angle↔Quality, Curve↔Process, Enclosure↔Concept |
| Σ_UL Signature | 4 sorts (Entity, Relation, Modifier, Assertion), 11 operations |
| Euclidean Postulates | 5 axioms → 5 grammatical rules |
| Erlangen Program | Symmetry hierarchy → abstraction levels |
| Jordan Curve Theorem | Closed curves partition space → categorization |
| Free Algebra Universal Property | Atom maps extend uniquely → embedding theorem |

**Explicitly excluded:** test-content.txt, mechanism-of-action.md, primer-analysis.md, reverse-engineering.md, +3elúm, Bᵉ, vrîtha, kâlithos, and all experimental infrastructure built around these.

---

## Design Philosophy

**"If UL is real, build the things that only work if UL is real."**

These aren't academic analyses. They're engineering specifications. A symbology you can read. A syntax you can parse. A grammar you can compose with. A thesaurus you can navigate. A lexicon you can look up. A writing system you can write in. A programming language you can compute with.

If UL is the geometry of meaning, then this is the engineering manual for that geometry.

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
