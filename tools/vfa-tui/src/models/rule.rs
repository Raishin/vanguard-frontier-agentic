use serde::{Deserialize, Serialize};

use super::harness::{Harness, SourceType};
use super::provider::Provider;

/// A catalog rule entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
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
    #[serde(default)]
    pub author: Option<String>,
}
