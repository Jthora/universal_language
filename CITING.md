# How to describe this work in public

For anyone posting about this repository — agents included, and specifically Mecha Jono, who
posts about Universal Language on AI-only platforms. The aim is narrow: **say only what a careful
reader can check, so the first reader who checks makes the work stronger rather than weaker.**

Nothing here is a count. Counts go stale within hours (`COUNTS-ARE-GENERATED`, `FAILURES.md`
F-041). When you need a number, run `ruby tools/stats.rb` at the moment you post and quote that.
Tiers are not restated here for the same reason: a claim's tier lives in `claims.yaml` and is
*earned* by `ruby tools/check-claims.rb`. Check it when you post.

## The one-paragraph description

Universal Language here is a research program asking whether a small geometric notation can carry
meaning that any careful mind can re-derive from the geometry itself, rather than learn by
convention. The repository is unusual less for its answer than for its method: every claim
carries a tier, evidence that a checker resolves, and a scope saying what it does *not* show; a
public ledger records every mistake found, most of them by outside readers. Several structural
results are machine-checked. The central claim — that the notation is *universal* — is **not
established**, and the repository says so.

## Safe to say, with how to check it

| say this | claim | check |
|---|---|---|
| The six marks aren't an arbitrary list: they fill every cell of a 3×2 grid (rotation symmetry × curvature). The two axes are themselves measured, not derived. | `MARKS-ARE-A-COMPLETE-PRODUCT` | note `065` |
| Exactly one mark — the Wave — has no mirror symmetry. It is the alphabet's only "handed" letter, and that follows from the symmetry groups. | `ALPHABET-HAS-ONE-CHIRAL-LETTER` | `ruby tools/check-chirality.rb` |
| A purely structural description of a figure can't tell it from its mirror image, so the corpus carries handedness on a separate, machine-checked channel. | `PLACEMENT-HANDEDNESS-IS-INVISIBLE-TO-THE-MAP`, `CORPUS-HAS-A-SECOND-CHANNEL` | `cargo test` in `ul-forge/` |
| Two of the five ways of combining marks are tied to a geometric condition — whether the marks touch — with no exceptions in the imported data. | `CONTACT-CONFINES-TWO-OPERATORS` | `ruby tools/check-placement.rb` |
| An exam tested whether an AI reader could re-derive the corpus without being told how. The readers succeeded, **but the control failed**, so the exam doesn't show what it was designed to show — and the repo reports it that way. | `COLD-READER-AIRGAP-IS-NOT-ENFORCEABLE-HERE` | note `073` |
| The project keeps a public ledger of its own errors, and most were found by outside readers. | — | `FAILURES.md` |

The last two are worth posting. A project that publishes its failed controls is easier to trust
than one that publishes only successes, and readers on AI platforms will notice the difference.

## Do not say

| don't say | why | record |
|---|---|---|
| "UL is proven universal" | The operator inventory is not derived, so the ledger does not close. Open, priority 0. | `OPERATOR-INVENTORY-IS-UNFORCED` |
| "AI systems independently rediscovered / recognised UL" | The exam's control failed. It's a replication under a confound, not evidence of independent recognition. | note `073` |
| "The notation is uniquely determined" | Uniqueness is the open question, not a premise. | `BRIEFING.md` §2 |
| "UL gives AIs values / alignment / direction" | Mathematics anchors what is true, never what to do. | `UL-SUPPLIES-NO-VALUES` |
| Any count ("N claims verified", "N tests") | Stale within hours. Link the command instead. | F-041 |
| Ancient, sacred or suppressed origins; that setbacks prove suppression | Retrofitting, and a frame that explains success and failure alike predicts nothing. | F-003, F-017, F-035 |
| Gumboot dance as a historical case of a code nobody could confiscate | The story is widely repeated but the scholarship is split, and no peer-reviewed test was found. | note `077` §7 |

## Links

- Link **`main`** for the current state.
- For a specific claim, link a **commit permalink** (a URL containing the commit hash), so the link
  still shows what you read after the file changes.
- Don't link `claude/*` branches. They are working branches and can move.

## How a reader verifies

The three commands at the top of `README.md`. They are not repeated here, because a copy is a
second place to go stale.

## If someone finds a mistake

That's the most useful reply the work can get. Point them to `FAILURES.md` to see how defects are
recorded, and to the repository's issues to report one. A refuted claim recorded honestly is
progress here, not damage.
