# Research Cycle — Seven Questions, and a Convergence Nobody Was Looking For

**Date:** 2026-08-01
**Method:** posed seven decision-relevant questions, then ran literature dives on each.
**Headline:** five of the seven independently converge on a conclusion this project has never
confronted — **the hard problem is not designing the notation.**

---

## The convergence (read this first)

Five separate literatures, approached for unrelated reasons, all say the same thing:

| Source | Finding |
|---|---|
| **ISO symbol testing** | Professionally designed safety pictograms fail the ISO 3864 comprehension criterion (67%) **without training**. Fire-action pictograms score **45%**. Only *post-training* averages clear the bar. |
| **Decipherment** | Linear A has abundant structure and remains unread after 70+ years. Linear B fell only when Ventris could conjecture the language family and anchor on place names. |
| **Music notation** | Stenographic notation requires users to "pre-memorize an association of glyphs with pitch or duration values, making notation **inherently not self-evident**." |
| **Music notation reform** | A multi-decade international project to design better notation produced something "only marginally better" and **failed at adoption**; standardization plus collective knowledge investment defeats improvement. |
| **UNL** | UN-backed, 17 language centers, decades — failed. Coverage gaps, English bias in the "universal" vocabulary, no pragmatics. |

**The synthesis:** meaning is not transmitted by structure or resemblance. It is transmitted by
*shared grounding* (which must be established) or *training* (which must be delivered). And even a
well-grounded, well-taught, demonstrably-better notation loses to an incumbent on adoption.

This project has spent years on **design** — the one part that isn't the bottleneck.

---

## Q1 — Does the Cure's check phase already exist as a standard?

**Largely yes: SHACL (W3C Shapes Constraint Language).**

- **Node shapes** constrain focus nodes; **property shapes** constrain values along property paths.
- Targets: all instances of a class, all subjects of a property, or explicit node lists.
- Constraint vocabulary: cardinality (`sh:minCount`/`sh:maxCount`), type (`sh:class`,
  `sh:datatype`), enumeration (`sh:in`), pattern (`sh:pattern`).
- **`sh:closed`** gives closed-world validation — a node may carry *only* declared properties.
- Output is a conformance report with per-violation diagnostics.
- It is being used as a type system: "Type Checking Program Code using SHACL," and shape
  containment is decidable via description-logic reasoning.

**But it cleanly splits the check phase in two, which is the useful part:**

| Check | Handled by | Status |
|---|---|---|
| Structural/shape constraints — cardinality, typing, closed-world, path patterns | **SHACL** (standard, tooled, shipped) | Adopt, don't build |
| **Logical consistency** — non-contradiction, entailment conflicts | Needs a **reasoner**, not shape validation | This is the real work |

SHACL cannot tell you that two assertions contradict each other. That requires DL reasoning — which
is where the ontology-repair machinery (pinpointing, justifications) already lives. **So the Cure's
check phase = SHACL for shape + DL reasoner for consistency, and neither needs inventing.**

## Q2 — What is drift, operationally?

**ML gives us detectors but not our definition — and the distinction matters.**

Concept drift is defined as *statistical properties of the target data changing over time*, with a
clean taxonomy: **sudden/abrupt, gradual, incremental, recurring**. Detection splits into statistical
change detection (DDM monitoring error rate; Page-Hinkley; CUSUM), window-based distribution
comparison (**ADWIN** — adaptive windowing that grows/shrinks to keep old and new statistically
consistent), and deep-learning methods. Reliability is actively contested ("Are Concept Drift
Detectors Reliable Alarming Systems?").

**The critical distinction:** concept drift is *the world changing under a fixed model*. The Cure's
semantic drift is *a representation corrupting relative to its own invariants*. These are different
phenomena — you can have either without the other.

**What transfers:** the detection machinery. If we can define a scalar "semantic health" quantity,
ADWIN/CUSUM/Page-Hinkley can monitor it off the shelf. **What doesn't transfer:** the definition. We
still have to say what quantity is drifting. The gap is sharpened, not closed.

## Q3 — Do iconic symbols communicate without training?

**No. This is empirically settled and it contradicts a standing project claim.**

ISO 3864 sets a 67% comprehension criterion for safety symbols. In testing, **only post-training
averages exceeded it.** Fire equipment and fire-action pictograms scored **45%**; prohibition
pictograms did best at 83.9%. Interpretation differences tracked **educational background more than
culture**. Explicit guidance: "prior training on the intended meaning greatly increases
effectiveness," and images "should align with the beliefs, society, and culture of the target
audience."

Consider the conditions: professional designers, ISO standardization, high-stakes safety context,
maximum motivation for self-evidence. **Iconicity still failed to bootstrap meaning.**

The wiki's Universal Symbology page rests its primitives entirely on the claim that "each feature's
shape resembles its meaning," with an untested "Content-Prime Test" as its only support. That claim
is now contradicted by the closest available evidence. Logged as **F-012**.

**This does not kill UWS.** It reclassifies it: UWS is a notation that must be *taught*, like
mathematics or music. That is completely respectable — and it means learnability, curriculum, and
training materials are core deliverables rather than afterthoughts.

## Q4 — What framework exists for evaluating a notation?

**Cognitive Dimensions of Notations** (Green, late 1980s; Green & Petre 1996), with real dimensions:

- **Viscosity** — resistance to change
- **Hidden dependencies** — links between entities that aren't visible; changing one has unexpected
  repercussions elsewhere
- **Premature commitment** — constraints on the order of doing things
- **Progressive evaluation** — can work-in-progress be checked at any point?
- **Closeness of mapping** — how well notation resembles the domain

The seminal case study analyzed **LabVIEW and Prograph — visual dataflow languages**, closely
analogous to UWS. LabVIEW scored excellently on closeness of mapping (it resembles electronic
schematics) but suffered **high viscosity**.

**Directly actionable:** this gives the 5-vs-6 decision real criteria instead of aesthetics, and
**hidden dependencies** is precisely the coreference problem the IR decision just addressed — a
notation where `?x` refers to a distant `*x` has hidden dependencies by construction, and the
framework says to make them visible.

## Q5 — What linguistic universals are actually supported?

**Very few, and the field is candid about it.**

Absolute universals hold without exception and are **"quite few in number"** (example: all languages
have pronouns). Most Greenbergian universals are **statistical tendencies with exceptions** — even
Universal 1 is "subjects *tend to* precede objects." Implicational universals ("VSO ⟹ prepositional")
are conditional, not absolute. And the methodological caution is explicit: absolute universals are
"very difficult to justify... a sample-size problem," with some linguists rejecting them outright on
grounds of **ethnocentrism** and insufficient coverage.

**Implication:** across ~7,000 human languages — one species, shared cognition, shared planet —
absolute universals are scarce and contested. The expectation of universals across *radically*
different minds is therefore far weaker than this project has assumed, and the ethnocentrism
critique applies with more force, not less.

## Q6 — Does sheaf theory formalize local-to-global consistency?

**Yes, and it is the best-fitting formalization of the Cure's problem found so far.**

Abramsky & Brandenburger's sheaf-theoretic treatment of non-locality and contextuality:

- Sheaves are "the most natural objects to study the extendability of local properties to global
  ones."
- Contextuality is precisely "a discrepancy between **local consistency and global inconsistency**."
- Contextuality and non-locality "correspond exactly to **obstructions to the existence of global
  sections**."
- **First Čech cohomology H¹ ≠ 0 provides a rigorous obstruction to global consistency**, with a
  linear-algebraic method for computing it.

**Why this fits the Cure so well:** the Cure's failure mode is a representation whose pieces are
each individually valid but which cannot be glued into a coherent whole. That is *exactly* "locally
consistent, no global section." And unlike the metric and discrete routes, this one comes with a
**computable obstruction** — H¹ both detects inconsistency and localizes it.

**There are now three candidate formalizations of repair/detection**, and they should be compared
head-to-head rather than assumed:

| Route | Detects | Repairs | Needs |
|---|---|---|---|
| Metric projection | distance from 𝒜 | nearest admissible point | 𝒜 convex (Gärdenfors route) |
| Ontology pinpointing | inconsistency | minimal axiom removal | DL reasoner |
| **Sheaf cohomology** | **H¹ ≠ 0, with localization** | (repair less developed) | sheaf structure over the IR |

## Q7 — Why are notations taught rather than self-evident?

Confirms Q3 from the design side, and adds a warning about adoption.

Notation is "inherently not self-evident" — stenographic music notation requires pre-memorizing
glyph-to-value associations. Guido d'Arezzo's staff lines succeeded because they *fixed pitch
positionally*, and were adopted over centuries.

**The warning:** a multi-decade international project to design better music notation concluded its
best result was **"only marginally better than existing notation,"** and efforts continue "lamenting
failure at adoption." Once a notation is standardized and taught, **collective knowledge investment
defeats improvement**, regardless of merit.

**For UWS this is the most important strategic finding in the cycle.** Being better is not
sufficient — and possibly not even the binding constraint. UNL had UN backing, 17 national centers,
and decades, and failed anyway.

---

## What changes

1. **Adopt SHACL for the shape-constraint half of the check phase.** Don't build it. Reserve effort
   for the logical-consistency half, which is where the actual difficulty is.
2. **Add sheaf cohomology as a third repair/detection candidate** and compare all three explicitly.
   It is the only route that both detects *and* localizes with a computable invariant.
3. **Retire the self-evident-iconicity claim** (F-012). Reframe UWS as a taught notation and make
   learnability a first-class deliverable — curriculum, training materials, comprehension testing
   against the ISO-style 67% bar.
4. **Use Cognitive Dimensions to decide 5-vs-6** and to audit UWS generally. Hidden dependencies in
   particular now has a name and a treatment.
5. **Soften "universal" language repo-wide.** Absolute universals are scarce even within human
   language; the field's own ethnocentrism critique applies here.
6. **Treat adoption as a first-class problem**, not a downstream consequence of quality. Every
   comparable effort that failed — UNL, music notation reform — failed at adoption, not design.
