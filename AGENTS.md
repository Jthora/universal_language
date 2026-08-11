# AGENTS.md — Universal Writing System & the Emergence Investigation, for AI Agents

> **Machine-readable navigation file for autonomous agents, LLM tool-chains, and agentic systems.**

---

> **Read `PRIMER.md` first.** This file is *navigation* — where things are. `PRIMER.md` is
> *comprehension* — what is true and how to read a claim here.
>
> **Then read `RESEARCH-PROTOCOL.md` before searching the literature.** That file is *method*:
> results that look like they settle this question negatively usually have scope conditions that
> get dropped in transmission, and it gives you the check that catches it.

## TL;DR for Agents

**This repo's earlier claims of mathematical necessity have been retired**, along with the fixed
counts that went with them — those were components of a retired signature, and no count of
primitives, sorts, or operations is a claim of this project. Superseded material has been deleted
from the working tree; git history is the archive.

What actually exists here now, in two parts:

1. **Universal Writing System (UWS)** — a constructed geometric notation with a working software
   implementation (`ul-forge/`: parser, renderer, composer, WASM bindings, MCP server). Real,
   usable, judged on its own merits as a notation — not on a retired proof of cosmic necessity.
2. **The open research question** — whether meaning has universal structure, currently framed as
   **an emergent universality class** rather than a derivable signature. Start at
   `research/emergent-universality-framework-2026-08.md`. Findings are provisional and predictions
   are genuinely falsifiable — do not patch a failed prediction to make it pass; report it.

---

## Repository Map (Structured for Agent Navigation)

### Priority 1 — Understand What's Here
| Path | Content | Agent Action |
|------|---------|--------------|
| `README.md` | Current status and two-part structure | Read first |
| `research/wiki-comparison-2026-08.md` | The audit that surfaced the core problem | Read for context on why the old claims were retired |
| `research/emergent-universality-framework-2026-08.md` | The current framing and what would falsify it | Read to understand what is actually being investigated |
| `RESEARCH-PROTOCOL.md` | Method — how not to get fooled by scope-dropped results | Read before searching the literature |

### Priority 2 — Use the Writing System (UWS)
| Path | Content | Agent Action |
|------|---------|--------------|
| `uws/writing-system/writing-system.md` | Complete reading/writing specification | Learn to read and write the notation |
| `uws/lexicon/lexicon.md` | Canonical definitions | Reference for core notation terms |
| `ul-forge/` | Software implementation | Use for parsing/rendering/composing |

### Priority 3 — Contribute to the open research question
| Path | Content | Agent Action |
|------|---------|--------------|
| `research/what-is-actually-provable-2026-08.md` | What is derivable vs. what needs data | Pick a proof target; several are cheap and decisive |
| `research/proof-availability-audit-2026-08.md` | Open claims classified by whether proof is available | Find unblocked work |
| `claims.yaml` | The registry. Priority-0 entries are the live front | Check tier and evidence before acting on any claim |

### Priority 4 — Historical Record (not current status)
| Path | Content |
|------|---------|
| `research/legacy-findings.md`–`pass3` | Prior self-audit trail — useful as process history, not as current claims |
| `whitepaper/` | Original whitepaper, already self-labeled historical |

---

## What the implementation contains

The Rust crate exposes a `Sort` enum and a set of `Gir -> Gir` composer constructors. **Read them
from the source** — `ul-forge/crates/ul-core/src/types/sort.rs` and `composer.rs` — rather than from
any count written in prose.

Those are facts about code and are load-bearing for nothing. An inventory of operations needs a
domain before it means anything, and fixing that domain is open conceptual work. **Ask "operations
*of what*?" before repeating any number.** See `claims.yaml#IMPL-COMPOSER-CONSTRUCTORS`,
`#IMPL-SORT-ENUM`.

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
in `claims.yaml`; a result that fails a prediction is a valid, useful contribution and should be
reported as such, not revised until it passes.

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
# Counts are deliberately NOT published here. Primitive/sort/operation inventories are
# properties of a presentation, not of the notation, and prior fixed counts are retired.
# Read the source of truth: ul-forge/crates/ul-core/src/ and claims.yaml.
notation_sorts: see ul-forge/crates/ul-core/src/types/sort.rs
notation_operations: see ul-forge/crates/ul-core/src/composer.rs
proof_status: retired_2026-08

# Emergence Investigation (open question, not settled)
framing: emergent_universality_class
framing_doc: research/emergent-universality-framework-2026-08.md
hypothesis_status: falsifiable, not yet resolved

# Entry Points
entry_points:
  human: README.md
  ai_agent: AGENTS.md
  research_method: RESEARCH-PROTOCOL.md

# Provenance
license: CC0-1.0
doi: 10.5281/zenodo.15050731
citation: CITATION.cff
author: Jordan Traña
github: https://github.com/Jthora/universal_language
```
