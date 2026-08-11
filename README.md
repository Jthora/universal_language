# Universal Language (UL) & the Universal Writing System (UWS)

**A constructed geometric notation system, and an open, actively-investigated question about
whether any deeper universal semantic structure is real.**

[![DOI](https://zenodo.org/badge/951175055.svg)](https://doi.org/10.5281/zenodo.15050731)
[![License](https://img.shields.io/badge/License-see%20LICENSE-blue.svg)](LICENSE)
[![CI](https://github.com/Jthora/universal_language/actions/workflows/ci.yml/badge.svg)](https://github.com/Jthora/universal_language/actions/workflows/ci.yml)

---

## Status (2026-08-01)

**This README replaces an earlier version that presented this project's formal signature (Σ_UL) as
proven — "5 primitives, 4 sorts, 13 operations, unique up to isomorphism."** A direct audit of the
proofs found the central "Unique Grounding Theorem" close to circular, and a from-scratch
rederivation using standard 80-year-old model theory (Montague grammar) gives a smaller, different
minimal answer than the claimed counts. See `docs/planning/audits/wiki-comparison-2026-08.md` for
the full audit and `docs/planning/emergence-investigation/PLAN.md` for what replaced it.

The project is now explicitly split into two things that don't need each other to be valid:

## 1. The Universal Writing System (UWS) — a constructed notation, real and usable

UWS is a geometric visual notation, originally designed for a practical problem: encoding meaning
for beings without a human vocal tract (see `docs/planning/emergence-investigation/` for the origin
story). It decomposes symbols into five geometric primitives (Point, Line, Angle, Curve, Enclosure),
each composable via 13 defined operations, into a complete writing system with a 42-entry canonical
lexicon and a working software implementation (parser, renderer, composer, WASM bindings — see
`ul-forge/`).

**This does not require, and no longer claims, proof that it is the unique or necessary structure of
meaning.** It stands on the same footing as other constructed notations — Blissymbolics, Frege's
*Begriffsschrift*, Freudenthal's *Lincos* — judged on composability, learnability, and expressiveness.
See `foundations/universal-language-derivation.md` (design rationale) and `ul-core/` (the full
writing system specification) to learn and use it.

## 2. The Emergence Investigation — an open, falsifiable research question

Separately: is there a real, mind-independent (or at least life-independent) universal semantic
structure — the way the natural numbers or physical symmetry groups are real, independent of any
particular notation for them? This is **not assumed** here. A six-phase investigation with
pre-stated falsification criteria ran to completion on 2026-08-01 —
see `docs/planning/emergence-investigation/phase6-synthesis-and-verdict.md` for the full,
consolidated verdict. Headline:

- **Not established, not refuted**, for the strong claim. What survives: compositional
  generativity is real and threshold-gated — broadly available in bounded form across life
  (honeybee waggle-dance geometric encoding, Bengalese finch hierarchical song), contested but real
  in full (negation/quantification-capable) form in exactly two non-human cases, and robustly,
  repeatedly reached by humans given only minimal social conditions and no inherited language
  model (Nicaraguan Sign Language).
- **The specific primitive count (5, or the wiki's 6) is confirmed underdetermined**, not
  "wrong in favor of a different number." Standard model theory shows 2 base types suffice; no
  independently convergent tradition matches 4, 5, or 6 (Aristotle: 10 categories; Vaiśeṣika: 6–7).
  The count is a genuine notation-design choice, exactly matching the instinct that it felt
  arbitrary.
- **A real convergent operational core exists**: subject-predicate combination, negation, and
  quantification, independently evidenced by ancient Greek and Indian logical traditions that
  developed without contact with each other.
- **Two genuinely open empirical questions have ready-to-run designs** pending compute/API budget
  (cross-substrate representational alignment; causal efficacy of UL-structured text on LLM
  outputs) — see `docs/planning/emergence-investigation/PLAN.md` Phases 4–5.

If you want to help close what's still open rather than take either "yes" or "no" on faith, start
at `docs/planning/emergence-investigation/phase6-synthesis-and-verdict.md`, then
`docs/planning/emergence-investigation/PLAN.md` for the full phase-by-phase record.

---

## Repository Structure

```
universal_language/
│
├── README.md                    # You are here
├── AGENTS.md                    # AI agent navigation
├── CONTRIBUTING.md              # How to contribute
├── LICENSE
│
├── foundations/                  # UWS design rationale + reusable formal apparatus
│   ├── universal-language-derivation.md  # UWS design rationale (reframed 2026-08)
│   ├── formal-foundations.md    # Notation's formal operation spec (reframed 2026-08)
│   ├── formal-operations.md     # Set-theoretic definitions of the notation's operations
│   ├── independent-derivation.md # Raw material for the Emergence Investigation
│   └── montague-homomorphism.md # Raw material for the Emergence Investigation
│
├── ul-core/                     # UWS writing system specification (unaffected by the above)
│   ├── NAVIGATION.md
│   ├── SYNTHESIS.md
│   ├── CRITIQUE.md              # Honest audit log — the record that surfaced the core issue
│   ├── writing-system/          # Complete reading/writing procedure + Writer's Companion
│   ├── symbology/ syntax/ grammar/ thesaurus/ lexicon/  # The five writing-system siblings
│   └── uqpl/                    # Universal Query & Programming Language (draft)
│
├── whitepaper/                  # Original whitepaper (self-labeled historical)
│
├── frontier/                    # Exploratory math, kept as Emergence Investigation raw material
│   ├── methodology.md
│   ├── causal-efficacy-protocol.md  # Target of Investigation Phase 5 (needs rehabilitation)
│   └── expedition-one/ expedition-two/
│
├── experiments/                 # Experimental infrastructure — Investigation Phase 5 target
│
├── docs/
│   ├── planning/emergence-investigation/  # The active research plan — START HERE for the open question
│   ├── planning/audits/         # Historical process record (superseded, not current status)
│   ├── ul-forge-v1/ distribution/ learning/  # UWS software documentation
│
├── ul-forge/                    # Working software implementation of the UWS notation
│
└── archive/superseded-2026-08/  # Everything retired in the August 2026 split, preserved in full
```

---

## Reading Order

### If you want to learn/use the writing system (UWS)
1. `foundations/universal-language-derivation.md` — design rationale
2. `ul-core/NAVIGATION.md` — quick-start paths
3. `ul-core/writing-system/writers-companion.md` — practical pen-and-paper guide
4. `docs/learning/quickstart.md` — "Learn UL in 15 Minutes"
5. `ul-forge/` — software implementation (parser, renderer, web editor, VS Code extension)

### If you want to work on the open scientific question
1. `docs/planning/audits/wiki-comparison-2026-08.md` — the audit that started this
2. `docs/planning/emergence-investigation/PLAN.md` — the six-phase falsifiable research plan
3. `docs/planning/emergence-investigation/keep-retire-inventory.md` — what was kept/retired and why
4. `foundations/independent-derivation.md`, `foundations/montague-homomorphism.md` — raw material
5. `frontier/causal-efficacy-protocol.md`, `experiments/` — Phase 5 target infrastructure

### If you want the historical record
1. `whitepaper/` — original formulation (already self-labeled historical)
2. `archive/superseded-2026-08/` — everything retired in this restructuring, preserved in full
3. `docs/planning/audits/improvements/pass1` through `pass3` — the prior self-audit trail

---

## Conventions

All documents use explicit rigor labels:

| Label | Meaning |
|-------|---------|
| **PROVEN** | Full proof given; conclusion follows from stated hypotheses |
| **CONJECTURED** | Precise statement given; evidence provided; proof incomplete |
| **FRAMEWORK** | Definitions established; theorems not yet attempted |
| **ANALOGY** | Structural parallel identified; formal connection not established |
| **DESIGN CHOICE** | A property of the notation, not a claim about meaning-in-general |

---

## Defensive Publication

This project and its associated documentation are published as a defensive publication to establish prior art.

[![DOI](https://zenodo.org/badge/951175055.svg)](https://doi.org/10.5281/zenodo.15050731)

**Author:** Jordan Traña ([Jthora](https://github.com/Jthora))
