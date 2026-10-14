# 075 — Tiers are earned, not declared — and the evidence that was never there

**Type:** correction
**Opened:** 2026-10-14
**Status:** closed
**Thread:** executes the tier redesign the owner asked me to brainstorm before ruling; resolves
`F-039`; turns on the propagation rule `065`/`067` imported and never ran at home.
**Prompted by:** a cold reader's screening pass, which found a regex in `tools/check-claims.rb`.
Then the owner: *"brainstorm around the rule differently on the tiers first... for good measure"*,
followed by *"use your best judgement on how to proceed; then execute."*

---

## 1. The diagnosis: one scalar doing four jobs

| job | values | state |
|---|---|---|
| **confidence** | VERIFIED / ARGUED / CONJECTURED | the only one the field was for |
| **kind** | DESIGN-CHOICE | not a point on a confidence scale at all |
| **lifecycle** | RETIRED | orthogonal to both |
| **reachability** | — | **had no field anywhere** |

That last row is `F-039`. Reachability — can a reader actually reach this evidence? — was the one
axis with no representation, and so the one whose failure was invisible.

The conflation also made the headline figure malformed. `VERIFIED / all claims` counts 37
DESIGN-CHOICE and 8 RETIRED in the denominator, neither of which can ever be VERIFIED. The
well-formed base is the **138** claims on the confidence scale, not 183 — and this error ran
*against* the project's interest, which is presumably why it survived.

## 2. Four diseases, one label

This session's failures each hit a different job:

- **reachability** — the seven `ul_agent` claims (`F-039`)
- **relevance** — the ℤ/2 ledger claim, whose test is `mirror().mirror() == self` on a
  construction that is an involution by definition. Its own `notes:` say *"NEAR-TAUTOLOGICAL TEST,
  flagged rather than counted"* — three layers down, while `README.md` says "proved and
  machine-checked" at the top.
- **coverage** — `F-038`: REQ-1 was violated in code because expressibility was never a registered
  claim, so no checker had anything to enforce.
- **mechanism** — five claims registered that same day sat in the regex's blind spot.

## 3. What was built

**Reachability is computed, never authored** (`in-tree` / `external` / `absent` / `no-evidence`).
An author cannot write it, so an author cannot get it wrong. `VERIFIED` + unreachable is a hard
error: **today's failure is now unwritable**, which is the only kind of fix worth having.

**Earned tier** = `min(declared, what the evidence supports)`. Runnable in-tree evidence can reach
VERIFIED; a corpus entry counts, because `corpus_tests.rs` asserts it field-by-field against the
implementation; documentary evidence (a note, a spec) caps at ARGUED; external or absent evidence
earns nothing above CONJECTURED. Declared and earned are printed side by side with every gap named.

```
VERIFIED: 34 declared  |  31 EARNED
DECLARED ABOVE EARNED (3)  — all three are documentary, and listed rather than hidden
```

**`premises:` — the edge the schema never had.** `uws/register/` carries *"a derivation routed
through an undetermined premise is itself undetermined"*, and `check-derivation-premises.rb`
enforces it **on the vendored register only**. It could never run here, because `claims.yaml` had
no premise field at all: the only inter-claim edges were `scope_reviewed` (an *acknowledgement* —
`074` established that) and `superseded_by` (lifecycle). Three one-off improvisations —
`blocked_by`, `blocks`, `bounded_by`, one use each — are authors reaching for the edge and it never
becoming schema.

> **The repo imported its best rule and applied it only to someone else's data.**

Coverage is **4 of 138** and reported as such, in the register's own style, because a pass over 4
edges says nothing about the other 134.

## 4. It bit me within the hour

`OPEN-LEDGER-LINE-IS-A-BLAST-RADIUS` was registered VERIFIED earlier the same day — by me, in
`074`. Its premise `AUTHORITY-ANCHORS-FAIL-TOTALLY` is ARGUED. A claim cannot be better established
than what it rests on, so the rule demoted it, and I yielded rather than arguing. The measured half
(65 of 183 load-bearing, 53 downstream of a conjecture) is exact; the interpretive half is the
anchor argument; ARGUED is the tier of the conjunction.

**A rule that only ever demotes other people's claims is not a rule.**

## 5. The seven, resolved

Re-derived in-tree, `verify.py`, stdlib only, 16 checks, exact unless noted:

| claim | outcome |
|---|---|
| `CODE-OCTONIONS-FROM-HAMMING` | **VERIFIED, earned.** [7,4,3] generated; seven weight-3 supports satisfy all four Fano axioms; an orientation exists making the algebra alternative, units squaring to −1, 168 associator failures. Hurwitz does the rest. |
| `CODE-TAMMES-FORCES-THE-SOLIDS` | **VERIFIED, scoped as numerical.** Seeded maximin search → 90.0000° and 63.4349°, degrees 4 and 5. Reproduces a classical theorem; does not prove it. |
| `CODE-ICOSIANS-ARE-E8` | **demoted to ARGUED** — see §6 |
| the other four | **demoted to CONJECTURED**, with `falsified_by` and `revival_condition` |

The four are not re-derivable *here*: they rest on "the sovereign-hexagram 6-cube", "the symbol
octahedron", "each solid's axis window" — constructions that appear nowhere in this tree, so there
is nothing to re-derive from. Their statements are recorded unchanged. Only the tier moved, to the
one the available evidence supports, which is none.

## 6. The refusal

The icosian group theory came out exactly: 120 units over `Z[φ]`, all of reduced norm 1,
exhaustively closed under quaternion multiplication (so the binary icosahedral group `2I`, vertex
set the 600-cell), `Z`-span of rank 8.

The E8 identification did not. Under `Tr(x·conj(y))` the Gram matrix of a `Z`-basis is non-integral
with determinant **`625/256 = 5⁴/2⁸`**, not 1. Two greedy bases and a Hermite normal form give the
same value, so it is a property of the form rather than the basis — the E8 presentation of the
icosian ring needs the inverse-different twist, which this script does not construct.

> `5⁴` over `2⁸` is an open invitation to divide by 625 and multiply by 256 until the determinant
> reads 1. **Refused.** Choosing a normalisation because it yields the wanted answer is `F-035`'s
> ratchet applied to arithmetic, and REQ-3 says a negative result is recorded rather than tuned.

The script carries it as `UNRESOLVED` — a third channel, neither passed nor failed — and the claim
sits at ARGUED with the determinant written into its scope as the precise thing that is missing.
Reviving it is a bounded, named piece of work, which is worth more than a label.

## 7. Limits

- **`premises:` coverage is 4 of 138.** The mechanism is live; the graph is nearly empty. Every
  number it produces is a lower bound, and a pass means little until coverage rises.
- **Earned tier measures reachability and runnability, not relevance.** A trivial passing test
  earns VERIFIED under these rules. The ℤ/2 case (§2) is untouched by this work.
  The fix I would propose and did **not** build: require VERIFIED evidence to carry a recorded
  **negative control** — a seeded failure the test demonstrably catches. That is a boolean, so it
  resists the ratchet, where a TAUTOLOGICAL/ENTAILING judgement field would not: every author rates
  their own evidence "entailing", which is precisely how the ℤ/2 flag ended up in `notes:` instead
  of in the tier.
- **The kind/lifecycle split was not performed.** DESIGN-CHOICE and RETIRED still live in the
  `tier` field; only the *denominator* was fixed. A real split touches 183 entries and buys less
  than the three changes above.
- **The four demoted claims may well be true.** Nothing here tests them. A demotion is a statement
  about this repository's evidence, not about the world.
