# 018 — Execution plan

**Type:** decision
**Opened:** 2026-08-01
**Status:** open — updated as phases close
**Thread:** follows `017` (pathways)
**Decision:** sequence all fifteen items from `017` into six phases, ordered by dependency and by
what protects later work.

---

## Sequencing principles

1. **Enforcement before production.** Everything downstream produces claims. The tiers are still
   conventions without enforcement — this repo's named historical failure mode — so the checker goes
   first, not last.
2. **Discharge debt before adding to it.** Two claims are prose-asserted. `014` showed that deriving
   a table I thought I already had produced four corrections; building on unverified foundations
   compounds that.
3. **Read before deriving.** Where a formalization already exists, reading it may change how the
   work is done. Cheap insurance.
4. **Decide on paper before coding.** `LOBIAN-OBSTACLE` forces an architecture choice, and making it
   in code is how it gets made implicitly.
5. **Declare the reachable tier per item in advance.** Knowing what a result *can* reach prevents
   overclaiming after the fact — a documented failure here.

---

## Phase 0 — Enforcement

**STATUS: CLOSED 2026-08-01.**

| Item | Output tier | Notes |
|---|---|---|
| **F4** `claims.yaml` CI checker | **VERIFIED** | `tools/check-claims.rb`, `tools/check.rb`, `.github/workflows/repo-checks.yml` |

Delivered: `check-claims.rb` enforces all five tier contracts, resolves evidence paths and
inter-claim references, and encodes **F-018 as a check** — a `DESIGN-CHOICE` whose
`alternatives_considered` holds only bare values or version history is rejected, because that
records no decision. `check.rb` runs all three validators; CI runs it with `--strict`.

**Verified by deliberate injection** — broken evidence path, `CONJECTURED` without a falsifier,
hollow alternatives, `RETIRED` without a revival condition. All four caught, exit 1 each, clean
after restore. An unverified checker is worse than none.

**Found on first run:** two claims referenced `UQPL-REDUCTION`, which does not exist — a dangling
reference that had survived the entire prune. Removed.

Consolidate the three ad-hoc validators (tier discipline, `check-links.rb`, `check-retired-content.rb`)
into one CI-runnable suite with `--strict`. **Completion:** CI fails on a tier violation, a broken
reference, or a retired count.

**Why first:** every subsequent phase adds claims. Without enforcement the registry drifts, which is
exactly what happened to Σ_UL.

---

## Phase 1 — Clear standing debt

**STATUS: CLOSED 2026-08-01.** All three items done — notes `019` (D2), `020` (A3), `021` (D1).
Five of nine load-bearing negatives were over-broad as recorded.

| Item | Output tier | Cost |
|---|---|---|
| ~~**D2** Read *Drawing with Strangers* + SEP on Zadrozny~~ **DONE** — note `019` | ARGUED (narrowed) | 2 fetches |
| ~~**A3** Intuitionistic-logic coherence check~~ **DONE** — note `020` | DESIGN-CHOICE + a lead | 2 searches |
| ~~**D1** R7 sweep~~ **DONE** — note `021` | varies | 3 unchecked negatives, all three changed |

**D2 is a standing S1 violation:** two priority-0 claims rest on summaries. One is our best empirical
result, one reopens the necessity route.

**D1 has the highest demonstrated yield in the project** — it has already overturned Zadrozny and
TopSim. Mechanical, and the only reason it is not first is that it is the largest.

**Completion:** no priority-0 claim rests on a tier-5 source; every cited negative has one
adversarial search logged.

---

## Phase 2 — Discharge the prose-asserted claims

| Item | Output tier | Notes |
|---|---|---|
| **C1** κ-stratification as a partition lattice, collapses as theorems | ARGUED | VERIFIED needs a proof assistant |
| **C2** Jordan separation ≅ Spencer-Brown's mark | ARGUED | Load-bearing for the anchor |

**Honest tier ceiling:** hand derivations reach ARGUED-properly-warranted, not VERIFIED. Claiming
otherwise would repeat the `014` lesson in reverse.

**Completion:** neither claim cites prose as its own evidence; both have worked derivations with
stated definitions, as `014` does.

---

## Phase 3 — New ground: the expressiveness bound

| Item | Output tier | Notes |
|---|---|---|
| **A4** Read *A Finite, Feasible, Quantifier-free Foundation for Constructive Geometry* | — | **First** — may change how A1 is done |
| **A1** Constructibility ladder as UWS expressiveness bound | ARGUED, possibly a genuine theorem | The highest-value item in the project |
| **A2** Huzita–Hatori as alternative basis; the base-6 coincidence check | ARGUED or explicitly dismissed | Check, do not retrofit |

**A1 is the payoff.** UWS has never had an expressiveness characterization. Constructibility supplies
a theorem-backed ladder: straightedge-compass → quadratic extensions; neusis → cube roots; origami →
strictly stronger.

**A2 discipline:** the six-axiom / base-six correspondence is recorded as a *coincidence to check*.
If it does not hold, say so in the note. Retrofitting it is the exact failure this repo documents.

**Completion:** a stated bound on what the notation can construct, with the argument written out.

---

## Phase 4 — Formal semantics and forced decisions

| Item | Output tier | Notes |
|---|---|---|
| **B** Mereotopology / RCC as spatial semantics for the IR | DESIGN-CHOICE | Existing decidability results |
| **E2** Löb architecture decision | DESIGN-CHOICE | **On paper, before code** |
| **E1** Entrenchment ordering specification | DESIGN-CHOICE | AGM then guarantees a rational repair operator |

**E1 is the Cure's whole remaining blocker.** AGM converted it from an ill-posed math problem into a
design task; this is that task.

**Completion:** three registered DESIGN-CHOICE entries with rationale and alternatives.

---

## Phase 5 — Code

| Item | Output tier | Depends on |
|---|---|---|
| **F2** Validator as abstract interpretation with a Galois connection | VERIFIED | Rice framing (done) |
| **F1** `semantically_equal` via e-graphs, scoped to the acyclic core | VERIFIED | Phase 4 B |
| **F3** `negate` reimplemented with a σ field | VERIFIED | F1, to test the involution |

**These are the only items that can reach VERIFIED**, because they are code with tests. That is worth
stating plainly: **of fifteen items, four can be machine-checked and eleven cannot.** The project's
`VERIFIED` count will stay low, and that is the accurate picture rather than a gap in bookkeeping.

---

## Dependency graph

```
F4 ──────────────────────────────────────────────► (protects everything after)

D2 ─┐
A3 ─┼─► (re-tier claims)
D1 ─┘

C1 ─┬─► foundations solid
C2 ─┘        │
             ▼
A4 ─► A1 ─► A2          (expressiveness bound)

B ──┬─► F1 ─► F3
E2 ─┤
E1 ─┘
     └─► F2
```

---

## Risks

- **Phase 1 may overturn things Phases 2–3 build on.** That is the point of doing it first, and it
  has happened twice already.
- **A1 may not close.** Constructibility bounds a *drawing* system; the step to a *semantic* bound
  needs an argument that expressible-meanings track constructible-figures. **That bridge is not
  free and is not yet made.** Recorded now so it is not discovered as a surprise in Phase 3.
- **Sequencing drift.** `017` flagged that this project produces reading more readily than
  derivation. Phases 2 and 3 are the derivation-heavy ones. **If they keep getting deferred in
  favour of Phase 1 crawls, that is the drift, and this line exists to catch it.**
