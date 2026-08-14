# Module X — Inoculation: how research on this problem gets killed, and how not to be the casualty

**Status:** packaged — the material predates this module (`RESEARCH-PROTOCOL.md`, `FAILURES.md`,
notes `043`–`049`); this is its teaching arrangement.
**Tier note:** everything below cites the registered record; where a lesson rests on an ARGUED or
CONJECTURED claim, the label is inline (`TIERS-TRAVEL-WITH-CONTENT`).
**Why this module is first:** in any serious literature scrub, the reader meets the impossibility
results *before* mastering the material they misjudge. The inoculation has to come first or the
rest of the curriculum never gets read. That is not a hypothetical — it is this project's own
documented history, across human and machine collaborators alike.

---

## Lesson X.1 — The pattern, from the record

A documented cycle, running for over a decade on this problem: **research is found that appears to
settle the question negatively; the finding is believed; the line of work is closed** — and on
examination, the result did not say what it was cited as saying. Seven instances in a single
session are tabulated in `RESEARCH-PROTOCOL.md` §1; five load-bearing negatives were overturned by
one adversarial search each.

**The mechanism is not deceit and not stupidity. It is scope-dropping in transmission:** a theorem
about microscopic compositionality read as a verdict on emergence; a theorem requiring convexity
read as a verdict on repair; a program-property theorem applied to a data structure. The summary
keeps the conclusion and sheds the conditions, and **the conclusion without its conditions is a
different, stronger, false claim.**

## Lesson X.2 — The five worked examples (with the answer key the record already wrote)

Work each case *before* reading its resolution: given the cited result, state its scope conditions,
then check the target is inside them.

| Case | Cited as | Work it, then compare to the recorded resolution |
|---|---|---|
| Zadrozny 1994 | "compositionality is vacuous, UL impossible" | `claims.yaml#ZADROZNY-SCOPE`; the three-part constraint, and what the encoding fails to preserve |
| Rice's theorem | "verification of GIR impossible" | `FAILURES.md` F-023 — extensional properties *of programs* |
| Hilbert projection / convexity | "repair operator impossible" | preorder-based revision (AGM); the obstruction was one formalization's artifact |
| "Four attempts, three centuries, none universal" | "adoption impossible" | note `049` — four *different, diagnosable* caps; a uniform negative manufactured from heterogeneous failures |
| "Far transfer is a chimera" | "language cannot align a mind" | note `052` — kills the *skill-transfer* reading; format acquisition is a different, documented effect |

**Self-check:** your scope statement for each case should identify the same dropped condition the
record identifies. If yours differs, one of you is wrong — and the record shows its work, so you
can determine which.

## Lesson X.3 — The trap signatures, as reflexes

`RESEARCH-PROTOCOL.md` §2, T1–T12, learned to the point of firing *mid-reading*. The three that
pay most often, per the record:

- **T2** — citing a result that settles the question → *state its scope conditions. If you cannot,
  you have read a summary, not a result.*
- **T4** — the conclusion closes the investigation → *negatives are terminal and feel like finished
  deliverables; positives open work. That asymmetry is about comfort, not truth.*
- **T10** — accepting a null result → *a theorem states its scope; a null's scope IS its
  operationalization. What was measured?*

**The master signature: output that sounds rigorous and ends the conversation.** That combination
is the alarm, every time.

## Lesson X.4 — The symmetric guard (do not leave this module half-armed)

The inoculation is **not** a license to dismiss negative results — that is the *other* documented
disease (`PRIMER.md` §1: a completeness score driven 32% → 100% by explaining away each failure).
The discipline is symmetric: **R5 — a finding that kills work meets the same evidentiary bar as
one that advances it. Both directions of free verdict are excluded** (`REQ-3`,
`claims.yaml#UNIVERSALITY-IS-A-LEDGER`).

**Self-check:** note `048` records seventeen protocol rules failing silently in three volleys —
committed by the same process that wrote the protocol. If your conclusion from this module is
"the traps are for other people," you have failed the module.

## Exercise (the module's exit test)

Take any impossibility-flavored paper touching universal language, semantics, or notation that is
**not** in the record above. Produce, in order: (1) the result's scope conditions, stated from the
paper itself; (2) whether the UL construct it would kill is inside them; (3) the one adversarial
search you would run before accepting it (R1); (4) its revival condition if accepted (R4).
**Only then** read its conclusion section. Compare what the abstract implied against what the
theorem states — the gap between the two is the thing this module exists to make visible.
