# STATE-OF-PLAY

**Written fresh 2026-08-12, replacing a patched version.** The previous one had been amended nine
times against thirty-eight notes — a delta applied to a partial picture, which is how a framing error
survived four notes (`FAILURES.md` F-027). This is a full restatement, not an amendment.

---

## 1. What we hold

### The one structural result

**Seven independent lines of work converged on the combinatorial map** — a graph plus a cyclic
ordering of branches at each vertex. Coarse-graining the primitives, line arrangements, realization
spaces, arithmetic geometry, the expressiveness question, spatial semantics, and the Cure's repair
ordering all arrived at the same object. **None were aimed at it.**

**That convergence is the strongest thing here.** What it *means* is scoped in §3.

### Geometry — derived, not chosen

- The primitives are **cells in a product of four axes**: curvature, singularity, closure, junction
  degree. Not a list anyone picked.
- **"How many primitives" is malformed**, not open. The collapse relations are theorems.
- **Junction degree was a missing axis**, and it is the one the project's strongest empirical
  evidence (Changizi — legibility, not semantic transparency) points at. `Angle` is degree-2 ∧
  tangent-discontinuous; **degree-1 has no name.**
- Axes are of **different types** — binary, discrete-unbounded, continuous. Only closure is binary.
  Curvature is a *function space* with canonical integer invariants (Whitney–Graustein).
- **Tables are projections** of the axis product; **transformation axes are group elements**, so
  Erlangen *is* the quotient. That explains both the hyper-dimensional blow-up and its collapse.
- **2D bounds iconicity, not encoding** — Coxeter diagrams determine arbitrary-dimensional polytopes
  from a flat graph.

### Obstructions, and why none are fatal

| Theorem | What it forces |
|---|---|
| **Löb** | The Cure cannot verify its own repairs at equal strength → **comparator architecture** |
| **Rice** | Does **not** apply to GIR — it concerns programs, not data structures. An unchecked import, corrected |
| **Plump** | Confluence undecidable for cyclic rewriting → the acyclic term-graph core |
| **AGM** | Rational repair needs a **preorder**, not a metric → the convexity blocker was a formalism artifact |

### The Cure — specified, not blocked

Every structural blocker moved. Architecture: **comparator, trust from the anchor, not self-proof.**
Entrenchment ordering: **derived from the coarse-graining survival order**, not stipulated. Drift:
**absolute, anchored to invariants** — the IPK fix. Signal: **coupling asymmetry**, not consistency.

**It is now blocked on implementation, and on one live error — see §4.**

### Method

Twenty-nine recorded failures, twelve trap signatures, nine rules, **five machine checks**. Four of
the last five protocol additions exist because *a correction was written down and had no force*.
**Prose is not enforcement** is the most repeated lesson here.

---

## 2. The one number that matters

**5 VERIFIED claims of 99. Four are facts about Rust files.**

The fifth — `IMPL-COMBINATORIAL-MAP` — is the only substantive machine-checked result, and it made
Jordan separation an executable test rather than an argued claim.

**Everything else is ARGUED or DESIGN-CHOICE: good arguments, unverified.**

---

## 3. What is genuinely open

**`meaning → map` does not exist.** `map → drawing → figure` is characterized by theorems (Mnëv,
nonstretchability, Galois). **The top link is unbuilt**, and the bridge test (`034`) failed — no
other semantic formalism converged on rotation systems.

**That failure is weaker than first recorded.** Its evidence was narrowed to a single formalism after
AMR was found inadmissible (English-only corpus, `research-register.md#D2-c`). *Not established as
UL's fixed point* stands; *positive evidence the gap is structural* does not.

**Where the map does have a defensible role: UP, the bootstrapping layer.** Rotation breaks **ℤ/2**
of arbitrary convention where labels break **Sₙ** — minimal shared prior. Supported cross-domain by
music, where the relational part travels cross-culturally and the conventional part (octave
equivalence) does not.

**Still open:** the order parameter (identify G and H), whether emergent protocols share invariants
under coarse-graining, and the axis derivation from invariant theory rather than by inspection.

---

## 4. What is wrong right now

**`REGIONS-ARE-FACE-UNIONS` is materially incorrect for real notations**, found by this review.

The Phase 4 decidability argument assumed regions are unions of the map's faces. But the map is
scoped to **connected** configurations, and **a notation with more than one stroke is disconnected by
default.** Face tracing then returns the wrong faces — four for two disjoint triangles, not three.

> **It stays decidable and becomes wrong.** Confident incorrect answers, which is worse than
> undecidability because nothing signals the failure.

**The fix is concrete:** the IR must carry the **nesting relation** between components, which is
additional structure beyond the rotation system, and is exactly what `CONNECTIVITY-AXIS-MISSING`
identifies as absent.

---

## 5. Per construct, plainly

| | Status |
|---|---|
| **UL** | The open question. Shape is concrete — a universality class whose candidate fixed point is a combinatorial map — and it has a falsifier. The bridge to meaning is unbuilt. |
| **UWS** | A **derived** notation, not an invented one. Its structure is the most developed thing here. It is *notation × derived*, a category with no historical precedent, because nobody has derived one. |
| **UP** | Was empty. Now has a candidate answer — the map as convention-minimizing — and it is testable. |
| **UQPL** | Least served. Constrained (intuitionistic, total, acyclic core, likely quantifier-free) but unspecified. |
| **The Cure** | Specified end to end. Blocked on implementation and on §4. |

---

## 6. Next

1. **Fix §4** — add the nesting relation to the IR. A Phase 4 conclusion is invalid until done.
2. **Run the ablation** — symbolic versus rotation encoding at matched population size. Two
   independent routes now point at it; it is the only forward-facing empirical test in the project.
3. **Derive the axes from invariant theory** rather than extending a list built by inspection.
4. **Move the VERIFIED count** — the only measure that has barely moved.

---

## 7. How this document is maintained

**Rewritten, not patched.** Nine patches against thirty-eight notes is what produced the failure this
review corrected. When it next drifts far enough that amendment is tempting, **that is the signal to
rewrite it again.**
