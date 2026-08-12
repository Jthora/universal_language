# Prior-Art Audit — What Already Exists That We Were Treating as Open

**Date:** 2026-08-01
**Question asked:** before building new foundational material, is anything critical missing from our
foundational research?
**Answer: yes, substantially.** The two problems designated as the program's central open questions
are largely solved in fields we had not examined. One search also surfaced a **hard technical
constraint on the GIR design** that nothing in the repo or wiki accounts for, and which would have
become expensive to discover later.

---

## The landmine: confluence is undecidable for graph rewriting

**This is the most important finding in this audit, and it is a constraint, not a gap.**

The plan for `semantically_equal` was: define the algebraic laws as rewrite rules, normalize, compare
normal forms. For **term** rewriting this is a solved, textbook approach — Knuth-Bendix completion
takes a set of equations and produces a confluent, terminating rewrite system, and then *"if two
distinct terms have the same normal form, equality holds; otherwise it does not."* A decision
procedure, essentially for free.

**That guarantee does not transfer to graphs.** Plump's results:

- *"Joinability of critical pairs does not entail confluence"* for DPO graph rewriting — unlike term
  rewriting, where the critical pair lemma makes confluence checkable.
- *"Confluence of terminating DPO rewriting systems is, in general, undecidable."*

**GIR is a graph.** The composer is `Gir → Gir`. So the natural route to `semantically_equal` —
normalize and compare — is **not guaranteed to work over the current IR**, and the property that
would make it work is undecidable in general.

### Three ways out (a real design decision, now, before building)

1. **Restrict the normalizing fragment to a term/tree structure.** The repo already has an
   architecture note titled `graph-with-tree-spine`; if the *spine* is a genuine term, Knuth-Bendix
   applies to the spine and graph edges become annotations outside the rewrite system. This is
   probably the cheapest path and it exploits a design instinct already present.
2. **Adopt DPO-with-Interfaces (DPOI).** Confluence for terminating DPO rewriting *without*
   interfaces is undecidable, but **confluence for DPOI is decidable**, via string-diagram rewrite
   theory and critical-pair analysis. This is a more invasive reframing of the IR but comes with
   real theory attached.
3. **Abandon normalization for equivalence.** Decide equality by some other means (see the ontology
   route below). Weakest option for the algebra, but viable for the Cure.

**Nothing downstream should be built until this is chosen.** It determines the IR, the evaluator,
and whether the algebraic laws are checkable at all.

---

## Problem 1 — `semantically_equal`: standard solution shape exists

**Field:** term rewriting systems / equational logic.

Knuth-Bendix completion converts an equational theory into a confluent terminating rewrite system,
which decides equality by normal-form comparison. The wiki's UQPL spec already states its laws in
rewrite-rule form (`negate(negate(a)) → a`, `conjoin(a,a) → a`) — that is *already* the right input
format. This was never a novel research problem; it was an unrecognized instance of a standard one.

**Caveat, stated honestly:** completion is a semi-algorithm — *"there is no guarantee that the
process terminates."* It may fail to complete, and per the landmine above, it applies cleanly to
terms rather than graphs.

**Revised status:** `SEMANTIC-EQUALITY` drops from "novel research" to "instantiate known machinery,
subject to resolving the term-vs-graph decision."

---

## Problem 2 — The repair operator: an entire mature field, and it sidesteps our blocker

**Field:** ontology debugging and repair (description logics / OWL).

We formulated repair as metric projection `P : ℳ → 𝒜` and correctly identified that this is
ill-defined for non-convex 𝒜 (`FAILURES.md` F-009). The ontology-repair literature solves the same
problem **discretely rather than metrically**, which makes the convexity obstruction simply not
arise:

- **Pinpointing** — identify minimal sets of axioms whose removal restores coherence.
- **Justifications** — minimal subsets entailing a contradiction; repair nullifies every justification.
- **MIS** — minimal inconsistent sub-ontologies.
- **Minimal diagnoses** vs. **minimal-cardinality diagnoses** — two distinct, well-studied notions of
  "smallest repair," with algorithms for each.
- **Root unsatisfiable concepts** — distinguishing originating faults from propagated ones, so repair
  targets causes rather than symptoms.

The multivalued-projection problem becomes "choose among minimal diagnoses" — a known problem with
known strategies, not a geometric impossibility.

> **Naming note:** Gärdenfors appears here only as an author of AGM. His *conceptual spaces* work is a
> separate matter and carries a caveat — **convexity is relative to a choice of quality dimensions and
> metric**, so given freedom over the embedding many semantic regions can be made convex
> (`../surveys/research-register.md#E9-c`). Do not read one as support for the other.

**Companion field:** AGM belief revision (Alchourrón, Gärdenfors, Makinson) supplies the rationality
postulates for minimal change — contraction, expansion, revision — and has already been extended to
description logics and ontology evolution, including weakening-based revision operators that satisfy
generalized AGM postulates.

**Revised status:** F-009 stands as a critique of the *metric formulation*. The fix is not new
research — it is adopting the logical formulation these fields already built. **This is the single
biggest scope reduction available to the program.**

---

## Problem 3 — Conceptual Graphs: direct prior art we did not know about

**This is the gap most likely to cause embarrassment or wasted work.**

John Sowa's **Conceptual Graphs** are a graph representation for logic descending from Peirce's
existential graphs, with:

- **Full first-order expressive power**, plus extensions for metalanguage, modules, and namespaces
- A **formal semantics defined by ISO/IEC 24707 (Common Logic)**
- **CGIF** — a standardized linear interchange format
- A context notation supporting metalanguage and modality
- Explicitly stated design principles addressing *"logical, linguistic, and cognitive requirements"*

That is, in substance, what GIR is being built toward: a typed graph IR for meaning with a
serialization format. It was standardized in 2007.

**This does not invalidate UWS.** UWS's differentiator is the *iconic 2D visual layer* and placement
grammar — CGs are graph-structured but not principally a visual writing system. But the **IR layer
should interoperate with or borrow from CGIF rather than being reinvented**, and any claim of
novelty at the IR level needs to be stated relative to CGs.

**Convergence worth noting:** the blind rederivation (Phase 7) independently surfaced Peirce's
existential graphs as achieving full FOL with two sorts and two operations. Existential graphs →
conceptual graphs → ISO standard is a straight line. Three independent routes pointed at the same
place.

---

## Problem 4 — Visual logic already has soundness and completeness results

**Field:** diagrammatic reasoning (Euler → Venn → Peirce → Shin).

Directly relevant to UWS, and previously unexamined:

- The tradition's explicit goal was making diagram systems *"sound, complete, and logically
  equivalent to monadic predicate logic."* **Shin (1994) proved soundness and completeness** for a
  revised Venn-Peirce system.
- The literature formalizes exactly UWS's central design tension: each increase in expressive power
  (Venn's shading, Peirce's x's, o's, lines) came at *"a loss of visual clarity... because of the
  introduction of more arbitrary conventions."* Shin's contribution was gaining expressiveness
  *without* that severe loss.

**Implication:** "expressive power vs. visual clarity" is not an unmapped tradeoff UWS must discover
by trial. It's a studied one with documented moves and known failure patterns — and there is an
existing template for proving a visual notation sound and complete, which is precisely the kind of
result UWS could aim at that would be *real*.

---

## Problem 5 — UQPL's invariant preservation is standard practice

**Field:** refinement types / liquid types.

UQPL's core condition — admissible operators preserve invariants, `I(T̂x) = I(x)` — is exactly what
refinement types express: *"a base type with a predicate,"* with type checking generating
verification conditions discharged by SMT. Liquid types add automatic inference of refinements,
including loop invariants. LiquidHaskell operates over GHC's Core IR — i.e. this is proven practice
for *precisely* the "typed IR plus invariant checking" architecture UQPL needs.

**Revised status:** UQPL's invariant layer is an engineering problem with mature tooling, not a
research problem. Combined with the totality decision (`claims.yaml#UQPL-TOTALITY`), this is
buildable.

---

## Still unexamined (flagged, not researched)

Lower priority — none block the immediate build decision.

| Area | Why it might matter |
|---|---|
| **Concept drift detection (ML)** | The Cure's *Detect* phase. There is an established literature on detecting distributional/representational drift that we have not looked at. |
| **Blissymbolics / AAC learnability data** | Decades of real evidence on whether constructed iconic notations are learnable and by whom — the closest empirical analogue to UWS's learnability claims. |
| **Mechanistic interpretability / representation engineering** | The Cure's nearest neighbours in current AI safety. Needed to position the work and avoid duplicating activation-steering results. |
| **Remaining wiki pages** | ~25 UL-adjacent pages surfaced but never fetched (Semantic Operator, Semantic Noether Principle, Semantic Conservation Law, etc.). |

---

## What this changes

1. **A design decision is now blocking:** term-structured IR (Knuth-Bendix applies) vs. DPOI
   (confluence decidable) vs. no normalization. Choose before building. The existing
   `graph-with-tree-spine` note suggests option 1 was already half-intuited.
2. **The repair operator is no longer the program's hardest problem.** Reformulate discretely using
   pinpointing/justifications/minimal diagnoses and drop metric projection. Large scope reduction.
3. **`semantically_equal` is no longer novel research** — it's instantiating known machinery, gated
   on decision 1.
4. **Position relative to Conceptual Graphs before writing any new IR spec.** Borrow CGIF or state
   the difference explicitly.
5. **UWS has a real target:** a soundness/completeness result in the Shin tradition is achievable and
   would be a genuine contribution — far more defensible than any claim about cosmic necessity.
6. **UQPL's invariant layer is tooling work**, not theory.

**Net effect: the program got substantially smaller and more tractable.** Most of what was labeled
"open research" is "adopt an existing field." What remains genuinely open is narrow and concrete —
the IR decision above, and the empirical UWS work (legibility, learnability, expressive-failure
logging).
