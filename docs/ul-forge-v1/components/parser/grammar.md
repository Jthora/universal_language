# PEG Grammar: UL-Script

> The formal Parsing Expression Grammar for UL-Script, annotated with AST node types.

---

## Notation

- `←` : is defined as
- `/` : ordered choice (PEG)
- `*` : zero or more
- `+` : one or more
- `?` : optional
- `!` : not-predicate
- `'...'` : literal string
- `[...]` : character class

---

## Grammar

```peg
# ═══════════════════════════════════════════════════
# TOP LEVEL
# ═══════════════════════════════════════════════════

Document      ← Spacing (Glyph (LineBreak+ Glyph)*)? Spacing EOF
                → DocumentNode { glyphs: Vec<Glyph> }

Glyph         ← Comment / Composition
                → GlyphNode

Comment       ← '#' (!LineEnd .)* LineEnd
                → (discarded)

# ═══════════════════════════════════════════════════
# COMPOSITION (spatial relationships between terms)
# ═══════════════════════════════════════════════════

Composition   ← Term (Spacing Operator Spacing Term)*
                → CompositionNode { terms: Vec<Term>, operators: Vec<Operator> }

Term          ← Group / Mark
                → TermNode

Group         ← '(' Spacing Composition Spacing ')'
                → GroupNode { inner: Composition }

# ═══════════════════════════════════════════════════
# OPERATORS (the 4 non-containment spatial relationships)
# ═══════════════════════════════════════════════════

Operator      ← Connection / Adjacency / Intersection
                → OperatorNode

Connection    ← Arrow AngleMod?
                → ConnectionNode { direction: Direction, angle: Option<f64> }

Adjacency     ← '|'
                → AdjacencyNode

Intersection  ← '&'
                → IntersectionNode

# ═══════════════════════════════════════════════════
# MARKS (geometric primitives with optional contents)
# ═══════════════════════════════════════════════════

Mark          ← Primitive Content?
                → MarkNode { primitive: Primitive, content: Option<Composition> }

Content       ← '{' Spacing Composition Spacing '}'
                → ContentNode { inner: Composition }

# ═══════════════════════════════════════════════════
# PRIMITIVES
# ═══════════════════════════════════════════════════

Primitive     ← Point / Enclosure / Curve / Arrow / AngleMod
                → PrimitiveNode

# Points
Point         ← FilledPoint
FilledPoint   ← '●' / '*'
                → PointNode

# Enclosures
Enclosure     ← Circle / Triangle / Square
Circle        ← '○' / '/0'
                → EnclosureNode { shape: Circle }
Triangle      ← '△' / '/3'
                → EnclosureNode { shape: Triangle }
Square        ← '□' / '/4'
                → EnclosureNode { shape: Square }

# Relations
Arrow         ← RightArrow / LeftArrow / BiArrow
RightArrow    ← '→' / '->'
                → ArrowNode { direction: Right }
LeftArrow     ← '←' / '<-'
                → ArrowNode { direction: Left }
BiArrow       ← '↔' / '<->'
                → ArrowNode { direction: Both }

# Curves
Curve         ← '~'
                → CurveNode

# Angle modifiers
AngleMod      ← ('∠' / '@') Number
                → AngleNode { measure: f64 }

# ═══════════════════════════════════════════════════
# TERMINALS
# ═══════════════════════════════════════════════════

Number        ← [0-9]+ ('.' [0-9]+)?
                → f64

Spacing       ← [ \t]*
LineBreak     ← '\n' / '\r\n'
LineEnd       ← LineBreak / EOF
EOF           ← !.
```

---

## AST Node Types

```rust
enum AstNode {
    Document { glyphs: Vec<AstNode> },
    Composition { terms: Vec<AstNode>, operators: Vec<AstNode> },
    Group { inner: Box<AstNode> },
    Mark { primitive: Primitive, content: Option<Box<AstNode>> },
    Connection { direction: Direction, angle: Option<f64> },
    Adjacency,
    Intersection,
}

enum Primitive {
    Point,
    Enclosure(Shape),
    Arrow(Direction),
    Curve,
    Angle(f64),
}

enum Shape { Circle, Triangle, Square }
enum Direction { Right, Left, Both }
```

---

## Precedence and Associativity

All operators are **left-associative**. Precedence (highest to lowest):

| Level | Operator | Syntax | Associativity |
|-------|----------|--------|---------------|
| 1 | Containment | `{...}` | Right (nests) |
| 2 | Angle modification | `@N` | Right (modifies preceding connection) |
| 3 | Intersection | `&` | Left |
| 4 | Adjacency | `\|` | Left |
| 5 | Connection | `→` `←` `↔` | Left |

Example: `○{●} | △{●} → □{●}` parses as `(○{●} | △{●}) → □{●}` because adjacency binds tighter than connection.

To override: `○{●} | (△{●} → □{●})`

---

## Ambiguity Resolution

PEG grammars are unambiguous by construction (ordered choice). Potential ambiguities:

| Input | Resolution | Rule |
|-------|-----------|------|
| `→@60` | Connection with angle modifier | `AngleMod` binds to preceding `Arrow` |
| `@60` standalone | Freestanding angle primitive | `AngleMod` as `Primitive` via `Mark` |
| `→ →` | Two consecutive connections | Each `→` is a separate connection operator |
| `{}` | Empty content = empty composition | Valid but produces no children |

---

## Recommended Parser Implementation

**Library:** `pest` (Rust PEG parser generator) or hand-written recursive descent.

`pest` advantages: grammar file is separate `.pest` file, auto-generates parser, good error messages.

Hand-written advantages: better error recovery for live preview, easier incremental parsing.

**Recommendation for v1:** Use `pest` for correctness, switch to hand-written if incremental parsing is needed (Phase 3+).
