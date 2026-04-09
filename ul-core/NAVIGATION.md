# Navigation Guide — Universal Writing System

> How to find what you need across the 5 siblings and the Writing System.

---

## The 5 Siblings at a Glance

| Primitive | Sibling | Concern | Read this when... |
|-----------|---------|---------|-------------------|
| **Point** (0D) | [Symbology](symbology/symbol-map.md) | What the atomic marks ARE | You need to know what a symbol looks like or what symbols exist |
| **Line** (1D) | [Syntax](syntax/syntax-dictionary.md) | How marks CONNECT | You need to know if a construction is valid or how to parse one |
| **Angle** (hybrid) | [Grammar](grammar/grammar-book.md) | Rules governing CONSTRUCTIONS | You want to understand *why* meanings emerge from geometry |
| **Curve** (1D in 2D) | [Thesaurus](thesaurus/thesaurus.md) | Paths between RELATED meanings | You're looking for synonyms, antonyms, or related concepts |
| **Enclosure** (2D) | [Lexicon](lexicon/lexicon.md) | BOUNDED canonical DEFINITIONS | You need the authoritative definition of a UL construction |

Additionally:
- [Writing System](writing-system/writing-system.md) — The full specification (reading, writing, digital encoding)
- [Writer's Companion](writing-system/writers-companion.md) — The **practical pen-and-paper guide** (start here if you want to write)
- [Glyph Composition](composition/glyph-composition.md) — The **visual composition algebra** (how symbols combine to create complex glyphs, layers, rendering)
- [Synthesis](SYNTHESIS.md) — How the 5-stage pipeline works, what the system can/can't do, 10 expansion paths

---

## Quick-Start Paths

### "I want to WRITE a concept in UL"

```
1. Writer's Companion §2 — Decompose your thought into sorts (Entity, Relation, Modifier, Assertion)
2. Writer's Companion §5 — Select operations using the decision tree
3. Lexicon — Search for your concept (if it's canonical, use the listed construction)
4.   If not found → Thesaurus §VII — Find related concepts and navigate from there
5.   If building from scratch → Grammar §VII — Follow the word-formation procedure
6. Writer's Companion §3–4 — Apply handwriting and enclosure conventions
7. Writer's Companion §7 — Verify with the 5-pass reading procedure
```

### "I want to READ / PARSE something written in UL"

```
1. Writer's Companion §1, Step 4 — The 5-pass reading procedure
   (Or: Syntax §VII — The parsing algorithm)
2. Pass 1: Identify enclosures (frames and concept boundaries)
3. Pass 2: Identify connections (lines and curves between entities)
4. Pass 3: Read angles (qualitative relationships)
5. Pass 4: Identify points (entities and their positions)
6. Pass 5: Read curvature (is this a process or a static relationship?)
7. Lexicon — Look up any recognized constructions
```

### "I want to understand WHY a symbol means what it means"

```
1. Symbology §II — What are the atomic symbols and their geometric properties?
2. Grammar §II — How do symmetry groups determine parts of speech?
3. Grammar §IV — How does the Erlangen hierarchy create layers of meaning?
4. formal-foundations.md — The mathematical proofs (Σ_UL, embedding theorem, uniqueness)
```

### "I want to find SYNONYMS or related concepts"

```
1. Thesaurus §II–VI — Look up your concept in the relevant family
   (Existence → §II, Relation → §III, Quality → §IV, Process → §V, Concept → §VI)
2. Thesaurus §VII — Cross-category and abstract concepts
3. Thesaurus §VIII — Apply a transformation to navigate to related meanings
4. Grammar §IV — Understand which Erlangen level your synonymy operates at
```

### "I don't know if my construction is valid"

```
1. Syntax §V — Well-formedness rules (6 rules to check)
2. Syntax §IV — Is it an atomic, compound, or recursive sentence?
3. Writer's Companion §7 — Diagnostic table for common problems
4. Writer's Companion §10 — Common mistakes and fixes
```

### "I want to COMPOSE complex multi-level glyphs"

```
1. Glyph Composition §I — The 5 spatial relationships (containment, intersection, adjacency, separation, connection)
2. Glyph Composition §II — Composition depth layers (how deep can you go?)
3. Glyph Composition §III — Composition parameters (scale, orientation, z-order, alignment)
4. Glyph Composition §VI — Three methods: deepening, broadening, blending
5. Glyph Composition §IX — Multi-glyph patterns (ternary, radial, fractal, tabular)
```

### "I want to build a RENDERER or NL→UWS TRANSLATOR"

```
1. Glyph Composition §X — Rendering system architecture (3 components)
2. Glyph Composition §XI — NL→UWS translation pipeline (7 steps)
3. writing-system.md §8 — JSON structural encoding and CSF serialization
4. Glyph Composition §V — The formal composition algebra (GCA)
```

---

## Two Classification Systems (They're Complementary)

A common source of confusion: **Syntax** and **Grammar** classify the same objects differently.

| System | Basis | Categories | Used by |
|--------|-------|-----------|---------|
| **Sort classification** | Σ_UL algebraic signature | Entity (e), Relation (r), Modifier (m), Assertion (a) | Syntax |
| **Symmetry classification** | Erlangen Program (Klein) | Noun, Verb, Adjective, Determiner, Proper Noun | Grammar |

These are **not** competing systems. They classify **different aspects** of the same symbol:

- A symbol like → has **low rotational symmetry** → Grammar classifies it as a **Verb**
- The same symbol → has sort **Relation (r)** → Syntax says it takes two Entities and produces an Assertion via `predicate(e, r, e) → a`

**Symmetry** tells you how the symbol behaves *linguistically*.
**Sort** tells you what algebraic *operations* apply to it.

---

## Canonical vs. Illustrative Constructions

Not all constructions in the 5 siblings are equally authoritative:

| Status | Meaning | Where to find | Example |
|--------|---------|---------------|---------|
| **T1 (Geometrically Forced)** | THE unique construction for this role | Lexicon §1–6 | Point = Entity (there's no other option) |
| **T2 (Structurally Distinguished)** | The extremal/minimal construction | Lexicon §1–6 | 90° = Independence (unique zero-projection angle) |
| **T3 (Conventional)** | A valid construction, but one choice among many | Symbology §V, Grammar §VII, Thesaurus §VII | "Love = ○{•≅•}" (illustrative, not canonical) |

**Rule of thumb:** If it's in the Lexicon with a tier justification, it's canonical. If it appears in Symbology §V, Grammar §VII worked examples, or Thesaurus §VII, treat it as illustrative — a valid construction but not THE construction.

---

## The G ≠ Σ_UL Boundary

A subtle but important distinction you'll encounter:

- **G** = the geometric algebra (all geometric constructions in the plane)
- **Σ_UL⁺** = the algebraic signature (4 sorts, 13 operations)

Most geometric features map cleanly to Σ_UL terms. But some geometric properties — particularly **enclosure boundary shapes** (triangle vs. hexagon) — are geometric-only features that enter Σ_UL indirectly through the modifier sort (via `abstract(e) → m`).

When writing, this means:
- The *existence* of an enclosure is algebraic (Σ_UL sort: Entity)
- The *shape* of an enclosure boundary is geometric data (G only)
- The shape can be *referenced* in Σ_UL terms via `abstract()`, turning the entity's shape into a modifier

For the full explanation, see Lexicon §0.8.

---

## Prescribed Learning Path (Day 1–9)

For a structured progression from zero to fluent UL composition, follow this 9-day curriculum. Each day builds on the previous.

| Day | Focus | Read | Practice |
|-----|-------|------|----------|
| **1** | Geometric primitives | `foundations/paradigm.md` | Identify 5 shapes in everyday objects |
| **2** | Symbology — atomic marks | Symbology §I–IV | Draw the 7 atomic symbols |
| **3** | Syntax — connection rules | Syntax §I–III | Trace 3 connection patterns |
| **4** | Grammar — valid formations | Grammar §I–IV | Classify sorts + parts of speech |
| **5** | First writing | Writer's Companion Ex 1–3 | Write your own simple glyph |
| **6** | Compound glyphs | Writer's Companion Ex 4–10 | Conjunction, negation, embedding |
| **7** | Meaning navigation | Thesaurus §I–VI | Find 3 synonymy paths |
| **8** | Advanced operations | Writer's Companion Ex 11–19 | Bind, modal, performative, pragmatic |
| **9** | Capstone | All materials | Compose a novel complex sentence |

**Full curriculum with daily exercises:** `docs/learning/curriculum.md`
**Practice exercises with answer key:** `docs/learning/exercises.md`

---

## Document Dependency Map

```
                  ┌───────────────────────────────────────┐
                  │         formal-foundations.md           │
                  │   (Σ_UL⁺: 4 sorts, 13 ops, proofs)    │
                  └──────────────┬────────────────────────┘
                                 │
         ┌───────────┬───────────┼───────────┬────────────┐
         ▼           ▼           ▼           ▼            ▼
   ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
   │SYMBOLOGY │ │  SYNTAX  │ │ GRAMMAR  │ │THESAURUS │ │ LEXICON  │
   │(Point)   │ │(Line)    │ │(Angle)   │ │(Curve)   │ │(Enclosure│
   │What marks│ │How they  │ │Why they  │ │How they  │ │Canonical │
   │exist     │ │combine   │ │mean      │ │relate    │ │entries   │
   └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘
        │             │           │             │            │
        └──────┬──────┴─────┬─────┴──────┬──────┴────────────┘
               │            │            │
               ▼            ▼            ▼
   ┌──────────────────┐ ┌──────────────────┐
   │  GLYPH           │ │ WRITING SYSTEM   │
   │  COMPOSITION     │ │ + WRITER'S       │
   │  (visual algebra,│ │   COMPANION      │
   │   layers,        │ │ (practical use)  │
   │   rendering)     │ └──────────────────┘
   └──────────────────┘

```

All 5 siblings derive from `formal-foundations.md` and are peers — none depends on another. Glyph Composition extends the system with the visual composition algebra and rendering pipeline. The Writing System and Writer's Companion synthesize all into practical procedures.
