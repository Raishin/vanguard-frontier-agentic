use std::path::Path;

use crate::error::TuiError;
use crate::models::{
    Agent, AssetIntegrity, McpReference, ModelAssignments, RoleCatalog, Rule, Skill, WorkflowDef,
};
use crate::security::sanitize::has_control_bytes;

const MAX_CATALOG_FILE_SIZE: u64 = 20 * 1024 * 1024;

/// Read a catalog file with size validation.
/// Returns an error if the file exceeds `max_size`.
fn read_catalog_file_with_limit(path: &std::path::Path, max_size: u64) -> Result<String, TuiError> {
    let metadata = std::fs::metadata(path).map_err(|_| TuiError::CatalogNotFound {
        path: path.display().to_string(),
    })?;
    if metadata.len() > max_size {
        return Err(TuiError::CatalogParse {
            path: path.display().to_string(),
            offset: 0,
            detail: format!(
                "file too large: {} bytes exceeds maximum of {} bytes",
                metadata.len(),
                max_size
            ),
        });
    }
    std::fs::read_to_string(path).map_err(|_| TuiError::CatalogNotFound {
        path: path.display().to_string(),
    })
}

/// Read a catalog file with the default size limit (100MB).
fn read_catalog_file(path: &std::path::Path) -> Result<String, TuiError> {
    read_catalog_file_with_limit(path, MAX_CATALOG_FILE_SIZE)
}

/// Load agents from catalog/agents.json.
/// Returns loaded agents and any errors encountered.
pub fn load_agents(workspace_root: &Path) -> (Vec<Agent>, Vec<TuiError>) {
    let file_path = workspace_root.join("catalog").join("agents.json");
    let path = file_path.display().to_string();
    let mut errors = Vec::new();

    let data = match read_catalog_file(&file_path) {
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
    let file_path = workspace_root.join("catalog").join("skills.json");
    let path = file_path.display().to_string();
    let mut errors = Vec::new();

    let data = match read_catalog_file(&file_path) {
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
    let file_path = workspace_root.join("catalog").join("mcp-references.json");
    let path = file_path.display().to_string();
    let mut errors = Vec::new();

    let data = match read_catalog_file(&file_path) {
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
    let file_path = workspace_root.join("catalog").join("rules.json");
    let path = file_path.display().to_string();
    let mut errors = Vec::new();

    let data = match read_catalog_file(&file_path) {
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
    let file_path = workspace_root.join("catalog").join("install-roles.json");
    let path = file_path.display().to_string();
    let mut errors = Vec::new();

    let data = match read_catalog_file(&file_path) {
        Ok(d) => d,
        Err(e) => {
            errors.push(e);
            return (None, errors);
        }
    };

    match serde_json::from_str::<RoleCatalog>(&data) {
        Ok(catalog) => {
            if check_role_catalog_tainted(&catalog) {
                errors.push(TuiError::TaintedEntry {
                    path,
                    offset: 0,
                    field: "control bytes detected".to_string(),
                });
                (None, errors)
            } else {
                (Some(catalog), errors)
            }
        }
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

/// Load executable workflows from `.claude/workflows/`.
/// Scans the directory for `.js` files, parses metadata, and returns sorted by name.
/// A missing `.claude/workflows/` directory is not an error — returns an empty vec with no errors.
/// Files that fail to parse are skipped silently.
pub fn load_workflows(workspace_root: &Path) -> (Vec<WorkflowDef>, Vec<TuiError>) {
    let workflows_dir = workspace_root.join(".claude").join("workflows");
    let mut workflows = Vec::new();
    let errors = Vec::new();

    // Missing directory is not an error for read-only discovery.
    if !workflows_dir.exists() {
        return (workflows, errors);
    }

    // Scan for .js files
    let entries = match std::fs::read_dir(&workflows_dir) {
        Ok(e) => e,
        Err(_) => {
            return (Vec::new(), errors);
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "js") {
            if let Ok(source) = std::fs::read_to_string(&path) {
                if let Some(workflow) =
                    WorkflowDef::parse_meta(&source, path.to_string_lossy().as_ref())
                {
                    workflows.push(workflow);
                }
            }
        }
    }

    // Sort by name for determinism
    workflows.sort_by(|a, b| a.name.cmp(&b.name));

    (workflows, errors)
}

/// Load asset integrity from catalog/asset-integrity.json.
pub fn load_integrity(workspace_root: &Path) -> (Option<AssetIntegrity>, Vec<TuiError>) {
    let file_path = workspace_root.join("catalog").join("asset-integrity.json");
    let path = file_path.display().to_string();
    let mut errors = Vec::new();

    let data = match read_catalog_file(&file_path) {
        Ok(d) => d,
        Err(e) => {
            errors.push(e);
            return (None, errors);
        }
    };

    match serde_json::from_str::<AssetIntegrity>(&data) {
        Ok(integrity) => {
            if check_integrity_tainted(&integrity) {
                errors.push(TuiError::TaintedEntry {
                    path,
                    offset: 0,
                    field: "control bytes detected".to_string(),
                });
                (None, errors)
            } else {
                (Some(integrity), errors)
            }
        }
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

/// Load resolved model assignments from catalog/model-assignments.json.
///
/// A missing file is not an error — the model-policy feature is additive and
/// older checkouts simply render without model information.
pub fn load_model_assignments(workspace_root: &Path) -> (Option<ModelAssignments>, Vec<TuiError>) {
    let file_path = workspace_root
        .join("catalog")
        .join("model-assignments.json");
    let path = file_path.display().to_string();
    let mut errors = Vec::new();

    if !file_path.exists() {
        return (None, errors);
    }

    let data = match read_catalog_file(&file_path) {
        Ok(d) => d,
        Err(e) => {
            errors.push(e);
            return (None, errors);
        }
    };

    match serde_json::from_str::<ModelAssignments>(&data) {
        Ok(assignments) => {
            if check_model_assignments_tainted(&assignments) {
                errors.push(TuiError::TaintedEntry {
                    path,
                    offset: 0,
                    field: "control bytes detected".to_string(),
                });
                (None, errors)
            } else {
                (Some(assignments), errors)
            }
        }
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

pub(crate) fn check_model_assignments_tainted(assignments: &ModelAssignments) -> bool {
    serde_json::to_value(assignments)
        .map(|value| value_has_control_bytes(&value))
        .unwrap_or(true)
}

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

fn check_role_catalog_tainted(role_catalog: &RoleCatalog) -> bool {
    serde_json::to_value(role_catalog)
        .map(|value| value_has_control_bytes(&value))
        .unwrap_or(true)
}

fn check_integrity_tainted(integrity: &AssetIntegrity) -> bool {
    serde_json::to_value(integrity)
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
    fn load_roles_rejects_tainted_strings() {
        let tmp = tempfile::TempDir::new().unwrap();
        let catalog_dir = tmp.path().join("catalog");
        std::fs::create_dir(&catalog_dir).unwrap();
        std::fs::write(
            catalog_dir.join("install-roles.json"),
            r#"{
              "version": "1",
              "description": "roles",
              "roles": {
                "security": {
                  "label": "Security\u001b[31m",
                  "description": "bad",
                  "agents": []
                }
              }
            }"#,
        )
        .unwrap();

        let (roles, errors) = load_roles(tmp.path());
        assert!(roles.is_none());
        assert!(matches!(
            errors.first(),
            Some(TuiError::TaintedEntry { .. })
        ));
    }

    #[test]
    fn load_integrity_rejects_tainted_strings() {
        let tmp = tempfile::TempDir::new().unwrap();
        let catalog_dir = tmp.path().join("catalog");
        std::fs::create_dir(&catalog_dir).unwrap();
        std::fs::write(
            catalog_dir.join("asset-integrity.json"),
            r#"{
              "manifest_version": 1,
              "algorithm": "sha256",
              "scope": { "trees": [], "root_files": [] },
              "trees": [{
                "tree": "assets\u001b[31m",
                "aggregate_sha256": "abc",
                "files": []
              }],
              "root_files": [],
              "aggregate_sha256": "abc"
            }"#,
        )
        .unwrap();

        let (integrity, errors) = load_integrity(tmp.path());
        assert!(integrity.is_none());
        assert!(matches!(
            errors.first(),
            Some(TuiError::TaintedEntry { .. })
        ));
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

    #[test]
    fn load_workflows_missing_directory() {
        let tmp = tempfile::TempDir::new().unwrap();
        let (workflows, errors) = load_workflows(tmp.path());
        assert!(workflows.is_empty());
        assert!(errors.is_empty());
    }

    #[test]
    fn load_workflows_parses_valid() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workflows_dir = tmp.path().join(".claude").join("workflows");
        std::fs::create_dir_all(&workflows_dir).unwrap();

        let workflow_src = r#"
export const meta = {
  name: "test-workflow",
  description: "A test workflow",
  phases: [
    { title: "Phase 1" },
    { title: "Phase 2", detail: "Optional detail" }
  ]
};
"#;
        std::fs::write(workflows_dir.join("test.js"), workflow_src).unwrap();

        let (workflows, errors) = load_workflows(tmp.path());
        assert!(errors.is_empty(), "errors: {errors:?}");
        assert_eq!(workflows.len(), 1);
        assert_eq!(workflows[0].name, "test-workflow");
        assert_eq!(workflows[0].description, "A test workflow");
        assert_eq!(workflows[0].phases.len(), 2);
    }

    #[test]
    fn load_workflows_skips_invalid_meta() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workflows_dir = tmp.path().join(".claude").join("workflows");
        std::fs::create_dir_all(&workflows_dir).unwrap();

        // Valid workflow
        let valid = r#"
export const meta = {
  name: "valid",
  description: "Valid workflow",
  phases: [{ title: "Step" }]
};
"#;
        std::fs::write(workflows_dir.join("valid.js"), valid).unwrap();

        // Invalid workflow (missing description)
        let invalid = r#"
export const meta = {
  name: "invalid",
  phases: [{ title: "Step" }]
};
"#;
        std::fs::write(workflows_dir.join("invalid.js"), invalid).unwrap();

        let (workflows, errors) = load_workflows(tmp.path());
        assert!(errors.is_empty());
        assert_eq!(workflows.len(), 1);
        assert_eq!(workflows[0].name, "valid");
    }

    #[test]
    fn load_workflows_sorted_by_name() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workflows_dir = tmp.path().join(".claude").join("workflows");
        std::fs::create_dir_all(&workflows_dir).unwrap();

        for name in &["zebra", "apple", "banana"] {
            let src = format!(
                r#"
export const meta = {{
  name: "{}",
  description: "desc",
  phases: [{{ title: "Step" }}]
}};
"#,
                name
            );
            std::fs::write(workflows_dir.join(format!("{}.js", name)), src).unwrap();
        }

        let (workflows, errors) = load_workflows(tmp.path());
        assert!(errors.is_empty());
        assert_eq!(workflows.len(), 3);
        assert_eq!(workflows[0].name, "apple");
        assert_eq!(workflows[1].name, "banana");
        assert_eq!(workflows[2].name, "zebra");
    }

    #[test]
    fn load_workflows_ignores_non_js_files() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workflows_dir = tmp.path().join(".claude").join("workflows");
        std::fs::create_dir_all(&workflows_dir).unwrap();

        let workflow_src = r#"
export const meta = {
  name: "valid",
  description: "Valid",
  phases: [{ title: "Step" }]
};
"#;
        std::fs::write(workflows_dir.join("workflow.js"), workflow_src).unwrap();

        // Non-.js files should be ignored
        std::fs::write(workflows_dir.join("readme.txt"), "ignored").unwrap();
        std::fs::write(workflows_dir.join("config.json"), "{}").unwrap();

        let (workflows, errors) = load_workflows(tmp.path());
        assert!(errors.is_empty());
        assert_eq!(workflows.len(), 1);
        assert_eq!(workflows[0].name, "valid");
    }
}
