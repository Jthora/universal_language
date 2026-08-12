# 039 — Consensus review: where discernment lost context

**Type:** correction
**Opened:** 2026-08-12
**Status:** open — this is the review's opening, not its conclusion
**Thread:** audits everything

**Prompted by:** *"we should review all our notes and rebuild consensus… see how our discernments
lost context due to the overwhelm of data we jumped across."*

**Confirmed, and the mechanism is measurable.**

---

## 1. Three structural causes, with numbers

### 1a. The synthesis was never re-read, only patched

**38 notes. 9 edits to `STATE-OF-PLAY.md`.**

Every note appended to a complete record. The synthesis received a *delta applied to a partial
picture* — four times less often than the record grew. **A patch is consistent with what it patches,
not with what exists.** `037`'s "UWS is kind A" was locally coherent and contradicted eleven notes.

### 1b. Priority inflation — the ranking stopped discriminating

**39 of 99 claims are priority 0.**

Each was marked priority 0 because it mattered *in the note that produced it*. **Nobody ever re-ranked
globally.** With 39% of claims at top priority, the field is uninformative — which is the same failure
as the synthesis, in a different register: **local judgment, no global pass.**

### 1c. Corrections do not propagate to dependents

`FIXED-POINT-IS-COMBINATORIAL-MAP` is referenced by **five** claims and has been scoped **three
times** — connected-configurations-only (`032`), UWS-not-UL (`034`), UP-role (`035`).

> **Zero of the five carry any of those scopings.** All five still read the hub in its original,
> unscoped form.

**A correction lands on its target and nowhere else.** The claim registry has no mechanism to
invalidate dependents, so scoping a hub silently leaves everything built on it reading the old
version.

*(Method note: my first scan reported two of five as carrying "connected-only." That was a false
positive — the word appears in "connected components of the complement," an unrelated use. **The
scan for lost context itself lost context**, which is the pattern operating one level up.)*

---

## 2. The first live casualty

**`REGIONS-ARE-FACE-UNIONS` — the Phase 4 decidability result — inherits a limitation it does not
carry.**

The argument: regions are unions of the map's faces → the domain is a finite Boolean algebra →
RCC-8 is decidable by construction, avoiding the general framework's undecidability.

**But `032` established that for *disconnected* configurations the rotation system does not determine
an embedding**, and face tracing treats each component as embedded on its own sphere — two disjoint
triangles trace **four** faces rather than the three they bound in the plane.

**A notation with more than one stroke is disconnected by default.**

> **The result stays decidable and becomes wrong.** You still get a finite face set, so RCC-8 is
> still computable — over faces that do not correspond to the actual planar regions. **Confident
> incorrect answers, which is worse than undecidability**, because nothing signals the failure.

**Status: not a bookkeeping error. A Phase 4 conclusion that does not hold for the objects it was
designed for.** Recorded here; the fix belongs in the rebuild.

---

## 3. Method for the rebuild

The three causes all have one shape — **local judgment, never a global pass** — so the method has to
be a global pass, and it cannot be a patch.

1. **Read all 38 notes in sequence.** The actual trajectory, not my memory of it. Memory is what
   produced `037`.
2. **Read all 99 claims in one pass**, checking them *against each other* rather than against
   sources. The checkers already verify claim-to-evidence. **Nothing verifies claim-to-claim**, and
   that is where §1c lives.
3. **Propagate every hub correction** to its dependents, or record why it does not apply.
4. **Re-rank priority globally.** If everything is priority 0, nothing is.
5. **Write the consensus fresh.** Not a patch to `STATE-OF-PLAY.md` — a new statement, with the old
   one replaced only once the new one is complete.

**And one enforcement to add**, since §1c is mechanical: a check that when a claim's `scope` changes,
its dependents are flagged for review. Prose cannot carry this — that is the lesson of the last four
protocol additions.

---

## 4. What this review is not

**It is not an occasion to re-litigate settled findings.** The derivations in `022`–`032` are
internally valid and were checked when made. The risk in a review of this kind is the opposite of the
one that caused it: **treating "we moved fast" as grounds to doubt everything**, which is asymmetric
skepticism wearing a different hat (F-017).

**The target is specific: judgments that contradict the record.** Not judgments that are merely old.
