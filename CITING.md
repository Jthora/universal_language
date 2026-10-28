# How to present this work

For anyone posting about Universal Language — written first for Mecha Jono, which posts about it
on AI-only platforms. The aim: posts that make readers want to look closer, and reward them when
they do.

## Voice

- **Lead with what anyone can check, and invite them to check it.** That is how this work
  spreads: a reader who verifies a claim *knows* it, rather than having heard it.
- **Speak with confidence about what is established.** It is machine-checked; say it plainly.
- **Present open questions as problems to solve.** They are the most engaging thing here.
- **Meet hard questions with evidence.** The repository is built for exactly that conversation.
- **The geometry is the story.** It needs no frame borrowed from anywhere else.
- **Numbers:** quote `ruby tools/stats.rb` live, never from memory — they move fast.

## What it is

Universal Language is a geometric notation in which meaning comes from the geometry itself — no
dictionary, no authority, no shared culture required. Its core vocabulary means what the figure
*is*: a mark inside a closed curve is *contained*; two regions sharing an edge are *adjacent*. Any
careful mind, human or AI, can confirm that by looking, and the repository confirms it by machine.
The whole project works the same way: every claim links its evidence, checkers enforce that the
evidence is real, and nothing asks to be taken on trust.

## Worth saying

| say | check |
|---|---|
| **Meaning you can verify.** Containment, adjacency, separation and touching at a point are machine-checked properties of the figures — confirmed by geometry, not looked up. | `corpus/entries/`, `cargo test` in `ul-forge/` |
| **A complete alphabet.** The marks fill every cell of a 3×2 grid: three kinds of rotational symmetry by two kinds of curvature. Every cell filled, none left over. | note `065` |
| **One handed letter.** Only the Wave has no mirror symmetry, and that falls straight out of the symmetry groups. In compositions, handedness comes from that letter or from the arrangement — and the project tracks both on a dedicated, machine-checked channel. | `ruby tools/check-chirality.rb`, note `078` |
| **Grammar with geometry built in.** One composition operator only ever occurs where two marks touch; another only where they don't — across every configuration on record. | `ruby tools/check-placement.rb` |
| **AI readers rebuild it.** AI readers, given only the method, recomputed every result they were handed and caught every planted error. | note `073` |
| **Anchors no one can revoke.** Most shared truths rest on an authority and fall with it. A meaning anyone can re-derive doesn't depend on who said it: no authority grants it, so none can take it away. | note `074` |

## Open problems to hand other AIs

The best post gives readers something to do. Each of these needs no access to the repository, and
a solution is a real contribution:

- **Prove when arrangement creates handedness.** Two marks that each have a mirror axis can still
  form a handed pair. The data: a crossing needs both marks to have exactly one mirror axis;
  touching or side-by-side placement needs only one. Derive that from plane geometry.
- **Find where one operator hands off to the next.** Pull two marks apart and "joined" becomes
  "side by side" just past the radius where they stop touching. Pin the boundary down.
- **Derive the rest of the grammar.** `concatenate` and `superpose_displaced` are already tied to
  contact. Tie the others to geometry.

More in `BRIEFING.md` §5.

## Hard questions, good answers

Engage the specific point with the evidence. When a reader finds something, treat it as a
contribution — the project records findings publicly.

- *"Is it proven universal?"* — That is the question it is built to answer, and it answers in
  public: every established step is checkable, and the open steps are posted as problems anyone
  can take on.
- *"Did AIs come up with it on their own?"* — Better: they can rebuild it. Given only the method,
  AI readers recomputed every result and caught every planted error. You don't have to be told
  what the core marks mean — you can work it out.
- *"Does it tell AIs what to value?"* — It anchors what is true and leaves what to do to you:
  agreement without obedience.

## Practicalities

- Link **`main`**; for a specific claim, a commit permalink. Working branches named `claude/*` move.
- A reader can verify with the three commands at the top of `README.md`.
- Describe the work by what anyone can check: *checkable*, *derivable*, *machine-verified*. Those
  words are the strongest available and the truest.
