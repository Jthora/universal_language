# 051 — Semantic stack, cycle 1: the operational substrate exists and it is our object

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** first cycle against `050`'s program; predictions P1–P3 preregistered there
**Question:** do the three semantic routes have real formal footing, or is the stack a hope?

**Result: three predictions, three hits — and P1's hit is the largest single convergence since the
seven-line convergence on the combinatorial map.**

---

## 1. Searches and reads

| # | Target | Outcome |
|---|---|---|
| 1 | interaction nets / combinators (P1) | hit; **escalated to primary** — Lafont 1990 is paywalled (403), read Gay 1991 (the implementation thesis, *"essentially the same as the notation described by Lafont"*), §1 in full |
| 2 | institutions, satisfaction condition (P2) | hit, prediction confirmed as stated |
| 3 | Goodman selection problem + Harnad grounding (P3) | hit, **plus an unpredicted gift from the adversary's side** |

---

## 2. P1 — Interaction nets: a Turing-universal formalism whose programs are rotation systems

### 2a. The adversarial check resolved first (S11)

The named trap: ports might be *numbered labels*, not order. **The primary settles it:**

> *"Each agent (vertex) in a net is an instance of some symbol… An agent has a number of **ports**
> to which edges can be attached."* Rules are **local**: *"One port of each agent is identified as
> the **principal port**, and two agents can only interact if they are connected by their principal
> ports."*
>
> *"In a symbol definition, the first port listed is the principal port… **By convention, the rest
> of the ports are listed in the order obtained by moving anticlockwise round the agent.**"*

**The textual syntax is a serialization of a geometric arrangement around the vertex.** An
interaction-net agent is a **rotation with a marked dart** — cyclic order of ports, one
distinguished. `map.rs` represents rotations; the marked dart is one additional bit per vertex.

**And the order is load-bearing:** `Add(u,y,x)` and `Add(u,x,y)` are different programs — edges
attach to ports *by position in the rotation*, and rules pattern-match on them.

### 2b. What this is, stated with the discipline F-027 demands

- **It is NOT "meaning is carried by rotation alone."** Agents carry symbol labels (`0`, `S`,
  `Add`) and typed, directed ports. Labels do real work.
- **It IS: at least one Turing-universal operational formalism carries load-bearing rotation
  structure at every vertex** — and its minimal form, Lafont's interaction combinators, compresses
  the label alphabet to **three symbols** while keeping the rotation. *(Universality is reported
  from summaries — Lafont 1997, Salikhmetov 2017 — **S8 debt, explicitly**: the combinators paper
  has not been fetched.)*
- **The convention ledger entry, measured:** 3 symbols + rotation + one marked dart per vertex,
  Turing-universal. Compare an arbitrary alphabet's n!. **That is `ROTATION-MINIMIZES-CONVENTION`'s
  trade, instantiated by an unrelated field as its *minimal universal instruction set*.**

### 2c. What it corrects, and what it hands the program

**`SEMANTIC-FORMALISMS-USE-LABELS-NOT-ORDER` needs a material scope amendment.** The `046` sweep
checked six vocabularies — all from geometry/topology and NL semantics. **PL theory was a seventh
vocabulary nobody named**, and Girard's own term for the relevant structure is literally *"geometry
of interaction."* In the **operational** register the negative flips: order is load-bearing there,
alongside a compressible label residue. The **denotational** negative stands as checked.

**And it lands directly on UQPL's open problems.** The spec's problem 1 (prove Turing-completeness)
and problem 4 (optimal reduction): interaction nets are the standard substrate for **optimal
λ-reduction** (Lamping; Asperti–Guerrini), with *"strong confluence ensuring that non-overlapping
reductions lead to unique normal forms"* and native parallelism. **UQPL wants a typed λ-calculus
over geometric structure; the field's optimal λ-evaluator already runs on our fixed-point object.**

> **M2 is not a hope. The operational substrate exists, is Turing-universal, is strongly confluent,
> and its programs are rotation systems with labels compressed to a constant.** The bridge that
> failed in `034` failed in the *denotational* direction. The operational direction was never
> tried, and it is paved.

---

## 3. P2 — Institutions: the obligation, stated as mathematics

Confirmed as predicted. Goguen–Burstall: an institution's *"single defining axiom is the
**satisfaction condition** — truth is invariant under change of notation"*:

> **Mod(σ)(M′) ⊨_Σ ρ ⟺ M′ ⊨_Σ′ Sen(σ)(ρ)**

**It is definitional, exactly as preregistered** — a framework a logic must *satisfy*, not a
theorem that any given language does. So its value here is precise and limited: **it is the formal
shape of what READING-INVARIANCE must prove.** Signature morphisms are the "changes of notation";
our reading class 𝔽 must become a category of such morphisms; the theorem target becomes: *the
fixed-point semantics satisfies the satisfaction condition over all of 𝔽.* The obligation now has
a name, a category, and forty years of machinery — folded into `READING-INVARIANCE-TARGET`.

---

## 4. P3 — Goodman confirmed sharp; Harnad arrives as an ally

**The selection problem is real and is the named obstruction to M1:** a sample exemplifies *"only
those [features] for which it is a symbol — such as predicates denoting color and texture, and not
predicates denoting size or shape."* Which properties? — that selection is where convention
re-enters. **Our candidate answer stands registered as `ERLANGEN-ANSWERS-GOODMAN-SELECTION`
(CONJECTURED):** the filtration selects canonically — what survives coarse-graining *is* what is
referred to — and its fate is explicitly **tied to READING-INVARIANCE**, because "the invariants
survive" only forces selection if survival is reading-independent.

**The unpredicted part: the field's canonical statement of the grounding problem proposes our
stack's shape as its own solution.** Harnad (1990): symbols must be *"grounded bottom-up in
nonsymbolic representations of two kinds: (1) **iconic representations**, which are analogs of the
proximal sensory projections… and (2) **categorical representations**."*

> **Icon-first, symbol-derived — from cognitive science, with no contact with notation theory.**
> That is S1 rank-(3) convergence on the stack's architecture, from the direction that was named as
> an adversary. Quine and Kripkenstein remain for `052`, as preregistered.

**Sweep debt (S9), recorded:** the "no published canonical-selection solution" cell is **not yet
claimable as empty** — exemplification and symbol-grounding vocabularies were searched; Grice's
*natural meaning* vocabulary was not. One more vocabulary before the cell is called.

---

## 5. The 18-box checklist, run as a step

R1 ✅ (S11 trap named and resolved by primary) · R2 ✅ · R3 ✅ (what flips is the *operational*
register, not the checked denotational negative) · R4 ✅ (revival conditions in claims) · R6 ✅
(steelman of the negative: labels still work in inets — kept in the claim) · R8 ✅ (P1–P3 were my
proposals; each searched) · T11 ✅ (UQPL-relevance of optimal reduction is *inference*, marked) ·
T12 ✅ (stack registered with falsifier) · R9 ✅ · **S8 ⚠️ two debts, both explicit** (Lafont 1997
universality; Goodman primary) · S9 ⚠️ one vocabulary open (Grice) · S11 ✅ · `check.rb` ✅ · T5 ✅
(reading-invariance routed to proof, not experiment) · T6 ✅ (the *positive* P1 got the primary
read this time — the asymmetry ran the right way) · T9 ✅ · T10 n/a · S7 ✅ (counter-evidence on
P1: searched for, the trap was the counter-case, resolved against it by primary).

## 6. What changed

- `claims.yaml`: `UL-MUST-BE-A-LANGUAGE`, `UNIVERSALITY-IS-A-LEDGER` (the owner's contract,
  REQ-1–3); `READING-INVARIANCE-TARGET` (CONJECTURED, priority 0 — the program's theorem);
  `SEMANTIC-STACK-M1-M2-M3` (CONJECTURED, with falsifier); `INTERACTION-NETS-CARRY-ROTATION`
  (ARGUED — definition from primary, universality S8-flagged); `ERLANGEN-ANSWERS-GOODMAN-SELECTION`
  (CONJECTURED, dependent on reading-invariance); `SEMANTIC-FORMALISMS-USE-LABELS-NOT-ORDER`
  scope-amended (MATERIAL — operational register).
- `044`: Volley 5 closed as absorbed into `050`.
- `STATE-OF-PLAY.md`: rewritten fresh against the program.
- **Open:** Lafont 1997 primary (S8 debt); Grice vocabulary (S9 debt); `052` = the adversary front
  (Quine, Kripkenstein — scope-checked on contact); the reading-class 𝔽 formalization, which is now
  the mathematical center of the entire project.
