# 014 — The Erlangen survival table, derived

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `012` (which raised the burden on Erlangen↔RG and left Newman open)
**Superseded by:** `024`  ← *the one permitted edit to a closed note (`../README.md`)*
**Question:** which primitives survive which transformation group — worked case by case, not asserted?

**Why this note exists:** `ERLANGEN-SURVIVAL-RANKING`, `ENCLOSURE-IS-DISTINCTION`,
`ANCHOR-IS-DISTINCTION-AND-INCIDENCE` and `FIXED-POINT-TABLE-IS-CANONICAL` all rest on a table
stated in prose and never computed. Note `005` concluded that proof was the route; three notes later
the geometry was still a sketch.

---

## 1. Definitions — fixed before computing

A primitive is a class **P** of plane curves (a stratum of curvature-function space). For a
transformation group **G** acting on the plane, say **P survives under G** iff:

- **(i) Closure.** `g(P) = P` for every `g ∈ G` — the defining property is G-invariant.
- **(ii) Distinguishability.** P is not merged with a different primitive under G.

Two distinct failure modes, which the earlier prose conflated:

| Failure | Meaning |
|---|---|
| **Merge** | (ii) fails. The class survives but is no longer distinct from another |
| **Dissolve** | (i) fails. The defining property is not G-invariant at all |

A third, partial case matters here: **the class survives while a *parameter* of it does not** — the
distinction persists but a quantity labelling its members is lost.

## 2. The groups

`E(2) ⊂ Sim(2) ⊂ Aff(2) ⊂ PGL(3) ⊂ Homeo` — each larger group preserves strictly less.

| Group | Preserves | Loses |
|---|---|---|
| **E(2)** rigid motions | distance, angle, area, arc length, κ | — |
| **Sim(2)** + uniform scaling | angle, distance *ratios* | absolute scale |
| **Aff(2)** invertible linear + translation | parallelism, midpoints, area ratios | angle, distance, κ |
| **PGL(3)** projective | incidence, collinearity, cross-ratio | parallelism |
| **Homeo** | connectedness, compactness, **separation** | collinearity, smoothness |

## 3. Derivation

### 3.1 Euclidean E(2)

By the Fundamental Theorem of Plane Curves, κ(s) is a complete invariant under rigid motion. Every
stratum is defined by a condition on κ, so every condition is preserved.

**All primitives survive, all distinct.** Parameters (radius `1/κ`, angle magnitude, period, turning
number) are all preserved.

### 3.2 Similarity Sim(2)

Under uniform scaling by λ: arc length `s ↦ λs`, curvature `κ ↦ κ/λ`.

- `κ = 0 ↦ 0`. **Line survives.**
- `κ = c ≠ 0 ↦ c/λ ≠ 0`. **Circle survives as a class**, but radius is no longer an invariant —
  *class survives, parameter lost.*
- κ periodic ↦ κ periodic (period rescales). **Wave survives.**
- Singularities are preserved, and **angles are similarity invariants**. **Angle survives with its
  magnitude.**
- Closed ↦ closed, turning number preserved. **Enclosure survives.**

**All survive. Absolute scale is the only casualty.**

### 3.3 Affine Aff(2) — the first real losses

- **Line survives.** Affine maps take lines to lines.
- **Circle merges.** Affine images of circles are ellipses, and every ellipse is the affine image of
  a circle — so under Aff(2), circle and ellipse are one object. Note this is a **dissolve as well as
  a merge**: `κ = const` is not affine-invariant, since an ellipse does not have constant Euclidean
  curvature. The defining property itself fails, not just the distinction.
- **Wave dies.** Periodicity of κ is not affine-invariant, and the failure is concrete rather than
  technical: apply the shear `(x, y) ↦ (x, y + cx)` to `y = sin x` and obtain `y = sin x + cx`, which
  is **not periodic**. Periodicity is genuinely destroyed, not merely re-parametrized.
- **Angle survives partially.** Affine maps are smooth diffeomorphisms, so a corner remains a corner
  — the singularity is preserved. But **angle magnitude is not affine-invariant**, so the class
  persists while its parameter is lost. *Corner survives; angle does not.*
- **Enclosure survives.** Affine maps are homeomorphisms of the plane; closedness and Jordan
  separation are preserved.
- **Point survives** throughout, trivially — affine maps are bijections.

### 3.4 Projective PGL(3)

- **Line survives.** Collinearity is *the* projective invariant — this is the defining property of
  the group.
- **All non-degenerate conics merge into one class.** Circle, ellipse, parabola and hyperbola are
  projectively equivalent. Second merge stage, after the affine one.
- **Corner survives** (projective maps are diffeomorphisms of RP²); angle magnitude long gone.
- **Enclosure survives conditionally, and this is a genuine subtlety the prose version missed.**
  On **RP²** projective maps are homeomorphisms, so Jordan separation is preserved. On the **affine
  plane R²** it can fail: a projective map may send part of a closed curve to infinity — a circle can
  map to a *hyperbola*, which bounds no compact region. **So enclosure is safe projectively only on
  the projective plane; on R² it is conditional.**

### 3.5 Topological Homeo

- **Point survives.** Zero-dimensionality is a topological invariant.
- **Line dies.** A homeomorphism bends a line into any simple arc; "straight" is not topological.
- **Conics die as shapes.** Every simple closed curve is homeomorphic to every other.
- **Corner dies.** Homeomorphisms need not preserve differentiability, so a corner can be smoothed.
- **Enclosure survives.** **Jordan Curve Theorem:** a simple closed curve separates the plane into
  exactly two components. This is a topological invariant and the separation is the survivor.

## 4. The derived table

| Primitive | E(2) | Sim(2) | Aff(2) | PGL(3) | Homeo |
|---|---|---|---|---|---|
| **Point** | ✓ | ✓ | ✓ | ✓ | **✓** |
| **Line** | ✓ | ✓ | ✓ | ✓ | ✗ dies |
| **Circle** | ✓ | ✓ (radius lost) | ✗ merges into conics | ✗ all conics one | ✗ |
| **Wave** | ✓ | ✓ | ✗ **dies** | ✗ | ✗ |
| **Angle** | ✓ | ✓ | ~ corner only | ~ corner only | ✗ dies |
| **Curve** | ✓ | ✓ | ✓ (as non-line) | ✓ | ~ as arc |
| **Enclosure** | ✓ | ✓ | ✓ | ~ RP² only | **✓ Jordan** |

## 5. What the derivation corrected

**The headline holds: Point and Enclosure are the only unconditional survivors at the topological
level.** Four corrections to the asserted version:

1. **"The curvature family dies by projective" was wrong.** Line is `κ = 0` — part of that family —
   and it survives projective, since collinearity *is* the projective invariant. Line dies at
   **topological**, not projective.
2. **Wave dies at affine, not projective** — one level earlier than claimed, with a concrete
   counterexample.
3. **Circle merges in two stages**, not one: into conics at affine, then all conics unify at
   projective.
4. **Angle has a partial survival** the binary table could not express — the corner persists to
   projective while the magnitude is lost at affine.

And one subtlety the prose version had no way to see: **enclosure at the projective level depends on
working in RP² rather than R².** The anchor is unconditional only at the topological level, which
*strengthens* the case for anchoring there rather than anywhere earlier.

**Tier honesty:** this stays **ARGUED**, not VERIFIED. VERIFIED requires a machine-checked artifact
and this is a hand derivation. What changed is not the tier but **whether the tier was warranted** —
it was asserted before and it is derived now.

## 6. The Newman obligation — an answer, and its limit

`012` left the sharpest open question: **why is the group tower not itself arbitrary?** Without an
answer, group-theoretic privileging is stipulation and Newman's triviality applies in full.

**Klein's move:** a geometry *is* a group. You do not select a group to study a pre-existing
geometry — the group constitutes what the geometry is. So "which group?" and "which geometry?" are
one question, not two.

**The stronger point, which falls out of §4:** the tower is not unique — one can insert conformal,
equiaffine or Möbius groups — but **the survivors are determined by the terminal group, not by the
path taken to it.** Any chain ending at Homeo yields the same fixed point, because survival at the
top depends only on invariance under the largest group. **The privileging is by the automorphism
group of the underlying structure, which is not a choice.**

**The limit, stated honestly.** This relocates the question rather than eliminating it: *why is
topology the minimal structure?* The available answer is that below topology there is no notion of
"nearby," hence no notion of a mark being anywhere — which connects to Layer 0 and the Jordan
result, but is an argument about what makes "space" meaningful rather than a theorem.

**So Newman is answered better than by stipulating natural relations, and not yet fully.** Recorded
as such rather than as closed.

## 7. What changed

- `claims.yaml`: `ERLANGEN-SURVIVAL-RANKING` corrected and re-warranted; `NEWMAN-OBJECTION` updated
  with the terminal-group argument and its residual limit; `ANCHOR-IS-DISTINCTION-AND-INCIDENCE`
  gains the RP²/R² condition.
- **Left open:** the minimal-structure question in §6; and machine-checking this derivation, which
  is what a genuine VERIFIED tier would require.
