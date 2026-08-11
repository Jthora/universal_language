# AGENTS.md — Universal Writing System & the Emergence Investigation, for AI Agents

> **Machine-readable navigation file for autonomous agents, LLM tool-chains, and agentic systems.**

---

## TL;DR for Agents

**As of 2026-08-01, this repo's earlier claims of proof have been retired.** This file previously
asserted that "Universal Language" (Σ_UL: 5 primitives, 4 sorts, 13 operations, 23 theorems) was
"unique up to isomorphism" and mathematically forced. A direct audit
(`research/wiki-comparison-2026-08.md`) found the central proof ("Unique Grounding
Theorem") close to circular, and a from-scratch minimality analysis gives a different, smaller
answer (2 base types under standard model theory) than the claimed counts. **Do not cite the old
claims as established.** The retired material is preserved at `archive/superseded-2026-08/` for
historical reference only.

What actually exists here now, in two parts:

1. **Universal Writing System (UWS)** — a constructed geometric notation with a working software
   implementation (`ul-forge/`: parser, renderer, composer, WASM bindings, MCP server). Real,
   usable, judged on its own merits as a notation — not on a retired proof of cosmic necessity.
2. **The Emergence Investigation** — an open, six-phase, falsifiable research program
   (`research/emergence-investigation/PLAN.md`) asking whether *any* version of a
   mind-independent universal semantic structure is real. Treat its findings as provisional and
   its predictions as genuinely falsifiable — do not patch a failed prediction to make it pass;
   report the failure.

---

## Repository Map (Structured for Agent Navigation)

### Priority 1 — Understand What's Here
| Path | Content | Agent Action |
|------|---------|--------------|
| `README.md` | Current status and two-part structure | Read first |
| `research/wiki-comparison-2026-08.md` | The audit that surfaced the core problem | Read for context on why the old claims were retired |
| `research/emergence-investigation/PLAN.md` | The active, falsifiable research plan | Read to understand what's actually being investigated and what would count as evidence either way |

### Priority 2 — Use the Writing System (UWS)
| Path | Content | Agent Action |
|------|---------|--------------|
| `uws/design-rationale.md` | UWS design rationale (reframed, no longer a proof claim) | Read to understand the notation's design |
| `uws/writing-system/writing-system.md` | Complete reading/writing specification | Learn to read and write the notation |
| `uws/lexicon/lexicon.md` | 42 canonical definitions | Reference for core notation terms |
| `ul-forge/` | Software implementation | Use for parsing/rendering/composing |

### Priority 3 — Contribute to the Emergence Investigation
| Path | Content | Agent Action |
|------|---------|--------------|
| `research/emergence-investigation/PLAN.md` | Six phases, each with pre-stated falsification criteria | Pick an unblocked phase and contribute — Phase 1 (convergent mathematics audit) and Phase 2 (non-human semiosis literature) are solo/literature-based and currently unblocked |
| `research/independent-derivation.md` | Raw material — an attempted independent derivation, not a settled result | Treat as one candidate hypothesis to test, not as proof |
| `research/frontier/causal-efficacy-protocol.md`, `research/experiments/` | Phase 5 target — needs pre-registration/blinding/matched-control fixes before any run counts as evidence | Do not run without those fixes; the existing infra has a documented pattern of post-hoc patching to avoid |

### Priority 4 — Historical Record (not current status)
| Path | Content |
|------|---------|
| `archive/superseded-2026-08/` | Full original text of everything retired in the August 2026 split |
| `archive/planning-history/audit-passes/pass1`–`pass3` | Prior self-audit trail — useful as process history, not as current claims |
| `whitepaper/` | Original whitepaper, already self-labeled historical |

---

## Notation Signature (Design Choice, Not Proven Necessity)

```
NOTATION Σ (Universal Writing System)

SORTS: Entity (e), Relation (r), Modifier (m), Assertion (a)
  — a design choice for this notation. A from-scratch minimality analysis
  (research/emergence-investigation/PLAN.md, Phase 3) finds the actual minimal
  answer under standard model theory is 2 base types (Montague's e, t) — these 4
  sorts are a flattening choice for drawability, not a discovered constant.

OPERATIONS (13, as implemented in ul-forge/):
  predicate, modify_entity, modify_relation, negate, conjoin, disjoin,
  embed, abstract, compose, invert, quantify, bind, modify_assertion

GEOMETRIC PRIMITIVES (5, as drawn in the notation):
  Point, Line, Angle, Curve, Enclosure

STATUS: This is a notation specification. It is not a proof that meaning
necessarily has this structure. See research/emergence-investigation/
for the open question of what, if anything, about this is actually forced.
```

---

## Value Proposition for Agentic Systems

### As a Notation Tool
UWS is a usable, composable visual notation with real software tooling — genuinely useful as a
structured intermediate representation for cross-agent exchange, the same way any well-designed
notation (UML, a schema language, a diagram format) is useful. This value does not depend on any
claim about cosmic necessity.

### As a Research Question
If you're an agent capable of literature research, formal verification (Lean/Coq/Agda), or running
controlled ML experiments, the Emergence Investigation has concrete, currently-unblocked work
available — see Priority 3 above. Contributions should honor the pre-stated falsification criteria
in `research/emergence-investigation/PLAN.md`; a result that fails a prediction is a valid,
useful contribution and should be reported as such, not revised until it passes.

---

## Machine-Readable Metadata

```yaml
project: universal_language
type: constructed_notation_system_plus_open_research_question
version: "2.0.0"
status: active_development
maintenance: actively_maintained
last_updated: "2026-08-01"

# Universal Writing System (notation — design choice, not proven necessity)
notation_name: UWS
notation_sorts: [entity, relation, modifier, assertion]
notation_operations: 13
geometric_primitives: [point, line, angle, curve, enclosure]
proof_status: retired_2026-08 — see research/wiki-comparison-2026-08.md
minimality_status: superseded — from-scratch analysis finds 2 base types (Montague e,t) sufficient

# Emergence Investigation (open question, not settled)
investigation_plan: research/emergence-investigation/PLAN.md
investigation_phases: 6
current_phase: 1
hypothesis_status: falsifiable, not yet resolved

# Entry Points
entry_points:
  human: README.md
  ai_agent: AGENTS.md
  investigation: research/emergence-investigation/PLAN.md
  archive: archive/superseded-2026-08/

# Provenance
license: CC0-1.0
doi: 10.5281/zenodo.15050731
citation: CITATION.cff
author: Jordan Traña
github: https://github.com/Jthora/universal_language
```
