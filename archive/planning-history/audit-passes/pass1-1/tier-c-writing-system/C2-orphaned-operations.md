# C2 — Which Operations Have No Visual Realization?

**Tier:** C (Writing System)  
**Finding:** F6 (Missing visual↔algebraic mapping)  
**Targets:** `ul-core/writing-system/`, `ul-core/symbology/`, `ul-core/syntax/`, `ul-core/grammar/`  
**Dependencies:** B4 (if an orphaned operation turns out to be redundant, the "orphan" is moot)  
**Effort:** 1–2 weeks for documentation; longer if new visual conventions must be designed

---

## The Orphaned Operations

Three Σ_UL operations have no clear visual construction in the writing system:

### 1. `abstract: e → m` ("Entity to Modifier")

**Algebraic definition:** Takes an entity and produces a modifier — the "quality-extracting" operation.  
**Linguistic analog:** "wood" (entity) → "wooden" (modifier); "child" (entity) → "childlike" (modifier)

**What the writing system says:** No dedicated section. The thesaurus mentions modifiers (angles, scaling) but does not describe how to visually derive a modifier FROM an entity.

**Question:** How would a writer of UL visually indicate that a mark (entity) is being used as a modifier? Possible conventions:
- A: Place the entity mark at an angle to the thing it modifies (the angle IS the modifier)
- B: Draw a dashed version of the entity mark (indicating it's being used metaphorically, not literally)
- C: Reduce the entity mark to a smaller scale and attach it to another mark (size reduction = adjectivalization)

None of these are documented. The operation exists in the algebra but not in the writing system.

### 2. `compose: r × r → r` ("Relation Composition")

**Algebraic definition:** Takes two relations and produces their transitive composition — "r₁ then r₂."  
**Linguistic analog:** "causes" + "enables" → "causes-to-enable" (causal chain); "above" + "left-of" → "above-and-left-of" (spatial composition)

**What the writing system says:** `syntax-dictionary.md` shows arrows (→) as relations and mentions "paths," but there is no explicit convention for showing that two consecutive arrows compose into a single combined relation (as opposed to being two separate predications).

**Question:** If I write `A → B → C`, is this:
- (a) `predicate(A, r₁, B)` AND `predicate(B, r₂, C)` — two separate assertions?
- (b) `predicate(A, compose(r₁, r₂), C)` — one assertion with a composed relation?

The visual is ambiguous. A convention is needed to distinguish these readings.

**Possible conventions:**
- A: A merged arrow (double-lined or thickened) indicates composition: `A ═══→ C` means `compose(r₁, r₂)`
- B: A labeled middle node (circled) indicates composition: `A → (B) → C` means the path through B composes
- C: An explicit composition mark: `A →∘→ C` where ∘ marks the join point

### 3. `quantify: m × e → a` ("Quantification")

**Algebraic definition:** Takes a modifier and an entity and produces an assertion — "all/some/no entities [of type e] satisfy [modifier m]."  
**Linguistic analog:** "∀x.P(x)" = "for all x, P holds"; "∃x.P(x)" = "there exists an x such that P holds"

**What the writing system says:** `symbol-map.md` §VII mentions universality (○ = Universal) and `grammar-book.md` mentions scope, but there is no explicit convention for writing "for all X" or "there exists an X" using visual marks.

**Question:** How does a UL writer express "all cats are mammals" vs. "some cats are black"?  
- The entity "cat" is a mark.  
- The modifier "all" or "some" is a modifier mark.  
- The resulting assertion needs a frame.

But the visual construction — how the quantifier-modifier attaches to the entity-mark within a frame — is not specified.

**Possible conventions:**
- A: Global modifier (large angle spanning the entire entity): "∀" = full-circle modifier; "∃" = partial-angle modifier
- B: Frame decoration: a frame with a specific border pattern indicates universal vs. existential scope
- C: Prefix mark: a special mark before the entity indicates quantification type

## Impact Assessment

| Operation | Frequency in Natural Language | Urgency |
|-----------|------------------------------|---------|
| abstract | Very common (adjective derivation) | HIGH — core operation |
| compose | Common (causal chains, spatial composition) | MEDIUM — can be approximated by sequential predication |
| quantify | Essential for logic and science | HIGH — blocks formal reasoning examples |

## Recommendation

Design visual conventions for all three operations. For each, the convention should:
1. Be consistent with existing writing system patterns
2. Be unambiguous (distinguishable from similar constructions)
3. Be geometrically motivated (not arbitrary symbol choices)
4. Have a clear relationship to the algebraic operation

Update: `symbol-map.md`, `syntax-dictionary.md`, `grammar-book.md`, `glyph-composition.md`

## Status

**Status:** ❌ OPEN — 3 of 11 operations lack visual realization. This is a significant completeness gap in the writing system.
