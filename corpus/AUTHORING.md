# AUTHORING — from a drawing to a verified entry

**Why this file exists:** a cold reader with no context authored a correct corpus entry using only
this repository (`research/notes/060`) — but reported that doing so required reading the test
harness line by line, and reverse-engineering the dart conventions from existing entries. Every
rule below already existed in the code. **None of it was written down.** This is the missing rung
between `GLOSSARY.md` (normative vocabulary) and `corpus_tests.rs` (executable truth).

---

## The worked example: a circle with a stroke inside it

### 1. Draw it, then name the parts

A closed curve (drawn here as a triangle — at the fixed point a triangle **is** a circle,
`SUBDIVISION-IS-INERT`) with one open stroke inside it. Three ring vertices, two stroke endpoints.

### 2. Number the darts — the rule, stated as a rule

> **α(d) = d ^ 1.** Darts `2i` and `2i+1` are the two halves of edge `i`. Edges are numbered from
> zero, in whatever order you choose — but once chosen, the pairing is forced.

So a 3-edge ring uses darts 0–5 (edges 0,1,2) and a 1-edge stroke added after it uses darts 6,7
(edge 3). **Components do not interleave**: give each component a contiguous block, or the
arithmetic below stops being readable.

### 3. Write the rotation at each vertex — the cycle convention, stated as a rule

For a ring built as `CombinatorialMap::cycle(n)`, vertex `v` carries **`[outgoing, incoming]`**:

> **`outgoing = 2v`** and **`incoming = 2·((v + n − 1) mod n) + 1`**

For n = 3: `v0: [0, 5]`, `v1: [2, 1]`, `v2: [4, 3]`. A free endpoint has a single dart: `s0: [6]`,
`s1: [7]`.

**The rotation is the content** — it is the cyclic order of marks meeting at that point, and
changing it changes what the expression says. (Chirality is a convention with a measured cost:
this repo and the interaction-net literature disagree on clockwise versus anticlockwise, which is
the ℤ/2 on the ledger — see `061`. Be internally consistent; nothing else is available.)

### 4. Find which orbit is bounded — load-bearing, and derivable

Faces are orbits of `φ = σ ∘ α`. For a ring built by the convention above:

> **Even darts trace one face, odd darts trace the other.** Which one you call *bounded* is the
> choice of a point at infinity — genuinely extra information, which is why `Nesting` exists.

By this repo's convention throughout `entries/`: **odd darts are the outward side.** So the ring's
outer face is represented by dart `1`, and the bounded face by dart `0`.

**Check it rather than trusting it** — `map.faces()` prints the orbits.

### 5. Write the nesting — the planar reading

```json
"nesting": { "top_level_outer_darts": [1], "placements": [[6, 0]] }
```

- `top_level_outer_darts` — one dart on the **outward** face of each component that sits in the
  unbounded region.
- `placements` — **`[outer_dart_of_the_contained_component, dart_on_the_containing_face]`**. Here:
  the stroke (dart 6) sits in the ring's bounded face (dart 0).

**Without nesting, a disconnected configuration's regions are objectively wrong** — face tracing
puts each component on its own sphere (compare entries `003` and `004`). This is W3 in
`spec/grammar-core.md`, and it is why `genus()` refuses rather than lying.

### 6. Write `expected` — the field contract

| Key | Asserts | Required? |
|---|---|---|
| `vertices`, `edges`, `components` | the structural floor | **yes — always** |
| `faces` | **raw** orbit count (each component on its own sphere) | optional |
| `genus` | a number asserts equality; **`null` asserts the formula must NOT apply** (disconnected) | optional — see below |
| `degree_sequence` | sorted degrees; carries junction/free-end meanings | optional |
| `planar_faces`, `euler_planar` | the planar reading — **require `nesting`** | optional |
| `same_face` / `different_face` | pairs of darts sharing / not sharing a region | optional — see below |

**Two rules with teeth, both enforced by the harness:**

1. **`genus: null` ≠ `genus` absent.** Null is the positive assertion *"disconnected, so the
   formula must not apply."* Absent asserts nothing. *(These were silently identical until `060`
   — an omitted `genus` was asserting something the author never wrote.)*
2. **No vacuous entries.** The structural floor is mandatory, and **an entry declaring a `lexicon`
   meaning must machine-check the structure that meaning consists in** — regional
   (`same_face`/`different_face`) or local (`degree_sequence`). A claim that risks nothing
   verifies nothing.

**Which face set do `same_face`/`different_face` consult?** The **planar** faces when the entry has
`nesting`; the **raw** orbits when it does not. This is a large semantic switch — check that your
entry has the nesting it needs before writing regional assertions.

### 7. The complete entry

```json
{
  "id": "005-containment",
  "tier": "VERIFIED",
  "lexicon": "CONTAINMENT — X is inside C",
  "description": "...why the meaning is a theorem about the mark...",
  "n_darts": 8,
  "rotations": { "v0": [0,5], "v1": [2,1], "v2": [4,3], "s0": [6], "s1": [7] },
  "nesting": { "top_level_outer_darts": [1], "placements": [[6, 0]] },
  "expected": {
    "vertices": 5, "edges": 4, "components": 2,
    "planar_faces": 2, "euler_planar": 3,
    "degree_sequence": [1,1,2,2,2],
    "same_face": [[0,6]], "different_face": [[1,6]]
  },
  "provenance": "...test or derivation the ground truth comes from...",
  "teaches": "...which curriculum module and what it demonstrates..."
}
```

### 8. Verify — and then try to break it

```
cd ul-forge && cargo test -p ul-core --test corpus_tests
```

**Then falsify one number and watch the harness fire, then restore it.** A first-try pass is weak
evidence — it may mean your entry is right, or that it asserts nothing that could be wrong. The
cold reader in `060` did this unprompted, and it is the habit this corpus is built on.

---

## Tiers travel

`tier` is required and enforced. `VERIFIED` means every field in `expected` is checked by the
harness against the implementation. If your entry asserts something the code cannot check, either
leave it out or mark the entry `CONJECTURED` and say which field is unchecked
(`TIERS-TRAVEL-WITH-CONTENT`).

## What this file does not yet cover

**Rendering.** There is no worked path from a *drawn image* to darts — the step above starts from a
drawing you have already decomposed by hand. Bridging pixels-or-strokes to a rotation system is
unbuilt, and it is the honest gap between calling this a *writing system* and having one
(`060` friction list; Module N/F territory).
