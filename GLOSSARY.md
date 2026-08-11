# GLOSSARY

**Normative for this repository.** One definition per term. If two layers need different concepts,
they get different names — never the same word twice.

> **Rule:** A term defined twice is a bug. Terminology collisions caused real damage in this
> project's history (see `research/postmortem-and-rebuild-2026-08.md`, FM4); this file exists to
> prevent recurrence. Adding a term here is part of the definition of done for any new subsystem.

---

## The four-layer stack

These four are routinely conflated. They are distinct.

| Term | Definition |
|---|---|
| **UL** — Universal Language | The hypothesized semantic structure itself. **Not a notation.** Whether it exists in any mind-independent sense is an *open research question*, not an assumption of this repo. See `research/emergence-investigation/`. |
| **UWS** — Universal Writing System | The **written** rendering of meaning as visible marks: an alphabet of iconic features plus a spatial placement grammar. A constructed notation. Real, buildable, exists today. Lives in `uws/`. |
| **UPL** — Universal Programming Language | A *class* of languages that execute on semantic structure rather than machine state. Design stage. |
| **UQPL** | A **subclass** of UPL exploring quantum-inspired semantic state spaces. **Not synonymous with UPL.** Spec exists; unimplemented. Lives in `design/uqpl/`. |

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
| **The Cure (for the Terminators)** | An AI-safety engineering program: detect and repair semantic drift in a system's representations via a closed loop — **Encode → Check → Detect → Repair → Reconstruct**. Enforces *local structural validity under defined semantic invariants*. **It is not alignment.** |
| **Terminator Syndrome** | The failure condition the Cure addresses. **Note: the wiki currently gives two unreconciled accounts** — (a) *trauma*: PTSD-like corruption in a conscious machine, cured by raising integration (φ); (b) *drift*: representational corruption, cured by geometric repair. These are different problems. This repo works on (b). See `research/deep-critique-2026-08-wiki-and-implementation.md` §W4. |
| **admissible region (𝒜)** | The subset of semantic space satisfying all semantic invariants. **Open problem:** projection onto 𝒜 is only a well-defined function if 𝒜 is convex (Hilbert projection theorem); it almost certainly is not. See the repair-operator problem. |
| **repair operator** | The map returning a corrupted semantic state to 𝒜. **Currently ill-defined** — the central open engineering problem of the program. |

---

## Retired terms

Do not use these as live claims. They appear only in `archive/` and in historical discussion.

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
