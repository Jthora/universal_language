import json, os, sys

ENTRIES = "/tmp/claude-0/-home-user-universal-language/2579ae26-a8cd-57fa-9120-f6539bfe0b3a/scratchpad/examrun-02/entries"

def alpha(d): return d ^ 1

def build(entry):
    n = entry["n_darts"]
    sigma = {}
    for vname, cyc in entry["rotations"].items():
        L = len(cyc)
        for i, d in enumerate(cyc):
            sigma[d] = cyc[(i+1) % L]
    # sanity
    assert sorted(sigma.keys()) == list(range(n)), (sorted(sigma.keys()), n)
    assert sorted(sigma.values()) == list(range(n)), "sigma not a permutation"
    return n, sigma

def orbits(n, f):
    seen = set(); out = []
    for s in range(n):
        if s in seen: continue
        cyc = []; d = s
        while d not in seen:
            seen.add(d); cyc.append(d); d = f(d)
        out.append(cyc)
    return out

def components(n, sigma):
    parent = list(range(n))
    def find(x):
        while parent[x]!=x:
            parent[x]=parent[parent[x]]; x=parent[x]
        return x
    def uni(a,b):
        ra,rb=find(a),find(b)
        if ra!=rb: parent[ra]=rb
    for d in range(n):
        uni(d, sigma[d]); uni(d, alpha(d))
    groups = {}
    for d in range(n):
        groups.setdefault(find(d), []).append(d)
    return list(groups.values())

def analyse(entry):
    n, sigma = build(entry)
    vorb = orbits(n, lambda d: sigma[d])
    phi = lambda d: sigma[alpha(d)]
    forb = orbits(n, phi)
    comps = components(n, sigma)
    V = len(vorb); E = n//2; F = len(forb); C = len(comps)
    chi = V - E + F
    res = dict(vertices=V, edges=E, faces=F, components=C, chi=chi,
               degree_sequence=sorted(len(o) for o in vorb))
    res["genus"] = (2-chi)//2 if C==1 else None
    res["genus_exact"] = (2-chi)/2 if C==1 else None
    res["vertex_orbits"] = {name: cyc for name, cyc in zip(entry["rotations"].keys(), [])} # placeholder
    res["_vorb"] = vorb
    res["_forb"] = forb
    res["_comps"] = comps
    # face id per dart
    fid = {}
    for i,o in enumerate(forb):
        for d in o: fid[d]=i
    res["_fid"] = fid
    # planar merge
    nest = entry.get("nesting")
    if nest and "top_level_outer_darts" in nest:
        tl = nest["top_level_outer_darts"]
        tlfaces = sorted(set(fid[d] for d in tl))
        pf = F - len(tlfaces) + 1
        res["planar_faces"] = pf
        res["euler_planar"] = V - E + pf
        res["_tlfaces"] = tlfaces
        merged = {}
        rep = tlfaces[0]
        for d in range(n):
            merged[d] = rep if fid[d] in tlfaces else fid[d]
        res["_mfid"] = merged
    else:
        res["_mfid"] = fid
        res["_tlfaces"] = None
    return res

for fn in sorted(os.listdir(ENTRIES)):
    if not fn.endswith(".json"): continue
    e = json.load(open(os.path.join(ENTRIES, fn)))
    r = analyse(e)
    print("="*72)
    print(fn, "  id=", e["id"])
    print("  n_darts:", e["n_darts"])
    print("  sigma:", {d: build(e)[1][d] for d in range(e["n_darts"])})
    print("  alpha:", {d: d^1 for d in range(e["n_darts"])})
    print("  phi = sigma o alpha:", {d: build(e)[1][d^1] for d in range(e["n_darts"])})
    print("  VERTEX orbits (sigma):")
    for o in r["_vorb"]: print("     ", o, " deg", len(o))
    print("  FACE orbits (phi):")
    for i,o in enumerate(r["_forb"]): print("      f%d"%i, o, " len", len(o))
    print("  COMPONENTS:")
    for o in r["_comps"]: print("     ", sorted(o))
    print("  V=%d E=%d F=%d C=%d chi=%d" % (r["vertices"], r["edges"], r["faces"], r["components"], r["chi"]))
    print("  degree_sequence:", r["degree_sequence"])
    print("  genus (connected only):", r["genus_exact"])
    if r["_tlfaces"] is not None:
        print("  top_level_outer_darts:", e["nesting"]["top_level_outer_darts"], "-> face ids", r["_tlfaces"])
        print("  planar_faces:", r["planar_faces"], " euler_planar:", r["euler_planar"])
    if "placements" in (e.get("nesting") or {}):
        print("  NOTE: nesting.placements present (not described in METHOD.md):", e["nesting"]["placements"])
    exp = e["expected"]
    print("  --- COMPARE ---")
    keys = ["vertices","edges","faces","components","genus","degree_sequence","planar_faces","euler_planar"]
    for k in keys:
        if k in exp:
            mine = r.get(k, "N/A")
            flag = "OK " if mine == exp[k] else "MISMATCH"
            print("   %-16s published=%-22r mine=%-22r %s" % (k, exp[k], mine, flag))
    for rel in ("same_face","different_face"):
        if rel in exp:
            for pair in exp[rel]:
                a,b = pair
                mf = r["_mfid"]
                same_raw = r["_fid"][a]==r["_fid"][b]
                same_merged = mf[a]==mf[b]
                want = (rel=="same_face")
                flag = "OK " if same_merged==want else "MISMATCH"
                print("   %-16s %r  merged_same=%s raw_same=%s (rawfaces %s,%s) %s" % (rel, pair, same_merged, same_raw, r["_fid"][a], r["_fid"][b], flag))
    print()
