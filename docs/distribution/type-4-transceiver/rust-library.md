# Type 4 — Rust Library (`ul-transceiver`)

> Rust crate for building, encoding, decoding, and transporting UL-Wire messages.

---

## Crate Info

```toml
# crates/ul-transceiver/Cargo.toml
[package]
name = "ul-transceiver"
version = "0.1.0"
edition = "2021"
license = "CC0-1.0"
description = "UL-Wire protocol codec and message builder"

[dependencies]
ul-core = { path = "../ul-core" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2"

[features]
default = ["json"]
json = []
msgpack = ["dep:rmp-serde"]

[dependencies.rmp-serde]
version = "1"
optional = true
```

---

## Module Layout

```
crates/ul-transceiver/src/
├── lib.rs              # Public API re-exports
├── message.rs          # ULMessage struct + Intent enum
├── builder.rs          # Fluent message builder
├── codec.rs            # JSON + MessagePack encode/decode
├── error.rs            # TransceiverError type
└── transport/
    ├── mod.rs          # Transport trait
    └── stdio.rs        # stdin/stdout line-delimited JSON
```

---

## Core Types

### `message.rs`

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ul_core::Gir;
use uuid::Uuid;

/// The 8 meaning-exchange intents.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(snake_case)]
pub enum Intent {
    Assert,
    Query,
    Propose,
    Refine,
    ValidateRequest,
    ValidateResponse,
    Capability,
    Ack,
}

/// Top-level wire message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ULMessage {
    pub protocol: String,
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub intent: Intent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Uuid>,
    #[serde(default)]
    pub payload: Payload,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Intent-specific payload variants.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum Payload {
    #[default]
    Empty {},
    Gir {
        gir: Gir,
    },
    GirWithConfidence {
        gir: Gir,
        confidence: f64,
    },
    Query {
        gir: Gir,
        match_mode: MatchMode,
    },
    Refine {
        gir: Gir,
        #[serde(default)]
        changes: Vec<Change>,
    },
    ValidationRequest {
        gir: Gir,
        #[serde(default)]
        checks: Vec<String>,
    },
    ValidationResponse {
        valid: bool,
        errors: Vec<String>,
        warnings: Vec<String>,
        checks_run: Vec<String>,
    },
    Capability {
        capabilities: Vec<CapabilityEntry>,
        protocol_versions: Vec<String>,
        #[serde(default)]
        encoding: Vec<String>,
    },
    Ack {
        status: AckStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(snake_case)]
pub enum MatchMode {
    Exact,
    Structural,
    Semantic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    #[serde(rename = "type")]
    pub change_type: String,
    pub node_id: Option<String>,
    pub edge_id: Option<String>,
    pub field: Option<String>,
    pub old: Option<serde_json::Value>,
    pub new: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityEntry {
    pub name: String,
    pub description: String,
    pub input: String,
    pub output: String,
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(snake_case)]
pub enum AckStatus {
    Received,
    Accepted,
    Rejected,
    NoMatch,
    Error,
}
```

---

## Builder

### `builder.rs`

Fluent API for constructing messages without manual UUID/timestamp handling:

```rust
use crate::message::*;
use chrono::Utc;
use ul_core::Gir;
use uuid::Uuid;

pub struct MessageBuilder {
    intent: Intent,
    reply_to: Option<Uuid>,
    payload: Payload,
    metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl MessageBuilder {
    /// Start building an assert message.
    pub fn assert(gir: Gir) -> Self {
        Self {
            intent: Intent::Assert,
            reply_to: None,
            payload: Payload::Gir { gir },
            metadata: Default::default(),
        }
    }

    /// Start building a query message.
    pub fn query(gir: Gir, match_mode: MatchMode) -> Self {
        Self {
            intent: Intent::Query,
            reply_to: None,
            payload: Payload::Query { gir, match_mode },
            metadata: Default::default(),
        }
    }

    /// Start building a propose message.
    pub fn propose(gir: Gir, confidence: f64) -> Self {
        Self {
            intent: Intent::Propose,
            reply_to: None,
            payload: Payload::GirWithConfidence { gir, confidence },
            metadata: Default::default(),
        }
    }

    /// Start building a refine message.
    pub fn refine(gir: Gir) -> Self {
        Self {
            intent: Intent::Refine,
            reply_to: None,
            payload: Payload::Refine { gir, changes: vec![] },
            metadata: Default::default(),
        }
    }

    /// Start building an ack.
    pub fn ack(status: AckStatus) -> Self {
        Self {
            intent: Intent::Ack,
            reply_to: None,
            payload: Payload::Ack { status, message: None },
            metadata: Default::default(),
        }
    }

    /// Set the reply_to field (mandatory for refine, validate_response, ack).
    pub fn in_reply_to(mut self, id: Uuid) -> Self {
        self.reply_to = Some(id);
        self
    }

    /// Add a metadata entry.
    pub fn meta(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Set domain metadata shorthand.
    pub fn domain(self, domain: &str) -> Self {
        self.meta("domain", domain)
    }

    /// Build the final message.
    pub fn build(self) -> ULMessage {
        ULMessage {
            protocol: "UL-Wire/0.1".to_string(),
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            intent: self.intent,
            reply_to: self.reply_to,
            payload: self.payload,
            metadata: self.metadata,
        }
    }
}
```

### Usage

```rust
use ul_transceiver::{MessageBuilder, MatchMode};

let gir = ul_core::parse("point(existence)")?;
let msg = MessageBuilder::assert(gir)
    .domain("ontology")
    .build();

let json = ul_transceiver::to_json(&msg)?;
```

---

## Codec

### `codec.rs`

```rust
use crate::{error::TransceiverError, message::ULMessage};

/// Encode a message to JSON bytes.
pub fn to_json(msg: &ULMessage) -> Result<Vec<u8>, TransceiverError> {
    serde_json::to_vec(msg).map_err(TransceiverError::Encode)
}

/// Encode a message to pretty-printed JSON.
pub fn to_json_pretty(msg: &ULMessage) -> Result<String, TransceiverError> {
    serde_json::to_string_pretty(msg).map_err(TransceiverError::Encode)
}

/// Decode a message from JSON bytes.
pub fn from_json(bytes: &[u8]) -> Result<ULMessage, TransceiverError> {
    let msg: ULMessage = serde_json::from_slice(bytes).map_err(TransceiverError::Decode)?;
    validate_protocol(&msg)?;
    Ok(msg)
}

/// Encode to MessagePack (requires `msgpack` feature).
#[cfg(feature = "msgpack")]
pub fn to_msgpack(msg: &ULMessage) -> Result<Vec<u8>, TransceiverError> {
    rmp_serde::to_vec(msg).map_err(TransceiverError::MsgpackEncode)
}

/// Decode from MessagePack (requires `msgpack` feature).
#[cfg(feature = "msgpack")]
pub fn from_msgpack(bytes: &[u8]) -> Result<ULMessage, TransceiverError> {
    let msg: ULMessage = rmp_serde::from_slice(bytes).map_err(TransceiverError::MsgpackDecode)?;
    validate_protocol(&msg)?;
    Ok(msg)
}

fn validate_protocol(msg: &ULMessage) -> Result<(), TransceiverError> {
    if !msg.protocol.starts_with("UL-Wire/") {
        return Err(TransceiverError::UnsupportedProtocol(msg.protocol.clone()));
    }
    // Parse version  
    let version = msg.protocol.strip_prefix("UL-Wire/")
        .ok_or_else(|| TransceiverError::UnsupportedProtocol(msg.protocol.clone()))?;
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 2 {
        return Err(TransceiverError::UnsupportedProtocol(msg.protocol.clone()));
    }
    let major: u32 = parts[0].parse()
        .map_err(|_| TransceiverError::UnsupportedProtocol(msg.protocol.clone()))?;
    if major != 0 {
        return Err(TransceiverError::UnsupportedProtocol(msg.protocol.clone()));
    }
    Ok(())
}
```

---

## Error Type

### `error.rs`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransceiverError {
    #[error("JSON encode error: {0}")]
    Encode(#[source] serde_json::Error),

    #[error("JSON decode error: {0}")]
    Decode(#[source] serde_json::Error),

    #[cfg(feature = "msgpack")]
    #[error("MessagePack encode error: {0}")]
    MsgpackEncode(#[source] rmp_serde::encode::Error),

    #[cfg(feature = "msgpack")]
    #[error("MessagePack decode error: {0}")]
    MsgpackDecode(#[source] rmp_serde::decode::Error),

    #[error("Unsupported protocol version: {0}")]
    UnsupportedProtocol(String),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
```

---

## Transport Trait

### `transport/mod.rs`

```rust
use crate::{error::TransceiverError, message::ULMessage};

/// A bidirectional message transport.
pub trait Transport {
    /// Send a message.
    fn send(&mut self, msg: &ULMessage) -> Result<(), TransceiverError>;

    /// Receive the next message (blocking).
    fn recv(&mut self) -> Result<ULMessage, TransceiverError>;
}
```

### `transport/stdio.rs`

```rust
use std::io::{self, BufRead, Write};
use crate::{codec, error::TransceiverError, message::ULMessage, transport::Transport};

/// Line-delimited JSON over stdin/stdout.
pub struct StdioTransport {
    reader: io::BufReader<io::Stdin>,
    writer: io::Stdout,
}

impl StdioTransport {
    pub fn new() -> Self {
        Self {
            reader: io::BufReader::new(io::stdin()),
            writer: io::stdout(),
        }
    }
}

impl Transport for StdioTransport {
    fn send(&mut self, msg: &ULMessage) -> Result<(), TransceiverError> {
        let json = codec::to_json(msg)?;
        self.writer.write_all(&json)?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;
        Ok(())
    }

    fn recv(&mut self) -> Result<ULMessage, TransceiverError> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        if line.is_empty() {
            return Err(TransceiverError::Transport("stdin closed".into()));
        }
        codec::from_json(line.trim_end().as_bytes())
    }
}
```

---

## `lib.rs`

```rust
pub mod message;
pub mod builder;
pub mod codec;
pub mod error;
pub mod transport;

pub use message::*;
pub use builder::MessageBuilder;
pub use codec::{to_json, to_json_pretty, from_json};
pub use error::TransceiverError;
pub use transport::Transport;

#[cfg(feature = "msgpack")]
pub use codec::{to_msgpack, from_msgpack};
```

---

## Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_json() {
        let gir = ul_core::parse("point(existence)").unwrap();
        let msg = MessageBuilder::assert(gir).domain("test").build();

        let bytes = to_json(&msg).unwrap();
        let decoded = from_json(&bytes).unwrap();

        assert_eq!(msg.id, decoded.id);
        assert_eq!(msg.intent, decoded.intent);
        assert_eq!(msg.protocol, decoded.protocol);
    }

    #[test]
    fn rejects_unknown_protocol() {
        let json = r#"{"protocol":"UNKNOWN/1.0","id":"550e8400-e29b-41d4-a716-446655440000","timestamp":"2025-01-01T00:00:00Z","intent":"ack"}"#;
        let result = from_json(json.as_bytes());
        assert!(result.is_err());
    }

    #[cfg(feature = "msgpack")]
    #[test]
    fn roundtrip_msgpack() {
        let gir = ul_core::parse("point(existence)").unwrap();
        let msg = MessageBuilder::assert(gir).build();

        let bytes = to_msgpack(&msg).unwrap();
        let decoded = from_msgpack(&bytes).unwrap();

        assert_eq!(msg.id, decoded.id);
    }

    #[test]
    fn builder_ack() {
        let original_id = uuid::Uuid::new_v4();
        let msg = MessageBuilder::ack(AckStatus::Accepted)
            .in_reply_to(original_id)
            .build();

        assert_eq!(msg.intent, Intent::Ack);
        assert_eq!(msg.reply_to, Some(original_id));
    }
}
```
