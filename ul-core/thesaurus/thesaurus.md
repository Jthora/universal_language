# Universal Language Thesaurus

> A map of meaning-equivalence classes under geometric transformation.  
> Two constructions are "synonyms" when related by structure-preserving maps.  
> This thesaurus is organized by the Erlangen hierarchy: the deeper the equivalence class, the more fundamental the synonymy.

---

## I. PRINCIPLE

In natural-language thesauri, synonymy is a matter of judgment: is "big" really the same as "large"? There's no principled answer.

In UL, **synonymy is mathematically defined.** Two constructions are synonyms at level L if they are related by a transformation in that level's symmetry group:

| Level | Group | Synonymy Type | Meaning |
|-------|-------|---------------|---------|
| **Euclidean** | E(n) | **Identical** | Same construction, different position/orientation |
| **Similarity** | Sim(n) | **Synonym** | Same structure, different scale (emphasis) |
| **Affine** | Aff(n) | **Paraphrase** | Same parallel structure, different proportions |
| **Projective** | PGL(n) | **Structural analog** | Same incidence structure, different perspective |
| **Topological** | Homeo | **Translation** | Same essential meaning, any form |

---

## II. THE EXISTENCE FAMILY (Point-Derived)

### Core: • (Existence)

| Construction | Level | Relation to Core | Meaning |
|-------------|-------|------------------|---------|
| • | — | Identity | "Something is" |
| •⁺ (scaled up) | Similarity | Synonym | "Something significant is" |
| •⁻ (scaled down) | Similarity | Synonym | "Something minor is" |
| •₁ (positioned) | Euclidean | Identical (moved) | "This specific thing is" |
| •̄ (reflected) | Euclidean | Identical (reflected) | "Something is" (negation requires assertion, not entity) |
| Ø (void) | — | **Antonym** | "Nothing is" |

### Extended Existence Family

| Concept | Construction | Synonyms (Similarity level) |
|---------|-------------|---------------------------|
| **Being** | • | •⁺, •⁻ |
| **Identity** (self-sameness) | •═• | ≅, ∠0° |
| **Distinction** (difference) | •──•  | •···•, •╌╌• (at distance) |
| **Multiplicity** | •,•,•,... | ••• (translation copies) |
| **Individuality** | •₁ (unique position) | (no similarity synonyms — uniqueness is the point) |

### Antonym Pairs

| Concept | Construction | Antonym | Construction |
|---------|-------------|---------|-------------|
| Existence | • | Non-existence | Ø |
| One | • | Many | •,•,•,... |
| Here | •_center | There | •_periphery |
| Specific | •₁ | General | ○ |

---

## III. THE RELATION FAMILY (Line-Derived)

### Core: ─ (Relation)

| Construction | Level | Relation to Core | Meaning |
|-------------|-------|------------------|---------|
| ─ | — | Identity | "Something relates to something" |
| → | Euclidean | Directed variant | "Something acts on something" |
| ← | Euclidean | Inverse | "Something is acted upon" |
| ↔ | Euclidean | Bidirectional | "Things relate mutually" |
| ═ | Similarity | Intensified | "Strongly relates" |
| ╌╌ | Similarity | Attenuated | "Weakly/possibly relates" |
| ━ | Similarity | Necessitated | "Must relate" |

### Synonymy Classes (Similarity Level)

| Class | Members | Common Meaning |
|-------|---------|---------------|
| **Action** | →, ══→, →→ | "Directed force from A to B" |
| **Reception** | ←, ←══, ←← | "Directed force received by A from B" |
| **Connection** | ─, ═, ━ | "Undirected link between A and B" |
| **Tentative link** | ╌╌, ···, - - - | "Possible/uncertain connection" |

### Relation Antonym Pairs

| Relation | Construction | Antonym | Construction |
|----------|-------------|---------|-------------|
| Acts on | → | Is acted upon | ← |
| Connected | ─ | Disconnected | • •  (gap) |
| Strong | ═ | Weak | ╌╌ |
| Necessary | ━ | Possible | ╌╌ |
| Direct | → | Indirect | →•→ (mediated) |

---

## IV. THE QUALITY FAMILY (Angle-Derived)

### Core Qualities (Structurally Distinguished Angles)

| Angle | Quality | Synonyms (nearby angles) | Antonym |
|-------|---------|-------------------------|---------|
| **0°** | Identity / Agreement | ~5° (near-identity) | 180° (Opposition) |
| **60°** | Harmony / Balance | 55°–65° (near-harmony) | 120°? or 180° |
| **90°** | Independence | 85°–95° (near-independence) | 0° (Identity) |
| **120°** | Complementarity | 115°–125° | 60° (Harmony) |
| **180°** | Opposition / Negation | 175°–180° (near-opposition) | 0° (Identity) |
| **360°** | Completion / Return | 355°–360° (near-completion) | 0° (at origin, no journey) |

### The Continuous Quality Spectrum

Between the structural landmarks, qualities form a continuous gradient. This is the thesaurus's richest space — analogous to a color wheel:

```
0°────────60°──────90°──────120°──────180°──────270°──────360°
Identity  Harmony  Orthog.  Compl.   Opposition  Reflexive  Completion
 ↑                                                           ↑
 └─────────── same point (0° = 360°) ──────────────────────┘
```

**Reading the spectrum:**
- 0°–60°: Agreement → Cooperation → Harmony
- 60°–90°: Harmony → Differentiation → Independence
- 90°–120°: Independence → Complementarity
- 120°–180°: Complementarity → Tension → Opposition
- 180°–270°: Opposition → Inversion → Reflexivity
- 270°–360°: Reflexivity → Return → Completion → Identity

### Quality Thesaurus Entries

| Quality Region | Angular Range | Synonymous Concepts |
|---------------|--------------|---------------------|
| **Agreement** | 0°–15° | same, identical, equivalent, matching, congruent |
| **Cooperation** | 15°–45° | allied, sympathetic, aligned, concordant |
| **Harmony** | 45°–75° | balanced, equitable, proportional, resonant |
| **Differentiation** | 75°–85° | distinct, separate, other, alternative |
| **Independence** | 85°–95° | orthogonal, unrelated, autonomous, free |
| **Complementarity** | 95°–135° | complementary, completing, supplementary |
| **Tension** | 135°–165° | conflicting, straining, competing, resistant |
| **Opposition** | 165°–180° | opposite, contrary, antithetical, negating |

---

## V. THE PROCESS FAMILY (Curve-Derived)

### Core Processes

| Process | Curve | Synonyms | Antonym |
|---------|-------|----------|---------|
| **Change** | ◠ (generic arc) | ◠⁺ (big change), ◠⁻ (small change) | ─ (no change, straight) |
| **Cycling** | ○ (circle) | ~~ (oscillation) | → (linear progression) |
| **Growth** | 𝒮 (outward spiral) | ⌒⁺ (expanding arc) | 𝒮⁻¹ (inward spiral = decay) |
| **Decay** | 𝒮⁻¹ (inward spiral) | ◠↓ (downward curve) | 𝒮 (growth) |
| **Oscillation** | ~ (sine wave) | ○ (circle), ◠◠◠ (repeated arcs) | ─ (constant) |
| **Progression** | ⧖ (helix) | 𝒮→ (spiral + direction) | ○ (cycling without advance) |

### Process Antonym Pairs

| Process | Symbol | Antonym | Symbol |
|---------|--------|---------|--------|
| Growth | 𝒮 | Decay | 𝒮⁻¹ |
| Change | ◠ | Stasis | ─ |
| Advance | ⧖ | Return | ○ |
| Acceleration | κ↑ (increasing curvature) | Deceleration | κ↓ |
| Creation | Ø → • | Destruction | • → Ø |

### Process Synonymy Classes (Topological Level)

At the topological level, processes are classified by their **fundamental group**:

| π₁ | Topological Type | Process Class | Members |
|----|-----------------|---------------|---------|
| {e} | Simply connected (arc) | **One-shot process** | ◠, ⌒, any open curve |
| ℤ | One loop | **Cyclic process** | ○, ~, oscillation |
| ℤ×ℤ | Two loops | **Doubly-cyclic** | Lissajous, coupled oscillators |
| Free | Figure-eight | **Self-crossing** | Paradoxical process, strange loop |

---

## VI. THE CONCEPT FAMILY (Enclosure-Derived)

### Concept Types by Symmetry

| Enclosure | Symmetry | Concept Class | Synonymous Enclosures |
|-----------|----------|--------------|----------------------|
| △ | D₃ | Fundamental | Any 3-sided enclosure |
| □ | D₄ | Structural | Any 4-sided regular enclosure |
| ⬠ | D₅ | Organic | Any 5-sided regular enclosure |
| ⬡ | D₆ | Networked | Any 6-sided regular enclosure |
| ○ | SO(2) | Universal | Any smooth closed curve |

### Concept Antonym Pairs

| Concept | Construction | Antonym | Construction |
|---------|-------------|---------|-------------|
| Whole | ○{...many...} | Part | △{•} |
| Defined | □{clearly bounded} | Undefined | ╌╌╌{fuzzy boundary} |
| Universal | ○ | Particular | △ |
| Open | enclosure with gap in boundary | Closed | enclosure with complete boundary |
| Simple | genus-0 enclosure | Complex | genus-2+ enclosure |
| Full | enclosure with dense interior | Empty | enclosure with vacant interior |

---

## VII. CROSS-CATEGORY THESAURUS

> **Note:** The "Primary Construction" column provides reference anchors for the synonym relationships — it is not a source of canonical definitions. For the canonical treatment of structurally distinguished constructions (with tier justifications), see the **Lexicon** (lexicon/lexicon.md). The synonym and level-of-equivalence data in this section IS the thesaurus's unique contribution.

### Abstract Concepts (Compound Synonymy)

| Concept | Primary Construction | Synonym 1 | Synonym 2 | Level |
|---------|---------------------|-----------|-----------|-------|
| **Truth** | ○{•} | ○{•⁺} | ○̄{Ø}̄ (double negation of emptiness) | Topological |
| **Knowledge** | ○{○{•}─∠60°─○} | □{•↔○} | ○{•═○} | Affine |
| **Change** | ◠ within □ | □{◠} | □{𝒮⁻} | Similarity |
| **Freedom** | •↑ (unbounded ray) | •↑↑ | •⁺↑ | Similarity |
| **Constraint** | □{•} | △{•} | ○small{•} | Affine |
| **Cause** | •══→→• | •━→• | •→→• | Similarity |
| **Time** | ⧖ | ◠→ | 𝒮→ | Topological |
| **Space** | □{•,•,•,•} | ○{•,•,•,•} | grid of • | Affine |

### Emotion/Experience Concepts

| Concept | Primary | Synonym at Similarity | Synonym at Topological |
|---------|---------|----------------------|----------------------|
| **Love** | ○{•≅•} | ○{•═•} | any simply-connected region containing identified pair |
| **Fear** | •←══□ | •←━□ | any receiving-relation from enclosure (constraint threatens existence) |
| **Joy** | ⬠{𝒮}⁺ | ⬠{◠↑}⁺ | any living-enclosure with outward growth |
| **Sorrow** | ⬠{𝒮⁻¹} | ⬠{◠↓} | any living-enclosure with inward decay |
| **Anger** | •══∠180°══• | •━∠180°━• | any strong opposition between existences |
| **Wonder** | ○{?} (rotated 180°) | ○{◠◠?} | any complete-concept containing questioning |
| **Peace** | △∠60°△∠60°△ | ⬡{∠60°,∠60°,...} | any fully-harmonious balanced structure |

---

## VIII. TRANSFORMATION THESAURUS

How to navigate between related meanings by applying geometric transformations:

### From Any Concept X:

| To Get: | Apply: | Example |
|---------|--------|---------|
| **Opposite of X** | Reflect (∠180°) | Truth → Falsity |
| **More intense X** | Scale up | warm → hot → burning |
| **Less intense X** | Scale down | warm → lukewarm → tepid |
| **X as a quality** | abstract(X) | wood → wooden, structure → structural |
| **X as a fact** | embed(predicate(•, →, X)) | rain → "the fact that it rains" |
| **X in the past** | Translate leftward | is → was |
| **X in the future** | Translate rightward | is → will be |
| **X from another perspective** | Rotate by θ | "A kills B" → rotate → "B dies because of A" |
| **X at a higher level** | Project (collapse dimension) | "this dog" → "dogs" → "animals" |
| **X at a lower level** | Section (intersect plane) | "animals" → "dogs" → "this dog" |
| **X as a question** | Rotate 180° | "It rains" → "Does it rain?" |
| **Many X** | Translation copies | dog → dogs |
| **Hypothetical X** | Dashed lines | "is" → "might be" |
| **Necessary X** | Bold lines | "is" → "must be" |
| **X done to another** | compose with predicate | "run" → "make run" |
| **Reversed X** | invert(X) | "A teaches B" → "B learns from A" |

---

## IX. NAVIGATING THE THESAURUS

### By Geometric Operation

Want to find a synonym? Apply a structure-preserving transformation:
- **Same meaning, different emphasis:** Scale (Similarity group)
- **Same structure, different content:** Affine transformation
- **Same deep meaning, any form:** Homeomorphism (topological equivalence)

### By Semantic Proximity

Close meanings share more geometric structure:
- **Nearest neighbors** share everything except position (Euclidean synonyms)
- **Close neighbors** share angles and ratios (Similarity synonyms)
- **Structural neighbors** share parallel structure (Affine synonyms)
- **Deep neighbors** share incidence (Projective synonyms)
- **Essential neighbors** share topology (Topological synonyms)

### By Antonym Search

Every concept's antonym is found by the transformation that maximally inverts it:
- **For angles:** Add 180° (opposition)
- **For directions:** Reverse (invert)
- **For scales:** Reciprocal (large → small)
- **For enclosures:** Interior ↔ Exterior (complement)
- **For processes:** Reverse curvature (growth → decay)

---

## X. INDEX: CONCEPTS BY TOPOLOGICAL EQUIVALENCE CLASS

The deepest classification. Concepts with the same fundamental group are the "same meaning" at the most abstract level.

### Simply Connected (π₁ = {e}) — Simple Concepts
Existence, Truth, Identity, Unity, Point, Being, Presence, Here, Now, This

### One-Loop (π₁ = ℤ) — Self-Referential Concepts
Cycle, Return, Self, Consciousness, Recursion, Feedback, Habit, Ritual, Orbit

### Two-Loop (π₁ = ℤ × ℤ) — Dialectical Concepts
Dialogue, Debate, Duality, Complementarity, Wave-Particle, Freedom-Constraint

### Free Group (π₁ = F₂) — Paradoxical Concepts
Paradox, Self-reference ("this statement is false"), Strange loop, Gödel sentence, Liar's paradox

### Higher Genus — Multiply-Complex Concepts
Networks, Ecosystems, Languages themselves, Societies, Economies
(These have fundamental groups of increasing complexity, reflecting their irreducible structural intricacy.)
