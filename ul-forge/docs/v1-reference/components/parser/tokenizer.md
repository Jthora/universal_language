# Tokenizer Specification

> How UL-Script text is split into tokens before parsing.

---

## Token Types

```rust
enum Token {
    // Primitives
    Point,              // ● or *
    Circle,             // ○ or /0
    Triangle,           // △ or /3
    Square,             // □ or /4
    RightArrow,         // → or ->
    LeftArrow,          // ← or <-
    BiArrow,            // ↔ or <->
    Tilde,              // ~
    AnglePrefix,        // ∠ or @
    
    // Structure
    OpenBrace,          // {
    CloseBrace,         // }
    Pipe,               // |
    Ampersand,          // &
    OpenParen,          // (
    CloseParen,         // )
    Comma,              // ,
    
    // Literals
    Number(f64),        // [0-9]+ ('.' [0-9]+)?
    
    // Meta
    Comment(String),    // # to end of line
    Newline,            // \n or \r\n
    EOF,                // end of input
}
```

---

## Unicode → Token Mapping

| Input | Token | Unicode codepoint |
|-------|-------|------------------|
| `●` | `Point` | U+25CF BLACK CIRCLE |
| `○` | `Circle` | U+25CB WHITE CIRCLE |
| `△` | `Triangle` | U+25B3 WHITE UP-POINTING TRIANGLE |
| `□` | `Square` | U+25A1 WHITE SQUARE |
| `→` | `RightArrow` | U+2192 RIGHTWARDS ARROW |
| `←` | `LeftArrow` | U+2190 LEFTWARDS ARROW |
| `↔` | `BiArrow` | U+2194 LEFT RIGHT ARROW |
| `~` | `Tilde` | U+007E TILDE |
| `∠` | `AnglePrefix` | U+2220 ANGLE |

---

## ASCII Fallback Sequences

Multi-character ASCII sequences are normalized to single-token Unicode equivalents:

| ASCII | Normalized to | Rule |
|-------|--------------|------|
| `*` | `Point` | Single character |
| `/0` | `Circle` | Slash + digit |
| `/3` | `Triangle` | Slash + digit |
| `/4` | `Square` | Slash + digit |
| `->` | `RightArrow` | Dash + greater |
| `<-` | `LeftArrow` | Less + dash |
| `<->` | `BiArrow` | Less + dash + greater (longest match) |
| `@` | `AnglePrefix` | Single character |

**Longest match rule:** `<->` is matched as `BiArrow` before `<-` is matched as `LeftArrow`. The tokenizer is greedy on multi-character sequences.

---

## Whitespace Handling

- Spaces and tabs between tokens are skipped (not emitted)
- Newlines are emitted as `Newline` tokens (significant for separating glyph definitions)
- Multiple consecutive newlines are collapsed to a single `Newline` token

---

## Comment Handling

- `#` begins a comment
- Everything from `#` to the next newline (or EOF) is a `Comment` token
- Comments are emitted (for source mapping) but ignored by the parser

---

## Error Handling

- Unknown characters: emit a `LexError` diagnostic and skip the character
- Unterminated multi-character sequences (e.g., `<` not followed by `-`): emit `<` as an error token
- Lone `/` not followed by `0`, `3`, or `4`: emit as error token

The tokenizer never stops on error — it always produces a complete token stream, with error tokens marked for the error reporter.

---

## Source Mapping

Every token carries its source span:

```rust
struct Span {
    start: usize,  // byte offset in source
    end: usize,    // byte offset in source
    line: u32,     // 1-based line number
    col: u32,      // 1-based column number (in characters, not bytes)
}

struct SpannedToken {
    token: Token,
    span: Span,
}
```

Source spans enable:
- Error messages with precise locations
- Click-to-select in the web editor (map rendered element → source position)
- Incremental re-tokenization (only re-tokenize the changed span)
