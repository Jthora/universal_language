# What Is Actually Provable

**Date:** 2026-08-01
**Challenge:** *"Why do you keep suggesting experiments matter here? If our primitives are
mathematical objects with geometric definitions, isn't that something we can simply prove
mathematically?"*
**Answer: largely yes, and I had been reaching for the wrong instrument.** The geometric layer is
provable — including three things I had filed as conjectures or experiments. This document states
what can be proven, what the proofs give us, and the one place proof genuinely does not reach.

---

## 0. The error being corrected

I labelled the Euclid/Aristotle dependency-order check an **experiment**. It is not. Formalizing two
partial orders and testing for isomorphism is a **proof**. That was sloppy vocabulary covering a
real methodological mistake: **defaulting to empirical validation as the only route to rigor, when
the objects under discussion are mathematical.**

Worse, it was self-defeating. The project's failure mode has always been *asserting* structure and
then patching evidence to fit (the D2 history). I was treating experiment as the antidote. But for
mathematical objects **proof is the stronger antidote** — it cannot be patched into agreement.

Three things I had wrongly filed as open or empirical turn out to be theorems.

---

## 1. The primitives are a stratification of curvature space — and this is a classification theorem

### The theorem

**Fundamental Theorem of Plane Curves:** given a smooth curvature function κ(s) and initial
conditions, *"there exists a **unique** arc-length parametrized curve with that curvature
function,"* and *"two unit-speed plane curves which have the same curvature differ only by a
**Euclidean motion**."*

**A plane curve simply *is* its curvature function, up to rigid motion.** Nothing else is left over.

### The primitives fall out as strata of κ

| Primitive | Condition on κ(s) |
|---|---|
| **Line** | κ = 0 |
| **Circle** | κ = constant ≠ 0 |
| **Wave** | κ periodic |
| **Curve** (general) | κ variable |
| **Angle** | κ = 0 a.e. with isolated singularities (corners) |
| **Enclosure** | closed: total turning ∮κ ds = 2πn |

**This is not a design choice. It is a partition of the space of curvature functions.**

### It *derives* the collapse relations you described

You said the sets collapse in specific ways, and that they're alternative decompositions rather than
disagreements. **Both fall out of the theorem:**

- *"Curve collapses into wave"* — **periodic κ is a sub-stratum of variable κ.** A partition that
  doesn't separate periodicity has no wave/curve distinction to make.
- *"Angle collapses into line"* — **an angle is piecewise κ = 0 with singularities.** A partition
  that doesn't resolve isolated singular points cannot separate them.

**So the different primitive sets are different partitions of one space, and the collapses are
exactly which distinctions a coarser partition fails to make.** That's the generating-set claim from
the previous document — now with the underlying object identified and the relations *derived*
rather than observed.

### What this proves, stated honestly

It does **not** prove the hand-chosen set was forced. The primitives were selected by hand and I am
reconstructing them.

**What it does prove is stronger than it sounds:** the question *"which primitives?"* is no longer
aesthetic. It becomes **"which partition of curvature-function space?"** — a well-posed mathematical
question with an enumerable answer space and derivable relations between answers. **A malformed
question became a well-posed one.** That is what a classification theorem buys.

---

## 2. Extension is *derivable*, which answers "how does this help us grow UL"

This is the part that matters most for your second question. **You do not brainstorm the next level.
You read it off the classification.**

**Fundamental Theorem of Space Curves:** *"a unique space curve (up to rigid motion) exists for given
positive curvature and arbitrary torsion as functions of arc length"* — and *"if a curve has torsion
τ(s) = 0 for all s, it is just a plane curve."*

**So the entire plane-curve symbology is exactly the τ = 0 slice of a larger system.** Moving to 3D
adds **one derived primitive axis: torsion.** Not invented — entailed.

The ladder, with each level's inventory derived rather than chosen:

| Level | Complete invariants | Derived primitive inventory |
|---|---|---|
| **0D** | — | Point |
| **1D in plane** | κ(s) | line, circle, wave, curve, angle, enclosure |
| **1D in space** | κ(s), τ(s) | **+ helix (κ, τ both const), knotting, chirality** |
| **2D surfaces** | κ₁, κ₂ (principal) | **plane (K=0,H=0), sphere (K>0), cylinder (K=0,H≠0), saddle (K<0), minimal (H=0)** |
| **nD** | curvature tensor | open, but the pattern is fixed |

**Two immediate consequences worth noticing:**

- **Chirality appears at 3D and cannot exist at 2D.** Torsion is signed; left- and right-handed
  helices are not related by rigid motion in ℝ³. So a 3D symbology gets a *handedness* distinction
  that is provably unavailable in the plane. If the semantics ever needs an asymmetry the plane
  can't express, this is where it lives.
- **The Theorema Egregium** makes Gaussian curvature K **intrinsic** — measurable from inside the
  surface, without reference to any embedding. For a symbology this is the difference between marks
  that require an external viewpoint and marks readable from within the system. That distinction is
  a theorem, not a stipulation.

**This is a growth mechanism that expands by derivation.** Every previous expansion of this project
was invention followed by justification. This one runs the other way.

---

## 3. The coarse-graining flow is provable — and it ranks the primitives

I had "primitive count is an irrelevant operator" filed as a **conjecture** requiring experiment.
**It is derivable from Erlangen, which is already in this repo.**

> *"Since the group of affine geometry is a subgroup of the group of projective geometry, any notion
> invariant in projective geometry is **a priori meaningful** in affine geometry; but **not the other
> way round**."*

That is a theorem about which distinctions survive which level. Applied to our primitives:

| Level | Group | Preserved | **Primitives surviving** |
|---|---|---|---|
| **Euclidean** | rigid motions | lengths, angles, areas | all distinct |
| **Affine** | + skew/stretch | parallelism, midpoints | **circle ≡ ellipse**; angle-as-magnitude dies |
| **Projective** | + perspective | incidence, cross-ratio only | **all conics merge**; curvature distinctions gone |
| **Topological** | homeomorphism | connectivity, separation | **Point and Enclosure only** |

**The primitives are provably ranked by robustness, and there is a clear survivor.**

**Point and Enclosure are the only unconditional survivors at the topological level.**
The full case-by-case derivation is in `../notes/014-erlangen-survival-derivation/`, which
corrects four points in the version originally asserted here — notably that **Line survives
projective** (collinearity is *the* projective invariant) and dies only at topological, and that
**Wave dies at affine**, one level earlier than claimed.

### The survivor is Layer 0, and this is not an analogy

**Jordan Curve Theorem:** a simple closed curve separates the plane into inside and outside.
**Enclosure is the topologically-surviving primitive, and what it does is separate.**

Spencer-Brown's *Laws of Form* opens by drawing a distinction — **a boundary, marked from unmarked.**

**These are the same object.** The primitive that survives maximal coarse-graining *is* the
pre-semantic distinction the framework puts at Layer 0. That was previously a philosophical
posit sitting next to a geometric inventory with no formal link between them. **The Erlangen
computation supplies the link, and it is a derivation.**

If there is a semantic fixed point, this says what sits at it: **not five or six primitives —
distinction and incidence.**

---

## 4. Cox's Theorem is the proof template this project has always needed

Your instinct that this should be *proven* has a precise model in the literature, and it is the shape
the retired Unique Grounding Theorem was reaching for and missed.

**Cox's theorem** *"proposes a handful of intuitively-appealing, qualitative requirements for any
system of plausible reasoning, and shows that these requirements imply that any such system is
**just probability theory in disguise**."* The conclusion is a **representation theorem**: *"there is
an order isomorphism between plausibilities and the unit interval [0,1]."*

Result: *"probability theory is the only consistent extension of Boolean logic to uncertain
situations."* **A uniqueness proof about a semantic-ish object — exactly what was claimed to be
impossible here.**

### The target theorem, stated in this form

> **Any system satisfying [desiderata D₁…Dₙ for a semantic notation] is isomorphic to [structure S].**

That is provable in principle, and it is the correct form of the claim this project has been trying
to make since the beginning.

### Why the old attempt failed, precisely

**The Unique Grounding Theorem defined its semantic primitives to mirror the geometric ones already
chosen, then presented the resulting bijection as proof.** Cox's desiderata are *"intuitively
appealing, qualitative requirements"* — **motivated independently and stated without reference to
probability.** That is the entire difference, and it is a discipline, not a difficulty:

**The desiderata may not mention the target structure.** If they do, the theorem is circular. This
is a checkable condition on any future attempt.

### And a caution worth carrying

Cox's theorem needed repair — Halpern's counterexamples showed the original argument requires
additional density/continuity assumptions, and there is a literature on patching it. **Even the
exemplar of this proof style had gaps found and closed.** That is not a reason to avoid the method;
it is a calibration on what "proven" costs, and a reminder that the first version will have holes.

---

## 5. Where proof does not reach — stated narrowly this time

I have been over-broad about this, so here is the honest boundary.

**Fully provable, no experiment relevant:**
- the classification of primitives and their collapse relations (§1)
- the extension to higher dimensions and its derived inventories (§2)
- which primitives survive which coarse-graining, and the fixed-point identification (§3)
- Enclosure ≅ Jordan separation ≅ Spencer-Brown's mark (§3)
- the dependency-order isomorphism (Euclid vs Aristotle vs Vaiśeṣika) — **this is formal, and I
  mislabelled it**
- any Cox-style characterization theorem we can state and prove (§4)

**Not provable, and no amount of geometry changes this:** *whether actual semantic systems fall in
the class.* Whether human languages, or independently emerged agent protocols, are instances of the
structure is a claim about **contingent objects in the world**. Mathematics can prove what the class
contains and how it behaves; **it cannot prove that anything out there is a member.**

That is a much narrower empirical residue than I have been implying — **one question, at the end,
about membership.** Everything upstream is proof, and I should have been building it that way.

---

## 6. Revised program

**Provable now, in rough dependency order:**
1. **Formalize the κ-stratification** and derive the known primitive sets as partitions, with the
   collapse relations as theorems. Makes "which primitives?" well-posed and closes it.
2. **Compute the Erlangen survival table rigorously.** Cheap, and it yields the robustness ranking
   plus the fixed-point identification in §3.
3. **Prove Enclosure ≅ distinction** via Jordan separation. Links Layer 0 to the geometry formally
   for the first time.
4. **Run the dependency-order isomorphism** (Euclid / Aristotle / Vaiśeṣika). Formal, non-circular,
   decisive either way.
5. **Derive the 3D inventory** from the space-curve theorem — torsion, helix, chirality — and check
   whether the semantics needs anything it supplies.
6. **State candidate desiderata for a semantic notation**, subject to the hard constraint that they
   may not mention the target structure. Then attempt the Cox-style characterization.

**Empirical, and only this:** does any real semantic system instantiate the class (§5).

**Retired from the plan:** treating primitive selection, coarse-graining behaviour, or the Layer 0
link as awaiting experimental resolution. They were awaiting a proof I hadn't looked for.
