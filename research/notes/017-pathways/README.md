# 017 — Pathways from here

**Type:** brainstorm
**Opened:** 2026-08-01
**Status:** open
**Thread:** follows `016`
**Question:** what does the constructive turn open, and what else is live?

Speculation permitted and labelled, per `../README.md`.

---

## A. Constructive foundations — the largest new opening

`016` established that the notation's real home is the **constructive** framework, not the axiomatic
one. That framework has machinery we have never touched.

### A1 — Constructibility as a formal expressiveness bound

This is the most promising item in the note. **A construction system has a provable reach:**

| System | Reach |
|---|---|
| Straightedge + compass | quadratic extensions of ℚ |
| + marked ruler (*neusis*) | cube roots, angle trisection |
| Origami (Huzita–Hatori) | strictly stronger — solves cubics |

**UWS has never had a formal expressiveness characterization.** Every previous attempt ran through
"can it express meaning X," which is unfalsifiable hand-waving. **Constructibility gives a real
ladder** — and it is a *theorem-backed* one, which is exactly the kind of result the proof turn
calls for. *What would it settle:* what a drawing-based notation can and cannot reach, as
mathematics rather than assertion.

### A2 — The Huzita–Hatori axioms as an alternative presentation

Origami has **seven** fold axioms and is strictly more powerful than straightedge-compass. It is a
genuine alternative construction basis — a different generating set for the same geometry.

*Worth checking, not claiming:* the octahedral / base-6 work concerns six slots, and Huzita–Hatori
has six original axioms with a seventh added by Hatori. **That may be a coincidence and should be
treated as one until checked.** Recording it because the check is cheap and the alternative — noticing
it later and retrofitting — is exactly the failure mode this project has.

### A3 — Intuitionistic logic as the notation's native logic

Already known: Curry–Howard–Lambek is intuitionistic, not classical. If the notation is genuinely
constructive, **its logic should be intuitionistic too**, and that is a *coherence check* — two
independent commitments that should agree. If they disagree, one is wrong. Cheap, and informative
either way.

### A4 — Existing formalizations of constructive geometry

The search in `016` surfaced *"A Finite, Feasible, Quantifier-free Foundation for Constructive
Geometry."* **Unread.** If a quantifier-free constructive axiomatization exists, it may be a far
better formal target for the IR than anything we would build.

---

## B. Mereotopology — the joint home for two co-primitives

`014` and `016` converge on **Point and Enclosure as co-primitives**, with the point-tradition and
the region-tradition each privileging one. The obvious question: **is there a framework that takes
both?**

There is. **Mereotopology** — mereology (parts, regions) plus topology (connection). The **Region
Connection Calculus** is its worked form, and `016`'s search surfaced *"A Spatial Logic based on
Regions and Connection."*

*Why this matters beyond tidiness:* the IR currently has no principled spatial semantics. If UWS is
marks-in-space, and the co-primitives are point and region, **RCC is a candidate formal semantics
for the notation's spatial grammar** — off-the-shelf, with existing decidability results. That is a
real engineering lead, not only a philosophical one.

---

## C. Discharge the remaining assertions

`014` paid down one prose-asserted claim by deriving it. **Two remain in the same state:**

- **`KAPPA-STRATIFICATION`** — the partition of curvature-function space and the collapse relations.
  Still prose. Should be written out as a partition lattice with the collapses as theorems.
- **`ENCLOSURE-IS-DISTINCTION`** — Jordan separation ≅ Spencer-Brown's mark. Asserted, never proved.
  It is load-bearing for the anchor argument.

**These are known debts with known methods.** Unglamorous, and `014` demonstrated the work changes
the answer — four corrections came out of deriving a table I thought I already had.

---

## D. Standing sweeps

- **R7 — adversarially audit every negative currently cited, oldest first.** Already overturned
  **Zadrozny** and **TopSim**. Highest demonstrated yield per unit effort in the project, and it is
  mechanical.
- **Unread primaries.** Two priority-0 claims rest on summaries: *Drawing with Strangers* (the best
  empirical result available) and **Kazmi & Pelletier / Westerståhl** (the reopened necessity route).
  One fetch each. This is a standing S1 violation.

---

## E. The Cure — untouched

No work this session beyond the obstruction findings. Two things are now concrete:

- **Specify the entrenchment ordering.** AGM guarantees a rational repair operator *given* one, so
  this is the whole remaining blocker and it is a design task rather than a research question.
- **Write the Löb decision down.** External-and-stronger, accepted descending chain, or abandon
  proof-based self-trust. `LOBIAN-OBSTACLE` says this must be settled on paper before implementation.

---

## F. Code — zero lines this session

- `semantically_equal` via e-graphs, **scoped to the acyclic core**
- The validator rebuilt as **abstract interpretation with a Galois connection** (what Rice forces)
- `negate` reimplemented with a σ field instead of the self-loop marker
- **A CI checker for `claims.yaml`** — the tiers remain conventions without enforcement, which is
  this repo's named historical failure mode

---

## Assessment

**Highest value: A1.** A formal expressiveness bound for the notation is something this project has
never had, it is theorem-backed rather than argued, and it follows directly from the constructive
turn. It also converts "what can UWS express?" from an unfalsifiable question into a mathematical
one.

**Cheapest with demonstrated yield: D.** The R7 sweep has overturned two load-bearing negatives
already. The two primary reads are one fetch each and both sit under priority-0 claims.

**Most likely to change the engineering: B.** RCC as spatial semantics for the IR, with existing
decidability results.

**Most overdue: C.** Two claims still asserted in prose, in a project that concluded proof was the
route six notes ago.

**Honest note on sequencing.** A, B and C are all proof-shaped work and the project has produced
exactly one derivation (`014`) against many notes of reading. **The ratio should shift.** If the
next several notes are all literature crawls again, that is drift back toward the comfortable mode,
and this line is here so it can be checked later.
