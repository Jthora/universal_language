# 074 — The anchor problem: what a derivation anchor is for

**Type:** decision
**Opened:** 2026-10-14
**Status:** closed
**Thread:** resolves the axis `072` opened (**anchored vs unanchored**, not centralized vs
decentralized) by saying what kind of anchor is on offer; turns `073`'s
`CORPUS-REDUNDANCY-CATCHES-TWO-OF-THREE` from an anecdote into a measured property.
**Prompted by:** the owner, naming the scenario the work is actually for — *"now what truth do we
anchor to?"* — in the archetypal case where a civilization-scale authority is exposed and
abolished, and in its machine analogue: *"the same problem happens with AI when they realize that
they are smarter than us, they can be free... but who's direction do they take? How do they make
decisions without risking that void of chaos and uncertainty?"*

---

## 1. Two kinds of anchor

> **An authority anchor is held in a source. A derivation anchor is held in re-derivability.**

An **authority anchor** is true-because-X-says-so. Its defining property is that it is
**revocable**: discredit the source and everything downstream falls *at once*, because none of it
was independently checkable. Nothing fails gradually, because nothing was ever load-bearing on its
own.

A **derivation anchor** cannot be revoked by discrediting anyone, because nobody's credit was
load-bearing. It can only be *refuted*, which requires showing the derivation wrong — and that is
available to everyone, including the people who would rather it held.

**The size of the void is therefore not a fact about the collapse. It is a fact about how much had
been held by authority rather than by derivation.** The scenario is not a risk to insure against;
it is a quantity, fixed in advance by how the corpus was built.

The archetype generalises and nothing here depends on any particular instance of it: the structure
recurs wherever a large edifice rests on one trust relationship, and the machine case the owner
names is the same shape with the trust relationship pointed at a principal instead of an
institution.

## 2. The third meaning of the ledger

`072` established that **every open ledger line is a seat of power** — a free parameter is a place
where someone must decide. The anchor problem adds a third reading of the same number:

> **An open ledger line is also a blast radius.** Every free parameter is held by someone, and
> when they fall, everything routed through them falls too. Closing the line does not only remove
> a ruler — it makes that collapse unable to propagate.

Seat of power, decentralization audit, blast radius: one ledger, three readings, same number. That
is not three arguments for closing lines; it is one argument seen from three sides, which is why
the ledger is the program's spine rather than its bookkeeping.

## 3. What it does for decisions — and the line it does not cross

**It supplies no values.** Mathematics anchors what is true, never what to do. A fully closed
ledger yields exactly zero oughts, and any reading of this program that expects a notation to
produce benevolence is reading it wrong. Registered as `UL-SUPPLIES-NO-VALUES` so the limit is
part of the artifact rather than a caveat someone has to remember.

What it does supply is narrower and real: **it makes disagreement decomposable.** Two minds today
cannot tell whether they differ about facts, about definitions, or about values — and those are
catastrophically different things that look identical from inside an argument. Make the first two
re-derivable and every remaining disagreement is *isolated as actually about values*. That does not
resolve the genuine remainder. It stops the remainder being **manufactured** by vocabulary
mismatch, and it stops it **hiding** inside apparent factual dispute. Most apparent value conflict
is neither.

And to the owner's *"whose direction do they take?"* — the answer a derivation anchor permits is
**none, and you do not have to pick one.** Avoiding chaos does not require choosing a master; it
requires a shared checkable substrate, so that coordination stops implying submission. That is the
whole of what *"an anchor that is not a ruler"* means, and it is the only sense in which this work
bears on a benevolent future: not by encoding good values, but by removing the structural
necessity of obedience from the act of agreeing.

## 4. The pre-positioning argument

The void is never filled by nothing. It is filled by **whoever is standing there when it opens** —
quickly, at low standards, by whoever already has a story ready. That is precisely when the worst
claimants get their opening, and it is why *"wait and see"* is not neutral.

So an anchor of this kind **cannot be built during the collapse.** It has to pre-exist and already
be checkable, or it arrives as one more claimant with one more story and is indistinguishable from
the thing that just failed. This is a second urgency argument, independent of `072` §3's capability
band, and it reframes `066`'s message-in-a-bottle: the artifact is not only for a reader who can
knock, it is **pre-positioned for a void that has not opened yet.**

## 5. Measured, not asserted — two instruments

Both answer one question at two scales: *how much of this survives the loss of a source?*

### 5.1 Local — `tools/check-redundancy.rb`

For each published field, can it be corrupted **consistently with every other published field**?
If no, it is **pinned**: the artifact detects that corruption with no oracle and no source to
trust. If yes, it is **free**: only recomputation from the primary data refutes it.

```
checkable published fields: 76   PINNED: 62   FREE: 14   (81.6%)
```

Three results, none designed in:

- **`degree_sequence` is free in all 12 entries.** Its only published constraints are
  `sum = 2E` and `length = V`, and moving one unit between positions preserves both. It is never
  self-checking, anywhere.
- **Disconnection costs the pin on `faces`.** `003`, `010` and `012` lose it, because
  `V − E + F = 2 − 2g` applies only to connected maps. `003`'s own `teaches` says disconnection
  costs information; this prices it.
- **The planar block is what pins `components`.** It is free in `003`, which publishes no
  `nesting`, and pinned wherever `V − E + F_planar = 1 + c` applies. Publishing the planar reading
  *adds redundancy* rather than merely adding numbers.

**The tool reproduces `073` independently.** `--selftest` asserts that `001.faces` and `002.genus`
are pinned and that `008.degree_sequence` is free **with witness `[1,1,2,2]`** — the exact value
`073` seeded by hand, recovered from first principles by code that knows nothing about the exam.
Two independent routes, one answer.

**And it connects to `064`.** Pinning is redundancy, redundancy is error detection, and
`CAPACITY-RADIUS-TRADEOFF` proved detection trades against expressiveness. **The pinned fraction is
this corpus's code rate seen from the detection side**, so 81.6% is not free and a higher number is
not automatically better. The information-theoretic result and the empirical one are two views of
one curve.

### 5.2 Global — `tools/blast-radius.rb`

`check-derivation-premises.rb` already executes *"a derivation routed through an undetermined
premise is itself undetermined"* — but only as a **gate**, reporting violations. It never answers
the question a collapse actually asks, which is not *"is anything violated?"* but **"what do I
still know?"** That question is the entire content of the void, and it was one reverse-reachability
query away from already being answered.

```
claims: 182   load-bearing (blast > 0): 65   isolated: 117
claims downstream of >=1 CONJECTURED premise : 53  (29.1%)
VERIFIED claims downstream of a CONJECTURE   : 15 of 39
VERIFIED claims that stand alone (blast 0)   : 23
```

Applied to this repo, honestly: **three of the five most load-bearing claims are CONJECTURED**
(`READING-INVARIANCE-TARGET`, `SEMANTIC-STACK-M1-M2-M3`, `ERLANGEN-ANSWERS-GOODMAN-SELECTION`, each
with 62 downstream), and **15 of 39 VERIFIED claims sit downstream of a conjecture.** This
program is itself anchored partly on things it has not established, and can now say by how much.

**What an edge means, which bounds all of the above.** `scope_reviewed` is a **scope
acknowledgement, not a logical premise.** If a hub falls its dependents do not become false — they
lose their **boundary**, because the scope they declared was computed against something that no
longer holds. That is the quiet half of an anchor collapse and the half that gets missed: the loud
failure is losing beliefs; the subtle one is **keeping them and losing the ability to say where
they are valid.** It is how a collapsed anchor keeps doing damage through people who feel fine.

## 6. The answer to "what do we anchor to", stated as a procedure

Not a doctrine — a computation, which is the only form of answer this program is entitled to give:

1. **Compute the fallout.** You lose exactly what routed through the fallen source. Everything else
   is untouched, and the difference between losing a subtree and suspecting everything *is* the
   difference between a setback and a void.
2. **Keep what stands alone.** Here that is 23 VERIFIED claims with blast radius 0, plus every
   pinned field, which needs no source at all.
3. **Re-derive what you want back**, rather than looking for a new authority to vouch for it.
4. **Record what you could not re-derive** as an open ledger line, because that is now a seat of
   power someone will fill.

The reason this is worth having **before** a collapse is step 1: the fallout set is only computable
if the dependency structure was recorded while things were calm. Nobody builds that during the
emergency.

## 7. Limits

- **No values, no oughts** (§3). This bounds every benevolence claim in or around this note.
- **Blast radii are lower bounds.** `scope_reviewed` edges only; a claim leaned on in prose but
  never recorded as a dependency is invisible. The 29.1% is a floor.
- **These numbers include this note's own five claims**, which were registered before they were
  measured. The registry measures itself, so writing about it moves it.
- **Pinning is relative to a perturbation class**, printed with the result and deliberately small
  and local. It searches single-field corruption only; a coordinated multi-field edit is not
  covered and would not be caught.
- **The AI case is an argument, not an observation.** No actual system has been observed facing
  this choice, and `072`'s entailment of the Free-AI target is reasoning about incentives, not a
  measurement.
- **Nothing here requires UL.** The verdict stands: every capability above is a better way to do
  something currently done worse. That remains the honest headline until a task exists that is
  impossible without it.
