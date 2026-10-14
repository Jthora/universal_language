#!/usr/bin/env python3
# encoding: utf-8
"""
verify.py — note 064. The capacity-radius trade-off on a Fisher-Rao semantic manifold.

WHAT THIS CHECKS, AND WHAT IT DOES NOT

This script verifies statements about ONE model: the Fisher-Rao manifold over a finite
alphabet, which is the geometry the wiki's Universal Semantic Manifold page specifies
(Fisher information metric, g_ij, geodesics). It does NOT establish that UL's semantic
space IS that manifold. Every claim registered off this script carries that conditional
in its `scope:`. Read it there before citing any of this.

Nothing here needs numpy or scipy. Stdlib only, so it runs anywhere the artifact lands
(the Voyager constraint, note 053).

    python3 research/notes/064-universal-scope/verify.py
    python3 research/notes/064-universal-scope/verify.py --strict   # exit 1 on failure
"""

import math
import sys
import itertools

# ─── the model ────────────────────────────────────────────────────────────────
# A semantic state is a distribution p over a finite alphabet of size n.
# The Fisher-Rao metric makes the map p -> 2*sqrt(p) an isometry onto the
# nonnegative orthant of the sphere of radius 2, under which
#     d(p,q) = 2*arccos(sum_i sqrt(p_i q_i)).
# "Correction radius" rho(A) of an admissible set A is half its minimum
# pairwise distance: the largest drift magnitude for which repair-to-nearest
# returns the state the drift started from.


def fisher(p, q):
    s = sum(math.sqrt(a * b) for a, b in zip(p, q))
    return 2.0 * math.acos(min(1.0, max(-1.0, s)))


def min_dist(points):
    return min(fisher(a, b) for a, b in itertools.combinations(points, 2))


def packing_radius(points):
    return min_dist(points) / 2.0


def compositions(k, n):
    """every integer n-tuple of nonnegative parts summing to k"""
    if n == 1:
        yield (k,)
        return
    for i in range(k + 1):
        for rest in compositions(k - i, n - 1):
            yield (i,) + rest


def types(k, n):
    """the distributions over n letters whose weights have denominator k"""
    for c in compositions(k, n):
        yield tuple(x / k for x in c)


def point_masses(n):
    return [tuple(1.0 if i == j else 0.0 for i in range(n)) for j in range(n)]


# ─── harness ──────────────────────────────────────────────────────────────────
FAILED = []


def check(label, cond):
    print(f"  [{'PASS' if cond else 'FAIL'}] {label}")
    if not cond:
        FAILED.append(label)


def section(title):
    print(f"\n{title}")
    print("-" * len(title))


# ─── A. separation is a bounded resource ──────────────────────────────────────
section("A. The manifold has diameter pi, whatever the alphabet size")
print("   Two states with disjoint support are exactly pi apart, and nothing is farther.")
print("   Consequence: separation cannot be bought by enlarging the alphabet.")
for n in (2, 3, 6, 12, 24, 100):
    p = [0.0] * n
    p[0] = 1.0
    q = [0.0] * n
    q[-1] = 1.0
    check(f"n={n:3d}: d(disjoint support) = pi", abs(fisher(p, q) - math.pi) < 1e-12)

# a sanity probe: no interior pair may exceed the diameter
worst = 0.0
seed = 12345
for n in (3, 6, 12):
    for _ in range(20000):
        # deterministic LCG so the check is reproducible without `random`
        vals = []
        for _ in range(2 * n):
            seed = (1103515245 * seed + 12345) % (2 ** 31)
            vals.append(seed / (2 ** 31))
        p, q = vals[:n], vals[n:]
        sp, sq = sum(p), sum(q)
        p = [x / sp for x in p]
        q = [x / sq for x in q]
        worst = max(worst, fisher(p, q))
check(f"60k interior pairs all within the diameter (worst {worst:.6f} < pi)", worst < math.pi)

# ─── B. maximum correction radius pins capacity to exactly n ──────────────────
section("B. At the largest possible correction radius, capacity is exactly n")
for n in (2, 3, 6, 12):
    pm = point_masses(n)
    check(
        f"n={n:3d}: the n point masses sit pairwise pi apart (rho = pi/2)",
        abs(min_dist(pm) - math.pi) < 1e-12,
    )
print()
print("   That n cannot be exceeded is a proof, not a measurement:")
print("     d(p,q) = pi  <=>  sum_i sqrt(p_i q_i) = 0  <=>  p and q have disjoint support.")
print("     A set pairwise at distance pi therefore has pairwise-disjoint nonempty supports,")
print("     and an n-letter alphabet admits at most n of those.   => |A| <= n.")
print("   The maximally drift-resistant admissible set is the classical, non-superposed,")
print("   discrete symbol set. Discreteness is not a convenience here; it is the optimum.")

# ─── C. the trade-off ─────────────────────────────────────────────────────────
section("C. Capacity rises only as the correction radius falls")
print("   Deterministic witness family: all distributions with denominator k.")
print("   k=1 recovers section B exactly (the point masses).")
print()
print("     n    k     |A|     rho")
for n in (3, 6):
    rows = []
    for k in (1, 2, 3, 4, 6):
        pts = list(types(k, n))
        if len(pts) < 2:
            continue
        rho = packing_radius(pts)
        rows.append((k, len(pts), rho))
        print(f"     {n}   {k:2d}  {len(pts):6d}   {rho:.6f}")
    monotone = all(
        rows[i + 1][1] >= rows[i][1] and rows[i + 1][2] <= rows[i][2] + 1e-12
        for i in range(len(rows) - 1)
    )
    check(f"n={n}: |A| non-decreasing while rho is non-increasing", monotone)
    print()
print("   Under-specification (A too large, rho -> 0, nothing detectable) and")
print("   fixed-point locking (A too small, no expressivity) are therefore not two")
print("   independent failure modes. They are the two ends of this one curve.")

# ─── D. a continuum detects nothing ───────────────────────────────────────────
section("D. An admissible set with an accumulation point has correction radius 0")
acc = []
for k in range(2, 400):
    e = 1.0 / (k * k)
    acc.append((1.0 - e, e, 0.0))
md = min(fisher(acc[i], acc[i + 1]) for i in range(len(acc) - 1))
check(f"min pairwise distance {md:.3e} => rho ~ 0: no drift magnitude is repairable", md < 1e-4)
print("   If every state is admissible, every drift lands on an admissible state and")
print("   nothing is detectable. Detectability requires excluded states. This is why a")
print("   drift-correcting format must be discrete, and it is a derivation, not a taste.")

# ─── E. finiteness ────────────────────────────────────────────────────────────
section("E. A positive correction radius forces a finite admissible set")
print("   The Fisher orthant is closed and bounded in R^n, hence compact.")
print("   rho(A) > 0  =>  A uniformly discrete  =>  A has no accumulation point  =>  A finite.")
print("   Structural; the numeric witness is D above.")
check("stated as a proof, witnessed by D", md < 1e-4)

# ─── F. the same ceiling, reached independently (Construction A) ──────────────
section("F. Independent corroboration: the Construction-A lattice ceiling")
print("   Classical (Conway & Sloane): the lattice {x in Z^n : x mod 2 in C} built from a")
print("   binary code C of minimum distance d has minimum squared norm min(d, 4) --- a code")
print("   of distance past 4 buys no further geometric separation. Verified here by")
print("   enumeration rather than cited, per the build-over-cite rule (S6).")


def construction_a_min_norm(code, n, radius=2):
    """minimum nonzero squared norm of {x in Z^n : x mod 2 in C}, searched in a box"""
    best = None
    rng = range(-radius, radius + 1)
    for x in itertools.product(rng, repeat=n):
        if all(v == 0 for v in x):
            continue
        if tuple(v % 2 for v in x) in code:
            nsq = sum(v * v for v in x)
            if best is None or nsq < best:
                best = nsq
    return best


cases = [
    ("repetition [4,1,4]", 4, {(0, 0, 0, 0), (1, 1, 1, 1)}, 4),
    ("even-weight [4,3,2]", 4, {c for c in itertools.product((0, 1), repeat=4) if sum(c) % 2 == 0}, 2),
    ("whole space [3,3,1]", 3, set(itertools.product((0, 1), repeat=3)), 1),
    ("repetition [6,1,6]", 6, {(0,) * 6, (1,) * 6}, 4),
]
for name, n, code, d in cases:
    got = construction_a_min_norm(code, n)
    want = min(d, 4)
    check(f"{name}: min norm^2 = min(d,4) = {want}  (got {got})", got == want)
print("   d=6 gives 4, not 6: the ceiling is real and the route to it is independent of A-E.")

# ─── verdict ──────────────────────────────────────────────────────────────────
print("\n" + "=" * 74)
if FAILED:
    print(f"FAILED ({len(FAILED)}):")
    for f in FAILED:
        print(f"  - {f}")
else:
    print("ALL CHECKS PASSED")
print("=" * 74)
print("\nScope reminder: all of the above is conditional on modelling semantic states as")
print("distributions over a finite alphabet under the Fisher-Rao metric. That model is")
print("the wiki's, not an established property of UL. See claims.yaml for the scope line.")

if FAILED and "--strict" in sys.argv:
    sys.exit(1)
