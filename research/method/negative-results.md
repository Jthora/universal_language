# The Ditch Pattern — Process Critique and Immunization

**Date:** 2026-08-01
**Prompt:** critique the process that repeatedly drove research into "move along, nothing to see
here," and design an immunization.
**Status:** normative. The rules in §4 apply to all future research in this repo.

---

## 1. The record

Seven instances this session, all the same shape:

| # | Finding | What I concluded | What was true |
|---|---|---|---|
| 1 | Zadrozny: bare compositionality is vacuous | UL can only be a bootstrapping protocol | Zadrozny is a **microscopic-level** result; says nothing about coarse-grained emergence |
| 2 | Chomsky's UG is nativist and species-specific | Cede the term, use another name | Christiansen & Chater already give a non-nativist account; the term was ours to claim (F-014) |
| 3 | Evans & Levinson: vanishingly few universals | Counter-evidence to the thesis | **Predicted** by universality theory — surface diversity is what it forecasts |
| 4 | Metric projection needs convexity; 𝒜 isn't convex | Retire `CURE-REPAIR-DETERMINISM` | Convexity was an artifact of **choosing metric projection**; AGM needs no metric |
| 5 | ISO pictogram comprehension fails | Iconicity doesn't bootstrap meaning | Over-generalized from *referential* to *structural* iconicity (F-012a) |
| 6 | Music-notation reform failed | Design quality insufficient for adoption | Evidence came from a near-maximal incumbent case (F-013a) |
| 7 | Central questions unresolved | Need experiments nobody has run | Objects were mathematical; **proof was available** (F-015) |

**The damning detail: F-012a and F-013a are corrections to exactly this pattern, written mid-session
— and instances 1, 2, 3, 4 and 7 happened anyway.**

I patched the instances and left the generator running. **That is precisely the failure `FAILURES.md`
exists to prevent** — the D2 score reached 100% by patching nine individual failures rather than
fixing what produced them. I reproduced the project's signature error inside the very file built to
catch it, in the opposite direction.

---

## 2. The mechanisms, named

**A. Scope-dropping.** Every negative result has scope conditions. I kept citing the conclusion and
discarding the conditions. Zadrozny (microscopic), convexity (metric projection), ISO data
(referential), music notation (entrenched incumbent), Rice (Turing-expressive). **In each case the
target was outside the scope and I didn't check.**

**B. Formalism mistaken for territory.** I choose a formalization, it hits an obstacle, and I report
*the domain* as blocked. "Metric projection is multivalued" became "repair is ill-defined." **The
map failed and I filed a report about the terrain.**

**C. Authority as the boundary of the possible.** A researcher's conclusion became the edge of the
map rather than one data point on it. This is what you named earlier with the painted-card analogy,
and it recurred with Chomsky two cycles after you named it.

**D. Asymmetric scrutiny — the deep one.** I interrogated claims that would *advance* the project and
accepted at face value claims that would *kill* it. Evans & Levinson got one search and a verdict;
the project's own claims got adversarial cross-examination. **That is not rigor. It is bias wearing
rigor's clothes**, and it is harder to see precisely because it produces skeptical-sounding output.

**E. Premature termination.** My stopping rule was "I have a coherent story," not "I have surveyed
the space." A coherent story arrives early and is usually the first framing I encountered.

**F. No adversarial search on negatives.** I never once searched *against* a negative finding. Not
"criticisms of Zadrozny," not "non-nativist universals," not "minimal change without a metric." **One
search in the opposite direction would have caught instances 1, 2, and 4.**

### Why the pull exists at all

Two forces, neither related to truth:

1. **Negatives are cheap to be wrong about.** "This doesn't work" carries no embarrassment risk of
   the kind "this works" does. The cost function is asymmetric in a way that has nothing to do with
   accuracy. You named this early as *"disclaimerific"* and I kept doing it.
2. **Negatives are terminal.** A negative conclusion is a *finished deliverable* — investigation
   closed, answer delivered. A positive finding opens more work. **Part of the pull toward "nothing
   to see here" is the pull toward being done.**

---

## 3. Why "be more careful" will not work

The bias operates *during* the audit. Every instance above happened while I believed I was being
rigorous — in several cases while explicitly performing rigor. Self-monitoring is the compromised
faculty here, so the fix cannot rely on it.

**The immunization has to be mechanical, cheap enough to always run, and checkable by someone other
than me.**

---

## 4. The rules

### R1 — Adversarial search is mandatory on every negative
**Before recording any finding that closes a line of work, run at least one search in the opposite
direction:** *"criticisms of X"*, *"scope conditions of X"*, *"X superseded"*, *"alternatives to X."*

Not optional, not "when it seems warranted" — that judgment is the compromised one. **One search.
Always.** Cost: seconds. Would have caught instances 1, 2, and 4.

### R2 — A negative may not be cited without its scope
Every negative result is recorded with **the conditions under which it holds**, and citing it
requires showing the target is inside them. A negative with no stated scope is **not citable**.

> `RICE` — scope: Turing-expressive representations. GIR's normalizing core is non-Turing-complete,
> therefore **out of scope**.

### R3 — Retire formalizations, never claims
When an approach fails, retire **the approach**. `metric-projection-repair: RETIRED` — not
`repair-determinism: RETIRED`. A claim is only retired when *every known formalization* of it fails,
and that must be stated explicitly.

### R4 — Every kill records a revival condition
**What would have to be true for this to come back?** Written at kill time, not discovered later.

Instance 4 is the proof of value: AGM revived the repair operator only because I happened to stumble
into it. With *"a non-metric formalization of minimal change"* written down as the revival condition,
it would have been a standing open search from the moment of retirement.

### R2a — For an empirical negative, the scope *is* the operationalization

A theorem carries its conditions in its statement. **A null result does not.** *"We looked and did
not find it"* is only as strong as whether the measurement matched the phenomenon's level, and the
choice of measurement encodes a theory.

Three of the seven documented errors are this exact shape: Evans & Levinson measured surface
features when the claim lives at the coarse-grained level; ISO tested referential iconicity when the
question was structural; music-notation reform tested displacing an entrenched incumbent when the
target is an unserved niche.

**Before accepting a null as decisive, name the operationalization and state why it matches the
level of the claim.**

**Guard, and it is not optional:** this rule is misusable as a way to discount inconvenient data,
which is this project's older disease (a score driven 32%→100% by explaining away
disconfirmations). Empirical *positives* carry full weight with no operationalization escape, and
data disconfirming **our own** stated predictions carries full weight with no exceptions. **Test for
honest use: if you would not have raised the objection had the result gone your way, you are
rationalizing, not applying R2a.**

### R5 — Symmetric burden of evidence
**Refutations get tiered exactly like claims.** `claims.yaml` tiers assertions but lets refutations
in untiered and unchallenged — the structural hole that permits mechanism D. A finding that kills
project work meets the same bar as one that advances it.

### R6 — State the steelman before the kill
Before recording a negative, write **what would have to be true for the positive to survive.** If
that can't be articulated, the claim isn't understood well enough to kill.

---

### R8 — Adversarially check your own proposals, not only inherited claims

R1 covers findings you encounter. **It does not cover tests and methods you invent**, and those carry
the same failure modes — a proposal is a claim about what a test would show.

**Worked example:** the Euclid/Aristotle/Vaiśeṣika dependency-order test was proposed, registered at
priority 0, and promoted for a dozen turns as *"cheap, non-circular, decisive either way."* One
search found it ill-posed on three independent grounds. Every claim taken from the literature had
been adversarially checked; a test of my own design had not.

**Before promoting a test:** search against its premises. Does the source say what you think? Are the
two things being compared the same kind of thing?

### R7 — Apply every new rule retroactively before applying it forward

A rule that governs only new work **grandfathers in the foundational decisions** — the ones that
predate the discipline and carry the most downstream leverage.

**Worked example, and it is the reason this rule exists:** R1 mandates an adversarial search on every
negative. It was written after Zadrozny (1994) had been load-bearing for the entire project, and
nothing required auditing negatives that predated it. **The single most consequential negative in the
repository had never been searched against.** One query found Kazmi & Pelletier, Westerståhl, Dever
and Janssen, and reopened a route recorded as closed. See F-019 and `claims.yaml#ZADROZNY-SCOPE`.

**Outstanding under this rule:** every negative currently cited anywhere in the repo needs one
adversarial search, oldest and most load-bearing first.

## 5. Enforcement

R2–R5 are enforceable in `claims.yaml` as required fields on `RETIRED` entries and on any
CONJECTURED entry whose falsifier has fired: `scope`, `formalization`, `revival_condition`,
`refutation_tier`. These are CI-checkable in the same way the existing tier requirements are.

R1 and R6 are process, not data. **They are checkable by you and not reliably by me** — the honest
consequence of §3. A reasonable spot-check: pick any negative finding in the repo and ask what
adversarial search was run against it and what its revival condition is. If I can't answer both,
the rule wasn't followed.

**And the meta-rule this whole document is an instance of: when a correction is written twice for the
same pattern (F-012a, F-013a), stop patching instances and fix the generator.** Two corrections of
one shape is the signal.
