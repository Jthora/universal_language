# 006 — What theorem says this is impossible?

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows 005

## 1. Before searching

**Expected:** that roughly half the CONJECTURED backlog would have mathematical objects.
**Objects mathematical?** Mixed — the audit was to sort exactly that.

## 2. Searches run

| Query | Direction | Result |
|---|---|---|
| Löbian obstacle, self-modifying agents | **adversarial (R1)** | An agent can only trust reasoning *strictly weaker* than its own |
| Rice's theorem, abstract interpretation | **adversarial (R1)** | Non-trivial semantic properties undecidable; Galois connections give provably-best approximation |
| AGM representation theorem | supporting | Katsuno-Mendelzon: faithful preorders are **precisely** the AGM-satisfying operators |
| Noether / Landau order parameter | supporting | Order parameter constructed systematically from the broken symmetry |

## 3. Findings

**Two obstructions were absent from the repo entirely** and both bite the Cure. Neither is fatal:
every working verification technology operates under them — Astrée under Rice, CompCert verified in
an external system.

**One blocker dissolved.** A preorder is not a metric, so the convexity problem that retired the
repair operator was an artifact of the chosen formalism, never a fact about semantics.

**One empirical search converted to a computation.** The order parameter follows from G/H.
*Landau applies; Noether does not* — it needs an action principle semantics lacks, and claiming it
would be borrowed authority.

**Near miss worth recording:** the acyclic non-Turing-complete IR, chosen for confluence reasons, is
what keeps the core outside Rice's reach. Right decision, adjacent reasons, partly luck.

## 5. What changed

- `claims.yaml`: `LOBIAN-OBSTACLE`, `RICE-BOUNDS-SEMANTIC-EQUALITY`, `AGM-PREORDER-REPAIR`,
  `ORDER-PARAMETER-FROM-SYMMETRY-BREAKING`
- `FAILURES.md`: F-016 (never audited for obstruction theorems)
- Promoted to `engineering/obstructions.md`
