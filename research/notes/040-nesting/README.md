# 040 — The nesting relation, implemented

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** fixes the material finding from `039`

## The problem

`REGIONS-ARE-FACE-UNIONS` — the Phase 4 decidability result — was **decidable and wrong** for any
notation with more than one stroke. Face tracing on a disconnected map treats each component as
embedded on its own sphere, so two disjoint triangles traced **four** faces rather than the **three**
they bound, and `χ = 2c` rather than the correct `1 + c`.

**Wrong answers with no failure signal is worse than no answer.**

## Why the data is genuinely missing rather than derivable

Heffter–Edmonds determines an embedding **for a connected graph**. For a disconnected configuration
the *relative placement* of components is not recoverable from the rotations — and **choosing an
outer face is choosing a point at infinity.** Combinatorially no face is distinguished, so this is
extra information by nature, not an omission that could be computed away.

## What was built

A `Nesting` type: for each component, a dart on its **own outer face**, and — unless it is top-level
— a dart on **the face of the container it sits in**.

`faces_planar()` traces faces per component, then **identifies each component's outer face with its
container's face** via union-find. `euler_characteristic_planar()` reports `V − E + F`.

## Verified by test

| Check | Result |
|---|---|
| Two triangles, no nesting | 4 faces — the old wrong answer, kept as a test |
| Two triangles, top-level nesting | **3 faces**, `χ = 1 + c = 3` |
| Connected map | nesting is a **no-op** |
| Side-by-side **vs** contained | **same count, different structure** |

**That last row is the point.** Containment is invisible to the face *count* and visible in the face
*structure* — which is exactly the distinction RCC-8 needs and the rotation system alone loses. The
Phase 4 decidability argument now has the data it always assumed.

## Status

Workspace **349 tests passing**, clippy clean for the module. `REGIONS-ARE-FACE-UNIONS` holds for
disconnected configurations given nesting data, and says so.

**Note for the axis list:** this makes `CONNECTIVITY-AXIS-MISSING` concrete rather than aspirational
— the missing axis now has a representation, though deriving the full axis list from invariant theory
remains outstanding.
