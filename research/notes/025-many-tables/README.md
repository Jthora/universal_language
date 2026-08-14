# 025 — Why there are many tables, and how many

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `024`
**Prompted by:** *"There are many tables, not just one 2D table. There are multi-dimensional tables
with symbols that show up across multiple tables in a type of hyper-dimensional super matrix of
tables."*

**This is derivation, not literature.** `017` recorded that the project produces reading more readily
than derivation and that drift back toward crawls should be caught; recent notes have been mostly
crawls.

---

## 1. The multiplicity is already implied

The classification established in `022` and `024` is a **product of four axes**:

| Axis | Values |
|---|---|
| **A₁ curvature** | Z · C · P · V |
| **A₂ singularity** | S₀ · S₊ |
| **A₃ closure** | O · L |
| **A₄ junction degree** | 0 · 1 · 2 · 3 · … |

A primitive is a **cell** — a point in A₁ × A₂ × A₃ × A₄. I have been writing this as *one*
classification. **It is a 4-dimensional space, and that is the "super matrix."**

## 2. A table is a projection, and symbols recur because cells cast shadows

**Definition.** A *table* is the projection of the product onto a subset of axes. A **2D table** is a
projection onto two.

**Two consequences follow immediately:**

**(a) The number of 2D tables is C(n, 2).** For n = 4 axes: **six 2D tables** — (A₁,A₂), (A₁,A₃),
(A₁,A₄), (A₂,A₃), (A₂,A₄), (A₃,A₄).

**(b) Every symbol appears in every table.** A cell has a coordinate on each axis, so it projects
into all six — at different positions, adjacent to different neighbours. **That is exactly "symbols
that show up across multiple tables," and it is forced rather than designed.**

**A symbol's identity is its coordinate vector; a table shows two components of it.** Two symbols
that sit adjacent in one table can be far apart in another, because adjacency is per-projection.

## 3. Why the tables are not redundant

If every table shows the same cells, why keep more than one?

**Because projections lose information, and different projections lose different information.** Two
cells differing only on A₃ are *identical* in the (A₁,A₂) table and *distinct* in any table
including A₃. So:

> **No single 2D table separates all symbols. The set of tables does — and only jointly.**

This is a concrete instance of `PRIMITIVE-SETS-ARE-GENERATING-SETS`: each table is a presentation,
none is privileged, and the object is what they jointly determine.

## 4. Many tables below the fixed point, one at it

`024` found that at the coarse-graining fixed point only the junction axis survives, with closure
recoverable from it via face tracing.

**So the count of tables is itself scale-dependent:**

| Level | Live axes | 2D tables |
|---|---|---|
| Euclidean | 4 | **6** |
| Affine | 3 (curvature partly dissolved) | 3 |
| Projective | ~2 | 1 |
| **Topological (fixed point)** | **1** (junction; closure derived) | **0 — a single indexed list** |

**The multiplicity of tables is a feature of the un-coarse-grained regime.** Coarse-graining does not
just delete primitives — it **collapses the table structure itself**, ending in the one indexed
family from `024`. The "hyper-dimensional super matrix" and the "single canonical table" are the same
object seen at two scales.

## 5. A test: counting your tables counts the axes

**This is the useful part, and it runs in the opposite direction to everything so far.**

If tables are pairwise projections, then **n axes produce exactly C(n, 2) two-dimensional tables**:

| Axes | 2D tables |
|---|---|
| 3 | 3 |
| **4** | **6** |
| 5 | 10 |
| 6 | 15 |
| 7 | 21 |

**So counting the distinct 2D tables actually drawn in UWS gives a lower bound on the number of
axes** — including axes that have never been named, only used.

`023` established that the formalism was missing an axis that the empirical practice had; `024`
derived it. **This inverts that: the practice can now be read as data about the formalism.** If the
count is above six, there are axes beyond the four derived, and the surplus says how many.

**Prediction, stated before hearing the number:** the count exceeds six, because the four axes here
are all *geometric*, and a working notation almost certainly encodes at least one non-geometric axis
— orientation, or ordering, or some role marker.

## 6. One coincidence, recorded as a coincidence

C(4,2) = 6, and the six-primitive system carried "6 = C(4,2) via tetrahedral rectification."

**These are different objects.** Six *tables* is not six *primitives*, and matching numbers is not
matching structure. `017` recorded that noticing such a correspondence later and retrofitting it is
this project's documented failure mode.

**So: recorded, not claimed.** It becomes interesting only if there is an independent reason
primitives should correspond to pairwise projections — and there is currently none.

## 7. What changed

- `claims.yaml`: `TABLES-ARE-PROJECTIONS` added, including the axis-counting test and the scale
  dependence.
- **Open:** the actual count of distinct tables in UWS, which is now a measurement that would
  constrain the formalism rather than a matter of taste.
