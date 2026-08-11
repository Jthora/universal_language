# Emergence Investigation — Is Universal Language Real, and What Would Prove It?

**Status:** ✅ All six phases complete (2026-08-01) — see `phase6-synthesis-and-verdict.md` for the
consolidated verdict. Two phases (4, 5) are designs only, pending compute/API budget for execution.
**Update (same day):** the previously-outstanding blind-rederivation pillar was executed as part of
a follow-on "Phase 7" (`phase7-position-c-mathematical-derivation.md`), pursuing direct mathematical
derivation rather than survey at the user's request. Headline result: Zadrozny's (1994) theorem
proves bare compositionality cannot mathematically determine any specific primitive count without
external naturalness conventions — sharper and more final than Phase 6's own verdict, not a
reversal of it. Three independent blind rederivation attempts converged on ~2 sorts +
application-as-core while independently reaching the same "not uniquely forced" conclusion.
**Date:** 2026-08-01
**Prerequisite context:** `docs/planning/audits/wiki-comparison-2026-08.md` (repo-vs-wiki audit + foundational
coherence check that motivated this investigation)
**Working stance:** We are operating on the hypothesis that Universal Language is real and asking
what would actually demonstrate that — not defending the current Σ_UL claims as-is, and not
disposing of the project. If a specific claim (e.g. "exactly 5 primitives") doesn't survive, that's
a result to report, not a failure to patch over.

---

## Where we got here from (context for future sessions)

A repo-vs-wiki content audit surfaced that the "5 primitives / 4 sorts / 13 operations / 23
theorems" Σ_UL signature is asserted identically by both the repo and the wiki (same project, two
surfaces) — but a direct read of the proofs found the flagship "Unique Grounding Theorem" is close
to circular (5 semantic primitives defined to structurally mirror 5 geometric ones, then a
bijection between the two hand-matched lists is called "proof"), and the repo's own internal audit
already flagged this exact tension in April 2026 (finding F7) without ever fixing the public docs.

A first-principles rederivation (ignoring Σ_UL's specific framing) found that the actual
minimal-signature answer, under 80 years of settled model theory / Montague grammar, is **2 base
types** (`e`, `t`), not 4, 5, 6, or 13 of anything — the larger counts are a *representational
flattening choice* (for drawability/writability), not a forced mathematical minimum. This explains,
rather than merely notes, why the repo (5) and the wiki's octahedral system (6) disagree: the count
was never pinned down by anything upstream.

The user then sharpened the actual target: not "a good notation," but a **mind-independent, real**
structure — literally present in reality, transcending human vocalization. Checking that claim
against real semiotic theory (Peirce's triadic sign model — signification requires an interpretant,
so meaning cannot be a property of dead matter alone) refined the target further, to the position
biosemiotics (Sebeok, Hoffmeyer, Deacon) actually holds: **semiosis is coextensive with life,
independent of species or substrate, but not with inanimate physics.** That's the working hypothesis
this plan investigates.

---

## The sharpened, falsifiable hypothesis

> Compositional generativity (the capacity to build unboundedly many novel relational meanings from
> finite parts) requires crossing a specific informational/cognitive threshold. Any living system
> that crosses this threshold — regardless of species or substrate — converges on structurally
> comparable primitive categories, grounded in mathematical/logical structure that is itself
> independently convergent (as evidenced by unconnected human civilizations arriving at the same
> core mathematics).

This explicitly does **not** commit to any specific primitive count (4, 5, 6, 13...) as the
invariant — evidence so far suggests the count is a design/flattening parameter, not a discovered
constant. The invariants under test are: (a) the *threshold* itself, (b) whatever mathematical
structure is *actually* independently convergent, and (c) whether full relational-algebra
properties (negation, quantification, embedding — not just compositional syntax) appear anywhere
outside humans.

---

## Process note (added after Phases 2–3, applies to all remaining phases)

Both Phase 2 and Phase 3 needed a second pass, and the common cause wasn't wrong facts — it was
**verification discipline dropping specifically on phases that don't force external checking by
their own shape.** A literature-review phase (Phase 1) forces search-and-check by construction. A
"solo/technical" or "design" phase (Phase 3, and Phase 4 below) doesn't — the only check available
is my own confidence, which is exactly the thing this investigation exists to not trust (it's the
same failure mode that produced the original circular Unique Grounding Theorem). Going forward:
**any phase whose main activity is reasoning/design rather than literature review gets an explicit
"check the technical claims and existing prior work before drafting" step, not after.** Phase 4
below applies this from the start rather than needing a correction pass afterward.

## Phase 0 — Pre-registration discipline (do this first, keep it honest)

**Why:** The repo's own resolution log (`ul-core/CRITIQUE.md`) shows its internal "D2 completeness"
score climbing from 32% to 100% via nine sequential patches, each one invented after a specific
case failed. That pattern must not repeat here. Before gathering more evidence:

- [ ] Write down, in this file, exactly what would count as confirming vs. falsifying evidence for
      each phase below — *before* doing the phase's research.
- [ ] If a phase's finding fails the stated prediction, record it as a failure in this file. Do not
      quietly redefine the prediction after the fact.
- [ ] Explicitly separate four sub-claims that keep getting merged: (1) geometry can encode
      relational content, (2) compositional/hierarchical generativity is real and general, (3) a
      *specific* primitive count is forced, (4) the whole thing is grounded in convergent math. (3)
      is currently the weakest and should not be assumed true just because (1) and (2) hold.

---

## Phase 1 — Convergent Mathematics Audit

**Status:** ✅ Complete (first pass) — see `phase1-convergent-mathematics-audit.md`.

**Summary of findings:** Partially falsified, partially supported. Real independent convergence
confirmed for arithmetic/geometry (positional numerals + zero: Babylon/China/Maya/India; the
Pythagorean relationship: Babylon/China/India, independent of Greece; π: Babylon/Egypt/China/India;
binomial coefficients: India/Persia/China) — but none of this bears on the notation's specific
semantic claims. More importantly, real independent convergence *does* exist for structure that
matters: Aristotelian and Nyāya logic (Greek and Indian, developed "rather independently of each
other") both converge on subject-predicate combination, negation, and quantification; Aristotle's
categories and Vaiśeṣika's *padārthas* both independently treat substance/entity as foundational
with quality/action as dependent. Mohist logic (China) is a genuine, honest counterexample —
analogical rather than deductive, not converging with the other two. **No independent support found
anywhere for the specific counts** (4 sorts: Aristotle has 10 categories, Vaiśeṣika has 6–7, neither
is 4; 5 primitives or 13 operations: no correspondence found at all). Feeds directly into Phase 3.

Original scope note (solo/literature-based, no compute or collaborators needed) confirmed accurate
in practice.

**Question:** Which specific mathematical/logical structures were independently (re)discovered by
non-contacting human civilizations (Babylon, Egypt, India, China, Mesoamerica, etc.), and how much
of Σ_UL's specific structure (4 sorts, 5 primitives, 13 operations) actually reduces to that
convergent core, versus being invention layered on top of it?

**Predicted-in-advance falsification condition:** If Σ_UL's specific operation/sort/primitive
counts show no correspondence at all to any independently-convergent mathematical structure
(Boolean/negation structure, basic compositional/group-theoretic symmetry, etc.), that's evidence
the counts are arbitrary design choices, not discoveries — record that plainly if found.

**Deliverable:** An inventory of confirmed independent mathematical convergences with sourcing,
plus an honest mapping (or non-mapping) against Σ_UL's specific claims.

---

## Phase 2 — Non-Human Semiosis Literature Deep-Dive

**Status:** Not started — scope revised 2026-08-01 following a self-critique of the original draft
(too narrow a taxon list, no pre-registered per-operation criteria, no replication filter, no
wild-vs-captive evidence weighting — a negative result from the original scope would not have been
trustworthy). Revised scope below.

**Question (unchanged):** Beyond the two confirmed data points already found (bee waggle-dance
geometric encoding of direction/distance; Bengalese finch bounded-hierarchical song syntax), is
there credible, peer-reviewed evidence of a non-human (or non-culturally-transmitted) system
exhibiting **full relational-algebra properties** — negation-like, quantification-like, or
embedding/recursive-reference-like structure, not just compositional sequencing or parametric
encoding?

### Pre-registered per-operation criteria (set before evidence-gathering, per Phase 0 discipline)

Defined now, before searching, so evidence can't be fit to a conclusion after the fact:

- **Structural cluster** (`predicate`/`compose`/`invert`/`embed`/`bind`): **PARTIAL** if the system
  combines ≥2 independently-meaningful signal units into a compound whose meaning is not simply the
  sum/concatenation of parts (rule-governed modification, not mere sequencing). **FULL** if the
  compound can be nested (a unit embedded within another) or if reference is trackable/bindable
  across an utterance (co-reference).
- **Logical cluster** (`negate`/`conjoin`/`disjoin`/`quantify`): **PARTIAL** if any signal is
  reliably associated with absence/cessation/contradiction of another signal's content, or with a
  many/one or all/some distinction. **FULL** if that signal composes productively with arbitrary
  other content (not a fixed idiom restricted to one context).
- **Modificatory cluster** (`modify_entity`/`modify_relation`/`modify_assertion`): **PARTIAL** if a
  signal reliably and combinably grades another signal's intensity/certainty/scope. **FULL** if the
  modification generalizes across different base signals rather than being tied to one.
- Every finding is tagged one of: **FULL** / **PARTIAL** / **ABSENT-TESTED** (searched for
  specifically, not found) / **ABSENT-UNTESTED** (no known study addresses this). Collapsing the
  last two into one "no evidence" bucket is not allowed — they are different results with different
  implications.

### Expanded target literatures (supersedes the original four-taxon list)

| Case | Why it matters | Evidence class |
|---|---|---|
| Vervet monkey referential alarm calls (Seyfarth & Cheney) | Foundational referential-signaling case | Wild/natural |
| Campbell's / putty-nosed monkey call combination (Zuberbühler et al.) | Strongest candidate for compositional *modification* of meaning in a wild non-human system — biggest omission from the original scope | Wild/natural |
| Prairie dog alarm calls (Slobodchikoff) | Claimed compositional descriptive content (predator size/color/shape/speed) | Wild/natural, **contested — replication check required** |
| Bengalese finch song syntax | Already confirmed (bounded hierarchical, no negation/quantification) | Wild/lab |
| Honeybee waggle dance | Already confirmed (geometric parametric encoding, no relational algebra) | Wild/natural |
| Parrot cognition (Pepperberg's Alex) | Explicit tests of categorical judgment, same/different, negation-like response | Trained/captive |
| Wild great-ape gestural communication (Hobaiter, Byrne) | No trainer-cueing confound, unlike lexigram studies | Wild/natural |
| Great-ape lexigram studies (Kanzi et al.) | Historically prominent but methodologically contested (cueing, over-interpretation — see the Nim Chimpsky critique) | Trained/captive, **contested — replication check required** |
| Cetacean artificial-language comprehension (Herman) | Directly tests syntax/embedding comprehension | Trained/captive |
| Corvid cognition/planning studies | Tests displaced reference / future-planning, relevant to embedding | Wild/lab |
| Cephalopod chromatophore signaling (Hanlon et al.) | Non-vocal, non-mammalian signaling system | Wild/natural |
| Elephant individually-directed, name-like calls | Recent, referential | Wild/natural |
| Bacterial gene-regulatory logic (quorum-sensing promoter architecture) | Tests whether AND/OR/NOT-like logic can appear in a non-cognitive substrate — **outcome not assumed in advance**, unlike the original draft which pre-judged this as a negative control | Non-cognitive |
| Plant signaling, incl. contested mycorrhizal-network ("wood wide web") claims | Lower-bound/frontier case; evidence quality flagged explicitly rather than dismissed or accepted | Non-cognitive, **frontier/contested** |
| Spontaneous grammar emergence without transmission (Nicaraguan Sign Language, home-sign, village sign languages) | Human, but tests whether compositional generativity is a readily-available capacity vs. one requiring inherited language — bears directly on how rare the threshold is | Human, no prior linguistic model |

### Methodology

1. First, search for an existing survey/meta-review of compositionality and referentiality in
   animal communication, and check this table's coverage against it before treating any negative
   result as exhaustive.
2. For each case, search for both (a) the primary evidence and (b) any published replication,
   critique, or failure-to-replicate. Record both — do not cite only the original claim.
3. Score each case against the per-operation criteria above (FULL/PARTIAL/ABSENT-TESTED/
   ABSENT-UNTESTED), tagged with evidence class (wild/natural, trained/captive, non-cognitive,
   contested).
4. Report contested cases as contested — both the original claim and the critique — not averaged
   away into a single confident verdict.

### Falsification condition (unchanged in substance, sharpened in evidentiary standard)

If, after this expanded and methodologically filtered search, no case anywhere reaches **FULL** on
the logical cluster or the structural cluster's embedding/binding criteria, that remains a
meaningful negative result — but only because this scope is actually thorough enough to support it.
A negative result from the original four-taxon list would not have earned that conclusion.

**Deliverable:** An evidence table — 13 operations (grouped into the three clusters above) ×
case — with FULL/PARTIAL/ABSENT-TESTED/ABSENT-UNTESTED, evidence class, and citations including
replication/critique status. Not a collapsed found/not-found binary.

**Status:** ✅ Complete (first pass) — see `phase2-non-human-semiosis-audit.md`.

**Summary of findings:** Genuinely mixed, not a clean confirm or falsify. Two non-human cases reach
PARTIAL-to-FULL on the clusters that matter most — Pepperberg's parrot Alex spontaneously
generalized a "none"/absence response to novel situations without training (logical cluster); Herman's
dolphins showed untrained, productive interpretation of syntactic argument structure in an artificial
language (structural cluster) — but both are contested in the literature (Kako 1999 vs. Herman &
Uyeyama 1999; N=1–2 captive subjects) and not replicated at the scale of the uncontested findings
(vervet calls, bee dance, birdsong, 2024 elephant name-calls). One important non-cognitive finding:
natural bacterial quorum-sensing promoter logic shows real AND/OR-like Boolean structure with no
mind involved at all — which sharpens rather than undermines the Phase 0 Peircean/biosemiotic
distinction (logic-structure without an interpretant is not the same as meaning). The
plant/mycorrhizal-network ("wood wide web") claim was not merely unconfirmed but actively
discredited by recent scholarship — reported as a real negative, not left open. The single
strongest finding in this phase is human, not non-human: Nicaraguan Sign Language shows full
compositional generativity emerging reliably across independent child cohorts with zero transmitted
model — strong evidence the "threshold" is readily reached in humans given minimal social
conditions, though it says nothing about non-human life specifically.

---

## Phase 3 — Formal Necessity Rebuild

**Status:** Not started. Solo/technical.

**Goal:** Replace the circular "Unique Grounding Theorem" (`foundations/formal-foundations.md`
§4.3–4.5) with an honest argument built on Phase 1's findings plus the already-identified
Curry–Howard–Lambek convergence (type theory ≅ lambda calculus ≅ Cartesian closed categories —
three independently-motivated 20th-century formalisms that provably coincide). Either (a) show a
specific primitive/sort count is forced *given an explicitly stated* set of representational
constraints (e.g. "flat, fixed-arity, drawable categories, no higher-order nesting") — and state
those constraints honestly instead of hiding them — or (b) conclude the count is a free parameter
and the real invariant is the underlying convergent algebraic structure, not any specific N.

**Deliverable:** A replacement necessity document, explicitly superseding
`foundations/formal-foundations.md` Part IV once complete.

**Status:** ✅ Complete (first pass) — see `phase3-formal-necessity-rebuild.md`; the pointer in
`foundations/formal-foundations.md` has been updated to reference it.

**Summary of findings (revised after a self-critique caught two real gaps in the first pass — see
`phase3-formal-necessity-rebuild.md` Part E for the full account):** Mostly resolves to "(b) the
count is a free parameter," with a real positive result for part of the argument, not just a
deflation. Correction 1: the Curry–Howard–Lambek convergence is intuitionistic, not classical — it
cleanly forces the predicate/quantifier hierarchy, but the notation's classical, bivalent `negate`
needs its own (separately-grounded, not mathematically-derived) justification, which Phase 1's
Aristotle/Nyāya convergence and Phase 2's parrot-negation finding actually provide. Correction 2:
the claim that 3-, 4-, and 6-category flattenings of the underlying hierarchy are "equally valid"
was asserted, not checked, in the first pass — the 3-category case was then actually worked through
operation-by-operation and confirmed equivalent (at the cost of moving disambiguating information
into explicit tags); the 6-category case was not run through the same check and is now explicitly
marked unverified rather than left implied. `embed`/`bind`/`compose`/`invert`/`modify_assertion`
remain honestly unresolved: no independent evidence found either way. Not machine-checked (would
need a proof assistant — flagged as a resourced follow-up, not silently dropped).

---

## Phase 4 — Cross-Substrate Test Design (draft now, run later)

**Status:** ✅ Design complete — see `phase4-cross-substrate-test-design.md`. Execution needs
compute budget not currently available (deferred, not blocked).

**Summary of findings:** Applying the process-note discipline (verify before drafting, not after)
surfaced three things the original one-paragraph sketch would have missed entirely. (1) Relevant
prior work already exists and complicates the original binary hypothesis: a 2026 published finding
shows convergent "platonic" representational geometry even in non-communicative physics-simulation
models, in tension with the clean "convergence marks interpretation" story. (2) The two metrics
named in the original sketch (CKA, SVCCA) have documented reliability problems — including random,
untrained networks showing spuriously high CKA from shared input structure alone — that would have
produced an uninterpretable or misleading result if run naively. (3) A real conceptual gap: this
kind of experiment measures representational *convergence*, not semiotic *interpretation* in the
Peircean sense the investigation's working hypothesis actually depends on — flagged explicitly
rather than allowing a clean numeric result to be over-read as resolving the deeper question. The
hypothesis was revised from a binary to a graded (dose-response) prediction, the metric plan now
requires debiased/covariate-adjusted estimators plus a mandatory untrained-network baseline, and
the design states plainly what it can and cannot settle.

**2026-08-01 — Phase 4 executed (design only).**

---

## Phase 5 — Causal Efficacy Protocol Rehabilitation

**Status:** ✅ Rehabilitation design complete — see `phase5-causal-efficacy-rehabilitation.md`.
Execution (630+ trials) needs compute/API budget — deferred, not blocked.

**Original three-item sketch (below) was itself premature** — written from section headers rather
than the full 977-line protocol. Reading the whole thing plus checking the actual methodology
literature (not assumed from memory) found the existing protocol is more sophisticated than the
sketch implied (it already has 4 matched controls, 5 negative controls, a blinding tool, and a
pre-registration hash generator) — but also surfaced a more serious, previously undetected problem:
the analysis plan's power justification treats repeated temperature-sampled trials from the same
model+prompt as independent observations, which the actual pseudo-replication literature says is
invalid — the real effective sample size is ~30 cells per condition (model × task), not the ~90
"observations" claimed. See the rehabilitation document for the full corrected plan: cell-level
aggregation before condition comparison, LLM-judge-bias mitigations, a genuinely independent
arbitrary-notation control (CT-3 alone doesn't satisfy this — it's a relabeling of UL's own
symbols, not a separate notation), and a stated prior (drawn from the closely-analogous
chain-of-thought literature) that partial/null ablation results are the expected outcome, not a
surprise to explain away later.

**Original three-item sketch, confirmed still valid as far as it went:**
1. Genuine pre-registration (timestamped/hashed, filed before any trial data is seen) — confirmed
   still not done; the tooling exists (`preregister.py`) but has never been run.
2. A matched **arbitrary-notation control** — confirmed as a real gap, sharpened: CT-3 (scrambled
   artifact) doesn't satisfy this because it's derived from UL's own symbols; a genuinely separate
   constructed notation (CT-5) is needed.
3. Blinded scoring actually executed — confirmed still not done.

---

## Phase 6 — Synthesis and Honest Verdict

**Status:** ✅ Complete — see `phase6-synthesis-and-verdict.md`.

**Headline verdict:** Not established, not refuted, for the strong mind-independent claim — but
something narrower survives: compositional generativity is real, threshold-gated (not universal,
not absent), and grounded in genuine partial convergent evidence (Aristotle/Nyāya logic;
Curry–Howard–Lambek for the intuitionistic fragment; contested non-human cases; robust human
language-emergence data). The specific primitive count (5, or the wiki's 6) is confirmed
underdetermined by everything checked across math, logic, and biology — not "wrong in favor of a
different number," but a genuinely open notation-design choice, exactly matching the user's own
instinct that the count felt arbitrary. UWS (the notation) and the Investigation (the open
question) are now fully decoupled — UWS needs none of the above to be legitimate. Two genuinely
open empirical questions have ready-to-run designs pending compute budget (Phases 4, 5); one
original pillar (a genuinely independent blind rederivation) was never executed and remains the
clearest actionable gap in the investigation's own coverage, not just in the evidence.

**Meta-finding:** this investigation, despite being designed to counter the original project's
overclaiming, still had to self-correct twice (Phases 2 and 3) — recorded as evidence the
discipline was necessary, not performative, and as a calibration note for how much confidence to
extend to this document itself.

---

## Execution log

**2026-08-01 — Repo restructuring executed.** Following the keep/retire inventory
(`keep-retire-inventory.md`), the Σ_UL proof apparatus was retired from the live repo and
preserved in full at `archive/superseded-2026-08/`:
- Archived outright: `RAMIFICATIONS.md`, `foundations/paradigm.md`, `frontier/strategic-plan.md`,
  `frontier/gap-analysis.md`, `history/`, `applications/`, `proto-analysis-papers/` (duplicate of
  `whitepaper/`).
- Split and rewritten in place (full originals archived): `foundations/formal-foundations.md`
  (kept: language-as-Σ-homomorphism definition, the notation's operation spec, modal/performative/
  pragmatic notation extensions; retired: Isomorphism Theorem, Unique Grounding Theorem,
  Strengthened Universality); `foundations/universal-language-derivation.md` (kept: primitive
  definitions, symbology/syntax/grammar design, Erlangen-hierarchy synonymy heuristic, lexicon,
  Appendix A prior-art comparison; retired: Foundational Axiom, Part VI proof).
- Rewritten to reflect the new two-part structure (UWS + Investigation), not proof claims:
  `README.md`, `AGENTS.md`, `FOR-AI.md`, `llms.txt`, `index.json`.
- Retagged as historical process record (not current status), via a new top-level notice:
  `docs/planning/audits/improvements/README.md`.
- `ul-core/` (the actual writing-system content), `ul-forge/`, `docs/ul-forge-v1/`,
  `docs/distribution/`, `docs/learning/`, `frontier/methodology.md`, `frontier/causal-efficacy-protocol.md`,
  `frontier/expedition-one/`, `frontier/expedition-two/`, and `experiments/` were left in place per
  the inventory (KEEP-UWS / KEEP-INVESTIGATION, no rewrite needed yet).

Nothing was hard-deleted — everything retired is preserved under `archive/superseded-2026-08/`.
No commits were made; all changes are staged in the working tree for review.

**2026-08-01 — Phase 1 executed.** See `phase1-convergent-mathematics-audit.md` for full findings,
citations, and the honest mapping table (what's supported, what isn't, what's unresolved).

**2026-08-01 — Phase 2 executed** (revised, expanded scope — see the Phase 2 section above for why
the original 4-taxon draft was insufficient). See `phase2-non-human-semiosis-audit.md` for the full
15-case evidence table with FULL/PARTIAL/ABSENT-TESTED/ABSENT-UNTESTED scoring, evidence class, and
replication status.

**2026-08-01 — Phase 3 executed.** See `phase3-formal-necessity-rebuild.md`. The retired-material
pointer in `foundations/formal-foundations.md` now references the replacement directly.

## Immediate next action

Phase 4 (Cross-Substrate Test Design) is next — drafting only, no execution (no compute budget
currently available). Design the representational-alignment protocol with the non-communicative-
model control identified earlier as the correction to the original (flawed) design, and incorporate
Phase 3's sharpened hypothesis: test for the convergent operational core (predicate/negate/quantify/
substance-primacy) specifically, not for the notation's specific sort count, which Phase 3 found is
not a meaningful thing to test for in the first place.
