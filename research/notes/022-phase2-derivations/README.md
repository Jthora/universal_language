# 022 — Phase 2: the κ-stratification derived, and a problem in the Layer 0 identification

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `018` (plan, items C1 and C2); explains `014`
**Superseded by:** `024` · `032`  ← *the one permitted edit to a closed note (`../README.md`)*
**Question:** derive the two claims that have been asserted in prose since `005`.

---

## C1 — the κ-stratification

### 1. The space, stated precisely

By the Fundamental Theorem of Plane Curves, a unit-speed regular C² plane curve is determined up to
rigid motion by its curvature function κ. But **three of the six primitives are not regular C²
curves**, so the space has to be bigger than "curvature functions":

- **Angle** has a corner — not C² there
- **Enclosure** is a *closed* curve — a global condition, not a condition on κ at a point
- **Point** is 0-dimensional — not a curve at all

So take 𝒞 = piecewise-C² curves γ : I → ℝ², regular except at finitely many points, with κ defined
a.e.

### 2. The classification is a product of three axes, not one stratification

**This is the correction the derivation forced.** I had been describing the primitives as strata of
a single curvature-function space. They are not. They are cells in a **product of three independent
classifications**:

| Axis | Values | Kind of condition |
|---|---|---|
| **1 · curvature** | `Z` (κ≡0) · `C` (κ≡c≠0) · `P` (non-constant, periodic) · `V` (non-constant, aperiodic) | **metric**, local, a.e. |
| **2 · singularities** | `S₀` (no corners) · `S₊` (finitely many) | **differential**, local |
| **3 · closure** | `O` (open) · `L` (closed loop) | **topological**, global |

The primitives as cells:

| Primitive | Cell |
|---|---|
| Line | (Z, S₀, O) |
| Circle | (C, S₀, **L**) — a (C, S₀, O) cell is a circular *arc* |
| Wave | (P, S₀, O) |
| Curve | (V, S₀, O) |
| Angle | (Z, **S₊**, O) |
| **Enclosure** | (·, ·, **L**) — **axis 3 alone; curvature unconstrained** |

### 3. The result — why Point and Enclosure survive, derived rather than computed

**Enclosure is defined by axis 3 alone.** It places no condition on curvature whatsoever. And
**Point is not in the space at all** — it is the degenerate 0-dimensional case.

**So the two survivors are exactly the two primitives that are not defined by curvature conditions.**

That explains `014` instead of merely agreeing with it. Coarse-graining up the Erlangen tower
discards **metric** structure first, then **differential** structure, retaining only **topological**
structure. So:

- Axis 1 (metric) dies earliest — Circle at affine, Wave at affine, the Z/C distinction at projective
- Axis 2 (differential) dies at topological — corners are not homeomorphism-invariant
- **Axis 3 (topological) survives to the top** — and Enclosure lives on axis 3

**`014` computed which primitives survive. This says why: survival is determined by which axis a
primitive is defined on, and the axes die in order metric → differential → topological.**

### 4. The collapse relations, as theorems

The two failure modes named in `014` now have precise formal statements:

**Merge = coarsening a partition.** Two cells become one when the partition forgets a distinction.
- **Wave/Curve:** `P` and `V` differ only by periodicity within "κ non-constant". A partition that
  does not resolve periodicity merges them. *This is why "curve collapses into wave" is reversible
  in description — they are siblings under one parent, not one inside the other.*
- **Angle/Line:** (Z, S₊, O) and (Z, S₀, O) differ **only on axis 2**. Forget the singularity set and
  they merge. And topological coarse-graining does exactly that, which is why `014` found Angle
  dying at Homeo.

**Dissolve = the defining condition is not invariant.** `κ ≡ const` is not affine-invariant — an
ellipse has non-constant Euclidean curvature — so Circle does not merely lose a distinction under
Aff(2), its defining condition fails. **Merge is a fact about the partition; dissolve is a fact
about the group.**

### 5. Honest limits

- The known primitive *sets* are selections and coarsenings of this classification, but I have **not**
  derived any particular set (4a, 4b, 5, 6) as canonical, and should not claim to have.
- Periodicity on axis 1 needs the domain long enough to exhibit a period; for short arcs the
  distinction is not defined. Recorded, not resolved.
- **Tier: ARGUED.** Hand derivation, as in `014`. VERIFIED needs a proof assistant.

---

## C2 — Jordan separation, and a problem

### 6. The topological half holds

**Jordan Curve Theorem:** a simple closed curve separates the plane into exactly two connected
components, one bounded and one unbounded, with the curve as their common boundary.

So an enclosure *does* produce a binary distinction with a boundary, and it is exactly the axis-3
condition from §2. That much is solid.

### 7. The identification with Spencer-Brown imports classical logic — which we just ruled out

`ENCLOSURE-IS-DISTINCTION` says the surviving primitive **is** Spencer-Brown's mark. Checking rather
than asserting:

> *"The primary algebra is **essentially isomorphic with classical propositional calculus**."*
> Spencer-Brown *"started off reasoning about 'the Distinction' and ended up with an algebra that
> later writers showed to be **the Boolean algebra of two elements**."*

**Laws of Form's calculus is Boolean.** Its crossing axiom is involutive — cross twice and you return
to the unmarked state — which is precisely the double-negation elimination that `020` retired as
incompatible with the notation's constructive framing.

**So two of our own claims conflict**, and it is the same conflict as `020`:

- `NOTATION-LOGIC-IS-INTUITIONISTIC` — DNE unavailable
- `ENCLOSURE-IS-DISTINCTION` — enclosure is the LoF mark, whose calculus has DNE

### 8. The split that resolves it

**Two separable things were bundled:**

| | Content | Status |
|---|---|---|
| **Jordan separation** | A closed curve yields exactly two components plus a boundary | **Holds.** Topology; commits to no logic |
| **LoF primary algebra** | The Boolean calculus built on the mark | **Not available** — Boolean, conflicts with `020` |

**Layer 0 as *distinction* survives. Layer 0 as *Spencer-Brown's calculus* does not.**

Recorded caveat from the same source: some scholars hold LoF is *"not isomorphic to Boolean
Algebra"* given representational differences, with *"a many-to-one map from Boolean Algebra to
LoF."* That weakens the identification in *both* directions — it is not a clean isomorphism either
way — which is a further reason not to rest Layer 0 on it.

### 9. The pattern worth naming

**Twice in three notes I have reached for a structure that turns out to be classical**, without
noticing: `NEG-INVOLUTION` (`020`) and now the LoF identification. Both are involutive; both import
DNE.

That is not coincidence. **Boolean framings are the default in most of the material this project
draws on**, so importing one is the path of least resistance and it will keep happening. Recorded as
a standing check rather than a one-off: **when adopting an external formalism, ask whether it is
classical before adopting it.**

---

## What changed

- `claims.yaml`: `KAPPA-STRATIFICATION` **derived**, restated as a three-axis product with the
  survival result; `ENCLOSURE-IS-DISTINCTION` **split** — Jordan half retained, LoF-calculus half
  withdrawn.
- `FAILURES.md`: F-024 — the recurring pull toward classical formalisms.
- **Phase 2 closed.**
