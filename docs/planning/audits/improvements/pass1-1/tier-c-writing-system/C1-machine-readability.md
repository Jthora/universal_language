# C1 — Is UL Machine-Readable?

**Tier:** C (Writing System)  
**Finding:** F6 (Missing visual↔algebraic mapping table)  
**Targets:** All writing system documents in `ul-core/`  
**Dependencies:** Independent  
**Effort:** 2–4 weeks for analysis; 2–6 months if a formal grammar is to be constructed

---

## The Problem

UL has:
- A **formal algebraic specification** (Σ_UL) with 4 sorts, 11 operations, rigorous type signatures
- A **visual writing system** with Marks, Connectors, Frames, and construction rules

But NO:
- Formal grammar (BNF, PEG, or context-free grammar) for parsing UL expressions
- Bijective mapping between algebraic operations and visual constructions
- Machine-readable specification that could be implemented in a parser

### What Exists

The writing system describes constructions in natural language prose:

> "An arrow (→) connecting two marks indicates a directed relation." (syntax-dictionary.md)

> "A closed boundary around a construction creates an assertion frame." (grammar-book.md)

This is human-readable but NOT machine-parseable. A parser cannot be built from these descriptions alone — they're ambiguous, context-dependent, and incomplete.

### What's Missing

A formal grammar might look like:

```bnf
<assertion>   ::= <frame> "(" <content> ")"
<content>     ::= <predication> | <negation> | <conjunction> | <disjunction>
<predication> ::= <entity> <relation> <entity>
<negation>    ::= "¬" <assertion>
<conjunction> ::= <assertion> "∧" <assertion>
<disjunction> ::= <assertion> "∨" <assertion>
<entity>      ::= <mark> | <modified-entity> | <embedded-assertion>
<relation>    ::= <connector> | <modified-relation> | <composed-relation> | <inverted-relation>
...
```

This doesn't exist for UL. The writing system describes visual constructions; the algebra describes typed operations. The bridge between them is informal.

## The Questions

### Q8a: Can UL be parsed by a machine?

Current state: **No.** The writing system is designed for human readers. It uses spatial relationships (above/below, inside/outside, connected/disconnected) that require 2D spatial reasoning to interpret — not just 1D string parsing.

**Fundamental challenge:** UL is a *2D* writing system (marks in a plane), but formal grammars are designed for *1D* strings. Parsing UL requires a 2D grammar formalism, such as:
- **Graph grammars** (rules for constructing and parsing labeled graphs)
- **Picture grammars** (rules for 2D spatial arrangements)
- **Categorical grammars** (morphisms between visual and algebraic categories)

This is a known hard problem in formal language theory. 2D grammars exist but are significantly more complex than 1D grammars.

### Q8b: Is the visual↔algebraic mapping bijective?

The current coverage:

| Algebraic Operation | Visual Realization | Documented? |
|---|---|---|
| predicate(e, r, e) → a | Entity–Arrow–Entity in frame | ✅ Yes |
| modify_entity(m, e) → e | Modifier mark near entity | ✅ Yes |
| modify_relation(m, r) → r | Modifier mark near connector | ✅ Yes |
| negate(a) → a | Reflection (WRONG — see B1) | ✅ But incorrect |
| conjoin(a, a) → a | Adjacent frames | ✅ Yes |
| disjoin(a, a) → a | Overlapping frames? | ⚠️ Unclear |
| embed(a) → e | Frame used as entity | ✅ Yes |
| abstract(e) → m | ? | ❌ No clear visual |
| compose(r, r) → r | ? | ❌ No clear visual |
| invert(r) → r | Reverse arrow direction | ✅ Yes |
| quantify(m, e) → a | ? | ❌ No clear visual |

**3 operations have no documented visual realization:** abstract, compose, quantify.

### Q8c: Does the writing system contain constructions NOT in the algebra?

The writing system documents 9 visual categories × several constructions each = ~50+ distinct visual constructions. The algebra has 11 operations. Many visual constructions may be *compositions* of operations, but the mapping is undocumented.

## Deliverable

Three items:
1. **Complete bidirectional map:** A table mapping every algebraic operation to its visual construction AND every visual construction to its algebraic decomposition
2. **Gap analysis:** List of operations without visuals and visuals without algebraic counterparts
3. **Feasibility assessment:** Can a 2D parser for UL be built? What formalism would be needed? What are the computational complexity bounds?

## Relationship to UL Forge

The `ul-forge/` directory contains a Rust implementation. The `schemas/` directory presumably contains machine-readable schemas. If these schemas define a parseable format, the "machine-readability" question may already be partially answered — but for a text serialization, not for the 2D visual writing system itself.

## Status

**Status:** ✅ RESOLVED — Spatial construction grammar created in `ul-core/grammar/formal-grammar.md`. Contains 11 construction rules (C1–C11), formal reading algorithm (5-pass), writing algorithm (7-step), disambiguation hierarchy, sort-compatibility table, and serialization format. Full 2D image parser unimplemented but the grammar–algebra bridge is now formally specified.
