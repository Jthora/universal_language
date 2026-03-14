# Architectural Decision Log

> Every significant architectural decision, the alternatives considered, the rationale, and the consequences.

---

## ADR-001: Graph-with-Tree-Spine GIR (not pure tree)

**Status:** Accepted  
**Date:** 2026-03-14

**Context:** UL glyphs have 5 spatial relationships. Containment is hierarchical (tree-like), but intersection, connection, and self-reference create non-tree edges. The naïve JSON tree model (nested `children`) cannot represent shared ownership, cross-references, or cycles.

**Decision:** Use a flat node pool + typed edge list. Extract the tree spine from `contains`/`modified_by` edges as a view.

**Alternatives:**
- **Pure tree with annotation attributes:** Tree for containment, cross-references as attributes. Problem: two representations of the same structure → desync risk.
- **RDF triple store:** Native graphs (scored R=5). Problem: heavy parser dependency, poor human readability, low AI legibility for LLMs.
- **Property graph (Neo4j-style):** Excellent graph model. Problem: not a document format — requires a database runtime.

**Consequences:** JSON serialization is slightly more verbose. All tools must handle graph traversal, not just tree traversal. The tree spine provides familiar top-down layout and serialization order.

---

## ADR-002: Rust for Core, TypeScript for Web

**Status:** Accepted  
**Date:** 2026-03-14

**Context:** UL has mathematical properties (sort correctness, geometric constraints) that need compile-time enforcement. TypeScript's type system cannot express Σ_UL sort constraints — you'd need runtime validation everywhere.

**Decision:** Rust for core (parser, renderer, validator, CLI). TypeScript for web frontend only. Core compiles to WASM for browser use.

**Alternatives:**
- **TypeScript everywhere:** Single language, but weak type safety for formal system. Sort errors become runtime bugs instead of compile errors.
- **OCaml:** Excellent type system, mature js_of_ocaml. Problem: smaller community, less WASM maturity than Rust.
- **Haskell:** Best type system. Problem: GHC runtime is heavy, WASM story is immature.

**Consequences:** Two-language project (Rust + TypeScript). WASM bridge adds build complexity. But sort safety is enforced at compile time in the core, and the same binary runs everywhere.

---

## ADR-003: Unicode UL-Script (not ASCII-only)

**Status:** Accepted  
**Date:** 2026-03-14

**Context:** The original UL-Script used ASCII only (`/3{*}` for "point inside triangle"). The implementation roster found that Unicode iconic symbols (`△{●}`) score higher on human legibility because the symbols *look like* what they represent.

**Decision:** UL-Script uses Unicode symbols (△□○→←↔●~∠) as the primary encoding. ASCII fallback is supported for environments without Unicode.

**Alternatives:**
- **ASCII-only:** Maximizes portability. Problem: `/3{*}` is opaque — requires memorizing numeric codes.
- **Full custom font:** Visual symbols rendered via a custom typeface. Problem: requires font installation, breaks in terminals.

**Consequences:** UL-Script files require UTF-8 encoding. Keyboard input methods needed for non-QWERTY layouts. ASCII fallback ensures nothing breaks in restricted environments.

---

## ADR-004: SVG as Primary Visual Output

**Status:** Accepted  
**Date:** 2026-03-14

**Context:** UL glyphs are geometric. The renderer needs a format that preserves exact geometry (not pixel approximations), scales infinitely, and embeds metadata.

**Decision:** SVG is the primary output format. TikZ for academic papers. Canvas/WebGL for dynamic rendering.

**Alternatives:**
- **Canvas (raster):** Fast rendering but loses geometry at zoom. No metadata embedding.
- **WebGL:** Overkill for 2D geometric primitives. Complex API.
- **PDF:** Good for print but not web-native. Cannot embed interactive metadata easily.

**Consequences:** SVG is XML-based, which adds some verbosity. But it's web-native, scalable, and supports embedded metadata via `<metadata>` element. D3.js and other libraries manipulate SVG directly.

---

## ADR-005: GIR-is-Canonical Policy

**Status:** Accepted  
**Date:** 2026-03-14

**Context:** When GIR is embedded in SVG, two representations coexist: the visual SVG paths and the structural GIR data. If someone edits just the SVG, they desynchronize.

**Decision:** GIR is always the canonical representation. UL-Script and SVG are derived views. Changes flow GIR→SVG, never SVG→GIR (except for extraction from existing files).

**Consequences:** Users must edit UL-Script or GIR, then re-render. Hand-editing SVG is explicitly unsupported (though hash detection warns about it). This parallels source code → compiled binary: you edit the source, not the binary.

---

## ADR-006: Template-First Layout Strategy

**Status:** Accepted  
**Date:** 2026-03-14

**Context:** General graph layout is NP-hard. But UL's 42 canonical lexicon entries are fixed glyphs with known visual forms. Starting from templates dramatically reduces scope.

**Decision:** Three-level layout strategy: (1) template lookup for known glyphs, (2) hierarchical constraint solver for novel compositions, (3) aesthetic optimization pass. Build them in order.

**Alternatives:**
- **Force-directed from scratch:** General but slow and aesthetically unpredictable for structured layouts.
- **Manual layout only:** User specifies all positions. Precise but tedious and prevents automation.

**Consequences:** Level 1 (templates) delivers immediate value — the 42 lexicon entries render correctly from day one. Level 2 (constraints) is the real engineering effort — comparable to a simplified Graphviz. Level 3 (optimization) is iterative polish.

---

## ADR-007: Stable Node IDs

**Status:** Accepted  
**Date:** 2026-03-14

**Context:** Structural diff, CRDT collaboration, and incremental rendering all require that node identities persist across edits.

**Decision:** Parser assigns stable IDs based on structural position in the AST. Collaborative mode uses UUID v7 (time-ordered). IDs never change once assigned unless the node is deleted and recreated.

**Alternatives:**
- **Content-based hashing:** ID = hash(node properties). Problem: changing any property changes the ID, breaking all references.
- **Sequential integers:** Simple but not stable across concurrent edits.

**Consequences:** GIR diffs are meaningful (same ID = same node). CRDTs can track node identity across replicas. Incremental rendering only recomputes nodes whose properties changed.

---

## ADR-008: Separate Parser and Validator

**Status:** Accepted  
**Date:** 2026-03-14

**Context:** The parser could reject invalid GIR during construction. But GIR can also be hand-authored or generated by AI agents, bypassing the parser.

**Decision:** Parser converts UL-Script → GIR with basic structural correctness. Validator is a separate tool that checks graph invariants, sort constraints, and geometry satisfiability on any GIR, regardless of origin.

**Consequences:** `ul-validate` works on hand-written JSON-GIR. AI agents can generate GIR and validate it without going through UL-Script. The parser is simpler because it doesn't carry all validation logic.
