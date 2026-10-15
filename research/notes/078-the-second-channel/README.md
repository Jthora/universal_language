# 078 — The second channel: the placement plane was already in the register, and `derived_orbits` moved

**Type:** cycle
**Opened:** 2026-10-15
**Status:** closed
**Thread:** the owner, after the estate crisis cleared — *"lets get back into UL"*. The committed
order was estate, then the placement plane.
**Instruments:** `tools/check-placement.rb` (new), `ul-forge/crates/ul-core/src/placement.rs` (new),
`076`'s specification, `077`'s design input, `F-040`'s rule.

---

## 0. The rule that decided the first move

`F-040` was written two days ago and its lesson was one sentence: *check an import against itself
before building anything to fill a gap it reports.* `076` then reported a gap and specified what
would fill it. So the first move was not to build.

**The placement plane is already in the register, fully parameterised.** Every one of the **2413**
cells under `slices/` carries its complete configuration:

```
  A, B                                which mark on each side
  separation, bearing                 where B sits relative to A        (r, theta)
  relative-rotation, relative-scale   how B is turned and sized
  reflect-B                           AS DRAWN | MIRRORED   <- a two-valued axis
  B-frequency, B-amplitude, B-phase   wave parameters
```

together with **which attested operators that configuration realises** and whether the two marks
are **in contact**. The operators are regions of this space, which is what the `placement-dial`
sheet says in prose and what `071` had no way to check.

`reflect-B` is not a footnote: it is a first-class axis, systematically crossed with every other
axis (`slice-b-separation-reflect-B`, `slice-b-bearing-reflect-B`, `slice-b-relative-rotation-reflect-B`,
and so on). **The handedness bit is part of their coordinate system.**

And the slice's own note is a **third independent witness** to `ALPHABET-HAS-ONE-CHIRAL-LETTER`:

> `"operand_chosen_for": {"reflect-B": "is chiral"}`
> `"note": "operand chosen so the axis acts: reflect-B needs one that is chiral"`

with `B` held at **Wave**. They could not draw the axis without the chiral letter. That is a design
decision forced by the structure, which is stronger evidence than a column.

## 1. What the plane turns out to carry — and where my first reading was wrong

**Mirroring an operand leaves the attested vocabulary exactly when that operand has two or more
mirror axes.** 16 informative pairs, **16 agreements, 0 disagreements**.

| reflected operand | mirror axes | pairs | attested operator |
|---|---|---|---|
| Wave | 0 | 11 | **destroyed** |
| Angle, Curve | 1 each | 2 | **destroyed** |
| Point, Circle, Line | ∞, ∞, 2 | 3 | **preserved** |

The mechanism is immediate: an O(2) mark is unmoved by a mirror and the Line maps to itself, so the
configuration is *literally unchanged*; Wave and Angle/Curve genuinely change.

**My first reading was "reflection is one-directional — it only ever leaves the vocabulary," and
my own checker refuted it on its first run.** I had analysed this in a scratch script, counted 13
changes and 41 non-changes, and concluded the 41 were all `none → none` and therefore vacuous.
Three of them were not: they are exactly the high-symmetry operands, and they **preserve** the
operator. So the true predicate is sharper than the one I nearly gated on, and the gate now asserts
it with a control that fails if the threshold is moved off two axes. **Third time in two sessions
that the instrument corrected me rather than the reverse.**

**The denominator is 16, not 54.** The other 38 pairs are `none → none` and carry no information.
Of the 16, thirteen sit on the destroying side and **only three on the surviving side**, one per
mark. Exceptionless, thinly sampled, and the checker prints the thinness.

## 2. A geometric predicate confines two operators

Over all 2413 cells, with **zero exceptions**:

- `concatenate` occurs **only** where the marks are in contact
- `superpose_displaced` occurs **only** where they are not

And both converses fail heavily — 788 cells have contact without `concatenate`, 1546 lack contact
without `superpose_displaced` — which is what rules out reading these as definitions rather than
confinements.

On the radius axis, `concatenate` runs to **0.995** and stops; `superpose_displaced` starts at
**1.1**. The extent of operand A is ~1.0, and the register samples 0.995 and 1.1 rather than round
numbers, which **brackets the tangency radius**. So the boundary between those two operators lies
in `(0.995, 1.1]` — where two marks of unit extent stop touching.

**Bracketed, not located.** Seven sampled radii cannot resolve a threshold more finely than the gap
between two of them, so "the boundary is at r = 1" is not claimed. That it was *sampled* at 0.995
and 1.1 is evidence of intent, not of the threshold.

This is the **first boundary in the attested operator inventory tied to a geometric quantity rather
than to a sheet.** It does not close `OPERATOR-INVENTORY-IS-UNFORCED` — one confinement among five
operators is not a derivation of the set — but it is the first reduction of that claim's surface
since it was opened.

## 3. A priority-0 claim whose name was looser than its text

`OPERATORS-ARE-CLOSED-UNDER-REORIENTATION` (note `067`, priority 0) says every attested operator
reads a quantity unchanged **when the whole canvas is turned**, drift ≤ 1.3e-15. That is rotation.
§1 measures that **reflection of an operand does not preserve the operators**.

There is no contradiction: canvas rotation and operand reflection are different operations. The
hazard is that "reorientation" colloquially covers reflection, so a reader citing the *name* would
overreach on a priority-0 claim. **Narrowed in place**, and its dependents re-reviewed:

- `OPERATOR-INVENTORY-IS-UNFORCED` — the net moves **both ways**. The closure result is narrower
  than its name suggested, so the surface `067` appeared to reduce is **smaller than it looked**
  and this claim is *more* open, not less. Against that, §2 is a genuinely new reduction. Stays
  open, stays priority 0.
- `067` itself, against §1's thin surviving side — the narrowing needs only the **well-sampled
  half** (reflection sometimes destroys an operator, 13 pairs, no exceptions), so it would stand
  even if the three-pair side were overturned.

## 4. The channel, built

`ul-forge/crates/ul-core/src/placement.rs`. A corpus entry may now carry a `placement` block, and
the harness **computes** handedness from it rather than reading an authored value.

The two channels are **provably disjoint**, and the proof was already in the harness: the existing
`every_corpus_entry_is_reading_invariant` test asserts that `map.mirror()` leaves every essential
invariant unchanged. That is the formal statement of why a map cannot carry handedness, and it is
why this channel is necessary rather than redundant.

Guards, all of which must fail loudly rather than drift:

1. The six-mark symmetry table is a const, and `tests/placement_tests.rs` asserts it against the
   register's `enantiomorphs` sheet **in both directions**, including against that sheet's
   *independent measurement* (mirror distance against threshold) and not only its flag.
2. The handedness function is asserted against the register's **180-cell attested column, cell by
   cell**, allowing exactly the **four** disputed cells `076` located and **failing on a fifth**.
   That is the difference between a known defect and silent drift.
3. A `placement` block with no `expected.chirality_source` is **refused** — an unasserted channel
   verifies nothing, and would look like the hole was filled.
4. `reflect_b` must not change the handedness *source*: it selects which enantiomorph the figure
   is, never whether the figure has enantiomorphs.
5. **The tier cap.** The `arrangement` branch rests on `ARRANGEMENT-RULE-IS-FITTED`, which is
   ARGUED. The harness **refuses** a VERIFIED tier for any entry reaching it —
   `UNDETERMINED-PREMISE-PROPAGATES`, enforced by assertion rather than by review. Verified by
   negative control: a shipped entry was set to VERIFIED and the assertion fired.

## 5. Two entries, deliberately one tier apart

**`017-inherited-handedness`, orbit 92, VERIFIED.** `cross(Wave,Wave)`. Its nearest named
neighbour is orbit 91, `cross(Line,Line)` — the `+` — and the two agree on **every** map invariant
the harness computes: 5 vertices, 4 edges, 1 face, genus 0, 1 component, degree sequence
`[1,1,1,1,4]`. They are separated by chirality alone. VERIFIED because the handedness is
**inherited** from the one chiral letter: one measured bit of the alphabet, no free parameter.

**`018-arrangement-handedness`, orbit 40, ARGUED.** `superpose_displaced(Angle,Curve)`. Both marks
encoded at **subdivision-minimal** form, because `016` established subdivision is inert: the
Angle's corner is a degree-2 vertex that smooths away, so an Angle and a Curve are the *same map*,
a single open arc. **That is why this entry's map is identical to orbit 39's** — the named
neighbour it must be distinguished from — and the distinction lives entirely in the new channel.
ARGUED because its handedness comes from arrangement, and the harness would not let it be anything
else.

The pair is the honest shape of the channel: **one side derived, one side fitted**, and the tier
gap enforced by the machine.

## 6. The number moved, under a stricter definition

**`derived_orbits`: 2 → 3**, with `derived_orbits_argued` = 1 reported separately and never summed.

The metric was **tightened first**. `071` counted every entry's `orbit` regardless of tier; it now
counts **VERIFIED entries only**. That change was made *before* anything was added to it, so the
move is harder rather than easier. Adjusting a normalisation until the number comes out right is
what `075` REQ-3 forbids; moving it the other way first is the test of whether that rule is real
here, and this is the first chance there has been to take it.

Had the metric been left alone it would read **4**. It reads 3.

399 tests pass (was 391), clippy clean under `-D warnings`, `check.rb` green but for the eight
pre-existing pending scope reviews.

## 7. What `077` contributed, and what it got right

`077` read one practitioner tradition's notation and extracted three design inputs. All three held:

| `077`'s input | how it landed |
|---|---|
| make it **a bit**, not a gradation (O(2) is disconnected) | `reflect_b` is a bool; `MirrorAxes` is an enum with an `Infinite` case because a map cannot hold O(2) at all |
| make it **mandatory** | a `placement` block without an asserted source is refused; and §1 shows flipping the bit can take a composition out of the language, so it is well-formedness-bearing on measured grounds and not only by analogy |
| rank it **above** timbre-like detail | the channel is disjoint from the map by construction, so no map invariant can override it |

The one that earned its keep was the second, because §1 reached the same conclusion from the
register and the two arguments are independent.

## 8. What is still open

1. **85 of 180 cells** need the channel and only 2 orbits now use it. The remaining work is
   per-orbit, and 85 of 93 orbits still have no named neighbour to derive against — `071`'s
   recommendation #2, untouched.
2. **The arrangement half is still fitted.** `077` named the external test that would settle it;
   nothing in this note tests it. Until then, every arrangement-sourced entry is capped at ARGUED
   by the harness, which is the correct outcome and not a workaround.
3. **The tangency threshold is bracketed, not located.** Refining it needs radii the register does
   not sample, which means *generating* configurations rather than reading them — a different and
   larger piece of work, and the first one here that would require this repo to draw.
