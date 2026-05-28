use serde::{Deserialize, Serialize};

use super::harness::{Harness, SourceType};
use super::provider::Provider;

/// A catalog skill entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
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
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub certifications: Option<Vec<String>>,
    #[serde(default)]
    pub companion_review_skills: Option<Vec<String>>,
    #[serde(default)]
    pub companion_skills: Option<Vec<String>>,
    #[serde(default)]
    pub execution_tier: Option<String>,
    #[serde(default)]
    pub feeds_skills: Option<Vec<String>>,
    #[serde(default)]
    pub lifecycle: Option<String>,
    #[serde(default)]
    pub mcp_servers: Option<Vec<String>>,
    #[serde(default)]
    pub oauth_scopes: Option<Vec<String>>,
    #[serde(default)]
    pub production_allowed: Option<bool>,
    #[serde(default)]
    pub run_as_permissions: Option<serde_json::Value>,
    #[serde(default)]
    pub sandbox_only: Option<bool>,
    #[serde(default)]
    pub source_attribution: Option<String>,
    #[serde(default)]
    pub verify_before_merge: Option<String>,
}
