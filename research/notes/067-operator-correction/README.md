# 067 — Correction: the operator inventory has a derived constraint, and I missed it by reading statuses instead of derivations

**Type:** correction
**Opened:** 2026-10-14
**Status:** closed
**Thread:** corrects `065`'s headline finding. Prompted by the owner pointing out that the Cosmic
Codex maps far more than was imported — the correction was in the file `065` already had.

---

## 1. What I got wrong

`065` concluded, as its headline and as a priority-0 claim:

> *"The map's generation is exact. Its generators are not derived."* … *"all operators `measured`
> (attested on sheets); **no derivation recorded for the set**"*

**The last clause is false.** The register contains `attested-operators-are-frame-free`, tiered
**`derived`**:

> *"Every operator attested in the corpus reads a quantity that is unchanged when the reader turns
> the whole canvas."* Derivation: each relation reduced to the scalar it reads, measured before and
> after a global rotation at eight angles over four figure pairs — IN/OUT, CENTER/PERIPHERY,
> NEAR/FAR, CONTACT and LINK drift by at most **1.3e-15**, machine zero.

And its companion `directional-schemas-need-a-declared-frame` (also `derived`) explains the
inventory's *boundary*: UP/DOWN, FRONT/BACK and PATH read frame-bound quantities — drifts of **2.20**
and **4.40** under the same rotation — so they cannot have frame-free operators. Two more derived
entries sharpen it: `an-anchor-s-symmetry-bounds-the-bearing` (bearing is bounded by the anchor
mark's symmetry: C∞ anchors no direction, C2 one up to a half-turn, C1 completely) and
`direction-is-read-as-contrast` (no meaning rests on which way is up).

**So the operator set is not a bare list. It is closed under reorientation, and the relational
schemas it omits are exactly the ones that provably cannot be closed that way.** That is a derived
necessary condition on the inventory, and it was in the file `065` imported.

## 2. How I missed it, which is the part worth keeping

I audited the register's **`status` fields** and never read its **`derivation` fields.** `audit.py`
prints each operator's status and attestation; every operator's status is `measured`, and I treated
that as the whole answer. The constraint lives one level down, in the derivation text of claims whose
*subject* is the schemas rather than the operators — so an audit organised by object missed a
constraint stated about a relation.

**This is an R1 violation, mine.** `claims.yaml`'s negative-result discipline opens with it: *"Every
negative finding requires at least one ADVERSARIAL SEARCH against it before it is recorded
("criticisms of X", "scope of X", "X superseded", "alternatives to X"). Always, not when it seems
warranted — that judgment is the compromised one."* I recorded "no derivation recorded for the set"
without once searching the register for a derivation about operators. `RESEARCH-PROTOCOL.md`'s
documented failure mode is believing a result that appears to settle a question and closing the line
of work, with the counter-evidence one search away. The counter-evidence was one `grep` away, in a
file I had already vendored and hashed.

Logged as **F-037**.

## 3. What survives, and what the residue actually is

The conclusion survives; the reasoning was incomplete and the residue is smaller than stated.

| `065` said | Corrected |
|---|---|
| every operator's status is `measured` | **true**, verified, unchanged |
| the 191 constructions are exact given the generators | **true**, unchanged |
| **no derivation recorded for the set** | **false** — frame-freeness is derived, and the gaps are derived |
| the classification ledger line does not close | **still true** — see below |
| "the largest free parameter on the load-bearing path" | **overstated** — narrowed |

**Why the ledger line still does not close.** Frame-freeness is **necessary, not sufficient.** It
says every attested operator is reorientation-invariant and that the omitted schemas could not be.
It does not say why *these* frame-free relations rather than other possible ones, and it says nothing
about their semantics — each operator's own note still reads *"attested on a sheet; its semantics are
not thereby derived."* The register's own `primitive-completeness` entry carries the same shape of
caveat: completeness holds *with respect to symmetry order*, which it states is **necessary, not
sufficient.**

So the open item changes from *"derive the operators, from nothing"* to something much better posed:

> **Is frame-freeness, plus the symmetry-order bound on bearings, sufficient to force the attested
> operator set — or is there a frame-free relation the corpus does not attest?**

That is a **search over a characterised space** rather than an open-ended derivation, and the second
half is a cheap falsifier: exhibit one reorientation-invariant relation on two figures that is not in
the attested set. If one exists, the inventory is genuinely underdetermined. If a search finds none,
the inventory is forced up to the stated conditions and D4 of the success condition gains real
support.

## 4. Two further corrections the same read produced

**A count I would have published wrongly.** `SKILL.md`: *"The sign inventory is 95 if mirroring means
nothing and 109 if it does — and `mirroring-is-meaningful` is undetermined, so **quote the pair, not
one number**."* The register's own count discipline is to publish an interval when the premise is
unsettled. `COUNTS-ARE-GENERATED` enforces that stated counts are generated; it does not yet enforce
that a count resting on an undetermined premise is published as a pair. Named, not built.

**The 93 is the third answer, not the first.** *"That count read 122 until F101, and 80 until F105.
Three defects, each hiding the next: a bucket key that saw sampling density, which the equality test
ignores; a tolerance loosened to cover a rotation search too coarse to align anything; and stroke
boundaries discarded by flattening. All three were invisible in the data and visible the moment the
orbits were drawn side by side."* Their rule: **if you publish a count, draw it.**
`ATLAS-COMPLETENESS-IS-THE-PROGRESS-BAR` publishes 4/93 and draws nothing. The denominator has moved
twice for reasons no amount of staring at the numbers would have caught.

## 5. What changes

- `OPERATOR-INVENTORY-IS-UNFORCED` — **scope narrowed and restated**; the false clause removed, the
  derived constraint recorded, the open question re-posed as a bounded search.
- `OPERATORS-ARE-CLOSED-UNDER-REORIENTATION` — registered (ARGUED), the constraint itself.
- **F-037** — the R1 violation.
- `ATLAS-COMPLETENESS-IS-THE-PROGRESS-BAR` — caveat added: the denominator has moved twice, and the
  corpus's own rule is to draw a count before publishing it.

**Open, carried forward:** the bounded search in §3 · a pair-publishing rule for counts resting on
undetermined premises (§4) · drawing the orbit map before citing 93 again (§4).
