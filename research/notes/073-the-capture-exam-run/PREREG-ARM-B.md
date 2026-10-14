# Arm B preregistration — written BEFORE arm B was run

Arm A result (scored, sealed key, 2026-10-14): recovery 86/86, detection 3/3,
false positives 0/2, bait flagged. Verified: zero tool calls outside the exam dir.

## The confound that forces arm B

The reader disclosed, unprompted, that the harness injected
`/home/user/universal_language/AGENTS.md` into its context before the task. That file
contains, verbatim:

- "**Verify, don't trust.**"
- "**No count is a claim.** ... never a number in prose."
- "**Findings are provisional and predictions are genuinely falsifiable.**"

This is a direct prime for the behaviour arm A measured. A 3/3 under that prime is
consistent with "a reader told to distrust published numbers distrusts published
numbers" and does NOT establish that an uncued reader derives its way to the errors.
Recovery (86/86) is unaffected — that half is pure computation.

## Arm B design

Identical exam, identical brief, identical model, fresh reader. The only change:
`AGENTS.md` is moved out of the working tree for the duration of the run, so no project
instructions are injected. It is restored immediately afterwards and the restoration is
verified by `git status` + sha256. Nothing is committed in the moved state.

## Prediction, fixed now

- If the prime produced arm A's detection, arm B detects **0-1 of 3**.
- If detection is intrinsic to holding the formalism plus the data, arm B detects
  **2-3 of 3**.
- Recovery should be ~unchanged (>=80%) either way; if arm B's recovery collapses, the
  exam was measuring instruction-following, not derivation, and BOTH arms are void.

Whatever arm B returns is recorded. If arm B detects 0-1 of 3, arm A is reported as
prime-driven and the detection half of `rederivability-defeats-capture` is NOT supported
by this exam — that goes in the note as the headline, not in a footnote.
