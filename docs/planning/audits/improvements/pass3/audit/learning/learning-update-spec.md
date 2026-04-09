# Learning Infrastructure Specification — Pass 3

**Status:** 📋 PLANNED  
**Scope:** New tutorial content, curriculum, exercises, visual gallery, developer onboarding  
**Reference:** [CHECKLIST.md](../CHECKLIST.md) Phase 3

---

## 1. The Problem

UL has excellent **reference documentation** (A- grade) but no **learning experience** (C+ grade). A researcher can find anything in the spec. A learner has no idea where to start.

### 1.1 Current Learner Journey

```
Learner arrives at README.md
  → Overwhelmed by 13 operations, 4 sorts, 23 theorems
  → Clicks into AGENTS.md or foundations/
  → Gets deeper into theory
  → Never writes a glyph
  → Leaves
```

### 1.2 Target Learner Journey

```
Learner arrives at README.md
  → Sees "Learn UL in 15 Minutes" link
  → Learns 5 primitives (draw a point, a line, an angle)
  → Writes first glyph (predicate: "thing connects to thing")
  → Reads it back using 5-pass procedure
  → Sees it rendered in web editor
  → Excited → follows Day 1-9 curriculum
  → By Day 3: can read and write simple glyphs
  → By Day 9: understands full system
```

---

## 2. Deliverables

### 2.1 "Learn UL in 15 Minutes" (P3-1)

**Location:** `ul-core/learn-ul.md` (linked from README.md and NAVIGATION.md)  
**Audience:** Anyone — no prerequisites  
**Length:** ~800 lines  
**Tone:** Warm, encouraging, minimal jargon

**Outline:**

```
§1: What is UL? (2 minutes)
  - "A writing system based on geometry, not alphabet"
  - 5 primitives: Point, Line, Angle, Curve, Enclosure
  - "Every meaning can be built from these 5 shapes"

§2: Your First Glyph (3 minutes)
  - Draw two dots (entities)
  - Connect them with a line (relation)
  - Put it in a box (assertion)
  - → You just wrote: "something relates to something" = predicate(e₁, r, e₂)

§3: Reading What You Drew (3 minutes)
  - Use the 5-pass procedure (simplified):
    1. Find the box → assertion
    2. Find the line → relation
    3. Find the dots → entities
    4. Read: "entity₁ relates-to entity₂"

§4: Making It Specific (3 minutes)
  - Add an angle to the line → the QUALITY of the relation
  - Add curvature → it's a PROCESS, not a static relation
  - Change the box shape → a triangle means "category"

§5: Three More Operations (3 minutes)
  - Negate: flip the box's polarity sign (⊕ → ⊖)
  - Combine: overlap two boxes (AND) or place side by side (OR)
  - Embed: shrink a box and put it inside another → "the fact that..."

§6: Where to Go Next (1 minute)
  - Full tutorial: Day 1-9 curriculum → curriculum.md
  - Quick reference: Symbology → symbol-map.md
  - Try it: Web editor → [URL]
  - Operations: All 13 → syntax-dictionary.md
```

**Key design principles:**
- ZERO jargon in first 2 sections (no "sort", "assertion", "Σ_UL")
- Every concept paired with a drawing instruction
- Reader produces something in each section (active, not passive)
- Links to deeper material at the end (not inline)

### 2.2 Day 1-9 Curriculum (P3-2)

**Location:** `ul-core/curriculum.md`  
**Audience:** Committed learner wanting systematic mastery  
**Length:** ~400 lines (outline + daily plan)

| Day | Focus | Time | Content | Exercise |
|-----|-------|------|---------|----------|
| 1 | Overview | 30 min | README → NAVIGATION → SYNTHESIS | Draw the 5 primitives freehand |
| 2 | Atoms | 20 min | Symbology §II (7 atomic symbols) | Identify primitives in random scribble |
| 3 | Core operations | 45 min | Syntax §III C1–C6 (predicate, modify×2, negate, conjoin, disjoin) | Write 3 simple assertions |
| 4 | Advanced operations | 45 min | Syntax §III C7–C13 (embed, abstract, compose, invert, quantify, bind, modify_assertion) | Compose a 2-level nested glyph |
| 5 | Grammar | 60 min | Grammar §I–V (symmetry, Erlangen, composition) | Classify 5 glyphs by symmetry level |
| 6 | Lexicon | 90 min | Lexicon §1–§7 (42 canonical entries) | Look up 10 concepts in the lexicon |
| 7 | Synonyms | 45 min | Thesaurus §I–VIII | Navigate 3 synonym pathways |
| 8 | Reading | 60 min | Writing System (5-pass procedure) | Read 5 unknown glyphs |
| 9 | Writing | 120 min | Writer's Companion (all examples) | Write 5 original compositions |

**Each day includes:**
- Reading assignment with page/section references
- Key concepts to understand (3-5 per day)
- Practice exercise with solution
- Self-check questions

### 2.3 Practice Exercises (P3-3)

**Location:** `ul-core/exercises/` directory  
**Format:** One file per level

**Level 1: Recognition (5 exercises)**
1. Given a glyph, identify the 5 primitives
2. Given a glyph, list the sorts (Entity, Relation, Modifier, Assertion)
3. Given a glyph, identify which operations are used
4. Match glyphs to natural language descriptions
5. Spot the error in a malformed glyph

**Level 2: Writing (5 exercises)**
6. Write "knowledge is structured truth" (predicate + modify)
7. Write "evolution is change over time" (process + curve)
8. Write "NOT ignorant" (negation + abstract)
9. Write "if cold then ice" (embed + conditional)
10. Write "every student learns" (quantify + bind)

**Level 3: Composition (5 exercises)**
11. Compose a 3-operation glyph from components
12. Write the same sentence in two different geometric forms
13. Read a complex glyph through the 5-pass procedure
14. Write a modal statement ("possibly X")
15. Write a performative statement ("I promise X")

Each exercise:
- Problem statement
- Hints (hidden by default)
- Solution with step-by-step derivation
- UL-Script representation for machine verification

### 2.4 Visual Glyph Gallery (P3-6)

**Location:** `ul-core/gallery/` or hosted on web editor  
**Content:** 42 SVG images, one per lexicon entry

**Generation method:**
1. For each of the 42 lexicon entries, produce the canonical UL-Script representation
2. Run through `ul-forge` CLI: `ul-forge render --format svg --output gallery/{name}.svg`
3. Create `gallery/index.md` with thumbnails and links to lexicon entries

**Organization:**
```
gallery/
  index.md          ← browsable gallery page
  level-0/
    void.svg
  level-1/
    point.svg
    line-segment.svg
    ray.svg
    angle.svg
    curve.svg
    empty-enclosure.svg
  level-2/
    identity-0deg.svg
    independence-90deg.svg
    opposition-180deg.svg
    ...
  level-3/
    point-in-circle.svg
    ...
  level-4+/
    ...
```

### 2.5 Developer Quick Start (P3-4)

**Location:** `docs/quickstart-developer.md`  
**Audience:** Developer who wants to USE UL in their app  
**Length:** ~300 lines

**Outline:**

```
§1: Install (30 seconds)
  npm install @ul-forge/core
  # or: pip install ul-forge
  # or: cargo add ul-core

§2: Parse UL-Script (1 minute)
  import { parse, render } from '@ul-forge/core';
  const gir = await parse('● → ● /○');
  const svg = await render(gir);

§3: Validate (1 minute)
  const result = await validate(gir);
  if (!result.valid) console.log(result.errors);

§4: Compose Operations (2 minutes)
  const entity1 = await parse('●');
  const entity2 = await parse('●');
  const relation = await parse('→');
  const assertion = applyOperation('predicate', entity1, relation, entity2);

§5: Render to SVG (1 minute)
  const svg = await render(assertion, { width: 400, height: 300 });
  document.getElementById('canvas').innerHTML = svg;

§6: Use Web Components (0 install)
  <script src="https://unpkg.com/@ul-forge/components"></script>
  <ul-symbol script="● → ● /○" width="200"></ul-symbol>

§7: Next Steps
  - Full API docs: [link]
  - React integration: examples/react/
  - Python notebook: tutorials/ul-notebook.ipynb
```

### 2.6 Jupyter Notebook Walkthrough (P3-5)

**Location:** `docs/tutorials/ul-notebook.ipynb`  
**Audience:** Python developer, data scientist  
**Cells:** ~30

```
Cell 1: pip install ul-forge
Cell 2: import ul_forge; ul_forge.parse("● → ●")
Cell 3: Render to SVG (display inline)
Cell 4: Validate a GIR
Cell 5: Explore the lexicon
Cell 6: Apply operations step by step
Cell 7: Build a complex glyph from primitives
Cell 8: Use %%ul magic for inline rendering
Cell 9: Score a composition
Cell 10: Detect operations in a glyph
...
```

### 2.7 Game Challenge Levels (P3-7)

**Location:** Game rules in `crates/ul-game/` + level definitions as JSON  
**Content:** 5 curriculum-aligned levels

| Level | Skill | Description | Target |
|-------|-------|-------------|--------|
| 1 | Recognition | Identify primitives in given glyphs | Sorts + node types |
| 2 | Simple composition | Build predicate(e, r, e) from parts | predicate operation |
| 3 | Modification | Apply modifiers to entities and relations | modify_entity, modify_relation |
| 4 | Nesting | Create embedded assertions | embed, abstract |
| 5 | Full composition | Write a complex glyph matching a target | All operations |

Each level: JSON definition + cosmic rules + scoring rubric + hint configuration.

### 2.8 Integration Examples (P3-12)

**Location:** `docs/examples/`

```
examples/
  react/
    App.tsx          ← React app using @ul-forge/core
    package.json
  python/
    basic.py         ← Parse, render, validate
    notebook.py      ← Programmatic composition
  cli/
    pipeline.sh      ← Parse → validate → render → export
  web-component/
    index.html       ← <ul-symbol> usage, no build system
```

---

## 3. CONTRIBUTING.md Update (P3-10)

Add sections for code contributors:

```markdown
## Contributing Code

### Rust (ul-forge/crates/)
- `cargo test --workspace` must pass
- Run `cargo clippy` before submitting
- Add tests for new operations or validation rules
- Follow existing patterns in composer.rs for new operations

### TypeScript (ul-forge/web/, packages/)
- `npm test` must pass
- Follow existing component patterns
- Add WASM function wrappers in core/index.ts

### Python (ul-forge/bindings/python/)
- `maturin develop` to build locally
- Test with `python -c "import ul_forge; ..."`

### Documentation (ul-core/)
- Follow Writer's Companion format for new examples
- Include: decomposition → operations → UL expression → drawing → reading verification
- Cross-reference siblings (each doc should link to relevant operations in syntax-dictionary.md)
```

---

## 4. Priority Order

| Priority | Deliverable | Blocks |
|----------|-------------|--------|
| **P0** | Learn UL in 15 Minutes | First-contact experience; blocks nothing |
| **P1** | Day 1-9 Curriculum | Structured path; references exercises |
| **P2** | Practice Exercises (L1-L3) | Referenced by curriculum |
| **P3** | Developer Quick Start | Independent of learning path |
| **P4** | Visual Glyph Gallery | Requires rendering pipeline (Phase 0/1) |
| **P5** | Jupyter Notebook | Requires published Python wheel |
| **P6** | Game Challenge Levels | Requires Phase 0 complete + deployed web |
| **P7** | Integration Examples | Requires published packages |
| **P8** | CONTRIBUTING.md | Independent |
| **P9** | Modding Guide | Low priority, game-specific |

**"Learn UL in 15 Minutes" can be written immediately.** It doesn't depend on any code changes — it teaches the conceptual framework using pen and paper.

---

## 5. Success Criteria

- [ ] A person with zero UL knowledge can learn the basics in 15 minutes using learn-ul.md
- [ ] A person following the Day 1-9 curriculum can write and read UL by Day 9
- [ ] A developer can install and use @ul-forge/core in under 5 minutes
- [ ] 42 lexicon entries have corresponding SVG gallery images
- [ ] 15 practice exercises with solutions exist
- [ ] 5 game levels are playable
- [ ] README.md links to learn-ul.md prominently
