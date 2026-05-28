use std::path::{Path, PathBuf};

use vfa_tui::catalog::loader;
use vfa_tui::catalog::store::CatalogStore;

/// Returns the path to the test fixtures directory (tests/fixtures/).
fn fixtures_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir).join("tests").join("fixtures")
}

#[test]
fn load_all_fixtures_successfully() {
    let root = fixtures_root();
    let store = CatalogStore::load(&root);

    assert!(
        store.load_errors.is_empty(),
        "unexpected load errors: {:?}",
        store.load_errors
    );
    assert_eq!(store.agent_count(), 5);
    assert_eq!(store.skill_count(), 3);
    assert_eq!(store.mcp_refs.len(), 2);
    assert_eq!(store.rules.len(), 2);
    assert!(!store.roles.is_empty());
    assert!(store.integrity.is_some());
}

#[test]
fn partial_loading_when_skills_missing() {
    let tmp = tempfile::TempDir::new().unwrap();
    let catalog_dir = tmp.path().join("catalog");
    std::fs::create_dir_all(&catalog_dir).unwrap();

    // Copy agents.json only
    let fixtures = fixtures_root();
    std::fs::copy(
        fixtures.join("catalog").join("agents.json"),
        catalog_dir.join("agents.json"),
    )
    .unwrap();

    let (agents, agent_errors) = loader::load_agents(tmp.path());
    assert!(agent_errors.is_empty());
    assert_eq!(agents.len(), 5);

    // Skills file missing should produce CatalogNotFound error
    let (skills, skill_errors) = loader::load_skills(tmp.path());
    assert!(skills.is_empty());
    assert_eq!(skill_errors.len(), 1);
}

#[test]
fn error_on_invalid_json() {
    let tmp = tempfile::TempDir::new().unwrap();
    let catalog_dir = tmp.path().join("catalog");
    std::fs::create_dir_all(&catalog_dir).unwrap();

    // Write invalid JSON as agents.json
    let fixtures = fixtures_root();
    let invalid_content = std::fs::read_to_string(fixtures.join("invalid.json")).unwrap();
    std::fs::write(catalog_dir.join("agents.json"), &invalid_content).unwrap();

    let (agents, errors) = loader::load_agents(tmp.path());
    assert!(agents.is_empty());
    assert_eq!(errors.len(), 1);
    // Should be a CatalogParse error
    match &errors[0] {
        vfa_tui::error::TuiError::CatalogParse { detail, .. } => {
            assert!(!detail.is_empty());
        }
        other => panic!("expected CatalogParse, got: {other:?}"),
    }
}

#[test]
fn catalog_store_agent_count() {
    let store = CatalogStore::load(&fixtures_root());
    assert_eq!(store.agent_count(), 5);
}

#[test]
fn catalog_store_skill_count() {
    let store = CatalogStore::load(&fixtures_root());
    assert_eq!(store.skill_count(), 3);
}

#[test]
fn catalog_store_provider_count() {
    let store = CatalogStore::load(&fixtures_root());
    // aws, azure, kubernetes = 3 providers
    assert_eq!(store.provider_count(), 3);
}

#[test]
fn agents_by_provider_returns_correct_subset() {
    let store = CatalogStore::load(&fixtures_root());

    let aws_agents = store.agents_by_provider("aws");
    assert_eq!(aws_agents.len(), 3);
    for agent in &aws_agents {
        assert!(
            agent.id.starts_with("aws-"),
            "expected aws agent, got: {}",
            agent.id
        );
    }

    let azure_agents = store.agents_by_provider("azure");
    assert_eq!(azure_agents.len(), 1);
    assert_eq!(azure_agents[0].id, "azure-rbac-review-agent");

    let k8s_agents = store.agents_by_provider("kubernetes");
    assert_eq!(k8s_agents.len(), 1);
    assert_eq!(k8s_agents[0].id, "kubernetes-rbac-review-agent");
}

#[test]
fn agents_for_role_returns_correct_agents() {
    let store = CatalogStore::load(&fixtures_root());

    let cse_agents = store.agents_for_role("cloud-security-engineer");
    assert_eq!(cse_agents.len(), 3);
    let ids: Vec<&str> = cse_agents.iter().map(|a| a.id.as_str()).collect();
    assert!(ids.contains(&"aws-iam-review-agent"));
    assert!(ids.contains(&"azure-rbac-review-agent"));
    assert!(ids.contains(&"kubernetes-rbac-review-agent"));

    let pe_agents = store.agents_for_role("platform-engineer");
    assert_eq!(pe_agents.len(), 2);
    let pe_ids: Vec<&str> = pe_agents.iter().map(|a| a.id.as_str()).collect();
    assert!(pe_ids.contains(&"kubernetes-rbac-review-agent"));
    assert!(pe_ids.contains(&"aws-bedrock-agent"));
}

#[test]
fn agents_with_skill_returns_correct_reverse_lookup() {
    let store = CatalogStore::load(&fixtures_root());

    let agents = store.agents_with_skill("aws-iam-review");
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].id, "aws-iam-review-agent");

    let k8s_agents = store.agents_with_skill("kubernetes-rbac-review");
    assert_eq!(k8s_agents.len(), 1);
    assert_eq!(k8s_agents[0].id, "kubernetes-rbac-review-agent");
}

#[test]
fn tainted_entry_is_skipped() {
    let tmp = tempfile::TempDir::new().unwrap();
    let catalog_dir = tmp.path().join("catalog");
    std::fs::create_dir_all(&catalog_dir).unwrap();

    // Write agents JSON with a tainted entry (control byte in name)
    let tainted_json = r#"[
  {
    "id": "tainted-agent",
    "name": "Tainted\u0007Agent",
    "type": "agent",
    "provider": "aws",
    "harnesses": ["codex"],
    "summary": "A tainted agent entry.",
    "companion_skills": [],
    "source_type": "original",
    "official_docs": [],
    "security_notes": "None.",
    "last_verified": "2026-01-01",
    "path": "agents/aws/tainted-agent/",
    "version": "0.1.0"
  },
  {
    "id": "clean-agent",
    "name": "Clean Agent",
    "type": "agent",
    "provider": "aws",
    "harnesses": ["codex"],
    "summary": "A clean agent entry.",
    "companion_skills": [],
    "source_type": "original",
    "official_docs": [],
    "security_notes": "None.",
    "last_verified": "2026-01-01",
    "path": "agents/aws/clean-agent/",
    "version": "0.1.0"
  }
]"#;
    std::fs::write(catalog_dir.join("agents.json"), tainted_json).unwrap();

    let (agents, errors) = loader::load_agents(tmp.path());
    // Tainted entry should be skipped, clean one kept
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].id, "clean-agent");
    // One TaintedEntry error
    assert_eq!(errors.len(), 1);
    match &errors[0] {
        vfa_tui::error::TuiError::TaintedEntry { .. } => {}
        other => panic!("expected TaintedEntry, got: {other:?}"),
    }
}

#[test]
fn agents_sorted_by_id_case_insensitive() {
    let store = CatalogStore::load(&fixtures_root());
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
fn skills_sorted_by_id_case_insensitive() {
    let store = CatalogStore::load(&fixtures_root());
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
fn integrity_manifest_loaded_correctly() {
    let store = CatalogStore::load(&fixtures_root());
    let integrity = store.integrity.unwrap();
    assert_eq!(integrity.manifest_version, 1);
    assert_eq!(integrity.algorithm, "sha256");
    assert_eq!(integrity.scope.trees, vec!["agents"]);
    assert_eq!(integrity.scope.root_files, vec!["README.md"]);
    assert_eq!(integrity.trees.len(), 1);
    assert_eq!(integrity.trees[0].tree, "agents");
    assert_eq!(integrity.trees[0].files.len(), 2);
    assert_eq!(integrity.root_files.len(), 1);
    assert_eq!(integrity.aggregate_sha256, "overall_hash_value");
}

#[test]
fn role_catalog_metadata_loaded() {
    let store = CatalogStore::load(&fixtures_root());
    assert_eq!(store.role_catalog_version, "0.1.0");
    assert_eq!(store.role_catalog_description, "Test roles");
    assert_eq!(store.roles.len(), 2);
    assert!(store.roles.contains_key("cloud-security-engineer"));
    assert!(store.roles.contains_key("platform-engineer"));
}
