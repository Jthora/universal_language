# 023 — Does 2D constrain the geometry? And are primitives missing?

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `022`; prerequisite for Phase 3 A1 — an expressiveness bound needs to know what
dimension it is bounding
**Prompted by:** *"It is plausible that there are more fundamental geometric feature primitives…
Since UWS is a 2D only environment, that constraint doesn't apply to higher dimensional geometry…
or does it?"*

---

## 1. The answer splits three ways

### 1a. At the fixed point — **no constraint at all**

**Jordan–Brouwer Separation Theorem:** *"A topological (n−1)-sphere in ℝⁿ separates ℝⁿ into two
connected regions… one bounded and one unbounded, each of which has the original hypersurface as its
boundary."*

Jordan generalizes perfectly. And a point is 0-dimensional in every ℝⁿ.

**So both fixed-point survivors from `014`/`022` are dimension-independent.** Enclosure is
*separation*, which exists in all dimensions; Point is dimensional degeneracy, which exists in all
dimensions. **The anchor is safe in any dimension**, and this is a theorem rather than an assumption.

### 1b. Below the fixed point — **yes, a real constraint on the curvature axis**

Plane curves are classified by κ. Space curves need **κ and τ**, and τ ≡ 0 *characterizes*
planarity. So torsion is not a feature a plane curve can have — not "hard to draw," but **absent by
definition.** Chirality follows: signed torsion makes left- and right-handed helices inequivalent
under rigid motion in ℝ³, and that distinction **cannot exist** in ℝ².

**Axis 1 of the `022` classification is dimension-bound.** Axis 3 is not. Axis 2 sits in between.

### 1c. The distinction that resolves it — **depiction versus encoding**

A 2D *picture* of a 3D object is lossy. A 2D *notation* for an n-dimensional object need not be.

**Coxeter–Dynkin diagrams are the proof case:**

> *"A Coxeter polyhedron is **fully determined** by its Gram matrix, which can be conveniently
> described via a Coxeter diagram."*
>
> *"Every uniform polytope with pure reflective symmetry [is] representable by a Coxeter–Dynkin
> diagram."* Nodes are mirrors; an edge labelled *k* is an angle of π/k.

**A flat graph of labelled nodes completely determines polytopes in arbitrary dimension.** So the 2D
surface is not a bound on expressible geometry — it is a bound on *iconic* geometry.

**And this is exactly the symmetry-grid method.** A Coxeter diagram is a symbol table on a symmetry
grid: it records the reflection group, and the polytope is *generated* from it by Wythoff
construction. **The methodology that started UWS is the known-correct method for encoding
higher-dimensional structure in a flat notation.** That is a real convergence, not a compliment.

**The cost, stated plainly:** a Coxeter diagram does not *look like* its polytope. **Iconicity is
dimension-bound; encoding is not.** UWS wants both, and for higher-dimensional content it cannot
have both. That is a genuine design fork, not a difficulty to engineer around.

*Caveat recorded:* *"The Coxeter diagram is not necessarily unique — multiple different diagrams can
all describe a cube but with different underlying symmetries."* So the encoding is complete but not
canonical, which is `PRIMITIVE-SETS-ARE-GENERATING-SETS` appearing yet again.

---

## 2. Are primitives missing? Yes — and there is a specific candidate

The three axes of `022` — curvature, singularity, closure — are **all properties of a single
curve.** A notation is not a single curve; it is *marks meeting other marks*.

**Junction type is a property of a configuration, and no axis captures it.**

- **Angle**, as currently held, is a *degree-2* junction: two segments meeting, tangent
  discontinuous.
- A **T-junction** is degree 3. An **X-junction** is degree 4. A **Y-junction** is degree 3 with
  different angular structure.

Axis 2 classifies **corners on one curve.** A T-junction is not a corner — it is three curve-ends
meeting at a point. **The classification structurally cannot see it.**

### Why this candidate specifically

**It is grounded in the project's strongest empirical result.** `UWS-PERCEPTUAL-GROUNDING` rests on
Changizi et al.: 100+ writing systems share **contour-configuration** statistics matching
natural-scene junction statistics. **That work is *about* junction types** — and junctions are the
one thing the geometric classification omits.

**So the project's best evidence points at a primitive its own formalism cannot express.**

And it reframes an existing primitive: **Angle is the degree-2 case of a more general junction
primitive.** If junction degree is the real axis, "angle" is not fundamental — it is one cell.

**Status: identified, not derived.** A fourth axis needs the same treatment `022` gave the first
three — definitions, cells, behaviour under coarse-graining. Two things are already predictable and
should be checked rather than assumed: junction *degree* is topological and should survive to the
fixed point, while junction *angles* are metric and should die early, exactly as angle-magnitude did.

---

## 3. What this does to Phase 3

The expressiveness bound (A1) must now be stated **per axis and per mode**, not globally:

| | 2D iconic | 2D encoding |
|---|---|---|
| Closure / separation | ✅ all dimensions | ✅ |
| Junction topology | ✅ | ✅ |
| Curvature | ✅ plane only | ✅ any dimension |
| Torsion / chirality | ❌ **impossible** | ✅ |

**"What can UWS express?" has no single answer.** It has one answer for what can be *drawn
iconically* and another for what can be *encoded*, and the gap between them is exactly the
higher-dimensional content.

---

## 4. What changed

- `claims.yaml`: `DIMENSION-BOUNDS-ICONICITY-NOT-ENCODING` added; `JUNCTION-AXIS-MISSING` added;
  `ANCHOR-IS-DISTINCTION-AND-INCIDENCE` gains Jordan–Brouwer, so the anchor is now dimension-general.
- **Open, and it is real work:** derive the junction axis properly, as `022` did for the other three.
