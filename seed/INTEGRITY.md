# INTEGRITY — verifying a copy of this artifact

**The threat model** (`research/notes/053`): in an adversarial network, copies of a load-bearing
artifact get tampered with. A reader must be able to distinguish the real artifact from a poisoned
one **without contacting the authors** (the Voyager constraint).

## What works today

- **Git is content-addressed.** Every commit hash covers the full tree; any two copies with equal
  head hashes have identical content. Cross-check a copy's history against independent mirrors of
  the repository (GitHub origin, archives) — divergent history is the tamper signal.
- **The DOI record** (Zenodo, on `README.md`) pins snapshots independently of any git host.
- **Self-consistency is checkable offline:** `ruby tools/check.rb --strict` and the test suite
  validate the artifact's internal integrity — a tampered claim registry, broken cross-references,
  or altered tier contracts fail mechanically. This does not prove provenance; it proves coherence.

## Policy — recorded now, honestly marked not started

- **Signed release tags**: each release tagged and cryptographically signed; the public key
  published in at least three independent locations (repo, DOI record, key server).
- **Hash manifest per release**: a `SHA256SUMS` over the tree, inside the signed tag.
- **Reproducible builds** for `ul-forge` artifacts (WASM, binaries), so a binary can be traced to
  the source it claims.

Status for all three: **not-started** (see `seed/INDEX.yaml`). Until they exist, the git-hash +
mirror cross-check above is the verification path, and this document says so rather than implying
otherwise — per `TIERS-TRAVEL-WITH-CONTENT`, integrity claims are tiered like everything else.
