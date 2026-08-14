# 045 — Volley 1: the graveyards we never surveyed

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Superseded by:** `048`  ← *the one permitted edit to a closed note (`../README.md`)* — the protocol audit; see it before relying on this note's evidence grades
**Thread:** executes Volley 1 of `044`
**Question:** who has already failed at the Cure, at UP, and at deriving a notation?

`037` ran this survey for *universal languages* and it outperformed every crawl before it. **The
Cure, UP and derived-notation never got one.** Four queries, hard stop.

---

## 1. Before searching  ← written first (S2)

**The Cure — ontology maintenance and semantic drift.** I expect the graveyard to be **populated**:
large ontologies (SNOMED, Gene Ontology) carry documented maintenance burdens, terms drift, schema
evolution breaks dependents.

**But I expect the documented cause to be *economic*, not *technical*** — maintenance cost exceeding
value, rather than drift being undetectable. **If that is right it reframes the Cure**: the problem
would not be detection but the cost of acting on what is detected, and our entire architecture
addresses the wrong half.

**UP — first contact.** I expect the assumption that mathematics is universal to be contested, plus
documented anthropocentrism critiques of Arecibo/Pioneer. I do **not** expect anyone to have solved
minimal-convention encoding.

**UWS-as-derived.** I expect **nothing** — notations are designed. A null here *confirms* `038`'s
claim that the notation × derived cell is empty, so the null is the informative outcome. **If
something exists, it is the most important find of the volley** and `038`'s "no precedent" claim
needs immediate revision.

**Would change the plan if:** the ontology failures are technical rather than economic *and* match
our proposed mechanism — that would mean the Cure's approach is already known not to work.

**Watch (T6):** three of four expectations are comfortable. The uncomfortable one — that the Cure's
real problem is economic — is the one I should scrutinize *least* hard if it appears, and hardest if
it does not.

## 2. Searches run

Four, as budgeted. **Stop condition honoured.**

## 3. Findings

### 3.1 The Cure's graveyard is populated — and the dominant cause is **not** detection

**My uncomfortable preregistered prediction was correct.**

> *"Without a steward and regular reconciliation, ontology models become **stale within two
> quarters**"* — *"a **governance problem** that has defeated a generation of master-data programs."*
>
> *"Common failures are **cultural and organizational rather than technical**."*
>
> *"Once deployed, the ontology enters a maintenance phase that is, **in practice, no maintenance at
> all**."*
>
> Ontology drift is *"the **silent killer** of enterprise KG projects."* Only **27% of organizations**
> have knowledge graphs in production as of late 2025.

**What this costs us, stated plainly:** the Cure is a detection-and-repair architecture. **The
documented dominant failure is that nobody maintains the system at all.** A better detector does not
help if no one acts on it. **We have been building for a sub-problem that is not the main one.**

**What survives, and it is not nothing:** *"little visibility into drift"* is cited as a contributing
cause, and that is precisely what the Cure addresses.

**And it yields a hard requirement, now derived rather than preferred:**

> **The Cure must be fully automatic.** Any design requiring a human steward inherits the documented
> failure mode directly — *"in practice, no maintenance at all."* `CURE-IS-COMPARATOR` is compatible
> with this; a human-in-the-loop repair step would not be.

### 3.2 A scope warning aimed directly at this project

> *"Scope creep, where teams attempt to **model all of reality before deployment**, typically causes
> projects to fail **after a year**."*

**That is a description of this project.** Recorded without softening.

### 3.3 METI — and it hits UWS's iconicity from an unrelated field

> Arecibo assumed *"mathematics represents a truly universal language"* — *"whether this assumption
> proves correct **remains unknown**."*
>
> *"Drake assumed that pictures would be **self-evidently interpretable**, but we now know that our
> interpretation of pictures relies on **cultural assumptions** that extraterrestrials might never
> imagine."*
>
> The 2017 signal was designed *"to be interpretable by **life-forms that do not have eyes**."*

**Independent corroboration of F-012** from a field with no connection to notation research: pictures
are not self-evident. And **the field has deliberately moved away from visual encoding** — which
UWS is.

**Nobody has solved minimal-convention encoding.** *"The lack of an established protocol has produced
unorganized or cryptic messages."* **UP's problem is open, and open for everyone.**

### 3.4 Derived notation — the empty cell confirmed, plus prior art we did not know

**No derived notation found.** `038`'s claim that *notation × derived* has no precedent **holds**, and
the null is the informative result as predicted.

**But the search returned something we should have had:**

> **Babbage's ten guidelines for mathematical notation design:** *conciseness, simplicity, univocity,
> mnemonics, **iconicity**, analogy, modularity, generality, **symmetry of symbols**, and **symmetry
> of structure**.*

**Iconicity and symmetry of structure are UWS's two central commitments, listed as notation-design
desiderata in the 1830s.** This does not fill the empty cell — Babbage's are *design* guidelines,
kind A — but **the desiderata are not original, and we did not know that.**

*(Also: notation "often reflects underlying structure" — ∫ as a stretched S, Σ from "sum." Weak
iconicity is normal in mathematical notation, not a UWS innovation.)*

## 4. What changed

- `claims.yaml`: `CURE-MUST-BE-AUTOMATIC` added — a requirement derived from the failure record;
  `CURE-SCOPE` gains the governance finding; `UWS-TAUGHT-NOTATION` gains the METI corroboration.
- **Scorecard:** three of four predictions correct, including the uncomfortable one. The fourth (no
  derived notation) was correct *and* returned unexpected prior art — **the null was informative and
  the query returned more than the null.**
- **Open and now sharper:** the Cure addresses a contributing cause, not the dominant one. Whether
  that is worth building depends on whether full automation is achievable, which is now the
  question rather than an assumption.
