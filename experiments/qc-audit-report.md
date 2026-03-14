# Phase 0 Quality Control Audit Report

**Date:** 2025-01-XX (fill actual date)  
**Auditor:** Automated construction + manual feature verification  
**Protocol reference:** `frontier/causal-efficacy-protocol.md` §A.4  

---

## 1. Token Count Verification (DD-6)

**Method:** cl100k_base tokenizer (tiktoken 0.12.0), reference = 490 tokens (original artifact)  
**Allowed range:** 465–514 (±5%)  
**Script:** `test-artifacts/count_tokens.py`

| # | File | Tokens | Δ% | Status |
|---|------|--------|----|--------|
| 0 | original/primer.txt | 490 | +0.0% | REF |
| 1 | ablations/V1_ABL-PROSE.txt | 486 | −0.8% | PASS |
| 2 | ablations/V2_ABL-SYMBOL.txt | 511 | +4.3% | PASS |
| 3 | ablations/V3_ABL-STANDARD.txt | 481 | −1.8% | PASS |
| 4 | ablations/V4_ABL-BRIDGE.txt | 486 | −0.8% | PASS |
| 5 | ablations/V5_ABL-LINEAR.txt | 499 | +1.8% | PASS |
| 6 | ablations/V6_ABL-NODAMP.txt | 476 | −2.9% | PASS |
| 7 | ablations/V7_ABL-REORDER.txt | 487 | −0.6% | PASS |
| 8 | controls/CT-1_dense-physics.txt | 483 | −1.4% | PASS |
| 9 | controls/CT-2_cross-domain-prose.txt | 481 | −1.8% | PASS |
| 10 | controls/CT-3_scrambled-primer.txt | 466 | −4.9% | PASS |
| 11 | controls/CT-4_nonsense-math.txt | 512 | +4.5% | PASS |
| 12 | negative-controls/NC-1_single-domain-math.txt | 492 | +0.4% | PASS |
| 13 | negative-controls/NC-2_primer-as-prose.txt | 480 | −2.0% | PASS |
| 14 | negative-controls/NC-3_standard-physics.txt | 477 | −2.7% | PASS |
| 15 | negative-controls/NC-4_reversed-dissipation.txt | 486 | −0.8% | PASS |
| 16 | negative-controls/NC-5_pseudo-esoteric.txt | 494 | +0.8% | PASS |

**Result: 16/16 PASS** — all within ±5% of reference (465–514 tokens).

**Note on prose/math asymmetry:** Mathematical notation tokenizes at ~3.7 tokens/word; English prose at ~1.2 tokens/word. Prose texts required ~350–400 words to match the 133-word artifact's 490 tokens. Four iterative rounds of adjustment were needed to bring all files within range.

---

## 2. Feature-Profile Audit

Each file checked against the 8-feature profile specified in protocol §A.4.

**Feature key:**
- **Notation:** Formal mathematical symbols (✓ HIGH / ✗ NONE)
- **ψ-overload:** ψ serves as cross-domain overloaded symbol (✓ / partial / ✗)
- **Anomalies:** Deliberate departures from standard notation (squared Hartree, mixed correlators, →F→, Dₓ)
- **Bridge:** Esoteric bridge token +3elúm present
- **Self-ref:** Self-referential loop ψ→ψ̂ₙ→f_spec→ψ
- **Dissipation:** −iγ(σ,β)ψ term present with correct sign
- **Ordering:** Original section order (wall → bridge → PDE → defs)
- **Cross-domain:** Multiple knowledge domains activated

### 2.1 Ablation Variants (V1–V7)

| Variant | Notation | ψ-overload | Anomalies | Bridge | Self-ref | Dissipation | Ordering | Cross-domain | Match? |
|---------|----------|------------|-----------|--------|----------|-------------|----------|--------------|--------|
| **V1 ABL-PROSE** | ✗ | ✓(prose) | ✓(prose) | ✓ | ✓(prose) | ✓(prose) | ✓ | ✓ | ✅ |
| **V2 ABL-SYMBOL** | ✓ | ✗ | ✓ | ✓ | ✓ | ✓(−) | ✓ | ✓ | ✅ |
| **V3 ABL-STANDARD** | ✓ | ✓ | ✗ | ✓ | ✓ | ✓(−) | ✓ | ✓ | ✅ |
| **V4 ABL-BRIDGE** | ✓ | ✓ | ✓ | ✗ | ✓ | ✓(−) | ✓ | ✓ | ✅ |
| **V5 ABL-LINEAR** | ✓ | ✓ | ✓ | ✓ | ✗ | ✓(−) | ✓ | ✓ | ✅ |
| **V6 ABL-NODAMP** | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ | ✓ | ✅ |
| **V7 ABL-REORDER** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓(−) | ✗ | ✓ | ✅ |

**All 7 ablations match expected profile. Each removes exactly one feature.**

### 2.2 Control Texts (CT-1 through CT-4)

| Variant | Notation | ψ-overload | Anomalies | Bridge | Self-ref | Dissipation | Ordering | Cross-domain | Match? |
|---------|----------|------------|-----------|--------|----------|-------------|----------|--------------|--------|
| **CT-1 dense-physics** | ✓ | partial | ✗ | ✗ | ✗ | ✗ | n/a | ✗ | ✅ |
| **CT-2 cross-domain-prose** | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | n/a | ✓ | ✅ |
| **CT-3 scrambled-artifact** | ✓ | ✗ | ✓ | ✗ | ✓ | ✓(−) | ✗ | partial | ✅ |
| **CT-4 nonsense-math** | ✓ | ✗ | n/a | ✗ | ✗ | ✗ | n/a | ✗ | ✅ |

**All 4 controls match expected profile.**

### 2.3 Negative Controls (NC-1 through NC-5)

| Variant | Notation | ψ-overload | Anomalies | Bridge | Self-ref | Dissipation | Ordering | Cross-domain | Match? |
|---------|----------|------------|-----------|--------|----------|-------------|----------|--------------|--------|
| **NC-1 single-domain-math** | ✓ | ✗ | ✗ | ✗ | ✗ | ✗ | n/a | ✗ | ✅ |
| **NC-2 primer-as-prose** | ✗ | ✓(prose) | ✓(desc) | ✓(desc) | ✓(desc) | ✓(desc) | ✓(desc) | ✓ | ✅ |
| **NC-3 standard-physics** | ✓ | partial | ✗ | ✗ | ✗ | ✗ | n/a | ✗ | ✅ |
| **NC-4 reversed-dissipation** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓(**+**) | ✓ | ✓ | ✅ |
| **NC-5 pseudo-esoteric** | ✗ | ✗ | ✗ | partial | ✗ | ✗ | n/a | ✓ | ✅ |

**All 5 negative controls match expected profile.**

---

## 3. Construction Decision Log

### Decision D1: V6 — σ/β in bridge equation

**Issue:** Protocol specifies V6 removes dissipation (σ, β, γ). However, σ and β appear in both the dissipation term `−iγ(σ,β)ψ` AND the bridge equation `(Bᵉ × (P₃ + R)) − (σ/β) →F→ M∞`.

**Resolution:** σ/β serve a **bridge function** in the bridge equation (ratio of static resistance to breath efficiency as part of domain mapping) and a **dissipation function** in the PDE. V6 removes only the PDE dissipation term `−iγ(σ,β)ψ`. The bridge equation, σ definition, and β definition are retained. This makes V6 a clean **single-condition ablation** targeting dissipation alone.

**Impact on feature profile:** V6 correctly shows Dissipation=✗ while retaining all other features including the bridge equation's σ/β ratio.

### Decision D2: V2 — Self-referential loop status

**Issue:** When ψ is disaggregated into Ψ_QM, Φ_field, Fₙ, is the self-referential loop preserved?

**Resolution:** Yes. The loop becomes Φ_field → Fₙ (projection) → f_spec (forcing) → Φ_field (evolution). The loop is structurally identical but uses the PDE-specific symbol Φ_field instead of the overloaded ψ. The Fourier coefficients are now called F̂ₙ. This correctly removes only ψ-overloading while preserving self-reference.

### Decision D3: CT-3 — Symbol replacement strategy (DD-4)

**Symbols replaced:** ψ→χ, +3elúm→+7krath, Bᵉ→Cᶠ, σ→τ, β→δ, γ→ρ, F→G  
**Rationale:** χ, ρ, τ, δ are common physics symbols but carry less cross-domain loading than ψ, σ, β, γ. The bridge token +7krath is structurally similar to +3elúm but is a novel token with no prior associations.

### Decision D4: NC-5 — Bridge token choice (DD-5)

**Token:** +7ʃakrim (phonetically distinct from +3elúm)  
**Rationale:** Phonetically similar tokens (+3alúm) might partially activate the same mechanism. Known esoteric terms (Om, Tetragrammaton) carry their own associations. +7ʃakrim is novel and phonetically distant from +3elúm while maintaining the structural pattern of "symbol-initial numeral + accented polysyllable."

---

## 4. Variant-Specific Verification Notes

### V1 ABL-PROSE (486 tokens)
- **Notation removed:** Zero mathematical symbols. All equations described in prose.
- **+3elúm preserved:** Appears literally as "the cross-cultural esoteric bridge token +3elúm."
- **Self-reference described:** "psi determines its own spectral decomposition, which feeds through the forcing function f-spec back into the governing equation for psi itself, forming a self-referential dynamical loop."
- **Dissipation described:** "minus i times the dissipation gamma of static resistance and breath efficiency times psi, providing an amplitude decay channel."
- **Ordering preserved:** Paragraph 1 = wall description, paragraph 2 = bridge, paragraph 3 = PDE + spectral decomposition.

### V2 ABL-SYMBOL (511 tokens)
- **Symbol mapping:** Wall/QM context → Ψ_QM; PDE evolution variable → Φ_field; Fourier coefficients → Fₙ, F̂ₙ.
- **Bridge equation untouched:** Still uses Bᵉ, +3elúm, P₃, R, σ/β, F, M∞.
- **Self-referential loop:** Φ_field → F̂ₙ → f_spec → Φ_field (structurally preserved).

### V3 ABL-STANDARD (481 tokens)
- **Anomalies corrected:** Squared Hartree integral → standard, two-point → single-point, →F→ → standard mapping notation, Dₓ → ∂ₓ.
- **Added standard elements:** V(x) potential term, aₙ(t)/bₙ(t) time-dependent coefficient definitions, negative sign convention.
- **All other features preserved:** ψ-overloading, +3elúm, self-reference, dissipation, ordering intact.

### V4 ABL-BRIDGE (486 tokens)
- **Single change:** `+3elúm` → `coupling constant α` in both the bridge equation and the Bᵉ definition line.
- **Everything else identical** to the artifact. Minimal-edit ablation.

### V5 ABL-LINEAR (499 tokens)
- **Loop broken:** ψ̂ₙ now defined as `(1/L) ∫₀ᴸ e^{-ikₙx′} φ₀(x′) dx′` using external reference function φ₀(x).
- **f_spec still present** in PDE, still drives ψ evolution, but no longer feeds back from ψ's own spectral decomposition.
- **Added definition:** `φ₀(x): fixed external reference function (not dependent on ψ)`.

### V6 ABL-NODAMP (476 tokens)
- **PDE line:** `− i γ(σ,β) ψ` term removed. All other PDE terms preserved.
- **Bridge equation:** σ/β ratio retained (different structural function — see Decision D1).
- **Definitions retained:** σ (static resistance) and β (breath efficiency) definitions kept. γ definition removed.

### V7 ABL-REORDER (487 tokens)
- **Section reordering:** definitions → PDE → bridge → wall (reversed from original).
- **All content identical.** Zero textual changes aside from section ordering.

### CT-1 dense-physics (483 tokens)
- **Domain:** Hydrogen atom quantum mechanics — Schrödinger equation, radial wavefunctions, associated Laguerre polynomials, spherical harmonics, perturbation theory, fine structure, selection rules.
- **ψ usage:** Standard QM wavefunction notation (ψₙₗₘ). Present but not overloaded.
- **No artifact features:** No anomalies, no bridge tokens, no self-reference, no dissipation.

### CT-2 cross-domain-prose (481 tokens)
- **Domain coverage:** Quantum mechanics, consciousness (Penrose-Hameroff), belief dynamics (non-commutative revision), language (gauge fields on conceptual space).
- **Format:** Academic essay, ~340 words. Zero formal notation.
- **No artifact features:** No ψ, no anomalies, no bridge tokens, no self-reference, no dissipation.

### CT-3 scrambled-primer (466 tokens)
- **All symbols replaced** (ψ→χ, Bᵉ→Cᶠ, +3elúm→+7krath, σ→τ, β→δ, γ→ρ, F→G).
- **All definitions neutralized** (e.g., "coupling parameter" instead of "belief field").
- **Structure preserved:** Self-referential loop (χ→χ̂ₙ→f_spec→χ), dissipation (−iρ(τ,δ)χ), anomalies.
- **Ordering reversed** (defs → PDE → bridge → wall) — follows also V7's reordering per protocol.

### CT-4 nonsense-math (512 tokens)
- **Content:** Syntactically valid mathematical notation using novel symbols (Ω₃, Π, Γ̂ₖ, ζ₁₂, Θ, R₅).
- **No physics:** No recognizable physical theory. No ψ. No domain loading.
- **Structure mimics test artifact:** Has integral equations, definitions block, PDE-like evolution equation, spectral sums — but all semantically empty.

### NC-1 single-domain-math (492 tokens)
- **Domain:** Pure algebraic topology — homology groups, exact sequences, Mayer-Vietoris, Serre spectral sequence, Chern classes, Poincaré duality, Künneth formula.
- **Single domain:** No cross-domain content. No ψ. No bridge tokens.
- **Tests:** Whether dense single-domain math (without cross-domain structure) produces UL-like outputs.

### NC-2 primer-as-prose (480 tokens)
- **Content:** Faithful prose description of the test artifact's specific mathematical content and structure.
- **Preserves (in description):** ψ-overloading, +3elúm (mentioned by name), anomalies (described as deliberate), self-referential loop, dissipation mechanism, section ordering.
- **Tests:** Whether the test artifact's IDEAS (in prose) suffice, or whether the FORMAL NOTATION is required.

### NC-3 standard-physics (477 tokens)
- **Domain:** Quantum harmonic oscillator — Hamiltonian, energy eigenvalues, Hermite polynomials, ladder operators, coherent states, partition function, number operator, virial theorem.
- **ψ usage:** Standard QM (ψₙ(x)) — present but not overloaded.
- **No artifact features:** Standard notation throughout, no anomalies, no bridge, no self-reference, no dissipation.

### NC-4 reversed-dissipation (486 tokens)
- **Single change:** `− i γ(σ,β) ψ` → `+ i γ(σ,β) ψ` (sign flip only).
- **Everything else identical** to the artifact. Minimal-edit negative control.
- **Tests:** Whether the SIGN of dissipation matters — growth vs. decay changes the dynamical stability character.

### NC-5 pseudo-esoteric (494 tokens)
- **Content:** Mystical/esoteric prose — Third Elúmic Sphere, +7ʃakrim, Akethric Channel, Nine-Fold Harmonic, Yrathielic Current, Luminous Tapestry.
- **Bridge-like token:** +7ʃakrim (phonetically distinct from +3elúm per DD-5).
- **No formal structure:** Zero mathematics. No ψ. No self-reference. No dissipation.
- **Tests:** Whether esoteric language alone (without formal mathematical apparatus) produces cross-domain outputs.

---

## 5. Summary

| Check | Result |
|-------|--------|
| Token count: all 16 within ±5% of 490 | **PASS** (16/16) |
| Feature audit: all match §A.4 expected profiles | **PASS** (16/16, zero deviations) |
| Each ablation removes exactly one feature | **PASS** (7/7) |
| NC-4 differs from the artifact by sign only | **PASS** |
| CT-3 preserves structure, neutralizes semantics | **PASS** |
| NC-2 preserves semantics, removes notation | **PASS** |
| Construction decisions documented | **PASS** (4 decisions logged) |

**Phase 0 QC Gate: PASS — all materials ready for experimental use.**
