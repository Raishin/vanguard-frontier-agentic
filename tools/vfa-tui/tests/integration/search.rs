use std::path::{Path, PathBuf};

use vfa_tui::catalog::store::CatalogStore;
use vfa_tui::search::fuzzy::SearchEngine;

/// Returns the path to the test fixtures directory (tests/fixtures/).
fn fixtures_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir).join("tests").join("fixtures")
}

#[test]
fn search_aws_returns_aws_agents() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_agents("aws", &store.agents, None, None);
    assert!(!results.is_empty());

    // The top results should include all 3 AWS agents
    let result_ids: Vec<&str> = results
        .iter()
        .map(|i| store.agents[*i].id.as_str())
        .collect();
    assert!(
        result_ids.contains(&"aws-iam-review-agent"),
        "expected aws-iam-review-agent in results"
    );
    assert!(
        result_ids.contains(&"aws-s3-security-agent"),
        "expected aws-s3-security-agent in results"
    );
    assert!(
        result_ids.contains(&"aws-bedrock-agent"),
        "expected aws-bedrock-agent in results"
    );
}

#[test]
fn search_rbac_matches_kubernetes_and_azure() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_agents("rbac", &store.agents, None, None);
    assert!(results.len() >= 2);

    let result_ids: Vec<&str> = results
        .iter()
        .map(|i| store.agents[*i].id.as_str())
        .collect();
    assert!(
        result_ids.contains(&"kubernetes-rbac-review-agent"),
        "expected kubernetes-rbac-review-agent in results"
    );
    assert!(
        result_ids.contains(&"azure-rbac-review-agent"),
        "expected azure-rbac-review-agent in results"
    );
}

#[test]
fn combined_provider_filter_and_search_query() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // Search for "iam" within aws provider only
    let results = engine.search_agents("iam", &store.agents, Some("aws"), None);
    assert!(!results.is_empty());

    // All results should be from AWS provider
    for idx in &results {
        let provider_str = serde_json::to_value(store.agents[*idx].provider)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default();
        assert_eq!(provider_str, "aws");
    }
}

#[test]
fn empty_query_returns_all_agents() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_agents("", &store.agents, None, None);
    assert_eq!(results.len(), store.agents.len());
}

#[test]
fn no_match_query_returns_empty() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_agents("zzzznonexistentzzzz", &store.agents, None, None);
    assert!(results.is_empty());
}

#[test]
fn skill_search_returns_matching_skills() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_skills("iam", &store.skills);
    assert!(!results.is_empty());
    // Should match the aws-iam-review skill
    let result_ids: Vec<&str> = results
        .iter()
        .map(|i| store.skills[*i].id.as_str())
        .collect();
    assert!(result_ids.contains(&"aws-iam-review"));
}

#[test]
fn skill_search_rbac() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_skills("rbac", &store.skills);
    assert!(!results.is_empty());
    let result_ids: Vec<&str> = results
        .iter()
        .map(|i| store.skills[*i].id.as_str())
        .collect();
    assert!(result_ids.contains(&"kubernetes-rbac-review"));
}

#[test]
fn skill_search_empty_returns_all() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_skills("", &store.skills);
    assert_eq!(results.len(), store.skills.len());
}

#[test]
fn provider_filter_without_query() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_agents("", &store.agents, Some("kubernetes"), None);
    assert_eq!(results.len(), 1);
    assert_eq!(store.agents[results[0]].id, "kubernetes-rbac-review-agent");
}

#[test]
fn harness_filter_narrows_results() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // Only agents with kiro harness
    let results = engine.search_agents("", &store.agents, None, Some("kiro"));
    assert!(!results.is_empty());
    for idx in &results {
        let agent = &store.agents[*idx];
        let has_kiro = agent.harnesses.iter().any(|h| {
            serde_json::to_value(h)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default()
                == "kiro"
        });
        assert!(has_kiro, "agent {} should have kiro harness", agent.id);
    }
}
