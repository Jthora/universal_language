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

Every candidate is scored on six axes. The first two are the user's core requirement — **human-AI interoperability** — and the rest are engineering viability.

| Axis | What it measures | Weight |
|------|------------------|--------|
| **H — Human legibility** | Can a trained human read/write it without tools? | Critical |
| **A — AI legibility** | Can an LLM parse, generate, and reason about it in-context? | Critical |
| **R — Roundtrip fidelity** | Does structure survive encode → decode → re-encode? | High |
| **E — Extensibility** | Can new primitives/operations be added without breaking existing encodings? | Medium |
| **I — Implementation cost** | How much code to build a working prototype? | Medium |
| **P — Ecosystem leverage** | Does it ride existing libraries, parsers, renderers? | Medium |

Scores: **5** = exceptional, **4** = strong, **3** = adequate, **2** = weak, **1** = failing.

---

## 1. LAYER 1 CANDIDATES: KEYBOARD SCRIPT

The keyboard script is where **human writability** matters most. This is what someone types at a keyboard or pastes in a chat message. It must be ASCII-only, parseable, and compact.

### Candidate 1A: UL-Script (Custom DSL)

The format specified in §XIII of glyph-composition.md. Purpose-built for UL.

```
[ /3{*} ->@60 /0{*} ]       # "A fundamental thing harmonizes with a universal thing"
/0{ /4{ /0{*} } }           # "structured truth" (circle containing square containing circle containing point)
[ * ->@0 #[*a ->@60 *b] ]   # "Something acts on the fact that a harmonizes with b"
```

| Axis | Score | Notes |
|------|-------|-------|
| H | **4** | Compact, learnable. `/3{}` for triangle is intuitive once learned. Not self-documenting for newcomers. |
| A | **5** | Small token footprint, unambiguous BNF grammar, trivially parseable by LLMs. |
| R | **5** | Every GIR parameter is encodable. Lossless by design. |
| E | **4** | New primitives = new keywords. `/7{}` for heptagon works automatically. |
| I | **4** | Parser is ~500 lines. BNF already written. |
| P | **2** | No existing ecosystem. Must build everything. |
| **Total** | **24** | |

**Verdict:** Strong. Purpose-built means optimal fit. Low ecosystem leverage is the only weakness, and it's inevitable for a novel system.

---

### Candidate 1B: S-Expressions (Lisp-style)

Meaning structures as nested s-expressions. Extremely regular syntax.

```lisp
(frame (predicate (tri (point)) (line :dir forward :angle 60) (circ (point))))
(circ (sq (circ (point))))
(frame (predicate (point) (line :dir forward :angle 0) (embed (frame (predicate (point :id a) (line :dir forward :angle 60) (point :id b))))))
```

| Axis | Score | Notes |
|------|-------|-------|
| H | **3** | Readable for Lisp users. Verbose for everyone else. Parenthesis depth taxes human parsing. |
| A | **5** | LLMs excel at s-expressions. Training data is vast (Lisp, Scheme, Clojure). Perfectly structured. |
| R | **5** | Complete fidelity. Every parameter is a named keyword argument. |
| E | **5** | S-expressions are inherently extensible — add new node types freely. |
| I | **3** | Parser is trivial. But need schema validation on top. |
| P | **4** | Vast s-expression ecosystem. Parsers in every language. |
| **Total** | **25** | |

**Verdict:** Highest total score. AI-optimal (LLMs handle s-expressions natively). Human readability is the weakness — but only for depth > 3 nesting, which is where ALL text formats struggle for UL.

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

| Axis | Score | Notes |
|------|-------|-------|
| H | **4** | Indentation is natural. Verbose but self-documenting. |
| A | **4** | LLMs handle YAML well. Slightly more tokens than s-expr. Whitespace-sensitive = fragile in chat contexts. |
| R | **5** | Full fidelity via explicit keys. |
| E | **5** | New keys = new capabilities. No breaking changes. |
| I | **3** | YAML parsers exist everywhere. Schema layer needed. |
| P | **5** | Massive ecosystem. JSON Schema for validation. |
| **Total** | **26** | |

**Verdict:** Highest ecosystem score. But **critical flaw for chat/messaging:** YAML's whitespace sensitivity means copy-paste between chat apps, emails, and forums frequently corrupts structure. A human-input format that breaks when pasted into Discord or Slack is unacceptable.

**Downgraded H score for real-world use: 2.** Revised total: **24**.

---

### Candidate 1D: Markdown-Embedded DSL

UL-Script embedded in Markdown code fences. Literate style.

````markdown
The concept of "structured truth":

```ul
/0{ /4{ /0{*} } }
```

Reads as: a universal concept (circle) containing an ordered structure (square)
containing a complete concept (circle) containing existence (point).
````

| Axis | Score | Notes |
|------|-------|-------|
| H | **5** | Annotated. Self-documenting. Humans see both formal and natural language. |
| A | **5** | LLMs parse code fences natively. The `ul` fence tag lets them switch parser modes. |
| R | **5** | Same as UL-Script inside the fence. Markdown around it is commentary. |
| E | **4** | Same as UL-Script. |
| I | **4** | Parse Markdown → extract `ul` fences → parse UL-Script. Two existing parsers chained. |
| P | **4** | Markdown is universal. Renders in GitHub, VS Code, Jupyter, etc. |
| **Total** | **27** | |

**Verdict:** Best dual-legibility candidate. Humans get annotated, readable documents. AI agents extract the `ul` code fences and parse them structurally. The Markdown envelope is NOT part of the formal language — it's a presentation layer for human consumption. The formal content is the UL-Script inside the fences.

---

### Candidate 1E: LaTeX-Style Macros

UL as a LaTeX package. Leverages the most successful mathematical typesetting ecosystem.

```latex
\ulframe{\ulpred{\ultri{\ulpoint}}{\ulrel[dir=forward,angle=60]}{\ulcirc{\ulpoint}}}
```

| Axis | Score | Notes |
|------|-------|-------|
| H | **2** | Only readable by LaTeX users. Macro nesting is visually noisy. |
| A | **4** | LLMs can generate LaTeX. But macro expansion is complex and error-prone. |
| R | **5** | Full fidelity via macro parameters. |
| E | **4** | New macros = new capabilities. |
| I | **2** | Requires LaTeX ecosystem. Parser must handle macro expansion. Overkill dependencies. |
| P | **3** | LaTeX ecosystem exists but is heavy and academic-only. |
| **Total** | **20** | |

**Verdict:** Weak. LaTeX is the right analogy (structural encoding → visual rendering) but the wrong vehicle (too heavy, too niche, too noisy).

---

### Candidate 1F: JSON Lines (Machine-Primary)

One JSON object per UL expression, newline-delimited.

```json
{"frame":{"subject":{"enc":"tri","c":[{"pt":true}]},"rel":{"dir":"fwd","a":60},"object":{"enc":"circ","c":[{"pt":true}]}}}
```

| Axis | Score | Notes |
|------|-------|-------|
| H | **1** | Essentially unreadable for humans. Bracket soup. |
| A | **5** | LLMs and machines parse JSON natively. Structured data is their natural habitat. |
| R | **5** | Full fidelity. |
| E | **5** | New keys trivially. |
| I | **5** | Zero parser work. JSON exists everywhere. |
| P | **5** | Maximum ecosystem leverage. |
| **Total** | **26** | |

**Verdict:** AI-optimal but human-hostile. This is effectively the GIR (Layer 2), not the keyboard script. Its presence here confirms: **a human-input format MUST be different from the machine-interchange format.**

---

### Layer 1 Ranking

| Rank | Candidate | Total | Dual-legibility |
|------|-----------|-------|-----------------|
| **1** | **1D: Markdown-Embedded UL-Script** | 27 | Both excellent |
| **2** | **1B: S-Expressions** | 25 | AI excellent, human good |
| **3** | **1A: UL-Script (plain)** | 24 | Both good |
| **3** | **1C: YAML** | 24* | Fragile in practice |
| **5** | **1E: LaTeX macros** | 20 | Both weak |
| **6** | **1F: JSON Lines** | 26 | AI only |

**Recommendation:** **1D (Markdown-Embedded UL-Script)** as the primary human-AI interchange format. The UL-Script inside the fences IS the formal language; the Markdown around it is the human annotation layer. AI agents strip the Markdown to get pure structure; humans read the Markdown to get context.

**For pure machine-to-machine communication:** Use raw UL-Script (1A) without the Markdown envelope. Compact, unambiguous, minimal tokens.

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

| Axis | Score | Notes |
|------|-------|-------|
| H | **3** | JSON is readable with effort. Deep nesting is hard. |
| A | **5** | JSON is the *de facto* data interchange format. Every LLM handles it natively. |
| R | **5** | All 9 composition parameters encoded. Explicit spatial relationships. |
| E | **5** | New keys, new types, versioned schema. |
| I | **5** | Zero parser work. JSON Schema for validation. |
| P | **5** | Maximum ecosystem leverage. Every language, every tool. |
| **Total** | **28** | |

**Verdict:** Hard to beat. JSON is the lingua franca of structured data. The only weakness is deep nesting readability for humans — but humans aren't the primary audience for Layer 2.

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

| Axis | Score | Notes |
|------|-------|-------|
| H | **1** | Binary format. Completely opaque to humans. |
| A | **2** | LLMs cannot generate or read binary protobuf. Need text serialization (protobuf JSON mapping). |
| R | **5** | Schema-enforced. Complete fidelity. |
| E | **5** | Protobuf is designed for backward-compatible evolution. |
| I | **3** | Requires protobuf toolchain. Schema must be maintained. |
| P | **4** | Large ecosystem (Google-backed). |
| **Total** | **20** | |

**Verdict:** Wrong paradigm for human-AI interop. Binary formats are optimal for bandwidth-constrained machine-to-machine communication, useless for dual-legibility. If bandwidth ever matters (e.g., streaming millions of GIR documents), protobuf can be a secondary wire format — but JSON is primary.

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

| Axis | Score | Notes |
|------|-------|-------|
| H | **3** | Readable with training. Verbose. Tag noise. |
| A | **4** | LLMs handle XML. More tokens than JSON for same content. |
| R | **5** | Full fidelity. Schema-validatable via XSD or RelaxNG. |
| E | **5** | Namespaces, attributes, extensible by design. |
| I | **3** | XML parsers everywhere. But XML tooling is declining. |
| P | **3** | XML ecosystem is mature but stagnant. MathML is supported in browsers. |
| **Total** | **23** | |

**Verdict:** MathML is the closest existing analogy (semantic markup for a formal system rendered visually). But XML has lost the ecosystem war to JSON. If MathML were thriving, this would be the best candidate — but it isn't.

**Notable:** If UL is ever formalized for W3C or browser-native rendering, an XML namespace may be required. Keep this in reserve.

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

| Axis | Score | Notes |
|------|-------|-------|
| H | **2** | Graph notation. Requires graph-thinking. Not linear. |
| A | **3** | LLMs can handle Turtle syntax but don't excel at graph reasoning. |
| R | **5** | Complete fidelity. Graphs preserve all relationships. |
| E | **5** | RDF is infinitely extensible by design (add new predicates freely). |
| I | **2** | Requires triple store or RDF library. Heavy infrastructure. |
| P | **3** | Semantic web ecosystem exists but is niche. SPARQL for querying. |
| **Total** | **20** | |

**Verdict:** Theoretically elegant — UL IS a semantic graph, and RDF IS a semantic graph format. But the ecosystem is too niche, LLM support is mediocre, and the triple-based representation fragmentes what should be coherent tree structures into disconnected statements. The GIR already captures relationships through its scene graph; RDF would flatten the tree into a graph unnecessarily.

**Notable:** If UL-GIR documents are ever published as linked data on the web (semantic web of meaning), RDF export from JSON-GIR would be straightforward.

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

| Axis | Score | Notes |
|------|-------|-------|
| H | **2** | SQL is readable for developers. Not for UL practitioners. |
| A | **4** | LLMs generate SQL well. But SQL doesn't embed in chat. |
| R | **5** | Full fidelity. Queryable: "SELECT * FROM marks WHERE parent_id = 'frame1'". |
| E | **4** | ALTER TABLE for new columns. Migration needed. |
| I | **3** | SQLite is everywhere. Schema must be maintained. |
| P | **4** | Massive SQL ecosystem. |
| **Total** | **22** | |

**Verdict:** Interesting for a large-scale UL corpus (thousands of documents, need querying). Wrong for single-document interchange. The relational model fragments the tree structure — reassembling a glyph from relational rows is extra work. But **a relational backend behind the JSON-GIR API** makes sense for storage.

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

| Axis | Score | Notes |
|------|-------|-------|
| H | **2** | Graph query languages are not human-friendly for reading. |
| A | **3** | LLMs handle GraphQL but graph traversal reasoning is weak. |
| R | **5** | Graph model perfectly captures UL's spatial relationships. |
| E | **5** | New node types, new edge types. |
| I | **2** | Requires graph database infrastructure. |
| P | **3** | Growing but niche ecosystem. |
| **Total** | **20** | |

**Verdict:** The data model is a near-perfect match — UL compositions ARE property graphs (typed nodes with geometric properties, typed edges for spatial relationships). But requiring a graph database is too heavy for interchange. **Best as a query backend** — index JSON-GIR documents into a property graph for advanced querying ("find all compositions where a D₃ triangle contains an SO(2) circle").

---

### Layer 2 Ranking

| Rank | Candidate | Total | Key strength |
|------|-----------|-------|-------------|
| **1** | **2A: JSON Scene Graph** | 28 | Universal, AI-native, maximum leverage |
| **2** | **2C: XML/MathML-style** | 23 | Closest existing precedent (MathML) |
| **3** | **2E: SQLite/Relational** | 22 | Best for corpus-scale querying |
| **4** | **2B: Protobuf** | 20 | Best for wire-format efficiency |
| **4** | **2D: RDF** | 20 | Best for linked-data publishing |
| **4** | **2F: Property Graph** | 20 | Best structural match for UL |

**Recommendation:** **2A (JSON Scene Graph)** as the primary GIR format. It wins on AI legibility, ecosystem leverage, and implementation cost. The schema should be formalized as **JSON Schema** for validation.

**Secondary formats for specific use cases:**
- **Protobuf** wire format for high-volume streaming
- **SQLite** backend for corpus storage and querying
- **Property graph** index for advanced structural queries
- **RDF** export for semantic web publishing

All secondary formats are generated FROM the canonical JSON-GIR, not authored directly.

---

## 3. LAYER 3 CANDIDATES: RENDERING ENGINE

The renderer is where **visual fidelity** matters most. This is what humans see. AI agents don't consume rendered output — they work with Layers 1 and 2.

### Candidate 3A: SVG via JavaScript (D3.js / custom)

Generate SVG elements from GIR programmatically in the browser.

| Axis | Score | Notes |
|------|-------|-------|
| H | **5** | SVG renders natively in all browsers. Scalable, printable, crisp. |
| A | **3** | AI can generate SVG but doesn't need to — it works with GIR. |
| R | **N/A** | Rendering is one-way (lossy). Round-trip requires CV pipeline. |
| E | **4** | New mark types = new SVG generation functions. |
| I | **3** | Custom renderer (~2000 lines). D3.js helps with layout. |
| P | **5** | SVG is a web standard. CSS styling. DOM manipulation. Event handling. |
| **Total** | **20** | |

**Verdict:** The default web rendering target. Browser-native, scalable, styleable, interactive. D3.js provides layout algorithms. This is the obvious first implementation.

---

### Candidate 3B: HTML Canvas 2D

Immediate-mode pixel rendering.

| Axis | Score | Notes |
|------|-------|-------|
| H | **4** | Smooth rendering. Fast for complex scenes. No DOM overhead. |
| A | **2** | Canvas output is pixels — opaque to AI. |
| R | **N/A** | Even more lossy than SVG (pixels, not vectors). |
| E | **3** | New marks = new draw functions. No structure in output. |
| I | **3** | Similar to SVG but no layout library. |
| P | **4** | Canvas is universal in browsers. |
| **Total** | **16** | |

**Verdict:** Better for real-time interactive editing (drag marks, animate transitions). Worse for static display (no DOM = no CSS = no accessibility). Use as a secondary target for interactive UL editors.

---

### Candidate 3C: WebGL / GPU Rendering

GPU-accelerated 2D rendering via shaders.

| Axis | Score | Notes |
|------|-------|-------|
| H | **5** | Smoothest possible rendering. Handles massive complexity. |
| A | **1** | Completely opaque to AI. |
| R | **N/A** | Pixel output. |
| E | **3** | Shader programming is expensive. |
| I | **1** | Significant engineering effort. Shader pipeline from scratch. |
| P | **3** | Three.js, regl, etc. exist but are overkill for 2D. |
| **Total** | **13** | |

**Verdict:** Future-proofs for 3D UL composition (when the system extends to 3D volumes per the open problems). Overkill for current 2D needs. Reserve for v2.0.

---

### Candidate 3D: PDF Generation (server-side)

Generate PDF documents from GIR. For formal/printed UL.

| Axis | Score | Notes |
|------|-------|-------|
| H | **5** | PDF is the gold standard for print. |
| A | **1** | PDF is opaque. |
| R | **N/A** | One-way to paper. |
| E | **3** | PDF generation libraries handle arbitrary graphics. |
| I | **3** | Libraries exist (pdfkit, ReportLab, Puppeteer screenshot). |
| P | **4** | PDF is universal for documents. |
| **Total** | **16** | |

**Verdict:** Necessary for formal publication ("UL papers"). Not the primary target. Generate PDF by rendering SVG → PDF (via Puppeteer or similar).

---

### Candidate 3E: TikZ / LaTeX Backend

Generate TikZ code from GIR. Renders via LaTeX.

| Axis | Score | Notes |
|------|-------|-------|
| H | **5** | LaTeX produces the highest-quality geometric figures. |
| A | **3** | AI can generate TikZ. But this is Layer 3, not Layer 1. |
| R | **N/A** | One-way to paper/PDF. |
| E | **4** | TikZ is extremely powerful for 2D geometry. |
| I | **3** | TikZ code generation from a tree is straightforward. |
| P | **4** | LaTeX is universal in academia. |
| **Total** | **19** | |

**Verdict:** Excellent for academic publication. If UL papers appear in journals, TikZ rendering ensures publication-quality figures. Secondary target after SVG.

---

### Candidate 3F: Pen Plotter / Physical Drawing

Generate G-code, HPGL, or SVG-for-plotter from GIR. Draws with an actual pen.

| Axis | Score | Notes |
|------|-------|-------|
| H | **5** | Full circle: geometric writing → physical pen on paper. |
| A | **1** | Physical artifact. |
| R | **N/A** | One-way to paper. |
| E | **3** | Limited by physical pen capabilities (no fill, no z-order, no overlap). |
| I | **3** | SVG → plotter conversion is established. |
| P | **3** | Niche but growing (AxiDraw, pen plotters). |
| **Total** | **15** | |

**Verdict:** Philosophically important — UL is a writing system, and physical writing grounds the digital representation. Practically, it's a bonus target after web rendering is solid.

---

### Layer 3 Ranking

| Rank | Candidate | Total | Key strength |
|------|-----------|-------|-------------|
| **1** | **3A: SVG (JavaScript)** | 20 | Web-native, scalable, interactive, styleable |
| **2** | **3E: TikZ/LaTeX** | 19 | Academic publication quality |
| **3** | **3B: Canvas 2D** | 16 | Real-time interactive editing |
| **3** | **3D: PDF** | 16 | Print/formal documents |
| **5** | **3F: Pen Plotter** | 15 | Physical grounding |
| **6** | **3C: WebGL** | 13 | Future 3D extension |

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

| Axis | Score | Notes |
|------|-------|-------|
| H | **5** | Split-pane: type on left, see glyph on right. Markdown mode for annotated docs. |
| A | **5** | GIR exposed via API. AI agents POST UL-Script or GIR, receive rendered SVG or GIR back. |
| R | **5** | GIR is the canonical state. Everything round-trips through it. |
| E | **5** | Web tech is infinitely extensible. |
| I | **3** | Moderate effort: parser + renderer + editor + API. |
| P | **5** | React/Vue, Monaco Editor, D3.js. Massive ecosystem. |
| **Total** | **28** | |

---

### Candidate 4B: CLI Toolchain (Unix Philosophy)

Separate tools piped together: `ul-parse`, `ul-render`, `ul-validate`, `ul-convert`.

```bash
echo '[ /3{*} ->@60 /0{*} ]' | ul-parse | ul-render --format svg > output.svg
cat document.ul | ul-parse | ul-validate --schema ul-gir-1.0.json
cat *.ul | ul-parse | ul-query "//enclosure[@symmetry='SO2']"
```

| Axis | Score | Notes |
|------|-------|-------|
| H | **3** | Command-line is developer-friendly. Not for general users. |
| A | **5** | Pipeable. AI agents can invoke CLI tools directly. Structured I/O. |
| R | **5** | Each tool preserves structure. GIR is the interchange format between pipes. |
| E | **5** | Add new tools. Composable by design. |
| I | **4** | Each tool is small and focused. Parse is ~500 lines. Render is ~2000. |
| P | **4** | Unix ecosystem. Wrap in npm/pip/brew packages. |
| **Total** | **26** | |

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

| Axis | Score | Notes |
|------|-------|-------|
| H | **5** | Best coding experience. Inline errors. Live preview. Autocomplete. |
| A | **5** | Copilot generates UL-Script from natural language. Extension API for agent access. |
| R | **5** | Editor state IS GIR. |
| E | **5** | Extension API is extensible. |
| I | **2** | VS Code extension development is moderately heavy. Language server protocol. |
| P | **5** | VS Code is the dominant editor. Extension marketplace for distribution. |
| **Total** | **27** | |

---

### Candidate 4D: Jupyter / Notebook Integration

UL as a notebook cell type. Inline rendering. Mix code/UL/prose.

```python
# In a Jupyter cell:
%%ul
[ /3{*} ->@60 /0{*} ]
```
→ Renders inline as SVG.

| Axis | Score | Notes |
|------|-------|-------|
| H | **4** | Notebook is natural for exploration and teaching. |
| A | **5** | Python kernel. LLMs generate notebook cells. Programmatic access to GIR. |
| R | **5** | GIR accessible from Python: `gir = ul.parse("[ /3{*} ->@60 /0{*} ]")`. |
| E | **4** | Python library extensible. |
| I | **3** | Python package + Jupyter magic command. Moderate effort. |
| P | **5** | Jupyter is universal in research/education. |
| **Total** | **26** | |

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

| Axis | Score | Notes |
|------|-------|-------|
| H | **2** | Humans don't use APIs directly. Needs a frontend. |
| A | **5** | API is the native interface for AI agents. |
| R | **5** | GIR is the canonical response format. |
| E | **5** | REST/GraphQL endpoints are infinitely extensible. |
| I | **3** | Server infrastructure needed. But the parse/render logic is shared with all other candidates. |
| P | **5** | REST/GraphQL is universal. |
| **Total** | **25** | |

---

### Integration Ranking

| Rank | Candidate | Total | Key strength |
|------|-----------|-------|-------------|
| **1** | **4A: Web Application** | 28 | Best dual-legibility. Humans see, AI uses API. |
| **2** | **4C: VS Code Extension** | 27 | Best developer experience. Copilot integration. |
| **3** | **4B: CLI Toolchain** | 26 | Most composable. AI agents invoke directly. |
| **3** | **4D: Jupyter Notebook** | 26 | Best for teaching, research, exploration. |
| **5** | **4E: API Service** | 25 | Machine backbone. All other frontends use it. |

**Recommendation:** Build **4B (CLI toolchain)** first — it's the fastest path to a working system and every other integration uses the same core logic. Then wrap the core as **4E (API service)**. Then build **4A (web app)** and **4C (VS Code extension)** as frontends that call the API.

---

## 5. THE RECOMMENDED STACK

Based on the analysis above, the optimal implementation path for maximum human-AI interoperability:

### Primary Stack

| Layer | Choice | Format | Human use | AI use |
|-------|--------|--------|-----------|--------|
| **Input** | UL-Script in Markdown fences | `.ul.md` files or ` ```ul ``` ` blocks | Type in editor, annotate in Markdown | Parse code fences, extract structure |
| **Interchange** | JSON Scene Graph (UL-GIR) | `.ul.json` files | Inspect with jq / JSON viewer | Native format for processing, storage, exchange |
| **Output** | SVG (primary), TikZ (academic) | `.svg`, `.tex` | View in browser, print | Generate for human consumption |
| **Toolchain** | CLI pipes → API → web app | `ul-parse`, `ul-render`, `ul-validate` | Web editor with live preview | REST API, CLI invocation |

### Implementation Order

```
PHASE 1: CORE (CLI toolchain)
├── ul-parse:    UL-Script → UL-GIR (JSON)          ~500 lines
├── ul-render:   UL-GIR → SVG                        ~2000 lines
├── ul-validate: UL-GIR → validation report           ~300 lines
└── ul-convert:  UL-GIR ↔ CSF/JSON (legacy compat)   ~200 lines

PHASE 2: API
├── REST service wrapping Phase 1 tools               ~500 lines
├── WebSocket for live preview                         ~200 lines
└── JSON Schema for UL-GIR validation                 ~100 lines

PHASE 3: FRONTENDS
├── Web editor (React/Vue + Monaco + D3.js)           ~3000 lines
├── VS Code extension (Language Server Protocol)       ~2000 lines
└── Jupyter magic command (`%%ul`)                     ~300 lines

PHASE 4: ECOSYSTEM
├── TikZ backend                                       ~500 lines
├── PDF export (via SVG → PDF)                         ~100 lines
├── Python library (`import ul`)                       ~500 lines
├── npm package (`import { parse } from 'ul'`)        ~500 lines
└── Pen plotter backend (SVG → HPGL)                   ~200 lines
```

### Dual-Legibility Architecture

```
                    HUMAN SIDE                        AI SIDE
                    ──────────                        ───────

    Write           Type UL-Script in Markdown        Generate UL-Script
    annotated:      ````ul.md` files                  or raw JSON-GIR
                           │                                │
                           ▼                                ▼
    Parse:          `ul-parse` extracts `ul` fences   Direct JSON-GIR
                    and converts to GIR               creation/consumption
                           │                                │
                           ▼                                ▼
    Store:          ┌─────────────────────────────────────────────┐
                    │         CANONICAL UL-GIR (JSON)              │
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
| `.ul` | Raw UL-Script (no Markdown) | Machine-to-machine, piping, embedding |
| `.ul.md` | UL-Script in Markdown fences with annotations | Human-authored documents, tutorials, specifications |
| `.ul.json` | UL-GIR (JSON Scene Graph) | Machine interchange, storage, API payloads |
| `.ul.svg` | Rendered SVG with embedded GIR metadata | Display + recoverable structure |

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

This means an SVG file IS ALSO a GIR file. Any tool that receives a `.ul.svg` can extract the GIR and work with it structurally. The SVG is simultaneously human-viewable AND machine-parseable.

### 6.3 Version Negotiation

GIR documents carry a version field. Tools must:
1. Refuse to process versions they don't understand
2. Support forward-compatible reading (ignore unknown fields)
3. Require backward-compatible writing (always include required fields from v1.0)

```json
{"ul_gir": "1.0", "marks": [...]}
```

### 6.4 Language for Implementation

| Component | Recommended language | Rationale |
|-----------|---------------------|-----------|
| Parser (UL-Script → GIR) | **TypeScript** | Runs in browser AND Node.js. Typed. |
| Renderer (GIR → SVG) | **TypeScript** | DOM manipulation. D3.js integration. |
| Validator (GIR → report) | **TypeScript** | JSON Schema validation. Same runtime. |
| CLI tools | **TypeScript (tsx/node)** | Single language for entire stack. |
| Python library | **Python** | Wraps CLI or reimplements core in Python for Jupyter. |
| API service | **TypeScript (Deno/Bun/Node)** | Same language as core tools. |

**TypeScript is the recommended implementation language** because:
- Runs on both client and server (browser + Node.js)
- Strong typing catches sort/geometry errors at compile time
- JSON is native
- The ecosystem includes Monaco Editor, D3.js, Express/Fastify
- LLMs generate TypeScript fluently

---

## 7. WHAT THIS ROSTER DOES NOT DECIDE

The following decisions require prototyping and user testing, not analysis:

1. **Layout algorithm** — how the renderer positions nested marks for readability. This is an empirical question (what looks good? what are users confused by?) that requires iteration.

2. **UL-Script syntax details** — the BNF in §XIII is a draft. User testing may reveal that `/3{...}` is less intuitive than `tri{...}`, that `->@60` should be `->60`, etc. The grammar WILL evolve.

3. **GIR schema specifics** — field names, nesting conventions, how to handle edge cases (what if two marks are declared as both containing and intersecting each other?). Requires implementation experience.

4. **Performance characteristics** — how large can a GIR document get before rendering slows down? Is JSON too verbose at scale? These are empirical.

5. **Accessibility mapping** — how does UL-GIR map to screen readers, tactile displays, or audio? This is a research problem that requires collaboration with accessibility experts.

---

*This roster evaluates candidates as of the current specification. As the system is implemented, candidates may be re-evaluated based on empirical results.*
