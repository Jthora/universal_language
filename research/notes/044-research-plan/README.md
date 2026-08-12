# 044 — Multi-volley research plan

**Type:** decision
**Opened:** 2026-08-12
**Status:** open — volleys close individually
**Thread:** follows `043`, which measured the search practice and produced S8–S11

**Design principles, all earned rather than chosen:**
**failure-first** (S10 — the graveyard survey ran at note 37 and outperformed everything before it) ·
**falsification-first** (Roster A was the most productive planned crawl and was designed to hurt) ·
**synonym sweep** (S9) · **primary reads for anything load-bearing** (S8 — four for four).

---

## Volley 1 — Failure surveys for the three constructs we never surveyed

**Why first:** `037` ran the graveyard survey for *universal languages* and it was the highest-value
crawl in the session. **We never ran the equivalent for the Cure, for UP, or for derived notation.**
And it is the volley most likely to hurt, which is why it goes first.

| Target | Query |
|---|---|
| **The Cure** | semantic drift detection in long-lived knowledge bases — why approaches failed |
| **The Cure** | ontology maintenance and degradation over decades; what killed the projects |
| **UP** | first-contact / METI message design — documented criticism of its assumptions |
| **UWS-as-derived** | has any notation ever been *derived* rather than designed? |

**Preregistered expectation:** the Cure's graveyard is **populated** — ontology maintenance has a
long record of decay, and if the documented causes match ours, that is bad news we need early.
**The derived-notation query I expect to return nothing**, which would confirm `038`'s claim that
the cell is empty — and a genuine null there is informative rather than disappointing.

**Stop condition:** four queries. Do not chase.

---

## Volley 2 — Synonym sweep on the fixed point

**Why second:** `043` established we found dessins by accident thirty notes late. **A scan just now
shows `DCEL` and `doubly-connected edge list` have never appeared in this project at all** — the
standard computational-geometry structure for planar subdivisions with face traversal, textbook since
the 1970s.

**We derived that object and implemented it from scratch without looking.**

| Vocabulary | Field |
|---|---|
| **DCEL / doubly-connected edge list / half-edge** | **computational geometry, CAD, GIS — never searched** |
| ribbon graph · fat graph | topology, moduli spaces, Kontsevich |
| rotation scheme · band decomposition | topological graph theory |
| combinatorial map | spatial databases, image analysis |

**Preregistered expectation:** `map.rs` **reinvents DCEL**, and the literature carries known
pitfalls — degenerate cases, unbounded faces, robustness — that we will otherwise hit one at a time.
**I expect this volley to cost us originality and save us engineering.**

**What would surprise me:** if rotation systems in those fields carry *semantic* interpretations. That
would bear directly on `034`'s failed bridge test, which never searched these vocabularies.

> **CLOSED — `046`.** Prediction confirmed: `map.rs` is a DCEL with the geometry deleted. `Nesting`
> re-derived one of the field's **two** standard fixes; the other (dummy edges) is now verified as a
> test. **The surprise did not occur** — `034`'s negative holds across six vocabularies and is
> upgraded from unchecked to checked. And the framing above was itself wrong: **`half-edge` was
> already written in our own source** (F-029, S12).

---

## Volley 3 — Primary reads on every priority-0 claim (S8)

**Ten priority-0 claims.** Audit which rest on summaries; fetch a primary for each that does.

**Four primary reads have changed a claim four times out of four.** With ten priority-0 claims, this
is the highest expected yield per query in the plan.

**Stop condition:** one primary per claim. Not a literature review.

---

## Volley 4 — The four modes we never used

Recorded in `043` as untested gaps. **Low confidence, genuinely unexplored.**

- **Non-English literature.** Zero queries so far, in a project that cites UNL's English bias
  approvingly. German and French work on notation; Russian semiotics.
- **Corpora rather than papers.** We searched for claims and never for data we could run something
  against. ELCC and the emergent-communication corpora are the obvious targets.
- **Null results specifically.** *"X was tried and did not work"* is a different query from *"does X
  work"*, and publication bias makes the first rarer and more valuable.
- **Practitioners.** Nobody who has actually built a notation has been consulted.

**Honest status:** this volley is speculative. It is included because *not knowing* whether these
pay is itself a gap, and three of the four are cheap.

---

## Volley 5 — The open questions proper

**Only after 1–3.** These are the actual research questions, and they are the most expensive and
least likely to resolve in one pass.

1. **Identify G and H** for the order parameter. `026` supplied the group action, so this is better
   placed than it has ever been.
2. **Derive the axes from invariant theory**, level by level, rather than extending a list built by
   inspection (`032`'s outstanding item).
3. **The orientation axis** — flagged in `032`, never derived.
4. **`meaning → map`** — the central gap. Volley 2 may bear on it; nothing else here will.

---

## What this plan does *not* include, and why

- **The ablation** (symbolic versus rotation at matched population size) needs training runs and
  cannot be done here. It remains the outstanding empirical item and no volley substitutes for it.
- **The curvature axis repair** (`033`) is deliberately excluded. It fixes a real defect in the
  *microscopic* layer, and by `UL-WORK-IS-FIXED-POINT-WORK` that layer cannot generalize. **Recorded
  because it is the second time this item has been demoted by its own criterion.**

---

## Discipline for every volley

**Preregister before searching** (S2) · **one adversarial query per negative** (R1) · **check a
source's scope before citing it** (S11) · **log whether counter-evidence was easy or hard to find**
(S7) · **each volley becomes its own numbered note.**

**And the stop conditions are real.** `043` found the searching was reactive — every query answering
whatever was in front of us. **A volley with no stop condition becomes reactive again.**
