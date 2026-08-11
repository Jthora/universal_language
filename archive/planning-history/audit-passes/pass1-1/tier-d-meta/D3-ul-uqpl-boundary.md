# D3 — Where Does UL End and UQPL Begin?

**Tier:** D (Meta)  
**Targets:** `ul-core/uqpl/`, `foundations/formal-foundations.md`, `ul-core/README.md`  
**Dependencies:** Independent  
**Effort:** 1–2 weeks for boundary analysis

---

## The Problem

The repository contains two related systems:
1. **UL (Universal Language)** — 5 primitives, 4 sorts, 13 operations, geometric grounding
2. **UQPL (Universal Quantum Programming Language)** — in `ul-core/uqpl/`

The boundary between them is unclear. Questions:
- Is UQPL a *specialization* of UL (a Σ_UL-algebra for quantum computing)?
- Is UQPL an *extension* of UL (adding operations not in Σ_UL)?
- Is UQPL a *separate system* that happens to share some primitives?

## The Questions

### Q13a: Do UQPL's operations match Σ_UL operations?

If UQPL is a Σ_UL-algebra, then every UQPL operation should be:
- One of the 11 Σ_UL operations, or
- A composition of Σ_UL operations, or
- A specialization of a Σ_UL operation to the quantum domain

If UQPL introduces operations NOT derivable from Σ_UL, then either:
- Σ_UL is incomplete (needs more operations), or
- UQPL is a separate system (not a UL algebra)

### Q13b: What IS the relationship?

Possible relationships:

| Model | Meaning | Implication |
|-------|---------|-------------|
| **UQPL = Σ_UL-algebra** | UQPL interprets the 4 sorts and 11 ops in the quantum domain | UL universality claim includes quantum computing |
| **UQPL extends Σ_UL** | UQPL adds new sorts or operations | Either UL is incomplete or quantum meaning needs extra structure |
| **UQPL parallels UL** | Independent system sharing notation/aesthetics | No formal relationship; they're separate projects |
| **UQPL specializes UL** | UQPL uses a *subset* of Σ_UL for its domain | UL is more general; UQPL is a focused application |

### Q13c: Why does this matter for the audit?

If UQPL is claimed as a Σ_UL-algebra, then the embedding theorem should apply: there should be a structure-preserving map from G → UQPL. This is a testable claim — either the map exists (and we can construct it) or it doesn't (and UQPL is not a UL algebra).

If UQPL introduces genuinely new operations, these operations might be candidates for the "missing operations" identified in B3 (e → r gap) or B4 (independence analysis). The quantum domain might expose algebraic structure that the geometric model misses.

## Investigation Plan

1. **Read all UQPL documentation** in `ul-core/uqpl/`
2. **List all UQPL operations** and their type signatures
3. **Map each UQPL operation** to a Σ_UL operation or composition
4. **Identify any unmapped operations** — these are either UQPL-specific extensions or evidence of Σ_UL gaps
5. **Determine the formal relationship** — algebra? extension? parallel system?

## Deliverable

A document with:
1. Complete UQPL operation inventory with type signatures
2. Mapping to Σ_UL operations (or "NO MAPPING" for each)
3. Classification: algebra / extension / parallel system
4. Implications for Σ_UL completeness

## Status

**Status:** ✅ RESOLVED (April 2026) — Full alignment analysis completed. UQPL is a programming language inspired by UL's geometric foundations, not a strict Σ_UL-algebra. 3 exact operation matches, 3 approximate, 5 UQPL-only, 4 Σ_UL-only. Relationship analogous to group theory vs GAP. See `ul-core/uqpl/D3-ul-uqpl-analysis.md`. Recommended future path: Option B (UQPL as computational extension of Σ_UL).
