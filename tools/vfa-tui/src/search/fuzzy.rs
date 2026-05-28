use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher};

use crate::models::{Agent, Skill};

/// Search result with index and match score.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub index: usize,
    pub score: u32,
}

/// Fuzzy search engine backed by nucleo-matcher.
pub struct SearchEngine {
    matcher: Matcher,
}

impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            matcher: Matcher::new(Config::DEFAULT),
        }
    }

    /// Search agents by fuzzy matching against id, name, provider, and summary.
    /// Optionally filter by provider and/or harness first (intersection).
    /// Returns indices sorted by match score descending.
    pub fn search_agents(
        &mut self,
        query: &str,
        agents: &[Agent],
        provider_filter: Option<&str>,
        harness_filter: Option<&str>,
    ) -> Vec<usize> {
        if query.is_empty() && provider_filter.is_none() && harness_filter.is_none() {
            return (0..agents.len()).collect();
        }

        // First apply filters
        let candidates: Vec<usize> = agents
            .iter()
            .enumerate()
            .filter(|(_, agent)| {
                if let Some(pf) = provider_filter {
                    let provider_str = serde_json::to_value(agent.provider)
                        .ok()
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                        .unwrap_or_default();
                    if provider_str.to_lowercase() != pf.to_lowercase() {
                        return false;
                    }
                }
                if let Some(hf) = harness_filter {
                    let has_harness = agent.harnesses.iter().any(|h| {
                        let h_str = serde_json::to_value(h)
                            .ok()
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                            .unwrap_or_default();
                        h_str.to_lowercase() == hf.to_lowercase()
                    });
                    if !has_harness {
                        return false;
                    }
                }
                true
            })
            .map(|(i, _)| i)
            .collect();

        if query.is_empty() {
            return candidates;
        }

        // Fuzzy match
        let pattern = Pattern::new(
            query,
            CaseMatching::Ignore,
            Normalization::Smart,
            AtomKind::Fuzzy,
        );

        let mut results: Vec<SearchResult> = candidates
            .into_iter()
            .filter_map(|idx| {
                let agent = &agents[idx];
                let provider_str = serde_json::to_value(agent.provider)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default();
                let haystack = format!(
                    "{} {} {} {}",
                    agent.id, agent.name, provider_str, agent.summary
                );
                let mut buf = Vec::new();
                let score = pattern.score(
                    nucleo_matcher::Utf32Str::new(&haystack, &mut buf),
                    &mut self.matcher,
                )?;
                Some(SearchResult { index: idx, score })
            })
            .collect();

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results.into_iter().map(|r| r.index).collect()
    }

    /// Search skills by fuzzy matching against id, name, and summary.
    /// Returns indices sorted by match score descending.
    pub fn search_skills(&mut self, query: &str, skills: &[Skill]) -> Vec<usize> {
        if query.is_empty() {
            return (0..skills.len()).collect();
        }

        let pattern = Pattern::new(
            query,
            CaseMatching::Ignore,
            Normalization::Smart,
            AtomKind::Fuzzy,
        );

        let mut results: Vec<SearchResult> = skills
            .iter()
            .enumerate()
            .filter_map(|(idx, skill)| {
                let haystack = format!("{} {} {}", skill.id, skill.name, skill.summary);
                let mut buf = Vec::new();
                let score = pattern.score(
                    nucleo_matcher::Utf32Str::new(&haystack, &mut buf),
                    &mut self.matcher,
                )?;
                Some(SearchResult { index: idx, score })
            })
            .collect();

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results.into_iter().map(|r| r.index).collect()
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::store::CatalogStore;
    use std::path::Path;

    fn workspace_root() -> &'static Path {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let p = Path::new(manifest_dir).parent().unwrap().parent().unwrap();
        Box::leak(Box::new(p.to_path_buf()))
    }

    #[test]
    fn search_agents_empty_query_returns_all() {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();
        let results = engine.search_agents("", &store.agents, None, None);
        assert_eq!(results.len(), store.agents.len());
    }

    #[test]
    fn search_agents_with_query() {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();
        let results = engine.search_agents("aws", &store.agents, None, None);
        assert!(!results.is_empty());
        // Results should be a subset of all agents
        assert!(results.len() <= store.agents.len());
    }

    #[test]
    fn search_agents_with_provider_filter() {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();
        let results = engine.search_agents("", &store.agents, Some("aws"), None);
        assert!(!results.is_empty());
        // All results should be AWS agents
        for idx in &results {
            let provider_str = serde_json::to_value(&store.agents[*idx].provider)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default();
            assert_eq!(provider_str, "aws");
        }
    }

    #[test]
    fn search_skills_with_query() {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();
        let results = engine.search_skills("cost", &store.skills);
        assert!(!results.is_empty());
    }

    #[test]
    fn search_skills_empty_query_returns_all() {
        let store = CatalogStore::load(workspace_root());
        let mut engine = SearchEngine::new();
        let results = engine.search_skills("", &store.skills);
        assert_eq!(results.len(), store.skills.len());
    }
}
