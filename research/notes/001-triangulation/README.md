# Reading Between the Lines — Three Converging Fields, and a Benchmark Nobody Handed Us

**Type:** cycle
**Status:** closed
**Question:** Where do purpose, notation and the safety application actually meet?
**Note:** predates the notes convention (see `../README.md`); retained as written.

**Date:** 2026-08-01
**Question:** across ~55 research threads, is there something the pattern indicates that no
individual source states? Something in the collective peripheral vision?

**Method:** formed three hypotheses about what's absent, then tried to falsify them. **All three
were falsified.** Each falsification is more valuable than the hypothesis would have been.

---

## The single most actionable finding: the Cure has a benchmark

**Knowledge editing** (ROME, MEMIT) modifies specific facts in an LLM's weights. Its central,
published, *quantified* failure mode:

> Edits "are mostly applied at a surface-level without propagating to other related facts."
> Change Leonardo DiCaprio's nationality to Syrian, and the model should update his primary
> language to Arabic. **It doesn't.**
>
> **Parametric methods like ROME score under 50% on ripple-effect tasks**, with the highest failure
> rates in **logical generalization and compositionality**.

That is *exactly* the Cure's problem statement — a representation that is locally updated and
globally inconsistent — except it already has **a task, a metric, a dataset, and a published
baseline**.

**This converts the Cure from an architecture we are designing into an intervention with a
measurable comparison.** If justification-tracking consistency machinery improves ripple-effect
propagation over ROME/MEMIT baselines, that is a real, falsifiable, publishable result — and it
requires no claim about UL, no notation, and no metaphysics.

**After a session in which zero experiments were run and the project's strongest evidence was
someone else's paper, this is the first thing found that is both concrete and immediately
testable.**

---

## H1 — "Nobody does consistency maintenance over learned representations"

**Falsified, and the shape of the falsification is the finding.**

It is being done — but almost entirely **from the neural side, by people rediscovering symbolic
machinery**:

- **ChainEdit**: *"Propagating Ripple Effects in LLM Knowledge Editing through Logical Rule-Guided
  Chains."* That is dependency-directed propagation, renamed.
- **NeuSymMS**: applies *"Truth Maintenance System concepts to maintain logical consistency in
  dynamic knowledge bases through justifications and dependency tracking"* for LLM agent memory —
  retracting an old fact when a new one shares subject and relation. Explicitly TMS-derived.
- **Logically Consistent Language Models via Neuro-Symbolic Integration**: a neuro-symbolic loss
  training LLMs toward consistency with external facts and rules.

**So the real observation is not absence — it's non-citation.** The model-editing community has the
problem and is at <50%. The TMS/AGM community has had justifications, nogood databases, and
dependency-directed backtracking since 1979–1986, with published complexity analyses. The bridge is
being built, recently, from one side, apparently without full awareness of the other's forty years
of results.

**Implication for this project:** the Cure's contribution is not the consistency machinery — that's
solved. It is **carrying mature symbolic consistency theory across to sub-symbolic substrates**,
where a measured 50% failure rate is waiting.

## H2 — "The Cure cannot distinguish corruption from legitimate learning"

**Real problem, and it already has a standard answer that the Cure has not adopted.**

Both a desired edit and a corruption present identically to a consistency checker: *this contradicts
what was there.* A system that repairs every inconsistency back toward its established invariants
cannot learn — it is a conservatism engine.

**The answer is TMS's own architecture:** the distinction between **assumptions** (revisable, held
directly) and **derived beliefs** (which must follow from assumptions). Revision targets assumptions;
derived consequences propagate. AGM's counterpart is **epistemic entrenchment** — an ordering saying
which beliefs yield first under contradiction.

**The Cure as currently specified has neither.** It has invariants and a repair operator, and no
notion of *what may be revised versus what must follow*. Without that, it cannot tell a legitimate
update from drift, and F-009's repair non-uniqueness partly dissolves once you add it: you don't
choose an arbitrary nearest state, you revise the least-entrenched assumption.

**This is a concrete missing component, not a philosophical problem.**

## H3 — "There is no theory of minimum shared grounding"

**Falsified — and this is the biggest gap in our coverage.**

There is an entire field asking this project's founding question, formalized:

> **"How does a signal signal its own signalhood?"** — that is, how does an agent recognize that
> communicative behaviour is communicative at all?

Plus: *"sufficient common ground is integral to the recognition of signalhood"*; Lewis on
coordination games and common knowledge; **signaling games** as the base formalism; **deep-learning
emergent communication**, where two neural agents develop a protocol from nothing; and **ELCC (the
Emergent Language Corpus Collection)** — an actual corpus of emerged languages.

**This is the mouth-less-robot scenario, in silico, with published results and datasets — and this
project has never touched it.** Across ~55 threads and three separate "what haven't we checked"
sweeps, emergent communication never surfaced. That is the most striking coverage gap found.

It bears directly on the founding premise (`GROUNDING-PROBLEM`, priority 1): rather than arguing
philosophically about whether structure suffices without shared reference, this field **runs the
experiment**.

---

## What the pattern actually indicates

Three communities are converging on this project's territory from three directions, and they are
largely not citing each other:

| Community | Has | Lacks |
|---|---|---|
| **Knowledge editing / model editing** | The problem, quantified, with benchmarks and a <50% baseline | Forty years of consistency theory |
| **TMS / AGM / ontology repair** | Justifications, entrenchment, dependency-directed backtracking, multi-context maintenance | Any application to sub-symbolic substrates |
| **Emergent communication / signaling games** | The bootstrapping question formalized, with simulation methodology and corpora | Connection to either of the above |

**This project is unusual in looking at all three at once.** But two honest caveats, because
"uniquely positioned" is exactly the kind of claim this project has made before and been wrong about:

1. **The bridge is already being built.** NeuSymMS and ChainEdit are 2025–2026 work. This is an
   *active* area, not an empty one. Any window is closing, not open.
2. **Noticing an intersection is not the same as being able to work it.** The project has no
   experimental capability, no compute, and has never run a trial. Recognizing a gap and being able
   to fill it are different things.

---

## What changes

1. **Adopt ripple-effect evaluation as the Cure's benchmark.** It supplies task, metric, dataset,
   and baseline. This is the first concrete empirical foothold the project has ever had, and it
   requires no UL claim.
2. **Add the assumption/derived distinction and an entrenchment ordering to the Cure's design.**
   Without them it cannot distinguish learning from corruption — a fatal flaw for a safety mechanism
   and an easy fix, since both TMS and AGM supply the machinery.
3. **Reposition the Cure's contribution honestly:** not novel consistency machinery, but
   **transferring mature symbolic consistency theory to sub-symbolic substrates**, where the failure
   is measured and the theory is unapplied.
4. **Research emergent communication / signaling games immediately.** Highest-value unchecked area
   found; it is the founding question with an experimental methodology attached, and it has evaded
   three prior sweeps.
5. **Check whether anyone has already applied TMS specifically to ripple-effect propagation.** If
   ChainEdit's "logical rule-guided chains" already *is* dependency-directed backtracking under
   another name, the contribution narrows further — and it is better to know that now.

---

## The honest summary

The pattern doesn't reveal a hidden truth about meaning. It reveals something more useful and more
mundane: **the project's actual problem is being worked on right now, by people who don't know the
solution already exists, while the people holding the solution aren't looking at the problem.**

That's a real position. It is also a much smaller and more defensible claim than anything in this
project's history — and unlike the metaphysical version, it comes with a benchmark that can prove
us wrong.
