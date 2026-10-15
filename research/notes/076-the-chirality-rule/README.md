# 076 — The handedness column is derivable, and 071's blocker was a hole in the data

**Type:** cycle
**Opened:** 2026-10-15
**Status:** closed
**Thread:** 071 ran the derivation loop, found three of four named pairs blocked, attributed two of
them to a missing instrument, and put "build the placement plane" first in its recommendations.
This note tested that diagnosis before building anything. It was wrong, in a way that mattered.
**Instruments:** `tools/check-chirality.rb` (new), `069`'s Aut(map), `070`'s crossing ruling,
`016`'s subdivision-inertness.

---

## 0. What 071 said, and what is actually true

> *"the distinguishing invariant is chirality, and `070` established that placement handedness is
> invisible to the combinatorial map... The map is the wrong instrument here. **The missing one is
> the placement plane.**"* — 071 §2

Three claims are bundled there. Taken one at a time:

| 071's claim | verdict |
|---|---|
| the pairs 39/40 and 91/92 are separated by chirality | **true** |
| the chirality comes from placement | **false for 91/92** — it is inherited from the alphabet |
| the placement plane is the instrument needed to derive them | **false** — see §4 for the real blocker |

Nothing had ever compared two sheets of the imported register against each other. Doing so took
one afternoon and produced a derivation, a located data defect, a refutation of one of my own
predictions, and a correct statement of what is actually blocking the deliverable. No new
instrument was built, because none was needed.

## 1. Handedness at the level of the alphabet: one bit, derived

A plane figure is achiral iff its symmetry group contains a reflection — iff the group is
dihedral or orthogonal rather than cyclic. That is the definition, not a model. The register's
`enantiomorphs` sheet publishes a group per mark:

```
  Point  O(2)   mirror_distance 0.00000    achiral
  Circle O(2)   mirror_distance 0.00828    achiral
  Line   D2     mirror_distance 0.00000    achiral
  Angle  D1     mirror_distance 0.00528    achiral
  Curve  D1     mirror_distance 0.00554    achiral
  Wave   C2     mirror_distance 0.38779    CHIRAL          (threshold 0.11)
```

**Exactly one group is cyclic. Wave is the unique chiral letter of the alphabet** — and it is the
*smooth* C2 one. The Line's half-turn is also realised by a reflection, because D2 contains C2;
reflecting a sine in any line returns a half-period shift and never the sine. The same sheet's
independently measured mirror distances agree, at 0.388 against ≤ 0.008, so the classification is
nowhere near the cut: the threshold is not doing delicate work and could move by a factor of 30
either way without changing a row.

**The six letters are 3 × 2 — rotation class × curvature class — and handedness is a single cell
of that grid.** It is therefore a function of neither coordinate alone. That is the structural
reason the alphabet can supply handedness that a rotation-class-only instrument cannot see.

## 2. From that one bit, half the handedness column follows with no free parameters

The register's `chirality-source` sheet attributes handedness for 180 ordered
(operator, a, b) cells to `achiral`, `inherited`, or `arrangement`.

**INHERITED ⟺ Wave is an operand.** 55 cells, 55 predicted, zero exceptions in either
direction. This is not fitted: it is one bit read off a different sheet.

`check-chirality.rb --selftest` asserts it as a biconditional per cell, plus a **negative control**
— the same test with `Angle` substituted for `Wave` must fail. Without the control the assertion
could be satisfied by an accident of the data, which is the defect `075` found in this repo's own
ℤ/2 test and the one `073` was built to catch.

## 3. The other half is fitted, and says so

The `arrangement` class needs a per-operator rule. Operators fall into three classes by what the
placement leaves free:

| class | operators | arrangement-chiral iff |
|---|---|---|
| concentric | `nest`, `superpose_concentric` | **never** |
| crossing | `cross` | `a ≠ b` and **both** operands are C1 |
| displacing | `concatenate`, `superpose_displaced` | `a ≠ b` and **at least one** is C1 |

Only the first row has an argument rather than a fit: at separation `r = 0` the relative-rotation
parameter is free and the centre is shared, so each mark's mirror axis can be rotated onto the
other's and a common axis always exists when both marks have one. **That is the placement plane's
own statement about its degenerate radius — which is why the plane was never the missing
instrument for these orbits.**

The other two rows were chosen to reproduce this column. **99 of 180 cells are achiral, so a
skewed column is cheap to fit, and agreement on the fitted part is worth much less than agreement
on §2's derived part.** The rule is tiered accordingly. What raises it above curve-fitting is §4.

## 4. The rule disagrees on four cells, and a third sheet settles it in the rule's favour

The rule predicts 30 arrangement cells. The sheet attests 26. The four it adds are exactly

```
  {concatenate, superpose_displaced} × {(Angle,Curve), (Curve,Angle)}
```

the both-modulating case under the displacing operators — consistent with a generator that
enumerated `{C1 mark} × {Point, Circle, Line}` and so silently dropped `{C1} × {C1}`. The `cross`
row of the same sheet *does* include the both-C1 case, which is why the omission reads as a hole
rather than a different rule.

Those four cells are two orbits, **40 and 53**. The per-orbit records under `find/` — a sheet the
rule never saw — publish **`mirror_axes: 0`** for both. No mirror axis means chiral.

**On precisely the cells where the rule contradicts the handedness column, an independent sheet
adjudicates in the rule's favour.** That is a prediction the fit could have failed and did not.
The checker reports the adjudication and **fails** if it ever stops holding, rather than asserting
it once in prose.

Across all 93 orbits, `mirror_axes == 0` and the handedness sheet agree on 84, disagree on 2
(these two), and 7 orbits are generated by unary operators and carry no handedness cell at all.

## 5. A prediction of mine, falsified by the same data

Having found that handedness needs the curvature coordinate, I predicted the combinatorial map
would be **blind** to that coordinate — that swapping a mark for its curvature partner
(`Point↔Circle`, `Line↔Wave`, `Angle↔Curve`) would leave the map-level fields unchanged, with
`016`'s subdivision-inertness as the mechanism.

**False.** Over 360 substitutions, counting only `crossings` and `closed`:

| row swapped | unchanged | changed | blind |
|---|---|---|---|
| Cinf (`Point↔Circle`) | 16 | 104 | 13% |
| C2 (`Line↔Wave`) | 90 | 30 | 75% |
| C1 (`Angle↔Curve`) | 114 | 6 | 95% |

Curvature changes how two marks intersect, and intersections are vertices, so the map sees it.
The blindness is real but **partial and uneven**, not structural. `concatenate(Line,Line)` has a
crossing; `concatenate(Wave,Wave)` has none.

So the honest statement is the narrow one, verified directly: **for the two pairs 071 called
blocked, every geometric field the register publishes coincides and only `mirror_axes` differs.**

I also computed, and then discarded, a figure of "794 orbit distinctions a map-based corpus cannot
express." That number is an artefact of signing orbits on three fields, which makes the
equivalence classes enormous. It is the same defect as the malformed VERIFIED denominator this
project fixed earlier: a ratio whose denominator was built by the convenience of the measurement.
Recording it here because it was nearly shipped.

## 6. The real blocker, which is not an instrument

**`derived_orbits` does not move.** Not for 071's reason, and this is the part that matters:

A corpus entry encodes a combinatorial map and the harness recomputes its invariants. Chirality is
not among them — already registered as `PLACEMENT-HANDEDNESS-IS-INVISIBLE-TO-THE-MAP`, VERIFIED.
So the derivation in §1–§4 is sound and **still cannot be expressed in the channel that counts.**

071 said the blocker was a missing instrument. The instrument was already in the register, twice
over. **The blocker is the corpus schema**: it has exactly one channel, and that channel is a map.
Moving `derived_orbits` on a chirality-separated pair requires the corpus to gain a second channel
that something other than the map computes — and the measured split in §2–§3 says how much that
channel would have to carry: **85 of 180 cells get handedness from outside the map**, 55 from the
alphabet and 30 from arrangement.

That is a specification, and it is the first one this project has had for the placement plane that
states what the plane is *for* rather than that it is next.

## 7. UNRESOLVED: what `components` means

Recorded as neither passed nor failed, per `075` REQ-3.

The orbit records publish a `components` field. Two natural readings are both contradicted by the
register's own numbers:

- **a count of the composed marks** — 16 of 86 binary orbits publish something other than 2, e.g.
  `superpose_concentric(Point,Point)` → 1 (two coincident points, reasonably one piece);
- **a count of connected pieces** — 35 orbits with a crossing, hence joined into one piece,
  publish something other than 1, e.g. `superpose_concentric(Point,Line)` → 2.

A third reading — distinguishable strokes after merging coincident ones — survives both of those
and then fails on `superpose_concentric(Point,Circle)` → 1, where a dot at a circle's centre is two
plainly distinguishable strokes.

**I could not establish the semantics.** That is a statement about me, not a defect in the Codex;
the field may well mean a fourth thing measured at a specific raster scale. The operational
consequence is firm either way and the checker prints it: **`components` must not be imported as a
topological invariant**, and nothing in this repo may key an identity on it.

## 8. What this says about direction

1. **Do not build the placement plane as an instrument.** Build it as a **corpus channel**, with
   §6's number as its specification. The difference is that the channel has to be recomputable by
   the harness from published data, which an instrument does not.
2. **Cross-check sheets before building tools.** This note's entire content came from comparing
   three sheets of a register that had been in the tree for weeks. The apparatus for checking the
   repo against itself was good; there was none for checking an import against itself. There is
   now one, and it found a defect on its first run.
3. **071's recommendation #2 still stands** — only 4 of 93 orbits have named neighbours, and the
   other 85 need pairing against each other.
