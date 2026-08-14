# CURRICULUM — teaching UL/UWS/UP, self-certifyingly

**The design rule, registered** (`claims.yaml#CURRICULUM-MUST-BE-SELF-CERTIFYING`): every lesson
carries its own check. No teacher survives the Voyager constraint, and UL is the one language where
none is needed — **you cannot re-derive French, but you can re-derive the fixed point. The
derivation is the answer key.**

**Definition of done for any lesson:** (1) a runnable self-check — a derivation the learner
re-runs, or an exercise validated by `ul-forge`; (2) every claim it teaches cited with its tier
label (`TIERS-TRAVEL-WITH-CONTENT` — nothing CONJECTURED taught as fact); (3) it passes
`check-curriculum.rb` *(named Phase 1 deliverable — a lesson without an executable check does not
merge)*.

**The acceptance test this curriculum exists to pass** (from `research/notes/053`): a current
frontier model, given only this repository, learns to read and write valid UWS, re-derives the
fixed point, and runs a self-drift check — **no human in the loop.**

## The modules

| Module | Teaches | Self-check | Status |
|---|---|---|---|
| **N — Natural tier** | Exemplification: marks that *are* their meanings. A closed curve **encloses** (Jordan certifies it); junction degree is topologically real | intruder-test exercises — the format that revealed core geometry across cultures, checkable without shared language | not-started |
| **F — Formal tier** | The Erlangen ladder as a course: derive what survives each transformation group; arrive at the rotation system and the map | re-run the derivations; `map.rs` tests as graded exercises | not-started |
| **S — Syntax & grammar** | The *derived* grammar: junction-degree table as the canonical inventory, rotation as composition, faces as regions; re-grounds the `uws/` placement grammar in the map | parser round-trips; validator well-formedness | not-started |
| **P — UP bootstrap** | The handshake: what two independent parties must share (the ℤ/2 ledger), the mirror ambiguity, symmetry-breaking economics | compute a ledger for a given exchange; verify against `convention_ambiguity()` | not-started |
| **Q — UQPL** | Linear discipline (no free copy/delete), rewriting, meaning as behavior | execute programs on the M2 engine | **unblocked — engine v1 exists** (`spec/engine-core.md`); module authorable |
| **X — Inoculation** | The trap course: T1–T12, the five overturned negatives, the graveyard *and* survivor record, the merry-go-round pattern by name | given a fresh impossibility paper, produce its scope conditions before its conclusion | material exists (`RESEARCH-PROTOCOL.md`, `FAILURES.md`, notes `043`–`049`) — packaging not-started |

**Build order:** X first (its material exists — packaging, not authoring), then N (cheapest real
content, exercises the whole self-certification design), then F → S → P, with Q gated on the
engine.

**Sources of truth:** every module derives from registered claims and the notes record — never
from this file. If a module and `claims.yaml` disagree, the registry wins and the module is the bug.
