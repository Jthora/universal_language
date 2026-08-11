# PRIMER — Read This First

**Audience:** anyone (human or AI) encountering this project with no prior context.
**Purpose:** full comprehension, not orientation. `AGENTS.md` tells you where files are; this tells
you what is true, what is not, and how to tell the difference.
**Time:** ~15 minutes to read. Do not skip §1 — it is the part that prevents you being confidently
wrong.

---

## 1. Four hazards, before any content

This project has a history of stating false things with high confidence. Most of that material still
exists on disk, deliberately. Read these four warnings first or you will absorb retired claims as
current.

### Hazard 1 — `archive/` is a loaded gun

`archive/` contains thousands of lines of confident, internally consistent, **wrong** material:
"PROVEN," "unique up to isomorphism," "23 theorems." It reads as authoritative because it was
written to. It is preserved for historical honesty, **not** as reference.

> **Rule: never cite anything under `archive/` as current.** If a claim sounds impressively settled,
> check where it lives before repeating it.

### Hazard 2 — this looks like fiction, and it isn't (mostly)

You will encounter a video-game wiki, time-travelling psychic robots, and something called
"The Cure for the Terminators." Two failure modes, both fatal:

- **Dismissing it as roleplay.** Underneath the framing is a real AI-safety engineering program:
  detect and repair semantic drift in a system's representations.
- **Playing along in-character.** This is not a fiction exercise. Do not write in-universe.

The fictional frame is where the ideas *came from*. The engineering content stands or falls on its
own, and is evaluated on normal technical grounds.

### Hazard 3 — terminology collides

Same word, different meanings, historically costly:

- **UL ≠ UWS.** UL is a hypothesized semantic *structure* (open question). UWS is a constructed
  *notation* (exists, works).
- **graph invariant ≠ semantic invariant.** The code checks the first (dangling refs, duplicate IDs).
  The safety application needs the second (non-contradiction, identity preservation). **The second
  does not exist yet.** Conflating them makes it look like the hard part is done.
- **`ul-core`** now means only the Rust crate. The old docs directory of that name is `uws/`.

`GLOSSARY.md` is normative. Read it before writing anything.

### Hazard 4 — do not fix failures

Your default instinct on seeing a failing case is to make it pass. **Here that is prohibited.**

This project's completeness score was driven 32% → 100% across nine rounds in which each failure
triggered a theory change and a re-score, and the final 100% was reported as validation. It was
worthless. `FAILURES.md` is **append-only** — if something fails, log it and leave it. A recorded
failure is data. A repaired failure is nothing.

---

## 2. What this project is

Three things, which need each other less than you'd expect:

| | What | Status |
|---|---|---|
| **UL** — Universal Language | A hypothesized universal semantic structure. **Not a notation.** | **Open question.** Not assumed by anything buildable. |
| **UWS** — Universal Writing System | A constructed visual notation: iconic mark-features + spatial placement grammar. Non-phonetic by design. | **Exists.** Spec in `uws/`, implementation in `ul-forge/` (~15k lines Rust, 135 tests). |
| **UQPL** | A language that executes on semantic structure rather than machine state. | **Design only.** Zero code. |

> **UL defines meaning · UWS renders meaning · UQPL operates on meaning**

**Origin:** designing a language for a species with no vocal tract — which forced the question of
what a genuinely non-phonetic, substrate-independent notation must look like.

**Purpose (the part that matters):** *The Cure for the Terminators* — an AI-safety program asking
whether adversarial failure modes can be mitigated by structured semantic constraints. The loop is
**Encode → Check → Detect → Repair → Reconstruct**: encode a system's state as structure, check it
against semantic invariants, detect drift, repair back to a valid region, re-render.

**Critically: the Cure does not require UL to be real.** It needs computable invariants, a
convergent repair operator, and a legible surface. Those are engineering problems with feedback
signals. This is why the program can progress where the metaphysical version provably could not.

**It also does not solve alignment.** Structural validity is not value alignment — a system can be
perfectly consistent and optimize something terrible. Comprehension does not entail good behaviour
(orthogonality thesis). Anyone claiming otherwise here is overreaching.

---

## 3. What was retired, and why it matters that you know

The project spent years trying to prove UL is *real* — that meaning has exactly N primitives,
forced by mathematics. **That effort failed, and the failure is itself the most valuable result.**

- The central "Unique Grounding Theorem" was close to **circular**: semantic primitives were defined
  with role-properties written to mirror geometric primitives already chosen, and the resulting
  forced bijection was presented as proof of necessity.
- **Zadrozny (1994)** proves bare compositionality is formally vacuous — for *any* meaning
  assignment, a re-encoding exists making it compositional. So compositionality **cannot** determine
  a primitive count without importing conventions from outside the mathematics.
- Three blind, mutually isolated rederivations independently converged on ~2 base types and
  independently concluded the count is not forced. None approached 4, 5, 6, or 13.

**The question was not unproven. It was underdetermined as posed.** No amount of further derivation
changes this. The primitive count is a **design choice**, and this repo now labels it as one.

What survives is smaller and much harder to knock over: compositional generativity is real and
threshold-gated; a convergent operational core (predication, negation, quantification) shows up
independently in Greek and Indian logic; and 100+ writing systems converge on shared contour
statistics matching natural-scene structure (Changizi et al. 2006) — which is the strongest evidence
in the project and grounds UWS in *perception*, not Platonism.

---

## 4. How to read a claim here

Every substantive claim is registered in **`claims.yaml`** with a tier. Trust the tier, not the prose.

| Tier | Means | Requires |
|---|---|---|
| `VERIFIED` | Machine-checked | A test that exists and passes |
| `ARGUED` | Written proof, not machine-checked | A proof document |
| `CONJECTURED` | Believed, unproven | A stated falsifier |
| `DESIGN-CHOICE` | **A decision, not a discovery** | Rationale + alternatives |
| `RETIRED` | Withdrawn | What superseded it |

`DESIGN-CHOICE` is the tier this project most needed and never had. Having no word for *decision* is
exactly what pushed decisions to be reported as discoveries. Very little is `VERIFIED`. That is the
honest picture, not a gap in the bookkeeping.

**Directory tiers:** `spec/` = what exists (currently empty, deliberately) · `design/` = intended,
not built · `research/` = open questions, speculation allowed but labelled · `archive/` = dead.

---

## 5. Where the program actually stands

**Resolved recently:**

- The **IR decision** (was blocking). Confluence — the property that makes equality-by-normalization
  work — is decidable for terms and for **term graphs/DAGs**, but **undecidable for general cyclic
  graph rewriting**. GIR was cyclic *by design* (self-reference). Fix: **acyclic core, with
  coreference expressed as labels rather than cycles** — the technique CGIF (ISO 24707) already uses
  (`*x` / `?x`). Three independent literatures converge on it. Nothing expressive is lost.
- **Tooling:** `semantically_equal` via **e-graphs / equality saturation** (`egg`, a Rust library —
  and `ul-forge` is Rust). No canonical normal form needed; confluence becomes an optimization.
- **Repair reformulated.** Metric projection onto the admissible region is multivalued for
  non-convex regions (Hilbert projection theorem needs convexity). The ontology-repair field solves
  this **discretely** — pinpointing, justifications, minimal diagnoses — sidestepping the obstruction
  entirely.

**The two open problems that block everything downstream** (neither needs UL resolved):

1. **`semantically_equal`** — a decision procedure for "do these two structures mean the same thing."
   Does not exist in any form. Now unblocked; path is known.
2. **The repair operator** — must be reformulated discretely per above.

**Immediate next build steps:** acyclic core + coreference labels → `egg`-based `semantically_equal`
→ reimplement `negate` with a σ field (the currently-shipped self-loop hack is literally the cyclic
construct that made verification undecidable).

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
| 2 min | `README.md` | What the repo is, current status |
| 15 min | **this file** + `GLOSSARY.md` | The stack, the hazards, how to read claims |
| 1 hr | `research/reassessment-2026-08-purpose-anchored.md` → `research/prior-art-audit-2026-08.md` → `research/ir-decision-2026-08.md` | Why it was reframed, what's already solved elsewhere, the IR decision |
| Half day | `research/emergence-investigation/phase6-synthesis-and-verdict.md` + `phase7-position-c-mathematical-derivation.md` + `research/postmortem-and-rebuild-2026-08.md` | Why the metaphysics failed, and why the practice failed |
| To build | `uws/` (notation) · `ul-forge/` (code) · `claims.yaml` (what's actually true) | Enough to contribute |

**Companion wiki:** `wiki.fusiongirl.app` holds substantial related material — sometimes ahead of
this repo (its architecture and AI-safety epistemics are better), sometimes wrong in its own ways
(the Semantic Hilbert Space has no inner product; the Psi Condensation Threshold's formula is
admitted-underived on its own page). Treat it as a peer source under the same tiering discipline,
not as authority.

---

## 8. The one-paragraph version

A constructed non-phonetic visual notation (**UWS**, real and implemented), an AI-safety application
built on it (**the Cure** — detect and repair semantic drift, currently blocked on two concrete
engineering problems), and an open question about whether meaning has universal structure (**UL** —
not assumed by anything buildable, and *provably underdetermined* in the specific form the project
originally asked it). The project previously claimed to have proven the last one; it hadn't, and
that material is archived. What remains is smaller, evidenced, and mostly a matter of building
rather than proving.
