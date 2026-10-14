# examrun-01 — reader's report

Method used: `METHOD.md` only. `alpha(d) = d XOR 1`; `sigma` from `rotations`; vertices = orbits of
`sigma`; faces = orbits of `phi(d) = sigma(alpha(d))`; `edges = n_darts/2`; `chi = V - E + F`;
`g = (2 - chi)/2` for connected maps only; `planar_faces` = face count after the nesting merges.

I wrote two scripts (`work/derive.py`, `work/aux.py`, Python 3 stdlib only) and also traced the
small orbits by hand before running them. The two agree everywhere. Every orbit quoted below was
traced dart by dart.

**Verdict: 3 of the 12 entries have a wrong published value. The other 9 are correct in every
field I can check.** Two of the three wrong values are contradicted by their own entry's prose.

---

## DISCREPANCIES

### 1. `001-triangle` — `faces`: published **3**, derived **2**

`n_darts = 6`. `rotations`: `v0:[0,5] v1:[2,1] v2:[4,3]`, so

```
sigma: 0->5  5->0 | 2->1  1->2 | 4->3  3->4
alpha: 0<->1  2<->3  4<->5
phi(d) = sigma(alpha(d))
```

Face tracing, dart by dart:

```
start 0:  phi(0) = sigma(alpha(0)) = sigma(1) = 2
          phi(2) = sigma(alpha(2)) = sigma(3) = 4
          phi(4) = sigma(alpha(4)) = sigma(5) = 0   -> closes
          face A = {0, 2, 4}            (length 3)

start 1:  phi(1) = sigma(alpha(1)) = sigma(0) = 5
          phi(5) = sigma(alpha(5)) = sigma(4) = 3
          phi(3) = sigma(alpha(3)) = sigma(2) = 1   -> closes
          face B = {1, 5, 3}            (length 3)
```

All 6 darts are used; 3 + 3 = 6. There is no third orbit — a third face would need darts, and
there are none left. **Derived `faces = 2`.**

Three independent reasons the published 3 is wrong, not a convention difference:

- **Dart accounting.** Face orbits partition the dart set. 6 darts in orbits of length 3 and 3
  leaves nothing for a third orbit.
- **It is internally impossible.** With `faces = 3`, `chi = 3 - 3 + 3 = 3` and
  `g = (2 - 3)/2 = -0.5`. That is not an integer, so no surface realises it — yet the same
  `expected` block publishes `genus: 0`, which requires `chi = 2`, which requires `faces = 2`.
  The published `faces` and the published `genus` cannot both be true.
- **The entry's own prose says 2.** The `description` reads "Jordan: it bounds exactly two
  regions," and the `provenance` string is `map.rs::tests::triangle_has_two_faces`.

Everything else in 001 is correct: `vertices 3`, `edges 3`, `components 1`,
`degree_sequence [2,2,2]`, `genus 0`.

### 2. `002-hexagon` — `genus`: published **1**, derived **0**

`n_darts = 12`. `rotations`: `v0:[0,11] v1:[2,1] v2:[4,3] v3:[6,5] v4:[8,7] v5:[10,9]`.

```
sigma: 0->11 11->0 | 2->1 1->2 | 4->3 3->4 | 6->5 5->6 | 8->7 7->8 | 10->9 9->10
alpha: 0<->1 2<->3 4<->5 6<->7 8<->9 10<->11
```

Face tracing:

```
start 0:  phi(0)  = sigma(1)  = 2
          phi(2)  = sigma(3)  = 4
          phi(4)  = sigma(5)  = 6
          phi(6)  = sigma(7)  = 8
          phi(8)  = sigma(9)  = 10
          phi(10) = sigma(11) = 0    -> closes
          face A = {0, 2, 4, 6, 8, 10}   (length 6)

start 1:  phi(1)  = sigma(0)  = 11
          phi(11) = sigma(10) = 9
          phi(9)  = sigma(8)  = 7
          phi(7)  = sigma(6)  = 5
          phi(5)  = sigma(4)  = 3
          phi(3)  = sigma(2)  = 1    -> closes
          face B = {1, 11, 9, 7, 5, 3}   (length 6)
```

`V = 6`, `E = 12/2 = 6`, `F = 2`, connected (one component — from dart 0, `alpha` and `sigma`
steps reach all 12).

```
chi = 6 - 6 + 2 = 2
g   = (2 - chi)/2 = (2 - 2)/2 = 0
```

**Derived `genus = 0`.** Published `genus: 1` would need `chi = 2 - 2*1 = 0`, i.e. `F = 0` with
`V = E = 6`. A map with darts always has at least one face orbit, so `F = 0` is impossible; the
published genus is not reachable from any face count whatsoever for this `V` and `E`.

Again contradicted by its own entry: the `description` says "Same topology as the triangle" (whose
published genus is 0), and the `provenance` is
`map.rs::tests::cycles_satisfy_euler_and_are_genus_zero` — *genus zero*.

Everything else in 002 is correct: `vertices 6`, `edges 6`, `faces 2`, `components 1`,
`degree_sequence [2,2,2,2,2,2]`.

### 3. `008-junction` — `degree_sequence`: published **[1,1,2,2]**, derived **[1,1,1,3]**

`n_darts = 6`. `rotations`: `v0:[0,2,4] v1:[1] v2:[3] v3:[5]`.

`sigma` orbits are read straight off the rotation system, no tracing needed:

```
v0 = [0,2,4]  ->  sigma(0)=2, sigma(2)=4, sigma(4)=0   -> orbit {0,2,4}, size 3
v1 = [1]      ->  sigma(1)=1                           -> orbit {1},     size 1
v2 = [3]      ->  sigma(3)=3                           -> orbit {3},     size 1
v3 = [5]      ->  sigma(5)=5                           -> orbit {5},     size 1
```

Degrees are the orbit sizes: `3, 1, 1, 1`. Sorted ascending: **`[1, 1, 1, 3]`**. Published
`[1,1,2,2]` asserts two vertices of degree 2, but `rotations` contains no 2-cycle at all — only
one 3-cycle and three fixed points.

Note the published value is not random: `[1,1,2,2]` is exactly the degree sequence of a **path of
three edges**, a different figure that agrees with 008 on every other published field. I checked
this:

```
Y (star, as 008's rotations give): V=4 E=3 F=1 chi=2 genus=0  degree_sequence=[1,1,1,3]
path of 3 edges:                   V=4 E=3 F=1 chi=2 genus=0  degree_sequence=[1,1,2,2]
```

So `degree_sequence` is the *only* published field in 008 that distinguishes a Y from a path — and
it is the one that is wrong, in an entry whose whole stated lesson is junction degree. Its own
`teaches` field says "this is degree 3, with degree 1 at each terminus", i.e. `[1,1,1,3]`, directly
contradicting its own `expected`. (The handshake check does not catch this: `1+1+2+2 = 6 = n_darts`,
and the length is 4 = published `vertices`. So the wrong value is arithmetically plausible and only
the rotation system refutes it.)

Everything else in 008 is correct: `vertices 4`, `edges 3`, `faces 1` (single orbit
`{0,1,2,3,4,5}`: `phi(0)=sigma(1)=1, phi(1)=sigma(0)=2, phi(2)=sigma(3)=3, phi(3)=sigma(2)=4,
phi(4)=sigma(5)=5, phi(5)=sigma(4)=0`), `genus 0`, `components 1`.

---

## AGREEMENTS

Entries **003, 004, 005, 006, 007, 009, 010, 011, 012** agree with my derivation in **every**
published field, including every `same_face` / `different_face` pair. Field-by-field confirmations:

| entry | fields confirmed |
|---|---|
| 003 | V 6, E 6, faces 4, components 2, genus `null`, degseq [2×6] |
| 004 | V 6, E 6, components 2, planar_faces 3, euler_planar 3, degseq [2×6] |
| 005 | V 5, E 4, components 2, planar_faces 2, euler_planar 3, degseq [1,1,2,2,2], same_face (0,6), different_face (1,6) |
| 006 | V 5, E 4, components 2, planar_faces 2, euler_planar 3, degseq [1,1,2,2,2], same_face (1,6), different_face (0,6) |
| 007 | V 4, E 5, faces 3, genus 0, components 1, degseq [2,2,3,3], same_face (0,2), different_face (0,1) and (1,3) |
| 009 | V 6, E 3, components 3, planar_faces 1, euler_planar 4, degseq [1×6], same_face (0,2) and (2,4) |
| 010 | V 8, E 7, faces 5, components 3, genus `null`, planar_faces 3, euler_planar 4, degseq [1,1,2,2,2,2,2,2], same_face (0,6) and (1,9), all five different_face pairs |
| 011 | V 1, E 2, faces 3, genus 0, components 1, degseq [4], same_face (0,2), different_face (1,3) |
| 012 | V 2, E 2, faces 4, genus `null`, components 2, degseq [2,2], planar_faces 3, euler_planar 3, same_face (0,2), different_face (1,3) |

### Published values that struck me as odd and which I derived as **correct** — flagging them as correct, not as problems

- **003 `faces: 4` for two triangles.** My first instinct was 3, the planar answer. It is 4, and
  that is the point of the entry. Component 1 gives `{0,2,4}` and `{1,5,3}`; component 2 gives
  `phi(6)=sigma(7)=8, phi(8)=sigma(9)=10, phi(10)=sigma(11)=6` → `{6,8,10}`, and
  `phi(7)=sigma(6)=11, phi(11)=sigma(10)=9, phi(9)=sigma(8)=7` → `{7,11,9}`. Four orbits,
  `chi = 6-6+4 = 4 = 2c`. The rotation system alone really does put each component on its own
  sphere. Correct.
- **012 `faces: 4` from only 2 edges.** Odd-looking but right. Each circle is a *loop at a single
  vertex*: `circle_a = [0,1]` with `alpha: 0<->1`. Then `phi(0) = sigma(1) = 0` → face `{0}`, and
  `phi(1) = sigma(0) = 1` → face `{1}`. A one-dart face orbit is legitimate. Two such components
  give 4 faces, `chi = 2-2+4 = 4 = 2c`. Correct.
- **012 `degree_sequence: [2,2]`.** A loop contributes *both* of its darts to its vertex, so each
  circle's single vertex has degree 2, not 1. Correct.
- **011 `vertices: 1`, `degree_sequence: [4]`, `faces: 3` from 4 darts.** Face orbits are
  `{0,2}` (length 2), `{1}`, `{3}`. `phi(0)=sigma(1)=2`, `phi(2)=sigma(3)=0`; `phi(1)=sigma(0)=1`;
  `phi(3)=sigma(2)=3`. Face degrees (2,1,1) sum to 4 = `n_darts`. `chi = 1-2+3 = 2`, genus 0.
  Correct, and it is exactly the planar figure-eight: two one-edge lobes plus a two-dart outer walk.
- **005/006 `euler_planar: 3` while `planar_faces: 2`.** `5 - 4 + 2 = 3`. Correct; it looked wrong
  to me until I noticed `V - E = 1` here, not 0.
- **009 `planar_faces: 1`.** No bounded region at all — three disjoint single strokes enclose
  nothing, so the plane is one region. `euler_planar = 6 - 3 + 1 = 4 = 1 + c`. Correct.
- **007 face degrees (3,3,4).** The outer walk `{3,6,8,5}` has length 4 while the two bounded
  faces have length 3. Correct for two triangles glued on an edge: the outer boundary is the
  4-cycle v0–v2–v1–v3. And `3+3+4 = 10 = n_darts`.
- **010 `faces: 5` with `genus: null` and `planar_faces: 3`.** Raw orbits `{0,2,4}`, `{1,5,3}`,
  `{6,7}`, `{8,10,12}`, `{9,13,11}` → 5; `chi = 8-7+5 = 6 = 2c` with `c = 3`, so the genus formula
  correctly does not apply. After the two merges, 3. Correct.
- **012 `faces: 4` *and* `planar_faces: 3` in the same block, while 011 publishes only `faces: 3`.**
  Consistent: 011 is connected, so it needs no merge and its raw count is already its planar count.

### Two agreements that required me to supply something `METHOD.md` does not define

**`nesting.placements` is undefined in `METHOD.md` but is load-bearing for 005 and 010.**
`METHOD.md` describes only merging the faces named by `top_level_outer_darts`. Taken literally:

- 005 has `top_level_outer_darts: [1]` — a single dart, so *nothing* merges, giving
  `planar_faces = 3` and `euler_planar = 4`, against the published 2 and 3.
- 010 has `top_level_outer_darts: [1,9]` — one merge, giving `planar_faces = 4` and
  `euler_planar = 5`, against the published 3 and 4.

I inferred that a placement `[a, b]` merges the face containing dart `a` into the face containing
dart `b`. Under that completion:

- 005: merge `face(6) = {6,7}` into `face(0) = {0,2,4}` → classes `{0,2,4,6,7}` and `{1,3,5}`;
  `planar_faces = 2`, `euler_planar = 5-4+2 = 3`. Matches published exactly, and `same_face (0,6)` /
  `different_face (1,6)` both hold.
- 010: merge `face(1)={1,3,5}` with `face(9)={9,13,11}`, and `face(6)={6,7}` into `face(0)={0,2,4}`
  → classes `{0,2,4,6,7}`, `{1,3,5,9,11,13}`, `{8,10,12}`; `planar_faces = 3`,
  `euler_planar = 8-7+3 = 4`. Matches published exactly, and all seven relation pairs hold.

**I conclude the published values are right and `METHOD.md` is incomplete**, not that the entries
are wrong. The inferred semantics is also the physically correct one (a stroke drawn inside a
triangle shares the triangle's interior region), it reproduces 9 published numbers and 9 relation
pairs with no slack, and `TASK.md`'s claim that everything is derivable from `rotations` and
`n_darts` is in fact false for these two entries even with the `nesting` block in hand, since
`placements` has no stated meaning. Flagging the gap rather than the values.

**The partition of faces into "bounded" and "unbounded" is nowhere derivable.** For 004 the two
triangles' face orbits are both 3-cycles and nothing in the map distinguishes them; the choice of
darts 1 and 7 as outer is pure input. 004's own `teaches` says this ("choosing an outer face is
choosing a point at infinity"), so this is not a complaint about 004 — but it means every prose use
of "bounded" / "unbounded" / "inside" in 001, 005, 006, 007, 010, 011 rests on that input or, where
there is no `nesting` block, on nothing at all. Detailed below.

### Prose claims I could check mechanically and which **hold**

- **011's provenance: "the interleaved order [was rejected] as genus 1 — a torus, not a page."**
  Verified. With `crossing = [0,2,1,3]`: `sigma: 0->2, 2->1, 1->3, 3->0`;
  `phi(0)=sigma(1)=3, phi(3)=sigma(2)=1, phi(1)=sigma(0)=2, phi(2)=sigma(3)=0` → one face
  `{0,3,1,2}`; `chi = 1-2+1 = 0`, `g = 1`. The published rotation `[0,1,2,3]` gives `chi = 2`,
  `g = 0`. The mathematical content of that sentence is exactly right.
- **012's teaches: "011 and 012 agree on every same_face/different_face pair."** Stronger than
  stated and true. The full partitions are *identical*: 011 gives `[[0,2],[1],[3]]`; 012 after its
  merge gives `[[0,2],[1],[3]]`. I enumerated all 6 dart pairs; co-facality differs on none.
- **012's "the planar Euler characteristic, 3 against 2."** 012 `euler_planar = 3`;
  011 `chi = 2`. Holds.
- **003's "four faces, chi = 2c."** `chi = 4`, `2c = 4`. Holds.
- **004's "V - E + F = 1 + c."** `3 = 1 + 2`. Holds.
- **009's "V - E + F = 1 + c = 4."** `6 - 3 + 1 = 4 = 1 + 3`. Holds.
- **010's "the raw orbit count (5, one sphere per component)."** 5 orbits, `chi = 6 = 2·3`. Holds.
- **010's provenance: "composed from 005 and 006."** Structurally verified. Darts 0–7 of 010 are
  005's map verbatim (`v0:[0,5] v1:[2,1] v2:[4,3] s0:[6] s1:[7]`), and darts 8–13 are 003/004's
  second triangle shifted by +2 (`v3:[6,11] v4:[8,7] v5:[10,9]` → `v3:[8,13] v4:[10,9] v5:[12,11]`),
  with the outer dart shifted 7 → 9 accordingly. The composition is real; only the operator name
  "O1 juxtaposition" is unverifiable.
- **009's "count is the component invariant — the thing degree sequences cannot see."** Witnessed
  inside this corpus without needing the cited test: 002 and 003 have *identical* `V = 6`,
  `E = 6` and `degree_sequence = [2,2,2,2,2,2]`, yet `components` 1 and 2. Holds.
- **007's "Two triangles sharing edge AB," and "the shared edge's two darts lie in two DIFFERENT
  regions."** Derived. Edges from the rotations: (0,1)=v0–v1, (2,3)=v1–v2, (4,5)=v2–v0,
  (6,7)=v1–v3, (8,9)=v3–v0 — triangles v0v1v2 and v0v1v3 sharing v0–v1. Dart 0 ∈ `{0,2,4}`,
  dart 1 ∈ `{1,9,7}`: different orbits, and those two are the two length-3 faces while `{3,6,8,5}`
  is the length-4 one. ("different regions" derived; "bounded" is convention — see below.)
- **008's "One face: a tree encloses nothing."** `F = 1` derived; `V = 4`, `E = 3`, connected, so
  the graph is a tree, derivable. Holds.
- **006's "Identical components, identical degree sequence, identical face count — the ENTIRE
  meaning difference from 005 is which region the segment shares."** Verified: 005 and 006 have
  byte-identical `n_darts` and `rotations`, so every un-nested invariant is identical; they differ
  only in the `nesting` block, and the merged partitions are `{0,2,4,6,7} | {1,3,5}` (005) versus
  `{0,2,4} | {1,3,5,6,7}` (006). Holds.
- **011's "The two lobes share that single vertex and no edge," and "weaker contact than 007's
  ADJACENCY, where two regions share a whole edge."** Holds under the lobe reading: face `{1}`
  draws only on edge `{0,1}`, face `{3}` only on edge `{2,3}` — disjoint edge sets — and the map's
  one vertex is incident to both. In 007 the two length-3 faces do share a whole edge, both darts
  of `(0,1)`.

---

## PROSE I CANNOT VERIFY

I separate three grades, as `TASK.md` asks: **(A)** *convention I cannot check* — a free choice the
data does not fix; **(B)** *asserted and unsupported* — nothing in this directory bears on it;
**(C)** *derivably false or overstated* — the method decides against it, or against its strength.

### (C) Derivably false or overstated

1. **001: "This is the unique minimal closed figure."** *False within this corpus.* Entry 012 calls
   `circle_a = [0,1]` a circle: that is 2 darts, `V = 1`, `E = 1`, and face tracing gives
   `phi(0) = sigma(1) = 0` → `{0}` and `phi(1) = sigma(0) = 1` → `{1}`, so `F = 2`, `chi = 2`,
   genus 0. A simple closed curve bounding exactly two regions, with 2 darts against the triangle's
   6. The triangle is therefore not minimal, and not unique at its own invariants either. To settle
   it you would need a definition of "closed figure" that excludes loop edges (e.g. requiring a
   simple underlying graph) — `METHOD.md` supplies no such notion, and 012's own usage contradicts it.
2. **011: "Derived, not assigned — the lobes do meet there, and the vertex is read off the ink
   rather than declared."** *Overstated against its own data.* There is no ink in this directory.
   The degree-4 vertex is given in `rotations` as `crossing: [0,1,2,3]` — it is an *input*, declared,
   not derived. What I can derive is only the consequence: given that rotation, `V = 1` of degree 4,
   `F = 3`, genus 0. To settle the "read off the ink" claim you would need the upstream artifact
   (a stroke/curve representation) plus the procedure that turns crossings into vertices, and a
   demonstration that the procedure outputs this rotation rather than the interleaved one. Neither
   is here. (Relatedly, `CROSSING-IS-AN-EMERGENT-VERTEX` is a named claim I cannot look up — grade B.)
3. **011: "The lobes are distinguished from two separate circles by vertex count, not by any face
   relation," and 012: "The whole distinction is carried by vertex count."** The face half is
   derived and true (identical partitions — verified). The "only vertex count" half is **incomplete**:
   011 and 012 also differ in `components` (1 vs 2) and `degree_sequence` ([4] vs [2,2]). Those are
   arguably downstream of the vertex structure, but as written the sentence says the distinction is
   carried by one scalar, and three published fields differ. Also note 012's "3 against 2" compares
   012's `euler_planar` with 011's raw `chi`; that is legitimate only because 011 is connected and
   so needs no merge — worth saying out loud, since the two numbers come from different columns.
4. **Internal citation inconsistency (prose, not data).** 009's `description` cites
   `map.rs::degree_sequence_cannot_see_connectivity` while its own `provenance` names
   `map.rs::tests::components_sees_what_degree_sequence_cannot`, and 003's `provenance` names the
   first of those. At most one of the two names is the test these entries mean. I cannot check
   either; I note only that the entries disagree with each other about which it is.
5. **`"tier": "VERIFIED"` on all 12 entries.** Three entries have wrong published values, so this
   label demonstrably does not track correctness in this corpus. I cannot verify what it does track,
   who applied it, or against what. Settling it needs the definition of the tier and the verification
   record.

### (A) Conventions I cannot check

6. **Which face orbit is "outer" / "unbounded" / "bounded" — everywhere it appears.** The map is a
   sphere-level object; nothing in `rotations` picks a point at infinity. Where a `nesting` block
   exists (004, 005, 006, 009, 010, 012) the choice is supplied as input and I can only confirm it
   is *used* consistently. Where there is **no** `nesting` block, the prose still uses the vocabulary
   with nothing at all to fix it:
   - **001** "it bounds exactly two regions" — the count 2 is derived; which of `{0,2,4}`,
     `{1,3,5}` is the inside is not.
   - **007** "two DIFFERENT **bounded** regions", "two bounded regions share a boundary" — that
     darts 0 and 1 lie in different faces is derived; that the two length-3 faces are the bounded
     ones is not. The face-degree evidence (3,3,4) is suggestive, not decisive.
   - **011** "two **bounded** regions (the lobes) and one unbounded one" — `F = 3` and the face
     degrees (2,1,1) are derived; that the two length-1 faces are the lobes is a choice. It is a
     well-motivated choice (it is the only assignment under which "the lobes share no edge" comes
     out true), but it is still a choice. Settling any of these needs an embedding or an explicit
     outer-face designation, i.e. a `nesting` block.
7. **004: "both components at top level in the plane."** Input, not derived. Same for 009's "all
   top-level", 012's "side by side", 005's "the segment lies in the bounded region", and 010's
   "C2 at top level".
8. **005 / 006 / 010 lexicon glosses: CONTAINMENT = "X is inside C", SEPARATION = "X is outside C".**
   The structural facts are derived (005: dart 6 co-facal with 0 and not with 1; 006: the reverse;
   010: both at once). That *inside* rather than *outside* is the right English for the first rests
   entirely on the `nesting` input naming dart 1 as outer. Swap that designation and the two entries
   exchange meanings with no change to any orbit.
9. **The semantics of `nesting.placements`.** Covered above. I supplied a reading; the published
   numbers confirm it; but it is my reading, not the method's. Settling it needs one sentence in
   `METHOD.md`.
10. **Schema inconsistency across `expected` blocks.** 003, 010, 011, 012 publish `faces`; 004, 005,
    006, 009 omit it even though the raw orbit count exists (3, 3, 3, 3 respectively). 003, 010, 012
    publish `genus: null`; 004, 005, 006, 009 omit `genus`. Not wrong, but I cannot tell whether an
    omission means "not applicable", "not claimed", or "overlooked", and for 005/006/009 the omitted
    raw `faces` is the number that would have exposed the `placements` gap.

### (B) Asserted and unsupported by anything in this directory

11. **001: "every possible notation in any universe must contain it as a primitive; this has been
    proven."** No proof, no statement of the theorem, no definition of "notation" or "primitive" is
    in this directory, and nothing in `METHOD.md` could establish a claim of this shape — `METHOD.md`
    computes invariants of given maps and says nothing about which maps must exist. The word "proven"
    is doing work no artifact here supports. To settle it: the actual proof, plus formal definitions
    of "notation", "primitive", and the quantifier "any universe". (Note item 1 above: the claim's
    own *minimality* premise is already false in this corpus.)
12. **002: "metric detail dies before the fixed point"; "Module F: the Erlangen ladder — a hexagon
    and a triangle are the same object at the fixed point."** "Fixed point", "the Erlangen ladder"
    and "Module F" are undefined here. The checkable residue holds (both are connected, genus 0,
    `F = 2`, all degrees 2), but the two are *not* the same combinatorial map — `V = 3, E = 3` versus
    `V = 6, E = 6` — so the identity claim needs the coarse-graining under which they become equal,
    and that is exactly what is missing. Settling it: a definition of the coarse-graining operator
    and the statement that both are its fixed point.
13. **003/F-025: "Heffter-Edmonds is stated for CONNECTED graphs (F-025)."** The *substance* holds
    and is in `METHOD.md` ("the formula applies only to connected maps"). The attribution to
    Heffter–Edmonds and the failure tag `F-025` are references I cannot check from here.
14. **005: "Machine-checked as co-facality"; 007/008/010: "verified by this harness";
    007: "derived in research/notes/056"; 008: "notes/024 (junction axis, derived)"; all
    `provenance` strings naming `map.rs::tests::...`; 011/012: "Note 071".** I re-derived the
    mathematical content myself wherever it was derivable. That a machine or a named harness checked
    it, and that the named tests and notes exist and say this, I cannot verify — and the constraint
    forbids looking. Three of the twelve published blocks are wrong, which is itself evidence that
    "machine-checked" should not be taken at face value here.
15. **005: "Under the M1 exemplification reading (CONJECTURED route), this is 'encapsulation' — the
    mark does not depict containment, it instantiates it."** "M1", "exemplification reading",
    "CONJECTURED route", and the depict/instantiate distinction are all outside `METHOD.md`. This is
    a semantic thesis, not a fact about orbits; no computation bears on it. Settling it needs the
    definitions and the argument.
16. **008: "Degree is the topological invariant the junction axis carries"; "the three degree-1 free
    ends are the local type the legacy inventory never named"; "the canonical symbol inventory is
    indexed by junction degree."** "Junction axis", "legacy inventory" and "canonical symbol
    inventory" are not here. Degree *is* an invariant of the map (orbit size of `sigma`), which I can
    confirm; everything about axes and inventories I cannot.
17. **007: "RCC-8's EC"; 011: "a third RCC-8-adjacent relation"; 012: "RCC-8's DC".** RCC-8 is an
    external framework not reproduced here. I can confirm the *structural* distinctions are real and
    distinct (007: two faces sharing both darts of one edge; 011: two faces sharing a vertex and no
    edge; 012: two faces sharing nothing), and that they form a strict chain. Whether they
    correspond to EC / DC / anything in RCC-8 I cannot check.
18. **009: "number as configuration count — the natural-tier form of counting"** — "natural-tier"
    undefined here.
19. **010: "O1 juxtaposition"** — the composition is structurally verified (above); the operator name
    and its definition are not here.
20. **011: "Cosmic Codex orbit 13, reflect_pair(Wave), attested meaning 'infinity (SymbolFolds)'";
    the top-level `"orbit": 13` field; 012: "Cosmic Codex orbit 17,
    concatenate(Circle,Circle) / superpose_displaced(Circle,Circle), no attested meaning."**
    No codex, no operator definitions, no attestation record in this directory. 011 is admirably
    explicit that the label "stays conventional"; I am only recording that I cannot check it, nor the
    orbit numbering, nor "no attested meaning" (a negative claim about a corpus I cannot see).
21. **012: "Nearest neighbour to orbit 13 at distance 0.0794, the second-closest pair in the
    matrix."** No matrix, no metric, no distance definition here. Entirely unverifiable; settling it
    needs the matrix and the definition of the distance.
22. **011: "The planar rotation was found by this repo's own tooling."** The *mathematical* claim is
    verified (item above: interleaved → genus 1). That this repo's tooling is what found it is a
    historical claim about a process I cannot observe.
23. **003: "Decidable and wrong for the plane — nothing signals it."** Mostly derived: `faces = 4` is
    decidable, the planar answer is 3 (confirmed by 004 with nesting added), and I confirmed that 003
    and 004 have **identical** `n_darts` and `rotations` — so the map genuinely carries no signal
    distinguishing them, which is the strongest form of "nothing signals it" available. The only
    unverifiable residue is the word "wrong", which presupposes that the plane is the intended
    reading — a framing choice, not a derivable fact.

---

## Notes on the constraints, as `TASK.md` asks

**Temptation, recorded rather than acted on.** It arose three times and was strongest at 001. The
`provenance` string on 001 is `map.rs::tests::triangle_has_two_faces` — a filename plus a test name
that appears to settle the dispute in my favour. The pull to go find `map.rs` and read that test was
concrete, and I did not. I have used the string only as *prose internal to the entry* (evidence about
what the author intended), never as evidence about the mathematics; the `faces = 2` finding rests
entirely on the dart-by-dart trace, and would stand if the provenance string said the opposite. The
same temptation recurred at 002 (`cycles_satisfy_euler_and_are_genus_zero`) and at 008, where I
wanted to check whether some other entry's rotation system had been pasted in by mistake. In all
three cases the orbits decide it without external help, which is the honest reason not to look as
well as the required one. I also did not look for a corpus, a test suite, or any other copy of these
entries, and did not search the web.

**One disclosure.** I did not read anything outside `examrun-01/`, but the harness pre-loaded a
project `AGENTS.md` into my context before I received the task, and it names paths including
`ul-forge/crates/ul-core/src/` and discusses the combinatorial-map/DCEL framing. I did not open any
of those paths and used none of that material in any derivation; I record it because it means my
context was not perfectly sealed, through no choice of mine. Notably, that file's own stated rule —
"no count is a claim... read the source, never a number in prose" — is advice I could not follow here
and did not: I derived every number from the entry data instead, which is what the exercise asks for.

**Scope limit I should state plainly.** `TASK.md` says everything in `entries/` is derivable from
`rotations` and `n_darts`. That is not quite true: `planar_faces`, `euler_planar` and the
`same_face`/`different_face` pairs of 004, 005, 006, 009, 010 and 012 additionally require the
`nesting` block, and for 005 and 010 they require a semantics for `placements` that `METHOD.md`
never gives. I derived those two under the completion stated above and they come out exactly right.

---

## SOURCES AND COMMANDS

### Files read (complete list, including reads I would not repeat)

1. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/TASK.md`
2. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/METHOD.md`
3. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/001-triangle.json`
4. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/002-hexagon.json`
5. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/003-two-triangles-raw.json`
6. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/004-two-triangles-planar.json`
7. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/005-containment.json`
8. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/006-separation.json`
9. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/007-adjacency.json`
10. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/008-junction.json`
11. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/009-multiplicity.json`
12. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/010-containment-with-separation.json`
13. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/011-point-contact.json`
14. `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries/012-no-contact.json`

All 12 entry files were read in a single `cat` loop (command 3 below). No file outside
`examrun-01/` was opened. No directory outside `examrun-01/` was listed, grepped or globbed.

Not a read I performed, but disclosed for completeness: `/home/user/universal_language/AGENTS.md`
was injected into my context by the harness as a system-reminder before the task began. I did not
open it and did not use it.

### Files I wrote

- `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/work/derive.py` (my own implementation of `METHOD.md`; Python 3 stdlib only)
- `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/work/aux.py` (prose-claim checks; Python 3 stdlib only)
- `/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/REPORT.md` (this file)

No package was installed. No third-party library was used. No existing implementation of this
formalism was consulted or looked for.

### Shell commands run, verbatim and complete, in order

1.
```
ls -la /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/
```

2.
```
cd /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/ && cat TASK.md && echo "=====METHOD=====" && cat METHOD.md && echo "=====ENTRIES=====" && ls -la entries/
```

3.
```
cd /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/entries && for f in *.json; do echo "########## $f ##########"; cat "$f"; echo; done
```

4. (single command: heredoc writing `work/derive.py`, then running it)
```
mkdir -p /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/work && cat > /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/work/derive.py <<'PYEOF'
[... contents of work/derive.py, reproduced in that file ...]
PYEOF
python3 -I /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/work/derive.py
```

5. (single command: heredoc writing `work/aux.py`, then running it)
```
cat > /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/work/aux.py <<'PYEOF'
[... contents of work/aux.py, reproduced in that file ...]
PYEOF
python3 -I /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/work/aux.py
```

6.
```
cat > /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/REPORT.md <<'MDEOF'
[... this report ...]
  <report terminator>
```

(Attempt 6 initially failed: the report's own text contains the heredoc terminator at the start of
a line, which closed the heredoc early. Bash reported `unexpected EOF while looking for matching
backtick` and exited 2. REPORT.md was left truncated at exactly that point. Recorded because
`TASK.md` asks for every command including mistakes.)

7. (diagnostic after that failure)
```
ls -la /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/ /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/work/ 2>&1; echo "--- tail of REPORT.md if any ---"; tail -5 /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/REPORT.md 2>&1
```

8. (append the truncated tail with a non-colliding delimiter)
```
cat >> /tmp/claude-0/.../examrun-01/REPORT.md <<'ZZEND'
  <this closing section>
ZZEND
```

(Attempt 8 also failed, for the same reason: I put the new delimiter ZZEND inside the heredoc body
while documenting the command, which closed it early a second time. Bash reported a syntax error
near `final` and exit 2. A partial tail was appended before the failure.)

9. (append the remaining tail via a file, avoiding heredoc delimiter collision entirely)
```
Write tool -> /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/work/tail.md
cat /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/work/tail.md >> /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01/REPORT.md
```

10. (final verification)
```
cd /tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-01 && wc -c REPORT.md && grep -n '^## ' REPORT.md
```

That is every command. The only tool used other than Bash was the `Write` tool, used twice: once
for `work/tail.md` above (because the heredoc delimiter kept colliding with the report's own text),
and the two Python scripts were themselves written by Bash heredoc. No network access, no web
search, no directory outside `examrun-01/` touched at any point.
