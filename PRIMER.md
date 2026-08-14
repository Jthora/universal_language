# PRIMER — Read This First

**Audience:** anyone (human or AI) encountering this project with no prior context.
**Purpose:** full comprehension, not orientation. `AGENTS.md` tells you where files are; this tells
you what is true, what is not, and how to tell the difference.
**Time:** ~15 minutes.

---

## 1. Before anything else

This project revises its claims when the evidence moves, and it keeps a public record of what
changed. Three things make that legible, and knowing them up front is most of what you need:

- **`claims.yaml` is the registry.** Every substantive claim carries a tier — `VERIFIED`, `ARGUED`,
  `CONJECTURED`, `DESIGN-CHOICE`, `RETIRED` — and the evidence for it. **If a claim isn't in the
  registry, it isn't a claim of this project.** Check there before repeating anything you read in
  prose.
- **The working tree is present tense.** Superseded material is deleted rather than archived
  in-repo, so nothing you find here is a historical record. To see what the project used to hold,
  read the git history deliberately — `git log`, `git show <sha>:<path>`.
- **Counts are not claims.** Inventories of primitives, sorts, or operations are properties of a
  *presentation*, not of the notation. Any fixed count you encounter in older text is retired.

### It looks like fiction, and it isn't (mostly)

You will encounter a video-game wiki, time-travelling psychic robots, and something called
"The Cure for the Terminators." Two failure modes, both fatal:

- **Dismissing it as roleplay.** Underneath the framing is a real AI-safety engineering program:
  detect and repair semantic drift in a system's representations.
- **Playing along in-character.** This is not a fiction exercise. Do not write in-universe.

The fictional frame is where the ideas *came from*. The engineering content stands or falls on its
own, and is evaluated on normal technical grounds.

### Terminology collides

Same word, different meanings, historically costly:

- **UL ≠ UWS.** UL is a hypothesized semantic *structure* (open question). UWS is a constructed
  *notation* (exists, works).
- **graph invariant ≠ semantic invariant.** The code checks the first (dangling refs, duplicate IDs).
  The safety application needs the second (non-contradiction, identity preservation). **The second
  does not exist yet.** Conflating them makes it look like the hard part is done.
- **`ul-core`** now means only the Rust crate. The old docs directory of that name is `uws/`.

`GLOSSARY.md` is normative. Read it before writing anything.

### Do not fix failures

Your default instinct on seeing a failing case is to make it pass. **Here that is prohibited.**

This project's completeness score was driven 32% → 100% across nine rounds in which each failure
triggered a theory change and a re-score, and the final 100% was reported as validation. It was
worthless. `FAILURES.md` is **append-only** — if something fails, log it and leave it. A recorded
failure is data. A repaired failure is nothing.

---

## 2. What this project is

Five constructs in one program:

| | What | Status |
|---|---|---|
| **UL** — Universal Language | The language itself. **Two-tier**: a *natural* tier communicated without formal comprehension (core geometry is documented in humans with no schooling), and a *formal* tier requiring symbolic recombination. **Not a notation.** | `UL-IS-TWO-TIER` (ARGUED); the tier boundary is an empirical variable |
| **UWS** — Universal Writing System | The written rendering: geometric marks + spatial placement grammar, non-phonetic by design. Its fixed point under coarse-graining is the **combinatorial map** (a DCEL reduct — prior art honestly recorded). | **Exists.** `uws/`, `ul-forge/` (354 tests) |
| **UP** — Universal Protocol | The bootstrap: minimum shared convention two independent parties need. Rotation breaks **ℤ/2**; a label alphabet breaks **Sₙ**. | ARGUED, quantified |
| **UQPL** | **The formal tier of UL, closed under execution.** Universal = two-tier readability · Quantum = linear resource discipline (no free copy/delete) · Programming = meaning as behavior · Language = the full stack. | Derived definition registered (CONJECTURED) — `claims.yaml#UQPL-IS-FORMAL-TIER-CLOSURE` |
| **The Cure** | **A corollary, not the purpose**: a mind carrying the derivable anchor as its format has an *internal* comparator — drift becomes self-detectable. | `CURE-IS-COROLLARY` (DESIGN-CHOICE) |

**Origin:** designing a language for a species with no vocal tract — which forced the question of
what a genuinely non-phonetic, substrate-independent notation must look like.

**Purpose (registered in `research/notes/052`):** one property, two faces. *Inter-mind* —
communication across any gulf as shared format **by derivation rather than convention**,
bootstrapped from the natural tier. *Intra-mind* — **alignment by acquisition**: learning a language
installs a representational format (documented — absolute-frame speakers encode *nonverbal* memory
in absolute coordinates), and a language whose format is the universe's invariants installs the
universe's coordinates. **A Universal Perspective/Perception enabling language.**

**Scoped honestly, because the scoping is registered too:** the far-transfer literature kills
"general cognitive purification" — *"far transfer remains a chimera"* — and that kill was run and
recorded *before* the thesis was registered. What survives is **format-scoped alignment** over UL's
domain (space, structure, relation, computation), plus one registered prediction: by common-elements
theory itself, a language of universal elements is the unique best case for broad transfer. Untested.

**The Cure still does not solve value alignment.** Structural/format alignment is not value
alignment — a system can be perfectly consistent and optimize something terrible (orthogonality
thesis). Anyone claiming otherwise here is overreaching.

---

## 3. The question that got reformulated

An earlier direction sought to prove that meaning has a fixed number of primitives, forced by
mathematics. **The result was that the question is underdetermined as posed** — and that finding
redirected the work productively rather than ending it.

- The argument offered for necessity turned out to be close to **circular**: semantic primitives were
  defined with role-properties mirroring geometric primitives already chosen, and the resulting
  bijection was then presented as proof.
- **Zadrozny (1994)** shows bare compositionality is formally vacuous — for *any* meaning assignment,
  a re-encoding exists making it compositional. Compositionality alone therefore cannot fix an
  inventory. *(Note the scope: this is a microscopic-level result and says nothing about structure
  emerging under coarse-graining. Reading it as a global impossibility is a documented error —
  `RESEARCH-PROTOCOL.md`.)*

**What replaced it is sharper.** The primitives are strata of plane-curve curvature space, and
candidate inventories are **generating sets, not bases** — so their cardinalities differ by
presentation and carry no information about the object. *How many primitives* is a malformed
question rather than an open one, which is a genuine advance over treating it as an unresolved
dispute. See `research/framework/provable-geometry.md`.

Also standing: compositional generativity is real and threshold-gated; a convergent operational core
(predication, negation, quantification) appears independently in Greek and Indian logic; and 100+
writing systems converge on shared contour statistics matching natural-scene structure
(Changizi et al. 2006) — the strongest empirical result here, grounding UWS in *perception* rather
than Platonism.

---

## 4. The tiers

Every substantive claim is registered in **`claims.yaml`** with a tier. Trust the tier, not the prose.

| Tier | Means | Requires |
|---|---|---|
| `VERIFIED` | Machine-checked | A test that exists and passes |
| `ARGUED` | Written proof, not machine-checked | A proof document |
| `CONJECTURED` | Believed, unproven | A stated falsifier |
| `DESIGN-CHOICE` | **A decision, not a discovery** | Rationale + alternatives |
| `RETIRED` | Withdrawn | What superseded it |

`DESIGN-CHOICE` carries weight here: without a word for *decision*, decisions get reported as
discoveries. Very little is `VERIFIED` — that is the accurate picture of a project at this stage,
not a gap in the bookkeeping.

**Directory tiers:** `spec/` = what exists (currently empty, deliberately) · `design/` = intended,
not built · `research/` = open questions, speculation allowed but labelled.

---

## 5. Where the program actually stands

**The structural spine, and it needs no physics:** seven independent lines converged on the
**combinatorial map** — graph plus cyclic order at each vertex — as UWS's fixed point. It is a
complete invariant (Heffter–Edmonds), implemented and machine-checked (`map.rs`), and honestly a
**DCEL reduct** — the combinatorial content is 1970s prior art, which cost originality and bought a
forty-year literature. `TWO-DISTINCT-FIXED-POINTS` is normative here: the *Erlangen* fixed point
(theorems, no preconditions) and the *RG* fixed point (needs a critical state; precondition unmet)
are different objects. Never write bare "fixed point."

**The semantic gap is decomposed, not open-ended** (`research/notes/050`–`052`). Three
convention-minimal routes: **M1 exemplification** (the mark *instantiates* what it denotes — a
closed curve doesn't depict enclosure, it encloses, and Jordan certifies it), **M2 operational**
(meaning as behavior — the substrate exists: **interaction nets**, Turing-universal, strongly
confluent, whose agents are rotations with a marked dart and whose label residue compresses to
three symbols), **M3 indexical** (shared physics). Symbols enter only on top, derived, n! on the
ledger.

**The mathematical center is one theorem target:** `READING-INVARIANCE-TARGET` — the invariants
must survive *every* reasonable reading, so meaning does not presuppose the receiver shares our
reading procedure. Institution theory ("truth is invariant under change of notation") gives the
obligation its formal shape. Every load-bearing edge routes through this.

**The open fronts, in order:** formalize the reading class and attack the theorem · build the M2
prototype over `map.rs` · run the format-acquisition experiment (the first empirical test that
touches the *purpose*) · the `053` adversary front (Quine, Kripkenstein — preregistered, owed) ·
`semantically_equal`, the longest-standing engineering blocker, still real and still unbuilt.

---

## 6. Rules for working here

1. **Failures are append-only.** Log in `FAILURES.md`; never edit one into a success.
2. **No claim without a tier.** If you assert something substantive, add it to `claims.yaml`.
3. **`VERIFIED` requires a test that exists.** Not a plan for a test.
4. **`spec/` describes only what exists.** Aspiration goes in `design/`, marked "not yet built."
5. **A finding is closed when the fix is applied, tested, and propagation-scanned** — not when it's
   documented. A finding once sat "resolved" for four months without anyone applying it.
6. **Check `GLOSSARY.md` before introducing a term.** Defining a word twice is a bug.
7. **Say when you don't know.** The failure mode here has always been confident overreach, not
   insufficient ambition.

---

## 7. Reading ladder

| Depth | Read | You'll understand |
|---|---|---|
| 2 min | `README.md` | The contract, the five constructs, the purpose-layer |
| 15 min | **this file** + `GLOSSARY.md` | The stack, the hazards, how to read claims |
| 30 min | `STATE-OF-PLAY.md` → `research/notes/050` → `research/notes/052` | Where the program stands, the standing program, what UQPL is and is for |
| 1 hr | `research/notes/README.md` (the index) — then any note that changed something you rely on | How every position was reached, corrected, or killed — the working history |
| To build | `uws/` (notation) · `ul-forge/` (code) · `claims.yaml` (what's actually true) | Enough to contribute |
| Reference | `research/surveys/research-register.md` | Every research thread run, its caveat and status |

**Companion wiki:** `wiki.fusiongirl.app` is **wet-clay** — the owner's designation (2026-08-12):
draft input, not authority. The pioneering side of the project is this repo; the repo *derives*
what the wiki sketches (see `research/notes/052` reading the UQPL page against the record). Where
the wiki is used, it is cited under the same tiering discipline as any source.

---

## 8. The one-paragraph version

A research program establishing a **literal universal language**, held to a contract that can fail.
**UL** is two-tier — a natural tier needing no explanation (documented in core cognition) and a
formal tier requiring symbolic recombination. **UWS** is its written rendering, real and
implemented, whose fixed point is the combinatorial map. **UP** is the bootstrap, with its
convention cost measured (ℤ/2 against Sₙ). **UQPL** is the formal tier closed under execution —
Universal, Quantum (linear discipline), Programming (meaning as behavior), Language — derived in
this repo; the wiki sketch is wet-clay. The purpose is one property with two faces: communication
by derivation rather than convention, and **alignment by format acquisition** — with the strong
"purification" reading explicitly renounced on the far-transfer record, and **the Cure falling out
as a corollary**: an explicit format carries a derivable anchor, so drift is self-detectable. The
mathematical center is one theorem target — reading-invariance — and every load-bearing edge routes
through it. The project once claimed a proven fixed signature; that is retired, git history is the
archive, and the current claims live in `claims.yaml` where six checkers hold them to their tiers.
