use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

use super::harness::{Harness, SourceType};
use super::provider::Provider;

/// Deserialize a field, treating an explicit JSON `null` as the type's default.
///
/// `#[serde(default)]` alone covers a *missing* key; the catalog also emits an
/// explicit `null` for empty `companion_skills`, which this helper coalesces.
fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

/// The type discriminator for agent entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentType {
    #[serde(rename = "agent")]
    Agent,
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentType::Agent => write!(f, "agent"),
        }
    }
}

/// Execution tier classification for agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionTier {
    StaticReview,
    ReadOnlyRuntime,
    MutatingRuntime,
}

impl std::fmt::Display for ExecutionTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionTier::StaticReview => write!(f, "static-review"),
            ExecutionTier::ReadOnlyRuntime => write!(f, "read-only-runtime"),
            ExecutionTier::MutatingRuntime => write!(f, "mutating-runtime"),
        }
    }
}

/// Lifecycle stage for agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Lifecycle {
    Experimental,
    Beta,
    Stable,
    Deprecated,
}

impl std::fmt::Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lifecycle::Experimental => write!(f, "experimental"),
            Lifecycle::Beta => write!(f, "beta"),
            Lifecycle::Stable => write!(f, "stable"),
            Lifecycle::Deprecated => write!(f, "deprecated"),
        }
    }
}

/// A catalog agent entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Agent {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: AgentType,
    pub provider: Provider,
    pub harnesses: Vec<Harness>,
    pub summary: String,
    pub source_type: SourceType,
    pub official_docs: Vec<String>,
    pub security_notes: String,
    pub last_verified: String,
    pub path: String,
    #[serde(default, deserialize_with = "null_to_default")]
    pub companion_skills: Vec<String>,
    pub execution_tier: Option<ExecutionTier>,
    pub lifecycle: Option<Lifecycle>,
    #[serde(default)]
    pub harness_variants: HashMap<String, String>,
    pub author: Option<String>,
    pub version: Option<String>,
    #[serde(default)]
    pub provider_coverage: Option<serde_json::Value>,
}
