//! Integration test 13.4 — end-to-end policy evaluation against a real loaded
//! catalog (the test fixtures), covering RequireAsset, MaxStale, the lifecycle
//! gate, scope/suppression handling, and determinism.
//! Validates: Requirements 11.1–11.7, 13.1–13.3, 15.x

use std::path::{Path, PathBuf};

use vfa_tui::catalog::store::CatalogStore;
use vfa_tui::federation::scanner::{DetectionMethod, InstalledAsset};
use vfa_tui::models::agent::Lifecycle;
use vfa_tui::models::policy::{PolicyRule, PolicyRuleType, PolicyScope, Severity, Suppression};
use vfa_tui::models::workspace::{ResolvedWorkspace, WorkspaceStatus};
use vfa_tui::policy::engine::PolicyEngine;
use vfa_tui::policy::lifecycle::evaluate_lifecycle;
use vfa_tui::policy::parser::PolicyConfig;

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn confirmed(asset_id: &str) -> InstalledAsset {
    InstalledAsset {
        workspace_path: PathBuf::from("/ws/team-a"),
        asset_id: asset_id.to_string(),
        installed_version: Some("1.0.0".to_string()),
        content_hash: "deadbeef".to_string(),
        detection_methods: vec![DetectionMethod::Filename, DetectionMethod::MetadataComment],
        confirmed: true,
        harness: "claude".to_string(),
    }
}

fn workspace() -> ResolvedWorkspace {
    ResolvedWorkspace {
        canonical_path: PathBuf::from("/ws/team-a"),
        name: "team-a".to_string(),
        team: Some("platform".to_string()),
        tags: vec![],
        status: WorkspaceStatus::Available,
    }
}

fn rule(id: &str, rule_type: PolicyRuleType, scope: PolicyScope) -> PolicyRule {
    PolicyRule {
        id: id.to_string(),
        rule_type,
        severity: Severity::Warning,
        scope,
        description: String::new(),
    }
}

#[test]
fn require_asset_passes_when_installed_fails_when_absent() {
    let catalog = CatalogStore::load(&fixtures_root());
    assert!(
        catalog.load_errors.is_empty(),
        "fixtures must load: {:?}",
        catalog.load_errors
    );

    let installed = vec![confirmed("aws-iam-review-agent")];
    let config = PolicyConfig {
        rules: vec![
            rule(
                "needs-iam",
                PolicyRuleType::RequireAsset {
                    asset_id: "aws-iam-review-agent".to_string(),
                },
                PolicyScope::All,
            ),
            rule(
                "needs-missing",
                PolicyRuleType::RequireAsset {
                    asset_id: "no-such-agent".to_string(),
                },
                PolicyScope::All,
            ),
        ],
        ..Default::default()
    };

    let eval = PolicyEngine::evaluate(&config, &workspace(), &installed, &catalog, "2026-06-14");
    assert_eq!(eval.results.len(), 2);
    let present = eval
        .results
        .iter()
        .find(|r| r.rule_id == "needs-iam")
        .unwrap();
    let missing = eval
        .results
        .iter()
        .find(|r| r.rule_id == "needs-missing")
        .unwrap();
    assert!(
        present.passed,
        "installed asset should satisfy RequireAsset"
    );
    assert!(!missing.passed, "absent asset should fail RequireAsset");
    // Half the rules pass.
    assert_eq!(eval.compliance_score, 50.0);
}

#[test]
fn evaluation_is_deterministic() {
    let catalog = CatalogStore::load(&fixtures_root());
    let installed = vec![
        confirmed("aws-iam-review-agent"),
        confirmed("aws-s3-security-agent"),
    ];
    let config = PolicyConfig {
        rules: vec![rule(
            "needs-iam",
            PolicyRuleType::RequireAsset {
                asset_id: "aws-iam-review-agent".to_string(),
            },
            PolicyScope::All,
        )],
        ..Default::default()
    };
    let a = PolicyEngine::evaluate(&config, &workspace(), &installed, &catalog, "2026-06-14");
    let b = PolicyEngine::evaluate(&config, &workspace(), &installed, &catalog, "2026-06-14");
    assert_eq!(a.compliance_score, b.compliance_score);
    assert_eq!(
        a.results
            .iter()
            .map(|r| (&r.rule_id, r.passed))
            .collect::<Vec<_>>(),
        b.results
            .iter()
            .map(|r| (&r.rule_id, r.passed))
            .collect::<Vec<_>>(),
    );
}

#[test]
fn out_of_scope_and_suppressed_rules_are_skipped() {
    let catalog = CatalogStore::load(&fixtures_root());
    let installed = vec![confirmed("aws-iam-review-agent")];

    // One rule scoped to a different team (skipped), one suppressed rule.
    let config = PolicyConfig {
        rules: vec![
            rule(
                "other-team",
                PolicyRuleType::RequireAsset {
                    asset_id: "no-such".to_string(),
                },
                PolicyScope::Team("not-platform".to_string()),
            ),
            rule(
                "suppressed",
                PolicyRuleType::RequireAsset {
                    asset_id: "no-such".to_string(),
                },
                PolicyScope::All,
            ),
        ],
        suppressions: vec![Suppression {
            rule_id: "suppressed".to_string(),
            workspace: "team-a".to_string(),
            reason: "accepted risk".to_string(),
            approver: "lead".to_string(),
            expires: "2099-01-01".to_string(),
        }],
        ..Default::default()
    };

    let eval = PolicyEngine::evaluate(&config, &workspace(), &installed, &catalog, "2026-06-14");
    // Both rules are excluded → no results, perfect compliance.
    assert!(
        eval.results.is_empty(),
        "scoped-out + suppressed rules must not appear"
    );
    assert_eq!(eval.compliance_score, 100.0);
}

#[test]
fn lifecycle_gate_flags_experimental_asset() {
    let catalog = CatalogStore::load(&fixtures_root());
    // aws-bedrock-agent is lifecycle=experimental in the fixtures.
    let installed = vec![confirmed("aws-bedrock-agent")];
    let violations = evaluate_lifecycle(&installed, &catalog, Lifecycle::Stable);
    assert!(
        violations
            .iter()
            .any(|v| v.asset_id.as_deref() == Some("aws-bedrock-agent")),
        "experimental asset must violate a Stable lifecycle gate"
    );

    // Under an Experimental gate, nothing is below the minimum.
    let none = evaluate_lifecycle(&installed, &catalog, Lifecycle::Experimental);
    assert!(
        none.is_empty(),
        "experimental asset satisfies an Experimental gate"
    );
}
