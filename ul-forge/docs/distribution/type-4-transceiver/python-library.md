# Type 4 — Python Library (`ul-forge-transceiver`)

> Python SDK for building and parsing UL-Wire messages.

---

## Package Info

```toml
# packages/transceiver-py/pyproject.toml
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[project]
name = "ul-forge-transceiver"
version = "0.1.0"
description = "UL-Wire protocol codec for Python"
license = "CC0-1.0"
requires-python = ">=3.10"
keywords = ["universal-language", "protocol", "wire-format", "codec"]
classifiers = [
    "Development Status :: 3 - Alpha",
    "License :: CC0 1.0 Universal (CC0 1.0) Public Domain Dedication",
    "Programming Language :: Python :: 3",
    "Typing :: Typed",
]

dependencies = []

[project.optional-dependencies]
msgpack = ["msgpack>=1.0"]
dev = ["pytest>=8.0", "msgpack>=1.0"]
```

Zero external dependencies for JSON mode — only stdlib. MessagePack support via optional `msgpack` extra.

---

## File Layout

```
packages/transceiver-py/
├── src/
│   └── ul_forge_transceiver/
│       ├── __init__.py       # Public API
│       ├── types.py          # Dataclasses + enums
│       ├── builder.py        # MessageBuilder
│       ├── codec.py          # JSON encode/decode
│       └── py.typed          # PEP 561 marker
├── tests/
│   ├── test_codec.py
│   ├── test_builder.py
│   └── test_roundtrip.py
├── pyproject.toml
└── README.md
```

---

## Types

### `types.py`

```python
from __future__ import annotations

import enum
from dataclasses import dataclass, field
from typing import Any


class Intent(str, enum.Enum):
    ASSERT = "assert"
    QUERY = "query"
    PROPOSE = "propose"
    REFINE = "refine"
    VALIDATE_REQUEST = "validate_request"
    VALIDATE_RESPONSE = "validate_response"
    CAPABILITY = "capability"
    ACK = "ack"


class MatchMode(str, enum.Enum):
    EXACT = "exact"
    STRUCTURAL = "structural"
    SEMANTIC = "semantic"


class AckStatus(str, enum.Enum):
    RECEIVED = "received"
    ACCEPTED = "accepted"
    REJECTED = "rejected"
    NO_MATCH = "no_match"
    ERROR = "error"


@dataclass
class Change:
    type: str
    node_id: str | None = None
    edge_id: str | None = None
    field_name: str | None = None  # 'field' in JSON, renamed to avoid builtin
    old: Any = None
    new: Any = None

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {"type": self.type}
        if self.node_id is not None:
            d["node_id"] = self.node_id
        if self.edge_id is not None:
            d["edge_id"] = self.edge_id
        if self.field_name is not None:
            d["field"] = self.field_name
        if self.old is not None:
            d["old"] = self.old
        if self.new is not None:
            d["new"] = self.new
        return d


@dataclass
class CapabilityEntry:
    name: str
    description: str
    input: str
    output: str
    options: dict[str, Any] = field(default_factory=dict)


@dataclass
class ULMessage:
    protocol: str
    id: str
    timestamp: str
    intent: Intent
    payload: dict[str, Any] = field(default_factory=dict)
    metadata: dict[str, Any] = field(default_factory=dict)
    reply_to: str | None = None

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {
            "protocol": self.protocol,
            "id": self.id,
            "timestamp": self.timestamp,
            "intent": self.intent.value,
            "payload": self.payload,
            "metadata": self.metadata,
        }
        if self.reply_to is not None:
            d["reply_to"] = self.reply_to
        return d
```

---

## Builder

### `builder.py`

```python
from __future__ import annotations

import uuid
from datetime import datetime, timezone
from typing import Any

from .types import (
    AckStatus, CapabilityEntry, Change, Intent, MatchMode, ULMessage,
)

PROTOCOL = "UL-Wire/0.1"


class MessageBuilder:
    """Fluent builder for UL-Wire messages."""

    def __init__(self, intent: Intent, payload: dict[str, Any]) -> None:
        self._intent = intent
        self._payload = payload
        self._reply_to: str | None = None
        self._metadata: dict[str, Any] = {}

    # -- Factory methods --

    @classmethod
    def assert_gir(cls, gir: Any) -> MessageBuilder:
        return cls(Intent.ASSERT, {"gir": gir})

    @classmethod
    def query(cls, gir: Any, match_mode: MatchMode = MatchMode.EXACT) -> MessageBuilder:
        return cls(Intent.QUERY, {"gir": gir, "match_mode": match_mode.value})

    @classmethod
    def propose(cls, gir: Any, confidence: float = 1.0) -> MessageBuilder:
        return cls(Intent.PROPOSE, {"gir": gir, "confidence": confidence})

    @classmethod
    def refine(cls, gir: Any, changes: list[Change] | None = None) -> MessageBuilder:
        return cls(Intent.REFINE, {
            "gir": gir,
            "changes": [c.to_dict() for c in (changes or [])],
        })

    @classmethod
    def validate_request(cls, gir: Any, checks: list[str] | None = None) -> MessageBuilder:
        return cls(Intent.VALIDATE_REQUEST, {"gir": gir, "checks": checks or []})

    @classmethod
    def validate_response(
        cls,
        valid: bool,
        errors: list[str],
        warnings: list[str],
        checks_run: list[str],
    ) -> MessageBuilder:
        return cls(Intent.VALIDATE_RESPONSE, {
            "valid": valid,
            "errors": errors,
            "warnings": warnings,
            "checks_run": checks_run,
        })

    @classmethod
    def capability(cls, capabilities: list[CapabilityEntry]) -> MessageBuilder:
        return cls(Intent.CAPABILITY, {
            "capabilities": [
                {"name": c.name, "description": c.description,
                 "input": c.input, "output": c.output, "options": c.options}
                for c in capabilities
            ],
            "protocol_versions": [PROTOCOL],
            "encoding": ["json"],
        })

    @classmethod
    def ack(cls, status: AckStatus, message: str | None = None) -> MessageBuilder:
        payload: dict[str, Any] = {"status": status.value}
        if message is not None:
            payload["message"] = message
        return cls(Intent.ACK, payload)

    # -- Chainable setters --

    def in_reply_to(self, msg_id: str) -> MessageBuilder:
        self._reply_to = msg_id
        return self

    def meta(self, key: str, value: Any) -> MessageBuilder:
        self._metadata[key] = value
        return self

    def domain(self, domain: str) -> MessageBuilder:
        return self.meta("domain", domain)

    # -- Build --

    def build(self) -> ULMessage:
        return ULMessage(
            protocol=PROTOCOL,
            id=str(uuid.uuid4()),
            timestamp=datetime.now(timezone.utc).isoformat(),
            intent=self._intent,
            payload=self._payload,
            metadata=self._metadata,
            reply_to=self._reply_to,
        )
```

### Usage

```python
from ul_forge_transceiver import MessageBuilder, encode, decode

gir = {"nodes": [{"id": "n1", "type": "point", "label": "existence"}], "edges": [], "root": "n1"}
msg = MessageBuilder.assert_gir(gir).domain("physics").build()

json_str = encode(msg)       # str
back = decode(json_str)      # ULMessage
```

---

## Codec

### `codec.py`

```python
from __future__ import annotations

import json
from typing import Any

from .types import AckStatus, Intent, ULMessage

_VALID_INTENTS = {i.value for i in Intent}


class TransceiverError(Exception):
    pass


def encode(msg: ULMessage) -> str:
    """Encode a ULMessage to a JSON string."""
    return json.dumps(msg.to_dict(), separators=(",", ":"))


def encode_pretty(msg: ULMessage) -> str:
    """Encode a ULMessage to a pretty-printed JSON string."""
    return json.dumps(msg.to_dict(), indent=2)


def decode(text: str) -> ULMessage:
    """Decode a JSON string into a ULMessage with validation."""
    try:
        data = json.loads(text)
    except json.JSONDecodeError as e:
        raise TransceiverError(f"Invalid JSON: {e}") from e

    if not isinstance(data, dict):
        raise TransceiverError("Message must be a JSON object")

    # Validate protocol
    protocol = data.get("protocol", "")
    if not isinstance(protocol, str) or not protocol.startswith("UL-Wire/"):
        raise TransceiverError(f"Unsupported protocol: {protocol!r}")

    # Validate required fields
    for field in ("id", "timestamp", "intent"):
        if field not in data:
            raise TransceiverError(f"Missing required field: {field}")

    intent_str = data["intent"]
    if intent_str not in _VALID_INTENTS:
        raise TransceiverError(f"Unknown intent: {intent_str!r}")

    return ULMessage(
        protocol=data["protocol"],
        id=data["id"],
        timestamp=data["timestamp"],
        intent=Intent(intent_str),
        payload=data.get("payload", {}),
        metadata=data.get("metadata", {}),
        reply_to=data.get("reply_to"),
    )


def encode_msgpack(msg: ULMessage) -> bytes:
    """Encode to MessagePack (requires msgpack extra)."""
    try:
        import msgpack
    except ImportError as e:
        raise TransceiverError(
            "msgpack not installed. Install with: pip install ul-forge-transceiver[msgpack]"
        ) from e
    return msgpack.packb(msg.to_dict(), use_bin_type=True)


def decode_msgpack(data: bytes) -> ULMessage:
    """Decode from MessagePack (requires msgpack extra)."""
    try:
        import msgpack
    except ImportError as e:
        raise TransceiverError(
            "msgpack not installed. Install with: pip install ul-forge-transceiver[msgpack]"
        ) from e
    unpacked = msgpack.unpackb(data, raw=False)
    return decode(json.dumps(unpacked))  # reuse JSON validation path
```

---

## `__init__.py`

```python
"""UL-Wire protocol codec for Python."""

from .types import (
    AckStatus,
    CapabilityEntry,
    Change,
    Intent,
    MatchMode,
    ULMessage,
)
from .builder import MessageBuilder
from .codec import (
    TransceiverError,
    decode,
    decode_msgpack,
    encode,
    encode_msgpack,
    encode_pretty,
)

__all__ = [
    "AckStatus",
    "CapabilityEntry",
    "Change",
    "Intent",
    "MatchMode",
    "MessageBuilder",
    "TransceiverError",
    "ULMessage",
    "decode",
    "decode_msgpack",
    "encode",
    "encode_msgpack",
    "encode_pretty",
]
```

---

## Tests

### `tests/test_roundtrip.py`

```python
import pytest
from ul_forge_transceiver import (
    AckStatus, Intent, MessageBuilder, TransceiverError, decode, encode,
)


def _sample_gir():
    return {
        "nodes": [{"id": "n1", "type": "point", "label": "existence"}],
        "edges": [],
        "root": "n1",
    }


class TestRoundtrip:
    def test_assert_roundtrip(self):
        msg = MessageBuilder.assert_gir(_sample_gir()).domain("test").build()
        json_str = encode(msg)
        decoded = decode(json_str)

        assert decoded.id == msg.id
        assert decoded.intent == Intent.ASSERT
        assert decoded.metadata["domain"] == "test"
        assert decoded.payload["gir"] == _sample_gir()

    def test_ack_with_reply_to(self):
        msg = (
            MessageBuilder.ack(AckStatus.ACCEPTED, "looks good")
            .in_reply_to("550e8400-e29b-41d4-a716-446655440000")
            .build()
        )
        decoded = decode(encode(msg))

        assert decoded.intent == Intent.ACK
        assert decoded.reply_to == "550e8400-e29b-41d4-a716-446655440000"
        assert decoded.payload["status"] == "accepted"
        assert decoded.payload["message"] == "looks good"

    def test_rejects_unknown_protocol(self):
        bad = '{"protocol":"NOPE/1.0","id":"x","timestamp":"x","intent":"ack"}'
        with pytest.raises(TransceiverError, match="Unsupported protocol"):
            decode(bad)

    def test_rejects_unknown_intent(self):
        bad = '{"protocol":"UL-Wire/0.1","id":"x","timestamp":"x","intent":"explode"}'
        with pytest.raises(TransceiverError, match="Unknown intent"):
            decode(bad)

    def test_rejects_invalid_json(self):
        with pytest.raises(TransceiverError, match="Invalid JSON"):
            decode("not json at all")

    def test_propose_with_confidence(self):
        msg = MessageBuilder.propose(_sample_gir(), confidence=0.75).build()
        decoded = decode(encode(msg))

        assert decoded.intent == Intent.PROPOSE
        assert decoded.payload["confidence"] == 0.75
```

---

## CI

```yaml
  test-transceiver-py:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python: ["3.10", "3.11", "3.12", "3.13"]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: ${{ matrix.python }}
      - working-directory: packages/transceiver-py
        run: |
          pip install -e ".[dev]"
          pytest tests/ -v

  publish-transceiver-py:
    needs: [test-transceiver-py]
    if: startsWith(github.ref, 'refs/tags/transceiver-py-v')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with: { python-version: '3.12' }
      - working-directory: packages/transceiver-py
        run: |
          pip install build twine
          python -m build
          twine upload dist/*
        env:
          TWINE_USERNAME: __token__
          TWINE_PASSWORD: ${{ secrets.PYPI_TOKEN }}
```

---

## Cross-Language Interop Test

A key success criterion is that messages created in one language can be decoded in another. The CI pipeline should include a cross-language roundtrip test:

```bash
# Generate a message in Python, decode in Rust
python -c "
from ul_forge_transceiver import MessageBuilder, encode
gir = {'nodes': [{'id': 'n1', 'type': 'point', 'label': 'existence'}], 'edges': [], 'root': 'n1'}
msg = MessageBuilder.assert_gir(gir).domain('interop-test').build()
print(encode(msg))
" | cargo run --bin ul-transceiver-validate

# Generate in Rust, decode in TypeScript
cargo run --bin ul-transceiver-emit -- assert '{"nodes":[]}' | npx tsx -e "
import { decode } from '@ul-forge/transceiver';
import { createInterface } from 'readline';
const rl = createInterface({ input: process.stdin });
rl.on('line', (line) => {
  const msg = decode(line);
  console.log('OK:', msg.intent, msg.id);
});
"
```

This can be formalized as a CI step that runs after all three packages are built.
