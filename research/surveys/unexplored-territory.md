# Unexplored Territory — What's Available, What We've Never Touched, What Still Vexes

**Date:** 2026-08-01
**Question:** how much open research can we still leverage, what avenues give auxiliary
perspectives, and what gaps genuinely remain hard?

**Short answer on scale:** this project has now run perhaps 40 literature probes. The directly
relevant literature spans roughly fifteen mature fields and is measured in thousands of papers.
**We have sampled, not surveyed.** Every probe so far has returned something that changed a
decision, which is itself evidence the sampling rate is still far too low.

---

## PART I — Four probes just run, and what they returned

### 1. Abstract Meaning Representation (AMR) — major prior art, and a free benchmark

AMR is a directed graph semantic representation: nodes are concepts, edges are labeled relations,
capturing "who is doing what to whom." Concepts come from PropBank framesets. **AMR 3.0 is an
annotated corpus of 59,255 English sentences.**

Two things matter here:

**(a) They hit our exact problem.** AMR permits **reentrancy** — the same concept participating in
multiple relations — and this is documented as its central parsing difficulty, requiring "more
complicated algorithms and grammars." Reentrancy is our coreference/sharing problem. AMR handles it
with **variables** (`x / person` with later re-reference), the same technique CGIF uses and the same
one the IR decision landed on. A third independent confirmation.

**(b) It hands UWS an expressiveness benchmark for free.** ~59k human-annotated meaning graphs is
exactly the corpus needed to run the "expressive failure as data" methodology at scale: attempt to
express each AMR structure in UWS, and log every failure. That converts UWS-as-instrument from an
aspiration into a runnable experiment with tens of thousands of cases — and one nobody can accuse of
being cherry-picked, because the corpus was built for other purposes.

**Also worth noting:** AMR is sentence-level; multi-sentence/document AMR is a known open problem.
UWS's 2D placement grammar may have something genuine to offer there, since spatial layout is a
natural carrier for discourse-level structure that linear formats struggle with.

### 2. Gärdenfors' Conceptual Spaces — partially rescues the repair operator

This one changes a conclusion I'd already closed.

Gärdenfors' criterion: **"A natural concept is a convex region of a conceptual space"** — for any two
points in the region, everything between them is in the region too. The motivation is that if two
objects are both examples of concept C, anything between them on the relevant quality dimensions
should be as well. Prototypes fall out as the geometric centroid. And critically, he argues
**convexity is what makes concepts learnable and communicable** — it "supports communication and
interaction and thus the negotiation of meaning between subjects."

**Why this matters for F-009:** I argued the admissible region 𝒜 is almost certainly non-convex, so
metric projection is multivalued, and recommended abandoning metric projection for discrete
ontology-repair methods. That reasoning still holds *as stated* — non-contradiction does carve a
non-convex region, and a union of convex regions is not convex.

**But it opens a path I dismissed too quickly.** The **intersection** of convex sets *is* convex. So
if the invariant set is designed such that each individual invariant carves a **convex** constraint,
then 𝒜 is convex **by construction**, projection is unique, and the Hilbert projection theorem
applies. That is a design strategy, not a discovery — and it now has independent cognitive-science
motivation rather than being mathematical convenience.

**Revised position:** two viable routes to repair, not one. (a) Discrete: pinpointing/minimal
diagnoses, no geometry. (b) Metric: design invariants as convex constraints so 𝒜 is convex by
construction. Route (b) is more restrictive but yields a *unique* repair and connects to a serious
theory of concepts. This should be an explicit comparison, not a foreclosed question.

*Adjacent thread not yet pulled:* "On convex decision regions in deep network representations" —
empirical work on whether trained networks actually exhibit convex concept regions. Directly
relevant to Phase 4.

### 3. Decipherment — the empirical science of our founding scenario, and it is sobering

The project's origin is: how would a mind with no shared biology read our notation? Decipherment is
the field that actually studies this, and its record is a warning.

Ventris cracked Linear B on **two conjectures**: that certain repeated words were Cretan place
names, and that **the underlying language was an early form of Greek**. Method: sign frequency and
positional analysis, plus comparison against the already-deciphered Cypriot syllabic script.

The essential point: **he had to guess the language.** Success came from a known related script, a
known language family, and external anchors (place names). Computational decipherment today still
rests on "assumptions... including knowledge of the language family."

**And the counterexample is decisive: Linear A has been under attack for 70+ years and remains
unread.** It is not short of structure. It is short of shared reference.

**Implication for UWS/UL, and it's uncomfortable:** structural regularity is not sufficient for
decipherability. A receiver needs shared *reference*, not merely shared *form*. A perfectly
compositional, perfectly regular notation can be permanently opaque to a reader lacking grounding.
This is a direct challenge to the founding premise that a well-designed geometric notation is
readable by an alien or artificial mind — and it means the grounding problem is not a philosophical
footnote but the operative engineering constraint.

### 4. Universal Networking Language (UNL) — the closest precedent, and it's a cautionary tale

UNL is the nearest historical analogue to this project, and we had never looked at it.

Launched 1996 at the **United Nations University** Institute of Advanced Studies, Tokyo. A
declarative formal language representing semantic content, usable as an interlingua for machine
translation or as a knowledge representation language. Spun out to the **UNDL Foundation** in
Geneva (2001). **17 language centers worldwide.**

Its documented criticisms map almost one-to-one onto this project's own history and roadmap:

| UNL criticism | This project's counterpart |
|---|---|
| Semantic coverage gaps — idioms, cultural nuance, ambiguity | The D2 completeness challenge, and its patch-until-100% failure |
| **English bias in the "Universal Words" inventory** — the "universal" vocabulary was not culturally neutral | The "universal" claim generally; the primitive inventory's cultural loading |
| Cannot encode pragmatics — focus, speech acts | Exactly what Pass 2's performative-force extension bolted on |
| "A colossal waste of resources and money" (contemporary reception) | The reputational risk of overclaiming |

A UN-backed, 17-nation, multi-decade effort at a universal semantic interlingua ran into precisely
the walls this project keeps rediscovering independently. **Reading UNL's post-mortem carefully is
probably the single highest-value literature task remaining** — it is the closest thing to a
controlled experiment on whether this class of project can succeed, and what breaks first.

---

## PART II — The map of what we have never touched

Ordered by expected value. None of these have been researched at all.

### Tier 1 — likely to change decisions

| Area | Why it matters |
|---|---|
| **UNL post-mortem (deep)** | Nearest precedent; documented failure modes; see above |
| **Cognitive Dimensions of Notations** (Green & Petre) | An actual HCI framework for *evaluating notation designs* — viscosity, hidden dependencies, premature commitment, secondary notation. Directly applicable to UWS, and would give the 5-vs-6 decision real criteria |
| **Semantic Web / knowledge graphs in practice** | RDF, OWL, JSON-LD, schema.org, SHACL. Decades of industrial experience with exactly "typed graph + constraints + validation." SHACL in particular is a shipped constraint language for graph validity — arguably the Cure's check phase, already standardized |
| **Linguistic typology (WALS, Greenberg universals)** | Empirical cross-linguistic universals from ~2,600 languages. The actual evidence base for what is universal in human language, which the project has been theorizing about without consulting |
| **Mechanistic interpretability / representation engineering** | The Cure's nearest neighbors in current AI safety: probing, sparse autoencoders, activation steering, concept erasure. Needed both to position the work and to avoid duplicating solved problems |

### Tier 2 — strong auxiliary perspective

| Area | Why it matters |
|---|---|
| **Institution theory** (Goguen & Burstall) | A formal theory of "what is a logical system" and how to translate between them. Directly relevant to UL-as-common-target-of-many-formalisms |
| **Sheaf theory / sheaf-theoretic consistency** | Formalizes exactly "when do locally consistent pieces glue into a globally consistent whole" — which is the Cure's core question. Contextuality results (Abramsky) may bear on what inconsistency *is* |
| **Blissymbolics / AAC empirical evidence** | Decades of real data on whether constructed iconic notations are learnable, by whom, and where they fail. The closest empirical analogue to UWS's learnability claims, flagged repeatedly and still not done |
| **SETI / METI message design** | Arecibo, Pioneer/Voyager plaques, Dutil-Dumas, astrolinguistics. Practitioners who have actually tried to build substrate-independent messages |
| **Lojban / Loglan** | A constructed logical language with an actual speaker community and decades of use data. Empirical evidence on whether humans can use a logically-designed language fluently |
| **Concept drift detection (ML)** | The Cure's *detect* phase. An established literature we have never opened |

### Tier 3 — worth knowing, lower urgency

Wilkins' *Real Character* (1668) and the history of philosophical languages · Discourse Representation
Theory · semantic parsing beyond AMR · conceptual blending (Fauconnier & Turner) · image schemas
(Lakoff & Johnson — the wiki invokes these without citing the literature) · force dynamics (Talmy) ·
Wierzbicka's NSM primes (cited but never researched) · code biology (Barbieri) · Deacon's
*Incomplete Nature* · applied category theory (Fong & Spivak) · runtime verification and monitors ·
abstract interpretation and Galois connections · notation design history in mathematics

### The remaining wiki frontier

~25 UL-adjacent wiki pages surfaced in the original crawl and never fetched: Semantic Operator,
Semantic Noether Principle, Semantic Conservation Law, Semantic Category, Semantic Morphism,
Semantic Observer, Semantic Agency, Semantic Consciousness, Semantic Hilbert Space (partially),
Semantic Action Principle, and the phonetics branch (Semantic Acoustic Correspondence, Vocal
Semantic Correspondence, Signal Manifold, Articulatory Polytope). The phonetics branch matters most —
it is the *speech* sibling to UWS, and this repo has no counterpart at all.

---

## PART III — The gaps that genuinely vex

Honest assessment of what remains hard after all of the above. These are not literature gaps; they
are unsolved problems.

### 1. The grounding problem — now the deepest one

Decipherment says it plainly: **structure without shared reference is unreadable.** Linear A is
structurally rich and permanently opaque. If UWS is to be readable by a mind that shares no biology,
culture, or referential world with us, compositional regularity is not enough — and we have no
account of what supplies the grounding. Changizi grounds the *marks* perceptually (in human visual
systems shaped by *terrestrial* natural scenes), which does not transfer to a mind with different
sensors. **This is the founding premise, and it is the least defended thing in the project.**

### 2. What *is* a semantic invariant, concretely?

The Cure needs invariants. We have category names — identity preservation, containment stability,
non-contradiction — and **not one of them has been written as a checkable predicate over the IR.**
Until one exists, the Cure has no check phase, and the difference between it and the existing graph
validator is aspirational.

### 3. What is drift, operationally?

The Cure *detects drift*. We have no operational definition, no measurement, no threshold, and no
example. "Semantic drift" is doing a lot of unexamined work.

### 4. Zero empirical contact

Every result in this repository is analysis. **No experiment has ever been run.** No legibility test,
no learnability trial, no confusion matrix, no causal-efficacy trial, no drift detection on a real
system. The strongest evidence the project has (Changizi) is someone else's. The AMR corpus and the
Blissymbolics AAC literature are both routes out of this, and both are sitting unused.

### 5. Decisions still open that block work

The 5-vs-6 primitive count (now empirically decidable, undecided). The two repair routes (discrete
vs. convex-by-construction) now need an explicit comparison. Neither is hard — both are just
undone.

### 6. The two-property problem

The repo and the wiki are diverging artifacts of one project with no sync mechanism, no shared
glossary, and different epistemic standards. This has already produced one round of claims migrating
between them unchecked, and nothing currently prevents a second.

---

## Bottom line

**Availability is not the constraint.** Every field named above is mature, open, and mostly
free to read. The constraint is that this project has consistently theorized in domains where
empirical and formal literatures already exist — and each time we finally check, the literature
either solves the problem (ontology repair, e-graphs, refinement types), sharpens it usefully
(Gärdenfors, term-graph confluence), or shows it has already been attempted and failed in an
instructive way (UNL, Linear A).

**The pattern is consistent enough to be a working rule: check the literature before theorizing,
not after.** On current evidence the expected value of one hour of literature search exceeds one
hour of derivation by a wide margin.
