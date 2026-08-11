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
dispute. See `research/what-is-actually-provable-2026-08.md`.

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
| Reference | `research/RESEARCH-REGISTER.md` | Every research thread run, what it returned, its caveat and status — the index, not an argument |

> **Note (2026-08-01):** this primer predates three later findings — that the Cure is a rediscovery
> of Truth Maintenance Systems (Doyle 1979 / de Kleer 1986), that invariants may be *inferred*
> rather than specified (Daikon-style), and that prevention-at-write-time is an unexamined
> alternative to detect-then-repair. See `research/RESEARCH-REGISTER.md` §E and §H.

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
