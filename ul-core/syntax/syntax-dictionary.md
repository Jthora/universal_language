# Universal Syntax Dictionary

> The complete ruleset for valid constructions in Universal Language.  
> Syntax IS geometric construction. Grammar rules ARE axioms.  
> Every valid sentence is a valid geometric figure. Every invalid sentence is a geometric impossibility.

### How This Document Relates to Its Siblings

| If you need... | Consult | Why |
|----------------|---------|-----|
| What the atomic symbols look like | **Symbology** (symbology/symbol-map.md) | Symbology defines the pieces; Syntax defines how they combine |
| Why meanings emerge from combinations | **Grammar** (grammar-book.md) | Grammar explains the semantic theory; Syntax provides the mechanical rules |
| Whether a construction is canonical | **Lexicon** (lexicon/lexicon.md) | Lexicon catalogs distinguished results of applying these rules |
| Synonyms under transformation | **Thesaurus** (thesaurus/thesaurus.md) | Thesaurus maps equivalence classes between valid constructions |
| Practical writing and reading procedures | **Writer's Companion** (writing-system/writers-companion.md) | Step-by-step pen-and-paper guidance |
| Full visual composition algebra | **Glyph Composition** (composition/glyph-composition.md) | Extends binary operations to ternary, radial, fractal; formalizes 5 spatial relationships |

> **Note on two classification systems:** This document classifies expressions algebraically by **sort** (Entity, Relation, Modifier, Assertion) from Σ_UL. Grammar classifies symbols linguistically by **symmetry group** (determining parts of speech). These are complementary: the same symbol (e.g., →) is both a Relation (sort, algebraic role) and a Verb (part of speech, linguistic role due to low rotational symmetry). Sort determines what algebraic operations apply; symmetry determines how the symbol behaves linguistically.

> **Geometric vs. Algebraic Representation:** Some geometric features (like enclosure boundary shapes: triangle vs. hexagon) are geometric-only properties that enter Σ_UL indirectly through the modifier sort (via `abstract(e) → m`). See Lexicon §0.8 for the full G ≠ Σ_UL distinction.

---

## I. FIRST PRINCIPLES

### The Syntactic Axiom
**Syntax is the set of rules governing valid constructions.** In geometry, axioms govern valid constructions. Therefore:

> **Geometric axioms ARE the grammatical rules of Universal Language.**

A "sentence" in UL is a geometric figure within a frame. A "valid sentence" is a figure that obeys the construction axioms. An "invalid sentence" is a figure that violates them — and a geometric violation is objectively detectable, not a matter of convention.

---

## II. THE FIVE CONSTRUCTION AXIOMS (From Euclid)

### Axiom 1: Universal Connectivity
> *"A straight line can be drawn from any point to any point."* — Euclid, Postulate 1

**Syntactic rule:** Any two entities can be related.

```
VALID:   •───•     (any two things can be connected)
VALID:   •───•───•  (any chain of connections)
VALID:   △───○     (any two concept-types can be connected)
```

**What this forbids:** Nothing. Any two symbols may be connected. There are no "syntactically incompatible" entities. This is the maximum-freedom postulate.

### Axiom 2: Extensibility
> *"A finite straight line can be extended continuously in a straight line."* — Euclid, Postulate 2

**Syntactic rule:** Any relation can be elaborated. Any statement can be extended or refined.

```
VALID:   •──•  →  •──•──•  →  •──•──•──•
         (extend any connection)

VALID:   △{•}──○{•}  →  △{•}──□{──}──○{•}
         (insert mediating structure into any relation)
```

**What this means:** There is no "maximum sentence length." Every construction can be extended with additional structure. UL has no upper bound on complexity.

### Axiom 3: Scope Freedom
> *"A circle can be described with any center and any radius."* — Euclid, Postulate 3

**Syntactic rule:** Any concept can be defined with any scope. Definitions can be as broad or narrow as needed.

```
VALID:   ○{•}           (concept containing one thing — very narrow)
VALID:   ○{•,•,•,...}    (concept containing many things — broad)
VALID:   ○{○{○{•}}}     (nested concepts — multi-level scope)
```

**What this means:** There is no fixed "granularity" of concepts. You can define concepts at any level of specificity. A single symbol can encompass a universe or a quark.

### Axiom 4: Orthogonal Consistency
> *"All right angles are equal to one another."* — Euclid, Postulate 4

**Syntactic rule:** Independence is a universal, consistent operation. When two relations are orthogonal (∠90°), their independence is absolute — not context-dependent.

```
PROPERTY:  If A ⊥ B (A orthogonal to B), then A ⊥ B everywhere.
           Independence does not depend on where you measure it.
```

**What this means:** The quality markers are consistent across all contexts. If two concepts are independent in one construction, they are independent in every construction. This prevents the syntactic ambiguity of natural languages where "independent" might mean different things in different sentences.

### Axiom 5: Unique Parallelism
> *"Through a point not on a line, exactly one parallel line can be drawn."* — Euclid, Postulate 5

**Syntactic rule:** For any statement and any external concept, exactly one non-intersecting analog exists.

```
PROPERTY:  Given statement S and point P not in S,
           there exists exactly ONE relation through P
           that is parallel to (never contradicts) S.
```

**What this means:** Every meaning has exactly one "structural analog" — a parallel meaning that never contradicts the original. This is the basis for **analogy** in UL: parallel lines = parallel meanings. The parallel postulate guarantees that analogy is unique (not many-to-many) in Euclidean meaning-space.

**Note:** Non-Euclidean extensions (hyperbolic, elliptic) modify this axiom and would produce meaning-spaces with different analogy properties. This is an open research direction.

---

## III. THE 11 SYNTACTIC OPERATIONS (From Σ_UL)

Each operation has a **type signature** (what goes in, what comes out) and a **geometric realization** (how it looks).

### 3.1 predicate: e × r × e → a
**"Combine subject, relation, and object into a complete statement."**

```
INPUT:  Entity₁ (subject), Relation, Entity₂ (object)
OUTPUT: Assertion (complete sentence)

GEOMETRIC REALIZATION:
  ┌─────────────────────────────┐
  │                             │
  │   [E₁] ──r──→ [E₂]        │
  │                             │
  └─────────────────────────────┘

  Place E₁ and E₂ in a sentence frame,
  connected by directed relation r.
  The frame encloses a complete thought.
```

**Examples:**
```
  ┌───────────────┐
  │  • ──→ •      │   "Something acts on something"
  └───────────────┘

  ┌───────────────┐
  │  △ ──∠60°──○  │   "A fundamental thing harmonizes with a universal thing"
  └───────────────┘

  ┌───────────────┐
  │  ⬠ ──→ □     │   "A living thing acts on a structured thing"
  └───────────────┘
```

### 3.2 modify_entity: m × e → e
**"Apply a quality/transformation to an entity."**

```
INPUT:  Modifier, Entity
OUTPUT: Modified Entity

GEOMETRIC REALIZATION:
  Apply geometric transformation to entity-glyph.
  - Scaling: changes emphasis/magnitude
  - Rotation: changes perspective
  - Projection: changes abstraction level
```

**Examples:**
```
  Scale(2×, •) = •⁺       "the significant thing" (emphasized existence)
  Scale(½×, □) = □⁻       "a minor structure" (diminished structure)
  Rotate(90°, △) = △₉₀    "the fundamental, from a perpendicular view"
```

### 3.3 modify_relation: m × r → r
**"Apply a quality/transformation to a relation."**

```
INPUT:  Modifier, Relation
OUTPUT: Modified Relation

GEOMETRIC REALIZATION:
  - Scaling: changes intensity of relation
  - Rotation: changes character
  - Reflection: reverses valence
```

**Examples:**
```
  Scale(2×, →) = ═→       "strong action" (intensified relation)
  Scale(½×, →) = ╌→       "weak action" (attenuated relation)
  Rotate(θ, →) = →_θ      "action with quality θ"
```

### 3.4 negate: a → a
**"Negate a statement."**

```
INPUT:  Assertion
OUTPUT: Negated Assertion

GEOMETRIC REALIZATION:
  Reflect the entire sentence-frame across its vertical axis.
  
  ORIGINAL:                      NEGATED:
  ┌───────────────┐              ┌───────────────┐
  │  △ ──→ ○      │    REFLECT   │      ○ ←── △  │
  └───────────────┘     ══►      └───────────────┘
  "fundamental acts               "fundamental does NOT
   on universal"                   act on universal"
```

**Properties:**
- negate(negate(a)) = a (reflection is involutory — double negation = identity) ✓
- negate is total (every assertion can be negated) ✓

### 3.5 conjoin: a × a → a
**"Combine two statements with AND."**

```
INPUT:  Assertion₁, Assertion₂
OUTPUT: Conjoined Assertion

GEOMETRIC REALIZATION:
  Overlap two sentence-frames. The shared boundary = shared context = AND.
  
  ┌────────────┬────────────┐
  │  A₁        │    A₂      │
  │     (shared region)      │
  └────────────┴────────────┘
  "A₁ AND A₂"
```

### 3.6 disjoin: a × a → a
**"Combine two statements with OR."**

```
INPUT:  Assertion₁, Assertion₂
OUTPUT: Disjoined Assertion

GEOMETRIC REALIZATION:
  Place two sentence-frames adjacent (touching but not overlapping).
  Either frame can be read. Touching = disjunction.
  
  ┌────────────┐┌────────────┐
  │     A₁     ││     A₂     │
  └────────────┘└────────────┘
  "A₁ OR A₂"
```

### 3.7 embed: a → e
**"Turn a statement into an entity (nominalization)."**

```
INPUT:  Assertion
OUTPUT: Entity (the assertion treated as a thing)

GEOMETRIC REALIZATION:
  Shrink the sentence-frame and place it inside an enclosure.
  A statement becomes a thing that can be talked about.
  
  BEFORE (assertion):           AFTER (entity):
  ┌───────────────┐             ○{┌──────┐}
  │  △ ──→ ○      │     →→      │ │△──→○│ 
  └───────────────┘             ○{└──────┘}
  "fundamental acts              "the fact that fundamental
   on universal"                  acts on universal"
```

**This is nominalization:** "It rains" → "the fact that it rains" → usable as subject/object.

### 3.8 abstract: e → m
**"Turn an entity into a modifier (adjectivalization)."**

```
INPUT:  Entity
OUTPUT: Modifier (the entity's properties as a quality)

GEOMETRIC REALIZATION:
  Extract the boundary/shape properties of the entity-glyph.
  Produce a transformation that imposes those properties on other entities.
  
  "wood" (entity: □{⬠}) → "wooden" (modifier: the quality of being structured-organic)
  △ (fundamental entity) → △-ness (the quality of being fundamental)
```

### 3.9 compose: r × r → r
**"Chain two relations (transitivity)."**

```
INPUT:  Relation₁, Relation₂
OUTPUT: Composed Relation

GEOMETRIC REALIZATION:
  Concatenate directed connections. Endpoint of r₁ = startpoint of r₂.
  
  •──r₁──→•──r₂──→•
  
  compose(r₁, r₂) = •──r₁∘r₂──→→•
  
  "father" ∘ "father" = "grandfather"
  → ∘ → = →→ (double action)
```

### 3.10 invert: r → r
**"Reverse a relation."**

```
INPUT:  Relation
OUTPUT: Inverted Relation

GEOMETRIC REALIZATION:
  Reverse the direction of the directed connection.
  
  •──→•    becomes    •←──•
  
  "A acts on B"  →  "B is acted upon by A"
  "A causes B"   →  "B is caused by A"
```

### 3.11 quantify: m × e → a
**"Apply a quantifier-modifier to an entity to make an assertion."**

```
INPUT:  Modifier (quantifier type), Entity
OUTPUT: Assertion (quantified statement)

GEOMETRIC REALIZATION:
  Scale entity relative to the glyph space boundary:
  
  UNIVERSAL (∀):     • scaled to fill GS    "ALL things"
  EXISTENTIAL (∃):   • small, off-center     "SOME thing"
  NEGATIVE (¬∃):     reflected + complement   "NO thing"
```

---

## IV. SENTENCE TYPES

### Atomic Sentence (Minimum Construction)
The minimum sentence has THREE elements — analogous to the triangle as the minimum polygon:

```
  ┌───────────────┐
  │  [S] ──r──→ [P] │
  └───────────────┘
  Subject + Relation + Predicate
```

### Compound Sentence (Multiple Frames)

| Structure | Geometric form | Logical meaning |
|-----------|---------------|-----------------|
| Conjunction | Overlapping frames | "A AND B" |
| Disjunction | Adjacent frames | "A OR B" |
| Subordination | Nested frames | "A BECAUSE B" / "A IN WHICH B" |
| Sequence | Left-to-right frames | "A THEN B" |
| Conditional | Frame A inside Frame B | "IF A THEN B" |
| Contrast | Frames connected by ∠180° line | "A BUT B" (opposition) |

### Recursive Sentence (Self-Embedding)

Via the `embed` operation, any sentence can become an entity inside another sentence:

```
  ┌──────────────────────────────┐
  │  ○{┌────────┐} ──→ △        │
  │    │• ──→ • │                │
  │    └────────┘                │
  └──────────────────────────────┘
  "The fact that [something acts on something] acts on a fundamental thing"
```

Recursion depth is theoretically unlimited. Practical legibility decreases with nesting level (fractal self-similarity at decreasing scales).

---

## V. WELL-FORMEDNESS RULES

### A sentence is WELL-FORMED if and only if:

1. **Every relation connects exactly two entities** (Axiom 1)
2. **Every enclosure has a closed boundary** (Jordan Curve Theorem)
3. **Every assertion is contained in a sentence frame** (completeness)
4. **Operations respect sort constraints:**
   - predicate takes (e, r, e) — not (r, e, r)
   - negate takes (a) — not (e) or (r)
   - embed takes (a) → e — assertion becomes entity
   - abstract takes (e) → m — entity becomes modifier
5. **No free-floating relations** — every line has endpoints
6. **No open enclosures** — every boundary is closed

### A sentence is ILL-FORMED if:

```
INVALID:  ──→        (relation with no entities — free-floating line)
INVALID:  • ∠ •      (angle requires two RELATIONS, not two entities)
INVALID:  ┌──── •     (open enclosure — boundary not closed)
INVALID:  negate(•)   (negation of entity — sort mismatch: negate takes assertion, not entity)
```

---

## VI. TRANSFORMATION RULES (How Sentences Are Modified)

### Meaning-Preserving Transformations (Isometries)

These change the form but not the meaning:

| Transformation | Effect | Linguistic Analog |
|---------------|--------|-------------------|
| Translation | Move entire construction | Change tense |
| Rotation | Turn entire construction | Change perspective/voice |
| Reflection | Mirror entire construction | Negate |

### Meaning-Scaling Transformations (Similarities)

These change magnitude but preserve structure:

| Transformation | Effect | Linguistic Analog |
|---------------|--------|-------------------|
| Scale up | Enlarge construction | Intensify / emphasize |
| Scale down | Shrink construction | Diminish / de-emphasize |

### Meaning-Abstracting Transformations (Projections)

These change the level of description:

| Transformation | Effect | Linguistic Analog |
|---------------|--------|-------------------|
| Projection (collapse dim) | Remove a dimension | Generalize: "this dog" → "dogs" → "animals" |
| Section (intersect plane) | Fix a dimension | Specify: "animals" → "dogs" → "this dog" |

---

## VII. PARSING ALGORITHM

To read a UL construction:

```
PARSE(construction):
  1. IDENTIFY FRAMES: Find all sentence-frame boundaries (outermost first)
  2. FOR EACH frame, outside → inside:
     a. IDENTIFY ENCLOSURES within the frame (concepts)
     b. IDENTIFY CONNECTIONS between enclosures (relations)
     c. READ ANGLES on connections (qualities)
     d. READ CURVES (processes)
     e. IDENTIFY TRANSFORMATIONS applied to base symbols
     f. READ NESTING (recursive sub-constructions)
  3. COMPOSE: Build meaning bottom-up from atomic to compound
  4. RETURN: Complete semantic interpretation
```

**Reading order:** Structural (enclosures → connections → details), not linear (left-to-right).

This mirrors reading a geometric diagram — from the largest features to the smallest details.

---

## VIII. COMPLETE OPERATION REFERENCE TABLE

| Operation | Signature | Geometric Act | Sort Flow |
|-----------|-----------|--------------|-----------|
| predicate | e × r × e → a | Place entities, connect by relation, enclose in frame | Entity + Relation → Assertion |
| modify_entity | m × e → e | Transform entity glyph | Modifier + Entity → Entity |
| modify_relation | m × r → r | Transform relation glyph | Modifier + Relation → Relation |
| negate | a → a | Reflect sentence frame | Assertion → Assertion |
| conjoin | a × a → a | Overlap sentence frames | Assertion² → Assertion |
| disjoin | a × a → a | Adjoin sentence frames | Assertion² → Assertion |
| embed | a → e | Shrink frame into enclosure | Assertion → Entity |
| abstract | e → m | Extract boundary properties | Entity → Modifier |
| compose | r × r → r | Concatenate directed connections | Relation² → Relation |
| invert | r → r | Reverse direction | Relation → Relation |
| quantify | m × e → a | Scale entity relative to GS | Modifier + Entity → Assertion |
