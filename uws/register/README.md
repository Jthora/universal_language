# uws/register/ — the imported derivation register

**`derivation-register.json`** — the companion site's derived layer, imported verbatim
2026-10-14 from `https://cosmiccodex.app/bundle.json`.

`sha256` (first 16): `135b523a670241a0` · schema `1.0.0` · 145 entries, 6 marks, 6 operators,
24 terms, one combination space.

---

## Why it is here

**The universality claim is about an object this repo did not contain.**

`UNIVERSAL-IS-MATHEMATICAL-SCOPE` says the shared thing is *the map of all possible symbolic
geometry* — the classification, not any chart drawn on it. The repo held UWS, which is one chart,
and a claim about the map. **The map was on the website.** A claim whose object lives outside the
registry cannot be audited, and auditing it is now the program's cheapest falsifier
(`LEDGER-CLOSURE-IS-UNIVERSE-SCOPE`). So the object is imported, and note `065` is the audit.

Imported **verbatim and hashed** rather than paraphrased. Paraphrasing a register defeats its
purpose: the statuses are the content.

## What it is, and what it is not

**It is wet clay** — the owner's designation for the companion corpus, and it holds here. This
register is *draft input under the same tiering discipline as any source*, not an authority. It is
also genuinely good work, which is why it was imported rather than summarized: withdrawn entries
explain the bug that killed them, conventional entries say why no derivation is available, and the
undetermined entries state what would settle them including, in one case, the author's own
admission that they cannot supply it — *"anything I group, I group having written the labels."*

**It is not a second claim registry.** `claims.yaml` remains the only place a claim of this project
lives. Nothing in this file is a claim of this repo until it is registered there, with a repo tier,
by a note that did the work. Note `065` registers what the audit established and nothing else.

## Status vocabulary, and how it maps to repo tiers

The register's vocabulary is **finer than this repo's on the conditional axis** and that is why it
was worth adopting rather than flattening.

| register status | n | means | nearest repo tier | why the fit is imperfect |
|---|---|---|---|---|
| `derived` | 39 | assert freely; derivation given | `ARGUED` | the register's derivations are written, not machine-checked |
| `derived_given` | 25 | **assert only together with its `given` premise** | — | **no repo equivalent.** The closest is `ARGUED` + `scope:`, but scope is a caveat on a standing claim, not a premise the claim travels with |
| `measured` | 71 | a fact about *these artifacts*, **not necessity** | `VERIFIED` about an artifact | the repo's `VERIFIED` means machine-checked; `measured` means measured off a specific drawn corpus. **Half the register is this, and by its own vocabulary it does not bear on universality** |
| `conventional` | 3 | assert as a choice, never as a result | `DESIGN-CHOICE` | clean fit |
| `undetermined` | 2 | **use, do not assert**; `success_condition` says what would settle it | `CONJECTURED` | clean fit; `success_condition` ≈ `falsified_by` inverted |
| `withdrawn` | 5 | do not assert; retained so citations resolve | `RETIRED` | clean fit |

**The two gaps worth naming.** `derived_given` has no repo equivalent and should get one — a
conditional derivation is not the same object as a scoped claim, and collapsing them loses exactly
the information the Cure's budget needed. And `measured` is a status this repo lacks a word for:
*true of this corpus, silent about necessity.* Under universe-scope that distinction is the whole
ballgame, and the register makes it at every assertion while `claims.yaml` does not.

## The one rule adopted from it

> **A derivation routed through an undetermined premise is itself undetermined.**

The register's own `SKILL.md` calls this *"the part that transfers to a corpus we did not write"*,
and it is right. This repo had nothing equivalent: `scope:` + `scope_reviewed:` is a **local,
per-node** acknowledgement, and `FAILURES.md` **F-035** records what per-node checking costs — a
perimeter retreats one locally-defensible step at a time with nothing summing the steps. Forward
propagation of uncertainty is the shape of check that cannot be defeated that way.

It is adopted, and **`tools/check-derivation-premises.rb` is what executes it** — because the
register states the rule in prose, and prose does not execute (note `048`).

Running it: **no violations in the traceable portion, and the traceable portion is 15%.** The
register's premises are mostly free text, so 13 of its 25 `derived_given` entries are invisible to
the rule its own author wrote. The checker therefore reports coverage as part of its result: *a
rule that passes by being blind has not passed.*

## Re-deriving it

The register is generated from sources it names, with their own hashes (`sources` key). Its
verification commands belong to the site's tree, not this one:

```
python3 tools/verify_symbology_grammar.py    # 12/12 off the sheets
python3 tools/symbology_glyph.py --check     # glyphs read back as their rule
python3 tools/symbol_combinations.py --closure
```

Those are **not** vendored here and are **not** run by this repo's CI. What this repo runs against
the register is `tools/check-derivation-premises.rb` and the audit in
`research/notes/065-atlas-audit/`. Treat the site's figures as re-derivable **there** and as
imported data **here**.
