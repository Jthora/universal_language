# Comparable Systems

> Existing tools that solve similar problems. How UL Forge relates to each.

---

## Graph Visualization

### Graphviz (DOT language)

| Aspect | Graphviz | UL Forge |
|--------|----------|----------|
| **Domain** | General-purpose graph visualization | Geometric meaning composition |
| **Input** | DOT language (text) | UL-Script (text) |
| **Data model** | Directed/undirected graph | Typed graph with sorts and geometric semantics |
| **Layout** | Automatic (dot, neato, fdp, etc.) | Template-first + constraint |
| **Output** | SVG, PNG, PDF | SVG, TikZ |
| **Semantic awareness** | None — all nodes and edges are equal | Sort system enforces entity/relation/modifier/assertion types |

**Relationship:** Graphviz is a general graph renderer. UL Forge is a **meaning-aware** graph renderer — it knows about sorts, operations, and geometric semantics. A Graphviz export could be added as an output format for debugging.

### D3.js

D3 is a visualization library, not a language tool. UL Forge uses D3 for SVG manipulation in the web editor, but D3 does not understand UL structure.

---

## Mathematical Typesetting

### LaTeX / TikZ

| Aspect | TikZ | UL Forge |
|--------|------|----------|
| **Domain** | General-purpose diagram drawing | UL glyph composition |
| **Input** | TikZ commands (LaTeX) | UL-Script or visual builder |
| **Semantics** | No semantic model — purely visual | GIR encodes geometric meaning |
| **Composability** | Manual (user places every element) | Template-based + constraint-based |
| **Interactive** | No (batch rendering) | Yes (live preview, visual builder) |

**Relationship:** TikZ is a target output format for UL Forge (for papers). UL Forge adds semantic structure that TikZ lacks.

### MathJax / KaTeX

These render mathematical notation. UL is not mathematical notation — it's a geometric meaning system. However, UL glyphs could appear alongside equations in academic papers, so MathJax interop (e.g., embedding UL SVGs in MathJax-rendered documents) may be useful.

---

## Diagrammatic Languages

### Penrose

| Aspect | Penrose | UL Forge |
|--------|---------|----------|
| **Domain** | Mathematical diagrams from substance/style/domain specifications | UL glyph composition |
| **Input** | Three separate files: .substance, .style, .domain | UL-Script (single text) |
| **Approach** | Constraint-based optimization | Template-first, constraints as fallback |
| **Formal system** | Domain-specific formal languages | Σ_UL (fixed, universal) |
| **Generality** | Any mathematical domain (user-defined) | UL meaning space (fixed primitives) |

**Relationship:** Penrose is the closest comparable system. It also generates diagrams from formal specifications. The key difference: Penrose is a **framework** for creating visual languages, while UL Forge renders one **specific** language (UL). Penrose's constraint optimization approach influenced UL Forge's constraint solver design.

### Mermaid

Mermaid renders diagrams from text (flowcharts, sequence diagrams, etc.). It's a practical tool for software documentation. UL Forge serves a fundamentally different purpose — encoding geometric meaning, not software architecture.

---

## Constructed Language Tools

### ConLang tools (Langmaker, Lexique Pro)

These help construct spoken/written languages with phonology, morphology, and lexicons. UL is not a constructed language in this sense — it has no phonology, no morphology. It is a formal system of geometric primitives. The tools don't overlap.

### Lojban tools

Lojban is a syntactically unambiguous spoken language. UL is a meaning-encoding system, not a spoken language. However, Lojban's predicate logic structure has parallels to Σ_UL's `predicate` operation.

---

## Formal Verification Tools

### Z3 (SMT Solver)

Z3 is the recommended solver for UL Forge's theorem prover interface. It's a dependency, not a competitor.

### Lean / Coq / Isabelle

These are proof assistants for mathematical theorem proving. UL's 23 theorems could potentially be formalized in Lean. This is not part of v1 but could be a future community contribution.

---

## Summary Matrix

| System | Semantic model | Interactive | UL-specific | Formal verification |
|--------|---------------|-------------|-------------|-------------------|
| **UL Forge** | **Yes (Σ_UL)** | **Yes** | **Yes** | **Planned** |
| Graphviz | No | No | No | No |
| TikZ | No | No | No | No |
| Penrose | Yes (user-defined) | Yes | No | No |
| Mermaid | No | Partial | No | No |
| D3.js | No | Yes | No | No |
| Z3 | N/A (solver) | No | No | Yes |

UL Forge occupies a unique niche: a **semantically-aware, interactive, formally-grounded** composition tool for a specific universal meaning system.
