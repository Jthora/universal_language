# Universal Grammar Book

> How meaning emerges from geometric relationships.  
> Grammar is not a set of arbitrary rules — it is the catalogue of possible geometric relationships between objects.  
> Parts of speech are symmetry classifications. Sentence structure is figure composition.

### How This Document Relates to Its Siblings

| If you need... | Consult | Why |
|----------------|---------|-----|
| What the atomic symbols look like | **Symbology** (symbology/symbol-map.md) | Symbology defines the marks; Grammar explains why they mean what they mean |
| Mechanical rules for valid combinations | **Syntax** (syntax-dictionary.md) | Syntax provides the construction rules; Grammar provides the semantic theory |
| Whether a construction is canonical | **Lexicon** (lexicon/lexicon.md) | Lexicon assigns tiers (T1/T2/T3) — see §8.3 for what's canonical vs. conventional |
| Related meanings and synonyms | **Thesaurus** (thesaurus/thesaurus.md) | Thesaurus navigates the Erlangen hierarchy for synonym-finding |
| Practical writing and reading | **Writer's Companion** (writing-system/writers-companion.md) | Step-by-step pen-and-paper guide |

> **Note on two classification systems:** This document classifies symbols linguistically by **symmetry group** (determining parts of speech via the Erlangen Program). Syntax classifies expressions algebraically by **sort** (Entity, Relation, Modifier, Assertion). These are complementary: a symbol like → has low rotational symmetry (Grammar → Verb) AND belongs to the Relation sort (Syntax → algebraic role in `predicate(e, r, e)`). Symmetry determines linguistic behavior; sort determines algebraic capability.

---

## I. THE GRAMMATICAL PRINCIPLE

In natural languages, grammar is a cultural convention: English puts adjectives before nouns; Japanese puts them after. These are arbitrary choices.

In Universal Language, **grammar IS geometry.** The rules are forced by mathematical necessity:

- **Parts of speech** are determined by the **symmetry group** of a symbol
- **Sentence structure** is determined by the **incidence relations** between symbols
- **Grammatical transformations** are determined by the **Erlangen hierarchy** of transformation groups
- **Logical connectives** are determined by **topological operations** on regions

None of this is chosen. All of it is derived.

---

## II. PARTS OF SPEECH (From Symmetry)

The symmetry group of a symbol determines its grammatical role. This is Klein's Erlangen Program applied to language:

> *"A geometric object is characterized by the group of transformations that leave it invariant."*

### The Symmetry Classification

| Symmetry Property | Grammatical Role | Why |
|-------------------|-----------------|-----|
| Full SO(2) symmetry (circle) | **Determiner / Article** | Looks the same from all directions = applies universally |
| High rotational symmetry (regular polygon) | **Noun (abstract/universal)** | Nearly context-independent = general concept |
| Low rotational symmetry (irregular shape) | **Verb (action/process)** | Has preferred direction = temporal/directional |
| Bilateral symmetry only (one mirror axis) | **Adjective / Adverb** | One axis of invariance = one dimension of comparison |
| No symmetry (asymmetric) | **Proper noun / Index** | Unique = refers to exactly one thing |
| Translation symmetry (repeating pattern) | **Plural / Iterative** | Repetition = multiple instances |

### Examples

```
NOUN (high symmetry):
  ○  — "totality" (SO(2), infinite rotational symmetry)
  □  — "structure" (D₄, 4-fold rotational symmetry)
  △  — "fundamental" (D₃, 3-fold rotational symmetry)

VERB (low symmetry, directional):
  →  — "acts upon" (no rotational symmetry, directed)
  ◠  — "becomes" (no rotational symmetry, curved)
  ↑  — "rises" (no rotational symmetry, directed upward)

ADJECTIVE (bilateral symmetry):
  ∠  — "in the manner of [angle]" (symmetric about bisector only)
  ─  — "connected" (symmetric about midpoint only)

PROPER NOUN (no symmetry):
  •₁ — "this specific thing" (no symmetry — unique position)
  
ARTICLE/DETERMINER:
  ○ (unqualified) — "the" / "all" (maximum symmetry = maximum generality)
```

### Why This Works

Symmetry measures **context-independence**. A circle looks the same from any angle — it's the most abstract, context-independent symbol. A directed arrow looks different depending on where you stand — it's inherently contextual, temporal, directional. This maps naturally to the noun/verb distinction: nouns name things that persist across contexts; verbs name actions that depend on direction and time.

---

## III. RELATIONSHIP TAXONOMY (Parts-of-Speech Interactions)

Grammar is the catalogue of possible relationships between symbols. These derive from the four classes of geometric relationship.

### Class 1: Incidence Relations (Fundamental Connections)

These are the most basic: "does X touch Y?"

| Geometric Relationship | Linguistic Function | UL Expression |
|----------------------|---------------------|---------------|
| Point lies on line | Element belongs to class | **Predication**: "X is a Y" |
| Line passes through point | Class contains element | **Classification**: "Ys include X" |
| Two lines intersect | Two relations share a context | **Conjunction**: "X and Y meet at Z" |
| Point between two points | Element mediates extremes | **Comparison**: "X is between Y and Z" |
| Line tangent to curve | Relation touches process at one point | **Instantaneous contact**: "X relates to Y at this moment" |
| Point inside enclosure | Entity within concept | **Membership**: "X belongs to C" |
| Point outside enclosure | Entity excluded from concept | **Exclusion**: "X is not a C" |

### Class 2: Metric Relations (Quantitative Grammar)

These involve distance and measurement: "how far? how much?"

| Geometric Relationship | Linguistic Function | UL Expression |
|----------------------|---------------------|---------------|
| Distance between points | Degree of difference | **Degree**: "X is [this far] from Y" |
| Length of segment | Magnitude of relation | **Quantity**: "X relates to Y [this much]" |
| Ratio of lengths | Proportional comparison | **Comparative**: "X is [N times] more than Y" |
| Angle measure | Quality of relation | **Manner/Quality**: "X relates to Y [in this way]" |
| Curvature | Rate of change | **Rate**: "X becomes Y [this quickly]" |
| Area of enclosure | Scope of concept | **Generality**: "C covers [this much]" |

### Class 3: Symmetry Relations (Structural Grammar)

These involve invariance under transformation: "what is preserved?"

| Geometric Relationship | Linguistic Function | UL Expression |
|----------------------|---------------------|---------------|
| Congruence (A ≅ B) | Exact equivalence | **Identity**: "A is identical to B" |
| Similarity (A ~ B) | Same shape, different scale | **Analogy**: "A is like B" |
| Reflection symmetry | Dual/complementary opposition | **Antonymy**: "A and B are opposites" |
| Rotational symmetry | Perspective invariance | **Universality**: "A is true from all perspectives" |
| Translation symmetry | Pattern/repetition | **Plurality**: "A, and then A again, and again..." |
| Scale invariance | Self-similarity | **Fractal/recursive**: "A contains copies of itself" |

### Class 4: Topological Relations (Essential Grammar)

These involve what is preserved under continuous deformation: "what is the deep structure?"

| Geometric Relationship | Linguistic Function | UL Expression |
|----------------------|---------------------|---------------|
| Homeomorphism | Same essential structure | **Deep synonymy**: "A is essentially B" |
| Genus (number of holes) | Complexity class | **Type marker**: simple (genus 0), compound (genus 1), complex (genus 2+) |
| Connectedness | Logical coherence | **Continuity**: "the parts form a whole" |
| Boundary | Definition/limit | **Determiner**: "the edge of" |
| Interior | Scope content | **Inside-reference**: "within C" |
| Exterior | Context | **Outside-reference**: "beyond C" |
| Simply connected | No self-reference | **Simple concept** (no loops in meaning) |
| Multiply connected | Self-referential | **Paradoxical/recursive concept** (loops in meaning) |

---

## IV. THE ERLANGEN HIERARCHY OF MEANING

Klein's Erlangen Program gives UL a **hierarchical grammar** — different levels of description, each preserving different aspects of meaning. Two expressions related by a given transformation group are the "same" at that level.

### Level 1: Euclidean (Surface Form)
**Group:** Rigid motions E(n) — translations, rotations, reflections  
**Preserves:** Exact distances, angles, orientations  
**Linguistic level:** Exact phrasing. Every symbol in exactly this position.  
**"Same" means:** Identical construction, possibly moved/rotated/reflected.

### Level 2: Similarity (Semantic Content)
**Group:** Similarity transformations — rigid motions + uniform scaling  
**Preserves:** Angles, ratios (not absolute distances)  
**Linguistic level:** Same meaning, different emphasis/scale.  
**"Same" means:** Same relationships, possibly larger/smaller/emphasized differently.

> Two expressions are **synonyms** if they are related by a similarity transformation.

### Level 3: Affine (Structural Meaning)
**Group:** Affine transformations — linear maps + translation  
**Preserves:** Parallelism, ratios of distances along lines, betweenness  
**Linguistic level:** Same logical structure, regardless of specific content.  
**"Same" means:** Same argument structure, possibly with different filling.

> Two expressions are **paraphrases** if they are related by an affine transformation.

### Level 4: Projective (Deep Structure)
**Group:** Projective transformations PGL  
**Preserves:** Incidence (what touches what), cross-ratio  
**Linguistic level:** What can be related to what, regardless of how.  
**"Same" means:** Same connection structure, possibly viewed from radically different perspectives.

> Two expressions are **structural analogs** if they are related by a projective transformation.

### Level 5: Topological (Essential Meaning)
**Group:** Homeomorphisms — continuous bijections  
**Preserves:** Connectedness, genus, fundamental group  
**Linguistic level:** The irreducible core of meaning.  
**"Same" means:** Same essential meaning in any language, any notation, any context.

> Two expressions are **translations** (of each other, across languages) if they are topologically equivalent.

### The Fundamental Group as Semantic Core

> **The fundamental group π₁ of a symbol IS its meaning.**

| Fundamental Group | Topological Type | Meaning Class |
|-------------------|-----------------|---------------|
| π₁ = {e} (trivial) | Simply connected | Simple / Atomic concept |
| π₁ = ℤ | One hole (torus) | Self-referential / Cyclic concept |
| π₁ = ℤ × ℤ | Two holes (genus-2) | Doubly self-referential / Dialectical |
| π₁ = F₂ (free group) | Figure-eight | Irreducibly complex / Paradoxical |

Two symbols with the same fundamental group mean the same thing, regardless of visual appearance. This is why UL is universal: **meaning is a topological invariant.**

---

## V. SENTENCE COMPOSITION RULES

### The Minimum Sentence: The Semantic Triangle

Just as the triangle is the minimum polygon, the minimum sentence has 3 elements:

```
  ┌───────────────────┐
  │  [Subject] ──r──→ [Predicate]  │
  └───────────────────┘
```

This corresponds to the `predicate` operation: predicate(e₁, r, e₂) → a.

### Extending Sentences

Every sentence can be extended via the 11 operations:

| Extension | Operation | Example |
|-----------|-----------|---------|
| Add a modifier | modify_entity(m, e) | "the BIG dog" (scale up subject) |
| Qualify a relation | modify_relation(m, r) | "acts STRONGLY on" (scale up relation) |
| Negate | negate(a) | "does NOT act on" (reflect entire frame) |
| Conjoin | conjoin(a₁, a₂) | "A AND B" (overlap frames) |
| Disjoin | disjoin(a₁, a₂) | "A OR B" (adjoin frames) |
| Nominalize | embed(a) | "the fact that A" (shrink-and-enclose) |
| Adjectivalize | abstract(e) | "X-like" (extract quality from entity) |
| Chain | compose(r₁, r₂) | "grandfather" = father∘father |
| Reverse | invert(r) | "is acted upon by" (reverse arrow) |
| Quantify | quantify(m, e) | "ALL X" (scale to fill) / "SOME X" (scale down) |

### Composition is Associative

The compose operation (r₁ ∘ r₂) is associative:

```
(r₁ ∘ r₂) ∘ r₃ = r₁ ∘ (r₂ ∘ r₃)
```

This means: "grandfather's father" = "great-grandfather" regardless of grouping. Relation-chains are unambiguous.

### Nesting is Recursive

The embed operation converts assertions to entities, which can then be used in new predications:

```
Level 0: predicate(•₁, →, •₂) = a₁     "A acts on B"

Level 1: embed(a₁) = e₃                  "the fact that A acts on B"
         predicate(e₃, →, •₃) = a₂       "the fact that A acts on B, acts on C"

Level 2: embed(a₂) = e₄                  "the fact that [the fact that A acts on B] acts on C"
         predicate(e₄, →, •₄) = a₃       ...
```

Nesting depth is theoretically unlimited, bounded only by the resolution of the glyph space.

---

## VI. LOGICAL GRAMMAR (Topological Connectives)

Logic in UL is topology — specifically, the set operations on enclosed regions.

| Operation | Set Form | Geometric Form | Logical Meaning |
|-----------|----------|---------------|-----------------|
| Intersection | A ∩ B | Overlapping enclosures | **AND** |
| Union | A ∪ B | Joined enclosures | **OR** |
| Complement | Aᶜ | Inverted fill (inside↔outside) | **NOT** |
| Containment | A ⊂ B | One enclosure inside another | **IF A THEN B** (implication) |
| Disjointness | A ∩ B = ∅ | Separated enclosures | **XOR** / mutual exclusion |
| Cover | A ∪ B = GS | Two enclosures filling the glyph space | **Exhaustive**: "either A or B, nothing else" |
| Proper subset | A ⊊ B | Smaller inside larger with gap | **A is a special case of B** |

### Truth Tables as Geometric Figures

```
AND (intersection):          OR (union):
  ┌───┬───┐                  ┌───────────┐
  │ A │∩│B│                  │  A  ∪  B  │
  └───┴───┘                  └───────────┘

NOT (complement):            IF...THEN (containment):
  ┌───────────┐              ┌───────────┐
  │  ┌───┐    │              │  ┌───┐    │
  │  │ A̅ │    │              │  │ A │  B │
  │  └───┘    │              │  └───┘    │
  └───────────┘              └───────────┘
  (outside of A is NOT A)    (A inside B = A implies B)
```

---

## VII. WORD FORMATION (Morphology)

### Atomic → Compound Construction

Words in UL are built by combining atomic symbols through the composition operations:

**Method 1: Nesting** (concept-membership)
```
• inside △ = "a fundamental existence"
◠ inside ⬠ = "a living process"
• inside ○ inside □ = "a structurally organized universal existence"
```

**Method 2: Connection** (relational compound)
```
△─□ = "fundamental-structure" (compound noun)
•→○ = "existence-toward-totality" (compound verb/process)
```

**Method 3: Overlap** (blended meaning)
```
△∩□ = "that which is both fundamental and structural"
○∩⬠ = "that which is both universal and living"
```

**Method 4: Transformation** (derived form)
```
Scale(△) = "fundamental-ness" (noun → quality)
Reflect(→) = "acted-upon" (active → passive)
Rotate(□, 45°) = "structure from a diagonal perspective"
```

### Worked Example: Constructing "Democracy"

```
Step 1: Start with CONCEPT of COMMUNITY      ⬡
Step 2: Place PLURALITY inside (many agents)  ⬡{•,•,•,...}
Step 3: Connect agents by MUTUAL relations    ⬡{•↔•↔•↔...}
Step 4: Enclose mutual relations in STRUCTURE □{↔,↔,↔}
Step 5: Place structure inside community      ⬡{□{↔,↔,↔},{•,•,•}}
Step 6: Add EQUALITY marker (all ∠60°)        ⬡{□{↔∠60°↔∠60°↔},{•,•,•}}

Result: "A community containing equally-connected agents
within a structural system"

→ The natural-language label for this construction is a grounding
  decision, not a geometric fact. See Lexicon §8.3.
```

### Worked Example: Constructing "Evolution"

```
Step 1: Start with GROWTH (spiral)            𝒮
Step 2: Place inside LIVING (pentagon)         ⬠{𝒮}
Step 3: Add TIME (helix/process direction)     ⬠{𝒮⧖}
Step 4: Add CHANGE (curve within structure)    ⬠{𝒮⧖◠}
Step 5: Scale up (this is a big process)       ⬠{𝒮⧖◠}⁺

Result: "A living growth-spiral with directed time
and continuous change, at great scale"

→ The natural-language label for this construction is a grounding
  decision, not a geometric fact. See Lexicon §8.3.
```

### Worked Example: Constructing "Negation"

```
Step 1: Start with any ASSERTION              [•──→──•]
Step 2: REFLECT through center of frame        [•──←──•] (directions reversed)

Result: The content is mirrored — every directed relation reverses.

→ This is negate(a) → a from Σ_UL.
  Reflection is an involution: negate(negate(a)) = a.
  The frame stays; the content flips.
  This is T1 (Geometrically Forced): reflection IS the unique
  content-reversing, frame-preserving, involutory transformation.
```

### Worked Example: Constructing "Causation"

```
Step 1: Write CAUSE assertion                  [•──→──•]  (A relates to B)
Step 2: Write EFFECT assertion                 [•──→──•]  (C relates to D)
Step 3: EMBED cause as entity                  embed([A→B]) = ê₁
Step 4: EMBED effect as entity                 embed([C→D]) = ê₂
Step 5: CONNECT with directed relation         ê₁ ══→══ ê₂
Step 6: FRAME the whole thing                  [ ê₁ ══→══ ê₂ ]

Result: "The fact that [A→B] leads to the fact that [C→D]"

→ Causation is embed + predicate + embed: turning two assertions
  into entities, then relating them. The directed relation between
  embedded facts IS the "because" / "therefore" / "causes."
  See Writer's Companion §9.5 (BECAUSE pattern).
```

### Worked Example: Constructing "Self-Reference"

```
Step 1: Start with CONCEPT enclosure           ○
Step 2: The concept CONTAINS itself             ○{○{○{...}}}
Step 3: This infinite nesting is a FIXED POINT  ○∞

Result: A concept whose content IS itself — self-reference.

→ Topologically, this is a space with fundamental group π₁ = ℤ
  (one loop — the concept refers back to itself once).
  Grammar §IX.1 discusses recursive embedding in discourse.
  The Lexicon lists this as Entry 5.5 (Self-Referencing Concept,
  T2 — Structurally Distinguished by the fixed-point property).
```

> **Note on Constructive Level:** Democracy and Evolution are at Constructive Level 4+ (see Lexicon §0.7) and represent *one possible* construction for each concept — multiple valid constructions may express similar meanings. The label (natural-language word) is a grounding decision, not a geometric fact. Negation, Causation, and Self-reference are at Level 2–4 and are structurally motivated (T1 or T2). For tier classifications, see Lexicon §8.

---

## VIII. AGREEMENT AND GOVERNMENT

In natural languages, agreement (subject-verb matching) and government (case assignment) are arbitrary conventions. In UL, they are geometric necessities:

### Agreement = Sort Matching
- `predicate` requires (e, r, e) → you cannot put a Relation where an Entity goes
- `negate` requires (a) → you cannot negate an Entity directly (you must first make it an assertion)
- This is not a rule someone wrote. It's the type system of the algebra.

### Government = Incidence Constraint
- A Line (Relation) must connect to two Points (Entities) — Euclid's Postulate 1
- An Angle (Quality) requires two Lines meeting at a Point — Euclid's Definition 8
- These geometric requirements ARE the government rules

### There Is No Grammatical Gender
Natural-language gender (le/la, der/die/das) is arbitrary classification. UL classifies by **symmetry group** (the Erlangen classification), which is mathematically forced. There is no residual arbitrariness.

---

## IX. DISCOURSE STRUCTURE (Multi-Sentence Composition)

### Frame Relations

Multiple sentence frames compose through the same operations as symbols within frames:

| Discourse Relation | Frame Arrangement | Meaning |
|-------------------|-------------------|---------|
| Sequence | Left → Right | "And then... And then..." |
| Cause-Effect | Frame A ⊂ Frame B | "Because A, therefore B" |
| Contrast | Frames connected by ∠180° | "A, however B" |
| Elaboration | Frame A with Frame B nested inside | "A, specifically B" |
| Parallel | Frames at ∠0° (parallel) | "Similarly, A and B" |
| Comparison | Frames connected by ~(similarity) | "A is like B" |

### Paragraphs = Enclosures of Enclosures

A paragraph is a larger enclosure containing multiple sentence frames, with the enclosure shape indicating the paragraph's rhetorical function:

```
□{  ┌─────┐  ┌─────┐  ┌─────┐  }   = "An ordered sequence of statements"
    │ S₁  │→ │ S₂  │→ │ S₃  │       = "Argument/proof"
    └─────┘  └─────┘  └─────┘

○{  ┌─────┐  ┌─────┐  ┌─────┐  }   = "A complete exploration of statements"
    │ S₁  │  │ S₂  │  │ S₃  │       = "Essay/meditation"
    └─────┘  └─────┘  └─────┘

△{  ┌─────┐ }                        = "A fundamental single statement"
    │ S₁  │                           = "Axiom/definition"
    └─────┘
```

---

## X. AMBIGUITY RESOLUTION

### UL Has No Syntactic Ambiguity

In English, "I saw the man with the telescope" is ambiguous (who has the telescope?). In UL, the geometric arrangement resolves this:

```
READING 1: "I used a telescope to see the man"
  ┌────────────────────────────┐
  │  •₁ ──→ □{🔭} ──→ •₂     │  (tool is on the relation)
  └────────────────────────────┘

READING 2: "I saw the man who had a telescope"
  ┌────────────────────────────┐
  │  •₁ ──→ •₂{□{🔭}}        │  (tool is inside the object)
  └────────────────────────────┘
```

The two-dimensional layout forces disambiguation. **Where** the telescope-symbol sits in the figure determines its grammatical role unambiguously.

### UL Has No Scope Ambiguity

In logic, "¬∀x P(x)" vs "∀x ¬P(x)" depends on precedence conventions. In UL, scope is spatial containment:

```
"Not all X are P":          "All X are not-P":
┌──────────────────┐        ┌──────────────────┐
│  REFLECT(         │        │  ∀x:              │
│    ┌─────────┐   │        │  ┌──────────┐    │
│    │ ∀x: P(x)│   │        │  │REFLECT(P) │    │
│    └─────────┘   │        │  └──────────┘    │
│  )               │        │                  │
└──────────────────┘        └──────────────────┘
(negation OUTSIDE quant)    (negation INSIDE quant)
```

The enclosure structure makes scope visually unambiguous.
