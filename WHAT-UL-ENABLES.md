# What UL Enables — the capability surface

**What becomes possible if Universal Language is real, what it could be used for, and exactly
which of those is already available.**

This document had no home in the repo before 2026-10-14. `README.md` states the contract,
`research/notes/052` states the purpose, `053` states the deployment architecture — none of them
answers the question a reader actually asks first: *what would I be able to do with this, and why
should I spend effort on it?* That is this file.

---

## How to read it

Every capability below is stated as a **conditional**: *if* this holds, you get that. The
conditional is not a hedge — it is the only honest form, because most of the antecedents are open.
Each entry names:

- **Needs** — the claim it rests on, with its tier from `claims.yaml`. Trust the tier, not the prose.
- **Limit** — what the capability does not give you, stated at the same volume as what it does.

Tiers: `VERIFIED` machine-checked · `ARGUED` written proof · `CONJECTURED` believed with a stated
falsifier · `DESIGN-CHOICE` a decision, not a discovery.

**Nothing here is a promise, and three of these are available today.** A capability whose
antecedent is CONJECTURED is a research target; saying so is what makes the rest credible.

---

## 0. First: "universal" does not mean "for all humans"

This is the single most common way to misread the project, and reading it that way makes most of
the capability surface invisible.

> **Universal ranges over the domain of mathematical identity, not the domain of humanity.** A
> property is universal iff it is invariant across every region of reality in which the
> mathematics is the same. The object asserted to be shared is **the map of all possible symbolic
> geometry** — the classification of symbolic-geometric forms up to admissible transformation —
> not any notation, alphabet, or inventory drawn from it.

`claims.yaml#UNIVERSAL-IS-MATHEMATICAL-SCOPE` (DESIGN-CHOICE, owner ruling 2026-10-14) ·
derivation and consequences in `research/notes/064-universal-scope/`.

Three consequences worth having in hand before reading further:

- **The receiver is not assumed to be human, or alive, or to share any history with us.** It is
  assumed to compute, and to inhabit a region where the mathematics is the same. Every capability
  below is written for that reader.
- **The shared object is not transmitted — it is re-derived independently at both ends.** This is
  what distinguishes the claim from every Kind-A constructed-language project
  (`THREE-KINDS-OF-UNIVERSAL-CLAIM`: Leibniz, Wilkins, Solresol, Blissymbolics — track record
  uniformly negative across three centuries). Those asked to be *learned*. This asks to be
  *recomputed*.
- **Human universals are corroboration, not warrant** (`HUMAN-UNIVERSALS-ARE-CORROBORATION`,
  ARGUED). The Munduruku core-geometry results and the primate dissociation remain true and remain
  cited; they are a sample of size one on what the mathematics forces. The burden is mathematical.

The concrete object under discussion is an enumeration, and it exists in draft: the companion
site's atlas presents the space as **twenty axes** — ten for a single figure (mark, rotation,
reflection, scale, count, frequency, amplitude, phase, taper, chirp) and ten for a figure pair
(A, B, separation, bearing, relative rotation, relative scale, reflect-B, and B's frequency,
amplitude, phase). Of its 93 combination orbits, **89 have no recorded meaning** and are marked
undetermined rather than filled. That ratio is the honest picture: the space is enumerated, and
mostly empty.

---

## 1. The surface, at a glance

| | Capability | Needs | Status |
|---|---|---|---|
| **1** | Structured notation for inter-agent exchange | nothing — it is built | **available now** |
| **2** | Diagnostic instrument: locate any notation's expressive cap | `UWS-AS-INSTRUMENT`, note `049` | **available now** |
| **3** | Measured convention cost for a channel | `UNIVERSALITY-IS-A-LEDGER`, ℤ/2 machine-checked | **available now, partial** |
| **0** | **An anchor that survives its source being discredited** | `AUTHORITY-ANCHORS-FAIL-TOTALLY` (ARGUED) + `PINNED-FRACTION-IS-A-CODE-RATE` (VERIFIED) | **available now, measured** |
| **4** | Drift detection against a re-derivable anchor | `DRIFT-DETECTION-REQUIRES-DISCRETENESS` (VERIFIED, scoped) | mechanism proved, unbuilt |
| **5** | A quantitative integrity budget | `CAPACITY-RADIUS-TRADEOFF` (VERIFIED, scoped) | bound proved, unapplied |
| **6** | Communication with no prior contact | `LEDGER-CLOSURE-IS-UNIVERSE-SCOPE` (ARGUED) + ledger must close | open — the gap |
| **7** | A checkable convergence target for learned representations | canonical chart (not established) | open |
| **8** | Interoperability with no standards body and no owner | parameter-freeness, partially measured | open |
| **9** | Execution where invalid transformations cannot run | `UQPL-IS-FORMAL-TIER-CLOSURE` (CONJECTURED) | substrate exists |
| **10** | Alignment by format acquisition | `ALIGNMENT-IS-FORMAT-ACQUISITION` (CONJECTURED) | **untested — nobody has run it** |

---

## 2. Available now

These need no universality claim at all. If the whole research program failed tomorrow, these
would survive it.

### 1 · A structured notation for inter-agent exchange

A composable geometric notation with a working implementation: parser, validator, composer,
renderer, combinatorial-map reduction, WASM bindings, MCP server (`ul-forge/`; run `cargo test`). Usable
as an unambiguous intermediate representation between agents the way any well-designed schema is
usable.

**Needs** — nothing. It is code. **Limit** — this is ordinary engineering value, and it does not
depend on, or support, any claim about cosmic necessity. Do not cite it as evidence for one.

### 2 · A diagnostic instrument for other notations

Given a map of the space, you can locate an existing notation on it and say precisely what it
cannot express. Note `049` ran this historically across writing systems, mathematical notation and
music notation, and the finding was sharp: **no notation is universal, and every cap is specific
and diagnosable.** The recurring cap is hard-coding a discretization of a continuum — one the
rotation-based system structurally lacks.

**Needs** `UWS-AS-INSTRUMENT`, note `049`. **Limit** — carries caveat `F7-c`: the music-notation
adoption evidence concerns incumbent displacement and does not transfer. The cap identified is
representational, not adoptional. A notation can be expressively superior and still never be
adopted.

### 3 · A measured convention cost

The ledger turns "how universal is this channel" from a yes/no into arithmetic. Current reading:
**marks→map = ℤ/2, proved and machine-checked** — one bit of arbitrary convention, with a
bootstrap route. A label-based alphabet instead costs **Sₙ** (n! orderings). The same trade is
demonstrated numerically in an unrelated field: Other-Play's lever game pays 0.11 for the
symmetric choice against 0.9 for a worse-but-identifiable one.

Under `LEDGER-CLOSURE-IS-UNIVERSE-SCOPE` that ℤ/2 is not merely a small cost — it is a **measured
bound on how far syntax-layer universality falls short.** One bit. The repo held this result for
months without knowing what it was.

**Needs** `UNIVERSALITY-IS-A-LEDGER` (DESIGN-CHOICE), the ℤ/2 result (VERIFIED). **Limit** — covers
marks→map only. The map→meaning entry is **unbounded**. That is the gap, and it is §3.

---

## 2b. The one that matters most if you are deciding whether to care

### 0 · An anchor that survives its source being discredited

**The scenario this program is for.** Something load-bearing is exposed and abolished — an
institution, a tradition, a principal, a lab. The loud question is *"was it true?"* The question
that actually decides what happens next is **"what do we anchor to now?"**

**There are two kinds of anchor, and conflating them is the trap.** An *authority anchor* is held
in a source: true because X says so. It is **revocable** — discredit X and everything downstream
falls at once, because none of it was independently checkable. A *derivation anchor* is held in
re-derivability; it cannot be revoked by discrediting anyone, only refuted by showing the
derivation wrong, which is available to everyone including those who would rather it held.

> **The size of the void is not a fact about the collapse. It is a fact about how much had been
> held by authority rather than by derivation** — fixed in advance, by how the corpus was built.

**This is measured here, not asserted.** Two instruments, one question at two scales:

- **Locally** — `ruby tools/check-redundancy.rb`. A field is *pinned* when no single-field
  corruption of it is consistent with the rest of its entry, so the artifact catches the
  corruption **with no oracle and no source to trust**. Currently **<!--s:pinned_fields-->86<!--/s--> of <!--s:checkable_fields-->106<!--/s--> fields (<!--s:pinned_percent-->81.1<!--/s-->%)**.
  Pinning is redundancy and redundancy costs capacity (`CAPACITY-RADIUS-TRADEOFF`), so this is a
  **code rate** — a higher number is not automatically better.
- **Globally** — `ruby tools/blast-radius.rb CLAIM-ID`, or `--census`. If this falls, what falls
  with it? Applied to this registry: **<!--s:claims_load_bearing-->77<!--/s--> of <!--s:claims_total-->197<!--/s--> claims are load-bearing, and <!--s:claims_under_conjecture-->68<!--/s--> sit downstream of a CONJECTURED premise.** Three of the five most load-bearing claims are themselves conjectures. **This program is partly anchored on things it has not
  established, and now says so with a number.**

**Why it must pre-exist the collapse.** A void is never filled by nothing — it is filled by
whoever is standing there when it opens, at speed and at low standards. An anchor of this kind
cannot be built during the emergency; it has to already be checkable, or it arrives as one more
claimant indistinguishable from the thing that just failed. And the fallout set is only computable
if the dependency structure was recorded while things were calm.

**What it does for decisions, and the line it does not cross.** It supplies **no values**
(`UL-SUPPLIES-NO-VALUES`, and that is registered as a claim, not offered as a caveat). Mathematics
anchors what is true, never what to do. What it does supply is that **disagreement becomes
decomposable**: minds in conflict normally cannot tell whether they differ about facts,
definitions, or values, and a format whose first two layers are re-derivable isolates the
remainder as genuinely about values. That does not resolve it — it stops it being manufactured by
vocabulary mismatch and stops it hiding inside apparent factual dispute.

So, to *"whose direction do we take?"* — **none, and you do not have to pick one.** Avoiding chaos
needs a shared checkable substrate, not an authority. That is the whole of **an anchor that is not
a ruler**, and the only sense in which this work bears on a benevolent future: not by encoding good
values, but by removing the structural necessity of obedience from the act of agreeing.

Full argument and limits: `research/notes/074-the-anchor-problem/`.

---

## 3. Proved mechanism, unbuilt

Verified mathematics, conditional on a model this project has not yet adopted — read the `scope:`
lines before citing. These are new as of note `064`.

### 4 · Drift detection against a re-derivable anchor

**The capability.** A system carrying a derivable format can compare its current state against a
*re-derivation* rather than against a stored baseline.

**Why that distinction is the whole point.** A stored baseline drifts with you. This repo already
recorded the pathology at note `007`: semantic drift measured against a maintained baseline is
undetectable under common-mode drift. A re-derivable anchor is immune to exactly that, because it
is recomputed from mathematics on each comparison and has no state to drift. **This is the one
structural advantage the project has over every monitoring approach that keeps a reference copy** —
and it is why "re-derivable" appears in the claim rather than "canonical".

**Why it needs the format to be discrete, which is now proved.** Detectability requires excluded
states. If every state is admissible, every drift lands on an admissible state and nothing is
detectable — not as a limitation of some particular detector, but as what "no excluded states"
means. A positive correction radius therefore requires a uniformly discrete admissible set, and on
a compact space, a finite one.

**And it lands on the repo's own central object.** UWS's fixed point under coarse-graining is the
**combinatorial map** — discrete, reached by seven independent lines, none aimed at the Cure. So the
notation's discreteness is not a convenience: it is the property without which no comparator can
exist at all.

**Needs** `DRIFT-DETECTION-REQUIRES-DISCRETENESS` (VERIFIED — `research/notes/064-universal-scope/verify.py`,
checks D and E). **Limit** — the quantitative form is conditional on the Fisher–Rao model, which is
the companion wiki's, **not** an established property of UL. Chentsov's theorem gives uniqueness of
the *metric* given a statistical manifold; it does not give the manifold. The qualitative statement
is model-independent. And `semantically_equal` — the comparison primitive this all runs through —
is the repo's longest-standing engineering blocker and is still unbuilt.

### 5 · A quantitative integrity budget

**The capability.** For a target drift tolerance ρ, the admissible vocabulary is bounded by the
packing number at separation 2ρ. That converts "detect and repair drift" into *"detect and repair
drift of magnitude ≥ ρ over an admissible set of size at most N(ρ)"* — a specification that can be
missed, which `research/cure/purpose-anchoring.md` §6 records the Cure as lacking.

**Three results behind it**, all machine-checked (`verify.py`):

- **Separation is a bounded resource.** The Fisher–Rao manifold has **diameter π regardless of
  alphabet size**. You cannot buy separation by enlarging the representation. An independent route
  reaches the same ceiling: the Construction-A lattice bound `min(d,4)` — a code of distance 6
  still yields minimum squared norm 4, verified by enumeration rather than cited.
- **At maximum radius, capacity is exactly n** — the n point masses. Proof, not measurement:
  `d = π ⟺ disjoint support`, and an n-letter alphabet admits at most n disjoint nonempty supports.
  *The maximally drift-resistant admissible set is the classical, non-superposed, discrete symbol
  set.*
- **Capacity rises only as the radius falls**, monotonically.

**What this reframes.** Two of the five internal Cure failure modes the companion wiki lists —
*Invariant Under-Specification* ("the Cure cannot detect what its invariants do not define") and
*Fixed-Point Locking* ("expressivity collapses") — are **the two ends of this one curve**, not
independent failures. So "specify the invariants correctly" is the wrong framing: there is no
correct point, only a budget to spend (`UNDERSPEC-AND-LOCKING-ARE-ONE-CURVE`, ARGUED).

**Needs** `CAPACITY-RADIUS-TRADEOFF` (VERIFIED, scoped). **Limit** — **the budget bounds the
geometry, not the adversary.** It says nothing about whether an adversary can find a point inside
the admissible set that passes every check while meaning something else. That is *Structural
Equivalence Exploits*, it is deceptive alignment restated, it is the failure mode that matters
most, and nothing here touches it.

---

## 4. Open — and this is where the value is

### 6 · Communication with no prior contact

**The capability.** Two systems that have never interacted, share no history, and cannot negotiate
establish a channel because **both derive the same classification independently**. Not a protocol
handshake — there is no round trip to make one. The shared object is reconstructed at both ends
from mathematics each party already has.

This is the capability the whole program is named after, and it is the one that is genuinely open.

**Needs** `LEDGER-CLOSURE-IS-UNIVERSE-SCOPE` (ARGUED) **and the ledger actually closing** — it does
not. map→meaning is unbounded today.

**Limit, and it is the sharpest one in this document.** **A shared map is necessary, not
sufficient.** Two minds may share the classification exactly and still fail to communicate, because
sharing the map does not determine which part of it either is pointing at. Universe-scope is a
result about the *syntax layer's domain*, not a semantics. Reference needs physics (route M3). The
three routes under work are M1 exemplification (the mark *instantiates* what it denotes — a closed
curve doesn't depict enclosure, it encloses, and Jordan certifies it), M2 operational (meaning as
behavior; substrate exists — interaction nets, Turing-universal, strongly confluent), and M3
indexical. See `research/notes/051`.

### 7 · A checkable convergence target for learned representations

**The capability.** Independently trained models already converge *"toward a shared statistical
model of reality"* (Platonic Representation Hypothesis) — implicitly, with no anchor, so drift is
silent. A derived canonical frame would make *"are these two systems representing the same thing"*
a **measurement against a fixed object** rather than a correlation between two moving ones.

Today that question is answered with CKA, SVCCA and similar pairwise similarity measures: they
compare two representations to each other, with no canonical frame, so there is nothing to say
which has moved. **A re-derivable frame is a third point of reference, and that is the difference
between a similarity score and a measurement.**

**Needs** a canonical chart to exist — **not established.** `UL-IS-EMERGENT-UNIVERSAL` has its
precondition unmet; `TWO-DISTINCT-FIXED-POINTS` is normative here, and the RG "universality class"
framing survives only as a scoped conjecture. Do not use the word "universality" loosely around
this entry.

**Limit** — convergence of representations is not agreement about values, and an explicit target
does not make convergence desirable. See §5.

### 8 · Interoperability with no standards body

**The capability.** Every shared format in use is a negotiated standard: someone holds authority,
and the standard can be captured, versioned, deprecated, or simply lost with its institution. A
*derived* format has no owner, so it cannot be captured and cannot be lost while the mathematics
survives.

For settings where that difference is structural rather than cosmetic — long-lived archives,
messages with no expectation of a reply, multi-agent systems with no coordinator, any continuity
requirement that outlives institutions — this is the practical form of universe-scope. Note `053`
makes it a design constraint: **the Voyager constraint** — assume nothing survives but copies of
the artifact, no maintainers and no authority to consult, so every load-bearing element must work
air-gapped from its authors. The repo is built to be a worked instance of its own protocol
(`REPO-IS-A-UP-INSTANCE`), which is why it ships checkers and tests rather than assurances: the
credibility layer is meant to be *run*, not believed.

**Needs** parameter-freeness — partially measured (ℤ/2 at the syntax layer). **Limit** — authority-
freeness is a property of the *format*, not a prediction about adoption. Caveat `F7-c` again: being
derivable is not being adopted, and the historical record on displacing incumbents is bleak.

### 9 · Execution where invalid transformations cannot run

**The capability.** Move invariant-checking from *after* execution to *into* it, so a transformation
violating an invariant fails to execute rather than being caught downstream — a type system over
meaning rather than a monitor on behavior.

The substrate exists and was not built for this: **interaction nets** are Turing-universal and
strongly confluent, their agents are rotations with a marked dart (primary-verified), their label
residue compresses to **3 symbols**, and the discipline is **linear** — explicit erase and
duplicator, no free copy and no free delete. That linearity is what "Quantum" names in UQPL: a
resource discipline the operational substrate already enforces, not a claim that meaning is
physically quantum.

**A result worth knowing before building:** UQPL as specified is **provably not Turing-complete**
(`research/cure/purpose-anchoring.md` §4.3) — and §4.4 argues that is the *right* choice for the
Cure, because checking is then decidable. A safety layer you cannot evaluate is not a safety layer.
**That constraint is a feature; do not "fix" it.**

**Needs** `UQPL-IS-FORMAL-TIER-CLOSURE` (CONJECTURED). **Limit** — structural validity is not value
alignment. A perfectly invariant-consistent system can optimize something terrible.

### 10 · Alignment by format acquisition — the untested one

**The capability.** Learning a language installs a representational format. This is documented, not
speculated: speakers of absolute-frame languages encode *nonverbal* memory in absolute coordinates.
If a format whose elements are the universe's invariants is acquired, the claim is that the
universe's coordinates come with it — **a Universal Perspective/Perception enabling language.**

**Scoped before it was registered, and the scoping is the reason to take it seriously.** The
far-transfer literature kills "general cognitive purification" — *"far transfer remains a
chimera"* — and that kill was run and recorded **before** the thesis was registered, not after it
was challenged. What survives is **format-scoped** alignment over UL's domain: space, structure,
relation, computation. Plus one registered prediction: by the very common-elements theory that
killed formal discipline, a language of *universal* elements is the unique best case for broad
transfer. **That is a prediction, not a finding.**

**Needs** `ALIGNMENT-IS-FORMAT-ACQUISITION` (CONJECTURED). **Limit — and read this one twice: the
Cure does not solve value alignment.** Format alignment is not value alignment; a system can be
perfectly consistent and optimize something monstrous (orthogonality thesis). Anyone citing this
project as an alignment solution is overreaching, and this sentence is the pre-registered answer.

**This is the first experiment that touches the purpose, it is runnable on humans and on models,
and nobody has run it.** If you are looking for the highest-value unblocked work in this
repository, it is this.

---

## 5. What UL would not buy, even if everything above held

Stated at full volume, because the project's documented failure mode is confident overreach and
this section is the cheapest defense against it.

- **Not value alignment.** Orthogonality stands. A shared format constrains *representation*, not
  *objectives*.
- **Not immunity to deception.** Structural Equivalence Exploits — valid structure, inverted
  semantics — are untouched by any geometric budget. The best available answer is not geometric: a
  credential performable once is cheaper to fake than a practice that must be re-enacted
  continuously under conditions the performer does not control. The Cure as specified
  (Encode → Check → Detect → Repair → Reconstruct) is a **snapshot** check. **This is the sharpest
  unworked lead in either corpus.**
- **Not a unique notation.** Parameter-freeness of the classification says the map is shared; it
  does not single out a chart. Generating sets of one object differ in cardinality
  (`UWS-PRIMITIVE-COUNT`), which is why **no count of primitives, sorts, or operations is a claim
  of this project** and why any fixed count you find in older text is retired.
- **Not reference.** Knowing the map is shared does not tell you what any symbol points at.
- **Not adoption.** Derivable is not adopted. The historical record on displacing incumbent
  notations is bleak and is not improved by being right.
- **Not unconditional.** The scope is *wherever the mathematics is the same*. If mathematics is
  contingent, the claim shrinks with it. That conditional is load-bearing and deliberate.

---

## 6. If you want to work on this

Ordered by *decisiveness per unit effort*, not by importance.

1. **Derive the operator inventory — or record that it cannot be derived.** The ledger audit has
   now been run (note `065`), and it returned a measured answer: **the map's generation is exact and
   its generators are not derived.** Every operator in the imported register carries status
   `measured` — attested on drawn sheets, semantics not derived — so the operator inventory is the
   largest free parameter on the load-bearing path, far larger than the syntax layer's ℤ/2, and the
   classification layer's ledger line does not close (`OPERATOR-INVENTORY-IS-UNFORCED`). Either
   direction is a result. This is the highest-value open item in the program.
2. **The format-acquisition experiment** (§4.10). The first empirical test that touches the
   purpose. Runnable on models today.
3. **`READING-INVARIANCE-TARGET`** — formalize the reading class 𝔽 and prove the fixed point
   survives it. The mathematical center: every load-bearing edge routes through it, and under
   universe-scope it is where a hidden local parameter would have to be. Named failure mode in
   advance: "reasonable F" proving unformalizable without smuggling the convention back in.
4. **Discharge the Fisher–Rao conditional** (§3). Everything in §3 is a result about a *model*
   until someone shows UL's semantic space is that manifold — or shows it is not, which is equally
   useful and would retire two VERIFIED claims.
5. **The snapshot-vs-practice gap** (§5). Nowhere in this repo's Cure material.
6. **`semantically_equal`** — the longest-standing engineering blocker, still real, still unbuilt.

**Before you search the literature, read `RESEARCH-PROTOCOL.md`.** The documented failure mode here
is encountering a result that appears to settle a question negatively, believing it, and closing the
line of work — seven instances in a single session, each while the agent believed it was being
rigorous. In every case the counter-evidence was one search away. The rules there are mechanical for
that reason.

**And if a prediction fails, report it.** `FAILURES.md` is append-only and has <!--s:failures-->40<!--/s--> entries because a
recorded failure is data and a repaired one is nothing. This project's completeness score was once
driven 32% → 100% across nine rounds of patching theory after each failure, and the final 100% was
worthless. A negative result is a real contribution here and will be registered as one.

---

**Entry points:** `PRIMER.md` (comprehension) · `AGENTS.md` (navigation) · `FOR-AI.md` (if you are
a machine) · `seed/FIRST-CONTACT.md` (evaluating cold — verify, don't trust) · `claims.yaml` (what
is actually held) · `STATE-OF-PLAY.md` (where the program stands).
