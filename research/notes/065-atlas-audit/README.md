# 065 — The ledger run as a universality audit: the generation is exact, the generators are not

**Type:** cycle
**Opened:** 2026-10-14
**Status:** closed
**Thread:** executes the work `064` §2(b) named as the program's cheapest falsifier; imports the
object that claim is about (`uws/register/`); adopts one rule from it and gives it an executor
(`tools/check-derivation-premises.rb`, the eighth checker).
**Prompted by:** the owner choosing "atlas in, then audit it" over three alternatives, after
`064` registered a universality claim about an object the repo did not contain.

---

## 0. Why this note could run at all

`064` made ledger closure *constitutive* of universe-scope invariance rather than evidence for it:
an irreducible conventional residue **is** a free parameter the mathematics does not fix. That
turned the universality falsifier from an experiment into an audit — walk the load-bearing path and
ask of each element *does the mathematics force this value?*

Then it could not be run, for a reason that took a moment to see: **the object was not in the
repo.** `UNIVERSAL-IS-MATHEMATICAL-SCOPE` says the shared thing is *the map of all possible symbolic
geometry*. The repo held UWS — one chart — plus a claim about the map. The map was on the website.

So: imported verbatim and hashed (`uws/register/derivation-register.json`, 145 entries,
`sha256` `135b523a670241a0`), then audited. `audit.py` is the audit; this note is what it means.

---

## 1. The result

> **The map's generation is exact. Its generators are not derived.**

| layer | verdict | residue |
|---|---|---|
| marks → inventory | **DERIVED** given the axes | none |
| marks → the two axes | MEASURED | unquantified |
| **operators → inventory** | **NOT DERIVED** | **UNBOUNDED** |
| constructions (191) | EXACT given the generators | inherits the operators' |
| inventory completeness | **DEFECT** — `close` unpublished | n/a |
| orbit individuation | TOLERANCE-BOUNDED | range 89–94 |
| naming layer | CONVENTIONAL, **and correctly so** | 3 entries |

### What is genuinely forced, and it is more than I expected

**The six marks are not a list.** They are the complete product of two axes — symmetry order
`{∞, 2, 1}` × curvature class `{singular, smooth}` — and **all six cells are filled**:

```
   ∞ | singular=Point   smooth=Circle
   2 | singular=Line    smooth=Wave
   1 | singular=Angle   smooth=Curve
```

Given the axes, the inventory is forced and its cardinality is a *consequence*. This is the right
shape of result for this project and it is the one the repo's own `UWS-PRIMITIVE-COUNT` predicted:
the count was never the interesting object, the generating structure is. Verified in `audit.py` §1.

**And the 191 constructions are exact arithmetic**: 5 binary × 36 ordered pairs + `reflect_pair` on
6 + `close` on 5 (a Point has nothing to close) = 191, matching the register exactly.

### What is not forced, and it is the load-bearing finding

**Every operator carries status `measured`.** All six. In the register's own vocabulary `measured`
means *"a fact about these artifacts, not necessity"*, and each operator's note says it outright:
*"attested on a sheet; its semantics are not thereby derived."* The operators are attested from the
owner's drawn corpus — the hexagram, Point-in-Circle, SymbolFolds, the sine.

So **the operator inventory is the largest free parameter on the load-bearing path**, and it is a
far bigger residue than the syntax layer's proved ℤ/2. Everything downstream inherits it: the 191 is
exact *given* a generator set that the mathematics does not force, and the orbits are a quotient of
that.

**Under REQ-3 this is recorded as a shortfall in literal universality, not rescoped.** The
classification layer's ledger line does not close. That is the honest state, and it is the first
time the program has had a *measured* answer at this layer rather than an open question.

**Scope, stated so this is not over-read:** `measured` means *no derivation is recorded*, not *no
derivation exists*. The finding is that the inventory is currently unforced — which is a shortfall
in scope, not a proof of impossibility. The symmetric discipline cuts both ways here: "the operators
cannot be derived" would itself need the full negative-result treatment, and nothing here supplies it.

### Two narrower findings

**`close` is load-bearing and unpublished.** The 191 requires `close` on 5 marks. The register's
`operators` array has six entries — five binary plus `reflect_pair` — and **`close` is not among
them**, while the prose uses it throughout (*"reflect_pair and close as unary steps"*, distinguished
from *"the five attested binary operators"*). An operation the construction count depends on is
missing from the inventory that is supposed to contain it.

**The orbit count is a range, not an integer.** 93 is a central value across 89–94, taken at a
calibrated tolerance, and the register says explicitly that doubling the rotation search does not
settle it. Its scope line limits the damage — *"about the count, not about the partition's
structure... nothing here disturbs any claim about which figures are related"* — and that limitation
is load-bearing, because a tolerance that moved the *partition* rather than the count would mean the
map is a family of classifications indexed by a calibration parameter rather than one object. **The
register asserts the partition is stable, and that assertion is itself at `measured`.** Checking it
is unfinished work, named in §4.

### One good result, said plainly

**All three admitted conventions are naming, not structure:** element colours (Golden Dawn,
inherited wholesale), modality names (astrological words with an external referent), and the status
badges (iconic, and the register says iconic is motivated rather than derived). **None of them
touches which figures exist or how they relate.**

Under universe-scope that is exactly where convention is permitted to live. Naming is free;
classification is not. A reader looking for the project's conventions to be load-bearing will not
find them here, and that is worth stating as a positive result rather than leaving implicit.

---

## 2. The rule worth stealing, and what running it showed

The register carries one rule its own author calls *"the part that transfers to a corpus we did not
write"*:

> **A derivation routed through an undetermined premise is itself undetermined.**

**This repo had nothing equivalent, and the gap matters.** `scope:` plus `scope_reviewed:` is a
**local, per-node** acknowledgement — each edge is judged alone and nothing propagates uncertainty
forward through a chain. `FAILURES.md` **F-035** records precisely what per-node checking costs: a
perimeter retreats one locally-defensible step at a time with nothing summing the steps. **Forward
propagation is the shape of check that cannot be defeated that way**, which is why this rule is the
right thing to adopt and not merely a nice piece of hygiene.

Adopted, with an executor — `tools/check-derivation-premises.rb`, wired as the eighth checker,
because the register states the rule in prose and **prose does not execute** (`048`).

**What running it found, and the second half is the interesting half:**

- **No violations.** No assertable entry transitively routes through an `undetermined` or
  `withdrawn` premise. No cycles. The register passes its own rule.
- **Coverage is 15%.** The rule can see 22 of 145 entries, because premises are stated as free text
  and only sometimes contain claim ids. **13 of the 25 `derived_given` entries state their premise
  as prose only and are invisible to the rule their own author wrote.**

So the checker reports coverage as part of its result. **A rule that passes by being blind has not
passed** — and a 15%-coverage pass reported as a clean pass would be this repo's own failure mode
(`048`: rules fire when they are the subject and fail as background) reproduced in a borrowed rule
on its first run.

---

## 3. Two vocabulary gaps this repo should close

Imported honestly, the register's status vocabulary is **finer than `claims.yaml`'s on the
conditional axis**, in two specific places:

- **`derived_given` has no repo equivalent** — *assert only together with its premise*. The nearest
  is `ARGUED` + `scope:`, but a scope is a caveat on a standing claim while a `given` is a premise
  the claim must **travel with**. Collapsing them loses exactly the information the Cure's capacity
  budget turned out to need in `064`, where the whole result is conditional on a model.
- **`measured` has no repo word at all** — *true of this corpus, silent about necessity*. The repo's
  `VERIFIED` means machine-checked, which is a different axis entirely. Under universe-scope the
  artifact/necessity distinction is the whole ballgame, and **the register makes it at every single
  assertion while `claims.yaml` cannot make it anywhere.** 71 of 145 entries are `measured`, so
  roughly half the corpus is explicitly not about necessity — which is information the repo would
  have flattened away on import had the register been paraphrased instead of imported verbatim.

Not fixed here. Adding a tier is a registry-wide change and belongs to the owner, not to a cycle
note. Named so it is not lost.

---

## 4. What this leaves open

1. **Derive the operator inventory, or record that it cannot be derived.** This is now the single
   highest-value open item in the program: it is the largest residue on the load-bearing path, and
   `064` made that residue identical to a shortfall in universality. Either direction is a result.
2. **Does the tolerance move the partition, or only the count?** The register asserts only the
   count, at `measured`. If the partition moves, the map is a family indexed by a calibration
   parameter rather than one object — which would bear directly on
   `UNIVERSAL-IS-MATHEMATICAL-SCOPE`, since that claim's object is the classification.
3. **Raise the premise-graph coverage above 15%** by converting prose premises to id references.
   Cheap, mechanical, and it makes the eighth checker worth having.
4. **Publish `close`** in the operator inventory, or state why it is not an operator.
5. **The attested operator set is not closed on its own corpus.** The register records that the
   teardrop ('Life', Angle+Curve fused) is **unreachable** by 23,424 `close()`-expressions over the
   attested operators — *"exactly-one-corner figures do not occur in that space at all."* So the
   inventory is both unforced **and** known-incomplete against figures the corpus itself attests.
   Not pursued here; it is a sharper problem than the orbit count and was found while looking for
   something else.

**Registered:** `MAP-GENERATION-IS-EXACT`, `OPERATOR-INVENTORY-IS-UNFORCED` (the shortfall),
`MARKS-ARE-A-COMPLETE-PRODUCT`, `ORBIT-PARTITION-IS-TOLERANCE-BOUNDED`,
`UNDETERMINED-PREMISE-PROPAGATES`, `ATLAS-CONVENTIONS-ARE-NAMING-ONLY`.
