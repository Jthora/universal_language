# 071 — The derivation loop, run: one pair derived, two blocked on the same missing instrument

**Type:** cycle
**Opened:** 2026-10-14
**Status:** closed
**Thread:** the first run of the loop proposed in conversation after an audit showed D1 had moved
zero all session. Uses `069`'s Aut(map) and `070`'s crossing ruling as instruments.
**Prompted by:** the owner — *"how do we get there; and are we making any progress... that How
will help re-anchor this project direction."*

---

## 0. The audit that preceded this

Seven notes, ~30 claims, three checkers, two mathematical results, a 374-file import — and
**`atlas_recorded` read 4 of 93 at the end of the session exactly as it had when the bar was
installed.** The apparatus improved and the deliverable did not move. This note is the first
attempt at the deliverable.

## 1. The loop

Per orbit: encode the figure as a map · compute its invariants · fetch its nearest **named**
neighbours from the Codex's `orbit-distance` matrix · isolate the minimal invariant that separates
them · ask **is that invariant a property the figure *is*, or one it *depicts*?**

The five existing lexicon entries all answer that question the same way — Jordan's theorem for
containment, face-sharing for adjacency, Euler for enclosure counts. The meaning *is* the
mathematical content, which is why it needs no convention (`GROUNDING-BY-SELF-DENOTATION`, tiers
0–1).

## 2. Run against all four named orbits

| named orbit | nearest unnamed | distance | minimal distinguishing invariant | verdict |
|---|---|---|---|---|
| 13 `infinity` | **17** | 0.0794 | **vertex count** (1 vs 2), hence planar Euler (2 vs 3) | **DERIVED** |
| 39 `Chaos pair` | 40 | 0.0771 | **chirality** (`mirror_axes` 1 vs 0) | blocked |
| 91 `+` | 92 | 0.3594 | **chirality** (`mirror_axes` 4 vs 0, both `symmetry` 4) | blocked |
| 22 `Sun-in-Circle` | 65 | 0.3962 | **continuous symmetry** (`inf` vs 1) | out of scope |

**One of four derivable with the instruments that exist. Two blocked on the same one. One
permanently out of reach.** That is the loop's real output, and it is more useful than the single
derivation, because it says what to build next.

- **Blocked, both cases, on the same thing:** the distinguishing invariant is chirality, and `070`
  established that placement handedness is **invisible to the combinatorial map** — orbits 39/40 and
  91/92 are `superpose_displaced` and `cross`, where the handedness comes from placement and from
  the constituents' asymmetry. The map is the wrong instrument here. **The missing one is the
  placement plane** (the Codex's `placement-dial`), not more map work.
- **Out of scope permanently:** orbit 22 has `symmetry: inf`. A map is discrete
  (`MAP-SYMMETRY-BOUNDS-DRAWN-SYMMETRY`'s scope). No amount of tooling reaches it.

## 3. The derivation: orbits 13 and 17

Under `070`'s ruling the lemniscate is **one** map — a degree-4 vertex with two loops, the lobes
*not* interleaved in the rotation. Two circles side by side are **two** components.

```
                      orbit 13 (∞)     orbit 17 (two circles)
  vertices                   1                  2
  edges                      2                  2
  planar faces               3                  3      ← identical
  same_face / different_face  [[0,2]] / [[1,3]]  identical
  planar Euler               2                  3      ← the difference
  Aut⁺ / Aut⁻              2 / 2              8 / 8
```

**Both figures have two bounded regions, and every face relation agrees.** What separates them is
whether the two region boundaries **meet**: in ∞ they share the single vertex, in two circles they
share nothing. The whole distinction is carried by vertex count.

**Is it a property the figure *is*?** Yes. The lobes do meet at the crossing, and under `070` that
vertex is read off the ink rather than declared. So this qualifies at the same tier as the existing
five.

**And it is a NEW relation, not an instance of an old one.** Corpus `007` ADJACENCY is two regions
sharing a whole **edge** (RCC-8's EC). Orbit 13's lobes share a **point and no edge** — weaker
contact than `007`, stronger than disconnection. Three distinct relations, and the repo held a
verified entry for only the middle one.

Shipped as corpus entries `011-point-contact` and `012-no-contact`, both VERIFIED, both exercised
by the harness (confirmed by corrupting `011`'s vertex count and watching the harness name it).

**What is NOT claimed:** that ∞ *means* infinity. That label is the Codex's attestation and stays
conventional. What is derived is the structural relation the figure instantiates — exactly the split
`LEXICON-V1`'s own scope makes, where VERIFIED covers the structural side and the reader-facing
meaning is delegated to M1.

## 4. Two things the instruments caught

**The tool rejected my own encoding.** My first rotation for the lemniscate interleaved the lobes;
`genus()` returned **1** — a torus, not a page. Going counter-clockwise around a drawn crossing the
branches are up-left, down-left, down-right, up-right, so each lobe's ends are *adjacent*. The
interleaved order is a different surface. **An invariant computer earns its keep by refusing a
plausible model**, and this is the first time in the session that a tool corrected me rather than
the reverse.

**The bridge held, twice more.** Orbit 13: map gives Aut⁺ 2, Aut⁻ 2 against the Codex's measured
`symmetry: 2`, `mirror_axes: 2` — **equality**. Orbit 17: map gives 8/8 against measured 2/2 —
**strict bound**, because the map does not know the two circles are placed in a particular
arrangement. Both consistent with `MAP-SYMMETRY-BOUNDS-DRAWN-SYMMETRY`; the second is another
instance of placement being invisible.

## 5. D1 now has a repo-side number

`atlas_recorded` counts orbits with an **attested** meaning in the imported register. That is
theirs, and nothing done here can move it. So `tools/stats.rb` now also reports **`derived_orbits`**
— orbits this repo has given machine-checked structural content, counted from corpus entries'
`orbit` field.

**It reads 2 of 93.** Low, and it is the first number in this project's history that moves when
derivation work is done. The two are reported separately and never summed: an attested label is not
a derivation, and a derivation is not an attested label.

## 6. What this says about direction

The method works, at a rate of roughly one derivation per nearest-neighbour pair examined, and the
binding constraint is **not** the method — it is that three of four pairs need instruments the map
does not provide. So:

1. **Build the placement plane.** Two of the four blocked cases turn on it, and the Codex already
   has `placement-dial` data. This is the next instrument, and `069`'s Aut(map) is not it.
2. **Continue the loop past the nearest neighbours.** Only 4 of 93 orbits have named neighbours at
   all; the other 85 need pairing against each other, which the distance matrix supports.
3. **Turning number is absent from both corpora**, and it is the classical invariant that separates
   a lemniscate from two circles in the *metric* category (0 against ±1 each). It is not needed for
   §3's derivation, which runs on vertex count, but it is a real gap in the feature vector and would
   need crossing signs that neither corpus records.
