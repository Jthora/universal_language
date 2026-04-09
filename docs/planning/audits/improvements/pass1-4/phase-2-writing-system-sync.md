# Phase 2 — Writing System Synchronization

**Status:** 🔲 NOT STARTED  
**Risk:** Medium (large surface area — ~25k lines across 7 files, but changes are additive)  
**New operations:** 0 (synchronizing existing ops into prose documents)  
**Dependencies:** Phase 1 (count sweep) complete  
**Blocks:** Phase 4 (worked examples need updated documents to reference)

---

## Problem Statement

The formal algebra documents (formal-foundations.md, formal-operations.md, formal-grammar.md) were updated during Pass 1.2. The prose writing system documents were not. Users trying to learn UL from the writing system will encounter an incomplete picture — 3 operations (bind, modify_assertion, graduated quantify) exist in the algebra but not in the teaching materials.

---

## Per-Document Update Plan

### 2A: writing-system.md (~10k lines)

**Current state:** Covers 11 operations. No mention of bind, modify_assertion, or graduated quantify.

**Updates needed:**
1. **§4.2 (Operation Mapping)** — Add bind, modify_assertion, and graduated quantify to the operation table with geometric grounding descriptions
2. **§6 (Reading/Writing)** — Add subsections for:
   - Reading bind constructions (hollow → filled marks, scope nesting)
   - Reading modify_assertion constructions (frame decoration styles: dotted, double, wavy)
   - Reading graduated quantification (partial frame fill vs. binary fill/empty)
3. **§8.2 (JSON Structural Encoding)** — Add `bind` and `modify_assertion` to the type system schema
4. **§8.3 (BNF Linear Serialization)** — Add production rules for bind and modify_assertion

**Estimated scope:** ~200 lines of additions across 4 sections.

### 2B: syntax-dictionary.md (~3.1k lines)

**Current state:** §III lists C1–C11 with detailed syntactic rules. No C12/C13.

**Updates needed:**
1. **§III heading** — "14 SYNTACTIC OPERATIONS"
2. **§III.12 (C12 — Bind)** — New section:
   - Input: slot entity (○_x) + assertion containing ○_x
   - Operation: replace ○_x with bound entity (●_x), establishing co-reference
   - Visual: hollow mark → filled mark transformation
   - Scope: nesting depth determines quantifier scope ordering
   - Connection rules: ○ must appear inside the assertion's frame; ● inherits position
3. **§III.13 (C13 — Modify Assertion)** — New section:
   - Input: modifier + assertion
   - Operation: decorate the assertion's frame boundary (∂F)
   - Visual: dotted border (evidential), double border (emphatic), wavy border (hedged)
   - Connection rules: modifier applies to frame as a whole, not to content
   - Distinction from negate: modify_assertion changes ∂F; negate changes σ
4. **§VIII (Reference Table)** — Add 3 new rows

**Estimated scope:** ~150 lines of additions.

### 2C: grammar-book.md (~3.3k lines)

**Current state:** Covers 11 operations in prose. §VIII reference table has 11 rows.

**Updates needed:**
1. **§V (Construction Rules)** — Add subsections:
   - Co-reference and scope nesting (how bind works in practice)
   - Assertion-level modification (evidentiality, emphasis, hedging)
   - Graduated quantification (proportion reading vs. binary reading)
2. **§VIII (Reference Table)** — Add 3 new rows with prose descriptions
3. **Cross-reference** to formal-grammar.md C12, C13 for precise definitions

**Estimated scope:** ~200 lines of additions.

### 2D: symbol-map.md (~2.4k lines)

**Current state:** Has hollow mark ○ entry (added in Pass 1.2) but incomplete. No frame decoration marks.

**Updates needed:**
1. **Hollow mark (○) entry** — Complete with:
   - Binding semantics: ○_x is a slot; bind(○_x, a) fills the slot
   - Visual: hollow circle at entity position; filled (●) after binding
   - Scope indication: which frame the ○ belongs to determines its quantifier scope
2. **Frame decoration marks** — New entries for:
   - Dotted frame boundary (evidential/reported)
   - Double frame boundary (emphatic/certain)
   - Wavy frame boundary (hedged/uncertain)
   - Distinction from solid (assertive) and dashed (negated)

**Estimated scope:** ~80 lines of additions.

### 2E: lexicon.md (~8.4k lines)

**Current state:** Has cross-reference entries for bind and modify_assertion in §7 operations table, but no body sections in §1–§6.

**Updates needed:**
1. **Body section for bind** — Canonical examples:
   - "Every student read some book" (two-quantifier scope)
   - "The person who loves herself" (reflexive co-reference)
2. **Body section for modify_assertion** — Canonical examples:
   - "Apparently she left" (evidential)
   - "He DEFINITELY won" (emphatic)
   - "Maybe it rained" (hedged)
3. **Verify §7 cross-reference table** — Correct signatures for bind (e × a → a, not m × e → a)

**Estimated scope:** ~100 lines of additions.

### 2F: thesaurus.md (~2.9k lines)

**Current state:** Semantic pathways for original operations only. Quantify described as binary (large/small).

**Updates needed:**
1. **Binding pathways** — How binding relates to other operations (embed for self-reference, quantify for scope)
2. **Evidentiality pathways** — How modify_assertion relates to modify_entity (entity-level vs. assertion-level quality)
3. **Quantification continuum** — Update from binary to graduated, with semantic pathway from "all" through "most" to "some" to "few"

**Estimated scope:** ~60 lines of additions.

---

## Execution Order

1. **symbol-map.md** first (atomic marks are the foundation)
2. **syntax-dictionary.md** second (connection rules reference marks)
3. **grammar-book.md** third (construction rules reference syntax)
4. **writing-system.md** fourth (complete specification references all below)
5. **lexicon.md** fifth (definitions reference writing rules)
6. **thesaurus.md** sixth (pathways reference definitions)

This follows the UL pipeline order: Symbology → Syntax → Grammar → (Writing System) → Lexicon → Thesaurus.

---

## Deliverables

- [ ] symbol-map.md — complete ○ entry + frame decoration marks
- [ ] syntax-dictionary.md — add C12, C13 + update heading/table
- [ ] grammar-book.md — add bind/modify_assertion/graduated sections + update table
- [ ] writing-system.md — add operation sections + update JSON/BNF schemas
- [ ] lexicon.md — add bind/modify_assertion body sections
- [ ] thesaurus.md — add binding/evidentiality/graduated pathways
- [ ] Cross-check: every new section references formal-grammar.md C12/C13 and formal-operations.md §1.12/§1.13
