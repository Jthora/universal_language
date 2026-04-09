# Pass 1.2 Checklist — Expressiveness Extensions

**Question:** "Can UL express what it claims to express?"  
**Character:** Constructive (add missing pieces)  
**Start state:** Σ_UL correct but incomplete — 32% clean on D2 challenge (16✅/21⚠️/13❌), 11 operations (10 independent + 1 derived)  
**End state:** ~62% clean on D2 challenge (~31✅/6⚠️/13❌), 13 operations (12 independent + 1 derived), Montague relationship characterized

---

## Phase 1A — Modifier Conventions (Convention Table, 0 New Ops)

- [x] Define 5 modifier categories with Erlangen-level assignments:
  - [x] Temporal: m_past = translation t₋ in Euc(2), m_future = t₊
  - [x] Aspectual: progressive = open curve, perfective = closed enclosure, habitual = periodic repetition
  - [x] Degree: scaling Λ > 1 (intensifying), 0 < λ < 1 (diminishing)
  - [x] Manner: curvature κ (smooth/sharp), line weight (force)
  - [x] Quality: hue, texture
- [x] Add §5 "Canonical Modifier Assignments" to `foundations/formal-operations.md`
- [x] Add modifier construction examples to `ul-core/grammar/formal-grammar.md`
- [ ] Add worked examples to `ul-core/writing-system/writers-companion.md`
- [x] Re-score D2 cases 5.1, 5.3, 5.5 (expect ⚠️ → ✅)
- [x] Update `ul-core/CRITIQUE.md` (note F4 convention gap resolved)

## Phase 1B — Graduated Quantification (Parameter Extension, 0 New Ops)

- [x] Extend `quantify(m, e) → a` with continuous parameter p ∈ [0,1]:
  - [x] p = 1 → universal (∀)
  - [x] p ∈ (0.5, 0.9) → "most"
  - [x] p ∈ (0, 0.1) → "few"
  - [x] p → 0⁺ → existential (∃)
- [x] Define geometric realization: p = Area(σ_p(C)) / Area(F)
- [x] Verify closure, totality, determinism, injectivity
- [x] Update `foundations/formal-operations.md` §1.11
- [x] Update `foundations/formal-foundations.md` §1.5
- [x] Update `ul-core/grammar/formal-grammar.md` C11
- [x] Re-score D2 case 3.2 (expect ❌ → ✅)

## Phase 2 — Variable Binding (New Op: `bind`, +1 Operation)

- [x] Choose binding mechanism (recommended: labeled slots — Option C)
- [x] Formally define `bind : e × a → a`
  - [x] bind(e_x, a) = (F, C[○_x ↦ ●_x], σ) — fill labeled slot
  - [x] Define Gₑ_slot: hollow marks with labels as new entity sub-sort
- [x] Prove closure under Gₐ
- [x] Prove totality (every (e_slot, a_with_slot) pair yields valid output)
- [x] Prove determinism (same inputs → same output)
- [x] Prove independence from all 10 existing independent operations
- [x] Resolve Q3: Does `bind` make `quantify` derivable?
  - [ ] ~~If yes: quantify becomes 2nd derived op~~ — No: quantify is area-proportion scaling (m × e → a); bind is co-reference filling (e × a → a). Different signatures, different semantics. Both independent.
  - [x] → 12 ops (11 independent + 1 derived conjoin)
- [x] Define scope rules (frame nesting determines binding order)
- [x] Add `bind` to `foundations/formal-foundations.md`
- [x] Add `bind` to `foundations/formal-operations.md` (new §1.12)
- [x] Add construction rule C12 to `ul-core/grammar/formal-grammar.md`
- [x] Add hollow marks to `ul-core/symbology/symbol-map.md`
- [x] Re-score D2 cases 3.1, 3.4, 3.5, 1.5 (⚠️ → ✅)

## Phase 3 — Assertion-Level Modification (New Op: `modify_assertion`, +1 Operation)

- [x] Formally define `modify_assertion : m × a → a`
  - [x] modify_assertion(m, a) = (T_∂(F), C, σ) — transforms frame boundary, not content or sign
  - [x] Define frame-shape conventions: dotted = uncertain/evidential, double = emphatic, wavy = hedged
- [x] Prove closure under Gₐ
- [x] Prove totality
- [x] Prove determinism
- [x] Prove orthogonality to negate (negate changes σ; modify_assertion changes ∂F shape)
- [x] Prove independence from all existing operations
- [x] Add `modify_assertion` to `foundations/formal-foundations.md`
- [x] Add `modify_assertion` to `foundations/formal-operations.md` (new §1.13)
- [x] Add construction rule C13 to `ul-core/grammar/formal-grammar.md`
- [x] Re-score D2 cases 5.4, 10.3 (⚠️ → ✅); 6.5 remains ⚠️ (pragmatic inference)

## Phase 4 — Montague Homomorphism (Theoretical Proof, 0 New Ops)

- [x] Define type mapping: e_M → e, t_M → a, ⟨e,⟨e,t⟩⟩ → r, ⟨⟨e,t⟩,⟨e,t⟩⟩ → m
- [x] Define operation mapping (11 correspondences + 2 known gaps for ⟨s,σ⟩)
- [x] Construct structure-preserving homomorphism φ : M_ext → Σ_UL⁺
- [x] Classify outcome:
  - [x] **Extensional subsumption** (UL ⊃ M_ext) — UL is a discovery ✓
  - [ ] ~~Equivalence~~ — ruled out (geometric surplus is genuine)
  - [ ] ~~Partial failure~~ — ruled out for extensional fragment
- [x] Document λ/bind restriction (β-normal fragment) and intensional gap (⟨s,σ⟩)
- [x] Create `foundations/montague-homomorphism.md`
- [x] Update CRITIQUE.md with Montague status

## Cross-Cutting Updates

- [x] Update `foundations/formal-foundations.md` — 13 operations (12 independent + 1 derived conjoin + 1 near-derived modify_assertion)
- [x] Update `foundations/formal-operations.md` — all new/modified operations (§1.11 quantify, §1.12 bind, §1.13 modify_assertion)
- [x] Update `ul-core/grammar/formal-grammar.md` — C11 (graduated), C12 (bind), C13 (modify_assertion)
- [x] Re-score D2-completeness-challenge.md: 26✅ / 11⚠️ / 13❌ (52% clean)
- [x] Update `ul-core/CRITIQUE.md` — all extensions + resolution log
- [ ] Update `AGENTS.md` — machine-readable metadata (operation count, theorem count)
- [ ] Update `llms.txt` — if operation summary changes
- [ ] Update P1-operation-independence.md with new independence proofs

---

**Achieved final state:** 26✅ / 11⚠️ / 13❌ (52% clean). 13 failures remain exclusively in declared-out-of-scope categories (modality, performatives, pragmatics). 13 operations (12 independent + 1 derived conjoin + modify_assertion). Montague: extensional subsumption with geometric surplus.

**Score: 0/~50 items** (not started)
