//! # UL Transceiver — Message Types
//!
//! Core message types for the UL Transceiver Protocol v1.0.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Top-level protocol envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ULEnvelope {
    /// Protocol version (always "1.0").
    pub version: String,
    /// The message payload.
    pub message: ULMessage,
}

/// A single UL Transceiver Protocol message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ULMessage {
    /// Unique message identifier.
    pub id: Uuid,
    /// ISO 8601 timestamp.
    pub timestamp: DateTime<Utc>,
    /// Identity of the sending agent.
    pub sender: AgentIdentity,
    /// Message intent.
    pub intent: Intent,
    /// Message payload.
    pub payload: Payload,
    /// Optional conversational context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<MessageContext>,
}

/// Identity and capabilities of an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    /// Agent identifier.
    pub id: String,
    /// UL operations this agent supports.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
}

/// A UL operation capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    Parse,
    Validate,
    Render,
    Evaluate,
    Compose,
    Deparse,
    Analyze,
}

/// Message intent — what the sender wants the receiver to do with this message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    /// Declare a meaning structure as true.
    Assert,
    /// Ask whether a meaning structure holds.
    Query,
    /// Suggest a meaning structure for agreement.
    Propose,
    /// Modify a previously sent structure.
    Refine,
    /// Ask recipient to validate a GIR.
    ValidateRequest,
    /// Report a validation result.
    ValidateResponse,
    /// Announce supported operations.
    CapabilityAdvertisement,
    /// Acknowledge receipt of a message.
    Ack,
}

/// Message payload with format discriminator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    /// Payload format.
    pub format: PayloadFormat,
    /// Payload content (type depends on format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    /// Confidence score (for `propose` intent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// Payload format discriminator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PayloadFormat {
    Gir,
    UlScript,
    ValidationResult,
    CapabilityList,
    Reference,
}

/// Conversational context for threading messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContext {
    /// Conversation group identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<Uuid>,
    /// ID of the message being replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<Uuid>,
    /// Knowledge domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Ordering index within a conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<u64>,
}

/// Capability advertisement payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityAdvertisement {
    /// Supported operations.
    pub operations: Vec<OperationDescriptor>,
    /// Which Σ_UL sorts this agent handles.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_sorts: Vec<String>,
    /// Max GIR node count this agent can process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_gir_nodes: Option<u64>,
    /// Protocol version.
    pub protocol_version: String,
}

/// Description of a single operation capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDescriptor {
    pub name: String,
    pub input: String,
    pub output: String,
}
