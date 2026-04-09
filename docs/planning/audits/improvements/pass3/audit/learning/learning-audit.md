# Learning & Teaching Readiness Audit

**Date:** 2026-04-08  
**Scope:** All learning materials, tutorial paths, demo content, game mechanics  
**Goal:** Assess what a learner experiences today and what's needed for a learning app

---

## Executive Summary

**Grade: C+ (Good reference docs, no tutorial path or demos)**

UL has excellent reference documentation — the Five Siblings architecture is well-structured, the Writer's Companion provides worked examples, and the formal foundations are rigorous. However, there is no guided learning experience. A new learner must self-organize their journey through ~15 documents without guidance on order, pacing, or practice activities. The ProtoFusionGirl game module (ul-game) has sophisticated learning mechanics but isn't playable.

---

## 1. Current Learning Materials Inventory

### 1.1 What Exists

| Material | Location | Type | Grade |
|----------|----------|------|-------|
| Writer's Companion | `ul-core/writing-system/writers-companion.md` | Worked examples (10) | **A** |
| 5-Pass Reading Procedure | `ul-core/writing-system/writing-system.md` | Algorithm | **A** |
| Lexicon | `ul-core/lexicon/lexicon.md` | Reference (42 entries) | **A** |
| Navigation Guide | `ul-core/NAVIGATION.md` | Quick-start paths (6) | **A-** |
| System Synthesis | `ul-core/SYNTHESIS.md` | Conceptual overview | **A** |
| Grammar Theory | `ul-core/grammar/grammar-book.md` | Mathematical theory | **A-** |
| Syntax Operations | `ul-core/syntax/syntax-dictionary.md` | Operation reference | **A** |
| Synonym Navigation | `ul-core/thesaurus/thesaurus.md` | Meaning pathways | **A** |
| Symbol Catalog | `ul-core/symbology/symbol-map.md` | Visual reference | **B+** |
| Glyph Composition | `ul-core/composition/glyph-composition.md` | Advanced theory | **A-** |
| Game Engine | `ul-forge/crates/ul-game/` | Rust WASM module | **A-** |
| Experiment Prompts | `experiments/demo/prompts/` | Test stimuli (not tutorials) | **C** |
| FOR-AI | `FOR-AI.md` | AI-audience philosophical guide | **A** |

### 1.2 What Doesn't Exist

| Material | Need | Impact |
|----------|------|--------|
| **"Learn UL in 15 Minutes" tutorial** | First-contact experience | CRITICAL |
| **Prescribed learning sequence** | Day-by-day curriculum | HIGH |
| **Practice exercises** | After each section | HIGH |
| **Visual glyph gallery** | 42 SVGs browsable | HIGH |
| **Interactive web demo** | Try UL without installing anything | HIGH |
| **Code tutorials** | "Build your first parser" | MEDIUM |
| **Video content** | Screencasts, animations | MEDIUM |
| **Jupyter notebooks** | Interactive Python exploration | MEDIUM |
| **Playable game demo** | ProtoFusionGirl in browser | MEDIUM |
| **Assessment/quiz system** | Test understanding | LOW |
| **Certification path** | "UL Practitioner" milestones | LOW |

---

## 2. Learner Journey Analysis

### 2.1 Current Journey (Today)

```
1. Land on README.md → "What is UL?"
   ↓ (unclear what to click next)
2. Maybe find AGENTS.md or FOR-AI.md (AI-specific)
   ↓ (not for human learners)
3. Navigate to ul-core/README.md
   ↓ (deliverable table, not a tutorial)
4. Find NAVIGATION.md → 6 quick-start paths
   ↓ (good! but which one?)
5. Read SYNTHESIS.md → conceptual overview
   ↓ (now understand the pipeline)
6. Jump between siblings based on interest
   ↓ (no guidance on ORDER)
7. Eventually find Writer's Companion
   ↓ (worked examples!)
8. Try to write a glyph by hand
   ↓ (no way to verify correctness)
9. Give up or persevere
```

**Pain points:**
- No "start here" for humans on the main README
- No way to verify correctness without installing tools
- No interactive feedback loop
- No practice exercises between concepts
- No visual examples until they find the web editor (buried in ul-forge/web)

### 2.2 Ideal Journey (Target)

```
1. Land on README.md → "Start Learning" button/link
   ↓
2. "Learn UL in 15 Minutes" quick tutorial
   ↓ (5 primitives, 3 simple glyphs, "try it yourself" link)
3. Interactive web demo: type UL-Script → see glyph
   ↓ (immediate feedback!)
4. Prescribed learning path: Day 1 → Day 9
   ↓ (clear expectations, pacing)
5. Each day: Read section → Practice exercise → Verify in editor
   ↓ (learn-by-doing loop)
6. Visual glyph gallery: browse 42 canonical entries
   ↓ (see what's possible)
7. Advanced: Writer's Companion worked examples
   ↓ (deep practice)
8. Game mode: ProtoFusionGirl challenges
   ↓ (gamified mastery)
9. "Build with UL": npm install @ul-forge/core
   ↓ (transition from learner to developer)
```

---

## 3. ProtoFusionGirl Game Module (ul-game)

### 3.1 What's Implemented

The game crate (`ul-forge/crates/ul-game/`) has 12 Rust modules:

| Module | Purpose | Pedagogical Feature |
|--------|---------|---------------------|
| `evaluation.rs` | Pattern matching against rules | Checks learner's glyphs |
| `scoring.rs` | 4-dimensional partial credit | Structure, sort, operation, sequence |
| `animation.rs` | Keyframe generation | Visual construction feedback |
| `context.rs` | Game state management | Persistent progress |
| `difficulty.rs` | EMA proficiency tracking | Adaptive per-primitive |
| `hints.rs` | Hint generation | Scaffolding for stuck learners |
| `sequence.rs` | Construction order analysis | Step-by-step guidance |
| `cosmic.rs` | Composition rule engine | Validates against Σ_UL |
| `modding.rs` | Runtime rule injection | Community content |
| `analysis.rs` | Glyph analysis | Diagnostic feedback |

### 3.2 Pedagogical Design Quality

**Excellent.** Key features:
- **Partial credit scoring** — not binary pass/fail; rewards structural understanding
- **Adaptive difficulty** — EMA tracks proficiency per geometric primitive (Point, Line, Angle, Curve, Enclosure) and adjusts challenge level
- **Hint system** — generates contextual hints when learner is stuck
- **Construction animation** — shows BFS build order for correct glyphs
- **Performance:** <1ms evaluate, <16ms score (60fps game-loop safe)

### 3.3 Current Blockers

| Issue | Impact | Fix |
|-------|--------|-----|
| WASM only — requires Phaser 3 integration | Can't play standalone | Build minimal web wrapper |
| No game levels defined | Nothing to play | Design curriculum-aligned challenges |
| Modding system undocumented | Community can't contribute | Write modding guide |
| Not exposed via Python bindings | Jupyter learners can't use | Add PyO3 wrappers |

---

## 4. Web Editor as Learning Tool

### 4.1 Current State

The web editor (`ul-forge/web/`) provides:
- ✅ Monaco editor with UL-Script syntax highlighting
- ✅ Live SVG preview (100ms debounce)
- ✅ Template palette (42 lexicon entries)
- ✅ Visual canvas (5 primitives, select, connect tools)
- ✅ Export (SVG/PNG/GIR)

### 4.2 Learning Features Missing

| Feature | Why It Matters | Effort |
|---------|---------------|--------|
| **Guided mode** | Step-by-step instructions overlaid on editor | HIGH |
| **Validation explanations** | "This fails because X" not just red squiggles | MEDIUM |
| **Operation composer** | Pick an operation, select inputs, see result | HIGH |
| **Hint integration** | Pull hints from ul-game engine | MEDIUM |
| **Progress tracking** | Remember what the learner has mastered | MEDIUM |
| **Challenge mode** | "Write the glyph for concept X" with scoring | HIGH |

### 4.3 Deployment

Current state: development build only (`npm run dev`). Not deployed anywhere.

**To make accessible:** Deploy to GitHub Pages, Vercel, or Netlify. Then learners can use without installing anything.

---

## 5. Example Coverage Analysis

### 5.1 Writer's Companion Examples by Concept

| Example | Concept | Operations Used |
|---------|---------|-----------------|
| Ex 1 | "Knowledge is structured truth" | predicate, modify_entity, modify_relation |
| Ex 2 | "Evolution is a process..." | predicate, compose (implied) |
| Ex 3 | "If temperature drops..." | embed, negate (conditional) |
| Ex 4 | "Democracy is a system..." | embed, quantify (self-reference) |
| Ex 5 | "Art transforms suffering..." | conjoin, embed (parallel claims) |
| Ex 6 | "Love is patient" | modify_entity, predicate (property) |
| Ex 7–10 | Compound examples | Various combinations |

### 5.2 Missing Concept Coverage

| Concept | No Example | Impact |
|---------|------------|--------|
| Modal necessity (□) | Learner can't write "X must be true" | HIGH |
| Modal possibility (◇) | Learner can't write "X might be" | HIGH |
| Counterfactual (□→) | Learner can't write "if X were..." | HIGH |
| Performative query | Learner can't write "is X?" | HIGH |
| Performative directive | Learner can't write "do X!" | HIGH |
| Pragmatic inference | Learner can't write surface→intended | MEDIUM |
| Variable binding (bind) | Learner can't write co-reference | MEDIUM |
| Relation inversion (invert) | Learner can't write active↔passive | LOW |
| Assertion modification | Learner can't write "reportedly, X" | LOW |

---

## 6. Comparison with State-of-Art Learning Systems

| Feature | Duolingo | Khan Academy | UL (Current) | UL (Target) |
|---------|----------|-------------|--------------|-------------|
| Progressive lessons | ✅ | ✅ | ❌ | ✅ |
| Immediate feedback | ✅ | ✅ | ⚠️ (editor only) | ✅ |
| Adaptive difficulty | ✅ | ⚠️ | ✅ (ul-game) | ✅ |
| Gamification | ✅ | ⚠️ | ✅ (ul-game) | ✅ |
| Reference docs | ⚠️ | ✅ | ✅✅ | ✅✅ |
| Visual content | ✅ | ✅ | ❌ | ✅ |
| Community challenges | ✅ | ⚠️ | ❌ | ✅ (modding) |
| Mobile access | ✅ | ✅ | ❌ | ⚠️ |
| No-install access | ✅ | ✅ | ❌ | ✅ |

---

## 7. Priority Actions for Pass 3

### Phase 2a: Tutorial Content (Critical Path)

| # | Action | Deliverable | Effort |
|---|--------|-------------|--------|
| 1 | Write "Learn UL in 15 Minutes" | New document, linked from README | MEDIUM |
| 2 | Create prescribed learning sequence | Day 1–9 curriculum | LOW |
| 3 | Add 3 advanced Writer's Companion examples | Modal, performative, pragmatic | MEDIUM |
| 4 | Create visual glyph gallery | 42 SVGs + browsable page | MEDIUM |
| 5 | Design 10 practice exercises (Levels 1–3) | After each section | MEDIUM |

### Phase 2b: Interactive Experience

| # | Action | Deliverable | Effort |
|---|--------|-------------|--------|
| 6 | Deploy web editor to GitHub Pages | Live URL for learners | LOW |
| 7 | Add validation explanations to web editor | Human-readable errors | MEDIUM |
| 8 | Build operation composer UI | Drag-drop operation application | HIGH |
| 9 | Integrate ul-game hints into web editor | Contextual help | MEDIUM |
| 10 | Create 5 game challenge levels | First playable UL game content | HIGH |

### Phase 2c: Developer Learning

| # | Action | Deliverable | Effort |
|---|--------|-------------|--------|
| 11 | Write "Build Your First UL Parser" tutorial | Code walkthrough | MEDIUM |
| 12 | Create Jupyter notebook walkthrough | Interactive Python exploration | MEDIUM |
| 13 | Write integration examples (React, Python, CLI) | 3 code samples | MEDIUM |
| 14 | Produce video screencast: "Writing Your First Glyph" | 5-minute video | MEDIUM |
