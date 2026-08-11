# UL Formal Construction Grammar

> **A formal specification of valid UL constructions, bridging the Σ_UL algebra to the visual writing system.**

This document defines what constitutes a **well-formed UL expression** — the construction rules that a human writer must follow and that a machine parser must enforce. It replaces the informal prose descriptions scattered across syntax-dictionary.md and grammar-book.md with a precise, implementable specification.

---

## 1. Design Constraints

UL is a **2D spatial language**, not a 1D string language. Standard BNF/PEG grammars assume linear sequences. UL constructions are planar geometric configurations where spatial relationships (containment, connection, adjacency, overlap) carry grammatical meaning.

We therefore use a **spatial construction grammar**: rules define valid spatial configurations, not string productions. Each rule specifies:
- **Input sorts** (what types of components are combined)
- **Spatial relationship** (how they must be arranged)
- **Output sort** (what type the result belongs to)
- **Visual signature** (how to recognize it)

This grammar is equivalent to a **labeled graph grammar** where nodes are geometric objects with sorts and edges are spatial relationships.

---

## 2. Primitive Elements

### 2.1 Marks (Sort: Entity)

A **mark** is any compact geometric configuration in the glyph space.

| Mark Type | Visual | Sort | Notes |
|-----------|--------|------|-------|
| Point | • | e | Simplest entity — existence at a position |
| Regular polygon | △, □, ⬠, ⬡, ... | e | Symmetry determines part-of-speech (see grammar-book.md §II) |
| Labeled mark | •_label | e | Entity with distinguishing label |
| Circle | ○ | e | Maximum symmetry — universal/abstract entity |

**Well-formedness:** Any compact subset of ℝ² with a finite decoration (labeled points) is a valid mark.

### 2.2 Connectors (Sort: Relation)

A **connector** is a directed path between two marks.

| Connector Type | Visual | Sort | Notes |
|----------------|--------|------|-------|
| Straight arrow | ──→── | r | Simplest relation — direct connection |
| Curved arrow | ~~→~~ | r | Process/gradual relation |
| Double arrow | ══→══ | r | Emphasized/strong relation |
| Bidirectional | ←──→ | r | Mutual relation (syntactic sugar for conjoin of a relation and its inverse) |

**Well-formedness:** A connector must have:
1. A **source** — the startpoint lies on or within a mark
2. A **target** — the endpoint lies on or within a mark
3. A **direction indicator** — arrowhead or equivalent

### 2.3 Modifiers (Sort: Modifier)

A **modifier** is a geometric transformation. Visually, modifiers appear as:
- **Scale change** applied to a mark or connector (degree: very/slightly)
- **Rotation** applied to a mark or connector
- **Outline** of an entity (result of `abstract`) applied to another entity
- **Angle mark** between connectors
- **Translation** along the temporal axis (tense: past/future)
- **Curve-shape transformation** on a connector (aspect: progressive/perfective/habitual)
- **Stroke weight change** on a connector (manner: forceful/gentle)

See `formal-operations.md` §5 for the full canonical modifier assignment table mapping semantic categories to Gₘ elements.

### 2.4 Frames (Sort: Assertion)

A **frame** is a closed boundary containing a construction.

| Frame Type | Visual | Meaning |
|------------|--------|---------|
| Solid boundary | `[  content  ]` | Asserted (σ = ⊕) — "this IS the case" |
| Dashed boundary | `╎  content  ╎` | Denied (σ = ⊖) — "this is NOT the case" |

**Well-formedness:** A frame boundary must be a Jordan curve (simple closed curve, no self-intersections). The interior must contain at least one entity-relation-entity triple.

---

## 3. Construction Rules

Each construction rule corresponds to one Σ_UL operation. The rules are numbered C1–C11.

### C1: Predication — `predicate(e₁, r, e₂) → a`

**Spatial arrangement:**
```
  ┌─────────────────────┐
  │  mark₁ ──→── mark₂  │
  └─────────────────────┘
```

**Rule:** Place two marks and a connector inside a new frame.
- The connector's source must lie on/in mark₁
- The connector's target must lie on/in mark₂
- The frame must enclose all three components
- Frame boundary is solid (asserted by default)

**Result sort:** Assertion (a)

**Recognition:** A frame containing two or more marks connected by at least one directed connector.

---

### C2: Entity Modification — `modify_entity(m, e) → e`

**Spatial arrangement:**
```
  Before: △              After: ▲ (scaled-up triangle)
  Before: •              After: •₁ (labeled/distinguished point)
```

**Rule:** Apply a geometric transformation to a mark.
- Scale: change the size of the mark
- Rotate: change the orientation of the mark
- Shape-impose: overlay an outline (from `abstract`) onto the mark

**Result sort:** Entity (e) — same sort, modified form

**Recognition:** A mark whose shape/size/orientation differs from the canonical form.

---

### C3: Relation Modification — `modify_relation(m, r) → r`

**Spatial arrangement:**
```
  Before: ──→──          After: ══→══ (thickened/intensified)
  Before: ──→──          After: ~~→~~ (curved)
  Before: ──→──  (center)  After: ←──→── (displaced left = past tense)
  Before: ──→──          After: ─⌢→⌢─ (open arc = progressive aspect)
```

**Rule:** Apply a geometric transformation to a connector.
- Scale: thicken/thin the connector stroke (manner: forceful/gentle)
- Curve: change the path curvature (manner: quickly/slowly)
- Style: change the stroke pattern (solid, wavy, etc.)
- Translate: displace along temporal axis (tense: past/future; see formal-operations.md §5.1)
- Shape: open arc (progressive), closed sub-frame (perfective), repeating wave (habitual), truncated (inchoative) (see formal-operations.md §5.2)

**Result sort:** Relation (r) — same sort, modified form

**Recognition:** A connector whose visual properties (thickness, curvature, style, position, shape) differ from the default straight arrow.

---

### C4: Negation — `negate(a) → a`

**Spatial arrangement:**
```
  Before:  ┌───────────────┐      After:  ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐
           │  △ ───→─── ○  │              ╎  △ ───→─── ○  ╎
           └───────────────┘              └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘
           σ = ⊕ (solid)                  σ = ⊖ (dashed)
```

**Rule:** Flip the frame boundary style.
- Solid → dashed (asserted → denied)
- Dashed → solid (denied → asserted)
- Content inside the frame is unchanged

**Result sort:** Assertion (a) — same content, opposite assertional sign

**Recognition:** A dashed-boundary frame. (Solid = default/asserted; dashed = negated/denied.)

---

### C5: Conjunction — `conjoin(a₁, a₂) → a`

**Spatial arrangement:**
```
  ┌───────────┐
  │  content₁ ├───────────┐
  │           │  content₂ │
  └───────────┤           │
              └───────────┘
```

**Rule:** Two assertion frames with overlapping boundaries.
- The overlap region belongs to BOTH assertions
- The merged construction is a single compound assertion

**Derivability note:** `conjoin(a₁, a₂) = negate(disjoin(negate(a₁), negate(a₂)))`. This construction is a convenience — it is algebraically derivable from C4 (negation) and C6 (disjunction).

**Result sort:** Assertion (a)

**Recognition:** Two frames whose boundaries intersect (spatial overlap).

---

### C6: Disjunction — `disjoin(a₁, a₂) → a`

**Spatial arrangement:**
```
  ┌───────────┐  ┌───────────┐
  │  content₁ │  │  content₂ │
  └───────────┘  └───────────┘
```

**Rule:** Two assertion frames placed adjacent (touching but not overlapping).
- Frames share a boundary segment or are immediately adjacent
- No overlap between frame interiors

**Result sort:** Assertion (a)

**Recognition:** Two frames that are adjacent (touching boundaries, no overlap).

---

### C7: Embedding — `embed(a) → e`

**Spatial arrangement:**
```
  Before:  ┌───────────────┐    After:  ┌─┐
           │  △ ───→─── ○  │           │·│  (shrunken frame)
           └───────────────┘           └─┘
```

**Rule:** Uniformly scale down an assertion frame and use it as an entity within another construction.
- Scale factor λ ∈ (0, 1) — the embedded frame must be strictly smaller than full assertion scale
- The embedded frame retains its internal structure
- In context, the shrunken frame occupies an entity position (connected to relations, inside larger frames)

**Result sort:** Entity (e)

**Recognition:** A small frame nested inside another construction, occupying a position where a mark would normally appear.

---

### C8: Abstraction — `abstract(e) → m`

**Spatial arrangement:**
```
  Before:  ┌───┐           After:  ┌───┐
           │ ⬠ │  "wood"           │   │  "wooden" (outline only)
           └───┘                   └───┘
```

**Rule:** Extract the entity's outline (boundary shape), discarding internal content.
- The outline is drawn without fill
- Detached from original spatial position — it becomes a free-standing modifier
- Applied to another entity (via C2), it reshapes the target's outline

**Result sort:** Modifier (m)

**Recognition:** An empty outline (no internal marks or connectors) used to modify another entity's shape.

---

### C9: Composition — `compose(r₁, r₂) → r`

**Spatial arrangement:**
```
  Before:  •──r₁──→•   •──r₂──→•

  After:   •──r₁──→•──r₂──→•
                    ↑
              shared endpoint
```

**Rule:** Join two connectors at a shared endpoint.
- The target of r₁ must be the source of r₂ (shared mark)
- The composed relation spans from r₁'s source to r₂'s target
- If endpoints don't coincide, a connecting segment is inserted

**Result sort:** Relation (r)

**Recognition:** A path passing through an intermediate mark — two sequential connectors sharing an endpoint.

---

### C10: Inversion — `invert(r) → r`

**Spatial arrangement:**
```
  Before:  •──→──•     After:  •──←──•
```

**Rule:** Reverse the direction indicator on a connector.
- Arrowhead moves from target to source
- Path traversal direction reverses
- All other connector properties (curvature, thickness, etc.) unchanged

**Result sort:** Relation (r)

**Recognition:** An arrow pointing in the opposite direction from the canonical reading order.

---

### C11: Quantification — `quantify(m, e) → a`

**Spatial arrangement:**
```
  Universal (∀, p=1):    ┌───────────┐
                         │  ●●●●●●●  │   Entity fills frame
                         └───────────┘

  Most (p≈0.7):          ┌───────────┐
                         │  ●●●●●    │   Entity fills ~70% of frame
                         └───────────┘

  Few (p≈0.05):          ┌───────────┐
                         │       ••  │   Entity small but visible
                         └───────────┘

  Existential (∃, p→0⁺): ┌───────────┐
                         │        •  │   Entity point-like
                         └───────────┘

  Negative (¬∃):         ┌╌╌╌╌╌╌╌╌╌╌╌┐
                         ╎     •     ╎   Dashed frame + small entity
                         └╌╌╌╌╌╌╌╌╌╌╌┘
```

**Rule:** Scale an entity relative to a frame to express scope. The entity's area as a proportion of the frame area encodes the quantificational force:
- Entity fills entire frame (p = 1) = universal ("all")
- Entity fills majority (p > 0.5) = "most"
- Entity fills minority (p < 0.5) = "few" / "some"
- Entity scaled to point-like (p → 0⁺) = existential ("some/a")
- Dashed frame + existential = negative universal ("none") — composed from C4 + C11

**Graduated quantification:** The frame-fill proportion p ∈ [0,1] is continuous. See `formal-operations.md` §1.11 for the area-proportion formula and conventional p-ranges for natural language quantifiers.

**Result sort:** Assertion (a)

**Recognition:** A single entity inside a frame, where the entity's relative size encodes quantificational scope.

---

### C12: Variable Binding — `bind(e_x, a) → a`

**Spatial arrangement:**
```
  Before bind:           After bind:
  ┌─────────────────┐    ┌─────────────────┐
  │  ○_x ──→ ●      │    │  ●_x ──→ ●      │
  │                  │ →  │     ↑ bound     │
  │  ○_x ──→ ●      │    │  ●_x ──→ ●      │
  └─────────────────┘    └─────────────────┘

  ○_x = hollow mark with label x (open slot)
  ●_x = filled mark with label x (bound variable)
```

**Rule:** Place a labeled hollow mark (○_x) in entity positions within a frame. Binding replaces all ○_x with filled marks (●_x), establishing that all occurrences denote the **same** entity. The frame boundary delimits the scope of the binding.

**Scope ordering via nesting:**
```
  ∀x. ∃y. x reads y:

  ┌───────────────────────────────────────┐    bind(x) — outer scope
  │   ┌───────────────────────────────┐   │
  │   │  ●_x ───read──→ ●_y          │   │    bind(y) — inner scope
  │   └───────────────────────────────┘   │
  └───────────────────────────────────────┘
```

**Result sort:** Assertion (a)

**Recognition:** Hollow marks (○) with matching labels indicate open slots; filled marks (●) with labels indicate bound variables. Frame nesting determines scope precedence — outer frame = wider scope.

---

### C13: Assertion Modification — `modify_assertion(m, a) → a`

**Spatial arrangement:**
```
  Default:              ┌───────────────┐
                        │  ● ──→ ●      │   Solid frame = asserted
                        └───────────────┘

  Evidential:           ┌···············┐
                        ╎  ● ──→ ●      ╎   Dotted frame = "apparently"
                        └···············┘

  Emphatic:             ╔═══════════════╗
                        ║  ● ──→ ●      ║   Double frame = "definitely"
                        ╚═══════════════╝

  Hedged:               ┌~~~~~~~~~~~~~~~┐
                        │  ● ──→ ●      │   Wavy frame = "sort of"
                        └~~~~~~~~~~~~~~~┘
```

**Rule:** Modify the frame boundary's visual properties (thickness, pattern, texture) without changing the content or truth status. The frame decoration encodes the speaker's epistemic access, emphasis, or attitude toward the assertion.

**Composability with negate (C4):** Negation flips σ (solid ↔ dashed for truth status). Assertion modification changes the frame decoration independently. Both can apply:
```
  "Apparently she didn't leave":
  ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐
  ╎  ● ──→ ●      ╎   Dashed (denied) + dotted (evidential)
  └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘
```

**Result sort:** Assertion (a)

**Recognition:** Frame boundary style indicates assertion-level modification. Content and truth status are preserved; only the frame's visual properties change.

---

## 4. Construction Composition Rules

Complex UL expressions are built by **nesting** the construction rules. The following meta-rules govern composition:

### 4.1 Sort Compatibility

Every construction rule input slot requires a specific sort. A component can fill a slot only if its sort matches:

| Slot requires | Can be filled by |
|---|---|
| Entity (e) | A mark (§2.1), a modified mark (C2), or an embedded assertion (C7) |
| Relation (r) | A connector (§2.2), a modified connector (C3), a composed relation (C9), or an inverted relation (C10) |
| Modifier (m) | A transformation (§2.3) or an abstracted entity (C8) |
| Assertion (a) | A predication (C1), a negation (C4), a conjunction (C5), a disjunction (C6), a quantification (C11), a binding (C12), or an assertion modification (C13) |

### 4.2 Nesting Depth

Constructions can nest to arbitrary depth via embedding:
```
Level 0: predicate(e₁, r, e₂) → a₁
Level 1: embed(a₁) → e₃; predicate(e₃, r', e₄) → a₂
Level 2: embed(a₂) → e₅; predicate(e₅, r'', e₆) → a₃
...
```

Each embedding (C7) converts an assertion to an entity, enabling recursive nesting.

### 4.3 Disambiguation

When a visual construction could be parsed as more than one algebraic expression:

1. **Containment before connection:** A mark inside a frame is an entity IN that assertion, not a separate assertion.
2. **Connection before adjacency:** Two marks connected by a connector are part of one predication, even if they're near another frame.
3. **Smaller is embedded:** A small frame inside a larger frame is an embedded entity (C7), not a separate assertion.
4. **Frame boundary style determines assertional sign:** Solid = asserted, dashed = denied. This takes precedence over any content interpretation.

---

## 5. Reading Algorithm (Formal)

Given a UL construction, extract its algebraic expression using this 5-pass algorithm:

**Pass 1 — Identify frames (Assertions):**
Find all closed boundaries (Jordan curves). Each is an assertion frame. Classify by boundary style: solid (⊕) or dashed (⊖). Nested frames are embedded assertions.

**Pass 2 — Identify marks (Entities):**
Within each frame, find all compact configurations that are not connectors or sub-frames. Each is an entity. Small nested frames are embedded entities (from C7).

**Pass 3 — Identify connectors (Relations):**
Find all directed paths connecting marks. Each is a relation. Note direction, curvature, style.

**Pass 4 — Identify modifiers:**
Detect transformations applied to marks (size changes, rotations) or connectors (curvature, thickness). Detect empty outlines applied as shape-modifiers (from C8).

**Pass 5 — Assemble algebraic expression:**
- Each frame with marks and connectors → `predicate(e₁, r, e₂)` (C1)
- Nested small frames → `embed(a)` (C7)
- Overlapping frames → `conjoin(a₁, a₂)` (C5)
- Adjacent frames → `disjoin(a₁, a₂)` (C6)
- Dashed frame → `negate(a)` (C4)
- Sequential connectors sharing endpoint → `compose(r₁, r₂)` (C9)
- Reversed connector → `invert(r)` (C10)
- Entity filling frame → `quantify(m_∀, e)` (C11)
- Entity point-like in frame → `quantify(m_∃, e)` (C11)
- Transformed marks → `modify_entity(m, e)` (C2)
- Transformed connectors → `modify_relation(m, r)` (C3)
- Empty outline modifying mark → `abstract(e) → m`, then `modify_entity(m, e')` (C8 + C2)
- Hollow marks (○_x) with matching labels → open slots; filled marks (●_x) with labels → `bind(e_x, a)` (C12)
- Frame boundary decoration (dotted, double, wavy) → `modify_assertion(m, a)` (C13)

---

## 6. Writing Algorithm (Formal)

Given a Σ_UL algebraic expression, construct its visual realization:

**Step 1 — Parse the expression tree:**
Decompose the expression into its constituent operation applications. Identify the root (outermost operation) and leaves (atomic entities, relations, modifiers).

**Step 2 — Allocate spatial positions:**
Assign positions to entities (marks) within the glyph space. Entities in the same predication should be spatially proximate.

**Step 3 — Draw connectors:**
For each `predicate(e₁, r, e₂)`, draw a directed path from mark₁ to mark₂. Style the connector according to any `modify_relation` applications.

**Step 4 — Draw frames:**
For each assertion, draw a closed boundary around its content. Use solid boundary for assertions (σ = ⊕), dashed for negations (σ = ⊖).

**Step 5 — Handle compositions:**
- `conjoin`: Overlap the frames of the two assertions
- `disjoin`: Place frames adjacent
- `embed`: Scale the assertion frame down and use as a mark
- `compose`: Route connectors through shared intermediate marks
- `quantify`: Scale the entity large (∀), proportional (p), or small (∃) within its frame
- `bind`: Place labeled hollow marks (○_x) at entity positions; replace with filled marks (●_x) to show binding. Nest frames for scope ordering
- `modify_assertion`: Style the frame boundary (dotted = evidential, double = emphatic, wavy = hedged) without changing content or σ

**Step 6 — Apply modifiers:**
Transform marks and connectors according to modifier specifications.

**Step 7 — Verify:**
Apply the reading algorithm (§5) to the completed construction. Confirm it recovers the original algebraic expression.

---

## 7. Toward Machine Parsing

### 7.1 Computational Model

A machine parser for UL would need:
1. **2D spatial segmentation** — identify frames, marks, and connectors in a planar image
2. **Graph extraction** — build a labeled graph: nodes = marks/frames, edges = connectors/spatial relationships
3. **Grammar matching** — match the graph against the 11 construction rules (C1–C11)
4. **Expression assembly** — compose the matched rules into a Σ_UL expression tree

### 7.2 Complexity

- **Spatial segmentation** is O(n log n) for n geometric primitives (standard computational geometry)
- **Graph extraction** is O(n) given segmented primitives
- **Grammar matching** is the bottleneck: in general, 2D graph grammar parsing is NP-hard; but UL's constrained rule set (11 rules, sort-typed) restricts the search space significantly
- **Expression assembly** is O(n) given matched rules

For practical UL expressions (bounded nesting depth, bounded number of entities), parsing is tractable.

### 7.3 Serialization

For machine interchange, UL expressions can be serialized as typed S-expressions:

```
(predicate 
  (modify_entity scale:2 (mark triangle))
  (relation arrow)
  (embed
    (predicate 
      (mark circle)
      (invert (relation arrow))
      (mark point))))
```

This serialization is unambiguous and directly maps to the Σ_UL algebra. See `ul-forge/schemas/` for implemented machine-readable formats.
