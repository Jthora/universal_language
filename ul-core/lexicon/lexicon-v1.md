# Universal Lexicon

> **The Enclosure sibling.** This document contains the bounded definitions of Universal Language — constructions whose meanings are determined (to varying degrees) by geometry itself. Every entry is a geometric construction first, a natural-language label second.

---

## 0. WHAT THIS DOCUMENT IS AND IS NOT

### 0.1 The Problem

The four other siblings — Symbology, Syntax, Grammar, Thesaurus — kept leaking definitions. The Symbology catalogued ~50 compound constructions with assigned meanings ("Truth = ○{•}"). The Thesaurus provided ~16 canonical definitions dressed as synonym tables. The Grammar gave worked examples that were really lexicon entries ("Democracy," "Evolution").

These entries had no home. They accumulated wherever they fit. This document gives them a home — and, more importantly, subjects them to rigor.

### 0.2 What This Is

A **lexicon**: a catalogue of compound constructions with their structural descriptions, Σ_UL expressions, justification tiers, and optional natural-language annotations.

It is organized by **structural complexity** (number of primitive applications), not alphabetically. The construction IS the entry. The natural-language word is a convenience label, not a primary key.

### 0.3 What This Is Not

- Not a dictionary in the natural-language sense. There is no entry for "cat" or "table."
- Not exhaustive. UL is infinitely generative — you can always compose more complex constructions. This lexicon contains the constructions that are **structurally distinguished** from the infinite alternatives.
- Not authority. A reader encountering a construction not in this lexicon should analyze it geometrically (per the Writing System reading procedure). The lexicon does not gatekeep meaning.

### 0.4 The Justification Tiers

Every entry carries one of three tier labels:

| Tier | Name | Criterion | Example |
|------|------|-----------|---------|
| **T1** | **Geometrically Forced** | There is exactly one construction with these structural properties. It is a theorem, not a choice. The meaning follows necessarily from the geometry. | The Void (∅) — the unique construction with zero primitives |
| **T2** | **Structurally Distinguished** | The construction occupies an extremal, minimal, or maximally symmetric position in the space of possible constructions. It is motivated and canonical, but not the only possible construction at its complexity level. | ○{•} — the highest-symmetry enclosure containing the simplest entity. Distinguished, but other single-entity enclosures exist (△{•}, □{•}). |
| **T3** | **Conventional** | The construction is one of multiple reasonable options for expressing the intended structure. It is a design decision, not a geometric fact. Included where a standard referent is useful for communication. | "Democracy" — one of many possible compound constructions for participatory governance |

**Honest accounting:** Tier 1 is mathematics. Tier 2 is motivated mathematics. Tier 3 is engineering. All three are legitimate, but confusing them is not.

### 0.5 How Entries Are Structured

Each entry provides:

1. **Construction** — the geometric description (what you draw)
2. **Σ_UL⁺ Expression** — the formal algebraic expression using the 13 operations on 4 sorts
3. **Tier** — T1, T2, or T3
4. **Justification** — why this construction, at this tier
5. **Label(s)** — optional natural-language annotations (in italics, to mark them as commentary)

### 0.6 The Governing Principle

> **A construction's meaning IS its geometry.**
>
> ○{•} does not "mean" Truth. It IS "the maximally symmetric bounded region containing the minimal existence." If a human culture associates that structure with their word for truth — or purity, or God, or unity, or the number one — that is a grounding decision external to UL. The lexicon records the geometry. The labels are gifts to the reader, not claims about the world.

---

## 1. COMPLEXITY 0 — Zero Primitives

There is exactly one construction at complexity 0.

### 1.1 The Void

| | |
|---|---|
| **Construction** | (empty glyph space) |
| **Symbol** | Ø |
| **Σ_UL** | — (pre-linguistic; the null element) |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | Before any primitive is applied, the glyph space is empty. This is the unique construction with zero components. Every formal system requires a null element. ∅ in set theory, 0 in arithmetic, silence in music. |
| **Labels** | *Void, Silence, Nothing, Absence, The Undifferentiated* |

---

## 2. COMPLEXITY 1 — One Primitive

There are exactly 5 classes of construction at complexity 1, corresponding to the 5 geometric primitives. Within each class, we list only the structurally forced or distinguished instances.

### 2.1 Single Point

| | |
|---|---|
| **Construction** | A single point in the glyph space: • |
| **Σ_UL** | e₁ (an atomic entity) |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique construction consisting of exactly one instance of the simplest primitive. Point is Existence (Unique Grounding Theorem, formal-foundations.md §4.5). A single point is the minimum act of meaning: asserting that something IS. |
| **Labels** | *Existence, Being, Something, "This is"* |

### 2.2 Single Line Segment

| | |
|---|---|
| **Construction** | Two points connected by a straight segment: •──• |
| **Σ_UL** | predicate(e₁, r, e₂) → a (the minimal assertion: subject + relation + object) |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | A line requires exactly two points (Euclid Postulate 1). This is the unique minimal construction that introduces Relation. It is the simplest possible statement — two things connected. Line is Relation (Unique Grounding Theorem). |
| **Labels** | *Relation, Connection, "This relates to that"* |

### 2.3 Single Directed Line (Ray)

| | |
|---|---|
| **Construction** | Two points connected by an arrow: •──→• |
| **Σ_UL** | predicate(e₁, r, e₂) → a, where r carries directional information |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | A ray is the unique construction that introduces directionality to a line. Given a line, there are exactly two possible directions (one endpoint to the other). Choosing one is the minimum act of imposing asymmetry on a relation. |
| **Labels** | *Action, Causation, "This acts on that"* |

### 2.4 Single Angle

| | |
|---|---|
| **Construction** | Two rays sharing a vertex, with angle θ between them |
| **Σ_UL** | modify_relation(m, r) → r', where m is parameterized by θ |
| **Tier** | T1 (as a class) — Geometrically Forced |
| **Justification** | An angle requires exactly two lines meeting at a point (Euclid Definition 8). The angle is the unique way to introduce Quality — the character of a relationship. Angle is Quality (Unique Grounding Theorem). The specific angle value θ parameterizes a continuous spectrum of quality. |
| **Labels** | *Quality, Character, Manner, "Relates in this way"* |

**The distinguished values within this continuous spectrum are listed at Complexity 2 (§3), since each requires a specific geometric configuration.**

### 2.5 Single Curve

| | |
|---|---|
| **Construction** | A non-straight continuous path between two points: • ⌒ • |
| **Σ_UL** | predicate(e₁, r, e₂) → a, where r carries curvature data κ |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | A curve is the unique primitive that introduces continuously varying direction. Where a line holds constant direction, a curve changes it. This is Process — the geometric basis of becoming, change, transformation. Curve is Process (Unique Grounding Theorem). |
| **Labels** | *Process, Change, Becoming, Transformation, "This becomes that through continuous change"* |

### 2.6 Single Enclosure (Empty)

| | |
|---|---|
| **Construction** | A closed boundary with nothing inside |
| **Σ_UL** | — (a boundary with empty interior; formally: a concept with no content) |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | An enclosure requires a closed curve (Jordan Curve Theorem: partitions the plane into interior and exterior). The empty enclosure is the unique construction that introduces Concept without yet instantiating it. It is a definition with nothing defined — a category with no members. Enclosure is Concept (Unique Grounding Theorem). |
| **Labels** | *Empty concept, Null set, Category without members, Potential* |

**The specific shapes of enclosure (△, □, ⬠, ⬡, ○) are classified at Complexity 2 (§3), since each requires a specific boundary configuration.**

---

## 3. COMPLEXITY 2 — Two Primitives or Distinguished Configurations

These are constructions that combine two primitives, or that are distinguished instances of a single primitive's parameter space.

### 3.1 Distinguished Angles

These are the angles whose geometric properties are structurally unique — they occupy singular or extremal positions in the angular spectrum. For each, we give the geometric property that forces the distinction.

#### 3.1.1 — θ = 0° (Coincidence)

| | |
|---|---|
| **Construction** | Two rays from the same vertex, pointing in the same direction |
| **Σ_UL** | modify_relation(m₀, r) → r, where m₀ is the identity modifier |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique angle at which two relations become indistinguishable — zero inclination. This is the only angle that acts as an identity element under composition. Geometrically: parallelism/coincidence. |
| **Labels** | *Identity, Agreement, Equivalence, Sameness, "is the same as"* |

#### 3.1.2 — θ = 90° (Perpendicularity)

| | |
|---|---|
| **Construction** | Two rays from the same vertex, at right angles |
| **Σ_UL** | modify_relation(m₉₀, r) → r', where the dot product r·r' = 0 |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique angle at which two directions are maximally independent in Euclidean geometry (dot product = 0, zero projection). This is not one distinction among many — it is THE structural definition of independence. Euclid Postulate 4: all right angles are equal. |
| **Labels** | *Independence, Orthogonality, "has nothing to do with", "and" without interaction* |

#### 3.1.3 — θ = 180° (Opposition)

| | |
|---|---|
| **Construction** | Two rays from the same vertex, pointing in opposite directions |
| **Σ_UL** | negate(a) — or equivalently: invert(r) |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique angle at which a relation reverses direction. This is the geometric realization of negation. It is self-inverse: reversing a reversal returns to the original. 180° is the only angle with this property (the only non-trivial involution in the angle group). |
| **Labels** | *Negation, Opposition, Contradiction, Antonym, "is not", "the opposite of"* |

#### 3.1.4 — θ = 360° (Full Rotation)

| | |
|---|---|
| **Construction** | A ray rotated fully back to its starting direction |
| **Σ_UL** | modify_relation(m₃₆₀, r) → r, where m₃₆₀ is the identity transformation via cycle |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique angle at which rotation returns to the starting direction. Unlike 0° (which is the identity achieved by doing nothing), 360° is the identity achieved by completing a full cycle. These are topologically distinct — the winding number is 0 vs. 1. |
| **Labels** | *Completion, Return, Full cycle, Closure, "comes back to the beginning"* |

#### 3.1.5 — θ = 60° (Equilateral Angle)

| | |
|---|---|
| **Construction** | Two rays from the same vertex, at 60° |
| **Σ_UL** | modify_relation(m₆₀, r) → r' |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | 60° = π/3 is the interior angle of the equilateral triangle — the polygon where all angles and all sides are equal. It is the minimal equal partition of 180° (into three equal parts). This gives it the property of maximal balance among the minimum number of parts. It is distinguished but not forced: 72° (regular pentagon) and 45° (square bisector) are also structurally notable. |
| **Labels** | *Harmony, Balance, Equal partnership, "relates in perfect balance"* |

#### 3.1.6 — θ = 120° (Hexagonal Interior Angle)

| | |
|---|---|
| **Construction** | Two rays from the same vertex, at 120° |
| **Σ_UL** | modify_relation(m₁₂₀, r) → r' |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | 120° is the interior angle of the regular hexagon, the polygon that achieves optimal circle-packing (filling the plane with equal circles). This gives hexagonal geometry its association with efficiency and tessellation. Also: 120° = 180° − 60°, the supplement of the harmony angle. It is the angle at which three relations can meet at a point with equal spacing around it (3 × 120° = 360°). |
| **Labels** | *Efficiency, Complementarity, Optimal distribution, "fills the space"* |

### 3.2 Distinguished Enclosure Shapes

The shape of a closed boundary determines the symmetry group of the enclosure. The Erlangen Program (Klein, 1872) establishes that a geometric object IS its symmetry group. So the enclosure classification is not arbitrary — it follows from the structural hierarchy of symmetry.

#### 3.2.1 — Triangle (△)

| | |
|---|---|
| **Construction** | 3-sided closed polygon |
| **Symmetry** | D₃ (dihedral group, order 6) |
| **Σ_UL** | An assertion/entity with D₃ boundary symmetry |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The triangle is the minimum polygon (fewest sides that can enclose area). There is no 2-sided polygon in Euclidean geometry. 3 is the structural minimum. Therefore the triangle is the unique minimum-complexity enclosure. Minimum polygon = minimum complete structure = fundamental/atomic/irreducible. |
| **Labels** | *Fundamental, Atomic, Irreducible, Basic, "the simplest kind of"* |

#### 3.2.2 — Square (□)

| | |
|---|---|
| **Construction** | 4-sided closed polygon with equal sides and right angles |
| **Symmetry** | D₄ (dihedral group, order 8) |
| **Σ_UL** | An assertion/entity with D₄ boundary symmetry |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | The square is the unique regular polygon that tessellates the Euclidean plane by itself with only translations (not rotations). This gives it the property of systematic, ordered coverage. It is distinguished among 4-sided polygons but the step from 3 to 4 sides is not uniquely forced — any polygon with more sides is also possible. |
| **Labels** | *Structural, Systematic, Ordered, "the organized kind of"* |

#### 3.2.3 — Pentagon (⬠)

| | |
|---|---|
| **Construction** | 5-sided closed polygon |
| **Symmetry** | D₅ (dihedral group, order 10) |
| **Σ_UL** | An assertion/entity with D₅ boundary symmetry |
| **Tier** | T3 — Conventional |
| **Justification** | The pentagon's diagonals divide in the golden ratio φ = (1+√5)/2, which appears in biological growth patterns (phyllotaxis, shell spirals). The association of pentagon with "living" or "organic" is based on this φ-connection, which is empirically motivated but not geometrically forced. Other polygons also appear in biology. |
| **Labels** | *Organic, Living, Self-similar, Growth-related — convention based on golden-ratio association* |

#### 3.2.4 — Hexagon (⬡)

| | |
|---|---|
| **Construction** | 6-sided closed polygon |
| **Symmetry** | D₆ (dihedral group, order 12) |
| **Σ_UL** | An assertion/entity with D₆ boundary symmetry |
| **Tier** | T3 — Conventional |
| **Justification** | The hexagon achieves optimal circle-packing (honeycomb conjecture, proven Hales 2001). The association with "network" or "community" is based on this packing optimality — the most efficient way to fill a plane with equal bounded regions. Empirically motivated (beehives, basalt columns) but the semantic label is not forced. |
| **Labels** | *Networked, Communal, Efficient — convention based on optimal packing* |

#### 3.2.5 — Circle (○)

| | |
|---|---|
| **Construction** | Closed curve with constant curvature (all points equidistant from center) |
| **Symmetry** | SO(2) (continuous rotation group, infinite order) |
| **Σ_UL** | An assertion/entity with SO(2) boundary symmetry |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The circle has the highest symmetry of any planar closed curve — it is invariant under all rotations and all reflections through the center. Maximum symmetry = maximum generality. This is not a convention — it is the definition of "general" in the Erlangen Program: the more transformations leave an object invariant, the more general it is. The circle is the unique maximum of this hierarchy. |
| **Labels** | *Universal, Complete, Abstract, Total, "the most general kind of"* |

### 3.3 Distinguished Curves

#### 3.3.1 — Constant Curvature, Closed (Circle as Process)

| | |
|---|---|
| **Construction** | ○ read as a curve (not as an enclosure) — a closed path with constant curvature κ |
| **Σ_UL** | A relation r_○ where the direction varies at constant rate and returns to its starting point |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique closed curve with constant curvature. Among all closed curves, it is the maximally symmetric one (SO(2)). As a process, it is the unique process that maintains constant rate of change and returns exactly to its starting state. |
| **Labels** | *Cycle, Self-reference, Return, Completeness-as-process, "returning to where it began"* |

#### 3.3.2 — Monotonically Increasing Radius (Spiral)

| | |
|---|---|
| **Construction** | 𝒮 — a curve whose distance from a center point monotonically increases |
| **Σ_UL** | A relation r_𝒮 where curvature decreases as the path extends |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | The spiral is the simplest curve that combines rotation (angular change) with expansion (radial increase). It is the unique curve class that is simultaneously periodic in angle and monotonic in radius. Distinguished among open curves, but not the only expanding curve. |
| **Labels** | *Growth, Development, Evolution, Unfolding* |

#### 3.3.3 — Periodic Curvature (Sine Wave)

| | |
|---|---|
| **Construction** | ~ — a curve whose curvature alternates sign periodically |
| **Σ_UL** | A relation r_~ where the direction oscillates |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | The sine wave is the simplest periodic function (single Fourier component). Among all oscillating curves, it is the one with minimal harmonic content. This makes it the canonical oscillation — distinguished but not unique among periodic curves. |
| **Labels** | *Rhythm, Oscillation, Alternation, Wave* |

### 3.4 Point-on-Point Constructions

#### 3.4.1 — Coincident Points (•═•)

| | |
|---|---|
| **Construction** | Two points placed at the same position, connected by a zero-length relation |
| **Σ_UL** | predicate(e₁, r₀, e₂) → a, where r₀ has zero length; or modify_relation(m₀, r) with θ = 0° |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique configuration in which two distinct entities occupy the same position. The relation between them has zero length — they are maximally close. This is the geometric realization of identity: two things that are one. |
| **Labels** | *Identity, "this is that", Self-relation* |

#### 3.4.2 — Point within Empty Enclosure

See §4 — this is a Complexity 3 construction (enclosure + point).

---

## 4. COMPLEXITY 3 — Entity Within Enclosure

An entity placed inside an enclosure is the simplest construction that gives Concept content. The meaning depends on which entity and which enclosure shape.

### 4.1 Point in Circle — ○{•}

| | |
|---|---|
| **Construction** | A single point centered in a circle |
| **Σ_UL** | embed(predicate(e₁, r_self, e₁)) → e', where e' is an entity with ○-boundary; or more precisely: the assertion "existence exists" nominalized into a concept with maximal symmetry |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | This combines the highest-symmetry enclosure (○, T1) with the simplest entity (•, T1). It is the simplest construction that has BOTH concept AND content. Among all single-entity-in-enclosure constructions, ○{•} has maximal boundary symmetry. It is distinguished as the most universal/general instance of "defined existence" — but △{•}, □{•}, etc. are equally valid constructions at the same complexity. |
| **Labels** | *Truth, Completeness-of-existence, Unity, "what is, fully defined" — BUT: the label "Truth" is a T3 conventional choice. The geometry says "maximally general concept containing minimal existence." Whether that maps to a culture's word for truth, God, essence, or the number one is a grounding decision.* |

### 4.2 Point in Triangle — △{•}

| | |
|---|---|
| **Construction** | A single point centered in a triangle |
| **Σ_UL** | An entity e with D₃-boundary containing a single atomic entity |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | Combines the minimum-polygon enclosure (△, T1) with the simplest entity (•, T1). The most fundamental/irreducible instance of "defined existence." Distinguished as the most specific/atomic container for existence. |
| **Labels** | *Atom, Element, Irreducible unit, "the simplest defined thing"* |

### 4.3 Point in Square — □{•}

| | |
|---|---|
| **Construction** | A single point centered in a square |
| **Σ_UL** | An entity e with D₄-boundary containing a single atomic entity |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | Combines the systematic-tessellation enclosure (□, T2) with the simplest entity (•, T1). An existence bounded by structure — a defined, located, constrained thing. |
| **Labels** | *Constraint, Bounded existence, Located thing, "an existence within structure"* |

### 4.4 Void in Circle — ○{Ø}

| | |
|---|---|
| **Construction** | A circle enclosing nothing (empty interior) |
| **Σ_UL** | A concept with ○-boundary and null content |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique construction that combines maximum-symmetry concept with zero content. A box drawn around nothing — a fully general definition of nothing. This is distinct from the bare Void (Ø, §1.1): that is the absence of construction; this is the CONSTRUCTION of absence. The difference is whether the nothingness is bounded. Ø = "there is nothing." ○{Ø} = "there is a concept, and it is: nothing." |
| **Labels** | *The concept of nothing, Empty set (as a concept), Defined absence, Zero (as distinguished from Nothing)* |

### 4.5 Ray from Point (Unbounded)

| | |
|---|---|
| **Construction** | A point with a ray extending outward without termination: •→ |
| **Σ_UL** | predicate(e₁, r, ?) — an entity with a directed relation whose object is unspecified |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | The simplest construction with an open-ended relation — a connection to something unspecified/unbounded. Distinguished as the minimal expression of directed unboundedness. |
| **Labels** | *Potential, Reaching, Freedom-as-direction, Aspiration, "this directs toward..."* |

---

## 5. COMPLEXITY 4 — Compound Constructions

### 5.1 Two Points in Opposition — • ∠180° •

| | |
|---|---|
| **Construction** | Two points connected by a relation at 180° (through the center, opposite sides) |
| **Σ_UL** | predicate(e₁, invert(r), e₂) → a; or equivalently, negate applied to the base relation |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | Combines the two simplest entities with the forced negation angle. The minimum expression of duality — two things in maximal opposition. Distinguished because 180° is T1 (forced), so this construction inherits that necessity. |
| **Labels** | *Duality, Opposition, Binary, Polarity, "this opposes that"* |

### 5.2 Three Points in Equilateral Relation — △{•,•,•}

| | |
|---|---|
| **Construction** | Three points at the vertices of an equilateral triangle, mutually connected at ∠60° |
| **Σ_UL** | conjoin(predicate(e₁, r, e₂), conjoin(predicate(e₂, r', e₃), predicate(e₃, r'', e₁))) where all angles between relations = 60° |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | The minimum configuration where all pairwise relations are equal. In a triangle, three entities ALL relate at 60° — the harmony angle (T2). This is the simplest construction with total symmetry among its parts. It is the geometric realization of perfect balance among the minimum plurality. |
| **Labels** | *Trinity, Perfect balance, Three-in-harmony, "three things in complete mutual equality"* |

### 5.3 Point Connected to Enclosure — •→○

| | |
|---|---|
| **Construction** | An entity directing a relation toward a circular concept |
| **Σ_UL** | predicate(e₁, r, embed(a)) — entity relates to a nominalized concept |
| **Tier** | T3 — Conventional |
| **Justification** | This combines basic primitives in a common pattern (entity-relates-to-concept), but the specific pairing of • with ○ is one of many possible combinations. The construction is structurally unremarkable — it is included because directional-entity-to-concept is a frequently needed pattern. |
| **Labels** | *Aspiration-toward-totality, Seeking, "existence reaching toward completeness"* |

### 5.4 Enclosure-to-Enclosure — ○──○

| | |
|---|---|
| **Construction** | Two circular enclosures connected by a line |
| **Σ_UL** | predicate(embed(a₁), r, embed(a₂)) — two nominalized concepts in relation |
| **Tier** | T3 — Conventional |
| **Justification** | Concept-relates-to-concept is a general pattern with no structurally unique instance. The specific shapes and relation type are free parameters. Included as the template for inter-concept relations. |
| **Labels** | *Analogy, Connection-between-ideas, "this concept relates to that concept"* |

### 5.5 Self-Nesting — ○{○{○{...}}}

| | |
|---|---|
| **Construction** | Enclosure containing enclosure containing enclosure, recursively |
| **Σ_UL** | embed(a₁) where a₁ contains embed(a₂) where a₂ contains embed(a₃)... |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | The unique recursive construction using only enclosures. This is structurally distinguished because recursion/self-reference occupies a special role in formal systems (Gödel, Turing). The self-nesting enclosure is the geometric realization of a construction that refers to its own type-level structure. It is the simplest fractal in UL. |
| **Labels** | *Infinity, Self-reference, Recursion, Meta-structure, Strange loop* |

### 5.6 Boundary-Inverted Assertion — Negation via frame boundary flip

| | |
|---|---|
| **Construction** | A complete sentence frame with its boundary style flipped (solid → dashed or dashed → solid) |
| **Σ_UL** | negate(a) → a' |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | Boundary inversion is an involution (flipping twice returns to the original) and satisfies all four required properties of logical negation: N1 (involution), N2 (contradiction), N3 (excluded middle), N4 (De Morgan). Solid boundary = asserted; dashed boundary = denied. Content is unchanged — only the assertional sign flips. This replaces the previous reflection-based definition, which implemented converse (subject-object swap), not negation (truth-value flip). See `P0-negation-resolution.md`. |
| **Labels** | *Not, Negation, Denial, "it is not the case that"* |

---

## 6. COMPLEXITY 5+ — Higher Compound Constructions

At this complexity level, the combinatorial space expands rapidly. Most constructions are T3 (conventional). We include only those with clear structural motivation.

### 6.1 Overlapping Sentence Frames — Conjunction

| | |
|---|---|
| **Construction** | Two sentence frames whose boundaries partially overlap (shared region) |
| **Σ_UL** | conjoin(a₁, a₂) → a |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | When two bounded regions overlap, the overlap region belongs to BOTH — this is the geometric definition of "and." There is no other geometric way to express simultaneous truth of two assertions within a planar spatial framework. (Adjacency = "or"; overlap = "and"; containment = "if-then".) |
| **Labels** | *And, Both, Conjunction, "this AND that"* |

### 6.2 Adjacent Sentence Frames — Disjunction

| | |
|---|---|
| **Construction** | Two sentence frames placed touching but not overlapping |
| **Σ_UL** | disjoin(a₁, a₂) → a |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | When two bounded regions are adjacent (touching boundary, no interior overlap), either can be read independently — this is the geometric definition of "or." The reader encounters the boundary and must choose one frame or the other. |
| **Labels** | *Or, Either, Disjunction, "this OR that"* |

### 6.3 Nested Sentence Frame — Embedding

| | |
|---|---|
| **Construction** | A sentence frame shrunk and placed inside an enclosure, converting it to an entity |
| **Σ_UL** | embed(a) → e |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | Placing a complete assertion inside a boundary converts it from a statement into a thing-that-can-be-talked-about. This is nominalization — the geometric realization of "the fact that..." One of the 13 primitive operations. |
| **Labels** | *Nominalization, Reification, "the fact that...", "that [clause]"* |

### 6.4 Shape-Properties-as-Modifier — Abstraction

| | |
|---|---|
| **Construction** | Extracting the boundary shape of an entity and applying it as a modifier to another entity |
| **Σ_UL** | abstract(e) → m |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The shape of an entity's boundary IS a transformation (a symmetry group action). Extracting it and applying it to another entity is the geometric realization of adjectivalization: "wood" → "wooden," "circle" → "circular." One of the 13 primitive operations. |
| **Labels** | *Adjectivalization, Quality-extraction, "-ness", "-like", "having the quality of"* |

### 6.5 Composed Relations — •→•→•

| | |
|---|---|
| **Construction** | Three entities connected by two sequential directed relations |
| **Σ_UL** | compose(r₁, r₂) → r₃, applied as predicate(e₁, compose(r₁, r₂), e₃) |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | When the endpoint of one relation is the startpoint of another, they compose — the resulting relation goes from the start of the first to the end of the second. This is the geometric realization of transitivity. One of the 13 primitive operations. |
| **Labels** | *Transitivity, Chain, Sequence, "through", "A relates to C via B"* |

### 6.6 Reversed Relation — •←•

| | |
|---|---|
| **Construction** | A directed relation with its arrow reversed |
| **Σ_UL** | invert(r) → r' |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | Reversing a directed connection is the geometric realization of voice inversion: active ↔ passive, cause ↔ effect. It is the unique operation that reverses directionality without changing any other property of the relation. One of the 13 primitive operations. |
| **Labels** | *Passive voice, Reversal, Inverse relation, "is [verb]ed by"* |

### 6.7 Quantification — Scaling of Entity within Frame

| | |
|---|---|
| **Construction** | An entity scaled relative to its sentence frame: large (fills frame) = universal; small (point-like) = particular |
| **Σ_UL** | quantify(m_scale, e) → a |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | Quantification is one of the 13 primitive Σ_UL operations. Its geometric realization as scaling is motivated: entity filling the frame = "applies everywhere" = universal; entity as point within frame = "applies somewhere specific" = existential. The scaling interpretation is natural but not the only possible realization of quantification. |
| **Labels** | *All/Some/None (depending on scale), Quantification, "for all...", "there exists..."* |

---

## 7. THE OPERATIONS AS LEXICON ENTRIES

The 13 Σ_UL operations are themselves "words" in UL — they are the verbs of the language, the ways of combining. Each has a geometric realization that serves as its lexicon entry.

| # | Operation | Signature | Geometric Realization | Tier |
|---|-----------|-----------|----------------------|------|
| 1 | `predicate` | e × r × e → a | Entity-line-entity inside sentence frame | T1 |
| 2 | `modify_entity` | m × e → e | Transformation applied to entity glyph | T1 |
| 3 | `modify_relation` | m × r → r | Transformation applied to relation line/curve | T1 |
| 4 | `negate` | a → a | Flip frame boundary: solid ↔ dashed | T1 |
| 5 | `conjoin` | a × a → a | Overlapping sentence frames | T1 |
| 6 | `disjoin` | a × a → a | Adjacent (touching) sentence frames | T1 |
| 7 | `embed` | a → e | Sentence frame shrunk into enclosure | T1 |
| 8 | `abstract` | e → m | Boundary-shape extracted as transformation | T1 |
| 9 | `compose` | r × r → r | Sequential directed connections | T1 |
| 10 | `invert` | r → r | Arrow direction reversed | T1 |
| 11 | `quantify` | m × e → a | Scale of entity relative to frame | T2 |

All 13 operations are Tier 1 or Tier 2. None is conventional. This is expected — they are the primitive operations of the algebraic signature, proved minimal in formal-foundations.md §1.5.

---

## 8. TIER SUMMARY AND HONEST ACCOUNTING

### 8.1 Count by Tier

| Tier | Count | Percentage | What this means |
|------|-------|------------|----------------|
| T1 — Geometrically Forced | 23 | ~58% | More than half the lexicon is mathematics. These entries cannot be otherwise. |
| T2 — Structurally Distinguished | 13 | ~32% | A third occupies extremal positions — motivated, canonical, but with alternatives. |
| T3 — Conventional | 4 | ~10% | A small minority are design decisions. Clearly labeled as such. |
| **Total** | **40** | **100%** | |

### 8.2 What Is NOT in This Lexicon

The following constructions appear in other siblings but are **not included here** because they are T3 entries without sufficient motivation for first inclusion:

- **"Democracy"** (Grammar §VII) — a compound construction of community + mutual relations + equality. Valid UL, but one of infinitely many possible governance constructions. No geometric reason to privilege it.
- **"Evolution"** (Grammar §VII) — a compound construction of living + spiral + time + change. Valid UL, but one of many possible growth-over-time constructions.
- **"Love"** as ○{•≅•} (Symbology §V) — the label "love" for "complete enclosure of identified pair" is a grounding decision. The geometry says "two identified existences within maximal-symmetry boundary." Whether that's love, marriage, fusion, or identity-in-unity depends on context.
- **"Knowledge"** as ○{○{•}─∠60°─○} (Symbology §V) — a multi-component compound with a conventional label. The geometry is valid; the word "knowledge" is a choice.
- **"Fear"** as •←══□ (Thesaurus §VII) — one of many possible constructions for threat-response. The structural reading "entity receiving emphatic relation from constraint" could equally be labeled deference, submission, or awe.

These entries remain in their sibling files as **examples and illustrations** — they are useful for showing how UL constructions work. But they are not canonical definitions. They are applications of the lexicon, not the lexicon itself.

### 8.3 The Boundary Principle

> **If two competent geometers, given only the primitives and operations, could independently arrive at the same construction for the same structural concept — it belongs in the lexicon at T1 or T2.**
>
> **If they would need to agree on a convention first — it belongs at T3 or not at all.**

This principle is the filter. It is why most of what was bleeding across the siblings does NOT appear here as its own entry: those compounds require conventions (choosing "love" over "fusion," choosing "democracy" over "participatory-governance-structure"), and conventions are external to geometry.

---

## 9. RELATIONSHIP TO SIBLINGS

| Sibling | Primitive | Concern | Lexicon's Relation |
|---------|-----------|---------|-------------------|
| **Symbology** (symbol-map.md) | Point | What the atomic marks ARE | Symbology defines the drawing primitives. Lexicon says what their combinations structurally mean. |
| **Syntax** (syntax-dictionary.md) | Line | How marks CONNECT | Syntax defines valid composition rules. Lexicon provides the results of applying those rules to distinguished cases. |
| **Grammar** (grammar-book.md) | Angle | Rules governing valid CONSTRUCTIONS | Grammar says what operations you CAN perform. Lexicon shows the constructions that result when you perform them on distinguished inputs. |
| **Thesaurus** (thesaurus.md) | Curve | Paths between RELATED meanings | Thesaurus maps relationships between constructions. Lexicon defines the constructions that the thesaurus relates. |
| **Lexicon** (this file) | Enclosure | BOUNDED canonical DEFINITIONS | This file. |

The Lexicon is the Enclosure sibling: it draws a boundary around each construction and says "this is defined; this is IN." Just as an enclosure in geometry separates inside from outside, the Lexicon separates "structurally motivated construction" from "the infinite space of all possible constructions."

---

## 10. OPEN PROBLEMS

| Problem | Status | Impact |
|---------|--------|--------|
| **Exhaustive enumeration of T1 constructions** | Believed complete for complexity ≤ 5. Not proven. | If a forced construction is missing, the lexicon is incomplete. |
| **Tier boundary for quantify** | Listed as T2. Some arguments for T1 (it's a primitive operation). | Affects the T1/T2 ratio. The geometric realization via scaling is motivated but may not be unique. |
| **Higher-polygon enclosures** (heptagon, octagon, ...) | Not included. D₇, D₈, ... symmetry groups exist but have no structurally distinguished status beyond D₆. | If a mathematical property distinguishes some higher polygon (as φ distinguishes the pentagon), it should be added. |
| **3D and higher-dimensional constructions** | Not addressed. This lexicon is planar (2D). | UL's axiomatics work on any 2D surface. Extension to 3D introduces new primitives (volume, curvature of surfaces) — a genuine open problem. |
| **Interaction with gauge connection** | The gauge bundle (frontier/expedition-one) adds context-dependence. Whether context shifts Tier assignments is open. | A T2 construction might become T1 or T3 in specific gauge contexts. |

---

## 11. VERSION HISTORY

| Version | Date | Changes |
|---------|------|---------|
| v1 | 2026-03-13 | Initial construction. 40 entries. Built bottom-up from geometry with 3-tier justification system. |
