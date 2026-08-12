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
| `024` | cycle | **Junction axis derived.** Degree is topological; Angle is degree-2 ∧ singular. **The fixed point is the rotation system (Heffter–Edmonds)** — separation and incidence are one object, and the canonical table is indexed by degree | `FIXED-POINT-IS-COMBINATORIAL-MAP`; `JUNCTION-AXIS` derived; `014` scoped to single curves |
| `025` | cycle | **Many tables derived.** A table is a projection of the 4-axis product; symbols recur because cells project into all of them; C(n,2) tables for n axes — **so counting tables counts axes** | `TABLES-ARE-PROJECTIONS` |
| `026` | cycle | **Two kinds of axis.** Shape vs transformation — the latter are *group elements*, so the structure is a G-set and Erlangen IS the quotient. Cross/X table = line arrangements; **nonstretchability is the first concrete expressiveness gap** | `TWO-KINDS-OF-AXIS`, `NONSTRETCHABILITY-BOUNDS-DRAWING`; `025` corrected |
| `027` | decision | **Phase status and next research.** Phases 0–2 closed; Phase 3 half-done from an unplanned direction. Mnëv + nonstretchability characterize the map/drawing gap; dessins recorded as precedent and fenced | `MAP-TO-DRAWING-GAP`, `DESSINS-PRECEDENT`; plan revised |
| `028` | cycle | **Phase 3 closed.** Apartness prediction *failed* — constructive geometry uses stable equality. Expressiveness chain: lower two links characterized, **meaning→map does not exist**. Base-6 dismissed as an enumeration error | `EXPRESSIVENESS-CHAIN`; `SEMANTIC-APARTNESS` narrowed; Tarski added as a fourth primitivity position |
| `029` | decision | **Phase 4 closed.** RCC *derived* from the map's faces, decidable by construction; Cure is a **comparator, not a self-prover**; **entrenchment = coarse-graining survival order**, so the Cure's last free parameter is derived | `REGIONS-ARE-FACE-UNIONS`, `CURE-IS-COMPARATOR`, `ENTRENCHMENT-IS-SURVIVAL-ORDER` |
| `030` | cycle | **Phase 5 begun — the fixed point in code.** Rotation system, face tracing, degree, genus. Jordan separation now an executable test. A failing test caught a *torus* embedding, not a bug | `IMPL-COMBINATORIAL-MAP` — the project's **first substantive VERIFIED claim** |
| `031` | cycle | **Junction clarified; octahedral question tested.** Angle = degree-2 ∧ corner; degree-1 unnamed. Four binary axes **are** hyper-octahedral — but junction degree is unbounded and breaks it, which explains why the cross/X table exploded | `BINARY-AXES-ARE-HYPEROCTAHEDRAL`; a checkable 3-vs-4 axis discrepancy |
| `032` | **correction** | **Axis audit.** Two of four axes underdetermined; **connectivity missing**; and `FIXED-POINT-IS-COMBINATORIAL-MAP` had dropped the word *connected* from a theorem it quoted verbatim — caught by a failing test | Claim scoped; `CONNECTIVITY-AXIS-MISSING`; **F-025** |
| `033` | cycle | **Axis types, and the UL/UWS focus audit.** Only one axis is binary; curvature is a *function space* with canonical integer invariants (Whitney–Graustein) — repairing `032`. And UWS work bears on UL **only at the fixed point** | `AXES-HAVE-DIFFERENT-TYPES`, `UL-WORK-IS-FIXED-POINT-WORK`; curvature axis repaired |
| `034` | cycle | **Bridge test — and it went against us.** Semantic formalisms use *labelled* edges; AMR discards word order deliberately; spatial layout elsewhere is secondary notation. The map is **UWS's** fixed point, not established as UL's | `SEMANTIC-FORMALISMS-USE-LABELS-NOT-ORDER`; `FIXED-POINT-IS-COMBINATORIAL-MAP` scoped again |
| `035` | cycle | **The map is protocol-optimal, not semantic.** Labels break Sₙ, rotation breaks only ℤ/2 — minimal shared prior. Formal home in Other-Play / zero-shot coordination. **UP is no longer the empty construct** | `ROTATION-MINIMIZES-CONVENTION`; retreat-guard applied and passed |
| `036` | cycle | **"Music/love is the universal language" — checked, not dismissed.** Music splits exactly as `035` predicts: transposition invariance is cross-culturally universal, octave equivalence is not. Relational travels, conventional does not | `UNIVERSAL-MEANS-CONVENTION-FREE`; cross-domain support for `ROTATION-MINIMIZES-CONVENTION` |
| `037` | cycle | **The universal-language landscape.** Three incompatible kinds of claim — invented notation, discovered structure, mode of engagement. Kind A has a 300-year uniform failure record; **Solresol is the natural experiment**; the genetic code is constrained but not determined | `THREE-KINDS-OF-UNIVERSAL-CLAIM`, `GENETIC-CODE-IS-CONSTRAINED-NOT-DETERMINED`; **F-026** |
| `038` | **correction** | **UWS is a *derived* notation, mis-filed as invented across four notes.** The taxonomy collapsed *what it is* with *how it is arrived at*. And `034`'s AMR evidence was already recorded as inadmissible by this repo | Taxonomy corrected to a grid; `034` narrowed to one formalism; **T12**; **F-027** |
| `039` | **correction** | **Consensus review — run.** Mechanism measured: 38 notes vs 9 syntheses; 39 of 99 claims at priority 0; **0 of 5 dependents carry a thrice-scoped hub's corrections**. First live casualty: the Phase 4 decidability result | Review method set; `REGIONS-ARE-FACE-UNIONS` flagged as wrong-for-real-notations |
| `040` | cycle | **Nesting implemented** — fixes `039`'s material finding. Two triangles now trace 3 faces not 4, χ = 1+c; side-by-side vs contained give the same count but different structure, which is what RCC-8 needs | `REGIONS-ARE-FACE-UNIONS` now correct for disconnected configurations |
| `041` | cycle | **Convention cost machine-checked** — mirror is an involution, so orientation really is ℤ/2; label cost is k! (40 320 at k=8). The vertex-correspondence failure mode was checked for and absent. **Behavioural half still untested** | `IMPL-CONVENTION-COST` (VERIFIED); one test flagged as near-tautological |
| — | — | *(F-028: a duplicate YAML key silently discarded a scope review. Caught by one checker disagreeing with the visible file. `check-claims.rb` now scans raw text, since a post-parse validator is blind to what the parser dropped.)* | |
| `042` | **correction** | **The note read I skipped in `039`.** Ten notes had contradictory status; **zero of nine corrections had backward links** — supersession was forward-only. Third instance of one failure class | `tools/check-notes.rb`; supersession now bidirectional |
| `043` | **correction** | **Search retrospective.** ~40 searches, **4 primary reads — all four changed a claim.** Found dessins by accident 30 notes late; ran the failure survey at note 37 of 43 | **S8–S11**: primary-before-registering, synonym sweep, failure-first, scope-before-cite |
| `044` | decision | **Multi-volley research plan.** Failure-first, then synonym sweep, then primary reads. Scan found **DCEL/half-edge never searched** — we may have reinvented a 1970s structure | **Open** — volleys 1–2 closed, 3–5 remain |
| `045` | cycle | **Volley 1 — the graveyards.** Ontology failure is *governance*, not detection: models go stale in two quarters, maintenance is "in practice no maintenance at all". METI independently corroborates that pictures are not self-evident. Derived-notation cell confirmed empty — but **Babbage listed iconicity and structural symmetry in the 1830s** | `CURE-MUST-BE-AUTOMATIC`; 3 of 4 predictions correct incl. the uncomfortable one |
| `046` | cycle | **Volley 2 — the synonym sweep.** Prediction confirmed: `map.rs` **is a DCEL with the geometry deleted**, and `Nesting` re-derived one of the field's *two* standard fixes — the other, **dummy edges**, restores Heffter-Edmonds' connectivity precondition instead of working around it (verified as a test). The semantic surprise **did not occur**: `034`'s negative now holds across **six** vocabularies | `MAP-IS-A-DCEL-REDUCT`, `NESTING-HAS-A-STANDARD-ALTERNATIVE`, `ORIENTATION-SPLITS-INTO-TWO-CANDIDATES`; **F-029**, **S12** |
