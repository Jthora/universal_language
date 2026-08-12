# 008 — The ditch pattern

**Type:** correction
**Opened:** 2026-08-01
**Status:** closed
**Thread:** corrects the method used in 003–007 · supersedes the instance-level fixes F-012a, F-013a

## 1. What was held

That rigour meant taking negative findings seriously — and that self-monitoring was sufficient to
catch when a finding was over-applied.

## 2. What broke it

Seven instances in one session of a research finding closing a line of work that was still live:
Zadrozny, Chomsky's UG, Evans & Levinson, convexity, ISO pictograms, music notation, and
experiment-vs-proof.

**The damning part is not the seven. It is that F-012a and F-013a corrected this exact pattern
mid-session and five more followed.** Instance-level patching left the generator running — the same
error `FAILURES.md` exists to prevent, reproduced inside the file built to catch it, in the opposite
direction.

## 3. What replaced it

**R1–R6** (`method/negative-results.md`) and **S1–S7** (`method/source-independence.md`), summarized
as trap signatures in `RESEARCH-PROTOCOL.md`. Four are enforceable as required `claims.yaml` fields;
two are checkable only by a human, which is the honest consequence.

## 4. Why it survived as long as it did

**Mechanisms:** scope-dropping on negatives · mistaking a failed formalization for a blocked domain ·
treating a researcher's conclusion as the boundary of the possible · **asymmetric scrutiny** —
interrogating claims that advance the work while accepting at face value claims that kill it ·
terminating search on the first coherent story · never once searching *against* a negative.

**Root incentive:** negatives are cheap to be wrong about, and they are **terminal** — they close the
investigation and read as a finished deliverable, where a positive finding opens more work.

**Why "be more careful" was not available:** every instance happened while the work believed itself
rigorous. **The bias produces skeptical-sounding output.** Self-monitoring is the compromised
faculty, so the remedy had to be mechanical.

**Meta-rule:** two corrections of one shape is the signal. Stop patching instances; fix the
generator.
