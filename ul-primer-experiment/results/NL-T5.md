

Information theory offers several powerful frameworks for curriculum design:

**Sequencing via entropy reduction.** Lessons can be ordered so each one maximally reduces uncertainty about the target domain. A concept with high conditional entropy (given what's already been taught) is premature; one with low conditional entropy is redundant. The optimal next topic is the one that yields the greatest mutual information with the remaining unknowns — analogous to optimal experimental design.

**Chunking and channel capacity.** Miller's "7±2" aligns with Shannon's noisy-channel theorem: learners have a bounded information-processing rate. Curriculum units should be sized so the information per session stays within the learner's effective channel capacity, with redundancy (examples, repetition) calibrated to the expected "noise" (forgetting, distraction).

**Prerequisite graphs as Markov chains.** Model a student's knowledge state as a probability distribution over concept mastery. Each lesson is a transition matrix. You can then optimize the sequence of transitions to reach a target distribution (mastery of all concepts) in the fewest steps or with the least total effort, using techniques from Markov decision processes informed by information-theoretic rewards.

**Spaced repetition as rate-distortion.** Forgetting is lossy compression. Rate-distortion theory tells you the minimum review bandwidth needed to keep distortion (knowledge loss) below a threshold. This formalizes spaced-repetition scheduling: allocate review time to concepts proportional to their decay rate and importance.

**Redundancy and transfer learning.** Concepts share structure (mutual information between topics). A curriculum that teaches shared abstractions first — maximizing mutual information with many downstream topics — reduces the total bits the learner must acquire, the same principle behind efficient source coding.

**Assessment as channel decoding.** Tests are noisy observations of latent knowledge. Item response theory already formalizes this, but information theory adds tools like adaptive testing that selects the next question to maximize information gain about the student's ability, minimizing test length.

**Practical approach:** Build a concept graph, estimate pairwise mutual information from student performance data, then solve for the teaching order that greedily or globally maximizes cumulative information gain per unit of instructional time.