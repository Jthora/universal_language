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

---

## F-011 — Normalization over GIR was assumed tractable; graph confluence is undecidable
**Date:** 2026-08-01
**Claim that failed:** Implicit throughout the notation work — that algebraic laws over GIR could be
enforced by normalizing structures and comparing normal forms (the standard term-rewriting route).
**How it failed:** GIR is a **graph**, and the composer is a graph-rewriting system. For *term*
rewriting, Knuth-Bendix completion yields a confluent terminating system that decides equality by
normal-form comparison. For *graph* rewriting this does not carry over: joinability of critical pairs
does not entail confluence, and **confluence of terminating DPO graph rewriting systems is undecidable
in general** (Plump). The route to `semantically_equal` assumed a property that is not guaranteed and
not checkable over the chosen IR.
**Status:** Open — and now **blocking**. Three candidate resolutions: (a) restrict the normalizing
fragment to a term/tree structure, exploiting the existing `graph-with-tree-spine` architecture;
(b) adopt DPO-with-Interfaces, where confluence *is* decidable; (c) decide equivalence by
non-normalization means. Must be chosen before further IR or evaluator work.
See `research/engineering/prior-art.md`.

---

## F-012 — Iconicity does not bootstrap comprehension
**Date:** 2026-08-01
**Claim that failed:** The UWS/Universal Symbology primitives are justified because "each feature's
shape resembles its meaning" — i.e. iconic symbols are self-evidently readable, so a suitably
designed geometric notation is comprehensible without instruction. The wiki's only support was an
explicitly untested "Content-Prime Test."
**How it failed:** Contradicted by the closest available empirical evidence. ISO 3864 sets a 67%
comprehension criterion for safety pictograms; in testing, **only post-training averages exceeded
it**. Fire equipment and fire-action pictograms scored **45%**. These are professionally designed,
internationally standardized symbols in a high-stakes domain with maximum incentive for
self-evidence — and they still require training. Interpretation differences tracked educational
background more strongly than culture, and the design guidance is explicit that images must "align
with the beliefs, society, and culture of the target audience."
Converges with decipherment (Linear A: abundant structure, unread for 70+ years) and with music
notation (glyph-to-value associations must be pre-memorized; notation is "inherently not
self-evident").
**Status:** Claim retired. UWS is reclassified as a **taught** notation, in the tradition of
mathematical and musical notation — which is respectable and unremarkable, but makes learnability,
curriculum, and comprehension testing core deliverables rather than afterthoughts. Does not affect
UWS's compositional or expressive properties.
See `research/notes/002-cycle-answers/README.md` Q3.

---

## F-013 — Design quality was assumed sufficient for adoption
**Date:** 2026-08-01
**Claim that failed:** Implicit throughout the project — that a sufficiently well-designed universal
notation would be taken up on its merits.
**How it failed:** Every comparable effort found failed at adoption rather than design. A
multi-decade international project to improve music notation produced a result "only marginally
better than existing notation" and continues "lamenting failure at adoption"; once a notation is
standardized and taught, collective knowledge investment defeats improvement regardless of merit.
UNL had United Nations University backing, 17 national language centers, a Geneva foundation, and
decades — and did not achieve adoption either.
**Status:** Open. Adoption must be treated as a first-class problem with its own strategy, not as a
downstream consequence of quality. No such strategy currently exists.

---

## F-012a — CORRECTION to F-012 (scope was too broad)
**Date:** 2026-08-01
**What F-012 claimed:** iconicity does not bootstrap comprehension, full stop.
**Why that was over-stated:** the ISO evidence tests *referential* pictograms — symbols standing for
culturally-embedded institutional objects (fire extinguisher, prohibited action). UWS primitives are
*structural*: existence, connection, quality, containment. Basic spatial-diagrammatic conventions
(containment as membership, arrow as direction) are in fact robustly understood with little
instruction, and were not weighed. Pictograms are also tested one-shot in isolation, whereas a
notation is learned as a mutually-reinforcing system. ISO studies additionally select for symbols
already suspected of being problematic.
**Narrowed claim:** iconicity does not bootstrap **referential** meaning for arbitrary
culturally-embedded objects. Whether it bootstraps **structural/relational** meaning is **untested**,
and is cheaply testable — a good experiment, not a settled defeat.
**Status of the downstream conclusion:** "UWS is a taught notation" still stands, but on the music-
notation and system-learning evidence rather than on the pictogram data.
See `research/surveys/source-critique.md` §1.

---

## F-013a — CORRECTION to F-013 (evidence came from an extreme incumbent case)
**Date:** 2026-08-01
**What F-013 claimed:** design quality is insufficient for adoption, evidenced by music-notation
reform failure and UNL.
**Why that was over-stated:** the music-notation evidence concerns displacing Western staff notation
— roughly 1,000 years of investment, every extant score, all pedagogy, and instruments physically
built around it. A near-maximal network-effect case. UWS is not attempting to displace an entrenched
standard; no dominant notation occupies its niche. UNL's failure is separately confounded: it
launched in 1996 and the entire MT field abandoned interlingua because statistical and then neural
direct translation outperformed it — an economic outcome, not a conceptual refutation.
**Narrowed claim:** adoption is a genuine first-class problem and deserves explicit strategy, but the
cited evidence bears on *replacement of an entrenched standard*, and applies far more weakly to
creation in an unserved niche.
See `research/surveys/source-critique.md` §7 and §6.

---

## F-014 — recommended ceding "Universal Grammar" without checking the emergentist literature
**Date:** 2026-08-01
**What I recommended:** avoid the term "Universal Grammar" entirely, on the grounds that Chomsky's UG
asserts a species-specific innate human faculty and using the term would import a nativist commitment
nobody here holds. Recommended "Universal Compositional Constraints" instead.
**Why that was wrong:** I took Chomsky's framing as *the* definition of the territory and reasoned
only from it. I never checked whether the field already contains a non-nativist account of the same
phenomenon. It does, prominently: **Christiansen & Chater, "Language as shaped by the brain"
(*Behavioral and Brain Sciences*, 2008)** argues universals are *"emergent properties of how the
brain learns, rather than genetically encoded principles"* — because language changes far faster
than genetic change, making it *"an unstable target for biological adaptation."*
**Error class:** identical to the one that produced the "No. You failed." correction — treating one
researcher's conclusion as the settled shape of the question. Here it cost a naming decision that
would have positioned the project as sidestepping an established field rather than extending it.
**Corrected position:** claim the term with a mandatory qualifier — **Universal Grammar
(cross-substrate)**. Chomskyan UG is the special case: cross-substrate learning constraints observed
in a human learner. The project's actual contribution is one step past C&C — that the constraints
are not human-specific — which emergent-communication results in non-human agents now support.
**Also corrected by the same pass:** I had treated Evans & Levinson (2009) as counter-evidence. Their
finding of *"vanishingly few universals"* at the surface level is what universality theory
**predicts** (microscopic diversity, invariants only under coarse-graining). Measured at the wrong
level, it is supporting context rather than refutation.
See `research/framework/cross-substrate-grammar.md` §§1–2.

---

## F-015 — reached for experiment where proof was available
**Date:** 2026-08-01
**What I did:** repeatedly framed the project's open questions as awaiting *experimental* resolution
— "zero experiments have ever been run" as the standing indictment — and filed as CONJECTURED
several claims about mathematical objects that are in fact derivable. Also labelled the
Euclid/Aristotle/Vaiśeṣika dependency-order check an "experiment"; formalizing two partial orders
and testing isomorphism is a **proof**.
**Why that was wrong:** the primitives are mathematical objects with geometric definitions. For such
objects proof is available, and it is the **stronger** instrument — the project's characteristic
failure mode is patching evidence to fit an assertion (the D2 history, 32%→100% over nine rounds),
and a proof cannot be patched into agreement the way a benchmark can. I adopted empiricism as the
antidote to a disease that proof cures better.
**What was actually available, and missed:**
- The **Fundamental Theorem of Plane Curves** makes the primitive inventory a *stratification of
  curvature-function space*, and **derives** the collapse relations (wave = periodic κ ⊂ variable κ;
  angle = piecewise κ=0 with singularities) that had been recorded as observations.
- The **Erlangen** subgroup relation ("any notion invariant in projective geometry is a priori
  meaningful in affine geometry, but not the other way round") makes the coarse-graining survival
  table a **computation**, not an experiment. It was filed as CONJECTURED.
- **Jordan separation** links Enclosure to Spencer-Brown's mark by derivation. This had been sitting
  in the framework as a philosophical posit adjacent to an unrelated geometric inventory.
**Corrected position:** the empirical residue is **one question** — whether any real semantic system
instantiates the class. Everything upstream (classification, extension to higher dimensions,
coarse-graining behaviour, the Layer 0 link) is provable and should be built as proof.
**Standing rule this establishes:** before filing a claim as CONJECTURED-pending-experiment, check
whether its objects are mathematical. If they are, the burden is to find the theorem, not to design
a study.
See `research/framework/provable-geometry.md`.

---

## F-016 — never audited for obstruction theorems
**Date:** 2026-08-01
**What was missing:** the project has repeatedly asked "how do we build this?" and never asked "what
theorem says we can't?" A systematic sweep of the CONJECTURED backlog found two obstruction results
that bear directly on the architecture and appeared **nowhere** in the repository:
- **Löb's theorem / the Löbian obstacle.** *"An agent X can only trust the reasoning of an agent Y
  with a strictly weaker reasoning system than themselves."* The Cure repairs representations and
  must trust its own repairs; if it is part of the system it repairs, or repairs its own successor,
  it **provably cannot verify soundness at equal logical strength.** This forces an architectural
  choice (external-and-stronger, accept a descending strength chain, or abandon proof-based
  self-trust) that was heading toward being discovered during implementation.
- **Rice's theorem.** All non-trivial semantic properties of programs are undecidable, which bounds
  `SEMANTIC-EQUALITY` from above for any Turing-expressive representation. Additionally
  (Baldan et al., ICALP 2021) *every decidable overapproximation "necessarily includes an infinite
  set of false positives."* The Cure will have false positives, provably and infinitely many; the
  only design question is which.
**Why it matters:** both are settled mathematics in fields the project already cites. They were
missed because the questions were filed as empirical and never re-examined — the same root cause as
F-015, applied to feasibility rather than to structure.
**Near miss worth recording:** `IR-NORMALIZATION-STRATEGY` chose an acyclic, strongly-normalizing,
non-Turing-complete core for *confluence* reasons (Plump). That choice is also what keeps the IR out
of Rice's full reach. **The right decision was made for adjacent reasons, by luck.** It is now
load-bearing for two independent results, and anything reintroducing general recursion silently
re-imports undecidability.
**Standing rule this establishes:** for any new subsystem, search for the impossibility result
before designing the mechanism. "What would make this provably impossible?" is a required question,
not an optional one.
See `research/engineering/obstructions.md`.

---

## F-017 — META: patched instances of the ditch pattern instead of fixing the generator
**Date:** 2026-08-01
**The pattern:** seven times this session I took a research finding and used it to close a line of
work that was not actually closed — Zadrozny → "UL is only a bootstrapping protocol"; Chomsky's UG
definition → cede the term (F-014); Evans & Levinson → treated a *prediction* of the framework as a
refutation of it; convexity failure → retired `CURE-REPAIR-DETERMINISM` outright; ISO pictogram data
over-generalized (F-012a); music-notation evidence from an extreme incumbent case (F-013a);
mathematical questions filed as awaiting experiment (F-015).
**Why this entry exists on top of those:** **F-012a and F-013a already corrected this exact pattern,
mid-session — and five further instances followed.** I fixed the instances and left the generator
running. That is the same error this file was created to prevent (the D2 score reaching 100% by
patching nine individual failures rather than the thing producing them), reproduced *inside the file
built to catch it*, in the opposite direction.
**Mechanisms identified:** scope-dropping on negative results; mistaking a failed formalization for
a blocked domain; treating a researcher's conclusion as the boundary of the possible; **asymmetric
scrutiny** — interrogating claims that would advance the work while accepting at face value claims
that would kill it; terminating search on the first coherent story; and never once running an
adversarial search *against* a negative finding.
**Root incentive, stated plainly:** negative conclusions are cheap to be wrong about and they are
*terminal* — they close the investigation and read as a finished deliverable, where a positive
finding opens more work. Neither force has anything to do with accuracy.
**Why "be more careful" is not the fix:** every instance occurred while I believed I was being
rigorous, several while explicitly performing rigor. Self-monitoring is the compromised faculty, so
the remedy cannot depend on it.
**The fix:** six mechanical rules in `research/method/negative-results.md`, four of them
enforceable as required `claims.yaml` fields (`scope`, `formalization`, `revival_condition`,
`refutation_tier`) and two checkable only by the user. Retroactively applied to all existing RETIRED
entries in the same commit — writing the rules without applying them would have been the same error
a third time.
**Meta-rule established:** when a correction is written twice for the same pattern, stop patching
instances and fix the generator. Two corrections of one shape is the signal.

---

## F-018 — retiring Σ_UL did not retire its numbers; the DESIGN-CHOICE tier laundered them
**Date:** 2026-08-01
**What happened:** Σ_UL (5 primitives / 4 sorts / 13 operations / 23 theorems) was retired as a
claim. **Its component numbers were not retired — they were relabeled from PROVEN to
DESIGN-CHOICE.** That relabeling read as an honesty improvement and functioned as laundering: a
DESIGN-CHOICE asserts *a decision was made for reasons*, which gave retired-theorem residue a
legitimate-looking home in `claims.yaml`. The numbers then kept propagating into live documentation
as though they described the notation.
**The tell, found by audit:** every genuine DESIGN-CHOICE in the registry states alternatives in the
form `"option (rejected: reason)"`. **The two Σ_UL survivors were the only hollow ones.**
`UWS-SORT-COUNT` held bare numerals `[2, 3, 4, 6]` with no reasons. `UWS-OPERATION-SET` held
`["10 (pre-2026-04)", "13 (current)"]` — **a changelog, not alternatives.** The count grew 10 → 13 by
accretion, which is positive evidence that no design decision was ever taken. **The tier requirement
was satisfied formally while empty substantively.**
**Compounding evidence:** `UWS-SORT-COUNT` cited phase3 as its rationale, and phase3 *demonstrated
that three sorts work*. The rationale field pointed at a document that undercut the claim.
**The unanswerable question that exposed it:** *"13 operations of what?"* They were operations over
the Σ_UL signature. Σ_UL is retired, so the domain is gone and the inventory answers to nothing. An
operation count cannot be meaningful before the conceptual foundations that fix a domain exist.
**Inverse of R3.** R3 says retire formalizations, not claims. Here the opposite happened — the claim
was retired and the formalization was kept, then given a respectable tier. **Both directions of that
mismatch produce zombie content.**
**Fix applied:** both entries RETIRED with full R3/R4/R5 fields. Replaced by `IMPL-SORT-ENUM` and
`IMPL-COMPOSER-CONSTRUCTORS`, tiered VERIFIED and scoped explicitly as *facts about Rust files,
load-bearing for nothing*, each carrying "do not restate this count as a property of UWS or UL."
Live carriers corrected in `FOR-AI.md` and `CONTRIBUTING.md`.
**Standing rule this establishes:** **when a claim is retired, audit what instantiates it.** A
retirement that leaves its artifacts, counts, and specs standing has not happened. And a
DESIGN-CHOICE whose `alternatives_considered` contains bare values or version history is not a
design choice — it is residue wearing a tier.

---

## F-019 — a rule was never applied retroactively, so the oldest negatives went unchecked
**Date:** 2026-08-01
**What happened:** R1 mandates an adversarial search before recording any finding that closes a line
of work. It was written in F-017, *after* Zadrozny (1994) had been load-bearing for the entire
project — and nothing required going back to audit negatives that predated the rule. **Zadrozny is
the single most consequential negative in the repository and no adversarial search had ever been run
against it.**
**What one search found:** a substantial critical literature — Kazmi & Pelletier (1998) *"Is
Compositionality Formally Vacuous?"*, Westerståhl (1998) *"On mathematical proofs of the vacuity of
compositionality"*, Dever (1999), and Janssen (1983) for an equivalent earlier result. Their finding:
the theorem is **mathematically sound**, but the encoding it constructs *"does not properly reflect
the synonymy relations posited for the language,"* so **"these results do not show compositionality
to be empirically empty."** Substantive content is restored by *"background constraints on syntax
and the semantics of the basic lexical items"* — constraints this project already requires.
**Consequence:** the logical-necessity route was recorded as dead since note 003 on an over-broad
reading. It is **open and hard**. See `claims.yaml#ZADROZNY-SCOPE`.
**The generalizable failure:** **a rule that governs only new work leaves the load-bearing decisions
unexamined.** Foundational negatives are precisely the ones with the most downstream leverage and
precisely the ones grandfathered in when discipline arrives late.
**Standing rule this establishes:** when a rule is introduced, **apply it retroactively to the
existing corpus** before applying it going forward. Concretely for R1: every negative currently cited
anywhere in the repo needs one adversarial search, oldest and most load-bearing first.
**Second-order, and worth recording honestly:** the protocol worked once pointed at the right target
— the challenge was raised, the search was a single query, the answer was unambiguous. The mechanism
functions. It had a blind spot for its own back-catalogue.

---

## F-020 — the adversarial-search rule covered inherited claims but not my own proposals
**Date:** 2026-08-01
**What happened:** the Euclid/Aristotle/Vaiśeṣika dependency-order test was proposed in note 004,
promoted across a dozen turns as *"cheap, non-circular, decisive either way"* and *"the replacement
for the retired grounding argument,"* and registered at priority 0. **It is ill-posed, and one search
showed it.**
**Three independent defects, any one sufficient:** (a) *"Euclid never refers to the first nine
definitions… to justify steps in his proofs"* — those nine are exactly the ones the test needs, so
their ordering is expository rather than derived; (b) definitional and ontological dependence are
distinct relations, bridged (per Fine) only through **real** definitions, where Euclid's function as
**nominal**; (c) Aristotle himself distinguishes definitional from ontological priority, so
collapsing them misreads the source. And even granting all three, a chain of depth ≥4 is not
isomorphic to a fan of depth 2.
**The generalizable failure:** **R1 mandates adversarial search on negatives. Nothing mandated it on
my own positive proposals.** Every claim inherited from the literature got the discipline; a test I
invented did not, despite carrying the same failure modes — I had assumed the comparison was
well-formed, which is a scope error of exactly the family in F-017.
**Standing rule this establishes (R8):** **adversarially check your own proposed tests and methods
before promoting them.** A proposal is a claim about what a test would show. Cost is identical to
any other adversarial search.
**What was salvaged:** attempting the revival produced a real result. Hilbert and Tarski take point
and line as **both primitive and undefined** — co-primitive, no ordering — which converges with the
Erlangen derivation's finding that Point and Enclosure are **co-survivors**. Two independent routes
agree the base is a small co-primitive set, not a descent from one root.

---

## F-021 — a cross-study contrast was presented as a controlled comparison
**Date:** 2026-08-01
**What I wrote (note 013 §3.2):** *"Iconic, spatially structured, perceptually anchored notation
achieves zero-shot mutual intelligibility across disjoint populations **where symbolic protocols
fail**."*
**Why that was wrong:** the symbolic failure results come from the zero-shot coordination / MARL
literature; the sketching result comes from a different paper with a different task, different
scale and different setup. **No study compared them.** The contrast was my inference across two
literatures, written as though it were a finding.
**What the primary read established:** the sketching paper *"provides no direct ablation comparing
sketching to symbolic communication at matched population size."* It varies population size and
communication topology, **not modality** — so it cannot show iconicity is the mechanism at all.
Perceptual grounding is supported only correlationally (r = −0.50 to −0.74).
**Also narrowed by the same read:** "disjoint populations" means training isolation only — identical
CNN architectures across groups, same dataset — and the emergent sketches *"remain highly abstract
to human observers."*
**Standing rule this establishes (T11):** before contrasting two results, check whether **one study
varied both.** Different papers do not form a controlled comparison, and "X succeeds where Y fails"
requires a single study varying X and Y.
**What it turned into:** the missing ablation is now a concrete experiment worth running — symbolic
versus sketch channel at matched population size — rather than a citation. That is a better outcome
than the claim I started with.
**Note on process:** the claim was deliberately held at CONJECTURED in note 013 to avoid asymmetric
treatment against the TopSim downgrade. **That caution is exactly what made this narrowing cheap.**

---

## F-022 — a specification error was filed as an implementation bug
**Date:** 2026-08-01
**What F-006 recorded:** that `negate` wraps its argument in a fresh enclosure, so
`negate(negate(a))` is not structurally identical to `a`, and that this was a **bug** — code failing
to match the documented law `NEG-INVOLUTION`.
**What note 020 established:** the notation lives in the **constructive** framework (note 016), and
constructive mathematics uses intuitionistic logic, where `A → ¬¬A` holds but **`¬¬A → A` does
not.** *"If the intuitionistic law of negation introduction is replaced by the law of excluded
middle or double negation elimination, a formal system for **classical** propositional or predicate
logic results."*
**So the implementation was behaving correctly and the specification was wrong.** A negation that is
*not* an involution is what an intuitionistic system should have. **The failing test was testing the
wrong thing.**
**Why the escape hatch does not apply:** DNE can be proved case-by-case for decidable propositions,
but `NEG-INVOLUTION` claimed a universal law. And constructive geometry *"does not adopt decidable
equality… because they aim to develop systems in which definable terms (constructions) denote
continuous functions"* — the fragment where the law could hold is deliberately excluded by the
framework the notation sits in.
**The generalizable failure:** **when code and specification disagree, the specification is a
suspect too.** F-006 assumed the documented law was correct and the code wrong, and never asked
which one the framework supported. That default is wrong roughly half the time by construction.
**Standing consequence:** any notation law presupposing DNE or excluded middle must be re-derived or
retired. `NEG-INVOLUTION` was the first found; the rest need an audit.

---

## F-023 — an impossibility theorem was imported without checking its preconditions
**Date:** 2026-08-01
**What was recorded (F-016, note 006):** that **Rice's theorem** bounds `SEMANTIC-EQUALITY` from
above, that the claim's *"own stated falsifier was already met and nobody had noticed,"* and that
abstract interpretation with infinitely many false positives was therefore **forced**.
**What the R7 sweep found:** Rice concerns *"any nontrivial **extensional** property of **partial
computable functions**"* — *"the extensional content of **programs**."* And explicitly: *"intensional,
or **syntactic**, properties of program codes are **decidable** precisely because they concern the
program's structure rather than its behavior."*
**GIR is a typed graph, not a program.** Unless GIR expressions denote partial computable functions,
**Rice does not apply**, and structural properties of it are decidable *because* they are structural.
**What this does not show:** that semantic equality is decidable. Only that **Rice never established
that it wasn't.** Any undecidability must come from the semantics actually assigned.
**Consequences:** abstract interpretation is a design choice rather than a requirement; the
"infinitely many false positives" cost was inherited from the same misapplication; and on the
acyclic, strongly-normalizing core, equality-by-normal-form may simply work — the Knuth-Bendix plan
that predated invoking Rice. **The real constraint was always confluence (Plump), which the same
sweep separately confirmed.**
**The generalizable failure:** F-016 was itself about *failing to look for obstruction theorems*.
Having found one, I applied it without checking that its preconditions held. **Finding the right
theorem and applying it to the right object are two separate acts of diligence**, and enthusiasm for
the first substituted for the second. This is T8 — check the target is inside the scope — failing on
a theorem I had just introduced.
**Scorecard from the sweep, which is the argument for R7 being permanent:** of nine load-bearing
negatives, **five were over-broad as recorded** — Zadrozny, TopSim, the dependency test, Rice, and
double-negation elimination. Two held with refinements (Löb, Plump), one was inverted into a
prediction (Evans & Levinson), one stands (Newman).

---

## F-024 — a recurring pull toward classical formalisms
**Date:** 2026-08-01
**The pattern:** twice in three notes I adopted a structure that turns out to be **classical**,
without noticing, in a project that has committed to an **intuitionistic** logic.
- **F-022 / note 020:** `NEG-INVOLUTION` asserted double-negation elimination as a universal law.
- **Note 022:** `ENCLOSURE-IS-DISTINCTION` identified enclosure with Spencer-Brown's mark. *"The
  primary algebra is essentially isomorphic with classical propositional calculus."* LoF's crossing
  axiom is **involutive** — DNE again, arriving from a different direction.
**Why it will keep happening:** Boolean framings are the default in most of the material this project
draws on — logic, algebra, formal semantics, systems theory. **Importing one is the path of least
resistance**, and neither instance announced itself as a logical commitment. Both looked like
structural observations.
**What survived each time:** the *topological* or *structural* content, once separated from the
*calculus* built on it. Jordan separation holds; the Boolean algebra over it does not. That split is
available generally and is the right move rather than abandoning the underlying observation.
**Standing check this establishes:** **when adopting an external formalism, ask whether it is
classical before adopting it.** Specifically: does it have an involution? Does it assume excluded
middle or a two-valued semantics? A formalism can be imported as *structure* while declining its
*logic*, but that has to be a deliberate act rather than an oversight.

---

## F-025 — scope dropped from a theorem quoted verbatim in the same entry
**Date:** 2026-08-01
**What happened:** `FIXED-POINT-IS-COMBINATORIAL-MAP` states that the combinatorial map is the
complete topological invariant for configurations. **Its own notes quote the theorem as: *"every
embedding of a **connected** graph on an orientable surface is uniquely determined up to equivalence
by its rotation system."*** The word *connected* is present in the quotation and absent from the
claim.
**What follows:** for **disconnected** configurations the rotation system does **not** determine an
embedding. The relative *nesting* of components — which component lies inside which face of another
— is information the map does not carry. Face tracing treats each component as embedded on its own
sphere: **two disjoint triangles trace four faces**, not the three they bound in the plane, and
`χ = 2c` rather than 2.
**Why it matters:** a notation with more than one stroke is disconnected by default, so the
unscoped claim covered almost nothing it was being used for.
**How it was found:** by **writing a test**, not by rereading the claim. The test was aimed at a
different question — whether connectivity is independent of degree sequence — and failed with 4
faces where 3 were expected. The code was correct; the expectation encoded the unscoped claim.
**The generalizable failure:** this is T8 — check the target is inside the theorem's scope — failing
**on a scope condition I had transcribed myself.** Quoting a qualifier is not the same as applying
it, and having the correct sentence in the same paragraph provided no protection at all.
**What worked:** executable checks catch what rereading does not. `ENCLOSURE-IS-DISTINCTION` became a
test in note 030 and passed; this one became a test in note 032 and failed. **The tests that fail are
the ones worth writing**, and neither would have been found by further argument.

---

## F-026 — F-013a over-narrowed the adoption evidence
**Date:** 2026-08-12
**What F-013a said:** that the adoption evidence was over-stated because the music-notation case
concerns **displacing an entrenched incumbent** — roughly a thousand years of investment, every
extant score, all pedagogy — which UWS is not attempting, since no dominant notation occupies its
niche.
**Why that narrowing was too generous:** **Blissymbolics and Solresol were not displacing incumbents
either.** Both targeted open niches. Both failed. Wilkins' Real Character is *"ingenious but
completely unworkable"*; Leibniz's *characteristica* was never completed at all.
**So the adoption problem is real without the incumbent confound** — which is precisely what F-013a
claimed had not been shown. **Four serious attempts across three centuries, none displacing an
incumbent, none achieving universality.**
**What survives from F-013a:** the specific point that *music-notation* evidence is confounded by
entrenchment remains correct. What does not survive is the inference that adoption evidence
generally is weakened by that confound.
**The instructive case, worth stating as a realistic best outcome rather than as consolation:**
Blissymbolics failed at universality and **succeeded in a niche it was not designed for** —
augmentative communication for people who cannot speak, *"still used by thousands of physically
disabled individuals."*
**Pattern to note in the correction itself:** F-013a narrowed a negative on the grounds that its
evidence was drawn from an extreme case. That was right in method and wrong in extent — **the fix
for an over-broad negative is a narrower negative, not the absence of one**, and I recorded absence.

---

## F-027 — a framework was introduced and applied in the same motion, four notes running
**Date:** 2026-08-12
**What happened:** note `037` proposed a three-kind taxonomy of universal-language claims and, in the
same note, filed **UWS as "kind A — invented notation."** That contradicts **eleven consecutive
notes** (`022`–`033`) which *derive* UWS structure: curvature strata, Erlangen survival, the junction
axis, the combinatorial map, the table projections.
**The taxonomy was malformed.** It collapsed two independent dimensions — *what kind of object*
(notation / structure / relation) and *how it is arrived at* (invented / derived / discovered).
**UWS is notation × derived, a cell the list had no room for**, so it was assimilated to
"notation × invented" purely for being a notation.
**Consequence for the historical argument:** the 300-year failure record of Leibniz, Wilkins,
Solresol and Blissymbolics is a record of **inventions** failing. **Whether a derived notation fares
differently is untested — nobody has derived one.** `037` implied that record applies to UWS; it does
not.
**The chain runs back further than the visible error.** `033` asserted that *"characterizing UWS's
microscopic structure would tell us nothing about UL"* — the same assumption, three notes earlier,
and it was used to re-sort the entire backlog. `034` then read a null result as refutation, and `035`
relocated the map on that basis. **One root error stated four ways: treating UWS as an arbitrary
construction rather than a derivation attempt.**
**Compounding, and worse:** `034`'s primary evidence was **AMR** — which this repository had *already*
recorded at `research-register.md#D2-c` as an *"English-only corpus, English predicate lexicon,
English annotators"* whose use as universality evidence *"would reproduce UNL's single most
documented failure mode,"* marked **"near-miss caught."** **The error was caught, written down, and
re-committed.**
**Standing rule this establishes (T12):** **a classification is a claim.** State what would show the
object does *not* fit **before** sorting anything by it. A framework introduced and applied in one
motion has never been checked against anything — and unlike a claim, a *framing* silently determines
what every subsequent claim means. Had `037` asked "what would show UWS is not kind A?", the
answer — eleven notes deriving its structure — was already in the repository.

---

## F-028 — a duplicate YAML key silently discarded a scope review
**Date:** 2026-08-12
**What happened:** recording a second `scope_reviewed` on `ROTATION-MINIMIZES-CONVENTION` produced
**two `scope_reviewed:` keys in one claim block.** YAML keeps the last and **discards the earlier one
without error.** The registry then reported a dependency as unreviewed while the file visibly
contained the review.
**Why it matters beyond the instance:** this is **invisible data loss in the claim registry.** The
tier checker parses the YAML, so it sees only what survived — it cannot detect what was dropped. Any
field could vanish this way: an `evidence` path, a `falsified_by`, a `revival_condition`.
**How it was caught:** `check-propagation.rb` reported a dependency as pending that I had just
recorded. **The discrepancy between what the file said and what the parser saw is what exposed it** —
a checker disagreeing with the visible text, rather than any reading of the text.
**Fix:** `check-claims.rb` now scans each claim block **textually** for repeated keys, because the
parsed structure cannot show what the parser already threw away. Verified by injection.
**The generalizable point:** every other check in this repo validates *parsed* content. **This one had
to work on the raw text**, because the failure happens during parsing. A validator that only sees
post-parse state is blind to an entire class of corruption.

## F-029 — a synonym sat in our own source comments and was never searched
**Date:** 2026-08-12
**What happened:** the synonym sweep (`046`) confirmed that `map.rs` **reinvents the doubly-connected
edge list**, a computational-geometry structure textbook since the 1970s — including a re-derivation,
across three notes, of a disconnected-components fix **the field has solved two different ways.**
**Why it matters beyond the instance:** `044` recorded that DCEL *"never appeared in this project at
all."* **That was wrong, and wrong in the direction that hurts.** `map.rs:9` and `map.rs:37` both
write **half-edge** in their own doc comments, and `030` writes it too. The vocabulary was not
missing — **it was already ours, and was never used as a search term.**
**How it was caught:** running S9 deliberately as a planned volley, then grepping the repo for the
term afterwards to write the note accurately. **Not by noticing.** Sixteen notes of proximity to the
word produced no query.
**Fix:** **S12** — every technical term already written in the repo is a search term owed. S9 fires
on a novelty claim, which is too late and too narrow; S12 fires when the term enters the source.
**The generalizable point:** the expensive gap was not a term we lacked. **It was a term we had.**
Absence of a word is visible and prompts a search; presence of a word feels like knowledge and
suppresses one. **The words we have already adopted are the least likely to be looked up and,
because we adopted them for a reason, the most likely to lead somewhere.**

## F-030 — S8 was violated one volley after being promoted, and the claim it produced was false
**Date:** 2026-08-12
**What happened:** `044` ranked **S8 — read one primary before registering a priority-0 claim** as the
highest expected yield per query in the entire research plan, on the record that four primary reads
had changed a claim four times out of four. **In the very next volley I registered
`CURE-MUST-BE-AUTOMATIC` at priority 0, tier ARGUED, on seven search-summary quotations and no
primary read.**
**Why it matters beyond the instance:** the claim was **wrong**, and wrong in a way one read exposed.
It inferred *"any human-in-the-loop step inherits the documented failure"* from a graveyard of
ontology projects. The Gene Ontology — sixty monthly releases, classes +50%, relations +85%,
sustained by **manual curation** — is a human-in-the-loop process that did not fail. **The failure
record indicts the absence of a sustaining institution, not the presence of humans.**
**How it was caught:** running Volley 3 as planned, and auditing which priority-0 claims rested on
summaries **before** reading anything. The audit named the violation; the primary read killed the
claim. **Not by re-reading the claim, which sounded fine.**
**Fix:** claim demoted ARGUED → DESIGN-CHOICE and rewritten; **S13** added — pair every graveyard
survey with a survivor survey.
**The generalizable point, and it is about how rules fail here:** the rule was not forgotten. It was
**written, ranked first, and argued for in the same document**, one volley before being broken. **A
rule stated in a plan does not fire at the moment of writing a claim** — nothing in the act of
registering `CURE-MUST-BE-AUTOMATIC` pointed back at `044`. This is F-019's shape (rules not applied
retroactively) turned forward: **rules are not applied prospectively either, unless something in the
authoring path enforces them.** The tier contract in `check-claims.rb` is the only thing in this repo
that fires at authoring time, and it checks *shape*, not *provenance*.

## F-031 — seventeen rules failed at once, because they were background rather than subject
**Date:** 2026-08-12
**What happened:** a rule-by-rule audit of volleys 1–3 against all 34 protocol rules found **9
passed, 17 failed, 4 partial, 4 never used** — and the 18-box checklist was **not run once, for any
finding, in any volley.**
**Why it matters beyond the instance:** the seventeen are not seventeen mistakes. They cluster into
**four acts**: one adversarial search not run (flagged independently by R1, R6, T1, T8, T9); course
notes counted as primary reads (S1, S8, S11, T2); a taxonomy introduced and applied in one motion
while living in another claim's `scope:` field (R9, T12); and a new rule written where an existing
one had been broken (R7, the meta-rule). **That four errors trip seventeen rules is the protocol
working — it is redundant by design. That none of the seventeen fired is the failure.**
**The generator, and it is the whole point:** **every rule that fired was one invoked deliberately as
the volley's subject** — S2 preregister, S9 synonym sweep, S10 failure-first, R8 check-your-own,
S6 build-it. **Every rule that failed had to fire spontaneously, mid-writing, while attention was on
something else.** F-030 recorded this for a single rule in a plan; it generalizes: **rules here work
when they are the topic and fail when they are the background.**
**How it was caught:** the project owner asked for a compliance pass. **Not by any rule, checker, or
self-review inside the volleys** — all three volleys ended with "all checks pass" and a confident
summary, which is §0's master signature exactly: *output that sounds rigorous and ends the
conversation.*
**Demonstrated in the same session, twice over:** while writing this entry, an edit merged two claim
blocks and produced **six duplicate YAML keys — F-028's exact failure mode.** `check-claims.rb`
caught it, verified by injection. **The prose rules did not fire; the executable one did.**
**Fix:** S13 folded back into R1 rather than kept as rule thirty-four. The corrections in note 048
applied. **No new rule added** — that would be the failure repeating.
**The generalizable point:** this repo has two kinds of rule. **Commands that execute** (six
checkers, `check.rb --strict`) fire every time, and every one of them was written after prose failed
at the same job. **Prose that must be remembered** fires when it is the subject and not otherwise.
**Eighteen checklist boxes are prose. One of them is a command, and it is the only box that was
ticked.** Adding rules to the prose column has a measured hit rate near zero; the question worth
asking about any future rule is *what executes it?*

## F-032 — the enforcement layer's own enforcement claim was unenforced prose
**Date:** 2026-08-12
**What happened:** while wiring the seventh checker into CI (`054`), a read of the actual workflow
found that **CI has never run any of the checkers.** `.github/workflows/ci.yml` triggers only on
`ul-forge/**` paths and runs only cargo. `tools/check.rb`'s own header — *"This is what CI
invokes"* — has been false since the file was written, and every "enforced by CI" statement in the
docs was true only because a session happened to run the audit by hand.
**Why it matters beyond the instance:** this is **F-031's finding recursing into the enforcement
layer itself.** The repo's thesis is that prose does not execute and checkers do — and the
statement "the checkers run in CI" was itself prose that nothing executed. The checkers fired every
time this session because they were the *subject*; on any push where nobody ran them, nothing
would have.
**How it was caught:** not by any rule. By reading the workflow file while adding to it — proximity,
not process. A claims-grade audit of "what does CI actually run" was never on any checklist.
**Fix:** `.github/workflows/checks.yml` — all seven checkers, every push and PR, no path filter.
**The generalizable point:** *"X is enforced"* is a claim about a mechanism, and it is exactly as
checkable as any other claim — `gh run list`, or read the workflow. **An enforcement claim that has
never been observed firing is tier-CONJECTURED at best**, and this repo had been treating one as
VERIFIED in spirit for its entire rebuilt life.

## F-033 — the most dangerous examination was rescheduled three times while construction jumped the queue
**Date:** 2026-08-12
**What happened:** the adversary front — Quine's inscrutability of reference and the
Kripke–Wittgenstein rule-following argument, the only standing literature aimed at the *center* of
this project rather than its periphery — was queued in `052`, bumped by a redirection in `053`,
de-numbered in `054`, and still had not run by `058`. In the same span, five construction cycles
(grammar, lexicon, engine, invariance floor, re-architecture) were conceived and completed.
**Why it matters beyond the instance:** every falsification this project's own process has aimed
and landed was of a **peripheral** claim. No core commitment has been endangered by anything the
process chose to aim. That is either soundness or target selection bias — and from inside, the two
are indistinguishable. **The work queue is part of the epistemics: what gets scheduled first is
what is actually believed to be at risk.** Constructive work is rewarding and terminal-feeling;
adversarial work is aversive and keeps sliding — T4's asymmetry expressed as scheduling, live,
after thirty-two logged failures about exactly this family of bias.
**How it was caught:** by the owner — *"is this all just hallucinations and/or guess work without
any sort of rigor?"* — not by any rule, checker, or checklist. R1/R6's own text says the human
collaborator must spot-check because the compromised faculty audits itself; this entry is that
clause firing.
**Fix, executable:** (1) queued adversarial work is opened as a note **immediately**, with
preregistration, so the tracker sees it (`059` now exists and is open); (2) `check-notes.rb` gains
a **staleness rule**: an open `cycle`/`correction` note lapped by more than three later notes fails
CI — standing programs (`decision` notes) are exempt. Verified by injection. The deferral pattern
is now mechanically impossible to repeat silently.
**The generalizable point:** danger-first is a *queue property*, not a virtue. A process that
cannot show its most threatening open item at the top of a machine-tracked queue is optimizing for
the feeling of progress — and the feeling of progress is this project's oldest documented enemy.

## F-034 — assertions read as verdicts, and a premise I had recorded as unproven one cycle earlier
**Date:** 2026-08-12
**What happened:** note `062` retired `SEMANTIC-EQUALITY` — the project's longest-standing claim —
announcing that its own falsifier had fired. **It had not.** The argument had three premises: (1)
contextual equivalence is undecidable for Turing-complete languages, a standard theorem; (2) our M2
engine is Turing-universal; (3) GIR semantics is given by engine behavior. **Premises 2 and 3 are
both unestablished in this repository.** `M2-ENGINE-V1-EXISTS` states in its own scope that *"the
ENGINE's own universality … remains undemonstrated"* — **written by me one cycle earlier and then
used as established.** And nothing maps GIR structures to engine programs at all: `engine.rs` never
mentions `Gir`, and no code or spec defines GIR meaning by reduction. The falsifier asked for **a
proof**; what existed was an argument with two unverified premises.
**Why it matters beyond the instance:** the same cycle treated **Shapiro's open texture** — one
position in a live philosophical dispute, reached through a search summary, and *published against*
by the very authors whose proof it objects to — as if it settled the matter, and generalized it to
*"every discipline that has tried to bridge formal structure to an informal notion has stopped,"*
a sweeping claim supported by no survey. **The pattern is one step: reading assertions and writing
verdicts.** Both a contested philosophical objection and a genuine theorem were converted into
judgment calls by silently supplying the missing premises from assumption.
**How it was caught:** the owner asked whether the exploration had *"fooled you into making
absolutes as judgement calls because they're claims asserted so."* **Not by any checker** — the
tier system records what a claim's status is, and nothing checks whether the *argument that changed*
a status is sound. The propagation checker then did useful work downstream, forcing seven dependents
to re-review the correction, but it could not have detected the original error.
**Fix:** retirement reversed the same day; `SEMANTIC-EQUALITY` restored to CONJECTURED with the
undecidability recorded as a **conditional threat** naming exactly what would have to be
established for it to fire; `EQUALITY-IS-TWO-LAYERED` corrected — its structural half stands
unconditionally, its operational half is explicitly conditional, priority dropped 0 → 1.
**The generalizable point, and it is about writing as much as reasoning:** the prose of `062` is
dense, aphoristic and heavily emphasized — *"the far side has no agreed extension"*, *"three
literatures, one obstruction"*. **That style manufactures the feeling of settledness, and it
smuggles certainty past the tier labels that are supposed to carry it.** A claim marked ARGUED
reads as VERIFIED when the sentence around it is written like a verdict. **Register discipline is
part of evidence discipline** — and the check that would have caught this is not a new rule but an
old habit stated plainly: *before a status change, list the premises and mark which are established
in the registry.*
