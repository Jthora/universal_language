# Universal Language · Universal Writing System · UQPL

**A research program establishing a literal universal language: a language whose natural tier needs
no explanation, whose formal tier is derived rather than designed, and whose acquisition is the
alignment mechanism — with a working geometric notation, a machine-checked claim registry, and a
falsifiable contract.**

[![DOI](https://zenodo.org/badge/951175055.svg)](https://doi.org/10.5281/zenodo.15050731)
[![License](https://img.shields.io/badge/License-see%20LICENSE-blue.svg)](LICENSE)
[![CI](https://github.com/Jthora/universal_language/actions/workflows/ci.yml/badge.svg)](https://github.com/Jthora/universal_language/actions/workflows/ci.yml)

---

## The contract

This project is governed by four owner-set requirements, registered in `claims.yaml` and enforced by
checkers (`UL-MUST-BE-A-LANGUAGE`, `UNIVERSALITY-IS-A-LEDGER`):

1. **It must be a language** — syntax, compositional semantics, expressive adequacy,
   transmissibility. Retreating to notation-only, protocol-only, or class-only and declaring
   victory is excluded.
2. **Universality must be literal and quantified** — every convention the design needs goes on a
   **ledger**, measured. Allowed prior: geometry, causality, computation. Current ledger:
   marks→map = **ℤ/2, proved and machine-checked**; map→meaning = **the gap, made quantitative**;
   any symbol alphabet = n!, permitted only as derived.
3. **No-retreat falsifiability, symmetric** — a residue that cannot be eliminated, derived, or
   bootstrapped is recorded as failure. And "it can't be done" is itself a claim that gets the full
   negative-result discipline. Neither verdict is free.
4. **Baseline before UQPL** — work lands on the earliest unfinished link of
   UWS (syntax) → semantics → UL → UQPL.

An earlier form of this project claimed a proven fixed signature; that claim is retired and its
counts with it — git history is the archive. **The contract above is not a retreat from the goal.
It is the goal, held to a standard that can fail.**

---

## The five constructs

| | What it is | Status |
|---|---|---|
| **UL** | The language itself. **Two-tier**: a *natural* tier communicated without formal comprehension — core geometry is documented in humans with no schooling at all — and a *formal* tier requiring symbolic recombination, which is the field's own explanation of where human cognition is singular. | `UL-IS-TWO-TIER` (ARGUED) |
| **UWS** | The written rendering: geometric marks whose fixed point under coarse-graining is the **combinatorial map** — a complete invariant, honestly recorded as a DCEL reduct (prior art, 1970s). | **Exists** — `uws/`, `ul-forge/`, 354 tests |
| **UP** | The bootstrapping protocol: the minimum shared convention two independent parties need. Rotation breaks **ℤ/2** of arbitrary convention where a label alphabet breaks **Sₙ** — quantified, with the same trade demonstrated numerically in an unrelated field. | `ROTATION-MINIMIZES-CONVENTION` (ARGUED) |
| **UQPL** | **The formal tier of UL, closed under execution.** *Universal* = two-tier readability. *Quantum* = linear resource discipline — no free copy (no-cloning), no free delete — already enforced by the operational substrate. *Programming* = meaning as behavior. *Language* = the full stack. | `UQPL-IS-FORMAL-TIER-CLOSURE` (CONJECTURED, derived here; the wiki sketch is wet-clay input) |
| **The Cure** | **A corollary, not the purpose.** A mind that carries the derivable anchor as its representational format has an internal comparator — drift becomes self-detectable against a re-derivable fixed point. | `CURE-IS-COROLLARY` (DESIGN-CHOICE) |

`GLOSSARY.md` is normative — read it before writing anything here.

---

## Why this exists — the purpose-layer

**One property, two faces.**

- **Inter-mind:** communication across any gulf — including between independently evolved minds —
  as shared format **by derivation rather than convention**, bootstrapped from the natural tier.
- **Intra-mind:** **alignment by acquisition.** Learning a language installs a representational
  format (documented: speakers of absolute-frame languages encode *nonverbal* memory in absolute
  coordinates). A language whose format is the universe's own invariants installs the universe's
  coordinates — **a Universal Perspective/Perception enabling language.** Scoped honestly: the
  far-transfer literature kills "general purification," and we registered that kill *before*
  registering the thesis. What survives is format-scoped alignment over UL's domain — space,
  structure, relation, computation — and by the very common-elements theory that killed formal
  discipline, a language of universal elements is the unique best case for broad transfer. That is
  the thesis's registered prediction, not a finding.

In artificial minds the convergence already happens implicitly — independently trained models
converge *"toward a shared statistical model of reality"* (Platonic Representation Hypothesis).
**UL is the explicit, derivation-checked form of that convergence target — and the difference is
the Cure: implicit convergence has no anchor, so drift is silent; an explicit format carries one.**

**The mathematical center is one theorem target** — `READING-INVARIANCE-TARGET`: the invariants
must be recoverable through *every* reasonable reading, so that meaning does not presuppose the
receiver shares our reading procedure. Every load-bearing edge of the program routes through it.

---

## Layout

```
├── GLOSSARY.md          Normative terminology. One definition per term. Read first.
├── PRIMER.md            Comprehension: what is true, what was retired, how to read a claim.
├── STATE-OF-PLAY.md     Where the program stands now. Rewritten fresh, never patched.
├── RESEARCH-PROTOCOL.md Method: how not to get fooled by scope-dropped results.
├── FAILURES.md          Append-only. Failures are never edited into successes.
├── claims.yaml          The claim registry. Every substantive claim, tiered, with evidence.
├── tools/               Six checkers + check.rb. Prose does not execute; these do.
│
├── research/            The record. notes/001–052 are the working history;
│                        notes/050 is the standing program, notes/052 the purpose-layer.
├── uws/                 The Universal Writing System corpus
├── ul-forge/            Working implementation: parser, validator, composer, renderer,
│                        combinatorial map (map.rs), WASM bindings, web editor. 354 tests.
├── design/              What is INTENDED. Every doc marked "not yet built."
└── spec/                What EXISTS. Present tense. (Empty — deliberately.)
```

---

## Where to start

- **Where the program stands:** `STATE-OF-PLAY.md` — then `research/notes/050` (the program) and
  `research/notes/052` (what UQPL is and what it is for).
- **New here, human or AI:** `PRIMER.md` first. ~15 minutes; prevents the standard ways of being
  confidently wrong about this project.
- **About to research anything:** `RESEARCH-PROTOCOL.md`. In every documented case here, the
  counter-evidence to a line-closing result was one search away.
- **To use the notation:** `uws/NAVIGATION.md` → `ul-forge/`.

**The three open fronts that matter most** (see `STATE-OF-PLAY.md` §6 for the full list):

1. **READING-INVARIANCE** — formalize the reading class and prove the fixed point survives it.
   The theorem that would make syntax-layer universality literal.
2. **The M2 prototype** — interaction-net-style rules over `map.rs`. The operational substrate is
   Turing-universal, strongly confluent, and its programs are already our fixed-point object.
3. **The format-acquisition experiment** — does learning a geometric/topological notation
   restructure nonverbal representation? Testable on humans and on models; nobody has run it.

---

## Practice

This repo's historical failure mode was **conventions without enforcement**. The rebuild replaces
conventions with mechanisms:

- Every substantive claim is registered in `claims.yaml` with a tier and evidence; six CI checkers
  enforce tier contracts, link integrity, retired-content absence, caveat propagation, scope
  propagation, and notes conventions.
- `VERIFIED` requires a test that exists and passes. `CONJECTURED` requires a stated falsifier.
  `DESIGN-CHOICE` requires stated alternatives.
- `FAILURES.md` is append-only and immutable — 31 entries and counting, because a recorded failure
  is data and a repaired one is nothing.
- A finding is closed only when the fix is applied, tested, and propagation-scanned.

---

**Author:** Jordan Traña ([Jthora](https://github.com/Jthora)) · Published as a defensive
publication to establish prior art.
