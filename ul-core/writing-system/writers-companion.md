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

  STEP 2 — SELECT OPERATIONS: Identify which of the 13 Σ_UL⁺ operations
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
    Draw: Flip the frame boundary (solid → dashed = denied).

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

Q11: Are you saying "all X", "most X", or "some X"?
     YES → quantify(m, e) → a
     Draw: Scale the entity to fill a PROPORTION of the frame:
           p=1 fills entire frame (all), p≈0.7 fills most (most),
           p→0⁺ point-like (some).

Q12: Do multiple positions in the claim refer to the SAME entity?
     YES → bind(e_x, a) → a
     Draw: Place hollow marks (○_x) at each position. Binding fills
           them (●_x), showing all refer to the same entity.
           Scope = nesting depth (outer frame = wider scope).

Q13: Are you marking your CERTAINTY, EVIDENCE, or STANCE toward the claim?
     YES → modify_assertion(m, a) → a
     Draw: Change the frame boundary style:
           dotted (···) = evidential ("apparently")
           double (═══) = emphatic ("definitely")
           wavy (~~~) = hedged ("maybe")
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
Drawing:    ┌╌╌╌╌╌╌╌╌╌╌╌╌╌┐
            ╎  •──→──•    ╎  (dashed frame = denied)
            └╌╌╌╌╌╌╌╌╌╌╌╌╌┘
            (boundary inversion = negation)
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
- negate(A) → flip frame boundary of A (solid → dashed)
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

### 6.6 Example 6: "Love is patient." (Property Encoding)

This example is critical because it exposes a common ambiguity: **is "patient" a second entity, a modifier on the relation, or a modifier on the entity?** The decomposition procedure handles this.

**Step 1 — Decompose:**

| Question | Answer | Sort |
|----------|--------|------|
| What things? | "Love" | Entity: e₁ (love) |
| How connected? | "is" — but "is patient" attributes a quality, not a connection to a second thing | This is NOT a two-entity predication. "Patient" is not a thing; it's a quality. |
| What manner? | "patient" — a quality of endurance, sustained over time | Modifier: m (patience as a quality applied to the entity) |
| What claimed? | "Love has the quality of patience" | Assertion: one claim |

**The key decision:** "Love is patient" is **not** "Love relates to Patience." It's "Love, modified by patient-quality." The word "is" here means "has the property of," not "is identical to" or "connects to."

**How to tell the difference:**
- "Love **is** patience" → identity between two entities → `predicate(e₁, r_identity, e₂)` — use 0° line between two enclosures
- "Love **is patient**" → entity has a quality → `modify_entity(m, e)` — transform the entity itself
- "Love **acts** patiently" → relation has a quality → `modify_relation(m, r)` — angle on the relation

**Test:** Can you replace "is [adjective]" with "has the property of [noun]"? If yes → `modify_entity`. "Love has the property of patience" ✓ → modify_entity.

**Step 2 — Select Operations:**
- `modify_entity(m_patient, e_love)` → apply patience-quality to love
- What IS patience geometrically? Patience is endurance through time — a sustained, gentle process. Geometrically: a long, gentle curve (low curvature, extended arc). As a modifier, it imposes this "sustained gentle arc" quality on whatever it modifies.
- The modified entity is then asserted (framed).

**Step 3 — Draw:**
```
  ┌──────────────────────────┐
  │                          │
  │     ○{ •  ⌒⌒⌒ }        │
  │                          │
  └──────────────────────────┘
```
A circle (○ — abstract/complete concept) containing a point (existence) and a long gentle curve (sustained process). The curve is *inside* the entity, not connecting to a second entity — it modifies the entity's internal character. The gentleness (low curvature) and length (sustained) encode patience.

**Step 4 — Verify (5-pass reading):**
- Pass 1 (Enclosures): Outer sentence frame. Inner circle enclosure.
- Pass 2 (Connections): A gentle curve inside the circle — not connecting two entities, but characterizing the enclosed space.
- Pass 3 (Angles): No sharp angles — the curve is smooth throughout.
- Pass 4 (Points): One point inside the circle.
- Pass 5 (Curvature): Low, sustained curvature — a gentle, enduring process.

**Reading:** "An abstract/complete concept containing an existence characterized by a gentle, sustained process." = "A complete thing whose nature is patient endurance." ✓

**Why other decompositions fail:**

| Alternative | What it reads as | Why it's wrong |
|-------------|-----------------|----------------|
| •──→──• (two entities linked) | "Love relates to patience" | "Patient" isn't a thing — it's a quality |
| •──∠θ──→──• (modified relation) | "Love acts in a patient way toward something" | There's no second entity being acted upon |
| ○{•} ══ ○{•} (identity between two concepts) | "Love is patience" | Close, but "is patient" ≠ "is patience" — the first is a property, the second is identity |

---

## 7. WHEN THE READING DOESN'T MATCH: THE REVISION LOOP

### 7.1 The Problem

You decomposed a thought, selected operations, and drew it. You read it back (5-pass procedure) and the reading **doesn't match** your intent. Now what?

**Don't start over.** Diagnose first.

### 7.2 Diagnostic: What Went Wrong?

The mismatch will be in exactly one of these categories:

| Symptom | Diagnosis | Fix |
|---------|-----------|-----|
| **Too few entities** — reading says "one thing" but you meant two | You drew a modifier where you needed a predicate | Add a second entity (•) and connect them with a relation |
| **Too many entities** — reading says "A relates to B" but you meant "A has quality B" | You drew a predicate where you needed a modifier | Remove the second entity; make it a quality modifying the first |
| **Wrong relation type** — reading says "static connection" but you meant "transformation" | You drew a line where you needed a curve | Redraw the path with visible curvature |
| **Missing frame** — reading finds entities and relations but no assertion | You forgot to close the sentence boundary | Draw a frame around the complete claim |
| **Angle not read** — reading doesn't capture the quality you intended | Your angle isn't exaggerated enough | Redraw with a more obvious angle; add a corner mark (∟) for 90° |
| **Reading finds extra structure** — more enclosures or intersections than intended | Your strokes accidentally cross or close | Separate strokes that shouldn't intersect; open strokes that shouldn't enclose |
| **Nesting ambiguity** — can't tell if inner enclosure is an entity or a sub-assertion | Inner frame is the same size as outer frame | Shrink the embedded assertion visibly; make inner frame clearly smaller |
| **Direction ambiguity** — can't tell which entity is subject and which is object | No arrowhead on the relation | Add an arrowhead (→) on directed relations |

### 7.3 The Revision Procedure

```
REVISION PROCEDURE

  1. READ BACK your drawing using the 5-pass procedure.
  2. COMPARE the reading to your original decomposition (Step 1 table).
  3. IDENTIFY which sort is wrong:
     - Wrong entities? → Revise Step 1a.
     - Wrong relations? → Revise Step 1b.
     - Wrong modifiers? → Revise Step 1c.
     - Wrong assertions? → Revise Step 1d.
  4. REDRAW only the affected part, not the entire construction.
  5. READ BACK again. Repeat until reading matches decomposition.
```

### 7.4 When Multiple Decompositions Are Valid

Sometimes a thought genuinely has multiple valid decompositions. "Love is patient" could be:
- `modify_entity(m_patient, e_love)` — love has the quality of patience
- `predicate(e_love, r_identity, e_patience)` — love equals patience (different claim!)

Both are valid UL. They express **different structural claims**. If your reading matches one decomposition but you intended the other, the issue isn't your drawing — it's your decomposition. Go back to Step 1 and be more precise about what you're claiming.

**The rule:** If the reading matches *some* valid decomposition of your thought, and you can't distinguish between decompositions, your thought may be structurally ambiguous in natural language. UL forces you to commit to one structure. This is a feature, not a bug.

---

## 8. RESOLVING COMMON AMBIGUITIES

Several points of apparent contradiction exist between this guide and the other UL documents. These are resolved here with explicit decisions for practical writing.

### 8.1 "Is" — The Three Readings

The word "is" in natural language is structurally ambiguous. UL forces you to pick:

| Natural language | Structure | UL operation | How to draw |
|-----------------|-----------|-------------|-------------|
| "A **is** B" (identity) | Two things are the same | `predicate(e₁, r_0°, e₂)` | •══• at 0° (parallel/identity) |
| "A **is** [adjective]" (property) | A thing has a quality | `modify_entity(m, e)` | Transform the entity itself |
| "A **is** [verb]-ing" (process) | A thing is undergoing change | `predicate(e, r_curve, e')` | Curved relation between entity states |

### 8.2 When Is a Frame Required?

**Formal rule** (from `formal-operations.md`): An assertion (Gₐ) requires a Jordan domain — a closed bounding region.

**Practical rules for pen and paper:**

| Situation | Frame required? | Why |
|-----------|----------------|-----|
| Making a standalone claim | **Yes** — always draw a frame | The frame IS the assertion boundary |
| Sketching a single entity for reference | No — just draw the entity | Entities aren't claims |
| Writing on a blank page with one claim | **Technically yes**, but the page edge serves as implicit frame | If there's only one claim, the whole page is the frame |
| Multiple claims on one page | **Yes** — each claim needs its own frame | Otherwise you can't distinguish where one claim ends and another begins |
| Embedded fact inside a larger claim | The embedded assertion has its own (shrunk) frame | `embed(a)` preserves the inner frame |

**When in doubt, draw a frame.** It's never wrong to have one, and it's always wrong to omit one when the claim is ambiguous.

### 8.3 When Does Enclosure Shape Matter?

**Two roles, two rules:**

| Role | Shape matters? | Which shapes? | Default |
|------|---------------|---------------|---------|
| **Sentence frame** (assertion boundary) | **No** — any closed shape works | Rectangle recommended for visual clarity | Rectangle |
| **Entity enclosure** (a concept) | **Yes, if you want to encode the kind of concept** | △=fundamental, □=structural, ⬠=organic, ⬡=networked, ○=universal | Circle ○ (most general) |

This resolves the apparent contradiction between documents: the Grammar's claim that "shape is always semantic" applies to **entity enclosures** where the symmetry group carries meaning. The Writing System's claim that "frames are shape-neutral" applies to **sentence frames** which are structural boundaries, not content.

### 8.4 Circle: Enclosure or Curve?

A circle can be read as:
- **An enclosure** (a bounded concept) — when it contains other marks
- **A curve** (a cyclic process) — when it stands alone as a path between entities or has visible start/endpoints
- **Both** — when a cyclic process defines a concept (the process IS the boundary)

**How to disambiguate when writing:**
- If you mean **concept/enclosure**: put something inside it (even a single dot)
- If you mean **cyclic process**: draw it as a path connecting entities, or leave it empty with visible pen direction
- If you mean **both**: draw a circle with contents that traces along the boundary (the content follows the process path)

### 8.5 Handwriting Precision: How Sloppy Is Too Sloppy?

**The tolerance rule:** UL reading operates at the **topological level by default**. This means:
- A sloppy circle and a perfect circle are the same enclosure ✓
- A wobbly line and a straight line are the same relation ✓
- Connected marks and disconnected marks are different ✗ (topology distinguishes these)

**When precision matters:** If you intend a **specific distinguished angle** (0°, 60°, 90°, 120°, 180°), you must **exaggerate** it past the point of topological ambiguity:

| You mean | What to draw | What NOT to do |
|----------|-------------|---------------|
| 0° (identity) | Clearly parallel lines | Two lines at "roughly 10°" — ambiguous |
| 90° (independence) | Clear L-shape with corner mark (∟) | An angle that could be 70° or 110° |
| 180° (opposition) | Lines clearly in a straight line, pointing apart | An obtuse angle that could be 160° |

**The middle ground:** Angles that don't match a distinguished value are read as points on a continuous quality spectrum. Drawing "roughly 45°" means "a quality between identity and independence" — which is a valid meaning. You only create ambiguity when your angle is *close to but not clearly at* a distinguished value.

### 8.6 Grounding: How to Label Your Entities

UL encodes **structure**, not **things**. A point (•) is "an existence" — not "a cat" or "love" or "electron." The label you assign is **external grounding**.

**For practical writing:**
- You MAY write natural-language labels near your entities as annotations: `(love)`, `(water)`, `(temperature)`
- These labels are **not part of the UL text** — they're marginalia for your own reference
- Two people grounding the same UL structure to different domains is not a contradiction — it's the universality working as intended
- When writing for yourself: label freely. When writing for structural analysis: omit labels and let the geometry speak

**The convention used in this guide:** Labels appear in parentheses below the construction and are always marked as commentary, never as part of the drawing.

---

## 9. COMPOUND STATEMENTS — COMBINING CLAIMS

Single assertions are the atoms of UL writing. This section teaches you to combine them using the four logical connectives and conditional embedding.

### 9.1 The Four Geometric Connectives

Every logical connective is a **spatial relationship between sentence frames**. No extra notation needed — the geometry IS the logic.

| Connective | Natural language | Frame relationship | Drawing |
|------------|-----------------|-------------------|---------|
| **AND** (conjoin) | "A and B are both true" | Frames **overlap** (shared interior region) | Two frames with shared boundary area |
| **OR** (disjoin) | "A or B (or both) is true" | Frames **touch** (boundaries contact, no shared interior) | Two frames side by side, edges touching |
| **NOT** (negate) | "A is not true" | Frame boundary **flipped** (solid → dashed) | Dashed frame boundary = denied |
| **IF...THEN** (implication) | "If A then B" | A's frame **embedded inside** B's frame | Inner frame (premise) shrunk to entity, connected to conclusion |

```
AND (conjoin):              OR (disjoin):

  ┌─────────┬────────┐     ┌─────────┐┌─────────┐
  │  •──→•  │  •──→• │     │  •──→•  ││  •──→•  │
  │    a₁   │  a₂    │     │    a₁   ││    a₂   │
  └─────────┴────────┘     └─────────┘└─────────┘
  (shared boundary =       (touching boundary =
   both must be true)       either can be true)


NOT (negate):               IF-THEN (implication):

  ┌─────────────┐          ┌────────────────────────┐
  │  •←──•      │          │  ┌──────┐              │
  │  (reflected) │          │  │ •──→•│ ══→══ •──→• │
  └─────────────┘          │  └──────┘  (then)      │
  (mirror of •──→•)        └────────────────────────┘
                           (embedded premise causes
                            conclusion)
```

### 9.2 How Overlap and Adjacency Work

**Conjunction (AND):** Two frames share part of their boundary or interior. The shared region represents what both claims hold in common. If the claims share entities, those entities sit in the overlap zone.

```
  "Fire is hot AND fire is bright"

  ┌──────────────┬──────────────┐
  │              │○{•}          │
  │  ○{•}       │(bright)      │
  │ (hot)  •    │              │
  │    ↖  (fire)│  ↗           │
  │      ╲    ╱ │              │
  │              │              │
  └──────────────┴──────────────┘

  The entity "fire" (•) sits in the overlap zone —
  shared between both claims.
```

**Disjunction (OR):** Two frames touch at their boundaries but don't share interior. Reading either frame independently yields a valid statement; the spatial adjacency signals "at least one of these."

```
  "The light is red OR the light is green"

  ┌──────────────┐┌──────────────┐
  │  •───→───○   ││  •───→───○   │
  │ (light) (red)││(light)(green)│
  └──────────────┘└──────────────┘

  Frames touch but don't overlap.
  At least one claim holds.
```

### 9.3 Negation in Practice

Negation is defined as **boundary inversion** — flipping the assertional sign σ ∈ {⊕, ⊖} (see `formal-operations.md` §1.4). In practice:

**Method 1 — Boundary flip (formal):** Change the frame boundary from solid (asserted, σ = ⊕) to dashed (denied, σ = ⊖). The content inside the frame is unchanged — only the assertion's sign is flipped.

**Method 2 — Negation mark (practical shorthand):** Draw a diagonal strike-through (╲) across the frame or the relation being negated. This is a conventional shorthand, not a geometric primitive — but it's unambiguous and much easier to draw.

```
  "Fire is NOT cold"

  Method 1 (formal):          Method 2 (practical):
  ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐            ┌──────────────┐
  ╎  •──══→─○     ╎            │  •──══→─○    │
  ╎  (dashed=denied)╎           │      ╲       │
  └╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘            │  (crossed out)│
                              └──────────────┘
```

**Scope matters:** Negation applies to whatever frame encloses the denied content (dashed boundary).
- NOT(A AND B) ≠ (NOT A) AND B — the negation frame must enclose the conjunction, not just one claim.

### 9.4 Conditionals and Implication

"If A then B" = "NOT-A OR B" algebraically, but geometrically it's more natural as **causal embedding**:

1. Write assertion A (the premise) in its own frame.
2. `embed(A)` — shrink it to entity size.
3. Connect the embedded premise to the conclusion with a directed relation.
4. Frame the whole thing as a new assertion.

This was demonstrated in Example 3 (§6.3, "If the temperature drops, then water freezes"). The embedded inner frame IS the "if" clause; the directed relation IS the "then."

### 9.5 Compound Statement Patterns Summary

| Pattern | Natural language | Operations | Frame geometry |
|---------|-----------------|-----------|----------------|
| Simple AND | "A and B" | `conjoin(a₁, a₂)` | Overlapping frames |
| Simple OR | "A or B" | `disjoin(a₁, a₂)` | Adjacent frames |
| Simple NOT | "not A" | `negate(a)` | Dashed frame boundary |
| IF-THEN | "if A then B" | `predicate(embed(a₁), r, ...)` | Embedded premise → conclusion |
| BUT (contrastive AND) | "A but B" | `conjoin(a₁, a₂)` with 180° between | Overlapping frames + opposing relations |
| BECAUSE | "A because B" | `predicate(embed(a_B), r_cause, embed(a_A))` | Two embedded facts linked causally |
| NEITHER-NOR | "neither A nor B" | `negate(disjoin(a₁, a₂))` | Negated adjacent frames |

---

## 10. COMPOUND WORKED EXAMPLES

### 10.1 Example 7: "Love is patient AND love is strong."

Two properties of the same entity — conjunction with a shared subject.

**Decompose each claim separately:**

Claim A: "Love is patient" → `modify_entity(m_patient, e_love)` (gentle curve inside circle — §6.6)

Claim B: "Love is strong" → `modify_entity(m_strong, e_love)` — strength geometrically is emphasis/boldness. A bold point or thick boundary.

**Combine:** `conjoin(a₁, a₂)` — overlapping frames, shared entity.

**Draw:**
```
  ┌──────────────────┬──────────────────┐
  │                  │                  │
  │  ○{ •  ⌒⌒⌒ }   │   ○{ ●  }       │
  │   (patience:     │    (strength:    │
  │    gentle curve) │     bold point)  │
  │                  │                  │
  └──────────────────┴──────────────────┘
```
Two overlapping frames. The entity (love) is implicit in both — the circle enclosure ○ appears in each. Left frame: patience as gentle curve inside. Right frame: strength as bold/enlarged point (●) inside. The shared boundary asserts both are true of the same thing.

**Verify:**
- Pass 1: Two overlapping frames = conjunction. Circle enclosures inside each.
- Pass 2: No relation between frames — each is a self-contained property claim.
- Pass 3: No angles.
- Pass 4: Points inside each circle — one gentle, one bold.
- Pass 5: Left curve = sustained process. Right = no curvature.

**Reading:** "A complete concept characterized by gentle process AND a complete concept characterized by emphasis." = "Something is both patient and strong." ✓

---

### 10.2 Example 8: "Either the answer is true OR the answer is false."

Exclusive disjunction — exactly one holds.

**Decompose:**

Claim A: "The answer is true" → `predicate(e_answer, r_identity, e_truth)`
- e_answer = • (an entity)
- e_truth = ○{•} (truth — Lexicon T2: maximally symmetric enclosure of existence)
- r_identity = 0° connection

Claim B: "The answer is false" → `predicate(e_answer, r_identity, negate(e_truth))`
- e_false = reflected ○{•} (negation of truth)

**Combine:** `disjoin(a₁, a₂)` — adjacent frames.

**Draw:**
```
  ┌──────────────────┐┌──────────────────┐
  │                  ││                  │
  │  •════○{•}      ││  •════○{•}      │
  │ (answer)(truth)  ││(answer)(false:   │
  │                  ││ dashed enclosure) │
  └──────────────────┘└╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘
```
Two frames touching but not overlapping. Left: entity connected at 0° to ○{•} (truth). Right: same entity connected at 0° to ○{•} with dashed boundary (falsehood — denied enclosure). Adjacent frames = at least one is true = disjunction.

**Verify:**
- Pass 1: Two adjacent frames = disjunction.
- Pass 2: Identity connections (0°) in each frame.
- Pass 3: 0° angles — identity.
- Pass 4: Points and enclosures.
- Pass 5: No curvature — static claims.

**Reading:** "Either [an entity is identical to truth] OR [an entity is identical to reflected-truth]." ✓

---

### 10.3 Example 9: "Love is NOT indifferent."

Negation of a property — negate a complete assertion.

**Decompose:**

Inner claim: "Love is indifferent" → What is indifference? 90° = independence/orthogonality. Indifference is the quality of being unrelated — a 90° modifier.

So: `modify_entity(m_90°, e_love)` = love with orthogonal/independent quality.

Negate: `negate(a_inner)` = reflect this claim.

**Draw:**
```
  ┌───────────────────────┐
  │                       │
  │   ○{ •  ∟ }  ╲       │
  │    (inner claim       │
  │     struck through)   │
  │                       │
  └───────────────────────┘
```
A circle enclosure containing an entity modified by a visible right-angle mark (∟ = 90° = indifference). The diagonal strike-through (╲) across the frame negates the claim. Alternatively, use a dashed frame boundary to denote denial.

**Verify:**
- Pass 1: One frame containing a circle enclosure.
- Pass 2: No connection between external entities — internal structure.
- Pass 3: 90° angle inside the circle.
- Pass 4: Point inside circle.
- Pass 5: No curvature.

**Reading:** "NOT [a complete concept containing an existence characterized by orthogonal/independent quality]." = "Not an indifferent thing." ✓

---

### 10.4 Example 10: "Peace emerges when harmony overcomes conflict."

This combines process (emerges), conditional (when), and embedded assertions.

**Decompose:**

Inner claim A: "Harmony overcomes conflict"
- e₁ = △{•,•,•} (harmony — equilateral triangle, Lexicon: perfect balance)
- e₂ = •∠180°• (conflict — two entities in opposition)
- r = directed curve (overcoming is a directed process)
- a₁ = `predicate(e₁, r_curve, e₂)`

Outer claim: "Peace emerges when [A]"
- e_peace = ○{◠↑} (peace — a complete concept with upward-tending process, resolving into calm)
- The "when" = embed(a₁) as a cause/condition
- "emerges" = a process (curve) from the embedded condition to peace

**Draw:**
```
  ┌──────────────────────────────────────────┐
  │                                          │
  │   ┌────────────────────┐                 │
  │   │  △{•,•,•} ══◠══→  │                 │
  │   │ (harmony)   •∠180°•│   ◠◠◠→  ○{◠↑} │
  │   │          (conflict) │         (peace) │
  │   └────────────────────┘                 │
  │   (harmony overcomes conflict)           │
  │                                          │
  └──────────────────────────────────────────┘
```
Inner frame: harmony (equilateral triangle) connected by a directed curve to conflict (two entities at 180° opposition). This frame is embedded (shrunk to entity position). A curved path connects it to peace (circle containing an upward-tending process). Outer frame asserts the whole causal structure.

**Verify:**
- Pass 1: Outer frame, inner frame, triangle enclosure, circle enclosure.
- Pass 2: Curved directed path in inner frame; curved path from inner frame to circle.
- Pass 3: 180° opposition between conflict entities. 60° (equilateral) within triangle.
- Pass 4: Three points in triangle, two in conflict, one in peace circle.
- Pass 5: Curvature throughout — dynamic, process-rich statement.

**Reading:** "An embedded claim [a balanced fundamental structure overcomes an opposed pair through a process] leads through a process to a complete concept containing upward change." = "Peace emerges from harmony overcoming conflict." ✓

### 10.5 Example 11: "Every student read some book" (Variable Binding + Quantification)

**Decompose:**

| Question | Answer | Sort |
|----------|--------|------|
| Who reads? | student (x) — bound variable | Entity (e) |
| What is read? | book (y) — bound variable | Entity (e) |
| What is the relation? | read | Relation (r) |
| How many students? | every (∀, p=1) | Modifier (m) |
| How many books? | some (∃, p→0⁺) | Modifier (m) |

**Select Operations:**
- `predicate(e_x, r_read, e_y) → a₁` — core claim
- `quantify(m_∃, e_y) → a_inner` — existential quantification on book
- `quantify(m_∀, e_x) → a_outer` — universal quantification on student
- `bind(e_x, ...)` and `bind(e_y, ...)` — co-reference for variables

**Formal decomposition (∀ > ∃ reading):**
```
bind(x, quantify(m_∀, x,
  bind(y, quantify(m_∃, y,
    predicate(●_x, r_read, ●_y)))))
```

**Draw:**
```
  ┌─────────────────────────────────────┐  bind(x), ∀x — OUTER scope
  │  ●_x fills frame (∀)               │
  │  ┌───────────────────────────┐      │
  │  │  ●_y small (∃)            │      │  bind(y), ∃y — inner scope
  │  │  ●_x ───read──→ ●_y      │      │
  │  └───────────────────────────┘      │
  └─────────────────────────────────────┘
```

**Verify:**
- Pass 1: Outer frame (solid, ⊕), inner frame (solid, ⊕)
- Pass 2: Two filled marks (●_x, ●_y) — bound variables. ●_x fills outer frame → ∀x. ●_y small in inner frame → ∃y.
- Pass 3: Directed relation "read" from ●_x to ●_y.
- Pass 4: Labels x and y match across occurrences — co-reference confirmed.
- Pass 5: Nesting depth: x outer (wider scope), y inner (narrower scope).

**Reading:** "For every student x, there exists some book y such that x read y." ✓

**Teaching point:** Scope = nesting depth. The ∀ > ∃ reading is unambiguous in the visual diagram — no scope ambiguity.

### 10.6 Example 12: "Apparently she didn't leave" (Assertion Modification + Negation)

**Decompose:**

| Question | Answer | Sort |
|----------|--------|------|
| Who? | she | Entity (e) |
| What happened? | leave | Relation (r) |
| Is it affirmed or denied? | denied (σ = ⊖) | Negation |
| What is the speaker's stance? | evidential ("apparently") | Modifier (m) |

**Select Operations:**
- `predicate(e_she, r_leave, e_place) → a₁` — core claim "she left"
- `negate(a₁) → a₂` — denial: "she didn't leave"
- `modify_assertion(m_evidential, a₂) → a₃` — evidential frame: "apparently she didn't leave"

**Formal decomposition:**
```
modify_assertion(m_evidential, negate(predicate(e_she, r_leave, e_place)))
```

**Draw:**
```
  ┌···················┐    Dotted boundary = evidential (m_evidential)
  ╎  ┌╌╌╌╌╌╌╌╌╌╌╌╌┐  ╎    Dashed interior = negated (σ = ⊖)
  ╎  ╎ • ──leave→ • ╎  ╎    Content unchanged
  ╎  └╌╌╌╌╌╌╌╌╌╌╌╌┘  ╎
  └···················┘

  Or compacted (single frame combining both):
  ┌╌·╌·╌·╌·╌·╌·╌·╌·┐    Dashed (denied) + dotted (evidential)
  ╎  • ──leave──→ •  ╎
  └╌·╌·╌·╌·╌·╌·╌·╌·┘
```

**Verify:**
- Pass 1: Frame with dotted boundary (evidential), dashed interior (negated).
- Pass 2: Two marks (she, place) connected by "leave" relation.
- Pass 3: No angle modifications.
- Pass 4: Boundary decoration: dotted → modify_assertion(m_evidential). Dashed → negate.
- Pass 5: Read inside-out: predicate → negate → modify_assertion.

**Reading:** "Evidentially: it is denied that she left." = "Apparently she didn't leave." ✓

**Teaching point:** Negation (σ) and assertion modification (∂F) are orthogonal. Dashed = truth status flipped. Dotted = epistemic stance. Both can co-occur.

### 10.7 Example 13: "Most birds can fly" (Graduated Quantification)

**Decompose:**

| Question | Answer | Sort |
|----------|--------|------|
| What? | birds | Entity (e) |
| What can they do? | fly | Relation (r) |
| How many? | most (p ≈ 0.7) | Modifier (m) |

**Select Operations:**
- `predicate(e_birds, r_fly, e_air) → a₁` — "birds fly"
- `quantify(m_most, e_birds) → a` — "most birds" (p ≈ 0.7)

**Formal decomposition:**
```
quantify(m_{p≈0.7}, predicate(e_birds, r_fly, e_air))
```

**Draw:**
```
  ┌────────────────────────────┐
  │                            │
  │  ████████████              │   ● fills ~70% of frame = "most"
  │  █ birds ███ ──fly──→ •   │
  │  ████████████   (air)      │
  │                            │
  └────────────────────────────┘

  Compare with:
  ALL (p=1):    ████████████████████  (fills entire frame)
  MOST (p≈0.7): ████████████          (fills ~70%)
  SOME (p→0⁺):        •              (point-like)
```

**Verify:**
- Pass 1: Frame (solid, ⊕).
- Pass 2: Entity "birds" fills approximately 70% of frame → quantify with p ≈ 0.7.
- Pass 3: Directed relation "fly" from birds to air.
- Pass 4: Frame-fill proportion directly encodes quantificational force.
- Pass 5: No assertion modification or negation.

**Reading:** "Most birds fly (through the air)." ✓

**Teaching point:** The visual fill proportion IS the quantificational force. You can literally see the difference between "all," "most," and "some."

### 10.8 Example 14: "She might be sleeping" (Epistemic Possibility)

**Decompose:**

| Question | Answer | Sort |
|----------|--------|------|
| Who? | she | Entity (e) |
| Doing what? | sleeping | Relation (r) |
| What kind of claim? | might — epistemic possibility | Modal operator (defined) |

**Select Operations:**
- `predicate(e_she, r_sleeping, e_implicit) → a₁` — "she sleeps"
- `possible(r_K, a₁)` — "it is possible (given what is known) that she sleeps"

**Expand to primitives:**
```
possible(r_K, a₁) = negate(necessary(r_K, negate(a₁)))
  = negate(bind(w, quantify(m_∀, w, disjoin(negate(predicate(w_current, r_K, w)), predicate(w, r_satisfies, embed(negate(a₁)))))))
```

**Draw:**
```
  ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐
  ╎                                ╎    ← dashed border = possibility (◇)
  ╎   •_she ──sleeping──→ •       ╎    ← core predication inside
  ╎                                ╎
  └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘
         label: r_K (epistemic)
```

**Verify:**
- Pass 1: Frame is dashed (◇ = possibility).
- Pass 2: Entity "she" with relation "sleeping."
- Pass 3: Label "r_K" indicates epistemic modal flavor.
- Pass 4: Reading: "In some world compatible with what is known, she is sleeping."

**Teaching point:** Dashed border = possibility. The reader sees the broken continuity and reads "this holds in SOME accessible world, not necessarily the actual one."

### 10.9 Example 15: "It is necessarily true that 2+2=4" (Alethic Necessity)

**Decompose:**

| Question | Answer | Sort |
|----------|--------|------|
| What equals what? | 2+2 equals 4 | Entities + Relation |
| What kind of claim? | necessarily true — alethic necessity | Modal operator (defined) |

**Select Operations:**
- `predicate(e_sum_2_2, r_equals, e_4) → a₁` — "2+2 = 4"
- `necessary(r_alethic, a₁)` — "necessarily, 2+2 = 4"

**Draw:**
```
  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
  ┃                                 ┃    ← bold border = necessity (□)
  ┃   •_{2+2} ──equals──→ •_4      ┃    ← core predication inside
  ┃                                 ┃
  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
         label: r_alethic (S5)
```

**Verify:**
- Pass 1: Frame is bold (□ = necessity).
- Pass 2: Predication: 2+2 relates-by-equality to 4.
- Pass 3: Label "r_alethic" with S5 frame = all worlds mutually accessible.
- Pass 4: Reading: "In ALL possible worlds, 2+2 = 4."

**Teaching point:** Bold border = necessity. The thick, unbroken line says "this holds everywhere, without exception." The S5 frame means every world can access every other — there's no escape from mathematical truth.

### 10.10 Example 16: "He could have won if he had tried" (Counterfactual)

**Decompose:**

| Question | Answer | Sort |
|----------|--------|------|
| Who? | he | Entity (e) |
| What is claimed? | won | Relation (r) |
| Under what condition? | if he had tried (counterfactual antecedent) | Another predication |
| What kind of claim? | counterfactual — "in the closest worlds where he tried..." | Modal operator (defined) |

**Select Operations:**
- `predicate(e_he, r_tried, e_implicit) → a_antecedent` — "he tries"
- `predicate(e_he, r_won, e_implicit) → a_consequent` — "he wins"
- `counterfactual(a_antecedent, a_consequent)` — "if he had tried, he would have won"

**Expand:**
```
counterfactual(a_ant, a_con) = necessary(r_closest(a_ant), a_con)
  where r_closest(a_ant) = modify_relation(abstract(embed(a_ant)), r_closeness)
```

**Draw:**
```
  ╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌
  ╎                                    ╎   ← dashed-dot border = counterfactual (□→)
  ╎  IF:  •_he ──tried──→ •           ╎   ← antecedent (condition)
  ╎  ─────────────────────             ╎
  ╎  THEN: •_he ──won──→ •            ╎   ← consequent (result)
  ╎                                    ╎
  ╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌·╌
         label: r_closeness (Lewis similarity)
```

**Verify:**
- Pass 1: Frame is dashed-dot (□→ = counterfactual).
- Pass 2: Two predications — antecedent and consequent.
- Pass 3: Label "r_closeness" indicates Lewis similarity ordering.
- Pass 4: Reading: "In the closest worlds where he tried, he won."

**Teaching point:** The dashed-dot border means "this isn't about the actual world — it's about the nearest worlds where the antecedent holds." The antecedent didn't actually happen (he didn't try), but the claim is about what WOULD have happened.

### 10.11 Example 17: "I promise to return" (Commissive Performative)

**Decompose:** Subject (speaker), relation (return), force (commissive — the utterance creates an obligation).

**Select operations:**
1. `predicate(e_speaker, r_return, e_destination)` — core content
2. Force parameter: φ = commit

**Expand:**
```
predicate(e_speaker, r_return, e_destination)[σ=⊕, φ=commit]
```

**Draw:**
```
←────────────────────┐
   •_I ──return──→ •  │
←────────────────────┘
    φ = commit (arrow-in border)
```

**Verify:**
- Pass 1: Frame has arrow-in borders (←) = commissive force.
- Pass 2: Content is a standard predication: speaker returns to destination.
- Pass 3: Without φ=commit, this would be indistinguishable from "I will return" (mere assertion).
- Pass 4: Reading: "I undertake to return" — the utterance creates the obligation.

**Teaching point:** The arrow-in border (←) means the force is directed inward — the speaker binds THEMSELVES. Compare with directives (→), where force is directed outward at the hearer.

### 10.12 Example 18: "I pronounce you married" (Declarative Performative)

**Decompose:** Subject (speaker/authority), relation (pronounce), embedded content (you are married), force (declarative — utterance changes reality).

**Select operations:**
1. `embed(predicate(e_you, r_married, e_you))` — embedded content
2. `predicate(e_speaker, r_pronounce, ...)` — pronouncement act
3. Force parameter: φ = declare

**Expand:**
```
predicate(e_speaker, r_pronounce, embed(predicate(e_you, r_married, e_you)))[σ=⊕, φ=declare]
```

**Draw:**
```
╔═══════════════════════════════════════╗
║  •_speaker ──pronounce──→ ┌───────┐  ║
║                           │•_you  │  ║
║                           │married│  ║
║                           └───────┘  ║
╚═══════════════════════════════════════╝
    φ = declare (bold double border)
```

**Verify:**
- Pass 1: Frame has bold double border (╔══╗) = declarative force.
- Pass 2: Content nests: outer predication wraps embedded predication.
- Pass 3: The bold double border signals that this utterance CREATES the married state — it doesn't merely describe it.
- Pass 4: Reading: "By uttering this, the speaker makes it the case that you are married."

**Teaching point:** Declarative force is the strongest — the utterance itself changes reality. The bold double border (same as world-enclosure) reflects this: the speech act creates a new state of affairs, like defining a new world.

### 10.13 Example 19: "Can you pass the salt?" (Indirect Speech Act — CI-1)

**Decompose:** Surface form is a query about ability. Intended meaning is a polite directive. This is the canonical case of conventional inference pattern CI-1.

**Select operations (surface):**
1. `predicate(e_hearer, r_pass, e_salt)` — content: hearer passes salt
2. `◇_ability(...)` — modal: ability of hearer
3. Force parameter: φ = query

**Surface form:**
```
possible(r_ability, predicate(e_hearer, r_pass, e_salt))[σ=⊕, φ=query]
```

**Expand to primitives (for reference):**
```
negate(necessary(r_ability, negate(predicate(e_hearer, r_pass, e_salt))))[σ=⊕, φ=query]
```

**Draw surface:**
```
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
  ◇_ability                     
│   •_hearer ──pass──→ •_salt │
└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘?
    φ = query (gapped border)
```

**Conventional inference (CI-1):**
```
φ_query(◇_ability(a))  ⟹  φ_direct(a)
```

**Draw intended meaning:**
```
┌──────────────────────────────→
│  •_hearer ──pass──→ •_salt
└──────────────────────────────→
    φ = direct (arrow-out border)
```

**Verify:**
- Pass 1: Surface has gapped border (query) + diamond (ability modal). Intended has arrow-out border (directive).
- Pass 2: Content is identical in both frames — only force differs.
- Pass 3: The ⟹ arrow between frames marks a conventional inference, not a logical entailment.
- Pass 4: Reading: Surface "Are you able to pass the salt?" conventionally infers directive "Pass the salt."

**Teaching point:** CI-1 is the most common indirect speech act in human language. The ⟹ notation distinguishes conventional inference (culturally stable, compositionally predictable) from truly non-compositional phenomena like sarcasm (⚠️ scope boundary). See `formal-foundations.md` §9.4.

### 10.14 Example 20: "The ball was kicked by the child" (Explicit Invert — Passive Voice)

**Decompose:** subject="child" (entity), action="kicked" (relation), object="ball" (entity). The passive voice reverses the relation direction.

**Select operations:**
1. `predicate(child, kick, ball)` — active form
2. `invert(kick)` — reverse the relation for passive voice
3. `predicate(ball, invert(kick), child)` — passive form

**Active form (for comparison):**
```
predicate(e_child, r_kick, e_ball)[σ=⊕]
```

**Passive form:**
```
predicate(e_ball, invert(r_kick), e_child)[σ=⊕]
```

**Draw active:**
```
┌─────────────────────────────┐
│  •_child ──kick──→ •_ball   │
└─────────────────────────────┘
```

**Draw passive (inverted):**
```
┌─────────────────────────────┐
│  •_ball ←──kick── •_child   │
└─────────────────────────────┘
```

**Verify:**
- Pass 1: Single frame (one assertion). Arrow direction REVERSED from active to passive.
- Pass 2: Same entities, same relation — only direction changes. This is exactly what `invert` does.
- Pass 3: No modifiers, no negation. Clean inversion.
- Pass 4: Reading: "The ball was kicked (invert of kicks) by the child."

**Teaching point:** `invert(r)` is the only operation that changes the *direction* of a relation without changing its content. This is how UL captures active/passive voice transformation as a single algebraic operation. The drawing makes it visible: the same stroke, reversed.

### 10.15 Example 21: "Grandfather is father's father" (Relation Composition)

**Decompose:** "grandfather" is the composition of the "father-of" relation with itself. This is the canonical case for `compose(r₁, r₂)`.

**Select operations:**
1. `r_father` — the "father-of" relation
2. `compose(r_father, r_father)` — chaining: X's father's father = X's grandfather

**Expression:**
```
compose(r_father, r_father) = r_grandfather
```

**Draw the chain:**
```
•_A ──father──→ •_B ──father──→ •_C
```

**Draw the composed result:**
```
•_A ──grandfather──→ •_C
```

**Verify:**
- Pass 1: Two chained arrows collapse into one. The intermediate entity (B) is internal to the composition.
- Pass 2: `compose` requires both operands to be Relation-sort. ✓
- Pass 3: The output is a new Relation-sort — it can itself be composed further (great-grandfather = compose(grandfather, father)).
- Pass 4: Reading: "grandfather = father composed with father."

**Teaching point:** `compose(r₁, r₂)` is transitive chaining. Any kinship term, organizational hierarchy, or multi-step process can be decomposed into compositions of simpler relations. The drawing shows this visually: two strokes meeting end-to-end collapse into a single stroke.

### 10.16 Example 22: "Wood becomes wooden" (Abstraction)

**Decompose:** "wooden" is the adjectival form of "wood." The entity "wood" is transformed into a modifier "wood-like" via the `abstract` operation.

**Select operations:**
1. `e_wood` — entity: the material wood
2. `abstract(e_wood)` — turn entity into modifier: "wood-ish" / "wooden"
3. `modify_entity(abstract(e_wood), e_table)` — "wooden table"

**Expression:**
```
modify_entity(abstract(e_wood), e_table)
```

**Draw:**
```
       •_wood → ∠_wooden
            ↓
•_table ──modified──→ •_wooden_table
```

Step by step:
1. Draw `•_wood` (entity dot)
2. Apply `abstract`: the dot's *shape* becomes a modifier — draw as an angle ∠ derived from the dot
3. The modifier ∠_wooden attaches to `•_table` → producing a modified entity `•_wooden_table`

**Verify:**
- Pass 1: `abstract` changes sort from Entity → Modifier. ✓
- Pass 2: The modifier is then applied via `modify_entity`. ✓
- Pass 3: Round-trip: could we recover "wood" from "wooden"? Yes — `abstract` turns e→m, the conceptual inverse would turn m→e (but UL has no formal "de-abstract" — this is one-directional).
- Pass 4: Reading: "wooden table = table modified by (wood made abstract)."

**Teaching point:** `abstract(e)` captures how language derives adjectives from nouns: wood→wooden, gold→golden, child→childish. The geometry: an entity's *shape* becomes a *transformation*. You take the properties of a thing and use them to reshape other things.

---

## 11. THE GRAMMAR BRIDGE — FROM OPERATIONS TO PARTS OF SPEECH

This table maps between the four systems you encounter when writing: your natural-language intuition, the Σ_UL operations, the Grammar Book's geometric classifications, and your pen on paper.

### 11.1 Operations ↔ Grammar Roles

| Σ_UL Operation | Grammar Role | Writer's Question | Natural Language Analog | Drawing Action |
|----------------|-------------|-------------------|------------------------|----------------|
| `predicate(e, r, e)` | Predication | "How does X relate to Y?" | Subject-verb-object: "A acts on B" | Two marks + connecting stroke + frame |
| `modify_entity(m, e)` | Noun qualification | "What is X like?" | Adjective: "the **big** dog" | Transform the entity (scale, reshape) |
| `modify_relation(m, r)` | Relation qualification | "In what manner?" | Adverb: "acts **strongly**" | Angle where relation meets context |
| `negate(a)` | Logical negation | "What is NOT true?" | "does **not** act on" | Flip frame: solid → dashed |
| `conjoin(a₁, a₂)` | Conjunction | "What holds for BOTH?" | "A **and** B" | Overlapping frames |
| `disjoin(a₁, a₂)` | Disjunction | "What holds for EITHER?" | "A **or** B" | Adjacent (touching) frames |
| `embed(a)` | Nominalization | "What is the FACT that...?" | "**that** A happened" | Shrink frame, use as entity |
| `abstract(e)` | Adjectivalization | "What is X-LIKE?" | "wood → **wooden**" | Use entity's shape as a transformation |
| `compose(r₁, r₂)` | Relation chaining | "X to Y, then Y to Z?" | "**grandfather** = father's father" | Two strokes end-to-end |
| `invert(r)` | Voice change | "Who acts on whom?" | Active → passive: "is **acted upon by**" | Reverse the arrow |
| `quantify(m, e)` | Quantification | "How many? Which ones?" | "**all** dogs" / "**some** dogs" | Scale entity large (all) or small (some) |

### 11.2 Parts of Speech ↔ Geometry

The Grammar Book classifies parts of speech by **symmetry group** — higher symmetry = more abstract/general:

| Part of Speech | Symmetry | Geometric Property | UL Sort | Example |
|---------------|----------|-------------------|---------|---------|
| **Universal noun** (abstract concept) | SO(2) — full rotational | Looks same from all angles | Entity (e) in ○ | "truth," "everything" |
| **Common noun** (structured concept) | Dₙ — discrete rotational | Specific symmetry pattern | Entity (e) in polygon | "system," "person" |
| **Verb** (action/process) | Low/no rotational | Has preferred direction | Relation (r) | "acts," "becomes" |
| **Adjective/Adverb** (quality) | Bilateral — one mirror axis | One comparison dimension | Modifier (m) | "big," "gently" |
| **Proper noun** (specific) | No symmetry | Unique, context-bound | Entity (e) as labeled • | "Mars," "Jono" |

### 11.3 Relationship Classes ↔ Writer's Decisions

When decomposing a thought, the type of relationship determines which geometric class you're in:

| If the relationship involves... | Relationship Class | Key geometric property | Examples |
|--------------------------------|-------------------|----------------------|----------|
| Contact, membership, inclusion | **Incidence** (Class 1) | What touches what | "X belongs to Y," "X is a Y," "X and Y meet" |
| Amount, degree, distance | **Metric** (Class 2) | How much, how far | "X is larger than Y," "X strongly relates to Y" |
| Sameness, opposition, analogy | **Symmetry** (Class 3) | What is preserved under transformation | "X is like Y," "X opposes Y," "X equals Y" |
| Essential structure, type | **Topological** (Class 4) | What survives continuous deformation | "X is essentially Y," "X is a type of Y" |

---

## 12. QUICK REFERENCE — THE WRITER'S CHECKLIST

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

## 13. COMMON MISTAKES AND FIXES

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

## 14. WHAT THIS GUIDE DOES NOT COVER

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

## APPENDIX B: THE 13 OPERATIONS AT A GLANCE

| Operation | Signature | Plain English | How to draw it |
|-----------|-----------|---------------|----------------|
| `predicate` | e × r × e → a | "A relates to B" | Two marks connected by a stroke, inside a frame |
| `modify_entity` | m × e → e | "A transformed" | Scale, rotate, or reshape an entity |
| `modify_relation` | m × r → r | "Relates *in this way*" | The angle at which a relation meets context |
| `negate` | a → a | "NOT this claim" | Flip frame boundary: solid → dashed |
| `conjoin` | a × a → a | "A AND B" | Two frames overlapping |
| `disjoin` | a × a → a | "A OR B" | Two frames touching |
| `embed` | a → e | "The fact that A" | Shrink a frame, use it as an entity |
| `abstract` | e → m | "A-shaped" / "A-like" | Use entity's shape as a transformation |
| `compose` | r × r → r | "then" (chain) | Two strokes meeting end to end |
| `invert` | r → r | "is done by" (reverse) | Reverse the arrow |
| `quantify` | m × e → a | "ALL/MOST/SOME A" | Entity fills frame proportion: p=1 all, p≈0.7 most, p→0⁺ some |
| `bind` | e × a → a | "the same x" / "for all x" | Hollow marks (○_x) → filled marks (●_x) for co-reference |
| `modify_assertion` | m × a → a | "apparently" / "definitely" | Frame decoration: dotted=evidential, double=emphatic, wavy=hedged |

Note: `conjoin` is derivable from `{negate, disjoin}` via De Morgan's law. The other 12 are independent.
