# Phase 4 — Worked Examples

**Status:** 🔲 NOT STARTED  
**Risk:** Low (additive content; follows established format)  
**New operations:** 0 (demonstrating existing operations)  
**Dependencies:** Phase 2 (writing system sync — examples should reference updated docs)  
**Blocks:** None

---

## Problem Statement

The `writers-companion.md` currently has ~6 worked examples. None use `bind`, `modify_assertion`, or graduated `quantify`. After Pass 1.2 extended the algebra and Pass 1.3 formalized conventions (cardinality, polyadic reduction, morphological transparency), the example set needs to cover the full operation inventory.

---

## Current Example Coverage

| # | Example | Operations Used | Covers New Ops? |
|---|---------|----------------|-----------------|
| 1 | Basic predication | predicate, modify_entity | ❌ |
| 2 | Negation | negate, predicate | ❌ |
| 3 | Conjunction/disjunction | conjoin, disjoin | ❌ |
| 4 | Nominalization | embed, predicate | ❌ |
| 5 | Adjectivalization | abstract, modify_entity | ❌ |
| 6 | "Love is patient" | modify_entity, abstract, predicate | ❌ |

**Gap:** Zero examples of bind, modify_assertion, graduated quantify, compose, invert, or the Pass 1.3 conventions.

---

## Proposed New Examples

### Example 7: Multi-Quantifier Scope (bind + quantify)

**Natural language:** "Every student read some book"  
**Why this example:** Demonstrates `bind` for variable binding and `quantify` for graduated quantification. Shows scope ambiguity resolution via nesting depth.

**Operations exercised:** bind, quantify, predicate, modify_entity

**Two decompositions:**
- Reading 1 (∀ > ∃): outer bind on student, inner bind on book
- Reading 2 (∃ > ∀): outer bind on book, inner bind on student

**Teaching point:** Scope = nesting depth. The visual diagram makes the scope ordering unambiguous even when the natural language is ambiguous.

### Example 8: Evidential Marking (modify_assertion)

**Natural language:** "Apparently she left early"  
**Why this example:** Demonstrates `modify_assertion` with evidential frame decoration. Clean, simple case.

**Operations exercised:** modify_assertion, predicate, modify_relation

**Decomposition:** Build assertion "she left early" (predicate + modify_relation for temporal), then apply modify_assertion with evidential modifier (dotted frame).

**Teaching point:** modify_assertion changes the *frame boundary* (how the assertion is presented), not the *content* (what is asserted) or the *sign* (whether it's asserted or denied).

### Example 9: Proportional Quantifier (graduated quantify)

**Natural language:** "Most birds can fly"  
**Why this example:** Demonstrates the p ∈ [0,1] extension of quantify. Shows the visual difference between "all" (p=1, full fill), "most" (p≈0.7, partial fill), and "some" (p→0⁺, point).

**Operations exercised:** quantify (graduated), predicate

**Teaching point:** The visual representation — how much of the frame is filled — directly encodes the quantificational force. Readers can see at a glance whether the claim is about all, most, some, or few.

### Example 10: Self-Reference (embed recursion)

**Natural language:** "The idea that all ideas are expressible is itself an idea"  
**Why this example:** Demonstrates `embed` creating self-referential structure. Non-paradoxical (unlike the Liar). Shows healthy recursion.

**Operations exercised:** embed, quantify, predicate, abstract

**Teaching point:** embed turns assertions into entities that can participate in further assertions — including assertions about themselves. When the recursion is non-contradictory, this is perfectly well-defined.

### Example 11: Ditransitive Decomposition (polyadic reduction convention)

**Natural language:** "She gave him the book"  
**Why this example:** Demonstrates the Pass 1.3 polyadic reduction convention. Shows how 3-place predicates decompose into binary chains.

**Operations exercised:** predicate, compose, conjoin

**Two decomposition strategies:**
- Conjunction: pred(she, give-to, him) ∧ pred(she, give-theme, book)
- Event-based: pred(emb(pred(she, give, book)), to, him)

**Teaching point:** Binary predication is not a limitation — it's a decomposition principle. All polyadic relationships reduce to binary chains (Peirce's reduction thesis).

### Example 12: Cross-Linguistic (full operation set)

**Natural language:** Turkish "Gelmiş" (evidential past — "apparently [someone] came")  
**Why this example:** Exercises multiple new operations in a single compact expression. Demonstrates cross-linguistic universality.

**Operations exercised:** modify_assertion (evidential), modify_relation (past tense), predicate, bind (implicit subject)

**Teaching point:** UL decomposes meaning, not morphology. A single Turkish morpheme encodes multiple UL operations, just as multiple English words may encode a single UL operation.

---

## Verification of Existing Examples

In addition to new examples, the 6 existing examples need **verification** against the corrected negation mechanism (boundary inversion σ⊕↔⊖ instead of the old reflection-based negation).

| Example | Uses negation? | Needs verification? |
|---------|---------------|-------------------|
| 1 | No | No |
| 2 | Yes | **YES** — must use boundary inversion, not reflection |
| 3 | Indirectly (conjoin/disjoin) | Maybe — check if De Morgan derivation is shown |
| 4 | No | No |
| 5 | No | No |
| 6 | No | No |

---

## Format for Each Example

Following the established writers-companion.md pattern:

```
### Example N: [Title]

**Natural language:** "[sentence]"

**Step 1 — Identify entities, relations, modifiers:**
[List all semantic primitives]

**Step 2 — Select operations:**
[Which operations are needed and why]

**Step 3 — Formal decomposition:**
[Algebraic expression using operation notation]

**Step 4 — Visual construction:**
[ASCII/description of the geometric construction]

**Step 5 — Reading back:**
[Read the visual construction back to natural language to verify]
```

---

## Deliverables

- [ ] Write Example 7 (multi-quantifier scope)
- [ ] Write Example 8 (evidential marking)
- [ ] Write Example 9 (proportional quantifier)
- [ ] Write Example 10 (self-reference)
- [ ] Write Example 11 (ditransitive)
- [ ] Write Example 12 (cross-linguistic)
- [ ] Verify Example 2 (negation) uses boundary inversion
- [ ] Verify Examples 3 (conjunction/disjunction) for De Morgan consistency
- [ ] Add all examples to writers-companion.md
- [ ] Update writers-companion.md §5.2 operation decision tree to include Q12/Q13
