# Type 4 — TypeScript Library (`@ul-forge/transceiver`)

> TypeScript/JavaScript SDK for building and parsing UL-Wire messages.

---

## Package Info

```json
{
  "name": "@ul-forge/transceiver",
  "version": "0.1.0",
  "type": "module",
  "license": "CC0-1.0",
  "description": "UL-Wire protocol codec for TypeScript/JavaScript",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "import": "./dist/index.js",
      "types": "./dist/index.d.ts"
    }
  },
  "files": ["dist/", "README.md"],
  "keywords": ["universal-language", "protocol", "wire-format", "codec"],
  "scripts": {
    "build": "tsc",
    "test": "vitest run",
    "type-check": "tsc --noEmit"
  },
  "devDependencies": {
    "typescript": "^5.7.0",
    "vitest": "^3.0.0"
  },
  "dependencies": {
    "uuid": "^11.0.0"
  }
}
```

Zero WASM dependency — this package is pure TypeScript. It handles only message construction and serialization, not UL parsing/rendering.

---

## File Layout

```
packages/transceiver-ts/
├── src/
│   ├── index.ts          # Barrel exports
│   ├── types.ts          # ULMessage, Intent, Payload types
│   ├── builder.ts        # Fluent MessageBuilder
│   ├── codec.ts          # JSON encode/decode with validation
│   └── errors.ts         # TransceiverError
├── tests/
│   ├── codec.test.ts
│   ├── builder.test.ts
│   └── roundtrip.test.ts
├── package.json
└── tsconfig.json
```

---

## Types

### `types.ts`

```typescript
export type Intent =
  | 'assert'
  | 'query'
  | 'propose'
  | 'refine'
  | 'validate_request'
  | 'validate_response'
  | 'capability'
  | 'ack';

export type MatchMode = 'exact' | 'structural' | 'semantic';
export type AckStatus = 'received' | 'accepted' | 'rejected' | 'no_match' | 'error';

export interface ULMessage {
  protocol: string;
  id: string;
  timestamp: string;
  intent: Intent;
  reply_to?: string | null;
  payload: Payload;
  metadata: Record<string, unknown>;
}

export type Payload =
  | GirPayload
  | GirWithConfidencePayload
  | QueryPayload
  | RefinePayload
  | ValidationRequestPayload
  | ValidationResponsePayload
  | CapabilityPayload
  | AckPayload
  | Record<string, never>;

export interface GirPayload {
  gir: unknown; // GIR JSON structure — typed more precisely when @ul-forge/sdk is available
}

export interface GirWithConfidencePayload {
  gir: unknown;
  confidence: number;
}

export interface QueryPayload {
  gir: unknown;
  match_mode: MatchMode;
}

export interface RefinePayload {
  gir: unknown;
  changes?: Change[];
}

export interface Change {
  type: string;
  node_id?: string;
  edge_id?: string;
  field?: string;
  old?: unknown;
  new?: unknown;
}

export interface ValidationRequestPayload {
  gir: unknown;
  checks?: string[];
}

export interface ValidationResponsePayload {
  valid: boolean;
  errors: string[];
  warnings: string[];
  checks_run: string[];
}

export interface CapabilityEntry {
  name: string;
  description: string;
  input: string;
  output: string;
  options?: Record<string, unknown>;
}

export interface CapabilityPayload {
  capabilities: CapabilityEntry[];
  protocol_versions: string[];
  encoding?: string[];
}

export interface AckPayload {
  status: AckStatus;
  message?: string;
}
```

---

## Builder

### `builder.ts`

```typescript
import { v4 as uuidv4 } from 'uuid';
import type {
  ULMessage, Intent, Payload, MatchMode, AckStatus,
} from './types';

export class MessageBuilder {
  private intent: Intent;
  private replyTo: string | null = null;
  private payload: Payload;
  private meta: Record<string, unknown> = {};

  private constructor(intent: Intent, payload: Payload) {
    this.intent = intent;
    this.payload = payload;
  }

  static assert(gir: unknown): MessageBuilder {
    return new MessageBuilder('assert', { gir });
  }

  static query(gir: unknown, matchMode: MatchMode): MessageBuilder {
    return new MessageBuilder('query', { gir, match_mode: matchMode });
  }

  static propose(gir: unknown, confidence: number): MessageBuilder {
    return new MessageBuilder('propose', { gir, confidence });
  }

  static refine(gir: unknown, changes?: Change[]): MessageBuilder {
    return new MessageBuilder('refine', { gir, changes: changes ?? [] });
  }

  static validateRequest(gir: unknown, checks?: string[]): MessageBuilder {
    return new MessageBuilder('validate_request', { gir, checks: checks ?? [] });
  }

  static validateResponse(valid: boolean, errors: string[], warnings: string[], checksRun: string[]): MessageBuilder {
    return new MessageBuilder('validate_response', { valid, errors, warnings, checks_run: checksRun });
  }

  static capability(capabilities: CapabilityEntry[]): MessageBuilder {
    return new MessageBuilder('capability', {
      capabilities,
      protocol_versions: ['UL-Wire/0.1'],
      encoding: ['json'],
    });
  }

  static ack(status: AckStatus, message?: string): MessageBuilder {
    return new MessageBuilder('ack', { status, message });
  }

  inReplyTo(id: string): this {
    this.replyTo = id;
    return this;
  }

  metadata(key: string, value: unknown): this {
    this.meta[key] = value;
    return this;
  }

  domain(domain: string): this {
    return this.metadata('domain', domain);
  }

  build(): ULMessage {
    return {
      protocol: 'UL-Wire/0.1',
      id: uuidv4(),
      timestamp: new Date().toISOString(),
      intent: this.intent,
      reply_to: this.replyTo,
      payload: this.payload,
      metadata: this.meta,
    };
  }
}
```

### Usage

```typescript
import { MessageBuilder, encode, decode } from '@ul-forge/transceiver';

const msg = MessageBuilder.assert(girObject)
  .domain('physics')
  .build();

const json = encode(msg);      // string
const back = decode(json);      // ULMessage
```

---

## Codec

### `codec.ts`

```typescript
import type { ULMessage, Intent } from './types';

const VALID_INTENTS: Intent[] = [
  'assert', 'query', 'propose', 'refine',
  'validate_request', 'validate_response',
  'capability', 'ack',
];

export class TransceiverError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'TransceiverError';
  }
}

/** Encode a ULMessage to JSON string. */
export function encode(msg: ULMessage): string {
  return JSON.stringify(msg);
}

/** Encode a ULMessage to pretty-printed JSON string. */
export function encodePretty(msg: ULMessage): string {
  return JSON.stringify(msg, null, 2);
}

/** Decode a JSON string to ULMessage with validation. */
export function decode(json: string): ULMessage {
  let parsed: unknown;
  try {
    parsed = JSON.parse(json);
  } catch {
    throw new TransceiverError('Invalid JSON');
  }

  if (typeof parsed !== 'object' || parsed === null) {
    throw new TransceiverError('Message must be an object');
  }

  const msg = parsed as Record<string, unknown>;

  // Validate required fields
  if (typeof msg.protocol !== 'string' || !msg.protocol.startsWith('UL-Wire/')) {
    throw new TransceiverError(`Unsupported protocol: ${String(msg.protocol)}`);
  }
  if (typeof msg.id !== 'string') {
    throw new TransceiverError('Missing or invalid id');
  }
  if (typeof msg.timestamp !== 'string') {
    throw new TransceiverError('Missing or invalid timestamp');
  }
  if (!VALID_INTENTS.includes(msg.intent as Intent)) {
    throw new TransceiverError(`Unknown intent: ${String(msg.intent)}`);
  }

  return parsed as ULMessage;
}
```

---

## Tests

### `tests/roundtrip.test.ts`

```typescript
import { describe, it, expect } from 'vitest';
import { MessageBuilder, encode, decode } from '../src';

describe('roundtrip', () => {
  it('assert message survives JSON roundtrip', () => {
    const gir = { nodes: [{ id: 'n1', type: 'point', label: 'existence' }], edges: [], root: 'n1' };
    const msg = MessageBuilder.assert(gir).domain('test').build();

    const json = encode(msg);
    const decoded = decode(json);

    expect(decoded.id).toBe(msg.id);
    expect(decoded.intent).toBe('assert');
    expect(decoded.metadata.domain).toBe('test');
    expect((decoded.payload as { gir: unknown }).gir).toEqual(gir);
  });

  it('ack with reply_to', () => {
    const msg = MessageBuilder.ack('accepted', 'looks good')
      .inReplyTo('550e8400-e29b-41d4-a716-446655440000')
      .build();

    const decoded = decode(encode(msg));
    expect(decoded.intent).toBe('ack');
    expect(decoded.reply_to).toBe('550e8400-e29b-41d4-a716-446655440000');
  });

  it('rejects unknown protocol', () => {
    const bad = JSON.stringify({ protocol: 'NOPE/1.0', id: 'x', timestamp: 'x', intent: 'ack' });
    expect(() => decode(bad)).toThrow('Unsupported protocol');
  });

  it('rejects unknown intent', () => {
    const bad = JSON.stringify({ protocol: 'UL-Wire/0.1', id: 'x', timestamp: 'x', intent: 'explode' });
    expect(() => decode(bad)).toThrow('Unknown intent');
  });
});
```

---

## CI

```yaml
  test-transceiver-ts:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: '20' }
      - working-directory: packages/transceiver-ts
        run: npm ci && npm test

  publish-transceiver-ts:
    needs: [test-transceiver-ts]
    if: startsWith(github.ref, 'refs/tags/transceiver-ts-v')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: '20', registry-url: 'https://registry.npmjs.org' }
      - working-directory: packages/transceiver-ts
        run: npm ci && npm run build && npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```
