# research/ — Open Questions

**Tier rule: speculation is permitted here, and must be labeled.**

This is the only tier where unresolved and exploratory material belongs. Documents may conjecture
freely, provided claims carry a tier (`GLOSSARY.md`) and land in `/claims.yaml` when substantive.

**One hard rule:** results that fail go in `/FAILURES.md` and stay there. Nothing in this directory
may retroactively convert a recorded failure into a success. That rule is the entire reason the
research here is worth anything — see `postmortem-and-rebuild-2026-08.md` FM2.

---

## Current state of the program (read in this order)

| Document | What it is |
|---|---|
| `reassessment-2026-08-purpose-anchored.md` | **Start here.** Reframes the whole program around its actual purpose (the Cure), and separates UL / UWS / UQPL into distinct projects. |
| `emergence-investigation/phase6-synthesis-and-verdict.md` | Consolidated verdict on whether UL is real. Not established, not refuted — and the primitive count is provably underdetermined. |
| `emergence-investigation/phase7-position-c-mathematical-derivation.md` | The follow-up that pursued direct derivation instead of survey. Contains the Zadrozny result — the sharpest finding in the program. |
| `deep-critique-2026-08-wiki-and-implementation.md` | Applies pressure to both the wiki's foundations and the repo's actual code. Contains the repair-operator problem. |
| `uws-as-methodology-2026-08.md` | Why UWS is an *instrument* for discovering UL, and where its real grounding is (perception, not Platonic solids). |
| `postmortem-and-rebuild-2026-08.md` | Why the repo's practice failed and what mechanisms replace it. |
| `wiki-comparison-2026-08.md` | The original audit that started the whole reassessment. |

## Supporting material

| Path | What it is |
|---|---|
| `emergence-investigation/` | Full seven-phase record with pre-registered falsification criteria. Two phases (4, 5) are designs pending compute budget. |
| `independent-derivation.md`, `montague-homomorphism.md` | Formal-semantics raw material. Treat as attempts, not settled results. |
| `frontier/` | Exploratory mathematics from the pre-2026-08 era (category theory, gauge bundles, probability). Contains real content and retired framing — read with `GLOSSARY.md` open. |
| `frontier/causal-efficacy-protocol.md` | Experimental protocol. **Do not run as written** — contains a pseudo-replication error (`FAILURES.md` F-008). |
| `experiments/` | Experimental infrastructure. Blinding and pre-registration tooling exists and has **never been run**. |

---

## The two open problems that matter most

Both are concrete, reachable, and block everything downstream. Neither requires resolving whether
UL is real.

1. **What is the repair operator, and is it deterministic?** (`FAILURES.md` F-009)
   Projection onto a non-convex admissible region is multivalued. Candidate directions: convex
   relaxation, convex-cell decomposition with declared tie-breaking, or AGM-style least-change
   belief revision instead of metric projection.

2. **What is `semantically_equal`?** (`claims.yaml#SEMANTIC-EQUALITY`)
   A decision procedure for whether two structures mean the same thing. Does not exist in any form.
   Prerequisite for the Cure, for UQPL, and for every algebraic law in the notation.
