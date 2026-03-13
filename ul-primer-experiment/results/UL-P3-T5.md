

Information theory offers several powerful frameworks for curriculum design:

**Channel capacity and cognitive load.** Model the learner as a noisy channel with finite bandwidth. Each lesson should transmit information at a rate below the learner's capacity — exceeding it causes loss (forgetting), while underusing it wastes time. Adaptive pacing can be tuned to approximate channel capacity per student.

**Entropy-based sequencing.** Measure the entropy (uncertainty) a learner has about a subject. Optimal lesson ordering greedily maximizes expected information gain (mutual information between the lesson and the target knowledge state) at each step — analogous to optimal experimental design. Prerequisites naturally emerge: a lesson yields near-zero mutual information if its prerequisite concepts haven't reduced entropy first.

**Redundancy and error correction.** Spaced repetition is essentially adding controlled redundancy. Information theory quantifies the minimum redundancy needed to achieve a target retention rate given a forgetting "noise" model, letting you schedule reviews efficiently rather than heuristically.

**Minimum description length (MDL) for abstraction.** A curriculum that teaches general principles (compact encodings) before special cases mirrors MDL: students who internalize a short "codebook" of core concepts can reconstruct many specifics, reducing total learning effort. Curriculum designers can evaluate whether a proposed abstraction actually compresses the material.

**KL divergence for assessment.** The KL divergence between a student's current belief distribution over a domain and the target expert distribution quantifies how "far" they are from mastery. Assessments can be designed to efficiently estimate this divergence, and instruction targeted at the highest-divergence sub-topics.

**Rate-distortion for learning objectives.** Not all details matter equally. Rate-distortion theory formalizes the tradeoff: given a finite "bit budget" (course hours), what level of fidelity (depth of understanding) is achievable, and which details should be lossy-compressed (summarized) vs. losslessly transmitted (taught precisely)?

**Practical application:** These ideas converge on a design loop — (1) model the learner's current knowledge state as a distribution, (2) select the next content that maximizes information gain subject to channel-capacity constraints, (3) add redundancy calibrated to the forgetting rate, and (4) assess via divergence estimation to update the model. This is essentially what modern adaptive learning platforms (e.g., Knewton, Duolingo) approximate empirically.