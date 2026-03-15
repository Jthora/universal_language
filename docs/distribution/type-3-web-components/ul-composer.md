# Type 3 — `<ul-composer>` Web Component

> Interactive UL-Script editor with live preview, validation, and export.

---

## API

```html
<ul-composer
  width="500"
  height="350"
  theme="light"
  placeholder="Type UL-Script here..."
></ul-composer>
```

### Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `width` | number | `500` | Component width in pixels |
| `height` | number | `350` | Component height in pixels |
| `theme` | `"light"` \| `"dark"` | `"light"` | Color scheme |
| `placeholder` | string | `"Type UL-Script..."` | Placeholder text for input |
| `initial-script` | string | `""` | Pre-filled UL-Script text |
| `show-palette` | boolean | `true` | Show primitive insertion buttons |
| `show-export` | boolean | `true` | Show export button |
| `show-validation` | boolean | `true` | Show validation indicator |

---

## Layout

```
┌──────────────────────────────────────────┐
│  ┌─ Palette ──────────────────────────┐  │
│  │ ● Point  ── Line  ∠ Angle         │  │
│  │ ⌒ Curve  ◯ Enclosure              │  │
│  └────────────────────────────────────┘  │
│                                           │
│  ┌─ Input ─────────────────────────────┐ │
│  │ enclosure(circle, point(existence)) │ │
│  │                                     │ │
│  └─────────────────────────────────────┘ │
│                                           │
│  ┌─ Preview ──────┐  ┌─ Validation ───┐ │
│  │                 │  │ ✓ Valid         │ │
│  │     [SVG]       │  │ 3 nodes        │ │
│  │                 │  │ 2 edges        │ │
│  └─────────────────┘  │                │ │
│                        │ [Export SVG]   │ │
│                        └────────────────┘ │
└──────────────────────────────────────────┘
```

---

## Behavior

### Input → Preview Pipeline

1. User types in the textarea (or clicks a palette button)
2. Debounce 150ms
3. Parse UL-Script → GIR
4. Validate GIR → update validation indicator
5. If valid: Render GIR → SVG → inject into preview panel
6. If invalid: Show error message in preview, highlight validation indicator red

### Palette Buttons

Clicking a primitive button inserts a template at the cursor position:

| Button | Inserts |
|--------|---------|
| Point | `point(existence)` |
| Line | `line(directed)` |
| Angle | `angle(90)` |
| Curve | `curve(flow)` |
| Enclosure | `enclosure(circle, )` (cursor placed inside) |

### Validation Indicator

| State | Color | Text |
|-------|-------|------|
| Valid | Green ● | "Valid — N nodes, M edges" |
| Warnings | Yellow ● | "Valid with warnings" + warning list |
| Invalid | Red ● | Error message |
| Empty | Gray ● | "Enter UL-Script" |

### Export

The export button downloads the current SVG as a file:

```typescript
exportSvg() {
  const svg = this.currentSvg;
  if (!svg) return;
  const blob = new Blob([svg], { type: 'image/svg+xml' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'glyph.svg';
  a.click();
  URL.revokeObjectURL(url);
}
```

---

## Events

| Event | Detail | When |
|-------|--------|------|
| `ul-change` | `{ script: string, gir: object \| null }` | Input changes (debounced) |
| `ul-validate` | `{ valid: boolean, errors: string[], warnings: string[] }` | After validation runs |
| `ul-render` | `{ svg: string }` | After successful render |
| `ul-error` | `{ message: string }` | On parse/render error |

```javascript
document.querySelector('ul-composer').addEventListener('ul-change', (e) => {
  console.log('Script changed:', e.detail.script);
  console.log('GIR:', e.detail.gir);
});
```

---

## Implementation Sketch

```typescript
import { ensureWasmLoaded, getWasm } from './wasm-loader';
import { composerStyles } from './styles';

export class ULComposer extends HTMLElement {
  static observedAttributes = ['width', 'height', 'theme', 'placeholder', 'initial-script',
                                'show-palette', 'show-export', 'show-validation'];

  private shadow: ShadowRoot;
  private textarea: HTMLTextAreaElement;
  private preview: HTMLDivElement;
  private validation: HTMLDivElement;
  private renderTimeout: number | null = null;
  private currentSvg: string | null = null;

  constructor() {
    super();
    this.shadow = this.attachShadow({ mode: 'open' });
  }

  async connectedCallback() {
    this.buildDOM();
    await ensureWasmLoaded();
    
    // If initial-script is set, trigger render
    const initial = this.getAttribute('initial-script');
    if (initial) {
      this.textarea.value = initial;
      this.onInput();
    }
  }

  private buildDOM() {
    const showPalette = this.getAttribute('show-palette') !== 'false';
    const showExport = this.getAttribute('show-export') !== 'false';
    const showValidation = this.getAttribute('show-validation') !== 'false';
    const placeholder = this.getAttribute('placeholder') || 'Type UL-Script...';
    const width = this.getAttribute('width') || '500';
    const height = this.getAttribute('height') || '350';
    const theme = this.getAttribute('theme') || 'light';

    this.shadow.innerHTML = `
      <style>${composerStyles}</style>
      <div class="ul-composer" data-theme="${theme}" style="width:${width}px;height:${height}px">
        ${showPalette ? `
        <div class="ul-palette">
          <button data-insert="point(existence)" title="Point">●</button>
          <button data-insert="line(directed)" title="Line">──</button>
          <button data-insert="angle(90)" title="Angle">∠</button>
          <button data-insert="curve(flow)" title="Curve">⌒</button>
          <button data-insert="enclosure(circle, )" title="Enclosure">◯</button>
        </div>
        ` : ''}
        <textarea class="ul-input" placeholder="${placeholder}" spellcheck="false"></textarea>
        <div class="ul-lower">
          <div class="ul-preview"></div>
          ${showValidation ? '<div class="ul-validation"><span class="ul-status">●</span> Enter UL-Script</div>' : ''}
          ${showExport ? '<button class="ul-export" disabled>Export SVG</button>' : ''}
        </div>
      </div>
    `;

    this.textarea = this.shadow.querySelector('.ul-input')!;
    this.preview = this.shadow.querySelector('.ul-preview')!;
    this.validation = this.shadow.querySelector('.ul-validation')!;

    // Event listeners
    this.textarea.addEventListener('input', () => this.scheduleRender());

    // Palette buttons
    this.shadow.querySelectorAll('.ul-palette button').forEach(btn => {
      btn.addEventListener('click', () => {
        const insert = (btn as HTMLButtonElement).dataset.insert!;
        this.insertAtCursor(insert);
      });
    });

    // Export button
    const exportBtn = this.shadow.querySelector('.ul-export');
    if (exportBtn) {
      exportBtn.addEventListener('click', () => this.exportSvg());
    }
  }

  private scheduleRender() {
    if (this.renderTimeout) clearTimeout(this.renderTimeout);
    this.renderTimeout = window.setTimeout(() => this.onInput(), 150);
  }

  private onInput() {
    const script = this.textarea.value;
    
    if (!script.trim()) {
      this.preview.innerHTML = '';
      this.setValidation('empty', 'Enter UL-Script');
      this.currentSvg = null;
      return;
    }

    try {
      const wasm = getWasm();
      const gir = wasm.parse(script);
      const result = wasm.validate(gir);

      if (result.valid) {
        const svg = wasm.render(gir, { format: 'svg', width: 200, height: 200, embed_gir: false });
        this.preview.innerHTML = svg;
        this.currentSvg = svg;
        
        const nodeCount = gir.nodes?.length ?? 0;
        const edgeCount = gir.edges?.length ?? 0;
        const status = result.warnings.length > 0 ? 'warning' : 'valid';
        this.setValidation(status, `Valid — ${nodeCount} nodes, ${edgeCount} edges`);
      } else {
        this.setValidation('error', result.errors[0] || 'Validation error');
      }

      this.dispatchEvent(new CustomEvent('ul-change', { detail: { script, gir } }));
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : 'Parse error';
      this.setValidation('error', msg);
      this.preview.innerHTML = '';
      this.currentSvg = null;
      this.dispatchEvent(new CustomEvent('ul-error', { detail: { message: msg } }));
    }
  }

  private setValidation(state: 'valid' | 'warning' | 'error' | 'empty', text: string) {
    if (!this.validation) return;
    this.validation.innerHTML = `<span class="ul-status ul-status-${state}">●</span> ${this.escapeHtml(text)}`;
    
    const exportBtn = this.shadow.querySelector('.ul-export') as HTMLButtonElement | null;
    if (exportBtn) exportBtn.disabled = state !== 'valid' && state !== 'warning';
  }

  private escapeHtml(text: string): string {
    const el = document.createElement('span');
    el.textContent = text;
    return el.innerHTML;
  }

  private insertAtCursor(text: string) {
    const start = this.textarea.selectionStart;
    const end = this.textarea.selectionEnd;
    const before = this.textarea.value.substring(0, start);
    const after = this.textarea.value.substring(end);
    this.textarea.value = before + text + after;
    this.textarea.selectionStart = this.textarea.selectionEnd = start + text.length;
    this.textarea.focus();
    this.scheduleRender();
  }

  private exportSvg() { /* see above */ }
}

customElements.define('ul-composer', ULComposer);
```
