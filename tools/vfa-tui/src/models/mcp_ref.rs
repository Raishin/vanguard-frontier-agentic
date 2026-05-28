use serde::{Deserialize, Serialize};

use super::harness::{Harness, SourceType};
use super::provider::Provider;

/// A catalog MCP reference entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpReference {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: String,
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

/// Security trust assessment for an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustMatrix {
    pub mutation_capable: bool,
    pub requires_egress: bool,
    pub requires_credentials: bool,
    pub signed_release: String,
    pub pin_strategy: String,
}
