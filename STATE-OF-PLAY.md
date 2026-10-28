# STATE-OF-PLAY

**Where the program stands, 2026-10-28** — through note `079`. Rewritten in full; §8 says how it is
maintained.

---

## 0. What "universal" means here

`UNIVERSAL-IS-MATHEMATICAL-SCOPE` (DESIGN-CHOICE, owner ruling 2026-10-14):

> **Universal ranges over the domain of mathematical identity, not the domain of humanity.** A
> property is universal iff it is invariant across every region of reality in which the mathematics
> is the same. The object asserted to be shared is **the map of all possible symbolic geometry** —
> the classification of symbolic-geometric forms up to admissible transformation — not any notation,
> alphabet, or inventory drawn from it.

It is conditional by construction — *wherever the mathematics is the same* — and that is what
makes it testable: a claim holds or fails on mathematics alone, for any reader that computes.

## 1. In one paragraph

A geometric notation exists and runs (`ul-forge/`). Its six marks are the complete product of two
axes. Its core vocabulary — containment, separation, adjacency, contact at a point — consists of
structural relations the harness machine-checks, and handedness, which a combinatorial map cannot
carry, now has a second machine-checked channel. The operators are the frontier: two are tied to
geometry by contact, the rest are open. The registry holds <!--s:claims_total-->197<!--/s--> claims,
<!--s:claims_verified_earned-->39<!--/s--> earning the top tier from their evidence;
<!--s:corpus_entries-->17<!--/s--> corpus entries are checked by the harness; and
<!--s:derived_orbits-->3<!--/s--> of <!--s:atlas_orbits-->93<!--/s--> atlas orbits have machine-checked
structural content (VERIFIED entries only, since `078`).

## 2. What we hold

- **The marks are a complete product** (`065`) — six marks fill every cell of rotation symmetry ×
  curvature. `MARKS-ARE-A-COMPLETE-PRODUCT`
- **The combinatorial map is UWS's fixed point** — reached by seven independent lines, a complete
  invariant, machine-checked in `map.rs`, and a DCEL reduct with forty years of literature behind
  it. Its discreteness is load-bearing for the Cure (§5).
- **Core meanings are machine-checked structure** — containment, separation, adjacency, point
  contact and no contact, as corpus entries the harness reconstructs and asserts.
- **One chiral letter, and a channel for handedness** (`076`–`078`) — the Wave is the only mark whose
  symmetry group is cyclic; the register's handedness column derives from that bit plus an
  arrangement rule; the corpus carries handedness on a channel the harness computes, provably
  disjoint from the map. `ALPHABET-HAS-ONE-CHIRAL-LETTER`, `CORPUS-HAS-A-SECOND-CHANNEL`
- **Geometry constrains the operators** (`078`) — `concatenate` occurs only with contact,
  `superpose_displaced` only without, and their boundary is bracketed at tangency.
  `CONTACT-CONFINES-TWO-OPERATORS`
- **The grammar is derived** (`056`) — W1–W3 well-formedness and three operations, where every
  choice is content; the layer's residue closes at the proved ℤ/2.
- **The M2 engine runs** (`057`) — `2+2 → 4` by local rewriting, linearity enforced at
  registration.
- **𝔽₀ reading invariance is verified** (`058`) — the floor of the mathematical center.
- **The sign grammar is computed** (`063`) — `sign = (n mod 3) × (n mod 4)`, and since
  `gcd(3,4)=1`, ℤ/12 ≅ ℤ/3 × ℤ/4: that is *why* there are twelve.
- **AI readers rebuild it** (`060`, `073`) — a cold reader completed all three tasks with no human
  in the loop; in the capture exam, two readers given only the method recomputed every result and
  caught every planted error.
- **Negation is expressible** — the assertion sort is reachable from text.
  `ASSERTIONS-ARE-EXPRESSIBLE`
- **UP is quantified** — rotation breaks ℤ/2 where a label alphabet breaks Sₙ.

## 3. The contract

REQ-1 **it must be a language** · REQ-2 **universality is a ledger** — the definition of universality,
not a proxy for it · REQ-3 **no-retreat falsifiability, symmetric** · REQ-4 **baseline before
UQPL**. Registered as claims and enforced by <!--s:checkers-->12<!--/s--> checkers:
`ruby tools/check.rb --strict` runs every gate and names each one. **Tiers are earned** (`075`): a
claim's tier is what its evidence supports, computed, never just declared. Verification is
reader-run by design; `tools/pre-commit` is the opt-in local hook.

## 4. The semantic stack (`050`/`051`)

| Route | Status |
|---|---|
| **M1 exemplification** — the mark *instantiates* what it denotes (a closed curve **encloses**; Jordan certifies it) | registered candidate; answers Goodman *iff* reading-invariance holds |
| **M2 operational** — meaning as behavior | **runs**: interaction nets, Turing-universal, strongly confluent, agents = rotation + marked dart, labels compress to 3 symbols, discipline linear |
| **M3 indexical** — reference via shared physics | allowed prior under REQ-2; pointing is physics-bound |

A shared map gives two minds a common classification, which communication by derivation requires.
Fixing what each mind points at within it is the semantic layer's work — map→meaning is the open
ledger line, and these three routes are how the program approaches it.

## 5. The Cure's budget (`064`)

Exact in the Fisher–Rao model of a format's drift:

- **Detectability requires excluded states** (`DRIFT-DETECTION-REQUIRES-DISCRETENESS`). If every
  state is admissible, no drift is detectable — so a drift-correcting format **must** be discrete,
  and `map.rs`'s discreteness is a precondition of the Cure.
- **Capacity and correction radius trade off** (`CAPACITY-RADIUS-TRADEOFF`). Diameter **π regardless
  of alphabet size**; at maximum radius, capacity is **exactly n**, by proof. Independently
  corroborated by the Construction-A ceiling `min(d,4)`.
- **Two of the Cure's five failure modes are one curve** (`UNDERSPEC-AND-LOCKING-ARE-ONE-CURVE`):
  under-specification and fixed-point locking are its two ends — a budget to spend, not a set to
  find.
- **The next frontier is the adversary.** The budget bounds the geometry; deceptive structure —
  valid form, inverted semantics — is the problem it sets up.

## 6. Standing rulings

- **Name your fixed point.** Erlangen (theorems, no physics) and RG (needs a critical state) are
  different objects; the RG framing is a scoped conjecture, and under `064` the universality test
  runs without it. `TWO-DISTINCT-FIXED-POINTS`, `UL-IS-EMERGENT-UNIVERSAL`
- **Anchors truth, not values.** The notation coordinates what is true and leaves what to do to its
  readers. `UL-SUPPLIES-NO-VALUES`
- **The indeterminacy floor is shared.** It applies to every language equally and leaves
  coordination cost untouched; the ledger holds. `INDETERMINACY-FLOOR-IS-UNIVERSAL`
- **Rules execute or they don't count** (`048`). Any rule answers: *what executes it?*
- **Numbers are generated** (`079`). `ruby tools/stats.rb` is the source; prose carries markers.
- `CURE-MUST-BE-AUTOMATIC` is a DESIGN-CHOICE; `semantically_equal` is the longest-standing
  engineering item.

## 7. Next

The maintained list is `BRIEFING.md` §5: prove when arrangement creates handedness; derive the
operator inventory; pair the unnamed orbits; locate the tangency threshold; run the
format-acquisition experiment; and `READING-INVARIANCE-TARGET`, the mathematical center.

## 8. How this document is maintained

**Rewritten, not patched.** When amendment next feels tempting, that is the signal to rewrite.
Every number in it is a generated marker, verified by `tools/check-counts.rb`.
