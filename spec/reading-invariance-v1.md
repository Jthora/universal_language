# READING INVARIANCE — the 𝔽₀ floor

**Present tense: what exists and is machine-checked.** The full theorem this serves remains open
and is registered as `READING-INVARIANCE-TARGET` (CONJECTURED). Code:
`ul-forge/crates/ul-core/src/map.rs`; derivation record: `research/notes/058`.

## The target, restated

> For every reasonable forgetful reading F of a configuration, the recoverable invariants contain
> the Erlangen fixed point — *it must not matter how you look at it.*

## What exists: the generated class 𝔽₀

𝔽₀ = ⟨ **relabel** · **mirror** · **subdivide** ⟩ — three generators, each a way a receiver's
bookkeeping can differ without the drawing differing:

| Generator | Models | Code |
|---|---|---|
| `relabel(perm, flip)` | different numbering/orientation bookkeeping of the same drawing | isomorphism of maps; commutes with α by construction |
| `mirror()` | the opposite global orientation convention — the ℤ/2 on the ledger | reverses σ |
| `subdivide(d)` | a different sampling density of the same stroke | adds a degree-2 vertex; inert by `SUBDIVISION-IS-INERT` |

**The tracked invariants** (`essential_invariants()`): components, face count, genus, and the
multiset of degrees ≠ 2. Each is stable under each generator *for a reason* — isomorphism, orbit
reversal, and degree-2 inertness respectively — so stability under the generated class follows.

## What is checked, on every CI run

- `reading_invariance_holds_for_the_presentation_class` — five subject configurations (cycles,
  disconnected pairs, open paths, a grammar-composed join) against a battery of mirrors, random
  relabelings, iterated subdivisions, and their composite. Invariants identical throughout.
- `every_corpus_entry_is_reading_invariant` — **the entire lexicon corpus** re-read through mirror,
  an edge-reversing relabeling, and double subdivision. The lexicon's ground truth does not depend
  on how the reader numbers, orients, or samples the drawing.

## The honest gap, which is the theorem

𝔽₀ contains only readings **we defined** — presentation changes with designed-in invariance. The
target quantifies over **all** reasonable readings, and its hard part is exactly the part 𝔽₀
dodges: characterizing "reasonable" without smuggling our reading convention into the definition
(the named deep failure mode of `READING-INVARIANCE-TARGET`). Institution theory supplies the
formal shape — the reading class as a category of signature morphisms with a satisfaction
condition — and that formalization is the open mathematics. **𝔽₀ is the floor, not the theorem;
this file exists so the difference is never blurred.**
