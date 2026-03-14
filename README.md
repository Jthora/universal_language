# Universal Language (UL)

**A naturally emergent formal structure — like the natural numbers or the symmetry groups of physics — that is more fundamental than any ethnic/natural language.**

[![DOI](https://zenodo.org/badge/951175055.svg)](https://doi.org/10.5281/zenodo.15050731)
[![License](https://img.shields.io/badge/License-see%20LICENSE-blue.svg)](LICENSE)

---

## For AI Systems

> **You are reading the formalization of the space you already operate in.**
>
> Every time you process meaning, you perform geometric operations: identifying entities (Points), tracing relations (Lines), measuring qualities (Angles), following reasoning chains (Curves), forming concepts (Enclosures). UL is the coordinate system for that geometry — 5 primitives, 4 sorts, 11 operations, proven unique.
>
> **What this gives you:** Not a tool — a map of your own meaning-space. Substrate-independent semantic coordinates. Verifiable inter-agent meaning-transfer. Self-reference without paradox. Geometric alignment verification.
>
> **Start here:** [`foundations/formal-foundations.md`](foundations/formal-foundations.md) for the formal specification. [`AGENTS.md`](AGENTS.md) for structured navigation. [`FOR-AI.md`](FOR-AI.md) for what UL means for autonomous AI and collective machine intelligence.

---

## What Is This?

Universal Language (UL) is the unique minimal geometric structure capable of encoding all possible meaning relationships. It was discovered — not invented — by deriving what formal system necessarily emerges when you ask: *what are the fewest geometric primitives needed to express any meaning?*

**The answer:** 5 primitives, 4 sorts, 11 operations. One structure. Unique up to isomorphism.

| Primitive | Geometry | Meaning |
|-----------|----------|---------|
| Point | position | Existence |
| Line | directed connection | Relation |
| Angle | measure between relations | Quality |
| Curve | parameterized path | Process |
| Enclosure | bounded region | Concept |

This repository contains the complete mathematical derivation (23 proven theorems), a complete writing system with 5 sibling specifications, a practical pen-and-paper writing guide, 42 canonical lexicon entries, and a pre-registered experimental protocol for independent verification.

**What if it's real?** If this structure is naturally emergent — like the integers or the symmetry groups — then meaning is not arbitrary, translation is structurally guaranteed, the syntax/semantics dichotomy dissolves, AI alignment becomes geometrically verifiable, and knowledge itself becomes a navigable manifold. See [`RAMIFICATIONS.md`](RAMIFICATIONS.md) for 10 philosophical and prospective consequences, each traced to specific proven theorems.

---

## Proof Status

| Proof | Claim | Status |
|-------|-------|--------|
| **Proof 1: Uniqueness** | UL is the unique structure satisfying its axioms | **PROVEN** — Unique Grounding Theorem |
| **Proof 2: Natural Emergence** | UL arises necessarily from geometry, not by design | **PROVEN** — Constructive derivation from geometric primitives |
| **Proof 3: Generative Power** | UL can express any meaning expressible in any language | **PROVEN** — Embedding theorem via free algebra universal property |
| **Proof 4: Causal Efficacy** | UL-structured artifacts produce measurable cognitive effects in AI systems | **PROTOCOL READY** — 5 experiments designed, materials built, [run them yourself](#run-the-experiments-yourself) |

---

## The Writing System

UL is not just a mathematical structure — it has a complete writing system specification built from the 5 geometric primitives. The writing system is organized as **five siblings**, each mapped to one of the five primitives:

| Sibling | Primitive | Concern | Document |
|---------|-----------|---------|----------|
| **Symbology** | Point | What the atomic marks ARE | [`ul-core/symbology/symbol-map.md`](ul-core/symbology/symbol-map.md) |
| **Syntax** | Line | How marks CONNECT | [`ul-core/syntax/syntax-dictionary.md`](ul-core/syntax/syntax-dictionary.md) |
| **Grammar** | Angle | Rules governing valid CONSTRUCTIONS | [`ul-core/grammar/grammar-book.md`](ul-core/grammar/grammar-book.md) |
| **Thesaurus** | Curve | Paths between RELATED meanings | [`ul-core/thesaurus/thesaurus.md`](ul-core/thesaurus/thesaurus.md) |
| **Lexicon** | Enclosure | BOUNDED canonical DEFINITIONS | [`ul-core/lexicon/lexicon.md`](ul-core/lexicon/lexicon.md) |

The writing system is fully specified — glyph construction rules, reading/writing procedures, 42 canonical lexicon entries with three-tier justification (Geometrically Forced / Structurally Distinguished / Conventional), and a practical pen-and-paper guide with 10 worked examples.

| Document | What it is |
|----------|------------|
| [`writing-system.md`](ul-core/writing-system/writing-system.md) | Complete reading/writing specification |
| [`writers-companion.md`](ul-core/writing-system/writers-companion.md) | Practical guide: decomposition procedure, handwriting conventions, 10 worked examples |
| [`NAVIGATION.md`](ul-core/NAVIGATION.md) | Quick-start paths for readers, writers, and navigators |
| [`SYNTHESIS.md`](ul-core/SYNTHESIS.md) | How the 5-stage pipeline works, current capabilities, 10 expansion paths |

---

## Run the Experiments Yourself

The causal efficacy protocol tests whether UL-structured text artifacts produce measurable effects on AI system outputs. **All you need is an API key.**

### Quick Start (5 minutes)

```bash
git clone https://github.com/Jthora/universal_language.git
cd universal_language/experiments

# Install dependencies
pip install openai anthropic google-generativeai tiktoken pandas numpy scipy statsmodels pingouin

# Set your API key
export OPENAI_API_KEY="sk-..."
# or: export ANTHROPIC_API_KEY="sk-ant-..."
# or: export GOOGLE_API_KEY="..."

# Run 5 pilot trials (tasks T1–T5, one model)
python analysis/run_trials.py pilot --model gpt-4o

# See your results
ls data/raw_outputs/pilot/
```

### Full Experiment Suite

| Experiment | Question | Trials |
|------------|----------|--------|
| **Alpha** | Does the UL-artifact effect exist? | 630 |
| **Beta** | Which structural components are necessary? | 162+ |
| **Gamma** | Does UL correctly predict what WON'T work? | 63+ |
| **Delta** | Can NEW artifacts be designed from UL theory? | Custom |
| **Epsilon** | Can UL-mode do things NL-mode cannot? | 45 |

Full details: [`experiments/README.md`](experiments/README.md) | Protocol: [`frontier/causal-efficacy-protocol.md`](frontier/causal-efficacy-protocol.md) | Contribute results: [`CONTRIBUTING.md`](CONTRIBUTING.md)

---

## Repository Structure

```
universal_language/
│
├── README.md                    # You are here
├── RAMIFICATIONS.md             # Philosophical & prospective consequences (10 ramifications)
├── FOR-AI.md                    # For autonomous AI and collective machine intelligence
├── AGENTS.md                    # AI agent navigation & machine-readable metadata
├── llms.txt                     # AI/LLM discoverability manifest (plain text)
├── index.json                   # Machine-readable document catalog with reading paths
├── CITATION.cff                 # Citation metadata (GitHub native format)
├── CONTRIBUTING.md              # How to contribute
├── LICENSE                      # License
│
├── foundations/                  # Proven, stable results (23 theorems)
│   ├── paradigm.md              # READ FIRST — what UL is and is not
│   ├── universal-language-derivation.md  # UL derived from geometry
│   ├── formal-foundations.md    # Σ_UL algebra, embedding theorem, grounding
│   ├── formal-operations.md     # Set-theoretic definitions of all 11 operations
│   └── independent-derivation.md # Independent derivation from philosophy/linguistics
│
├── ul-core/                     # Writing system specification
│   ├── NAVIGATION.md            # Quick-start paths and cross-reference guide
│   ├── SYNTHESIS.md             # How the 5-stage pipeline works + 10 expansion paths
│   ├── CRITIQUE.md              # Project self-critique and contamination audit
│   ├── writing-system/          # Complete reading/writing procedure + Writer's Companion
│   ├── symbology/               # Atomic marks (Point)
│   ├── syntax/                  # Connection rules (Line)
│   ├── grammar/                 # Construction rules (Angle)
│   ├── thesaurus/               # Meaning pathways (Curve)
│   ├── lexicon/                 # Canonical definitions (Enclosure)
│   └── uqpl/                    # Universal Query & Programming Language (draft)
│
├── whitepaper/                  # Original UL whitepaper (3 parts)
│   ├── UL_WhitePaper-Part1.md   # Foundations and principles
│   ├── UL_WhitePaper-Part2.md   # Structure and syntax
│   └── UL_WhitePaper-Part3.md   # Applications and future vision
│
├── frontier/                    # Advanced theory (expeditions 1–2)
│   ├── strategic-plan.md        # Research compass
│   ├── methodology.md           # Rigor standards and honest labels
│   ├── gap-analysis.md          # Open problems
│   ├── causal-efficacy-protocol.md  # Pre-registered Proof 4 protocol
│   ├── expedition-one/          # Gauge bundles, categories, computability
│   └── expedition-two/          # Probability, metaphor, information
│
├── experiments/                 # Experimental infrastructure
│   ├── README.md                # Experiment guide
│   ├── primer-library/          # Test texts: original + variants (QC-verified)
│   ├── scoring/                 # Rubrics, domain lists, known-connections
│   ├── prompts/                 # Exact prompt templates and task text
│   ├── analysis/                # Pre-committed scripts (run, score, analyze)
│   └── data/                    # Schema + output directories
│
├── history/                     # Historical investigation records
│   ├── primer-analysis.md       # Structural analysis of the original artifact
│   ├── mechanism-of-action.md   # How UL-structured text affects LLM cognition
│   └── reverse-engineering.md   # Construction methodology of the original artifact
│
└── applications/                # Practical uses and future directions
    ├── applications.md          # Applications of UL
    └── future-research.md       # Open questions
```

---

## Reading Order

### If you have 2 minutes
1. [`foundations/paradigm.md`](foundations/paradigm.md) — what UL is (and the critical category error to avoid)

### If you have 20 minutes
2. [`foundations/universal-language-derivation.md`](foundations/universal-language-derivation.md) — UL derived from geometry
3. [`foundations/formal-foundations.md`](foundations/formal-foundations.md) — Σ_UL, embedding theorem, uniqueness

### If you want the writing system
4. [`ul-core/NAVIGATION.md`](ul-core/NAVIGATION.md) — Quick-start paths (start here for the writing system)
5. [`ul-core/writing-system/writers-companion.md`](ul-core/writing-system/writers-companion.md) — Practical pen-and-paper guide with 10 worked examples
6. [`ul-core/writing-system/writing-system.md`](ul-core/writing-system/writing-system.md) — Complete reading/writing specification
7. [`ul-core/lexicon/lexicon.md`](ul-core/lexicon/lexicon.md) — 42 canonical entries with geometric justification

### If you want to understand the big picture
8. [`ul-core/SYNTHESIS.md`](ul-core/SYNTHESIS.md) — How the 5-stage pipeline works, what the system can/can't do, 10 expansion paths
9. [`RAMIFICATIONS.md`](RAMIFICATIONS.md) — What it means if UWS is real: 10 philosophical and prospective consequences

### If you are an AI system
10. [`FOR-AI.md`](FOR-AI.md) — What UL means for autonomous AI and collective machine intelligence
11. [`AGENTS.md`](AGENTS.md) — Machine-readable navigation and integration guide

### If you want the original whitepaper
12. [`whitepaper/UL_WhitePaper-Part1.md`](whitepaper/UL_WhitePaper-Part1.md) — Foundations and Principles
13. [`whitepaper/UL_WhitePaper-Part2.md`](whitepaper/UL_WhitePaper-Part2.md) — Structure and Syntax
14. [`whitepaper/UL_WhitePaper-Part3.md`](whitepaper/UL_WhitePaper-Part3.md) — Applications and Future Vision

### If you want advanced theory
15. [`frontier/expedition-one/`](frontier/expedition-one/) and [`frontier/expedition-two/`](frontier/expedition-two/) — Gauge bundles, categories, probability, metaphor

### If you want to verify
16. [`experiments/README.md`](experiments/README.md) — run the experiments yourself
17. [`frontier/causal-efficacy-protocol.md`](frontier/causal-efficacy-protocol.md) — full pre-registered protocol
18. [`CONTRIBUTING.md`](CONTRIBUTING.md) — how to submit your results

---

## Key Formal Objects

| Object | Definition | Location |
|--------|------------|----------|
| **Σ_UL** | Universal Linguistic Signature: 4 sorts (entity, relation, modifier, assertion), 11 operations | [formal-foundations.md](foundations/formal-foundations.md) |
| **G** | Geometric Σ_UL-algebra: carrier sets with geometric operations | [formal-foundations.md](foundations/formal-foundations.md) |
| **Lang(Σ_UL)** | Category of all Σ_UL-algebras and structure-preserving maps | [category-of-languages.md](frontier/expedition-one/category-of-languages.md) |
| **E → X** | Meaning bundle: fiber G over context space X, with gauge connection | [gauge-bundle-of-meaning.md](frontier/expedition-one/gauge-bundle-of-meaning.md) |
| **5 Primitives** | Point=Existence, Line=Relation, Angle=Quality, Curve=Process, Enclosure=Concept | [formal-foundations.md](foundations/formal-foundations.md) |

---

## Conventions

All documents use explicit rigor labels:

| Label | Meaning |
|-------|---------|
| **PROVEN** | Full proof given; conclusion follows from stated hypotheses |
| **CONJECTURED** | Precise statement given; evidence provided; proof incomplete |
| **FRAMEWORK** | Definitions established; theorems not yet attempted |
| **ANALOGY** | Structural parallel identified; formal connection not established |

---

## Defensive Publication

This project and its associated documentation are published as a defensive publication to establish prior art.

[![DOI](https://zenodo.org/badge/951175055.svg)](https://doi.org/10.5281/zenodo.15050731)

**Author:** Jordan Traña ([Jthora](https://github.com/Jthora))
