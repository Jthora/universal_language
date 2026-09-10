# 058 — Construction item 4, first result: the 𝔽₀ floor of reading invariance

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** begins Construction item 4 of `055` — the theorem everything routes through. Also
records the merge to `main` (working branch per owner instruction, 2026-08-12).
**Question:** how much of "it must not matter how you look at it" can be machine-checked today?

**Answer: the presentation floor — and the whole lexicon corpus now carries it.**

---

## 1. What was built

**𝔽₀ = ⟨relabel · mirror · subdivide⟩** — the generated class of readings-as-presentation-changes:
different numbering/orientation bookkeeping, the opposite orientation convention (the ledger's
ℤ/2), and different sampling density of the same stroke. New: `relabel()` (an isomorphism
commuting with α by construction) and `essential_invariants()` — components, face count, genus,
degrees ≠ 2, each stable under each generator *for a stated reason*.

**Checked on every CI run:**
- five subject configurations × a battery of mirrors, random relabelings, iterated subdivisions,
  and their composite — invariants identical throughout;
- **every corpus entry re-read three ways** — the lexicon's ground truth survives how the reader
  numbers, orients, or samples the drawing.

## 2. What this is and is not

**Is:** the first machine-checked instance of the project's central theorem-target, and a real
property of the artifact: the lexicon cannot be broken by bookkeeping differences between
receivers. Also the right *shape* for the eventual proof — invariance argued generator-by-
generator, then inherited by the generated class.

**Is not:** the theorem. 𝔽₀ contains only readings **we defined**, with designed-in invariance.
The target quantifies over *all* reasonable readings, and its hard part is exactly what 𝔽₀
dodges — characterizing "reasonable" without smuggling our own reading convention into the
definition. That deep failure mode stays named on `READING-INVARIANCE-TARGET`, which stays
CONJECTURED. `spec/reading-invariance-v1.md` exists so the difference is never blurred.

## 3. Also this cycle: merged to `main`

Owner instruction: work out of `main`. Done — fast-forwarded to remote, merged the 91-commit
rebuild, one conflict (README) resolved in the rebuild's favor: remote main's four commits had
added a *"finite set of universal semantic primitives"* hypothesis block, which is the **retired**
framing; the registered contract supersedes it. All checkers and 368 tests green on `main` before
push. The restructure branch remains for history.

## 4. What changed

- `map.rs`: `relabel`, `essential_invariants`, the 𝔽₀ battery test; corpus harness gains
  `every_corpus_entry_is_reading_invariant`. Workspace: **370 tests.**
- `spec/reading-invariance-v1.md` — third file in `spec/`.
- `claims.yaml`: `READING-INVARIANCE-F0` (VERIFIED); progress line on `READING-INVARIANCE-TARGET`.
- **Open (the item continues):** formalize the reading class as a category of signature morphisms
  (institutions) and characterize "reasonable" — the actual mathematics; Module Q authoring is the
  other unblocked front.
