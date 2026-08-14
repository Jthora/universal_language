# 020 — Phase 1 · A3: is the notation's logic consistent with its constructive framing?

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `016` (constructive turn), `018` (plan, item A3)
**Question:** `016` established the notation lives in the **constructive** framework. Constructive
mathematics uses **intuitionistic** logic. Does anything the notation claims contradict that?

A coherence check between two commitments arrived at independently. If they disagree, one is wrong.

---

## 1. Before searching  ← written first (S2)

**The specific suspicion:** `NEG-INVOLUTION` claims `negate(negate(a)) ≡ a`. **That is double-negation
elimination, which is exactly the principle intuitionistic logic rejects.** Intuitionistically
`A → ¬¬A` holds and `¬¬A → A` does not.

**So I predict a genuine inconsistency:** if the notation is constructive, `NEG-INVOLUTION` is not
merely unproven — **it should be false.** And if it is true, the notation is classical, and `016`'s
constructive framing is wrong. Both cannot stand.

**Second prediction, and it inverts an existing entry.** F-006 recorded that the implementation
wraps negation in a fresh enclosure so double negation is structurally *not* identity, and filed
that as a **bug** — code failing to match the documented law. **Under an intuitionistic reading the
implementation may be correct and the documentation wrong.**

**Expected caveat I should check rather than assume:** constructive systems sometimes recover
classical principles for **decidable** predicates. If notation-level negation is decidable, the
involution might hold legitimately in a restricted fragment. I do not want to declare an
inconsistency that a decidability condition dissolves.

**Would change the plan if:** constructive geometry turns out not to require intuitionistic logic,
in which case the coherence check has no teeth and A3 closes as uninformative.

**Objects mathematical?** Yes. This is a question about which principles hold in which logic —
find the theorem, do not design a study (T5).

---

## 2. Searches run

| Query | Direction | Result |
|---|---|---|
| Constructive geometry, intuitionistic logic, decidable equality, apartness | supporting | Confirms the framing; **surfaces apartness** |
| Double-negation elimination, when classical principles hold in restricted fragments | **adversarial (R1)** — testing my own suspicion | Caveat is real but narrow |

**Counter-evidence: easy to find.** (S7)

## 3. Findings

### 3.1 The inconsistency is real

> *"Heyting's calculus is a restriction of classical logic in which the law of excluded middle
> **and double negation elimination** have been removed."*
>
> **Decisive:** *"If the intuitionistic law of negation introduction is replaced by the law of
> excluded middle **or double negation elimination**, a formal system for **classical**
> propositional or predicate logic results."*

`NEG-INVOLUTION` asserts `negate(negate(a)) ≡ a` — double-negation elimination, **as a universal
law**. Adopting it does not merely fail to be constructive: **it converts the system to classical
logic.**

**So the two commitments cannot both stand.** Either the notation is constructive (`016`) and this
law is wrong, or the law holds and the constructive framing is wrong.

### 3.2 The decidability escape does not apply — and I checked before relying on that

I flagged in §1 that constructive systems can recover classical principles for *decidable*
propositions, and that I should not declare an inconsistency a decidability condition would dissolve.

The caveat is real: *"Excluded middle and double negation elimination can still be proved for some
propositions **on a case by case basis**, but do not hold universally."* Case-by-case is not a law
over all assertions, which is what `NEG-INVOLUTION` claims.

And in **constructive geometry specifically the escape is deliberately closed**:

> *"Some approaches to constructive geometry **do not adopt decidable equality**, nor even the
> substitute concept of 'apartness'… primarily because they aim to develop systems in which definable
> terms (constructions) **denote continuous functions**."*

Decidable equality is given up *on purpose*, to keep constructions continuous. **The escape hatch is
closed in exactly the framework `016` placed the notation in.**

### 3.3 F-006 inverts — the implementation may be right and the specification wrong

F-006 recorded that `negate` wraps its argument in a fresh enclosure, so double negation is
structurally not identity, and filed this as a **bug**: code failing to match a documented law.

**Under the constructive reading the implementation is behaving correctly and the documented law is
the error.** `A → ¬¬A` holds; `¬¬A → A` does not. A negation that is *not* an involution is what an
intuitionistic system should have.

This does not make the implementation *right* — the self-loop marker is still an ad-hoc encoding
(`NEGATE-REIMPLEMENTATION`). But **the failing test was testing the wrong thing.**

### 3.4 The payoff — apartness, and it may unblock `SEMANTIC-EQUALITY`

> *"In constructive mathematics, an **apartness relation** is a constructive form of inequality, and
> is **often taken to be more basic than equality**."* The negation of equality — *"denial
> inequality"* — is **weaker**.

**`SEMANTIC-EQUALITY` has been the project's longest-standing blocker**: it does not exist in any
form, and `RICE-BOUNDS-SEMANTIC-EQUALITY` shows it is undecidable in general.

**Constructively, that is the wrong primitive to reach for.** Apartness — a *positive* demonstration
that two things are definitely different — is the basic notion, and it can be establishable where
equality is not decidable.

**And it is the right notion for the Cure.** Drift detection does not require proving two states are
equal; it requires detecting that they have **separated**. That is apartness, stated positively.

Two further resonances, flagged as suggestive rather than established: apartness is *formalized
distinguishability*, which is Layer 0's distinction; and Jordan separation — the topological
survivor from `014` — is literally a separation relation.

**Status: a lead, not a result.** What makes it more than a resonance is that it is concrete —
specify a semantic apartness relation and check whether it is decidable on the acyclic core where
equality is not.

## 4. What changed

- `claims.yaml`: `NEG-INVOLUTION` → RETIRED as a universal law; `NOTATION-LOGIC-IS-INTUITIONISTIC`
  added as the DESIGN-CHOICE the fork forces; `SEMANTIC-APARTNESS` added as the constructive route.
- `FAILURES.md`: F-022 — F-006 recorded a specification error as an implementation bug.
- **Phase 1 A3: complete**, and it produced a live engineering lead rather than a bookkeeping
  result.
