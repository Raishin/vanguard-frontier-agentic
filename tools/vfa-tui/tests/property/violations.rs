//! Feature: rust-tui-v2, Property 28 — violations aggregation & ranking.
//! Validates: Requirements 11.3, 15.1, 15.4
//!
//! `aggregate_violations` produces a deterministic dashboard: violations sorted
//! by (severity, workspace), workspaces ranked worst-compliance-first with
//! alphabetic tie-breaks, and `resolve_violations` reports exactly the
//! remediated keys.

use proptest::prelude::*;
use proptest::test_runner::Config;

use vfa_tui::models::policy::{
    PolicyEvaluation, PolicyRule, PolicyRuleType, PolicyScope, PolicyViolation, Severity,
};
use vfa_tui::policy::violations::{
    aggregate_violations, rank_workspaces_by_compliance, resolve_violations,
};

fn arb_severity() -> impl Strategy<Value = Severity> {
    prop_oneof![
        Just(Severity::Critical),
        Just(Severity::Warning),
        Just(Severity::Info),
    ]
}

fn violation(rule_id: &str, severity: Severity, workspace: &str, asset: Option<&str>) -> PolicyViolation {
    PolicyViolation {
        rule: PolicyRule {
            id: rule_id.to_string(),
            rule_type: PolicyRuleType::MaxStale { threshold: 1 },
            severity,
            scope: PolicyScope::All,
            description: String::new(),
        },
        workspace: workspace.to_string(),
        asset_id: asset.map(|s| s.to_string()),
        first_detected: "2026-01-01".to_string(),
        details: String::new(),
        remediation: String::new(),
    }
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Aggregated violations are sorted by (severity, workspace) regardless of
    /// input order.
    #[test]
    fn violations_sorted_by_severity_then_workspace(
        raw in proptest::collection::vec(
            (arb_severity(), "[a-z]{1,6}"), 0..20)
    ) {
        let violations: Vec<PolicyViolation> = raw.iter().enumerate()
            .map(|(i, (sev, ws))| violation(&format!("r{i}"), sev.clone(), ws, None))
            .collect();
        let dash = aggregate_violations(&[], &violations);

        // Output is a permutation of the input (same count).
        prop_assert_eq!(dash.violations.len(), violations.len());

        // Output is sorted by (severity, workspace).
        for w in dash.violations.windows(2) {
            let a = (&w[0].rule.severity, &w[0].workspace);
            let b = (&w[1].rule.severity, &w[1].workspace);
            prop_assert!(a <= b, "violations not sorted: {:?} then {:?}", a, b);
        }
    }

    /// `rank_workspaces_by_compliance` lists every workspace exactly once,
    /// ascending by (1dp-rounded) score with alphabetic tie-breaks.
    #[test]
    fn ranking_is_ascending_permutation(
        entries in proptest::collection::hash_map("[a-z]{1,6}", 0.0f64..=100.0, 0..15)
    ) {
        let ranked = rank_workspaces_by_compliance(&entries);

        // Permutation: same set of names.
        prop_assert_eq!(ranked.len(), entries.len());
        let ranked_names: std::collections::HashSet<&String> = ranked.iter().map(|(n, _)| n).collect();
        prop_assert_eq!(ranked_names.len(), entries.len());

        // Ascending by rounded score, ties alphabetical.
        for w in ranked.windows(2) {
            let a = ((w[0].1 * 10.0).round() as i64, &w[0].0);
            let b = ((w[1].1 * 10.0).round() as i64, &w[1].0);
            prop_assert!(a <= b, "ranking not ordered: {:?} then {:?}", a, b);
        }
    }

    /// Workspace scores in the dashboard mirror the per-workspace evaluations.
    #[test]
    fn workspace_scores_mirror_evaluations(
        evals in proptest::collection::vec(("[a-z]{1,6}", 0.0f64..=100.0), 0..10)
    ) {
        let mut seen = std::collections::HashSet::new();
        let evals: Vec<PolicyEvaluation> = evals.into_iter()
            .filter(|(name, _)| seen.insert(name.clone()))
            .map(|(name, score)| PolicyEvaluation {
                workspace: name,
                results: vec![],
                compliance_score: score,
            })
            .collect();
        let dash = aggregate_violations(&evals, &[]);
        prop_assert_eq!(dash.workspace_scores.len(), evals.len());
        for e in &evals {
            prop_assert_eq!(dash.workspace_scores.get(&e.workspace), Some(&e.compliance_score));
        }
    }

    /// `resolve_violations` returns exactly the prev keys not still violated.
    #[test]
    fn resolve_returns_remediated_keys(
        prev in proptest::collection::vec("[a-z]{1,4}", 0..12),
        still in proptest::collection::vec("[a-z]{1,4}", 0..12),
    ) {
        // Build prev violations with unique (rule_id) keys, workspace fixed.
        let mut seen = std::collections::HashSet::new();
        let prev_v: Vec<PolicyViolation> = prev.iter()
            .filter(|id| seen.insert((*id).clone()))
            .map(|id| violation(id, Severity::Warning, "ws", None))
            .collect();
        let now_v: Vec<PolicyViolation> = still.iter()
            .map(|id| violation(id, Severity::Warning, "ws", None))
            .collect();
        let still_set: std::collections::HashSet<&String> = still.iter().collect();

        let resolved = resolve_violations(&prev_v, &now_v, "2026-06-14");
        let resolved_ids: std::collections::HashSet<String> =
            resolved.iter().map(|r| r.violation.rule.id.clone()).collect();

        for v in &prev_v {
            let expected_resolved = !still_set.contains(&v.rule.id);
            prop_assert_eq!(resolved_ids.contains(&v.rule.id), expected_resolved,
                "rule {} resolution mismatch", v.rule.id);
        }
    }
}
