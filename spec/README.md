# spec/ — What Exists

**Tier rule: this directory describes only what is built and working. Present tense. No aspiration.**

If a thing is intended but not built, it belongs in `design/`. If it is an open question, it belongs
in `research/`. Mixing these three voices in one document is the failure mode that produced a README
instructing users to install packages that were never published (see `FAILURES.md` F-010).

## Requirements for a document here

1. Every substantive claim has an entry in `/claims.yaml`.
2. Claims are tiered `VERIFIED` or `ARGUED` — a `CONJECTURED` claim does not belong in `spec/`.
3. Anything describing code links to the code, and the code exists.
4. Examples are executable and have been executed.

## Status

**Currently empty.** This is deliberate, not an oversight.

The 2026-08 restructuring retired the previous foundational material because its central claims did
not survive audit. The notation corpus that survived lives in `uws/`; the working implementation
lives in `ul-forge/`. Promoting material into `spec/` requires giving each claim a registry entry
and evidence — that work is the next build phase, not a formality to backfill.

Candidates for promotion, in order:

| Candidate | Blocker |
|---|---|
| GIR schema + validator behavior | None — could be promoted now |
| Parser/deparser round-trip | None — covered by existing tests |
| Notation operation set | Needs `claims.yaml` entries per operation |
| Algebraic laws (involution, De Morgan) | Blocked on `SEMANTIC-EQUALITY` — see `claims.yaml` |
