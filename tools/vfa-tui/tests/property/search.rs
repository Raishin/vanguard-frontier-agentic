// Feature: rust-tui, Property 2: Fuzzy search returns only matching items
// Feature: rust-tui, Property 3: Combined filter returns correct intersection

use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};
use proptest::prelude::*;
use proptest::test_runner::Config as ProptestConfig;
use std::collections::HashSet;
use std::path::Path;
use vfa_tui::catalog::store::CatalogStore;
use vfa_tui::search::fuzzy::SearchEngine;

fn workspace_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir).parent().unwrap().parent().unwrap();
    Box::leak(Box::new(p.to_path_buf()))
}

/// Helper: check if a query fuzzy-matches at least one searchable field of an agent.
/// Uses the same nucleo-matcher configuration as the SearchEngine.
fn agent_matches_query(agent: &vfa_tui::models::Agent, query: &str) -> bool {
    let pattern = Pattern::new(
        query,
        CaseMatching::Ignore,
        Normalization::Smart,
        AtomKind::Fuzzy,
    );
    let mut matcher = Matcher::new(Config::DEFAULT);

    let provider_str = serde_json::to_value(agent.provider)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    // Check the combined haystack (same as SearchEngine implementation)
    let haystack = format!(
        "{} {} {} {}",
        agent.id, agent.name, provider_str, agent.summary
    );
    let mut buf = Vec::new();
    pattern
        .score(Utf32Str::new(&haystack, &mut buf), &mut matcher)
        .is_some()
}

/// Helper: check if a query fuzzy-matches at least one searchable field of a skill.
fn skill_matches_query(skill: &vfa_tui::models::Skill, query: &str) -> bool {
    let pattern = Pattern::new(
        query,
        CaseMatching::Ignore,
        Normalization::Smart,
        AtomKind::Fuzzy,
    );
    let mut matcher = Matcher::new(Config::DEFAULT);

    let haystack = format!("{} {} {}", skill.id, skill.name, skill.summary);
    let mut buf = Vec::new();
    pattern
        .score(Utf32Str::new(&haystack, &mut buf), &mut matcher)
        .is_some()
}

// **Validates: Requirements 1.3, 2.3**
// Property 2: Fuzzy search returns only matching items
//
// For any non-empty query, every item in the result set SHALL fuzzy-match
// the query against at least one searchable field, and no item that does not
// match any searchable field SHALL appear in the result set.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn fuzzy_search_agents_returns_only_matching(query in "[a-zA-Z0-9]{1,8}") {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();
        let results = engine.search_agents(&query, &store.agents, None, None);

        // Every returned index must correspond to an agent that fuzzy-matches the query
        for idx in &results {
            prop_assert!(
                *idx < store.agents.len(),
                "index {} out of bounds (len={})", idx, store.agents.len()
            );
            let agent = &store.agents[*idx];
            prop_assert!(
                agent_matches_query(agent, &query),
                "agent '{}' at index {} was returned but does not fuzzy-match query '{}'",
                agent.id, idx, query
            );
        }

        // No matching agent should be missing from results (completeness)
        let result_set: HashSet<usize> = results.iter().copied().collect();
        for (idx, agent) in store.agents.iter().enumerate() {
            if agent_matches_query(agent, &query) {
                prop_assert!(
                    result_set.contains(&idx),
                    "agent '{}' at index {} matches query '{}' but was not in results",
                    agent.id, idx, query
                );
            }
        }

        // No duplicate indices
        prop_assert_eq!(results.len(), result_set.len(), "duplicate indices in results");
    }

    #[test]
    fn fuzzy_search_skills_returns_only_matching(query in "[a-zA-Z0-9]{1,8}") {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();
        let results = engine.search_skills(&query, &store.skills);

        // Every returned index must correspond to a skill that fuzzy-matches the query
        for idx in &results {
            prop_assert!(
                *idx < store.skills.len(),
                "index {} out of bounds (len={})", idx, store.skills.len()
            );
            let skill = &store.skills[*idx];
            prop_assert!(
                skill_matches_query(skill, &query),
                "skill '{}' at index {} was returned but does not fuzzy-match query '{}'",
                skill.id, idx, query
            );
        }

        // No matching skill should be missing from results (completeness)
        let result_set: HashSet<usize> = results.iter().copied().collect();
        for (idx, skill) in store.skills.iter().enumerate() {
            if skill_matches_query(skill, &query) {
                prop_assert!(
                    result_set.contains(&idx),
                    "skill '{}' at index {} matches query '{}' but was not in results",
                    skill.id, idx, query
                );
            }
        }

        // No duplicate indices
        prop_assert_eq!(results.len(), result_set.len(), "duplicate indices in results");
    }
}

// **Validates: Requirements 1.5, 1.6, 1.7**
// Property 3: Combined filter returns correct intersection
//
// For any provider filter, harness filter, and search query (each independently
// optional), the filtered result set SHALL equal the intersection of:
// - agents matching the provider (if set)
// - agents containing the harness (if set)
// - agents matching the search query (if set)
// The result set SHALL be a subset of the original list.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn combined_filter_returns_intersection(
        use_provider in any::<bool>(),
        use_harness in any::<bool>(),
        use_query in any::<bool>(),
        query in "[a-zA-Z]{0,6}",
    ) {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();

        // Pick a provider filter from the actual catalog data
        let provider_filter: Option<String> = if use_provider {
            // Use "aws" as a known provider that exists in the catalog
            Some("aws".to_string())
        } else {
            None
        };

        // Pick a harness filter from the actual catalog data
        let harness_filter: Option<String> = if use_harness {
            // Use "claude-code" as a known harness that exists in the catalog
            Some("claude-code".to_string())
        } else {
            None
        };

        // Determine the effective query
        let effective_query = if use_query { &query } else { "" };

        let results = engine.search_agents(
            effective_query,
            &store.agents,
            provider_filter.as_deref(),
            harness_filter.as_deref(),
        );

        // Results must be a subset of the original list
        let result_set: HashSet<usize> = results.iter().copied().collect();
        for idx in &results {
            prop_assert!(*idx < store.agents.len(), "index out of bounds");
        }

        // Independently compute the expected intersection
        let mut expected: HashSet<usize> = (0..store.agents.len()).collect();

        // Apply provider filter independently
        if let Some(ref pf) = provider_filter {
            let provider_matches: HashSet<usize> = store.agents.iter().enumerate()
                .filter(|(_, agent)| {
                    let provider_str = serde_json::to_value(agent.provider)
                        .ok()
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                        .unwrap_or_default();
                    provider_str.to_lowercase() == pf.to_lowercase()
                })
                .map(|(i, _)| i)
                .collect();
            expected = expected.intersection(&provider_matches).copied().collect();
        }

        // Apply harness filter independently
        if let Some(ref hf) = harness_filter {
            let harness_matches: HashSet<usize> = store.agents.iter().enumerate()
                .filter(|(_, agent)| {
                    agent.harnesses.iter().any(|h| {
                        let h_str = serde_json::to_value(h)
                            .ok()
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                            .unwrap_or_default();
                        h_str.to_lowercase() == hf.to_lowercase()
                    })
                })
                .map(|(i, _)| i)
                .collect();
            expected = expected.intersection(&harness_matches).copied().collect();
        }

        // Apply query filter independently
        if !effective_query.is_empty() {
            let query_matches: HashSet<usize> = store.agents.iter().enumerate()
                .filter(|(_, agent)| agent_matches_query(agent, effective_query))
                .map(|(i, _)| i)
                .collect();
            expected = expected.intersection(&query_matches).copied().collect();
        }

        // The result set must equal the expected intersection
        let in_result_not_expected: Vec<_> = result_set.difference(&expected).copied().collect();
        let in_expected_not_result: Vec<_> = expected.difference(&result_set).copied().collect();
        let result_len = result_set.len();
        let expected_len = expected.len();
        prop_assert_eq!(
            result_set, expected,
            "Combined filter result does not match expected intersection.\n\
             provider_filter={:?}, harness_filter={:?}, query='{}'\n\
             result_set has {} items, expected has {} items\n\
             in result but not expected: {:?}\n\
             in expected but not result: {:?}",
            provider_filter, harness_filter, effective_query,
            result_len, expected_len,
            in_result_not_expected,
            in_expected_not_result
        );
    }

    #[test]
    fn combined_filter_with_varied_providers(
        provider_idx in 0usize..5,
        harness_idx in 0usize..4,
        query in "[a-zA-Z]{0,5}",
    ) {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();

        // Use different providers and harnesses to test variety
        let providers = ["aws", "azure", "gcp", "kubernetes", "terraform"];
        let harnesses = ["claude-code", "codex", "copilot", "cursor"];

        let pf = providers[provider_idx % providers.len()];
        let hf = harnesses[harness_idx % harnesses.len()];

        // Test with both provider and harness active
        let results = engine.search_agents(&query, &store.agents, Some(pf), Some(hf));
        let result_set: HashSet<usize> = results.iter().copied().collect();

        // Every result must satisfy ALL active filters
        for idx in &results {
            let agent = &store.agents[*idx];

            // Must match provider
            let provider_str = serde_json::to_value(agent.provider)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default();
            prop_assert_eq!(
                provider_str.to_lowercase(), pf.to_lowercase(),
                "agent '{}' has provider '{}' but filter is '{}'",
                agent.id, provider_str, pf
            );

            // Must match harness
            let has_harness = agent.harnesses.iter().any(|h| {
                let h_str = serde_json::to_value(h)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default();
                h_str.to_lowercase() == hf.to_lowercase()
            });
            prop_assert!(
                has_harness,
                "agent '{}' does not have harness '{}' but was in results",
                agent.id, hf
            );

            // Must match query (if non-empty)
            if !query.is_empty() {
                prop_assert!(
                    agent_matches_query(agent, &query),
                    "agent '{}' does not match query '{}' but was in results",
                    agent.id, query
                );
            }
        }

        // No duplicates
        prop_assert_eq!(results.len(), result_set.len(), "duplicate indices in results");
    }
}

// Property 5: Reverse-lookup returns correct associated agents
// (Kept from original for completeness — validates Requirement 2.2)
proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn reverse_lookup_consistent(idx in 0usize..100) {
        let store = CatalogStore::load(workspace_root());
        if store.skills.is_empty() {
            return Ok(());
        }
        let skill_idx = idx % store.skills.len();
        let skill_id = &store.skills[skill_idx].id;

        let agents = store.agents_with_skill(skill_id);
        for agent in &agents {
            prop_assert!(
                agent.companion_skills.contains(skill_id),
                "agent {} does not list skill {} in companion_skills",
                agent.id,
                skill_id
            );
        }

        // Completeness: every agent that has this skill in companion_skills
        // should be returned by agents_with_skill
        let returned_ids: HashSet<&str> = agents.iter().map(|a| a.id.as_str()).collect();
        for agent in &store.agents {
            if agent.companion_skills.contains(skill_id) {
                prop_assert!(
                    returned_ids.contains(agent.id.as_str()),
                    "agent '{}' has skill '{}' in companion_skills but was not returned by agents_with_skill",
                    agent.id, skill_id
                );
            }
        }
    }
}
