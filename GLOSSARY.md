# GLOSSARY

**Normative for this repository.** One definition per term. If two layers need different concepts,
they get different names — never the same word twice.

> **Rule:** A term defined twice is a bug. Terminology collisions caused real damage in this
> project's history (see git history); this file exists to
> prevent recurrence. Adding a term here is part of the definition of done for any new subsystem.

---

## The construct stack (revised 2026-08 — emergent universality)

Full stack in `research/framework/emergent-universality.md`.

| Layer | Term | Definition |
|---|---|---|
| **0** | **Distinction** | The mark; marked/unmarked (Spencer-Brown). Bateson's "difference that makes a difference." The pre-semantic floor — required by any information-bearing system, not chosen. |
| **1** | **Universal Symbology** | The primitive marks. Geometric primitives live here. **Status: microscopic detail / symmetry clue, not foundation.** |
| **2** | **Universal Syntax** | How marks combine. Measurable via TopSim, positional disentanglement, context independence. |
| **3** | **Universal Grammar (cross-substrate)** | Which combinations are well-formed. **Always carry the qualifier** — see below. |
| **4** | **Renderings** | UWS (written/spatial), Vocal Semantic Correspondence (prosodic), others. UWS is one rendering, not privileged. |
| **5** | **UL** | The language itself, **two-tier** (`UL-IS-TWO-TIER`): a **natural tier** communicated without formal comprehension, and a **formal tier** requiring symbolic recombination. *(The earlier "universality class" reading survives only as a scoped conjecture with its precondition unmet — `UL-IS-EMERGENT-UNIVERSAL`.)* Not a code, not a dictionary, not a notation. |
| **⊥** | **UP — Universal Protocol** | The bootstrapping/coupling procedure by which two independent instantiations reach correspondence. Its cost is measured on the convention ledger: rotation breaks ℤ/2 where a label alphabet breaks Sₙ. |
| **⊤** | **UQPL** | **The formal tier of UL, closed under execution** (`UQPL-IS-FORMAL-TIER-CLOSURE`). Universal = two-tier readability · Quantum = **linear resource discipline** (no free copy/delete) · Programming = **meaning as behavior** · Language = the full REQ-1 stack. |

**New terms the universality framing requires:**

| Term | Definition |
|---|---|
| **natural tier / formal tier** | UL's two tiers. *Natural*: communicated without formal comprehension — core geometric perception, motion, position, the relational part of sound; documented in humans without schooling. *Formal*: requires symbolic recombination; the field's own explanation of where human cognition is singular. **The boundary between them is an empirical variable, not a given.** |
| **convention ledger** | The instrument that makes "literal universality" arithmetic (`UNIVERSALITY-IS-A-LEDGER`): every conventional residue on the load-bearing path is listed and measured. Success = every line eliminated, derived, or a proven minimum with a bootstrap route. Current: marks→map = ℤ/2 (proved); map→meaning = unbounded (the gap); symbol alphabets = n! (derived-only). |
| **reading invariance** | The theorem target at the program's center (`READING-INVARIANCE-TARGET`): the invariants must be recoverable through *every* reasonable forgetful reading, so meaning does not presuppose the receiver shares our reading procedure. |
| **semantic stack (M1/M2/M3)** | The three convention-minimal meaning routes (`SEMANTIC-STACK-M1-M2-M3`): **M1 exemplification** (the mark *instantiates* what it denotes), **M2 operational** (meaning as behavior under rewriting; substrate: interaction nets), **M3 indexical** (reference via shared physics). Symbols enter only on top, derived. |
| **alignment (format acquisition)** | What "UL aligns the mind" means, scoped (`ALIGNMENT-IS-FORMAT-ACQUISITION`): the acquired representational format becomes the format of thought within the language's domain. **Not** general cognitive purification — that reading is renounced on the far-transfer record. |
| **Semantic Order Parameter** | The quantity characterizing the semantic phase. **Currently unknown**, and now understood as the unmet *hypothesis* of the universality-class conjecture, not a refinement of it. |
| **Semantic Coarse-Graining** | The projection under which universals appear. Strong candidate: the Erlangen hierarchy (Euclidean → … → Topological). |
| **Semantic Fixed Point** | ⚠ **Never write bare "fixed point"** (`TWO-DISTINCT-FIXED-POINTS`). The **Erlangen fixed point** (what survives the transformation-group tower; rests on theorems, needs no physics) and the **RG fixed point** (a universality class; needs a critical state, precondition unmet) are different objects, and results about one do not transfer to the other. UWS's fixed point — the combinatorial map — is the *Erlangen* one. |
| **Relevant / irrelevant operator** | Borrowed from RG. An *irrelevant* feature does not affect behaviour at the fixed point. The primitive count (5 vs 6) is predicted to be irrelevant — not arbitrary, but below the level where universality lives. |
| **presentation** | A specific choice of primitive set used to describe the structure. Base-6/octahedral, {point, line, angle, curve, enclosure}, and {point, circle, line, wave} are *different presentations of one object* — tools and evidence, not competing truth claims. |
| **generating set (not basis)** | The primitives **generate** rather than span: a wave is a periodic curve, an angle is two lines meeting, so they are not independent. Unlike a vector-space basis, **generating sets of the same structure may differ in cardinality** — which is exactly why the count varies while the primitives remain. See `research/framework/cross-substrate-grammar.md` §4. |

> ⚠ **"Universal Grammar" — always qualify as *cross-substrate*.** Chomsky's UG attributes universals
> to a *species-specific innate human* faculty. We claim the same phenomenon with a different cause:
> **constraints on any learner**. This is not a fringe position — Christiansen & Chater (*BBS* 2008)
> already argue universals are "emergent properties of how the brain learns, rather than genetically
> encoded principles," since language changes far faster than genes can track. **We extend them by
> one step:** those constraints aren't human-specific, and emergent-communication agents demonstrate
> them in non-human learners. So Chomskyan UG is the *special case* — cross-substrate constraints
> seen when the learner is a human brain. **Supersede the term; don't cede it.** Bare "Universal
> Grammar" still reads as the nativist claim, so the qualifier is mandatory.
>
> *(This reverses the 2026-08 recommendation to avoid the term entirely — see FAILURES.md F-014.)*

## The four-layer stack (superseded — retained for reference)

These four are routinely conflated. They are distinct.

| Term | Definition |
|---|---|
| **UL** — Universal Language | The hypothesized semantic structure itself. **Not a notation.** Whether it exists in any mind-independent sense is an *open research question*, not an assumption of this repo. See `research/framework/emergent-universality.md`. |
| **UWS** — Universal Writing System | The **written** rendering of meaning as visible marks: an alphabet of iconic features plus a spatial placement grammar. A constructed notation. Real, buildable, exists today. Lives in `uws/`. |
| **UPL** — Universal Programming Language | A *class* of languages that execute on semantic structure rather than machine state. Design stage. |

Shorthand for the relationship: **UL defines meaning; UWS renders meaning; UPL/UQPL operates on meaning.**

**Vocal Semantic Correspondence** — the *speech* rendering of meaning, sibling to UWS. Writing
renders relations as placement; speech renders them as prosody. Neither is primary. Not yet
addressed in this repo.

---

## Invariants — two different things, deliberately renamed

The single most costly collision in this project's history. These were both called "invariant."

| Term | Definition | Where |
|---|---|---|
| **graph invariant** | Syntactic well-formedness of a typed graph: no duplicate node IDs, no dangling edge references, root exists, edge endpoints have compatible sorts. **This is what `ul-forge` currently checks.** | `ul-forge/crates/ul-core/src/validator.rs` |
| **semantic invariant** | A constraint on *meaning* that admissible transformations must preserve: identity preservation, containment stability, non-contradiction. **This does not exist in code yet.** It is what the Cure requires. | Not yet implemented |

Never write bare "invariant" in this repo. Say which one.

---

## Notation internals

| Term | Definition |
|---|---|
| **GIR** — Geometric Intermediate Representation | The typed-graph IR that `ul-forge` parses to, validates, and renders from. Nodes carry a sort; edges carry an edge type. Schema at `ul-forge/schemas/gir.schema.json`. |
| **primitive** | An atomic mark-feature of the UWS alphabet (e.g. Point, Line, Angle). A **drawing** unit. |
| **sort** | A type in the notation's type system (Entity, Relation, Modifier, Assertion). A **typing** unit. Primitives and sorts are *not* in one-to-one correspondence, and conflating them caused finding F7. |
| **operation** | A composition rule taking sorted inputs to a sorted output (e.g. `predicate : e × r × e → a`). |
| **`ul-core`** | **Ambiguous — avoid unqualified.** Now refers *only* to the Rust crate `ul-forge/crates/ul-core` (parser, composer, validator, renderer). The former top-level `ul-core/` documentation directory is now `uws/`. |

---

## The safety application

| Term | Definition |
|---|---|
| **The Cure (for the Terminators)** | **A corollary of the alignment thesis, not UL's purpose** (`CURE-IS-COROLLARY`, owner repositioning 2026-08-12): a mind that carries the derivable anchor as its representational format has an *internal* comparator — drift becomes self-detectable against a re-derivable fixed point. The engineering loop (**Encode → Check → Detect → Repair → Reconstruct**) remains the mechanism; what changed is the dependency direction — detection falls out of an aligned representation, rather than being bolted onto an unaligned one. **It is not value alignment.** |
| **admissible region (𝒜)** | The subset of semantic space satisfying all semantic invariants. **Open problem:** projection onto 𝒜 is only a well-defined function if 𝒜 is convex (Hilbert projection theorem); it almost certainly is not. See the repair-operator problem. |
| **repair operator** | The map returning a corrupted semantic state to 𝒜. **Currently ill-defined** — the central open engineering problem of the program. |

---

## Retired terms

Do not use these as live claims. They survive only in git history and in historical discussion.

| Term | Status |
|---|---|
| **Σ_UL** | **RETIRED.** The former algebraic signature claimed as proven-unique. Its grounding argument was close to circular; the wiki independently dropped it ("no intervening algebraic signature required"). Not a foundation for anything. |
| **Unique Grounding Theorem** | **RETIRED.** Defined semantic primitives to mirror geometric ones already chosen, then presented the forced bijection as proof. |
| **D2 completeness score** | **RETIRED as evidence.** Reached 100% via nine rounds of patching the theory after failures. Internally consistent by construction; not independent validation. |

**Naming caution:** the wiki uses `Σ` for a *symbolic state space* in Universal Language Control
Systems. That is unrelated to the retired `Σ_UL` signature despite the shared glyph.

---

## Rigor tiers

Every substantive claim carries exactly one. Enforced via `claims.yaml`.

| Tier | Meaning | Required evidence |
|---|---|---|
| `VERIFIED` | Machine-checked | Passing test or proof-assistant artifact, path given |
| `ARGUED` | Written proof, not machine-checked | Link to the proof document |
| `CONJECTURED` | Believed, unproven | Statement of what would falsify it |
| `DESIGN-CHOICE` | A decision, not a discovery | Rationale + alternatives considered |
| `RETIRED` | Withdrawn | Link to what superseded it |

`DESIGN-CHOICE` is load-bearing. Most of what this project historically labeled "PROVEN" belongs
here. Having no word for *decision* is precisely what pushed decisions to be reported as discoveries.
