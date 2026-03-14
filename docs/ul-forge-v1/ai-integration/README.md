# AI Integration

> How AI models interact with the UL Forge toolchain.

---

## Overview

UL Forge provides four AI integration surfaces, each addressing a different modality:

| Interface | Model Type | Input | Output | Phase |
|-----------|-----------|-------|--------|-------|
| [LLM Interface](llm-interface.md) | Text LLM (GPT, Claude, Gemini) | Natural language | UL-Script or GIR JSON | Phase 4 |
| [Vision Interface](vision-interface.md) | Vision model | Handwritten/photo glyph | GIR JSON | Phase 4 |
| [GNN Interface](gnn-interface.md) | Graph Neural Network | GIR graph | Similarity scores, completions | Phase 4 |
| [Theorem Prover](theorem-prover.md) | SAT/SMT solver | Sort constraints | Validity proof | Phase 4 |

All AI interfaces consume and produce GIR — UL Forge's canonical data format. The AI never operates on raw SVG or pixel data; it works on the structured graph representation.

---

## Design Principles

### 1. GIR as Interchange

Every AI interface converts to/from GIR. This means:
- AI-generated glyphs are validated by the same validator
- AI output is rendered by the same renderer
- No special cases for AI-generated content

### 2. Validation-in-the-Loop

AI-generated GIR is **never trusted**. Every AI output passes through the validator before rendering:

```
AI generates GIR → Validator → pass? → Render
                              │
                              fail? → Feed errors back to AI → Retry (max 3)
```

### 3. Human-in-the-Loop (by default)

AI suggestions are presented to the user for approval. The AI does not autonomously modify the user's glyph. The user can:
- Accept the suggestion
- Modify and accept
- Reject and ask for alternatives

### 4. Model-Agnostic

Interfaces define the **contract** (input/output format), not the model. Any LLM, vision model, or GNN that can produce valid GIR is compatible.

---

## Integration Architecture

```
┌──────────────────────────────────────────┐
│               Web Editor                  │
│                                          │
│  ┌──────────┐     ┌──────────────────┐  │
│  │ AI Panel  │────→│ AI Router        │  │
│  │ (user     │     │ (selects model   │  │
│  │  prompt)  │     │  and interface)  │  │
│  └──────────┘     └──────────────────┘  │
│                          │               │
│              ┌───────────┼───────────┐   │
│              ▼           ▼           ▼   │
│         ┌────────┐ ┌─────────┐ ┌──────┐ │
│         │  LLM   │ │ Vision  │ │ GNN  │ │
│         │ Client │ │ Client  │ │Client│ │
│         └────────┘ └─────────┘ └──────┘ │
│              │           │           │   │
│              └───────────┼───────────┘   │
│                          ▼               │
│                   ┌────────────┐         │
│                   │ Validator  │         │
│                   └────────────┘         │
│                          ▼               │
│                   ┌────────────┐         │
│                   │  GIR Store │         │
│                   └────────────┘         │
└──────────────────────────────────────────┘
```

---

## API Keys and Configuration

AI model API keys are never stored in the UL Forge codebase. They are provided by the user:

- **Web editor**: User enters API key in settings (stored in browser localStorage, never sent to UL Forge servers)
- **CLI**: Environment variables (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, etc.)
- **API server**: Server-side environment variables (for hosted deployments)

---

## Scope in v1

AI integration is Phase 4 — the interfaces are designed and documented, but implementation depends on the core pipeline (Phases 1-2) being stable. The LLM interface is the highest priority AI feature because it has the lowest implementation barrier (API call + prompt engineering, no model training required).
