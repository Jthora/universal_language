# 062 — The Church–Turing template: is thesis-by-convergence a legitimate endpoint for UL?

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** opened at the owner's prompt (Zipf / PL-fundamentals / UG). This is the PL thread, run
first as the safest and most consequential. Follows `059`, which made the theorem route harder.
**Question:** computation had many independent formalizations that turned out to be one object.
Is that a real model for UL's endpoint — or am I reaching for it because `059` bruised the theorem
route?

---

## 1. The self-aimed prediction, written first

**The suspicion, stated before searching (R8, T4):** `059` showed `READING-INVARIANCE-TARGET` runs
into Putnam's just-more-theory objection. Within a day I proposed an alternative endpoint that
requires no theorem. **That is exactly the shape of a comfortable retreat, and REQ-3 forbids
rescoping-to-win.**

**What would show I am rationalizing** (preregistered, so it can catch me):
- if Church–Turing turns out to be *weaker* than a theorem in a way that would let any hopeful
  research program declare victory by collecting resemblances;
- if the convergence in computation rests on something UL structurally cannot have;
- if the "many formalisms, one object" story is retrospective tidying rather than real history.

**What would show it is legitimate:** if Church–Turing has an internal structure that separates a
**proved** part from an **assumed** part, and UL can be located precisely within that structure
rather than gesturing at the whole.

## 2. Predictions (S2)

**P1 — Church–Turing decomposes into theorems plus one thesis.** I expect the formalism
equivalences (λ-calculus ↔ Turing machines ↔ μ-recursive ↔ combinatory logic) to be **proved
theorems**, with only the identification of that class with the informal notion "effectively
calculable" being the thesis. If so, "thesis-by-convergence" is the wrong summary — it is
**theorem-by-convergence plus one bridging thesis**, and UL must be located in both halves
separately.

**P2 — the load-bearing disanalogy: an extensional target.** Computation's formalisms converge on
a set of *functions* — an object with an agreed extensional criterion, so "same" is checkable.
**Meaning has no such agreed extension.** I predict this is the real disanalogy and that it is
fatal to the naive version of the template.

**P3 — the syntactic half may already be in the theorem column.** Combinatorial map ↔ DCEL ↔
ribbon graph ↔ dessins ↔ rotation system are, to my knowledge, *proved* equivalent — not merely
similar. If so, **UL's syntactic object already has the theorem-part**, and what is missing is the
bridging thesis, exactly parallel to Church–Turing's structure.

**P4 — someone has tried to prove Church's thesis.** I expect axiomatization attempts
(Dershowitz–Gurevich is the name I associate). If a bridging thesis can be *derived from axioms
about the informal notion*, then the thesis route is not a retreat from theorem — it is a staging
post, and the corresponding move for UL would be to axiomatize "reading" rather than characterize
"reasonable."

**P5 — name elimination is the ledger argument with a track record.** De Bruijn indices and
combinatory logic remove variable *names* by structural position. I expect this to be exactly
`ROTATION-MINIMIZES-CONVENTION`'s trade, in PL, with decades of engineering.

**Stop condition:** four searches, two primaries. **Adversarial search is mandatory on the
template itself** (R1), not only on its parts.

## 3. Searches run

| # | Target | Outcome |
|---|---|---|
| 1 | Church–Turing: what is proved vs assumed | **P1 confirmed** — the decomposition is real |
| 2 | Dershowitz–Gurevich, proof of Church's Thesis | **P4 confirmed** — and it carries a sting |
| 3 | de Bruijn / combinatory logic, name elimination | **P5 confirmed** |
| 4 | **adversarial, aimed at the template itself** (R1) | **the disanalogy found, and it is our own obstruction** |

**S8 status, stated plainly:** the Dershowitz–Gurevich paper **failed to fetch** (TLS certificate
error) and the Shapiro critique was reached only through search summaries. **This cycle's central
finding rests on rank-(5) evidence and is registered ARGUED, not VERIFIED**, with the primary
recorded as an outstanding debt. Search 1's decomposition is standard textbook material I can
also check against my own knowledge; searches 2–4 cannot be checked that way.

## 4. Findings

### 4a. The template is real, and it is two things, not one (P1 ✓)

*"The equivalence of formal models is proven, but the identification of this equivalence with
intuitive computability is assumed."* Turing himself proved the Turing-machine/λ-calculus
equivalence. **So "thesis-by-convergence" was the wrong summary and my own framing was sloppy:**

> **Church–Turing = (a) proved equivalences among formalisms, over an object with an extensional
> criterion + (b) one bridging thesis identifying that formal class with an informal notion.**

UL must be located in each half **separately**, and that is the whole value of the exercise.

### 4b. Where UL actually sits — and the sharpest statement yet of the semantic gap

| | Extensional criterion? | Equivalences proved? |
|---|---|---|
| **Computation** | yes — a *set of functions* | yes — Turing, Church, Kleene |
| **UL syntax** (the map) | **yes** — map isomorphism | **yes, and not by us**: rotation system ↔ DCEL ↔ ribbon graph ↔ dessins are standard results (`046`, `MAP-IS-A-DCEL-REDUCT`) |
| **UL semantics** | **no** | n/a |

> **The semantic gap is not "we have not built the bridge." It is that the far side has no agreed
> extension.** There is no extensional criterion for *meaning* against which two formalisms could
> be proved to agree.

And that is not our failure — **it is Quine's conclusion** (`059`): there is no fact of the matter
about reference. **The same finding, arrived at from computability instead of from philosophy.**

### 4c. The self-aimed prediction: the template is legitimate AND it does not rescue me

**P4 confirmed: someone did prove Church's Thesis** — Dershowitz & Gurevich (2008, *BSL*) derive
it from "Sequential Postulates" via the Abstract State Machine theorem. So a bridging thesis *can*
be promoted toward theorem, and the thesis route is a staging post rather than a dead end.

**But the adversarial search found the sting, and it is our own obstruction wearing a third
costume.** Shapiro's objection, extending Waismann: pre-theoretical concepts have **open texture** —
a sharpening may *replace* the original concept rather than capture it — and *"informal notions are
too rich for their meanings to be settled by any single finite expression."* D–G's own
justification for their first-order framing is an appeal to **the track record of mathematical
practice**, not a proof. (Their result is also scoped: it excludes randomness, parallelism and
quantum computing.)

> **"You cannot axiomatize the informal notion without smuggling or replacing it" is Putnam's
> just-more-theory objection and Kripkenstein's finitude problem, in the philosophy of
> computability.** Three independent literatures, one obstruction. `READING-INVARIANCE-TARGET`'s
> deep failure mode is not a quirk of our program — **it is where every discipline that has tried
> to bridge formal structure to an informal notion has stopped.**

**Verdict on the self-suspicion:** the template is legitimate and I was **not** rescued by it.
It gives a precise location and a precise obstruction; it licenses no victory-by-resemblance,
because the part we would be collecting resemblances *toward* is the contested part in computation
too. **What it does buy is status:** ninety years of computer science resting on an unproved
bridging thesis shows that such a thesis, with strong convergence evidence, is a **respectable
scientific endpoint rather than a failure.** That reframes the ambition without lowering the bar.

### 4d. The unlooked-for payoff: our oldest blocker has a standard answer

`SEMANTIC-EQUALITY` — *"a decision procedure for whether two structures mean the same thing"* — has
sat as the project's longest-standing blocker, recorded as not existing in any form.

**If meaning has no extensional criterion but *use* does — behavior is observable — then the
use-theoretic route is the one that HAS an extension.** And PL theory built that criterion decades
ago: **contextual (observational) equivalence** — two programs are equivalent iff no context
distinguishes them. That is `semantically_equal`, in its honest form, for the M2 layer.

**With the catch stated up front: contextual equivalence is undecidable in general**, so the
engineering path is the standard one — bisimulation, logical relations, and other sound
approximations. **That is a large upgrade from "does not exist in any form" to "exists, is
standard, is undecidable, and has known approximations."**

### 4e. Unpreregistered: this cycle fired one of our own falsifiers

While registering 4d I read `SEMANTIC-EQUALITY`'s falsifier — *"a proof that GIR semantic
equivalence is undecidable in the general case"* — and realized **this cycle supplies it**:

1. contextual equivalence is **undecidable** for Turing-complete languages;
2. our M2 engine is **Turing-universal** (Lafont 1997, primary-read in `061`);
3. `059` committed the project to **use-theoretic** semantics.

**Those three together are the proof its falsifier named.** The claim has stood since before the
rebuild as the project's longest-running blocker, and it has now been **retired in its general
form** — not by an outside objection, but by our own research reaching the conclusion the claim
itself nominated in advance.

**Retired per R3 — the formalization, not the question.** Replaced by `EQUALITY-IS-TWO-LAYERED`:
**structural equality (map isomorphism) is decidable** and our shipped invariant signature is a
sound-but-incomplete approximation of it; **operational equality is undecidable** and has known
sound approximations (bisimulation, logical relations, e-graphs). **Revival condition recorded**
(R4): restrict the operational layer to a *total* fragment and decidability returns — at an
expressiveness cost nobody has measured.

**And the architecture was already right where the claim was wrong.** The comparator we built
works on the decidable layer. We had been holding a blocker for a procedure that provably cannot
exist, while shipping the thing that can.

### 4f. Name elimination confirmed (P5 ✓)

De Bruijn indices give α-equivalence *"for free — two nameless expressions are α-equivalent if
they are syntactically identical."* Schönfinkel's combinators eliminate variables entirely.
**Removing names makes equivalence structural** — `ROTATION-MINIMIZES-CONVENTION`'s trade,
independently made in PL, with the payoff stated as a checkable property rather than an argument.

## 5. The 18-box checklist

R1 ✅ (search 4 aimed at the template itself, and it hurt) · R2 ✅ · R3 ✅ (what is withdrawn is my
"thesis-by-convergence" framing, not the template) · R4 ✅ (revival: the bridging move returns to
theorem status if open texture is answered) · R6 ✅ (steelman: D–G shows the bridge is provable
*from postulates* — recorded as the strongest form) · R8 ✅ **(the self-suspicion was preregistered
and tested; verdict recorded against my comfort)** · T4 ✅ · T8 ✅ (the template's scope conditions
are the finding) · T11 ✅ (the three-literature convergence on one obstruction is my inference,
marked) · **S8 ⚠ the central source failed to fetch — flagged in §3, claim tiered ARGUED
accordingly** · S11 ✅ · `check.rb` ✅ · S7 ✅ (counter-evidence to the template was one query away
and was run before registering).

## 6. What changed

- `claims.yaml`: `CHURCH-TURING-TEMPLATE` (ARGUED — the decomposition, UL's location in it, and
  the explicit refusal of victory-by-resemblance); **`SEMANTIC-EQUALITY` RETIRED** in its general
  form with full R3/R4/R5 contract, its own falsifier having fired; **`EQUALITY-IS-TWO-LAYERED`**
  added (ARGUED, priority 0) as its successor.
- **Open:** the D–G primary (S8 debt); whether contextual equivalence over the M2 engine is
  implementable as a useful approximation — **the most concrete next build this cycle produced**;
  and the Zipf and UG threads, still unrun.
