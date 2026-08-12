# 009 — Delete rather than archive

**Type:** decision
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows 008

## The decision

Superseded material was kept in an in-repo archive on the theory that preserving it was more honest
than deleting it. **It was read as current for months** — cited, restated, and propagated by humans
and agents alike.

**Deleted. Git history is the archive.**

## Alternatives considered

- *Keep and banner* — rejected. A document that reads as authoritative because it was written to does
  not stop doing so because a folder name says otherwise.
- *Keep and disclaim inline* — rejected, and this was the sharper lesson: **a negated number is still
  a restated number.** Disclaiming propagates.

## What it cost and what it caught

224 → 49 markdown files. Removed: the D2 experiment infrastructure (never run, scoring withdrawn as
evidence), the six-phase investigation, Σ_UL-era derivations, `ul-forge/docs/` and the UQPL spec.

Two finds worth recording, both of which had been invisible:

- The implementation's own entry document (`ul-forge/docs/v1-reference/README.md`, since removed)
  asserted the retired claim as established fact.
- `CONTRIBUTING.md`'s style guide **mandated the retired number**: *"Operation counts must say 13."*

**Generalized (F-018):** retiring a claim does not retire what instantiates it. A retirement that
leaves its artifacts, counts and specs standing has not happened. And a `DESIGN-CHOICE` whose
alternatives are bare values or version history is not a design choice — it is residue wearing a
tier.
