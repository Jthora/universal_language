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

**⚠ Caveat — The Euclidean Assumption:** The claim that analogy is unique (exactly one structural parallel per meaning) is a consequence of adopting Euclidean geometry. Empirical evidence suggests this may be wrong for global meaning-space: a single concept like "love" admits multiple non-intersecting structural analogs from a single external domain (chemical bonding, gravitational attraction, economic exchange, symbiosis, harmonic resonance). This pattern is characteristic of **hyperbolic** geometry (infinitely many parallels), not Euclidean (exactly one). Most core UL theorems (Unique Grounding, Free Algebra, Polysemy-Holonomy) are geometry-independent and survive under non-Euclidean alternatives. The Euclidean plane should be understood as a **local simplification**, not a claim about global meaning-space geometry. See `docs/planning/audits/improvements/pass1-1/tier-a-foundational/tier-a-working-analysis.md` §A2 for the full analysis.

**Note:** Non-Euclidean extensions (hyperbolic, elliptic) modify this axiom and would produce meaning-spaces with different analogy properties. This is an active research direction — not merely open, but motivated by the evidence above.

---

## III. THE 16 SYNTACTIC OPERATIONS (From Σ_UL⁺ + Extensions)

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
  Flip the frame boundary from solid to dashed (or dashed to solid).
  Content is unchanged — only the assertional status flips.
  
  ASSERTION:                   NEGATION:
  ┌───────────────┐              ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐
  │  △ ──→ ○      │              ╎  △ ──→ ○      ╎
  └───────────────┘              └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘
  "fundamental acts              "NOT: fundamental
   on universal"                  acts on universal"
```

> **Design note:** The previous definition used reflection of content through a vertical axis. This was incorrect — reflection produces the *converse* (subject-object swap), not logical *negation* (truth-value flip). The converse is derivable from `predicate` + `invert`. See `docs/planning/audits/improvements/pass1-1/tier-b-structural/P0-negation-resolution.md`.

**Properties:**
- negate(negate(a)) = a (double boundary flip = identity) ✓
- negate is total (every assertion can be negated) ✓
- conjoin(a, negate(a)) = ⊥ (same content, both asserted and denied = contradiction) ✓
- disjoin(a, negate(a)) = ⊤ (content is either asserted or denied = tautology) ✓

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
  Draw the entity's OUTLINE only (no interior content), detached from
  any spatial position. This outline is now a modifier — a shape-template 
  that can be applied to other entities.

  Step 1: Original entity       Step 2: Extract outline      Step 3: Apply as modifier
  
    ┌───┐                          ┌───┐                       ┌───┐
    │ ⬠ │  "wood"           →     │   │  "wooden"        →    │ △ │  "wooden thing"
    │   │  (entity with            └───┘  (outline only,       └───┘  (triangle entity
    └───┘   structure)                     no fill)                    reshaped by
                                                                      wood-outline)

  RULE: abstract(e) visually = ∂(e), the boundary of the entity glyph,
        drawn as an empty outline detached from its original position.
        When applied via modify_entity, the outline encloses/reshapes 
        the target entity.

  "wood" (entity: □{⬠}) → "wooden" (modifier: □ outline)
  "circle" (entity: ●) → "circular" (modifier: ○ outline)
  △ (entity) → "△-like" (modifier: △ outline, no fill)
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

  GRADUATED (p ∈ [0,1]):
  Scale the entity to fill a PROPORTION of the frame:
    p = 1.0 → fills entire frame         "ALL" (∀)
    p ≈ 0.7 → fills most of frame        "MOST"
    p ≈ 0.5 → fills half of frame         "ABOUT HALF"  
    p ≈ 0.2 → fills small area            "FEW / SOME"
    p → 0⁺  → point-like                  "AT LEAST ONE" (∃)
  Area(σ_p(C))/Area(F) = p — the visual fill proportion IS the quantificational force.
```

### 3.12 bind: e × a → a
**"Bind a slot entity to its assertion, establishing co-reference and scope."**

```
INPUT:  Slot Entity (○_x), Assertion (containing ○_x)
OUTPUT: Assertion (with all ○_x replaced by ●_x)

GEOMETRIC REALIZATION:
  Before binding:              After binding:
  ┌─────────────────┐          ┌─────────────────┐
  │  ○_x ──→ ●      │          │  ●_x ──→ ●      │
  │                  │    →     │     ↑ bound     │
  │  ○_x ──→ ●      │          │  ●_x ──→ ●      │
  └─────────────────┘          └─────────────────┘

  ○_x = hollow mark (open slot for entity x)
  ●_x = filled mark (bound variable — all occurrences denote the same entity)
```

**Scope ordering via nesting depth:**
```
  ∀x. ∃y. R(x, y):

  ┌─────────────────────────────────┐    bind(x) — OUTER scope (wider)
  │   ┌─────────────────────────┐   │
  │   │  ●_x ───R──→ ●_y       │   │    bind(y) — INNER scope (narrower)
  │   └─────────────────────────┘   │
  └─────────────────────────────────┘
```

**Key rules:**
- ○_x must appear inside the assertion's frame before binding
- After binding, all ○_x → ●_x within that frame (co-reference)
- Nesting depth = scope ordering (outer frame = wider scope)
- See `formal-operations.md` §1.12 and `formal-grammar.md` C12

### 3.13 modify_assertion: m × a → a
**"Apply a modifier to an assertion's frame boundary, encoding epistemic stance."**

```
INPUT:  Modifier (epistemic type), Assertion
OUTPUT: Assertion (with decorated frame boundary)

GEOMETRIC REALIZATION:
  The modifier transforms the frame BOUNDARY (∂F), not the content (C) or sign (σ):

  Default:      ┌───────────────┐     Solid — direct assertion
                │  ● ──→ ●      │
                └───────────────┘

  Evidential:   ┌···············┐     Dotted — "apparently," "reportedly"
                ╎  ● ──→ ●      ╎
                └···············┘

  Emphatic:     ╔═══════════════╗     Double — "definitely," "certainly"
                ║  ● ──→ ●      ║
                ╚═══════════════╝

  Hedged:       ┌~~~~~~~~~~~~~~~┐     Wavy — "maybe," "sort of," "arguably"
                │  ● ──→ ●      │
                └~~~~~~~~~~~~~~~┘
```

**Distinction from negate (C4):**
- `negate` flips σ (solid ↔ dashed) — changes TRUTH STATUS
- `modify_assertion` decorates ∂F — changes EPISTEMIC STANCE
- Both can co-occur: dashed + dotted = "apparently NOT p"

**Key rules:**
- The modifier applies to the frame as a whole, not to individual entities or relations
- Content inside the frame is unchanged
- Multiple modifications can compose (dotted + emphatic = "reportedly and certainly")
- See `formal-operations.md` §1.13 and `formal-grammar.md` C13

### 3.14 Modal Operators (Defined Patterns)

> These are **defined abbreviations**, not new primitive operations. They compose existing operations into standard modal constructions. See `formal-foundations.md` §7.1–7.9 for formal definitions.

#### necessary(r_R, a): Necessity □_R

**"In all R-accessible worlds, a holds."**

```
DEFINED AS:  bind(w, quantify(m_∀, w, disjoin(negate(predicate(w_current, r_R, w)),
             predicate(w, r_satisfies, embed(a)))))

GEOMETRIC REALIZATION:
  Bold-border frame wrapping content, labeled with accessibility type:

  ┏━━━━━━━━━━━━━━━━━━━━━━┓
  ┃  •₁ ──r──→ •₂        ┃   r_R label on border
  ┗━━━━━━━━━━━━━━━━━━━━━━┛
```

#### possible(r_R, a): Possibility ◇_R

**"In some R-accessible world, a holds."**

```
DEFINED AS:  negate(necessary(r_R, negate(a)))

GEOMETRIC REALIZATION:
  Dashed-border frame wrapping content, labeled with accessibility type:

  ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐
  ╎  •₁ ──r──→ •₂         ╎   r_R label on border
  └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘
```

#### counterfactual(a, b): Counterfactual □→

**"In the closest worlds where a holds, b holds."**

```
DEFINED AS:  necessary(r_closest(a), b)
  WHERE:     r_closest(a) = modify_relation(abstract(embed(a)), r_closeness)

GEOMETRIC REALIZATION:
  Dashed-dot-border frame. Antecedent and consequent inside, separated:

  ╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌
  ╎  IF:   [antecedent]    ╎
  ╎  THEN: [consequent]    ╎
  ╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌
```

#### Stacking Rule

Modal operators nest: each takes assertion-in, assertion-out. Nesting depth is limited only by legibility (see OP-4, §6.5).

```
  ┏━━━━━━━━━━━━━━━━━━━━━━━━━┓     outer: necessity (r_O)
  ┃  ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐  ┃     inner: possibility (r_ability)
  ┃  ╎  •₁ ──r──→ •₂     ╎  ┃
  ┃  └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘  ┃
  ┗━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---

### 3.15 Performative Force Annotations

> Force annotations set the **illocutionary force** on assertion frames — they determine whether a claim is stated, questioned, commanded, promised, expressed, or declared. Not new operations: 0 new primitives. See `formal-foundations.md` §8.1–8.7.

#### Notation: `φ[force]{content}`

| Force | Token | ASCII | Border Convention | Example |
|-------|-------|-------|-------------------|---------|
| Assert | (default) | (none) | Single solid | "Water boils at 100°C" |
| Query | `query{...}` | `query{...}` | Single + `?` corner | "Is it raining?" |
| Direct | `direct{...}` | `direct{...}` | Double solid | "Close the door!" |
| Commit | `commit{...}` | `commit{...}` | Bold + `⊢` corner | "I promise to return" |
| Express | `express{...}` | `express{...}` | Wavy | "How beautiful!" |
| Declare | `declare{...}` | `declare{...}` | Triple + `⊨` corner | "I pronounce you married" |

#### Interaction Rules

1. **One force per frame:** Each assertion enclosure has at most one force annotation. Nested frames may have different forces.
2. **Force under negation:** `negate(query{a})` negates the content, not the force. Force is preserved.
3. **Force under conjunction:** `conjoin(query{a}, query{b})` = `query{conjoin(a, b)}` (FC2: same-force conjunction).
4. **Embedding strips force:** `embed(query{a})` produces an entity; the force is discarded (FC4).
5. **Default = assert:** Bare assertion frames `○{...}` are assertive by default.

#### Geometric Realization

```
  assert{a}        query{a}         direct{a}
  ┌──────────┐     ┌──────────┐?    ╔══════════╗
  │  content  │     │  content  │     ║  content  ║
  └──────────┘     └──────────┘     ╚══════════╝
```

### 3.16 Pragmatic Inference Notation

> Pragmatic notation documents the gap between surface (literal) and intended (implicated) meaning. The inference arrow ⟹ is **meta-syntactic** — it annotates meaning relationships, not compositional content. See `formal-foundations.md` §9.1–9.6.

#### Notation: `surface ⟹ intended`

The inference arrow ⟹ is not a Σ_UL operation. It sits outside the object language, documenting how compositional structure triggers pragmatic inferences.

#### Inference Rules

| Rule | Name | Pattern | Example |
|------|------|---------|---------|
| SI-1 | Scalar implicature | `quantify(∠p, e)` where 0 < p < 1.0 ⟹ `negate(quantify(∠1.0, e))` | "Some students passed" ⟹ "Not all passed" |
| SI-2 | Quantity scale | `modify_entity(∠warm, e)` ⟹ `negate(modify_entity(∠hot, e))` | "It's warm" ⟹ "It's not hot" |
| SI-3 | Disjunction exclusivity | `disjoin(a, b)` ⟹ `negate(conjoin(a, b))` | "A or B" ⟹ "not both" |
| CI-1 | Contrastive "but" | `conjoin_but(a, b)` ⟹ contrast(a, b) | "Poor but honest" ⟹ unexpected combination |
| CI-2 | Appositive | `modify_assertion(appositive, a)` ⟹ presupposed content | "John, who is tall, left" ⟹ John is tall (presupposed) |
| CI-3 | Indirect speech act | `query{possible(r_ability, a)}` ⟹ `direct{a}` | "Can you pass the salt?" ⟹ "Pass the salt!" |

#### Cancellability

- **Scalar/conversational (SI):** Cancellable. "Some students passed — in fact, all did."
- **Conventional (CI):** Not cancellable. *"Poor but honest — and it's not surprising" is incoherent.

#### Geometric Realization

```
  Surface                    Intended
  ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐       ╔══════════════════╗
  ╎ query{<>{ a }}   ╎  ⟹   ║  direct{ a }      ║
  └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘       ╚══════════════════╝
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
| Reflection | Mirror entire construction | Converse (role swap; derivable from predicate + invert) |

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
| negate | a → a | Flip frame boundary (solid ↔ dashed) | Assertion → Assertion |
| conjoin | a × a → a | Overlap sentence frames | Assertion² → Assertion |
| disjoin | a × a → a | Adjoin sentence frames | Assertion² → Assertion |
| embed | a → e | Shrink frame into enclosure | Assertion → Entity |
| abstract | e → m | Extract boundary properties | Entity → Modifier |
| compose | r × r → r | Concatenate directed connections | Relation² → Relation |
| invert | r → r | Reverse direction | Relation → Relation |
| quantify | m × e → a | Scale entity relative to GS | Modifier + Entity → Assertion |
| bind | e × a → a | Fill slot entity ○→● within frame | Entity + Assertion → Assertion |
| modify_assertion | m × a → a | Decorate frame boundary | Modifier + Assertion → Assertion |
| *necessary* (defined) | r × a → a | Bold-border frame + accessibility label | Relation + Assertion → Assertion |
| *possible* (defined) | r × a → a | Dashed-border frame + accessibility label | Relation + Assertion → Assertion |
| *counterfactual* (defined) | a × a → a | Dashed-dot frame, antecedent + consequent | Assertion² → Assertion |

#### Force Parameter Convention

The **illocutionary force** φ ∈ {assert, query, direct, commit, express, declare} is a frame-boundary decoration on assertions, orthogonal to content and epistemic modification:

| Force | Frame decoration | Syntactic effect |
|-------|-----------------|-----------------|
| assert (default) | Solid border | Unchanged from base syntax |
| query | Gapped border (open side) | Frame has open right side |
| direct | Arrow-out border (→) | Frame arrows point outward |
| commit | Arrow-in border (←) | Frame arrows point inward |
| express | Wavy border (~~~) | Frame uses wavy lines |
| declare | Bold double border (╔══╗) | Frame uses bold double lines |

See `grammar-book.md` §VI-C for ASCII diagrams. See `formal-foundations.md` §8.1–8.7 for formal specification.
