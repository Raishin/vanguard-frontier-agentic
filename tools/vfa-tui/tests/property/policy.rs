//! Feature: rust-tui-v2, Properties 22, 23, 24 — policy determinism, scope
//! matching, and lifecycle/severity ordering.
//! Validates: Requirements 11.3, 11.6, 15.3, 15.6
//!
//! P22: `compliance_score` and `is_suppressed` are deterministic and obey their
//!      range / expiry contracts.
//! P23: `rule_applies` matches All / NamePattern / Team scopes exactly.
//! P24: lifecycle and severity orderings are total and self-consistent — the
//!      backbone of lifecycle gating and violation ranking.
//!
//! The catalog-dependent `PolicyEngine::evaluate` end-to-end path is exercised
//! by the integration test `policy_evaluation.rs`.

use std::path::PathBuf;

use proptest::prelude::*;
use proptest::test_runner::Config;

use vfa_tui::models::agent::Lifecycle;
use vfa_tui::models::policy::{PolicyRule, PolicyRuleType, PolicyScope, Severity, Suppression};
use vfa_tui::models::workspace::{ResolvedWorkspace, WorkspaceStatus};
use vfa_tui::policy::engine::{lifecycle_below, lifecycle_rank, PolicyEngine};

fn workspace(name: &str, team: Option<&str>) -> ResolvedWorkspace {
    ResolvedWorkspace {
        canonical_path: PathBuf::from(format!("/ws/{name}")),
        name: name.to_string(),
        team: team.map(|s| s.to_string()),
        tags: vec![],
        status: WorkspaceStatus::Available,
    }
}

fn rule(id: &str, scope: PolicyScope) -> PolicyRule {
    PolicyRule {
        id: id.to_string(),
        rule_type: PolicyRuleType::MaxStale { threshold: 2 },
        severity: Severity::Warning,
        scope,
        description: "test".to_string(),
    }
}

fn config(rules: Vec<PolicyRule>, suppressions: Vec<Suppression>) -> vfa_tui::policy::parser::PolicyConfig {
    vfa_tui::policy::parser::PolicyConfig {
        rules,
        suppressions,
        ..Default::default()
    }
}

const LIFECYCLES: [Lifecycle; 4] = [
    Lifecycle::Experimental,
    Lifecycle::Beta,
    Lifecycle::Stable,
    Lifecycle::Deprecated,
];

proptest! {
    #![proptest_config(Config::with_cases(256))]

    // ---- P23: scope matching -----------------------------------------------

    /// `PolicyScope::All` applies to every workspace.
    #[test]
    fn scope_all_applies_everywhere(name in "[a-z0-9-]{1,16}", team in proptest::option::of("[a-z]{1,8}")) {
        let r = rule("r", PolicyScope::All);
        prop_assert!(PolicyEngine::rule_applies(&r, &workspace(&name, team.as_deref())));
    }

    /// `NamePattern` applies iff the glob matches the workspace name.
    #[test]
    fn scope_name_pattern_matches_glob(prefix in "[a-z]{1,6}", name in "[a-z0-9-]{1,16}") {
        let pat = format!("{prefix}*");
        let r = rule("r", PolicyScope::NamePattern(pat.clone()));
        let applies = PolicyEngine::rule_applies(&r, &workspace(&name, None));
        prop_assert_eq!(applies, name.starts_with(&prefix));
    }

    /// `Team` applies iff the workspace declares exactly that team.
    #[test]
    fn scope_team_matches_exact(team in "[a-z]{1,8}", ws_team in proptest::option::of("[a-z]{1,8}")) {
        let r = rule("r", PolicyScope::Team(team.clone()));
        let applies = PolicyEngine::rule_applies(&r, &workspace("ws", ws_team.as_deref()));
        prop_assert_eq!(applies, ws_team.as_deref() == Some(team.as_str()));
    }

    // ---- P22: determinism & contracts --------------------------------------

    /// `compliance_score` is in [0, 100], is 100 for an empty rule set, is
    /// monotonic in `passed`, and is deterministic.
    #[test]
    fn compliance_score_contract(total in 0usize..500, passed in 0usize..500) {
        let passed = passed.min(total);
        let s = PolicyEngine::compliance_score(passed, total);
        prop_assert!((0.0..=100.0).contains(&s));
        if total == 0 {
            prop_assert_eq!(s, 100.0);
        }
        // Deterministic.
        prop_assert_eq!(PolicyEngine::compliance_score(passed, total), s);
        // Monotonic in passed.
        if total > 0 && passed < total {
            prop_assert!(PolicyEngine::compliance_score(passed + 1, total) >= s);
        }
    }

    /// `is_suppressed` is true iff a suppression matches the rule+workspace and
    /// has not expired as of `now_date` (empty `expires` = never expires).
    #[test]
    fn is_suppressed_respects_match_and_expiry(
        expires in proptest::option::of("20[0-9]{2}-[0-1][0-9]-[0-3][0-9]"),
        now in "20[0-9]{2}-[0-1][0-9]-[0-3][0-9]",
        right_rule in any::<bool>(),
        right_ws in any::<bool>(),
    ) {
        let r = rule("rule-1", PolicyScope::All);
        let ws = workspace("ws-1", None);
        let supp = Suppression {
            rule_id: if right_rule { "rule-1" } else { "other" }.to_string(),
            workspace: if right_ws { "ws-1" } else { "other-ws" }.to_string(),
            reason: "r".into(),
            approver: "a".into(),
            expires: expires.clone().unwrap_or_default(),
        };
        let cfg = config(vec![r.clone()], vec![supp]);
        let got = PolicyEngine::is_suppressed(&r, &ws, &cfg, &now);
        let not_expired = match &expires {
            None => true, // empty string = never expires
            Some(e) => e.as_str() >= now.as_str(),
        };
        let expected = right_rule && right_ws && not_expired;
        prop_assert_eq!(got, expected);
    }

    // ---- P24: lifecycle & severity ordering --------------------------------

    /// `lifecycle_below` is a strict order: never reflexive, and consistent with
    /// the documented Experimental < Beta < Stable < Deprecated ranking.
    #[test]
    fn lifecycle_order_is_total_and_strict(i in 0usize..4, j in 0usize..4) {
        let a = LIFECYCLES[i];
        let b = LIFECYCLES[j];
        prop_assert_eq!(lifecycle_below(a, b), lifecycle_rank(a) < lifecycle_rank(b));
        prop_assert!(!lifecycle_below(a, a)); // irreflexive
        if lifecycle_below(a, b) {
            prop_assert!(!lifecycle_below(b, a)); // asymmetric
        }
    }
}

#[test]
fn severity_orders_critical_first() {
    // Critical < Warning < Info (used for severity-ranked dashboards).
    assert!(Severity::Critical < Severity::Warning);
    assert!(Severity::Warning < Severity::Info);
}

#[test]
fn lifecycle_rank_is_strictly_increasing() {
    let ranks: Vec<u8> = LIFECYCLES.iter().map(|l| lifecycle_rank(*l)).collect();
    assert_eq!(ranks, vec![0, 1, 2, 3]);
}
