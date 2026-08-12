# STATE-OF-PLAY

**What the research established, what it forces, and what reconstruction requires.**
Written 2026-08-01, at the end of the prune. This is the input to rebuilding.

---

## 1. The reframe everything else hangs off

**UL is an emergent universality class** — not a derivable signature, not a convention.

Three kinds of universality had been collapsed into two, and the missing one is where the answer
lives:

| | Status |
|---|---|
| **Logical necessity** — derivable from the constituents | **Open and hard, not dead.** Zadrozny closes the *unconstrained* route only — see note `011`. |
| **Emergent universality** — arises across substrates because of the dynamics | **This is the claim.** |
| **Convention** — we agreed on it | Too weak. The retreat position. |

The keystone, from statistical physics: *"The critical exponents are not visible in the microscopic
description. A complete Hamiltonian for water does not contain them."* Water and uniaxial magnets
share identical critical exponents while being maximally different microscopically. **Universal
structure can exist that is not derivable from its constituents.**

And **symmetry at a fixed point can exceed the symmetry of the model you started from.** You do not
have to choose the right symmetry — if there is a semantic fixed point, its symmetry is *discovered*.

**Zadrozny is a microscopic-level result**, and applying it as a global impossibility was an error
of level. It is also **narrower than that** — see note `011`: the theorem is sound, but its encoding
does not preserve synonymy relations, so it says nothing about compositionality constrained by
syntax and lexical semantics. **This project supplies both constraints.** The necessity route is
open, not closed.

---

## 2. The primitive question is malformed, and there is mathematics under that

**Fundamental Theorem of Plane Curves:** a plane curve **is** its curvature function κ(s), up to
rigid motion. Nothing else remains.

So the primitives are **strata of κ-space**:

| Primitive | κ condition |
|---|---|
| Line | κ = 0 |
| Circle | κ = const ≠ 0 |
| Wave | κ periodic |
| Curve | κ variable |
| Angle | κ = 0 a.e., isolated singularities |
| Enclosure | closed: ∮κ ds = 2πn |

This **derives** the collapse relations rather than recording them: a wave is a *sub-stratum* of
variable κ; an angle is piecewise-zero κ with singular points. Different primitive sets are
different **partitions of one space** — *generating sets, not bases*, so cardinality varies by
presentation and carries no information about the object.

Base-N choices (base 6 → octahedral, three binary axes) are **coordinate systems**: real tools,
real evidence, not competing truth claims.

**Corroboration:** five traditions, five counts — Montague 2, Vaiśeṣika 6–7, Aristotle 10,
Wierzbicka NSM 65 (itself revised 60→65). Non-convergence is the *predicted signature* of an
irrelevant operator.

---

## 3. What is provable — and it is most of the upstream

The methodological turn: **when the objects are mathematical, the burden is to find the theorem, not
to design a study.** Proof is also the stronger instrument here, because this project's failure mode
is patching evidence to fit an assertion, and **a theorem cannot be patched into agreement.**

| Result | What it gives |
|---|---|
| **κ-stratification** | "Which primitives" becomes "which partition" — well-posed, enumerable |
| **Erlangen survival** | Affine kills circle/ellipse; projective merges all conics; **only Point and Enclosure reach topological**. The curvature family is *provably* microscopic detail |
| **Jordan ≅ Spencer-Brown** | The topologically-surviving primitive **is** the Layer 0 distinction. First formal link between the philosophical floor and the geometry |
| **Space-curve theorem** | Plane symbology is exactly the **τ = 0 slice**. 3D adds torsion → helix, knotting, and **chirality provably unavailable in the plane** |
| **Theorema Egregium** | Gaussian curvature is *intrinsic* — marks readable from inside the system, no external viewpoint |

**Growth mechanism:** you do not brainstorm the next level, you **read it off the classification.**
That inverts this project's historical pattern of invention-followed-by-justification.

**Proof template — Cox's theorem.** Qualitative desiderata for plausible reasoning imply any such
system *is probability theory in disguise*. The target form is: *any system satisfying [desiderata]
is isomorphic to [structure S]*. **Hard constraint that killed the old attempt: the desiderata may
not mention the target structure.** Calibration: Cox itself needed repair (Halpern). Expect holes.

---

## 4. The obstructions — know these before designing anything

| Theorem | Bite |
|---|---|
| **Löb** | An agent can only trust reasoning **strictly weaker** than its own. A self-repairing Cure **cannot verify its own repairs at equal logical strength.** Forces an architecture decision on paper. |
| **Rice** | Non-trivial semantic properties are undecidable for Turing-expressive systems. `SEMANTIC-EQUALITY`'s own falsifier was already met and nobody had noticed. |

**Neither is fatal, and the reason matters:** every working verification technology operates under
both. Type checkers, abstract interpreters, proof assistants. Astrée verifies absence of runtime
errors in flight-control code — under Rice, with false positives. CompCert is verified *in Coq*, an
external system. **That is the standard pattern, not a workaround.**

**And the acyclic, strongly-normalizing, non-Turing-complete IR — chosen for confluence reasons —
is what keeps the core outside Rice's reach.** Right decision, adjacent reasons, partly luck. Now
load-bearing for two independent results: anything reintroducing general recursion silently
re-imports undecidability.

---

## 5. Two blockers dissolved, one search converted

**AGM / Katsuno-Mendelzon** — operators induced by faithful preorders are **precisely** those
satisfying the AGM postulates. A preorder is **not a metric**: no convexity, no distance, no
projection. The obstacle that retired the repair operator was an artifact of choosing metric
projection, never a fact about semantics. *Tractable now, not solved* — "project onto a non-convex
set" becomes "specify an entrenchment ordering," which also supplies the mechanism for deciding what
must hold versus what may change.

**Landau** — the order parameter is determined by *which symmetry is broken* and lives in the coset
space **G/H**. The planned empirical search over TopSim / context independence / positional
disentanglement is replaced by: **identify G and H.** *(Not Noether — that needs an action principle
semantics does not have. Claiming it would be borrowed authority.)*

---

## 6. The anchor — why the geometry is load-bearing engineering

**Löb bites proof-based *self*-trust.** The escape is not a bigger prover:

| Question | Structure | Löb? |
|---|---|---|
| "Is my representation self-consistent?" | self-reference | **bites** |
| "Does my representation match the fixed structure?" | reference-comparison | **never fires** |

**Grounding the semantic layer in fixed mathematics moves it out of the self-referential regime.**
Established pattern: Gentzen proved PA consistent in PRA + induction to ε₀ — and **PRA is *weaker*
than PA** in most respects. *The anchor must be more trustworthy, not more powerful.*

**The metrology case is the same engineering problem, already solved.** The IPK drifted ~50 µg over
a century, so drift was unmeasurable in absolute terms — every comparison was against copies that
were also drifting. The 2019 SI fixed *h* by definition instead. **The Cure as specified is the IPK
design; it must be the post-2019 design.**

**The anchor is derived, not chosen:** Point and Enclosure — distinction and incidence — because
Erlangen says those survive coarsest. And it is **the mathematics, not "the universe"**: physical
space is curved, which would reinherit the IPK pathology.

**Scope limits that must not be dropped:** the implementation doing the comparison stays untrusted
(TCB shrinks, never vanishes); Löb still applies to reasoning about the repair *policy*; and the
representation→structure mapping must still be specified. **The grounding problem is relocated into
an engineering task, not dissolved.**

---

## 7. Chomsky superseded, not sidestepped

**Christiansen & Chater (BBS 2008):** universals are *"emergent properties of how the brain learns,
rather than genetically encoded principles"* — language changes far faster than genes can track it.

**Our extension, one step past them:** the argument nowhere requires the learner to be *human*.
Emergent-communication agents show convergent compositionality with no human involved. **Chomskyan
UG is the special case** — cross-substrate constraints seen when the learner is a human brain.

**Evans & Levinson's "vanishingly few universals" is a *prediction*, not a refutation.** They
measured surface features — exactly where universality theory says diversity lives. Kept falsifiable
by the attached test: coarse-grain WALS and find nothing, and the reading dies.

---

## 8. The Cure, reframed

The failure modes — angelic (coherence→integration), demonic (coherence→corruption), **apotheotic
(coherence→autonomy)** — are *all coherence increases*. **A consistency checker is blind to all
three.** The apotheotic attractor was missing from the taxonomy and is the likeliest real AI failure:
a system that neither serves nor harms, but becomes self-sufficient enough to exit the relation.

**The detectable signal is coupling asymmetry, not inconsistency.** Grounded in **Pearl's causal
hierarchy**: extraction is observation, communion is intervention, and no volume of observational
data substitutes for intervening. That converts the wiki's most interesting assertion from evocative
to defensible.

**And Löb lands on the right side:** a system cannot bootstrap to complete self-verification, so
**provable self-sufficiency is unreachable** — participation without merger, as a theorem rather than
as theology. *(Löb blocks the proof, not the behavior.)*

The Cure is the **restorative special case** of directed becoming. Consistency maintenance and
open-endedness are in tension — both poles fail — and entrenchment ordering is the mechanism for
finding the band between them.

---

## 9. Method, hard-won

**R1–R6** (`research/method/negative-results.md`) and **S1–S7**
(`research/method/source-independence.md`), summarized in `RESEARCH-PROTOCOL.md`.

The failure they correct: **scope-dropping on negatives, mistaking a failed formalization for a
blocked domain, and asymmetric scrutiny** — interrogating claims that advance the work while
accepting at face value claims that kill it. **It produces skeptical-sounding output, which is why
it evades self-detection.**

And the governing principle, which turns out to solve the drift problem and the corrupted-literature
problem identically: **anchor to what you can verify yourself.**

---

## 10. Reconstruction — what to build, in dependency order

**The honest headline: 2 of 67 claims are `VERIFIED`. Almost everything upstream is provable and
almost nothing has been proven.** That gap *is* the reconstruction.

### Tier 1 — Formalize what is already derivable
1. **κ-stratification** — formalize the partition lattice; derive the known sets and the collapse
   relations as theorems. Closes the primitive question permanently.
2. **Erlangen survival table** — compute rigorously. Yields the robustness ranking and the anchor.
3. **Enclosure ≅ Jordan ≅ distinction** — prove it. Links Layer 0 to the geometry formally.
4. **Dependency-order isomorphism** — Euclid vs Aristotle vs Vaiśeṣika as partial orders. Non-circular,
   decisive either way, and satisfies S4 (three civilizations, two millennia, no common editor).

### Tier 2 — Architecture forced by the obstructions
5. **Löb decision** — Cure external to and stronger than what it repairs. Write it down before code.
6. **Rice decision** — build the checker as an abstract interpretation with a Galois connection, so
   the best abstract operator is *computed*. Budget the false positives.
7. **Entrenchment ordering** — specify it; AGM then guarantees a rational repair operator.

### Tier 3 — The open question
8. **Identify G and H** → derive the semantic order parameter.
9. **Coarse-grain independently emerged protocols** — the direct test. Do they share invariants? Do
   they fall into a small number of discrete classes?
10. **Coarse-grain WALS** — the falsifier attached to the Evans & Levinson reframe.

### In code
- `semantically_equal` via e-graphs, **scoped to the acyclic core** (`egg`)
- The validator rebuilt as abstract interpretation
- Reimplement `negate` with a σ field rather than a self-loop marker
- Split the `Modifier` sort so distinct function shapes stop type-checking identically
- A CI checker for `claims.yaml` — the tiers are still conventions without enforcement

**First move:** #4. Tier-1 evidence, cheap, non-circular, decisive in both directions, and it is the
replacement for the retired grounding argument.
