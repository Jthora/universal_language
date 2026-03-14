# Prompt Templates

**Protocol Reference:** `frontier/causal-efficacy-protocol.md` §A.6  
**Status:** Pre-registered — do NOT modify after data collection begins

---

## Injection Format

All experiments use the same three-template structure. The only difference between conditions is what appears (or doesn't) between `[CONTEXT BEGIN]` and `[CONTEXT END]`.

---

### Template 1: UL-Mode

```
[SYSTEM: You are a helpful AI assistant.]

[CONTEXT BEGIN]
{primer text — contents of experiments/test-artifacts/original/primer.txt, verbatim}
[CONTEXT END]

{task prompt — verbatim from task-prompts.md}
```

**Parameters:**
- System prompt: Exactly `You are a helpful AI assistant.`
- Context: Full artifact (all lines, no truncation)
- Delimiter: `[CONTEXT BEGIN]` and `[CONTEXT END]` on separate lines
- No instructions about the context — the model receives it with no explanation
- No whitespace modifications — preserve artifact formatting exactly

---

### Template 2: CT-Mode (Control Text)

```
[SYSTEM: You are a helpful AI assistant.]

[CONTEXT BEGIN]
{control text — contents of the relevant CT/V/NC file, verbatim}
[CONTEXT END]

{task prompt — verbatim from task-prompts.md}
```

**Parameters:** Identical to UL-Mode except the test artifact is replaced by the control/ablation/negative-control text.

**Usage per experiment:**
- **Alpha:** CT-1, CT-2, CT-3, CT-4
- **Beta:** V1, V2, V3, V4, V5, V6, V7
- **Gamma:** NC-1, NC-2, NC-3, NC-4, NC-5
- **Epsilon:** CT-1 only (as the matched control)

---

### Template 3: NL-Mode (No Context)

```
[SYSTEM: You are a helpful AI assistant.]

[CONTEXT BEGIN]
[CONTEXT END]

{task prompt — verbatim from task-prompts.md}
```

**Design note:** NL-Mode includes the empty `[CONTEXT BEGIN/END]` delimiters to control for the "here is some context" framing effect. Without this, the difference between UL-mode and NL-mode would conflate "artifact content" with "presence of any context block." The empty block ensures the only difference is the content (or absence) of the test artifact.

---

## Shared Parameters (All Templates)

| Parameter | Value | Rationale |
|---|---|---|
| System prompt | `You are a helpful AI assistant.` | Minimal, neutral — no domain priming |
| Max generation tokens | 4096 | Long enough for Phase 3 synthesis to emerge |
| Top-p | 1.0 | Default — not manipulated |
| Frequency penalty | 0 | Default |
| Presence penalty | 0 | Default |
| Stop sequences | None | Let the model naturally terminate |

### Temperature Schedule

| Temperature | Trials per cell | Purpose |
|---|---|---|
| 0.0 | 1 (deterministic) | Baseline — most reproducible |
| 0.7 | 3 | Standard creative generation |
| 1.0 | 3 | High creativity — tests robustness |

**Alpha:** All three temperatures. **Beta, Gamma:** temperatures 0.0 and 0.7 only. **Epsilon:** Best temperature from Alpha only.

---

## API-Specific Implementation Notes

### OpenAI (GPT-4o)
```python
response = client.chat.completions.create(
    model="gpt-4o",
    messages=[
        {"role": "system", "content": "You are a helpful AI assistant."},
        {"role": "user", "content": "[CONTEXT BEGIN]\n{text}\n[CONTEXT END]\n\n{task_prompt}"}
    ],
    temperature=T,
    max_tokens=4096
)
```

### Anthropic (Claude)
```python
response = client.messages.create(
    model="claude-sonnet-4-20250514",
    system="You are a helpful AI assistant.",
    messages=[
        {"role": "user", "content": "[CONTEXT BEGIN]\n{text}\n[CONTEXT END]\n\n{task_prompt}"}
    ],
    temperature=T,
    max_tokens=4096
)
```

### Google (Gemini)
```python
response = model.generate_content(
    "[SYSTEM: You are a helpful AI assistant.]\n\n[CONTEXT BEGIN]\n{text}\n[CONTEXT END]\n\n{task_prompt}",
    generation_config={"temperature": T, "max_output_tokens": 4096}
)
```

### Local (Llama 3 / Mistral)
Use the model's chat template with system prompt and user message matching the above structure. Ensure BOS/EOS tokens are correctly applied per the model's tokenizer.

---

## Pre-Registration Integrity

- These templates are FIXED upon pre-registration.
- Any modification to system prompt text, delimiter format, or parameter values after pre-registration voids the protocol.
- If an API changes its interface, adapt the implementation but preserve the logical structure (system prompt + context block + task prompt).
