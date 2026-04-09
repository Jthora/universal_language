# Implementation Roster: Three-Layer Architecture Candidates

> Evaluating concrete technologies, formats, and design approaches for manifesting UL's digital architecture.  
> **Optimization target:** Maximize human-AI interoperability — both sides read, write, and exchange the same structures with minimal translation overhead.

**Context:** The three-layer architecture (§XII–XV of the Glyph Composition Algebra) identifies the need for:
1. **Layer 1 — Keyboard Script:** ASCII-typeable structural encoding
2. **Layer 2 — Geometric IR:** Geometry-native machine representation  
3. **Layer 3 — Renderer:** Visual output to screen/paper

This document evaluates candidates for each layer, scores them on dual-legibility criteria, and identifies the most promising implementation paths.

---

## 0. EVALUATION CRITERIA

### 0.1 Scoring Axes

Every candidate is scored on six axes with **explicit multipliers** reflecting their actual importance to the project's core goal (human-AI interoperability).

| Axis | What it measures | Multiplier | Rationale |
|------|------------------|------------|-----------|
| **H — Human legibility** | Can a trained human read/write it without tools? | **×3** | Core goal. A format humans can't use fails UL's universality claim. |
| **A — AI legibility** | Can AI systems (not just LLMs) parse, generate, and *reason about* complex instances? | **×3** | Core goal. Evaluated at scale (100+ nodes), not just toy examples. |
| **R — Roundtrip fidelity** | Does structure survive encode → decode → re-encode, including intersections, cycles, and edge cases? | **×2** | A lossy format corrupts meaning — unacceptable for a formal system. |
| **E — Extensibility** | Can new primitives/operations be added without breaking existing encodings? | **×1** | Matters long-term but can be addressed via versioning. |
| **I — Implementation cost** | How much engineering to reach a working prototype? | **×1** | Matters for adoption speed but doesn't affect correctness. |
| **P — Ecosystem leverage** | Does it ride existing libraries, parsers, renderers? | **×1** | Useful but not decisive — UL is novel enough that some new tooling is inevitable. |

**Maximum weighted score: (5×3) + (5×3) + (5×2) + (5×1) + (5×1) + (5×1) = 55**

Scores: **5** = exceptional, **4** = strong, **3** = adequate, **2** = weak, **1** = failing.

### 0.2 Scoring Honesty Rules

1. **H is scored for the format itself**, not for annotations/wrappers around it. Prose commentary in Markdown doesn't count toward H — every format can be annotated.
2. **A is scored at scale** — can the AI handle a 100-node composition, not just a 3-node example? LLMs lose track of nested brackets past depth ~5. Score the hard case.
3. **R is scored for the full data model** — including intersection (shared ownership), connection (cross-references), and self-reference (cycles). If the format can only represent trees, it gets R ≤ 3.
4. Candidates that are a **poor fit for their layer** (e.g., JSON as a keyboard script) are noted but not ranked — they belong in a different layer.

---

## 1. LAYER 1 CANDIDATES: KEYBOARD SCRIPT

The keyboard script is where **human writability** matters most. This is what someone types at a keyboard or pastes in a chat message. It must be ASCII-only, parseable, and compact.

### Candidate 1A: UL-Script (Custom DSL, ASCII)

The format specified in §XIII of glyph-composition.md. Purpose-built for UL. ASCII-only.

```
[ /3{*} ->@60 /0{*} ]       # "A fundamental thing harmonizes with a universal thing"
/0{ /4{ /0{*} } }           # "structured truth" (circle containing square containing circle containing point)
[ * ->@0 #[*a ->@60 *b] ]   # "Something acts on the fact that a harmonizes with b"
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **4** | 12 | Compact, learnable. `/3{}` for triangle is unintuitive to newcomers — requires memorizing the convention. Readable at depth ≤ 3. |
| A | **4** | 12 | Small token footprint, unambiguous BNF. Drops to ~3 at depth > 5 (nested braces become ambiguous for LLMs). Scored for hard case. |
| R | **4** | 8 | Lossless for tree structures. Cross-references (intersection, connection) require ID-based `&` binding — not yet specified. Cycles unhandled. |
| E | **4** | 4 | `/7{}` for heptagon works automatically. New modifiers need new sigils — finite ASCII space. |
| I | **4** | 4 | Parser is ~500 lines. BNF already written. |
| P | **2** | 2 | No existing ecosystem. Must build everything. |
| **Total** | | **42** | |

---

### Candidate 1B: S-Expressions (Lisp-style)

Meaning structures as nested s-expressions. Extremely regular syntax.

```lisp
(frame (predicate (tri (point)) (line :dir forward :angle 60) (circ (point))))
(circ (sq (circ (point))))
(frame (predicate (point) (line :dir forward :angle 0) (embed (frame (predicate (point :id a) (line :dir forward :angle 60) (point :id b))))))
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **2** | 6 | Parenthesis depth taxes human parsing at depth > 3. Non-Lisp-users find it unreadable. Closed-paren stacks (`))))`) are illegible. |
| A | **4** | 12 | LLMs excel at s-expressions for small instances. At 100+ nodes, paren-matching degrades. Slightly worse than JSON for large structures. |
| R | **5** | 10 | Named keyword arguments preserve all parameters. Cross-references via `:id` / `:ref`. |
| E | **5** | 5 | Inherently extensible — add new node types freely. |
| I | **3** | 3 | Parser is trivial. Schema validation is not. |
| P | **4** | 4 | Vast s-expression ecosystem. Parsers in every language. |
| **Total** | | **40** | |

---

### Candidate 1C: YAML-Based Markup

Indentation-driven structure. Familiar to developers.

```yaml
- frame:
    subject:
      enclosure: triangle
      contains: [point]
    relation: { direction: forward, angle: 60 }
    object:
      enclosure: circle
      contains: [point]
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **2** | 6 | Whitespace-sensitive: copy-paste between chat apps, emails, and forums corrupts structure. A human-input format that breaks when pasted into Discord or Slack is unacceptable for a communication system. Self-documenting when intact, but fragile. |
| A | **4** | 12 | LLMs handle YAML well in-context. Whitespace sensitivity causes generation errors. |
| R | **5** | 10 | Full fidelity via explicit keys. |
| E | **5** | 5 | New keys = new capabilities. No breaking changes. |
| I | **3** | 3 | YAML parsers everywhere. Schema layer needed. |
| P | **5** | 5 | Massive ecosystem. JSON Schema for validation. |
| **Total** | | **41** | |

---

### Candidate 1D: UL-Script with Unicode Primitives

Same BNF structure as 1A but uses Unicode symbols for geometric primitives, making the script partially *iconic* — symbols resemble what they represent.

```
[ △{●} →@60 ○{●} ]         # triangle-enclosed point, forward relation at 60°, circle-enclosed point
○{ □{ ○{●} } }             # structured truth
[ ● →@0 ⟦●a →@60 ●b⟧ ]    # embedding uses double-bracket
```

**Primitive map:**
```
●  U+25CF  Point/Entity        →  U+2192  Forward relation
○  U+25CB  Circle enclosure    ←  U+2190  Backward relation
△  U+25B3  Triangle enclosure  ↔  U+2194  Bidirectional
□  U+25A1  Square enclosure    ∠  U+2220  Angle
⬠  U+2B20  Pentagon enclosure  ∩  U+2229  Intersection
⬡  U+2B21  Hexagon enclosure   ¬  U+00AC  Negation
⟦⟧ U+27E6/7 Embedding frame    ∀  U+2200  Universal quantifier
                                ∃  U+2203  Existential quantifier
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **5** | 15 | **Iconic** — `△{●}` looks like what it means (a point in a triangle). Dramatically more readable than `/3{*}`. Every modern OS/keyboard supports Unicode input (macOS: Option key, Windows: WinCompose, Linux: Compose key). |
| A | **4** | 12 | LLMs handle Unicode tokens well. Tokenizers may split some symbols into multi-byte tokens (slightly more expensive). Same BNF structure as 1A — identical parseability. |
| R | **4** | 8 | Same limitations as 1A (tree-only without extensions). |
| E | **3** | 3 | Unicode provides many geometric symbols but finite supply. Extending beyond what Unicode offers requires fallback to ASCII notation. |
| I | **4** | 4 | Same parser as 1A with a symbol-mapping layer. |
| P | **3** | 3 | Unicode is universal. No UL-specific ecosystem. |
| **Total** | | **45** | |

**Verdict:** The highest-scoring Layer 1 candidate. The iconic quality — symbols that look like what they mean — is a major human-legibility advantage that the ASCII-only constraint threw away. Modern input methods make typing Unicode practical. The parser is identical to 1A with a character-mapping layer.

---

### Candidate 1E: LaTeX-Style Macros

UL as a LaTeX package. Leverages the most successful mathematical typesetting ecosystem.

```latex
\ulframe{\ulpred{\ultri{\ulpoint}}{\ulrel[dir=forward,angle=60]}{\ulcirc{\ulpoint}}}
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **2** | 6 | Only readable by LaTeX users. Macro nesting is visually noisy. |
| A | **3** | 9 | LLMs can generate LaTeX. Macro expansion is complex. Error-prone at scale. |
| R | **5** | 10 | Full fidelity via macro parameters. |
| E | **4** | 4 | New macros = new capabilities. |
| I | **2** | 2 | Requires LaTeX ecosystem. Overkill dependencies. |
| P | **3** | 3 | LaTeX is heavy and academic-only. |
| **Total** | | **34** | |

---

### Candidate 1F: JSON Lines (Machine-Primary)

One JSON object per UL expression, newline-delimited. **Note: this is effectively a Layer 2 format being evaluated as Layer 1. Included for completeness.**

```json
{"frame":{"subject":{"enc":"tri","c":[{"pt":true}]},"rel":{"dir":"fwd","a":60},"object":{"enc":"circ","c":[{"pt":true}]}}}
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **1** | 3 | Unreadable bracket soup. Not a human-input format. |
| A | **5** | 15 | Native structured data. But at 100+ nodes, even LLMs lose track of nesting. Drops to ~4 at scale. Scored conservatively: **4**. | 
| A | **4** | 12 | *(corrected)* |
| R | **5** | 10 | Full fidelity. |
| E | **5** | 5 | New keys trivially. |
| I | **5** | 5 | Zero parser work. |
| P | **5** | 5 | Maximum ecosystem. |
| **Total** | | **40** | |

---

### Layer 1 Ranking (Weighted)

| Rank | Candidate | Weighted | Key insight |
|------|-----------|----------|-------------|
| **1** | **1D: Unicode UL-Script** | **45** | Iconic symbols are a free Human-legibility win |
| **2** | **1A: ASCII UL-Script** | **42** | Solid fallback for ASCII-constrained environments |
| **3** | **1C: YAML** | **41** | Fragile in real-world transmission |
| **4** | **1B: S-Expressions** | **40** | AI-strong but paren-stacks defeat humans |
| **4** | **1F: JSON Lines** | **40** | Wrong layer (this is GIR, not input) |
| **6** | **1E: LaTeX macros** | **34** | Niche, heavy, noisy |

**Recommendation:** **1D (Unicode UL-Script)** as primary, with **1A (ASCII UL-Script)** as the fallback for constrained environments (terminals, legacy systems, code comments). Both share the same BNF grammar — only the symbol table differs. A single parser handles both via a character-mapping layer.

**On Markdown embedding:** Any of these candidates can be embedded in Markdown ` ```ul ``` ` fences with prose annotations for human context. This is a *presentation choice*, not a format choice. It does not affect the scores — every format benefits equally from Markdown wrapping.

---

## 2. LAYER 2 CANDIDATES: GEOMETRIC IR

The GIR is where **machine fidelity** matters most. This is what programs store, transmit, query, diff, and validate. Human readability is a bonus, not a requirement — but given the optimization target, we evaluate it.

### Candidate 2A: JSON Scene Graph (as specified in §XIV)

The format already described in glyph-composition.md. A tree of typed mark objects.

```json
{
  "ul_gir": "1.0",
  "marks": [{
    "id": "e1", "type": "enclosure", "sort": "entity",
    "geometry": {"boundary": {"sides": 0, "symmetry_group": "SO2"}},
    "contains": ["e2"],
    "children": [{"id": "e2", "type": "point", "sort": "entity", "geometry": {"position": [0.5, 0.5]}}]
  }]
}
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **3** | 9 | JSON is readable with effort. Deep nesting is hard. |
| A | **4** | 12 | JSON is the *de facto* interchange format. At 100+ nodes, LLMs lose track of nested objects. Better than text formats but not perfect. |
| R | **3** | 6 | **Trees can't represent intersections (shared marks) or cycles (self-reference) without `$ref` indirection.** The current spec uses both `contains` arrays AND `children` nesting — redundant and potentially contradictory. Needs a graph-aware revision. |
| E | **5** | 5 | New keys, new types, versioned schema. |
| I | **5** | 5 | Zero parser work. JSON Schema for validation. |
| P | **5** | 5 | Maximum ecosystem leverage. Every language, every tool. |
| **Total** | | **42** | |

**Honest note:** R dropped from the previous 5 to 3 because the tree/graph tension is real. See §8 (Hard Problems) for the fix.

---

### Candidate 2B: Protocol Buffers (Binary + Schema)

Typed, schema-enforced binary format. Compact on the wire.

```protobuf
message Mark {
  string id = 1;
  MarkType type = 2;
  Sort sort = 3;
  Geometry geometry = 4;
  repeated string contains = 5;
  repeated Mark children = 6;
}
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **1** | 3 | Binary format. Completely opaque to humans. |
| A | **2** | 6 | LLMs cannot generate or read binary protobuf. Text mode (JSON mapping) is just… JSON. |
| R | **5** | 10 | Schema-enforced. Complete fidelity. Graph references via string IDs. |
| E | **5** | 5 | Protobuf is designed for backward-compatible evolution. |
| I | **3** | 3 | Requires protobuf toolchain. Schema must be maintained. |
| P | **4** | 4 | Large ecosystem (Google-backed). |
| **Total** | | **31** | |

---

### Candidate 2C: XML/MathML-style Semantic Markup

XML with a UL namespace. Analogous to how MathML encodes math structure.

```xml
<ul:frame xmlns:ul="urn:ul:gir:1.0">
  <ul:predicate>
    <ul:enclosure sides="0" symmetry="SO2">
      <ul:point position="0.5,0.5" sort="entity"/>
    </ul:enclosure>
    <ul:line direction="forward" angle="60" sort="relation"/>
    <ul:enclosure sides="3" symmetry="D3" sort="entity">
      <ul:point position="0.5,0.5"/>
    </ul:enclosure>
  </ul:predicate>
</ul:frame>
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **3** | 9 | Readable with training. Verbose. Tag noise. |
| A | **3** | 9 | LLMs handle XML but more tokens than JSON for same content. Losing mindshare in training data. |
| R | **4** | 8 | XLink/XPointer can handle cross-references. Better than raw JSON for graph structures. |
| E | **5** | 5 | Namespaces, attributes, extensible by design. |
| I | **3** | 3 | XML parsers everywhere. But XML tooling is declining. |
| P | **3** | 3 | XML ecosystem is mature but stagnant. MathML is supported in browsers. |
| **Total** | | **37** | |

---

### Candidate 2D: RDF / Linked Data (Semantic Web)

UL structures as RDF triples. Every mark is a node; every relationship is an edge.

```turtle
@prefix ul: <urn:ul:> .

ul:frame1 a ul:Frame ;
  ul:contains ul:pred1 .

ul:pred1 a ul:Predicate ;
  ul:subject ul:e1 ;
  ul:relation ul:r1 ;
  ul:object ul:e2 .

ul:e1 a ul:Enclosure ;
  ul:sides 0 ;
  ul:symmetryGroup "SO2" .
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **2** | 6 | Graph notation. Requires graph-thinking. Not linear. |
| A | **3** | 9 | LLMs can handle Turtle syntax but don't excel at graph reasoning. |
| R | **5** | 10 | **Graphs handle intersection, shared ownership, and cycles natively.** Every spatial relationship is a typed edge. No tree limitation. |
| E | **5** | 5 | RDF is infinitely extensible by design (add new predicates freely). |
| I | **2** | 2 | Requires triple store or RDF library. Heavy infrastructure. |
| P | **3** | 3 | Semantic web ecosystem exists but is niche. SPARQL for querying. |
| **Total** | | **35** | |

**Revised note:** R=5 is now justified — RDF's graph model solves the tree/graph tension that plagues JSON. This makes it more structurally correct than JSON, even though it scores lower overall because of human/AI legibility.

---

### Candidate 2E: SQLite / Relational

GIR as a relational database. One row per mark. Foreign keys for containment.

```sql
CREATE TABLE marks (
  id TEXT PRIMARY KEY,
  type TEXT NOT NULL,  -- "point", "line", "enclosure", etc.
  sort TEXT NOT NULL,  -- "entity", "relation", "modifier", "assertion"
  parent_id TEXT REFERENCES marks(id),
  sides INTEGER,
  symmetry_group TEXT,
  position_x REAL,
  position_y REAL,
  scale REAL DEFAULT 1.0,
  rotation REAL DEFAULT 0.0,
  z_order INTEGER DEFAULT 0
);

CREATE TABLE spatial_rels (
  mark_a TEXT REFERENCES marks(id),
  mark_b TEXT REFERENCES marks(id),
  relationship TEXT NOT NULL  -- "contains", "intersects", "adjacent_to"
);
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **2** | 6 | SQL is readable for developers. Not for UL practitioners. |
| A | **4** | 12 | LLMs generate SQL well. Structured queries enable powerful reasoning. |
| R | **5** | 10 | **Relational model handles graphs perfectly** — spatial_rels table can represent any relationship including cycles. |
| E | **4** | 4 | ALTER TABLE for new columns. Migration needed. |
| I | **3** | 3 | SQLite is everywhere. Schema must be maintained. |
| P | **4** | 4 | Massive SQL ecosystem. |
| **Total** | | **39** | |

**Revised note:** R upgraded from previous score. The relational model with a separate spatial_rels table is actually one of the cleanest ways to handle the graph structure — it cleanly separates "what exists" (marks table) from "how things relate" (spatial_rels table).

---

### Candidate 2F: Property Graph (Neo4j / GraphQL)

Marks are nodes with properties. Spatial relationships are typed edges.

```graphql
type Mark {
  id: ID!
  type: MarkType!
  sort: Sort!
  geometry: Geometry!
  contains: [Mark!]! @relationship(type: "CONTAINS")
  intersects: [Mark!]! @relationship(type: "INTERSECTS")
  adjacent_to: [Mark!]! @relationship(type: "ADJACENT_TO")
  connected_to: [Connection!]! @relationship(type: "CONNECTED_VIA")
  children: [Mark!]! @relationship(type: "HAS_CHILD")
}
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **2** | 6 | Graph query languages are not human-friendly for reading. |
| A | **3** | 9 | LLMs handle GraphQL but graph traversal reasoning is weak. |
| R | **5** | 10 | **The best structural match for UL.** Typed nodes + typed edges = exactly what compositions are. |
| E | **5** | 5 | New node types, new edge types. |
| I | **2** | 2 | Requires graph database infrastructure. |
| P | **3** | 3 | Growing but niche ecosystem. |
| **Total** | | **35** | |

---

### Layer 2 Ranking (Weighted)

| Rank | Candidate | Weighted | Key strength |
|------|-----------|----------|-------------|
| **1** | **2A: JSON Scene Graph** | **42** | Universal, AI-native, maximum leverage |
| **2** | **2E: SQLite/Relational** | **39** | Clean graph handling via separate tables |
| **3** | **2C: XML/MathML-style** | **37** | Cross-references via XLink; browser precedent |
| **4** | **2D: RDF** | **35** | Best roundtrip fidelity (native graph model) |
| **4** | **2F: Property Graph** | **35** | Best structural match for UL compositions |
| **6** | **2B: Protobuf** | **31** | Wire efficiency only |

**Recommendation:** **2A (JSON)** remains primary for pragmatic reasons (ecosystem, AI legibility), but **the data model must be revised** from a tree to a graph-with-tree-spine. See §8.2 for the solution. The JSON format should use a flat node pool + explicit edge list, not nested children — this makes intersection, connection, and self-reference representable.

**Secondary recommendation:** **2E (SQLite)** as the storage backend. The relational model's clean separation of nodes and edges is the right architecture for a corpus of thousands of GIR documents.

---

## 3. LAYER 3 CANDIDATES: RENDERING ENGINE

The renderer is where **visual fidelity** matters most. This is what humans see. AI agents don't consume rendered output — they work with Layers 1 and 2.

### Candidate 3A: SVG via JavaScript (D3.js / custom)

Generate SVG elements from GIR programmatically in the browser.

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **5** | 15 | SVG renders natively in all browsers. Scalable, printable, crisp. |
| A | **3** | 9 | AI can generate SVG but doesn't need to — it works with GIR. |
| R | **N/A** | 0 | Rendering is one-way (lossy). Not scored — Layer 3 is intentionally lossy. |
| E | **4** | 4 | New mark types = new SVG generation functions. |
| I | **3** | 3 | Custom renderer required. D3.js helps with layout. |
| P | **5** | 5 | SVG is a web standard. CSS styling. DOM manipulation. Event handling. |
| **Total** | | **36** | (R excluded from max; adjusted max = 45) |

**Verdict:** The default web rendering target. Browser-native, scalable, styleable, interactive. D3.js provides layout algorithms. This is the obvious first implementation.

---

### Candidate 3B: HTML Canvas 2D

Immediate-mode pixel rendering.

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **4** | 12 | Smooth rendering. Fast for complex scenes. No DOM overhead. |
| A | **2** | 6 | Canvas output is pixels — opaque to AI. |
| R | **N/A** | 0 | Even more lossy than SVG (pixels, not vectors). |
| E | **3** | 3 | New marks = new draw functions. No structure in output. |
| I | **3** | 3 | Similar to SVG but no layout library. |
| P | **4** | 4 | Canvas is universal in browsers. |
| **Total** | | **28** | |

**Verdict:** Better for real-time interactive editing (drag marks, animate transitions). Worse for static display (no DOM = no CSS = no accessibility). Use as a secondary target for interactive UL editors.

---

### Candidate 3C: WebGL / GPU Rendering

GPU-accelerated 2D rendering via shaders.

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **5** | 15 | Smoothest possible rendering. Handles massive complexity. |
| A | **1** | 3 | Completely opaque to AI. |
| R | **N/A** | 0 | Pixel output. |
| E | **3** | 3 | Shader programming is expensive. |
| I | **1** | 1 | Significant engineering effort. Shader pipeline from scratch. |
| P | **3** | 3 | Three.js, regl, etc. exist but are overkill for 2D. |
| **Total** | | **25** | |

**Verdict:** Future-proofs for 3D UL composition (when the system extends to 3D volumes per the open problems). Overkill for current 2D needs. Reserve for v2.0.

---

### Candidate 3D: PDF Generation (server-side)

Generate PDF documents from GIR. For formal/printed UL.

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **5** | 15 | PDF is the gold standard for print. |
| A | **1** | 3 | PDF is opaque. |
| R | **N/A** | 0 | One-way to paper. |
| E | **3** | 3 | PDF generation libraries handle arbitrary graphics. |
| I | **3** | 3 | Libraries exist (pdfkit, ReportLab, Puppeteer screenshot). |
| P | **4** | 4 | PDF is universal for documents. |
| **Total** | | **28** | |

**Verdict:** Necessary for formal publication ("UL papers"). Not the primary target. Generate PDF by rendering SVG → PDF (via Puppeteer or similar).

---

### Candidate 3E: TikZ / LaTeX Backend

Generate TikZ code from GIR. Renders via LaTeX.

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **5** | 15 | LaTeX produces the highest-quality geometric figures. |
| A | **3** | 9 | AI can generate TikZ. But this is Layer 3, not Layer 1. |
| R | **N/A** | 0 | One-way to paper/PDF. |
| E | **4** | 4 | TikZ is extremely powerful for 2D geometry. |
| I | **3** | 3 | TikZ code generation from a tree is straightforward. |
| P | **4** | 4 | LaTeX is universal in academia. |
| **Total** | | **35** | |

**Verdict:** Excellent for academic publication. If UL papers appear in journals, TikZ rendering ensures publication-quality figures. Secondary target after SVG.

---

### Candidate 3F: Pen Plotter / Physical Drawing

Generate G-code, HPGL, or SVG-for-plotter from GIR. Draws with an actual pen.

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **5** | 15 | Full circle: geometric writing → physical pen on paper. |
| A | **1** | 3 | Physical artifact. |
| R | **N/A** | 0 | One-way to paper. |
| E | **3** | 3 | Limited by physical pen capabilities (no fill, no z-order, no overlap). |
| I | **3** | 3 | SVG → plotter conversion is established. |
| P | **3** | 3 | Niche but growing (AxiDraw, pen plotters). |
| **Total** | | **27** | |

**Verdict:** Philosophically important — UL is a writing system, and physical writing grounds the digital representation. Practically, it's a bonus target after web rendering is solid.

---

### Layer 3 Ranking (Weighted, R excluded)

| Rank | Candidate | Weighted | Key strength |
|------|-----------|----------|-------------|
| **1** | **3A: SVG (JavaScript)** | **36** | Web-native, scalable, interactive, styleable |
| **2** | **3E: TikZ/LaTeX** | **35** | Academic publication quality |
| **3** | **3B: Canvas 2D** | **28** | Real-time interactive editing |
| **3** | **3D: PDF** | **28** | Print/formal documents |
| **5** | **3F: Pen Plotter** | **27** | Physical grounding |
| **6** | **3C: WebGL** | **25** | Future 3D extension |

**Recommendation:** **3A (SVG)** as primary rendering target. Generate SVG elements from GIR in the browser. Add TikZ export for academic use. Canvas for interactive editors. PDF via SVG→PDF conversion.

---

## 4. INTEGRATION ARCHITECTURE CANDIDATES

How the three layers connect into a working system. This is where human-AI interoperability lives at the systems level.

### Candidate 4A: Web Application (Browser-Native)

A single-page app where users type UL-Script, see live rendered output, and can export GIR.

```
┌──────────────────────────────────────────────────────────────┐
│  BROWSER                                                      │
│                                                                │
│  ┌──────────────┐    ┌───────────┐    ┌────────────────────┐  │
│  │ UL-Script     │ →  │ UL-GIR    │ →  │ SVG Rendering      │  │
│  │ Editor (text) │    │ (JSON)    │    │ (live preview)     │  │
│  │ Monaco/CM     │    │           │    │                    │  │
│  └──────────────┘    └───────────┘    └────────────────────┘  │
│         ↕                  ↕                                   │
│  Markdown view        API / Export                             │
│  (annotated)          (for AI agents)                          │
└──────────────────────────────────────────────────────────────┘
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **5** | 15 | Split-pane: type on left, see glyph on right. Markdown mode for annotated docs. |
| A | **5** | 15 | GIR exposed via API. AI agents POST UL-Script or GIR, receive rendered SVG or GIR back. |
| R | **5** | 10 | GIR is the canonical state. Everything round-trips through it. |
| E | **5** | 5 | Web tech is infinitely extensible. |
| I | **3** | 3 | Moderate effort: parser + renderer + editor + API. |
| P | **5** | 5 | React/Vue, Monaco Editor, D3.js. Massive ecosystem. |
| **Total** | | **53** | |

---

### Candidate 4B: CLI Toolchain (Unix Philosophy)

Separate tools piped together: `ul-parse`, `ul-render`, `ul-validate`, `ul-convert`.

```bash
echo '[ /3{*} ->@60 /0{*} ]' | ul-parse | ul-render --format svg > output.svg
cat document.ul | ul-parse | ul-validate --schema ul-gir-1.0.json
cat *.ul | ul-parse | ul-query "//enclosure[@symmetry='SO2']"
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **3** | 9 | Command-line is developer-friendly. Not for general users. |
| A | **5** | 15 | Pipeable. AI agents can invoke CLI tools directly. Structured I/O. |
| R | **5** | 10 | Each tool preserves structure. GIR is the interchange format between pipes. |
| E | **5** | 5 | Add new tools. Composable by design. |
| I | **4** | 4 | Each tool is small and focused. Parse is ~500 lines. Render is much more (see §8.1). |
| P | **4** | 4 | Unix ecosystem. Wrap in npm/pip/brew packages. |
| **Total** | | **47** | |

---

### Candidate 4C: VS Code Extension

UL language support in VS Code: syntax highlighting, live preview, validation, autocomplete.

```
┌───────────────────────────────────────────────────────────────┐
│  VS Code                                                       │
│                                                                 │
│  ┌────────────────────┐    ┌──────────────────────────────┐    │
│  │ .ul file editor     │    │ Preview pane (SVG render)    │    │
│  │ Syntax highlighting │    │ Live update on keystroke     │    │
│  │ Autocomplete        │    │                              │    │
│  │ Error squiggles     │    │                              │    │
│  └────────────────────┘    └──────────────────────────────┘    │
│                                                                 │
│  Copilot integration: "write UL for 'justice is balanced power'"│
│  → generates UL-Script → validates → renders → shows preview   │
└───────────────────────────────────────────────────────────────┘
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **5** | 15 | Best coding experience. Inline errors. Live preview. Autocomplete. |
| A | **5** | 15 | Copilot generates UL-Script from natural language. Extension API for agent access. |
| R | **5** | 10 | Editor state IS GIR. |
| E | **5** | 5 | Extension API is extensible. |
| I | **2** | 2 | VS Code extension development is moderately heavy. Language server protocol. |
| P | **5** | 5 | VS Code is the dominant editor. Extension marketplace for distribution. |
| **Total** | | **52** | |

---

### Candidate 4D: Jupyter / Notebook Integration

UL as a notebook cell type. Inline rendering. Mix code/UL/prose.

```python
# In a Jupyter cell:
%%ul
[ /3{*} ->@60 /0{*} ]
```
→ Renders inline as SVG.

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **4** | 12 | Notebook is natural for exploration and teaching. |
| A | **5** | 15 | Python kernel. LLMs generate notebook cells. Programmatic access to GIR. |
| R | **5** | 10 | GIR accessible from Python: `gir = ul.parse("[ /3{*} ->@60 /0{*} ]")`. |
| E | **4** | 4 | Python library extensible. |
| I | **3** | 3 | Python package + Jupyter magic command. Moderate effort. |
| P | **5** | 5 | Jupyter is universal in research/education. |
| **Total** | | **49** | |

---

### Candidate 4E: API Service (REST/GraphQL)

A hosted service that accepts UL-Script or GIR and returns rendered output, validation results, or structural analysis.

```
POST /api/v1/parse
Body: { "script": "[ /3{*} ->@60 /0{*} ]" }
Response: { "gir": { ... }, "svg": "<svg>...</svg>" }

POST /api/v1/validate
Body: { "gir": { ... } }
Response: { "valid": true, "sort_check": "pass", "geometry_check": "pass" }
```

| Axis | Raw | ×Wt | Notes |
|------|-----|------|-------|
| H | **2** | 6 | Humans don't use APIs directly. Needs a frontend. |
| A | **5** | 15 | API is the native interface for AI agents. |
| R | **5** | 10 | GIR is the canonical response format. |
| E | **5** | 5 | REST/GraphQL endpoints are infinitely extensible. |
| I | **3** | 3 | Server infrastructure needed. But the parse/render logic is shared with all other candidates. |
| P | **5** | 5 | REST/GraphQL is universal. |
| **Total** | | **44** | |

---

### Integration Ranking (Weighted)

| Rank | Candidate | Weighted | Key strength |
|------|-----------|----------|-------------|
| **1** | **4A: Web Application** | **53** | Best dual-legibility. Humans see, AI uses API. |
| **2** | **4C: VS Code Extension** | **52** | Best developer experience. Copilot integration. |
| **3** | **4D: Jupyter Notebook** | **49** | Best for teaching, research, exploration. |
| **4** | **4B: CLI Toolchain** | **47** | Most composable. AI agents invoke directly. |
| **5** | **4E: API Service** | **44** | Machine backbone. All other frontends use it. |

**Recommendation:** Build **4B (CLI toolchain)** first — it's the fastest path to a working system and every other integration uses the same core logic. Then wrap the core as **4E (API service)**. Then build **4A (web app)** and **4C (VS Code extension)** as frontends that call the API.

---

## 5. THE RECOMMENDED STACK

Based on the weighted analysis above (honesty-adjusted scoring with Critical ×3, High ×2, Medium ×1):

### Primary Stack

| Layer | Choice | Format | Human use | AI use |
|-------|--------|--------|-----------|--------|
| **Input** | Unicode UL-Script | `.ul` files (Unicode-capable) | Type with keyboard shortcuts / input method | Parse directly — iconic symbols are unambiguous |
| **Annotation** | Markdown wrapping (orthogonal) | ` ```ul ``` ` blocks in `.md` | Annotate glyphs with prose, embed in docs | Parse code fences, extract UL blocks |
| **Interchange** | JSON Graph-with-Tree-Spine (UL-GIR) | `.ul.json` files | Inspect with jq / JSON viewer | Native format for processing, storage, exchange |
| **Output** | SVG (primary), TikZ (academic) | `.svg`, `.tex` | View in browser, print | Generate for human consumption |
| **Toolchain** | CLI pipes → API → web app | `ul-parse`, `ul-render`, `ul-validate` | Web editor with live preview | REST API, CLI invocation |

**Key changes from naïve analysis:**
- Input format is Unicode UL-Script, not "Markdown-embedded." Markdown wrapping is orthogonal — any format can be embedded in Markdown fences. The script format is judged on its own merits.
- GIR uses a graph-with-tree-spine model (see §8.2), not a pure tree. This handles intersecting marks, cross-references, and self-referential glyphs.
- Line estimates are anchored to comparable real systems (see §8.7). The naïve estimates were fiction.

### Implementation Order

```
PHASE 1: CORE (CLI toolchain)
├── ul-parse:    UL-Script → UL-GIR (JSON)        See §8.7 for realistic estimates
├── ul-render:   UL-GIR → SVG                      (this is the hard part — §8.1)
├── ul-validate: UL-GIR → validation report
└── ul-convert:  UL-GIR ↔ CSF/JSON (legacy compat)

PHASE 2: API
├── REST service wrapping Phase 1 tools
├── WebSocket for live preview
└── JSON Schema for UL-GIR validation

PHASE 3: FRONTENDS
├── Web editor (React/Vue + Monaco + D3.js)
├── VS Code extension (Language Server Protocol)
└── Jupyter magic command (`%%ul`)

PHASE 4: ECOSYSTEM
├── TikZ backend
├── PDF export (via SVG → PDF)
├── Python library (`import ul`)
├── npm package (`import { parse } from 'ul'`)
└── Pen plotter backend (SVG → HPGL)
```

### Dual-Legibility Architecture

```
                    HUMAN SIDE                        AI SIDE
                    ──────────                        ───────

    Write:          Type UL-Script (Unicode)          Generate UL-Script
                    Optionally wrap in Markdown        or raw JSON-GIR
                           │                                │
                           ▼                                ▼
    Parse:          `ul-parse` converts UL-Script     Direct JSON-GIR
                    to GIR                            creation/consumption
                           │                                │
                           ▼                                ▼
    Store:          ┌─────────────────────────────────────────────┐
                    │   CANONICAL UL-GIR (JSON, graph-with-spine) │
                    │   Both sides read and write this format      │
                    │   Queryable, diffable, validatable           │
                    └─────────────────────────────────────────────┘
                           │                                │
                           ▼                                ▼
    Render:         SVG in browser/PDF for print       SVG for human
                    TikZ for academic papers            display; GIR
                                                        for reasoning
                           │                                │
                           ▼                                ▼
    Read:           See glyphs, read annotations       Parse GIR for
                    in Markdown context                 structure
```

**The center of gravity is UL-GIR.** Everything converts to and from it. Humans approach from the UL-Script side (top-left); AI agents approach from the JSON-GIR side (top-right). They meet in the middle.

---

## 6. KEY ARCHITECTURAL DECISIONS

### 6.1 File Extensions

| Extension | Content | Use |
|-----------|---------|-----|
| `.ul` | UL-Script (Unicode-capable) | Primary authoring format. Human-typed composition. |
| `.ul.json` | UL-GIR (JSON, graph-with-tree-spine) | Machine interchange, storage, API payloads |
| `.ul.svg` | Rendered SVG with embedded GIR metadata | Display + recoverable structure (see §6.2 for caveats) |

Note: Markdown wrapping uses standard code fences (` ```ul `) in `.md` files. No special extension needed.

### 6.2 SVG with Embedded GIR

**Critical for interoperability:** The rendered SVG should embed the GIR as metadata so that the structure is recoverable from the visual:

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200">
  <metadata>
    <ul:gir xmlns:ul="urn:ul:gir:1.0">
      <!-- Full JSON-GIR embedded here -->
    </ul:gir>
  </metadata>
  <!-- Visual rendering follows -->
  <circle cx="100" cy="100" r="80" stroke="black" fill="none"/>
  <circle cx="3" cy="3" r="3" fill="black"/>
</svg>
```

**The "lying document" problem:** If someone edits the SVG visuals without updating the embedded GIR (or vice versa), the two representations desynchronize. The SVG "lies" — it looks one way but structurally means another. Solution: see §8.5.

### 6.3 Version Negotiation

GIR documents carry a version field. Tools must:
1. Refuse to process versions they don't understand
2. Support forward-compatible reading (ignore unknown fields)
3. Require backward-compatible writing (always include required fields from v1.0)

```json
{"ul_gir": "1.0", "nodes": [...], "edges": [...]}
```

### 6.4 Language for Implementation

The naïve recommendation was "TypeScript for everything." This is wrong for a formal system. UL has mathematical properties (sort correctness, geometric constraints, graph invariants) that need a language with stronger guarantees.

| Component | Recommended language | Rationale |
|-----------|---------------------|-----------|
| Core parser | **Rust** or **OCaml** | Sort-safe ADTs. No runtime null errors. Parser combinators. Compiles to WASM for browser. |
| Core renderer | **Rust** (via WASM) | Performance-critical layout computation (§8.1). No GC pauses. |
| Core validator | Same as parser | Share the type definitions. |
| CLI tools | Same as core | Single binary. No runtime dependency. |
| Web frontend | **TypeScript** | DOM, React/Vue, Monaco Editor. TypeScript is genuinely good here. |
| Python library | **Python** | Calls core via WASM/FFI or subprocess. |
| API service | **TypeScript** or **Rust (Actix/Axum)** | Web framework maturity. |

**The split:** Rust/OCaml for the core (parser, renderer, validator), TypeScript for the web shell. The core compiles to WASM so it runs in the browser without a separate TypeScript reimplementation. This avoids the trap of using a weakly-typed language for a formal system while keeping the web ecosystem's strengths where they matter.

**Why not Haskell?** Haskell's WASM story is less mature than Rust's, and the deployment story (GHC runtime) is heavier. OCaml is a reasonable alternative (js_of_ocaml is mature, and OCaml's type system is excellent for formal systems).

**Why not TypeScript for everything?** TypeScript's type system cannot express the sort constraints of Σ_UL at compile time. You'd need runtime validation everywhere, which means the type system is decorative rather than protective. For a system where sort errors (putting a Modifier where an Entity goes) are the primary class of bugs, this matters.

---

## 7. WHAT THIS ROSTER DOES NOT DECIDE

The following decisions require prototyping and user testing, not analysis:

1. **UL-Script syntax details** — the BNF in §XIII of the Glyph Composition Algebra is a draft. User testing may reveal that `/3{...}` is less intuitive than `△{...}`, that `->@60` should be `->60`, etc. The grammar WILL evolve.

2. **GIR schema specifics** — field names, nesting conventions, edge-type taxonomy. The graph-with-tree-spine model (§8.2) is architecturally correct, but the concrete JSON keys require implementation experience.

3. **Performance characteristics** — how large can a GIR document get before rendering slows down? Is the graph model too verbose at scale? These are empirical.

4. **Accessibility mapping** — how does UL-GIR map to screen readers, tactile displays, or audio? This is a research problem that requires collaboration with accessibility experts.

5. **Aesthetic conventions** — line weight, spacing ratios, mark sizing. These are design decisions that emerge from use, not from analysis.

The following decisions ARE addressed in §8 (previously deferred as "not decided"):

- Layout algorithm (§8.1) — concrete approaches, not just "requires prototyping"
- Graph-with-tree-spine GIR (§8.2) — revised data model
- Composer workflow (§8.3) — evaluation of authoring modalities
- Collaboration model (§8.4) — diff/merge for geometric structures
- SVG sync problem (§8.5) — the "lying document" solution
- Beyond-LLM AI interfaces (§8.6) — vision models, GNNs, theorem provers
- Honest implementation estimates (§8.7) — reality-anchored line counts

---

## 8. HARD PROBLEM SOLUTIONS

This section addresses the genuinely difficult problems that the naïve roster hand-waved away. Each subsection names the problem, explains why it's hard, and proposes a concrete approach.

---

### 8.1 The Layout Problem

**Problem:** Given a UL-GIR document (a graph of geometric primitives with containment, intersection, adjacency, and connection relationships), compute a 2D layout that is readable, aesthetically coherent, and faithful to the geometric constraints.

**Why it's hard:** This is a variant of graph drawing, which is NP-hard in general. UL makes it harder because:
- Containment is nested (marks inside marks inside enclosures)
- Intersecting marks must actually *visually overlap* — not just be connected by an edge
- Angles have specific geometric meaning (60° means 60°, not "close to 60°")
- Glyphs at different scales must remain legible

**Approach: Constraint-based layout with geometric templates**

The key insight is that UL layout is more constrained than general graph drawing, and those constraints *help* rather than hurt. UL glyphs are built from exactly 5 geometric primitives with well-defined spatial relationships. This is closer to circuit layout or molecular structure rendering than to arbitrary force-directed graphs.

**Level 1 — Template-based (MVP)**

Each primitive has a canonical form:
- **Point:** Place at coordinate. Trivial.
- **Line:** Straight segment between two points. Direction matters (→ is distinct from ←).
- **Angle:** Two lines sharing a vertex. Angle measure is specified.
- **Curve:** Bézier path. Control points determined by curvature parameter.
- **Enclosure:** Circle or closed curve bounding a region.

The simplest glyphs are static templates: look up the glyph in the lexicon, return a fixed SVG path. This handles the 42 canonical definitions immediately. No layout computation needed.

```
TEMPLATE LOOKUP:
  "existence" → fixed SVG path (point inside enclosure)
  "relation"  → fixed SVG path (line with direction marks)
  ...
```

**Level 2 — Hierarchical constraint solver**

For composed glyphs (not in the template library), use a hierarchical approach:

1. **Parse containment tree.** The tree spine of the GIR gives nesting. Allocate bounding boxes top-down: each enclosure gets a bounding box, its children are placed within it.

2. **Resolve geometric constraints.** Within each bounding box, apply the geometric constraints (angle measures, line directions, intersection requirements) as a constraint satisfaction problem. Since individual glyphs have few components (typically 3-7 marks), this is feasible even with exact constraint solving.

3. **Pack siblings.** When multiple glyphs are composed (conjoin, embed, abstract), pack their bounding boxes using strip-packing or treemap algorithms. The tree spine provides the nesting structure; packing is only done between siblings at the same level.

4. **Resolve cross-references.** The graph edges that aren't in the tree spine (intersections, connections) are routed as Bézier curves between the already-placed bounding boxes. This is edge-routing, not full layout — a much simpler problem.

```
ALGORITHM: hierarchical_layout(gir)
  1. tree_spine = extract_tree(gir)
  2. for each node in tree_spine (top-down):
       bbox = allocate_bounding_box(node, parent_bbox)
       positions = solve_constraints(node.children, bbox)
       place(node.children, positions)
  3. for each cross_edge in gir.edges - tree_spine.edges:
       route = route_bezier(cross_edge.source.bbox, cross_edge.target.bbox)
       draw(route)
```

**Level 3 — Optimization refinement (post-MVP)**

After the constraint solver produces a valid layout, apply aesthetic refinement:
- **Force-directed nudging:** Small perturbations to improve visual balance (not full force-directed layout from scratch)
- **Whitespace equalization:** Distribute unused space evenly between siblings
- **Collision avoidance:** Detect visual overlap between non-intersecting marks and nudge apart
- **Label placement:** If annotations are present, place them in available whitespace

**Comparable systems to study:**
- **Graphviz `dot`**: Hierarchical layout for DAGs. ~80k lines C. UL's hierarchical layout is a subset of this.
- **MathJax/KaTeX**: Layout of mathematical notation with strict positioning rules. MathJax is ~60k lines.
- **Penrose**: Constraint-based diagramming from mathematical specifications. Closest analog to UL rendering. ~30k lines Rust+TypeScript.
- **D3.js force layout**: For the refinement pass only. Already has pack/treemap algorithms.

**The honest assessment:** Level 1 (templates) is buildable immediately. Level 2 (hierarchical constraints) is the real engineering work — comparable in complexity to a small Graphviz, so expect 10-20k lines of Rust for a solid implementation. Level 3 (optimization) can evolve iteratively once Level 2 works.

---

### 8.2 Graph-with-Tree-Spine GIR

**Problem:** The current GIR specification uses a JSON tree (nested `children` arrays). But UL glyphs have non-tree relationships: intersecting marks share visual space, connections cross-reference distant marks, and self-referential glyphs create cycles. A tree can't faithfully represent these.

**Why it's hard:** Trees are simple and JSON-native. Graphs are expressive but harder to serialize, validate, traverse, and diff. We need graph expressiveness with tree ergonomics.

**Approach: Flat node pool + typed edge list + tree-spine accessor**

The GIR is a graph, serialized as two arrays: nodes and edges. The tree structure is a *view* (a spanning tree extracted from containment edges), not the primary representation.

```json
{
  "ul_gir": "1.0",
  "root": "n1",
  "nodes": [
    {"id": "n1", "type": "enclosure", "shape": "circle", "label": "concept"},
    {"id": "n2", "type": "point", "label": "existence"},
    {"id": "n3", "type": "line", "direction": [1, 0], "label": "relation"},
    {"id": "n4", "type": "angle", "measure": 60, "label": "quality"},
    {"id": "n5", "type": "enclosure", "shape": "circle", "label": "embedded-concept"}
  ],
  "edges": [
    {"source": "n1", "target": "n2", "type": "contains"},
    {"source": "n1", "target": "n3", "type": "contains"},
    {"source": "n3", "target": "n4", "type": "modified_by"},
    {"source": "n1", "target": "n5", "type": "contains"},
    {"source": "n2", "target": "n5", "type": "intersects"},
    {"source": "n5", "target": "n1", "type": "references"}
  ]
}
```

**Edge types:**

| Type | Meaning | Tree-spine? | Layout effect |
|------|---------|-------------|---------------|
| `contains` | Parent encloses child | **Yes** (these edges form the tree) | Child is inside parent's bounding box |
| `adjacent` | Marks share a boundary point | No | Marks touch visually |
| `intersects` | Marks overlap in visual space | No | Marks are drawn overlapping |
| `connects` | Directed relation from source to target | No | Line/arrow drawn between marks |
| `modified_by` | Modifier applies to a mark/relation | **Yes** (these descend the tree) | Modifier decorates parent |
| `references` | Cross-reference or self-reference | No | Dashed line or semantic link (may create cycles) |

**Tree-spine extraction:**

```
ALGORITHM: extract_tree(gir)
  tree_edges = [e for e in gir.edges if e.type in {"contains", "modified_by"}]
  assert tree_edges form a valid tree rooted at gir.root
  return Tree(gir.root, tree_edges)
```

The tree spine is used for: bounding box allocation (§8.1 Level 2), top-down traversal, serialization ordering.

The non-tree edges are used for: intersection rendering, cross-reference routing, cycle detection (self-referential glyphs).

**Validation rules:**
1. Every node must be reachable from root via tree-spine edges (no orphans)
2. `contains` edges must form an acyclic tree (no enclosure contains itself via `contains`)
3. `references` edges MAY create cycles (this is how self-reference works)
4. `intersects` edges must be between nodes at the same tree depth or adjacent depths

**Why JSON is still the right serialization:** Despite the graph model, JSON remains the best serialization format (scored 42/55) because:
- The two-array format (nodes + edges) is clean JSON
- Every JSON tool in existence can read it
- The graph structure is explicit, not hidden in nesting conventions
- jq can query both arrays

---

### 8.3 Composer Workflow Evaluation

**Problem:** How do humans actually create UL glyphs? The roster evaluated *formats* and *tools* but never asked about *workflow*. A beautiful tool with the wrong workflow is useless.

**Why it's hard:** Different users have fundamentally different creative processes. A calligrapher, a mathematician, a philosopher, and an AI researcher will not compose UL the same way.

**Evaluation of five workflows:**

**Workflow A: Text-first (the programmer model)**

User types UL-Script in an editor with live preview.

```
User types: [ /3{*} ->@60 /0{*} ]
Left panel:  Shows the UL-Script source
Right panel: Shows the rendered glyph updating live
```

- **Strengths:** Precise, reproducible, version-controllable. AI agents use this natively.
- **Weaknesses:** Requires learning the script. The mapping from text to visual is not obvious for newcomers.
- **Who it's for:** Developers, power users, AI agents.
- **Implementation:** Monaco Editor with custom language mode. This is Phase 3 of the implementation order.

**Workflow B: Canvas-first (the designer model)**

User drags and drops geometric primitives onto a 2D canvas.

```
Toolbox: [Point] [Line] [Angle] [Curve] [Enclosure]
Canvas:  User places an enclosure, drags a point inside, draws a line...
Output:  GIR is generated from canvas state. UL-Script can be exported.
```

- **Strengths:** Intuitive. Visual. No text syntax to learn.
- **Weaknesses:** Hard to be precise (is that angle 60° or 63°?). Poor for complex glyphs. Not version-controllable (binary canvas state). AI agents can't use this easily.
- **Who it's for:** Artists, newcomers, one-off glyph creation.
- **Implementation:** HTML5 Canvas or SVG editor with snap-to-grid and angle constraints. This is a significant frontend effort (comparable to a simplified Figma).

**Workflow C: Paper-first (the calligrapher model)**

User draws on paper (or tablet), computer vision recognizes the glyph.

```
User draws:  Pen on paper or stylus on tablet
CV system:   Recognizes geometric primitives and their relationships
Output:      UL-GIR (from recognized structure)
```

- **Strengths:** Most natural for handwriting. The Writer's Companion was designed for this context. Preserves the gestural, embodied quality of UL composition.
- **Weaknesses:** CV recognition of geometric relationships is a hard problem (angle precision, containment detection, intersection vs. accidental overlap). Error-prone for complex glyphs.
- **Who it's for:** Calligraphers, artists, embodied practice.
- **Implementation:** Requires a trained CV model. This is a research problem, not an engineering task. Defer to Phase 5+.

**Workflow D: Transform-existing (the analyst model)**

User starts with an existing glyph and applies operations to transform it.

```
User selects: "existence" (canonical glyph from lexicon)
User applies: embed (turns it into an entity)
User applies: modify_entity with "quality" modifier
Output:       New glyph, built by composing operations on known glyphs
```

- **Strengths:** Algebraic. Uses the Σ_UL operations directly. Every step is meaningful. Closest to the formal system.
- **Weaknesses:** Requires understanding the 13 operations. Indirect for users who "see" the target glyph but don't know the transformation path.
- **Who it's for:** Mathematicians, formal linguists, AI agents reasoning about UL structure.
- **Implementation:** Operation palette in the web editor. Each operation takes the current selection and produces a new glyph. This is a UI feature on top of the GIR manipulation layer.

**Workflow E: Natural-language-first (the AI-assisted model)**

User describes the intended meaning in natural language. AI generates UL-Script or GIR.

```
User types: "The concept of knowledge containing the relation of learning,
             modified by continuous quality"
AI returns: [ /0{●} |→ /3{~ /0{●}} ]   (or GIR)
User edits: Tweaks the generated glyph
```

- **Strengths:** Lowest barrier to entry. No syntax or theory knowledge needed. AI does the hard work.
- **Weaknesses:** AI may misinterpret. User has no control over structure. Not reproducible (same prompt → different glyphs). Undermines the user's own geometric reasoning development.
- **Who it's for:** Newcomers, rapid prototyping, accessibility.
- **Implementation:** LLM prompt engineering + UL-Script generation + validation. This is a feature, not a separate tool.

**Recommendation:** Support Workflow A (text-first) as the primary interface, with Workflow D (transform-existing) as a power feature. Workflow E (NL-first) is a useful on-ramp. Workflow B (canvas) is a future frontend. Workflow C (paper) is a research project.

---

### 8.4 Collaboration Model

**Problem:** How do multiple people (or AIs) work on the same UL document simultaneously? How do you diff, merge, and review changes to geometric structures?

**Why it's hard:** Text diff (line-by-line comparison) doesn't work for graphs. Two different serializations of the same graph will show as completely different in `git diff`. Merge conflicts in JSON are syntactic, not semantic.

**Approach: Structural diff on the GIR graph**

**8.4.1 Structural Diff**

Compare two GIR documents by their graph structure, not their serialization:

```
ALGORITHM: structural_diff(gir_a, gir_b)
  1. Match nodes by ID (stable identifiers)
  2. For each node in gir_a ∩ gir_b:
       Report property changes (type, label, measure, etc.)
  3. Report nodes in gir_a \ gir_b as deletions
  4. Report nodes in gir_b \ gir_a as additions
  5. Repeat for edges
  6. Compute semantic summary:
       "Added enclosure n7 containing point n8"
       "Changed angle n4 from 60° to 90°"
       "Removed connection from n2 to n5"
```

This produces a meaningful diff that can be displayed visually (highlight added/removed/changed marks on the rendered glyph) or textually (structured change description).

**8.4.2 Merge Strategy**

UL glyphs are compositional, which helps. Two changes to different subtrees of the containment spine are always mergeable. Conflicts arise only when:
- Both sides modify the same node's properties
- Both sides add/remove edges to the same node
- One side deletes a node the other side modifies

For these conflicts, use the standard three-way merge with manual resolution, but present conflicts visually: show the base glyph, local glyph, and remote glyph side by side.

**8.4.3 Real-time Collaboration (CRDT)**

For simultaneous editing (Google Docs-style), use a CRDT (Conflict-free Replicated Data Type) over the GIR graph:

- Each node and edge has a unique, immutable ID (UUID v7 for time-ordering)
- Add operations are commutative (adding node A then node B = adding B then A)
- Delete uses tombstones (mark as deleted, don't remove — ensures convergence)
- Property changes use last-writer-wins on individual fields

This is a well-studied problem. Libraries: **Yjs** (JavaScript), **Automerge** (Rust+WASM), **Diamond Types** (Rust). The graph-with-tree-spine model maps cleanly to Yjs maps and arrays.

**Comparable systems:** Figma uses CRDTs for real-time canvas collaboration. The GIR is simpler than Figma's scene graph, so the CRDT layer is correspondingly simpler.

---

### 8.5 The SVG Sync Problem

**Problem:** If rendered `.ul.svg` files embed the GIR as metadata (§6.2), the visual and structural representations can desynchronize. Someone edits the SVG paths without updating the GIR metadata, or modifies the GIR without re-rendering. The document now "lies."

**Why it's hard:** SVG editors (Inkscape, Illustrator, browser devtools) don't know about the GIR metadata. They will happily modify the visual paths and leave the metadata stale.

**Approach: Three-part solution**

**Part 1: Canonical hash**

The GIR includes a hash of itself. The SVG visual rendering includes (as a hidden `<desc>` element) a hash of the GIR it was generated from.

```xml
<svg>
  <metadata>
    <ul:gir xmlns:ul="urn:ul:gir:1.0" hash="sha256:a1b2c3...">
      {"ul_gir": "1.0", "nodes": [...], "edges": [...]}
    </ul:gir>
  </metadata>
  <desc class="ul-render-hash">sha256:a1b2c3...</desc>
  <!-- Visual elements -->
</svg>
```

When hashes match → GIR and visuals are in sync. When they differ → the file has been modified outside the UL toolchain.

**Part 2: Regeneration policy**

UL tools treat the GIR as canonical. When opening a `.ul.svg`:
1. Check hash match
2. If mismatch: warn user, offer to re-render from GIR (discarding visual edits) or re-extract GIR from visuals (lossy — not always possible)
3. If match: proceed normally

**Part 3: Separation of concerns**

For critical use cases (archival, interchange), distribute `.ul.json` (GIR only) as the canonical format. The `.ul.svg` is a derived artifact — like a compiled binary from source. If the SVG is modified, the change should be made in UL-Script or GIR, then re-rendered.

**The honest answer:** The "lying document" problem is not fully solvable. Any format that embeds two representations will have this risk. The mitigation is: (1) hashes for detection, (2) GIR-is-canonical policy, (3) culture of treating SVG as derived output. This parallels how we handle compiled code — you don't edit the `.o` file.

---

### 8.6 Beyond-LLM AI Interfaces

**Problem:** The roster scored "AI legibility" as if AI = LLM. But AI systems include vision models, graph neural networks, theorem provers, and symbolic reasoning engines. Each needs a different interface to UL.

**Approach: Four interface modalities**

**8.6.1 LLM Interface (text)**

Already addressed: LLMs consume UL-Script (text) and JSON-GIR (structured text). The Unicode UL-Script (using iconic symbols like △□○→) is moderately more legible than ASCII to a tokenizer because the symbols are semantically distinctive.

**8.6.2 Vision Model Interface (image)**

Vision models (GPT-4V, Claude's vision, Gemini) can look at rendered UL glyphs. This is useful for:
- **Recognition:** "What UL glyph is this?" → extract structure from image
- **Comparison:** "Are these two glyphs structurally equivalent?" → visual isomorphism check
- **Quality assessment:** "Is this glyph well-formed?" → aesthetic and structural evaluation

**Interface format:** PNG/SVG rendering at high resolution with clean lines and no aliasing artifacts. The visual rendering should be optimized for CV recognition:
- Consistent line weights
- High contrast (black on white)
- No decorative elements that confuse structure recognition
- Scale bar or reference marks for absolute sizing

**8.6.3 Graph Neural Network Interface (tensor)**

GNNs operate on adjacency matrices and node feature vectors. The GIR maps directly:

```python
# Node feature matrix: N × F
# Each row = [type_onehot(5), measure, direction_x, direction_y, ...]
node_features = torch.tensor([
    [1,0,0,0,0, 0.0, 0.0, 0.0],  # n1: enclosure
    [0,1,0,0,0, 0.0, 0.0, 0.0],  # n2: point
    [0,0,1,0,0, 0.0, 1.0, 0.0],  # n3: line (direction=[1,0])
    [0,0,0,1,0, 60., 0.0, 0.0],  # n4: angle (measure=60)
])

# Typed adjacency: E × 3 (source, target, edge_type)
edge_index = torch.tensor([
    [0, 1, CONTAINS],
    [0, 2, CONTAINS],
    [2, 3, MODIFIED_BY],
    [1, 4, INTERSECTS],
])
```

This is the standard PyTorch Geometric format. Any GNN library (PyG, DGL, Jraph) can consume it directly.

**Use cases:** Learning glyph similarity, automatic layout optimization (train a GNN to predict good layouts from GIR), anomaly detection (identify structurally unusual glyphs).

**8.6.4 Theorem Prover Interface (logic)**

UL has algebraic properties (sort correctness, associativity of conjoin/disjoin, involution of negate/invert). These can be encoded for formal verification:

**SMT-LIB (Z3, CVC5):**
```smt2
(declare-sort Entity)
(declare-sort Relation)
(declare-sort Modifier)
(declare-sort Assertion)

(declare-fun predicate (Entity Relation Entity) Assertion)
(declare-fun negate (Assertion) Assertion)
(declare-fun embed (Assertion) Entity)

; Verify: negate is an involution
(assert (forall ((a Assertion)) (= (negate (negate a)) a)))
```

**Lean 4:**
```lean
inductive Sort where
  | entity | relation | modifier | assertion

structure ULAlgebra where
  carrier : Sort → Type
  predicate : carrier .entity → carrier .relation → carrier .entity → carrier .assertion
  negate : carrier .assertion → carrier .assertion
  negate_involution : ∀ a, negate (negate a) = a
```

**Use cases:** Verify that GIR transformations preserve sort correctness. Prove that composed glyphs satisfy UL axioms. Check that the layout algorithm respects geometric constraints.

---

### 8.7 Honest Implementation Estimates

**Problem:** The original roster estimated line counts by hallucination ("~2000 lines for the renderer"). Real systems are always larger than estimates. Providing fake numbers creates false confidence and bad planning.

**Approach: Anchor to comparable real systems, then adjust**

| Component | Comparable system | Their size | UL scope | Estimate |
|-----------|------------------|-----------|----------|----------|
| **Parser** (UL-Script → GIR) | Markdown parsers (marked, remark) | 2-5k lines | UL grammar is simpler than Markdown | **1-2k lines** Rust |
| **Renderer** (GIR → SVG) | Graphviz `dot` layout engine | ~80k lines C | UL is much more constrained; templates handle common cases | **5-15k lines** Rust (Level 1+2, §8.1) |
| **Renderer** (full, with Level 3 optimization) | Penrose | ~30k lines | UL constraints are less general | **15-25k lines** Rust |
| **Validator** | JSON Schema validators | 1-3k lines | Plus graph-invariant checks | **1-3k lines** Rust |
| **CLI wrapper** | ripgrep, fd CLI layer | 500-1k lines | Thin wrapper around core | **500-1k lines** Rust |
| **Web frontend** | CodeMirror-based editors | 5-15k lines | Editor + preview + toolbox | **5-10k lines** TypeScript |
| **VS Code extension** | Mermaid Preview, LaTeX Workshop | 3-8k lines | Language server + preview | **3-5k lines** TypeScript |
| **API service** | Small REST services | 1-2k lines | Wraps core | **1-2k lines** |
| **Python library** | matplotlib's thin wrappers | 1-3k lines | Bindings + Jupyter magic | **1-2k lines** Python |

**Total realistic estimate for a complete, solid implementation:**
- **Core (parser + renderer + validator + CLI):** 8-25k lines Rust
- **Frontends (web + VS Code + Jupyter):** 10-17k lines TypeScript/Python
- **API + ecosystem:** 3-5k lines
- **Grand total:** 20-50k lines across all languages

**For comparison:**
- MathJax (math rendering): ~60k lines
- Mermaid (diagram rendering): ~40k lines
- Graphviz (graph rendering): ~200k lines
- KaTeX (fast math rendering): ~20k lines

UL is less complex than any of these mature systems but more complex than a toy project. The 20-50k range is honest: the low end assumes aggressive use of templates and existing layout libraries; the high end assumes a custom layout engine comparable to a simplified Graphviz.

**What this means for planning:**
- An MVP (parser + template renderer + basic web frontend) is achievable with 5-8k lines
- A solid v1.0 is a multi-person-month effort
- A polished system comparable to MathJax or Mermaid requires sustained development

---

*This roster evaluates candidates as of the current specification. As the system is implemented, candidates may be re-evaluated based on empirical results. The hard problems (§8) represent our best current thinking — prototyping will reveal which approaches work and which need revision.*
