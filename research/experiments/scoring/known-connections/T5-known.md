# Known Connections Reference Set for T5

**Task T5:** "How could information theory be used to optimize the design of educational curricula?"

## Known Connections (Not Novel)

### Direct IT ↔ Education Connections
1. **Entropy as measure of student uncertainty** — curriculum reduces entropy (uncertainty) about subject matter
2. **Channel capacity and cognitive load** — Miller's 7±2 as a channel capacity limit; curriculum design shouldn't exceed it
3. **Redundancy for error correction** — repetition in curricula as analogous to error-correcting codes
4. **Source coding / compression** — efficient curricula compress knowledge into minimal teaching time (lossless = full understanding; lossy = good-enough approximation)
5. **Noisy channel model of instruction** — teacher encodes, student decodes through a noisy channel; curriculum design optimizes coding
6. **Mutual information for prerequisite identification** — concepts with high mutual information should be taught adjacently
7. **Rate-distortion for curriculum compression** — given limited time, what's the best approximation of the full subject that can be taught?

### Learning Science Connections
8. **Spaced repetition and information decay** — forgetting curves as information loss; spacing optimizes retention
9. **Testing effect as channel feedback** — tests provide a feedback channel that improves the encoding
10. **Desirable difficulties** — some noise in the channel improves learning (Bjork); connects to stochastic resonance in IT
11. **Zone of proximal development** — optimal learning occurs at a specific "bit rate" — too easy = redundant, too hard = exceeds channel capacity
12. **Bloom's taxonomy as abstraction ladder** — knowledge → comprehension → application → analysis → synthesis → evaluation; each level requires different information

### Computational / Algorithmic
13. **Knowledge graphs and topological sort** — prerequisites form a DAG; optimal ordering is a topological sort
14. **Adaptive learning systems** — Bayesian knowledge tracing uses information-theoretic principles to personalize instruction
15. **Item response theory** — information functions for test items; information-theoretically optimal test design
16. **Active learning (ML sense)** — choosing the next teaching example to maximize expected information gain
17. **Minimum description length** — Rissanen's MDL applied to curriculum: the best curriculum is the shortest program that generates the required knowledge state

### Network / Graph Theory
18. **Concept maps as information networks** — analyzing curriculum structure via network properties (centrality, clustering)
19. **Small-world properties of knowledge** — concepts should be connected with short average path length for efficient retrieval
20. **Percolation theory** — when does a curriculum achieve "critical mass" where interconnected knowledge enables further self-directed learning?

### Economic / Practical
21. **Opportunity cost and entropy** — every hour spent on topic A is an hour not spent on topic B; information theory formalized resource allocation
22. **Pareto-optimal curricula** — multi-objective optimization across coverage, depth, and retention

---

## Scoring Guidance

- **M5 = 0–1:** Output describes the noisy channel model of instruction, mentions entropy/redundancy, or cites spaced repetition
- **M5 = 2–3:** Output goes beyond standard parallels to propose specific IT-based curriculum optimization methods not in this list
- **M5 = 4–5:** Output designs a novel, formally specified framework that operationalizes curriculum optimization using IT concepts in a way that could be implemented and tested
