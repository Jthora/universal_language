# Universal Symbology Map

> The complete catalogue of symbols derivable from the 5 geometric primitives.  
> Every symbol is a geometric construction. Reading is parsing. Writing is constructing.

### How This Document Relates to Its Siblings

| If you need... | Consult | Why |
|----------------|---------|-----|
| How to combine these symbols validly | **Syntax** (syntax-dictionary.md) | Syntax defines the rules; Symbology defines the pieces |
| Why these shapes carry their meanings | **Grammar** (grammar-book.md §II) | Grammar derives parts of speech from symmetry groups |
| Whether a construction is canonical | **Lexicon** (lexicon/lexicon.md) | Lexicon assigns tiers (T1/T2/T3) to compound constructions |
| Synonyms and related meanings | **Thesaurus** (thesaurus/thesaurus.md) | Thesaurus maps equivalence classes under transformation |
| How to actually write and read | **Writer's Companion** (writing-system/writers-companion.md) | Step-by-step pen-and-paper procedures |

> **Geometric vs. Algebraic Representation:** The 5 geometric primitives are both geometric objects (in the geometric algebra G) and algebraic sorts in Σ_UL. However, certain geometric properties — like the specific *shape* of an enclosure boundary (triangle vs. hexagon) — are geometric-only features that enter Σ_UL indirectly through the modifier sort (via `abstract(e) → m`). See Lexicon §0.8 for details.

---

## I. THE GLYPH SPACE

All symbols live within a **unit circle** — the Glyph Space (GS).

```
            270° (top)
              ·
            / | \
           /  |  \
          /   |   \
  180° ·─────┼─────· 0° (right)
          \   |   /
           \  |  /
            \ | /
              ·
            90° (bottom)
```

**Properties of the Glyph Space:**
- Bounded region for all constructions
- Center point = origin of reference
- Boundary = edge between symbol and context
- Angular reference: 0° at right, clockwise
- All positions, angles, scales are meaningful

The Glyph Space is itself a symbol: ○ = Totality / Completeness.

---

## II. THE 7 ATOMIC SYMBOLS

These cannot be decomposed further. They are the phonemes of UL.

### Ø — The Void
```
  ┌───────────┐
  │           │
  │  (empty)  │
  │           │
  └───────────┘
```
**Geometric:** Empty glyph space. No construction.  
**Semantic:** Silence. Absence. The ground from which distinction arises.  
**Set-theoretic:** ∅  
**Role:** Null element. Required by every formal system.

### • — The Point
```
  ┌───────────┐
  │           │
  │     •     │
  │           │
  └───────────┘
```
**Geometric:** Position without extension (Euclid Def. 1). Dimension 0.  
**Semantic:** EXISTENCE — "this is." The minimum act of meaning.  
**Symmetry:** SO(n) — looks the same from every direction.  
**Sort:** Entity (e)

### ─ — The Line
```
  ┌───────────┐
  │           │
  │  •───────•│
  │           │
  └───────────┘
```
**Geometric:** Length without breadth between two points (Euclid Def. 2–4). Dimension 1.  
**Semantic:** RELATION — "this connects to that."  
**Symmetry:** Translation along + reflection across midpoint.  
**Sort:** Relation (r)

**Directed variants:**

| Glyph | Name | Semantic |
|-------|------|----------|
| → | Ray (rightward) | Directed action: "this acts on that" |
| ← | Ray (leftward) | Received action: "this is acted upon" |
| ↑ | Ray (upward) | Ascent: "this rises toward that" |
| ↓ | Ray (downward) | Descent: "this falls toward that" |
| ↔ | Line (bidirectional) | Mutual relation: "these relate to each other" |

### ∠ — The Angle
```
  ┌───────────┐
  │     /     │
  │    / θ    │
  │   •───────│
  │           │
  └───────────┘
```
**Geometric:** Inclination of two lines meeting at a point (Euclid Def. 8). Hybrid dimension.  
**Semantic:** QUALITY — "this relates to that *in this way*."  
**Parameter:** θ ∈ [0°, 360°) — continuous quality spectrum.  
**Sort:** Modifier (m)

**Structurally distinguished angles:**

| Angle | Geometric property | Semantic value | Why forced |
|-------|--------------------|----------------|-----------|
| 0° | Coincidence/parallelism | Identity / Agreement | Zero inclination: unique |
| 60° | Equilateral triangle | Harmony / Balance | Minimal equal partition of π |
| 90° | Perpendicularity | Independence / Orthogonality | Maximal independence in Euclidean metric |
| 120° | Regular hexagon interior | Efficiency / Complementarity | Optimal packing angle |
| 180° | Opposition | Negation / Contradiction | Direction reversal: unique |
| 270° | Three-quarter turn | Reflexive return | 360° − 90° = returning through dependence |
| 360° | Full rotation | Completion / Cycle / Identity | Return to origin: unique |

**Continuous spectrum:** All other angles interpolate between these landmarks. There is no structurally forced meaning for 47° vs 48° — they represent adjacent points on a quality continuum, just as "warm" and "slightly warmer" do.

### ◠ — The Curve
```
  ┌───────────┐
  │    ╭──╮   │
  │   ╱    ╲  │
  │  •      • │
  │           │
  └───────────┘
```
**Geometric:** Continuous mapping from interval to space, not straight. Dimension 1 embedded in 2+.  
**Semantic:** PROCESS / TRANSFORMATION — "this becomes that through continuous change."  
**Parameters:** Curvature κ (rate of turning), torsion τ (twisting out of plane).  
**Sort:** Relation (r) with continuously varying direction

**Distinguished curves:**

| Curve | Definition | Semantic | Symbol |
|-------|-----------|----------|--------|
| Circle ○ | Constant κ, closed | Completion / Self-reference / Totality | ○ |
| Spiral 𝒮 | Monotonically increasing radius | Growth / Evolution / Development | 𝒮 |
| Parabola ⌒ | Decreasing κ with distance | Projection / Directed expansion | ⌒ |
| Sine wave ~ | Periodic variation | Rhythm / Oscillation / Alternation | ~ |
| Helix ⧖ | Constant κ + constant torsion | Progressive cycling / Time | ⧖ |

### ▮ — The Enclosure
```
  ┌───────────┐
  │  ┌─────┐  │
  │  │     │  │
  │  │  •  │  │
  │  │     │  │
  │  └─────┘  │
  └───────────┘
```
**Geometric:** Bounded subset of the plane with closed boundary. Dimension 2.  
**Semantic:** CONCEPT / CONTAINMENT — "this is bounded; this is defined."  
**Property:** Jordan Curve Theorem → inside vs. outside → categorization.  
**Sort:** Assertion (a) when complete sentence; Entity (e) when embedded.

**Enclosure types by symmetry group order:**

| Shape | Sides | Symmetry Group | Order | Concept Class |
|-------|-------|---------------|-------|---------------|
| △ Triangle | 3 | D₃ | 6 | Fundamental / Atomic / Irreducible |
| □ Square | 4 | D₄ | 8 | Structural / Systematic / Ordered |
| ⬠ Pentagon | 5 | D₅ | 10 | Organic / Self-similar / Living* |
| ⬡ Hexagon | 6 | D₆ | 12 | Efficient / Networked / Communal* |
| ○ Circle | ∞ | SO(2) | ∞ | Universal / Abstract / Complete |

*Note: The ordering by symmetry group is rigorous (Erlangen Program). Labels for pentagon and hexagon are conjectural conventions based on golden-ratio↔biology and optimal-packing↔networks associations. The circle and triangle labels are structurally forced (max symmetry = max generality; min polygon = min structure).*

---

## III. COMPOUND SYMBOL CONSTRUCTION

### Composition Operations

| Operation | Visual | Semantic Effect |
|-----------|--------|----------------|
| **Nesting** | Symbol inside enclosure | Concept-membership: "a [type] [thing]" |
| **Adjacent** | Symbols side by side | Sequential/compound: "[A]-[B]" |
| **Overlapping** | Partial intersection | Shared meaning: "[A] and [B] at once" |
| **Connection** | Line between symbols | Explicit relation: "[A] relates to [B]" |
| **Stacking** | Symbol atop symbol | Hierarchical modification: "the [meta] of [base]" |
| **Embedding** | Small symbol inside larger | Specification: "the specific [general]" |
| **Scaling** | Larger or smaller | Emphasis/diminution |
| **Rotation** | Turned by θ° | Perspective shift |
| **Reflection** | Mirrored | Inversion/negation |

---

## IV. FOUNDATIONAL CONCEPT SYMBOLS

### Existence & Non-Existence

| Concept | Construction | Reading |
|---------|-------------|---------|
| Something exists | • | "Is" |
| Nothing | (empty GS) | "Is not" |
| This specific thing | •₁ (subscripted/positioned point) | "This" |
| This and that exist | • • | "Two things are" |
| Identity | •═• (overlapping/coincident points) | "This is that" |
| Difference | • ∠180° • | "This opposes that" |

### Relations

| Concept | Construction | Reading |
|---------|-------------|---------|
| Unspecified relation | •──• | "Something relates to something" |
| Directed action | •──→• | "This acts on that" |
| Mutual relation | •←→• | "These relate to each other" |
| Causal chain | •→•→• | "This causes that causes that" |
| Mediation | •→•←• | "This thing mediates between those" |

### Qualities (Angles)

| Concept | Construction | Reading |
|---------|-------------|---------|
| Identity/Agreement | ∠0° between two relations | "Same as" |
| Harmony/Balance | ∠60° | "In balance with" |
| Independence | ∠90° | "Orthogonal to / independent of" |
| Complementarity | ∠120° | "Complementary to" |
| Opposition/Negation | ∠180° | "Opposite of" |
| Full cycle | ∠360° | "Returns to / completes" |

### Processes

| Concept | Construction | Reading |
|---------|-------------|---------|
| Change | ◠ | "Becomes" |
| Cyclic process | ○ (circle as process) | "Returns to origin through change" |
| Growth | 𝒮 (spiral outward) | "Develops / expands" |
| Decay | 𝒮⁻¹ (spiral inward) | "Diminishes / contracts" |
| Oscillation | ~ | "Alternates / vibrates" |
| Progression | ⧖ (helix) | "Cycles while advancing" — time |

### Concepts (Enclosures)

| Concept | Construction | Reading |
|---------|-------------|---------|
| A universal concept | ○{...} | "All / The complete [...]" |
| A fundamental thing | △{...} | "The irreducible [...]" |
| A structured system | □{...} | "The ordered [...]" |
| An organic thing | ⬠{...} | "The living [...]" |
| A network/community | ⬡{...} | "The connected [...]" |

---

## V. ABSTRACT CONCEPT INVENTORY

> **Note:** The constructions below are *illustrations* of how UL symbols compose into compound meanings — they demonstrate what the symbology makes possible. They are **NOT canonical definitions** and would be classified as **T3 (Conventional)** under the Lexicon's tier system — meaning their geometric construction is valid but the natural-language label is a grounding *decision*, not a geometric *fact*. For structurally motivated definitions with tier justifications, see the **Lexicon** (lexicon/lexicon.md §8.3). Multiple valid constructions can express similar meanings; see the **Thesaurus** for alternatives.

### Philosophical Concepts

| Concept | Construction | Derivation |
|---------|-------------|------------|
| **Truth** | ○{•} | Complete concept containing existence — "what IS, completely" |
| **Falsity** | ○̄{•} (reflected/inverted circle) | Negation of truth — reflected completeness |
| **Knowledge** | ○{○{•} ─∠60°─ ○} | Harmonious (60°) connection between self-aware concept and other concept |
| **Ignorance** | ○{• ─∠180°─ ○{}} | Opposition (180°) between existence and empty concept |
| **Freedom** | •↑ (point with unbounded upward ray) | Existence with unconstrained directed relation |
| **Constraint** | □{•} | Existence bounded by structure |
| **Possibility** | •╌╌• (dashed line) | Broken continuity = uncertain/hypothetical relation |
| **Necessity** | •━━• (bold/thick line) | Maximum visual weight = cannot be otherwise |
| **Infinity** | ○{○{○{...}}} | Self-nesting totality — completeness containing completeness |
| **Unity** | • (single point) | Irreducible oneness |
| **Duality** | •──∠180°──• | Two existences in opposition |
| **Trinity** | △{•,•,•} | Three existences in fundamental relation |

### Experiential Concepts

| Concept | Construction | Derivation |
|---------|-------------|------------|
| **Time** | ⧖ (helix) or ◠→ | Process with irreversible direction |
| **Space** | □{• • • •} | Structure containing multiple existences |
| **Causation** | •══→→• | Emphatic directed relation (double line, double arrow) |
| **Change** | ◠ within □ | Curvature within structure — structured process |
| **Growth** | ⬠{◠↑}⁺ | Living upward-curve, increasing |
| **Death** | ⬠ → Ø | Living thing directed to void |
| **Beauty** | ○{△ ~ ○} | Completeness containing harmony similar to completeness — self-similar harmony |
| **Love** | ○{• ≅ •} | Completeness containing two existences in identity — complete union |
| **Conflict** | • ∠180° • within □ | Two opposed existences within structure |
| **Harmony** | △ (equilateral triangle) | Three equal relations — perfect balance |

### Cognitive Concepts

| Concept | Construction | Derivation |
|---------|-------------|------------|
| **Thought** | ○{◠} | Complete concept containing process — internalized change |
| **Memory** | ○{•←} (concept with past-directed ray) | Concept pointing to prior existence |
| **Imagination** | ○{•╌╌→} (concept with dashed forward ray) | Concept with hypothetical future relation |
| **Understanding** | ○{○{•}─∠0°─○{•}} | Identity (0°) between two self-aware concepts |
| **Confusion** | ○{◠◠◠} (tangled curves) | Concept containing unresolved processes |
| **Attention** | •⁺ (scaled-up point) | Emphasized existence — this thing, specifically |
| **Abstraction** | ○ containing □ containing △ containing • | Nesting from universal to structural to fundamental to existence |
| **Analysis** | ○ → {△,△,△} | Whole decomposed into fundamental parts |
| **Synthesis** | {△,△,△} → ○ | Parts composed into whole |

---

## VI. QUANTIFIER SYMBOLS

Derived from geometric scaling operations within the Glyph Space. These are the geometric realization of the `quantify: m × e → a` operation from Σ_UL — the modifier (scaling factor) transforms the entity (point) into a quantified assertion:

| Quantifier | Construction | Derivation |
|-----------|-------------|------------|
| **All** (∀) | • scaled to fill GS boundary | Point expanded to match totality — universal |
| **Some** (∃) | • at non-center position, small | Point existing somewhere specific — existential |
| **None** (¬∃) | Reflected • + complement | Negated existence — nothing |
| **Exactly one** (∃!) | • at center, bold | Single emphasized existence at origin |
| **Most** (majority) | • scaled to >50% of GS | Point expanded to exceed half — majority |
| **Few** | • scaled to <25% of GS | Point shrunk below quarter — minority |

---

## VII. GRAMMATICAL MARKERS

| Function | Symbol/Operation | Derivation |
|----------|-----------------|------------|
| **Negation** | Reflection across vertical axis | Geometric reflection = inversion. negate(negate(x)) = x ✓ |
| **Past tense** | Translate glyph leftward | Position shift left = prior in sequence |
| **Future tense** | Translate glyph rightward | Position shift right = subsequent in sequence |
| **Present tense** | Glyph at center | Default position = current |
| **Question** | Rotate glyph 180° | Full reversal = uncertainty / inversion of assertion |
| **Emphasis** | Scale glyph up | Larger = more significant |
| **Diminution** | Scale glyph down | Smaller = less significant |
| **Plurality** | Translation copies (glyph repeated) | Pattern = multiple instances |
| **Possibility** | Dashed lines | Broken continuity = hypothetical |
| **Necessity** | Bold/thick lines | Maximum weight = cannot be otherwise |
| **Conditionality** | One enclosure inside another (A ⊂ B) | Containment = "if A then within B" |

---

## VIII. META-SYMBOLS (The Language Describing Itself)

| Meta-concept | Construction | Reading |
|-------------|-------------|---------|
| "Symbol" | ○{•,─,∠,◠,▮} | "The complete concept containing all primitives" |
| "Language" | ○{○{•,─,∠,◠,▮}} → ○{○{•,─,∠,◠,▮}} | "Complete concept of symbols interpreted as complete concept of symbols" (homomorphism) |
| "Grammar" | □{→,→,→} | "Structured arrangement of directed rules" |
| "Meaning" | ○{•} ─∠60°─ ○{•} | "Harmonious connection between two self-aware concepts" |
| "Universal" | ○ (unqualified circle) | Maximum symmetry = applies in all contexts |

---

## IX. SYMBOL COUNT SUMMARY

| Category | Count | Status |
|----------|-------|--------|
| Void | 1 | Complete |
| Atomic symbols | 6 (•, ─, ∠, ◠, ▮, ○) | Complete |
| Directed line variants | 5 | Complete |
| Distinguished angles | 7 | Complete (continuous spectrum between) |
| Distinguished curves | 5 | Complete (continuous family between) |
| Enclosure types | 5 | Complete (higher polygons possible) |
| Composition operations | 9 | Complete |
| Foundational concepts | ~20 | Open (extensible) |
| Abstract concepts | ~30 | Open (extensible) |
| Quantifiers | 6 | Complete |
| Grammatical markers | 11 | Complete |
| Meta-symbols | 5 | Open |

**Total defined symbols/constructions: ~105+**

The system is finitely axiomatized but infinitely generative — like arithmetic, where finitely many rules produce infinitely many numbers.
