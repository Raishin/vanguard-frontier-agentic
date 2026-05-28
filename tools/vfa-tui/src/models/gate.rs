use std::path::Path;
use std::time::Duration;

/// Status of a validation gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateStatus {
    NotRun,
    Running,
    Passed,
    Failed,
    TimedOut,
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
        .map(|(name, value)| {
            let description = value.as_str().unwrap_or("").to_string();
            ValidationGate::new(name.clone(), description)
        })
        .collect();

    gates.sort_by(|a, b| a.script_name.cmp(&b.script_name));
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
}
