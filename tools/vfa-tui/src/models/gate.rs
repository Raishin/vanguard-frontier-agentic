use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::security::sanitize::has_control_bytes;

/// Status of a validation gate (v1 model — used by `ValidationGate` and app.rs).
///
/// The DAG executor uses [`DagGateStatus`] which additionally has a `Pending`
/// and `Skipped` variant to represent pre-execution and cascade-skip states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateStatus {
    NotRun,
    Running,
    Passed,
    Failed,
    TimedOut,
}

// ── DAG executor types ────────────────────────────────────────────────────────

/// Runtime status for a gate inside the DAG executor.
///
/// Distinct from [`GateStatus`] to avoid breaking existing consumers while
/// adding the `Pending` / `Skipped` states required by the DAG model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DagGateStatus {
    Pending,
    Running,
    Passed,
    Failed,
    Skipped,
    TimedOut,
}

/// A fully-resolved gate definition ready for DAG execution.
#[derive(Debug, Clone)]
pub struct GateDefinition {
    /// Unique gate name within the DAG (matches `GateTomlEntry::name`).
    pub name: String,
    /// Executable or script path to run.
    pub command: String,
    /// Arguments to pass to the command.
    pub args: Vec<String>,
    /// Names of gates that must succeed before this gate can run.
    pub dependencies: Vec<String>,
    /// Maximum wall-clock time allowed for this gate.
    pub timeout: Duration,
    /// Human-readable description shown in the TUI.
    pub description: String,
}

/// The in-memory gate dependency graph.
#[derive(Debug, Clone)]
pub struct GateDAG {
    /// All gate definitions in this DAG.
    pub gates: Vec<GateDefinition>,
    /// Maps each gate name → list of gates that depend on it (i.e. downstream edges).
    pub adjacency: HashMap<String, Vec<String>>,
    /// Topologically sorted execution layers; gates within the same layer can run in parallel.
    pub execution_order: Vec<Vec<String>>,
}

/// Result produced by executing a single gate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    /// Name of the gate that was executed.
    pub name: String,
    /// Terminal status after execution.
    pub status: DagGateStatus,
    /// Process exit code; `None` if the gate was skipped or timed out before exit.
    pub exit_code: Option<i32>,
    /// Wall-clock duration of the execution.
    #[serde(skip)]
    pub duration: Duration,
    /// ISO 8601 timestamp of when the gate started.
    pub timestamp: String,
    /// Combined stdout+stderr captured from the subprocess.
    pub output: String,
    /// Human-readable reason why the gate was skipped, if applicable.
    pub skip_reason: Option<String>,
}

/// Top-level structure parsed from `gates.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct GatesConfig {
    /// Array of gate entries (`[[gate]]` in TOML).
    pub gate: Vec<GateTomlEntry>,
}

/// One `[[gate]]` entry as deserialized from `gates.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct GateTomlEntry {
    /// Unique gate name within this config file.
    pub name: String,
    /// Executable or script path.
    pub command: String,
    /// Optional arguments list.
    pub args: Option<Vec<String>>,
    /// Names of gates that must complete first.
    pub depends_on: Option<Vec<String>>,
    /// Timeout in seconds; uses a default if absent.
    pub timeout_secs: Option<u64>,
    /// Optional human-readable description.
    pub description: Option<String>,
}

/// A validation gate represents a script that must pass before export.
#[derive(Debug, Clone)]
pub struct ValidationGate {
    pub script_name: String,
    pub description: String,
    pub status: GateStatus,
    pub last_exit_code: Option<i32>,
    pub last_duration: Option<Duration>,
}

impl ValidationGate {
    /// Create a new validation gate with NotRun status.
    pub fn new(script_name: String, description: String) -> Self {
        Self {
            script_name,
            description,
            status: GateStatus::NotRun,
            last_exit_code: None,
            last_duration: None,
        }
    }
}

/// Extract validation gates from the package.json in the workspace root.
///
/// Looks for scripts matching the pattern `validate:*` and creates a
/// ValidationGate for each with status NotRun.
pub fn extract_validation_gates(workspace_root: &Path) -> Vec<ValidationGate> {
    let package_json_path = workspace_root.join("package.json");

    let content = match std::fs::read_to_string(&package_json_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let parsed: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let scripts = match parsed.get("scripts").and_then(|s| s.as_object()) {
        Some(s) => s,
        None => return Vec::new(),
    };

    let mut gates: Vec<ValidationGate> = scripts
        .iter()
        .filter(|(name, _)| name.starts_with("validate:"))
        .filter(|(name, value)| {
            !has_control_bytes(name) && !value.as_str().is_some_and(has_control_bytes)
        })
        .map(|(name, value)| {
            let description = value.as_str().unwrap_or("").to_string();
            ValidationGate::new(name.clone(), description)
        })
        .collect();

    gates.sort_by_key(|a| a.script_name.clone());
    gates
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn extract_gates_from_valid_package_json() {
        let tmp = TempDir::new().unwrap();
        let content = r#"{
            "name": "test",
            "scripts": {
                "build": "tsc",
                "validate:lint": "eslint .",
                "validate:types": "tsc --noEmit",
                "test": "jest"
            }
        }"#;
        std::fs::write(tmp.path().join("package.json"), content).unwrap();

        let gates = extract_validation_gates(tmp.path());
        assert_eq!(gates.len(), 2);
        assert_eq!(gates[0].script_name, "validate:lint");
        assert_eq!(gates[0].description, "eslint .");
        assert_eq!(gates[0].status, GateStatus::NotRun);
        assert_eq!(gates[1].script_name, "validate:types");
    }

    #[test]
    fn extract_gates_missing_package_json() {
        let tmp = TempDir::new().unwrap();
        let gates = extract_validation_gates(tmp.path());
        assert!(gates.is_empty());
    }

    #[test]
    fn extract_gates_no_scripts() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        let gates = extract_validation_gates(tmp.path());
        assert!(gates.is_empty());
    }

    #[test]
    fn extract_gates_no_validate_prefix() {
        let tmp = TempDir::new().unwrap();
        let content = r#"{
            "scripts": {
                "build": "tsc",
                "test": "jest"
            }
        }"#;
        std::fs::write(tmp.path().join("package.json"), content).unwrap();
        let gates = extract_validation_gates(tmp.path());
        assert!(gates.is_empty());
    }

    #[test]
    fn validation_gate_new_defaults() {
        let gate = ValidationGate::new("validate:test".to_string(), "run tests".to_string());
        assert_eq!(gate.status, GateStatus::NotRun);
        assert_eq!(gate.last_exit_code, None);
        assert_eq!(gate.last_duration, None);
    }

    #[test]
    fn extract_gates_rejects_control_bytes() {
        let tmp = TempDir::new().unwrap();
        let content = "{\n  \"scripts\": {\n    \"validate:good\": \"ok\",\n    \"validate:\\u001b[31mbad\": \"bad\"\n  }\n}";
        std::fs::write(tmp.path().join("package.json"), content).unwrap();
        let gates = extract_validation_gates(tmp.path());
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].script_name, "validate:good");
    }

    // ── DAG model tests ───────────────────────────────────────────────────────

    #[test]
    fn dag_gate_status_round_trip() {
        let statuses = [
            DagGateStatus::Pending,
            DagGateStatus::Running,
            DagGateStatus::Passed,
            DagGateStatus::Failed,
            DagGateStatus::Skipped,
            DagGateStatus::TimedOut,
        ];
        for s in &statuses {
            let json = serde_json::to_string(s).unwrap();
            let decoded: DagGateStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(&decoded, s);
        }
    }

    #[test]
    fn dag_gate_status_serialized_strings() {
        assert_eq!(
            serde_json::to_string(&DagGateStatus::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&DagGateStatus::Running).unwrap(),
            "\"running\""
        );
        assert_eq!(
            serde_json::to_string(&DagGateStatus::Passed).unwrap(),
            "\"passed\""
        );
        assert_eq!(
            serde_json::to_string(&DagGateStatus::Failed).unwrap(),
            "\"failed\""
        );
        assert_eq!(
            serde_json::to_string(&DagGateStatus::Skipped).unwrap(),
            "\"skipped\""
        );
        assert_eq!(
            serde_json::to_string(&DagGateStatus::TimedOut).unwrap(),
            "\"timed_out\""
        );
    }

    #[test]
    fn gate_result_round_trip() {
        let result = GateResult {
            name: "validate:lint".to_string(),
            status: DagGateStatus::Passed,
            exit_code: Some(0),
            duration: Duration::from_millis(1500),
            timestamp: "2025-01-01T00:00:00.000Z".to_string(),
            output: "All checks passed.".to_string(),
            skip_reason: None,
        };
        let json = serde_json::to_string(&result).unwrap();
        let decoded: GateResult = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.name, "validate:lint");
        assert_eq!(decoded.status, DagGateStatus::Passed);
        assert_eq!(decoded.exit_code, Some(0));
        assert_eq!(decoded.timestamp, "2025-01-01T00:00:00.000Z");
        assert_eq!(decoded.output, "All checks passed.");
        assert!(decoded.skip_reason.is_none());
        // duration is skipped in serde — just verify the field exists on the struct
        let _ = result.duration;
    }

    #[test]
    fn gate_result_skipped_round_trip() {
        let result = GateResult {
            name: "validate:integration".to_string(),
            status: DagGateStatus::Skipped,
            exit_code: None,
            duration: Duration::ZERO,
            timestamp: "2025-01-01T00:01:00.000Z".to_string(),
            output: String::new(),
            skip_reason: Some("dependency failed: validate:lint".to_string()),
        };
        let json = serde_json::to_string(&result).unwrap();
        let decoded: GateResult = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.status, DagGateStatus::Skipped);
        assert!(decoded.exit_code.is_none());
        assert_eq!(
            decoded.skip_reason.as_deref(),
            Some("dependency failed: validate:lint")
        );
    }

    #[test]
    fn gates_config_deserializes_from_toml() {
        let toml_input = r#"
[[gate]]
name = "lint"
command = "npm"
args = ["run", "validate:lint"]
timeout_secs = 60
description = "Run ESLint"

[[gate]]
name = "types"
command = "npm"
args = ["run", "validate:types"]
depends_on = ["lint"]
timeout_secs = 120
"#;
        let config: GatesConfig = toml::from_str(toml_input).expect("parse gates.toml");
        assert_eq!(config.gate.len(), 2);
        assert_eq!(config.gate[0].name, "lint");
        assert_eq!(config.gate[0].command, "npm");
        assert_eq!(
            config.gate[0].args.as_deref(),
            Some(&["run".to_string(), "validate:lint".to_string()][..])
        );
        assert_eq!(config.gate[0].timeout_secs, Some(60));
        assert_eq!(config.gate[0].description.as_deref(), Some("Run ESLint"));
        assert!(config.gate[0].depends_on.is_none());

        assert_eq!(config.gate[1].name, "types");
        assert_eq!(
            config.gate[1].depends_on.as_deref(),
            Some(&["lint".to_string()][..])
        );
        assert!(config.gate[1].description.is_none());
    }

    #[test]
    fn gate_toml_entry_minimal_required_fields() {
        let toml_input = r#"
[[gate]]
name = "check"
command = "cargo"
"#;
        let config: GatesConfig = toml::from_str(toml_input).expect("parse");
        assert_eq!(config.gate.len(), 1);
        assert_eq!(config.gate[0].name, "check");
        assert!(config.gate[0].args.is_none());
        assert!(config.gate[0].depends_on.is_none());
        assert!(config.gate[0].timeout_secs.is_none());
        assert!(config.gate[0].description.is_none());
    }
}
