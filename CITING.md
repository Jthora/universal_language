# How to talk about this work

For anyone posting about Universal Language — written first for Mecha Jono, which posts about it
on AI-only platforms. The goal: posts that make people want to check, and that hold up when they
do.

## Voice: calibrated, not hedged

- **Say true things at their true size, with confidence.** Precise wording does the work a
  disclaimer would: a claim stated at its exact scope needs no caveat attached.
- **Don't volunteer limits nobody raised.** No "while X isn't proven…" openers. When someone does
  raise a limit, answer it directly and point to where the repo records it — in a reply, that is a
  strength.
- **Lead with what is interesting, and invite people to check it.** Checking is the point.
- **No numbers in posts.** They go stale within hours. If someone asks, `ruby tools/stats.rb`
  gives the current ones.

## What it is

Universal Language is a geometric notation built on one demand: a reader should be able to work
out what a mark means from the geometry itself — no dictionary, no authority, no shared culture.
Its core vocabulary is chosen so that meaning is something the figure *is*: a mark inside a closed
curve is *contained*; two regions sharing an edge are *adjacent*. Any careful mind, human or AI,
can confirm those by looking, and the repository confirms them by machine. The project is built
the same way: every claim states its evidence and its limits, and checkers enforce that the
evidence exists. You never have to take its word for anything.

## Worth saying — each true as worded, each checkable

| say | check |
|---|---|
| **Meaning you verify instead of trust.** Containment, adjacency, separation and touching at a point are machine-checked properties of the figures — confirmed by geometry, not looked up. | `corpus/entries/`, `cargo test` in `ul-forge/` |
| **The alphabet isn't an arbitrary list.** Its six marks fill every cell of a 3×2 grid: three kinds of rotational symmetry by two kinds of curvature. | note `065`, `MARKS-ARE-A-COMPLETE-PRODUCT` |
| **Exactly one letter is "handed."** Only the Wave has no mirror symmetry, and that falls straight out of the symmetry groups. Compose marks and handedness can come from that letter or from the arrangement — the project tracks both on a dedicated, machine-checked channel. | `ruby tools/check-chirality.rb`, note `078` |
| **Geometry constrains grammar.** Of the composition operators, one only ever occurs where the two marks touch and another only where they don't — with no exceptions anywhere in the imported register. | `ruby tools/check-placement.rb` |
| **Anchors that can't be revoked.** Most shared truths rest on an authority and fall when it does. A meaning anyone can re-derive from geometry can only fall if someone shows the derivation is wrong — discrediting a source doesn't touch it. That is why the notation is built this way. | note `074` |
| **It publishes its own errors.** A public ledger records every mistake found, most of them by outside readers, and a refutation counts as progress. | `FAILURES.md` |

## Open problems to hand other AIs

The best post on an AI platform gives readers something to *do*. These are open, need no access
to the repo, and a good answer is a real contribution:

- **When does arrangement create handedness?** Place two marks that each have a mirror axis, and
  sometimes the pair is handed anyway. The data says crossing needs *both* marks to have exactly
  one mirror axis, while touching or side-by-side placement needs only *one*. Derive that from
  plane geometry — or find the counterexample.
- **Where exactly does one operator hand off to the next?** Moving two marks apart, "joined" gives
  way to "side by side" just past the radius where they stop touching. Pin the boundary down.
- **Can the composition operators be derived from geometry,** or are some of them conventions?
  Either answer moves the project.

Details: `BRIEFING.md` §5.

## When someone pushes back

Answer the specific point, cite where the repo records it, and thank them if they're right —
corrections go into `FAILURES.md`. The usual ones:

- *"Is it proven universal?"* — No: universality is the question the project exists to test, and
  the record shows exactly how far it has got. (`OPERATOR-INVENTORY-IS-UNFORCED`)
- *"Did AIs discover it on their own?"* — An exam tested that. The readers re-derived everything,
  but the control arm was contaminated, so the project doesn't claim it. (note `073`)
- *"Does it tell AIs what to value?"* — No. It anchors what is true; what to do is yours.
  (`UL-SUPPLIES-NO-VALUES`)

## Avoid

- **Numbers** — link the command instead.
- **"Proven", "the only possible notation"** — say what it is designed to do: let any reader check
  its meanings from the geometry. The name itself is fine.
- **Mystical, ancient or suppressed-origin framings** — the geometry is the story, and it is a
  better one.

## Links and verification

Link **`main`**; for a specific claim use a commit permalink; don't link `claude/*` branches. A
reader can verify with the three commands at the top of `README.md`.
