#!/usr/bin/env python3
# encoding: utf-8
"""
audit.py — note 065. The convention ledger, run as a universality audit.

WHAT THIS IS

Under claims.yaml#LEDGER-CLOSURE-IS-UNIVERSE-SCOPE, ledger closure and universe-scope
invariance are the same statement: an irreducible conventional residue IS a free parameter
the mathematics does not fix. So the universality falsifier is an audit, not an experiment.

This runs that audit against the imported classification (uws/register/), one layer at a
time, asking of each: DOES THE MATHEMATICS FORCE THIS VALUE? Every "no" is a free parameter
and a measured shortfall in scope.

It is deliberately not a pass/fail script. It prints a ledger. The shortfalls it finds are
the result, and note 065 is where they are interpreted.

    python3 research/notes/065-atlas-audit/audit.py
"""

import json
import os
import sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, '..', '..', '..'))
REG = os.path.join(ROOT, 'uws', 'register', 'derivation-register.json')

if not os.path.exists(REG):
    print(f"MISSING: {REG}")
    sys.exit(1)

d = json.load(open(REG, encoding='utf-8'))
C, M, O, CS = d['claims'], d['marks'], d['operators'], d['combination_space']

ledger = []   # (layer, verdict, residue, note)


def line(layer, verdict, residue, note):
    ledger.append((layer, verdict, residue, note))


def head(t):
    print(f"\n{t}\n{'-' * len(t)}")


print("=" * 78)
print("THE CONVENTION LEDGER, RUN AS A UNIVERSALITY AUDIT")
print("Question asked of every layer: does the mathematics force this value?")
print("=" * 78)

# ── 1. the marks ──────────────────────────────────────────────────────────────
head("1. The marks — is the inventory forced, or listed?")
sym = sorted({m['symmetry_order'] for m in M}, key=str)
cur = sorted({m['curvature_class'] for m in M})
cells = {(m['symmetry_order'], m['curvature_class']): m['id'] for m in M}
complete = len(cells) == len(sym) * len(cur) == len(M)
print(f"   axes: symmetry {sym} ({len(sym)})  x  curvature {cur} ({len(cur)})")
for s in sym:
    print(f"     {s:>4} | " + "  ".join(f"{c}={cells.get((s,c),'--')}" for c in cur))
print(f"   all {len(sym)*len(cur)} product cells filled: {complete}")
print("   The count is NOT listed: it is the complete product of two axes. Given the axes,")
print("   the inventory is forced and its cardinality is a consequence rather than a choice.")
line("marks → inventory", "DERIVED given the axes", "none",
     f"complete {len(sym)}x{len(cur)} product, all cells filled")
print("   RESIDUE: the two AXES are themselves measured off a drawn corpus, not derived.")
print("   So the inventory is forced; the choice of what to classify by is not.")
line("marks → the two axes", "MEASURED", "unquantified",
     "symmetry order and curvature class are measured off the corpus, not derived")

# ── 2. the operators ──────────────────────────────────────────────────────────
head("2. The operators — the load-bearing finding")
st = Counter(o['status'] for o in O)
print(f"   statuses: {dict(st)}")
for o in O:
    print(f"     {o['id']:22s} arity={o['arity']}  [{o['status']}]  <- {o.get('attested_by','')[:44]}")
all_measured = set(st) == {'measured'}
print()
print(f"   every operator is `measured`: {all_measured}")
print("   `measured` is the register's own word for 'a fact about these artifacts, NOT")
print("   necessity'. So the operator inventory is ATTESTED FROM ONE DRAWN CORPUS.")
print("   Nothing here says the mathematics forces this set, or forbids another.")
line("operators → inventory", "NOT DERIVED", "UNBOUNDED",
     "all operators `measured` (attested on sheets); no derivation recorded for the set")

# ── 3. the generated space ────────────────────────────────────────────────────
head("3. The generated space — exact, but exact GIVEN the generators")
binary = [o for o in O if o['arity'] == 2]
unary = [o for o in O if o['arity'] == 1]
n = len(M)
calc = len(binary) * n * n + len(unary) * n + (n - 1)
print(f"   {len(binary)} binary x {n}x{n}={n*n} ordered pairs      = {len(binary)*n*n}")
print(f"   + reflect_pair on all {n} marks                = {len(unary)*n}")
print(f"   + close on {n-1} (a Point has nothing to close) = {n-1}")
print(f"   = {calc}    register says {CS['combinations']}    MATCH: {calc == CS['combinations']}")
line("constructions (191)", "EXACT given the generators", "inherits operators' residue",
     "arithmetic verified; exactness is conditional on an unforced generator set")

# ── 4. the close() omission ───────────────────────────────────────────────────
head("4. A gap in the published inventory")
op_ids = {o['id'] for o in O}
print(f"   operator inventory: {sorted(op_ids)}")
print(f"   the 191 requires `close` on {n-1} marks, and `close` is NOT in that inventory.")
print(f"   `close` in inventory: {'close' in op_ids}")
print("   The register uses it as a unary step throughout ('reflect_pair and close as unary")
print("   steps') and distinguishes it from 'the five attested binary operators'. So an")
print("   operation that is load-bearing for the construction count is unpublished.")
line("inventory completeness", "DEFECT", "n/a",
     "`close` is load-bearing for the 191 and absent from the operators array")

# ── 5. the quotient ───────────────────────────────────────────────────────────
head("5. The orbits — where the map stops being a single object")
lo, hi = CS['orbit count range']
print(f"   orbits: central {CS['orbits']}, range {lo}-{hi}, exact={CS['orbit count is exact']}")
print(f"   recorded meanings {CS['recorded']}  +  undetermined {CS['undetermined']}  =  {CS['recorded']+CS['undetermined']}")
print("   The quotient is taken at a CALIBRATED TOLERANCE. The register is explicit: cite")
print("   93 as a central value, never as an exact count, and doubling the rotation search")
print("   does not settle it.")
print()
print("   Its own scope line limits the damage, and the limit is load-bearing:")
print("   'About the count, not about the partition's structure... Nothing here disturbs")
print("   any claim about which figures are related.' That assertion is itself `measured`.")
line("orbit individuation", "TOLERANCE-BOUNDED", f"range {lo}-{hi}",
     "the synonymy quotient depends on a calibration parameter; partition asserted stable, at `measured`")

# ── 6. what the conventions actually are ──────────────────────────────────────
head("6. The admitted conventions — structure, or naming?")
conv = [(k, v) for k, v in C.items() if v['status'] == 'conventional']
for k, v in conv:
    print(f"   * {k}")
    print(f"     {v['statement'][:96]}")
print()
print("   All three are LABELS: colour assignments, inherited astrological words, and the")
print("   badge shapes. None is a statement about which figures exist or how they relate.")
print("   Under universe-scope that is exactly where conventions belong — naming is free,")
print("   classification is not. This is a GOOD result and it should be said as one.")
line("naming layer", "CONVENTIONAL, and correctly so", f"{len(conv)} entries",
     "all three conventions are labels, none touches the classification")

# ── 7. how much of the register bears on universality at all ──────────────────
head("7. How much of the corpus bears on necessity")
sc = Counter(c['status'] for c in C.values())
tot = sum(sc.values())
meas = sc.get('measured', 0)
print(f"   {tot} entries: " + "  ".join(f"{k}={v}" for k, v in sc.most_common()))
print(f"   `measured` = {meas} ({100*meas//tot}%) — by the register's OWN vocabulary these are")
print("   facts about these artifacts and not claims of necessity.")
print(f"   unconditionally derived: {sc.get('derived',0)}")
print(f"   conditional on a stated premise: {sc.get('derived_given',0)}")
print("   So roughly half the corpus is measurement of one drawn corpus, and the part that")
print("   could bear on universe-scope is the derived + derived_given fraction.")
line("corpus composition", "MIXED", f"{sc.get('derived',0)} derived / {sc.get('derived_given',0)} conditional / {meas} measured",
     "half the register is explicitly about artifacts rather than necessity")

# ── the ledger ────────────────────────────────────────────────────────────────
print("\n" + "=" * 78)
print("LEDGER")
print("=" * 78)
w = max(len(l[0]) for l in ledger)
for layer, verdict, residue, note in ledger:
    print(f"  {layer:<{w}}  {verdict}")
    print(f"  {'':<{w}}  residue: {residue}")
    print(f"  {'':<{w}}  {note}")
    print()
print("=" * 78)
print("""HEADLINE

The map's GENERATION is exact. Its GENERATORS are not derived.

Given six marks and the attested operators, the 191 constructions follow by arithmetic,
and the six marks are themselves the complete product of two axes rather than a list. That
is real structure and it is forced.

But every operator carries status `measured` — attested on the owner's sheets, with the
register itself noting that attestation does not derive the semantics. The operator
inventory is therefore the largest free parameter on the load-bearing path, and it is a far
bigger residue than the syntax layer's proved Z/2. The classification layer's ledger line
does not close, and under REQ-3 that is recorded as a shortfall rather than rescoped.

Two narrower findings: `close` is load-bearing for the 191 and missing from the published
inventory, and the orbit count is a calibration-dependent range rather than an integer.

One good result worth stating plainly: all three admitted conventions are NAMING, not
structure. Under universe-scope that is exactly where convention is permitted to live.""")
print("=" * 78)
