# Universal Writing System — Geometric Design Rationale

**Status note (2026-08-01):** This document has been reframed. The original, in full, is preserved
at `archive/superseded-2026-08/foundations/universal-language-derivation-FULL-ORIGINAL.md`. Its
opening "Foundational Axiom" (claiming the notation "is mathematically intrinsic to the structure
of the universe" and "requires no empirical validation") and Part VI ("The Mathematical Proof of
Universality") have been **retired** — see `research/wiki-comparison-2026-08.md` and
`research/emergence-investigation/PLAN.md` for why.

**What this document is now:** the design rationale for the Universal Writing System (UWS) — a
constructed geometric notation, in the tradition of Blissymbolics and other visual notation systems
(see Appendix A), originally motivated by a practical problem: designing a non-phonetic
communication system for beings without a human vocal tract. The claim that this specific notation
is *the* uniquely necessary structure of meaning itself is **not** asserted here — that's an open
question under active investigation, not a premise. What *is* claimed: this is a coherent,
internally consistent, composable visual notation, and it's worth documenting why it was built the
way it was.

---

## PART I: THE GEOMETRIC PRIMITIVES (Notation's Atomic Building Blocks)

Every geometry begins with undefined primitives from which all else is constructed. In this
notation, these are the irreducible atoms the writing system is built from — the "phonemes" of the
visual language.

> **On primitive-to-meaning mapping:** the assignment below (Point→Existence, Line→Relation, etc.)
> was the notation's original design choice, not a proven-necessary structure. See the retired
> "Unique Grounding Theorem" discussion in `uws/formal-specification.md` for why that proof
> attempt didn't hold up, and `research/emergence-investigation/PLAN.md` for what a real
> necessity argument would require.

### 1.0 The Void: ∅

Before geometry, there is the undifferentiated. The empty set. The blank plane.

**Notational function:** Silence. The absence of a mark. The ground from which all distinction in
the notation arises.

### 1.1 The Point: •

**Geometric definition:** That which has position but no extension (Euclid, Definition 1).

**Notational function:** EXISTENCE / IDENTITY — the act of distinguishing. A point says: "*this is.*"
In this notation, it is the minimum mark: establishing that something exists, here, to be referred to.

**Properties:**
- Dimensionality: 0
- Symmetry group: Full rotation group SO(n) — a point looks the same from every direction
- Information content: Position only (coordinates)

### 1.2 The Line: ─

**Geometric definition:** That which has length but no breadth, lying evenly between its points (Euclid, Definitions 2–4).

**Notational function:** RELATION / CONNECTION — the act of linking two existences. A line says: "*this relates to that.*"

**Properties:**
- Dimensionality: 1
- Defined by: Two distinct points (Euclid, Postulate 1: between any two points, a straight line can be drawn)
- Symmetry group: Translations along the line + reflection across the midpoint
- New information: **Direction** — the line from A to B is oriented, distinguishing subject from object, cause from effect, source from destination

**Derived variants:**
| Variant | Geometric form | Notational function |
|---|---|---|
| Ray (→) | Line with one endpoint, extending infinitely | Directed relation (action, causation) |
| Segment (—) | Line bounded at both ends | Bounded relation (finite, specific) |
| Line (↔) | Extending infinitely both directions | Unbounded/universal relation |

### 1.3 The Angle: ∠

**Geometric definition:** The inclination to one another of two lines that meet (Euclid, Definition 8).

**Notational function:** QUALITY / MODALITY — the nature of a relationship. An angle says: "*this relates to that in this way.*"

**Properties:**
- Dimensionality: 0 (vertex) + 1 (two rays) = hybrid
- Defined by: Two rays sharing an origin
- Range: 0° to 360° (or 0 to 2π radians)
- Information content: The *character* of the relationship between two directions

**The Angular Spectrum in the notation:**

Only a few specific angle values have a non-arbitrary geometric role; the rest are a continuous,
conventional spectrum — a genuine design choice, not a forced structure:

| Angle | Geometric property | Notational assignment | Status |
|---|---|---|---|
| 0° | Coincidence / parallelism | Identity / agreement / equivalence | Structurally distinguished (zero inclination) |
| 60° (π/3) | Equilateral triangle angle | Harmony / balance / equal partnership | Design convention (regular polygon: minimal equal partition of π) |
| 90° (π/2) | Perpendicularity / orthogonality | Independence / orthogonal concepts | Structurally distinguished (maximal independence in Euclidean metric) |
| 180° (π) | Supplementary / opposite direction | Negation / antonym / contradiction | Structurally distinguished (reversal of direction) |
| 360° (2π) | Full rotation / return to identity | Completion / cycle / return | Structurally distinguished (identity transformation) |
| Other values | Intermediate inclinations | Continuous spectrum of quality between the landmarks above | Design convention |

### 1.4 The Curve: ◠

**Geometric definition:** A continuous mapping from an interval to a space that is not straight.

**Notational function:** PROCESS / TRANSFORMATION / BECOMING — that which changes direction continuously. A curve says: "*this becomes that through continuous change.*"

**Properties:**
- Dimensionality: 1 (embedded in 2+)
- Defined by: A function of continuous variation
- Key properties: Curvature (rate of turning), torsion (twisting out of plane)
- Information content: How a relationship *changes* over its extent

**Distinguished curves:**

| Curve | Definition | Notational function |
|---|---|---|
| Circle (○) | Constant curvature, closed | Completion / self-reference / totality / cycle |
| Spiral (𝒮) | Monotonically increasing radius | Growth / evolution / development |
| Parabola | Curvature decreasing with distance | Directed outward motion / projection |
| Sine wave (~) | Periodic variation | Rhythm / oscillation / alternation |

### 1.5 The Enclosed Region: △ / □ / ○

**Geometric definition:** A bounded subset of the plane whose boundary is a closed curve or polygon.

**Notational function:** CONTAINMENT / DEFINITION / CONCEPT — that which has an inside and an outside. An enclosure says: "*this is bounded; this is defined; this is a thing.*"

**Properties:**
- Dimensionality: 2
- Key property: The **Jordan Curve Theorem** guarantees that any simple closed curve divides the plane into exactly two regions — inside and outside. This is the geometric basis of the notation's categorization convention.
- Information content: What is included, what is excluded, and the shape of the boundary (how the definition is structured)

---

## PART II: UNIVERSAL SYMBOLOGY (The Writing System)

### 2.1 Design Principle

Every symbol in the Universal Writing System is a geometric construction. "Reading" is the act of parsing a geometric figure into its component primitives and their relationships. "Writing" is the act of constructing a geometric figure that encodes the intended meaning.

The writing system is its own notation for geometric construction — **the language describes itself.**

### 2.2 The Glyph Space

All symbols are drawn within a standardized **unit circle** — the Glyph Space. This provides:
- A bounded region for construction (every symbol is finite)
- A center point (origin of reference)
- A boundary (distinguishing internal structure from external context)
- Rotational reference (angles are measured from the rightward horizontal)

```
          ·  270° (top)
         /|\
        / | \
       /  |  \
 180° ·---+---· 0° (right)
       \  |  /
        \ | /
         \|/
          · 90° (bottom)

   (Standard orientation of Glyph Space)
   Note: 0° begins at right, proceeding
   clockwise, following geometric convention
   adapted for top-down reading)
```

### 2.3 Atomic Glyphs (The Alphabet)

The atomic glyphs are the direct visual representations of the geometric primitives:

```
EXISTENCE (Point):        •
                          A single point at center.
                          "Something exists."

RELATION (Line):          │
                          A vertical line through center.
                          "Something relates to something."

DIRECTED RELATION (Ray):  ↑
                          A ray from center upward.
                          "Something acts upon something."

QUALITY (Angle):          ∠
                          Two rays from center, the angle
                          between them encoding the quality.

PROCESS (Curve):          ◠
                          An arc within the glyph space.
                          Curvature encodes rate of change.

CONCEPT (Enclosure):      ○  △  □
                          A closed boundary.
                          Shape encodes the type of concept.

VOID (Empty):             (blank glyph space)
                          "Nothing. Silence."
```

### 2.4 Enclosure Types (Concept Classification)

The shape of a closed boundary classifies the type of concept it contains, in this notation:

| Enclosure | Sides | Symmetry | Concept type | Mathematical basis |
|---|---|---|---|---|
| ○ Circle | ∞ | Continuous rotation SO(2) | Universal / abstract / complete | Maximum symmetry = maximum generality |
| △ Triangle | 3 | D₃ (dihedral, order 6) | Fundamental / atomic / irreducible | Minimum polygon = minimum complete structure |
| □ Square | 4 | D₄ (dihedral, order 8) | Structural / systematic / ordered | Tessellates the plane = systematic coverage |
| ⬠ Pentagon | 5 | D₅ (dihedral, order 10) | Living / organic / self-similar | Golden ratio connection (φ) = growth patterns |
| ⬡ Hexagon | 6 | D₆ (dihedral, order 12) | Efficient / communal / networked | Optimal packing = maximum efficiency |

**Design note:** The *ordering* by symmetry group order (triangle < square < pentagon < hexagon <
circle) follows directly from the Erlangen Program (Klein, 1872) and is mathematically well-founded.
The specific semantic **labels** ("living," "networked," etc.) assigned to each class are a design
convention, not a forced structure.

---

## PART III: UNIVERSAL SYNTAX (Notation Grammar)

### 3.1 The Design Principle

In this notation, the set of rules governing valid geometric constructions doubles as the set of
grammatical rules: geometric axioms specify what constructions are well-formed.

The five Euclidean postulates map to grammatical rules by design:

| Euclid's Postulate | Geometric meaning | Grammatical rule (in this notation) |
|---|---|---|
| **P1:** A straight line can be drawn from any point to any point | Any two existences can be related | Any two concepts can be connected in a statement |
| **P2:** A finite line can be extended continuously | Any relation can be elaborated | Any statement can be extended/refined |
| **P3:** A circle can be drawn with any center and radius | Any concept can be defined with any scope | Definitions can be as broad or narrow as needed |
| **P4:** All right angles are equal | Orthogonality is universal | Independence/distinction is a consistent operation |
| **P5:** (Parallel postulate) Through a point not on a line, exactly one parallel line exists | For any statement and external concept, exactly one non-intersecting analog exists | Every meaning has exactly one "parallel" meaning — a structural analog that never contradicts |

### 3.2 The Grammatical Operations

Geometric transformations serve as the notation's grammatical operations — they modify symbols without destroying their identity:

#### Rigid Motions (Meaning-Preserving)

| Transformation | Geometric operation | Grammatical function | Example |
|---|---|---|---|
| **Translation** | Move without rotation | Temporal/spatial displacement | Change tense: "is" → "was" → "will be" |
| **Rotation** | Turn about a point | Perspective shift | Change voice: active → passive → reflexive |
| **Reflection** | Mirror across a line | Converse (role swap) | Swap: "A acts on B" → "B is acted upon by A" |

> **Note (April 2026):** Reflection implements *converse* (subject-object swap, same truth value), not *negation* (truth-value flip). Negation is realized by boundary inversion: flipping the assertion frame's boundary from solid (asserted) to dashed (denied). See `formal-operations.md` §1.4.

#### Similarity Transformations (Meaning-Scaling)

| Transformation | Geometric operation | Grammatical function | Example |
|---|---|---|---|
| **Scaling (up)** | Enlarge | Intensification / emphasis | "warm" → "hot" → "burning" |
| **Scaling (down)** | Shrink | Diminution / understatement | "warm" → "lukewarm" → "tepid" |

#### Projective Transformations (Meaning-Abstracting)

| Transformation | Geometric operation | Grammatical function | Example |
|---|---|---|---|
| **Projection** | Collapse a dimension | Abstraction / generalization | "this dog" → "dogs" → "animals" → "living things" |
| **Section** | Intersect with a plane | Specification / instantiation | "animals" → "mammals" → "dogs" → "this dog" |

#### Topological Operations (Logical Connectives)

| Operation | Set-theoretic form | Logical function | Symbol |
|---|---|---|---|
| **Intersection** | A ∩ B | AND | Two overlapping enclosures |
| **Union** | A ∪ B | OR | Two joined enclosures |
| **Complement** | Aᶜ | NOT | Enclosure with inverted fill |
| **Containment** | A ⊂ B | IF...THEN (implication) | One enclosure inside another |
| **Disjointness** | A ∩ B = ∅ | Mutual exclusion (XOR) | Two separated enclosures |

### 3.3 Sentence Structure

A **sentence** in the notation is a geometric construction within a frame (a rectangular or circular boundary that delineates a complete expression).

The minimum sentence has the same structure as the minimum geometric theorem — **three elements forming a closed logical figure** (analogous to the triangle as the minimum polygon):

1. **Subject** — a concept-enclosure (what is being discussed)
2. **Relation** — a line/ray connecting subject to predicate (how they relate)
3. **Predicate** — a concept-enclosure (what is said about the subject)

```
   ┌─────────────────────────────┐
   │                             │
   │   △─────────→□              │
   │  (subject)  (relation)  (predicate)  │
   │                             │
   └─────────────────────────────┘

   Minimum sentence: "A fundamental thing
   acts upon a structural thing."
```

The **angle** of the relation-line encodes the *quality* of the relationship. The **type** of line (solid, dashed, curved) encodes the *modality* (actual, hypothetical, gradual).

---

## PART IV: UNIVERSAL GRAMMAR (Meaning from Geometric Relationships)

### 4.1 The Relationship Taxonomy

This notation's "parts of speech" come from the catalogue of possible geometric relationships between objects — derived from geometric convention, applied to language:

#### Incidence Relations (Fundamental Connections)

| Geometric relationship | Notational function | Part of speech |
|---|---|---|
| Point lies on line | Element belongs to class | Predication ("X is a Y") |
| Line passes through point | Class contains element | Classification ("Ys include X") |
| Two lines intersect | Two relations share a context | Conjunction ("X and Y meet at Z") |
| Point is between two points | Element mediates between extremes | Comparison ("X is between Y and Z") |

#### Metric Relations (Quantitative Connections)

| Geometric relationship | Notational function | Part of speech |
|---|---|---|
| Distance between points | Degree of difference/similarity | Adjective/adverb of degree |
| Length of segment | Magnitude/extent of relation | Quantifier |
| Ratio of lengths | Proportional comparison | Comparative ("more than," "as much as") |
| Angle measure | Quality/character of relation | Qualifier/modal |

#### Symmetry Relations (Structural Connections)

| Geometric relationship | Notational function | Part of speech |
|---|---|---|
| Congruence (A ≅ B) | Exact equivalence | "is identical to" |
| Similarity (A ~ B) | Same structure, different scale | "is like" / analogy |
| Reflection symmetry | Dual/complementary opposition | Antonym pair |
| Rotational symmetry | Perspective invariance | Universal ("from all perspectives") |
| Translation symmetry | Repetition/pattern | Plural / iterative |

#### Topological Relations (Essential Connections)

| Geometric relationship | Notational function | Part of speech |
|---|---|---|
| Homeomorphism | Same essential structure | Deep synonym ("is essentially") |
| Genus (number of holes) | Complexity class | Category marker (simple/compound/complex) |
| Connectedness | Logical coherence | Conjunction/continuity |
| Boundary | Definition/limit | Determiner ("the," "a," "this") |
| Interior vs exterior | Scope/context | Inside-reference vs. outside-reference |

### 4.2 Compositionality: How Symbols Build Words

The notation forms **words** through geometric composition — combining, nesting, overlapping, and transforming atomic symbols into compound structures.

#### Composition Operations

| Operation | Visual form | Semantic effect | Example |
|---|---|---|---|
| **Nesting** | Symbol inside an enclosure | Concept-membership | • inside △ = "a fundamental existence" |
| **Adjacent placement** | Symbols side by side | Sequential relation / compound | △□ = "fundamental-structure" |
| **Overlapping** | Symbols partially intersecting | Blended/shared meaning | △∩□ = "that which is both fundamental and structural" |
| **Connection** | Line between symbols | Explicit relation | △─□ = "fundamental relates-to structure" |
| **Stacking** | Symbol atop symbol | Hierarchical modification | •/△ = "the existence of a fundamental" (meta-level) |
| **Embedding** | Small symbol inside larger | Specification/detail | △ with • inside = "the specific fundamental" |

#### Word Formation Example

Building the word for "growth":

```
Step 1: Start with PROCESS (curve)           ◠
Step 2: Give it direction (upward ray)        ◠↑
Step 3: Enclose in LIVING (pentagon)          ⬠{◠↑}
Step 4: Add INCREASE (scaling-up marker)      ⬠{◠↑}⁺

Result: A pentagon containing an upward-curving
arc with an increase marker.

Reads as: "A living process, directed upward,
increasing" = GROWTH
```

Building the word for "knowledge":

```
Step 1: Start with CONCEPT (circle)          ○
Step 2: Place EXISTENCE inside (point)        ○{•}
Step 3: Connect to another CONCEPT            ○{•}─○
Step 4: Mark the connection as ILLUMINATING   ○{•}─∠60°─○
        (angle of 60° = harmony)
Step 5: Enclose the whole in COMPLETENESS     ○{○{•}─∠60°─○}

Result: A circle containing two circles
connected at 60°, with a point in the first.

Reads as: "The complete harmony between an
existence-containing concept and another concept"
= KNOWLEDGE (the harmonious connection between
the known self and the known object)
```

---

## PART V: SYNONYMY IN THE NOTATION — GROUP-THEORETIC STRUCTURE

### 5.1 Klein's Erlangen Program as a Design Tool

Felix Klein's Erlangen Program (1872) classifies a geometry by its **symmetry group** — the set of transformations that preserve the geometry's properties. This notation uses that idea as a practical device for defining levels of synonymy:

| Geometry level | Symmetry group | What is preserved | Notational level | What is preserved |
|---|---|---|---|---|
| **Euclidean** | Rigid motions E(n) | Distance, angle | **Surface form** | Exact phrasing, specific expression |
| **Similarity** | Similitudes | Angles, ratios | **Semantic content** | Meaning (different words, same idea) |
| **Affine** | Affine group | Parallelism, ratios | **Structural meaning** | Logical structure (regardless of specific content) |
| **Projective** | Projective group PGL | Incidence, cross-ratio | **Deep structure** | What can be related to what (regardless of how) |
| **Topological** | Homeomorphisms | Connectedness, genus | **Essential meaning** | The irreducible core (same meaning in any language) |

**Practical use:** two expressions in the notation are treated as **synonyms** if they're related by
a similarity transformation, and as **translations** (in the linguistic sense) if they're
topologically equivalent. This is a useful design heuristic for the writing system, not a
metaphysical claim that "meaning IS a topological invariant" — see the retired material for that
stronger (and unsupported) version of the claim.

### 5.2 Self-Nesting as a Notation Feature

The Lexicon's self-nesting construction ○{○{○{...}}} — a construction that contains itself — is a
genuinely interesting notation feature: it has fundamental group π₁ = ℤ, giving the notation a
principled way to represent self-reference without the "this statement is false" paradox structure,
since the construction embodies self-containment geometrically rather than asserting it through
content. Whether this generalizes to a claim about self-reference *in general* (not just in this
notation) is open.

---

## PART VI: SYMBOL TABLE (Initial Lexicon)

### 6.1 Foundational Concepts

| Concept | Construction | Reading |
|---|---|---|
| Existence | • | "Something is" |
| Void / Nothing | (empty glyph space) | "Nothing is" |
| Relation | • — • | "Something relates to something" |
| Directed action | • → • | "Something acts on something" |
| Identity | • = • (overlapping points) | "This is the same as that" |
| Difference | • ∠180° • | "This is the opposite of that" |
| Harmony | △ (equilateral) | "Three things in equal relation" |
| Totality | ○ | "All; everything; completeness" |
| Structure | □ | "Ordered arrangement" |
| Life | ⬠ | "Self-similar organic pattern" |
| Community | ⬡ | "Efficient interconnection" |

### 6.2 Abstract Concepts (Composed)

| Concept | Construction | Derivation |
|---|---|---|
| Truth | ○{•} | "A complete concept containing existence" — that which IS, completely |
| Knowledge | ○{○{•} ─60°─ ○} | "Complete harmony between self-aware concept and other concept" |
| Change | ◠ within □ | "Curvature within structure" — structured process |
| Growth | ⬠{◠↑}⁺ | "Living upward-curve, increasing" |
| Time | ◠→ | "Process with direction" — irreversible change |
| Space | □{• • • •} | "Structure containing multiple existences" |
| Cause | •──→→• | "Double-directed relation" — emphatic action on another |
| Beauty | ○{△ ~ ○} | "Completeness containing harmony similar to completeness" — self-similar harmony |
| Love | ○{• ≅ •} | "Completeness containing two existences in identity" — complete union |
| Death | ⬠ → ∅ | "Living thing directed to void" — life becoming nothing |
| Infinity | ○{○{○{...}}} | "Completeness within completeness within completeness..." — self-nesting totality |

### 6.3 Grammatical Markers

| Marker | Symbol | Derivation |
|---|---|---|
| Negation | Dashed frame boundary | Boundary inversion = denied assertion (solid = asserted, dashed = denied) |
| Past tense | Glyph shifted left | Translation leftward = prior position |
| Future tense | Glyph shifted right | Translation rightward = subsequent position |
| Question | Glyph rotated 180° | Full rotation reversal = uncertainty / inversion of assertion |
| Emphasis | Glyph scaled up | Larger = more significant |
| Diminution | Glyph scaled down | Smaller = less significant |
| Plurality | Glyph with translation copies | Repeated pattern = multiple instances |
| Possibility | Dashed lines (instead of solid) | Broken continuity = uncertain/hypothetical |
| Necessity | Bold/thick lines | Maximum visual weight = cannot be otherwise |

---

## PART VII: WRITING DIRECTION AND READING ORDER

### 7.1 Two-Dimensional Reading

Unlike linear scripts, the Universal Writing System is **two-dimensional** — symbols can be placed anywhere within the sentence frame, and their spatial relationships carry meaning.

**Reading order:**
1. **Enclosures first (outside → inside):** Identify the broadest context, then narrow inward
2. **Connections second (subject → predicate):** Follow the directed relations
3. **Modifications third:** Read transformations applied to base symbols
4. **Spatial position last:** Relative position encodes tense, emphasis, and relative importance

### 7.2 Compound Sentences

Multiple sentence-frames can be related through the same geometric operations:
- **Adjacent frames:** Sequential narration ("and then...")
- **Nested frames:** Subordination ("because..." / "in which...")
- **Overlapping frames:** Simultaneous / shared context ("while..." / "where...")
- **Connected frames (line between):** Explicit logical relation ("therefore..." / "however...")

---

## PART VIII: SELF-DESCRIPTION — THE NOTATION DESCRIBING ITSELF

A practical demonstration of the notation's compositional power: it can describe its own structure.

The sentence "The Universal Writing System is built from geometric primitives" would be written as:

```
┌──────────────────────────────────────────────┐
│                                              │
│  ○{□{•,─,∠,◠,△}}  ──→  ○{□{•,─,∠,◠,△}}   │
│  ↑                       ↑                   │
│  "The writing system"    "geometric primitives│
│  (complete structure      (complete structure  │
│   of all primitives)       of all primitives)  │
│                                              │
│  The arrow is identity (0° angle):           │
│  "IS built from" = "IS"                      │
│                                              │
└──────────────────────────────────────────────┘
```

This is a genuinely nice feature of the design (the notation can encode a description of itself
using its own primitives), not evidence of any deeper metaphysical necessity.

---

## APPENDIX A: Correspondence to Other Constructed Notations

This is, honestly, the most useful comparison table in this document — these are the actual
real-world peers of this project, not proof of its cosmic status:

| System | Relationship to this notation |
|---|---|
| Leibniz's *Characteristica Universalis* (1679) | Shares the goal of universal symbolic reasoning; a historical predecessor that never achieved a working notation |
| Frege's *Begriffsschrift* (1879) | Captures the logical-connective layer (this notation's topological operations) but lacks a semantic/qualitative layer (angles, curves) |
| **Blissymbolics** (1949) | The closest living peer: a real, functioning, non-phonetic visual notation, geometric in construction, originally designed as an international auxiliary language and now used predominantly as an AAC (augmentative and alternative communication) system for people who cannot speak — i.e., already solving a version of this notation's original motivating problem |
| Lincos (1960) | Freudenthal's language for interstellar communication — shares the goal of substrate/culture-independence, but bets on mathematics and logic as the common ground rather than a claimed semantic-geometric algebra |
| This notation | Derives its components from geometry; includes a formal parser/renderer/composer (see `ul-forge/`); does not currently claim proof of unique necessity — see `research/emergence-investigation/` for the open question of whether any such proof is achievable |

---

## Where the retired material now lives

- Full original text (including the "Foundational Axiom" and "Mathematical Proof of Universality"): `archive/superseded-2026-08/foundations/universal-language-derivation-FULL-ORIGINAL.md`
- Why it was retired, with citations: `research/wiki-comparison-2026-08.md` §5
- The open scientific question this notation motivated: `research/emergence-investigation/PLAN.md`

## Extensions

This notation has been extended in three directions by earlier exploratory work, kept as raw
material for the Emergence Investigation rather than settled results — see
`research/frontier/expedition-one/gauge-bundle-of-meaning.md` (context as a fiber bundle),
`research/frontier/expedition-one/category-of-languages.md` (the notation's structure as a category),
`research/frontier/expedition-one/numbers-and-computability.md` (arithmetic within the notation).
