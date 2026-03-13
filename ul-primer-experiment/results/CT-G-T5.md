

Information theory offers several powerful frameworks for optimizing curriculum design:

**Entropy and knowledge gaps.** A learner's uncertainty (entropy) about a subject can be modeled as $H(X) = -\sum p(x)\log p(x)$. An optimal curriculum maximizes the expected reduction in entropy per unit of instruction time — prioritizing topics that resolve the most uncertainty earliest, creating efficient "decoding" paths through the material.

**Sequencing via mutual information.** Concepts can be ordered by maximizing the mutual information $I(X_i; X_{i+1})$ between successive topics. If concept B shares high mutual information with concept A, teaching A first gives the learner the most "free" bits toward understanding B — formalizing the intuition behind prerequisite chains.

**Channel capacity and cognitive load.** The learner can be modeled as a noisy channel with finite capacity $C$. Presenting information at a rate exceeding $C$ causes loss (analogous to exceeding Shannon's channel capacity). This suggests chunking material into units whose information rate stays below the learner's capacity, adapting to individual differences.

**Redundancy and spaced repetition.** Just as error-correcting codes add structured redundancy to ensure reliable transmission, curricula benefit from deliberate repetition. The optimal redundancy rate can be tuned: too little and retention fails; too much and time is wasted. Spaced repetition schedules can be viewed as adaptive coding strategies that match review frequency to each concept's "error rate" (forgetting probability).

**Minimum description length (MDL) for abstraction.** The best conceptual framework is the one that compresses the subject most — the shortest description that still predicts all the examples. Curricula can be evaluated by whether they guide learners toward representations that minimize description length, favoring transferable abstractions over rote enumeration.

**Assessment as source coding.** Test design can be optimized by selecting questions that maximize information gain about a student's knowledge state — each question should maximally reduce uncertainty about what the student knows, analogous to optimal source coding or 20-questions strategies (binary search over knowledge states).

In practice, these ideas converge on adaptive learning systems that continuously estimate a learner's knowledge distribution, select the next content to maximize information gain, and calibrate pacing to the learner's channel capacity.