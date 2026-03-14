# Writer's Companion Guide

> **How to go from a thought in your head to marks on paper.**
>
> This document is the practical complement to `writing-system.md`. That document specifies *how to read* — how to extract structure from any drawing. This document specifies *how to write* — how to decide what to draw when you have something to say.

**Prerequisites:** Familiarity with the 5 geometric primitives (Point, Line, Angle, Curve, Enclosure) and the 5-pass reading procedure (`writing-system.md` §3).

**Materials:** Paper, pen. No ruler or protractor needed.

---

## 1. THE WRITING PROCEDURE: FOUR STEPS

Every act of writing in UL follows this procedure:

```
WRITING PROCEDURE

  Given: A thought, idea, claim, or meaning you want to express.

  STEP 1 — DECOMPOSE: Break the meaning into Σ_UL sorts.
           What are the THINGS? (entities)
           What CONNECTS them? (relations)
           WHAT KIND of connection? (modifiers)
           What is being CLAIMED? (assertions)

  STEP 2 — SELECT OPERATIONS: Identify which of the 11 Σ_UL operations
           combine your sorts into the intended structure.

  STEP 3 — DRAW: Realize each operation as geometry on paper.

  STEP 4 — VERIFY: Read back your drawing using the 5-pass procedure.
           Does the reading recover your intended decomposition?
           If not, revise.
```

Steps 3 and 4 are covered in `writing-system.md`. **This document teaches Steps 1 and 2** — the meaning-decomposition that the writing system delegates to the writer.

---

## 2. STEP 1 — DECOMPOSITION: From Thought to Sorts

### 2.1 The Four Questions

To decompose any thought into Σ_UL sorts, ask these four questions in order:

| Question | What you're finding | Σ_UL Sort | Geometric Primitive |
|----------|-------------------|-----------|-------------------|
| **1. What things exist in this thought?** | The nouns, subjects, objects — anything that *is* | Entity (e) | Point • |
| **2. How are they connected?** | The verbs, actions, relations — anything that *links* | Relation (r) | Line ─ or Curve ◠ |
| **3. In what manner?** | The adjectives, adverbs, qualities — anything that *qualifies* | Modifier (m) | Angle ∠ |
| **4. What is being stated?** | The complete claims — anything that is *asserted* | Assertion (a) | Enclosure ▮ |

### 2.2 The Decomposition Algorithm

Work through these steps for any natural-language thought:

**Step 1a — Identify Entities.**
Circle every noun or noun-phrase. Each becomes a point (•) or an enclosure containing sub-structure.

Ask: Is this entity *simple* (a single thing) or *compound* (a thing with internal structure)?
- Simple → Point (•)
- Compound → Enclosure containing its parts

**Step 1b — Identify Relations.**
Underline every verb, preposition, or connective. Each becomes a line or curve.

Ask: Is this relation *static* (a fixed connection) or *dynamic* (a process of change)?
- Static → Straight line (─)
- Dynamic → Curve (◠), with curvature encoding the nature of the change

Ask: Is this relation *directed* (A acts on B) or *symmetric* (A and B relate mutually)?
- Directed → Arrow (→)
- Symmetric → Plain line (─) or double arrow (↔)

**Step 1c — Identify Modifiers.**
Note every adjective, adverb, or qualifier. Each becomes an angle.

Ask: What is the *quality* of this relationship? Use the Distinguished Angle Table (§3.2) to select the nearest geometric angle.

**Step 1d — Identify Assertions.**
Group entities + relations + modifiers into complete claims. Each claim gets a sentence frame (enclosure).

Ask: How many distinct things am I *claiming*? Each gets its own frame.

### 2.3 Decomposition Examples (Simple)

**Thought:** "Something exists."

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Something" | Entity: e₁ |
| How connected? | (no connection — pure existence) | — |
| What manner? | — | — |
| What claimed? | "It exists" | Assertion (but minimal — just the entity) |

**Drawing:** A single dot.
```
  •
```

---

**Thought:** "Fire causes heat."

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Fire" and "Heat" | Entities: e₁ (fire), e₂ (heat) |
| How connected? | "causes" — directed action | Relation: r (directed, →) |
| What manner? | causation — strong directed force | Modifier: bold/double line for emphasis |
| What claimed? | "Fire causes heat" — one claim | Assertion: one frame |

**Drawing:**
```
  ┌────────────────┐
  │  •══════→•     │
  └────────────────┘
   (fire)    (heat)
```
The double line (══) emphasizes the strength of the causal relation. The arrow shows direction. The frame asserts it as a claim.

---

**Thought:** "Music and mathematics are independent."

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Music" and "Mathematics" | Entities: e₁, e₂ |
| How connected? | "are" — they have a relation | Relation: r |
| What manner? | "independent" — orthogonal, unrelated | Modifier: 90° angle |
| What claimed? | One claim about their relationship | Assertion: one frame |

**Drawing:**
```
  ┌──────────┐
  │  •       │
  │  │       │
  │  •───•   │
  └──────────┘
```
The two relations meet at 90°, encoding independence. Reading Pass 3 recovers: "The quality of the relationship between these entities is orthogonality."

---

### 2.4 Decomposition Examples (Compound)

**Thought:** "Water transforms into ice while remaining the same substance."

This has two claims woven together. Decompose them separately, then combine.

**Claim A:** "Water transforms into ice"

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Water" and "Ice" | Entities: e₁, e₂ |
| How connected? | "transforms into" — a process of change | Relation: r₁ (curve ◠, directed →) |
| What manner? | phase transition — a gradual process | Modifier: medium curvature |
| What claimed? | "Water transforms into ice" | Assertion: a₁ |

**Claim B:** "remaining the same substance"

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Water" and "Ice" (same entities) | e₁, e₂ (shared with Claim A) |
| How connected? | "remaining the same" — identity relation | Relation: r₂ (straight line, 0° — parallel to itself) |
| What manner? | "the same" — identity/agreement | Modifier: 0° angle |
| What claimed? | "they are the same substance" | Assertion: a₂ |

**Combination:** Two claims about the same entities → conjoin(a₁, a₂) → overlapping frames.

**Drawing:**
```
  ┌──────────────────────────┐
  │         ◠                │
  │       ╱   ╲              │
  │  •  ╱       ╲   •       │
  │  ├──────────────┤       │
  │  (same: 0° parallel)    │
  └──────────────────────────┘
```
Reading: Two entities connected by a curved path (process/transformation) AND by a straight path at 0° (identity). The reader recovers: "Something transforms into something else through a process, while maintaining an identity relationship."

The contextual grounding ("water," "ice," "substance") is external to UL. UL encodes the structural relationship: **transformation with preserved identity**.

---

**Thought:** "The fact that the sun rises causes the flowers to bloom."

This has an embedded assertion — "the sun rises" functions as an entity (the *cause*).

**Decomposition:**

**Inner assertion (a₁):** "The sun rises"

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Sun" | Entity: e₁ |
| How connected? | "rises" — directed upward process | Relation: r₁ (curve, directed ↑) |
| What manner? | upward, cyclical | Modifier: curvature profile |

**Embed:** embed(a₁) → e₃ (the-fact-that-the-sun-rises becomes a thing)

**Outer assertion (a₂):** "e₃ causes the flowers to bloom"

| Question | Answer | Sort |
|----------|--------|------|
| What things? | e₃ (the embedded fact) and "flowers" (e₂) | Entities |
| How connected? | "causes" — directed action | Relation: r₂ (directed →) |
| What manner? | causal, transformative | Modifier: emphasis (bold) + curve (blooming is a process) |

**Drawing:**
```
  ┌──────────────────────────────────────┐
  │  ┌──────────┐                        │
  │  │  •  ◠↑   │ ═════◠═════→ •        │
  │  └──────────┘               (flower)  │
  │   (sun rises)                         │
  └──────────────────────────────────────┘
```
The inner frame (sun rises) is shrunk and placed inline as an entity — this is the `embed` operation. It then connects via a strong curved relation to an entity outside it. The outer frame asserts the whole causal claim.

---

## 3. HANDWRITING CONVENTIONS

### 3.1 The Practical Problem

The formal definitions (see `formal-operations.md`) specify exact angles and curvature functions. But you're writing with a pen on paper. You can't draw a mathematically precise 60° angle freehand. You don't need to.

**Principle:** Handwriting operates at the **topological level** by default (connectivity and containment) with **exaggerated features** to signal when metric-level properties matter.

This means:
- If you just draw two strokes meeting at roughly some angle, a reader will note "there's an angle here" but not assign it a specific value.
- If you *exaggerate* the angle — draw it clearly wide-open or clearly sharp or clearly right-angled — the reader knows you intend a specific quality.

**The writer must deliberately signal precision. Lack of signal means "read topologically."**

### 3.2 Distinguished Angles — The Writer's Reference

These are the angles whose meanings are geometrically forced or structurally distinguished (from the Lexicon §3.1). For handwriting, you only need to distinguish between these categories:

| Angle | How to draw it | What it means | Visual cue |
|-------|---------------|---------------|------------|
| **0°** | Two strokes going the same direction (parallel) | Identity / Agreement / "the same" | Lines clearly parallel or overlapping |
| **60°** | A sharp but open angle (like one corner of an equilateral triangle) | Harmony / Balance | Draw a small equilateral triangle as guide mark |
| **90°** | Clearly perpendicular strokes — draw the ∟ symbol in the corner | Independence / Orthogonality / "unrelated" | Use the square-corner mark (∟) |
| **120°** | A wide-open angle (supplement of 60°) | Complementarity / Efficiency | Visibly wider than 90°, almost flat |
| **180°** | Two strokes going in opposite directions (pointing away from each other) | Opposition / Negation / Reversal | Strokes clearly on a straight line, pointing apart |
| **360°** | A closed loop returning to start | Completion / Full cycle | A complete loop or spiral that returns |

**Practical rule:** If your intended meaning maps to one of these six, **exaggerate it**. Draw the right angle with a corner mark. Draw parallel lines clearly parallel. Draw opposite directions clearly straight. If your meaning falls *between* these landmarks, draw the angle naturally — the reader will interpret it as a point on the quality spectrum between its two nearest landmarks.

**You do NOT need a protractor.** The six distinguished angles divide the circle into recognizable visual zones:

```
         same (0°)
           ↑
   harmony  |  harmony
    (60°)   |   (300°=returning to 60°)
            |
 compl. ----+---- indep.
 (120°)     |     (90°)
            |
   opposition (180°)
```

If two strokes look like they're going the "same way" → 0°. If they make an "L" → 90°. If they make a "V" → ~60°. If they make a wide "V" → ~120°. If they point opposite → 180°. That's sufficient precision for handwriting.

### 3.3 Curvature Conventions

Curves encode **process** — the nature of change. Freehand curvature carries meaning through three properties you can control:

| Property | How to draw | What it means |
|----------|------------|---------------|
| **Tightness** (amount of curvature) | Tight bend vs. gentle sweep | Intense/rapid change vs. gradual change |
| **Direction** (which way it curves) | Bending left vs. right, up vs. down | The direction of change (growth↑, decline↓, etc.) |
| **Consistency** (constant vs. varying) | Smooth arc vs. wobbly path | Steady process vs. irregular/turbulent process |

**The three basic curve profiles:**

```
Gentle sweep (low κ):     • ⌒ •       Gradual, smooth change
                            (wide arc)

Medium curve:              •  ◠  •     Moderate change
                            (clear arc)

Tight curve (high κ):      • ⊂ •       Rapid, intense change
                            (tight bend)
```

**Special curves:**

| Curve shape | What it encodes | When to use |
|-------------|----------------|-------------|
| **Straight line** (κ = 0) | No change — static relation | When the connection doesn't involve transformation |
| **Constant arc** (κ = constant) | Steady, uniform process | Phase transitions, uniform motion, consistent change |
| **Tightening spiral** (κ increasing) | Accelerating process | Growth, intensification, convergence |
| **Loosening spiral** (κ decreasing) | Decelerating process | Dissipation, settling, divergence |
| **S-curve** (κ changes sign) | Reversal within a process | Oscillation, back-and-forth, correction |
| **Loop** (self-intersecting) | Self-referential process | Feedback, recursion, self-modification |

**Practical rule:** Don't overthink curvature. A natural hand-drawn curve already carries meaningful curvature information. If you mean "transformation," draw a curve. If you mean "gradual transformation," draw a gentle curve. If you mean "violent transformation," draw a tight curve. Your hand knows how to do this — trust it.

### 3.4 Line Variation Conventions

Beyond direction and curvature, lines carry additional information through their visual weight:

| Line style | How to draw | What it encodes |
|------------|------------|-----------------|
| **Thin solid** (─) | Normal pen stroke | Standard relation |
| **Thick/bold** (━) | Heavy pressure or double stroke | Emphasis — strong, necessary, or causal relation |
| **Dashed** (╌╌) | Lift pen repeatedly | Uncertainty — possible, hypothetical, or weakened relation |
| **Dotted** (···) | Tap along the path | Minimal/liminal relation — barely connected, implied |
| **Double** (══) | Two parallel strokes | Equivalence, or emphasized causation |
| **Wavy** (~) | Oscillating stroke | Periodic, rhythmic, or oscillating relation |

These are conventions of clarity, not rules of the language. A reader encountering a dashed line will read "a weakened or uncertain connection" — consistent with the geometric data (discontinuous path = incomplete relation).

---

## 4. ENCLOSURE CONVENTIONS

### 4.1 When Frames Are Required

**Rule: Every complete assertion (something being claimed) should have a sentence frame.**

This is because the formal definition of Assertion (Gₐ) requires a Jordan domain — a bounded enclosing region. Without a frame, a collection of marks is a *configuration of entities and relations* but not a *claim*.

**When you can omit frames:**
- When drawing a single entity (a point, an entity-enclosure) — entities don't need assertion frames.
- When sketching informally and the page boundary serves as the implicit frame. (The physical edges of the paper are themselves an enclosure — the "page frame.")
- When nesting — an inner assertion embedded via `embed()` already has its own frame; it doesn't need a second one.

**When frames are mandatory:**
- When you make a claim you intend to be read as a standalone statement.
- When you have multiple assertions on one page that must be distinguished.
- When you want to conjoin (overlapping frames) or disjoin (adjacent frames) statements.

### 4.2 When Enclosure Shape Matters

Enclosures serve two distinct roles and the shape conventions differ:

**Role 1: Sentence Frame** (the boundary of an assertion)
- Shape is **not semantically loaded** — any closed boundary works.
- Use whatever is easiest to draw: rectangle, oval, freeform blob.
- The frame's job is to partition inside (the claim) from outside (not the claim).
- **Convention for clarity:** Use rectangles for sentence frames, to visually distinguish them from entity-enclosures.

**Role 2: Entity Enclosure** (a concept — a bounded thing with internal structure)
- Shape **can carry semantic content** via the symmetry-group hierarchy:

| Shape | Symmetry | Semantic content | When to use |
|-------|----------|-----------------|-------------|
| △ Triangle | D₃ (order 6) | Fundamental / irreducible / atomic | The entity is a basic building block |
| □ Square | D₄ (order 8) | Structural / ordered / systematic | The entity is an organized system |
| ⬠ Pentagon | D₅ (order 10) | Organic / living / self-similar | The entity is biological or self-organizing |
| ⬡ Hexagon | D₆ (order 12) | Networked / communal / efficient | The entity is a collective or network |
| ○ Circle | SO(2) (order ∞) | Universal / abstract / complete | The entity is general, ideal, or total |

**Rule of thumb:**
- If the *kind* of concept matters to your meaning, choose the shape deliberately.
- If you just need *an enclosure* (any bounded thing), use a circle — it's the easiest to draw and the most general.
- **Triangle and circle are the most commonly needed:** fundamental thing (△) vs. complete/abstract thing (○).

### 4.3 Nesting

Enclosures can contain other enclosures. This is how complex concepts are built:

```
○{△{•}} = "A complete concept containing a fundamental thing containing an existence"
```

**Reading rule:** Inner enclosures are read *before* their containers, bottom-up. The innermost entity is the most specific; the outermost frame is the most general scope.

**Writing rule:** Build from the inside out. Draw the innermost entity first, then enclose it, then enclose that, working outward to the sentence frame.

---

## 5. STEP 2 — SELECTING OPERATIONS

### 5.1 The Operation Decision Tree

After decomposition (Step 1), you have a set of sorts. Now decide how they combine. Walk this decision tree:

```
START: You have entities, relations, modifiers, and/or assertions.

Q1: Are you connecting two entities with a relation to make a claim?
    YES → predicate(e₁, r, e₂) → a
    Draw: Two marks connected by a stroke, inside a frame.

Q2: Are you applying a quality/transformation to an entity?
    YES → modify_entity(m, e) → e
    Draw: Transform the entity (scale it, rotate it, reflect it).

Q3: Are you applying a quality/transformation to a relation?
    YES → modify_relation(m, r) → r
    Draw: The angle at which the relation meets another stroke.

Q4: Are you denying a claim?
    YES → negate(a) → a
    Draw: Reflect the content inside the frame (mirror it).

Q5: Are you combining two claims that are BOTH true?
    YES → conjoin(a₁, a₂) → a
    Draw: Two overlapping frames (shared boundary region).

Q6: Are you presenting two claims where AT LEAST ONE is true?
    YES → disjoin(a₁, a₂) → a
    Draw: Two adjacent frames (touching but not overlapping).

Q7: Are you turning a whole claim into a thing you can talk about?
    YES → embed(a) → e
    Draw: Shrink the frame and place it as an entity in a larger construction.

Q8: Are you extracting the quality/shape of a thing to use as a modifier?
    YES → abstract(e) → m
    Draw: Use the entity's shape as a transformation applied to other things.

Q9: Are you chaining two relations end-to-end?
    YES → compose(r₁, r₂) → r
    Draw: Two strokes meeting at a shared point, forming one path.

Q10: Are you reversing a relation's direction?
     YES → invert(r) → r
     Draw: Reverse the arrow.

Q11: Are you saying "all X" or "some X"?
     YES → quantify(m, e) → a
     Draw: Scale the entity large (fills frame = all) or small
           (localized in frame = some).
```

### 5.2 Common Patterns

Most everyday statements fall into a small number of patterns:

**Pattern: Simple Predication** — "A does B to C"
```
Operations: predicate(e₁, r, e₂) → a
Drawing:    [  •───→───•  ]
```

**Pattern: Modified Predication** — "A gently does B to C"
```
Operations: predicate(e₁, modify_relation(m, r), e₂) → a
Drawing:    [  •───∠θ──→───•  ]
            (the angle θ encodes "gently")
```

**Pattern: Process** — "A transforms into B"
```
Operations: predicate(e₁, r_curve, e₂) → a
Drawing:    [  •  ◠  •  ]
            (the curve encodes transformation)
```

**Pattern: Conjunction** — "A does B to C AND D does E to F"
```
Operations: conjoin(predicate(e₁, r₁, e₂), predicate(e₃, r₂, e₄))
Drawing:    [  •──→──•  ]∩[  •──→──•  ]
            (overlapping frames)
```

**Pattern: Embedded Fact** — "The fact that A relates to B causes C"
```
Operations: predicate(embed(predicate(e₁, r₁, e₂)), r₂, e₃)
Drawing:    [  [•──→──•]  ══→══  •  ]
            (inner frame shrunk to entity size)
```

**Pattern: Negation** — "A does NOT relate to B"
```
Operations: negate(predicate(e₁, r, e₂))
Drawing:    [  •──→──•  ] but content reflected/mirrored
            (reflection = negation)
```

**Pattern: Quantified Claim** — "ALL things relate to B"
```
Operations: quantify(m_universal, e₁) with predicate
Drawing:    [  ●══→══•  ]
            (large entity filling frame = "all")
```

---

## 6. WORKED EXAMPLES — FULL PIPELINE

Each example follows the complete procedure: Thought → Decompose → Select Operations → Draw → Verify.

### 6.1 Example 1: "Knowledge is structured truth."

**Step 1 — Decompose:**

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Knowledge," "truth" | Entities: e₁ (knowledge), e₂ (truth) |
| How connected? | "is" — identity relation | Relation: r (straight, 0°) |
| What manner? | "structured" — a quality imposed on truth | Modifier: m (structural = □-shape) |
| What claimed? | "Knowledge is structured truth" | Assertion: one claim |

**Step 2 — Select Operations:**
- "structured truth" = modify_entity(m_structural, e_truth) → Apply □-shape to truth
- "is" = predicate with identity (0° angle)
- Result: predicate(e_knowledge, r_identity, modify_entity(m_□, e_truth))

**Step 3 — Draw:**
```
  ┌──────────────────────────┐
  │                          │
  │    ○{•} ════ □{•}       │
  │                          │
  └──────────────────────────┘
```
Left entity: ○{•} = truth (complete concept containing existence — Lexicon T2). Right entity: □{•} = structured truth (structural enclosure containing existence). The double line at 0° encodes identity ("is"). The sentence frame asserts the claim.

**Step 4 — Verify (5-pass reading):**
- Pass 1 (Enclosures): Outer frame, circle enclosure (left), square enclosure (right).
- Pass 2 (Connections): Double line connecting the two entity-enclosures.
- Pass 3 (Angles): 0° — the connection is parallel/identity.
- Pass 4 (Points): One point inside each enclosure.
- Pass 5 (Curvature): Zero — straight connection, no process.

**Reading:** "A complete concept containing an existence has an identity/equivalence relation to a structural concept containing an existence." ✓ Matches intended meaning.

---

### 6.2 Example 2: "Evolution is a process where simple things become complex things."

**Step 1 — Decompose:**

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Simple things" and "complex things" | Entities: e₁ (simple), e₂ (complex) |
| How connected? | "become" — transformation, a process | Relation: r (curve — process) |
| What manner? | From simple to complex — increasing complexity | Modifier: tightening curvature |
| What claimed? | "Evolution is [this process]" | Assertion: the whole thing is named "evolution" |

**Step 2 — Select Operations:**
- "simple things" = △{•} (minimal/fundamental entity)
- "complex things" = □{•, •, •──•} (structural entity with internal relations)
- "become" = curved relation (process)
- "evolution is" = the whole assertion is the definition of "evolution"

**Step 3 — Draw:**
```
  ┌────────────────────────────────────┐
  │                                    │
  │               ╭────────╮           │
  │              ╱          ╲          │
  │   △{•}    ╱              ╲  □{•──•}│
  │           ╱                ╲  │ • │ │
  │                                    │
  └────────────────────────────────────┘
```
Left: A triangle enclosing a single point (fundamental/simple thing). Right: A square enclosing multiple connected points (structured/complex thing). Connecting them: a wide, gentle curve (gradual process from simple to complex). Frame asserts the whole claim.

**Step 4 — Verify:**
- Pass 1: Outer frame, triangle enclosure, square enclosure.
- Pass 2: Curved path from triangle-entity to square-entity.
- Pass 3: (No angles between separate relations — single path.)
- Pass 4: Points inside each enclosure; the square contains connected points.
- Pass 5: Gentle, sustained curvature — a steady process.

**Reading:** "A fundamental entity (containing a single existence) transforms through a gradual process into a structural entity (containing multiple interconnected existences)." ✓

---

### 6.3 Example 3: "If the temperature drops, then water freezes."

This is a conditional — "if A then B." In logic: implies(A, B) = disjoin(negate(A), B).

**Step 1 — Decompose:**

**Claim A:** "The temperature drops"

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Temperature" | Entity: e₁ |
| How connected? | "drops" — directed downward process | Relation: r₁ (curve, directed ↓) |

**Claim B:** "Water freezes"

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Water" and "ice" (implied by "freezes") | Entities: e₂, e₃ |
| How connected? | "freezes" — transformation process | Relation: r₂ (curve, directed →) |

**Combination:** implies(A, B) = disjoin(negate(A), B)
- negate(A) → reflected content in A's frame
- disjoin(negate(A), B) → adjacent frames

But this algebraic decomposition is clunky for practical writing. A more natural geometric approach:

**Alternative — Causal chain:** embed(A) then predicate as cause of B.
- embed(a₁) → e_fact (the temperature dropping becomes a thing)
- predicate(e_fact, r_causes, a₂) → the fact causes freezing

**Step 2 — Select Operations:**
Using the causal chain approach: predicate(embed(a₁), r, e₃) where a₁ encodes the temperature drop.

**Step 3 — Draw:**
```
  ┌──────────────────────────────────────┐
  │                                      │
  │   ┌────────┐                         │
  │   │  •  ◠↓ │  ═══◠═══→  •  ◠  •    │
  │   └────────┘            (water) (ice) │
  │  (temp drops)                         │
  └──────────────────────────────────────┘
```
Inner frame: temperature entity with downward curve (dropping). That frame is embedded as an entity. Strong curved relation connects it to the water→ice transformation.

**Step 4 — Verify:**
- Pass 1: Outer frame, inner frame.
- Pass 2: Downward curve inside inner frame; strong curved path from inner frame to entities; curved path between two entities at right.
- Pass 3: (Angles at junctions.)
- Pass 4: Three points — one in inner frame, two in outer frame.
- Pass 5: Downward curvature (dropping); forward curvature (freezing).

**Reading:** "An embedded assertion [something descends through a process] causes [through a process] an entity to transform into another entity." ✓

---

### 6.4 Example 4: "Democracy is a system where all members collectively govern themselves."

This is complex: it involves quantification ("all"), self-reference ("themselves"), and an embedded definition.

**Step 1 — Decompose:**

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Members" (collectively, multiple), "themselves" (= the same members), "system" (the whole structure) | Entities: e₁ (members, plural), e₂ (= e₁, self-reference) |
| How connected? | "govern" — directed action; "collectively" — modifier on the group | Relation: r (directed →); self-referential (source = target) |
| What manner? | "collectively" — the group acts as one; "all" — universal quantifier | Modifier: quantify(universal, members) |
| What claimed? | "Democracy is [this system]" | Assertion: definition |

**Step 2 — Select Operations:**
- "all members" = quantify(m_universal, e_members)
- "govern themselves" = predicate(e_members, r_govern, e_members) — self-predication (a loop)
- "a system where" = embed the assertion as entity enclosed in □ (structure)
- "Democracy is" = predicate with identity (0°)

**Step 3 — Draw:**
```
  ┌──────────────────────────────────────────┐
  │                                          │
  │   ○{◯} ════ □{ ● ══◠══↺ }              │
  │  (democracy)    (all members govern      │
  │                  themselves — loop)       │
  └──────────────────────────────────────────┘
```
Left: ○{◯} — a circle containing a circle (abstract concept, generalized). Right: □{...} — a structural system containing:
  - ● — a large point (scaled up = universal quantifier = "all")
  - ◠↺ — a curved arrow looping back to itself (self-governance, reflexive relation)

The ══ at 0° between them = identity ("is").

**Step 4 — Verify:**
- Pass 1: Outer frame, circle-in-circle (left), square (right).
- Pass 2: Identity connection (══) between the two main enclosures. Loop (↺) inside square.
- Pass 3: 0° between main connection strokes.
- Pass 4: Large point inside square. Points inside circles.
- Pass 5: Curvature on the self-loop.

**Reading:** "An abstract complete concept is identical to a structural system in which a universally-quantified entity acts upon itself through a reflexive process." ✓

---

### 6.5 Example 5: "Art transforms suffering into beauty, but science transforms suffering into knowledge."

Two parallel structures with a contrast ("but" = conjunction with opposition).

**Step 1 — Decompose:**

**Claim A:** "Art transforms suffering into beauty"

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Art," "suffering," "beauty" | e₁ (art), e₂ (suffering), e₃ (beauty) |
| How connected? | "transforms...into" — mediated process | Art mediates: e₂ → e₁ → e₃ (compose two relations) |
| What manner? | Transformation — process | Curves for both relations |

**Claim B:** "Science transforms suffering into knowledge"

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Science," "suffering," "knowledge" | e₄ (science), e₅ (suffering = e₂), e₆ (knowledge) |
| How connected? | Same structure as Claim A | e₅ → e₄ → e₆ |

**Combination:** conjoin(a₁, a₂) — both are true — but with contrast (the parallel structure with different outcomes is the content).

**Step 2 — Select Operations:**
- Each claim: compose(r₁, r₂) through a mediating entity
- Combined: conjoin(a₁, a₂) — overlapping frames (shared entity = suffering)

**Step 3 — Draw:**
```
  ┌─────────────────────┬─────────────────────┐
  │                     │                      │
  │    ○{•}             │            ○{•}      │
  │   (beauty)     ○{•}│(suffering)   │        │
  │      ↑        ╱    │    ╲        ↓        │
  │       ◠      ╱     │     ╲      ◠        │
  │        ╲   ╱       │      ╲   ╱          │
  │      △{•}          │         □{•}         │
  │      (art)         │       (science)      │
  │                    │         ○{○{•}──∠60°──○}│
  │                    │        (knowledge)    │
  └─────────────────────┴─────────────────────┘
```
Two frames sharing a boundary (conjoin). Both connect from suffering (shared entity in the overlap zone) through a mediating entity (art / science) via curved relations (process/transformation) to different outcomes (beauty / knowledge). The parallel structure IS the contrast — same input, different mediator, different output.

**Step 4 — Verify:**
- Pass 1: Two frames sharing a boundary = conjunction. Multiple enclosures inside.
- Pass 2: Curved paths from shared entity through mediators to outcomes.
- Pass 3: Angles at junctions.
- Pass 4: Points within each enclosure.
- Pass 5: Curvature encodes transformation.

**Reading:** "Two claims, conjoined: [A fundamental entity mediates a process from entity X to entity Y] AND [A structural entity mediates a process from entity X to a different entity Z]." The shared entity X and parallel structure highlights contrast. ✓

---

## 7. QUICK REFERENCE — THE WRITER'S CHECKLIST

Before drawing, verify:

```
□  I can name each ENTITY in my thought          (→ Points or enclosures)
□  I can name each RELATION between them          (→ Lines or curves)
□  I know the QUALITY of each relation            (→ Angles)
□  I know which parts are CLAIMS vs. THINGS       (→ Frames vs. bare marks)
□  I've decided: static (line) or process (curve)? (→ Curvature)
□  I've decided: directed (→) or symmetric (─)?    (→ Arrows)
□  Each complete claim has a frame                (→ Enclosures around assertions)
□  Embedded facts are shrunk into entity position (→ Small inner frames)
□  Conjoined claims share frame boundaries        (→ Overlapping)
□  Disjoined claims touch but don't overlap       (→ Adjacent)
```

After drawing, verify:

```
□  Read it back: 5-pass procedure (enclosures → connections → angles → points → curvature)
□  Does the reading match my decomposition?
□  Would someone ELSE reading this recover the same structure?
□  If not: which primitive is ambiguous? Exaggerate it.
```

---

## 8. COMMON MISTAKES AND FIXES

| Mistake | Why it happens | Fix |
|---------|---------------|-----|
| **No sentence frame** | Forgot to close the boundary | Always draw a frame around complete claims |
| **Ambiguous angle** | Drew a roughly-some-angle without committing | If the angle matters, exaggerate it to the nearest distinguished angle |
| **Curve that looks like a line** | Drew too gently | If you mean *process*, make the curvature visible — a clear arc, not a near-straight stroke |
| **Embedding too large** | Inner frame same size as outer frame | Embedded assertions should be visibly *smaller* — they're entities now |
| **Can't decompose the thought** | The thought is vague or extremely compound | Break it into simpler claims first, then conjoin/compose them |
| **Too many entities** | Trying to represent every word | Only the *structural* components need entities. "The big red ball quickly rolled" → one entity (ball) + one relation (rolled) + modifiers. Not five entities. |
| **Shape anxiety** | "Did I pick the right enclosure shape?" | If unsure, use ○ (circle). It's the most general and always valid. Triangle (△) for fundamental/atomic things. Square (□) for structures. |

---

## 9. WHAT THIS GUIDE DOES NOT COVER

| Topic | Why not | Where to find it |
|-------|---------|-----------------|
| **Digitization** (JSON, CSF serialization) | This guide is for pen and paper | `writing-system.md` §8 |
| **Computer vision / font rendering** | Open engineering problem | `writing-system.md` §10 |
| **Document-scale composition** | Multi-page documents need further specification | Open problem — `writing-system.md` §10 |
| **Accessibility** (non-visual UL) | Critical but unsolved | Open problem — `writing-system.md` §10 |
| **Non-planar surfaces** | Jordan Curve Theorem fails on some surfaces | `writing-system.md` §6.2 |
| **Domain grounding** | Mapping UL structure to specific domains (physics, music, etc.) is external to UL | Depends on context — UL provides structure, grounding is the writer's choice |

---

## APPENDIX A: THE FOUR SORTS AT A GLANCE

| Sort | Symbol | What it IS | How to draw it | Example |
|------|--------|-----------|----------------|---------|
| **Entity (e)** | • or enclosure | A thing that exists | Tap (dot) or closed boundary with contents | •  ○{•}  △{•,•} |
| **Relation (r)** | ─ or ◠ | A connection between things | Straight pull (line) or curved pull | •──•  •◠• |
| **Modifier (m)** | ∠θ | A quality/transformation | The angle between two strokes | ∠0° ∠60° ∠90° ∠180° |
| **Assertion (a)** | [ ... ] | A complete claim | A closed frame around entities + relations | [•──→──•] |

## APPENDIX B: THE 11 OPERATIONS AT A GLANCE

| Operation | Signature | Plain English | How to draw it |
|-----------|-----------|---------------|----------------|
| `predicate` | e × r × e → a | "A relates to B" | Two marks connected by a stroke, inside a frame |
| `modify_entity` | m × e → e | "A transformed" | Scale, rotate, or reshape an entity |
| `modify_relation` | m × r → r | "Relates *in this way*" | The angle at which a relation meets context |
| `negate` | a → a | "NOT this claim" | Reflect the content of a frame |
| `conjoin` | a × a → a | "A AND B" | Two frames overlapping |
| `disjoin` | a × a → a | "A OR B" | Two frames touching |
| `embed` | a → e | "The fact that A" | Shrink a frame, use it as an entity |
| `abstract` | e → m | "A-shaped" / "A-like" | Use entity's shape as a transformation |
| `compose` | r × r → r | "then" (chain) | Two strokes meeting end to end |
| `invert` | r → r | "is done by" (reverse) | Reverse the arrow |
| `quantify` | m × e → a | "ALL A" or "SOME A" | Entity scaled large (all) or small (some) in frame |
