#!/usr/bin/env python3
"""Measure the Universal Symbology sheets and check the sign grammar against them.

The claim under test is that a zodiac glyph in this corpus is not assigned but
computed, from three modular functions of the sign index n (1 = Aries):

    geometry  = n mod 3     angle / square / arc   -> Cardinal / Fixed / Mutable
    direction = n mod 2     apex up / apex down    -> Fire,Air / Water,Earth
    baseline  = n mod 4 in {2,3}   crossbar present -> Air, Earth

direction and baseline together are n mod 4, the element. Since gcd(3,4) = 1,
Z/12 is isomorphic to Z/3 x Z/4, which is why twelve signs exist and why
3 modalities x 4 elements has no repeats and no gaps.

Nothing here reads a label. Every feature is measured off the bitmap: ink
distribution for apex direction, interior cavity count for the crossbar,
flanking horizontal rules for the waveform baseline, and colour-blob centroids
for the wheel positions. Then the measurements are compared against the
arithmetic. Run with --verbose to see every measurement.

The three sheets sit in images/ beside this file. Findings F42-F67, and the
reading they came from, are in the agent repo at
reference/NOTES-symbology-reading.md.
"""

from __future__ import annotations

import argparse
import json
import math
import sys
from collections import deque
from pathlib import Path

import numpy as np
from PIL import Image

# The three sheets sit beside this file so the claim is checkable from a bare
# clone of this repository. The agent repo keeps the full 24-sheet set; if this
# note is being run from there, prefer that copy so the two cannot diverge.
_HERE = Path(__file__).resolve().parent
_CANDIDATES = [
    _HERE / "images",
    _HERE.parents[3] / "reference" / "universal-symbology" / "UniversalSymbologyImages",
]
IMAGE_DIR = next((d for d in _CANDIDATES if d.is_dir()), _CANDIDATES[0])

SIGNS = [
    ("Aries", "Fire", "Cardinal"), ("Taurus", "Earth", "Fixed"),
    ("Gemini", "Air", "Mutable"), ("Cancer", "Water", "Cardinal"),
    ("Leo", "Fire", "Fixed"), ("Virgo", "Earth", "Mutable"),
    ("Libra", "Air", "Cardinal"), ("Scorpio", "Water", "Fixed"),
    ("Sagittarius", "Fire", "Mutable"), ("Capricorn", "Earth", "Cardinal"),
    ("Aquarius", "Air", "Fixed"), ("Pisces", "Water", "Mutable"),
]

# The two bits. Nothing below reads these from a sheet; they are the arithmetic.
UP = {"Fire", "Air"}
BARRED = {"Air", "Earth"}


def element_of(n: int) -> str:
    """n mod 4, in the order the wheel runs."""
    return ["Water", "Fire", "Earth", "Air"][n % 4]


def modality_of(n: int) -> str:
    """n mod 3."""
    return ["Mutable", "Cardinal", "Fixed"][n % 3]


# ── bitmap helpers ────────────────────────────────────────────────────────────

def ink(path: Path, box=None, threshold: int = 128) -> np.ndarray:
    img = Image.open(path).convert("L")
    arr = np.array(img)
    if box:
        y0, y1, x0, x1 = box
        arr = arr[y0:y1, x0:x1]
    return arr < threshold


def column_runs(mask: np.ndarray, min_width: int = 4) -> list[tuple[int, int]]:
    """Contiguous spans of columns that carry ink."""
    present = mask.any(axis=0)
    runs, start = [], None
    for i, v in enumerate(present):
        if v and start is None:
            start = i
        elif not v and start is not None:
            if i - start >= min_width:
                runs.append((start, i))
            start = None
    if start is not None and len(present) - start >= min_width:
        runs.append((start, len(present)))
    return runs


def cavities(mask: np.ndarray, min_size: int = 4) -> int:
    """Background components fully enclosed by ink. A plain triangle has one;
    a crossbar splits the interior and gives two."""
    h, w = mask.shape
    seen = np.zeros_like(mask, bool)
    found = 0
    for y in range(h):
        for x in range(w):
            if mask[y, x] or seen[y, x]:
                continue
            queue, seen[y, x] = deque([(y, x)]), True
            touches_border, size = False, 0
            while queue:
                cy, cx = queue.popleft()
                size += 1
                if cy in (0, h - 1) or cx in (0, w - 1):
                    touches_border = True
                for dy, dx in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                    ny, nx = cy + dy, cx + dx
                    if 0 <= ny < h and 0 <= nx < w and not mask[ny, nx] and not seen[ny, nx]:
                        seen[ny, nx] = True
                        queue.append((ny, nx))
            if not touches_border and size >= min_size:
                found += 1
    return found


def blobs(mask: np.ndarray, min_pixels: int) -> list[tuple[float, float, int]]:
    """Connected components as (centroid_x, centroid_y, size)."""
    h, w = mask.shape
    seen = np.zeros_like(mask, bool)
    out = []
    for y in range(h):
        for x in range(w):
            if not mask[y, x] or seen[y, x]:
                continue
            queue, seen[y, x] = deque([(y, x)]), True
            pixels = []
            while queue:
                cy, cx = queue.popleft()
                pixels.append((cy, cx))
                for dy in (-1, 0, 1):
                    for dx in (-1, 0, 1):
                        ny, nx = cy + dy, cx + dx
                        if 0 <= ny < h and 0 <= nx < w and mask[ny, nx] and not seen[ny, nx]:
                            seen[ny, nx] = True
                            queue.append((ny, nx))
            if len(pixels) >= min_pixels:
                ys = [p[0] for p in pixels]
                xs = [p[1] for p in pixels]
                out.append((sum(xs) / len(xs), sum(ys) / len(ys), len(pixels)))
    return out


# ── check 1: the Base 12 waveform strip ───────────────────────────────────────

def check_base12(verbose: bool) -> dict:
    """Twelve marks on a waveform strip. Measure apex direction from which side
    of the strip the mark occupies, and the crossbar from whether a horizontal
    rule flanks it at that level."""
    path = IMAGE_DIR / "UniversalSymbology-Base12_signs.png"
    band = ink(path, box=(115, 180, 8, 596))
    height = band.shape[0]

    # The strip has an upper signal and a lower one, each sitting on a horizontal
    # rule that runs most of the width. Find the two rules rather than hardcoding
    # them. The flat tops of the Fixed marks come close to the same density, so
    # the threshold has to sit just above them.
    row_ink = band.sum(axis=1)
    dense = [i for i, c in enumerate(row_ink) if c > band.shape[1] * 0.20]
    clusters, current = [], [dense[0]]
    for row in dense[1:]:
        if row - current[-1] <= 2:
            current.append(row)
        else:
            clusters.append(current)
            current = [row]
    clusters.append(current)
    if len(clusters) != 2:
        return {"sheet": "Base12_signs", "passed": False, "failures":
                [f"expected two horizontal rules, found {len(clusters)}"], "detail": []}
    upper_rule, lower_rule = clusters[0][0], clusters[1][0]

    peaks = column_runs(band[upper_rule - 19:upper_rule - 1, :])
    troughs = column_runs(band[lower_rule + 5:lower_rule + 23, :])
    # Discard any trough wholly inside a peak's span (a descender bleeding down).
    troughs = [t for t in troughs if not any(p[0] <= t[0] and t[1] <= p[1] for p in peaks)]

    segments = sorted([(s, e, "up") for s, e in peaks] + [(s, e, "down") for s, e in troughs])

    results, failures = [], []
    for i, (s, e, direction) in enumerate(segments):
        n = i + 1
        rule = band[upper_rule:upper_rule + 4, :] if direction == "up" \
            else band[lower_rule:lower_rule + 4, :]
        flank = rule.sum(axis=0) >= 3
        baseline = flank[max(0, s - 14):s].sum() >= 6 and flank[e:e + 14].sum() >= 6

        element = element_of(n)
        want_up = element in UP
        want_bar = element in BARRED
        ok = (direction == "up") == want_up and baseline == want_bar
        if not ok:
            failures.append(n)
        results.append({
            "n": n, "sign": SIGNS[i][0], "element": element,
            "measured_direction": direction, "measured_baseline": baseline,
            "expected_direction": "up" if want_up else "down", "expected_baseline": want_bar,
            "ok": ok,
        })
        if verbose:
            mark = "ok" if ok else "FAIL"
            print(f"  base12  {n:2d} {SIGNS[i][0]:12s} {element:6s} "
                  f"dir={direction:5s} bar={str(baseline):5s}  {mark}")

    return {
        "sheet": "Base12_signs", "segments_found": len(segments),
        "expected_segments": 12, "failures": failures,
        "passed": len(segments) == 12 and not failures, "detail": results,
    }


# ── check 2: the Base 36 decan row ────────────────────────────────────────────

def check_base36(verbose: bool) -> dict:
    """Twelve element triangles with satellites. Measure apex direction from the
    ink split top-vs-bottom and the crossbar from the interior cavity count."""
    path = IMAGE_DIR / "UniversalSymbology-Base36_Decans.png"
    full = ink(path, box=(170, 280, 0, 600))

    # Triangles occupy the vertical middle; satellites do not reach it. Some
    # neighbours touch, so rather than splitting merged runs, fit a uniform comb
    # of twelve teeth to the column-ink profile — the triangles are evenly
    # spaced, which makes that well posed.
    profile = full[25:85, :].sum(axis=0).astype(float)
    tooth = 38
    best = None
    for pitch in np.arange(40.0, 55.0, 0.05):
        for offset in np.arange(0.0, pitch, 0.5):
            centres = [offset + pitch * (k + 0.5) for k in range(12)]
            lo, hi = int(centres[0] - tooth / 2), int(centres[-1] + tooth / 2)
            if lo < 0 or hi > len(profile):
                continue
            total = sum(profile[int(c - tooth / 2):int(c + tooth / 2)].sum() for c in centres)
            if best is None or total > best[0]:
                best = (total, pitch, offset)
    _, pitch, offset = best
    split = [(int(round(offset + pitch * (k + 0.5) - 20)),
              int(round(offset + pitch * (k + 0.5) + 20))) for k in range(12)]

    # Satellites sit above and below every triangle, so the window alone is not
    # enough — bound the rows too. The triangle band is the set of rows carrying
    # ink in ALL twelve windows; a satellite only ever occupies some of them.
    occupancy = np.array([[full[row, x0:x1].any() for x0, x1 in split]
                          for row in range(full.shape[0])])
    band_rows = np.where(occupancy.all(axis=1))[0]
    top, bottom = int(band_rows.min()), int(band_rows.max())

    results, failures = [], []
    for i, (x0, x1) in enumerate(split):
        n = i + 1
        core = full[top:bottom + 1, x0:x1]
        up = core[-3:].sum() > core[:3].sum()
        # Pad sideways so the crossbar's overhang is inside the frame, and by two
        # rows so the triangle does not touch the border and leak its cavities.
        padded = full[top - 2:bottom + 3, max(0, x0 - 10):x1 + 10]
        bar = cavities(padded) >= 2

        element = element_of(n)
        ok = up == (element in UP) and bar == (element in BARRED)
        if not ok:
            failures.append(n)
        results.append({
            "n": n, "sign": SIGNS[i][0] if i < 12 else "?", "element": element,
            "measured_direction": "up" if up else "down", "measured_crossbar": bar, "ok": ok,
        })
        if verbose:
            print(f"  base36  {n:2d} {SIGNS[i][0] if i < 12 else '?':12s} {element:6s} "
                  f"dir={'up' if up else 'down':5s} bar={str(bar):5s}  {'ok' if ok else 'FAIL'}")

    return {
        "sheet": "Base36_Decans", "segments_found": len(split), "expected_segments": 12,
        "failures": failures, "passed": len(split) == 12 and not failures, "detail": results,
    }


# ── check 3: the Cosmic Families wheels ───────────────────────────────────────

COLOUR = {
    "Fire": lambda r, g, b: (r > 150) & (g < 110) & (b < 110),
    "Earth": lambda r, g, b: (g > 150) & (r < 130) & (b < 130),
    "Water": lambda r, g, b: (b > 150) & (r < 130) & (g < 130),
    "Air": lambda r, g, b: (r > 170) & (g > 170) & (b < 120),
}
FAMILIES = [("Eternity", "Fire", "Earth"), ("Destiny", "Fire", "Water"),
            ("Infinity", "Air", "Water"), ("Reality", "Air", "Earth")]
PANEL_WIDTH = 898
THETA0 = 60.0          # where Aries begins, clockwise from the top
TOLERANCE_DEG = 6.0


def check_families(verbose: bool) -> dict:
    """Each panel draws its two elements' six signs around a zodiac ring, in the
    element's colour. Check every mark sits at its sign's angle."""
    path = IMAGE_DIR / "UniversalSymbology-CosmicFamilies.png"
    rgb = np.array(Image.open(path).convert("RGB")).astype(int)

    results, failures = [], []
    for index, (name, first, second) in enumerate(FAMILIES):
        x0 = 30 + PANEL_WIDTH * index
        panel = rgb[100:640, x0:x0 + 490]
        r, g, b = panel[:, :, 0], panel[:, :, 1], panel[:, :, 2]

        found = {}
        for element in (first, second):
            candidates = blobs(COLOUR[element](r, g, b), min_pixels=200)
            found[element] = candidates

        # Centre the wheel on the first element's three marks, which are never
        # confused with the ring's own gradient.
        anchor = sorted(found[first], key=lambda t: -t[2])[:3]
        cx = sum(p[0] for p in anchor) / 3
        cy = sum(p[1] for p in anchor) / 3

        for element in (first, second):
            # The marks sit outside the ring; the gradient inside it can share a
            # hue, so keep the three furthest from the centre.
            ranked = sorted(found[element],
                            key=lambda t: -math.hypot(t[0] - cx, t[1] - cy))[:3]
            wanted = [n for n in range(1, 13) if element_of(n) == element]
            for x, y, _ in ranked:
                angle = math.degrees(math.atan2(x - cx, -(y - cy))) % 360
                best_err, best_n = min(
                    (min(abs(angle - ((THETA0 + 30 * (n - 1) + 15) % 360)),
                         360 - abs(angle - ((THETA0 + 30 * (n - 1) + 15) % 360))), n)
                    for n in wanted)
                ok = best_err < TOLERANCE_DEG
                if not ok:
                    failures.append(f"{name}/{element}@{angle:.0f}deg")
                results.append({"family": name, "element": element, "sign": best_n,
                                "angle": round(angle, 1), "error_deg": round(best_err, 1),
                                "ok": ok})
                if verbose:
                    print(f"  wheel   {name:9s} {element:6s} sign {best_n:2d} "
                          f"at {angle:6.1f} err {best_err:4.1f}  {'ok' if ok else 'FAIL'}")

    return {"sheet": "CosmicFamilies", "marks_checked": len(results),
            "expected_marks": 24, "failures": failures,
            "passed": len(results) == 24 and not failures, "detail": results}


# ── check 4: the element-pair to mark bijection (combinatorial) ───────────────

# Stated independently in the text primer's element-combination table (F25) and
# drawn on CosmicFamilies (F59). Not measured here; this check tests whether the
# stated table has the structure the notes claim, which is a matter of arithmetic.
PAIR_TO_MARK = {
    frozenset({"Fire", "Air"}): "Angle",
    frozenset({"Water", "Earth"}): "Curve",
    frozenset({"Fire", "Water"}): "Wave",
    frozenset({"Earth", "Air"}): "Line",
    frozenset({"Earth", "Fire"}): "Point",
    frozenset({"Water", "Air"}): "Circle",
}
# From the rotation table: which marks are unchanged under rotation (F5).
ROTATION_CLASS = {
    "Point": "invariant", "Circle": "invariant",
    "Line": "orientable", "Wave": "orientable",
    "Angle": "modulating", "Curve": "modulating",
}


def bits(element: str) -> tuple[int, int]:
    return (1 if element in UP else 0, 1 if element in BARRED else 0)


def check_bijection(verbose: bool) -> dict:
    elements = ["Fire", "Air", "Water", "Earth"]
    pairs = [frozenset({a, b}) for i, a in enumerate(elements) for b in elements[i + 1:]]

    problems = []
    if len(pairs) != 6:
        problems.append(f"C(4,2) came out {len(pairs)}")
    if set(pairs) != set(PAIR_TO_MARK):
        problems.append("the stated table does not cover every pair exactly once")
    if len(set(PAIR_TO_MARK.values())) != 6:
        problems.append("the stated table reuses a mark")

    # Which bit differs -> which rotation class the mark falls in.
    derived = {}
    for pair in pairs:
        a, b = sorted(pair)
        (ua, ba), (ub, bb) = bits(a), bits(b)
        differing = (ua != ub, ba != bb)
        key = {(True, True): "invariant", (True, False): "orientable",
               (False, True): "modulating"}[differing]
        derived.setdefault(key, set()).add(PAIR_TO_MARK[pair])

    stated = {}
    for mark, cls in ROTATION_CLASS.items():
        stated.setdefault(cls, set()).add(mark)

    if derived != stated:
        problems.append(f"partition mismatch: derived {derived} vs stated {stated}")

    if verbose:
        for cls in ("invariant", "orientable", "modulating"):
            print(f"  bijection  hamming->{cls:11s} derived {sorted(derived.get(cls, []))} "
                  f"stated {sorted(stated[cls])}")

    return {"check": "element-pair to mark bijection", "pairs": len(pairs),
            "derived_partition": {k: sorted(v) for k, v in derived.items()},
            "stated_partition": {k: sorted(v) for k, v in stated.items()},
            "failures": problems, "passed": not problems}


# ── check 5: the arithmetic itself ────────────────────────────────────────────

def check_crt(verbose: bool) -> dict:
    """Z/12 = Z/3 x Z/4, so element x modality covers the twelve signs exactly."""
    cells = {(element_of(n), modality_of(n)) for n in range(1, 13)}
    computed = [(element_of(n), modality_of(n)) for n in range(1, 13)]
    stated = [(e, m) for _, e, m in SIGNS]
    problems = []
    if len(cells) != 12:
        problems.append(f"element x modality gave {len(cells)} distinct cells, not 12")
    if computed != stated:
        mismatch = [i + 1 for i, (a, b) in enumerate(zip(computed, stated)) if a != b]
        problems.append(f"computed element/modality disagrees with the zodiac at signs {mismatch}")
    if math.gcd(3, 4) != 1:
        problems.append("gcd(3,4) != 1")
    if verbose:
        print(f"  crt        distinct (element, modality) cells: {len(cells)}/12; "
              f"agrees with the zodiac: {computed == stated}")
    return {"check": "Z/12 = Z/3 x Z/4", "distinct_cells": len(cells),
            "failures": problems, "passed": not problems}


# ── driver ────────────────────────────────────────────────────────────────────

CHECKS = [
    ("crt", check_crt),
    ("bijection", check_bijection),
    ("base12", check_base12),
    ("base36", check_base36),
    ("families", check_families),
]


def run(verbose: bool = False) -> dict:
    missing = [p.name for p in (
        IMAGE_DIR / "UniversalSymbology-Base12_signs.png",
        IMAGE_DIR / "UniversalSymbology-Base36_Decans.png",
        IMAGE_DIR / "UniversalSymbology-CosmicFamilies.png",
    ) if not p.exists()]
    if missing:
        return {"passed": False, "error": f"source images not found: {missing}",
                "image_dir": str(IMAGE_DIR)}

    report = {"image_dir": str(IMAGE_DIR), "checks": {}}
    for name, fn in CHECKS:
        if verbose:
            print(f"[{name}]")
        report["checks"][name] = fn(verbose)
    report["passed"] = all(c["passed"] for c in report["checks"].values())
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--json", action="store_true", help="emit the full report as JSON")
    parser.add_argument("--verbose", "-v", action="store_true", help="print every measurement")
    args = parser.parse_args()

    report = run(verbose=args.verbose and not args.json)
    if args.json:
        print(json.dumps(report, indent=2))
    else:
        if "error" in report:
            print(f"cannot run: {report['error']}")
            return 2
        print()
        for name, result in report["checks"].items():
            status = "PASS" if result["passed"] else "FAIL"
            extra = ""
            if "segments_found" in result:
                extra = f"  ({result['segments_found']}/{result['expected_segments']} marks)"
            elif "marks_checked" in result:
                extra = f"  ({result['marks_checked']}/{result['expected_marks']} marks)"
            print(f"  {status}  {name}{extra}")
            for failure in result.get("failures", []):
                print(f"         - {failure}")
        print()
        print("sign grammar verified against the drawn corpus"
              if report["passed"] else "VERIFICATION FAILED")
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    sys.exit(main())
