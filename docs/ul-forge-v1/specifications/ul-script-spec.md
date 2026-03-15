# UL-Script Language Specification v1.0

> The formal grammar, tokenization rules, and semantics of UL-Script — the keyboard-typeable encoding of UL glyphs.

---

## Overview

UL-Script is a text format that encodes the geometric structure of UL glyphs. It is designed to be:
- **Typeable:** Composed using standard keyboard input with Unicode support
- **Parseable:** Unambiguous grammar with deterministic parsing
- **Compact:** Minimal syntax overhead — structure is expressed concisely
- **Bidirectional:** Every valid UL-Script has a unique GIR, and every GIR has a canonical UL-Script representation (deparse)

---

## Character Set

### Primary symbols (Unicode)

| Symbol | Name | Unicode | Geometric primitive | Σ_UL sort |
|--------|------|---------|-------------------|-----------|
| `●` | Filled point | U+25CF | Point | Entity |
| `○` | Circle | U+25CB | Enclosure (circle) | Entity |
| `△` | Triangle | U+25B3 | Enclosure (triangle) | Entity |
| `□` | Square | U+25A1 | Enclosure (square) | Entity |
| `→` | Right arrow | U+2192 | Line (right) | Relation |
| `←` | Left arrow | U+2190 | Line (left) | Relation |
| `↔` | Bidirectional arrow | U+2194 | Line (both) | Relation |
| `~` | Tilde | U+007E | Curve | Relation/Process |
| `∠` | Angle symbol | U+2220 | Angle | Modifier |

### ASCII fallback

| Unicode | ASCII | Description |
|---------|-------|-------------|
| `●` | `*` | Filled point |
| `○` | `/0` | Circle |
| `△` | `/3` | Triangle |
| `□` | `/4` | Square |
| `→` | `->` | Right arrow |
| `←` | `<-` | Left arrow |
| `↔` | `<->` | Bidirectional arrow |
| `~` | `~` | Curve (same) |
| `∠` | `@` | Angle prefix |

### Structural syntax

| Symbol | Meaning |
|--------|---------|
| `{` `}` | Containment (contents are inside the preceding mark) |
| `\|` | Adjacency separator (marks sharing a boundary) |
| `&` | Intersection marker (marks overlapping) |
| `@N` | Angle measure (N degrees, e.g., `@60`) |
| `#` | Comment (to end of line) |
| `(` `)` | Grouping (for precedence) |
| `,` | Sequence separator within groups |

---

## Grammar (PEG)

> **Abridged grammar** for quick reference. The canonical annotated grammar is in
> [components/parser/grammar.md](../components/parser/grammar.md). The Pest
> implementation is in `crates/ul-core/src/parser/ul_script.pest`.

```peg
# Top-level
Document    ← Glyph (Newline+ Glyph)* EOF
Glyph       ← Composition / Mark
Comment     ← '#' (!Newline .)* Newline

# Composition
Composition ← Term (Operator Term)*
Term        ← Mark / Group
Group       ← '(' Composition ')'

# Operators (spatial relationships)
Operator    ← Adjacent / Connect / Intersect
Adjacent    ← '|'
Connect     ← Arrow AngleMod?
Intersect   ← '&'

# Marks (geometric primitives)
Mark        ← Primitive Content?
Primitive   ← Point / Enclosure / Line / Curve / Angle
Content     ← '{' Composition '}'

# Primitives  
Point       ← '●' / '*'
Enclosure   ← Circle / Triangle / Square
Circle      ← '○' / '/0'
Triangle    ← '△' / '/3'
Square      ← '□' / '/4'
Line        ← Arrow
Arrow       ← '→' / '->' / '←' / '<-' / '↔' / '<->'
Curve       ← '~'
Angle       ← AngleMod
AngleMod    ← ('∠' / '@') Number

# Literals
Number      ← [0-9]+
Newline     ← '\n' / '\r\n'
Whitespace  ← [ \t]*
```

---

## Semantics

### Containment: `Mark{Content}`

A mark followed by `{...}` means the content is *inside* the mark. This maps to `contains` edges in the GIR.

```
○{●}          # Point inside circle → "existence within concept"
△{○{●}}       # Point inside circle inside triangle → nested containment
□{● | ●}      # Two adjacent points inside square
```

### Connection: `Mark → Mark`

An arrow between marks creates a `connects` edge — a directed relation.

```
● → ●         # Point relates to point
○{●} → △{●}   # Concept connects to structure (directed)
● ← ●         # Reverse direction
● ↔ ●         # Bidirectional
```

### Adjacency: `Mark | Mark`

Pipe separator means marks share a boundary — adjacent but not overlapping.

```
○ | △          # Circle adjacent to triangle → "concept or structure"
● | ● | ●     # Three adjacent points (chained)
```

### Intersection: `Mark & Mark`

Ampersand means marks overlap — shared visual/semantic space.

```
○ & △          # Circle overlaps triangle → shared meaning
○{●} & △{●}   # Overlapping enclosures, each containing a point
```

### Angle modification: `@N`

Specifies the angle (in degrees) of a connection or modifier.

```
● →@60 ●       # Connection at 60 degrees
∠60             # Freestanding angle modifier (60°)
△{●} →@90 □{●} # Right-angle connection between enclosures
```

> **Pipeline note (v1.1):** At parse time, `@N` is a syntactic property of the
> Connection operator. During AST-to-GIR transformation, the angle becomes a
> separate Angle node attached to the Line node via a `modified_by` edge. See
> [ast-to-gir.md](../components/parser/ast-to-gir.md) Rule 3 for details.

### Comments

```
# This is a comment
○{●}  # Inline comment: the concept of existence
```

---

## Precedence

From highest to lowest binding:

1. **Containment** `{...}` — binds tightest to preceding mark
2. **Angle modification** `@N` — binds to preceding connection
3. **Intersection** `&` — binary operator
4. **Adjacency** `|` — binary operator
5. **Connection** `→` `←` `↔` — binary operator

Parentheses override precedence: `(○ | △) → □` means "circle-adjacent-to-triangle connects to square."

---

## Canonical Form

Every GIR can be deparsed to a canonical UL-Script representation. The canonical form is determined by:

1. **Tree-spine DFS order:** Nodes are emitted in depth-first order of the tree spine
2. **Containment before connection:** `{...}` before `→`
3. **Left-to-right for siblings:** Multiple children of the same parent are separated by `|`
4. **Cross-edges after tree structure:** Connections that aren't in the tree spine are appended

This ensures that `parse(deparse(gir)) == gir` for all valid GIR documents.

---

## Error Recovery

The parser attempts error recovery for interactive use (live preview):

- **Unclosed braces:** Insert implicit closing `}` at end of line
- **Unknown characters:** Skip and report warning
- **Ambiguous operators:** Default to adjacency (least semantic commitment)
- **Empty marks:** Treat as point (minimal primitive)

Error recovery is advisory — the resulting GIR is flagged with warnings and should not be used for persistence or interchange.
