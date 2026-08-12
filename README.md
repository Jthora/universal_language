# Universal Language · Universal Writing System · UQPL

**A constructed geometric notation, an AI-safety application built on it, and an open research
question about whether meaning has universal structure.**

[![DOI](https://zenodo.org/badge/951175055.svg)](https://doi.org/10.5281/zenodo.15050731)
[![License](https://img.shields.io/badge/License-see%20LICENSE-blue.svg)](LICENSE)
[![CI](https://github.com/Jthora/universal_language/actions/workflows/ci.yml/badge.svg)](https://github.com/Jthora/universal_language/actions/workflows/ci.yml)

---

## Status: rebuilt August 2026

This repository previously claimed a formal signature that was *proven* and fixed. **That claim is
retired**, along with the counts attached to it — an audit found its central argument close to
circular. What replaced it is stronger: the primitives are strata of plane-curve curvature space,
and the candidate inventories are **generating sets, not bases**, so their cardinalities differ by
presentation and carry no information about the object. "How many primitives" is a malformed
question, not an open one.

Superseded material is deleted from the working tree rather than archived in it — git history is the
archive. Full reasoning in `research/`.

**What this means practically:** the project is no longer trying to prove a metaphysical claim. It
is building a notation, an application, and a methodology — none of which need that claim to be
true. The work got smaller, more concrete, and considerably harder to knock over.

---

## The four layers

> **UL defines meaning · UWS renders meaning · UPL/UQPL operates on meaning**

| Layer | What it is | Status |
|---|---|---|
| **UL** | The hypothesized semantic structure itself. Not a notation. | Open question — `research/` |
| **UWS** | The written rendering: iconic mark-features plus a spatial placement grammar. | **Exists** — `uws/`, `ul-forge/` |
| **The Cure** | AI-safety application: detect and repair semantic drift via Encode → Check → Detect → Repair → Reconstruct. | Blocked on the repair operator |

These are routinely conflated. `GLOSSARY.md` is normative — read it before writing anything here.

---

## Why this exists

The origin was practical: designing a language for a species with no vocal tract, which forced the
question of what a non-phonetic, substrate-independent notation would have to look like.

The purpose it grew into is **The Cure for the Terminators** — an engineering program asking whether
adversarial failure modes in AI systems can be mitigated by structured semantic constraints:
representations that carry checkable invariants, and a repair operator that returns drifted states
to a valid region.

Crucially, **the Cure does not require UL to be real.** It requires computable invariants, a
convergent repair operator, and a legible surface for inspection. Those are engineering problems
with feedback signals, which is why the program can make progress where the metaphysical version
provably could not.

It also does not claim to solve alignment. Structural validity is not value alignment; a system can
be perfectly consistent and still optimize something terrible.

---

## Layout

```
├── GLOSSARY.md          Normative terminology. One definition per term. Read first.
├── RESEARCH-PROTOCOL.md Method: how not to get fooled by scope-dropped results. Read before searching.
├── FAILURES.md          Append-only. Failures are never edited into successes.
├── claims.yaml          The claim registry. Every substantive claim, tiered, with evidence.
│
├── spec/            What EXISTS. Present tense, no aspiration. (Currently empty — deliberately.)
├── design/          What is INTENDED. Every doc marked "not yet built."
│   └── uqpl/        UQPL specification drafts
├── research/        Open questions. Speculation permitted, must be labeled.
│
├── uws/             The Universal Writing System corpus
│   ├── symbology/ syntax/ grammar/ thesaurus/ lexicon/   The five siblings
│   ├── writing-system/            Reading/writing procedure + Writer's Companion
│   ├── formal-specification.md    Notation type system and operations
│   └── design-rationale.md        Why the notation is shaped this way
│
├── ul-forge/        Working implementation: parser, validator, composer, renderer,
│                    WASM bindings, web editor, MCP server. 135 tests.
│
```

---

## Where to start

**New here — human or AI?** Read **`PRIMER.md`** first. It is the comprehension document: what is
true, what was retired, and how to read a claim in this repo. ~15 minutes, and it prevents the most
common way of being confidently wrong about this project.

**About to research anything?** Read **`RESEARCH-PROTOCOL.md`**. Impossibility results in this
domain carry scope conditions that get dropped in transmission — a theorem about microscopic
compositionality read as a verdict on emergence, a theorem needing convexity read as a verdict on
repair. In every documented case here the counter-evidence was one search away. The protocol makes
that check automatic.

**To use the notation:** `uws/NAVIGATION.md` → `uws/writing-system/writers-companion.md` →
`ul-forge/`

**To understand the current state:** `research/reassessment-2026-08-purpose-anchored.md` →
`research/README.md`

**To contribute:** the two open problems that block everything else are both concrete and neither
needs the metaphysics resolved —

1. **The repair operator** (`FAILURES.md` F-009). Projection onto a non-convex admissible region is
   multivalued, so repair as currently specified is nondeterministic. Needs redesign.
2. **`semantically_equal`** (`claims.yaml#SEMANTIC-EQUALITY`). A decision procedure for whether two
   structures mean the same thing. Does not exist in any form; prerequisite for the Cure, for UQPL,
   and for every algebraic law in the notation.

---

## Practice

This repo's failure mode was **conventions without enforcement** — rigor labels nobody checked,
audits that never reached the artifacts, and claims asserted in prose that the code contradicted.
The rebuild replaces conventions with mechanisms:

- Every substantive claim is registered in `claims.yaml` with a tier and evidence.
- `VERIFIED` requires a test that exists and passes. `DESIGN-CHOICE` requires stated alternatives.
- Failures are append-only and immutable.
- A finding is closed only when the fix is applied, tested, and propagation-scanned — not when it's
  documented.

Full diagnosis and rebuild plan: git history.

---

**Author:** Jordan Traña ([Jthora](https://github.com/Jthora)) · Published as a defensive
publication to establish prior art.
