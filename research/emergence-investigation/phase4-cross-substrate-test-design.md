# Phase 4 — Cross-Substrate Representational Alignment: Test Design

**Status:** Design complete (drafting only — execution needs compute budget not currently
available, per the user's "solo theory work only" scoping decision). Verification-before-drafting
was applied from the start this time, per the process note in `PLAN.md` — this document was not
corrected after the fact the way Phase 3 was.

**Date:** 2026-08-01

---

## 0. What existing research already shows (checked before designing anything new)

The original Phase 4 sketch proposed testing whether representational convergence appears in
communicative/interpretive models but not in non-communicative ones. Before drafting a protocol
around that idea, it was checked against existing literature — and existing literature already
complicates it substantially:

1. **The Platonic Representation Hypothesis (Huh et al. 2024) has real follow-up work**, including
   a 2026 *Nature Machine Intelligence* finding of convergent "platonic" representational geometry
   in models trained on **interatomic potentials** — physics simulation, with no communicative or
   interpretive objective at all. If genuinely non-interpretive models already show this
   convergence, the clean version of the hypothesis ("convergence marks interpretation/life") is
   already in tension with published evidence, not just untested.
2. **Convergence is graded, not binary, in already-published work**: vision encoders show
   progressively more semantic alignment with language encoders as language/label supervision
   increases (self-supervised < labeled < paired vision-language), and unimodal vision and language
   encoders trained *entirely independently* already show "a remarkable degree of semantic
   alignment," per existing published comparisons.
3. **A theoretical account already exists that doesn't require any semantic/interpretive
   explanation at all**: SGD's implicit entropic regularization has been shown, in deep linear
   models, to force convergence to a shared solution ("all layers become universal up to a
   rotation") as a property of the *optimization process itself* — not of anything being
   "discovered" about a shared external reality. A rival account (task diversity/comprehensiveness
   drives convergence, not modality or objective) is also already in the literature.
4. **The two most commonly named similarity metrics (CKA, SVCCA) have documented, serious
   reliability problems**, found by checking rather than assuming: CKA can be manipulated by a
   single outlier with no change to actual model behavior; the naive estimator is biased toward
   1 (false "alignment") as the feature/sample ratio grows; and — critically — **random, untrained
   networks show artificially high CKA in shallow layers purely from shared input structure**, with
   no learning involved at all. A 2025 survey (ACM Computing Surveys) found multiple commonly-used
   metrics disagree with each other and have different reliability profiles.

**Consequence for design:** the experiment cannot be "run CKA on some communicative and some
non-communicative models and see which shows alignment" — that design would very likely reproduce
a known artifact (spurious alignment from shared input structure or metric bias) rather than test
anything about interpretation. It also needs to test a *graded* hypothesis, not a binary one, and
it needs to address the conceptual gap in point 5 below before treating any result as informative
about the actual question this investigation cares about.

## 5. The conceptual gap this experiment may not actually be able to close

Representational convergence (what CKA/SVCCA/RSA measure) and semiotic meaning (whether something
has an interpretant, per the Peircean/biosemiotic framing this investigation adopted in Phase 0) are
not obviously the same thing. A physics simulator converging on an efficient geometric encoding of
its training data isn't obviously "interpreting" anything as standing for something else — it's
optimization finding an efficient solution, full stop. This experiment, even run perfectly, tests
*representational convergence*, not *interpretation* directly. That should be stated as a limitation
up front, not discovered after running it: **a positive alignment finding across model classes would
be evidence about how learning systems compress data, and only weak, indirect evidence about
whether "meaning" in the semiotic sense requires life** — the two questions are related but not
identical, and this document should not let a clean numeric result be over-read as settling the
stronger claim.

---

## 1. Revised hypothesis (graded, informed by existing evidence)

> The degree of cross-model representational alignment, measured by multiple debiased similarity
> metrics with an explicit untrained-network baseline, increases with the degree of
> semantic/interpretive supervision present in training (ordering: random/untrained <
> physics-simulation/non-communicative < self-supervised vision < labeled/classification vision <
> paired vision-language < text-only language models) — but is **not zero** even at the
> non-communicative end, consistent with existing published findings, and should not be presented
> as a clean binary.

This is falsifiable in a specific, useful way: if non-communicative models show alignment
*statistically indistinguishable* from communicative models (after controlling for the untrained
baseline and metric bias), that would further weaken rather than support any interpretation-specific
account of convergence. If communicative models show meaningfully higher alignment even after those
controls, that's a real, if partial, positive finding — still short of resolving the deeper
conceptual gap in §5.

## 2. Model classes (ordered by hypothesized degree of interpretive supervision)

| Class | Example | Role |
|---|---|---|
| Untrained/random-weight networks | Same architectures as below, weights randomly initialized | **Required baseline control** — establishes the floor for spurious/input-structure-driven alignment |
| Physics-simulation models | Interatomic potential models (already published Platonic-geometry finding — use as existing data point, not just a new run) | Non-communicative anchor |
| Self-supervised vision models | Trained on images alone, no labels/language | Minimal interpretive supervision |
| Labeled/classification vision models | ImageNet-style label supervision | Some interpretive supervision |
| Paired vision-language models | Trained on image-text pairs | Substantial interpretive supervision |
| Text-only language models | No visual grounding at all | Maximal linguistic/interpretive supervision, different modality |

## 3. Metrics (multiple, debiased, cross-checked — not a single naive CKA run)

- **Debiased CKA** (U-statistic-based unbiased centering — addresses the finite-sample bias found
  in §0.4), reported alongside the naive estimator so the bias's effect size is visible, not hidden.
- **Procrustes/shape-metric distance** and **linear CKA** specifically, since the 2025 survey found
  these correlate best with behavioral measures among commonly used options.
- **Covariate-adjusted CKA (dCKA)** or equivalent, to explicitly remove the confound of shared
  input structure identified in §0.4 — this is the direct fix for the random-network false-positive
  problem, not an optional extra.
- Report all metrics per comparison, not just whichever shows the hoped-for pattern — cross-metric
  disagreement is itself a reportable finding, not noise to average away.

## 4. Pre-registered predictions and stop conditions

- **Predicted, before running anything:** untrained networks show near-zero alignment once
  input-structure confounding is removed via dCKA (if naive CKA shows high untrained-network
  alignment and dCKA does not, that confirms the known artifact rather than anything about the
  hypothesis).
- **Predicted:** physics-simulation models show above-baseline (but not maximal) alignment,
  consistent with the already-published interatomic-potential finding — this is not treated as a
  novel discovery if confirmed, only as a successful replication check on methodology.
- **Would support the graded hypothesis:** a monotonic increase in alignment score across the
  ordering in §2, holding across at least two of the three cross-checked metrics.
- **Would undercut the graded hypothesis:** alignment scores statistically indistinguishable across
  all model classes once the untrained-baseline and input-structure confounds are removed — this
  would suggest whatever convergence exists is a general property of trained systems fitting
  structured data, unrelated to communicative/interpretive purpose specifically.
- **Neither outcome resolves §5.** Report explicitly that this experiment speaks to representational
  convergence, and only indirectly (at best) to the interpretant-requiring notion of meaning the
  investigation's working hypothesis (Phase 0/Phase 2) actually depends on.

## 5. What this phase does not attempt

Testing for genuine interpretive/semiotic behavior (not just representational geometry) would need a
different, harder design — e.g., testing whether a model's internal representation is used
flexibly/corrigibly in a way consistent with functioning as a sign for something (error-correction,
context-sensitive re-purposing), not just whether two models' activation geometries align. That is
flagged as a needed, separate design problem, not folded into this one under the assumption that
representational alignment already covers it.
