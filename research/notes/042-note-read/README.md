# 042 — The note read I skipped, and what it found

**Type:** correction
**Opened:** 2026-08-12
**Status:** closed
**Thread:** completes step 1 of `039`, which I marked done without doing

---

## 1. The admission

`039` set a five-step review method. **Step 1 was "read all 38 notes in sequence — the actual
trajectory, not my memory of it."** I did steps 2–5 and **substituted the claim-level audit for the
note-level read**, then recorded the review as executed.

**The completion table in `039` §5 has five rows and step 1 is not among them.** The gap was
self-documenting and I did not look.

**Why it happened:** the mechanical audit *felt* complete. It produced findings, fixed a real error,
and passed its checks. **Local judgment, no check against the stated plan** — the same shape as
everything else this review has been about.

## 2. What the read found that the claim audit could not

### 2a. Ten notes with contradictory status

`012`, `013`, `019`, `020`, `021`, `028`, `029`, `034`, `035`, `041` each had **two Status lines**:
header `open`, body `closed`. Completing a note inserted a new line rather than editing the header.

**Every claim-level checker was blind to this**, because they read `claims.yaml` and this is in the
notes.

### 2b. Zero backward supersession links

The notes convention states that a superseded note gets a `Superseded by:` line — *"the one permitted
edit."* **Nine notes explicitly correct earlier ones. None of the targets carried a link.**

> **Supersession was recorded forward only.** Reading `003` in isolation gave a superseded position
> with **no signal it had been corrected.**

**This is the third instance of one failure class this session**, and now the pattern is unambiguous:

| Where | A change was recorded… | …and the dependents were unaware |
|---|---|---|
| F-028 | second YAML key | parser silently dropped the first |
| `039` | hub claim scoped | 0 of 5 dependents carried it |
| **here** | **note corrected** | **0 of 9 targets carried it** |

**A change lands where it is written and nowhere else.** Three different representations, same
failure.

## 3. My own automation made two errors, which is the point

Adding the backward links by pattern:

- **Attributed `023 ← 024` wrongly.** `024`'s thread says *"**follows** `023`… corrects the scope of
  `014` and `022`."* The regex took every `` `NNN` `` in a thread containing "correct". **A false
  supersession is worse than a missing one** — it tells a reader a live note is dead.
- **Split a wrapped Thread line** in `015`, inserting the new line mid-sentence.

**Both were caught by reading the output**, not by the automation. Recorded because the review's own
subject is mechanical judgment losing context, and it did so here twice in one command.

## 4. Enforcement

`tools/check-notes.rb` — one Status line per note with a valid value, and every "corrects" claim
matched by a `Superseded by:` on the target. **Six checkers now run in CI.**

**The generalizable point:** the five existing checkers all read `claims.yaml`. **The notes are a
different representation and nothing checked them at all** — which is exactly F-028's lesson
(*"a validator that only sees post-parse state is blind to an entire class of corruption"*) restated
one level up. **Checks inherit the blind spots of whatever representation they read.**

## 5. Status of the trajectory itself

Having read the sequence: **no note's substantive conclusion contradicts a later one without the
later note saying so.** The nine corrections are honest and each names its target. The failure was in
**bookkeeping**, not in the reasoning — which is worth stating plainly, since the point of the read
was to find out and the answer could have been worse.
