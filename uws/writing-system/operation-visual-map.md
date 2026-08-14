# Operation ↔ Visual Mapping Reference

> **Complete bidirectional mapping between Σ_UL algebraic operations, visual realizations, and spatial relationships.**

---

## Master Mapping Table

| # | Σ_UL Operation | Signature | Visual Realization | Spatial Relationship | Drawing Instruction | Status |
|---|---|---|---|---|---|---|
| 1 | `predicate` | e × r × e → a | Connection inside frame | Connection | Draw two entities (points/shapes) connected by a directed line, all inside an enclosure (frame) | ✅ Complete |
| 2 | `modify_entity` | m × e → e | Transformation of entity | — | Apply a geometric transformation (scale, rotate, translate) to the entity glyph | ✅ Complete |
| 3 | `modify_relation` | m × r → r | Transformation of connection | — | Apply a geometric transformation to the connecting line (curve it, thicken it, etc.) | ✅ Complete |
| 4 | `negate` | a → a | Boundary inversion | — | Flip the assertion frame boundary: solid → dashed (asserted → denied) or dashed → solid | ✅ Complete |
| 5 | `conjoin` | a × a → a | Overlapping frames | Intersection | Draw two assertion frames that spatially overlap; shared region = shared content | ✅ Complete (derived from {negate, disjoin}) |
| 6 | `disjoin` | a × a → a | Adjacent frames | Adjacency | Draw two assertion frames side by side, not overlapping | ✅ Complete |
| 7 | `embed` | a → e | Nested/scaled frame | Containment | Shrink an assertion frame and place it inside another construction as an entity | ✅ Complete |
| 8 | `abstract` | e → m | Outline extraction | — | Draw the entity's outline only (no fill/content), detached from spatial position — this outline IS the modifier | ✅ Complete |
| 9 | `compose` | r × r → r | Sequential connection | Connection (sequential) | Two directed lines meeting at a shared point, forming one continuous path: `•──r₁──→•──r₂──→•` | ✅ Complete |
| 10 | `invert` | r → r | Arrow reversal | — | Reverse the direction indicator on a directed line: `•──→•` becomes `•←──•` | ✅ Complete |
| 11 | `quantify` | m × e → a | Entity scaling in frame | — | Scale entity to fill frame (∀ = all), shrink to point (∃ = some), or boundary-invert + complement (¬∃ = none) | ✅ Mostly complete |
| 12 | `bind` | e × a → a | Variable slot + binding arrows | Co-reference | Draw hollow marks (○_x) for variable slots and filled marks (●_x) for bound references, linked by dashed arrows | ✅ Complete |
| 13 | `modify_assertion` | m × a → a | Frame boundary decoration | — | Apply border style to assertion frame: dotted = evidential, double = emphatic, wavy = hedged | ✅ Complete |

---

## Detailed Visual Specifications

### Operations with complete visual forms

**predicate(e₁, r, e₂) → a**
```
  ┌─────────────────┐
  │  △ ───→─── ○    │   solid frame = asserted
  └─────────────────┘
  entity₁  relation  entity₂
```

**modify_entity(m, e) → e**
```
  Before: △          After: ▲ (scaled up)
  Before: △          After: △ rotated 45°
```
Modifier determines transformation type (Erlangen hierarchy: isometry, similarity, affine, projective, topological).

**modify_relation(m, r) → r**
```
  Before: ──→──      After: ══→══ (thickened/intensified)
  Before: ──→──      After: ~~→~~ (curved/softened)
```

**negate(a) → a**
```
  Before:  ┌───────────────┐      After:  ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐
           │  △ ───→─── ○  │              ╎  △ ───→─── ○  ╎
           └───────────────┘              └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘
           ASSERTED (solid)               DENIED (dashed)
```
Content unchanged. Only the frame boundary style flips.

**conjoin(a₁, a₂) → a** (derived from {negate, disjoin})
```
  ┌───────────┐
  │  △──→──○  ├───────────┐
  │           │  □──→──◇  │
  └───────────┤           │
              └───────────┘
  Overlapping frames = conjunction (AND)
```

**disjoin(a₁, a₂) → a**
```
  ┌───────────┐  ┌───────────┐
  │  △──→──○  │  │  □──→──◇  │
  └───────────┘  └───────────┘
  Adjacent frames = disjunction (OR)
```

**embed(a) → e**
```
  Before:  ┌───────────────┐    After:  ┌─┐
           │  △ ───→─── ○  │           │·│  (shrunken to entity-scale)
           └───────────────┘           └─┘
```
The embedded assertion appears as a small enclosed shape inside larger constructions:
```
  ┌──────────────────────┐
  │  ┌─┐ ════→════ •     │
  │  │·│                  │
  │  └─┘                  │
  └──────────────────────┘
  "The fact that [embedded] causes [entity]"
```

**compose(r₁, r₂) → r**
```
  Before: •──r₁──→•   •──r₂──→•
  After:  •──r₁──→•──r₂──→•      (paths joined at shared point)
          └─── r₁ ∘ r₂ ────┘     (single composed relation)
```
Example: "father" ∘ "father" = "grandfather"

**invert(r) → r**
```
  Before: •──→──•    "A acts on B"
  After:  •──←──•    "B is acted upon by A"
```
Arrow direction reverses. Path reverses parameterization.

**quantify(m, e) → a**
```
  Universal (∀):    ┌───────────┐
                    │  ●●●●●●●  │   Entity fills frame = "ALL things"
                    └───────────┘

  Existential (∃):  ┌───────────┐
                    │        •  │   Entity small/localized = "SOME thing"
                    └───────────┘

  Negative (¬∃):    ┌╌╌╌╌╌╌╌╌╌╌╌┐
                    ╎     ○     ╎   Boundary-inverted + complement = "NO thing"
                    └╌╌╌╌╌╌╌╌╌╌╌┘
```

### Operations with complete visual forms (continued)

**abstract(e) → m**
```
  Step 1: Original entity         Step 2: Extract outline        Step 3: Apply as modifier

    ┌───┐                            ┌───┐                         ┌───┐
    │ ⬠ │  "wood"             →     │   │  "wooden"          →    │ △ │  "wooden thing"
    │   │  (filled entity)           └───┘  (outline only,         └───┘  (target entity
    └───┘                                    no content)                    reshaped by
                                                                           wood-outline)
```

The visual convention: **abstract = boundary extraction (∂)**. Draw the entity's outline without fill or internal content. This empty outline IS the modifier — a shape-template. When applied (via `modify_entity`), the outline encloses or reshapes the target entity.

More examples:
```
  ● "circle"     →    ○ "circular"      (filled → unfilled)
  △ "triangle"   →    △ "triangular"    (outline, no marked center)
  ■ "square"     →    □ "square-like"   (filled → unfilled)
```

**Why this works geometrically:** The formal definition says `abstract(e) = T_C`, the shape-imposing transformation derived from the entity's convex hull. The visual "outline only" is exactly the convex hull boundary — the geometric information that `T_C` preserves and imposes. Stripping the interior content shows that only the SHAPE matters, not the specific contents.

**bind(e_x, a) → a**
```
  ┌─────────────────────────┐
  │  ○_x ──→── ●_x ──→── ● │   ○_x = variable slot (hollow)
  │       ╌╌╌╌╌╌╌╌╌╌╌╌>     │   ●_x = bound reference (filled)
  └─────────────────────────┘   ╌╌╌> = binding arrow (dashed)
```
Variable slots introduce a bound variable (hollow mark with subscript). Each bound reference (filled mark with same subscript) co-refers. Dashed binding arrows visually connect slots to their references. Scope is the enclosing assertion frame.

**modify_assertion(m, a) → a**
```
  Evidential:     ┌·····················┐
    (?{...})      ·  △ ───→─── ○       ·   dotted border = reported/uncertain
                  └·····················┘

  Emphatic:       ╔═════════════════════╗
    (!{...})      ║  △ ───→─── ○       ║   double border = emphasized/certain
                  ╚═════════════════════╝

  Hedged:         ┌~~~~~~~~~~~~~~~~~~~~~┐
    (~?{...})     ~  △ ───→─── ○       ~   wavy border = hedged/tentative
                  └~~~~~~~~~~~~~~~~~~~~~┘
```
The modifier transforms the assertion's frame boundary, not its content. This is distinct from `negate` (which flips solid↔dashed); here the *style* of the boundary changes to encode epistemic attitude.

---

## Reverse Mapping: Visual → Algebraic

| Visual Pattern | Σ_UL Operation(s) |
|---|---|
| Enclosure with content inside | `predicate` (if content is e-r-e structure) |
| Connected shapes inside frame | `predicate(e₁, r, e₂)` |
| Transformed/resized entity | `modify_entity(m, e)` |
| Curved/styled connection line | `modify_relation(m, r)` |
| Dashed frame boundary | `negate(a)` — denied assertion |
| Overlapping frames | `conjoin(a₁, a₂)` |
| Adjacent (non-overlapping) frames | `disjoin(a₁, a₂)` |
| Small frame nested inside construction | `embed(a)` — nominalized assertion |
| Shape outline used as modifier | `abstract(e)` |
| Two arrows meeting at shared point | `compose(r₁, r₂)` |
| Reversed arrow direction | `invert(r)` |
| Entity filling entire frame | `quantify(m_∀, e)` — universal |
| Small entity localized in frame | `quantify(m_∃, e)` — existential |
| Dashed frame + complement entity | `quantify(m_¬∃, e)` — negative |
| Hollow marks (○_x) with dashed arrows to filled marks (●_x) | `bind(e_x, a)` — variable binding |
| Dotted/double/wavy frame boundary | `modify_assertion(m, a)` — evidential/emphatic/hedged |

---

## Spatial Relationship → Operation Mapping

From `glyph-composition.md` §I, the 5 spatial relationships:

| Spatial Relationship | Primary Σ_UL Operation | Notes |
|---|---|---|
| **Containment** | `embed(a) → e` | Inner object is embedded entity; outer is assertion frame |
| **Intersection** | `conjoin(a₁, a₂)` | Overlapping assertion frames = logical AND |
| **Adjacency** | `disjoin(a₁, a₂)` | Side-by-side assertion frames = logical OR |
| **Separation** | (no direct operation) | Two unrelated constructions |
| **Connection** | `predicate(e₁, r, e₂)` (within frame); `compose(r₁, r₂)` (between frames) | Line connecting two entities |

---

## Independence & Derivability Notes

- **the composition operations are independent** 
- **conjoin** is derivable: `conjoin(a,b) = negate(disjoin(negate(a), negate(b)))` (De Morgan)
- The visual realization of conjoin (overlapping frames) remains a valid and useful construction pattern
- **converse** (subject-object swap) is derivable: `predicate(e₂, invert(r), e₁)` — not a separate operation
