# Learn Universal Language in 15 Minutes

> From zero to reading and writing geometric meaning structures.

---

## 1. What UL Is (2 min)

Universal Language (UL) treats **meaning as geometry**. Every meaningful statement can be decomposed into 5 geometric primitives:

| Primitive | Shape | Semantic Role | Think of it as... |
|-----------|-------|---------------|-------------------|
| **Point** | • | Entity — a thing that exists | A noun |
| **Line** | ─→ | Relation — a directed connection | A verb |
| **Angle** | ∠ | Modifier — a quality or measure | An adjective/adverb |
| **Curve** | ◠ | Process — a parameterized path | A dynamic verb |
| **Enclosure** | ○ | Concept — a bounded region | A sentence frame |

These 5 shapes + 13 algebraic operations = a complete system for expressing any compositional meaning.

---

## 2. The 4 Sorts (3 min)

Every element in UL belongs to one of 4 **sorts** (types):

| Sort | What it represents | UL notation | Example |
|------|--------------------|-------------|---------|
| **Entity (e)** | A thing, concept, or nominalized fact | `●` or `*` | "dog", "truth", "the fact that..." |
| **Relation (r)** | A connection between entities | `→` or `->` | "loves", "causes", "is-a" |
| **Modifier (m)** | A quality or transformation | `∠60` or `@60` | "big", "quickly", "most" |
| **Assertion (a)** | A complete claim | `○{...}` or `/0{...}` | "Dogs are loyal" |

**Key rule:** Operations have type signatures. You can't put a Relation where an Entity is expected. The sort system prevents nonsense.

---

## 3. Your First Glyph (5 min)

Let's encode: **"Knowledge is structured truth."**

### Step 1: Decompose

| Word/phrase | Sort | Why |
|-------------|------|-----|
| knowledge | Entity (e) | A thing |
| truth | Entity (e) | A thing |
| structured | Modifier (m) | Describes how truth is organized |
| is | Relation (r) | Connects knowledge to truth |

### Step 2: Select operations

1. `modify_entity(structured, truth)` → "structured truth" (modifier + entity → entity)
2. `predicate(knowledge, is, structured_truth)` → full assertion

### Step 3: Write in UL-Script

```
○{ ● "knowledge" → ○{ ∠ "structured" ● "truth" } }
```

Or in ASCII-only:
```
/0{ * "knowledge" -> /0{ @90 * "truth" } }
```

### Step 4: Read it back

1. Outer enclosure = assertion frame ("this is a claim")
2. First point = "knowledge"
3. Arrow = "is" (the relation)
4. Inner enclosure = "structured truth" (modifier + entity)

You've just written your first UL glyph.

---

## 4. Reading a Glyph (3 min)

Use the **5-pass reading procedure**:

1. **Enclosures first** — find all frames. Each frame = one claim or concept.
2. **Connections** — trace lines/arrows between entities. Direction matters.
3. **Angles** — check modifiers. What quality? What degree?
4. **Points** — identify entities (dots, labels).
5. **Curvature** — straight = static relation, curved = process/change.

### Practice: Read this

```
○{ ●"water" ~"freezes" }
```

- Pass 1: One enclosure (one assertion)
- Pass 2: Curve `~` = process
- Pass 3: No angles (no modifiers)
- Pass 4: One entity: "water"
- Pass 5: The curve means change over time

Reading: "Water freezes" (a process claim).

---

## 5. Next Steps (2 min)

| I want to... | Go to... |
|--------------|----------|
| Write more complex glyphs | `ul-core/writing-system/writers-companion.md` — 22 worked examples |
| Understand the formal system | `foundations/formal-foundations.md` — Σ_UL specification |
| Follow a structured curriculum | `docs/learning/curriculum.md` — Day 1–9 learning path |
| Practice with exercises | `docs/learning/exercises.md` — 10 exercises across 3 levels |
| Use UL programmatically | `docs/learning/developer-quickstart.md` — Rust, Python, WASM |
| Try the web editor | `ul-forge/web/` — live UL-Script → SVG rendering |
| Browse the glyph gallery | `ul-core/lexicon/gallery/index.md` — all 42 canonical entries (placeholder SVGs — real renders pending) |

**The 13 operations at a glance:**

| Category | Operations |
|----------|-----------|
| Core | `predicate`, `modify_entity`, `modify_relation` |
| Logic | `negate`, `conjoin`, `disjoin` |
| Embedding | `embed`, `abstract` |
| Relations | `compose`, `invert` |
| Quantification | `quantify`, `bind` |
| Assertion | `modify_assertion` |

That's it. Five shapes, four sorts, thirteen operations. Welcome to Universal Language.
