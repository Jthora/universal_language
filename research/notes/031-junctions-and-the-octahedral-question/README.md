# 031 — What "junction" means, and whether the axes are hyper-octahedral

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** clarifies `023`/`024`; tests a structural question against `025`/`026`
**Prompted by:** *"aren't all 2 lines that converge to a point resulting in an angle?"* and *"does
this make the geometry qualified for mapping of the axes 4D hyper-octahedral?"*

---

## 1. Junction vs angle — the terminology was mine and unexplained

**Two lines meeting at a point is an angle. That is correct.** The problem is that not every
meeting-point is two lines.

| Degree | Configuration | Angles around it |
|---|---|---|
| 2 | two branches | **one** — this is *Angle* |
| 3 | T, Y | three |
| 4 | X, + | four |

**"Junction" is only the general word for a point where branches meet.** *Angle* is the degree-2
case. A degree-*n* junction has *n* angles around it, so *Angle* as a primitive covers **n = 2 only**
— and the cross/X exploration was already working in degrees 3 and 4 without a primitive for them.

### Why the extra condition

Degree 2 alone is insufficient: **a smooth curve passing through a point also has degree 2** — the
incoming and outgoing halves.

| | Degree | Tangent | Angle? |
|---|---|---|---|
| Mid-curve point | 2 | continuous | **no** |
| Corner | 2 | **discontinuous** | **yes** |

Hence *degree-2 ∧ tangent-discontinuous*. Both conditions, or smooth curves would count as angles.

### Degree 1

Where the pen lifts — one branch attached. **Not a Point:** a Point is degree 0, isolated. A
stroke-end is a point *with one stroke on it*, and the two are topologically distinct (remove a small
disc: 0 components versus 1). **The inventory has no primitive for it**, and they are drawn
constantly.

---

## 2. Is it hyper-octahedral? Tested, not assumed

A polytope structure needs **uniform, finite** axes. Checking the arities honestly:

| Axis | Arity |
|---|---|
| curvature | 4 — `Z`, `C`, `P`, `V` |
| singularity | 2 |
| closure | 2 |
| **junction degree** | **unbounded** |

**Two obstructions, one fatal.**

**Curvature is not binary — but it splits.** Two binary choices reproduce all four values: *is κ
constant?* (`{Z,C}` vs `{P,V}`) and then *is it the degenerate member?* (`Z` vs `C`, `P` vs `V`).
So curvature is two binary axes, not one four-valued one. **Obstruction removed.**

**Junction degree is unbounded, and that one is fatal.** No finite polytope has an infinite axis.

### The result

**Set junction aside and the structure is exactly hyper-octahedral:**

- 4 binary axes → **2⁴ = 16 cells** = vertices of the **tesseract**
- 4 axes × 2 poles → **8 poles** = vertices of its dual, the **16-cell**
- **The 4-dimensional cross-polytope is the 16-cell — the hyper-octahedron.**

> **Yes, for the binary shape axes. No, once junctions are included — and junction degree is exactly
> what breaks it.**

---

## 3. What this explains

**The cross/X exploration "got very complex very fast" and produced "a fractal of table cells."
That is what an unbounded axis does.** It cannot be folded into a polytope, so instead of a closed
symmetric figure it generates an unbounded family — which is why `026` found the enumeration running
into the open pseudoline-arrangement problem.

> **The octahedral system and the cross/X table are structurally different kinds of object. One is a
> finite polytope over binary axes; the other is an unbounded family over a countable axis. That is
> why one closes cleanly and the other explodes.**

That was observed empirically before it was explained.

## 4. A concrete discrepancy worth checking

- A **3D octahedron** has 6 vertices = **3** binary axes × 2 poles.
- A **4D 16-cell** has 8 vertices = **4** binary axes × 2 poles.

**The derived classification has four binary shape axes. An octahedral (6-pole) system encodes
three.** So either:

1. the octahedral system is **missing an axis** we derived, or
2. **our curvature split is wrong** — the two binary choices should be one atomic axis, giving 3.

**This is checkable rather than a matter of taste**, and it is the first place where the derived
classification and the existing symmetry-grid work make *different* predictions. Whichever way it
resolves, one of the two decompositions is doing unnecessary work — which is the same open item
`026` recorded from the waveform table.

## 5. Method note

**This is a test, not a retrofit.** The four axes were derived in `022` and `024` without reference
to any octahedral structure; the polytope question was asked afterwards and could have come back
negative. `017` recorded retrofitting number-correspondences as this project's documented failure
mode, and the guard here is that **the answer is conditional and names its own obstruction** rather
than being a bare numerical match.
