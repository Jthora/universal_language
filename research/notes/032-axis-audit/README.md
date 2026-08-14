# 032 — Axis audit: are the four right, and are there more?

**Type:** correction
**Opened:** 2026-08-01
**Status:** closed
**Thread:** audits `022`, `024`; corrects `FIXED-POINT-IS-COMBINATORIAL-MAP`
**Prompted by:** *"are we confident there aren't more? are we confident in the existing ones?"*

**Answer: no, and correctly so. Two of the four axes are underdetermined, one axis is missing, and
the fixed-point claim had a dropped scope condition that a test caught.**

---

## 1. Auditing the four

| Axis | Verdict |
|---|---|
| **Closure** (O / L) | **Solid.** Binary, topological, unambiguous — a curve closes or it does not |
| **Junction degree** (0,1,2,…) | **Solid.** Well-defined, topological, derived in `024` |
| **Curvature** (Z / C / P / V) | **Underdetermined** |
| **Singularity** (S₀ / S₊) | **Underdetermined** |

### Why curvature is underdetermined

**`P ⊂ V`.** Periodic κ is a *special case* of non-constant κ, so the four values are not a partition
of equal status — they are a partition with one arbitrary refinement inside it.

**And κ has unboundedly many properties**: monotonic, bounded, sign-definite, analytic, integrable,
of bounded variation… **Nothing selects `{Z, C, P, V}`.** `022` derived that primitives *are* strata
of κ-space. It never derived **which strata**, and I did not notice the difference.

### Why singularity is underdetermined

`S₊` lumps together every singular case. It could refine by **count** of corners, by **order** of
discontinuity (C¹-not-C², versus not even C⁰), by whether corners are isolated. **Binary is a
choice, not a result.**

---

## 2. The missing axis: connectivity

**A hexagon and two disjoint triangles have the same degree sequence** — `[2,2,2,2,2,2]` — the same
vertex count, and the same edge count. **No existing axis distinguishes them.**

Connectivity is topological, independent of degree, and absent from the classification. **That is a
fifth axis**, and it is now an executable test (`map::tests::degree_sequence_cannot_see_connectivity`).

---

## 3. The error the test caught — and it is worse than a missing axis

Writing that test produced a failure: two triangles trace **four** faces, not three.

**The code was right.** In the plane, two disjoint triangles bound three regions — two insides and
one shared outside. **The combinatorial map traces four**, because it treats each component as
embedded on its own sphere and never identifies the two "outside" faces. `χ = 2c`, not `2`.

Then checking the theorem statement:

> *"Every embedding of a **connected** graph on an orientable surface is uniquely determined up to
> equivalence by its rotation system."*

**`FIXED-POINT-IS-COMBINATORIAL-MAP` quotes that sentence verbatim, including the word "connected",
and then states the result without the scope.**

### What actually follows

**For disconnected configurations the rotation system does not determine an embedding.** The
*relative nesting* of components — which component sits inside which face of another — is information
the map **does not carry**.

So the fixed point is **not** the complete invariant for configurations in general. It is complete
for connected ones. A notation with more than one stroke is disconnected by default.

**This weakens `FIXED-POINT-IS-COMBINATORIAL-MAP` materially**, and it was found by writing a test
rather than by rereading the claim.

---

## 4. The method problem underneath all of this

**The axes were listed, not derived.** `022` and `024` produced them by inspection — noticing
features and organizing them. That is why two are underdetermined and one was missed: **inspection
has no completeness criterion.**

**The principled method is to derive axes from invariant theory, level by level**, which is what
Erlangen is for:

| Level | Invariants → axes |
|---|---|
| **Topological** | components (**missing**), degree sequence ✓, face structure ✓, genus, orientation class (**missing**) |
| **Differential** | smoothness class, singularity structure — *needs refinement, currently binary* |
| **Projective** | incidence, cross-ratio |
| **Affine** | parallelism, midpoints, area ratios |
| **Metric** | curvature, length, angle — *needs a principled stratification, currently ad hoc* |

**Orientation class is a second likely omission.** Mirror-image configurations are distinguishable
under an orientation-preserving group and identified under one including reflection. `024` noted the
map is determined "up to reflection" and never made that an axis.

---

## 5. Honest status

**Confidence in the four axes: two solid, two underdetermined. Completeness: no — at least one and
probably two axes are missing. The fixed-point claim needed a scope condition it had quoted and
dropped.**

**But the anxiety about proceeding is answerable rather than paralysing**, because the two solid axes
are exactly the ones everything else rests on:

- **Closure** and **junction degree** are the topological ones. They carry `FIXED-POINT-IS-COMBINATORIAL-MAP`
  (now scoped), `REGIONS-ARE-FACE-UNIONS`, `ENTRENCHMENT-IS-SURVIVAL-ORDER`, and the Jordan result.
- **Curvature** and **singularity** are the metric and differential ones — **and those are precisely
  the axes that die under coarse-graining.** Their underdetermination does not propagate upward,
  because nothing at the fixed point depends on them.

> **The unsound axes are the ones already known to be irrelevant operators. That is not luck — it is
> the same reason they are hard to pin down: metric structure has no canonical stratification, which
> is why coarse-graining discards it.**

## 6. What changed

- `claims.yaml`: `FIXED-POINT-IS-COMBINATORIAL-MAP` **scoped to connected configurations**;
  `KAPPA-STRATIFICATION` marked underdetermined on the curvature and singularity axes;
  `CONNECTIVITY-AXIS-MISSING` added.
- `FAILURES.md`: **F-025** — scope dropped from a theorem quoted verbatim in the same entry.
- `map.rs`: connectivity test added, asserting the true behaviour with the reason recorded.
- **Open:** derive the axis list from invariant theory per level, rather than continuing to extend a
  list built by inspection.
