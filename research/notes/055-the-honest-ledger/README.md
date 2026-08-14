# 055 — The honest ledger: is the owner right that the actual work is not done?

**Type:** decision
**Opened:** 2026-08-12
**Status:** closed
**Thread:** answers the owner's direct question against the registry; opens the **Construction
phase** the answer forces. Also records this turn's preparation work (corpus live, Module X
packaged, `uws/` boundary marked) and one near-miss.
**Prompted by:** *"we still haven't done the actual work ourselves to have a completed and
established UL with Syntax/Grammar... let alone a full UQPL. We know the foundations of the
ideas... but barely settled on how this is done. Am I wrong? Am I right?"*

---

## 1. The verdict, from the registry's own arithmetic

**Right on the first part — fully, and the numbers say so.**

| | Count |
|---|---|
| Registered claims | **122** |
| VERIFIED | **8** — and all eight are facts about code |
| VERIFIED claims about semantics, grammar, or vocabulary | **0** |
| Claims mentioning grammar or lexicon at all | **2**, neither VERIFIED |

Concretely, today:

- **There is no derived grammar.** The combinatorial map is the *substrate* — derived and
  machine-checked — but a grammar needs composition rules and well-formedness conditions **derived
  from that substrate**, and those do not exist. The grammar that exists (`uws/`) is the
  **designed** legacy notation: real, usable, implemented — and its glyph-to-concept assignments
  are design choices, now labeled as such at its front door.
- **There is no derived lexicon.** Zero meaning-to-configuration mappings established by
  derivation. **Nobody, today, can write a UL sentence whose meaning is derived rather than
  stipulated.** The nearest thing — the natural-tier exemplification core (enclosure, junction
  degree, containment) — is a registered *route* (M1), not built vocabulary.
- **There is no semantics in code.** No M2 engine, no `semantically_equal`, no comparator API.
- **There is no UQPL.** A derived *definition* (CONJECTURED) and zero executable core.

**"Let alone a full UQPL" — right.** The registry's tier arithmetic is the proof: the project's
epistemic engine is strong and its *object* — the language — is not yet built.

## 2. Where the statement is slightly too dark — "barely settled on how"

**Half right.** The correction matters because it determines what happens next:

- **Settled, genuinely:** the constraints any UL must satisfy (the contract, the ledger); the
  syntax substrate (the map, VERIFIED-adjacent); the decomposition of the semantic gap into three
  routes with named objections; the purpose and its scoping; the teaching architecture
  (self-certifying, acceptance-tested); the deployment architecture. **That is a specified,
  falsifiable program — more than "barely."**
- **Not settled, and the statement is right to feel it:** the program's keystone —
  `READING-INVARIANCE-TARGET` — is **unproven**, and M1's answer to Goodman hangs on it. So the
  "how" is a **registered plan whose central theorem is outstanding**, not an established method.

> **The precise grade: foundations established · method specified · construction not begun.**
> The decade of merry-go-round was spent clearing the ground everyone else abandoned — that was
> necessary, it is finished, and it is not the building.

## 3. The Construction phase — what "doing the actual work" is, in order

Registered here as the successor focus to the volleys. Each item lands VERIFIED-able artifacts, not
notes:

1. **Derive the grammar** (Module S's substance): composition as operations on rotation systems,
   well-formedness as map validity — each rule either derived from the substrate or entered on the
   ledger as convention with its cost. *Deliverable: a grammar spec whose every rule carries
   derivation-or-ledger status, plus validator code.*
2. **Build the natural-tier lexicon** (M1's substance): the exemplification vocabulary —
   configurations whose meanings are theorems (enclosure/Jordan, degree, containment/nesting,
   adjacency/shared face, count). *Deliverable: lexicon entries as corpus items, each
   machine-verified, each tier-labeled.*
3. **Build the M2 engine**: interaction-style rewriting over `map.rs`, linear discipline.
   *Deliverable: executing programs; Module Q unblocks; "a full UQPL" starts here.*
4. **Prove (or break) reading-invariance**: the theorem that makes the result *universal* rather
   than another well-made constructed notation. *Deliverable: formalized reading class + proof for
   it, or the recorded failure per REQ-3.*

**Order rationale:** 1 and 2 need no new theory and produce the "completed and established
Syntax/Grammar" the owner named; 3 is the critical path to UQPL; 4 is what the word "universal"
rests on and can proceed in parallel.

## 4. Preparation executed this turn

- **Corpus live**: four entries extracted from `map.rs` tests, and a harness
  (`corpus_tests.rs`) that reconstructs each through the real implementation and asserts its
  ground truth — including that the genus formula *refuses* where it must. An untier-labeled entry
  fails the harness.
- **Module X packaged** (`curriculum/x-inoculation/`): the trap course, first in build order,
  with the five worked examples and the symmetric guard.
- **`uws/` boundary marked**: the designed/derived cut now stated at the corpus front door,
  pointing at Module S as the re-grounding.
- **Near-miss recorded:** editing `seed/INDEX.yaml` produced a duplicate `school:` key — F-028's
  failure mode recurring in a second file, caught by eye, not by process. **Generator fixed:**
  `check-index.rb` now scans raw text for duplicate keys at every nesting level, verified by
  injection. Two of one shape; the checker is the fix.

## 5. What changed

- `research/notes/README.md` index row; `seed/INDEX.yaml` statuses (acquisition and school →
  in-progress); this note.
- **Open — the Construction phase items 1–4 above.** They are the project now.
