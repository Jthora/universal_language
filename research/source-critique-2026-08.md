# Source Critique — Where the Literature Doesn't Transfer, and Where I Over-Claimed

**Date:** 2026-08-01
**Motivation:** the last several cycles adopted findings from ~15 fields fast, on the (correct)
principle that checking the literature beats deriving from scratch. But every one of those
literatures was built to answer a *different* question, on a *different* population, under
assumptions that may not survive transfer. This document applies the same adversarial pressure to
the sources that was previously applied to the repo and the wiki.

**Result: five findings I stated too strongly, including one where I was about to reproduce the
exact failure mode I had just finished criticizing.**

---

## The general pattern

Each source answers a question of the form *"does X work for population P under conditions C?"* We
have been reading them as *"is X true?"* The gap between those is where unforeseen bias lives — and
the original teams had no reason to guard against it, because our use case wasn't theirs.

---

## 1. ISO pictogram comprehension — I over-generalized this badly

**What was actually tested:** comprehension of *safety pictograms* — fire equipment, prohibition
signs, hazard warnings — by naive adults, one-shot, out of context.

**What I concluded (F-012):** iconicity does not bootstrap comprehension, therefore UWS's iconic
primitives are not self-evident.

**Why that's an over-reach:**

- **Referential vs. structural iconicity are different things.** A fire-extinguisher pictogram must
  convey an *arbitrary institutional referent* — a specific object embedded in specific fire-safety
  conventions. That is not what UWS primitives do. Point/Line/Angle/Curve/Enclosure denote
  *structural relations*: existence, connection, quality, process, containment. The 45% failure rate
  on fire-action pictograms is plausibly a fact about **culturally-specific referents**, not about
  whether a line between two marks reads as "these are connected."
- **There is contrary evidence I didn't weigh.** Basic spatial-diagrammatic conventions —
  containment as membership, an arrow as direction, proximity as association — are robustly
  understood with little instruction. That is exactly the register UWS operates in.
- **One-shot testing vs. system learning.** Pictograms are tested in isolation. A notation is
  learned as a *system* where primitives mutually reinforce and compositional regularity does work
  that isolated symbols can't. Different learning curve, not measured here.
- **Selection effect.** ISO comprehension studies get run on symbols that were *already suspected
  of being problematic*. Unproblematic symbols don't generate studies.

**Revised claim:** iconicity does not bootstrap **referential** meaning for arbitrary
culturally-embedded objects. Whether it bootstraps **structural/relational** meaning is *untested* —
and it is testable, cheaply, which makes it a good experiment rather than a settled defeat.

**F-012 should be narrowed accordingly.** The conclusion "UWS is a taught notation" survives on other
grounds (music notation, system-level learning), but the pictogram evidence does not support the
strong form I gave it.

## 2. AMR as a benchmark — I nearly reproduced UNL's documented failure

**What it is:** 59,255 annotated sentences. **All English.** Built on **PropBank**, an English
predicate-argument lexicon. Annotated by English-speaking annotators with known inter-annotator
variability. Sentence-level by design.

**What I proposed:** using it as UWS's expressiveness benchmark at scale.

**The bias I missed:** that benchmark measures *"can UWS express English sentence semantics, as
carved by an English predicate lexicon, as judged by English-speaking annotators."* Passing it would
demonstrate English-expressibility and could easily be reported as universality.

**This is precisely UNL's documented failure mode** — "English bias in the Universal Words
inventory... hinders accurate representation of non-Indo-European linguistic elements" — which I
quoted approvingly two cycles ago and then walked straight into.

**Revised position:** AMR remains genuinely useful as a *stress test* (does UWS break on structures
someone else designed?), but **must not be used as evidence of universality**, and any coverage
figure must be reported as *"English coverage"*. If used, it needs a non-English counterpart —
Chinese AMR and other cross-lingual AMR efforts exist and should be checked.

## 3. Concept drift detectors — the transfer claim was wrong

**What I said:** the detection machinery (ADWIN, CUSUM, Page-Hinkley, DDM) transfers off the shelf.

**Why that's wrong:** **DDM monitors classification error rate — it requires ground-truth labels.**
Most classical drift detectors are supervised: they detect drift by watching performance degrade
against known-correct answers. The Cure has **no oracle**. There is nothing that says "this
representation is semantically correct," which is the entire reason the invariant approach exists.

Secondly, concept drift assumes a **temporal data stream**. Semantic corruption may be **static** —
a representation can be internally inconsistent right now, having never drifted.

**Revised position:** unsupervised, distribution-based detectors (ADWIN-style windowing over an
internally-computed quantity) may transfer *if* we can define a scalar semantic-health measure. The
supervised majority of the field does not. The gap is narrower than I implied it had become.

## 4. Gärdenfors convexity — may inherit a Zadrozny-style vacuity

**What I proposed:** design invariants as convex constraints so 𝒜 is convex by construction,
restoring unique projection — with Gärdenfors' "natural concepts are convex regions" as independent
motivation.

**The problem:** convexity is **not a property of a concept — it is a property of a concept relative
to a choice of quality dimensions and metric.** Given freedom to choose the embedding, a very wide
class of sets can be made convex. So "natural concepts are convex" risks being *near-vacuous without
independent constraints on the space*.

**This is structurally the same trap as Zadrozny.** Compositionality constrains nothing without
extra-mathematical naturalness conditions; convexity likewise constrains nothing without independent
justification for the quality dimensions. I flagged the first and then adopted the second uncritically.

Note also that Gärdenfors' criterion is **normative/definitional** — a proposal for what should
*count* as a natural concept — not an empirical finding. The empirical question (do trained systems
exhibit convex concept regions?) is live, not settled.

**Revised position:** the convex route is still worth comparing, but its advantage is *conditional
on justifying the quality dimensions independently* — and that justification is exactly as hard as
the problem it was meant to solve.

## 5. Decipherment — bites the founding scenario, not the practical notation

**What I concluded:** structure without shared reference is undecipherable, therefore UWS's
readability premise is in trouble.

**The mismatch:** decipherment is the problem of recovering meaning **with no specification, no
teacher, no bilingual anchor, and a dead author.** That is the hardest possible case. UWS ships with
a specification, a lexicon, and (per the taught-notation reframing) a curriculum. The analogy binds
only where those are absent.

**Where it does bite, hard:** the *founding scenario* — a mouth-less alien intelligence reading UWS
cold — is exactly the no-specification, no-teacher case. So the finding is fatal to the origin story
while being largely irrelevant to the practical notation.

**Also, alternative explanations for Linear A that I didn't weigh:** the corpus is small and
dominated by short administrative texts (a sample-size problem, not a principle), and the underlying
language may be an isolate with no surviving relatives — a *lexical recovery* problem specific to
natural language, which a designed notation with a published lexicon does not have.

## 6. UNL — attribution is confounded

The criticisms are real and the English-bias lesson transfers. But **why** UNL failed is confounded
across at least four causes that the sources do not separate:

1. Interlingua is conceptually inadequate (the lesson I drew)
2. **It was outcompeted.** UNL launched in 1996; the entire MT field abandoned interlingua because
   statistical and then neural direct translation simply performed better. That is an *economic and
   competitive* outcome, not a conceptual refutation.
3. Governance, funding, and institutional structure
4. Scope: it had to handle idiom, register, and cultural nuance because it was translating *human
   text*. A notation targeting structured machine-to-machine content carries none of that burden.

**Revised position:** UNL is a serious cautionary case, but "interlingua is impossible" is not what
its failure demonstrates. The transferable lessons are narrower: English bias in a "universal"
vocabulary is a real and recurring trap, and pragmatics resists this class of representation.

## 7. Music-notation reform — an extreme incumbent case

The "better notation fails at adoption" finding comes from attempts to displace **Western staff
notation: ~1,000 years of investment, every extant score, all pedagogy, and instruments physically
designed around it.** That is close to a maximal network-effect case.

**UWS is not in that position.** There is no dominant incumbent notation for compositional semantic
structure in machine-readable exchange — the closest occupants (RDF, AMR, CGIF) are partial and
non-dominant. The lesson "being better doesn't win" applies to *replacement*, and applies far more
weakly to *creation in an unserved niche*.

**F-013 should be narrowed:** adoption is a real first-class problem, but the music-notation
evidence overstates the difficulty for a niche without an entrenched standard.

## 8. SHACL — gives the language, not the content

SHACL was designed for **RDF data-quality validation under an open-world assumption** — checking
that published data conforms to expected shapes. Two mismatches:

- **It presupposes a curated shapes graph.** Someone must author the constraints. *That is our
  actual unsolved problem* — we do not know what the semantic invariants are. SHACL supplies the
  language, not the content, and adopting it does not advance the hard part.
- **Open-world baggage.** RDF is open-world; the Cure likely wants closed-world semantics.
  `sh:closed` exists but is working against the grain of the substrate.

## 9. Sheaf/contextuality — developed for a very specific structure

Abramsky–Brandenburger's framework is built for **quantum measurement scenarios**: measurement
contexts, outcome distributions, empirical models as probability tables.

- It requires a genuine sheaf structure — a **cover** on a base space. What plays the role of the
  measurement cover for a semantic representation is **not obvious**, and the whole apparatus is
  vacuous without it.
- Contextuality is a **specific and strong** phenomenon. General semantic inconsistency may be
  broader and less structured than what H¹ detects, so the formalism could be simultaneously too
  narrow (missing ordinary contradictions) and too demanding (requiring structure we can't supply).

**Revised position:** still the best structural *analogy* found, but the gap between analogy and
application is larger than I represented. The first real test is whether the IR admits a natural
cover at all.

## 10. Cognitive Dimensions — a discussion framework, not a metric

Green and Petre's framework was designed for **individual humans interactively using programming
notations on desktop tools, pre-2000**. Two limits:

- It is explicitly a **discussion vocabulary, not a measurement instrument.** It will not settle
  5-vs-6 quantitatively; it will structure an argument about it. I implied more decisiveness than
  it offers.
- Several dimensions (viscosity, premature commitment, progressive evaluation) presuppose a **human
  editing a notation interactively.** If UWS's primary use is machine-to-machine exchange, those
  dimensions are largely irrelevant, and the framework's applicability narrows sharply.

## 11. Changizi — about form, not meaning

The strongest evidence in the project's favor deserves the same scrutiny:

- It measures **topological contour statistics** — i.e. *what shapes are easy to see and
  distinguish*. It says nothing about whether a shape conveys its **meaning**. I used it to ground
  the mark inventory, which is legitimate, but it supports **legibility**, not semantic transparency.
- All 100+ writing systems are **human**, produced and read by one visual architecture, shaped by
  *terrestrial* scene statistics. It is convergence within a single perceptual system.
- **Survivorship:** only scripts that survived and were recorded are in the sample.
- Mild circularity: human eyes evolved for natural scenes; humans made scripts for human eyes;
  scripts match natural scene statistics. True, and less deep than it first sounds.

## 12. Linguistic universals — the field's own caveats are the finding

WALS-based typology has well-known **sampling bias**: uneven coverage skewed toward well-documented
(often colonially-contacted) languages, with genealogical and areal clustering. Some "universals"
may be artifacts of feature coding. The field says all this itself — which is why the honest
takeaway isn't a specific universal but the methodological caution: *if absolute universals are this
hard to establish within one species, claims about universals across minds need far more humility.*

---

## Consolidated corrections

| # | What I said | What's defensible |
|---|---|---|
| 1 | Iconicity doesn't bootstrap comprehension | Doesn't bootstrap **referential** meaning; **structural** iconicity is untested and cheaply testable |
| 2 | AMR gives UWS a benchmark | Gives an **English** stress test; using it as universality evidence repeats UNL's exact failure |
| 3 | Drift detectors transfer off the shelf | Mostly **supervised**, require an oracle we don't have; only unsupervised windowing may transfer |
| 4 | Convexity-by-construction rescues metric repair | Conditional on independently justifying quality dimensions — same underdetermination as Zadrozny |
| 5 | Decipherment threatens UWS readability | Threatens the **founding scenario**; largely irrelevant to a notation shipped with a spec and curriculum |

**Also narrowed:** UNL's failure is confounded with being outcompeted by statistical MT; the
music-notation lesson is about *displacement*, not creation; SHACL supplies language not invariants;
sheaf theory needs a cover we haven't identified; Cognitive Dimensions won't settle counts.

---

## The meta-lesson

The rule adopted last cycle — *check the literature before theorizing* — is right, and produced real
results. But it has a failure mode of its own: **adopting a finding at the confidence level the
source states, rather than at the confidence level that survives transfer to our question.**

Every source above is competent work. None of it was designed for us. The discipline that needs
adding is a transfer check: *what population, what conditions, what question — and does our case sit
inside them?* On the evidence here, roughly a third of what I imported needed narrowing on first
inspection.
