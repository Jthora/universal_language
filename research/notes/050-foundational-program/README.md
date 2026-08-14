# 050 — The foundational program: baseline for UL, en route to UQPL

**Type:** decision
**Opened:** 2026-08-12
**Status:** open — this is the standing program; cycles close against it
**Thread:** absorbs Volley 5 of `044`; sets the requirements every later cycle answers to
**Prompted by:** *"We need a solid foundation for UL/UWS/UP. Really consider that we want to
establish that UQPL... not practical until we've established a baseline... We cannot settle for 'it
can't be done', and we cannot settle for 'lets just reclassify it or rename it so we win'. It must
be a language, and it has to authentically be universal in a meaningful and literal way."*

---

## 1. The contract — four requirements, owner-set, registered as claims

These are **success criteria, not aspirations.** They bind every future cycle and every future agent,
and they are registered in `claims.yaml` so the checkers see them.

- **REQ-1 — IT MUST BE A LANGUAGE.** Syntax, compositional semantics, expressive adequacy,
  transmissibility. A notation alone is not a language. A protocol alone is not a language. A
  universality class alone is not a language. **Retreating to any one of these and declaring victory
  is the rename-to-win move, and it is excluded.**
- **REQ-2 — UNIVERSALITY MUST BE LITERAL AND QUANTIFIED.** The shared prior an interpreter needs is
  bounded by what any embodied computational agent in this universe has: geometry, causality,
  computation. Cultural priors on the load-bearing path are failure. **Every convention the design
  requires goes on a ledger, measured** — the way `ROTATION-MINIMIZES-CONVENTION` already measures
  ℤ/2 against S_n.
- **REQ-3 — NO-RETREAT FALSIFIABILITY.** If a conventional residue can neither be eliminated nor
  derived, that is recorded as **failure of literal universality** — not re-scoped, not renamed.
  This is what makes REQ-2 a claim rather than a slogan. Symmetrically: **"it can't be done" is
  itself a claim and gets the full negative-result discipline** (R1–R6). Neither direction gets a
  free verdict.
- **REQ-4 — BASELINE BEFORE UQPL.** The dependency chain is UWS (syntax) → grounded semantics →
  UL (language) → **UQPL** (a typed calculus over it). Work lands on the earliest unfinished link.

## 2. What UQPL actually presupposes — the spec read against the repo

The wiki spec (`wiki.fusiongirl.app/wiki/UQPL`) is a typed lambda calculus over the five geometric
primitives, computing `MeaningStructure* → MeaningStructure*`, **"SPECIFICATION-LEVEL — the page is
the spec, not a running implementation."** Read against the rebuilt repo:

| UQPL presupposes | Repo status | What discharges it |
|---|---|---|
| **Sorts from primitives** — Point→Entity, Line→Relation, Angle/Curve→Modifier, Enclosure→Assertion | **This is `meaning → map` asserted by fiat.** The unbuilt bridge, named | the semantic stack, §5 |
| **"Five geometric primitives generate all meaning structures"** | five is a retired count; the fixed point is **indexed by junction degree** | derive sorts from the map, not from the legacy inventory |
| **11 primitive operations** | a chosen count. The origami lesson (`047`): a derived count is an **enumeration theorem over a stated domain** — Lucero's 7→8 | enumerate admissible operations; prove the enumeration |
| **Erlangen levels 0–4, "abstraction is irreversible"** | ✅ **derived and machine-checked** (`022`–`024`, `026`) — the strongest alignment | already discharged |
| **"Open-ended class of admissible, invariant-preserving transformations"** (Simulation Hypothesis page) | ✅ matches the repo's retirement of fixed operator tables | already aligned |
| **`negate`/`conjoin`/`quantify` as topological region operations** | `REGIONS-ARE-FACE-UNIONS`: complement, intersection over a **finite face algebra** — decidable by construction | ground assertion logic in the face algebra; `quantify` over ℝ² vs a finite face set is the known hard spot |
| **Turing-completeness, optimal reduction, Curry–Howard** | spec's own open problems 1, 4, 5 | the operational layer, §5 M2 — and cycle `051` has a specific candidate |

**The spec's good bones are real, and its foundation is the pre-rebuild one.** The baseline program
is what replaces fiat with derivation underneath it.

## 3. Why UL is still hard — stated precisely, once

Not "universality is impossible" — `049` dismantled that aggregation. The actual hard core is one
circularity:

> **Every grounding scheme presupposes that the receiver applies the right reading procedure. The
> reading procedure is itself a convention — unless it is forced.**

Icons need the receiver to know *which* resemblance counts (F-012: *"our interpretation of pictures
relies on cultural assumptions"*). Indices need the receiver to recognize the pointing relation.
Symbols are convention outright. **Every prior failure in the record bottoms out here**, and every
surface-level effort tripped on it without naming it.

**The exit, and it is proof-shaped (T5):** meaning recovery must be **reading-procedure-independent**.

> **READING-INVARIANCE (theorem target):** for every reasonable forgetful reading F of a
> configuration, the invariants recoverable through F contain the Erlangen fixed point — and over
> the class of all such F, equal it. *It does not matter how you look at it.*

`014`/`022`/`024` prove the survival direction for specific towers. **The quantification over
readings is the open mathematics**, and it is the single theorem that would make syntax-layer
universality literal rather than argued. Registered as `READING-INVARIANCE-TARGET`, CONJECTURED,
with its falsifier.

## 4. The convention ledger — universality as arithmetic

REQ-2, operationalized. Every layer lists its conventional residue; success is every line either
**0 (derived)** or **a proven minimum with a bootstrap route (UP)**.

| Layer | Residue | Status |
|---|---|---|
| marks → combinatorial map | **ℤ/2** (orientation) | **proved, machine-checked** (`convention_ambiguity()`) |
| map → meaning | **unbounded** | the gap, made quantitative — this is what "the gap" *is* |
| symbol layer (any alphabet Σ) | **n!** per alphabet | permitted only as *derived/bootstrapped*, cost on ledger |
| physical anchors | shared physics | allowed prior under REQ-2 (SI-style invariants) |

**"Authentically universal in a meaningful and literal way" = the ledger closes.** That is the
definition this program commits to, and REQ-3 is what keeps it from being quietly renegotiated.

## 5. The semantic stack — three convention-minimal routes, each with its named objection

The gap is not attacked head-on as one problem; it decomposes into three routes with three distinct
literatures, **each objection stated before the evidence is gathered:**

- **M1 — EXEMPLIFICATION.** The mark *instantiates* the property it denotes. A closed curve does not
  depict enclosure; **it encloses** — Jordan certifies the possessed property as a theorem about the
  mark itself. This dodges F-012, which is about *depiction*.
  **Named objection (Goodman's selection problem):** a sample exemplifies only *some* of its
  properties — the tailor's swatch exemplifies weave, not size. *Which* properties? **Candidate
  answer, ours:** the Erlangen filtration selects canonically — what survives coarse-graining *is*
  what is referred to. The selection is derived, not chosen.
- **M2 — OPERATIONAL.** Meaning as behavior under rewriting: the program's meaning is what it does,
  no interpreter convention required beyond the rules — and rules can be *exhibited* by execution,
  which is UP's job. **Named objection:** operational meaning is closed (computation about
  computation) until M3 connects it to the world; and the rules' transmission cost goes on the
  ledger.
- **M3 — INDEXICAL.** Reference into the shared world by causal/physical connection — Peirce's
  index, METI's physics anchors, SI-style invariants. **Named objection:** F-012's cousin — the
  receiver must recognize the pointing relation; bounded by shared physics, which is exactly the
  prior REQ-2 allows.

**Symbols enter only on top, derived, with their n! on the ledger.** That inversion — icon/index
first, symbol last — is the structural difference from every kind-A failure in `037`'s record.

## 6. Preregistration for cycle `051` (S2 — written before searching)

**P1 — Interaction nets.** Lafont's interaction nets/combinators: local graph rewriting where agents
carry a **principal port and ordered auxiliary ports** — prediction: this is **rotation structure
carrying operational meaning**, in a vocabulary (PL theory) the `046` sweep never searched. If it
holds, it (a) flips `SEMANTIC-FORMALISMS-USE-LABELS-NOT-ORDER` in the *operational* register, in the
direction that **helps** the bridge; (b) hands M2 a Turing-universal substrate that is already our
fixed-point object; (c) bears directly on UQPL's open problems 1 (Turing-completeness) and 4
(optimal reduction — Lamping/Asperti-Guerrini optimal λ-evaluation runs on sharing graphs).
**Adversarial check, named now (S11):** ports may be *numbered labels*, not cyclic order — the
word-match trap. Check the actual structure before claiming.
**P2 — Institutions.** Goguen–Burstall: *"truth is invariant under change of notation"* as a formal
satisfaction condition. Prediction: it gives the **shape** of the notation-independence obligation
but is definitional — a specification of what we must prove, not a discharge of it.
**P3 — Goodman.** The selection problem is the sharpest published objection to convention-free
meaning; prediction: **no published solution via a canonical invariant filtration** (sweep:
exemplification · symbol grounding · natural meaning vocabularies before claiming the cell empty).

**The adversary front, named in advance for `052`:** Quine (inscrutability of reference),
Kripke–Wittgenstein (rule-following), Harnad (symbol grounding). **These are the field's standing
impossibility results for semantics, and they get Zadrozny treatment on first contact** — scope
checked before import (T8), adversarially searched (R1), not allowed to end the conversation (T4).
Note the prior: Harnad's own *solution* direction grounds symbols in icons — which is M1's side, not
against it.

**Stop condition for `051`:** four searches, one primary attempt. The 18-box checklist runs at close
— as a step, not a memory (`048`).

## 7. What failure would look like — written now, because REQ-3 demands it

- The ledger's semantic line cannot be bounded: some load-bearing element stays an S_n choice that
  can neither be eliminated, derived, nor bootstrapped. **Then literal universality fails and we say
  so.**
- READING-INVARIANCE is refuted: a reasonable reading class is exhibited through which the fixed
  point is not recoverable. Then syntax-layer universality is convention-relative, and the ledger
  records *which* convention.
- The adversary front holds after scope-checking: a Quine/Kripkenstein-shaped argument survives R1
  and T8 **inside our scope**, not just near it. Then the semantic stack is rebuilt or the failure
  is recorded.

**None of these are expected. All of them are stated, because a program that cannot fail cannot
succeed either.**
