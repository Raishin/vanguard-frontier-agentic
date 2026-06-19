use serde::{Deserialize, Serialize};

use super::harness::{Harness, SourceType};
use super::provider::Provider;

/// The type discriminator for MCP reference entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum McpType {
    #[serde(rename = "mcp-reference")]
    McpReference,
}

impl std::fmt::Display for McpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McpType::McpReference => write!(f, "mcp-reference"),
        }
    }
}

/// Signed release verification strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignedRelease {
    Cosign,
    GhAttestation,
    Unsigned,
    Unknown,
}

impl std::fmt::Display for SignedRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignedRelease::Cosign => write!(f, "cosign"),
            SignedRelease::GhAttestation => write!(f, "gh-attestation"),
            SignedRelease::Unsigned => write!(f, "unsigned"),
            SignedRelease::Unknown => write!(f, "unknown"),
        }
    }
}

/// Pin strategy for MCP server versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PinStrategy {
    Digest,
    Tag,
    Version,
    None,
}

impl std::fmt::Display for PinStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PinStrategy::Digest => write!(f, "digest"),
            PinStrategy::Tag => write!(f, "tag"),
            PinStrategy::Version => write!(f, "version"),
            PinStrategy::None => write!(f, "none"),
        }
    }
}

/// Security trust assessment for an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrustMatrix {
    pub mutation_capable: bool,
    pub requires_egress: bool,
    pub requires_credentials: bool,
    pub signed_release: SignedRelease,
    pub pin_strategy: PinStrategy,
}

/// A catalog MCP reference entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpReference {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: McpType,
    pub provider: Provider,
    pub harnesses: Vec<Harness>,
    pub summary: String,
    pub source_type: SourceType,
    pub official_docs: Vec<String>,
    pub security_notes: String,
    pub last_verified: String,
    pub path: String,
    pub official_project_url: String,
    pub vendor: String,
    pub auth_model: String,
    pub install_example: String,
    pub unofficial_warning: String,
    #[serde(default)]
    pub trust_matrix: Option<TrustMatrix>,
}
