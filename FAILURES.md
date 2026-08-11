# FAILURES

**Append-only. Entries are never edited into successes.**

> If a prediction fails, log it here and leave it. If the theory later changes, **append a new
> entry** — do not rescore, reword, or delete an old one. The value of this file is that it cannot
> be gamed.
>
> **Why this file exists:** the project's internal completeness score was driven from 32% to 100%
> across nine sequential rounds in which each failing case triggered a change to the theory, after
> which the case was re-scored as passing, and the final 100% was reported as validation. That is
> the specific failure mode this file makes structurally impossible. See
> `research/postmortem-and-rebuild-2026-08.md` FM2.
>
> **This file must exist and be in use before any new experimental or completeness work begins.**

Format: one entry per failure. Newest at the bottom. Never reorder.

---

## F-001 — Negation-as-reflection produced converse, not negation
**Date:** 2026-04-07 (recorded retroactively 2026-08-01)
**Claim that failed:** `negate` implemented as geometric reflection realizes logical negation.
**How it failed:** Reflection swaps subject and object, producing the relational *converse*
("B is acted upon by A"), which preserves truth value. Negation must flip truth value. Downstream,
the functional-completeness argument for `{negate, conjoin, disjoin}` and all De Morgan derivations
were invalid.
**Status:** Superseded in documentation by a boundary-inversion design. **See F-006 — the
replacement was never implemented in code.**

---

## F-002 — Minimality claim for the operation set was false
**Date:** 2026-04-07 (recorded retroactively 2026-08-01)
**Claim that failed:** The operation set is minimal; removing any operation loses expressive power.
**How it failed:** `conjoin` is derivable from `{negate, disjoin}` by De Morgan. The set is complete
and natural, not minimal.
**Status:** Claim weakened to "complete and natural generating set." Correct resolution.

---

## F-003 — Unique Grounding Theorem is close to circular
**Date:** 2026-08-01
**Claim that failed:** The mapping from geometric to semantic primitives is the unique
structure-preserving bijection, therefore forced rather than chosen.
**How it failed:** The five semantic primitives were defined by role properties written to mirror
the five geometric primitives already selected. A bijection between two hand-matched five-element
lists is not evidence of necessity. The repo's own `independent-derivation.md` was written to
address this and independently arrived at **4** sorts, not 5 — undercutting rather than rescuing
the claim.
**Status:** RETIRED. Material archived at `archive/superseded-2026-08/`.

---

## F-004 — The primitive count is not determinable from compositionality
**Date:** 2026-08-01
**Claim that failed:** There exists a forced number of semantic primitives (5, or 6) discoverable
by sufficiently rigorous derivation.
**How it failed:** Zadrozny (1994) proves bare compositionality is formally vacuous — for *any*
assignment of meanings to expressions, a re-encoding exists making it compositional. Compositionality
constrains nothing without additional, extra-mathematical naturalness conventions. The question is
therefore **provably underdetermined as posed**, not merely unproven.
Corroborating: no independently convergent tradition matches any candidate count (Aristotle: 10
categories; Vaiśeṣika: 6–7). Three blind, mutually isolated rederivations independently converged on
~2 base types and independently concluded the count is not forced; none approached 4, 5, 6, or 13.
**Status:** The count is reclassified `DESIGN-CHOICE`. This is a permanent result — no further
derivation effort will change it.

---

## F-005 — "G is weakly terminal" has an uncaught proof gap
**Date:** 2026-08-01
**Claim that failed:** Labeled PROVEN — every expressively complete language admits an injective
homomorphism into the geometric algebra G.
**How it failed:** The argument maps atoms injectively into G and invokes the universal property of
free algebras to extend. That property applies cleanly only when the source *is* free on those atoms.
Real languages carry semantic identities (synonymy, logical equivalence), making them quotients of
the free algebra. For the extension to be well-defined the kernel must contain those identities, and
for injectivity it must match them exactly. Choosing distinct positions for atoms does not secure
this, and the argument never addresses it.
**Status:** Open. Claim downgraded from `PROVEN`; not yet repaired or formally retired.

---

## F-006 — Documented negation fix was never implemented
**Date:** 2026-08-01
**Claim that failed:** `negate` satisfies involution — `negate(negate(a)) = a` — and is implemented
as boundary inversion (a σ ∈ {⊕,⊖} field on the assertion).
**How it failed:** The implementation (`ul-forge/crates/ul-core/src/composer.rs`) wraps the
assertion in a **new enclosure node** plus a self-loop `references` edge used as a marker. Double
negation therefore produces a two-frame-deep structure that is not equal to the original. There is
**no normalization, reduction, or equivalence machinery anywhere in the crate**, so nothing can even
detect the violation, and **no test asserts the law**. The documented boundary-inversion design (the
F-001 fix) does not exist in code.
**Status:** Open. Prerequisite: a `semantically_equal` decision procedure, which does not exist.

---

## F-007 — Finding F7 was specified and never propagated
**Date:** 2026-08-01
**Claim that failed:** The 4-sort / 5-primitive tension was resolved in April 2026 (marked
"CLEAR RESOLUTION — needs documentation additions," with reconciling text drafted and three target
files named).
**How it failed:** Four months later a repo-wide search of all three named targets returned **zero
matches**. The fix was written down and never applied. "Finding documented" had been treated as the
terminal state.
**Status:** Open. Designated as the first finding to be run through the new definition of done
(fix + test + propagation scan).

---

## F-008 — Causal-efficacy protocol contains a pseudo-replication error
**Date:** 2026-08-01
**Claim that failed:** Labeled "PROTOCOL READY" with a power analysis yielding ~90 independent
observations per condition.
**How it failed:** Repeated temperature-sampled generations from the same model and prompt are not
independent draws — they share weights and training history. Treating them as independent inflates
effect sizes and significance. The true unit of replication is the (model × task) cell: ~30 per
condition, not ~90. The stated power analysis is invalid and the design is underpowered relative to
its own claims.
**Status:** Open. No trials have been run. Protocol must be rebuilt around cell-level aggregation
before any execution.

---

## F-009 — The Cure's repair operator is not well-defined
**Date:** 2026-08-01
**Claim that failed:** Repair is projection onto the admissible region, `P : ℳ → 𝒜`, optionally
implemented as gradient descent.
**How it failed:** By the Hilbert projection theorem, a unique nearest point is guaranteed only for
a nonempty **closed convex** set; uniqueness follows directly from convexity. 𝒜 is almost certainly
non-convex and plausibly disconnected — non-contradiction admits states asserting A and states
asserting ¬A while excluding their midpoint. Therefore projection is **multivalued**, and gradient
descent on a non-convex potential converges to local minima, making repair **path-dependent**: the
same corrupted state repairs differently depending on trajectory history.
The wiki lists "Non-Injective Repair Collapse" as a *risk*; given non-convex 𝒜 it is a **geometric
certainty**.
**Status:** Open. **This is the central engineering problem of the program.** Candidate directions:
convex relaxation of 𝒜; explicit decomposition into convex cells with declared tie-breaking;
or abandoning metric projection for AGM-style least-change belief revision.

---

## F-010 — Distribution documentation described artifacts that did not exist
**Date:** 2026-08-01
**Claim that failed:** README instructed users to install packages and "run the experiments
yourself."
**How it failed:** Every package sat at version `0.1.0`, unpublished, with no publish workflow in
CI. README code examples used `point(existence)`, syntax the parser rejects — every first-time user
hit an immediate failure. `preregister.py` and `blind.py` were built, documented, and never run.
**Status:** Partially addressed by the 2026-08 restructuring (README rewritten). Publish workflow
still absent; example syntax still needs verification against the parser.
