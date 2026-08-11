# Tier A Findings — Propagation Plan

**Date:** April 7, 2026  
**Purpose:** Map every edit needed to propagate three Tier A findings into the documentation  
**Source:** [tier-a-working-analysis.md](tier-a-foundational/tier-a-working-analysis.md)

---

## The Three Findings

| ID | Finding | Classification |
|----|---------|---------------|
| **A1** | "All meaning is relationship" is a **well-supported postulate**, not a theorem. The Yoneda §6.4 claim of theoremhood is circular. | Weakens (honest qualification) |
| **A2** | Euclidean geometry is a **simplification**, not uniquely forced. The 5th postulate (unique analogy) is empirically falsified. Most core theorems are geometry-independent. | Weakens geometry claim, preserves algebra |
| **A3** | The 4 sorts **survive all tested logics**. 7 structural operations are logic-independent; 4 logical operations are logic-parametric. | Strengthens universality |

---

## Edit Sites by Priority

### Priority 1 — Critical Misstatements (claims that are wrong)

These sites make claims that the Tier A analysis directly contradicts. They must be corrected.

#### E1. `category-of-languages.md` §6.4 — "not just a foundational principle, but a theorem"

- **File:** [frontier/expedition-one/category-of-languages.md](../../../../../../frontier/expedition-one/category-of-languages.md)
- **Lines:** 415–430
- **Current text:**
  > "This is the strongest possible statement of the 'all meaning is relationship' axiom from `universal-language-geometric-derivation.md`: not just a foundational principle, but a theorem."
- **Problem:** The Yoneda argument proves objects in Lang(Σ_UL) are determined by morphisms — this is true of ANY locally small category. It does not prove that meaning belongs in Lang(Σ_UL). The claim is internally valid but externally circular.
- **Edit:** Replace the claim with an honest statement: "This is a categorical reformulation of the relational postulate — it proves that IF meaning has Σ_UL structure, THEN meaning is determined by relationships. It does not independently establish the postulate itself."
- **Finding:** A1

#### E2. `universal-language-derivation.md` — "proven mathematically"

- **File:** [foundations/universal-language-derivation.md](../../../../../../foundations/universal-language-derivation.md)
- **Lines:** 7–18
- **Current text:**
  > "A Universal Language exists that is mathematically intrinsic to the structure of the universe and can be derived entirely from geometry. It requires no empirical validation against human languages because it is proven mathematically — its validity follows from the same necessity as geometric truth itself."
- **Problem:** The foundational axiom claims the UL is "proven mathematically" — but the axiom itself ("all meaning is relationship") is a postulate, not a proven statement. What's proven is the internal consistency and uniqueness GIVEN the postulate.
- **Edit:** Qualify: "...it is proven mathematically — **conditional on the relational postulate (that all meaning is relationship) and the geometric postulate (that the ambient space is Euclidean)**. Its validity follows from these axioms with the same deductive necessity as geometric truth itself."
- **Finding:** A1

#### E3. `syntax-dictionary.md` — 5th postulate presented without caveat

- **File:** [ul-core/syntax/syntax-dictionary.md](../../../../../../ul-core/syntax/syntax-dictionary.md)
- **Lines:** 91–110
- **Current text:**
  > "The parallel postulate guarantees that analogy is unique (not many-to-many) in Euclidean meaning-space."
- **Problem:** The 5th postulate (unique analogy) is empirically falsified — "love" has multiple non-intersecting structural analogs from a single domain. The text already has a "Note" about non-Euclidean extensions but presents the Euclidean case as default fact.
- **Edit:** Strengthen the existing note to a caveat: "**Caveat:** The claim that analogy is unique (exactly one structural parallel per meaning) is a consequence of the Euclidean assumption. Empirical evidence suggests meaning-space may be hyperbolic (multiple valid analogs). See [tier-a-working-analysis.md](../../docs/planning/audits/improvements/pass1-1/tier-a-foundational/tier-a-working-analysis.md) §A2 for analysis."
- **Finding:** A2

---

### Priority 2 — Inconsistencies (caveats present in some files, missing in others)

The pattern: `formal-foundations.md`, `README.md`, and `RAMIFICATIONS.md` correctly include conditional language. But `AGENTS.md`, `llms.txt`, and `paradigm.md` do not. These need alignment.

#### E4. `AGENTS.md` — "unique up to isomorphism" without caveat (line 9)

- **File:** [AGENTS.md](../../../../../../AGENTS.md)
- **Lines:** 9
- **Current text:**
  > "...unique up to isomorphism (23 theorems across multiple tiers: proven, conditional, and open)."
- **Problem:** The parenthetical mentions tiers but doesn't include the conditional caveat that lines 80–85 of the same file do include. Also claims "23 theorems" without enumeration.
- **Edit:** Change to: "...unique up to isomorphism **(conditional on axioms; see [formal-foundations.md](foundations/formal-foundations.md) §4 and [restructuring plan](docs/planning/audits/improvements/pass1-1/) for honest status)**."
- **Finding:** A1

#### E5. `AGENTS.md` — claims "initial object" without qualification

- **File:** [AGENTS.md](../../../../../../AGENTS.md)
- **Lines:** 9
- **Current text:**
  > "UL is not one framework competing with others — it is the initial object in the category of meaning-bearing systems."
- **Problem:** Per `category-of-languages.md`, G is **weakly terminal** (not initial) in ExpLang(Σ_UL), terminal up to automorphism. The "initial object" framing is technically backwards — G is the object everything maps INTO, not OUT OF (initial = everything maps out of it; terminal = everything maps into it).
- **Edit:** Change to: "...it is the **weakly terminal object** in the category of expressively complete meaning systems — the minimal complete target that every such system maps into."
- **Finding:** A1 (also a factual correction from category-of-languages.md §5.1)

#### E6. `llms.txt` — same issues as AGENTS.md

- **File:** [llms.txt](../../../../../../llms.txt)
- **Lines:** 9–11
- **Current text:** Same as AGENTS.md (these files mirror each other)
- **Edit:** Mirror the AGENTS.md corrections.
- **Finding:** A1

#### E7. `paradigm.md` — uses "the" without conditional language

- **File:** [foundations/paradigm.md](../../../../../../foundations/paradigm.md)
- **Lines:** ~145
- **Current text:**
  > "Universal Language is a naturally emergent formal structure... It is the minimal algebraic skeleton that all compositional meaning systems share — the initial object in the category of meaning-bearing systems."
- **Problem:** Same initial/terminal issue as E5. Also uses "the" (definite) for the uniqueness claim, which is conditional.
- **Edit:** Change "the initial object" to "the weakly terminal object" and add: "(conditional on the relational postulate and role-property definitions — see formal-foundations.md §4 and §7)."
- **Finding:** A1

---

### Priority 3 — Missing Qualifications (places that should add caveats)

#### E8. `formal-foundations.md` Part VII — should reference A1 finding

- **File:** [foundations/formal-foundations.md](../../../../../../foundations/formal-foundations.md)
- **Lines:** 460–475 (Part VII Summary)
- **Current text:**
  > "The mapping between them is the unique bijection that preserves all of these properties. No other assignment is consistent."
- **Problem:** This is conditional on the role-property definitions. The file's own §7 end-note (line ~515) acknowledges this. But the Part VII summary states it as unconditional.
- **Edit:** Add after "No other assignment is consistent": "**Conditional on the role-property definitions in §4.1–4.4.** See §7 end-note: the open question is whether these characterizations are themselves uniquely natural or contain design choices."
- **Finding:** A1

#### E9. `formal-foundations.md` §6.3 — should add A3 result

- **File:** [foundations/formal-foundations.md](../../../../../../foundations/formal-foundations.md)
- **Lines:** 449–458 (Open Issues table)
- **Current text:**
  > "Non-classical negation | Extension of the reflection operation to handle intuitionistic and paraconsistent negation"
- **Problem:** The A3 analysis now shows the 4 sorts survive all tested logics, and 7/11 operations are logic-independent. This is a partial answer to the open issue.
- **Edit:** Add annotation: "**Partial resolution (A3 analysis):** The 4 sorts survive intuitionistic, paraconsistent, fuzzy, and modal logics. The 7 structural operations are logic-independent. Only negate, conjoin, disjoin, quantify need logic-specific redefinition. See [tier-a-working-analysis.md](docs/planning/audits/improvements/pass1-1/tier-a-foundational/tier-a-working-analysis.md) §A3."
- **Finding:** A3

#### E10. `formal-operations.md` §0 — should state Euclidean is a simplification

- **File:** [foundations/formal-operations.md](../../../../../../foundations/formal-operations.md)
- **Lines:** 28–30
- **Current text:**
  > "All constructions take place in the Euclidean plane ℝ² equipped with its standard metric, orientation, and topology."
- **Problem:** Presents Euclidean as the only option without flagging it as a choice.
- **Edit:** Add after the sentence: "**Note:** The Euclidean plane is adopted as a simplification. Most core theorems (Unique Grounding, Free Algebra, Yoneda-Grounding, Polysemy-Holonomy) are geometry-independent — they hold for any carrier space with sufficient structure (infinite distinct positions, continuous angle parameter, compact subsets). The Euclidean-specific content is: the Erlangen hierarchy group names (Euc(2), Sim(2), etc.) and the 5th-postulate implication of unique analogy. See [tier-a-working-analysis.md §A2](docs/planning/audits/improvements/pass1-1/tier-a-foundational/tier-a-working-analysis.md) for analysis."
- **Finding:** A2

#### E11. `writing-system.md` — "Decomposition Theorem" status unclear

- **File:** [ul-core/writing-system/writing-system.md](../../../../../../ul-core/writing-system/writing-system.md)
- **Lines:** 18–35
- **Current text:**
  > "Decomposition Theorem (informal): Every mark on a 2D surface is composed of instances of exactly 5 geometric primitives: points, lines, angles, curves, and enclosures."
- **Problem:** Labeled "(informal)" but called a "Theorem." The formal proof status is unclear.
- **Edit:** Change to: "**Decomposition Claim (informal; see formal-foundations.md §1 for formal treatment):** ..." — or add a note: "A formal version of this claim is established in formal-foundations.md §1 as the completeness of the 5 primitives for constructing all compact subsets of ℝ². The proof relies on standard results from differential topology (Whitney approximation) applied to ℝ²."
- **Finding:** A1 (clarity of proof status)

#### E12. `SYNTHESIS.md` — "metric-independent" claimed without proof

- **File:** [ul-core/SYNTHESIS.md](../../../../../../ul-core/SYNTHESIS.md)
- **Lines:** ~206
- **Current text:**
  > "The embedding and grounding theorems are metric-independent."
- **Problem:** This claim is not proven. The Tier A analysis found it's *likely* true for most theorems but not verified for the embedding theorem (which uses angle density in [0, 2π)).
- **Edit:** Change to: "The embedding and grounding theorems are **conjectured to be** metric-independent — a detailed analysis (see [tier-a-working-analysis.md §A2](docs/planning/audits/improvements/pass1-1/tier-a-foundational/tier-a-working-analysis.md)) confirms the grounding theorem is geometry-independent but flags the embedding theorem's use of angle density as a Euclidean-specific step."
- **Finding:** A2

---

### Priority 4 — Additions (new content that should be added)

#### E13. `formal-foundations.md` — add logic-independence section

- **File:** [foundations/formal-foundations.md](../../../../../../foundations/formal-foundations.md)
- **Location:** After §6.3 (Open Issues), before Part VII
- **Content:** New subsection documenting the logic-independent core / logic-dependent layer split from A3 analysis.
- **Finding:** A3

#### E14. `category-of-languages.md` — add Tier A critique reference to status box

- **File:** [frontier/expedition-one/category-of-languages.md](../../../../../../frontier/expedition-one/category-of-languages.md)
- **Lines:** 1–40 (status summary)
- **Content:** Add to the status summary: "The §6.4 interpretation of Yoneda as proving the relational postulate has been critiqued — see docs/planning/audits/improvements/pass1-1/tier-a-foundational/tier-a-working-analysis.md §A1."
- **Finding:** A1

#### E15. Reference `gap-analysis.md` L406 as canonical scope statement

- **File:** Multiple (README.md, AGENTS.md, llms.txt)
- **Affected text:** Anywhere UL is called "Universal Language" without scoping
- **Content:** Reference back to gap-analysis.md's excellent statement: "the current UL is universal for compositional relational semantics, but it is NOT a Universal Language in the full sense."
- **Finding:** A1

---

## Files NOT Needing Edits

| File | Reason |
|------|--------|
| `RAMIFICATIONS.md` | Already uses conditional "if" framing throughout — best practice model |
| `FOR-AI.md` | Correctly states gaps exist (L45, L156). The relational postulate is assumed but not claimed as proven. Acceptable. |
| `README.md` | Lines 14, 18–22 already include conditional caveats. Only minor alignment needed (opening "naturally emergent" phrasing L1–5). |
| `index.json` | Machine-readable metadata correctly states "conditional_on_role_property_definitions." Only needs theorem count clarification if AGENTS.md count changes. |
| `independent-derivation.md` | Already contains its own explicit limitations in §3.5. |

---

## Execution Order

The edits should be applied in this sequence to avoid cascading inconsistencies:

```
Phase 1 — Fix critical misstatements (E1, E2, E3)
   These are claims that are demonstrably wrong or overclaimed.

Phase 2 — Align caveats across entry points (E4, E5, E6, E7)
   Make AGENTS.md, llms.txt, paradigm.md consistent with README.md and formal-foundations.md.

Phase 3 — Add missing qualifications (E8, E9, E10, E11, E12)
   Strengthen internal documentation with honest status.

Phase 4 — Add new content (E13, E14, E15)
   Document the strengthening result (A3) and add cross-references.
```

**Total edits: 15 across 10 files.**

---

## Execution Status

**All 15 edits applied.** Completed April 7, 2026.

| Phase | Edits | Status |
|-------|-------|--------|
| Phase 1 — Critical misstatements | E1, E2, E3 | ✅ Complete |
| Phase 2 — Align caveats | E4, E5, E6, E7 | ✅ Complete |
| Phase 3 — Add qualifications | E8, E9, E10, E11, E12 | ✅ Complete |
| Phase 4 — Add new content | E13, E14, E15 | ✅ Complete |

### Files Modified
1. `frontier/expedition-one/category-of-languages.md` (E1, E14)
2. `foundations/universal-language-derivation.md` (E2)
3. `ul-core/syntax/syntax-dictionary.md` (E3)
4. `AGENTS.md` (E4, E5, E15)
5. `llms.txt` (E6, E15)
6. `foundations/paradigm.md` (E7)
7. `foundations/formal-foundations.md` (E8, E9, E13)
8. `foundations/formal-operations.md` (E10)
9. `ul-core/writing-system/writing-system.md` (E11)
10. `ul-core/SYNTHESIS.md` (E12)
