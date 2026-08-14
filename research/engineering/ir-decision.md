# The IR Decision — Acyclic Core, Coreference Labels, E-Graphs

**Date:** 2026-08-01
**Resolves:** `claims.yaml#IR-NORMALIZATION-STRATEGY` (priority 0, blocking) and `FAILURES.md` F-011
**Status:** Recommendation, with the decidability boundary now precisely located.

---

## 1. The decidability boundary, precisely

Confluence — the property that makes normalization (and hence equality-by-normal-form) work — is
decidable for some rewriting settings and not others. The line falls here:

| Setting | Confluence decidable (given termination)? |
|---|---|
| **Terms** (trees) | **Yes** — Critical Pair Lemma; joinability of critical pairs entails confluence |
| **Term graphs / DAGs** (terms with *sharing*, acyclic) | **Yes** — Plump's Critical Pair Lemma for term graph rewriting gives "a decision procedure for confluence in the presence of termination" |
| **General graphs, DPO** (cycles allowed) | **No** — undecidable in general; critical-pair joinability does *not* entail confluence |
| **DPO with interfaces (DPOI)** | **Yes** — "interfaces play the same role as variables in term rewriting systems" |
| **DPO with all critical pairs *coverable*** | **Yes** — decidable under this syntactic condition |

**The boundary is cycles, not graphs.** Sharing is free — DAGs keep the decision procedure. What
breaks it is cyclicity (and unrestricted graph structure generally).

---

## 2. Where GIR sits, and why — this was a deliberate design choice

From the repo's architecture notes (since removed — git history):

> *"**Self-reference:** A glyph can refer to itself (the concept of 'self-awareness' contains
> itself). **This creates a cycle.** Trees are acyclic by definition."*

The document's own worked example contains a cycle: node `n1` *contains* `n5`, and `n5`
*references* `n1`. GIR is cyclic **on purpose** — self-reference was treated as a headline
capability throughout the project (the self-nesting construction ○{○{○{...}}}, π₁ = ℤ,
"self-reference without paradox").

**So the single feature the project most prized is exactly what places GIR in the undecidable
regime.** That is the real tradeoff, and it has never been stated: *self-reference as literal graph
cycles* versus *decidable semantic equivalence*. You can have either cleanly. Not both, in that form.

### The detail that makes this concrete

`negate` is implemented (`composer.rs`) by wrapping the assertion and attaching a **self-loop
`references` edge** as a negation marker.

So the operation whose algebraic law we most want to verify — involution, `negate(negate(a)) = a` —
is implemented using precisely the cyclic construct that makes verification undecidable. F-006 (the
law is unverified) and F-011 (normalization is not guaranteed) are not two problems. They are one
problem seen from two directions.

---

## 3. The fix: three independent lines converge

Three separate literatures, approached for different reasons, all point at the same technique.

**(a) Term graph rewriting** — confluence is decidable for DAGs. Keep sharing, drop cycles.

**(b) DPO with interfaces** — confluence becomes decidable because *"interfaces play the same role
as variables in term rewriting systems."* Variables, not cycles, carry the cross-linking.

**(c) CGIF (ISO/IEC 24707)** — Sowa's Conceptual Graphs face exactly this problem (a graph notation
for logic needing coreference) and solve it with **labels rather than edges**:

> Concepts are `[Type: Referent]`; relations are `(Relation [Concept] [Concept])`. Coreference uses
> `*x` as a **defining label** and `?x` as a matching **bound label** indicating reference to the
> same instance.

Example: `(Agnt [Go: *x] [Person: John]) (Dest ?x [City: Boston]) (Inst ?x [Bus: *y])`

Three occurrences of the same event, linked by `?x` — **no cycle in the term structure.** Coreference
is expressed as variable binding, which is exactly what (a) and (b) say restores decidability. An
ISO standard already solved this in the way the rewriting theory says you must.

### Recommendation

> **Make the normalizing core of GIR an acyclic term graph. Express self-reference and coreference
> as labels/variables (CGIF-style `*x` / `?x`), not as cyclic edges.**

This is the `graph-with-tree-spine` instinct, sharpened: the spine should be a **DAG** (sharing is
fine, and it's what term graphs are *for*), and cross-links must be **bindings**, not back-edges.

**Nothing expressive is lost.** Coreference and self-reference remain fully representable — they
move from the edge layer to the label layer. What's gained is a decidable confluence check and
access to the entire term-rewriting toolchain.

---

## 4. The tooling: e-graphs, and they're already in Rust

For deciding equality modulo a set of equations, the modern approach is **e-graphs with equality
saturation**, and it fits this project unusually well:

- An **e-graph** compactly stores a congruence relation over terms, built on union-find.
- **Equality saturation** applies rewrite rules non-destructively until saturation, then asks
  whether two terms landed in the same e-class.
- **`egg`** (POPL 2021) is a fast, extensible, **open-source Rust library** implementing this, with
  "rebuilding" for amortized congruence maintenance and "e-class analyses" for domain-specific
  extensions. It's production-used in compiler optimization and program synthesis.

**`ul-forge` is Rust.** This is close to drop-in for the acyclic core.

### Why this is better than plain Knuth-Bendix here

Knuth-Bendix requires orienting equations into terminating rules and may fail to complete. Equality
saturation **does not require choosing a canonical normal form** — it explores all rewrites
simultaneously and checks e-class membership. Confluence stops being a prerequisite and becomes an
optimization. With a timeout it degrades gracefully to a sound semi-decision procedure, which is
exactly the right failure mode for a checker.

**Concrete first milestone:** encode the notation's laws (`negate(negate(a)) → a`,
`conjoin(a,a) → a`, De Morgan) as `egg` rewrite rules over the acyclic core, and implement
`semantically_equal` as e-class equality. That single step closes F-006, unblocks
`SEMANTIC-EQUALITY`, and gives the Cure its missing equivalence primitive.

---

## 5. What this costs — stated honestly

1. **GIR's data model changes.** `references` edges that create cycles must become coreference
   labels. This touches the schema, parser, composer, validator, and serialization. It is not
   cosmetic.
2. **`negate` must be reimplemented.** The self-loop marker has to go — replaced by the σ field
   (⊕/⊖ on the assertion) that the documentation has claimed since April but never had. This
   finally makes F-001's documented fix real.
3. **Some genuinely cyclic structures leave the normalizing core.** If a construction is
   irreducibly cyclic and not expressible as coreference, it lives outside the fragment where
   equivalence is decidable. That's a real boundary — and it should be *documented as the price of
   decidability*, not hidden.
4. **CGIF alignment is now a live option, not just prior art.** If the coreference mechanism is
   adopted anyway, exporting to CGIF becomes cheap, and that buys interoperability with an ISO
   standard and the Common Logic ecosystem.

---

## 6. Bonus: the UWS target, now concrete

The diagrammatic-reasoning tradition gives UWS a *template* for a real result:

**Shin (1994)** built two systems — **Venn-I** (Venn's shading for emptiness, Peirce's `x` for
existential import, connecting lines for disjunction) and **Venn-II** (adds lines *between*
diagrams) — presented each as a formal system with its own syntax (which diagrams are well-formed,
which manipulations are permitted) and semantics (logical consequence among diagrams), and **proved
both sound and complete.** Venn-II is provably equivalent to monadic predicate logic. Miller later
published a shorter completeness proof.

The significance, in the literature's own words: the result *"directly refuted a widely-held
assumption that diagrams are inherently misleading, and abolished theoretical objections to diagrams
being used in proofs."*

**The method transfers directly.** Define UWS's syntax, define its semantics, define transformation
rules, prove soundness, prove completeness relative to a stated fragment.

**One honest scope caveat:** Venn-II reaches **monadic** predicate logic — no polyadic relations.
UWS is explicitly relational (Line = Relation is a core primitive), so it targets something strictly
harder, where completeness is correspondingly more difficult. The realistic move is to prove
soundness and completeness **for a stated fragment first** and grow the fragment — which is exactly
how the Euler → Venn → Peirce → Shin sequence actually progressed.

---

## 7. Decisions to record

| Decision | Recommendation |
|---|---|
| `IR-NORMALIZATION-STRATEGY` | **Acyclic term-graph core; coreference via labels, not cycles.** Decidable confluence, full term-rewriting toolchain. |
| Equivalence implementation | **E-graphs / equality saturation via `egg`** (Rust, drop-in). Confluence becomes an optimization rather than a prerequisite. |
| Self-reference | **Retained, relocated** — expressed as coreference labels rather than graph cycles. |
| `negate` | **Reimplement with a σ field.** Deletes the self-loop hack and makes the April 2026 documented fix real. |
| CGIF | **Align the coreference mechanism**; treat CGIF export as a cheap follow-on and state GIR's relationship to ISO 24707 explicitly. |
| UWS formal target | **Soundness + completeness for a stated fragment**, following the Shin template, growing the fragment over time. |
