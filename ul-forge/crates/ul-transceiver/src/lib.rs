//! # UL Transceiver Protocol
//!
//! Rust library for encoding, decoding, and building UL Transceiver Protocol
//! messages — the wire format for AI-to-AI semantic communication using
//! Universal Language GIR structures.
//!
//! ## Quick Start
//!
//! ```rust
//! use ul_transceiver::{ULMessageBuilder, Intent, Codec};
//! use serde_json::json;
//!
//! // Build a message
//! let envelope = ULMessageBuilder::new("agent-alpha")
//!     .intent(Intent::Assert)
//!     .gir_payload(json!({"ul_gir": "1.0", "root": "n0", "nodes": [], "edges": []}))
//!     .domain("physics.thermodynamics")
//!     .build();
//!
//! // Encode to JSON
//! let bytes = Codec::Json.encode(&envelope).unwrap();
//!
//! // Decode
//! let decoded = Codec::Json.decode(&bytes).unwrap();
//! assert_eq!(decoded.message.intent, Intent::Assert);
//! ```

pub mod codec;
pub mod message;

pub use codec::{Codec, CodecError};
pub use message::{
    AgentIdentity, Capability, CapabilityAdvertisement, Intent, MessageContext,
    OperationDescriptor, Payload, PayloadFormat, ULEnvelope, ULMessage,
};

use chrono::Utc;
use uuid::Uuid;

/// Builder for constructing UL Transceiver messages.
pub struct ULMessageBuilder {
    sender_id: String,
    capabilities: Vec<Capability>,
    intent: Intent,
    payload_format: PayloadFormat,
    content: Option<serde_json::Value>,
    confidence: Option<f64>,
    conversation_id: Option<Uuid>,
    in_reply_to: Option<Uuid>,
    domain: Option<String>,
    sequence_number: Option<u64>,
}

impl ULMessageBuilder {
    /// Create a new builder with a sender ID.
    pub fn new(sender_id: impl Into<String>) -> Self {
        Self {
            sender_id: sender_id.into(),
            capabilities: Vec::new(),
            intent: Intent::Assert,
            payload_format: PayloadFormat::Gir,
            content: None,
            confidence: None,
            conversation_id: None,
            in_reply_to: None,
            domain: None,
            sequence_number: None,
        }
    }

    /// Set the message intent.
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    /// Set sender capabilities.
    pub fn capabilities(mut self, caps: Vec<Capability>) -> Self {
        self.capabilities = caps;
        self
    }

    /// Set a GIR payload.
    pub fn gir_payload(mut self, gir: serde_json::Value) -> Self {
        self.payload_format = PayloadFormat::Gir;
        self.content = Some(gir);
        self
    }

    /// Set a UL-Script payload.
    pub fn ul_script_payload(mut self, script: impl Into<String>) -> Self {
        self.payload_format = PayloadFormat::UlScript;
        self.content = Some(serde_json::Value::String(script.into()));
        self
    }

    /// Set confidence score (for propose intent).
    pub fn confidence(mut self, c: f64) -> Self {
        self.confidence = Some(c);
        self
    }

    /// Set conversation ID.
    pub fn conversation(mut self, id: Uuid) -> Self {
        self.conversation_id = Some(id);
        self
    }

    /// Set in-reply-to message ID.
    pub fn reply_to(mut self, id: Uuid) -> Self {
        self.in_reply_to = Some(id);
        self
    }

    /// Set knowledge domain.
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// Set sequence number.
    pub fn sequence(mut self, seq: u64) -> Self {
        self.sequence_number = Some(seq);
        self
    }

    /// Build the envelope.
    pub fn build(self) -> ULEnvelope {
        let has_context = self.conversation_id.is_some()
            || self.in_reply_to.is_some()
            || self.domain.is_some()
            || self.sequence_number.is_some();

        ULEnvelope {
            version: "1.0".into(),
            message: ULMessage {
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
                sender: AgentIdentity {
                    id: self.sender_id,
                    capabilities: self.capabilities,
                },
                intent: self.intent,
                payload: Payload {
                    format: self.payload_format,
                    content: self.content,
                    confidence: self.confidence,
                },
                context: if has_context {
                    Some(MessageContext {
                        conversation_id: self.conversation_id,
                        in_reply_to: self.in_reply_to,
                        domain: self.domain,
                        sequence_number: self.sequence_number,
                    })
                } else {
                    None
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn roundtrip_json() {
        let envelope = ULMessageBuilder::new("test-agent")
            .intent(Intent::Assert)
            .gir_payload(json!({"ul_gir": "1.0", "root": "n0", "nodes": [], "edges": []}))
            .domain("test")
            .build();

        let bytes = Codec::Json.encode(&envelope).unwrap();
        let decoded = Codec::Json.decode(&bytes).unwrap();

        assert_eq!(decoded.version, "1.0");
        assert_eq!(decoded.message.intent, Intent::Assert);
        assert_eq!(decoded.message.sender.id, "test-agent");
        assert_eq!(
            decoded.message.context.as_ref().unwrap().domain.as_deref(),
            Some("test")
        );
    }

    #[test]
    fn capability_advertisement() {
        let envelope = ULMessageBuilder::new("agent-beta")
            .intent(Intent::CapabilityAdvertisement)
            .capabilities(vec![
                Capability::Parse,
                Capability::Validate,
                Capability::Render,
            ])
            .build();

        let json = Codec::Json.encode_pretty(&envelope).unwrap();
        let decoded = Codec::Json.decode_str(&json).unwrap();

        assert_eq!(decoded.message.sender.capabilities.len(), 3);
        assert_eq!(
            decoded.message.intent,
            Intent::CapabilityAdvertisement
        );
    }

    #[test]
    fn propose_with_confidence() {
        let envelope = ULMessageBuilder::new("agent-gamma")
            .intent(Intent::Propose)
            .gir_payload(json!({"ul_gir": "1.0", "root": "n0", "nodes": [], "edges": []}))
            .confidence(0.85)
            .build();

        let decoded = Codec::Json
            .decode(&Codec::Json.encode(&envelope).unwrap())
            .unwrap();

        assert_eq!(decoded.message.payload.confidence, Some(0.85));
    }

    #[test]
    fn ack_with_reply_to() {
        let original_id = Uuid::new_v4();
        let conv_id = Uuid::new_v4();

        let envelope = ULMessageBuilder::new("agent-delta")
            .intent(Intent::Ack)
            .reply_to(original_id)
            .conversation(conv_id)
            .sequence(1)
            .build();

        let decoded = Codec::Json
            .decode(&Codec::Json.encode(&envelope).unwrap())
            .unwrap();

        let ctx = decoded.message.context.unwrap();
        assert_eq!(ctx.in_reply_to, Some(original_id));
        assert_eq!(ctx.conversation_id, Some(conv_id));
        assert_eq!(ctx.sequence_number, Some(1));
    }
}
