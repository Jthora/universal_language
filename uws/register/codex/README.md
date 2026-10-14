# uws/register/codex/ — the Cosmic Codex data layer, imported

Imported 2026-10-14 from `https://cosmiccodex.app`. **374 JSON files + `SKILL.md`, every one
hash-verified against the site's own `SHA256SUMS`** (375 OK, 0 mismatched). `SHA256SUMS.json.txt`
here is a local re-hash so a bare clone can re-check without network access.

The i18n HTML (~1300 files, ~24 languages) and the rendered SVG were **not** imported: the HTML is
translation of content already present as data, and the SVG is generated from the same JSON. What is
here is the data layer, which is what can be audited.

---

## Why this is in the repo

`UNIVERSAL-IS-MATHEMATICAL-SCOPE` says the shared object is *the map of all possible symbolic
geometry*. Note `065` imported the register — the **synthesis**, <!--s:register_entries--><!--/s--> claims — and audited it. This is
the **detail** the synthesis summarises, and `067` is why it matters: the correction to `065`'s
headline was sitting in a `derivation` field I had not read. A summary is not an auditable object.

**Wet clay, same as the rest** (owner designation). Nothing here is a claim of this project until it
is registered in `claims.yaml` by a note that did the work.

## What is in it

| path | n | what it is |
|---|---|---|
| `find/` | 94 | **the construction catalog** — one file per construction (`close-angle`, `concatenate-angle-angle`, …). This is the atlas D1 is measured against |
| `slices/` | 187 | the 20-axis slice grids — pairs of axes with everything else held fixed, plus their cell data |
| `charts/` | 44 | **the mathematics** — 22 topics × (chart + cells): `cayley`, `subgroup-lattice`, `fundamental-domain`, `orbit-distance`, `orbit-collapse`, `orbit-dendrogram`, `commutativity`, `chirality-source`, `enantiomorphs`, `fixed-points`, `capacity`, `incidence`, `depth-ladder`, `frame-table`, `insert`, `orbit-map`, `placement-dial`, `point-line-grid`, `schema-join`, `signs-as-waveforms`, `substitution-graph`, `wave-families` |
| `s/` | 19 | per-symbol data (`mark-angle`, …) plus `index.json` |
| `primers/` | 10 | generated teaching sheets with JSON twins — includes `crt` (the ℤ/12 ≅ ℤ/3 × ℤ/4 derivation) |
| `cells/` | 4 | symbol grids; each `<g class="cell">` states its own features in markup |
| `scripts/` | 2 | `writing-systems` comparison data |
| root | 14 | `bundle.json` (the synthesis, also at `../derivation-register.json`), indexes |
| `SKILL.md` | 1 | **the method, not the content** — see below |

## Two items worth reading before the rest

**`SKILL.md`** is the corpus's own method file, and its author's claim that it is *"the part that
transfers to a corpus we did not write"* is justified. It carries the propagation rule this repo
adopted in `065` (`tools/check-derivation-premises.rb`), the status vocabulary, and four working
rules this repo did not have:

- **never trace a glyph** — compute it from the rule; a traced glyph encodes errors the rule catches
- **a shipped artifact must carry its own check** — which is why `SHA256SUMS` ships
- **detectors get seeded both ways before running at scale** — *"every detector in this repo
  over-fired the first time, one by 100×. A large flag count means suspect the checker first."*
- **if you publish a count, draw it** — the orbit count read 122, then 80, then 93; three defects
  each hiding the next, *"all three invisible in the data and visible the moment the orbits were
  drawn side by side"*

**`charts/orbit-distance.json`** is a metric on the orbit space — a 93×93 distance matrix, 36 steps,
max distance 1.4029, plus `nearest_to_named`: which unnamed orbits sit closest to the four that have
attested meanings. Two uses, both open:

1. **It may discharge the Fisher–Rao conditional.** `CAPACITY-RADIUS-TRADEOFF` and
   `DRIFT-DETECTION-REQUIRES-DISCRETENESS` are conditional on a model this project has not adopted.
   This is a native metric on the actual symbol space with a finite diameter, so the
   bounded-separation half of the argument may transfer directly. Not yet attempted.
2. **It is the obvious attack on D1's 89 undetermined orbits.** Not to label them — the register
   forbids that — but to ask what the geometry forces at a shape 0.0794 from the one meaning
   "infinity".

## How to use it honestly

Read the `derivation` and `given` fields, not only `status`. **That is the whole lesson of `F-037`:**
an audit of this register organised by status concluded the opposite of what its derivations say, and
published it as a priority-0 claim. The constraint was one `grep` away in a file already vendored.

Re-derivation belongs to the source tree, not this one — `SKILL.md` names
`python3 tools/ul_cli.py verify` and those tools are not vendored here. Treat these figures as
re-derivable **there** and as imported data **here**.
