# Glyph Composition Algebra

> The formal theory of how symbols combine to produce new symbols of increasing definition.  
> Composition is the engine of UL's infinite generativity — finitely many primitives, infinitely many meanings.

**TL;DR:** Symbols combine through exactly 5 spatial relationships (containment, intersection, adjacency, separation, connection) across unbounded composition depth. Each combination produces a new compound glyph with strictly more meaning than its parts. Layers, z-ordering, alignment, grouping, and scale all carry semantic weight. This document formalizes what the other 5 siblings leave implicit: the complete visual algebra of how drawn marks compose into arbitrarily complex meaning-structures.

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

## XII. OPEN PROBLEMS

| Problem | Difficulty | Impact |
|---------|-----------|--------|
| **Optimal 2D layout algorithm** for arbitrary nesting depth | Hard (NP-hard in general) | Critical for rendering |
| **Canonical form for compounds** — when are two different compositions "the same meaning"? | Hard (equivalence of geometric constructions) | Critical for Thesaurus extension |
| **Multi-page composition** — how do glyphs on different pages relate? | Medium | Needed for documents |
| **Animation semantics** — what does it mean for a glyph to change over time? | Medium | Needed for interactive UL |
| **3D composition** — extending from 2D surface to 3D volume | Research | Opens new spatial relationships |
| **Automatic style** — choosing visually pleasing layouts from structural encoding | Medium | Quality-of-life for rendering |
| **Disambiguation of NL→UL** — "I saw the man with the telescope" has one UL encoding per reading | Medium (requires dialogue) | Needed for reliable translation |

---

## XIII. SUMMARY

### What You Now Know

1. **Five spatial relationships** exhaust all possible ways marks can compose on a 2D surface
2. **Composition depth** (layers) increases monotonically — deeper = more defined
3. **Nine parameters** control how any two marks compose (spatial relationship, depth, scale, orientation, z-order, alignment, connection properties, boundary properties, transformation)
4. Intersection and adjacency are **both valid** and carry **different meanings** (AND vs. OR/sequence)
5. New symbols of increasing definition are created by **climbing the composition ladder** through three methods: deepening (nesting), broadening (connecting/adjacency), and blending (overlapping)
6. A rendering system has **three components**: NL decomposer, structural parser, and geometric renderer
7. The practical path from English to UWS glyphs runs through **LLM decomposition** → **deterministic rendering**

### What This Document Adds to the System

| Sibling | What this extends |
|---------|------------------|
| **Symbology** | Formalizes the 9 composition operations as a complete algebra (GCA) |
| **Syntax** | Extends beyond binary operations to ternary, radial, fractal, and tabular composition |
| **Grammar** | Adds symmetry composition theory (how component symmetries combine) |
| **Lexicon** | Provides the stability criteria for when compounds earn canonical status |
| **Thesaurus** | Defines the space of compositional equivalences |
| **Writing System** | Specifies the rendering pipeline from NL input to visual output |
