# 033 — Axis types, and whether we are over-focused on UWS

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** repairs the curvature gap left open by `032`; audits the project's own attention
**Superseded by:** `038`  ← *the one permitted edit to a closed note (`../README.md`)*
**Prompted by:** *"Do these axes have to be binary? can they be float?"* and *"How does any of this
apply to UL specifically? or are we extra-focusing on UWS?"*

---

## 1. They are not binary — and they are not all the same kind of thing

I have been writing the axes as if they were one type. **They are four types.**

| Axis | Actual type |
|---|---|
| **Closure** | **genuinely binary** — a curve closes or it does not; there is no partial closure |
| **Junction degree** | **discrete, unbounded** — ℕ |
| **Curvature** | **continuous, infinite-dimensional** — κ is a *function*, so the axis is a function space |
| **Singularity** | **mixed** — corner *count* in ℕ, corner *positions* continuous |

**Only one of the four is binary.** The `{Z, C, P, V}` presentation of curvature was **a discretization
I imposed on a function space**, and `032` correctly flagged it as underdetermined without
identifying why: **there is no canonical finite partition of a function space.**

## 2. But curvature has canonical *integer* invariants — which repairs `032`

`032` left curvature as "underdetermined." That was too pessimistic. Continuous objects can carry
canonical discrete invariants, and plane curves carry several:

> **Whitney–Graustein theorem:** *"Regular closed curves in the 2-plane are classified, up to regular
> homotopy, by their **rotation number**"* — two regular closed curves are regularly homotopic **iff**
> their rotation numbers are equal.

**That is a complete classification by a single integer.**

| Invariant | Type | Status |
|---|---|---|
| **Rotation / turning number** | ℤ | **Complete** classifier up to regular homotopy |
| **Vertex count** (curvature extrema) | ℕ | Four-vertex theorem: **≥ 4** for a simple closed curve |
| **Inflection count** | ℕ | Standard invariant |

**So the curvature axis should be indexed by integer invariants, not by ad-hoc classes.** `{Z, C, P,
V}` was a guess; rotation number is a theorem. **The axis was never unfixable — I had simply used the
wrong coordinates on it.**

## 3. Why the type matters structurally

**Grids and polytopes need finite discrete axes.** That is exactly what `031` found without naming
the reason:

- **Closure** — finite, binary → sits in a grid cleanly
- **Junction degree** — discrete but **unbounded** → breaks any finite polytope
- **Curvature** — continuous → enters a grid only after discretization, and the discretization must
  be canonical or the grid is arbitrary

**And the deeper pattern:** what survives coarse-graining is **topological**, and topological
invariants are typically **discrete** — counts and classes. Metric structure is **continuous**.

> **The fixed point is discrete. The axes that die are continuous.**

That is a sharper form of `032`'s observation that the underdetermined axes are the ones that do not
matter — and it explains it. *Continuous axes are hard to stratify canonically because they carry no
canonical stratification; that is the same property that makes coarse-graining discard them.*

---

## 4. Are we over-focused on UWS? **Yes, heavily — and here is the criterion that sorts it**

**The counts are stark:** 27 claims mention UWS; **4 mention UL.**

### The problem, stated at its sharpest

**If UL is a universality class, then microscopic details do not determine the class.** That is what
universality *means*. So characterizing UWS's microscopic structure would tell us **nothing** about
UL — and much of the recent work has been microscopic.

### The resolution — and it is a clean cut

**The fixed point is not microscopic.** It is the class-level object, by construction. So:

| Work on… | Level | UL-relevant? |
|---|---|---|
| Curvature strata, singularity refinements | microscopic | **No** — one notation's detail |
| Closure, junction degree, connectivity, the combinatorial map | **class-level** | **Yes** |

**And `032` found the metric and differential axes are exactly the underdetermined ones.**

> **The axes that are hard to pin down are precisely the ones that do not matter for UL. The
> difficulty and the irrelevance have the same cause.**

### Honest accounting of the misdirection

`022` and `032` spent real effort on curvature strata, which under this criterion is work on one
notation's microscopic structure. **That was partly misdirected.**

Not wholly: `022`'s derivation is what revealed that **enclosure lives on a different axis from
curvature**, which is what produced the fixed-point explanation. **It paid off by contrast** — but
the payoff came from the boundary, not from the strata.

### So: is UWS an acceptable pre-focus?

> **Conditionally. Yes for work on the fixed point — that is genuinely class-level and therefore
> UL-relevant. No for work on curvature strata — that is one notation's microscopic detail, and
> universality says it cannot generalize.**

## 5. What UL work would actually look like

The criterion above makes this concrete rather than aspirational:

1. **Does the combinatorial map appear as a fixed point in systems that are not UWS?** This is *the*
   bridge test. If the map is UL's fixed point it must show up in independently emerged structure.
   Coarse-grain something that is not a notation and look for it.
2. **Identify G and H** for the order parameter — `026` supplied the group action, so this is better
   placed than it has ever been.
3. **Coarse-grain independently emerged protocols** and check for shared invariants — the standing
   test from `010`, still unrun.

**All three are about the class. None require UWS to be correct.** That is the test of whether they
are UL work: **a UL result should survive UWS being wrong.**

## 6. What changed

- `claims.yaml`: `KAPPA-STRATIFICATION` repaired — canonical integer invariants replace ad-hoc
  classes; `AXES-HAVE-DIFFERENT-TYPES` added; `UL-WORK-IS-FIXED-POINT-WORK` added with the criterion.
- **Open:** re-derive the curvature axis using rotation number, vertex count and inflection count in
  place of `{Z, C, P, V}`.
