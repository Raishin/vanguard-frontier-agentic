//! Violations aggregation — group, rank, and resolve policy violations.
//!
//! Req 15.1–15.7.

#![deny(warnings)]

use std::collections::HashMap;

use crate::models::policy::{PolicyEvaluation, PolicyViolation, Severity};

// ---------------------------------------------------------------------------
// ViolationsDashboard
// ---------------------------------------------------------------------------

/// Aggregated violations across all workspaces, grouped for the dashboard view.
///
/// Grouped: Critical → Warning → Info, then by workspace within each severity
/// (Req 15.1).
#[derive(Debug, Clone, Default)]
pub struct ViolationsDashboard {
    /// All active violations, sorted by severity (Critical first) then workspace name.
    pub violations: Vec<PolicyViolation>,
    /// Per-workspace compliance scores (workspace name → score).
    pub workspace_scores: HashMap<String, f64>,
    /// Workspaces ranked worst-first by compliance score (ascending).
    pub ranked_workspaces: Vec<(String, f64)>,
}

// ---------------------------------------------------------------------------
// ResolvedViolation
// ---------------------------------------------------------------------------

/// A violation that has been remediated; carries an audit note.
#[derive(Debug, Clone)]
pub struct ResolvedViolation {
    /// The original violation.
    pub violation: PolicyViolation,
    /// ISO 8601 timestamp when the remediation was detected.
    pub resolved_at: String,
    /// Audit event describing the resolution.
    pub audit_note: String,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Aggregate per-workspace evaluations into a [`ViolationsDashboard`].
///
/// Violations are sorted by severity (Critical < Warning < Info in `Severity::Ord`)
/// then by workspace name (alphabetic, stable).  This ensures deterministic
/// output for any input order (Req 11.3, Req 15.1).
pub fn aggregate_violations(
    per_workspace_evals: &[PolicyEvaluation],
    all_violations: &[PolicyViolation],
) -> ViolationsDashboard {
    // Collect compliance scores
    let workspace_scores: HashMap<String, f64> = per_workspace_evals
        .iter()
        .map(|e| (e.workspace.clone(), e.compliance_score))
        .collect();

    // Sort violations: by severity (Critical first) then by workspace name.
    let mut sorted_violations = all_violations.to_vec();
    sorted_violations.sort_by(|a, b| {
        let sev_cmp = a.rule.severity.cmp(&b.rule.severity);
        if sev_cmp != std::cmp::Ordering::Equal {
            return sev_cmp;
        }
        a.workspace.cmp(&b.workspace)
    });

    let ranked = rank_workspaces_by_compliance(&workspace_scores);

    ViolationsDashboard {
        violations: sorted_violations,
        workspace_scores,
        ranked_workspaces: ranked,
    }
}

/// Rank workspaces by compliance score ascending (worst first).
///
/// On ties, workspaces are sorted alphabetically by name (stable, deterministic,
/// Req 15.4).
pub fn rank_workspaces_by_compliance(
    workspace_scores: &HashMap<String, f64>,
) -> Vec<(String, f64)> {
    let mut ranked: Vec<(String, f64)> = workspace_scores
        .iter()
        .map(|(name, score)| (name.clone(), *score))
        .collect();

    // Sort ascending by score; ties broken alphabetically by workspace name.
    ranked.sort_by(|a, b| {
        // Compare scores with 1-decimal precision to match compliance_score rounding
        let a_score = (a.1 * 10.0).round() as i64;
        let b_score = (b.1 * 10.0).round() as i64;
        match a_score.cmp(&b_score) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            other => other,
        }
    });

    ranked
}

/// Identify violations that have been remediated.
///
/// A violation is "resolved" when its `(rule_id, workspace, asset_id)` key is
/// present in `prev_active` but absent from `now_satisfied`.
/// `now_satisfied` lists the set of keys that **currently violate** rules; any
/// key no longer present means it was remediated.
///
/// Returns [`ResolvedViolation`] records (with a synthetic audit note) for
/// every remediated violation.
pub fn resolve_violations(
    prev_active: &[PolicyViolation],
    now_violated: &[PolicyViolation],
    resolved_at: &str,
) -> Vec<ResolvedViolation> {
    // Build a set of (rule_id, workspace, asset_id) that are STILL violated.
    let still_violated: std::collections::HashSet<(String, String, Option<String>)> =
        now_violated
            .iter()
            .map(|v| (v.rule.id.clone(), v.workspace.clone(), v.asset_id.clone()))
            .collect();

    prev_active
        .iter()
        .filter(|v| {
            !still_violated.contains(&(v.rule.id.clone(), v.workspace.clone(), v.asset_id.clone()))
        })
        .map(|v| ResolvedViolation {
            violation: v.clone(),
            resolved_at: resolved_at.to_string(),
            audit_note: format!(
                "violation_resolved: rule='{}' workspace='{}' asset={:?} resolved_at='{}'",
                v.rule.id, v.workspace, v.asset_id, resolved_at
            ),
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Helper: severity display order (for tests / comments)
// ---------------------------------------------------------------------------

/// Returns a static slice of severities in display order (Critical first).
#[allow(dead_code)]
fn severity_display_order() -> &'static [Severity] {
    &[Severity::Critical, Severity::Warning, Severity::Info]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::policy::{PolicyRule, PolicyRuleType, PolicyScope, PolicyViolation, RuleResult, Severity};

    fn make_eval(workspace: &str, passed: usize, total: usize) -> PolicyEvaluation {
        let score = if total == 0 {
            100.0
        } else {
            ((passed as f64 / total as f64) * 10000.0).round() / 100.0
        };
        PolicyEvaluation {
            workspace: workspace.to_string(),
            results: (0..total)
                .map(|i| RuleResult {
                    rule_id: format!("rule-{i}"),
                    passed: i < passed,
                    details: None,
                })
                .collect(),
            compliance_score: score,
        }
    }

    fn make_violation(workspace: &str, severity: Severity, rule_id: &str, asset_id: Option<&str>) -> PolicyViolation {
        PolicyViolation {
            rule: PolicyRule {
                id: rule_id.to_string(),
                rule_type: PolicyRuleType::MaxStale { threshold: 0 },
                severity,
                scope: PolicyScope::All,
                description: String::new(),
            },
            workspace: workspace.to_string(),
            asset_id: asset_id.map(|s| s.to_string()),
            first_detected: "2026-01-01T00:00:00Z".to_string(),
            details: String::new(),
            remediation: String::new(),
        }
    }

    // -----------------------------------------------------------------------
    // aggregate_violations
    // -----------------------------------------------------------------------

    #[test]
    fn aggregate_empty() {
        let dashboard = aggregate_violations(&[], &[]);
        assert!(dashboard.violations.is_empty());
        assert!(dashboard.workspace_scores.is_empty());
        assert!(dashboard.ranked_workspaces.is_empty());
    }

    #[test]
    fn aggregate_violations_sorted_by_severity_then_workspace() {
        let evals = vec![
            make_eval("alpha", 0, 1),
            make_eval("beta", 1, 2),
        ];
        let violations = vec![
            make_violation("beta", Severity::Warning, "w1", None),
            make_violation("alpha", Severity::Critical, "c1", None),
            make_violation("alpha", Severity::Warning, "w2", None),
            make_violation("beta", Severity::Info, "i1", None),
        ];
        let dash = aggregate_violations(&evals, &violations);

        // First should be Critical (alpha), then Warning (alpha, beta), then Info (beta)
        assert_eq!(dash.violations[0].rule.severity, Severity::Critical);
        assert_eq!(dash.violations[0].workspace, "alpha");
        assert_eq!(dash.violations[1].rule.severity, Severity::Warning);
        assert_eq!(dash.violations[1].workspace, "alpha");
        assert_eq!(dash.violations[2].rule.severity, Severity::Warning);
        assert_eq!(dash.violations[2].workspace, "beta");
        assert_eq!(dash.violations[3].rule.severity, Severity::Info);
    }

    // -----------------------------------------------------------------------
    // rank_workspaces_by_compliance
    // -----------------------------------------------------------------------

    #[test]
    fn rank_workspaces_ascending() {
        let mut scores = HashMap::new();
        scores.insert("alpha".to_string(), 100.0);
        scores.insert("beta".to_string(), 50.0);
        scores.insert("gamma".to_string(), 75.0);

        let ranked = rank_workspaces_by_compliance(&scores);
        // Ascending: beta(50) < gamma(75) < alpha(100)
        assert_eq!(ranked[0].0, "beta");
        assert_eq!(ranked[1].0, "gamma");
        assert_eq!(ranked[2].0, "alpha");
    }

    #[test]
    fn rank_workspaces_tie_broken_by_name() {
        let mut scores = HashMap::new();
        scores.insert("z-ws".to_string(), 50.0);
        scores.insert("a-ws".to_string(), 50.0);

        let ranked = rank_workspaces_by_compliance(&scores);
        // Same score → alphabetical: a-ws < z-ws
        assert_eq!(ranked[0].0, "a-ws");
        assert_eq!(ranked[1].0, "z-ws");
    }

    // -----------------------------------------------------------------------
    // resolve_violations
    // -----------------------------------------------------------------------

    #[test]
    fn resolve_detects_remediated_violation() {
        let v = make_violation("prod", Severity::Critical, "r1", Some("asset-a"));
        let prev = vec![v.clone()];
        let now: Vec<PolicyViolation> = vec![]; // remediated

        let resolved = resolve_violations(&prev, &now, "2026-06-13T12:00:00Z");
        assert_eq!(resolved.len(), 1);
        assert!(resolved[0].audit_note.contains("violation_resolved"));
        assert!(resolved[0].audit_note.contains("r1"));
    }

    #[test]
    fn resolve_skips_still_active_violations() {
        let v = make_violation("prod", Severity::Critical, "r1", Some("asset-a"));
        let prev = vec![v.clone()];
        let now = vec![v.clone()]; // still active

        let resolved = resolve_violations(&prev, &now, "2026-06-13T12:00:00Z");
        assert!(resolved.is_empty());
    }

    #[test]
    fn resolve_handles_empty_prev() {
        let resolved = resolve_violations(&[], &[], "2026-06-13T12:00:00Z");
        assert!(resolved.is_empty());
    }

    // -----------------------------------------------------------------------
    // Property 28 (Req 15.1, 15.4): grouping/ranking invariants
    // -----------------------------------------------------------------------

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(256))]

        #[test]
        fn prop28a_violations_sorted_severity_then_workspace(
            workspaces in proptest::collection::vec("[a-z][a-z0-9]{1,8}", 1..5),
            severities in proptest::collection::vec(0usize..3, 1..8),
        ) {
            let sev_arr = [Severity::Critical, Severity::Warning, Severity::Info];
            let violations: Vec<PolicyViolation> = severities
                .iter()
                .enumerate()
                .map(|(i, &sev_idx)| {
                    let ws = &workspaces[i % workspaces.len()];
                    make_violation(ws, sev_arr[sev_idx].clone(), &format!("r{i}"), None)
                })
                .collect();

            let evals: Vec<PolicyEvaluation> = workspaces
                .iter()
                .map(|ws| make_eval(ws, 0, 1))
                .collect();

            let dash = aggregate_violations(&evals, &violations);

            // Verify sorted: every adjacent pair respects severity order then workspace order
            for w in dash.violations.windows(2) {
                let a = &w[0];
                let b = &w[1];
                let sev_a = a.rule.severity.clone();
                let sev_b = b.rule.severity.clone();
                proptest::prop_assert!(
                    sev_a < sev_b || (sev_a == sev_b && a.workspace <= b.workspace),
                    "violations not sorted: ({:?},{}) > ({:?},{})",
                    sev_a, a.workspace, sev_b, b.workspace
                );
            }
        }

        #[test]
        fn prop28b_ranking_ascending_by_score(
            workspace_names in proptest::collection::vec("[a-z][a-z0-9]{1,8}", 1..6),
            scores_raw in proptest::collection::vec(0u32..=100, 1..6),
        ) {
            let n = workspace_names.len().min(scores_raw.len());
            let mut score_map = HashMap::new();
            for i in 0..n {
                score_map.insert(workspace_names[i].clone(), scores_raw[i] as f64);
            }

            let ranked = rank_workspaces_by_compliance(&score_map);

            // Verify ascending order: every adjacent pair has score[i] <= score[i+1]
            for w in ranked.windows(2) {
                let (name_a, score_a) = &w[0];
                let (name_b, score_b) = &w[1];
                let sa = (score_a * 10.0).round() as i64;
                let sb = (score_b * 10.0).round() as i64;
                proptest::prop_assert!(
                    sa < sb || (sa == sb && name_a <= name_b),
                    "ranking not ascending: ({},{}) > ({},{})",
                    name_a, score_a, name_b, score_b
                );
            }

            // Total count matches input
            proptest::prop_assert_eq!(ranked.len(), score_map.len());
        }
    }
}
