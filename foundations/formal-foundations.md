# Formal Specification — Universal Writing System Notation & Operations

**Status note (2026-08-01):** This document has been split. The original, in full, is preserved at
`archive/superseded-2026-08/foundations/formal-foundations-FULL-ORIGINAL.md`. Its Parts III
("Isomorphism Theorem"), IV ("The Grounding Problem — Resolved" / the Unique Grounding Theorem),
V ("Strengthened Universality Proof"), and the old Part VII summary have been **retired**, not
kept here — a direct audit found the Unique Grounding Theorem is close to circular (it defines
5 "semantic primitives" with role properties written to mirror the 5 geometric primitives
property-for-property, then proves a bijection exists between two hand-matched lists), and a
from-scratch rederivation using standard model theory / Montague grammar gives a smaller, different
minimal answer (2 base types, `e` and `t`) than the claimed 4-sort / 5-primitive / 13-operation
signature. See `docs/planning/emergence-investigation/PLAN.md` and
`docs/planning/audits/wiki-comparison-2026-08.md` for the full reasoning.

**What's kept below** is the part of this document that was always doing honest work regardless of
the retired metaphysical claims: a formal definition of "a language" as a Σ-algebra homomorphism
(standard universal algebra, Birkhoff 1935), and the operational specification of how the Universal
Writing System's geometric notation realizes each operation. This is **notation software spec**,
not a proof that the notation is uniquely forced by the nature of meaning. Whether any version of
"Universal Language" is real is now an open, actively-investigated question — see
`docs/planning/emergence-investigation/`.

**Companions:** `foundations/formal-operations.md` (rigorous set-theoretic operation definitions),
`foundations/independent-derivation.md` and `foundations/montague-homomorphism.md` (kept as raw
material for the Emergence Investigation, not as settled results).

---

## PART I: FORMAL ALGEBRAIC DEFINITION OF "LANGUAGE"

### 1.1 What Must a Definition Capture?

A definition of "language" as a mathematical object must capture:
- **Expressions:** finite structures built from atoms
- **Meanings:** a domain of things that can be meant
- **Interpretation:** a systematic mapping from expressions to meanings
- **Compositionality:** the meaning of a compound expression is determined by the meanings of its parts and how they're combined

The correct mathematical framework is **universal algebra** (Birkhoff, 1935) combined with **Montague semantics** (Montague, 1970): a language is a homomorphism between two algebras of the same type.

### 1.2 Definition: Signature

A **signature** Σ = (S, Ω) consists of:
- A set S of **sorts** (types of objects)
- A set Ω of **operation symbols**, each with an arity: ω: s₁ × s₂ × ... × sₙ → s₀

Intuitively: sorts are the categories of linguistic objects (entities, relations, modifiers, etc.), and operations are the ways of combining them.

### 1.3 Definition: Algebra

A **Σ-algebra** A consists of:
- For each sort s ∈ S, a carrier set Aₛ
- For each operation ω: s₁ × ... × sₙ → s₀ in Ω, a function ωᴬ: Aₛ₁ × ... × Aₛₙ → Aₛ₀

### 1.4 Definition: Language

A **language** over signature Σ is a triple L = (E, M, ⟦·⟧) where:

- **E** is a Σ-algebra called the **expression algebra** (syntax)
- **M** is a Σ-algebra called the **meaning algebra** (semantics)
- **⟦·⟧: E → M** is a Σ-**homomorphism** called the **interpretation function**

The homomorphism condition enforces **compositionality**:

> For every operation ω ∈ Ω and expressions e₁, ..., eₙ:
>
> ⟦ω^E(e₁, ..., eₙ)⟧ = ω^M(⟦e₁⟧, ..., ⟦eₙ⟧)

This says: combining expressions and then interpreting gives the same result as interpreting and then combining meanings. This is the **fundamental theorem of compositionality** — the defining property of a compositional language.

### 1.5 The Universal Writing System's Signature

The notation defined below is built around the following signature. **This is a design choice for
a specific notation, not a claim that it is the unique minimal signature for meaning** — see the
status note above; a from-scratch minimality analysis gives a smaller answer (2 base types under
standard simple type theory).

**Sorts:**

| Sort | Symbol | Intuition |
|---|---|---|
| Entity | **e** | Things that can be talked about |
| Relation | **r** | Ways things can relate |
| Modifier | **m** | Ways to alter entities or relations |
| Assertion | **a** | Complete statements (sentences) |

**Operations:**

| Operation | Type signature | Intuition |
|---|---|---|
| **predicate** | e × r × e → a | Combine subject, relation, predicate into a statement |
| **modify_entity** | m × e → e | Apply modifier to an entity |
| **modify_relation** | m × r → r | Apply modifier to a relation |
| **negate** | a → a | Negate a statement |
| **conjoin** | a × a → a | Combine two statements with AND |
| **disjoin** | a × a → a | Combine two statements with OR |
| **embed** | a → e | Turn a statement into an entity (nominalization: "that it rains...") |
| **abstract** | e → m | Turn an entity into a modifier (adjectivalization: "the wooden...") |
| **compose** | r × r → r | Chain two relations (transitivity) |
| **invert** | r → r | Reverse a relation (active ↔ passive) |
| **quantify** | m × e → a | Apply a quantifier-modifier to an entity to make an assertion |
| **bind** | e × a → a | Bind a slot entity (co-reference + scope delimitation) |
| **modify_assertion** | m × a → a | Apply modifier to assertion's frame (evidentiality, emphasis, hedging) |

This gives a **complete and natural** generating set for the notation (in the sense that natural
languages universally have both AND and OR, etc.) — not a claim of minimal independence.
`conjoin` is derivable from `{negate, disjoin}` via De Morgan's law and is retained for
naturalness/readability, consistent with `docs/planning/audits/improvements/pass1-1/findings/critical-errors.md`
finding F2, which corrected an earlier overclaim that this set was minimal.

### 1.6 Definition: Expressively Complete Language (for this notation)

A language L = (E, M, ⟦·⟧) over this signature is **expressively complete** if:
1. The entity sort Eₑ contains at least a countably infinite set of distinct atoms (one can talk about arbitrarily many things)
2. The interpretation ⟦·⟧ is surjective onto a meaning algebra M that contains all finite relational structures over distinguishable elements
3. The interpretation ⟦·⟧ distinguishes non-equivalent expressions (if two expressions have different "structural meaning," they map to different meanings)

---

## PART II: THE GEOMETRIC NOTATION AS A Σ-ALGEBRA (Operational Specification)

### 2.1 Construction

> **Note:** The operation table below provides intuitive geometric descriptions. For **rigorous set-theoretic definitions** with formal proofs of closure, totality, determinism, and injectivity for each operation, see `foundations/formal-operations.md`.

This section specifies how the geometric notation `G` realizes each operation — i.e., how to
actually draw/compose the writing system's marks. This is software/notation specification, not a
metaphysical claim about G being the unique or necessary realization.

**Carrier sets:**

| Sort | Geometric carrier set | Definition |
|---|---|---|
| **e** (Entity) | Gₑ = {all geometric constructions within a glyph space} | Points, enclosures (with content), labeled regions |
| **r** (Relation) | Gᵣ = {all directed connections between geometric constructions} | Lines, rays, arcs, with angle and curvature data |
| **m** (Modifier) | Gₘ = {all geometric transformations} | Elements of the Euclidean group E(2), similarity group Sim(2), and projective group PGL(3), plus topological operations |
| **a** (Assertion) | Gₐ = {all geometric constructions within a sentence frame} | A bounded region containing a complete subject-relation-predicate construction |

**Operations:**

| Operation | Geometric realization |
|---|---|
| **predicate(e₁, r, e₂)** | Place entity-glyph e₁ and entity-glyph e₂ in a sentence frame, connected by relation r (a directed line/arc at a specific angle). The construction is a complete figure: e₁ →ʳ e₂ within a bounding frame. |
| **modify_entity(m, e)** | Apply transformation m to entity-glyph e. Scaling changes emphasis; rotation changes perspective; projection changes abstraction level. The result is a new entity-glyph: m(e). |
| **modify_relation(m, r)** | Apply transformation m to relation r. Scaling changes intensity of relation; rotation changes the character; reflection reverses the valence. The result is a new relation: m(r). |
| **negate(a)** | Flip the frame boundary from solid to dashed (or vice versa). Content is unchanged; only the assertional sign flips. Solid = asserted, dashed = denied. Involution: negate(negate(a)) = a. ✓ |
| **conjoin(a₁, a₂)** | Place sentence-frames a₁ and a₂ so that they overlap (shared boundary). The overlapping region is the shared context = AND. |
| **disjoin(a₁, a₂)** | Place sentence-frames a₁ and a₂ adjacent (touching but not overlapping). Either frame can be read = OR. |
| **embed(a)** | Shrink sentence-frame a and place it inside an enclosure, converting it into an entity-glyph. This is **nominalization**: a statement becomes a thing that can be talked about. |
| **abstract(e)** | Extract the boundary/shape properties of entity-glyph e and produce a transformation (modifier) that imposes those properties on other entities. This is **adjectivalization**: an entity becomes a quality. |
| **compose(r₁, r₂)** | Concatenate two directed connections: the endpoint of r₁ becomes the startpoint of r₂. The resulting relation has the combined angle and curvature. |
| **invert(r)** | Reverse the direction of the directed connection. A ray → becomes ←. |
| **quantify(m, e)** | Apply a quantifier-transformation to an entity, parameterized by a continuous frame-fill proportion p ∈ [0,1]: **p = 1** (fills frame) = universal ("all"); **p ∈ (0,1)** (partial fill) = graduated ("most," "few," "several"); **p → 0⁺** (point-like) = existential ("some"); **negate + existential** = negative ("no"). See `formal-operations.md` §1.11 for the area-proportion formula. |
| **bind(e_x, a)** | Given a slot entity e_x (hollow mark ○ with label x) and an assertion a containing occurrences of ○_x, replace all ○_x with ●_x (filled mark, same label). This establishes **co-reference** (all occurrences denote the same entity) and delimits **scope** (the frame of a is the scope boundary). Scope ordering = nesting depth of bind operations. See `formal-operations.md` §1.12. |
| **modify_assertion(m, a)** | Apply a modifier (transformation) to the assertion's **frame boundary** without changing content C or assertional sign σ. Encodes evidentiality (dotted frame = "apparently"), emphasis (double frame = "definitely"), hedging (wavy frame = "sort of"). Orthogonal to `negate` (which flips σ) and composes freely with it. See `formal-operations.md` §1.13. |

### 2.2 Verification: G Is a Well-Defined Σ-Algebra

Each operation must be:
1. **Closed:** output is in the correct carrier set ✓ (predicate produces a sentence-frame; modify_entity produces an entity-glyph; etc.)
2. **Total:** defined for all valid inputs ✓ (any two entities can be connected by a relation; any transformation can be applied to any glyph; etc.)
3. **Deterministic:** each input combination produces exactly one output ✓ (geometric construction is deterministic)

G is a valid Σ-algebra for this notation. ∎ (This confirms the notation is internally consistent —
it does not, on its own, establish anything about meaning-in-general.)

---

## PART III–V: RETIRED — replaced by the Convergent Operational Core

The original Parts III ("The Isomorphism Theorem"), IV ("The Grounding Problem — Resolved"), and
V ("Strengthened Universality Proof") — including the "Unique Grounding Theorem" — have been
retired to `archive/superseded-2026-08/foundations/formal-foundations-FULL-ORIGINAL.md`. They are
not restated here because a direct audit found the central argument close to circular: it defines
5 "semantic primitives" with role properties constructed to mirror the 5 geometric primitives
already chosen for the notation, then presents the resulting forced bijection as a proof of
necessity.

**Replacement, completed (and self-corrected once — see that document's Part E for what was fixed
after review):** `docs/planning/emergence-investigation/phase3-formal-necessity-rebuild.md`.
Summary: the Curry–Howard–Lambek correspondence (type theory ≅ lambda calculus ≅ Cartesian closed
categories) forces an infinite, freely generated hierarchy of derived semantic categories from
`e`/`t` — but only for the *intuitionistic* fragment (predicate combination, quantification); the
notation's *classical*, bivalent `negate` does not inherit justification from that correspondence
(classical logic needs the separate Griffin/Parigot control-operator extension) and instead rests on
its own, separately-grounded evidence: independent convergence in Aristotelian/Nyāya logic plus the
strongest non-human data point found (Alex the parrot's "none"). The notation's further step —
flattening the hierarchy into 4 named, drawable sorts — was demonstrated (not merely asserted) to be
expressively equivalent to a 3-sort alternative, at the cost of relocating disambiguating
information into explicit tags; a 6-sort alternative was described but not checked with the same
rigor and is marked unverified. `embed`, `bind`, `compose`, `invert`, and `modify_assertion` have no
independent convergent evidence found either for or against necessity, and are left honestly
unresolved rather than assumed.

---

## PART VI: NOTATION EXTENSIONS — MODAL, PERFORMATIVE, PRAGMATIC

The subsections below specify additional notational conventions for modality, illocutionary force,
and pragmatic inference. They are kept as **operational notation specification** — how to draw and
compose these constructions — with the earlier "this proves the signature is complete/minimal"
framing removed. Whether the underlying 4-sort/13-operation signature they extend is itself
necessary (vs. a design choice) is the open question under investigation, not something these
extensions settle either way.

### 6.1 Modal Extension

Modality — necessity, possibility, obligation, ability, counterfactuals — is expressible in this
notation via **defined operators** built from existing operations, without adding new sorts or
operations to the signature in §1.5.

**Distinguished elements added to existing carrier sets:**

| Element | Sort | Role |
|---------|------|------|
| $w_{\text{current}}$ | $e$ (Entity) | The current evaluation world — shifts under world-quantification (indexical, analogous to `this` in OOP) |
| $r_{\text{satisfies}}$ | $r$ (Relation) | Satisfaction relation: "world $w$ makes assertion $a$ true" |
| $r_{\text{alethic}}$ | $r$ (Relation) | Alethic accessibility: reflexive, transitive, symmetric (S5 frame) |
| $r_{K,\alpha}$ | $r$ (Relation) | Epistemic accessibility for agent $\alpha$: reflexive, transitive (S4 frame) |
| $r_O$ | $r$ (Relation) | Deontic accessibility: serial (KD frame) — at least one ideal world exists |
| $r_{\text{ability},\alpha}$ | $r$ (Relation) | Dynamic/ability accessibility for agent $\alpha$ |
| $r_{\text{closeness}}$ | $r$ (Relation) | World-closeness metric for counterfactuals (Lewis similarity) |

**Possible worlds.** A **possible world** is an entity $w \in G_e$ such that for any assertion $a$,
either `predicate(w, r_satisfies, embed(a))` or `predicate(w, r_satisfies, embed(negate(a)))` holds.
**Geometric realization:** world-enclosures are drawn with **double borders** (⫙); accessibility
relations are directed lines between world-enclosures, labeled by modal flavor.

**Defined operator: Necessity ($\square_R$).**

$$\text{necessary}(r_R, a) \;\overset{\text{def}}{=}\; \text{bind}\!\Big(w, \; \text{quantify}\!\big(m_\forall, \; w, \; \text{disjoin}\!\big(\text{negate}(\text{predicate}(w_{\text{current}}, r_R, w)), \; \text{predicate}(w, r_{\text{satisfies}}, \text{embed}(a))\big)\big)\!\Big)$$

Read: "For all worlds $w$: if $w$ is $R$-accessible from the current world, then $w$ satisfies $a$."
This matches the standard Kripke truth condition for necessity under accessibility relation $R$.

**Defined operator: Possibility ($\lozenge_R$).** Derived via De Morgan duality:
$\text{possible}(r_R, a) \overset{\text{def}}{=} \text{negate}(\text{necessary}(r_R, \text{negate}(a)))$

**Defined operator: Counterfactual.** Following Lewis (1973), necessity over closest
antecedent-satisfying worlds: $\text{counterfactual}(a, b) \overset{\text{def}}{=} \text{necessary}(r_{\text{closest}}(a), b)$,
where $r_{\text{closest}}(a) \overset{\text{def}}{=} \text{modify\_relation}(\text{abstract}(\text{embed}(a)), r_{\text{closeness}})$.

**K/T/4 axioms** hold under the standard conditions (K by construction; T when $r_R$ reflexive;
4 when $r_R$ transitive) — these are properties of the Kripke-semantics encoding, not additional
claims about the notation's necessity.

### 6.2 Performative Extension (Illocutionary Force)

An assertion is extended to a quadruple $a = (F, C, \sigma, \varphi)$ where $F$ = frame, $C$ =
content, $\sigma \in \{\oplus, \ominus\}$ = assertional sign, and $\varphi \in \Phi$ = illocutionary
force, ranging over:

| Force ($\varphi$) | Speech act type | Example | Frame decoration |
|---|---|---|---|
| $\varphi_{\text{assert}}$ | Assertive | "It is raining" | Solid border (default) |
| $\varphi_{\text{query}}$ | Interrogative | "Is it raining?" | Gapped border (open side) |
| $\varphi_{\text{direct}}$ | Directive | "Close the door" | Arrow-tipped border (→) |
| $\varphi_{\text{commit}}$ | Commissive | "I promise to return" | Arrow-tipped border (←) pointing inward |
| $\varphi_{\text{express}}$ | Expressive | "I apologize for the delay" | Wavy border (~~~) |
| $\varphi_{\text{declare}}$ | Declarative | "I pronounce you married" | Bold double border |

Default $\varphi = \varphi_{\text{assert}}$; all pre-existing assertions remain valid.

**Distinguished elements:** $e_{\text{speaker}}$, $e_{\text{hearer}}$ (both sort $e$).

**Force-operation interaction rules:** negate flips $\sigma$, preserves $\varphi$ (FC1); conjoin
collapses matching force or produces set-valued force (FC2); embed preserves force as metadata
(FC3); modal operators act on content, are force-transparent (FC4).

### 6.3 Pragmatic Inference Interface

A finite, rule-based interface between the notation and pragmatic reasoning, kept **outside** the
core signature (§1.5) as a separate inference layer operating on notation expressions:

**Scalar implicature rules** (defeasible): SI-1 (asserting a weaker quantifier implicates the
stronger one doesn't hold), SI-2 (asserting a disjunction implicates exclusivity), SI-3 (asserting
possibility implicates non-necessity).

**Conventionalized inference patterns:** CI-1 (indirect request: "Can you X?" → directive X),
CI-2 (indirect offer), CI-3 (rhetorical question → assertion).

These rules let the notation represent conventional indirect speech acts (e.g. "Can you pass the
salt?") as a bridge between a literal-query surface form and a directive intended form, without
adding operations to the core signature. Sarcasm/irony can be represented via a
belief/assertion-mismatch-plus-reflexive-intention structure using only existing operators
(`conjoin`, `negate`, `predicate`, `embed`, `bind`, plus the modal operators in §6.1) — *representable*
is the claim here; *which* reading a hearer should select from context is a parsing problem outside
this notation's scope, exactly as with any structural ambiguity (e.g. quantifier scope).

---

## Where the retired proofs and their reasoning now live

- Full original text: `archive/superseded-2026-08/foundations/formal-foundations-FULL-ORIGINAL.md`
- Why they were retired, with citations: `docs/planning/audits/wiki-comparison-2026-08.md` §5
- What replaces the necessity argument going forward: `docs/planning/emergence-investigation/PLAN.md`
