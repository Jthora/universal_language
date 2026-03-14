# Glyph Composition Algebra

> The formal theory of how symbols combine to produce new symbols of increasing definition.  
> Composition is the engine of UL's infinite generativity — finitely many primitives, infinitely many meanings.

**TL;DR:** Symbols combine through exactly 5 spatial relationships (containment, intersection, adjacency, separation, connection) across unbounded composition depth. Each combination produces a new compound glyph with strictly more meaning than its parts. Layers, z-ordering, alignment, grouping, and scale all carry semantic weight. This document also critiques why existing digital text paradigms (fonts, SVG, Canvas) are structurally inadequate for UL, and specifies a three-layer digital architecture: **UL-Script** (ASCII keyboard-friendly structural encoding), **UL-GIR** (geometry-native intermediate representation where primitives ARE geometric objects), and a **rendering engine** that converts GIR to visual output.

**Prerequisites:** [Symbology](../symbology/symbol-map.md) (the atomic pieces), [Syntax](../syntax/syntax-dictionary.md) (the algebraic operations). **Reading time:** ~25 minutes.

### How This Document Relates to Its Siblings

| If you need... | Consult | Why |
|----------------|---------|-----|
| What the atomic symbols are | **Symbology** (symbol-map.md) | Symbology defines the pieces; this document defines how pieces compose visually |
| The algebraic rules for valid constructions | **Syntax** (syntax-dictionary.md) | Syntax gives the Σ_UL type system; this gives the geometric composition algebra |
| Why shapes carry their meanings | **Grammar** (grammar-book.md) | Grammar derives parts of speech from symmetry; composition preserves or breaks symmetry |
| Whether a compound is canonical | **Lexicon** (lexicon.md) | Lexicon assigns tiers (T1/T2/T3); compounds inherit or earn tiers through composition |
| Alternative compositions for the same meaning | **Thesaurus** (thesaurus.md) | Thesaurus maps equivalence classes; composition generates the class members |
| How to actually write and read | **Writer's Companion** (writers-companion.md) | Step-by-step pen-and-paper procedures for compound construction |

---

## 0. THE CENTRAL QUESTION

The Symbology defines 7 atomic symbols and 9 composition operations. The Syntax defines 11 algebraic operations with type signatures. These give you the *atoms* and the *rules* — but they don't answer:

> **When I draw one symbol on top of another, or next to another, or inside another — what exactly happens to the meaning? What are the complete rules? How deep can this go?**

This document answers that question.

---

## I. THE FIVE SPATIAL RELATIONSHIPS

When two marks exist on the same surface, they relate in exactly one of five topologically distinct ways. These five relationships are **exhaustive** — there is no sixth possibility for how two bounded regions in the Euclidean plane can relate.

### 1.1 Containment (A inside B)

```
  ┌───────────────┐
  │   ┌───────┐   │
  │   │       │   │
  │   │   A   │ B │
  │   │       │   │
  │   └───────┘   │
  └───────────────┘
```

**Topology:** A ⊂ B (proper subset)  
**Semantic:** A is a **member of**, **instance of**, or **specification of** B.  
**Σ_UL mapping:** `embed(a) → e` (assertion becomes entity within enclosure), or entity-nesting  
**Reading:** "A is inside the concept B" → "A is a kind of B" or "B contains A"

**Containment is the most powerful composition operation** — it creates hierarchies of meaning. A point inside a triangle inside a circle (• ∈ △ ∈ ○) reads as "a fundamental instance of a universal concept." Each nesting layer adds a new level of categorization.

### 1.2 Intersection (A overlaps B)

```
  ┌──────────┬─────────┐
  │          │         │
  │    A     │∩│  B    │
  │          │         │
  └──────────┴─────────┘
```

**Topology:** A ∩ B ≠ ∅, A ⊄ B, B ⊄ A (partial overlap)  
**Semantic:** A and B **share** properties or context. The overlap region IS the shared meaning.  
**Σ_UL mapping:** `conjoin(a₁, a₂)` — logical AND  
**Reading:** "A and B are both true; what they share is [the overlap]"

**The overlap region is itself a semantic object.** Marks that sit in the intersection zone belong to both A and B simultaneously. This is not just logical conjunction — it's meaning *blending*. The shared zone is a new emergent meaning that neither A nor B have alone.

### 1.3 Adjacency (A touches B)

```
  ┌──────────┐┌──────────┐
  │          ││          │
  │    A     ││    B     │
  │          ││          │
  └──────────┘└──────────┘
```

**Topology:** ∂A ∩ ∂B ≠ ∅, int(A) ∩ int(B) = ∅ (boundaries touch, interiors don't)  
**Semantic:** A and B are **alternatives** or **sequential**. Context determines which reading.  
**Σ_UL mapping:** `disjoin(a₁, a₂)` — logical OR; or sequential composition (left → right = temporal order)  
**Reading:** "A or B" (disjunction) or "A, then B" (sequence)

**Adjacency without overlap means the marks are related but independent.** They share a boundary point or line, signaling conceptual proximity, but maintain distinct interiors — distinct truths.

### 1.4 Separation (A apart from B)

```
  ┌──────────┐     ┌──────────┐
  │          │     │          │
  │    A     │     │    B     │
  │          │     │          │
  └──────────┘     └──────────┘
```

**Topology:** A ∩ B = ∅, ∂A ∩ ∂B = ∅ (completely disjoint)  
**Semantic:** A and B are **independent** or **unrelated** in the current context.  
**Σ_UL mapping:** No direct operation — independence is the *absence* of composition  
**Reading:** "A and B are separate" — the gap IS the meaning (independence)

**The size of the gap carries meaning.** Marks close together but not touching suggest potential relationship; marks far apart suggest deep independence. This is the continuous analog of adjacency — the distance is a parameter on the independence spectrum.

### 1.5 Connection (A linked to B by explicit mark)

```
  ┌──────────┐─────────┌──────────┐
  │          │ ══════→ │          │
  │    A     │         │    B     │
  │          │         │          │
  └──────────┘─────────└──────────┘
```

**Topology:** A ∩ B = ∅, but a mark (line/curve) has one endpoint in A and one in B  
**Semantic:** A and B have an **explicit relation**. The connecting mark IS the relation.  
**Σ_UL mapping:** `predicate(e₁, r, e₂)` — the fundamental assertion operation  
**Reading:** "A relates to B through [the connection type]"

**Connection is the only relationship that creates new structure between separated marks.** The properties of the connecting mark (straight vs. curved, directed vs. undirected, thin vs. bold, angled) all carry meaning according to the Symbology specifications.

### 1.6 The Exhaustiveness Theorem

> **Theorem (Spatial Exhaustiveness):** For any two bounded marks A, B on an oriented 2D surface satisfying the Jordan Curve Theorem, exactly one of the following holds:
> 1. A ⊂ B or B ⊂ A (containment)
> 2. A ∩ B ≠ ∅ ∧ A ⊄ B ∧ B ⊄ A (intersection)
> 3. ∂A ∩ ∂B ≠ ∅ ∧ int(A) ∩ int(B) = ∅ (adjacency)
> 4. A ∩ B = ∅ ∧ ∂A ∩ ∂B = ∅ (separation)
>
> Connection (5) is separation + an additional mark bridging A and B, therefore reducible to (4) plus a line primitive.

**Proof sketch:** By Jordan Curve Theorem, each closed mark partitions the plane into interior and exterior. Two such partitions interact in exactly the 4 configurations above. Connection is a composite: separation (two marks) plus a line (third mark). ∎

This means the 5 spatial relationships constitute the **complete basis** for visual composition. No other spatial relationship between marks is possible.

---

## II. COMPOSITION DEPTH (LAYERS)

### 2.1 The Layer Hierarchy

Every glyph has a **composition depth** — the number of composition operations applied to reach it from atomic primitives.

| Depth | Name | Description | Examples |
|-------|------|-------------|----------|
| **0** | Atomic | Primitive marks, no composition | •, ─, ∠, ◠, ▮, ○, Ø |
| **1** | Simple compound | One composition operation | △{•}, •→•, ∠60°, ○{◠} |
| **2** | Complex compound | Two nested operations | ○{△{•}}, [•→•] (framed predication), •→○{•} |
| **3** | Embedded structure | Contains at least one embedded assertion | [FACT[•→•] →•], ○{[•→•] ∠60° [•→•]} |
| **4** | Recursive compound | Self-similar or deeply nested | ○{○{○{•}}}, [FACT[FACT[•→•]→•]→•] |
| **N** | Arbitrary depth | N composition operations deep | Theoretically unlimited |

### 2.2 The Definition Principle

> **Principle (Monotonic Definition):** Composition depth strictly increases meaning-specificity. A depth-N compound is *more defined* than any of its depth-(N-1) substructures.

**Why this is true:** Each composition operation adds one of:
- A categorization (containment adds a concept-class)
- A conjunction (intersection adds shared truth)
- An alternative (adjacency adds a choice)
- A relation (connection adds a link)
- A transformation (modifier changes quality)

None of these can *remove* meaning — they can only constrain, qualify, or extend what was there. The compound is always at least as specific as any of its parts.

**Consequence:** You can always make a symbol more precise by composing it further. There is no "maximum definition" — just as you can always add more decimal places to a number.

### 2.3 Layer Reading Order

When reading a multi-layer glyph, process **outside-in**:

```
READING ORDER:
  Layer 0 (outermost frame)     → "This is an assertion about..."
    Layer 1 (main enclosures)   → "...these types of concepts..."
      Layer 2 (inner structure) → "...specifically, these things related in this way..."
        Layer 3+ (embedded)     → "...where [embedded fact]..."
```

This mirrors the 5-pass reading procedure from the Writer's Companion, but generalizes it to arbitrary depth. At each layer, you apply the same 5-step process (enclosures → connections → angles → points → curvature).

---

## III. COMPOSITION PARAMETERS

Beyond the 5 spatial relationships and composition depth, several continuous and discrete parameters affect how composed glyphs are read.

### 3.1 Relative Scale

When A and B compose, their relative sizes carry meaning:

| Scale relationship | Meaning | Example |
|-------------------|---------|---------|
| A >> B (A much larger) | A is the primary concept; B is a detail/modifier | Large circle with small inner point: a concept with a minor instance |
| A ≈ B (similar size) | A and B are co-equal participants | Two similar frames overlapping: balanced conjunction |
| A << B (A much smaller) | A is subordinate to B; A is embedded within B's context | Tiny embedded frame inside large frame: subordinate clause |

**Scale ratio is continuous.** There are no discrete breakpoints — the relative size interpolates smoothly between "subordinate" and "dominant," just as angles interpolate between distinguished values.

### 3.2 Relative Orientation (Rotation)

The angular relationship between composed marks:

| Orientation | Meaning |
|-------------|---------|
| Aligned (0°) | Agreement, identity, parallelism |
| Perpendicular (90°) | Independence, orthogonality |
| Opposed (180°) | Contradiction, contrast |
| Rotated by θ | Quality along the continuous spectrum |

**Orientation preserves or breaks symmetry.** Two identical triangles composed at 0° (aligned) have the combined symmetry of a single triangle. Two triangles at 60° form a Star of David — a new emergent symmetry (D₆) that neither had alone. This is **symmetry composition**: the resulting symmetry group depends on how component symmetries interact.

### 3.3 Z-Ordering (Visual Depth)

When marks overlap, **which is visually "on top"** carries meaning:

| Z-order | Interpretation |
|---------|---------------|
| A on top of B | A is the **foreground** concept — more salient, more recent, or more active |
| B on top of A | B is the foreground concept |
| Unclear/equal | Both concepts are equally present — true blending |

**Z-ordering convention:** In handwritten UL, z-ordering is determined by **drawing order** — what you draw last is on top. This is a natural convention: the most recently added meaning is visually dominant. In digital UL, z-ordering must be explicitly specified.

**Practical rule:** If z-ordering matters to your meaning, make it visually unambiguous. If it doesn't matter (true blend), draw the marks so neither clearly occludes the other.

### 3.4 Alignment

Where composed marks sit relative to each other's centers:

| Alignment | Meaning |
|-----------|---------|
| Center-aligned | Direct, symmetric relationship — the composition is "about" the center |
| Offset vertically | Hierarchical relationship — upper = prior/cause, lower = posterior/effect |
| Offset horizontally | Temporal or sequential — left = before, right = after |
| Offset diagonally | Both hierarchical and temporal — complex relationship |

### 3.5 Grouping

**Grouping determines what composes with what.** An enclosure boundary marks a group: everything inside it composes first (forms a unit), then the unit composes with what's outside.

```
  Without grouping:        With grouping:
  • → • → •               •→ ○{•→•}
  (A acts on B acts on C)  (A acts on [the concept of B-acting-on-C])
```

**Grouping is the parenthesization of visual composition.** Enclosures ARE parentheses — they determine evaluation order. Without enclosures, marks compose left-to-right and outside-in by default.

---

## IV. COMPOUND GLYPH TAXONOMY

Compound glyphs can be classified by their composition structure:

### 4.1 By Spatial Relationship Used

| Type | Primary operation | Result |
|------|------------------|--------|
| **Nested glyph** | Containment | Category hierarchy: • ∈ △ ∈ ○ |
| **Blended glyph** | Intersection | Meaning fusion: A∩B = new emergent concept |
| **Sequential glyph** | Adjacency | Compound word or disjunction: A│B |
| **Linked glyph** | Connection | Relational structure: A—r→B |
| **Constellation glyph** | Separation | Field of independent concepts: A   B   C |

### 4.2 By Composition Depth

| Type | Depth | Linguistic analog |
|------|-------|-------------------|
| **Root** | 0 | Phoneme or morpheme |
| **Stem** | 1 | Root word |
| **Word** | 2-3 | Compound word or phrase |
| **Clause** | 3-4 | Dependent/independent clause |
| **Sentence** | 4-5 | Complete assertion |
| **Paragraph** | 5-7 | Discourse unit |
| **Document** | 7+ | Extended argument |

### 4.3 By Symmetry Behavior

Composition can **preserve**, **increase**, or **break** the symmetry of component glyphs:

| Behavior | How it happens | Result |
|----------|---------------|--------|
| **Symmetry-preserving** | Components aligned along symmetry axes | Combined glyph inherits parent symmetry |
| **Symmetry-increasing** | Components arranged to create new symmetry | Combined glyph has MORE symmetry than parts (e.g., two triangles → hexagram) |
| **Symmetry-breaking** | Components misaligned or asymmetrically placed | Combined glyph has LESS symmetry — more specific/concrete |

**The Grammar's principle applies:** Higher symmetry = more abstract/general. Lower symmetry = more specific/concrete. So symmetry-breaking composition = increasing definition (making meaning more specific), while symmetry-increasing composition = increasing generality (making meaning more abstract).

---

## V. THE COMPOSITION ALGEBRA

### 5.1 Formal Definition

The **Glyph Composition Algebra** (GCA) is a tuple:

```
GCA = (M, ⊕, ⊗, ⊙, ‖, —, ε)

where:
  M       = set of all marks (glyphs) on the glyph space GS
  ⊕       = containment operation:  A ⊕ B = "B inside A"
  ⊗       = intersection operation: A ⊗ B = "A overlapping B"
  ⊙       = adjacency operation:    A ⊙ B = "A touching B"
  ‖       = separation operation:   A ‖ B = "A apart from B"
  —(r)    = connection operation:    A —r— B = "A related to B by r"
  ε       = empty mark (Void)
```

### 5.2 Properties

| Property | Holds for ⊕ | Holds for ⊗ | Holds for ⊙ | Holds for ‖ | Holds for — |
|----------|-------------|-------------|-------------|-------------|-------------|
| Associative | Yes: (A⊕B)⊕C = A⊕(B⊕C) | Yes: (A⊗B)⊗C = A⊗(B⊗C) | Yes: (A⊙B)⊙C = A⊙(B⊙C) | Yes | Yes (compose) |
| Commutative | **No:** A⊕B ≠ B⊕A | Yes: A⊗B = B⊗A | **No:** order = sequence | Yes | **No:** A→B ≠ B→A |
| Identity | ε ⊕ A = A (void inside anything = just the thing) | No | No | No | No |
| Idempotent | **No:** A⊕A ≠ A (self-nesting ≠ self) | Yes: A⊗A = A | Yes: A⊙A = A | Yes: A‖A = A | **No** |
| Depth-increasing | **Yes:** always +1 | Sometimes | Sometimes | No | Sometimes |

**Key insight: Containment (⊕) is non-commutative.** "A inside B" ≠ "B inside A" — which is the container and which is the contained matters enormously. This asymmetry is what gives nesting its power to create hierarchies.

### 5.3 Interaction Laws

The five operations interact:

```
1. CONTAINMENT ABSORBS CONNECTION:
   If A ⊕ B and B —r— C, then A ⊕ (B —r— C)
   = "A contains [B related to C]"
   The connection is internalized within the container.

2. INTERSECTION DISTRIBUTES OVER ADJACENCY:
   A ⊗ (B ⊙ C) = (A ⊗ B) ⊙ (A ⊗ C)
   "A overlapping [B next to C]" = "[A overlapping B] next to [A overlapping C]"

3. CONTAINMENT IS TRANSITIVE:
   If A ⊕ B and B ⊕ C, then A ⊕ C
   "C inside B inside A" → "C is inside A" (nesting chains)

4. SEPARATION IS BROKEN BY CONNECTION:
   If A ‖ B and we add A —r— B, then A is no longer separated from B
   Connection overrides separation.

5. ADJACENCY PROMOTES TO INTERSECTION:
   If A ⊙ B and we thicken the shared boundary, A ⊗ B
   Adjacency is the limiting case of intersection as overlap → 0.
```

---

## VI. CREATING NEW SYMBOLS OF INCREASED DEFINITION

### 6.1 The Definition Ladder

Every new symbol is constructed by climbing the composition ladder:

```
Level 0: •                              "something exists"
Level 1: ○{•}                           "a complete concept of existence"
Level 2: ○{• →@60° •}                  "a concept of two things in harmony"
Level 3: □{○{• →@60° •}}              "a structured system containing harmonious concepts"
Level 4: ⬡{□{○{• →@60° •}}, □{○{...}}} "a network of structured harmonious systems"
```

Each level increases definition — you can read each level as: 
- Level 0: "something"
- Level 1: "a whole something"
- Level 2: "a whole harmonious pair"
- Level 3: "an ordered system of harmonious pairs"
- Level 4: "a community of ordered systems of harmonious pairs"

### 6.2 Three Methods for Increasing Definition

**Method 1: Deepening (Vertical Composition)**
Add a nesting layer. This categorizes or specifies the existing symbol.

```
•           → △{•}         → □{△{•}}        → ○{□{△{•}}}
"exists"    → "fundamental  → "structurally   → "universally
               existence"     organized         organized
                              fundamental       structural
                              existence"        fundamental
                                                existence"
```

**Method 2: Broadening (Horizontal Composition)**
Add adjacent or connected marks at the same depth level. This relates the symbol to other concepts.

```
•          → • → •        → • → • ← •      → • ↔ • ↔ •
"exists"   → "causes       → "two things     → "three things
              another"       act on a          in mutual
                             mediator"         relation"
```

**Method 3: Blending (Overlap Composition)**
Intersect with another symbol. This creates emergent meaning in the overlap zone.

```
△          → △ ⊗ ○        → △ ⊗ ○ ⊗ □
"fundamental" → "that which    → "that which
                is both          is fundamental,
                fundamental      universal, AND
                and universal"   structural"
```

### 6.3 Compound Symbol Stability

Not all compound glyphs are equally useful. Some compositions stabilize into recognizable "words" — glyphs that a community agrees to read as a unit rather than decomposing each time.

**Stability criteria:**
1. **Geometric naturalness:** Does the compound have higher symmetry than expected? (If yes, it's a natural "attractor" in composition space)
2. **Semantic utility:** Does the compound express a concept that's frequently needed?
3. **Decomposition transparency:** Can a reader immediately see the composition structure?
4. **Tier classification:** T1 compounds (geometrically forced) are the most stable; T3 (conventional) are the least.

**Example of high stability:** ○{•} (Truth) — circle containing point. This is stable because:
- Geometrically natural (maximum symmetry enclosing minimum structure)
- Semantically essential (truth is a fundamental concept in any system)
- Transparently decomposable (one enclosure, one point)
- T2 tier (structurally distinguished by maximality)

**Example of low stability:** ⬡{□{↔,↔,↔},{•,•,•}} (Democracy) — complex multi-level compound. Less stable because:
- Lower symmetry than components separately
- Culturally specific (not universally needed)
- Multi-step decomposition required
- T3 tier (conventional grounding decision)

---

## VII. BEYOND LAYERS: WHAT ELSE MATTERS

### 7.1 Semantic Dimensions of Composition

When two glyphs compose, the result is determined not just by *which* spatial relationship they have, but by multiple simultaneous semantic dimensions:

| Dimension | What it encodes | How to control it |
|-----------|----------------|-------------------|
| **Spatial relationship** | Logical connective (AND/OR/containment/relation) | Choose containment, intersection, adjacency, separation, or connection |
| **Depth** | Specificity / abstraction level | Add or remove nesting layers |
| **Scale ratio** | Dominance / subordination | Make one component visually larger or smaller |
| **Orientation** | Quality of relationship | Rotate components relative to each other |
| **Z-order** | Salience / recency | Draw the dominant component last (on top) |
| **Alignment** | Structural relationship | Center, offset vertically, offset horizontally |
| **Connection properties** | Relation type | Straight/curved, directed/undirected, thin/bold, solid/dashed |
| **Boundary properties** | Concept class | Triangle/square/pentagon/hexagon/circle (for enclosures) |
| **Transformation** | Derived meaning | Scale/rotate/reflect the compound after composition |

### 7.2 The Full Composition Decision Space

When composing glyph A with glyph B, the writer makes (at minimum) these decisions:

```
COMPOSITION DECISION TREE:

1. SPATIAL RELATIONSHIP: How do A and B relate?
   ├── Containment: A inside B (or B inside A?)
   ├── Intersection: A overlapping B
   ├── Adjacency: A touching B (left-right? top-bottom?)
   ├── Separation: A apart from B (how far?)
   └── Connection: A linked to B (by what type of mark?)

2. SCALE: How big is A relative to B?
   ├── A >> B (A dominates)
   ├── A ≈ B (co-equal)
   └── A << B (B dominates)

3. ORIENTATION: What angle between A and B?
   ├── 0° (aligned, agreeing)
   ├── 60° (harmonious)
   ├── 90° (independent)
   ├── 120° (complementary)
   ├── 180° (opposing)
   └── θ° (continuous quality)

4. Z-ORDER: When overlapping, which is in front?
   ├── A in front (A more salient)
   ├── B in front (B more salient)
   └── Equal (true blend)

5. ALIGNMENT: Where is A positioned relative to B's center?
   ├── Centered (symmetric composition)
   ├── Offset up/down (hierarchical)
   ├── Offset left/right (sequential)
   └── Offset diagonal (both)
```

This is a **5-dimensional composition space** with both discrete and continuous parameters. The richness of this space is why UL is infinitely generative — even with only 7 atomic symbols, the composition space produces unbounded variety.

---

## VIII. INTERSECTION vs. ADJACENCY: THE CRITICAL DISTINCTION

The user asks: "Do symbols have to be intersecting or can they just be next to each other?"

**Both are valid, and they mean different things:**

### 8.1 When to Intersect (Overlap)

Use intersection when:
- Both meanings are **simultaneously true** of the same thing (logical AND)
- You want to create an **emergent meaning** in the overlap zone that neither component has alone
- The concepts are **inseparable** in the context you're expressing

```
  △ ⊗ ○ = "the fundamental-universal" — not just fundamental, not just universal,
           but the specific thing that is BOTH AT ONCE

  The overlap zone (△ ∩ ○) is the new meaning:
  "that which is both irreducible AND complete"
```

### 8.2 When to Be Adjacent (Touching)

Use adjacency when:
- The meanings are **alternatives** (logical OR)
- You're expressing a **sequence** (temporal or logical order)
- The concepts are **related but distinct** — maintaining their own identity

```
  △ ⊙ ○ = "fundamental OR universal" — one or the other (or both),
           but maintaining their distinctness

  Or with left-right reading:
  △ ⊙ ○ = "first fundamental, then universal" — a progression
```

### 8.3 When to Separate (Gap)

Use separation when:
- The concepts are **independent** — no direct logical connection
- You want to show **conceptual distance** (gap size = relationship distance)
- The marks are part of a **constellation** — a field of related but autonomous concepts

```
  △     ○ = "fundamental ... universal" — both exist, but separately.
           They share a page but not a relationship.
```

### 8.4 Visual Summary

```
  INTERSECTING:         ADJACENT:            SEPARATED:
  ┌────╱────┐           ┌──────┐┌──────┐    ┌──────┐  ┌──────┐
  │  A ╳ B  │           │  A   ││  B   │    │  A   │  │  B   │
  │   ╲╱    │           │      ││      │    │      │  │      │
  └─────────┘           └──────┘└──────┘    └──────┘  └──────┘
  "A AND B"             "A OR B" / "A, B"   "A.  B."
  (blended identity)    (distinct but near)  (independent)
```

---

## IX. MULTI-GLYPH COMPOSITION PATTERNS

### 9.1 Ternary and Higher Composition

The current system only formally defines binary operations (two operands). But three or more glyphs often need to compose simultaneously. The rules:

**Ternary containment:** A ⊕ B ⊕ C = deeply nested
```
  ○{□{△{•}}} = "a universal concept of a structural system of a fundamental existence"
  Reading: outside-in, each layer categorizes the next
```

**Ternary intersection:** A ⊗ B ⊗ C = three-way blend
```
  △ ⊗ □ ⊗ ○ = "that which is fundamental AND structural AND universal"
  The triple-overlap zone is the emergent meaning
```

**Ternary adjacency:** A ⊙ B ⊙ C = sequence or multiple alternatives
```
  △ ⊙ □ ⊙ ○ = "fundamental, then structural, then universal" (sequence)
              or "fundamental OR structural OR universal" (alternatives)
```

**Mixed composition:** Different relationships for different pairs
```
  (A ⊕ B) ⊙ (C ⊕ D) = "[B-inside-A] next to [D-inside-C]"
  Two nested units side by side — adjacency of containments
```

### 9.2 Radial Composition

Instead of linear (left-to-right), marks can compose **radially** around a center:

```
       ○
      / \
     △   □
      \ /
       ⬡

  Four concepts arranged around a center, connected radially.
  This is NOT sequential — it's a simultaneous field.
  Reading: identify the center first, then the periphery.
```

### 9.3 Fractal / Self-Similar Composition

A glyph can contain a smaller copy of itself:

```
  ○{○{○{•}}}     Self-reference / Recursion / Infinity

  △{•, △{•, △{•}}}   Self-similar hierarchy — fractal fundamental structure
```

**Self-similar composition is the geometric basis of recursion.** The fixed-point ○∞ = ○{○{○{...}}} is the limit of infinite self-nesting — a concept that IS its own content.

### 9.4 Tabular Composition

Marks arranged in a regular grid:

```
  • • • •
  • • • •
  • • • •

  A grid of entities — "many things, ordered in rows and columns"
  The grid structure itself carries meaning: regularity, repetition, pattern.
```

---

## X. TOWARD A RENDERING SYSTEM

### 10.1 Architecture Overview

A UL rendering system has three components:

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│  INPUT          PROCESSING          OUTPUT              │
│  ─────          ──────────          ──────              │
│                                                         │
│  Natural     ┌──────────────┐                           │
│  Language ──→│   NL→UL      │                           │
│  (English,   │  Decomposer  │                           │
│   etc.)      └──────┬───────┘                           │
│                     │                                   │
│                     ▼                                   │
│  CSF Text   ┌──────────────┐                           │
│  ─────────→ │  Structural  │──→ JSON Structural        │
│             │   Parser     │    Encoding               │
│  JSON       └──────┬───────┘                           │
│  ─────────→        │                                   │
│                     ▼                                   │
│              ┌──────────────┐                           │
│              │   Geometric  │──→ SVG / Canvas /         │
│              │   Renderer   │    PNG / PDF              │
│              └──────────────┘                           │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 10.2 Component 1: NL→UL Decomposer

**Purpose:** Take natural language input and produce a UL structural encoding.

**Pipeline:**

```
INPUT: "Knowledge is structured truth."

Step 1: SEMANTIC PARSE
  → Identify entities: "knowledge", "truth"
  → Identify relations: "is" (identity)
  → Identify modifiers: "structured"

Step 2: UL SORT ASSIGNMENT
  → knowledge = Entity (e₁)
  → truth = Entity (e₂): ○{•} (Lexicon T2)
  → structured = Modifier (m): □-ness (abstract(□) → quality of being structured)
  → is = Relation (r): identity at 0°

Step 3: OPERATION SELECTION
  → "structured truth" = modify_entity(m_structured, e_truth) = □{○{•}}
  → "knowledge is structured truth" = predicate(e_knowledge, r_0°, □{○{•}})

Step 4: STRUCTURAL ENCODING OUTPUT
  → JSON tree representing the full composition
```

**Implementation approach:** This is fundamentally a **semantic parsing** task. Modern LLMs can perform Steps 1-3 with high accuracy given the UL specification as context. The system would:
1. Use the Lexicon as a lookup table for canonical encodings (T1/T2 entries)
2. Use the Grammar's decomposition algorithm for novel concepts
3. Use the Syntax's type system to validate sort assignments
4. Output the JSON structural encoding defined in writing-system.md §8.2

### 10.3 Component 2: Structural Parser

**Purpose:** Convert between CSF text notation and JSON structural encoding (bidirectional).

**This is a standard parser/serializer.** The BNF grammar for CSF is defined in writing-system.md §8.3. Implementation is straightforward compiler-construction work:
- CSF → JSON: Parse CSF using the BNF grammar, produce AST, serialize to JSON
- JSON → CSF: Walk the JSON tree, emit CSF tokens

### 10.4 Component 3: Geometric Renderer

**Purpose:** Take a JSON structural encoding and produce a visual glyph.

**This is the hard part.** It requires solving several sub-problems:

**Sub-problem A: Layout**
Given a structural tree, determine where each mark goes in 2D space.

```
Layout algorithm:

1. ALLOCATE GLYPH SPACE: Start with unit circle (or rectangle for documents)

2. PROCESS OUTSIDE-IN:
   a. Outermost frame → boundary of glyph space
   b. Entity enclosures → subdivide interior
      - Containment: center child, scale to fit inside parent
      - Adjacency: place children side-by-side
      - Intersection: offset children so they overlap by ~30%
      - Separation: place children with gap proportional to independence

3. PLACE CONNECTIONS:
   - Straight lines: direct path between entity centers
   - Curved lines: arc between entities (curvature determines path)
   - Directed lines: add arrowhead at endpoint

4. ADD MODIFIERS:
   - Scale transformations: resize the target glyph
   - Rotations: rotate the target glyph in place
   - Reflections: mirror the target glyph

5. ADJUST FOR READABILITY:
   - Separate overlapping text labels
   - Ensure minimum mark size for visibility
   - Scale nesting depth to available resolution
```

**Sub-problem B: Stroke Rendering**
Each mark must be drawn with appropriate visual properties:

| Mark type | Rendering |
|-----------|-----------|
| Point (•) | Filled circle, radius proportional to emphasis |
| Line (─) | Straight stroke connecting two coordinates |
| Directed line (→) | Line + arrowhead at endpoint |
| Angle (∠) | Two lines meeting at a point, with arc indicator |
| Curve (◠) | Cubic Bézier or arc between two points, curvature as parameter |
| Enclosure (▮) | Closed polygon or circle centered on contents |
| Frame ([ ]) | Rectangular boundary around assertion |

**Sub-problem C: Nesting Recursion**
For each level of nesting, recursively apply the layout algorithm at reduced scale. The renderer must handle:
- Smooth scale reduction per level
- Minimum readable size (below which: render as a filled dot or abbreviation)
- Maximum practical depth (display limit, not theoretical limit)

### 10.5 Rendering Output Formats

| Format | Use case | Properties |
|--------|----------|------------|
| **SVG** | Web display, scalable prints | Vector, infinite scalability, CSS stylable, DOM-accessible |
| **Canvas/WebGL** | Interactive applications | Rasterized, fast, supports animation and interaction |
| **PNG/PDF** | Static documents, archival | Fixed resolution (PNG) or vector (PDF) |
| **ASCII art** | Text-only environments, terminal | Lossy but universal — the CSF text format serves this role |

### 10.6 Interactive Features

A rendering system should support:

| Feature | Purpose |
|---------|---------|
| **Zoom into nesting levels** | Navigate composition depth interactively |
| **Hover for decomposition** | Show the Σ_UL operation tree on mouseover |
| **Click to edit** | Modify the structural encoding, re-render in real time |
| **Animation of composition** | Show how a compound glyph is built step-by-step |
| **Bidirectional editing** | Edit the visual → update the JSON; edit the JSON → update the visual |

---

## XI. NATURAL LANGUAGE → UWS TRANSLATION

### 11.1 The Translation Pipeline

```
INPUT: English sentence (or any natural language)
  │
  ▼
┌─────────────────────────────────────┐
│ 1. SYNTACTIC PARSE                  │
│    Identify: nouns, verbs, adj,     │
│    adverbs, connectives, scope      │
└────────────────┬────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────┐
│ 2. SEMANTIC DECOMPOSITION           │
│    Map NL categories to UL sorts:   │
│    noun → Entity (e)                │
│    verb → Relation (r)              │
│    adj/adv → Modifier (m)          │
│    clause → Assertion (a)           │
│    "the fact that" → embed(a)       │
└────────────────┬────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────┐
│ 3. OPERATION SELECTION              │
│    Walk the Writer's Companion      │
│    decision tree (§5.1):            │
│    - What type of statement?        │
│    - What type of relation?         │
│    - What modifiers?                │
│    - Any embedding/abstraction?     │
└────────────────┬────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────┐
│ 4. LEXICON LOOKUP                   │
│    Check if any sub-expression      │
│    matches a canonical entry        │
│    (T1/T2 from the Lexicon).        │
│    Use canonical form if available. │
└────────────────┬────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────┐
│ 5. STRUCTURAL ENCODING              │
│    Produce JSON tree per            │
│    writing-system.md §8.2           │
└────────────────┬────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────┐
│ 6. COMPOSITION & LAYOUT             │
│    Apply GCA rules to determine     │
│    spatial arrangement of all       │
│    marks in the glyph space.        │
└────────────────┬────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────┐
│ 7. RENDER                           │
│    Geometric Renderer → SVG/image   │
└─────────────────────────────────────┘

OUTPUT: Visual UWS glyph
```

### 11.2 Translation Examples

**Example A: "The cat sat on the mat."**

```
Step 1: Parse
  Subject: "the cat" (noun phrase)
  Verb: "sat on" (prepositional verb)
  Object: "the mat" (noun phrase)
  Tense: past

Step 2: Decompose
  e₁ = cat (entity — a specific existence)
  e₂ = mat (entity — a specific existence)
  r  = sat-on (relation — directed spatial relation)
  tense = past (modifier — temporal translation leftward)

Step 3: Operations
  predicate(e₁, r, e₂) → a
  Then translate leftward for past tense

Step 4: Lexicon lookup
  "cat" → not canonical (T3/grounding decision) → labeled •₁
  "mat" → not canonical (T3) → labeled •₂
  "sat on" → directed spatial relation → →

Step 5: Structural encoding
  {
    "type": "assertion",
    "tense": "past",
    "contents": {
      "type": "predicate",
      "subject": {"type": "entity", "label": "cat"},
      "relation": {"type": "relation", "direction": "forward", "quality": "spatial-contact"},
      "object": {"type": "entity", "label": "mat"}
    }
  }

Step 6-7: Render
  ┌─────────────────────┐
  │  •₁ ──→── •₂       │  ← (shifted left = past tense)
  │ (cat)    (mat)      │
  └─────────────────────┘
```

**Example B: "If democracy thrives, then freedom endures."**

```
Step 1: Parse
  Conditional: "if... then..."
  Clause A: "democracy thrives" (premise)
  Clause B: "freedom endures" (conclusion)

Step 2: Decompose
  a₁ = "democracy thrives" — assertion
  a₂ = "freedom endures" — assertion
  Structure: implication (if a₁ then a₂)

Step 3: Operations
  Inner a₁: predicate(e_democracy, r_thrive, e_state)
    e_democracy = ⬡{□{↔},{•,•,•}} (Lexicon §7.3 — compound)
    r_thrive = upward spiral 𝒮
  Inner a₂: predicate(e_freedom, r_endure, e_state)
    e_freedom = •↑ (point with unbounded upward ray)
    r_endure = bold horizontal line (necessity + persistence)
  Combine: embed(a₁) as premise, connect to a₂
  predicate(embed(a₁), r_causal, embed(a₂))

Step 5-7: Render (simplified)
  ┌──────────────────────────────────────────────┐
  │   ┌───────────────┐                          │
  │   │ ⬡{□{↔},{•••}} │                          │
  │   │    𝒮          │  ══→══  [•↑ ━━━━ •]     │
  │   └───────────────┘         (freedom endures) │
  │   (democracy thrives)                         │
  └──────────────────────────────────────────────┘
```

### 11.3 Implementation Strategy

The most practical implementation leverages **LLMs as the decomposition engine** and **a deterministic renderer** for the visual output:

```
ARCHITECTURE:

  User Input (English)
       │
       ▼
  ┌─────────────┐     ┌─────────────────────────────────┐
  │  LLM with   │     │ System prompt includes:          │
  │  UL Context  │◄────│ - Σ_UL specification             │
  │             │     │ - Lexicon (42 canonical entries)  │
  │             │     │ - GCA composition rules           │
  │             │     │ - JSON schema for output          │
  └──────┬──────┘     └─────────────────────────────────┘
         │
         ▼ (JSON structural encoding)
  ┌──────────────┐
  │  Deterministic│
  │  Renderer     │──→ SVG Output
  │  (code)       │
  └──────────────┘
```

**Why this split?** The decomposition step (NL → UL sorts → operations) requires semantic understanding — which LLMs excel at. The rendering step (JSON → SVG) is purely geometric — which deterministic code excels at. Combining both gives reliable, reproducible visual output from arbitrary natural language input.

---

## XII. CRITIQUE: THE DIGITAL REPRESENTATION PROBLEM

### 12.1 The Fundamental Mismatch

Everything in Sections I–XI describes a **2D geometric algebra**. Computers, however, are built on a fundamentally different substrate:

| What UL needs | What computers provide |
|---------------|----------------------|
| Continuous 2D plane with arbitrary mark placement | Discrete 1D character streams (text) or pixel grids (raster) |
| Marks defined by geometric properties (curvature, symmetry group, incidence) | Characters defined by Unicode codepoints; shapes defined by Bézier control points |
| Composition via spatial relationships (containment, intersection, adjacency) | Composition via concatenation (left-to-right, top-to-bottom) |
| Meaning in the *relationships between* marks | Meaning in the *identity of* individual characters |
| Scale, rotation, z-order all semantic | Scale = font size (cosmetic), rotation = unsupported, z-order = unsupported |
| Infinite nesting depth | Finite pixel resolution; finite font glyph tables |

**The mismatch is not cosmetic — it's structural.** Every existing digital text technology (Unicode, OpenType, TrueType, WOFF, HTML/CSS, PDF, even SVG) assumes that writing is a 1D stream of discrete glyphs laid out in rows. UL is a 2D continuous geometric field where spatial relationships ARE the grammar.

### 12.2 Why Fonts Cannot Work

A font maps character codes to glyph outlines. This fails for UL because:

1. **Glyphs are not atomic.** A UL "glyph" is a compound construction that can contain other glyphs recursively. There is no finite glyph table for UL — the symbols are generated, not enumerated.

2. **Position is semantic.** In Latin text, "ab" and "ba" differ by character order. In UL, the *spatial position* of A relative to B within a 2D frame changes the meaning (containment vs. adjacency vs. overlap). Fonts have no concept of "this character goes inside that character."

3. **Overlap is meaningful.** When two UL marks partially overlap, the overlap zone IS a new semantic object. Font rendering treats overlap as a visual defect (kerning collision), not as semantic content.

4. **Scale is meaningful.** A large circle containing a small point means something different from two equal-sized marks. Fonts render all glyphs at the same point size.

5. **Angle is meaningful.** The rotation of one mark relative to another encodes quality (0° = identity, 90° = independence, 180° = opposition). Fonts do not support per-character rotation, and even CSS `transform: rotate()` treats it as cosmetic, not semantic.

6. **Ligatures are inadequate.** OpenType ligatures (fi → ﬁ) can combine adjacent characters, but they're pre-baked substitutions from a finite table — not generative geometric composition. You'd need an infinite ligature table to cover all possible UL compositions.

**Summary:** Fonts are **lookup tables**. UL composition is **generative algebra**. A lookup table cannot represent a generative system.

### 12.3 Why SVG/Canvas Are Insufficient

SVG and HTML Canvas are closer — they can draw arbitrary shapes in 2D. But they still have fundamental problems:

**SVG problems:**
- Shapes are defined by coordinate lists and path commands, not by geometric properties. A circle is `<circle cx="50" cy="50" r="40"/>` — there's no concept of "this is an SO(2)-symmetric enclosure containing an entity."
- Spatial relationships (containment, intersection) must be computed by the renderer, not declared in the format. SVG has no native way to say "A contains B" — you position A and B such that one *happens to be* inside the other.
- No semantic layer. An SVG file full of paths is as opaque to a parser as a JPEG — without external metadata, you can't recover the UL structure.
- Composition is manual. Every pixel-position, curve control point, and stroke width is specified absolutely. There's no "place this triangle inside that circle in harmonious relationship."

**Canvas problems:**
- Immediate-mode rendering — draw commands execute and produce pixels, with no retained scene graph. You can't query "what is inside this enclosure?" after drawing.
- Even less semantic than SVG — it's pure pixel manipulation.

**Both SVG and Canvas are *output formats*, not *representation formats*.** They can render a UL glyph, but they cannot encode one. They're the paper, not the language.

### 12.4 The Three-Layer Solution

UL actually needs three distinct digital layers, each solving a different problem:

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│  LAYER 1: KEYBOARD SCRIPT                                   │
│  ─────────────────────                                      │
│  ASCII-friendly linear encoding for TYPING and TRANSMISSION │
│  Human-writable, machine-parseable, semantically lossless   │
│  Analogy: LaTeX source code, Lilypond notation              │
│                                                             │
│  "The thing you type"                                       │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  LAYER 2: GEOMETRIC INTERMEDIATE REPRESENTATION (GIR)       │
│  ────────────────────────────────────────────────           │
│  Geometry-native structural description                     │
│  All values are geometric: positions, angles, curvatures,   │
│  symmetry groups, containment relations, incidence          │
│  NOT pixels, NOT Bézier curves — actual geometry            │
│  Analogy: MathML, CSG (Constructive Solid Geometry)         │
│                                                             │
│  "The thing the machine understands"                        │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  LAYER 3: RENDERED OUTPUT                                   │
│  ────────────────────                                       │
│  Visual rendering to screen, paper, or other surface        │
│  SVG, Canvas, WebGL, PDF, pen-plotter, 3D print, etc.      │
│  Lossy: continuous geometry → finite resolution             │
│  Analogy: PDF output from LaTeX, audio from Lilypond        │
│                                                             │
│  "The thing you see"                                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

Each layer is necessary. No single format can serve all three roles.

---

## XIII. LAYER 1: THE KEYBOARD SCRIPT (UL-SCRIPT)

### 13.1 Requirements

The keyboard script must be:
1. **Typeable** on a standard ASCII keyboard (no Unicode required)
2. **Parseable** into the geometric IR without ambiguity
3. **Human-readable** — a trained user should be able to read it without rendering
4. **Semantically lossless** — every composition parameter must be expressible
5. **Compact** — not excessively verbose for simple constructions

### 13.2 Design Principles

The script is **not** a visual representation of UL. It is a **structural description language** — like LaTeX describes mathematical formulas without visually being them:

| UL concept | LaTeX analog |
|-----------|-------------|
| Enclosure containing entity | `\frac{a}{b}` — fraction containing numerator and denominator |
| Spatial relationships | `\underset{}{}`, `\overset{}{}` — relative positioning |
| Composition operations | `{}` grouping, `\left(\right)` matched delimiters |
| Geometric parameters | Numeric arguments to commands |

### 13.3 UL-Script Specification

**Primitives:**

```
VOID        .           Empty mark (Void symbol)
POINT       *           Point/Entity
LINE        --          Undirected line
ARROW       ->          Directed line (forward)
BACKARROW   <-          Directed line (backward)
BIARROW     <>          Bidirectional line
ANGLE       @60         Angle value (degrees)
CURVE       ~           Curve (generic)
SPIRAL      ~+          Spiral (increasing radius)
SINE        ~~          Sine wave
HELIX       ~|          Helix (curve with torsion)
```

**Enclosures (boundary shapes):**

```
TRIANGLE    /3{ ... }   or  tri{ ... }
SQUARE      /4{ ... }   or  sq{ ... }
PENTAGON    /5{ ... }   or  pent{ ... }
HEXAGON     /6{ ... }   or  hex{ ... }
CIRCLE      /0{ ... }   or  circ{ ... }
FRAME       [ ... ]     Sentence frame (assertion boundary)
```

The numeric notation `/N{...}` uses N = number of sides (0 = infinite = circle). This is extensible to any polygon.

**Composition operators:**

```
CONTAINMENT     A { B }         B inside A
INTERSECTION    A & B           A overlapping B (AND)
ADJACENCY       A | B           A next to B (OR / sequence)
SEPARATION      A ; B           A apart from B (independent)
CONNECTION      A --r-> B       A connected to B by relation r
```

**Modifiers:**

```
SCALE UP        A^2             Scale A by factor 2
SCALE DOWN      A^0.5           Scale A by factor 0.5
ROTATE          A@90            Rotate A by 90°
REFLECT         A!              Reflect (negate) A
BOLD            A**             Bold stroke (necessity)
DASH            A--             Dashed stroke (possibility)
Z-FRONT         A^z             Bring A to front
Z-BACK          A_z             Send A to back
```

**Structural operators:**

```
EMBED           #[ ... ]        Embed assertion as entity (nominalization)
ABSTRACT        %A              Abstract entity to modifier
COMPOSE         r1 . r2         Compose two relations
INVERT          r!              Invert a relation
QUANTIFY_ALL    *!              Universal quantifier (scaled to fill)
QUANTIFY_SOME   *?              Existential quantifier (small, offset)
```

### 13.4 Examples

**Simple assertion:** "Something acts on something"
```
[ * ->@0 * ]
```

**Modified:** "A fundamental thing harmonizes with a universal thing"
```
[ /3{*} ->@60 /0{*} ]
```

**Conjunction:** "A and B"
```
[ * ->@0 * ] & [ * ->@0 * ]
```

**Nested concept:** "Truth" (complete concept containing existence)
```
/0{ * }
```

**Complex:** "Knowledge is structured truth"
```
[ * ->@0 /4{ /0{*} } ]
```

**Conditional:** "If democracy thrives, then freedom endures"
```
[ #[ /6{/4{<>},*,*,*} ~+ * ] =>@0 #[ *-> ** * ] ]
```

**Self-reference:** "A concept that contains itself"
```
/0{ /0{ /0{ * }}}
```

### 13.5 BNF Grammar for UL-Script

```bnf
<page>        ::= <statement> | <statement> <page_sep> <page>
<page_sep>    ::= '&&' | '||' | '>>' | '<<'

<statement>   ::= <frame> | <expr>
<frame>       ::= '[' <assertion> ']'
<assertion>   ::= <expr> <relation> <expr>
                |  <assertion> '&' <assertion>
                |  <assertion> '|' <assertion>
                |  <assertion> '!'
                |  '*!' <expr> ':' <assertion>
                |  '*?' <expr> ':' <assertion>

<expr>        ::= <atom>
                |  <enclosure>
                |  <embed>
                |  <expr> <modifier>
                |  '%' <expr>

<atom>        ::= '.'           (* void *)
                |  '*'           (* point *)
                |  '*' <id>      (* labeled point *)

<enclosure>   ::= <enc_type> '{' <contents> '}'
<enc_type>    ::= '/0' | '/3' | '/4' | '/5' | '/6' | '/' <number>
                |  'circ' | 'tri' | 'sq' | 'pent' | 'hex'
<contents>    ::= <expr> | <expr> ',' <contents>

<embed>       ::= '#' <frame>

<relation>    ::= <dir> '@' <number>
                |  <dir> '~' <number>
<dir>         ::= '->' | '<-' | '<>' | '--'

<modifier>    ::= '^' <number>   (* scale *)
                |  '@' <number>   (* rotate *)
                |  '!'            (* reflect *)
                |  '**'           (* bold *)
                |  '--'           (* dash *)
                |  '^z'           (* z-front *)
                |  '_z'           (* z-back *)

<id>          ::= [a-z][a-z0-9_]*
<number>      ::= [0-9]+ ('.' [0-9]+)?
```

---

## XIV. LAYER 2: GEOMETRIC INTERMEDIATE REPRESENTATION (UL-GIR)

### 14.1 Why a New Format?

Existing 2D formats describe *appearances*. UL-GIR describes *geometric structure*:

| Existing format | What it stores | What's missing for UL |
|-----------------|---------------|----------------------|
| SVG | Paths, coordinates, styles | No containment semantics, no symmetry groups, no composition operations |
| PDF | Page descriptions, text positioning | Even less structural than SVG — totally opaque |
| Canvas API | Draw commands | Immediate-mode; no scene graph; no queryable structure |
| OpenSCAD/CSG | Constructive solid geometry (3D) | Right idea (constructive), wrong domain (3D solids, not 2D meaning) |
| MathML | Mathematical structure | Right idea (semantic markup for math), too narrow (math only) |
| GeoJSON | Geographic geometry | Geometry-native but geographic, not semantic |

What UL needs is something like **Constructive Semantic Geometry** — a format where:
- Every element is defined by its geometric properties (not pixel coordinates)
- Spatial relationships are **declared**, not rendered-and-inferred
- The format IS a scene graph that can be queried: "what's inside this enclosure?"
- Composition operations are first-class: containment, intersection, adjacency are operations in the format itself
- Symmetry groups are explicit metadata, not visually implied
- The format roundtrips perfectly: structure → render → re-parse → identical structure

### 14.2 UL-GIR Data Model

UL-GIR uses a **geometric scene graph** — a tree of typed nodes where every node carries geometric properties and semantic metadata:

```
GIR Document
├── metadata: { version, author, timestamp }
├── glyph_space: { shape: "circle", radius: 1.0, origin: [0,0] }
└── marks: [
      Mark {
        id: unique identifier
        type: "point" | "line" | "curve" | "angle" | "enclosure" | "frame"
        
        // GEOMETRIC PROPERTIES (primary — the actual data)
        geometry: {
          // For points:
          position: [x, y]  // relative to parent, normalized 0..1
          
          // For lines:
          start: ref(mark_id) | [x, y]
          end: ref(mark_id) | [x, y]
          direction: "forward" | "backward" | "both" | "none"
          
          // For curves:
          curvature: number       // κ: 0 = straight, >0 = curving
          curvature_type: "constant" | "increasing" | "periodic" | "decreasing"
          torsion: number         // τ: 0 = planar
          
          // For angles:
          vertex: ref(mark_id) | [x, y]
          ray1: ref(mark_id)
          ray2: ref(mark_id)
          degrees: number         // θ: the actual angle value
          
          // For enclosures:
          boundary: {
            sides: number        // 3=tri, 4=sq, 5=pent, 6=hex, 0=circle
            symmetry_group: string  // "D3", "D4", "D5", "D6", "SO2"
            orientation: number  // rotation of boundary in degrees
          }
          
          // For frames:
          boundary: {
            shape: "rectangle" | "ellipse" | "arbitrary"
          }
        }
        
        // SEMANTIC PROPERTIES
        sort: "entity" | "relation" | "modifier" | "assertion"
        operation: null | "predicate" | "modify_entity" | ... // which Σ_UL op created this
        
        // COMPOSITION PROPERTIES
        contains: [ref(mark_id), ...]    // containment: these marks are inside this one
        intersects: [ref(mark_id), ...]  // intersection: overlapping marks
        adjacent_to: [ref(mark_id), ...] // adjacency: touching marks
        connected_to: [{                 // connection: explicit relations
          target: ref(mark_id),
          via: ref(mark_id)    // the line/curve that connects them
        }, ...]
        
        // VISUAL MODIFIERS
        scale: number        // relative to parent (1.0 = normal)
        rotation: number     // degrees, relative to parent
        reflected: boolean   // mirrored
        z_order: number      // higher = in front
        stroke: {
          weight: "thin" | "normal" | "bold"
          style: "solid" | "dashed" | "dotted"
        }
        
        // NESTING (recursive)
        children: [Mark, ...]  // marks contained within this mark
      }
    ]
```

### 14.3 Key Design Decisions

**Positions are relative, not absolute.** Each mark's position is relative to its parent container, normalized to [0,1]. This means:
- The same structure renders at any size (scale-independent)
- Nesting works naturally: a child at [0.5, 0.5] is centered in its parent regardless of the parent's size
- The renderer decides actual pixel coordinates; the GIR only specifies *relationships*

**Spatial relationships are explicit, not computed.** The `contains`, `intersects`, `adjacent_to` arrays declare structure. A renderer doesn't need to compute "is point P inside polygon Q?" — the GIR tells it. This means:
- Parsing a GIR file recovers the full UL structure without geometric computation
- Two renderers will always agree on the meaning (even if they lay things out differently)

**Symmetry groups are first-class metadata.** An enclosure with `symmetry_group: "D6"` can be rendered as any D₆-symmetric shape (regular hexagon at any orientation). The renderer chooses the specific visual; the GIR constrains it to the correct symmetry class. This is the right level of abstraction — *geometric invariant*, not *coordinate data*.

**Angles are stored as numeric values, not as path control points.** A 60° angle IS 60° — not "two line segments that happen to form approximately 60° depending on screen resolution." This ensures meaning is preserved through any rendering.

### 14.4 GIR Example: "Knowledge is structured truth"

```json
{
  "ul_gir": "1.0",
  "glyph_space": {"shape": "circle", "radius": 1.0},
  "marks": [
    {
      "id": "frame_1",
      "type": "frame",
      "sort": "assertion",
      "operation": "predicate",
      "geometry": {"boundary": {"shape": "rectangle"}},
      "children": [
        {
          "id": "e_knowledge",
          "type": "point",
          "sort": "entity",
          "geometry": {"position": [0.2, 0.5]},
          "scale": 1.0,
          "stroke": {"weight": "normal", "style": "solid"}
        },
        {
          "id": "r_identity",
          "type": "line",
          "sort": "relation",
          "geometry": {
            "start": {"ref": "e_knowledge"},
            "end": {"ref": "e_structured_truth"},
            "direction": "forward"
          },
          "connected_to": [
            {"target": "e_knowledge", "via": "r_identity"},
            {"target": "e_structured_truth", "via": "r_identity"}
          ],
          "stroke": {"weight": "normal", "style": "solid"}
        },
        {
          "id": "e_structured_truth",
          "type": "enclosure",
          "sort": "entity",
          "operation": "modify_entity",
          "geometry": {
            "position": [0.7, 0.5],
            "boundary": {
              "sides": 4,
              "symmetry_group": "D4",
              "orientation": 0
            }
          },
          "contains": ["e_truth_inner"],
          "children": [
            {
              "id": "e_truth_inner",
              "type": "enclosure",
              "sort": "entity",
              "geometry": {
                "position": [0.5, 0.5],
                "boundary": {
                  "sides": 0,
                  "symmetry_group": "SO2",
                  "orientation": 0
                }
              },
              "contains": ["e_existence"],
              "children": [
                {
                  "id": "e_existence",
                  "type": "point",
                  "sort": "entity",
                  "geometry": {"position": [0.5, 0.5]}
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

This GIR encodes: "An assertion (frame) containing a predicate: entity₁ (knowledge, a point) forward-related to entity₂ (structured truth, a D₄ square containing an SO(2) circle containing a point)."

Any renderer receiving this GIR:
- Knows exactly what's inside what (containment is declared)
- Knows the symmetry groups (D₄, SO(2)) — can choose any compliant visual
- Knows the sort of every mark (entity, relation, assertion)
- Knows the composition operation (predicate at the frame level, modify_entity at the square)
- Can reconstruct the full UL decomposition without geometric inference

### 14.5 GIR vs. Existing Formats

| Property | UL-GIR | SVG | JSON (writing-system.md §8.2) | CSF (text) |
|----------|--------|-----|-------------------------------|------------|
| Geometry-native | **Yes** — angles, curvatures, symmetry groups are primary | No — coordinates/paths | Partial — has types but no geometry | No — linear text |
| Spatial relationships declared | **Yes** — contains, intersects, adjacent_to | No — inferred from coordinates | Partial — nesting implicit | No |
| Roundtrips losslessly | **Yes** — structure survives render/re-parse | No — structure lost in paths | Mostly — some geometry lost | No — 2D→1D lossy |
| Renderable | Needs renderer | Directly renderable | Needs renderer | Needs renderer |
| Human-readable | Somewhat (JSON) | Somewhat (XML) | Somewhat (JSON) | **Yes** |
| Keyboard-typeable | No | No | No | Partially |
| Symmetry groups explicit | **Yes** | No | No | No |
| Sort/operation metadata | **Yes** | No | **Yes** | Partially |

**UL-GIR is what the JSON encoding in writing-system.md §8.2 should have been.** The original JSON encoding was a good first attempt but lacked geometric properties (symmetry groups, explicit angles, containment declarations). UL-GIR is its successor — a proper geometric scene graph.

---

## XV. LAYER 3: THE RENDERING ENGINE (UL-RENDER)

### 15.1 From GIR to Pixels: What the Renderer Does

The renderer takes a UL-GIR document and produces a visual image. This is a **one-way, lossy** process — geometry → pixels. The renderer makes aesthetic decisions that the GIR intentionally leaves open:

| GIR says | Renderer decides |
|----------|-----------------|
| "D₄-symmetric enclosure" | Exact square dimensions, line thickness, corner rounding |
| "Point at relative position [0.5, 0.5]" | Actual pixel coordinates, dot radius |
| "60° angle between these lines" | Exact line lengths, optional arc indicator style |
| "Scale factor 2.0" | How much larger in pixels, respecting layout constraints |
| "Mark A contains mark B" | Where exactly to place B within A for best readability |

**The renderer is an aesthetic engine constrained by geometric invariants.** It has freedom in visual style but zero freedom in structural relationships.

### 15.2 Rendering Algorithm

```
RENDER(gir_document):

  1. ALLOCATE CANVAS
     Read glyph_space → set up coordinate system
     Determine output dimensions from target format

  2. PROCESS MARKS (outside-in, top of tree first)
     For each mark in tree order:
     
     a. COMPUTE POSITION
        - Convert relative [0..1] position to absolute coordinates
        - Apply parent's transform (scale, rotation)
        - Respect containment: children must be geometrically inside parent
     
     b. COMPUTE GEOMETRY
        - Enclosure: Generate boundary path from sides + symmetry_group
          (D₃ → equilateral triangle, D₄ → square, SO(2) → circle, etc.)
          Apply orientation rotation
        - Line/Curve: Generate path from start/end refs + curvature
          Apply stroke properties (weight, style, direction arrows)
        - Angle: Generate two rays from vertex + arc indicator
        - Point: Generate filled circle at position
     
     c. APPLY MODIFIERS
        - scale → multiply dimensions
        - rotation → rotate around mark center
        - reflected → mirror across vertical axis
        - z_order → set draw order (higher z = drawn later = on top)
        - stroke properties → line weight, dash pattern
     
     d. RECURSE INTO CHILDREN
        For nested marks, set the parent boundary as the new coordinate frame
        Recurse with scaled coordinate system

  3. RESOLVE INTERSECTIONS
     For marks declared as intersecting:
     - Compute overlap region geometrically
     - Render overlap zone (potentially with distinct styling)
     - Handle z-ordering for the overlap
     
  4. DRAW CONNECTIONS
     For connected marks:
     - Draw line/curve from source to target
     - Apply relation properties (curvature, direction, angle, style)
     - Attach arrowheads for directed relations
  
  5. DRAW ADJACENCIES
     For adjacent marks:
     - Position so boundaries touch but don't overlap
     - Maintain declared ordering (left-to-right or top-to-bottom)

  6. OUTPUT
     Serialize to target format (SVG, PNG, PDF, Canvas commands)
```

### 15.3 Output Formats

The renderer should target multiple output backends:

| Backend | Format | Best for | Interaction |
|---------|--------|----------|-------------|
| **SVG** | Vector XML | Web display, print, archival | CSS styling, DOM manipulation, hover events |
| **Canvas 2D** | Raster bitmap | Real-time interactive display | Fast redraw, animation, touch/mouse input |
| **WebGL/GPU** | GPU primitives | Complex glyphs, 3D future, large documents | Massive parallelism, shader-based rendering |
| **PDF** | Page description | Print, formal documents | Static output |
| **Pen plotter** | G-code / HPGL | Physical drawing, art | Actual pen on paper — full circle back to handwriting |

### 15.4 Why This Architecture Works

The three-layer split solves each problem with the right tool:

```
HUMAN WRITES: UL-Script (ASCII, keyboard-friendly)
    │
    │  Parser (deterministic, ~500 lines of code)
    ▼
MACHINE STORES: UL-GIR (geometry-native, lossless)
    │
    │  Can be queried, transformed, compared, validated
    │  This is where AI systems exchange meaning
    │
    │  Renderer (deterministic, ~2000 lines of code)
    ▼
HUMAN SEES: Rendered visual (SVG/Canvas/PDF)
    │
    │  Optional: Computer vision (ML model)
    ▼
MACHINE READS: Back to UL-GIR (lossy but useful for handwritten input)
```

**The GIR is the center of gravity.** Everything converts to and from it. UL-Script is the human-friendly input; rendered visuals are the human-friendly output; GIR is the machine-native representation. AI agents exchange GIR, not text and not images.

---

## XVI. OPEN PROBLEMS

| Problem | Difficulty | Impact |
|---------|-----------|--------|
| **Optimal 2D layout algorithm** for arbitrary nesting depth | Hard (NP-hard in general) | Critical for rendering |
| **Canonical form for compounds** — when are two different compositions "the same meaning"? | Hard (equivalence of geometric constructions) | Critical for Thesaurus extension |
| **Multi-page composition** — how do glyphs on different pages relate? | Medium | Needed for documents |
| **Animation semantics** — what does it mean for a glyph to change over time? | Medium | Needed for interactive UL |
| **3D composition** — extending from 2D surface to 3D volume | Research | Opens new spatial relationships |
| **Automatic style** — choosing visually pleasing layouts from structural encoding | Medium | Quality-of-life for rendering |
| **Disambiguation of NL→UL** — "I saw the man with the telescope" has one UL encoding per reading | Medium (requires dialogue) | Needed for reliable translation |
| **UL-GIR formal specification** — full schema, validation rules, versioning | Medium | Required before implementation |
| **UL-Script parser implementation** — BNF → working parser in multiple languages | Straightforward | Required for any tooling |
| **Lossy round-trip** — how much structure survives render → computer vision → GIR? | Hard (ML + geometry) | Needed for handwritten input digitization |
| **GIR diff/merge** — comparing and combining two GIR documents | Medium | Needed for collaborative editing |
| **Accessibility mapping** — GIR → audio/tactile representation | Research | Critical for universality claim |

---

## XVII. SUMMARY

### What You Now Know

1. **Five spatial relationships** exhaust all possible ways marks can compose on a 2D surface
2. **Composition depth** (layers) increases monotonically — deeper = more defined
3. **Nine parameters** control how any two marks compose (spatial relationship, depth, scale, orientation, z-order, alignment, connection properties, boundary properties, transformation)
4. Intersection and adjacency are **both valid** and carry **different meanings** (AND vs. OR/sequence)
5. New symbols of increasing definition are created by **climbing the composition ladder** through three methods: deepening (nesting), broadening (connecting/adjacency), and blending (overlapping)
6. **Fonts and text rendering are structurally inadequate** for UL — the mismatch between 1D character streams and 2D geometric algebra is unfixable within existing text paradigms
7. **Three digital layers** are needed: UL-Script (ASCII keyboard input), UL-GIR (geometry-native intermediate representation), and rendered output (SVG/Canvas/PDF)
8. **UL-Script** is a keyboard-friendly structural description language — like LaTeX for meaning geometry
9. **UL-GIR** is the machine-native center of gravity — declared geometric structure with explicit spatial relationships, symmetry groups, and Σ_UL metadata
10. The rendering engine is an **aesthetic engine constrained by geometric invariants** — free in style, bound in structure

### What This Document Adds to the System

| Sibling | What this extends |
|---------|------------------|
| **Symbology** | Formalizes the 9 composition operations as a complete algebra (GCA) |
| **Syntax** | Extends beyond binary operations to ternary, radial, fractal, and tabular composition |
| **Grammar** | Adds symmetry composition theory (how component symmetries combine) |
| **Lexicon** | Provides the stability criteria for when compounds earn canonical status |
| **Thesaurus** | Defines the space of compositional equivalences |
| **Writing System** | Replaces the preliminary rendering pipeline (§8) with a three-layer architecture: UL-Script (keyboard), UL-GIR (geometry-native), rendered output |
