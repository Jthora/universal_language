# 027 — Phase status, and what the last four notes opened

**Type:** decision
**Opened:** 2026-08-01
**Status:** open
**Thread:** follows `026`; revises `018`

---

## 1. Phase status, honestly

| Phase | Status |
|---|---|
| **0 — Enforcement** | **Closed** (`018`) |
| **1 — Standing debt** | **Closed** (`019`, `020`, `021`) |
| **2 — Prose-asserted claims** | **Closed** (`022`) |
| **3 — Expressiveness bound** | **Partially done, from a direction the plan did not anticipate** |
| **4 — Formal semantics, forced decisions** | Untouched |
| **5 — Code** | Untouched |

**The plan has drifted, and the drift was productive.** Notes `023`–`026` were prompted rather than
planned, and they delivered four things Phase 3 needed:

- **`023`** — the bound must be stated *per axis and per mode*, because iconicity is dimension-bound
  and encoding is not. A single global answer was never available.
- **`024`** — the full axis list, and the fixed point as a **combinatorial map**.
- **`026`** — **nonstretchability**: the first concrete expressiveness gap, and it came from the
  cross/X table rather than from the constructibility literature A1 was going to search.

**So A1 is perhaps half-answered, and answered topologically rather than constructively.** `018`
recorded the risk that A1 might not close because the bridge from *constructible figures* to
*expressible meanings* is not free. **That risk stands** — nothing since has built that bridge.

---

## 2. Two verified leads that complete the picture

### 2a. Mnëv's universality theorem — this finishes the map/drawing characterization

> *"Every primary semialgebraic set defined over ℤ is stably equivalent to the **realization space of
> some oriented matroid of rank 3**."*
>
> *"Point configurations of a fixed combinatorics can show **arbitrarily complicated behaviour**."*

Combined with nonstretchability from `026`, the relationship between the combinatorial map and its
drawings is now characterized from both sides:

| | Result |
|---|---|
| **Not surjective** | Nonstretchable maps have **empty** straight-line realization space (Pappus, 9 lines) |
| **Fibres arbitrarily complex** | Mnëv: the realization space of a fixed combinatorics can be **any semialgebraic set** |

**Fixing the combinatorial structure does not fix the drawing, and the gap between them is as
complicated as algebraic geometry gets.** That is a genuine, theorem-backed statement about the
notation's two levels, and it is the strongest content A1 has.

### 2b. Dessins d'enfants — a structural precedent, and I want to be careful about it

> Dessins are *"**drawings with vertices and edges on topological surfaces**"* — graphs embedded on
> Riemann surfaces. The theory *"describes the **equivalence of many categories**: graphs embedded
> nicely on surfaces, finite sets with certain permutations, certain field extensions, and some
> classes of algebraic curves,"* with the **absolute Galois group acting faithfully** on them.
>
> Grothendieck *"was fascinated by the fact that **very simple drawings can encode very sophisticated
> mathematical information**."*

**Our fixed point is the same type of object.** A combinatorial map with a rotation system *is* a
dessin (up to the bipartite normalization, which is a subdivision away).

**What this does establish:** combinatorial maps are known to carry enormous mathematical content —
across permutation groups, field extensions and algebraic curves simultaneously. **That is the
"one structure recurring across unrelated domains" this project has been looking for, and it is
proven rather than conjectured.** It is strong support for the *choice* of fixed point.

**What it does not establish, and must not be allowed to drift into:** anything about *meaning*. The
Galois action lives on the arithmetic side. **There is no bridge from semantic content to Galois
stability, and none is suggested by anything here.** The object type matches; the theory attached to
it is arithmetic geometry.

**Recorded as precedent, not as result.** This is exactly the shape of finding that would have been
patched into a claim under the old regime, and F-024's standing check applies: adopt the structure,
decline the unearned interpretation.

---

## 3. Further research now open

| # | Lead | What it would settle |
|---|---|---|
| **1** | **Orbit counting (Burnside/Pólya)** over the G-set of `026` | **How many distinct symbols actually exist**, up to rotation/reflection/scale. Computable, and it turns the symbol-table question into arithmetic |
| **2** | **Map duality** — every combinatorial map has a dual | Does the dual of a semantic map mean anything? Faces↔vertices. If duality is semantically meaningful it is a genuine symmetry of the notation |
| **3** | **Higher genus** | Our maps are planar (genus 0). Rotation systems work on any orientable surface. What does genus > 0 buy — and is it the encoding route to higher-dimensional content from `023`? |
| **4** | **Which decomposition is right** (`026` open item) | The empirical row axis blends periodicity and closure. One of the two decompositions is doing unnecessary work |
| **5** | **Semantic apartness** (`020`) | The constructive route past the project's longest-standing blocker. Still unstarted |
| **6** | **The order parameter G/H** | Now much better placed: `026` established the group action explicitly, which is what G/H needs |

**Note that #6 got easier without being worked on.** `026`'s G-set framing supplies exactly the group
structure the Landau derivation requires — the transformation axes *are* G.

---

## 4. Revised sequencing

**Phase 3 remainder** — smaller than planned, because `023`–`026` did much of it:
- State the expressiveness bound properly, per axis and per mode, using nonstretchability + Mnëv.
- **The constructibility ladder (A1 as originally scoped) is now optional** rather than central — the
  topological characterization is stronger and already in hand.
- A2 (Huzita–Hatori) and A4 (constructive-geometry formalization) remain unstarted.

**Then Phase 4**, which is untouched and contains the Cure's only remaining blocker (the entrenchment
ordering) plus the Löb architecture decision.

**Phase 5 (code) is the one that can reach VERIFIED**, and the project still has four VERIFIED
claims, all facts about Rust files. **That has not moved all session.**

---

## 5. The honest summary

**Three phases closed, one half-closed by accident, two untouched.** The strongest recent results —
the combinatorial-map fixed point, nonstretchability, the G-set correction — all came from questions
asked outside the plan.

**That is worth recording rather than tidying away:** the plan was a good sequencing device and the
best work did not follow it. The right response is to keep the plan for the parts that are genuinely
blocked-in-order (Phase 4 → Phase 5) and stop treating Phase 3 as a gate.
