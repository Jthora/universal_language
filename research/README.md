# research/

**How this folder works.** Five themed folders hold what we *currently hold*. `notes/` holds work in
progress. Material moves **notes → themed folder** when a position stabilizes, and never the other
way.

`claims.yaml` is the registry — **if something here is not registered there with a tier, it is not a
claim of this project.** These documents are the reasoning behind claims, not the claims themselves.

**Read `RESEARCH-PROTOCOL.md` before searching the literature.**
**Read `STATE-OF-PLAY.md`** for the current synthesis and what to build next.

```
research/
├── framework/     the positions we currently hold
├── cure/          the safety application
├── engineering/   buildable decisions and what constrains them
├── surveys/       literature, prior art, gaps, source critique
├── method/        normative rules for doing research here
└── notes/         iterative — one numbered folder per cycle
```

---

## framework/ — what we currently hold

| Document | Establishes |
|---|---|
| `emergent-universality.md` | UL as an **emergent universality class** — characterized by order parameter and symmetry, not by an inventory. The construct stack. |
| `provable-geometry.md` | Primitives as strata of curvature space; the Erlangen survival ranking; Jordan separation ≅ Layer 0. **What is derivable rather than empirical.** |
| `cross-substrate-grammar.md` | Universal Grammar superseded rather than ceded; why inventories are generating sets, not bases. |
| `external-anchor.md` | The geometry as **trust anchor**: absolute vs. relative drift, and the semantic layer outside Löb's regime. |
| `uws-as-instrument.md` | UWS as an instrument — expressive failures as data, under the rule that failures are never patched. |

## cure/ — the safety application

| Document | Content |
|---|---|
| `theosis.md` | Directed becoming; the consistency/open-endedness tension; verification-as-practice. |
| `coupling-and-attractors.md` | **Coupling asymmetry as the detectable signal**, the apotheotic gap, Pearl's causal hierarchy. |
| `purpose-anchoring.md` | What the program is for, and the scope limits that follow. |

## engineering/ — buildable decisions

| Document | Content |
|---|---|
| `obstructions.md` | Löb, Rice, AGM, Landau. **What theorem says this is impossible** — asked before designing the mechanism. |
| `ir-decision.md` | Acyclic term-graph core, coreference as labels. Load-bearing for two independent results. |
| `prior-art.md` | Conceptual Graphs, ISO 24707, TMS, SHACL, belief revision. |

## surveys/ — the landscape

| Document | Content |
|---|---|
| `research-register.md` | Index of research threads, findings, follow-ups. |
| `unexplored-territory.md`, `who-has-our-problem.md` | Gap analysis — who is and isn't working on this. |
| `source-critique.md` | Where sources were misapplied on transfer, and the narrowed claims that resulted. |

## method/ — normative

| Document | Rules |
|---|---|
| `negative-results.md` | **R1–R6.** Adversarial search, scope, formalization-not-claim, revival conditions, symmetric burden, steelman. |
| `source-independence.md` | **S1–S7.** Evidence ranked by verifiability; research under an untrusted literature. |

## notes/ — iterative

One numbered folder per cycle. **Expectations are written before searching (S2)**; the template
enforces it. See `notes/README.md` for the convention and the index.

---

## Checks

```bash
ruby tools/check-links.rb            # internal references resolve
ruby tools/check-retired-content.rb  # no retired counts in the working tree
```

Both exit non-zero with `--strict`, for CI.

## Conventions

- **Dates live in document headers, not filenames.** Git has the rest.
- **Speculation is permitted and must be labelled.**
- **Superseded material is deleted, not archived in-repo.** Git history is the archive — a document
  that reads as authoritative does not stop doing so because a folder name says otherwise.
- **A negative result is not citable without its scope conditions (R2).**
