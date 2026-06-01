use std::path::{Path, PathBuf};

use vfa_tui::catalog::store::CatalogStore;
use vfa_tui::search::fuzzy::SearchEngine;

/// Returns the path to the test fixtures directory (tests/fixtures/).
fn fixtures_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir).join("tests").join("fixtures")
}

// ---------------------------------------------------------------------------
// Fuzzy matching with known inputs and expected results (Req 1.3, 2.3)
// ---------------------------------------------------------------------------

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

/// Fuzzy matching against agent name field (Req 1.3)
#[test]
fn fuzzy_match_by_agent_name() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // "Bedrock Security" appears only in the name of aws-bedrock-agent
    let results = engine.search_agents("Bedrock Security", &store.agents, None, None);
    assert!(!results.is_empty());
    let result_ids: Vec<&str> = results
        .iter()
        .map(|i| store.agents[*i].id.as_str())
        .collect();
    assert!(
        result_ids.contains(&"aws-bedrock-agent"),
        "expected aws-bedrock-agent when searching by name"
    );
}

/// Fuzzy matching against agent summary field (Req 1.3)
#[test]
fn fuzzy_match_by_summary() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // "privilege violations" appears only in the summary of aws-iam-review-agent
    let results = engine.search_agents("privilege violations", &store.agents, None, None);
    assert!(!results.is_empty());
    let result_ids: Vec<&str> = results
        .iter()
        .map(|i| store.agents[*i].id.as_str())
        .collect();
    assert!(
        result_ids.contains(&"aws-iam-review-agent"),
        "expected aws-iam-review-agent when searching by summary content"
    );
}

/// Fuzzy matching against agent provider field (Req 1.3)
#[test]
fn fuzzy_match_by_provider_name() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // "kubernetes" as a search term should match the kubernetes agent via provider field
    let results = engine.search_agents("kubernetes", &store.agents, None, None);
    assert!(!results.is_empty());
    let result_ids: Vec<&str> = results
        .iter()
        .map(|i| store.agents[*i].id.as_str())
        .collect();
    assert!(
        result_ids.contains(&"kubernetes-rbac-review-agent"),
        "expected kubernetes-rbac-review-agent when searching by provider"
    );
}

/// Fuzzy matching is case-insensitive (Req 1.3)
#[test]
fn fuzzy_match_case_insensitive() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results_lower = engine.search_agents("azure", &store.agents, None, None);
    let results_upper = engine.search_agents("AZURE", &store.agents, None, None);

    // Both should find the azure agent
    let ids_lower: Vec<&str> = results_lower
        .iter()
        .map(|i| store.agents[*i].id.as_str())
        .collect();
    let ids_upper: Vec<&str> = results_upper
        .iter()
        .map(|i| store.agents[*i].id.as_str())
        .collect();
    assert!(ids_lower.contains(&"azure-rbac-review-agent"));
    assert!(ids_upper.contains(&"azure-rbac-review-agent"));
}

// ---------------------------------------------------------------------------
// Combined filter intersection semantics (Req 1.5, 1.7)
// ---------------------------------------------------------------------------

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

/// Combined provider AND harness filter intersection (Req 1.7)
/// When both filters are active, only agents matching BOTH should appear.
#[test]
fn combined_provider_and_harness_filter_intersection() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // AWS agents with "codex" harness: aws-iam-review-agent, aws-s3-security-agent, aws-bedrock-agent
    // AWS agents with "cursor" harness: aws-s3-security-agent only
    let results = engine.search_agents("", &store.agents, Some("aws"), Some("cursor"));
    assert_eq!(results.len(), 1);
    assert_eq!(store.agents[results[0]].id, "aws-s3-security-agent");
}

/// Combined provider + harness + search query (Req 1.7)
/// All three constraints must be satisfied simultaneously.
#[test]
fn combined_provider_harness_and_query_intersection() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // Search "security" within aws provider with codex harness
    // aws-iam-review-agent (codex, aws) - no "security" in name/summary? Actually summary has "violations"
    // aws-s3-security-agent (codex, aws) - has "security" in name and summary
    // aws-bedrock-agent (codex, aws) - has "security" in name and summary
    let results = engine.search_agents("security", &store.agents, Some("aws"), Some("codex"));
    assert!(!results.is_empty());

    // All results must be aws + codex
    for idx in &results {
        let agent = &store.agents[*idx];
        let provider_str = serde_json::to_value(agent.provider)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default();
        assert_eq!(provider_str, "aws");
        let has_codex = agent.harnesses.iter().any(|h| {
            serde_json::to_value(h)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default()
                == "codex"
        });
        assert!(has_codex, "agent {} should have codex harness", agent.id);
    }
}

/// Provider filter excludes agents from other providers (Req 1.5)
#[test]
fn provider_filter_excludes_other_providers() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_agents("", &store.agents, Some("azure"), None);
    assert_eq!(results.len(), 1);
    assert_eq!(store.agents[results[0]].id, "azure-rbac-review-agent");

    // Verify no aws or kubernetes agents leak through
    for idx in &results {
        let provider_str = serde_json::to_value(store.agents[*idx].provider)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default();
        assert_eq!(provider_str, "azure");
    }
}

/// Harness filter returns only agents supporting that harness (Req 1.5)
#[test]
fn harness_filter_gemini_returns_correct_agents() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // Agents with gemini: kubernetes-rbac-review-agent, aws-bedrock-agent
    let results = engine.search_agents("", &store.agents, None, Some("gemini"));
    assert_eq!(results.len(), 2);
    let result_ids: Vec<&str> = results
        .iter()
        .map(|i| store.agents[*i].id.as_str())
        .collect();
    assert!(result_ids.contains(&"kubernetes-rbac-review-agent"));
    assert!(result_ids.contains(&"aws-bedrock-agent"));
}

// ---------------------------------------------------------------------------
// Empty result handling (Req 1.8, 2.5)
// ---------------------------------------------------------------------------

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

/// Empty result when combined filters match no agents (Req 1.8)
#[test]
fn combined_filters_yielding_empty_result() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // kubernetes provider + cursor harness: kubernetes-rbac-review-agent has
    // [codex, copilot, claude-code, cursor, gemini, kiro] — actually it DOES have cursor.
    // Let's use a combination that yields empty: azure + gemini
    // azure-rbac-review-agent has [copilot, claude-code, kiro] — no gemini
    let results = engine.search_agents("", &store.agents, Some("azure"), Some("gemini"));
    assert!(
        results.is_empty(),
        "expected empty results for azure+gemini filter combination, got {} results",
        results.len()
    );
}

/// Empty result when provider filter matches no agents (Req 1.8)
#[test]
fn nonexistent_provider_filter_returns_empty() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_agents("", &store.agents, Some("nonexistent-provider"), None);
    assert!(results.is_empty());
}

/// Empty result when harness filter matches no agents (Req 1.8)
#[test]
fn nonexistent_harness_filter_returns_empty() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_agents("", &store.agents, None, Some("nonexistent-harness"));
    assert!(results.is_empty());
}

/// Empty result when query + filter combination matches nothing (Req 1.8)
#[test]
fn query_with_filter_yielding_empty() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // Search for "bedrock" within azure provider — no azure agent has "bedrock"
    let results = engine.search_agents("bedrock", &store.agents, Some("azure"), None);
    assert!(
        results.is_empty(),
        "expected empty results for 'bedrock' query with azure filter"
    );
}

// ---------------------------------------------------------------------------
// Skill search (Req 2.3, 2.5)
// ---------------------------------------------------------------------------

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

/// Skill search with no matches returns empty (Req 2.5)
#[test]
fn skill_search_no_match_returns_empty() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    let results = engine.search_skills("zzzznonexistentzzzz", &store.skills);
    assert!(
        results.is_empty(),
        "expected empty results for non-matching skill query"
    );
}

/// Skill search matches by summary content (Req 2.3)
#[test]
fn skill_search_matches_by_summary() {
    let store = CatalogStore::load(&fixtures_root());
    let mut engine = SearchEngine::new();

    // "misconfigurations" appears only in aws-s3-security skill summary
    let results = engine.search_skills("misconfigurations", &store.skills);
    assert!(!results.is_empty());
    let result_ids: Vec<&str> = results
        .iter()
        .map(|i| store.skills[*i].id.as_str())
        .collect();
    assert!(
        result_ids.contains(&"aws-s3-security"),
        "expected aws-s3-security when searching by summary"
    );
}

// ---------------------------------------------------------------------------
// Existing filter tests
// ---------------------------------------------------------------------------

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
