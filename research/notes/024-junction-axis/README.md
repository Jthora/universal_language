# 024 — The junction axis, derived — and the fixed point is larger than computed

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `023` (which identified the gap); corrects the scope of `014` and `022`
**Superseded by:** `032`  ← *the one permitted edit to a closed note (`../README.md`)*
**Question:** derive the fourth axis properly, as `022` did for the first three.

---

## 1. The object, stated precisely

The first three axes classify **single curves**. A notation is **marks meeting marks**, so the object
is a configuration.

Let **K ⊂ ℝ²** be a finite union of arcs — a plane graph, i.e. a 1-complex embedded in the plane.
For p ∈ K define the **local degree**

> **deg(p) = number of connected components of (N(p) ∩ K) \ {p}**, for a sufficiently small
> neighbourhood N(p).

## 2. Degree is topological — immediately

The definition uses only connected components of a punctured neighbourhood. **A homeomorphism
preserves both, so it preserves degree.** No further argument needed.

This confirms the `023` prediction by derivation rather than by expectation.

## 3. The cells

| deg | Cell | Existing primitive |
|---|---|---|
| 0 | isolated point | **Point** |
| 1 | free end / stroke terminus | *(unnamed — a gap)* |
| 2 | pass-through, tangent continuous | interior of a curve |
| 2 | pass-through, **tangent discontinuous** | **Angle** |
| 3 | T / Y junction | *(unnamed)* |
| 4 | X junction | *(unnamed)* |
| n | general junction | *(unnamed)* |

**Angle is refined, not merely relocated.** `023` predicted "Angle is the degree-2 junction." The
derivation sharpens that: **Angle is degree-2 ∧ tangent-discontinuous** — the *intersection* of the
junction axis with the singularity axis, not a cell of either alone.

**And degree 1 has no name in the current inventory.** A stroke's free end is a distinct local type
and the notation has never registered it.

## 4. Sub-structure — what dies when

The junction axis is not homogeneous. Three things live on it and they behave differently:

| Feature | Kind | Dies at |
|---|---|---|
| **degree** | topological | **survives to the fixed point** |
| **cyclic order** of branches around the vertex | topological, orientation-sensitive | **survives, up to reflection** |
| **angles between branches** | metric | **affine** — with angle-magnitude, as in `014` |

`023` predicted the first and third. **Cyclic order was not predicted and is the load-bearing one.**

## 5. The result — the fixed point is larger than `014` computed

`014` asked which **single-curve** primitives survive, and found Point and Enclosure. That
computation is correct *for single curves*. **But the notation is configurations**, and for those the
complete topological invariant is known:

> **Heffter–Edmonds principle:** *"Every embedding of a connected graph on an orientable surface is
> **uniquely determined up to equivalence by its rotation system**"* — the cyclic order of edges
> around each vertex — and *"this cyclic order… determines the embedding up to homeomorphism."*

**So the fixed point of the coarse-graining is the rotation system: the graph together with a cyclic
ordering of branches at each vertex — a combinatorial map.**

That is a named mathematical object and a **complete invariant**, which is a far sharper statement
than "Point and Enclosure survive."

### 5a. And it unifies the two survivors

**Faces come free.** By Jordan–Brouwer the complement of K has connected components — the faces — and
they are **recoverable from the rotation system by face tracing.** Euler's relation V − E + F = 2
ties them together.

**So Enclosure and Junction are not independent survivors. They are aspects of one object.**
Separation is derived from the combinatorial map; incidence *is* the combinatorial map.

### 5b. It derives what was previously asserted

`ANCHOR-IS-DISTINCTION-AND-INCIDENCE` claimed the anchor is *distinction and incidence*. `014`
derived **distinction** (Jordan) and **point**. **Incidence was asserted and never derived.** This
note derives it, and shows the two are one structure rather than two.

## 6. The canonical symbol table at the fixed point

This answers a question that has been open since the symbol-table exchange.

At the fixed point, a local vertex type is determined by its rotation. **For unlabelled branches, all
cyclic orders of n identical things coincide** — so each degree yields **exactly one** local type.

> **The canonical table at the fixed point is indexed by junction degree: 0, 1, 2, 3, 4, …**

**Countable, canonical, and derived rather than chosen.** It is not the 4/5/6-element inventory —
it is an infinite family with one entry per degree, of which the current primitives occupy degrees
0 and 2.

## 7. Honest limits

- Assumes **finitely many** junctions. Reasonable for a notation; not proved necessary.
- The map is determined **up to reflection** unless orientation is fixed, since a reflection reverses
  every rotation.
- **Not shown:** that every notation-relevant feature reduces to the combinatorial map. Curvature
  demonstrably does not — it dies earlier. The claim is about the *fixed point*, not about the
  notation as drawn.
- **Tier: ARGUED.** Hand derivation resting on a cited standard theorem.

## 8. What changed

- `claims.yaml`: `JUNCTION-AXIS-MISSING` → derived and renamed in substance;
  `FIXED-POINT-IS-COMBINATORIAL-MAP` added; `ANCHOR-IS-DISTINCTION-AND-INCIDENCE` — incidence now
  derived; `ERLANGEN-SURVIVAL-RANKING` scoped to single curves.
- **Open:** degree-1 (free end) is an unnamed local type in the current inventory. Registering it is
  cheap and it falls out of the same derivation.
