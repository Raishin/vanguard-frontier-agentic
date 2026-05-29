use std::path::Path;

use crate::error::TuiError;
use crate::models::{Agent, AssetIntegrity, McpReference, RoleCatalog, Rule, Skill};
use crate::security::sanitize::has_control_bytes;

const MAX_CATALOG_FILE_SIZE: u64 = 20 * 1024 * 1024;

/// Read a catalog file with size validation.
/// Returns an error if the file exceeds `max_size`.
fn read_catalog_file_with_limit(path: &std::path::Path, max_size: u64) -> Result<String, TuiError> {
    let metadata = std::fs::metadata(path).map_err(|_| TuiError::CatalogNotFound {
        path: path.to_path_buf(),
    })?;
    if metadata.len() > max_size {
        return Err(TuiError::CatalogParse {
            path: path.to_path_buf(),
            offset: 0,
            detail: format!(
                "file too large: {} bytes exceeds maximum of {} bytes",
                metadata.len(),
                max_size
            ),
        });
    }
    std::fs::read_to_string(path).map_err(|_| TuiError::CatalogNotFound {
        path: path.to_path_buf(),
    })
}

/// Read a catalog file with the default size limit (100MB).
fn read_catalog_file(path: &std::path::Path) -> Result<String, TuiError> {
    read_catalog_file_with_limit(path, MAX_CATALOG_FILE_SIZE)
}

/// Load agents from catalog/agents.json.
/// Returns loaded agents and any errors encountered.
pub fn load_agents(workspace_root: &Path) -> (Vec<Agent>, Vec<TuiError>) {
    let path = workspace_root.join("catalog").join("agents.json");
    let mut errors = Vec::new();

    let data = match read_catalog_file(&path) {
        Ok(d) => d,
        Err(e) => {
            errors.push(e);
            return (Vec::new(), errors);
        }
    };

    let all_agents: Vec<Agent> = match serde_json::from_str(&data) {
        Ok(a) => a,
        Err(e) => {
            errors.push(TuiError::CatalogParse {
                path,
                offset: e.column(),
                detail: e.to_string(),
            });
            return (Vec::new(), errors);
        }
    };

    let mut agents = Vec::new();
    for (idx, agent) in all_agents.into_iter().enumerate() {
        if check_agent_tainted(&agent) {
            errors.push(TuiError::TaintedEntry {
                path: path.clone(),
                offset: idx,
                field: "control bytes detected".to_string(),
            });
        } else {
            agents.push(agent);
        }
    }

    (agents, errors)
}

/// Load skills from catalog/skills.json.
pub fn load_skills(workspace_root: &Path) -> (Vec<Skill>, Vec<TuiError>) {
    let path = workspace_root.join("catalog").join("skills.json");
    let mut errors = Vec::new();

    let data = match read_catalog_file(&path) {
        Ok(d) => d,
        Err(e) => {
            errors.push(e);
            return (Vec::new(), errors);
        }
    };

    let all_skills: Vec<Skill> = match serde_json::from_str(&data) {
        Ok(s) => s,
        Err(e) => {
            errors.push(TuiError::CatalogParse {
                path,
                offset: e.column(),
                detail: e.to_string(),
            });
            return (Vec::new(), errors);
        }
    };

    let mut skills = Vec::new();
    for (idx, skill) in all_skills.into_iter().enumerate() {
        if check_skill_tainted(&skill) {
            errors.push(TuiError::TaintedEntry {
                path: path.clone(),
                offset: idx,
                field: "control bytes detected".to_string(),
            });
        } else {
            skills.push(skill);
        }
    }

    (skills, errors)
}

/// Load MCP references from catalog/mcp-references.json.
pub fn load_mcp_refs(workspace_root: &Path) -> (Vec<McpReference>, Vec<TuiError>) {
    let path = workspace_root.join("catalog").join("mcp-references.json");
    let mut errors = Vec::new();

    let data = match read_catalog_file(&path) {
        Ok(d) => d,
        Err(e) => {
            errors.push(e);
            return (Vec::new(), errors);
        }
    };

    let all_refs: Vec<McpReference> = match serde_json::from_str(&data) {
        Ok(r) => r,
        Err(e) => {
            errors.push(TuiError::CatalogParse {
                path,
                offset: e.column(),
                detail: e.to_string(),
            });
            return (Vec::new(), errors);
        }
    };

    let mut refs = Vec::new();
    for (idx, mcp_ref) in all_refs.into_iter().enumerate() {
        if check_mcp_ref_tainted(&mcp_ref) {
            errors.push(TuiError::TaintedEntry {
                path: path.clone(),
                offset: idx,
                field: "control bytes detected".to_string(),
            });
        } else {
            refs.push(mcp_ref);
        }
    }

    (refs, errors)
}

/// Load rules from catalog/rules.json.
pub fn load_rules(workspace_root: &Path) -> (Vec<Rule>, Vec<TuiError>) {
    let path = workspace_root.join("catalog").join("rules.json");
    let mut errors = Vec::new();

    let data = match read_catalog_file(&path) {
        Ok(d) => d,
        Err(e) => {
            errors.push(e);
            return (Vec::new(), errors);
        }
    };

    let all_rules: Vec<Rule> = match serde_json::from_str(&data) {
        Ok(r) => r,
        Err(e) => {
            errors.push(TuiError::CatalogParse {
                path,
                offset: e.column(),
                detail: e.to_string(),
            });
            return (Vec::new(), errors);
        }
    };

    let mut rules = Vec::new();
    for (idx, rule) in all_rules.into_iter().enumerate() {
        if check_rule_tainted(&rule) {
            errors.push(TuiError::TaintedEntry {
                path: path.clone(),
                offset: idx,
                field: "control bytes detected".to_string(),
            });
        } else {
            rules.push(rule);
        }
    }

    (rules, errors)
}

/// Load roles from catalog/install-roles.json.
/// This is an object, not an array.
pub fn load_roles(workspace_root: &Path) -> (Option<RoleCatalog>, Vec<TuiError>) {
    let path = workspace_root.join("catalog").join("install-roles.json");
    let mut errors = Vec::new();

    let data = match read_catalog_file(&path) {
        Ok(d) => d,
        Err(e) => {
            errors.push(e);
            return (None, errors);
        }
    };

    match serde_json::from_str::<RoleCatalog>(&data) {
        Ok(catalog) => (Some(catalog), errors),
        Err(e) => {
            errors.push(TuiError::CatalogParse {
                path,
                offset: e.column(),
                detail: e.to_string(),
            });
            (None, errors)
        }
    }
}

/// Load asset integrity from catalog/asset-integrity.json.
pub fn load_integrity(workspace_root: &Path) -> (Option<AssetIntegrity>, Vec<TuiError>) {
    let path = workspace_root.join("catalog").join("asset-integrity.json");
    let mut errors = Vec::new();

    let data = match read_catalog_file(&path) {
        Ok(d) => d,
        Err(e) => {
            errors.push(e);
            return (None, errors);
        }
    };

    match serde_json::from_str::<AssetIntegrity>(&data) {
        Ok(integrity) => (Some(integrity), errors),
        Err(e) => {
            errors.push(TuiError::CatalogParse {
                path,
                offset: e.column(),
                detail: e.to_string(),
            });
            (None, errors)
        }
    }
}

// Taint checks for each model type.

fn check_agent_tainted(agent: &Agent) -> bool {
    serde_json::to_value(agent)
        .map(|value| value_has_control_bytes(&value))
        .unwrap_or(true)
}

fn check_skill_tainted(skill: &Skill) -> bool {
    serde_json::to_value(skill)
        .map(|value| value_has_control_bytes(&value))
        .unwrap_or(true)
}

fn check_mcp_ref_tainted(mcp_ref: &McpReference) -> bool {
    serde_json::to_value(mcp_ref)
        .map(|value| value_has_control_bytes(&value))
        .unwrap_or(true)
}

fn check_rule_tainted(rule: &Rule) -> bool {
    serde_json::to_value(rule)
        .map(|value| value_has_control_bytes(&value))
        .unwrap_or(true)
}

fn value_has_control_bytes(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => has_control_bytes(s),
        serde_json::Value::Array(items) => items.iter().any(value_has_control_bytes),
        serde_json::Value::Object(map) => map
            .iter()
            .any(|(key, value)| has_control_bytes(key) || value_has_control_bytes(value)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn workspace_root() -> &'static Path {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        // tools/vfa-tui -> project root
        let p = Path::new(manifest_dir).parent().unwrap().parent().unwrap();
        // Leak to get static lifetime for testing
        Box::leak(Box::new(p.to_path_buf()))
    }

    #[test]
    fn load_agents_succeeds() {
        let (agents, errors) = load_agents(workspace_root());
        assert!(errors.is_empty(), "errors: {errors:?}");
        assert!(!agents.is_empty());
    }

    #[test]
    fn load_skills_succeeds() {
        let (skills, errors) = load_skills(workspace_root());
        assert!(errors.is_empty(), "errors: {errors:?}");
        assert!(!skills.is_empty());
    }

    #[test]
    fn load_mcp_refs_succeeds() {
        let (refs, errors) = load_mcp_refs(workspace_root());
        assert!(errors.is_empty(), "errors: {errors:?}");
        assert!(!refs.is_empty());
    }

    #[test]
    fn load_rules_succeeds() {
        let (rules, errors) = load_rules(workspace_root());
        assert!(errors.is_empty(), "errors: {errors:?}");
        assert!(!rules.is_empty());
    }

    #[test]
    fn load_roles_succeeds() {
        let (roles, errors) = load_roles(workspace_root());
        assert!(errors.is_empty(), "errors: {errors:?}");
        assert!(roles.is_some());
        let catalog = roles.unwrap();
        assert!(!catalog.roles.is_empty());
    }

    #[test]
    fn load_integrity_succeeds() {
        let (integrity, errors) = load_integrity(workspace_root());
        assert!(errors.is_empty(), "errors: {errors:?}");
        assert!(integrity.is_some());
    }

    #[test]
    fn load_agents_missing_file() {
        let tmp = tempfile::TempDir::new().unwrap();
        let (agents, errors) = load_agents(tmp.path());
        assert!(agents.is_empty());
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            TuiError::CatalogNotFound { .. } => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn read_catalog_file_rejects_oversized() {
        // Use the configurable limit function to test the rejection path
        // without creating a 100MB file
        let tmp = tempfile::TempDir::new().unwrap();
        let file = tmp.path().join("small.json");
        std::fs::write(&file, "[1,2,3]").unwrap(); // 7 bytes

        // With a limit smaller than the file, it should be rejected
        let result = read_catalog_file_with_limit(&file, 5);
        assert!(result.is_err());
        match result.unwrap_err() {
            TuiError::CatalogParse { detail, .. } => {
                assert!(
                    detail.contains("file too large"),
                    "unexpected detail: {detail}"
                );
                assert!(
                    detail.contains("7 bytes"),
                    "should report actual size: {detail}"
                );
            }
            other => panic!("expected CatalogParse error, got: {other:?}"),
        }

        // With a limit larger than the file, it should succeed
        let result = read_catalog_file_with_limit(&file, 100);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[1,2,3]");
    }

    #[test]
    fn read_catalog_file_rejects_missing() {
        let tmp = tempfile::TempDir::new().unwrap();
        let missing = tmp.path().join("missing.json");
        let result = read_catalog_file(&missing);
        assert!(result.is_err());
    }
}
