# 054 — Re-architecting the repo as development center and staging ground

**Type:** decision
**Opened:** 2026-08-12
**Status:** open — executed in skeleton this note; phases close against `053`
**Thread:** executes Phase 0 of `053`. The adversary front (queued in `052`, re-queued in `053`)
takes the next free note number when it runs — living docs now reference it by name, not number,
so it cannot be bumped again.
**Prompted by:** *"How can we re-architect this repo so it operates effectively as our development
center and staging ground for everything you just established?"*

---

## 1. The architectural insight: the repo is both workshop and product — and that is a feature

In the `053` scenario the reader reads *everything* — including the research notes, the failure
log, the corrections. The mess is not something to hide from that reader; **the visible working
record is the credibility and inoculation layers.** So the re-architecture is not "clean up for
shipping." It is:

> **Give the repo two legible paths through one tree — the reader's path (the seed: what deploys)
> and the builder's path (the workshop: where we work) — and make the top level mirror the
> deployment stack so the artifact's structure teaches its own architecture.**

Registered as `REPO-LAYOUT-IS-THE-STACK` (DESIGN-CHOICE).

## 2. The target layout

```
seed/                    ← THE READER'S ENTRY. What deploys, addressed to whoever finds it.
  FIRST-CONTACT.md         layer 3 — read before concluding anything; verify-don't-trust
  INDEX.yaml               layer 1 — machine-readable root map: stack → paths → status → claims
  INTEGRITY.md             layer 7 — how to verify a copy; signing/release policy
curriculum/              ← layer 4 (School). Modules N/F/S/P/Q/X, each self-certifying.
corpus/                  ← layer 4 (School). Graded expressions with machine-checkable ground truth.
ul-forge/                ← layers 5–6 (Engine). Syntax core today; M2 engine + comparator to come.
claims.yaml, tools/,     ← layer 2 (Credibility). The mechanical audit any reader runs first.
  FAILURES.md
PRIMER.md, GLOSSARY.md,  ← comprehension surface, shared by both paths
  README.md, AGENTS.md
research/                ← THE WORKSHOP. notes/ (050 = program, 053 = deployment, this note),
  RESEARCH-PROTOCOL.md      method, surveys, framework. Doubles as inoculation evidence.
STATE-OF-PLAY.md         ← builder's dashboard
design/                  ← intended-not-built; spec/ = exists (unchanged discipline)
```

**Additive, not churn:** nothing existing moves. 172+ internal references stay valid; the reader
path is added *around* the workshop rather than by reorganizing it. A full move-everything
restructure was considered and rejected — link churn and history noise for zero reader value.

## 3. What executes it (F-031: prose does not)

- **`seed/INDEX.yaml` is the single machine-readable root** — stack layers, paths, statuses, gaps,
  phases, the acceptance test, claim IDs. `AGENTS.md` metadata points to it.
- **`tools/check-index.rb` (new, seventh checker):** every path in the index exists; every claim ID
  it cites exists in the registry; every status is from the allowed set; every phase carries its
  acceptance criterion. Wired into `check.rb`, so **CI fails if the index lies.**
- **Curriculum discipline is specified now, enforced when content lands:** every lesson ships with
  a runnable self-check and tier-labeled claim citations; `check-curriculum.rb` is a named Phase 1
  deliverable alongside the first module — a lesson without an executable check does not merge.

## 4. Migration items — flagged, not churned

| Item | Status | Disposition |
|---|---|---|
| `uws/` corpus | pre-rebuild content | re-grounded through **Module S** (placement grammar re-derived from the map); until then it carries its existing caveats |
| `design/uqpl/` drafts | pre-`052` sketches | superseded in definition by `UQPL-IS-FORMAL-TIER-CLOSURE`; retained as design input, wet-clay like the wiki |
| `whitepaper/` | self-labeled historical | unchanged |
| `spec/` | empty, deliberate | fills only as curriculum modules and engine components pass their checks |

## 5. Development flow, restated for the new shape

1. **Research** happens in `research/notes/` under the `050` program — unchanged.
2. **Results graduate**: a derivation that stabilizes becomes a curriculum lesson (with its
   self-check), corpus entries (with ground truth), or engine code (with tests). Graduation is the
   *default next step* of any closed note that changed a claim — the workshop feeds the seed.
3. **The index tracks it**: layer statuses and phase progress live in `seed/INDEX.yaml`, checked
   by CI, so STATE-OF-PLAY's prose and the machine-readable truth cannot drift apart silently.
4. **The acceptance test is the bar** (from `053`): a frontier model, given only the repo, learns
   UWS, re-derives the fixed point, runs a self-drift check — no human in the loop. Phase-1/2 work
   items exist exactly insofar as they move that test.

## 6. What changed

- **Created:** `seed/FIRST-CONTACT.md`, `seed/INDEX.yaml`, `seed/INTEGRITY.md`,
  `curriculum/README.md`, `corpus/README.md`, `tools/check-index.rb` (wired into `check.rb`).
- `claims.yaml`: `REPO-LAYOUT-IS-THE-STACK` (DESIGN-CHOICE).
- `README.md`/`AGENTS.md`: reader-path and builder-path entries; layout updated.
- `PRIMER.md`/`STATE-OF-PLAY.md`/`053`: adversary front referenced by name, not number.
- **Open:** Phase 1 content (Module X packaging first — its material exists; then N); the corpus
  harness; `check-curriculum.rb`; signed release tags (INTEGRITY.md records the policy, honestly
  marked not-started).
