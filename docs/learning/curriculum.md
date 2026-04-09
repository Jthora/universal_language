# Universal Language — Day 1–9 Curriculum

> A structured learning path from geometric intuition to fluent UL composition.

> **Note:** Days are sequential — each day builds on the previous. Complete them in order.

---

## Overview

| Day | Theme | Core Reading | Practice |
|-----|-------|-------------|----------|
| 1 | Geometric primitives | paradigm.md | Identify 5 shapes in everyday objects |
| 2 | Symbology — marks & marks | symbol-map.md §I–IV | Draw the 7 atomic symbols |
| 3 | Syntax — how marks connect | syntax-dictionary.md §I–III | Trace 3 connection patterns |
| 4 | Grammar — valid formations | grammar-book.md §I–IV | Classify parts of speech by symmetry |
| 5 | First writing | Writer's Companion Ex 1–3 | Write your own simple glyph |
| 6 | Compound glyphs | Writer's Companion Ex 4–6 | Conjunction, disjunction, embedding |
| 7 | Meaning navigation | thesaurus.md §I–VI | Find 3 synonymy paths |
| 8 | Advanced operations | Writer's Companion Ex 11–19 | Bind, modal, performative, pragmatic |
| 9 | Capstone | All materials | Compose a novel sentence end-to-end |

---

## Day 1: Geometric Primitives

**Goal:** Internalize that UL is geometry, not vocabulary. Every meaning has a shape.

### Reading
- `foundations/paradigm.md` — full read (the core framing document)
- `foundations/universal-language-derivation.md` §1–3 (why these 5 shapes are necessary)

### Key Concepts
| Primitive | Shape | What it captures |
|-----------|-------|-----------------|
| **Point** | ● | Existence — something *is* |
| **Line** | ──→ | Relation — something *connects to* something |
| **Angle** | ∠ | Measure — *how much*, *in what way* |
| **Curve** | ◠ | Process — change over *time/parameter* |
| **Enclosure** | ○ | Boundary — *inside* vs *outside* |

### Practice
1. Look at a room. Name 5 things that are Points (entities that simply exist).
2. Name 3 relationships (Lines) between those entities.
3. Name 2 qualities (Angles) that modify those entities or relationships.
4. Name 1 process (Curve) — something changing over time.
5. Name 1 concept (Enclosure) — an abstract bounded idea like "safety" or "home."

### Self-Check
Can you explain why UL needs exactly 5 primitives, not 4 or 6? (Hint: paradigm.md §III)

---

## Day 2: Symbology — Atomic Marks

**Goal:** Learn the visual alphabet — the marks that represent each primitive.

### Reading
- `ul-core/symbology/symbol-map.md` §I–IV (Dimensionless → 2D marks)
- Pay special attention to §V (enclosure shapes) and §VII (grammatical markers)

### The 7 Atomic Symbols

| Symbol | Unicode | ASCII | Sort | Meaning |
|--------|---------|-------|------|---------|
| Dot | ● | * | Entity | A thing |
| Open dot | ○ | /0 | Entity (variable) | A placeholder |
| Arrow | → | -> | Relation | Directed connection |
| Reverse arrow | ← | <- | Relation | Reversed connection |
| Double arrow | ↔ | <-> | Relation | Symmetric connection |
| Angle mark | ∠ | @ | Modifier | Quality/measure |
| Tilde | ~ | ~ | Process (Curve) | Change/flow |

### Practice
1. On paper, draw each of the 7 symbols 5 times until they feel natural.
2. Draw the 6 enclosure shapes: ○ (circle), △ (triangle), □ (square), ⬠ (pentagon), ⬡ (hexagon), freeform.
3. For each enclosure shape, recall its symmetry group (from grammar-book.md if needed).

### Self-Check
Without looking, draw all 7 atomic symbols and label their sort.

---

## Day 3: Syntax — Connection Rules

**Goal:** Learn how marks legally connect to form expressions.

### Reading
- `ul-core/syntax/syntax-dictionary.md` §I–III (Mark adjacency, connection rules, expression formation)

### Key Rules
1. **Lines connect Points** — a `→` must connect two `●` marks.
2. **Angles modify** — `∠` attaches to a Point or Line, giving it measure.
3. **Enclosures contain** — `○{...}` wraps any valid sub-expression.
4. **Curves parameterize** — `~` indicates change along a path.

### Practice: Trace These Patterns
1. `● → ●` — simplest relation: two entities connected
2. `○{ ● → ● }` — an assertion: the relation is *claimed*
3. `∠60 ●` — modified entity: a "60-degree quality" applied to a point

### Self-Check
Is `→ → ●` valid? Why or why not? (Hint: a Line needs Points at both ends.)

---

## Day 4: Grammar — Valid Formations

**Goal:** Understand which constructions are well-formed and why.

### Reading
- `ul-core/grammar/grammar-book.md` §I–IV (Sorts, construction rules, well-formedness)

### The 4 Sorts
Every node in a UL expression has one of 4 sorts:

| Sort | Denotation | Geometric source |
|------|-----------|-----------------|
| Entity (e) | Things, concepts | Point, Enclosure (nominalized) |
| Relation (r) | Connections | Line |
| Modifier (m) | Qualities | Angle |
| Assertion (a) | Claims | Enclosure (top-level) |

### Sort-Checking Rules
- `predicate(e, r, e) → a` — two entities connected by a relation make an assertion
- `modify_entity(m, e) → e` — a modifier applied to an entity gives a (modified) entity
- `embed(a) → e` — an assertion can become an entity ("the fact that...")
- Every operation has a **type signature**. If sorts don't match, the expression is invalid.

### Practice
For each sentence, identify the sorts:
1. "Dogs are loyal" → Dogs: ?, are: ?, loyal: ?, whole sentence: ?
2. "Big dogs run fast" → Big: ?, dogs: ?, run: ?, fast: ?, whole: ?
3. "The fact that dogs are loyal makes people happy" → inner clause: ?, makes: ?, happy: ?

### Self-Check
Answers: (1) e, r, m, a; (2) m, e, r, m, a; (3) e (embedded), r, m

---

## Day 5: First Writing

**Goal:** Write your first complete UL glyphs using the 4-step procedure.

### Reading
- `ul-core/writing-system/writers-companion.md` §1–6 (the Procedure + Examples 1–3)

### The 4-Step Procedure
1. **Decompose** — break the sentence into pieces, assign each a sort
2. **Select operations** — choose which Σ_UL operations combine them
3. **Write** — express the result in UL-Script notation
4. **Verify** — check sort constraints, trace the expression back to English

### Practice: Write These
Follow the full 4-step procedure for each:
1. "Water flows" (entity + process)
2. "The sky is blue" (entity + relation + modifier-as-entity)
3. "Knowledge is structured truth" (Example 1 — follow along with the Companion)

### Self-Check
Can you deparse your written glyph back to the original English sentence?

---

## Day 6: Compound Glyphs

**Goal:** Combine simple glyphs using conjunction, disjunction, and embedding.

### Reading
- Writer's Companion Examples 4–6 (complex constructions)
- Writer's Companion §10.1–10.4 (Examples 7–10: conjoin, disjoin, negate, embed)

### Key Operations
| Operation | What it does | Example |
|-----------|-------------|---------|
| `conjoin(a, a)` | AND — both claims hold | "Love is patient AND love is strong" |
| `disjoin(a, a)` | OR — at least one holds | "Either true OR false" |
| `negate(a)` | NOT — reverse the claim | "Love is NOT indifferent" |
| `embed(a)` | Turn a claim into a thing | "The fact that water freezes" |

### Practice
1. Write "Cats are independent AND dogs are loyal" using `conjoin`
2. Write "Either the light is on OR the light is off" using `disjoin`
3. Write "It is not the case that all birds fly" using `negate`

### Self-Check
For exercise 3: Did you negate the *entire assertion* (frame inversion) rather than just one entity?

---

## Day 7: Meaning Navigation

**Goal:** Navigate the thesaurus — find related meanings through geometric paths.

### Reading
- `ul-core/thesaurus/thesaurus.md` §I–VI (meaning families, navigational paths)

### Key Idea
The thesaurus organizes meanings not alphabetically but *geometrically*. Related concepts share structural features:
- **Synonymy** = similar structure, different labels
- **Antonymy** = same structure, negated or inverted
- **Hypernymy** = enclosure containment (general → specific)

### Practice
1. Find 3 synonyms for "connect" using the thesaurus paths.
2. Find the antonym path: starting from "presence," navigate to "absence."
3. Find a hypernymy chain: "animal" → "dog" → "terrier" using enclosure containment.

### Self-Check
Can you draw the geometric path (sequence of operations) that transforms one meaning into its synonym?

### Notation Bridge
The thesaurus uses a compact notation for meaning relationships. Here's how it maps to the formal notation you've learned:

| Thesaurus Notation | Formal Operation | Example |
|-------------------|-----------------|---------|
| `A → B` (arrow) | `compose(r₁, r₂)` | "cause → effect" is relation composition |
| `A ⊂ B` (containment) | `modify_entity(m, e)` | "dog ⊂ animal" is enclosure |
| `A ↔ B` (bidirectional) | `invert(r)` | "buy ↔ sell" swaps direction |
| `¬A` (negation) | `negate(a)` | "presence → absence" is boundary inversion |

---

## Day 8: Advanced Operations

**Goal:** Master the advanced operations: binding, modality, performatives, and pragmatics.

### Reading
- Writer's Companion Examples 11–19 (all advanced examples)
- `foundations/formal-foundations.md` §6 (bind), §7 (modality), §8 (performatives), §9 (pragmatics)

### Binding (Ex 11)
`bind(e, a)` — "For every x, ..." Creates co-reference across an assertion.
- Variable slots `○_x` mark where the variable appears
- Bound references `●_x` mark where it's used
- Example: "Every student read some book" uses nested `bind` + `quantify`

### Modality (Ex 14–16)
- `□{a}` = necessarily a (in all accessible worlds)
- `<>{a}` = possibly a (in some accessible world)
- `[]->{p}{q}` = if p were true, q would hold (counterfactual)

### Performatives (Ex 17–18)
Six illocutionary forces: assert, query, direct, commit, express, declare
- `assert{...}` = stating a fact (default)
- `query{...}` = asking a question
- `direct{...}` = giving a command

### Pragmatics (Ex 19)
Surface meaning ≠ intended meaning:
- "Can you pass the salt?" surface = query about ability, intended = request (direct)

### Practice
1. Write "It is possible that it will rain" using `<>{...}`
2. Write "Close the door!" using `direct{...}`
3. Write "Every cat has some favorite spot" using `bind` twice

---

## Day 9: Capstone

**Goal:** Compose a novel, complex sentence using everything you've learned.

### The Challenge
Choose a sentence of your own that uses at least 5 different operations. Suggestions:
- "If everyone were kind, the world would necessarily be peaceful"
  (bind + counterfactual + necessity + predicate + modify_entity)
- "Can you tell me whether she definitely left?"
  (query force + embed + modify_assertion + predicate)
- "Some birds can fly, but penguins cannot"
  (quantify + conjoin + negate + predicate + abstract)

### Procedure
1. **Decompose** — identify every element, assign sorts
2. **Select operations** — list all operations needed in application order
3. **Write** — produce the full UL-Script expression
4. **Verify** — check every sort constraint, read it back aloud
5. **Render** — if you have UL Forge installed, parse and render to SVG

### Done?
If you can complete the capstone, you are ready to:
- Contribute to UL-Core documentation
- Use UL Forge programmatically (see `docs/learning/developer-quickstart.md`)
- Explore the frontier: modality, metaphor, gauge bundles (`frontier/`)
