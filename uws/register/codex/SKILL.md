---
name: universal-symbology
description: Read, draw, and check Universal Symbology (the Cosmic Codex) — the 6 marks, the 12 signs, the operators, and the derivation-status discipline that says which of it you may assert. Use when working with UL/Universal Symbology, the Cosmic Codex, symbol taxonomy, or any corpus where you need to separate what was derived from what was merely stated.
allowed-tools: Bash(python3 tools/ul_cli.py:*) Bash(python3 tools/*.py:*) Read Grep
---

# Universal Symbology

A notation whose meanings are computed from geometry rather than assigned by
authority. That property is the point: you can re-derive it, so you do not have
to trust whoever handed it to you — including this file.

## Start here — the rule that outlives the content

**A derivation routed through an undetermined premise is itself undetermined.**

It propagates. One soft premise anywhere in the chain makes the conclusion soft,
however rigorous every other step was. This is the whole method; the symbols are
just where it was applied. It works on any corpus, including one built to
deceive you.

So every claim carries a **status**, and a status is a **licence to cite**:

| status | what you may do with it |
|---|---|
| `derived` | assert freely; the derivation is attached |
| `derived_given` | assert **only together with** its stated premise |
| `measured` | assert as a fact about *these artifacts*, never as a necessity |
| `undetermined` | **use, do not assert**; it carries what would settle it |
| `conventional` | assert as a choice, never as a result |
| `withdrawn` | do not assert; kept so old citations reach the retraction |

The invariant: *the status travels at the assertion, not in a footnote.*
An `undetermined` claim restated without its status is a fabrication, even when
it later turns out true.

## The tool

```
python3 tools/ul_cli.py glyph 11 [--svg]   draw a sign from its rule
python3 tools/ul_cli.py mark Wave          what a mark is, and how that was fixed
python3 tools/ul_cli.py claim <id>         status, statement, premises
python3 tools/ul_cli.py find --closed      search the orbits by measured shape
python3 tools/ul_cli.py open               everything unsettled, + what would settle it
python3 tools/ul_cli.py verify             re-derive it; do not trust the above
```

`ul_cli.py glyph 11` prints, for Aquarius:

```
  geometry  n mod 3 = square   (Fixed)   [derived]
  direction n mod 2 = up                 [derived]
  crossbar  n mod 4 in (2,3) = True      [derived]
  element   n mod 4 = Air
  which element sits in which cell is UNDETERMINED
```

Note the last line. Three quarters of that glyph is derived and one quarter is
not, and the tool says so at the point of use. Copy that habit.

## What is actually derived

**The signs.** `sign = (n mod 3) × (n mod 4)`. Since gcd(3,4)=1, the Chinese
Remainder Theorem gives `Z/12 ≅ Z/3 × Z/4` — the twelve signs are not a list to
memorise, they are one grid. `n mod 3` selects the geometry (arc / angle /
square, i.e. the modalities); `n mod 4` selects the element, as a direction bit
(up / down) crossed with a crossbar bit. Verified three ways against the drawn
sheets, plus one held-out prediction.

**The marks.** Six marks = 3 symmetry classes × 2 curvature classes,
`Z/6 ≅ Z/3 × Z/2`, every cell filled exactly once.
- *symmetry class* = the order of the mark's rotational symmetry group
  (∞ for Point and Circle, 2 for Line and Wave, 1 for Angle and Curve).
  This is the corpus's "rotation trichotomy", measured rather than asserted.
- *curvature class* = zero-or-singular vs smooth. This is the corpus's
  "masculine/feminine" axis. The old names carry connotations the geometry does
  not; prefer the measured name.

A sine is odd, so a half-turn returns it — which is why Wave groups with Line
and not with Curve. If that surprised you, that is the measurement doing work
the intuition would have got wrong.

**What a transformation can say.** A transformation distinguishes two states of a
figure exactly when it is **not in that figure's stabiliser** — so the symmetry
group is not just a label, it is the operator's capacity on that mark.

```
Point  O(2)   a turn says nothing        Wave   C2   a turn says an angle, blind
Circle O(2)   a turn says nothing                    to which end; a MIRROR says
Line   D2     a turn says an angle,                  something — the only mark
              blind to which end                     for which it does
Angle  D1     a turn says everything
Curve  D1     a turn says everything
```

Two consequences worth carrying. **A Line cannot signal direction** — it is C2, a
half-turn returns it, so "from" and "toward" are the same drawn mark; only Angle
and Curve can carry a direction alone. And **exactly one mark is chiral**, the
Wave, so on five of six marks reflection conveys nothing at all.

Handedness enters through *placement*, not through mirroring: **A-beside-B and
B-beside-A are enantiomorphs**. That makes "composition is non-commutative" and
"chirality is real" the same claim. The sign inventory is 95 if mirroring means
nothing and 109 if it does — and `mirroring-is-meaningful` is undetermined, so
quote the pair, not one number.

**The grammar is frame-free, and that is why it has gaps.** Every attested
operator reads a quantity unchanged when the reader turns the whole canvas —
five of them to machine zero. The relational schemas with no operator are
exactly the ones that cannot be: UP/DOWN drifts 2.20, FRONT/BACK 4.40. So the
inventory is *closed under reorientation*, and the three directional schemas are
sayable only by declaring the frame. Watch for this when you phrase things:
"B is the flip of A about the horizontal" is frame-bound, "these two are mirror
twins" is not. **Naming an axis imports a frame you may not have.**

**Primitive completeness** holds *with respect to symmetry order*: a mark is
primitive when its symmetry order cannot be produced by composing others. That
is necessary, not sufficient — say the scope when you cite it.

## What is not settled

Run `ul_cli.py open`. 2 claims, each with a success condition. The ones most
likely to trip you:

- **which element sits in which cell** — the 2×2 is derived, the labelling is
  not. Fire/Air/Water/Earth is inherited, not computed.
- **up/down is settled, and the claim that once held this slot is WITHDRAWN.**
  `up-is-shared-across-canvases` is retracted: the statement was false, and
  `orientation-is-a-canvas-parameter` already said so. A plane has no privileged
  up, but no symbol is drawn on a bare plane — a canvas is a plane **plus a
  frame**, and the frame supplies one. SVG pins it (+y downward); gravity pins it
  physically, and the anchor recurses without ending. So *whether* there is an up
  is settled: `updown-is-meaningful` is **derived_given**
  `orientation-is-a-canvas-parameter`, and you must cite it with that premise,
  never bare. Whether up is *shared* is settled too, in the other direction:
  `direction-is-read-as-contrast` is **derived** — no meaning in this notation
  rests on which way is up, because every meaning-bearing use of the direction
  bit is a relation between marks on one canvas, and that survives any
  reorientation. Do not cite the withdrawn id.
- **89 of 93 orbits are unnamed.** The combination space was enumerated
  geometrically; only 4 have attested meanings. An unnamed orbit is a shape, not
  a word. Search them with `ul_cli.py find`; see them all at once in
  `build/codex/charts/orbit-map.svg`.

  That count read 122 until F101, and 80 until F105. Three defects, each hiding the next: a bucket
  key that saw sampling density, which the equality test ignores; a tolerance
  loosened to cover a rotation search too coarse to align anything; and stroke
  boundaries discarded by flattening. **All three were invisible in the data and
  visible the moment the orbits were drawn side by side.** If you publish a
  count, draw it.

## How to check it

```
python3 tools/ul_cli.py verify
```

Two independent paths: the grammar measured off the original drawn sheets
(Base 12 12/12, Base 36 12/12, families 24/24), and glyphs rendered then read back
blind — the renderer has to agree with a reader that never saw the rule. A
negative control perturbs the renderer only; if both sides move together the
test is fake and proves nothing. That mistake was made here once.

## Working rules

- **Never trace a glyph.** Compute it from the rule (`symbology_glyph.py`).
  A traced glyph can encode an error the rule would have caught.
- **A shipped artifact must carry its own check.** `build/codex/` ships
  `SHA256SUMS` and its verifiers, so a bare clone can refute it.
- **Detectors get seeded both ways before they run at scale.** Every detector in
  this repo over-fired the first time — one by 100×. A large flag count means
  suspect the checker first.
- **Writing systems are compared to each other, not to UL.** UL is not phonetic,
  and the vocalization bridge that would license the comparison does not exist
  yet (`claim ul-is-not-phonetic`).

## Where things are

| | |
|---|---|
| the whole system, one fetch | `build/codex/bundle.json` |
| landing, no JavaScript | `build/codex/index.html` |
| symbols as parseable SVG | `build/codex/cells/*.svg` — `<title>`, `<desc>`, `data-*` |
| claims of record | `data/derivation/derivation_register.json` |
| the reading log | `reference/NOTES-symbology-reading.md` |

Cells are readable **without a renderer**: every `<g class="cell">` states its
own features in markup. If you cannot decode pixels, you can still read every
symbol here.
