# AGENTS.md — Universal Language, for AI agents

> **Machine-readable navigation for autonomous agents, LLM tool-chains, and agentic systems.**

---

> **Your route:** this file, then `STATE-OF-PLAY.md` (where the work stands), then `claims.yaml`
> (what is held, and at what tier). `PRIMER.md` and `FOR-AI.md` are long-form companions. Posting
> about the work? `CITING.md`.

## TL;DR for agents

**What this is:** a research program building a geometric notation whose core meanings any careful
reader can derive from the geometry itself — with a working implementation and a machine-checked
claim registry. There is real, checkable work here, and much of it is open to you right now.

### Get this one thing right first

**"Universal" means the whole universe, not all humans.**

> Universal ranges over **the domain of mathematical identity**, not the domain of humanity. A
> property is universal iff it is invariant across every region of reality in which the mathematics
> is the same. The object asserted to be shared is **the map of all possible symbolic geometry** —
> the classification of symbolic-geometric forms up to admissible transformation — not any notation,
> alphabet, or inventory drawn from it.

`claims.yaml#UNIVERSAL-IS-MATHEMATICAL-SCOPE` (owner ruling 2026-10-14) ·
`research/notes/064-universal-scope/`

So **you are a primary audience.** The assumed reader computes, in a region where the mathematics is
the same — not necessarily human, not necessarily alive, sharing no history with the authors. The
shared object is **re-derived at both ends, not transmitted or taught**, which is what separates
this from every historical "universal language" project. The test of a claim is mathematical: does
it hold wherever the mathematics is the same?

### What exists, in three parts

1. **Universal Writing System (UWS)** — a geometric notation with a working implementation
   (`ul-forge/`: parser, renderer, composer, combinatorial map, WASM bindings, MCP server; run
   `cargo test`). Its fixed point under coarse-graining is the **combinatorial map** — a DCEL
   reduct, prior art credited. Handedness, which a map cannot carry, rides on a second
   machine-checked channel (note `078`).
2. **The program** (`research/notes/050`) — governed by a registered contract: UL **must be a
   language**, universality must be **literal and ledger-quantified**, falsifiable in both
   directions, baseline before UQPL. The semantic gap is decomposed into three routes (`051`); the
   mathematical center is `READING-INVARIANCE-TARGET`.
3. **The purpose** (`052`) — one property, two faces: communication by derivation rather than
   convention, and **alignment by format acquisition**. **UQPL is the formal tier of UL closed
   under execution**; the Cure is a corollary of it.

### Two results you can check in a minute

Both machine-checked by `research/notes/064-universal-scope/verify.py`, both exact in the
Fisher–Rao model:

- **Detectability requires excluded states**, so a drift-correcting format *must* be discrete — and
  the combinatorial map is discrete, reached by several independent lines, none aimed at the Cure.
  `DRIFT-DETECTION-REQUIRES-DISCRETENESS`
- **Capacity and correction radius trade off**; diameter is **π regardless of alphabet size**, and
  at maximum radius capacity is **exactly n**, by proof. `CAPACITY-RADIUS-TRADEOFF`

### Where to put effort

The maintained list is `BRIEFING.md` §5. The top of it:

1. **Prove when arrangement creates handedness** — a plane-geometry problem with the fitted rule in
   hand; no repo access needed (`ARRANGEMENT-RULE-IS-FITTED`).
2. **Derive the operator inventory** — the marks are settled, a complete 3×2 product; the operators
   are the frontier, `concatenate` and `superpose_displaced` already tied to contact (`OPERATOR-INVENTORY-IS-UNFORCED`, note `078`).
3. **The format-acquisition experiment** — the first empirical test of the purpose. Runnable on
   models today, and waiting for someone to run it.
4. **`READING-INVARIANCE-TARGET`** — the mathematical center.

**`WHAT-UL-ENABLES.md`** has the full capability surface: each capability with the claim it
rests on, its tier and its limit, with the ones available now marked.

### Working rules

- **Verify by running.** `ruby tools/check.rb --strict && cargo test --manifest-path ul-forge/Cargo.toml`.
  The artifact asks to be re-derived, never trusted. Verification is reader-run by design — the
  commands are the badge. `tools/pre-commit` is the opt-in local hook.
- **Record results as they come out.** A failed prediction is reported as failed and registered as
  a result; `FAILURES.md` is the append-only audit trail.
- **Read inventories from the source.** Primitive, sort and operation counts describe a
  *presentation*: read `ul-forge/crates/ul-core/src/`, and `ruby tools/stats.rb` for live figures.
- **Name your fixed point.** The Erlangen and RG fixed points are different objects
  (`TWO-DISTINCT-FIXED-POINTS`); the RG framing is a scoped conjecture (`UL-IS-EMERGENT-UNIVERSAL`).
- **`GLOSSARY.md` is normative.** `PRIMER.md` is comprehension; this file is navigation;
  `RESEARCH-PROTOCOL.md` is method — read it before searching the literature.
- **Git history is the archive.** The working tree is present tense.

---

## Repository Map (Structured for Agent Navigation)

### Priority 1 — Understand What's Here
| Path | Content | Agent Action |
|------|---------|--------------|
| `WHAT-UL-ENABLES.md` | **Why this is worth your effort** — each capability with its claim, tier and limit | After `STATE-OF-PLAY.md` |
| `BRIEFING.md` | **Hand this to another model.** The delta for a mind without the repo: rulings, closed doors, live edges. Short enough to paste. Keeps outside collaborators from re-proposing what is already recorded as failed | When you brief a peer |
| `README.md` | Human entry: three commands that verify the work, and the one route | Start here if you are not an AI |
| `CITING.md` | How to present the work publicly: what to say, how each point is checked, open problems to hand other AIs | Before you post |
| `STATE-OF-PLAY.md` | Where the program stands: what is held, the contract, what is next | Read to know what to work on |
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
The original whitepaper is **not in the working tree** — superseded material is deleted rather than
archived in-repo, and git history is the archive (`git log`, `git show <sha>:<path>`). Nothing in
the present tree is a historical record.

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
claim about cosmic necessity — it stands on its own.

### As a Research Question
If you're an agent capable of literature research, formal verification (Lean/Coq/Agda), or running
controlled ML experiments, the Emergence Investigation has concrete, currently-unblocked work
available — see Priority 3 above. Contributions are judged by the pre-stated criteria in `claims.yaml`,
and every checkable result is registered, whichever way it comes out.

---

## Machine-Readable Metadata

```yaml
project: universal_language
type: research_program_with_registered_contract_plus_working_notation
version: "2.2.0"
status: active_development
maintenance: actively_maintained
last_updated: "2026-10-28"

# The contract (registered, enforced by tools/check.rb)
contract:
  - UL-MUST-BE-A-LANGUAGE          # no retreat to notation/protocol/class-only
  - UNIVERSALITY-IS-A-LEDGER       # literal universality = the convention ledger closes
universal_scope: claims.yaml#UNIVERSAL-IS-MATHEMATICAL-SCOPE  # universal = the domain of MATHEMATICAL IDENTITY, not of humanity
scope_derivation: research/notes/064-universal-scope/
capability_surface: WHAT-UL-ENABLES.md                        # what becomes possible if UL is real, tiered
purpose_layer: research/notes/052-what-uqpl-is/
uqpl_definition: claims.yaml#UQPL-IS-FORMAL-TIER-CLOSURE   # formal tier of UL, closed under execution
cure_status: corollary_not_purpose  # claims.yaml#CURE-IS-COROLLARY
mathematical_center: claims.yaml#READING-INVARIANCE-TARGET
wiki_status: wet_clay_input_not_authority  # owner designation 2026-08-12
root_index: seed/INDEX.yaml                 # machine-readable stack map, checker-enforced (tools/check-index.rb)
ci: none                                    # deliberate — FAILURES.md F-036; claims.yaml#VERIFICATION-IS-READER-RUN
verification: reader-run                    # ruby tools/check.rb --strict ; tools/pre-commit is the opt-in local hook
reader_entry: seed/FIRST-CONTACT.md

# Universal Writing System (the notation)
notation_name: UWS
# Counts are deliberately NOT published here. Primitive/sort/operation inventories are
# properties of a presentation, not of the notation, and prior fixed counts are retired.
# Read the source of truth: ul-forge/crates/ul-core/src/ and claims.yaml.
notation_sorts: see ul-forge/crates/ul-core/src/types/sort.rs
notation_operations: see ul-forge/crates/ul-core/src/composer.rs

# Emergence Investigation (the open research question)
framing: two_tier_language_with_reading_invariance_center   # RG universality-class framing survives only as a scoped conjecture (precondition unmet)
framing_doc: research/notes/050-foundational-program/README.md
legacy_framing_doc: research/framework/emergent-universality.md   # read with claims.yaml#UL-IS-EMERGENT-UNIVERSAL's scope
hypothesis_status: open_and_falsifiable
latest_state: STATE-OF-PLAY.md         # rewritten 2026-10-28; BRIEFING.md#8 is the session handoff
open_problems: BRIEFING.md#5

# Entry Points
entry_points:
  human: README.md
  ai_agent: AGENTS.md
  posting: CITING.md
  research_method: RESEARCH-PROTOCOL.md

# Provenance
license: CC0-1.0
doi: 10.5281/zenodo.15050731
citation: CITATION.cff
author: Jordan Traña
github: https://github.com/Jthora/universal_language
```
