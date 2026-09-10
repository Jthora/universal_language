# 061 — The S8/S9 debt settlement: five reads, and the record's pattern holds

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** pays the primary-read debts flagged across `051`–`059`, in the danger order. Runs
concurrently with the `060` cold-reader baseline.
**Question:** do the load-bearing summaries survive their primaries?

**Score: five debts settled — one claim strengthened to its strongest form, two of our own
positives honestly weakened, one objection sharpened, one sweep closed.** The record's rule
("a primary read changes something, every time") holds at 13-for-13.

---

## 1. Lafont 1997 — the dangerous one, and it came back *stronger* (↑)

Read in the original (Information and Computation 137). The universality claim the engine leaned
on from summaries is real and **stronger than summarized**:

> *"a very simple system of interaction combinators, with only three symbols and six rules, is a
> universal model of distributed computation"* — where universal means *"any other interaction
> system can be translated into it"* preserving *"the complexity and the degree of parallelism"* —
> and *"Turing machines… are intrinsically sequential and **cannot** be universal in the above
> sense."*

So the substrate is not merely Turing-equivalent (*"from the viewpoint of computability, our
interaction nets are equivalent to the Turing machines"*) — its universality notion is *stricter*.
And the port-order fact is inventor-confirmed: auxiliary ports *"are **not interchangeable**…
implicitly numbered in **clockwise** order"* — where Gay 1991 wrote *anticlockwise*. **The two
primary sources use opposite chiralities: the mirror convention ℤ/2, live in the literature's own
bookkeeping.** Bonus for Module Q: *"the fundamental laws of computation are commutation and
annihilation."*

**Registry:** `INTERACTION-NETS-CARRY-ROTATION` and `M2-ENGINE-V1-EXISTS` — universality debts
cleared, rank (2).

## 2. PRH limitations — our own positive, honestly weakened (↓)

The authors' own section is sharper than any critique I had written: the convergence argument
*"only strictly holds for bijective projections"* (lossy/stochastic observation breaks it);
vision-language alignment *"only reaches a score of 0.16"* with the authors themselves asking
whether that signifies strong or poor alignment; special-purpose systems may not converge; and a
**sociological-bias alternative** — convergence toward human-like representations *because the
field aims at them*, not because reality forces it.

**Registry:** `ALIGNMENT-IS-FORMAT-ACQUISITION`'s PRH support downgraded from
"documented convergence" to "hypothesis with acknowledged counterexamples and a deflationary
alternative." T6 satisfied in the direction that costs us.

## 3. Li & Gleitman 2002 — the mechanism's flagship case is contested (↓)

Primary, from the abstract: the studies *"reproduce these different problem-solving strategies in
speakers of a single language (English) by manipulating landmark cues, suggesting that language
itself may not be the key causal factor in choice of spatial perspective"* — a cue-availability
account covering infants and lab animals too. Levinson, Kita, Haun & Rasch replied the same year
(*"Returning the tables: language affects spatial reasoning"* — **unread; new debt, recorded**).

**Registry:** the Levinson-type mechanism is now marked **contested at primary level, both sides
in Cognition 2002**. The claim stays CONJECTURED; its named falsifier is partially live; and the
**format-acquisition experiment rises in priority** — we cannot lean on this literature as
settled, so we must run our own.

## 4. Goodman via SEP — the objection sharpened, and it points home (→)

*"Which of its properties does a sample exemplify depends on the **system within which the sample
is being used**."* The selector is a *use-system* — which both sharpens the objection to M1 (the
selector is conventional practice, exactly what we must not smuggle) **and** converges with where
`059` already landed: the use-theoretic turn. Our move, restated at its crispest: **make the
use-system itself the derivable one.** Same normativity gap, same eligibility-from-invariance
target; nothing new to dodge, one more converging description of the same summit.

## 5. Grice — the sweep closes, with a terminology gift (✓)

Natural meaning (*smoke means fire*: causal, factive — "x means-naturally p" entails p) versus
nonnatural meaning (intentional/conventional). **The S9 sweep across the three promised
vocabularies — exemplification, symbol grounding, natural meaning — found no prior
filtration-based solution to the selection problem**; that cell can now be called as swept. And
the mapping is clean enough to record: **Grice's natural/nonnatural distinction is the
philosophical ancestor of the natural/formal tier split** — M3's indexical route lives exactly
where natural meaning does (causal, factive, convention-free), and the formal tier is where
nonnatural meaning begins.

## 6. What changed

- `claims.yaml`: universality debts cleared on the two engine claims (↑); PRH and Levinson
  evidence downgraded on `ALIGNMENT-IS-FORMAT-ACQUISITION` (↓↓) with the Levinson-reply debt
  newly recorded; Goodman primary + closed S9 sweep on `ERLANGEN-ANSWERS-GOODMAN-SELECTION`;
  Grice mapping recorded on `SEMANTIC-STACK-M1-M2-M3` and `UL-IS-TWO-TIER`.
- **Score integrity note:** two of five reads weakened our own positives. The batch was aimed at
  exactly the claims we most wanted to keep — which is what T6 demands and what the F-033 review
  promised.
- **Open:** Levinson et al. 2002 reply (new S8 debt, small); the format-acquisition experiment,
  now carrying more of the thesis's weight.
