#![deny(warnings)]
#![allow(dead_code)]

mod app;
mod catalog;
mod cli;
mod error;
mod logging;
mod models;
mod search;
mod security;
mod subprocess;
mod ui;
mod workspace;

use clap::Parser;

use crate::cli::Cli;

fn main() {
    let _cli = Cli::parse();
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::models::{Agent, AssetIntegrity, McpReference, RoleCatalog, Rule, Skill};

    fn catalog_path(name: &str) -> std::path::PathBuf {
        // Walk up from the executable to find the workspace root.
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        Path::new(manifest_dir)
            .join("..")
            .join("..")
            .join("catalog")
            .join(name)
    }

    #[test]
    fn deserialize_agents_json() {
        let data = std::fs::read_to_string(catalog_path("agents.json"))
            .expect("failed to read agents.json");
        let agents: Vec<Agent> = serde_json::from_str(&data).expect("failed to parse agents.json");
        assert!(!agents.is_empty());
    }

    #[test]
    fn deserialize_skills_json() {
        let data = std::fs::read_to_string(catalog_path("skills.json"))
            .expect("failed to read skills.json");
        let skills: Vec<Skill> = serde_json::from_str(&data).expect("failed to parse skills.json");
        assert!(!skills.is_empty());
    }

    #[test]
    fn deserialize_mcp_references_json() {
        let data = std::fs::read_to_string(catalog_path("mcp-references.json"))
            .expect("failed to read mcp-references.json");
        let refs: Vec<McpReference> =
            serde_json::from_str(&data).expect("failed to parse mcp-references.json");
        assert!(!refs.is_empty());
    }

    #[test]
    fn deserialize_rules_json() {
        let data =
            std::fs::read_to_string(catalog_path("rules.json")).expect("failed to read rules.json");
        let rules: Vec<Rule> = serde_json::from_str(&data).expect("failed to parse rules.json");
        assert!(!rules.is_empty());
    }

    #[test]
    fn deserialize_install_roles_json() {
        let data = std::fs::read_to_string(catalog_path("install-roles.json"))
            .expect("failed to read install-roles.json");
        let catalog: RoleCatalog =
            serde_json::from_str(&data).expect("failed to parse install-roles.json");
        assert!(!catalog.roles.is_empty());
    }

    #[test]
    fn deserialize_asset_integrity_json() {
        let data = std::fs::read_to_string(catalog_path("asset-integrity.json"))
            .expect("failed to read asset-integrity.json");
        let integrity: AssetIntegrity =
            serde_json::from_str(&data).expect("failed to parse asset-integrity.json");
        assert!(integrity.manifest_version >= 1);
    }
}
