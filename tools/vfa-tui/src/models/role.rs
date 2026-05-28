use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Top-level structure of install-roles.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCatalog {
    pub version: String,
    pub description: String,
    pub roles: HashMap<String, Role>,
}

/// A single install role definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub label: String,
    pub description: String,
    pub agents: Vec<String>,
    #[serde(default)]
    pub skills: Vec<String>,
}
