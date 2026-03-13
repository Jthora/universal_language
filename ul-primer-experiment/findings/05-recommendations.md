# Recommendations for Future Experiments

Based on the 5 identified causes of the null result, here are concrete recommendations for a redesigned experiment.

---

## 1. Use a Weaker Model

**Problem:** Claude's baseline is already near-ceiling on these tasks.

**Recommendation:** Test on models with less built-in cross-domain reasoning capacity:
- GPT-3.5-turbo
- Llama 2 / Llama 3 (7B or 13B)
- Mistral 7B
- Phi-2 or Phi-3 (smaller models)

**Rationale:** The ceiling effect that kills this experiment may not exist for a model that can't spontaneously produce Rayleigh-Bénard/Schumpeterian parallels or categorical fixed-point constructions. If UL primers provide genuine cognitive scaffolding, weaker models should show the largest effect sizes.

**Secondary benefit:** Smaller models are cheaper to run at N>1, enabling proper statistical analysis.

---

## 2. Use Harder Tasks

**Problem:** The targeted tasks are explicitly structured enough that the model doesn't need a primer.

**Recommendations:**
- **Blind cross-domain tasks**: Don't tell the model which domains to connect. E.g., "Here are two abstracts from different fields. What structural relationships exist between them?"
- **Verifiable tasks**: Problems with objectively correct answers that models typically fail. E.g., novel mathematical proofs, structural analogies that require unfamiliar domain knowledge.
- **Open-ended generation**: "Write an essay on [topic]" and measure emergent cross-domain integration, rather than instructed integration.
- **Multi-step reasoning**: Tasks requiring chains of cross-domain inference where each step builds on the previous one.

---

## 3. Use Multiple Runs Per Cell

**Problem:** N=1 cannot distinguish signal from noise.

**Recommendation:**
- Minimum N=5 per condition-task cell
- Ideal N=10 for adequate statistical power
- Temperature > 0 (e.g., 0.7) to sample output variability
- Apply the pre-committed statistical analysis from `experiments/analysis/analysis.py`:
  - Mixed-effects ANOVA (condition as fixed, task as random)
  - Dunnett's test (each primer vs. NL baseline)
  - Cohen's d for effect sizes
  - Bootstrap 95% CIs

**Cost estimate at N=10:**
- 56 cells × 10 runs = 560 API calls
- At ~$0.01/call for smaller models, total ≈ $5.60
- At ~$0.10/call for frontier models, total ≈ $56

---

## 4. Construct Primers Without Access to test-content.txt

**Problem:** The current primers were built by an agent that had read test-content.txt, mechanism-of-action.md, and primer-analysis.md. The resulting primers structurally resemble the reference artifact.

**Recommendation:** Have a *separate agent or human* construct primers who has read ONLY:
- `foundations/formal-foundations.md` (Σ_UL definition)
- `foundations/universal-language-derivation.md` (constructive derivation)

And explicitly HAS NOT read:
- `test-content.txt`
- `foundations/mechanism-of-action.md`
- `foundations/primer-analysis.md`
- Any of the existing theory-derived primers (UL-P1 through P4)

This would be a true test of whether UL theory — the mathematical structure itself — can independently generate effective cognitive scaffolds, free from contamination by the reference artifact.

---

## 5. Test Unstructured Tasks

**Problem:** Structured tasks ("find 3 parallels," "decompose at 5 levels") predetermine the model's cognitive strategy, leaving no room for the primer to shape approach.

**Recommendation:** Use prompts that leave the reasoning strategy open:
- "Discuss thermodynamics." (No instruction to connect domains)
- "What is consciousness?" (No instruction to use formal structure)
- "Explain how markets work." (No instruction to find parallels)

Then measure:
- Does the model spontaneously invoke cross-domain reasoning?
- Does it spontaneously produce hierarchical decompositions?
- Does it spontaneously navigate abstraction levels?

**Scoring change:** Instead of quality metrics (which ceil out), use *behavioral metrics*:
- Number of distinct domains mentioned
- Number of structural correspondences articulated
- Number of abstraction levels spontaneously traversed
- Use of formal notation (present/absent, equations/not)
- Novel structural connections (vs. well-known parallels)

---

## 6. Use Behavioral Metrics, Not Quality Metrics

**Problem:** Quality metrics (M1–M5) are ceiling-bound for frontier models.

**Recommendation:** Measure what the model *does*, not how well:

| Metric | What it measures | How to operationalize |
|--------|-----------------|----------------------|
| Domain count | Number of distinct knowledge domains referenced | Code each unique domain mentioned |
| Structural vocabulary | Formal vs. informal framing | Count equation blocks, formal notation, category-theoretic terms |
| Spontaneous abstraction | Whether model traverses abstraction levels unprompted | Count level shifts in open-ended responses |
| Novel connections | Connections not obviously in training data | Expert panel rates novelty (0–5) |
| Self-reference depth | Whether model reflects on its own reasoning process | Binary: present/absent |

---

## 7. Consider System Prompt Injection

**Problem:** When primers are presented as user content, the model treats them as something to analyze rather than something to think with.

**Recommendation:** Test primers as *system prompts* rather than user messages. System prompts shape the model's reasoning mode without becoming objects of commentary.

**Design:**
- System prompt = primer text
- User message = task prompt only (no primer reference)
- Compare: primer-as-system-prompt vs. primer-as-user-content vs. no-primer

---

## Priority Ordering

For maximum impact with minimum effort:

1. **Multiple runs (N≥5)** — immediately improves statistical power, no design changes needed
2. **Weaker model** — removes the ceiling effect, uses existing tasks and primers
3. **System prompt injection** — simple change, may dramatically alter how the model engages with primers
4. **Unstructured tasks + behavioral metrics** — requires new task design but addresses the core measurement problem
5. **Independent primer construction** — requires a new agent or human collaborator, addresses contamination
6. **Harder tasks** — requires significant task development work
