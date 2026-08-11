# Phase 1 — Modal Semantics: Design Questions

**Status:** OPEN — All questions require formal resolution before implementation

---

## M1: What is the sort of a possible world?

**Options:**

### A: World = Entity
A world w ∈ Gₑ — worlds are entities (compact subsets of ℝ²).

- **For:** Reuses existing sort. Worlds are "things" we can predicate about.
- **Against:** Entities are meant to be compact subsets of glyph space; a world is a global configuration. The size mismatch is problematic.
- **Test:** Can `embed(conjoin(a₁, a₂, ...))` produce an entity that behaves as a world?

### B: World = Embedded assertion-collection
A world is `embed(w_content)` where `w_content` is a maximal consistent conjunction of assertions.

- **For:** Semantically natural. Stays within 4 sorts. Uses `embed` (nominalization) as intended.
- **Against:** "Maximal consistent" is not a first-order property of the algebra. Requires a meta-level consistency notion.
- **Test:** Can accessibility relations be defined on embedded assertion-collections?

### C: World = New sort W
Add W as a 5th sort to Σ_UL.

- **For:** Clean separation. Worlds become first-class. Modal operations typecheck cleanly.
- **Against:** Breaks 4-sort minimality. Must re-derive sort necessity from geometric primitives (Enclosure → W?). All downstream proofs need updating.
- **Test:** Is there a geometric primitive that naturally generates W? (Possibly: Enclosure generates Concept/World.)

### Recommendation
Investigate B first. If B works, maximum parsimony is preserved. If B fails (likely on maximality/consistency requirements), fall back to C.

**Decision criterion:** B works if and only if all 5 modal cases can be formalized without requiring operations that take worlds as a distinct type.

---

## M2: How many modal operators?

**Options:**

### Single □/◇ pair + accessibility parameter
One pair of operators, parameterized by accessibility relation:
```
necessary(r_acc, a) → a
possible(r_acc, a) → a
```

Different modal flavors = different r_acc: r_alethic, r_epistemic, r_deontic, etc.

- **For:** Minimal. Standard Kripke semantics uses this approach.
- **Against:** Fundamentally different modal logics have different axioms (S5 for alethic, KD for deontic, S4 for epistemic). A single operator with different accessibility relations doesn't capture these structural differences.

### Separate operators per flavor
```
necessary_alethic(a)    possible_alethic(a)
necessary_epistemic(a)  possible_epistemic(a)
necessary_deontic(a)    possible_deontic(a)
```

- **For:** Each operator has its own axioms. Clean typing.
- **Against:** Combinatorial explosion. Each new modal flavor adds 2 operations.

### Recommendation
Single □/◇ pair parameterized by accessibility relation. The axiomatic differences between modal flavors are captured by constraints on r_acc (reflexive, transitive, serial, etc.), not by distinct operators. This is standard in multi-modal logic.

**Key insight:** The accessibility relation already carries the modal flavor. r_epistemic is reflexive + transitive (KT4 = S4). r_deontic is serial (KD). r_alethic is reflexive + transitive + symmetric (S5). The operator is the same; the relation varies.

---

## M3: How do modal operators compose?

"She should have been able to finish earlier" (4.5) stacks: deontic + ability + counterfactual + temporal.

**Structure:** `should(able(counterfactual(earlier(finish(she)))))`

Or in Kripke terms: In all deontically ideal worlds, there exists a world accessible via her capacities, such that in the closest world where the timeline permits...

**Key question:** Is composition just function composition? Given:
```
□_O(a) = ∀w. O-accessible(w) → a_in_w
◇_A(a) = ∃w. A-accessible(w) ∧ a_in_w
```

Then `□_O(◇_A(a))` = `∀w₁. O-acc(w₁) → ∃w₂. A-acc_in_w₁(w₂) ∧ a_in_w₂`

**Observation:** Composition works if:
1. Each modal operator maps assertions to assertions (closure)
2. Accessibility relations in inner modals can depend on the evaluation world (cross-world accessibility)

**Risk:** Cross-world accessibility (condition 2) is well-studied in multi-modal logic but geometrically non-trivial.

**Decision needed:** Accept function composition as the default, or design an explicit composition operation?

---

## M4: What is the geometric realization of accessibility?

**Options:**

### A: Gauge connection on world-space
Accessibility = parallel transport along paths in a world-space manifold.
- Worlds are points. Accessible worlds are connected by paths. The gauge connection determines how propositions "transform" between worlds.
- **Leverages:** Gauge bundle machinery from Expedition One.

### B: Lines between enclosures
Accessibility = directed relation (line) connecting world-enclosures.
- Simpler, directly uses the Line → Relation primitive.
- No differential geometry needed.

### C: Nested enclosures
Accessible worlds are enclosures INSIDE a larger enclosure representing the modal domain.
- Alethic accessibility: the containing enclosure includes ALL worlds.
- Epistemic accessibility: the containing enclosure includes worlds compatible with knowledge.
- **Leverages:** Enclosure → Concept primitive.

### Recommendation
Start with **B** (lines between enclosures) for simplicity. Upgrade to **A** (gauge connection) when counterfactuals require a metric on world-space (for "closest world" semantics).

---

## M5: Does modality need a 5th sort?

**Analysis:**

The `modify_assertion` operation (Pass 1.2) established the pattern of enriching the assertion frame: (F, C, σ) → (F, C, σ, decorations). Modal operators could follow this pattern: add modal decorations to the assertion frame.

But Kripke semantics treats □/◇ as **quantifiers over worlds**, not as frame decorations. This makes them structurally closer to `quantify` (which maps m × e → a) than to `modify_assertion` (which maps m × a → a).

**Resolution path:** If modality is expressible via `bind` + `quantify` over embedded worlds (no new operations), no 5th sort is needed. If a new operation is needed that takes a "world" argument, the argument's type determines whether we need a new sort.

**Decision criterion:** Try to express all 5 cases using existing sorts + distinguished relations. Add a 5th sort only if type errors force it.

---

## M6: Is the topos embedding necessary?

**Analysis:**

Standard Kripke semantics works in set theory (no topos needed). The topos approach gives:
- Internal logic: the subobject classifier Ω replaces {true, false}
- Presheaf semantics: truth values are "truth over a context" (assignment of truth to each world)
- This is more general but potentially overkill for the initial extension.

**Recommendation:** Skip the topos for Phase 1. Use standard Kripke semantics. Note the topos approach as a future upgrade path (provides constructive proofs, connects to intuitionistic logic, enables richer truth values for fuzzy/graded modality).

**Revisit criterion:** If Phase 1 encounters a case where classical Kripke semantics is insufficient (e.g., graded modality "probably" requiring continuous truth values), then bring in the topos.

---

## M7: How does modality interact with negate?

In classical modal logic: ◇a = ¬□¬a and □a = ¬◇¬a.

In UL terms: `possible(r_acc, a) = negate(necessary(r_acc, negate(a)))`

This means if we define □, we get ◇ for free (as derived), or vice versa. This is exactly the pattern of conjoin being derived from negate + disjoin.

**Key check:** Does boundary inversion σ⊕↔⊖ compose correctly with modal operators?

If `necessary(r_acc, a)` is an assertion with σ = ⊕, then:
- `negate(necessary(r_acc, a))` flips to σ = ⊖: "it is NOT necessary that a"
- But `possible(r_acc, negate(a))` means "it IS possible that NOT-a"
- These are different! ¬□a ≠ ◇¬a in general. (They're equivalent iff: ¬□a ↔ ◇¬a, which is the definition of ◇.)

**Resolution:** This is not a problem — it's the DEFINITION. ◇ is defined as ¬□¬. The boundary inversion interacts correctly because negate is applied at two levels: outer (negating the modal assertion) and inner (negating the embedded content).

**Formal proof needed:** Show that `negate(necessary(r_acc, negate(a)))` produces an assertion with the correct truth conditions for ◇(a). This requires formalizing what "truth in a world" means for the necessary operator.
