use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::harness::{Harness, SourceType};
use super::provider::Provider;

/// A catalog agent entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub provider: Provider,
    pub harnesses: Vec<Harness>,
    pub summary: String,
    #[serde(default)]
    pub companion_skills: Vec<String>,
    pub source_type: SourceType,
    pub official_docs: Vec<String>,
    pub security_notes: String,
    pub last_verified: String,
    pub path: String,
    #[serde(default)]
    pub harness_variants: Option<HashMap<String, String>>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub execution_tier: Option<String>,
    #[serde(default)]
    pub lifecycle: Option<String>,
    #[serde(default)]
    pub provider_coverage: Option<serde_json::Value>,
}
