# Phase 0 — Consolidation: Open Proofs and Gaps

**Status:** � IN PROGRESS — OP-1, OP-4, CT-1–CT-4 complete; OP-2, OP-3, OP-5 remain  
**Goal:** Close or characterize all open proofs/gaps that can be addressed without empirical testing or major new theory

---

## Open Proofs — Closable in Phase 0

These are proofs or formalizations that were identified in Pass 1 or Expeditions 1–2 but not completed. They require only careful writing, not new theory.

### OP-1: ℚ Construction Translation ✅

**Source:** formal-foundations.md §5 (embedding theorem)  
**Status:** COMPLETE — Full Σ_UL construction added as §2.2.1 in `frontier/expedition-one/numbers-and-computability.md`.

The construction expresses ℚ as a quotient field of ℤ using only Σ_UL operations: `predicate` for integer pairs, `embed`+`bind`+`quantify` for equivalence classes, `compose`/`invert` for field operations.

### OP-2: Embedding Injectivity (Step 4)

**Source:** formal-foundations.md, embedding theorem proof  
**What's needed:** Complete the proof that the Montague embedding is injective (distinct Montague types map to distinct Σ_UL sorts).

**Current state:** Steps 1–3 complete. Step 4 (injectivity) stated but not proven.

**Plan:**
1. Enumerate all Montague basic types (e, t, s)
2. Show each maps to a distinct Σ_UL sort
3. Show type constructors (→, ×) preserve distinctness
4. Handle the intensional type ⟨s, α⟩ (currently outside scope — note as Phase 1 dependency)

**Difficulty:** Medium (partially depends on Phase 1 for intensional types)  
**Dependencies:** Phase 1 for full result; extensional fragment closable now

### OP-3: Left Adjoints F₃, F₄

**Source:** frontier/expedition-one/category-of-languages.md §IV  
**What's needed:** Construct F₃ ⊣ U₃ (AffLang → ProjLang) and F₄ ⊣ U₄ (ProjLang → TopLang), or prove they don't exist.

**Current state:** F₁ ⊣ U₁ and F₂ ⊣ U₂ proven by explicit construction. F₃ and F₄ conjectured.

**Plan:**
1. Attempt to construct F₃ by analogy with F₁, F₂
2. The key question: can curvature (Angle → Curve transition) be freely generated?
3. If free construction works, prove the adjunction
4. If not, characterize the obstruction

**Difficulty:** High  
**Dependencies:** None (pure category theory)

### OP-4: Recursion Depth Proof ✅

**Source:** formal-foundations.md §3 (Proposition — UL expressions have finite depth)  
**Status:** COMPLETE — Proven by structural induction in `foundations/formal-foundations.md` §6.5. The free term algebra T(Σ_UL, X) consists of finite trees by construction. Each operation increases depth by exactly 1. No fixpoint equations or circular references are possible. §6.3 “Recursion depth” entry updated to RESOLVED.

### OP-5: 13-Operation Formal Completeness

**Source:** D2-structural-predictions.md (P3), CRITIQUE.md §3  
**What's needed:** Prove that 13 operations are sufficient for all compositional relational semantics within the declared scope.

**Current state:** Sufficiency is empirically supported (37/37 in-scope D2 cases pass) but not formally proven. This is the hardest open proof.

**Plan:**
1. Define "compositional relational semantics" precisely (what is in scope)
2. Show that any meaning expressible in Montague's IL (extensional fragment) is expressible in Σ_UL
3. This follows from the embedding theorem if injectivity (OP-2) is established
4. For completeness beyond Montague: either prove a more general result or characterize the boundary

**Difficulty:** Very High  
**Dependencies:** OP-2 (injectivity)

---

## Open Proofs — Blocked or Deferred

These cannot be closed in Phase 0 and must wait for later work.

| Proof | Blocker | When |
|-------|---------|------|
| Intensional Montague embedding | Requires modality (Phase 1) | Pass 2 Phase 1 |
| Natural emergence (Proof 3/4) | Requires independent validation methodology | Post-Pass 2 |
| Causal efficacy (Proof 4/4) | Requires empirical testing | Not in Pass 2 |
| Full Riemannian metric on G | Complex differential geometry | Expedition Three |
| Topological undecidability | Requires explicit reduction to halting problem | Expedition Three |

---

## Consolidation Tasks (Non-Proof)

### CT-1: pass1-summary.md correction ✅

Corrected. Added a detailed correction note in `docs/planning/audits/improvements/pass1-summary.md` §Phase 1: Operation Count Sweep. Documents ~25 stale instances across 15+ files that were missed by the original Pass 1.4 sweep.

### CT-2: CRITIQUE.md §3.1 audit accuracy ✅

Corrected. Fixed C2 count (14→13), construction rules count (11→13 C1–C13), and F2 log entry (now records final 12+1=13 total).

### CT-3: Update gap-analysis.md ✅

Reviewed. The gap-analysis.md entries for ℚ construction (§1.5) and computability (§1.3) are already current. No OP-4 specific entry exists; the proof was added to formal-foundations.md §6.5 directly.

### CT-4: Formal-operations.md body audit ✅

Audited. All 13 operations have complete set-theoretic definitions, type signatures, domain/codomain, geometric interpretations, and closure/totality/determinism proofs. Added a design note to §1.5 (conjoin) cross-referencing its derived status in §3.4. No TODOs or placeholders remain.

---

## Priority Ordering

| Priority | Item | Effort | Impact |
|----------|------|--------|--------|
| P0 | Stale-ref sweep (see stale-refs-sweep.md) | Low | High — consistency |
| P1 | OP-4: Recursion depth proof | Low | Medium — closes easy gap |
| P2 | CT-1, CT-2: Correction notes | Low | Medium — accuracy |
| P3 | OP-1: ℚ construction translation | Medium | Medium — worked example |
| P4 | CT-4: formal-operations.md body audit | Medium | Medium — completeness |
| P5 | OP-2: Injectivity (extensional) | Medium | High — enables OP-5 |
| P6 | CT-3: gap-analysis.md update | Low | High — navigation |
| P7 | OP-3: F₃, F₄ adjoints | High | Medium — category theory |
| P8 | OP-5: Formal completeness | Very High | Critical — but may not be closable |

---

## Success Criteria for Phase 0

- [ ] Zero stale "11 operations" in active documents
- [ ] Recursion depth proof written
- [ ] ℚ construction fully translated to Σ_UL
- [ ] Embedding injectivity (extensional fragment) proven or obstruction characterized
- [ ] formal-operations.md complete for all 13 operations
- [ ] gap-analysis.md updated
- [ ] No regression in any existing proofs
