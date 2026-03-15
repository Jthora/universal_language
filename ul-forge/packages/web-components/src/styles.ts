/** Encapsulated Shadow DOM styles for UL web components. */

export const baseStyles = `
  :host {
    display: inline-block;
    contain: content;
    font-family: system-ui, -apple-system, sans-serif;
  }

  .container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  .container svg {
    max-width: 100%;
    max-height: 100%;
  }

  .error {
    color: #c33;
    font-size: 12px;
    padding: 8px;
    text-align: center;
    word-break: break-word;
  }

  .loading {
    color: #999;
    font-size: 12px;
    text-align: center;
  }
`;

export const composerStyles = `
  ${baseStyles}

  :host {
    display: block;
  }

  .composer {
    display: flex;
    flex-direction: column;
    gap: 8px;
    height: 100%;
  }

  .input-area {
    display: flex;
    gap: 8px;
    flex-direction: column;
  }

  .script-input {
    width: 100%;
    min-height: 40px;
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-family: 'SF Mono', Monaco, 'Cascadia Code', monospace;
    font-size: 14px;
    resize: vertical;
    box-sizing: border-box;
    background: #fafafa;
    color: #333;
  }

  .script-input:focus {
    outline: none;
    border-color: #4a9eff;
    box-shadow: 0 0 0 2px rgba(74, 158, 255, 0.2);
  }

  .palette {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }

  .palette button {
    padding: 4px 10px;
    border: 1px solid #ddd;
    border-radius: 3px;
    background: #fff;
    cursor: pointer;
    font-size: 12px;
    color: #555;
    transition: background 0.15s;
  }

  .palette button:hover {
    background: #e8f0fe;
    border-color: #4a9eff;
  }

  .preview {
    flex: 1;
    min-height: 100px;
    border: 1px solid #eee;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    background: #fff;
  }

  .status {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: #888;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .status-dot.valid { background: #4caf50; }
  .status-dot.warning { background: #ff9800; }
  .status-dot.invalid { background: #f44336; }

  .toolbar {
    display: flex;
    gap: 4px;
    justify-content: flex-end;
  }

  .toolbar button {
    padding: 4px 12px;
    border: 1px solid #ddd;
    border-radius: 3px;
    background: #fff;
    cursor: pointer;
    font-size: 12px;
  }

  .toolbar button:hover {
    background: #f0f0f0;
  }
`;

export const dictionaryStyles = `
  ${baseStyles}

  :host {
    display: block;
  }

  .dictionary {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .search-input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 14px;
    box-sizing: border-box;
    background: #fafafa;
    color: #333;
  }

  .search-input:focus {
    outline: none;
    border-color: #4a9eff;
  }

  .entries-grid {
    display: grid;
    gap: 12px;
  }

  .entry-card {
    border: 1px solid #eee;
    border-radius: 6px;
    padding: 12px;
    display: flex;
    gap: 12px;
    align-items: flex-start;
    background: #fff;
    transition: box-shadow 0.15s;
  }

  .entry-card:hover {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  }

  .entry-preview {
    flex-shrink: 0;
    width: 64px;
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  .entry-preview svg {
    max-width: 100%;
    max-height: 100%;
  }

  .entry-info {
    flex: 1;
    min-width: 0;
  }

  .entry-name {
    font-weight: 600;
    font-size: 14px;
    color: #333;
    margin: 0 0 2px;
  }

  .entry-tier {
    font-size: 11px;
    color: #888;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .entry-sigma {
    font-family: 'SF Mono', Monaco, monospace;
    font-size: 12px;
    color: #555;
    margin-top: 4px;
  }

  .entry-labels {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
    margin-top: 4px;
  }

  .label-tag {
    font-size: 10px;
    padding: 1px 6px;
    border-radius: 10px;
    background: #f0f0f0;
    color: #666;
  }

  .empty-state {
    text-align: center;
    color: #999;
    padding: 24px;
    font-size: 13px;
  }
`;
