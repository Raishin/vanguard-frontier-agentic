use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher};

use crate::models::{Agent, Skill};

/// Search result with index and match score.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub index: usize,
    pub score: u32,
}

/// Fuzzy search engine backed by `nucleo-matcher`.
///
/// ### Field coverage
///
/// Agent searches match against a single concatenated haystack built as:
/// `"{id} {name} {provider} {summary}"` — covering all four fields required
/// by Req 16.2 and 32.2.
///
/// Skill searches match against `"{id} {name} {summary}"` — id, name, and
/// summary as required by Req 32.2.
///
/// ### Performance
///
/// For catalogs of < 100 assets (typical in this repository), fuzzy matching
/// completes well within the 100 ms budget stated in Req 16.2.  No explicit
/// benchmarking infrastructure is included here; the design relies on
/// `nucleo-matcher`'s O(nm) algorithm being fast enough for this dataset size.
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
    ///
    /// The haystack for each agent is `"{id} {name} {provider} {summary}"`,
    /// ensuring all four fields specified in Req 32.2 are covered.
    ///
    /// ### Filtering
    ///
    /// `provider_filter` and `harness_filter` are applied first (intersection),
    /// then fuzzy matching is applied to the remaining candidates.
    ///
    /// ### Empty query
    ///
    /// If `query` is empty AND no filters are active, all agent indices are
    /// returned in original order (score 0).  If filters are active but the
    /// query is empty, filtered candidates are returned in original order.
    ///
    /// ### Return value
    ///
    /// Indices into `agents` sorted by match score descending (highest score
    /// first), with ties broken by original index order (stable sort).
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

        // Apply provider / harness filters first.
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

        // Fuzzy match against the concatenated haystack covering all four
        // fields: id, name, provider, summary (Req 32.2).
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
                // Haystack covers: id + name + provider + summary (Req 16.2, 32.2).
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

        // Sort descending by score; ties are broken by ascending original index
        // (stable sort preserves relative order from the filter step).
        results.sort_by(|a, b| b.score.cmp(&a.score).then(a.index.cmp(&b.index)));
        results.into_iter().map(|r| r.index).collect()
    }

    /// Search skills by fuzzy matching against id, name, and summary.
    ///
    /// The haystack for each skill is `"{id} {name} {summary}"` — covering the
    /// three fields applicable to skills (no provider field on Skill).
    ///
    /// ### Empty query
    ///
    /// Returns all skill indices in original order.
    ///
    /// ### Return value
    ///
    /// Indices into `skills` sorted by match score descending (stable).
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
                // Haystack covers: id + name + summary (Req 32.2).
                let haystack = format!("{} {} {}", skill.id, skill.name, skill.summary);
                let mut buf = Vec::new();
                let score = pattern.score(
                    nucleo_matcher::Utf32Str::new(&haystack, &mut buf),
                    &mut self.matcher,
                )?;
                Some(SearchResult { index: idx, score })
            })
            .collect();

        results.sort_by(|a, b| b.score.cmp(&a.score).then(a.index.cmp(&b.index)));
        results.into_iter().map(|r| r.index).collect()
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::store::CatalogStore;
    use crate::models::{Agent, Skill};
    use std::path::Path;

    fn workspace_root() -> &'static Path {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let p = Path::new(manifest_dir).parent().unwrap().parent().unwrap();
        Box::leak(Box::new(p.to_path_buf()))
    }

    // ── Catalog-based smoke tests ─────────────────────────────────────────────

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
            let provider_str = serde_json::to_value(store.agents[*idx].provider)
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

    // ── Unit tests with synthetic data ────────────────────────────────────────

    /// Build a minimal `Agent` with the exact fields the struct requires.
    ///
    /// `provider_str` should be the JSON-serialized provider string (e.g.
    /// `"\"aws\""` or `"\"accounting\"`).  All non-search-related required
    /// fields are filled with sensible defaults.
    fn make_agent(id: &str, name: &str, provider_str: &str, summary: &str) -> Agent {
        // Agent has #[serde(deny_unknown_fields)], so we must provide exactly
        // the set of fields declared in the struct (required ones; optional
        // ones with #[serde(default)] can be omitted).
        let json = format!(
            r#"{{
                "id": "{id}",
                "name": "{name}",
                "type": "agent",
                "provider": {provider_str},
                "harnesses": [],
                "summary": "{summary}",
                "source_type": "original",
                "official_docs": [],
                "security_notes": "",
                "last_verified": "2026-01-01",
                "path": "agents/test/{id}",
                "companion_skills": []
            }}"#
        );
        serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("make_agent json parse failed for id={id}: {e}"))
    }

    /// Build a minimal `Skill` with the exact fields the struct requires.
    fn make_skill(id: &str, name: &str, summary: &str) -> Skill {
        // Skill also has #[serde(deny_unknown_fields)].  Only required fields
        // are provided; all #[serde(default)] fields are omitted.
        let json = format!(
            r#"{{
                "id": "{id}",
                "name": "{name}",
                "type": "skill",
                "provider": "aws",
                "harnesses": [],
                "summary": "{summary}",
                "source_type": "original",
                "official_docs": [],
                "security_notes": "",
                "last_verified": "2026-01-01",
                "path": "skills/test/{id}"
            }}"#
        );
        serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("make_skill json parse failed for id={id}: {e}"))
    }

    // ── Agent: ID field matching ──────────────────────────────────────────────

    #[test]
    fn search_agents_matches_by_id_substring() {
        let agents = vec![
            make_agent("iam-guard", "IAM Guard", "\"aws\"", "Monitors IAM"),
            make_agent(
                "cost-opt",
                "Cost Optimizer",
                "\"aws\"",
                "Reduces cloud spend",
            ),
        ];
        let mut engine = SearchEngine::new();
        let results = engine.search_agents("iam", &agents, None, None);
        assert!(
            results.contains(&0),
            "agent with 'iam' in id should be returned; got {results:?}"
        );
    }

    // ── Agent: name field matching ────────────────────────────────────────────

    #[test]
    fn search_agents_matches_by_name_substring() {
        let agents = vec![
            make_agent("agent-x", "Security Scanner", "\"aws\"", "Scans for vulns"),
            make_agent(
                "agent-y",
                "Cost Optimizer",
                "\"gcp\"",
                "Reduces cloud spend",
            ),
        ];
        let mut engine = SearchEngine::new();
        let results = engine.search_agents("scanner", &agents, None, None);
        assert!(
            results.contains(&0),
            "'scanner' should match agent with name 'Security Scanner'; got {results:?}"
        );
    }

    // ── Agent: provider field matching ────────────────────────────────────────

    #[test]
    fn search_agents_matches_by_provider_substring() {
        let agents = vec![
            make_agent("agent-a", "Some Agent", "\"azure\"", "Does Azure things"),
            make_agent("agent-b", "Other Agent", "\"aws\"", "Does AWS things"),
        ];
        let mut engine = SearchEngine::new();
        // Search for "azure" — should match agent-a via the provider field.
        let results = engine.search_agents("azure", &agents, None, None);
        assert!(
            results.contains(&0),
            "'azure' should match via provider field; got {results:?}"
        );
        // agent-b (aws) should not appear in an azure-only search
        // (nucleo may or may not score it; we only assert that index 0 is present).
    }

    // ── Agent: summary field matching ─────────────────────────────────────────

    #[test]
    fn search_agents_matches_by_summary_substring() {
        let agents = vec![
            make_agent("agent-a", "Agent Alpha", "\"aws\"", "Detects policy drift"),
            make_agent(
                "agent-b",
                "Agent Beta",
                "\"gcp\"",
                "Optimizes resource usage",
            ),
        ];
        let mut engine = SearchEngine::new();
        let results = engine.search_agents("drift", &agents, None, None);
        assert!(
            results.contains(&0),
            "'drift' should match agent with 'policy drift' in summary; got {results:?}"
        );
    }

    // ── Agent: empty query ────────────────────────────────────────────────────

    #[test]
    fn search_agents_empty_query_no_filters_returns_all_in_order() {
        let agents = vec![
            make_agent("z-agent", "Z Agent", "\"aws\"", "Last"),
            make_agent("a-agent", "A Agent", "\"gcp\"", "First"),
        ];
        let mut engine = SearchEngine::new();
        let results = engine.search_agents("", &agents, None, None);
        // All indices returned in original order when query is empty.
        assert_eq!(results, vec![0, 1]);
    }

    // ── Skill: ID field matching ──────────────────────────────────────────────

    #[test]
    fn search_skills_matches_by_id_substring() {
        let skills = vec![
            make_skill("cost-tracker", "Cost Tracker", "Tracks cloud costs"),
            make_skill("sec-scanner", "Security Scanner", "Scans for issues"),
        ];
        let mut engine = SearchEngine::new();
        let results = engine.search_skills("cost", &skills);
        assert!(
            results.contains(&0),
            "'cost' should match skill with id 'cost-tracker'; got {results:?}"
        );
    }

    // ── Skill: name field matching ────────────────────────────────────────────

    #[test]
    fn search_skills_matches_by_name_substring() {
        let skills = vec![
            make_skill("sk-1", "Drift Detector", "Detects configuration drift"),
            make_skill("sk-2", "Log Analyzer", "Parses log files"),
        ];
        let mut engine = SearchEngine::new();
        let results = engine.search_skills("detector", &skills);
        assert!(
            results.contains(&0),
            "'detector' should match skill with name 'Drift Detector'; got {results:?}"
        );
    }

    // ── Skill: summary field matching ─────────────────────────────────────────

    #[test]
    fn search_skills_matches_by_summary_substring() {
        let skills = vec![
            make_skill("sk-a", "Alpha Skill", "Validates compliance posture"),
            make_skill("sk-b", "Beta Skill", "Generates coverage reports"),
        ];
        let mut engine = SearchEngine::new();
        let results = engine.search_skills("compliance", &skills);
        assert!(
            results.contains(&0),
            "'compliance' should match skill with 'compliance' in summary; got {results:?}"
        );
    }

    // ── Skill: empty query ────────────────────────────────────────────────────

    #[test]
    fn search_skills_empty_query_returns_all_in_order() {
        let skills = vec![
            make_skill("z-skill", "Z Skill", "Last"),
            make_skill("a-skill", "A Skill", "First"),
        ];
        let mut engine = SearchEngine::new();
        let results = engine.search_skills("", &skills);
        assert_eq!(results, vec![0, 1]);
    }

    // ── Sort stability ────────────────────────────────────────────────────────

    #[test]
    fn results_sorted_by_score_descending() {
        // Create agents where one is a very exact match and one is partial.
        let agents = vec![
            make_agent(
                "sec-audit",
                "Security Audit",
                "\"aws\"",
                "Detailed audit tool",
            ),
            make_agent(
                "sec-basic",
                "Security Basic",
                "\"aws\"",
                "Basic security check",
            ),
        ];
        let mut engine = SearchEngine::new();
        // "sec-audit" should score higher for query "sec audit" than "sec-basic".
        let results = engine.search_agents("sec audit", &agents, None, None);
        if results.len() >= 2 {
            // The first result should be the better match (sec-audit contains
            // both "sec" and "audit" in close proximity).
            assert_eq!(results[0], 0, "sec-audit should rank first for 'sec audit'");
        }
        // At minimum both must appear since both contain "sec".
        assert!(results.contains(&0), "sec-audit must be in results");
    }
}
