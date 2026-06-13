//! Policy engine data models — rules, violations, evaluations, and suppressions.

use serde::{Deserialize, Serialize};

use super::agent::Lifecycle;

/// A single declarative policy rule loaded from `policies.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Unique rule identifier (e.g. `"require-security-scanner"`).
    pub id: String,
    /// The rule kind and its variant-specific parameters.
    pub rule_type: PolicyRuleType,
    /// How severe a violation of this rule is.
    pub severity: Severity,
    /// Which workspaces this rule applies to.
    pub scope: PolicyScope,
    /// Human-readable description of the rule's intent.
    pub description: String,
}

/// Discriminator for the kind of policy check being performed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PolicyRuleType {
    /// Require a specific asset to be installed.
    RequireAsset { asset_id: String },
    /// Require all agents from a role to be installed.
    RequireRole { role_id: String },
    /// Cap the number of stale (version-behind) assets.
    MaxStale { threshold: u32 },
    /// Enforce trust-boundary constraints on MCP references.
    TrustBoundary {
        max_mutation: bool,
        max_egress: bool,
        max_credentials: bool,
    },
    /// Enforce a minimum lifecycle stage on installed assets.
    LifecycleGate { min_stage: Lifecycle },
}

/// Violation severity level — determines grouping order in the dashboard.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Highest priority — blocks deployment in most policies.
    Critical,
    /// Medium priority — should be addressed but does not block.
    Warning,
    /// Low priority — informational only.
    Info,
}

/// Defines which workspaces a policy rule applies to.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyScope {
    /// Applies to every workspace.
    All,
    /// Applies to workspaces whose name matches the glob pattern.
    NamePattern(String),
    /// Applies only to workspaces belonging to the named team.
    Team(String),
}

/// A single rule violation discovered during a policy evaluation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolation {
    /// The rule that was violated.
    pub rule: PolicyRule,
    /// Name of the workspace where the violation was found.
    pub workspace: String,
    /// Asset identifier involved in the violation, if applicable.
    pub asset_id: Option<String>,
    /// ISO 8601 timestamp of when the violation was first detected.
    pub first_detected: String,
    /// Detailed description of the violation.
    pub details: String,
    /// Suggested remediation action.
    pub remediation: String,
}

/// Aggregate evaluation outcome for a single workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluation {
    /// Name of the workspace that was evaluated.
    pub workspace: String,
    /// Per-rule pass/fail results.
    pub results: Vec<RuleResult>,
    /// `(passed / total_applicable) × 100`, rounded half-up.
    pub compliance_score: f64,
}

/// Pass/fail result for a single rule within a `PolicyEvaluation`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleResult {
    /// Identifies the rule that was evaluated.
    pub rule_id: String,
    /// `true` if the rule passed; `false` if it was violated.
    pub passed: bool,
    /// Optional detail string (populated on failure or for informational rules).
    pub details: Option<String>,
}

/// A time-bounded exception that suppresses a specific violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suppression {
    /// ID of the rule being suppressed.
    pub rule_id: String,
    /// Name of the workspace for which the suppression applies.
    pub workspace: String,
    /// Human-readable justification.
    pub reason: String,
    /// Email address of the person who approved the suppression.
    pub approver: String,
    /// ISO 8601 date (`YYYY-MM-DD`) after which the suppression no longer applies.
    pub expires: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rule(severity: Severity) -> PolicyRule {
        PolicyRule {
            id: "test-rule".to_string(),
            rule_type: PolicyRuleType::RequireAsset {
                asset_id: "some-asset".to_string(),
            },
            severity,
            scope: PolicyScope::All,
            description: "Test rule".to_string(),
        }
    }

    #[test]
    fn severity_round_trip() {
        for s in [Severity::Critical, Severity::Warning, Severity::Info] {
            let json = serde_json::to_string(&s).unwrap();
            let decoded: Severity = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded, s);
        }
    }

    #[test]
    fn severity_serialized_strings() {
        assert_eq!(
            serde_json::to_string(&Severity::Critical).unwrap(),
            "\"critical\""
        );
        assert_eq!(
            serde_json::to_string(&Severity::Warning).unwrap(),
            "\"warning\""
        );
        assert_eq!(serde_json::to_string(&Severity::Info).unwrap(), "\"info\"");
    }

    #[test]
    fn severity_ordering() {
        // Critical < Warning < Info in Ord (lower = higher priority is intentional —
        // callers sort ascending to put Critical first).
        assert!(Severity::Critical < Severity::Warning);
        assert!(Severity::Warning < Severity::Info);
    }

    #[test]
    fn policy_rule_round_trip() {
        let rule = make_rule(Severity::Critical);
        let json = serde_json::to_string(&rule).unwrap();
        let decoded: PolicyRule = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.id, "test-rule");
        assert_eq!(decoded.severity, Severity::Critical);
    }

    #[test]
    fn policy_rule_type_require_role_round_trip() {
        let rt = PolicyRuleType::RequireRole {
            role_id: "cloud-security-engineer".to_string(),
        };
        let json = serde_json::to_string(&rt).unwrap();
        let decoded: PolicyRuleType = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, rt);
    }

    #[test]
    fn policy_rule_type_max_stale_round_trip() {
        let rt = PolicyRuleType::MaxStale { threshold: 5 };
        let json = serde_json::to_string(&rt).unwrap();
        let decoded: PolicyRuleType = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, rt);
    }

    #[test]
    fn policy_rule_type_trust_boundary_round_trip() {
        let rt = PolicyRuleType::TrustBoundary {
            max_mutation: false,
            max_egress: true,
            max_credentials: true,
        };
        let json = serde_json::to_string(&rt).unwrap();
        let decoded: PolicyRuleType = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, rt);
    }

    #[test]
    fn policy_rule_type_lifecycle_gate_round_trip() {
        let rt = PolicyRuleType::LifecycleGate {
            min_stage: Lifecycle::Stable,
        };
        let json = serde_json::to_string(&rt).unwrap();
        let decoded: PolicyRuleType = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, rt);
    }

    #[test]
    fn policy_scope_round_trip() {
        for scope in [
            PolicyScope::All,
            PolicyScope::NamePattern("production-*".to_string()),
            PolicyScope::Team("platform-security".to_string()),
        ] {
            let json = serde_json::to_string(&scope).unwrap();
            let decoded: PolicyScope = serde_json::from_str(&json).unwrap();
            // Compare via serialized form (PolicyScope doesn't derive PartialEq for simplicity).
            assert_eq!(
                serde_json::to_string(&decoded).unwrap(),
                serde_json::to_string(&scope).unwrap()
            );
        }
    }

    #[test]
    fn policy_violation_round_trip() {
        let v = PolicyViolation {
            rule: make_rule(Severity::Warning),
            workspace: "prod".to_string(),
            asset_id: Some("aws-iam-scanner".to_string()),
            first_detected: "2025-01-01T00:00:00.000Z".to_string(),
            details: "Asset not found".to_string(),
            remediation: "Install the asset".to_string(),
        };
        let json = serde_json::to_string(&v).unwrap();
        let decoded: PolicyViolation = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.workspace, "prod");
        assert_eq!(decoded.asset_id, Some("aws-iam-scanner".to_string()));
    }

    #[test]
    fn policy_evaluation_round_trip() {
        let eval = PolicyEvaluation {
            workspace: "staging".to_string(),
            results: vec![RuleResult {
                rule_id: "test-rule".to_string(),
                passed: true,
                details: None,
            }],
            compliance_score: 100.0,
        };
        let json = serde_json::to_string(&eval).unwrap();
        let decoded: PolicyEvaluation = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.workspace, "staging");
        assert!((decoded.compliance_score - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn suppression_round_trip() {
        let s = Suppression {
            rule_id: "no-mutation-mcp".to_string(),
            workspace: "staging-infra".to_string(),
            reason: "Approved exception".to_string(),
            approver: "security-lead@example.com".to_string(),
            expires: "2025-06-01".to_string(),
        };
        let json = serde_json::to_string(&s).unwrap();
        let decoded: Suppression = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.rule_id, "no-mutation-mcp");
        assert_eq!(decoded.expires, "2025-06-01");
    }
}
