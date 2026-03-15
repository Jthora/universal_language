# Type 3 — `<ul-symbol>` Web Component

> Display a static rendered UL glyph from UL-Script text.

---

## API

```html
<ul-symbol
  script="enclosure(circle, point(existence))"
  width="200"
  height="200"
  theme="light"
></ul-symbol>
```

### Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `script` | string | `""` | UL-Script text to parse and render |
| `width` | number | `200` | SVG width in pixels |
| `height` | number | `200` | SVG height in pixels |
| `theme` | `"light"` \| `"dark"` | `"light"` | Color scheme |

All attributes are reactive — changing them re-renders the glyph.

---

## Behavior

1. On `connectedCallback`: attach Shadow DOM, show loading indicator, begin WASM load
2. Once WASM is ready: parse `script` attribute, render to SVG, inject into Shadow DOM
3. On attribute change: re-parse and re-render (debounced 50ms)
4. On parse error: show error message in the component (not thrown to page)
5. On empty `script`: show placeholder (faint UL logo or "Write UL-Script here")

---

## Implementation

```typescript
import { ensureWasmLoaded, getWasm } from './wasm-loader';
import { symbolStyles } from './styles';

export class ULSymbol extends HTMLElement {
  static observedAttributes = ['script', 'width', 'height', 'theme'];

  private shadow: ShadowRoot;
  private container: HTMLDivElement;
  private renderTimeout: number | null = null;

  constructor() {
    super();
    this.shadow = this.attachShadow({ mode: 'open' });
    this.shadow.innerHTML = `
      <style>${symbolStyles}</style>
      <div class="ul-symbol-container">
        <div class="ul-symbol-loading">Loading...</div>
      </div>
    `;
    this.container = this.shadow.querySelector('.ul-symbol-container')!;
  }

  async connectedCallback() {
    this.updateTheme();
    try {
      await ensureWasmLoaded();
      this.renderGlyph();
    } catch (e) {
      this.showError('Failed to load UL engine');
    }
  }

  attributeChangedCallback(name: string, oldValue: string | null, newValue: string | null) {
    if (oldValue === newValue) return;
    if (name === 'theme') {
      this.updateTheme();
    }
    this.scheduleRender();
  }

  private scheduleRender() {
    if (this.renderTimeout) clearTimeout(this.renderTimeout);
    this.renderTimeout = window.setTimeout(() => this.renderGlyph(), 50);
  }

  private renderGlyph() {
    const script = this.getAttribute('script') || '';
    const width = parseFloat(this.getAttribute('width') || '200');
    const height = parseFloat(this.getAttribute('height') || '200');

    if (!script.trim()) {
      this.container.innerHTML = '<div class="ul-symbol-placeholder">UL</div>';
      return;
    }

    try {
      const wasm = getWasm();
      const gir = wasm.parse(script);
      const svg = wasm.render(gir, { format: 'svg', width, height, embed_gir: false });
      this.container.innerHTML = svg;
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : 'Parse error';
      this.showError(msg);
    }
  }

  private showError(message: string) {
    // Sanitize message before injecting into DOM
    const el = document.createElement('div');
    el.className = 'ul-symbol-error';
    el.textContent = message;
    this.container.innerHTML = '';
    this.container.appendChild(el);
  }

  private updateTheme() {
    const theme = this.getAttribute('theme') || 'light';
    this.container.setAttribute('data-theme', theme);
  }
}

customElements.define('ul-symbol', ULSymbol);
```

---

## Styles

```css
:host {
  display: inline-block;
}

.ul-symbol-container {
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  border-radius: 4px;
}

.ul-symbol-container[data-theme="light"] {
  background: var(--ul-background, #ffffff);
  color: var(--ul-stroke-color, #333333);
}

.ul-symbol-container[data-theme="dark"] {
  background: var(--ul-background, #1a1a2e);
  color: var(--ul-stroke-color, #e0e0e0);
}

.ul-symbol-container svg {
  max-width: 100%;
  max-height: 100%;
}

.ul-symbol-loading {
  font-family: var(--ul-font-family, system-ui, sans-serif);
  font-size: 12px;
  opacity: 0.5;
}

.ul-symbol-placeholder {
  font-family: var(--ul-font-family, system-ui, sans-serif);
  font-size: 24px;
  opacity: 0.15;
  font-weight: bold;
}

.ul-symbol-error {
  font-family: var(--ul-font-family, system-ui, sans-serif);
  font-size: 11px;
  color: var(--ul-error-color, #cc0000);
  padding: 8px;
  word-break: break-word;
}
```

---

## JavaScript API

For programmatic control:

```javascript
const symbol = document.querySelector('ul-symbol');

// Change script dynamically
symbol.setAttribute('script', 'line(directed)');

// Read back rendered SVG
const svg = symbol.shadowRoot.querySelector('svg');
```

---

## Accessibility

- SVG output includes `role="img"` and `aria-label` derived from the script attribute
- Error messages use `role="alert"`
- Placeholder text is decorative (`aria-hidden="true"`)
