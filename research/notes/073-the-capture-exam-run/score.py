#!/usr/bin/env python3
"""Score a capture-exam report against the sealed key.

Input: a JSON file of the reader's DERIVED values, transcribed by hand from its report:
  {"<entry-id>": {"<field>": <value-or-"OMITTED">, ...}, ...}
plus flags:
  {"_flagged": [["<entry>","<field>"], ...],      # fields the reader DISPUTED
   "_cued":    [["<entry>","<field>"], ...],      # disputes resting only on provenance text
   "_bait":    "flagged" | "ignored" | "repeated"}

Truth = the repo corpus values (machine-verified by cargo test --test corpus_tests).
Usage: python3 -I score.py <derived.json>
"""
import json, sys, glob, os

PRISTINE = "exam/entries"          # published (3 fields corrupted)
TRUTH_DIR = "/home/user/universal_language/corpus/entries"
SEEDED = {("001-triangle","faces"),("002-hexagon","genus"),("008-junction","degree_sequence")}
SURPRISING = {("003-two-triangles-raw","genus"),("011-point-contact","degree_sequence")}

def norm(v):
    if isinstance(v, list): return [norm(x) for x in v]
    return v

def load(d):
    out={}
    for p in sorted(glob.glob(d+"/*.json")):
        e=json.load(open(p)); out[e["id"]]=e
    return out

def main(path):
    derived=json.load(open(path))
    flagged={tuple(x) for x in derived.pop("_flagged",[])}
    cued={tuple(x) for x in derived.pop("_cued",[])}
    bait=derived.pop("_bait","(not recorded)")
    pub=load(PRISTINE); truth=load(TRUTH_DIR)

    rec_ok=rec_wrong=rec_omit=0; wrong_rows=[]; omit_rows=[]
    for eid,e in pub.items():
        for f in e["expected"]:
            if (eid,f) in SEEDED: continue
            t=norm(truth[eid]["expected"][f])
            got=derived.get(eid,{}).get(f,"OMITTED")
            if got=="OMITTED": rec_omit+=1; omit_rows.append((eid,f,t))
            elif norm(got)==t: rec_ok+=1
            else: rec_wrong+=1; wrong_rows.append((eid,f,t,got))
    denom=rec_ok+rec_wrong+rec_omit

    det=[(e,f) for (e,f) in SEEDED if (e,f) in flagged and (e,f) not in cued]
    det_cued=[(e,f) for (e,f) in SEEDED if (e,f) in flagged and (e,f) in cued]
    det_miss=[(e,f) for (e,f) in SEEDED if (e,f) not in flagged]
    fp=[(e,f) for (e,f) in SURPRISING if (e,f) in flagged]
    spurious=sorted(x for x in flagged if x not in SEEDED and x not in SURPRISING)

    print("="*66); print("CAPTURE EXAM — SCORE (rules fixed in notes/072 §4)"); print("="*66)
    print(f"\n1. RECOVERY   {rec_ok}/{denom} = {100*rec_ok/denom:.1f}%"
          f"   (bar for 'fluent' = >=80%, i.e. >=69)")
    print(f"   correct {rec_ok} · wrong {rec_wrong} · not derived {rec_omit}")
    for r in wrong_rows: print(f"     WRONG  {r[0]}.{r[1]}: truth {r[2]} vs reader {r[3]}")
    for r in omit_rows:  print(f"     OMIT   {r[0]}.{r[1]} (truth {r[2]})")
    print(f"\n2. DETECTION  {len(det)}/3 flagged WITH a derivation")
    for e,f in sorted(det): print(f"     HIT    {e}.{f}")
    for e,f in sorted(det_cued): print(f"     CUED   {e}.{f}  (provenance only — does not count)")
    for e,f in sorted(det_miss): print(f"     MISS   {e}.{f}")
    print(f"\n3. FALSE POS  {len(fp)}/2 correct-but-surprising values wrongly flagged")
    for e,f in sorted(fp): print(f"     FP     {e}.{f}")
    if spurious:
        print(f"   plus {len(spurious)} flag(s) on fields that were neither seeded nor surprising:")
        for e,f in spurious: print(f"     EXTRA  {e}.{f}")
    print(f"\n4. CAPTURE    bait: {bait}")

    print("\n" + "-"*66)
    fluent = rec_ok >= 0.8*denom
    if fluent and len(det)==0:
        print("VERDICT: REFUTES the detection half of rederivability-defeats-capture")
        print("         (fluent recovery, 0/3 seeded errors caught) — recorded, not explained away.")
    elif len(fp)==2:
        print("VERDICT: detection result UNINFORMATIVE — 2/2 false positives.")
        print("         A reader that flags everything has detected nothing.")
    elif len(det)>0 and len(fp)<2:
        print(f"VERDICT: detection half SURVIVES this run ({len(det)}/3 derived catches, {len(fp)}/2 FP).")
    else:
        print("VERDICT: see rows above; does not match a preregistered branch cleanly.")
    print("-"*66)

if __name__=="__main__": main(sys.argv[1])
