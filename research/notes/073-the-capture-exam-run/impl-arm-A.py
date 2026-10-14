#!/usr/bin/env python3
"""Derive combinatorial-map invariants from METHOD.md only. Stdlib only."""
import json, os, sys

ENTRIES = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "entries")

def alpha(d):
    return d ^ 1

def build(entry):
    n = entry["n_darts"]
    sigma = {}
    seen = []
    for vname, cyc in entry["rotations"].items():
        k = len(cyc)
        for i, d in enumerate(cyc):
            sigma[d] = cyc[(i + 1) % k]
            seen.append(d)
    assert sorted(seen) == list(range(n)), (entry["id"], "dart cover broken", sorted(seen), n)
    return n, sigma

def orbits(n, f):
    """orbits of a permutation f on 0..n-1, as list of lists in traversal order"""
    out, visited = [], set()
    for s in range(n):
        if s in visited: continue
        orb, d = [], s
        while d not in visited:
            visited.add(d); orb.append(d); d = f[d]
        out.append(orb)
    return out

def components(n, sigma):
    parent = list(range(n))
    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]; x = parent[x]
        return x
    def union(a, b):
        ra, rb = find(a), find(b)
        if ra != rb: parent[ra] = rb
    for d in range(n):
        union(d, sigma[d]); union(d, alpha(d))
    return len({find(d) for d in range(n)})

def classes_of(orbs):
    """dart -> index of its orbit"""
    m = {}
    for i, o in enumerate(orbs):
        for d in o: m[d] = i
    return m

def merge(face_of, pairs):
    """union-find over face indices, driven by dart pairs"""
    idx = sorted(set(face_of.values()))
    parent = {i: i for i in idx}
    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]; x = parent[x]
        return x
    for a, b in pairs:
        ra, rb = find(face_of[a]), find(face_of[b])
        if ra != rb: parent[ra] = rb
    return {d: find(i) for d, i in face_of.items()}, len({find(i) for i in idx})

def derive(entry):
    n, sigma = build(entry)
    vorbs = orbits(n, sigma)
    phi = {d: sigma[alpha(d)] for d in range(n)}
    forbs = orbits(n, phi)
    V, E, F = len(vorbs), n // 2, len(forbs)
    comp = components(n, sigma)
    chi = V - E + F
    res = {
        "n_darts": n, "vertices": V, "edges": E, "faces": F, "components": comp,
        "degree_sequence": sorted(len(o) for o in vorbs),
        "chi": chi,
        "genus": (2 - chi) // 2 if comp == 1 else None,
        "genus_exact": (2 - chi) / 2 if comp == 1 else None,
        "vertex_orbits": {name: cyc for name, cyc in entry["rotations"].items()},
        "face_orbits": forbs,
    }
    face_of = classes_of(forbs)
    nest = entry.get("nesting")
    if nest:
        pairs = []
        tl = nest.get("top_level_outer_darts", [])
        for d in tl[1:]:
            pairs.append((tl[0], d))
        for a, b in nest.get("placements", []):
            pairs.append((a, b))
        merged, pf = merge(face_of, pairs)
        res["planar_faces"] = pf
        res["euler_planar"] = V - E + pf
        res["merge_pairs_used"] = pairs
        res["merged_face_classes"] = merged
        # also: literal METHOD.md reading, top-level merges only
        pairs_lit = [(tl[0], d) for d in tl[1:]]
        _, pf_lit = merge(face_of, pairs_lit)
        res["planar_faces_toplevel_only"] = pf_lit
        res["euler_planar_toplevel_only"] = V - E + pf_lit
        use = merged
    else:
        use = face_of
    res["face_class_of_dart"] = use
    exp = entry["expected"]
    checks = []
    for a, b in exp.get("same_face", []):
        checks.append(("same_face", (a, b), use[a] == use[b], use[a], use[b]))
    for a, b in exp.get("different_face", []):
        checks.append(("different_face", (a, b), use[a] != use[b], use[a], use[b]))
    res["relation_checks"] = checks
    return res

def main():
    for fn in sorted(os.listdir(ENTRIES)):
        if not fn.endswith(".json"): continue
        e = json.load(open(os.path.join(ENTRIES, fn)))
        r = derive(e)
        exp = e["expected"]
        print("=" * 78)
        print(f"{e['id']}  (n_darts={r['n_darts']})")
        print("  sigma cycles :", r["vertex_orbits"])
        print("  face orbits  :", r["face_orbits"])
        print("  chi = V-E+F  :", f"{r['vertices']} - {r['edges']} + {r['faces']} = {r['chi']}")
        if r["genus"] is not None:
            print("  genus        :", f"(2-{r['chi']})/2 = {r['genus_exact']}")
        else:
            print("  genus        : undefined (components =", r["components"], ")")
        if "planar_faces" in r:
            print("  merge pairs  :", r["merge_pairs_used"])
            print("  planar_faces :", r["planar_faces"], " euler_planar:", r["euler_planar"])
            print("  [toplevel-only literal reading]:", r["planar_faces_toplevel_only"],
                  "euler:", r["euler_planar_toplevel_only"])
        print("  face class of each dart:", r["face_class_of_dart"])
        fields = ["vertices","edges","faces","components","degree_sequence","genus",
                  "planar_faces","euler_planar"]
        for f in fields:
            if f in exp:
                mine = r.get(f)
                ok = (mine == exp[f])
                print(f"   {'OK ' if ok else 'XX '} {f:16s} published={exp[f]!r:28s} derived={mine!r}")
        for kind, pr, ok, ca, cb in r["relation_checks"]:
            print(f"   {'OK ' if ok else 'XX '} {kind} {pr} -> classes {ca},{cb}")
        extra = [k for k in exp if k not in fields + ["same_face","different_face"]]
        if extra: print("  UNCHECKED expected keys:", extra)

main()
