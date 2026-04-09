# Phase 1A — Canonical Modifier Conventions

**Status:** ✅ COMPLETE (April 7, 2026)  
**Risk:** None  
**New operations:** 0  
**D2 cases affected:** 5.1, 5.2, 5.3, 5.5, 1.2, 10.1, 10.4  
**Expected case conversions:** ~7 (⚠️ → ✅)  
**Dependencies:** None  
**Blocks:** Phase 4 (Montague mapping needs stable modifier semantics)

---

## Problem Statement

The modifier carrier set Gₘ is formally defined as the union of Erlangen-hierarchy transformation groups:

```
Gₘ = Euc(2) ∪ Sim(2) ∪ Aff(2) ∪ Proj(2) ∪ Homeo(ℝ²)
```

This is mathematically precise but **semantically unspecified** for common linguistic categories. When the D2 challenge decomposes "She was running," it writes:

```
mr(m_past, mr(m_progressive, r_run)) → r_was_running
```

But `m_past` and `m_progressive` are invoked without saying WHICH element of Gₘ they are. The algebra has room for these modifiers (the carrier set is vast), but the convention that connects a specific transformation to a specific meaning has never been formally stated.

This is the "F4: Convention Gap" from the D2 challenge findings (§ Key Findings, F4).

### Why It Matters

1. **Learning UL:** A student encounter "apply a modifier for past tense" and has no reference for what to draw
2. **Machine parsing:** A parser cannot assign geometric transformations to temporal categories without a lookup table
3. **D2 scoring:** 7 cases scored ⚠️ specifically because "works if tense modifiers are accepted as Gₘ elements" — formalizing the convention turns these into ✅

---

## Proposed Solution: Canonical Modifier Assignment Table

A reference table mapping common semantic modifier categories to specific geometric transformation families within Gₘ. This is **naming**, not algebra — we're identifying which named elements live at which addresses in an already-defined space.

### Design Principle

Each assignment must satisfy two constraints:
1. **Geometric naturalness:** The transformation should be the most obvious geometric counterpart of the semantic operation
2. **Erlangen coherence:** The transformation must be an element of the appropriate Erlangen-level group (meaning the preserved/discarded properties match the semantic category)

### The Table

#### A. Temporal Modifiers (Tense)

| Modifier | Semantic Function | Gₘ Element | Erlangen Level | Geometric Realization | Justification |
|---|---|---|---|---|---|
| m_past | Locate event before speech time | Translation t₋ = (−d, 0) | Euc(2) | Displace relation leftward along temporal axis | Time-as-line metaphor; past = behind/left in most writing systems |
| m_present | Locate event at speech time | Identity I | Euc(2) | No displacement (zero translation) | Present = no temporal displacement; default |
| m_future | Locate event after speech time | Translation t₊ = (+d, 0) | Euc(2) | Displace relation rightward along temporal axis | Mirror of past |

**Convention:** A horizontal axis within the sentence frame is designated the **temporal axis**. Leftward displacement relative to the frame center = past; rightward = future. The magnitude d encodes temporal distance (near past vs. remote past), though exact scaling is domain-specific.

**Visual realization:** The relation connector is drawn shifted left (past) or right (future) from center within its frame.

#### B. Aspectual Modifiers

| Modifier | Semantic Function | Gₘ Element | Erlangen Level | Geometric Realization | Justification |
|---|---|---|---|---|---|
| m_progressive | Mark event as ongoing/unfinished | Open curve (no closure) | Sim(2) | Relation path drawn as open arc (not reaching target boundary) | Progressive = process in motion, not yet bounded |
| m_perfective | Mark event as completed/bounded | Closed enclosure of relation | Sim(2) | Relation path enclosed in a sub-frame within the assertion | Perfective = completed action = bounded region |
| m_habitual | Mark event as repeated pattern | Periodic repetition of path | Euc(2) | Relation path drawn as repeating wave/series of arcs | Habitual = iterated translation along time axis |
| m_inchoative | Mark beginning of event | Truncation to initial segment | Sim(2) | Only the start of the relation path is drawn (fade/taper at end) | Inchoative = the onset; only the beginning of the curve |

**Visual realization:** Aspectual modifiers transform the **shape** of the relation connector.

#### C. Degree/Scalar Modifiers

| Modifier | Semantic Function | Gₘ Element | Erlangen Level | Geometric Realization | Justification |
|---|---|---|---|---|---|
| m_very / m_intense | Increase degree | Scaling σ_Λ with Λ > 1 | Sim(2) | Enlarge the modified element | "Very big" = bigger than big; scaling up |
| m_slightly / m_diminish | Decrease degree | Scaling σ_λ with 0 < λ < 1 | Sim(2) | Shrink the modified element | "Slightly warm" = less than warm; scaling down |
| m_superlative | Maximum degree in frame | Scaling to fill frame boundary | Sim(2) | Entity/relation expanded to touch frame edges | "Biggest" = fills available scope completely |
| m_comparative | Greater than reference | Scaling relative to another element | Sim(2) | Modified element drawn larger than comparison element | "Bigger than X" = X and element in same frame, element visibly larger |

#### D. Manner Modifiers (on Relations)

| Modifier | Semantic Function | Gₘ Element | Erlangen Level | Geometric Realization | Justification |
|---|---|---|---|---|---|
| m_quickly | High rate/speed | High curvature κ on connector | Sim(2) | Relation path drawn with steep curve (compressed spatial extent) | Quick action = shorter path in more time → higher curvature |
| m_slowly | Low rate/speed | Low curvature κ on connector | Sim(2) | Relation path drawn as gentle, extended arc | Slow action = longer path in same time → lower curvature |
| m_forcefully | High intensity | Bold/thick path | Sim(2) | Connector drawn with thicker line weight | Force = visual weight/emphasis |
| m_gently | Low intensity | Thin/light path | Sim(2) | Connector drawn with thin, light line | Gentleness = visual lightness |

#### E. Quality/Property Modifiers (on Entities)

| Modifier | Semantic Function | Gₘ Element | Erlangen Level | Geometric Realization | Justification |
|---|---|---|---|---|---|
| m_color | Assign chromatic quality | Hue transformation H_θ | Aff(2) | Entity boundary/fill receives a hue variant | Color = quality that doesn't change structure; affine-level (preserves shape, changes surface) |
| m_material ("wooden") | Impose substance quality | abstract(e_material) → m | Sim(2) | Entity drawn with texture/pattern from source entity | Material quality = transferred boundary pattern (via abstract) |
| m_size ("big", "small") | Scale the entity | Scaling σ | Sim(2) | Entity drawn larger/smaller | See degree modifiers above |

---

## Formalization Requirements

For each modifier assignment:

1. **Closure:** m ∈ Gₘ (is the named transformation actually in the carrier set?) → YES for all above, since Gₘ includes all Euclidean isometries, similarity transformations, and higher
2. **Invertibility:** Each modifier has an inverse (required for Gₘ membership) → YES: t₋¹ reverses translation, σ₋₁ reverses scaling, etc.
3. **Composition compatibility:** Stacking modifiers (e.g., "past progressive") = composing transformations (t₋ ∘ m_progressive) → Must yield a valid element of Gₘ → YES: composition of invertible transformations is invertible

---

## Impact Assessment on D2 Cases

| Case | Current Verdict | After Phase 1A | Reason |
|---|---|---|---|
| 5.1 "She was running when it started to rain" | ⚠️ | ✅ | m_past, m_progressive, m_inchoative now formally defined |
| 5.2 "I have been studying for three hours" | ⚠️ | ⚠️ | Tense/aspect now defined, but "three hours" still requires cardinality |
| 5.3 "He will have finished by tomorrow" | ⚠️ | ✅ | m_future, m_perfective now formally defined |
| 5.5 "She used to like coffee" | ⚠️ | ✅ | m_habitual now formally defined (periodic transformation) |
| 1.2 "All birds fly" | ✅ | ✅ | Already clean (no change) |
| 10.1 Polysynthetic (Mohawk) | ⚠️ | ⚠️ | Semantic decomposition works; morphological complexity unchanged |
| 10.4 Classifier (Mandarin) | ⚠️ | ⚠️ | Classifier via abstract works; numeral still needs cardinality |

**Net conversions:** 3 cases ⚠️→✅ directly from tense/aspect.

**Additional conversions from convention clarity:** Cases involving manner ("quickly ran"), degree ("very big"), and quality modifiers across Categories 1, 7, 10 become unambiguous. Estimated 3–4 more cases improve from "expressible but how?" to "expressible with specified convention."

**Revised estimate:** ~5–7 cases total improve, with 3 definitively ⚠️→✅.

---

## Files to Modify

| File | Change |
|---|---|
| `foundations/formal-operations.md` | New §1.12 "Canonical Modifier Assignments" — the full table with geometric justifications |
| `foundations/formal-foundations.md` | Brief note in §1.5 or §2.1 referencing the convention table |
| `ul-core/grammar/formal-grammar.md` | Update modifier construction rules with examples using named modifiers |
| `ul-core/writing-system/writers-companion.md` | Add worked examples showing tense/aspect visual conventions |
| `experiments/D2-completeness-challenge.md` | Re-score Cases 5.1, 5.3, 5.5 as ✅; update summary statistics |
| `ul-core/CRITIQUE.md` | Add note in §4 that convention gap F4 is now resolved |

---

## Open Questions

1. **Temporal axis orientation:** Is left=past universally appropriate? Arabic and Hebrew write right-to-left. The convention might need to be "source-direction = past, target-direction = future" to be writing-system-neutral.
2. **Magnitude calibration:** How much displacement d = "past" vs. "remote past"? Propose: d is relative to frame size (1/4 frame width = near, 1/2 = remote), but this is a convention, not an algebraic constraint.
3. **Color as modifier:** Hue transformation lives at Aff(2) level. Is this the right Erlangen assignment, or should color be Sim(2) (preserving ratios) or Proj(2)?

None of these block implementation. They can be resolved during Phase 1A execution or deferred to future convention refinement.
