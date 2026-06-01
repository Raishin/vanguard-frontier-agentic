use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::error::TuiError;
use crate::models::{Agent, AssetIntegrity, McpReference, Role, Rule, Skill};

use super::loader;

/// In-memory catalog store holding all loaded catalog data.
pub struct CatalogStore {
    pub agents: Vec<Agent>,
    pub skills: Vec<Skill>,
    pub roles: HashMap<String, Role>,
    pub role_catalog_version: String,
    pub role_catalog_description: String,
    pub mcp_refs: Vec<McpReference>,
    pub rules: Vec<Rule>,
    pub integrity: Option<AssetIntegrity>,
    pub load_errors: Vec<TuiError>,
}

impl CatalogStore {
    /// Load all catalog files from the workspace root.
    /// Sorts agents, skills, mcp_refs, and rules by ID (case-insensitive).
    pub fn load(workspace_root: &Path) -> Self {
        let mut load_errors = Vec::new();

        let (mut agents, errs) = loader::load_agents(workspace_root);
        load_errors.extend(errs);

        let (mut skills, errs) = loader::load_skills(workspace_root);
        load_errors.extend(errs);

        let (mut mcp_refs, errs) = loader::load_mcp_refs(workspace_root);
        load_errors.extend(errs);

        let (mut rules, errs) = loader::load_rules(workspace_root);
        load_errors.extend(errs);

        let (role_catalog, errs) = loader::load_roles(workspace_root);
        load_errors.extend(errs);

        let (integrity, errs) = loader::load_integrity(workspace_root);
        load_errors.extend(errs);

        // Sort by ID, case-insensitive
        agents.sort_by_key(|a| a.id.to_lowercase());
        skills.sort_by_key(|a| a.id.to_lowercase());
        mcp_refs.sort_by_key(|a| a.id.to_lowercase());
        rules.sort_by_key(|a| a.id.to_lowercase());

        let (roles, role_catalog_version, role_catalog_description) = match role_catalog {
            Some(rc) => (rc.roles, rc.version, rc.description),
            None => (HashMap::new(), String::new(), String::new()),
        };

        CatalogStore {
            agents,
            skills,
            roles,
            role_catalog_version,
            role_catalog_description,
            mcp_refs,
            rules,
            integrity,
            load_errors,
        }
    }

    /// Number of loaded agents.
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    /// Number of loaded skills.
    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    /// Number of distinct providers across all agents.
    pub fn provider_count(&self) -> usize {
        let providers: HashSet<_> = self.agents.iter().map(|a| a.provider).collect();
        providers.len()
    }

    /// Filter agents by provider (case-insensitive comparison via Provider enum serialization).
    pub fn agents_by_provider(&self, provider: &str) -> Vec<&Agent> {
        let provider_lower = provider.to_lowercase();
        self.agents
            .iter()
            .filter(|a| {
                let serialized = serde_json::to_value(a.provider)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()));
                match serialized {
                    Some(p) => p.to_lowercase() == provider_lower,
                    None => false,
                }
            })
            .collect()
    }

    /// Get agents listed in a role's agents vec.
    pub fn agents_for_role(&self, role_id: &str) -> Vec<&Agent> {
        let role = match self.roles.get(role_id) {
            Some(r) => r,
            None => return Vec::new(),
        };

        let agent_ids: HashSet<&str> = role.agents.iter().map(|s| s.as_str()).collect();
        self.agents
            .iter()
            .filter(|a| agent_ids.contains(a.id.as_str()))
            .collect()
    }

    /// Get agents for a role grouped by provider.
    /// Returns a vec of (provider_name, agents) tuples sorted alphabetically by provider,
    /// with agents sorted alphabetically by ID within each group.
    pub fn agents_for_role_by_provider(&self, role_id: &str) -> Vec<(String, Vec<&Agent>)> {
        let agents = self.agents_for_role(role_id);

        // Group agents by their serialized provider name
        let mut groups: HashMap<String, Vec<&Agent>> = HashMap::new();
        for agent in agents {
            let provider_name = serde_json::to_value(agent.provider)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_else(|| format!("{:?}", agent.provider));
            groups.entry(provider_name).or_default().push(agent);
        }

        // Sort agents within each group by ID (case-insensitive)
        for agents_in_group in groups.values_mut() {
            agents_in_group.sort_by_key(|a| a.id.to_lowercase());
        }

        // Sort groups alphabetically by provider name
        let mut result: Vec<(String, Vec<&Agent>)> = groups.into_iter().collect();
        result.sort_by_key(|(provider, _)| provider.to_lowercase());
        result
    }

    /// Find skills whose id is in an agent's companion_skills.
    pub fn skills_for_agent(&self, agent_id: &str) -> Vec<&Skill> {
        let agent = match self.agents.iter().find(|a| a.id == agent_id) {
            Some(a) => a,
            None => return Vec::new(),
        };

        let skill_ids: HashSet<&str> = agent.companion_skills.iter().map(|s| s.as_str()).collect();
        self.skills
            .iter()
            .filter(|s| skill_ids.contains(s.id.as_str()))
            .collect()
    }

    /// Reverse lookup: agents whose companion_skills contains the given skill_id.
    pub fn agents_with_skill(&self, skill_id: &str) -> Vec<&Agent> {
        self.agents
            .iter()
            .filter(|a| a.companion_skills.iter().any(|s| s == skill_id))
            .collect()
    }

    /// Reverse lookup: roles that contain the given agent_id.
    pub fn roles_containing_agent(&self, agent_id: &str) -> Vec<&str> {
        self.roles
            .iter()
            .filter(|(_id, role)| role.agents.iter().any(|a| a == agent_id))
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// Get all distinct provider names from agents.
    pub fn provider_names(&self) -> Vec<String> {
        let mut providers: std::collections::HashSet<String> = std::collections::HashSet::new();
        for agent in &self.agents {
            let p = serde_json::to_value(agent.provider)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_else(|| format!("{:?}", agent.provider));
            providers.insert(p);
        }
        let mut list: Vec<String> = providers.into_iter().collect();
        list.sort();
        list
    }

    /// Get all distinct harness names from agents.
    pub fn harness_names(&self) -> Vec<String> {
        let mut harnesses: std::collections::HashSet<String> = std::collections::HashSet::new();
        for agent in &self.agents {
            for h in &agent.harnesses {
                let s = serde_json::to_value(h)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| format!("{h:?}"));
                harnesses.insert(s);
            }
        }
        let mut list: Vec<String> = harnesses.into_iter().collect();
        list.sort();
        list
    }

    /// Get all role IDs sorted.
    pub fn role_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.roles.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Get all valid platform names for export.
    pub fn platform_names(&self) -> Vec<&'static str> {
        vec!["kiro", "claude-code", "cursor", "copilot", "codex", "gemini"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn workspace_root() -> &'static Path {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let p = Path::new(manifest_dir).parent().unwrap().parent().unwrap();
        Box::leak(Box::new(p.to_path_buf()))
    }

    #[test]
    fn store_loads_successfully() {
        let store = CatalogStore::load(workspace_root());
        assert!(
            store.load_errors.is_empty(),
            "load errors: {:?}",
            store.load_errors
        );
        assert!(store.agent_count() > 0);
        assert!(store.skill_count() > 0);
        assert!(!store.mcp_refs.is_empty());
        assert!(!store.rules.is_empty());
        assert!(!store.roles.is_empty());
        assert!(store.integrity.is_some());
    }

    #[test]
    fn store_provider_count() {
        let store = CatalogStore::load(workspace_root());
        assert!(store.provider_count() > 1);
    }

    #[test]
    fn store_agents_by_provider() {
        let store = CatalogStore::load(workspace_root());
        let aws_agents = store.agents_by_provider("aws");
        assert!(!aws_agents.is_empty());
    }

    #[test]
    fn store_agents_sorted_case_insensitive() {
        let store = CatalogStore::load(workspace_root());
        for window in store.agents.windows(2) {
            assert!(
                window[0].id.to_lowercase() <= window[1].id.to_lowercase(),
                "agents not sorted: {} > {}",
                window[0].id,
                window[1].id
            );
        }
    }

    #[test]
    fn store_skills_sorted_case_insensitive() {
        let store = CatalogStore::load(workspace_root());
        for window in store.skills.windows(2) {
            assert!(
                window[0].id.to_lowercase() <= window[1].id.to_lowercase(),
                "skills not sorted: {} > {}",
                window[0].id,
                window[1].id
            );
        }
    }

    #[test]
    fn store_agents_for_role() {
        let store = CatalogStore::load(workspace_root());
        // Get the first role
        if let Some((role_id, role)) = store.roles.iter().next() {
            if !role.agents.is_empty() {
                let agents = store.agents_for_role(role_id);
                assert!(!agents.is_empty());
            }
        }
    }

    #[test]
    fn store_skills_for_agent() {
        let store = CatalogStore::load(workspace_root());
        // Find an agent with companion skills
        if let Some(agent) = store.agents.iter().find(|a| !a.companion_skills.is_empty()) {
            let skills = store.skills_for_agent(&agent.id);
            assert!(!skills.is_empty());
        }
    }

    #[test]
    fn store_agents_with_skill() {
        let store = CatalogStore::load(workspace_root());
        // Find a skill that some agent references
        if let Some(agent) = store.agents.iter().find(|a| !a.companion_skills.is_empty()) {
            let skill_id = &agent.companion_skills[0];
            let agents = store.agents_with_skill(skill_id);
            assert!(!agents.is_empty());
            assert!(agents.iter().any(|a| a.id == agent.id));
        }
    }

    #[test]
    fn store_agents_with_skill_returns_empty_for_unknown() {
        let store = CatalogStore::load(workspace_root());
        let agents = store.agents_with_skill("nonexistent-skill-id-xyz");
        assert!(agents.is_empty());
    }

    #[test]
    fn store_agents_for_role_by_provider() {
        let store = CatalogStore::load(workspace_root());
        // Find a role with agents from multiple providers
        if let Some((role_id, _role)) = store.roles.iter().find(|(_, r)| r.agents.len() > 2) {
            let grouped = store.agents_for_role_by_provider(role_id);
            assert!(!grouped.is_empty());

            // Verify provider groups are sorted alphabetically
            for window in grouped.windows(2) {
                assert!(
                    window[0].0.to_lowercase() <= window[1].0.to_lowercase(),
                    "provider groups not sorted: {} > {}",
                    window[0].0,
                    window[1].0
                );
            }

            // Verify agents within each group are sorted by ID
            for (_provider, agents) in &grouped {
                for window in agents.windows(2) {
                    assert!(
                        window[0].id.to_lowercase() <= window[1].id.to_lowercase(),
                        "agents in group not sorted: {} > {}",
                        window[0].id,
                        window[1].id
                    );
                }
            }

            // Verify total count matches flat query
            let flat = store.agents_for_role(role_id);
            let grouped_total: usize = grouped.iter().map(|(_, agents)| agents.len()).sum();
            assert_eq!(grouped_total, flat.len());
        } else {
            // If no role has >2 agents, just test with any role
            if let Some((role_id, _)) = store.roles.iter().next() {
                let grouped = store.agents_for_role_by_provider(role_id);
                let flat = store.agents_for_role(role_id);
                let grouped_total: usize = grouped.iter().map(|(_, agents)| agents.len()).sum();
                assert_eq!(grouped_total, flat.len());
            }
        }
    }

    #[test]
    fn store_agents_for_role_by_provider_unknown_role() {
        let store = CatalogStore::load(workspace_root());
        let grouped = store.agents_for_role_by_provider("nonexistent-role-xyz");
        assert!(grouped.is_empty());
    }
}
