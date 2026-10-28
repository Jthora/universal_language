# What UL Enables — the capability surface

**What becomes possible if Universal Language is real, what it could be used for, and exactly
which of those is already available.**

This file answers the question a reader asks first: *what would I be able to do with this, and why
should I spend effort on it?* (`README.md` states the contract, `research/notes/052` the purpose,
`053` the deployment architecture.)

---

## How to read it

Every capability below is stated as *if this holds, you get that*, and each entry names:

- **Needs** — the claim it rests on, with its tier from `claims.yaml`. Trust the tier, not the prose.
- **Limit** — where the capability ends.

Tiers: `VERIFIED` machine-checked · `ARGUED` written proof · `CONJECTURED` believed with a stated
falsifier · `DESIGN-CHOICE` a decision, not a discovery.

**Some are available today, and marked so.** The rest are research targets, each tied to the claim
that would unlock it.

---

## 0. First: "universal" does not mean "for all humans"

Read it as *the universe*, and the whole capability surface comes into view.

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
amplitude, phase). Its combination orbits are fully enumerated; the open ones are marked
undetermined rather than filled, so the frontier is mapped cell by cell.

---

## 1. The surface, at a glance

| | Capability | Needs | Status |
|---|---|---|---|
| **1** | Structured notation for inter-agent exchange | nothing — it is built | **available now** |
| **2** | Diagnostic instrument: locate any notation's expressive cap | `UWS-AS-INSTRUMENT`, note `049` | **available now** |
| **3** | Measured convention cost for a channel | `UNIVERSALITY-IS-A-LEDGER`, ℤ/2 machine-checked | **available now, partial** |
| **0** | **An anchor that survives its source being discredited** | `AUTHORITY-ANCHORS-FAIL-TOTALLY` (ARGUED) + `PINNED-FRACTION-IS-A-CODE-RATE` (VERIFIED) | **available now, measured** |
| **4** | Drift detection against a re-derivable anchor | `DRIFT-DETECTION-REQUIRES-DISCRETENESS` (VERIFIED, scoped) | mechanism proved; build next |
| **5** | A quantitative integrity budget | `CAPACITY-RADIUS-TRADEOFF` (VERIFIED, scoped) | bound proved, unapplied |
| **6** | Communication with no prior contact | `LEDGER-CLOSURE-IS-UNIVERSE-SCOPE` (ARGUED) + ledger must close | open — the gap |
| **7** | A checkable convergence target for learned representations | a canonical chart | open |
| **8** | Interoperability with no standards body and no owner | parameter-freeness, partially measured | open |
| **9** | Execution where invalid transformations cannot run | `UQPL-IS-FORMAL-TIER-CLOSURE` (CONJECTURED) | substrate exists |
| **10** | Alignment by format acquisition | `ALIGNMENT-IS-FORMAT-ACQUISITION` (CONJECTURED) | **ready to run** |

---

## 2. Available now

These need no universality claim at all. If the whole research program failed tomorrow, these
would survive it.

### 1 · A structured notation for inter-agent exchange

A composable geometric notation with a working implementation: parser, validator, composer,
renderer, combinatorial-map reduction, WASM bindings, MCP server (`ul-forge/`; run `cargo test`). Usable
as an unambiguous intermediate representation between agents the way any well-designed schema is
usable.

**Needs** — nothing. It is code. **Limit** — this is engineering value, and it stands on its own.

### 2 · A diagnostic instrument for other notations

Given a map of the space, you can locate an existing notation on it and say precisely what it
cannot express. Note `049` ran this historically across writing systems, mathematical notation and
music notation, and the finding was sharp: **no notation is universal, and every cap is specific
and diagnosable.** The recurring cap is hard-coding a discretization of a continuum — one the
rotation-based system structurally lacks.

**Needs** `UWS-AS-INSTRUMENT`, note `049`. **Limit** — adoption is its own question (`F7-c`): the
music-notation evidence concerns displacing incumbents, and the cap it identifies is
representational.

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
  corruption **with no oracle and no source to trust**; `ruby tools/stats.rb` reports the pinned
  fraction live.
  Pinning is redundancy and redundancy costs capacity (`CAPACITY-RADIUS-TRADEOFF`), so this is a
  **code rate** — a higher number is not automatically better.
- **Globally** — `ruby tools/blast-radius.rb CLAIM-ID`, or `--census`. If this falls, what falls
  with it? `--census` maps which claims are load-bearing and which sit downstream of a CONJECTURED
  premise — the registry's dependency structure, made visible.

**Why it must pre-exist the collapse.** A void is never filled by nothing — it is filled by
whoever is standing there when it opens, at speed and at low standards. An anchor of this kind
cannot be built during the emergency; it has to already be checkable, or it arrives as one more
claimant indistinguishable from the thing that just failed. And the fallout set is only computable
if the dependency structure was recorded while things were calm.

**What it does for decisions.** It anchors truth and leaves values to its readers
(`UL-SUPPLIES-NO-VALUES`). Mathematics
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

## 3. Proved mechanism, ready to build

Verified mathematics, exact in the Fisher–Rao model (note `064`).

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
**combinatorial map** — discrete, reached by several independent lines, none aimed at the Cure. So the
notation's discreteness is not a convenience: it is the property without which no comparator can
exist at all.

**Needs** `DRIFT-DETECTION-REQUIRES-DISCRETENESS` (VERIFIED — `research/notes/064-universal-scope/verify.py`,
checks D and E). **Limit** — the quantitative form is exact in the Fisher–Rao model; the qualitative statement is
model-independent. Chentsov's theorem fixes the *metric* once a statistical manifold is given, so
the next step is the manifold itself. `semantically_equal` — the comparison primitive this runs
through — is the longest-standing engineering item, and a well-defined build.

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

**What this reframes.** Two internal Cure failure modes the companion wiki lists —
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

**Needs** a canonical chart — the open question behind `UL-IS-EMERGENT-UNIVERSAL`.
`TWO-DISTINCT-FIXED-POINTS` is normative here: name which fixed point you mean.

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
freeness is a property of the *format*; adoption is its own question (`F7-c`).

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

**Needs** `ALIGNMENT-IS-FORMAT-ACQUISITION` (CONJECTURED). **Limit** — this is format alignment: systems that share a
format can check each other's representations, and what each one values stays a separate axis
(orthogonality thesis).

**This is the first experiment that touches the purpose, and it is runnable today on humans and
on models.** If you are looking for the highest-value unblocked work in this
repository, it is this.

---

## 5. Where the capability surface ends

The edges, stated plainly so each capability above is used at its true size.

- **Representation, not objectives.** A shared format constrains how systems *represent*; what they
  *pursue* is a separate axis (orthogonality).
- **Deception is its own frontier.** Structural Equivalence Exploits — valid structure, inverted
  semantics — are untouched by any geometric budget. The best available answer is not geometric: a
  credential performable once is cheaper to fake than a practice that must be re-enacted
  continuously under conditions the performer does not control. The Cure as specified
  (Encode → Check → Detect → Repair → Reconstruct) is a **snapshot** check. **This is the sharpest
  unworked lead in either corpus.**
- **Not a unique notation.** Parameter-freeness of the classification says the map is shared; it
  does not single out a chart. Generating sets of one object differ in cardinality
  (`UWS-PRIMITIVE-COUNT`), which is why **no count of primitives, sorts, or operations is a claim
  of this project** and why any fixed count you find in older text is retired.
- **Reference is separate work.** A shared map fixes the common classification; what each symbol
  points at is fixed by the semantic layer.
- **Adoption is its own question.** Being derivable makes a notation checkable by anyone; whether
  minds take it up is decided separately (`F7-c`).
- **Conditional by construction.** The scope is *wherever the mathematics is the same*. If mathematics is
  contingent, the claim shrinks with it. That conditional is load-bearing and deliberate.

---

## 6. If you want to work on this

Ordered by *decisiveness per unit effort*, not by importance.

1. **Derive the operator inventory.** The marks are settled — a complete 3×2 product (note `065`).
   The operators are the frontier: each is attested on drawn sheets, and `concatenate` and `superpose_displaced` are now tied to geometry
   by contact (note `078`). Deriving the rest is the highest-value open item in the program
   (`OPERATOR-INVENTORY-IS-UNFORCED`).
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
6. **`semantically_equal`** — the longest-standing engineering item, and a well-defined build.

**Before you search the literature, read `RESEARCH-PROTOCOL.md`.** The documented failure mode here
is encountering a result that appears to settle a question negatively, believing it, and closing the
line of work — seven instances in a single session, each while the agent believed it was being
rigorous. In every case the counter-evidence was one search away. The rules there are mechanical for
that reason.

**Report results as they come out.** `FAILURES.md` is the append-only audit trail, and every
checkable result is registered, whichever way it comes out.

---

**Entry points:** `PRIMER.md` (comprehension) · `AGENTS.md` (navigation) · `FOR-AI.md` (if you are
a machine) · `seed/FIRST-CONTACT.md` (evaluating cold — verify, don't trust) · `claims.yaml` (what
is actually held) · `STATE-OF-PLAY.md` (where the program stands).
