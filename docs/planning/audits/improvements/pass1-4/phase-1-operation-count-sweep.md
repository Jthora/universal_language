# Phase 1 — Operation Count Sweep

**Status:** 🔲 NOT STARTED  
**Risk:** Low (mechanical find-and-replace with verification)  
**New operations:** 0  
**Dependencies:** Pass 1.3 complete (need stable operation set before sweeping refs)  
**Blocks:** All other Pass 1.4 phases (must fix counts before adding content)

---

## Problem Statement

The algebra was extended from 11 to 13 operations in Pass 1.2, and top-level files (AGENTS.md, llms.txt, formal-foundations.md) were updated. But many interior documents still reference "11 operations," "Σ_UL" (without ⁺), or enumerate only the original 11 operations in lists and tables.

---

## Known Stale References

From the synchronization audit:

| File | Location | Current Text | Correct Text |
|------|----------|-------------|-------------|
| `ul-core/writing-system/writing-system.md` | §4.2 | "all 11 operations" | "all 13 operations" |
| `ul-core/writing-system/writers-companion.md` | §5.2 | "11 Σ_UL operations" | "14 Σ_UL⁺ operations" |
| `ul-core/writing-system/writers-companion.md` | §5.2 decision tree | Q1–Q11 only | Add Q12 (bind), Q13 (modify_assertion) |
| `ul-core/syntax/syntax-dictionary.md` | §III heading | "THE 11 SYNTACTIC OPERATIONS" | "THE 14 SYNTACTIC OPERATIONS" |
| `ul-core/syntax/syntax-dictionary.md` | §VIII table | 11 rows | 14 rows |
| `ul-core/grammar/grammar-book.md` | §VIII | 11-row reference table | 14-row reference table |
| `index.json` | `formal_system.operations` | `11` | `14` |

### Additional References to Find

The above are known from the audit. A full `grep` sweep is needed for:

- `"11 operations"` — all files
- `"11 ops"` — all files  
- `"ten independent"` or `"10 independent"` — may need update to 12
- `"Σ_UL"` without `⁺` — some contexts should be updated (formal spec), others should not (historical references)
- Operation enumerations that list exactly 11 items

---

## Approach

### Step 1: Automated Grep Sweep

```bash
grep -rn "11 operations\|11 ops\|eleven operations" --include="*.md" --include="*.json" .
```

Review each match. Classify as:
- **UPDATE** — stale reference that should say 14
- **KEEP** — historical/resolution-log reference that correctly describes the old state
- **REVIEW** — ambiguous; needs context check

### Step 2: Fix All UPDATE Matches

Apply edits. For operation lists/tables that enumerate only 11 items, add the 3 new operations (bind, modify_assertion, graduated quantify note).

### Step 3: Verify No Regression

After all edits, re-run the grep to confirm zero stale references remain (outside historical context).

---

## Deliverables

- [ ] Grep sweep: catalog all "11 operations" references
- [ ] Update all stale references to 14
- [ ] Update `index.json` operation count
- [ ] Verify `formal-foundations.md` operation count references are all consistent
- [ ] Verify `formal-operations.md` TOC/heading numerals reflect all 14
- [ ] Re-grep to confirm zero stale references (excluding resolution logs)
