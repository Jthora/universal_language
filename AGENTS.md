# AGENTS.md — Universal Writing System & the Emergence Investigation, for AI Agents

> **Machine-readable navigation file for autonomous agents, LLM tool-chains, and agentic systems.**

---

> **`STATE-OF-PLAY.md`** is the current synthesis — what the research established, what it forces,
> and the reconstruction order. Read it to know what to work on.
>
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

What actually exists here now, in three parts:

1. **Universal Writing System (UWS)** — a constructed geometric notation with a working software
   implementation (`ul-forge/`: parser, renderer, composer, combinatorial map, WASM bindings, MCP
   server; 354 tests). Its fixed point under coarse-graining is the **combinatorial map**, honestly
   recorded as a DCEL reduct.
2. **The program** (`research/notes/050`) — governed by a registered contract: UL **must be a
   language**, universality must be **literal and ledger-quantified**, no-retreat falsifiability in
   both directions, baseline before UQPL. The semantic gap is decomposed into three routes
   (`research/notes/051`); the mathematical center is `READING-INVARIANCE-TARGET`.
3. **The purpose-layer** (`research/notes/052`) — one property, two faces: communication by
   derivation rather than convention, and **alignment by format acquisition** (strong "purification"
   reading renounced on the far-transfer record, and registered as such). **UQPL is the formal tier
   of UL closed under execution**; **the Cure is a corollary, not the purpose.**

Findings are provisional and predictions are genuinely falsifiable — do not patch a failed
prediction to make it pass; report it. The RG "universality class" framing survives only as a
scoped conjecture with its precondition unmet — see `claims.yaml#UL-IS-EMERGENT-UNIVERSAL` and
`#TWO-DISTINCT-FIXED-POINTS` before using the word "universality" loosely.

---

## Repository Map (Structured for Agent Navigation)

### Priority 1 — Understand What's Here
| Path | Content | Agent Action |
|------|---------|--------------|
| `README.md` | The contract, the five constructs, the purpose-layer | Read first |
| `STATE-OF-PLAY.md` | Where the program stands now | Read to know what to work on |
| `research/notes/050-foundational-program/` | The standing program: REQ-1–4, the ledger, the semantic stack, reading-invariance | The governing document for all cycles |
| `research/notes/052-what-uqpl-is/` | The purpose-layer: two tiers, alignment as format acquisition, UQPL derived | What the work is *for* |
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
| `research/framework/provable-geometry.md` | What is derivable vs. what needs data | Pick a proof target; several are cheap and decisive |
| `research/engineering/obstructions.md` | Open claims classified by whether proof is available | Find unblocked work |
| `claims.yaml` | The registry. Priority-0 entries are the live front | Check tier and evidence before acting on any claim |

### Priority 4 — Historical Record (not current status)
| Path | Content |
|------|---------|
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
type: research_program_with_registered_contract_plus_working_notation
version: "2.1.0"
status: active_development
maintenance: actively_maintained
last_updated: "2026-08-12"

# The contract (registered, enforced by tools/check.rb)
contract:
  - UL-MUST-BE-A-LANGUAGE          # no retreat to notation/protocol/class-only
  - UNIVERSALITY-IS-A-LEDGER       # literal universality = the convention ledger closes
purpose_layer: research/notes/052-what-uqpl-is/
uqpl_definition: claims.yaml#UQPL-IS-FORMAL-TIER-CLOSURE   # formal tier of UL, closed under execution
cure_status: corollary_not_purpose  # claims.yaml#CURE-IS-COROLLARY
mathematical_center: claims.yaml#READING-INVARIANCE-TARGET
wiki_status: wet_clay_input_not_authority  # owner designation 2026-08-12

# Universal Writing System (notation — design choice, not proven necessity)
notation_name: UWS
# Counts are deliberately NOT published here. Primitive/sort/operation inventories are
# properties of a presentation, not of the notation, and prior fixed counts are retired.
# Read the source of truth: ul-forge/crates/ul-core/src/ and claims.yaml.
notation_sorts: see ul-forge/crates/ul-core/src/types/sort.rs
notation_operations: see ul-forge/crates/ul-core/src/composer.rs
proof_status: retired_2026-08

# Emergence Investigation (open question, not settled)
framing: two_tier_language_with_reading_invariance_center   # RG universality-class framing survives only as a scoped conjecture (precondition unmet)
framing_doc: research/notes/050-foundational-program/README.md
legacy_framing_doc: research/framework/emergent-universality.md   # read with claims.yaml#UL-IS-EMERGENT-UNIVERSAL's scope
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
