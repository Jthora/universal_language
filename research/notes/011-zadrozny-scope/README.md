# 011 — Zadrozny is narrower than we treated it, and the symbol-table position was over-general

**Type:** correction
**Opened:** 2026-08-01
**Status:** closed
**Thread:** corrects `003` (which established Zadrozny as killing logical necessity) and the
"no fixed symbol table" position stated repeatedly since.
**Prompted by:** *"I don't trust Zadrozny's work here for some reason… I think that's a card that
looks like a brick."*

---

## 1. What was held

**Two things, both over-broad.**

**(a) Zadrozny (1994) closes the necessity route.** For any meaning assignment a re-encoding exists
making it compositional; therefore compositionality constrains nothing; therefore no primitive
inventory can be derived. This was the first row of the three-kinds-of-universality table —
*"Logical necessity: **dead**"* — and it has been load-bearing since `003`.

**(b) There is no fixed universal symbol table.** Inventories are properties of a presentation.

## 2. What broke it

### (a) The critical literature, which had never been searched

**Zadrozny is the single most load-bearing negative in this project, and R1 had never been run
against it.** One search:

- **Kazmi & Pelletier (1998)**, *"Is Compositionality Formally Vacuous?"* — *Linguistics and
  Philosophy* 21, 629–633
- **Westerståhl (1998)**, *"On mathematical proofs of the vacuity of compositionality"* — same journal
- **Dever (1999)**
- And **Janssen (1983)** had an equivalent earlier result, similarly scoped

**The hole is specific and it is where you said it would be:**

> *"The primary motivation for the objections brought against Zadrozny's argument is the view that
> **his encoding of the original meaning function does not properly reflect the synonymy relations**
> posited for the language."*
>
> *"While Zadrozny's theorem is **mathematically sound**… the task of semantics is to identify a
> meaning assignment that respects both what our best syntax tells us about structure and what our
> best intuitions tell us about synonymy — **these results do not show compositionality to be
> empirically empty.**"*
>
> *"For compositionality to be a substantive constraint… it must be **paired with background
> constraints on syntax and the semantics of the basic lexical items**."*
>
> *"If we insist any acceptable semantic theory must respect what syntax tells us about the structure
> of complex expressions, **such results have limited force**."*

**The theorem is sound. The conclusion drawn from it is not.** Zadrozny buys compositionality by
re-encoding meanings in a way that **destroys synonymy relations** — it satisfies the letter of
compositionality while discarding the structure that made the question worth asking. Exactly a card
that is genuinely a card, presented as a brick.

### (b) The symbol-table position contradicted our own computation

`005` computed that under Erlangen coarse-graining, **Point and Enclosure survive to the topological
level** while the curvature family dies by projective. That identifies a *canonical* surviving
structure — derived, not chosen.

**"There is no fixed symbol table" and "these two survive coarsest" cannot both be stated
unqualified.** The first was applied at a level where it does not hold.

## 3. What replaced it

### On Zadrozny — scope, per R2

> **Scope:** Zadrozny applies to **unconstrained** compositionality — where the encoding of meanings
> is free and synonymy relations need not be preserved. It does **not** apply once a theory imposes
> background constraints on syntax and lexical semantics, which every substantive semantic theory
> does.

**And the constraints that close his escape hatch are ones this project already requires**: a
synonymy relation (that is `SEMANTIC-EQUALITY`, which we want regardless) and a fixed syntactic
structure (the geometric composition rules). Under our own commitments, the re-encoding trick is
unavailable.

**So the necessity route is not dead. It was killed by an over-broad reading of a correctly-proved
theorem.** It is *open and hard*, which is a materially different status. Note this does not
resurrect the retired grounding argument — that failed for circularity, a separate defect.

### On the symbol table

| Level | Status |
|---|---|
| Below the fixed point | Many presentations, cardinality varies, **no privileged choice** |
| **At the fixed point** | **The surviving structure is canonical — derived, not chosen** |

**A fixed universal symbol table is the table at the fixed point**, and the Erlangen computation
already says what is in it: **distinction (Jordan separation) and incidence.** Small, derived, not
arbitrary.

And the symmetry-grid approach and the Landau **G/H** derivation are **the same move from two
directions**: if a symmetry group generates the table, the group determines its contents. "Which
symbol table" and "what are G and H" are one question.

## 4. Why it survived as long as it did

**The rule existed and was never applied retroactively.** R1 mandates an adversarial search on every
negative — but it was written *after* Zadrozny had been in use for the whole project, and nothing
required auditing negatives that predated the rule.

**So the oldest and most load-bearing negatives — the ones that shaped everything downstream — were
exactly the ones that never got checked.** New findings got the discipline; foundational ones were
grandfathered in.

**Generalized (F-019): when a rule is introduced, apply it retroactively to the existing corpus.**
A rule that only governs new work leaves the load-bearing decisions unexamined, and those are the
ones with the most leverage.

**Second-order note:** this is the fourth instance of scope-dropping on a negative. But it was caught
by the protocol working as designed — the challenge was raised, the search was one query, the answer
was unambiguous. The mechanism functions; it just had a blind spot for its own back-catalogue.

## 5. What changed

- `claims.yaml`: `ZADROZNY-SCOPE` added; `UL-IS-EMERGENT-UNIVERSAL` notes amended so "logical
  necessity is dead" is no longer stated unqualified; `ANCHOR-IS-DISTINCTION-AND-INCIDENCE` amended
  to state the fixed-point table explicitly.
- `FAILURES.md`: F-019.
- **Left open — and now genuinely open:** whether *constrained* compositionality plus a fixed
  syntax can derive an inventory. That is the necessity route, reopened. Reading
  Kazmi & Pelletier, Westerståhl and Dever in full is the next step, at evidence tier S1-2
  (primary source, read directly) rather than tier 5.
