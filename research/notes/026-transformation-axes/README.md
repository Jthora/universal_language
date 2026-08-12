# 026 — Two kinds of axis, and the cross table has a name

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `025` (which assumed all axes were the same kind — wrong)
**Prompted by:** the waveform/shape table, the transformation axes, and *"a table for lines that
intersect to form crosses and Xs… results in a fractal of table cells."*

---

## 1. Decoding the example table

```
sine wave  |  square wave  |  triangle wave
curve      |  line         |  angle
Circle     |  Square       |  Triangle
```

**Columns — corner structure:** smooth (no corners) · right-angled · oblique-angled.
**Rows — extent:** periodic · open-once · closed.

Both are recognisable in the `022`/`024` scheme, **but neither is one of my axes.** The column axis
is singularity *crossed with* branch angle. The row axis blends **periodicity** (axis 1) with
**closure** (axis 3) — a *diagonal* through my product, not a coordinate of it.

**That is informative rather than a discrepancy.** The empirical tables use a different
coordinatization than the derived classification. Under `PRIMITIVE-SETS-ARE-GENERATING-SETS` that is
expected — but it is the first time the principle has applied to **axes** rather than to primitives,
and it means neither decomposition is privileged.

## 2. The correction — there are two kinds of axis, and `025` assumed one

`025` treated the table structure as a product of **shape axes** and derived C(n,2) tables from it.
**That was incomplete.** Orientation, mirroring and scale are not shape properties:

| Kind | Examples | What it is |
|---|---|---|
| **Shape axes** | curvature · singularity · closure · junction degree | properties of a configuration |
| **Transformation axes** | orientation · mirroring · scale · translation | **elements of a group acting on configurations** |

**These are categorically different.** A shape axis says *what the mark is*. A transformation axis
says *how the same mark is placed*.

So the full structure is not a product of like things. It is

> **(shape space) × (group)** — a **G-set**, not an n-dimensional grid.

**This explains the hyper-dimensionality directly.** Rotation is continuous, scale is continuous.
Crossing a discrete shape classification with a continuous group produces exactly the unbounded
blow-up described — and no amount of enumeration terminates, because one factor is not finite.

## 3. And it explains the collapse

**The transformation axes are precisely what coarse-graining quotients away.** "Invariant under G"
*means* "constant along the G-directions." So:

| Axis kind | Under coarse-graining |
|---|---|
| Shape axes | partially survive — the `014`/`022` ordering |
| **Transformation axes** | **die entirely — this is what Erlangen *is*** |

**The Erlangen programme is not a separate tool applied to the tables. It is the statement that the
transformation axes are the ones to quotient out.** The tables blow up because the group is crossed
in; they collapse to `024`'s single indexed family because the group is quotiented out.

**One object, two operations: crossing with G explodes it, quotienting by G collapses it.**

## 4. The cross/X table is a known object — and the explosion is quantified

> *"Each rank-3 **oriented matroid** is equivalent to an arrangement of pseudolines, and each
> oriented matroid which is also uniform is equivalent to a **simple pseudoline arrangement**."*

**The cross/X exploration is the classification of line arrangements**, and it is a live research
area with a century of work behind it.

**The growth, exactly** — simple arrangements of *n* pseudolines with a marked cell (OEIS A006247):

| n | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
|---|---|---|---|---|---|---|---|---|---|
| count | 1 | 2 | 3 | 16 | 135 | 3 315 | 158 830 | 14 320 182 | 2 343 203 071 |

By n = 13 it is **366 477 801 792 538**, and **enumeration beyond that is open.**

*"Gets very complex very fast"* is correct and now has a number against it. The exploration was not
inconclusive through lack of rigour — **it ran into an open enumeration problem.**

### 4a. And there is a hard limit that matters for expressiveness

> *"The **Pappus arrangement of 9 lines**… is an arrangement of pseudolines which **cannot be
> straightened** while preserving their face structure — called **nonstretchable**."*

**There are combinatorial configurations that no straight-line drawing realizes.** You can specify a
face structure that is perfectly coherent as a combinatorial map and **cannot be drawn with straight
strokes.**

**This is a genuine expressiveness gap, and it is the first concrete one found:** the combinatorial
map (`024`'s fixed point) is *strictly more expressive* than straightedge construction. It bears
directly on Phase 3 A1 and it arrived from the cross/X table rather than from the constructibility
literature.

## 5. "Limited to pen strokes on paper" is not a limitation

Pen strokes are the **constructive regime** (`016`) — and that is exactly where the shape axes are
alive. Coarse-graining kills them; the drawing surface is where they can still be seen.

**The exploration was conducted in the only regime where those distinctions exist.** Everything above
the constructive level has already quotiented them away.

## 6. What changed

- `claims.yaml`: `TWO-KINDS-OF-AXIS` added; `TABLES-ARE-PROJECTIONS` corrected — its C(n,2) count
  applies to **shape axes only**; `NONSTRETCHABILITY-BOUNDS-DRAWING` added.
- **Open:** whether the row axis of the example table (periodic/open/closed) is a better
  coordinatization than my periodicity-and-closure split. It blends two of my axes, which suggests
  one of the two decompositions is doing unnecessary work.
