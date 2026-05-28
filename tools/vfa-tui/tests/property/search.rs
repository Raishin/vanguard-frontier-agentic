use proptest::prelude::*;
use proptest::test_runner::Config;
use std::path::Path;
use vfa_tui::catalog::store::CatalogStore;
use vfa_tui::search::fuzzy::SearchEngine;

fn workspace_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir).parent().unwrap().parent().unwrap();
    Box::leak(Box::new(p.to_path_buf()))
}

// Property 2: Fuzzy search returns only valid indices.
proptest! {
    #![proptest_config(Config::with_cases(64))]

    #[test]
    fn search_returns_valid_indices(query in "[a-zA-Z]{1,10}") {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();
        let results = engine.search_agents(&query, &store.agents, None, None);

        for idx in &results {
            prop_assert!(*idx < store.agents.len(), "index {} out of bounds", idx);
        }

        // No duplicate indices
        let mut seen = std::collections::HashSet::new();
        for idx in &results {
            prop_assert!(seen.insert(*idx), "duplicate index {}", idx);
        }
    }
}

// Property 3: Combined filter intersection - results satisfy all filters.
proptest! {
    #![proptest_config(Config::with_cases(64))]

    #[test]
    fn provider_filter_respected(query in "[a-zA-Z]{0,5}") {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();
        let results = engine.search_agents(&query, &store.agents, Some("aws"), None);

        for idx in &results {
            let agent = &store.agents[*idx];
            let provider_str = serde_json::to_value(agent.provider)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default();
            prop_assert_eq!(provider_str, "aws", "agent {} has wrong provider", agent.id);
        }
    }
}

// Property 5: Reverse-lookup correctness - agents_with_skill returns agents
// whose companion_skills actually contain the skill_id.
proptest! {
    #![proptest_config(Config::with_cases(64))]

    #[test]
    fn reverse_lookup_consistent(idx in 0usize..100) {
        let store = CatalogStore::load(workspace_root());
        if store.skills.is_empty() {
            return Ok(());
        }
        let skill_idx = idx % store.skills.len();
        let skill_id = &store.skills[skill_idx].id;

        let agents = store.agents_with_skill(skill_id);
        for agent in agents {
            prop_assert!(
                agent.companion_skills.contains(skill_id),
                "agent {} does not list skill {} in companion_skills",
                agent.id,
                skill_id
            );
        }
    }
}
