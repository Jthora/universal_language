# 057 — Construction item 3: the M2 engine, v1

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** executes Construction item 3 of `055`; builds what `051` found paved
**Question:** can meaning-as-behavior run on our substrate — with the linear discipline enforced
rather than promised?

**Yes. Six tests, all green on the first run, second file in `spec/`.**

---

## 1. What was built

`engine.rs`: interaction-net reduction — systems (symbols + rules), nets (agents + total wiring),
active-pair detection, local rewriting, normalization. The demo system is **exactly the `Add/S/Z`
arithmetic from the primary source** (Gay 1991 §1), plus explicit `Erase` and `Dup`.

**The three results that matter:**

1. **`2 + 2 → 4` by local rewriting alone** — three interactions, no interpreter that knows
   arithmetic. Meaning as behavior, running.
2. **The linear discipline is structural.** Rule registration *refuses* an RHS that drops a wire
   (*"no free delete"*) or uses one twice (*"no free copy"*). Deleting and copying exist only as
   the explicit `Erase`/`Dup` agents. **`QUANTUM-IS-LINEAR-DISCIPLINE` went from a claim about the
   literature to a property the code enforces.**
3. **Programs are expressions.** `Net::to_map()` exports any engine state as a combinatorial map —
   each agent a vertex whose rotation is its port order **principal first**, which is precisely
   Gay's *"anticlockwise round the agent"* serialization — and the test walks a full reduction
   asserting **every intermediate state passes the grammar's `validate()`**. The `051` finding
   ("its programs are our fixed-point object") is now machine-checked in our own code rather than
   read off a paper.

## 2. Honest boundaries (spec/engine-core.md carries them too)

- **v1 executes; it does not yet mean.** M3 (world-reference) is not code. What exists is the
  behavior layer UQPL's "Programming" word requires.
- Confluence is **smoke-tested** (two reduction orders, same normal form), inherited as Lafont's
  theorem — not re-proved. No optimal reduction, no λ-encoding, no type layer.
- The **S8 debt stands**: Lafont 1997 (combinator universality) remains unread; v1 demonstrates
  arithmetic, not universality.

## 3. Also in this cycle: CI made honest for the whole workspace

Clippy gates the workspace with `-D warnings`, and **12 pre-existing warnings across
`ul-api`/`ul-game`/`ul-mcp` (plus a fmt drift in the Python bindings) had the branch CI red before
this cycle touched anything** — F-032's shape again: the enforcement was believed, not observed.
All fixed (mechanical: collapsible ifs, literal-bool asserts, a redundant closure, formatting);
**workspace: 0 warnings, 368 tests, fmt clean.** The engine lands with CI actually green.

## 4. What changed

- `engine.rs` (new module) + 6 tests; `spec/engine-core.md` (second file in `spec/`).
- `claims.yaml`: `M2-ENGINE-V1-EXISTS` (VERIFIED, scope-limited);
  `QUANTUM-IS-LINEAR-DISCIPLINE` and `INTERACTION-NETS-CARRY-ROTATION` each gain a
  machine-checked leg (noted in their scopes).
- `seed/INDEX.yaml`: execution → in-progress. `curriculum/`: Module Q unblocked (the engine it
  was gated on now exists in v1).
- **Open:** the typed layer (UQPL proper) over the engine; λ/combinator encodings; the M3
  connection; Lafont 1997 (S8).
