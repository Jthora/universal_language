# Phase 5 — Causal Efficacy Protocol Rehabilitation

**Status:** Rehabilitation design complete (verified-before-drafting, per the process note). Execution
still needs compute/API budget not currently available (deferred, not blocked).
**Date:** 2026-08-01
**Target:** `research/frontier/causal-efficacy-protocol.md` and its supporting infrastructure in `research/experiments/`.

**Correcting a premature summary from earlier in this investigation:** the original Phase 5 sketch
in `PLAN.md` said three fixes were needed (real pre-registration, a matched arbitrary-notation
control, executed blinding). Having now actually read the full protocol (977 lines) rather than
working from its section headers, the existing design is considerably more sophisticated than that
sketch implied — it already has 4 matched controls, 5 negative controls including one specifically
designed to rule out "esoteric register alone" (NC-5), a blinding tool, and a pre-registration hash
generator. But reading it fully, plus checking the actual methodological literature for this kind
of experiment (not assumed from memory), surfaced a **more serious, previously undetected problem**
that the three original fixes would have left untouched.

---

## Finding 1 (most important, previously unflagged): the analysis plan has a real pseudo-replication problem

The protocol's §8.1 justifies statistical power by treating raw trials as independent: "Alpha
provides 270 trials... After aggregation by condition: ~90 observations per condition — sufficient
for primary analysis." Checking this against the actual methodology literature on evaluating LLM
outputs: **repeated samples from the same model and prompt, even at nonzero temperature, are not
independent draws from a stable population** — they share identical weights and training history,
and treating them as independent "artificially inflates effect sizes and statistical significance."
This is a documented, named problem in the field (pseudo-replication), not a stylistic quibble.

**Concrete consequence:** the actual independent unit in Experiment Alpha is not "270 trials" or
"~90 observations per condition" — it's the number of distinct (model × task) cells, which is
5 tasks × 6 models = **30 cells per condition**. The repeated samples within a cell (3 per cell at
nonzero temperature) should be aggregated (mean, or explicitly modeled as nested/repeated-measures
within cell) *before* the between-condition significance test, not fed into the ANOVA as if each
were an independent observation. At n=30 per condition, detecting d=0.5 at power=0.80, α=0.01
(Bonferroni for 5 metrics) is **underpowered** relative to what §8.1 claims — this needs a revised
power analysis and, realistically, either more model×task combinations or a relaxed
effect-size/power target stated honestly rather than inheriting the original (invalid) calculation.

**Fix:** restructure the primary analysis as a mixed-effects model with **cell (model × task) as
the unit of replication**, condition as the fixed effect, and within-cell repeated samples entered
as a nested random effect (or pre-aggregated to one value per cell via mean/median before the
between-condition test) — not a flat ANOVA over all raw trials. Redo the power analysis on this
basis before any data collection, and report the corrected required-n honestly even if it means
more models/tasks are needed than originally planned.

## Finding 2: LLM-as-judge biases, unaddressed for the "automated" scoring path

The protocol allows scoring by "human or automated" evaluators but only specifies blinding
(condition labels stripped, order randomized) as the safeguard. Checking the LLM-as-judge
literature: three specific, well-documented biases are relevant and currently unmitigated —
**verbosity bias** (judges reward longer output regardless of added value, more severe than
position bias in some studies), **position bias** (partially addressed by the existing
randomization, but not the *cause* of the bias — order randomization spreads the bias across
conditions rather than eliminating it as a source of noise), and **self-enhancement bias** (a judge
model tends to score outputs from its own model family more favorably).

**Fix, if automated scoring is used at all:** (a) report output length as a covariate for every
metric, since UL-mode's unusual priming text may itself change output length independent of
quality; (b) use a judge model from a *different* family than any model under evaluation, or if
that's infeasible given budget, explicitly test and report judge-family overlap as a limitation
rather than omitting it; (c) keep human blinded scoring as the primary analysis and automated
scoring strictly supplementary, as the protocol's own §1.2 already gestures at but should state as
a hard requirement, not a preference.

## Finding 3: residual surface-formatting confound, likely not fully eliminable, should be reported as a limitation rather than assumed solved

Checking the literature on prompt sensitivity: non-semantic features (formatting, punctuation,
lexical substitution alone) can cause large behavioral changes in LLM outputs independent of
content. The CT/NC controls are carefully matched on token count and content category, but not
explicitly validated for surface formatting/whitespace/symbol-density similarity to the original
artifact. **Fix:** add a formatting-similarity check to the QC audit (already tracked in
`research/experiments/qc-audit-report.md`) as an explicit dimension, and report this as a residual limitation
in any published results — some of this confound may not be fully separable from "UL structure"
using text-based controls alone, and that should be stated up front rather than discovered after
data collection when it's too late to design around.

## Finding 4: the chain-of-thought literature gives a real, checkable prior that should recalibrate expectations

Chain-of-thought prompting is the closest well-studied analog to what this protocol tests (does
specific reasoning/structural content in a prompt cause better output, versus just its format or
presence). The actual literature there is sobering: claimed CoT improvements have failed to
replicate in different setups, effect sizes are often small when rigorously checked, and — most
relevant here — **ablations show invalid reasoning retains 80–90% of the performance benefit of
valid reasoning**, meaning format/presence-of-structured-looking-content typically matters far more
than specific content validity in this adjacent, better-studied domain.

**Consequence, stated in advance per Phase 0 discipline (not after seeing results):** the
pre-registered expectation should be that Experiment Beta's ablation thresholds (e.g., "B3:
ABL-PROSE shows ≥50% degradation") are more likely to be **not met**, or met only partially, than
the original protocol's framing implies — a base rate drawn from a closely analogous, well-studied
domain suggests most of any effect is more likely to come from the general presence of dense,
unusual, formally-structured priming text than from UL's specific symbolic content. This is stated
now, before any data exists, specifically so that a null or weak Beta result cannot later be
explained away as surprising — the honest prior is that it's the expected outcome, and a strong
positive result (content clearly matters, not just format) would be the more remarkable finding.

## Finding 5: the "matched arbitrary-notation control" gap from the original sketch is real and still open

CT-3 (scrambled test artifact) relabels UL's own symbols — it is a transformation *of* the UL
artifact, not an independently-invented notation system. It does not satisfy "equally complex,
equally novel, but not derived from any UL claim" the way a genuinely separate constructed notation
would. **Fix:** add a new control, CT-5, built from a wholly separate constructed symbolic system
with matched token count and visual/formal density but no structural relationship to UL's specific
primitive-to-meaning mapping — e.g., an arbitrary invented notation with its own (different)
internal rules, not a transform of the existing artifact. Without this, Experiment Alpha's H4
("specific symbolic structure matters," tested via CT-3) is weaker evidence than it's presented as,
because CT-3 only tests *which labels* are used within UL's own structure, not whether UL's
structure specifically (versus any comparably complex constructed structure) is what's doing the
work.

## Finding 6: pre-registration and blinding are correctly designed but not yet executed

This one confirms rather than revises the original sketch: `analysis/preregister.py` and
`analysis/blind.py` exist and are well-designed, but neither has actually been run — there is no
`data/preregistration_receipt.json` and no completed blinded scoring round in the repository. This
remains a real, simple, execution-order requirement: run `preregister.py` and publish the hash
*before* any pilot trial, not after.

---

## What Phase 5 recommends, concretely

1. Rebuild the statistical analysis plan (`research/experiments/analysis/analysis.py`) around cell-level
   (model × task) aggregation before condition comparison, with a corrected, honestly-reported
   power analysis.
2. Add explicit LLM-judge-bias mitigations to the scoring protocol if any automated scoring path is
   used, and demote automated scoring to strictly supplementary.
3. Add a formatting-similarity dimension to the QC audit, and report residual formatting confound
   as a stated limitation rather than an assumed non-issue.
4. State the chain-of-thought-literature-informed prior (partial/null Beta results are the expected
   outcome, not a surprise) in the pre-registration document itself, before data collection.
5. Construct CT-5 (an independently-invented notation control) before running Experiment Alpha, and
   revise H4's interpretation to note what CT-3 alone can and cannot establish.
6. Run `preregister.py` and publish the resulting hash before any pilot trial — this is unchanged
   from the original sketch and still simply needs to happen first.

None of this requires abandoning the existing protocol — it requires fixing one serious latent
statistical error, closing one real control gap, adding judge-bias safeguards, and setting honest,
literature-informed expectations before data collection begins. Execution remains deferred pending
compute/API budget, per the user's earlier scoping decision.
