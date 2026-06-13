//! Policy TOML parser — loads `policies.toml` into [`PolicyConfig`].
//!
//! Missing file → empty config (no error, Req 11.5).
//! Syntax errors → collect with line number, skip malformed rules (Req 25.3).
//! Validate rules against catalog (Req 11.4).

#![deny(warnings)]

use std::path::Path;

use serde::Deserialize;

use crate::catalog::store::CatalogStore;
use crate::error::TuiError;
use crate::models::agent::Lifecycle;
use crate::models::policy::{PolicyRule, PolicyRuleType, PolicyScope, Severity, Suppression};

// ---------------------------------------------------------------------------
// TOML intermediate structs
// ---------------------------------------------------------------------------

/// Top-level deserialization target for `policies.toml`.
#[derive(Debug, Deserialize, Default)]
struct TomlConfig {
    #[serde(default)]
    rule: Vec<TomlRule>,
    #[serde(default)]
    suppression: Vec<TomlSuppression>,
}

/// A single `[[rule]]` entry as it appears in TOML.
#[derive(Debug, Deserialize)]
struct TomlRule {
    id: String,
    #[serde(rename = "type")]
    rule_type: String,
    severity: String,
    /// Scope: either the string "all" OR an inline table {team="…"} / {name_pattern="…"}.
    scope: toml::Value,
    description: String,
    // Type-specific optional fields
    asset_id: Option<String>,
    role_id: Option<String>,
    threshold: Option<u32>,
    max_mutation: Option<bool>,
    max_egress: Option<bool>,
    max_credentials: Option<bool>,
    min_stage: Option<String>,
}

/// A single `[[suppression]]` entry as it appears in TOML.
#[derive(Debug, Deserialize)]
struct TomlSuppression {
    rule_id: String,
    workspace: String,
    reason: String,
    #[serde(default)]
    approver: String,
    #[serde(default)]
    expires: String,
}

// ---------------------------------------------------------------------------
// PolicyConfig — the parsed result returned to callers
// ---------------------------------------------------------------------------

/// Fully-parsed policy configuration.
#[derive(Debug, Default)]
pub struct PolicyConfig {
    /// Parsed and valid policy rules.
    pub rules: Vec<PolicyRule>,
    /// Parsed suppressions.
    pub suppressions: Vec<Suppression>,
    /// True when the file was absent; callers may display a "no policies" notice.
    pub no_policies_file: bool,
    /// Parse errors collected during loading (malformed rules were skipped).
    /// Stored as error messages so PolicyConfig remains cheaply cloneable.
    pub parse_errors: Vec<String>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Load `policies.toml` from `path`.
///
/// - File absent → returns `PolicyConfig { no_policies_file: true, .. }`, no error.
/// - Syntax error → returns collected errors in `parse_errors`; valid rules are kept.
pub fn load(path: &Path) -> Result<PolicyConfig, TuiError> {
    if !path.exists() {
        return Ok(PolicyConfig {
            no_policies_file: true,
            ..Default::default()
        });
    }

    let content = std::fs::read_to_string(path).map_err(|e| TuiError::PolicyParse {
        path: path.display().to_string(),
        line: None,
        detail: e.to_string(),
    })?;

    parse_content(&content, path)
}

/// Parse TOML `content` as a policy config; `path` is used only for error messages.
///
/// Returns `Err` only for fatal top-level parse failures.
/// Rule-level failures are collected in `PolicyConfig::parse_errors`.
pub fn parse_content(content: &str, path: &Path) -> Result<PolicyConfig, TuiError> {
    let raw: TomlConfig = match toml::from_str(content) {
        Ok(v) => v,
        Err(e) => {
            // Extract 1-based line number from the span if available.
            let line = e.span().and_then(|span| {
                let before = content.get(..span.start).unwrap_or("");
                let n = before.lines().count();
                if n == 0 {
                    Some(1usize)
                } else {
                    Some(n)
                }
            });
            return Err(TuiError::PolicyParse {
                path: path.display().to_string(),
                line,
                detail: e.to_string(),
            });
        }
    };

    let mut rules = Vec::new();
    let mut parse_errors = Vec::new();

    for toml_rule in raw.rule {
        match convert_rule(toml_rule, path) {
            Ok(rule) => rules.push(rule),
            Err(e) => parse_errors.push(e.to_string()),
        }
    }

    let suppressions: Vec<Suppression> = raw
        .suppression
        .into_iter()
        .map(|s| Suppression {
            rule_id: s.rule_id,
            workspace: s.workspace,
            reason: s.reason,
            approver: s.approver,
            expires: s.expires,
        })
        .collect();

    Ok(PolicyConfig {
        rules,
        suppressions,
        no_policies_file: false,
        parse_errors,
    })
}

/// Validate rules in `config` against the catalog, returning errors for references
/// to nonexistent assets or roles (Req 11.4).
pub fn validate_rules(config: &PolicyConfig, catalog: &CatalogStore) -> Vec<TuiError> {
    let mut errors = Vec::new();
    let asset_ids = catalog.all_asset_ids();

    for rule in &config.rules {
        match &rule.rule_type {
            PolicyRuleType::RequireAsset { asset_id } => {
                if !asset_ids.contains(asset_id) {
                    errors.push(TuiError::PolicyInvalidRule {
                        rule: rule.id.clone(),
                        reason: format!("references nonexistent asset '{}'", asset_id),
                    });
                }
            }
            PolicyRuleType::RequireRole { role_id } => {
                if !catalog.roles.contains_key(role_id.as_str()) {
                    errors.push(TuiError::PolicyInvalidRule {
                        rule: rule.id.clone(),
                        reason: format!("references nonexistent role '{}'", role_id),
                    });
                }
            }
            // MaxStale, TrustBoundary, LifecycleGate have no catalog references to validate.
            _ => {}
        }
    }

    errors
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

fn convert_severity(s: &str, rule_id: &str, path: &Path) -> Result<Severity, TuiError> {
    match s.to_lowercase().as_str() {
        "critical" => Ok(Severity::Critical),
        "warning" => Ok(Severity::Warning),
        "info" => Ok(Severity::Info),
        other => Err(TuiError::PolicyParse {
            path: path.display().to_string(),
            line: None,
            detail: format!("rule '{}': unknown severity '{}'", rule_id, other),
        }),
    }
}

fn convert_scope(val: &toml::Value, rule_id: &str, path: &Path) -> Result<PolicyScope, TuiError> {
    match val {
        toml::Value::String(s) if s == "all" => Ok(PolicyScope::All),
        toml::Value::Table(t) => {
            if let Some(toml::Value::String(team)) = t.get("team") {
                Ok(PolicyScope::Team(team.clone()))
            } else if let Some(toml::Value::String(pat)) = t.get("name_pattern") {
                Ok(PolicyScope::NamePattern(pat.clone()))
            } else {
                Err(TuiError::PolicyParse {
                    path: path.display().to_string(),
                    line: None,
                    detail: format!(
                        "rule '{}': scope table must have 'team' or 'name_pattern'",
                        rule_id
                    ),
                })
            }
        }
        _ => Err(TuiError::PolicyParse {
            path: path.display().to_string(),
            line: None,
            detail: format!(
                "rule '{}': scope must be \"all\" or a table {{team=…}} / {{name_pattern=…}}",
                rule_id
            ),
        }),
    }
}

fn convert_lifecycle(s: &str, rule_id: &str, path: &Path) -> Result<Lifecycle, TuiError> {
    match s.to_lowercase().as_str() {
        "experimental" => Ok(Lifecycle::Experimental),
        "beta" => Ok(Lifecycle::Beta),
        "stable" => Ok(Lifecycle::Stable),
        "deprecated" => Ok(Lifecycle::Deprecated),
        other => Err(TuiError::PolicyParse {
            path: path.display().to_string(),
            line: None,
            detail: format!("rule '{}': unknown lifecycle stage '{}'", rule_id, other),
        }),
    }
}

fn convert_rule(r: TomlRule, path: &Path) -> Result<PolicyRule, TuiError> {
    let severity = convert_severity(&r.severity, &r.id, path)?;
    let scope = convert_scope(&r.scope, &r.id, path)?;

    let rule_type = match r.rule_type.as_str() {
        "require_asset" => {
            let asset_id = r.asset_id.ok_or_else(|| TuiError::PolicyParse {
                path: path.display().to_string(),
                line: None,
                detail: format!("rule '{}': require_asset needs 'asset_id'", r.id),
            })?;
            PolicyRuleType::RequireAsset { asset_id }
        }
        "require_role" => {
            let role_id = r.role_id.ok_or_else(|| TuiError::PolicyParse {
                path: path.display().to_string(),
                line: None,
                detail: format!("rule '{}': require_role needs 'role_id'", r.id),
            })?;
            PolicyRuleType::RequireRole { role_id }
        }
        "max_stale" => {
            let threshold = r.threshold.ok_or_else(|| TuiError::PolicyParse {
                path: path.display().to_string(),
                line: None,
                detail: format!("rule '{}': max_stale needs 'threshold'", r.id),
            })?;
            PolicyRuleType::MaxStale { threshold }
        }
        "trust_boundary" => {
            let max_mutation = r.max_mutation.unwrap_or(true);
            let max_egress = r.max_egress.unwrap_or(true);
            let max_credentials = r.max_credentials.unwrap_or(true);
            PolicyRuleType::TrustBoundary {
                max_mutation,
                max_egress,
                max_credentials,
            }
        }
        "lifecycle_gate" => {
            let stage_str = r.min_stage.ok_or_else(|| TuiError::PolicyParse {
                path: path.display().to_string(),
                line: None,
                detail: format!("rule '{}': lifecycle_gate needs 'min_stage'", r.id),
            })?;
            let min_stage = convert_lifecycle(&stage_str, &r.id, path)?;
            PolicyRuleType::LifecycleGate { min_stage }
        }
        other => {
            return Err(TuiError::PolicyParse {
                path: path.display().to_string(),
                line: None,
                detail: format!("rule '{}': unknown rule type '{}'", r.id, other),
            });
        }
    };

    Ok(PolicyRule {
        id: r.id,
        rule_type,
        severity,
        scope,
        description: r.description,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn path() -> &'static Path {
        Path::new("policies.toml")
    }

    #[test]
    fn parse_full_policy_toml() {
        let toml = r#"
[[rule]]
id = "require-scanner"
type = "require_asset"
asset_id = "aws-iam-analyzer"
severity = "critical"
scope = "all"
description = "Need scanner"

[[rule]]
id = "require-role"
type = "require_role"
role_id = "cloud-security-engineer"
severity = "warning"
scope = { team = "platform-security" }
description = "Need role"

[[rule]]
id = "max-stale"
type = "max_stale"
threshold = 5
severity = "warning"
scope = "all"
description = "Max stale"

[[rule]]
id = "trust-boundary"
type = "trust_boundary"
max_mutation = false
max_egress = true
max_credentials = true
severity = "critical"
scope = { name_pattern = "production-*" }
description = "Trust boundary"

[[rule]]
id = "lifecycle-gate"
type = "lifecycle_gate"
min_stage = "stable"
severity = "critical"
scope = { name_pattern = "production-*" }
description = "Lifecycle gate"

[[suppression]]
rule_id = "trust-boundary"
workspace = "staging-infra"
reason = "Approved"
approver = "lead@example.com"
expires = "2025-06-01"
"#;
        let cfg = parse_content(toml, path()).unwrap();
        assert_eq!(cfg.rules.len(), 5);
        assert_eq!(cfg.suppressions.len(), 1);
        assert!(!cfg.no_policies_file);
        assert!(cfg.parse_errors.is_empty());
    }

    #[test]
    fn missing_file_gives_empty_config() {
        let cfg = load(Path::new("/nonexistent/policies.toml")).unwrap();
        assert!(cfg.no_policies_file);
        assert!(cfg.rules.is_empty());
    }

    #[test]
    fn malformed_rule_is_skipped_but_valid_rules_kept() {
        let toml = r#"
[[rule]]
id = "good-rule"
type = "max_stale"
threshold = 3
severity = "warning"
scope = "all"
description = "OK"

[[rule]]
id = "bad-rule"
type = "require_asset"
# missing asset_id — should be skipped
severity = "critical"
scope = "all"
description = "Missing asset_id"
"#;
        let cfg = parse_content(toml, path()).unwrap();
        assert_eq!(cfg.rules.len(), 1, "expected 1 valid rule");
        assert_eq!(cfg.rules[0].id, "good-rule");
        assert_eq!(cfg.parse_errors.len(), 1, "expected 1 parse error");
    }

    #[test]
    fn validate_rules_catches_nonexistent_asset() {
        let toml = r#"
[[rule]]
id = "bad-asset"
type = "require_asset"
asset_id = "nonexistent-asset-xyz"
severity = "critical"
scope = "all"
description = "Bad"
"#;
        let cfg = parse_content(toml, path()).unwrap();
        let store = CatalogStore::from_parts(vec![], vec![], HashMap::new(), vec![], vec![]);
        let errors = validate_rules(&cfg, &store);
        assert_eq!(errors.len(), 1);
        assert!(
            matches!(&errors[0], TuiError::PolicyInvalidRule { rule, .. } if rule == "bad-asset")
        );
    }

    #[test]
    fn validate_rules_catches_nonexistent_role() {
        let toml = r#"
[[rule]]
id = "bad-role"
type = "require_role"
role_id = "nonexistent-role-xyz"
severity = "warning"
scope = "all"
description = "Bad"
"#;
        let cfg = parse_content(toml, path()).unwrap();
        let store = CatalogStore::from_parts(vec![], vec![], HashMap::new(), vec![], vec![]);
        let errors = validate_rules(&cfg, &store);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn invalid_rule_type_is_collected_as_error() {
        let toml = r#"
[[rule]]
id = "unknown-type"
type = "future_rule"
severity = "info"
scope = "all"
description = "Future"
"#;
        let cfg = parse_content(toml, path()).unwrap();
        assert!(cfg.rules.is_empty());
        assert_eq!(cfg.parse_errors.len(), 1);
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(256))]

        #[test]
        fn parse_does_not_panic_on_arbitrary_input(s in ".*") {
            // Must never panic — may return Ok or Err.
            let _ = parse_content(&s, path());
        }
    }
}
