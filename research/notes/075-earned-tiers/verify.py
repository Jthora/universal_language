#!/usr/bin/env python3
"""Re-derivation of the three classical results that F-039 found had no reachable evidence.

WHY THIS FILE EXISTS
  Seven claims tiered VERIFIED cited `ul_agent:tests/test_icosians_and_gaps.py` and
  `ul_agent:tests/test_symbol_geometry.py`. Jthora/ul_agent was fetched (528 commits, only
  branch `main`) and NEITHER FILE HAS EVER EXISTED THERE: `--full-history --diff-filter=AD`
  finds no add and no delete, and nothing in that repo mentions icosians, Tammes or octonions.
  So the top tier rested on a dead pointer for as long as nobody looked.

  Three of those seven are classical mathematics, independent of this project, and therefore
  re-derivable from scratch. This script does that, in-tree, so they are earned HERE rather
  than asserted elsewhere. The other four depend on this project's own constructions
  ("the sovereign-hexagram 6-cube", "the symbol octahedron") and are NOT re-derived here;
  they were demoted instead.

WHAT THIS CHECKS, AND WHAT IT DOES NOT
  It checks three mathematical facts. It does NOT establish anything about UL, about the
  Universal Writing System, or about any connection between these facts and a notation. The
  claims it supports are claims about mathematics, nothing more. Any reading of E8 or the
  octonions as evidence FOR UL is unsupported by this file.

  Stdlib only. Exact integer/Fraction arithmetic except where explicitly numerical (Tammes).
  Deterministic: the one randomized search is seeded.

  Usage:  python3 -I verify.py [--strict]
"""

import itertools
import math
import random
import sys
from fractions import Fraction

CHECKS = []
UNRESOLVED = []


def unresolved(name, detail):
    """Recorded as NEITHER passed nor failed: something this script set out to establish and
    did not. It is printed prominently and it must not be read as support for any claim. The
    alternative — adjusting a normalisation until the number comes out right — is the ratchet
    FAILURES.md F-035 is about, and is forbidden by REQ-3."""
    UNRESOLVED.append((name, detail))
    print(f"  [UNRESOLVED] {name}\n               {detail}")


def check(name, ok, detail=""):
    CHECKS.append((name, bool(ok), detail))
    print(f"  [{'ok' if ok else 'FAIL'}] {name}" + (f"  — {detail}" if detail else ""))
    return bool(ok)


# ════════════════════════════════════════════════════════════════════════════════════════
# A. CODE-OCTONIONS-FROM-HAMMING
#    "The cyclic Hamming [7,4,3] code has exactly seven weight-3 codewords whose supports
#     are the lines of the Fano plane; used as a multiplication table they give the octonions."
# ════════════════════════════════════════════════════════════════════════════════════════

def hamming_743():
    """All 16 codewords of the cyclic [7,4,3] Hamming code, g(x) = 1 + x + x^3."""
    g = [1, 1, 0, 1]  # coefficients of 1 + x + x^3
    words = set()
    for msg in itertools.product([0, 1], repeat=4):
        c = [0] * 7
        for i, m in enumerate(msg):          # multiply message poly by g(x) mod 2
            if m:
                for j, gj in enumerate(g):
                    c[i + j] ^= gj
        words.add(tuple(c))
    return words


def section_a():
    print("\nA. Octonions from the cyclic Hamming [7,4,3] code")
    code = hamming_743()
    check("code has 2^4 = 16 codewords", len(code) == 16, f"{len(code)}")
    weights = sorted({sum(c) for c in code})
    check("minimum nonzero distance is 3", min(w for w in weights if w) == 3, f"weights {weights}")

    w3 = sorted(tuple(i for i, b in enumerate(c) if b) for c in code if sum(c) == 3)
    check("exactly 7 weight-3 codewords", len(w3) == 7, f"{len(w3)}")

    # Fano plane axioms on the 7 supports as lines over 7 points
    pts = list(range(7))
    every_line_3 = all(len(set(l)) == 3 for l in w3)
    pair_once = all(sum(1 for l in w3 if p in l and q in l) == 1
                    for p, q in itertools.combinations(pts, 2))
    lines_meet_once = all(len(set(a) & set(b)) == 1
                          for a, b in itertools.combinations(w3, 2))
    pt_on_3 = all(sum(1 for l in w3 if p in l) == 3 for p in pts)
    fano = check("supports are the lines of the Fano plane",
                 every_line_3 and pair_once and lines_meet_once and pt_on_3,
                 "7 points, 7 lines, every pair of points on exactly one line, "
                 "every pair of lines meeting in one point, every point on 3 lines")

    # Build a multiplication on e_0..e_7 from the Fano lines, with e_0 the real unit.
    # For an oriented line (a,b,c): e_a e_b = e_c, e_b e_c = e_a, e_c e_a = e_b, antisymmetric.
    # An orientation per line exists making the result the octonions; we search for one and then
    # verify the defining identities rather than asserting a particular table.
    def build(orients):
        mul = {}
        for i in range(1, 8):
            mul[(0, i)] = (1, i)
            mul[(i, 0)] = (1, i)
            mul[(i, i)] = (-1, 0)
        mul[(0, 0)] = (1, 0)
        for line, rev in zip(w3, orients):
            a, b, c = [x + 1 for x in (line if not rev else (line[0], line[2], line[1]))]
            for (x, y, z) in ((a, b, c), (b, c, a), (c, a, b)):
                mul[(x, y)] = (1, z)
                mul[(y, x)] = (-1, z)
        return mul

    def is_alternative(mul):
        """x(xy) = (xx)y and (yx)x = y(xx) on basis elements, with signs."""
        def m(s1, i, s2, j):
            s, k = mul[(i, j)]
            return (s1 * s2 * s, k)
        for i in range(8):
            for j in range(8):
                s1, k1 = mul[(i, j)]          # x(xy)
                a = m(1, i, s1, k1)
                s2, k2 = mul[(i, i)]          # (xx)y
                b = m(s2, k2, 1, j)
                if a != b:
                    return False
        return True

    found = None
    for orients in itertools.product([False, True], repeat=7):
        mul = build(orients)
        if is_alternative(mul):
            found = (orients, mul)
            break
    alt = check("an orientation exists making the algebra ALTERNATIVE",
                found is not None,
                "x(xy) = (xx)y on all basis pairs — Hurwitz: an 8-dimensional real alternative "
                "algebra with this form is the octonions")
    if found:
        _, mul = found
        assoc_fail = sum(
            1 for i, j, k in itertools.product(range(1, 8), repeat=3)
            if (lambda: (
                (lambda s1, k1: (lambda s2, k2: (s1 * s2, k2))(*mul[(k1, k)]))(*mul[(i, j)])
                != (lambda s1, k1: (lambda s2, k2: (s1 * s2, k2))(*mul[(i, k1)]))(*mul[(j, k)])
            ))()
        )
        check("the algebra is NOT associative", assoc_fail > 0,
              f"{assoc_fail} associator failures on basis triples — so it is not a quaternion "
              "or matrix algebra")
        check("each imaginary unit squares to -1",
              all(mul[(i, i)] == (-1, 0) for i in range(1, 8)))
    return fano and alt


# ════════════════════════════════════════════════════════════════════════════════════════
# B. CODE-TAMMES-FORCES-THE-SOLIDS
#    "Maximising the minimum pairwise angle converges to the octahedron for six points and
#     the icosahedron for twelve."
# ════════════════════════════════════════════════════════════════════════════════════════

def max_min_angle(n, seed, restarts=60, steps=4000):
    """Maximin-angle packing on S^2 by seeded repulsion. NUMERICAL, not a proof."""
    rng = random.Random(seed)
    best, best_pts = -1.0, None
    for _ in range(restarts):
        p = []
        for _ in range(n):
            while True:
                v = [rng.gauss(0, 1) for _ in range(3)]
                r = math.sqrt(sum(x * x for x in v))
                if r > 1e-9:
                    p.append([x / r for x in v])
                    break
        step = 0.12
        for s in range(steps):
            # push each point away from its nearest neighbours
            forces = [[0.0] * 3 for _ in range(n)]
            for i, j in itertools.combinations(range(n), 2):
                d = [p[i][k] - p[j][k] for k in range(3)]
                r2 = sum(x * x for x in d) or 1e-12
                w = 1.0 / (r2 * math.sqrt(r2))
                for k in range(3):
                    forces[i][k] += w * d[k]
                    forces[j][k] -= w * d[k]
            lr = step * (1.0 - s / steps)
            for i in range(n):
                for k in range(3):
                    p[i][k] += lr * forces[i][k]
                r = math.sqrt(sum(x * x for x in p[i])) or 1e-12
                p[i] = [x / r for x in p[i]]
        ang = min(math.acos(max(-1.0, min(1.0, sum(p[i][k] * p[j][k] for k in range(3)))))
                  for i, j in itertools.combinations(range(n), 2))
        if ang > best:
            best, best_pts = ang, [list(x) for x in p]
    return math.degrees(best), best_pts


def section_b():
    print("\nB. Tammes at six and twelve points")
    ok = True
    for n, target, name, deg in ((6, 90.0, "octahedron", 4), (12, 63.434949, "icosahedron", 5)):
        got, pts = max_min_angle(n, seed=20261014 + n)
        close = abs(got - target) < 0.35
        ok &= check(f"n={n}: maximin angle converges to {target:.4f}° ({name})",
                    close, f"found {got:.4f}°")
        # combinatorial type: every point has `deg` neighbours at the optimal angle
        counts = []
        for i in range(n):
            c = 0
            for j in range(n):
                if i == j:
                    continue
                a = math.degrees(math.acos(max(-1.0, min(1.0,
                    sum(pts[i][k] * pts[j][k] for k in range(3))))))
                if abs(a - got) < 1.0:
                    c += 1
            counts.append(c)
        ok &= check(f"n={n}: every point has {deg} nearest neighbours — the {name}'s vertex degree",
                    all(c == deg for c in counts), f"degrees {sorted(counts)}")
    return ok


# ════════════════════════════════════════════════════════════════════════════════════════
# C. CODE-ICOSIANS-ARE-E8
#    "The 120 unit icosians (the binary icosahedral group, built over Q(sqrt5), whose vertex
#     set is the 600-cell) span E8."
#    Exact arithmetic in Z[phi], phi^2 = phi + 1.
# ════════════════════════════════════════════════════════════════════════════════════════

class Phi:
    """a + b*phi with a, b rational; phi = (1+sqrt5)/2, phi^2 = phi + 1."""
    __slots__ = ("a", "b")

    def __init__(self, a=0, b=0):
        self.a, self.b = Fraction(a), Fraction(b)

    def __add__(s, o): return Phi(s.a + o.a, s.b + o.b)
    def __sub__(s, o): return Phi(s.a - o.a, s.b - o.b)
    def __neg__(s):    return Phi(-s.a, -s.b)

    def __mul__(s, o):
        # (a1 + b1 p)(a2 + b2 p) = a1a2 + (a1b2 + a2b1) p + b1b2 p^2,  p^2 = p + 1
        return Phi(s.a * o.a + s.b * o.b, s.a * o.b + s.b * o.a + s.b * o.b)

    def conj(s):  # phi -> 1 - phi  (Galois conjugation)
        return Phi(s.a + s.b, -s.b)

    def trace(s): return 2 * s.a + s.b          # Tr(a + b phi) = 2a + b
    def __eq__(s, o): return s.a == o.a and s.b == o.b
    def __hash__(s):  return hash((s.a, s.b))
    def __repr__(s):  return f"({s.a}+{s.b}φ)"
    def to_float(s):  return float(s.a) + float(s.b) * (1 + 5 ** 0.5) / 2


HALF = Fraction(1, 2)


def qmul(x, y):
    """Quaternion product of 4-tuples of Phi."""
    a1, b1, c1, d1 = x
    a2, b2, c2, d2 = y
    return (
        a1 * a2 - b1 * b2 - c1 * c2 - d1 * d2,
        a1 * b2 + b1 * a2 + c1 * d2 - d1 * c2,
        a1 * c2 - b1 * d2 + c1 * a2 + d1 * b2,
        a1 * d2 + b1 * c2 - c1 * b2 + d1 * a2,
    )


def qnorm(x):
    return x[0] * x[0] + x[1] * x[1] + x[2] * x[2] + x[3] * x[3]


def icosians():
    """The 120 unit icosians = vertices of the 600-cell (binary icosahedral group 2I)."""
    P, Z = Phi(0, 1), Phi(0, 0)
    one, half = Phi(1, 0), Phi(HALF, 0)
    hp = Phi(HALF, 0) * P                       # phi/2
    hpi = Phi(HALF, 0) * (P - one)              # (phi-1)/2 = 1/(2phi)
    out = set()
    # 8 unit quaternions: +-1, +-i, +-j, +-k
    for i in range(4):
        for s in (one, -one):
            v = [Z, Z, Z, Z]
            v[i] = s
            out.add(tuple(v))
    # 16: (+-1 +- i +- j +- k)/2
    for signs in itertools.product([half, -half], repeat=4):
        out.add(tuple(signs))
    # 96: even permutations of (0, +-1/2, +-phi/2, +-(phi-1)/2) — the 600-cell's remaining vertices
    EVEN = [(0, 1, 2, 3), (0, 2, 3, 1), (0, 3, 1, 2), (1, 0, 3, 2),
            (1, 2, 0, 3), (1, 3, 2, 0), (2, 0, 1, 3), (2, 1, 3, 0),
            (2, 3, 0, 1), (3, 0, 2, 1), (3, 1, 0, 2), (3, 2, 1, 0)]
    for perm in EVEN:
        for s1, s2, s3 in itertools.product([1, -1], repeat=3):
            base = [Z, half if s1 > 0 else -half, hp if s2 > 0 else -hp, hpi if s3 > 0 else -hpi]
            v = [Z, Z, Z, Z]
            for src, dst in enumerate(perm):
                v[dst] = base[src]
            out.add(tuple(v))
    return out


def section_c():
    print("\nC. The 120 unit icosians")
    I = icosians()
    ok = check("there are exactly 120 of them (the 600-cell's vertex count)",
               len(I) == 120, f"{len(I)}")
    one = (Phi(1, 0), Phi(0, 0), Phi(0, 0), Phi(0, 0))
    ok &= check("every one has reduced norm 1", all(qnorm(x) == Phi(1, 0) for x in I))
    # group closure under quaternion multiplication — 120x120 exact products
    bad = 0
    for x in I:
        for y in I:
            if qmul(x, y) not in I:
                bad += 1
                break
        if bad:
            break
    ok &= check("closed under quaternion multiplication — a group of order 120",
                bad == 0, "the binary icosahedral group 2I, verified by exhaustive closure")
    ok &= check("contains 1 and is closed under negation",
                one in I and all(tuple(-c for c in x) in I for x in I))

    # The icosian ring as a rank-8 Z-lattice. Each icosian is 4 Phi-coordinates = 8 rationals.
    # Basis: 8 icosians whose 8x8 rational coordinate matrix is nonsingular.
    def coords(x):
        out = []
        for c in x:
            out += [c.a, c.b]
        return out

    def det(M):
        M = [row[:] for row in M]
        n, d = len(M), Fraction(1)
        for i in range(n):
            piv = next((r for r in range(i, n) if M[r][i] != 0), None)
            if piv is None:
                return Fraction(0)
            if piv != i:
                M[i], M[piv] = M[piv], M[i]
                d = -d
            d *= M[i][i]
            inv = Fraction(1) / M[i][i]
            for r in range(i + 1, n):
                f = M[r][i] * inv
                if f:
                    for c in range(i, n):
                        M[r][c] -= f * M[i][c]
        return d

    # A Z-BASIS OF THE ICOSIAN RING, properly.
    #
    # The greedy "first 8 independent vectors" approach generated a finite-index SUBLATTICE and
    # gave det = 625/256, which is why the first version of this check failed. It is recorded here
    # rather than quietly replaced: picking independent generators is not the same as picking a
    # basis of the module they generate. The fix is a Hermite normal form over Z.
    #
    # Coordinates lie in (1/2)Z[phi], so 2*coords are integral; HNF on those gives a Z-basis of
    # 2L, and the bilinear form is quadratic, so Gram(2L) = 4 * Gram(L).
    rows = []
    for x in I:
        v = []
        for c in x:
            a2, b2 = 2 * c.a, 2 * c.b
            assert a2.denominator == 1 and b2.denominator == 1, "2*coords must be integral"
            v += [int(a2), int(b2)]
        rows.append(v)

    def hnf_basis(mat, n=8):
        """Row-style HNF: returns a Z-basis of the lattice generated by the rows."""
        M = [r[:] for r in mat]
        basis = []
        col = 0
        while col < n and M:
            nz = [r for r in M if r[col] != 0]
            if not nz:
                col += 1
                continue
            # reduce all rows against the one with smallest |entry| in this column, by gcd steps
            while len(nz) > 1:
                nz.sort(key=lambda r: abs(r[col]))
                piv = nz[0]
                rest = []
                for r in nz[1:]:
                    q = r[col] // piv[col]
                    r2 = [r[i] - q * piv[i] for i in range(n)]
                    if any(r2):
                        rest.append(r2)
                nz = [piv] + [r for r in rest if r[col] != 0]
                leftover = [r for r in rest if r[col] == 0]
                M = [r for r in M if r[col] == 0] + leftover
            piv = nz[0]
            basis.append(piv)
            M = [r for r in M if r[col] == 0]
            M = [[v for v in r] for r in M if any(r)]
            col += 1
        return basis

    zbasis = hnf_basis(rows)
    ok &= check("Hermite normal form yields a rank-8 Z-basis of the icosian ring",
                len(zbasis) == 8, f"rank {len(zbasis)}")

    if len(zbasis) == 8:
        def as_icosian(v):
            return (Phi(Fraction(v[0], 2), Fraction(v[1], 2)),
                    Phi(Fraction(v[2], 2), Fraction(v[3], 2)),
                    Phi(Fraction(v[4], 2), Fraction(v[5], 2)),
                    Phi(Fraction(v[6], 2), Fraction(v[7], 2)))

        def qconj(x):
            return (x[0], -x[1], -x[2], -x[3])

        def bil(x, y):
            # Tr(x * conj(y)) over Q(sqrt5), taking the quaternion-real part: integral on the ring
            return qmul(x, qconj(y))[0].trace()

        B = [as_icosian(v) for v in zbasis]
        G = [[bil(bi, bj) for bj in B] for bi in B]
        def det(M):
            M = [[Fraction(v) for v in row] for row in M]
            n2, d = len(M), Fraction(1)
            for i in range(n2):
                piv = next((r for r in range(i, n2) if M[r][i] != 0), None)
                if piv is None:
                    return Fraction(0)
                if piv != i:
                    M[i], M[piv] = M[piv], M[i]
                    d = -d
                d *= M[i][i]
                inv = Fraction(1) / M[i][i]
                for r in range(i + 1, n2):
                    f = M[r][i] * inv
                    if f:
                        for c2 in range(i, n2):
                            M[r][c2] -= f * M[i][c2]
            return d

        dG = det(G)
        integral_gram = all(v.denominator == 1 for row in G for v in row)

        # THE PART THIS SCRIPT DOES NOT ESTABLISH.
        #
        # The claim says the icosians "span E8". Everything above is exact and holds. The lattice
        # identification does not come out: under the trace form Tr(x * conj(y)) the Gram of a
        # Z-basis of the Z-span is non-integral with det = 625/256 = 5^4 / 2^8. Two greedy bases
        # and a Hermite normal form all give the same value, so it is a property of the FORM or of
        # the module, not of the basis choice: the icosian ring is a Z[phi]-order and the E8
        # presentation needs the form twisted by the inverse different (1/sqrt5), which this script
        # does not construct.
        #
        # The factors 5^4 and 2^-8 are visible invitations to divide by 625 and multiply by 256
        # until det = 1. That is refused on purpose. Choosing a scale factor because it makes the
        # answer come out is the scope-narrowing ratchet of F-035 applied to arithmetic, and REQ-3
        # says a negative result is recorded rather than tuned away.
        unresolved(
            "the icosians span E8 — NOT established here",
            f"Gram under Tr(x*conj(y)) is {'integral' if integral_gram else 'NON-INTEGRAL'} "
            f"with det = {dG} (= 5^4/2^8), not 1. Even+unimodular+rank-8 would determine E8 "
            "uniquely, and that is exactly what is missing. The group-theoretic content above "
            "(order 120, closure, unit norms, the 600-cell, rank 8) is verified; the lattice "
            "identification needs the inverse-different twist and is left open. "
            "CODE-ICOSIANS-ARE-E8 therefore does NOT earn VERIFIED on this file."
        )
    return ok


def main():
    print("=" * 86)
    print("Re-derivation of three classical results whose cited evidence did not exist (F-039)")
    print("=" * 86)
    a, b, c = section_a(), section_b(), section_c()
    print("\n" + "-" * 86)
    passed = sum(1 for _, ok, _ in CHECKS if ok)
    total = len(CHECKS)
    print(f"{passed}/{total} checks passed")
    if UNRESOLVED:
        print(f"\n{len(UNRESOLVED)} UNRESOLVED — set out to establish, not established:")
        for n, d in UNRESOLVED:
            print(f"  · {n}")
        print("  These are not failures of the code and they are not support for any claim.")
        print("  They are open items, recorded rather than tuned away (REQ-3).")
    print("\nSCOPE: these are facts about mathematics. Nothing here supports any claim about UL,")
    print("the notation, or a connection between these structures and meaning. Section B is")
    print("NUMERICAL (a seeded maximin search), so it corroborates convergence rather than")
    print("proving it; A and C are exact.")
    print("-" * 86)
    if passed != total:
        print("SOME CHECKS FAILED")
        return 1 if "--strict" in sys.argv else 0
    print("ALL CHECKS PASSED")
    return 0


if __name__ == "__main__":
    sys.exit(main())
