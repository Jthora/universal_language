# ENGINE CORE — the M2 execution layer, v1

**Present tense: everything here exists and is test-backed.** Code:
`ul-forge/crates/ul-core/src/engine.rs`. Derivation record: `research/notes/051`, `057`.

## The model

Interaction-net reduction (Lafont 1990, read via Gay 1991): agents with a **principal port** and
ordered auxiliary ports; rewriting happens **only on active pairs** — two agents connected
principal-to-principal; each `(symbol, symbol)` pair has at most one rule, applied locally.

**Meaning as behavior (M2):** a program's meaning is its reduction. `2 + 2` is not evaluated by an
interpreter that knows arithmetic — it is a configuration whose local rewrites *are* the addition
(test `two_plus_two_reduces_to_four`, using exactly the `Add/S/Z` system from the primary source).

## The guarantees, each with its test

| Property | Mechanism | Test |
|---|---|---|
| **No free delete** | a rule whose RHS drops a wire is **refused at registration** | `linearity_is_enforced_at_registration` |
| **No free copy** | a rule using a wire twice is refused; copying is an explicit `Dup` agent | same + `copying_is_an_agent_not_an_ambient_right` |
| **Deletion is explicit** | `Erase` consumes structure one interaction at a time | `deletion_is_an_agent_not_an_ambient_right` |
| **Order-independence** | different reduction orders reach the same normal form | `reduction_order_does_not_change_the_answer` *(smoke test — strong confluence is Lafont's theorem, not re-proved here)* |
| **Programs are expressions** | `Net::to_map()` exports any state as a combinatorial map — rotation = port order, **principal first** (Gay's serialization) — and every intermediate state passes `validate()` | `every_net_state_is_a_valid_map_with_principal_first_rotations` |

The first two rows are `QUANTUM-IS-LINEAR-DISCIPLINE` as running code: the resource discipline is
structural, not a convention anyone must remember.

## Limits, plainly

- **v1 executes; it does not yet mean.** The engine gives UL's formal tier its behavior layer; the
  connection from net-behavior to world-reference is M3's job and is not code.
- No optimal-reduction machinery (Lamping/Asperti–Guerrini), no λ-encoding, no typed layer — UQPL's
  type system sits *above* this and does not exist yet.
- Strong confluence is inherited from the model's theory and smoke-tested, not formally verified
  here. Rule coverage (an agent pair with no rule simply never fires) is by construction, not
  checked totality.
- The S8 debt on the substrate's *universality* (Lafont 1997 unread) stands — v1's demos are
  arithmetic, not a universality proof.
