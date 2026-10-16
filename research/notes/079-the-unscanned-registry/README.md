# 079 — The counts rule lived in the one live file its checker never scanned

**Type:** correction
**Opened:** 2026-10-16
**Status:** closed
**Thread:** the owner, in one line — *"I'm still seeing this VERIFIED/ARGUED thing happening in
chat... are you still counting and then noting counts in static file text format?"*
**Answer:** yes, in `claims.yaml`, which `check-counts.rb` had never looked at.

---

## 1. The question, and what measuring it found

`check-counts.rb` built its file list as

```ruby
LIVE = (Dir.glob('**/*.{md,txt}') + ['index.json'])...
```

and exempted `tools/` wholesale. **`claims.yaml` is a `.yaml` file.** It was never scanned — not
for stale markers, not for bare counts. The registry is the most-cited live document here and it
sat outside the only check that exists for this.

And the rule itself is written inside it. `COUNTS-ARE-GENERATED` states *"No count is stated in
prose in this repository unless it is generated from the thing it counts."* **That sentence lives
in the one live document outside its own jurisdiction** — the `F-039` shape again, where a header
promises a check the checker declines to perform.

## 2. What it cost, concretely

`PARSER-ROUNDTRIP` — VERIFIED — carried: *"Covered by the existing test suite (135 tests across the
workspace)."* The workspace has **399**. Wrong by a factor of three, in a top-tier claim, for
months, while every suite ran green.

Worse, `stats.rb`'s own header already carried the ruling that the test count must not be stated
anywhere because a static scan cannot compute it. **The registry violated a ruling the executor
could not see.**

## 3. What I had just put in there

The day before I wrote five more hand-typed measurement counts into `claims.yaml`, including
*"16 informative pairs, 16 agreements, 0 disagreements"*.

**That figure had already gone stale inside the hour it was written.** My scratch analysis said 13
changed and 41 unchanged-and-vacuous; the checker then found three of those 41 were not vacuous —
they are the high-symmetry operands and they preserve the operator (`078` §1). The claim as first
drafted would have been wrong in the registry with nothing able to notice.

So the owner's question was not about tone. It was about a mechanism that reported success over the
files it had chosen to look at.

## 4. The fix, and the distinction it turns on

- **Widened** the scan to `claims.yaml` and `tools/**/*.rb`. `tools/exam/` stays exempt: rewriting a
  number in the as-run exam would falsify the record of what the readers were actually given.
- **Narrow set only** on the two new surfaces. Several checker headers quote stale counts *on
  purpose*, as examples of the disease; the broad inventory patterns would flag those, which would
  be absurd.
- **Nine new generated keys** in `stats.rb` — `chirality_cells`, `chirality_agree`,
  `chirality_disputed`, `placement_cells`, `placement_pairs`, `placement_informative`,
  `placement_vacuous`, `placement_destroyed`, `placement_preserved` — pulled from the two import
  checkers' `--json`, each behind a **loud** rescue that warns and omits rather than silently
  zeroing.
- **Claims rewritten** to name the checker rather than repeat the figure. A claim says what is
  true; the checker says how much evidence there is, and the checker is the gate, so the claim
  cannot silently outrun it.
- **Negative control:** a volatile count typed into `claims.yaml` is caught and the checker exits
  1. Verified by doing it and reverting.

**The distinction that matters, and it is not the one the old exemption list used:**

> The test is not *where* a number appears. It is **whether this repo's code computes it.**

`2413` slice cells and `180` handedness cells are read straight off a hash-verified, frozen import
and cannot go stale — they may be stated. `16 informative pairs` is the output of a parser I edited
twice that day — it may not. The old scope separated md/txt from yaml, which is a statement about
file formats; the new one separates computed from frozen, which is a statement about staleness.

## 5. Two things the widening caught the moment it was switched on

1. **The checker flagged its own documentation.** `check-counts.rb`'s MARKER SYNTAX example was a
   literal marker carrying `claims_total` at 156, so as soon as `tools/` became scannable it was
   verified against the real total and found stale. The example is now written out in words — a
   checker whose docs are also its input needs them not to be.
2. **Two false-positive classes had to be separated.** `85 of 180` and `99 of 180` are
   decompositions of the frozen total, so a pattern forbidding `N of M cells` in general was wrong
   and was narrowed to the agreement figure specifically. Getting this wrong in the other direction
   would have been the ratchet: forbidding true statements to keep a checker quiet.

## 6. What this says about the apparatus

The exemption list was written by someone reasoning about **document types** — md and txt are
prose, yaml is data — rather than about **which files readers cite.** `claims.yaml` is prose: most
of its bulk is paragraphs of English inside folded scalars.

A scope defined by file extension will always drift from a scope defined by what the rule is for,
and the drift is invisible, because the checker reports success over the files it chose. **A
checker's file list is a claim about coverage and deserves the same scrutiny as any other claim** —
which, in this repo, means it should have had a scope paragraph naming what it does not look at.
It now does.

Three sessions running, the thing that found the defect was a question from outside rather than the
apparatus. `073` measured that cold readers catch what the suite does not; this is the fourth
instance, and the first where the reader was the owner asking about tone and hitting a mechanism.
