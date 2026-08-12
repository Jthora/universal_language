# research/notes/

**The trajectory.** Research, brainstorms, and course corrections, in the order they happened.

This is the only place in the repo that records *how thinking changed*. It is deliberately distinct
from its two neighbours:

| | Records | Shape |
|---|---|---|
| `claims.yaml` | **what we hold now** | current state only — no history |
| `FAILURES.md` | **what went wrong** | append-only, one slice of the trajectory |
| **`notes/`** | **how we got here** | the full arc, including reversals |

A position that changed is not noise to be tidied away. **The reversal is usually more informative
than either endpoint**, because it shows what was mistaken for evidence.

---

## How it evolves

**Individual notes are immutable. The collection is what evolves.** A note is a record of what was
thought at a particular time; editing it destroys the only copy of that. When a note is overtaken,
you write a new one and link back — supersession is explicit, never silent.

That is exactly how the Σ_UL retirement went wrong: the claim was withdrawn but nothing traced the
change, so its artifacts survived for months and kept being restated as current.

```
notes/
├── README.md
└── NNN-slug/
    ├── README.md      the note
    └── …              data, derivations, scripts
```

- **Number sequentially, never renumber.**
- **Never edit a note after it closes.** Corrections go in a new note that links back.
- **Supersession is recorded in both directions** — the old note gets a `Superseded by:` line, which
  is the one permitted edit.
- **Promotion is explicit.** When a position stabilizes, move it into `framework/`, `cure/`, or
  `engineering/`. Notes are working material; the themed folders hold what we currently hold.

## Types

| Type | For |
|---|---|
| `cycle` | A research push — searches run, literature checked, findings recorded |
| `brainstorm` | Generative and exploratory. **Not everything needs a citation.** Speculation is permitted here and must be labelled |
| `correction` | **Something we held turned out wrong.** What was believed, what broke it, what replaced it |
| `decision` | A fork chosen, with the alternatives and why |

`brainstorm` matters as much as `cycle`. Several of the strongest positions here began as an
unsourced hunch that survived checking — the primitives-as-generating-sets insight started that way.
Requiring a citation to open a thought is how you lose those.

## Template

```markdown
# NNN — <title>

**Type:** cycle | brainstorm | correction | decision
**Opened:** YYYY-MM-DD
**Status:** open | closed | superseded by NNN
**Thread:** follows NNN · corrects NNN
**Question:** <the one thing this is trying to settle>

## 1. Before searching  ← WRITE THIS FIRST (S2)
**Expected to find:**
**Would change the plan if:**
**Objects mathematical?** yes/no — if yes, find the theorem, don't design a study (T5)

## 2. Searches run
| Query | Direction | Result |
|---|---|---|
| … | supporting / **adversarial (R1)** | … |

A cycle that closes a line of work with no adversarial row is **not finished**.

## 3. Findings
**Counter-evidence: easy or hard to find?** ← log it (S7)

## 4. Negatives recorded
Each needs: **scope** (R2) · **which formalization failed, not which claim** (R3) ·
**revival condition** (R4) · **steelman stated before the kill** (R6)

## 5. What changed
- `claims.yaml`:
- `FAILURES.md`:
- Promoted to:
- Left open:
```

For a `correction`, replace §§1–2 with:

```markdown
## 1. What was held
## 2. What broke it
## 3. What replaced it
## 4. Why it survived as long as it did   ← the generalizable part
```

§4 is the one that pays. Two corrections of the same shape means the *generator* needs fixing, not
the instances.

---

## Trajectory

Course corrections in **bold** — the load-bearing entries.

| # | Type | What happened | Outcome |
|---|---|---|---|
| `001` | cycle | Where purpose, notation and safety application meet | Cure benchmark; emergent-communication gap identified |
| `002` | cycle | Open questions from 001 | SHACL checking; sheaf route; taught-notation |
| `003` | **correction** | **Three kinds of universality collapsed into two.** Zadrozny applied as a global impossibility when it is a microscopic result | UL reframed as an **emergent universality class** → `framework/emergent-universality.md` |
| `004` | brainstorm | Primitive counts arbitrary; Chomsky too human-centric | Generating sets not bases; curvature stratification → `framework/provable-geometry.md`, `framework/cross-substrate-grammar.md` |
| `005` | **correction** | **Reached for experiment where proof was available.** Empiricism adopted as the antidote to a disease proof cures better | Proof turn; F-015 |
| `006` | cycle | Which open claims are provable? | Löb, Rice, AGM, Landau → `engineering/obstructions.md` |
| `007` | brainstorm | Is the external reference the mathematics itself? | Geometry as **trust anchor**; drift must be absolute → `framework/external-anchor.md` |
| `008` | **correction** | **Seven negatives closed lines of work that were still live.** Asymmetric scrutiny producing skeptical-sounding output | R1–R6, S1–S7; `RESEARCH-PROTOCOL.md`; F-017 |
| `009` | decision | Superseded material kept in-repo was being read as current | Deleted; git history is the archive. 224 → 49 files |
| `010` | brainstorm | Which literature crawls would most change what we can see? | Eight rosters, falsification-first. **Open** |
| `011` | **correction** | **Zadrozny had never had an adversarial search run against it.** Sound theorem, over-broad conclusion — its encoding does not preserve synonymy | Necessity route reopened; fixed-point table is canonical; F-019 |

| `012` | cycle | **Roster A — adversarial search against our own load-bearing legs.** TopSim damaged; Newman survives as a constraint; Erlangen↔RG burden raised | `TOPSIM-INSUFFICIENT`, `NEWMAN-OBJECTION`; compositionality claim downgraded ARGUED→CONJECTURED |
| `013` | cycle | **Roster G — emergent communication.** Symbolic protocols fail cross-play (permutation invariance); **iconic/spatial ones succeed**. Compositionality needs two pressures | `SYMBOLIC-PROTOCOLS-FAIL-CROSSPLAY`, `ICONIC-GROUNDING-ENABLES-CROSSPLAY`, `COMPOSITIONALITY-REQUIRES-PRESSURE` |
| `014` | cycle | **Erlangen survival table derived case by case**, having been asserted in prose since `005`. Four corrections; Newman partially answered | `ERLANGEN-SURVIVAL-RANKING` re-warranted; `NEWMAN-OBJECTION` advanced |
| `015` | **correction** | **The dependency-order test was ill-posed** — Euclid's first nine definitions do no mathematical work, and definitional ≠ ontological dependence. Promoted a dozen turns unchecked | Test RETIRED with revival condition; **R8**; F-020; co-primitive convergence salvaged |
| `016` | **correction** | **`015` over-reached.** The point IS prior — in Euclid's *postulates*, which are constructive and load-bearing, not his definitions. Two rival traditions explained by two co-primitives | `CONSTRUCTIVE-PRIORITY-OF-POINT`; dependency test revival gains a third route |
| `017` | brainstorm | Pathways after the constructive turn — constructibility as an expressiveness bound, mereotopology for two co-primitives, remaining prose-asserted claims | **Open** |
| `018` | decision | **Execution plan** — fifteen items in six phases, enforcement first. **Phase 0 closed** | `tools/check.rb` + CI; F-018 encoded as a check |
| `019` | cycle | **Phase 1 · D2 — primary reads.** Sketching claim narrows sharply: no modality ablation, populations share architecture and dataset. Zadrozny scope confirmed and sharpened to a three-part constraint | `ICONIC-GROUNDING-ENABLES-CROSSPLAY` narrowed; **T11**; F-021 |
| `020` | cycle | **Phase 1 · A3 — coherence check.** The notation's own negation law is classical and contradicts its constructive framing. Apartness surfaces as the constructively correct primitive | `NEG-INVOLUTION` RETIRED; `NOTATION-LOGIC-IS-INTUITIONISTIC`; **`SEMANTIC-APARTNESS`**; F-022 |
| `021` | cycle | **Phase 1 · D1 — the R7 sweep.** Rice was never applicable to GIR; Löb holds with better escapes and independently supports the anchor; Plump confirmed | **Phase 1 closed.** `RICE-BOUNDS-SEMANTIC-EQUALITY` scope corrected; `SEMANTIC-EQUALITY` un-blocked; F-023 |
| `022` | cycle | **Phase 2 — derivations.** κ-classification is a product of three axes, not one stratification, and that *explains* which primitives survive. Jordan holds; the Spencer-Brown identification imports Boolean logic and is withdrawn | **Phase 2 closed.** `KAPPA-STRATIFICATION` derived; `ENCLOSURE-IS-DISTINCTION` split; F-024 |
| `023` | cycle | **Does 2D constrain the geometry?** Split: iconicity is dimension-bound, encoding is not (Coxeter diagrams). Anchor is dimension-general (Jordan–Brouwer). **Junction topology identified as a missing axis** | `DIMENSION-BOUNDS-ICONICITY-NOT-ENCODING`, `JUNCTION-AXIS-MISSING` |
