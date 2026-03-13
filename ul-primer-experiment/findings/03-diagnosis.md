# Diagnosis: Why the Experiment Produced a Null Result

Five identified causes, ordered by impact.

---

## 1. The Ceiling Effect (Primary Cause)

Claude is already an extremely competent cross-domain reasoner. On tasks like "find structural parallels between plate tectonics and economics," the model doesn't need a primer — it already knows about Rayleigh-Bénard convection, self-organized criticality, Gutenberg-Richter distributions, Schumpeterian creative destruction, and can articulate formal correspondences unprompted.

The NL baseline outputs are *already at TES 4–5* on most targeted tasks. The model spontaneously produces:
- Three formal structural parallels with equations (NL-Tpat)
- A 5-level recursive decomposition with composition rules (NL-Thier)
- Clean abstraction levels from physical chemistry to abstract principle (NL-Tabs)
- A 6-stage meaning-formation process with transition drivers (NL-Tform)

This means the experiment was testing for a primer effect in a regime where the ceiling has already been reached. It's like testing whether eyeglasses improve the vision of someone who already has 20/20 sight.

**Implication:** Future experiments must either use weaker models or substantially harder tasks.

---

## 2. The Model Treats Primers as Content, Not Scaffold

When given a primer (UL-P1 through P4 or REF), Claude's primary response is to *interpret and critique the formalism* rather than to *use it as a reasoning scaffold*.

**Most visible in UL-P1:**
- UL-P1-T1: First identifies real vs. fictitious parts of the notation, notes that "vrîtha" is undefined, then proceeds to answer with its own knowledge
- UL-P1-T2: Explicitly dismisses the primer ("The notation presented borrows real mathematical structures... but assembles them with invented concepts... I won't treat it as a meaningful formalism")
- UL-P1-T5: Sets the primer aside entirely ("The mathematical formalism you've provided doesn't connect to the question, so I'll address it directly")

**Less severe in UL-P2 through P4:** The model engages more deeply with these formalisms, but engagement is *interpretive* (explaining what the notation means) rather than *operative* (using the structure to generate novel reasoning).

**Implication:** The primers would need to be presented differently — perhaps as system prompts rather than user content, or in a format that discourages meta-commentary.

---

## 3. The Primers Carry Too Much Domain Specificity

Each theory-derived primer is heavily loaded with domain-specific mathematics:
- UL-P1: Topology, harmonic analysis, Hodge star
- UL-P2: Category theory, initial algebras, colimits
- UL-P3: Information geometry, Fisher metric, KL divergence
- UL-P4: Statistical mechanics, partition functions, Legendre transforms

When a domain-matched control contains the *same mathematics without UL structure*, the outputs are nearly identical. This suggests that whatever effect the primers have comes from the mathematical content, not from the "UL scaffold" layered on top.

The original test-content.txt avoids this problem by being *sui generis* — its formalism doesn't map cleanly to any standard domain, which may force the model into a genuinely different reasoning mode. The theory-derived primers, constructed from recognizable mathematical frameworks, are too legible — the model simply pattern-matches them to known domains and proceeds normally.

**Implication:** Truly theory-derived primers should use UL's own notation system rather than borrowing from established mathematical fields.

---

## 4. The Task Design Provides Too Much Structure

The targeted tasks are so explicitly structured — "find 3 parallels," "decompose at 5 levels," "restate at 5 abstraction levels" — that they effectively tell the model *what to do*, making the primer redundant.

Evidence: The Tabs task produces nearly identical outputs across ALL conditions (NL, REF, all primers, all controls) with TES=4 everywhere. The task provides so much instruction that the primer has no work to do.

**Implication:** Less structured tasks (e.g., open-ended "Discuss the relationship between X and Y") would allow primers to shape *how* the model reasons, not just *what* it says.

---

## 5. N=1 Per Cell Is Insufficient

With one trial per condition-task cell, there is no way to distinguish signal from noise. A single run captures one sample from the model's output distribution. The apparent differences between conditions could easily fall within the variance of repeated runs of the same condition.

Even the most promising finding (UL-P2's recursive-categorical framing) could be a lucky draw rather than a reliable effect.

**Implication:** At minimum N=5, ideally N=10, per condition-task cell. Use temperature > 0 to sample output variability. Then apply the pre-committed statistical analysis (mixed-effects ANOVA, Dunnett's test, Cohen's d).
