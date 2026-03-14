# LLM Interface

> Natural language ↔ UL-Script translation via large language models.

---

## Capabilities

| Direction | Input | Output | Example |
|-----------|-------|--------|---------|
| **Compose** | Natural language description | UL-Script (or GIR JSON) | "justice as balanced force" → `◯(△(• •) ☰)` |
| **Explain** | UL-Script | Natural language explanation | `◯(•) → △(•)` → "A concept directed toward a structure" |
| **Suggest** | Partial UL-Script | Completion suggestions | `◯(•) → ` → `△(•)`, `□(•)`, `◯(• •)` |

---

## Compose Flow

```
User: "Represent justice as balanced opposition"
  │
  ▼
Prompt construction:
  System: "You are a Universal Language composer. Given a concept,
           produce UL-Script using these 5 geometric primitives:
           point (•), line (→), angle (/), curve (~), enclosure (◯ △ □).
           [Σ_UL sort rules]. [Canonical lexicon reference]."
  User: "Represent justice as balanced opposition"
  │
  ▼
LLM generates: "◯(△(• →← •))"
  │
  ▼
Parse UL-Script → GIR
  │
  ▼
Validate GIR
  │
  ├── valid? → Present to user with explanation
  └── invalid? → Feed error back to LLM → Retry (max 3)
```

---

## Prompt Engineering

The system prompt includes:
1. **Σ_UL specification** — 5 primitives, 4 sorts, 11 operations (condensed)
2. **Canonical lexicon** — 42 standard glyphs with their UL-Script and meanings
3. **Sort rules** — what connects to what (Entity→Relation→Entity, Modifier→Entity, etc.)
4. **Output format** — strict UL-Script text or JSON GIR

The prompt is designed to be:
- **Concise** — fits in a single system message (under 2000 tokens)
- **Parseable** — LLM output can be directly fed to `parse()` or `JSON.parse()`
- **Validated** — the validator catches any sort or structural errors the LLM makes

---

## Explain Flow

```
User clicks "Explain" on a glyph
  │
  ▼
System: "You are a Universal Language interpreter. Given UL-Script,
         explain the geometric meaning. [Σ_UL reference]."
User: "◯(•) → △(•)"
  │
  ▼
LLM: "This glyph shows a concept (circle containing a point of 
      existence) directed toward a structure (triangle containing 
      a point of existence). The arrow indicates a relation from 
      the conceptual to the structural."
```

---

## Suggest Flow

```
User types: "◯(•) → "
  │
  ▼
System: "Complete this UL-Script expression. Suggest 3 possible
         continuations that are geometrically valid."
User: "◯(•) → "
  │
  ▼
LLM: ["△(•)", "□(•)", "◯(• •)"]
  │
  ▼
Parse and validate each → filter invalid → present to user
```

---

## Model Requirements

Any LLM that supports structured output can be used:

| Provider | Models | Notes |
|----------|--------|-------|
| OpenAI | GPT-4o, GPT-4o-mini | JSON mode for GIR output |
| Anthropic | Claude 3.5 Sonnet, Claude Opus 4 | System prompt + structured output |
| Google | Gemini 1.5 Pro | Structured generation |
| Local | Llama 3, Mistral | Via Ollama or vLLM |

The interface is provider-agnostic. It sends a prompt, receives text, and parses the result.

---

## Configuration

```json
{
  "ai": {
    "provider": "openai",
    "model": "gpt-4o",
    "temperature": 0.3,
    "max_retries": 3,
    "output_format": "ul-script"
  }
}
```

- `temperature`: Low (0.2-0.4) for compose/suggest (precision), higher (0.6-0.8) for creative exploration
- `output_format`: `"ul-script"` for text, `"gir-json"` for structured output
- `max_retries`: Number of validation-feedback loops before giving up

---

## Validation Feedback Loop

When the LLM generates invalid UL-Script:

```
Attempt 1: LLM generates "◯(△)" → parse error (triangle needs content)
  → Feedback: "Parse error: enclosure '△' requires at least one enclosed element"
Attempt 2: LLM generates "◯(△(•))" → valid ✓
```

The feedback includes the specific error message from the parser/validator, giving the LLM actionable information for correction.
