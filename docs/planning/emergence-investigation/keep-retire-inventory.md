# Keep / Retire Inventory — Splitting UWS from the Σ_UL Proof Apparatus

**Date:** 2026-08-01
**Purpose:** Classify every major file/directory in the repo into one of three buckets before any
deletion or rewrite happens. No files have been touched yet — this is the proposal for review.
**Context:** `docs/planning/emergence-investigation/PLAN.md` (why this split is happening),
`docs/planning/audits/wiki-comparison-2026-08.md` (the audit that surfaced the problem)

## The three buckets

- **KEEP-UWS** — Universal Writing System: the constructed geometric notation and its software
  tooling. Stands on its own merits (composability, learnability, expressiveness) without needing
  any claim of cosmic necessity. Reframe framing language; keep the substance.
- **KEEP-INVESTIGATION** — Raw material, protocols, or infrastructure the Emergence Investigation
  (the six-phase plan) can actually use. Kept as *candidate hypotheses / reusable tooling*, not as
  established results.
- **RETIRE** — The Σ_UL proof apparatus specifically: theorem numbering, "PROVEN" labeling, the
  Unique Grounding Theorem, "mathematically intrinsic to the universe" framing, and everything
  built assuming those hold. This is what's actually load-bearing-but-broken; retiring it doesn't
  touch UWS or the Investigation.

---

## Top-level docs

| File | Bucket | Reason |
|---|---|---|
| `README.md` | **RETIRE** (rewrite) | Proof-status table, "unique up to isomorphism" framing is the public face of the thing that doesn't hold up. Needs a full rewrite around UWS + the Investigation. |
| `AGENTS.md` | **RETIRE** (rewrite) | Same — machine-readable metadata block asserts the theorem counts as settled fact. |
| `FOR-AI.md` | **RETIRE** (rewrite) | The single most overclaiming document ("you are a geometric engine," every framework "is a Σ_UL-algebra"). |
| `RAMIFICATIONS.md` | **RETIRE** | Ten consequences explicitly "traced to specific theorems" that are being retired — the whole document's structure depends on them. |
| `CONTRIBUTING.md` | KEEP-UWS | Generic contribution process, not proof-dependent. |
| `CITATION.cff` | KEEP (as-is) | Just citation metadata. |
| `llms.txt`, `index.json` | **RETIRE** (rewrite) | AI-discoverability manifests built around the same claims; regenerate once top-level docs change. |
| `archive/README2-draft.md` | No action | Already inert/unused draft (never became the live README); actually the *most* overclaiming text in the repo ("sacred geometry," "bridges human consciousness"), but it's already archived, not live. Leave alone. |

## `foundations/`

| File | Bucket | Reason |
|---|---|---|
| `paradigm.md` | **RETIRE** | The "read first" document establishing UL as "the initial object in the category of meaning-bearing systems" — foundational overclaim everything else inherits. |
| `universal-language-derivation.md` | **SPLIT** | Part I (Point/Line/Angle/Curve/Enclosure definitions, drawability properties) and Parts II–IV (Symbology/Syntax/Grammar writing-system content) → KEEP-UWS. The "Foundational Axiom" and Part VI ("mathematical proof of universality") → RETIRE. |
| `formal-foundations.md` | **SPLIT** | Part IV (Unique Grounding Theorem) and the "PROVEN"/theorem-count framing → RETIRE. Part I (language-as-Σ-homomorphism definition) → KEEP-INVESTIGATION, useful raw formal apparatus for Phase 3's rebuild. |
| `formal-operations.md` | KEEP-UWS | Rigorous operational definitions for drawing/composing symbols — useful for the notation regardless of metaphysical status. Strip "proof of universality" language if present. |
| `independent-derivation.md` | KEEP-INVESTIGATION | Valuable as a *documented hypothesis/attempt*, not proof — it's what surfaced the Montague `e,t` point that drove the blind rederivation. Keep as raw material, reframe as "an attempt," not "resolution." |
| `montague-homomorphism.md` | KEEP-INVESTIGATION | Real formal-semantics content, directly useful for Phase 3. |

## `ul-core/` (Writing System)

| File/dir | Bucket | Reason |
|---|---|---|
| `CRITIQUE.md` | KEEP-INVESTIGATION | Valuable as an honest historical audit log (it's what surfaced finding F7 in the first place). Its target shifts once Σ_UL's proof framing is gone, but it stays as process-transparency record. |
| `NAVIGATION.md`, `SYNTHESIS.md` | KEEP-UWS | Structural guides to the writing system. Reframe away from "this pipeline IS reality" language. |
| `writing-system/` (writing-system.md, writers-companion.md) | KEEP-UWS | The actual practical read/write spec and worked examples — the heart of the real deliverable. |
| `symbology/`, `syntax/`, `grammar/`, `thesaurus/`, `lexicon/` | KEEP-UWS | The five writing-system siblings. This is the substance worth keeping. |
| `uqpl/` | KEEP-UWS | Programming-language-adjacent tool, low metaphysical baggage either way. |

## `whitepaper/` + `proto-analysis-papers/`

Near-duplicate directories (4-line diff across all 3 parts), **both already self-labeled**
`⚠ HISTORICAL DOCUMENT`. No action needed on framing — someone already did this correctly. Only
housekeeping suggestion: `proto-analysis-papers/` is a pure duplicate of `whitepaper/` and could be
deleted to remove redundancy; not urgent.

## `frontier/`

| File/dir | Bucket | Reason |
|---|---|---|
| `strategic-plan.md`, `gap-analysis.md` | **RETIRE** (rewrite) | Built on "UL is the initial object in Lang(Σ_UL)" framing throughout. |
| `methodology.md` | **SPLIT** | The four-label rigor system and self-critique discipline is genuinely good methodology — directly reusable for the Investigation's Phase 0 discipline. Strip the Σ_UL-specific framing, keep the discipline. |
| `causal-efficacy-protocol.md` | KEEP-INVESTIGATION | This *is* Phase 5's target for rehabilitation, not something to retire. |
| `expedition-one/`, `expedition-two/` | KEEP-INVESTIGATION | Exploratory math (category theory, gauge bundles, probability/information) — raw material for Phase 3, especially since Phase 3 already leans on the Curry–Howard–Lambek convergence this material gestures toward. Reframe away from "proven" language. |

## `experiments/`

KEEP-INVESTIGATION as infrastructure — this is Phase 5's target (test artifacts, scoring rubrics,
`blind.py`, analysis scripts are genuinely reusable). `qc-audit-report.md` and
`D2-completeness-challenge.md` need their "100% complete / proof" framing rewritten, but the
underlying 50-case test data can be kept as raw material.

## `history/`

**RETIRE** (rewrite) — `mechanism-of-action.md`, `primer-analysis.md`, `reverse-engineering.md` all
assert "the primer works because it IS compressed Universal Language" as established fact, ahead of
the rehabilitated causal-efficacy protocol ever actually being run. This is premature-conclusion
framing that should wait for real Phase 5 results. The original primer artifact itself (whatever
inspired this line of inquiry) is worth preserving as a historical curiosity, not as evidence.

## `applications/`

**RETIRE** (rewrite) — `applications.md` explicitly states "Derives from: ...the Unique Grounding
Theorem. Zero external artifacts." Once that theorem is retired, this document's foundation is
gone; rewrite from whatever the Investigation actually establishes, or hold until then.
`applications/research/` is an empty directory — no action needed.

## `docs/planning/audits/improvements/` (pass1 through pass3)

**Keep as historical record, retag — do not delete.** This is the actual paper trail showing the
patch-until-it-passes pattern (useful, honest evidence of process, referenced in the emergence
investigation plan itself). Retag as "superseded process history," not "current status of the
theory."

## `docs/planning/testing/rigor/pass1/`

Adversarial stress-test plans (sci-fi scenarios, sort-boundary, invariant-laws, cross-operation)
built to defend Σ_UL specifically. Low priority to touch — the *adversarial-testing methodology*
itself is reusable for whatever the Investigation concludes; the current content assumes Σ_UL is
the thing being defended.

## `ul-forge/` + `docs/ul-forge-v1/` + `docs/distribution/` + `docs/learning/`

**KEEP-UWS, largely untouched.** This is real software engineering (parser/renderer/composer,
WASM bindings, web editor, VS Code extension, MCP server, transceiver protocol) implementing the
*notation*, not the metaphysical claim. None of it requires Σ_UL's proof apparatus to be valuable.
Light touch only: anywhere docs describe "13 operations" as proven-necessary rather than as the
notation's compositional feature set.

---

## Summary counts

| Bucket | Rough scope |
|---|---|
| RETIRE (rewrite) | README.md, AGENTS.md, FOR-AI.md, RAMIFICATIONS.md, llms.txt, index.json, `foundations/paradigm.md`, the Foundational-Axiom/proof parts of `universal-language-derivation.md` and `formal-foundations.md`, `frontier/strategic-plan.md` + `gap-analysis.md`, `history/*`, `applications/*` |
| KEEP-INVESTIGATION | `independent-derivation.md`, `montague-homomorphism.md`, `ul-core/CRITIQUE.md`, `frontier/causal-efficacy-protocol.md`, `frontier/expedition-one/`, `frontier/expedition-two/`, `experiments/`, `docs/planning/audits/*` (retagged) |
| KEEP-UWS | All of `ul-core/` (writing system content), `ul-forge/`, `docs/ul-forge-v1/`, `docs/distribution/`, `docs/learning/`, plus the salvageable Parts I–IV of `universal-language-derivation.md` |

No files have been modified. This is the proposal for your review before anything is touched.
