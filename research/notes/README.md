# research/notes/

**Iterative working notes. One numbered folder per research cycle.**

This is where work happens before it is settled. A cycle folder is cheap to create, never edited
after it closes, and either promotes something into `framework/` `cure/` `engineering/` or records
why it didn't.

```
research/notes/
├── 001-triangulation/
├── 002-cycle-answers/
└── NNN-short-slug/
    ├── README.md      the note itself (template below)
    └── …              data, scripts, derivations as needed
```

## Rules

1. **Number sequentially, never renumber.** `NNN-short-slug`, zero-padded to three digits.
2. **Fill §1 of the template *before* searching.** That is rule **S2** — literature-pull is only
   detectable against a recorded prior. A note whose expectations were written afterward is worth
   much less and should say so.
3. **A closed cycle is not edited.** Corrections go in a later cycle that links back. Same principle
   as `FAILURES.md`: the record of what you thought at the time is the data.
4. **Promotion is explicit.** When a position stabilizes, move it into the appropriate themed folder
   and record the promotion in §5. Notes are working material; the themed folders hold what we
   currently hold.
5. **Register the claims.** Anything asserted lives in `claims.yaml` with a tier, or it is not a
   claim of this project.

## Template

Copy this into `NNN-slug/README.md`.

```markdown
# NNN — <title>

**Opened:** YYYY-MM-DD
**Question:** <the one thing this cycle is trying to settle>

## 1. Before searching  ← WRITE THIS FIRST (S2)

**Expected to find:**
**Would change the plan if:**
**Objects mathematical?** yes/no — if yes, the burden is to find the theorem, not design a study (T5)

## 2. Searches run

| Query | Direction | Result |
|---|---|---|
| … | supporting / **adversarial (R1)** | … |

**Adversarial search on every negative is mandatory (R1).** If this cycle closes a line of work and
this table has no adversarial row, the cycle is not finished.

## 3. Findings

**Counter-evidence: easy or hard to find?** ← log it (S7)

## 4. Negatives recorded

For each, all of: **scope** (R2) · **what formalization failed, not what claim** (R3) ·
**revival condition** (R4) · **steelman stated before the kill** (R6)

## 5. What changed

- Claims added/retired in `claims.yaml`:
- `FAILURES.md` entries:
- Promoted to a themed folder:
- Left open:
```

## Index

| Cycle | Question | Outcome |
|---|---|---|
| `001-triangulation` | Where do purpose, notation, and the safety application actually meet? | Cure benchmark, assumption-derivation, emergent-communication gap |
| `002-cycle-answers` | Open questions from the preceding cycle | SHACL checking, sheaf route, taught-notation, cognitive dimensions |
