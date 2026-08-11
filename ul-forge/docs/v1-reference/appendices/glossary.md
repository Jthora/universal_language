# Glossary

> Key terms used throughout UL Forge documentation.

---

## Core Concepts

| Term | Definition |
|------|-----------|
| **Universal Language (UL)** | The minimal algebraic skeleton that all compositional meaning systems share, derived from 5 geometric primitives and unique up to isomorphism |
| **Σ_UL⁺** | The algebraic signature of UL: 4 sorts, 13 operations, unique up to isomorphism (conditional on role-property definitions) |
| **Glyph** | A visual composition of geometric primitives that encodes a meaning in UL |
| **Mark** | A single geometric primitive instance within a glyph (a point, a line, an enclosure, etc.) |

## Geometric Primitives

| Term | Symbol | Sort | Semantic Role |
|------|--------|------|--------------|
| **Point** | `•` | Entity | Existence, identity, position |
| **Line** | `→` `—` | Relation | Directed or undirected connection between entities |
| **Angle** | `∠` `/` | Modifier | Measure between two relations, quality |
| **Curve** | `~` | Relation (process) | Parameterized path, transformation, process |
| **Enclosure** | `◯` `△` `□` | Entity | Bounded region, concept, structure, definition |

## Sorts

| Term | Symbol | Description |
|------|--------|-------------|
| **Entity** | `e` | Things that exist — points, enclosures, nominalized assertions |
| **Relation** | `r` | Connections between entities — lines, curves, composed relations |
| **Modifier** | `m` | Properties that alter entities or relations — angles, abstracted entities |
| **Assertion** | `a` | Complete statements — predicates, negated assertions, conjoined/disjoined assertions |

## Data Structures

| Term | Definition |
|------|-----------|
| **GIR** | Glyph Intermediate Representation — the canonical graph-based data structure for UL glyphs. Consists of a flat node pool + typed edge list. JSON-serialized. |
| **Node** | An element in the GIR representing one geometric primitive. Has `id`, `type`, `sort`, and optional metadata. |
| **Edge** | A typed relationship between two nodes in the GIR. Types: `contains`, `modified_by`, `adjacent`, `intersects`, `connects`, `references`. |
| **Tree spine** | The tree structure extracted from `contains` and `modified_by` edges. Defines the hierarchical reading order of a glyph. |
| **Cross-edge** | An edge (`adjacent`, `intersects`, `connects`, `references`) that bridges different branches of the tree spine. |

## Edge Types

| Term | Definition |
|------|-----------|
| **contains** | Parent mark spatially encloses child mark (enclosure contains point) |
| **modified_by** | Source mark is qualified by target modifier (entity modified by angle) |
| **adjacent** | Two marks are positioned next to each other (touching) |
| **intersects** | Two marks spatially overlap |
| **connects** | A line/curve links two marks (directed or undirected) |
| **references** | A non-spatial semantic reference between marks |

## Σ_UL Operations

| Term | Signature | Description |
|------|-----------|-------------|
| **predicate** | `e × r × e → a` | Subject-relation-object assertion |
| **modify_entity** | `m × e → e` | Apply modifier to entity |
| **modify_relation** | `m × r → r` | Apply modifier to relation |
| **negate** | `a → a` | Negate an assertion |
| **conjoin** | `a × a → a` | AND two assertions |
| **disjoin** | `a × a → a` | OR two assertions |
| **embed** | `a → e` | Turn assertion into entity (nominalization) |
| **abstract** | `e → m` | Turn entity into modifier (adjectivalization) |
| **compose** | `r × r → r` | Chain two relations |
| **invert** | `r → r` | Reverse a relation |
| **quantify** | `m × e → a` | Apply quantifier to entity |

## UL-Script

| Term | Definition |
|------|-----------|
| **UL-Script** | Text-based input notation for UL. Uses Unicode characters (with ASCII fallback) to represent geometric primitives and their relationships. |
| **ASCII fallback** | Alternative character sequences for environments without Unicode support. E.g., `(O)` for `◯`, `(.)` for `•`. |
| **Canonical form** | The unique normalized UL-Script representation of a GIR document. Used for deduplication and comparison. |

## Tool Components

| Term | Definition |
|------|-----------|
| **UL Forge** | The complete toolchain for composing, validating, and rendering UL glyphs |
| **Parser** | Converts UL-Script text to GIR (tokenize → parse → transform → validate) |
| **Renderer** | Converts GIR to visual output (template lookup → constraint layout → SVG generation) |
| **Validator** | Checks GIR against schema, sort rules, graph invariants, and geometric satisfiability |
| **Template** | Pre-computed layout for a canonical glyph. Matched by subgraph isomorphism. |
| **Constraint solver** | Computes positions for marks that don't match any template, using geometric constraints |
| **WASM bridge** | The Rust core library compiled to WebAssembly for use in browsers and VS Code |

## Collaboration

| Term | Definition |
|------|-----------|
| **Structural diff** | Difference between two GIR documents computed on the graph structure, not JSON text |
| **CRDT** | Conflict-free Replicated Data Type — data structure that automatically merges concurrent edits |
| **Yjs** | JavaScript CRDT library used for real-time collaborative editing in the web editor |
