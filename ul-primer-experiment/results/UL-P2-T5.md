

Information theory offers several powerful lenses for curriculum design:

**Channel capacity & cognitive load.** Treat each lesson as a noisy channel with finite bandwidth (working memory). Shannon's channel capacity theorem suggests matching information rate to the learner's current throughput — sequencing material so bit-rate stays below capacity, reducing overload and maximizing retained signal.

**Entropy-based sequencing.** Measure the entropy (uncertainty) a student has about a subject at each stage. Optimal sequencing minimizes expected total entropy over time — introduce high-mutual-information concepts first (ones that reduce uncertainty about the most other topics), creating an efficient "compression tree" of knowledge.

**Redundancy & error correction.** Curricula need redundancy (revisiting concepts, spiral design) analogous to error-correcting codes. Information theory quantifies the minimum redundancy needed for a target retention rate given a measured "forgetting noise" channel, avoiding both under-repetition (loss) and over-repetition (wasted time).

**KL divergence for assessment.** Measure the Kullback-Leibler divergence between a student's current knowledge distribution and the target distribution. This gives a principled metric for how far a learner is from mastery and which topics contribute most to the gap — enabling adaptive, personalized pathways.

**Mutual information for prerequisite mapping.** Compute mutual information $I(X;Y)$ between performance on topic $X$ and topic $Y$ across a population. High MI reveals true prerequisite relationships, letting you build an empirically grounded dependency graph rather than relying on intuition.

**Rate-distortion for abstraction level.** Rate-distortion theory formalizes the tradeoff between simplification and fidelity. For any concept, you can characterize the minimum description complexity needed to keep "distortion" (misconceptions) below a threshold — guiding how much to simplify for a given audience.

**Minimum description length (MDL) for concept design.** Concepts that compress many observations into short descriptions (low Kolmogorov complexity relative to explanatory power) are easier to learn. MDL can evaluate whether a curriculum's conceptual framework is optimally compressed or unnecessarily baroque.

In short: model the learner as a receiver, the curriculum as an encoder, and forgetting/confusion as noise — then apply the full apparatus of source coding, channel coding, and rate-distortion theory to minimize total instructional cost for a target level of mastery.