# Universal Lexicon

> **The Enclosure sibling.** This document contains the bounded definitions of Universal Language — constructions whose meanings are determined (to varying degrees) by geometry itself. Every entry is a geometric construction first, a natural-language label second.

### How This Document Relates to Its Siblings

| If you need... | Consult | Why |
|----------------|---------|-----|
| What the atomic symbols look like | **Symbology** (symbology/symbol-map.md) | Symbology defines the drawing primitives that Lexicon entries compose |
| Rules for valid combinations | **Syntax** (syntax-dictionary.md) | Syntax defines which compositions are well-formed |
| Why meanings emerge from geometry | **Grammar** (grammar-book.md) | Grammar provides the semantic theory; Lexicon records the results |
| Synonyms and related meanings | **Thesaurus** (thesaurus/thesaurus.md) | Thesaurus maps relationships between the entries defined here |
| Practical writing and reading | **Writer's Companion** (writing-system/writers-companion.md) | How to draw and parse Lexicon entries on paper |

> **This document is normative.** Where Symbology, Grammar, or Thesaurus provide constructions with natural-language labels (e.g., "Truth = ○{•}"), those are illustrative. Only entries in this Lexicon with tier justifications (T1/T2/T3) are canonical. See §8.3 for what is excluded and why.

---

## 0. WHAT THIS DOCUMENT IS AND IS NOT

### 0.1 The Problem

The four other siblings — Symbology, Syntax, Grammar, Thesaurus — kept leaking definitions. The Symbology catalogued ~50 compound constructions with assigned meanings ("Truth = ○{•}"). The Thesaurus provided ~16 canonical definitions dressed as synonym tables. The Grammar gave worked examples that were really lexicon entries ("Democracy," "Evolution").

These entries had no home. They accumulated wherever they fit. This document gives them a home — and, more importantly, subjects them to rigor.

### 0.2 What This Is

A **lexicon**: a catalogue of compound constructions with their structural descriptions, Σ_UL expressions, justification tiers, and optional natural-language annotations.

It is organized by **constructive level** (see §0.7) — defined by the geometric construction steps required — not alphabetically. The construction IS the entry. The natural-language word is a convenience label, not a primary key.

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
2. **Σ_UL** — the formal algebraic expression using the 11 operations on 4 sorts, OR "—" when the construction has no clean single-term Σ_UL expression (see §0.8)
3. **Geometric** — *(only when Σ_UL is "—")* the geometric characterization that stands in place of a formal expression
4. **Tier** — T1, T2, or T3
5. **Justification** — why this construction, at this tier
6. **Labels** — optional natural-language annotations (in italics, to mark them as commentary)

### 0.6 The Governing Principle

> **A construction's meaning IS its geometry.**
>
> ○{•} does not "mean" Truth. It IS "the maximally symmetric bounded region containing the minimal existence." If a human culture associates that structure with their word for truth — or purity, or God, or unity, or the number one — that is a grounding decision external to UL. The lexicon records the geometry. The labels are gifts to the reader, not claims about the world.

### 0.7 Constructive Level Definitions

Entries are organized by **constructive level** — the minimum number of geometric construction steps required to produce the construction from the empty glyph space.

| Level | Name | Definition | Examples |
|-------|------|------------|----------|
| **0** | Pre-Constructive | Zero primitives applied. The glyph space before any construction. | Void |
| **1** | Atomic Carriers | Exactly one instance of one of the 5 geometric primitives (as a generic class, not parameterized). | Single point, single line, single angle (generic), single curve (generic), single empty enclosure |
| **2** | Distinguished Parameters and Single Operations | A single primitive with a specific parameter value that is structurally forced or distinguished, OR a single Σ_UL operation applied to atomic inputs. | 0° angle, 90° angle, circular enclosure, modify_entity, modify_relation |
| **3** | Two-Primitive Combinations | Two distinct primitives combined in a single construction. | Point in circle (○{•}), point in triangle (△{•}), unbounded ray (•→) |
| **4** | Multi-Primitive Compositions | Three or more primitives, or two or more operations, combined. | Trinity (△{•,•,•}), concept-to-concept (○──○), self-nesting (○{○{...}}) |
| **5+** | Higher Compositions | Constructions requiring complex setup of multiple primitives and operations. | Conjunction (overlapping frames), quantification (scaled entity in frame) |

**Note:** The construction space is a partial order (a DAG), not a linear sequence. Items at the same level are not necessarily comparable. Level assignment reflects minimum construction steps, not unique ranking.

### 0.8 On Geometric vs. Algebraic Expressions

The geometric system **G** (defined in formal-foundations.md §2.1) and the algebraic signature **Σ_UL** (§1.5) are different algebras connected by a structure-preserving homomorphism. They are NOT identical. This has consequences for lexicon entries:

- **Some geometric constructions have clean Σ_UL expressions.** Example: two entities connected by a directed relation = `predicate(e₁, r, e₂) → a`. These entries carry a **Σ_UL** expression.

- **Some geometric constructions have no single Σ_UL term.** Example: the specific *shape* of an enclosure boundary (triangle vs. circle) is a continuous geometric property. It enters Σ_UL indirectly through the modifier sort (via `abstract()`), but the shape itself is geometric data, not an algebraic term. These entries carry **Σ_UL: —** with a **Geometric** explanation.

- **Some geometric constructions realize multiple sort-distinct operations.** Example: 180° rotation is ONE geometric angle but realizes THREE algebraically distinct operations on THREE different sorts (see §3.1.3). The lexicon entry must distinguish these carefully.

The Unique Grounding Theorem (formal-foundations.md §4) guarantees that G embeds into any Σ_UL-algebra. But the embedding may require multiple operations and parameter choices — not every geometric form maps to a single algebraic term.

---

## 1. CONSTRUCTIVE LEVEL 0 — Pre-Constructive

There is exactly one construction at Level 0.

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

## 2. CONSTRUCTIVE LEVEL 1 — Atomic Carriers

There are exactly 5 classes of construction at Level 1, corresponding to the 5 geometric primitives. Within each class, we list the generic (unparameterized) form. One additional entry (Ray) captures the concept of oriented line.

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
| **Justification** | A line requires exactly two points (Euclid Postulate 1). This is the unique minimal construction that introduces Relation. It is the simplest possible statement — two things connected. Line is Relation (Unique Grounding Theorem). This entry is also the geometric realization of the `predicate` operation — the most fundamental of the 11 Σ_UL operations. |
| **Labels** | *Relation, Connection, "This relates to that"* |

### 2.3 Single Directed Line (Ray)

| | |
|---|---|
| **Construction** | Two points connected by an arrow: •──→• |
| **Σ_UL** | predicate(e₁, r, e₂) → a, where r carries an orientation parameter |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | Given a line, there are exactly two possible orientations (one endpoint to the other). The concept of directed relation is geometrically forced — every asymmetric relation must pick a direction. But a specific direction choice is a selection from two equally valid options, not a geometric theorem. The ray is structurally distinguished as the minimal directed connection, occupying a canonical position between undirected line and fully parameterized relation. |
| **Labels** | *Action, Causation, "This acts on that"* |

### 2.4 Single Angle

| | |
|---|---|
| **Construction** | Two rays sharing a vertex, with angle θ between them |
| **Σ_UL** | modify_relation(m, r) → r', where m is parameterized by θ |
| **Tier** | T1 (as a class) — Geometrically Forced |
| **Justification** | An angle requires exactly two lines meeting at a point (Euclid Definition 8). The angle is the unique way to introduce Quality — the character of a relationship. Angle is Quality (Unique Grounding Theorem). The specific angle value θ parameterizes a continuous spectrum of quality. |
| **Labels** | *Quality, Character, Manner, "Relates in this way"* |

**The distinguished values within this continuous spectrum are listed at Level 2 (§3.1), since each requires a specific geometric configuration.**

### 2.5 Single Curve

| | |
|---|---|
| **Construction** | A non-straight continuous path between two points: • ⌒ • |
| **Σ_UL** | — (the curvature profile κ(s) is geometric data, not an algebraic term; see §0.8) |
| **Geometric** | A relation whose direction changes continuously along its length. The curvature κ(s) parameterizes the rate of change at each point. |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | A curve is the unique primitive that introduces continuously varying direction. Where a line holds constant direction, a curve changes it. This is Process — the geometric basis of becoming, change, transformation. Curve is Process (Unique Grounding Theorem). |
| **Labels** | *Process, Change, Becoming, Transformation, "This becomes that through continuous change"* |

### 2.6 Single Enclosure (Empty)

| | |
|---|---|
| **Construction** | A closed boundary with nothing inside |
| **Σ_UL** | — (a boundary with empty interior; the closed-curve property is geometric) |
| **Geometric** | A closed curve partitioning the plane into interior and exterior (Jordan Curve Theorem). The interior is empty. |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | An enclosure requires a closed curve (Jordan Curve Theorem: partitions the plane into interior and exterior). The empty enclosure is the unique construction that introduces Concept without yet instantiating it. It is a definition with nothing defined — a category with no members. Enclosure is Concept (Unique Grounding Theorem). |
| **Labels** | *Empty concept, Null set, Category without members, Potential* |

**The specific shapes of enclosure (△, □, ⬠, ⬡, ○) are classified at Level 2 (§3.2), since each requires a specific boundary configuration.**

---

## 3. CONSTRUCTIVE LEVEL 2 — Distinguished Parameters and Single Operations

These are constructions that fix a specific parameter value within a Level 1 primitive's continuous spectrum, or that apply a single Σ_UL operation to atomic inputs. Each is included because its parameter value occupies a structurally singular or extremal position.

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
| **Σ_UL** | modify_relation(m₁₈₀, r) → r' (as an angular modifier applied to a relation) |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique angle at which a relation reverses direction. 180° is the only non-trivial involution in the angle group: applying it twice returns to the starting direction. This self-inverse property is what makes 180° structurally singular. |
| **Labels** | *Opposition, Reversal, Inversion, "the opposite direction"* |

> **Sort Disambiguation (see §0.8).** The 180° angle is ONE geometric object but realizes THREE algebraically distinct operations on THREE different Σ_UL sorts:
>
> | Operation | Signature | What it does |
> |-----------|-----------|-------------|
> | `modify_relation(m₁₈₀, r)` | m × r → r | Applies the 180° modifier to a relation — changes its angular quality |
> | `invert(r)` | r → r | Reverses a directed relation — swaps source and target |
> | `negate(a)` | a → a | Negates an assertion — logical complement |
>
> These are sort-distinct: `modify_relation` takes a Modifier and a Relation, `invert` takes a Relation alone, `negate` takes an Assertion. In the geometric algebra G, all three collapse to "180° rotation applied at the appropriate level." In Σ_UL, they are three separate primitives. The lexicon entry is for the *geometric angle* — the individual operations have their own entries at §6.6 (`invert`) and §5.6 (`negate`).

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
| **Σ_UL** | — (boundary shape is geometric; enters Σ_UL via the modifier sort through `abstract()`; see §0.8) |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The triangle is the minimum polygon (fewest sides that can enclose area). There is no 2-sided polygon in Euclidean geometry. 3 is the structural minimum. Therefore the triangle is the unique minimum-complexity enclosure. Minimum polygon = minimum complete structure = fundamental/atomic/irreducible. |
| **Labels** | *Fundamental, Atomic, Irreducible, Basic, "the simplest kind of"* |

#### 3.2.2 — Square (□)

| | |
|---|---|
| **Construction** | 4-sided closed polygon with equal sides and right angles |
| **Symmetry** | D₄ (dihedral group, order 8) |
| **Σ_UL** | — (boundary shape is geometric; enters Σ_UL via the modifier sort through `abstract()`; see §0.8) |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | The square is the unique regular polygon that tessellates the Euclidean plane by itself with only translations (not rotations). This gives it the property of systematic, ordered coverage. It is distinguished among 4-sided polygons but the step from 3 to 4 sides is not uniquely forced — any polygon with more sides is also possible. |
| **Labels** | *Structural, Systematic, Ordered, "the organized kind of"* |

#### 3.2.3 — Pentagon (⬠)

| | |
|---|---|
| **Construction** | 5-sided closed polygon |
| **Symmetry** | D₅ (dihedral group, order 10) |
| **Σ_UL** | — (boundary shape is geometric; enters Σ_UL via the modifier sort through `abstract()`; see §0.8) |
| **Tier** | T3 — Conventional |
| **Justification** | The pentagon's diagonals divide in the golden ratio φ = (1+√5)/2, which appears in biological growth patterns (phyllotaxis, shell spirals). The association of pentagon with "living" or "organic" is based on this φ-connection, which is empirically motivated but not geometrically forced. Other polygons also appear in biology. |
| **Labels** | *Organic, Living, Self-similar, Growth-related — convention based on golden-ratio association* |

#### 3.2.4 — Hexagon (⬡)

| | |
|---|---|
| **Construction** | 6-sided closed polygon |
| **Symmetry** | D₆ (dihedral group, order 12) |
| **Σ_UL** | — (boundary shape is geometric; enters Σ_UL via the modifier sort through `abstract()`; see §0.8) |
| **Tier** | T3 — Conventional |
| **Justification** | The hexagon achieves optimal circle-packing (honeycomb conjecture, proven Hales 2001). The association with "network" or "community" is based on this packing optimality — the most efficient way to fill a plane with equal bounded regions. Empirically motivated (beehives, basalt columns) but the semantic label is not forced. |
| **Labels** | *Networked, Communal, Efficient — convention based on optimal packing* |

#### 3.2.5 — Circle (○)

| | |
|---|---|
| **Construction** | Closed curve with constant curvature (all points equidistant from center) |
| **Symmetry** | SO(2) (continuous rotation group, infinite order) |
| **Σ_UL** | — (boundary shape is geometric; enters Σ_UL via the modifier sort through `abstract()`; see §0.8) |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The circle has the highest symmetry of any planar closed curve — it is invariant under all rotations and all reflections through the center. Maximum symmetry = maximum generality. This is not a convention — it is the definition of "general" in the Erlangen Program: the more transformations leave an object invariant, the more general it is. The circle is the unique maximum of this hierarchy. |
| **Labels** | *Universal, Complete, Abstract, Total, "the most general kind of"* |

#### 3.2.6 — Circle Enclosing Void — ○{Ø}

| | |
|---|---|
| **Construction** | A circle enclosing nothing (empty interior, consciously constructed) |
| **Σ_UL** | — (combines a specific enclosure shape with empty content; no single Σ_UL term; see §0.8) |
| **Geometric** | Maximum-symmetry boundary (○, SO(2)) with null interior. Distinguished from the bare empty enclosure (§2.6) in that a specific shape has been chosen, and from the bare Void (§1.1) in that a boundary exists. |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique construction that combines maximum-symmetry concept with zero content. A boundary drawn around nothing — a fully general definition of nothing. This is distinct from the bare Void (Ø, §1.1): the Void is the absence of construction; ○{Ø} is the CONSTRUCTION of absence. The difference is whether the nothingness is bounded. Ø = "there is nothing." ○{Ø} = "there is a concept, and it is: nothing." |
| **Labels** | *The concept of nothing, Empty set (as a concept), Defined absence, Zero (as distinguished from Nothing)* |

### 3.3 Distinguished Curves

#### 3.3.1 — Constant Curvature, Closed (Circle as Process)

| | |
|---|---|
| **Construction** | ○ read as a curve (not as an enclosure) — a closed path with constant curvature κ |
| **Σ_UL** | — (constant curvature and closure are geometric properties of the relation; see §0.8) |
| **Geometric** | The unique closed curve with constant curvature. Among all closed curves, the maximally symmetric one (SO(2)). As a process: the unique process that maintains constant rate of change and returns exactly to its starting state. |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The unique closed curve with constant curvature. Among all closed curves, it is the maximally symmetric one (SO(2)). As a process, it is the unique process that maintains constant rate of change and returns exactly to its starting state. |
| **Labels** | *Cycle, Self-reference, Return, Completeness-as-process, "returning to where it began"* |

#### 3.3.2 — Monotonically Increasing Radius (Spiral)

| | |
|---|---|
| **Construction** | 𝒮 — a curve whose distance from a center point monotonically increases |
| **Σ_UL** | — (the monotonic radial increase is a geometric property; see §0.8) |
| **Geometric** | A curve that simultaneously rotates (angular change) and expands (radial increase). The unique curve class that is periodic in angle and monotonic in radius. |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | The spiral is the simplest curve that combines rotation (angular change) with expansion (radial increase). It is the unique curve class that is simultaneously periodic in angle and monotonic in radius. Distinguished among open curves, but not the only expanding curve. |
| **Labels** | *Growth, Development, Evolution, Unfolding* |

#### 3.3.3 — Periodic Curvature (Sine Wave)

| | |
|---|---|
| **Construction** | ~ — a curve whose curvature alternates sign periodically |
| **Σ_UL** | — (the alternating curvature is a geometric property; see §0.8) |
| **Geometric** | A curve whose direction oscillates. The simplest periodic function (single Fourier component) with minimal harmonic content. |
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

### 3.5 Generic Modification Operations

These two entries give the 11-operation primitives `modify_entity` and `modify_relation` their own dedicated lexicon entries. The individual angle entries (§3.1) are distinguished *instances* of `modify_relation`; these entries capture the *generic* operations.

#### 3.5.1 — modify_entity

| | |
|---|---|
| **Construction** | A transformation (modifier) applied to an entity glyph, altering its properties while preserving its identity as an entity |
| **Σ_UL** | modify_entity(m, e) → e' |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | One of the 11 primitive Σ_UL operations (formal-foundations.md §1.5). This is the geometric realization of qualification applied to entities: "the *large* point," "the *red* entity," "the *wooden* thing." Without this operation, modifiers cannot affect entities — every entity would be structurally identical except by position. Minimal: removing it makes the system unable to express modified entities. |
| **Labels** | *Qualification of entity, "the [adjective] thing", Entity transformation* |

#### 3.5.2 — modify_relation (generic)

| | |
|---|---|
| **Construction** | A transformation (modifier) applied to a relation, altering its quality while preserving its identity as a relation |
| **Σ_UL** | modify_relation(m, r) → r' |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | One of the 11 primitive Σ_UL operations (formal-foundations.md §1.5). The parameterized family of which the distinguished angle entries (§3.1.1–3.1.6) are specific instances. Geometrically: applying an angular modifier to a relation changes its character without changing its endpoints. Minimal: removing it makes the system unable to distinguish the quality of different relations ("loves" vs. "hates" vs. "ignores"). |
| **Labels** | *Qualification of relation, "relates [adverb]", Relation transformation* |

**Note:** Every distinguished angle entry in §3.1 is an instance of this operation with a specific modifier value. This entry captures the general pattern; those entries capture the structurally singular instances.

---

## 4. CONSTRUCTIVE LEVEL 3 — Two-Primitive Combinations

An entity placed inside an enclosure is the simplest construction that gives Concept content. The meaning depends on which entity and which enclosure shape. This level also includes open-ended constructions with two primitives.

### 4.1 Entity-in-Enclosure Constructions

#### 4.1.1 — Point in Circle — ○{•}

| | |
|---|---|
| **Construction** | A single point centered in a circle |
| **Σ_UL** | embed(a₁) → e, where a₁ is an assertion involving e₁ (the specific ○ boundary shape is geometric; see §0.8) |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | This combines the highest-symmetry enclosure (○, T1) with the simplest entity (•, T1). It is the simplest construction that has BOTH concept AND content. Among all single-entity-in-enclosure constructions, ○{•} has maximal boundary symmetry. It is distinguished as the most universal/general instance of "defined existence" — but △{•}, □{•}, etc. are equally valid constructions at the same complexity. |
| **Labels** | *Truth, Completeness-of-existence, Unity, "what is, fully defined" — BUT: the label "Truth" is a T3 conventional choice. The geometry says "maximally general concept containing minimal existence." Whether that maps to a culture's word for truth, God, essence, or the number one is a grounding decision.* |

#### 4.1.2 — Point in Triangle — △{•}

| | |
|---|---|
| **Construction** | A single point centered in a triangle |
| **Σ_UL** | embed(a₁) → e, with D₃ boundary (geometric; see §0.8) |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | Combines the minimum-polygon enclosure (△, T1) with the simplest entity (•, T1). The most fundamental/irreducible instance of "defined existence." Distinguished as the most specific/atomic container for existence. |
| **Labels** | *Atom, Element, Irreducible unit, "the simplest defined thing"* |

#### 4.1.3 — Point in Square — □{•}

| | |
|---|---|
| **Construction** | A single point centered in a square |
| **Σ_UL** | embed(a₁) → e, with D₄ boundary (geometric; see §0.8) |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | Combines the systematic-tessellation enclosure (□, T2) with the simplest entity (•, T1). An existence bounded by structure — a defined, located, constrained thing. |
| **Labels** | *Constraint, Bounded existence, Located thing, "an existence within structure"* |

### 4.2 Open-Ended Constructions

#### 4.2.1 — Ray from Point (Unbounded) — •→

| | |
|---|---|
| **Construction** | A point with a ray extending outward without termination: •→ |
| **Σ_UL** | — (the unspecified target has no clean Σ_UL expression; the closest approximation involves existential quantification over the target entity) |
| **Geometric** | An entity with a directed relation extending to no fixed endpoint — a connection whose object is open/unresolved. |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | The simplest construction with an open-ended relation — a connection to something unspecified/unbounded. Distinguished as the minimal expression of directed unboundedness. |
| **Labels** | *Potential, Reaching, Freedom-as-direction, Aspiration, "this directs toward..."* |

---

## 5. CONSTRUCTIVE LEVEL 4 — Multi-Primitive Compositions

### 5.1 Two Points in Opposition — • ∠180° •

| | |
|---|---|
| **Construction** | Two points connected by a relation at 180° (through the center, opposite sides) |
| **Σ_UL** | predicate(e₁, invert(r), e₂) → a |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | Combines the two simplest entities with the forced opposition angle. The minimum expression of duality — two things in maximal opposition. Distinguished because 180° is T1 (forced), so this construction inherits that necessity. Note: this uses `invert(r)` (Relation → Relation), not `negate(a)` (Assertion → Assertion) — the opposition is between the entities via their relation, not a negation of the whole assertion. See §3.1.3 sort disambiguation. |
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

### 5.6 Reflected Assertion — Reflection of complete sentence frame

| | |
|---|---|
| **Construction** | A complete sentence frame reflected across its vertical axis |
| **Σ_UL** | negate(a) → a' |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | Reflection is an involution (reflecting twice returns to the original). It is the unique geometric operation that satisfies negate(negate(x)) = x. This makes it the forced realization of logical negation. Not a construction per se, but an operation on constructions — included here because negation is the most basic logical operation and its geometric realization is unique. See also §3.1.3 (180° sort disambiguation): this entry is negate on the Assertion sort specifically, distinct from invert on Relations (§6.6) and modify_relation with m₁₈₀ on Relations (§3.1.3). |
| **Labels** | *Not, Negation, Denial, "it is not the case that"* |

---

## 6. CONSTRUCTIVE LEVEL 5+ — Higher Compositions

At this constructive level, the combinatorial space expands rapidly. Most constructions are T3 (conventional). We include only those with clear structural motivation.

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
| **Justification** | Placing a complete assertion inside a boundary converts it from a statement into a thing-that-can-be-talked-about. This is nominalization — the geometric realization of "the fact that..." One of the 11 primitive operations. |
| **Labels** | *Nominalization, Reification, "the fact that...", "that [clause]"* |

### 6.4 Shape-Properties-as-Modifier — Abstraction

| | |
|---|---|
| **Construction** | Extracting the boundary shape of an entity and applying it as a modifier to another entity |
| **Σ_UL** | abstract(e) → m |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | The shape of an entity's boundary IS a transformation (a symmetry group action). Extracting it and applying it to another entity is the geometric realization of adjectivalization: "wood" → "wooden," "circle" → "circular." One of the 11 primitive operations. |
| **Labels** | *Adjectivalization, Quality-extraction, "-ness", "-like", "having the quality of"* |

### 6.5 Composed Relations — •→•→•

| | |
|---|---|
| **Construction** | Three entities connected by two sequential directed relations |
| **Σ_UL** | compose(r₁, r₂) → r₃, applied as predicate(e₁, compose(r₁, r₂), e₃) |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | When the endpoint of one relation is the startpoint of another, they compose — the resulting relation goes from the start of the first to the end of the second. This is the geometric realization of transitivity. One of the 11 primitive operations. |
| **Labels** | *Transitivity, Chain, Sequence, "through", "A relates to C via B"* |

### 6.6 Reversed Relation — •←•

| | |
|---|---|
| **Construction** | A directed relation with its arrow reversed |
| **Σ_UL** | invert(r) → r' |
| **Tier** | T1 — Geometrically Forced |
| **Justification** | Reversing a directed connection is the geometric realization of voice inversion: active ↔ passive, cause ↔ effect. It is the unique operation that reverses directionality without changing any other property of the relation. One of the 11 primitive operations. See also §3.1.3 (180° sort disambiguation): this entry is `invert` on the Relation sort (r → r), distinct from `negate` on Assertions (§5.6) and `modify_relation(m₁₈₀, r)` (§3.1.3). |
| **Labels** | *Passive voice, Reversal, Inverse relation, "is [verb]ed by"* |

### 6.7 Quantification — Scaling of Entity within Frame

| | |
|---|---|
| **Construction** | An entity scaled relative to its sentence frame: large (fills frame) = universal; small (point-like) = particular |
| **Σ_UL** | quantify(m_scale, e) → a |
| **Tier** | T2 — Structurally Distinguished |
| **Justification** | Quantification is one of the 11 primitive Σ_UL operations. Its geometric realization as scaling is motivated: entity filling the frame = "applies everywhere" = universal; entity as point within frame = "applies somewhere specific" = existential. The scaling interpretation is natural but not the only possible realization of quantification. |
| **Labels** | *All/Some/None (depending on scale), Quantification, "for all...", "there exists..."* |

---

## 7. OPERATIONS CROSS-REFERENCE

The 11 Σ_UL operations are themselves "words" in UL — they are the verbs of the language, the ways of combining. Each has a dedicated entry elsewhere in this lexicon (or is realized through its distinguished instances). **This section is a navigation aid — it does not contribute additional entries to the count in §8.**

| # | Operation | Signature | Entry Location | Tier |
|---|-----------|-----------|----------------|------|
| 1 | `predicate` | e × r × e → a | §2.2 (Single Line) | T1 |
| 2 | `modify_entity` | m × e → e | §3.5.1 | T1 |
| 3 | `modify_relation` | m × r → r | §3.5.2 (generic) + §3.1.x (instances) | T1 |
| 4 | `negate` | a → a | §5.6 (Reflected Assertion) | T1 |
| 5 | `conjoin` | a × a → a | §6.1 (Conjunction) | T1 |
| 6 | `disjoin` | a × a → a | §6.2 (Disjunction) | T1 |
| 7 | `embed` | a → e | §6.3 (Embedding) | T1 |
| 8 | `abstract` | e → m | §6.4 (Abstraction) | T1 |
| 9 | `compose` | r × r → r | §6.5 (Composed Relations) | T1 |
| 10 | `invert` | r → r | §6.6 (Reversed Relation) | T1 |
| 11 | `quantify` | m × e → a | §6.7 (Quantification) | T2 |

All 11 operations are Tier 1 or Tier 2. None is conventional. This is expected — they are the primitive operations of the algebraic signature, proved minimal in formal-foundations.md §1.5.

---

## 8. TIER SUMMARY AND HONEST ACCOUNTING

### 8.1 Count by Tier

| Tier | Count | Percentage | What this means |
|------|-------|------------|----------------|
| T1 — Geometrically Forced | 24 | ~57% | More than half the lexicon is mathematics. These entries cannot be otherwise. |
| T2 — Structurally Distinguished | 14 | ~33% | A third occupies extremal positions — motivated, canonical, but with alternatives. |
| T3 — Conventional | 4 | ~10% | A small minority are design decisions. Clearly labeled as such. |
| **Total** | **42** | **100%** | |

### 8.2 Changes from v1

| Change | Old | New | Reason |
|--------|-----|-----|--------|
| Ray (§2.3) | T1 | T2 | Direction choice is a selection, not a theorem (Fix D) |
| ○{Ø} placement | Level 3 (§4.4) | Level 2 (§3.2.6) | It parameterizes a single enclosure shape, not a two-primitive combo (Fix A/D) |
| modify_entity (§3.5.1) | Missing | T1 entry | Primitive operation with no prior dedicated entry (Fix B) |
| modify_relation (§3.5.2) | Implicit in angle entries | T1 entry | Generic operation made explicit; angle entries are instances (Fix B) |
| 180° (§3.1.3) | Conflated negate/invert/modify_relation | Sort-separated | Three sort-distinct operations on one geometric angle (Fix C) |
| §7 Operations table | Ambiguous count source | Cross-reference only | Eliminated double-counting risk (Fix B) |
| "Complexity N" sections | Undefined metric | Defined Constructive Levels (§0.7) | Levels now have formal definitions (Fix A) |
| Σ_UL field | Mixed algebraic and geometric | Split: Σ_UL (algebraic or "—") + Geometric (when needed) | Honest about G ≠ Σ_UL (Fix C) |

### 8.3 What Is NOT in This Lexicon

The following constructions appear in other siblings but are **not included here** because they are T3 entries without sufficient motivation for first inclusion:

- **"Democracy"** (Grammar §VII) — a compound construction of community + mutual relations + equality. Valid UL, but one of infinitely many possible governance constructions. No geometric reason to privilege it.
- **"Evolution"** (Grammar §VII) — a compound construction of living + spiral + time + change. Valid UL, but one of many possible growth-over-time constructions.
- **"Love"** as ○{•≅•} (Symbology §V) — the label "love" for "complete enclosure of identified pair" is a grounding decision. The geometry says "two identified existences within maximal-symmetry boundary." Whether that's love, marriage, fusion, or identity-in-unity depends on context.
- **"Knowledge"** as ○{○{•}─∠60°─○} (Symbology §V) — a multi-component compound with a conventional label. The geometry is valid; the word "knowledge" is a choice.
- **"Fear"** as •←══□ (Thesaurus §VII) — one of many possible constructions for threat-response. The structural reading "entity receiving emphatic relation from constraint" could equally be labeled deference, submission, or awe.

These entries remain in their sibling files as **examples and illustrations** — they are useful for showing how UL constructions work. But they are not canonical definitions. They are applications of the lexicon, not the lexicon itself.

### 8.4 The Boundary Principle

> **If two competent geometers, given only the primitives and operations, could independently arrive at the same construction for the same structural concept — it belongs in the lexicon at T1 or T2.**
>
> **If they would need to agree on a convention first — it belongs at T3 or not at all.**

This principle is the filter. It is why most of what was bleeding across the siblings does NOT appear here as its own entry: those compounds require conventions (choosing "love" over "fusion," choosing "democracy" over "participatory-governance-structure"), and conventions are external to geometry.

---

## 9. RELATIONSHIP TO SIBLINGS

> **Navigation tip:** Each sibling now includes a cross-reference table at the top of the document. If you arrived here from another sibling, you can use the table below to understand how these documents relate, or return to any sibling via its cross-reference block.

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
| **Exhaustive enumeration of T1 constructions** | Believed complete for Level ≤ 5. Not proven. An enumeration audit (v2) checked for omissions at each level but cannot guarantee completeness. | If a forced construction is missing, the lexicon is incomplete. |
| **Tier boundary for quantify** | Listed as T2. Some arguments for T1 (it's a primitive operation). | Affects the T1/T2 ratio. The geometric realization via scaling is motivated but may not be unique. |
| **Higher-polygon enclosures** (heptagon, octagon, ...) | Not included. D₇, D₈, ... symmetry groups exist but have no structurally distinguished status beyond D₆. | If a mathematical property distinguishes some higher polygon (as φ distinguishes the pentagon), it should be added. |
| **3D and higher-dimensional constructions** | Not addressed. This lexicon is planar (2D). | UL's axiomatics work on any 2D surface. Extension to 3D introduces new primitives (volume, curvature of surfaces) — a genuine open problem. |
| **Interaction with gauge connection** | The gauge bundle (frontier/expedition-one) adds context-dependence. Whether context shifts Tier assignments is open. | A T2 construction might become T1 or T3 in specific gauge contexts. |
| **G ≠ Σ_UL boundary** | Several entries have Σ_UL: — because their geometric properties don't map to single algebraic terms (§0.8). Enriching Σ_UL with shape/curvature sorts could close this gap. | This is a foundational question: should Σ_UL be extended, or is the geometric-only characterization correct? |

---

## 10.5 SEMANTIC INDEX — FIND YOUR CONCEPT

If you're looking for a specific meaning, use this index to jump to the right entry. Concepts are grouped by semantic category, not constructive level.

**Existence & Non-Existence:**
- Existence → §2, Entry 2.1 (Point) | Something exists → §4, Entry 4.1 (•-in-○)
- Non-existence / Nothing → §1, Entry 1.0 (Void) | Emptiness → §3, Entry 3.2.5 (○{Ø})
- Negated existence → §5, Entry 5.6 (Negation)

**Relationships & Connections:**
- Connection between things → §2, Entry 2.2 (Line)
- Directed relationship → §2, Entry 2.2.1 (Ray)
- Identity / sameness → §3, Entry 3.1.1 (0° Angle)
- Opposition / conflict → §3, Entry 3.1.3 (180° Angle)
- Independence / orthogonality → §3, Entry 3.1.2 (90° Angle)
- Harmony / cooperation → §3, Entry 3.1.5 (60° Angle)
- Complementarity → §3, Entry 3.1.6 (120° Angle)
- Transitivity / chains → §6, Entry 6.5 (Composition)
- Reversal / passivity → §6, Entry 6.6 (Reversal)

**Qualities & Modifications:**
- Quality in general → §2, Entry 2.3 (Angle, generic)
- Entity transformation → §2, Entry 2.6 (modify_entity)
- Relation qualification → §2, Entry 2.7 (modify_relation)
- Abstraction (entity → quality) → §6, Entry 6.4 (abstract)

**Processes & Change:**
- Process / change → §2, Entry 2.4 (Curve, generic)
- Cyclical process → §3, Entry 3.3.1 (Closed curve)
- Expanding growth → §3, Entry 3.3.2 (Spiral)
- Completeness → §3, Entry 3.1.4 (360° / full rotation)

**Concepts & Categories:**
- Concept / bounded idea → §2, Entry 2.5 (Enclosure, generic)
- Fundamental thing → §3, Entry 3.2.1 (△, Triangle)
- Structural system → §3, Entry 3.2.2 (□, Square)
- Living / organic → §3, Entry 3.2.3 (⬠, Pentagon)
- Community / network → §3, Entry 3.2.4 (⬡, Hexagon)
- Universal / complete → §3, Entry 3.2.5 (○, Circle)
- Thing-in-concept → §4, Entries 4.1–4.5

**Logical & Compositional:**
- Claiming something is true → §5, Entry 5.1 (Predication / Assertion frame)
- "A AND B" → §6, Entry 6.1 (Conjunction)
- "A OR B" → §6, Entry 6.2 (Disjunction)
- "The fact that A" → §6, Entry 6.3 (Embedding)
- "NOT A" → §5, Entry 5.6 (Negation)
- "How many?" → §6, Entry 6.7 (Quantification)

**Not in this Lexicon:**
- Love, Fear, Joy, Democracy, Evolution, Knowledge → See Lexicon §8.3 (excluded as T3/conventional); see Thesaurus §VII for synonym navigation; see Symbology §V for illustrative constructions.

---

## 11. VERSION HISTORY

| Version | Date | Changes |
|---------|------|---------|
| v1 | 2026-03-13 | Initial construction. 40 entries. Built bottom-up from geometry with 3-tier justification system. |
| v2 | 2026-03-13 | 42 entries. Five fixes applied: (A) "Complexity N" → defined Constructive Levels §0.7; (B) modify_entity and modify_relation given dedicated entries, §7 restructured as cross-reference; (C) Σ_UL/Geometric fields split, 180° sort conflation resolved with disambiguation table, §0.8 added; (D) Ray downgraded T1→T2, ○{Ø} relocated to Level 2, enumeration audit performed; (E) deferred to sibling-cleaning pass. |
