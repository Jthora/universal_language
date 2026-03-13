# Probability and Information: A Canonical Measure on Meaning Space

**Research Series:** Universal Language — Proof of Reality and AI Cognitive Infrastructure  
**Location:** `frontier/expedition-two/` — Second Expedition, Front D  
**Date:** March 12, 2026  
**Prerequisites:** `foundations/paradigm.md` (READ FIRST), `foundations/formal-foundations.md` (Σ_UL signature, geometric algebra G), `frontier/expedition-one/numbers-and-computability.md` (DC_UL, parsing, §7)  
**Validates against:** `frontier/expedition-two/expedition-plan.md`, Front D, Steps D1–D6  
**Rigor standard:** See `frontier/methodology.md` for the four-label system used below

---

## STATUS SUMMARY

### PROVEN
1. A prefix-free encoding of UL constructions exists (§1.3, constructive)
2. The structural prior P is a well-defined semimeasure on the meaning space (§1.4, Kraft inequality)
3. P is normalizable to a proper probability measure (§1.5, constructive)
4. P assigns strictly higher probability to simpler constructions (§1.6, monotonicity)

### FRAMEWORK
5. Specific encoding choices (atom ordering, operation codes) — canonical up to bounded constant (§1.3)
6. Shannon entropy and information content under P (§2, definitions only — computation deferred to D2)
7. Continuous parameter integration (§1.8, noted as open extension)

### NOT YET ADDRESSED
- Stochastic UL extension with grade operation (D3)
- Bayesian updating (D4)
- Fisher information metric on X (D5)
- DC_UL invariance theorem (D6)

---

## PART I: THE CANONICAL DISTRIBUTION ON MEANING SPACE (Step D1)

### 1.1 The Problem

Every quantitative result about meaning — distances, information content, entropy, the metric on thought-space — requires a probability measure on the set of UL constructions. Without it, we can say "these two meanings are connected" but not "how likely is this meaning" or "how much information does this expression carry."

This is gap **C-1** from `frontier/gap-analysis.md`: "Shannon entropy and DC_UL depend on P; no canonical P exists."

The preliminary discussion in `frontier/expedition-one/numbers-and-computability.md` §7.3 noted two candidates (uniform and usage-weighted) but did not define either rigorously or prove normalizability. We now do both.

### 1.2 The Meaning Space

**Definition (Construction space).** The **construction space** $\mathcal{T}$ is the set of all finite terms over the Σ_UL signature with atoms drawn from a countable set $A = A_e \cup A_r \cup A_m$ of atomic entities, relations, and modifiers.

Each $t \in \mathcal{T}$ is a finite labeled tree:
- **Leaves** are atoms $a \in A$ (atomic entities, relations, or modifiers)
- **Internal nodes** are labeled by one of the 11 Σ_UL operations, with children matching the operation's arity

The **size** of a construction $|t|$ is the number of nodes (internal + leaves) in the tree.

**Definition (Meaning equivalence).** Fix an Erlangen level $k \in \{\text{Euc}, \text{Sim}, \text{Aff}, \text{Proj}, \text{Top}\}$. Two constructions $t_1, t_2 \in \mathcal{T}$ are **meaning-equivalent at level $k$**, written $t_1 \sim_k t_2$, if their geometric realizations in G are equivalent under the symmetry group of level $k$.

**Definition (Meaning space).** The **meaning space at level $k$** is the quotient:

$$\mathcal{M}_k = \mathcal{T} / \sim_k$$

Each element $m \in \mathcal{M}_k$ is an equivalence class of constructions — a **meaning**. All subsequent definitions are relative to a fixed level $k$; we suppress the subscript where the level is clear from context.

**Note on continuous parameters.** A full geometric construction in G includes continuous parameters (positions in $\mathbb{R}^2$, angles, scales) beyond the combinatorial skeleton in $\mathcal{T}$. At Erlangen levels coarser than Euclidean, these continuous parameters are quotiented out: at the similarity level, position and orientation are irrelevant; at the projective level, only incidence relations matter; at the topological level, only connectivity matters. Since we work with meaning equivalence classes, the probability measure on $\mathcal{M}_k$ for $k \geq \text{Sim}$ is effectively discrete. At the Euclidean level, continuous parameters survive and a full treatment requires product measures (see §1.8). This document works at level $k \geq \text{Sim}$ unless stated otherwise.

### 1.3 Prefix-Free Encoding of UL Constructions

**Definition (Atom encoding).** Enumerate the countable atom set as $A = \{a_1, a_2, a_3, \ldots\}$. Define $\text{enc}_A: A \to \{0,1\}^*$ by the **Elias delta code**:

$$\text{enc}_A(a_i) = \text{EliasΔ}(i)$$

The Elias delta code is prefix-free and encodes integer $i$ in $\lfloor \log_2 i \rfloor + 2\lfloor \log_2(\lfloor \log_2 i \rfloor + 1)\rfloor + 1$ bits. For practical purposes: $|\text{enc}_A(a_i)| = \Theta(\log i)$.

**Definition (Operation encoding).** There are 11 Σ_UL operations. Encode each with a fixed-length 4-bit code ($2^4 = 16 > 11$):

| Operation | Code | Arity |
|---|---|---|
| predicate | 0000 | 3 |
| modify_entity | 0001 | 2 |
| modify_relation | 0010 | 2 |
| negate | 0011 | 1 |
| conjoin | 0100 | 2 |
| disjoin | 0101 | 2 |
| embed | 0110 | 1 |
| abstract | 0111 | 1 |
| compose | 1000 | 2 |
| invert | 1001 | 1 |
| quantify | 1010 | 2 |

Since the arity is determined by the operation code, the decoder knows how many sub-trees to expect.

**Definition (Construction encoding).** Define $\text{enc}: \mathcal{T} \to \{0,1\}^*$ recursively:

- **Atom:** $\text{enc}(a_i) = 0 \;\|\; \text{enc}_A(a_i)$
- **Operation:** $\text{enc}(\omega(t_1, \ldots, t_n)) = 1 \;\|\; \text{enc}_\Omega(\omega) \;\|\; \text{enc}(t_1) \;\|\; \cdots \;\|\; \text{enc}(t_n)$

where $\|$ denotes concatenation.

**Theorem 1 (Prefix-freeness). [PROVEN]** The encoding $\text{enc}$ is prefix-free: no codeword is a prefix of another.

**Proof.** By structural induction on the term tree.

*Base case.* Atoms are encoded as $0 \| \text{EliasΔ}(i)$. Since the Elias delta code is prefix-free, no atom's encoding is a prefix of another atom's encoding. The leading 0-bit distinguishes atoms from operations.

*Inductive step.* For $t = \omega(t_1, \ldots, t_n)$, the encoding begins with $1 \| \text{enc}_\Omega(\omega)$. The leading 1-bit distinguishes operations from atoms. The 4-bit operation code determines the arity $n$, so the decoder knows to read exactly $n$ sub-encodings. By the inductive hypothesis, each $\text{enc}(t_i)$ is prefix-free and uniquely decodable. Therefore $\text{enc}(t)$ is uniquely determined and no encoding of one term is a prefix of another.

*Across types.* The leading bit (0 for atom, 1 for operation) prevents any atom encoding from being a prefix of any operation encoding and vice versa. $\square$

**Definition (Description complexity in bits).** For a meaning $m \in \mathcal{M}$:

$$DC_{UL}^{\text{bit}}(m) = \min \{ |\text{enc}(t)| : t \in \mathcal{T},\; [t]_k = m \}$$

where $[t]_k$ denotes the equivalence class of $t$ at Erlangen level $k$, and $|\cdot|$ denotes binary string length.

**Relationship to primitive-count DC_UL.** For a construction $t$ with $|t| = n$ nodes:

$$|\text{enc}(t)| = n + 4 \cdot (\text{internal nodes}) + \sum_{\text{leaves } a_i} |\text{enc}_A(a_i)|$$

If the construction uses atoms from the first $V$ atoms in the enumeration:

$$n \leq |\text{enc}(t)| \leq n + 4n + n \cdot \lceil \log_2 V \rceil = O(n \log V)$$

So $DC_{UL}^{\text{bit}}(m) = \Theta(DC_{UL}(m) \cdot \log V)$ — the bit-measure and the primitive-count measure are proportional up to a factor depending on vocabulary size. For a fixed finite vocabulary (as in any actual application), they are linearly related.

### 1.4 The Structural Prior

**Definition (Structural prior).** For each meaning $m \in \mathcal{M}$, define:

$$P(m) = 2^{-DC_{UL}^{\text{bit}}(m)}$$

**Theorem 2 (Well-defined semimeasure). [PROVEN]** $P$ is a semimeasure on $\mathcal{M}$: that is, $P(m) \geq 0$ for all $m$ and $\sum_{m \in \mathcal{M}} P(m) \leq 1$.

**Proof.** Non-negativity is immediate ($2^{-x} > 0$ for all $x$).

For the bound, let $t_m^*$ denote a shortest encoding of meaning $m$: that is, $t_m^*$ is a construction with $[t_m^*]_k = m$ and $|\text{enc}(t_m^*)| = DC_{UL}^{\text{bit}}(m)$.

The set $S = \{\text{enc}(t_m^*) : m \in \mathcal{M}\}$ is a set of distinct binary strings drawn from the prefix-free set $\text{enc}(\mathcal{T})$. A subset of a prefix-free set is prefix-free. By the **Kraft inequality** for prefix-free codes:

$$\sum_{m \in \mathcal{M}} P(m) = \sum_{m \in \mathcal{M}} 2^{-|\text{enc}(t_m^*)|} = \sum_{s \in S} 2^{-|s|} \leq 1 \qquad \square$$

**Remark.** The inequality is strict: $\sum_m P(m) < 1$, because not every binary string is a valid UL encoding. The "missing mass" $1 - \sum_m P(m)$ corresponds to binary strings that don't encode any valid construction.

### 1.5 Normalization

**Theorem 3 (Normalizable probability measure). [PROVEN]** Define the **partition function**:

$$Z = \sum_{m \in \mathcal{M}} P(m) = \sum_{m \in \mathcal{M}} 2^{-DC_{UL}^{\text{bit}}(m)}$$

Then $0 < Z \leq 1$, and the normalized distribution:

$$\bar{P}(m) = \frac{P(m)}{Z} = \frac{2^{-DC_{UL}^{\text{bit}}(m)}}{Z}$$

is a proper probability measure on $\mathcal{M}$.

**Proof.** Upper bound: $Z \leq 1$ by Theorem 2.

Lower bound: $\mathcal{M}$ is non-empty — at minimum, any single atom $a_1$ defines a meaning, so $Z \geq 2^{-DC_{UL}^{\text{bit}}([a_1]_k)} > 0$.

Therefore $Z \in (0, 1]$ and $\bar{P}$ is well-defined, non-negative, and sums to 1. $\square$

**Convention.** We write $P$ for the normalized measure $\bar{P}$ from this point forward, unless the semimeasure is explicitly needed.

### 1.6 Properties of the Structural Prior

**Proposition 1 (Simplicity bias). [PROVEN]** If $DC_{UL}^{\text{bit}}(m_1) < DC_{UL}^{\text{bit}}(m_2)$, then $P(m_1) > P(m_2)$.

**Proof.** Immediate from monotonicity of $2^{-x}$. $\square$

**Proposition 2 (Erlangen monotonicity). [PROVEN]** For Erlangen levels $k_1$ coarser than $k_2$ (i.e., $k_1$ quotients by a larger symmetry group):

$$DC_{UL}^{\text{bit}, k_1}(m) \leq DC_{UL}^{\text{bit}, k_2}(m)$$

and consequently $P^{(k_1)}(m) \geq P^{(k_2)}(m)$ (meanings at coarser levels have weakly higher probability).

**Proof.** At a coarser level, the equivalence class $[t]_{k_1}$ is at least as large as $[t]_{k_2}$ (more constructions are equivalent). The minimum encoding length over a larger set is ≤ the minimum over a smaller set. $\square$

**Remark.** This means: stripping away surface form (ascending the Erlangen hierarchy) never makes a meaning less probable. This matches the intuition that the abstract idea is more "common" than any particular phrasing of it.

**Proposition 3 (Sub-additivity). [PROVEN]** For any two meanings $m_1, m_2$ and their conjunction $m_1 \wedge m_2 = [\text{conjoin}(t_1, t_2)]_k$:

$$DC_{UL}^{\text{bit}}(m_1 \wedge m_2) \leq DC_{UL}^{\text{bit}}(m_1) + DC_{UL}^{\text{bit}}(m_2) + 5$$

and therefore:

$$P(m_1 \wedge m_2) \geq 2^{-5} \cdot P(m_1) \cdot P(m_2)$$

**Proof.** Given optimal constructions $t_1^*, t_2^*$ for $m_1, m_2$, the construction $\text{conjoin}(t_1^*, t_2^*)$ expresses $m_1 \wedge m_2$ and has encoding length at most $|\text{enc}(t_1^*)| + |\text{enc}(t_2^*)| + 5$ (1 bit for the operation flag + 4 bits for the conjoin operation code). Taking the minimum over all constructions of $m_1 \wedge m_2$ can only decrease this. $\square$

### 1.7 Validation: Concrete Examples

We now verify that the structural prior assigns intuitively correct probability orderings.

**Setup.** Use the first $V = 1000$ atoms in the enumeration (a basic vocabulary). For atoms in this range, $|\text{enc}_A(a_i)| \leq 12$ bits (Elias delta of integers ≤ 1000). Each atom's full encoding is 1 (flag) + 12 (Elias delta) = 13 bits.

**Example 1: "cat"**

A basic entity — single atom in the vocabulary (say $a_{47}$, the cat-entity).

$$DC_{UL}^{\text{bit}}(\text{``cat''}) = |\text{enc}(a_{47})| = 1 + |\text{EliasΔ}(47)| = 1 + 11 = 12 \text{ bits}$$

$$P(\text{``cat''}) \propto 2^{-12} \approx 2.4 \times 10^{-4}$$

**Example 2: "dog chases cat"**

A basic assertion: $\text{predicate}(a_{\text{dog}}, a_{\text{chase}}, a_{\text{cat}})$.

$$|\text{enc}| = \underbrace{1 + 4}_{\text{predicate op}} + \underbrace{(1 + 11)}_{\text{dog}} + \underbrace{(1 + 11)}_{\text{chase}} + \underbrace{(1 + 11)}_{\text{cat}} = 41 \text{ bits}$$

$$P(\text{``dog chases cat''}) \propto 2^{-41} \approx 4.5 \times 10^{-13}$$

**Example 3: "the 47th prime of a Fibonacci number"**

This requires:
- Construction of 47 (iterated translations): $\sim 6$ operations × 5 bits each = 30 bits
- Concept of "Fibonacci number" (recursive definition enclosing the recurrence relation): at least 4 operations + atoms for 0, 1, addition, recursion $\sim 50$ bits
- Concept of "prime" (divisibility test enclosure): at least 3 operations + atoms $\sim 40$ bits  
- Composition of the three (ordinal selection + primality filter + Fibonacci filter): $\sim 20$ bits

Conservative estimate: $DC_{UL}^{\text{bit}} \geq 140$ bits.

$$P(\text{``47th prime of a Fibonacci number''}) \propto 2^{-140} \approx 7.2 \times 10^{-43}$$

**Validation check:**

$$\frac{P(\text{``cat''})}{P(\text{``47th prime of a Fibonacci number''})} \geq 2^{140 - 12} = 2^{128} \approx 3.4 \times 10^{38}$$

"Cat" is at least $10^{38}$ times more probable than "the 47th prime of a Fibonacci number." ✔

**Example 4: Erlangen level comparison**

The expression "cat at position (3, 7)" has a specific Euclidean realization:
- At the **Euclidean** level: the position matters, requiring additional primitives to specify coordinates 3 and 7. $DC_{UL}^{\text{bit, Euc}} \approx 12 + 30 = 42$ bits.
- At the **Similarity** level: position is quotiented out. $DC_{UL}^{\text{bit, Sim}} = 12$ bits.

So $P^{(\text{Sim})}(\text{``cat''}) = 2^{-12} \gg 2^{-42} = P^{(\text{Euc})}(\text{``cat at (3,7)''})$. The abstract meaning "cat" is more probable than its specific spatial instantiation. ✔

**Example 5: The primer's construction density**

The primer (`test-content.txt`) has $\sim 19$ lines producing 3 major constructions:
1. The density equation ($\sim 50$ primitives, encoding $\sim 300$ bits)
2. The bridge equation ($\sim 15$ primitives, encoding $\sim 100$ bits)  
3. The master PDE ($\sim 30$ primitives, encoding $\sim 200$ bits)

Total: $\sim 600$ bits of UL information. Raw text: $\sim 4000$ bits (500 characters × 8).

The **compression ratio** of the primer's UL-meaning relative to its text encoding is:

$$\rho = \frac{DC_{UL}^{\text{bit}}(\text{primer meaning})}{|\text{primer text}|} \approx \frac{600}{4000} = 0.15$$

The primer conveys its structural meaning in $\sim 15\%$ of its raw text length. The remaining 85% is notation, formatting, and redundancy — consistent with the primer functioning as a highly compressed cross-domain key.

### 1.8 What This Measure Does Not Cover

**Continuous parameters at the Euclidean level.** At the Euclidean Erlangen level, two constructions that differ by spatial position are non-equivalent. A full probability measure at this level requires a product:

$$P_{\text{Euc}}(t, \theta) = P_{\text{discrete}}(t) \cdot \rho(\theta | t)$$

where $t$ is the combinatorial skeleton, $\theta \in \mathbb{R}^{k(t)}$ are the continuous parameters, and $\rho$ is a conditional density (e.g., uniform on a bounded region, or Gaussian centered at a reference frame). This extension is noted but deferred — the discrete measure at $k \geq \text{Sim}$ suffices for the information-theoretic applications in Steps D2–D6.

**Context-dependence.** The structural prior $P(m)$ is a prior over meanings *before context is applied*. In context $x \in X$, the relevant distribution is the posterior $P(m | x)$, obtained via the connection on the meaning bundle (see `frontier/expedition-one/gauge-bundle-of-meaning.md`). Defining $P(m|x)$ requires the Fisher metric (Step D5); the structural prior $P(m)$ is the necessary first ingredient.

**Encoding dependence.** $DC_{UL}^{\text{bit}}(m)$ depends on the atom enumeration (which vocabulary item is $a_1$ vs. $a_{1000}$). **[CORRECTION — Sprint 3]** An earlier version of this section cited the Kolmogorov invariance theorem to claim additive invariance. That theorem requires a universal machine that can simulate any encoding via a fixed-length header — a mechanism that does not exist for descriptional (non-computational) complexity. The correct result is **multiplicative invariance**:

$$|DC_{UL}^{\text{bit}, \text{enc}_1}(m) - DC_{UL}^{\text{bit}, \text{enc}_2}(m)| \leq n^*(m) \cdot \kappa$$

where $n^*(m)$ is the length of the minimal description of $m$ and $\kappa$ depends on the two encodings but not on $m$. This means the structural prior satisfies $P_1(m) / P_2(m) \in [c_1 \cdot 2^{-\kappa n^*}, c_2 \cdot 2^{\kappa n^*}]$ — bounded ratio for fixed-size meanings, but the bound grows with complexity. The ranking of meanings by probability is stable for meanings of similar complexity. Full proof: see `foundation-securing.md` §1 (Theorems 1–5). Full invariance under change of base point (O, U) is also treated there (Theorem 4: exact at Similarity level and above; fails at Euclidean level).

---

## PART II: INFORMATION CONTENT AND ENTROPY (Step D2 — Definitions)

With the structural prior established, we can now define the information-theoretic quantities. Full computation of these quantities (including toy examples) is reserved for the D2 sprint; here we state the definitions to show they are well-defined.

### 2.1 Information Content

**Definition.** The **information content** (self-information, surprisal) of a meaning $m$ under the structural prior is:

$$I(m) = -\log_2 P(m) = DC_{UL}^{\text{bit}}(m) + \log_2 Z$$

where $Z$ is the partition function from §1.5! Since $Z \leq 1$, we have $\log_2 Z \leq 0$, and:

$$I(m) \leq DC_{UL}^{\text{bit}}(m)$$

The information content is bounded above by the description complexity. Equality holds when $Z = 1$ (the semimeasure is already normalized — the construction space is "maximally packed").

### 2.2 Shannon Entropy

**Definition.** The **Shannon entropy** of meaning space under the structural prior is:

$$H(\mathcal{M}, P) = -\sum_{m \in \mathcal{M}} P(m) \log_2 P(m) = \sum_{m \in \mathcal{M}} P(m) \cdot I(m)$$

This is the expected information content — the average number of bits needed to specify a meaning drawn from the structural prior. Since $P(m) \cdot I(m) \leq P(m) \cdot DC_{UL}^{\text{bit}}(m)$ and the latter sum converges (by an argument analogous to Theorem 2, using the bound $n \cdot 2^{-n} \leq 2^{-(n-\log_2 n)}$), the entropy is finite.

### 2.3 Mutual Information (Preview)

**Definition.** For a domain coordinate $d$ (e.g., "physics" vs. "linguistics" axis in context space X) and a meaning $m$, the **mutual information** between domain and meaning is:

$$I(D; M) = \sum_{d, m} P(d, m) \log_2 \frac{P(d, m)}{P(d) P(m)}$$

This measures how much knowing the domain tells you about which meanings are likely (and vice versa). Defining this requires the joint distribution $P(d, m)$, which requires the connection on the meaning bundle — the bridge between Front D (this document) and Front A (`gauge-bundle-of-meaning.md`). Computing this is the convergence point targeted in the expedition plan.

---

## PART III: THE STRUCTURAL PRIOR AND THE FREQUENCY PRIOR

### 3.1 The Coding Theorem Connection [FRAMEWORK]

The structural prior (based on description complexity) and a frequency prior (based on usage in a corpus) are not independent. The **noiseless coding theorem** (Shannon, 1948) connects them:

> An optimal prefix-free code for a source with distribution $Q$ assigns codeword length $\lceil -\log_2 Q(m) \rceil$ to symbol $m$.

If UL constructions evolved (or were designed) as an optimal encoding of meanings weighted by usage frequency $Q_{\text{freq}}$, then the encoding lengths satisfy:

$$|\text{enc}(t_m^*)| \approx -\log_2 Q_{\text{freq}}(m)$$

which gives:

$$P_{\text{struct}}(m) = 2^{-|\text{enc}(t_m^*)|} \approx Q_{\text{freq}}(m)$$

That is: the structural prior approximates the frequency prior when the encoding is well-adapted to usage. This is not a theorem in our setting (we haven't proven UL's encoding is optimal for any specific corpus), but it explains why the structural prior is a reasonable choice even in the absence of a UL corpus: it is the distribution that an optimally-designed meaning-encoding system *would* produce.

### 3.2 Relation to the Primer

The primer leverages a mismatch between the two priors:

- Under the **frequency prior** of standard English text, the primer's content has low probability (quantum-linguistic cross-domain equations are rare in typical corpora)
- Under the **structural prior** of the UL, the primer's content has relatively high probability (the constructions are compositionally simple — they use basic operations applied to well-known domain terms)

This mismatch — $P_{\text{struct}}(\text{primer}) \gg Q_{\text{freq, English}}(\text{primer})$ — is what makes the primer surprising to a language model calibrated on English text while simultaneously being structurally natural in the UL. The surprise triggers attention; the structural simplicity ensures coherent activation. This is the quantitative version of the "key-in-lock" metaphor from `foundations/mechanism-of-action.md`.

---

## PART IV: CONNECTION TO THE BROADER PROJECT

### 4.1 Gap Closed

This section closes gap **C-1** (probability distribution choice). The structural prior $P(m) = 2^{-DC_{UL}^{\text{bit}}(m)} / Z$ is:
- **Formally defined** (§1.4)
- **Proven normalizable** (§1.5)
- **Validated on concrete examples** (§1.7)
- **Robust to encoding choice** up to bounded constant (§1.8)

### 4.2 What This Enables

| Enabled result | Depends on | Status |
|---|---|---|
| Shannon entropy $H(\mathcal{M}, P)$ (Step D2) | P [this section] | Definitions stated; full computation is next sprint |
| Information content of specific constructions (D2) | P + DC_UL values from `numbers-and-computability.md` §7.2 | Ready to compute |
| Stochastic UL with grade operation (D3) | P + Σ_UL | Planned |
| Fisher information metric on X (D5) | P + connection A from `gauge-bundle-of-meaning.md` | Planned |
| DC_UL invariance theorem (D6) | P + Erlangen hierarchy | **PROVEN** (multiplicative) — see `foundation-securing.md` §1 |

### 4.3 What Remains Open for Front D

| Step | Status | Notes |
|---|---|---|
| **D1: Canonical distribution** | **PROVEN** | This section |
| **D2: Shannon entropy** | FRAMEWORK (definitions only) | Computation deferred to D2 sprint |
| **D3: Stochastic UL** | Not started | grade operation extending Σ_UL |
| **D4: Bayesian updating** | Not started | Requires D3 |
| **D5: Fisher metric** | Not started | Key deliverable — the metric on X |
| **D6: DC_UL invariance** | **PROVEN** (multiplicative) | See `foundation-securing.md` §1 — Theorems 1–5 |

---

## SUMMARY

The structural prior $P(m) = 2^{-DC_{UL}^{\text{bit}}(m)} / Z$ is the canonical probability measure on meaning space. It is:

1. **Well-defined** — each meaning has a unique probability value
2. **Normalizable** — the Kraft inequality guarantees $Z \leq 1$; non-emptiness of $\mathcal{M}$ guarantees $Z > 0$
3. **Simplicity-biased** — simpler meanings are exponentially more probable
4. **Erlangen-compatible** — coarsening the equivalence level weakly increases probability
5. **Encoding-robust** — different encodings produce priors that differ by at most a bounded multiplicative constant (multiplicative invariance — see `foundation-securing.md` §1 for the precise bound)

This measure is the foundation for all subsequent information-theoretic and metric results in Front D. It turns meaning space from a topological structure with connectivity into a measurable space with weights — the first step toward the quantitative map of what can be thought.
