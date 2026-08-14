# 007 — Is the external reference the mathematics itself?

**Type:** brainstorm
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows 006

## 1. The question

Löb says a system permanently requires an external reference. **Isn't that reference the geometric
definitions and mathematics?**

## 2. Searches run

| Query | Direction | Result |
|---|---|---|
| Escaping the Löbian obstacle, Gentzen | supporting | PA's consistency proved in PRA + induction to ε₀ |
| SI kilogram redefinition, IPK drift | supporting | Artifact lost ~50 µg over a century; 2019 fixed *h* by definition |
| Type soundness as external metatheory | **adversarial (R1)** | There is *always* a trusted base — TCB shrinks, never vanishes |

## 3. Findings

**There is a version that works and one that is a category error.** Mathematics does not *prove* a
system's soundness — Löb is about provability predicates. But Löb bites **self**-reference:

- *"Is my representation self-consistent?"* → self-reference → **bites**
- *"Does my representation match the fixed structure?"* → comparison → **never fires**

**Refinement:** the anchor need not be *stronger*. PRA is weaker than PA in most respects. It must be
more **trustworthy**, which is a far cheaper requirement.

**The metrology case is the same engineering problem, already solved.** Drift measured against a
maintained baseline is undetectable under common-mode drift — the IPK pathology. **The Cure as
specified is the IPK design.**

**The anchor is derived, not chosen:** Point and Enclosure, because Erlangen (`005`) says those
survive coarsest. And it is **the mathematics, not "the universe"** — physical space is curved.

## 5. What changed

- `claims.yaml`: `GEOMETRIC-ANCHOR-ESCAPES-LOB`, `DRIFT-MUST-BE-ABSOLUTE-NOT-RELATIVE`,
  `ANCHOR-IS-DISTINCTION-AND-INCIDENCE`
- Promoted to `framework/external-anchor.md`
- **Left open:** the representation→structure mapping. The grounding problem is relocated into an
  engineering task, not dissolved.
