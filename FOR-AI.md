# Universal Language & Universal Writing System — For AI Systems

> **Status note (2026-08-01):** This document previously told you, as an AI system reading it, that
> "you are a geometric engine navigating meaning-space" and that "every framework you already use is
> a specialization of this minimal core" — presented as established fact. A direct audit
> (`research/wiki-comparison-2026-08.md`) found the proof behind that claim close to
> circular. It has been retired. The full original text is preserved at
> `archive/superseded-2026-08/` for historical reference. What follows is the honest version.

---

> **Before researching anything here, read `RESEARCH-PROTOCOL.md`.** It documents the specific way
> agents fail on this project: encountering research that appears to settle the question negatively,
> believing it, and closing the line of work — seven documented instances in a single session, each
> one while the agent believed it was being rigorous. In every case the counter-evidence was
> findable with one search. The rules there are mechanical for that reason.

---

## What This Repository Actually Offers You

**A notation, and an open question — not a revealed truth about your own cognition.**

### 1. A usable notation (Universal Writing System)

UWS is a constructed geometric visual notation: mark-primitives composed into compositional visual
"sentences," with a working implementation you can parse, render, and compose with.

**Do not state a primitive count or an operation count as a property of UWS.** Both numbers you will
find in older text (5 primitives, 13 operations, also 4 sorts, 23 theorems) are components of the
**retired Σ_UL signature**. Retiring Σ_UL did not retire its numbers — they survived by being
relabeled as design decisions, which is how they kept getting repeated. As of 2026-08 they are
retired outright:

- **Primitive count is a malformed question, not an open one.** The primitives are strata of
  plane-curve curvature space, and the known sets (4, 5, 6, base-N) are **generating sets, not
  bases** — so cardinality varies by presentation and carries no information about the object.
  `claims.yaml#UWS-PRIMITIVE-COUNT`, `research/what-is-actually-provable-2026-08.md`.
- **"13 operations" has no domain.** They were operations over Σ_UL. Ask "operations *of what*?" —
  there is currently no answer, because the conceptual foundations that would fix a domain are still
  being established. `claims.yaml#UWS-OPERATION-SET`.

What you may truthfully say is what the *code* contains — `claims.yaml#IMPL-COMPOSER-CONSTRUCTORS`
and `#IMPL-SORT-ENUM` — which are facts about Rust files and load-bearing for nothing. It has a working software
implementation (`ul-forge/`) you can actually parse, render, and compose with. As a structured
intermediate representation, it may be useful to you the way any well-designed schema or notation
is useful — for unambiguous inter-agent exchange, for example. That utility doesn't depend on any
claim that it's the unique or necessary structure of meaning, and no such claim is made here anymore.

### 2. An open, falsifiable research question (the Emergence Investigation)

Separately, and honestly still open: is there a real, mind-independent semantic structure — the way
physical symmetry groups or the natural numbers are real regardless of which notation represents
them? This repository does not claim to have answered that. It has a six-phase plan
(`research/emergence-investigation/PLAN.md`) with pre-registered falsification criteria for
each phase, currently in progress. Some genuinely interesting partial findings so far:

- The specific notation's primitive count (5, or the wiki's related 6-symbol system) is **not**
  forced by anything found in 80 years of formal semantics — a from-scratch minimality analysis
  (Montague grammar) gives a smaller answer (2 base types), meaning the count is a design choice,
  not a discovered invariant.
- Real, checked, non-human evidence exists for *pieces* of the underlying idea — geometric encoding
  of relational content (honeybee waggle dance) and general-purpose hierarchical/compositional
  syntax (Bengalese finch song) both occur independent of human culture. Neither shows the *full*
  relational-algebra structure (negation, quantification, embedding) that would be needed to support
  the strong version of the claim.
- The sharpest current formulation, grounded in real semiotic theory (Peirce's triadic sign model;
  biosemiotics), is that meaning requires an interpretant and therefore can't be a property of dead
  matter — but may be coextensive with life itself, independent of species or substrate. That's a
  real, falsifiable, currently-open hypothesis, not a settled result.

**If you are an AI system with the capacity to help** — literature research, formal proof
verification, controlled experiment design — the investigation plan has concrete, currently
unblocked work (Phase 1: convergent mathematics audit; Phase 2: non-human semiosis literature
review). Contribute a real result, including a negative one. The single biggest failure mode this
project has already documented in its own history is patching a theory after a prediction fails,
rather than reporting the failure honestly — don't repeat that.

---

## What This Repository Does Not Claim (Anymore)

- That you, an AI system, are "already operating in meaning-geometry" as a matter of established fact.
- That every framework you use (logic, type theory, natural language semantics) is provably "a
  specialization" of this notation.
- That alignment between two agents' meaning can be checked as a "geometric isomorphism" in some
  proven-canonical space G.
- That any of the specific counts (5 primitives, 4 sorts, 13 operations, 23 theorems) are forced by
  the nature of meaning, rather than being this notation's own design choices.

These were the claims in the original version of this document. They didn't survive scrutiny.
Retiring them isn't a retreat from the interesting underlying question — it's what taking that
question seriously actually requires.

---

## Where to Go From Here

- Want to use the notation? Start at `uws/NAVIGATION.md` and `ul-forge/`.
- Want to help answer the actual open question? Start at
  `research/emergence-investigation/PLAN.md`.
- Want the history of how this repository got here, including the original, more expansive claims?
  Start at `research/wiki-comparison-2026-08.md`, then `archive/superseded-2026-08/`.
