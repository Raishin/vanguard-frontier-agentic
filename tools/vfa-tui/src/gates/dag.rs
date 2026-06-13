//! Gate DAG construction — parsing and topological sort.
//!
//! This module converts a `gates.toml` file (or inferred `package.json`
//! `validate:*` scripts) into a [`GateDAG`] with layered execution order
//! computed by Kahn's algorithm.
//!
//! # Design note
//! All construction is synchronous and dependency-free from async runtimes,
//! making unit and property tests fast and deterministic.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::time::Duration;

use crate::error::TuiError;
use crate::models::gate::{
    extract_validation_gates, GateDAG, GateDefinition, GatesConfig,
};

/// Default gate timeout when none is specified in `gates.toml`.
const DEFAULT_TIMEOUT_SECS: u64 = 300;

// ─────────────────────────────────────────────────────────────────────────────
// GateDagExecutor
// ─────────────────────────────────────────────────────────────────────────────

/// Constructs and executes validation gates as a DAG.
///
/// The executor is split into two concerns:
/// - **Construction** (this file): parsing gate definitions and building the
///   topological execution layers.
/// - **Execution** (`executor.rs`): running gates in parallel with cascading
///   failure propagation.
pub struct GateDagExecutor {
    /// Maximum number of gates that may run concurrently within a single layer.
    pub concurrency_limit: usize,
}

impl GateDagExecutor {
    /// Create a new executor with the given concurrency limit.
    ///
    /// Pass `0` to use the default of 4.
    pub fn new(concurrency_limit: usize) -> Self {
        Self {
            concurrency_limit: if concurrency_limit == 0 {
                4
            } else {
                concurrency_limit
            },
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// parse_gates
// ─────────────────────────────────────────────────────────────────────────────

/// Parse gate definitions from `gates_toml_path` if supplied and the file
/// exists, otherwise infer gates from `package.json` `validate:*` scripts.
///
/// # Gates.toml format
/// ```toml
/// [[gate]]
/// name        = "lint"
/// command     = "npm"
/// args        = ["run", "validate:lint"]
/// depends_on  = []
/// timeout_secs = 60
/// description = "Run ESLint"
/// ```
///
/// # Inferred gates
/// When falling back to `package.json`, each `validate:*` script becomes a
/// gate whose command is `npm` and whose single arg is `run <script-name>`.
/// No dependency relationships can be inferred; all gates are independent.
///
/// # Errors
/// Returns [`TuiError::GateConfigParse`] if the TOML file is present but
/// cannot be parsed.  Returns an empty `Vec` (not an error) when neither
/// source provides any gates.
pub fn parse_gates(
    gates_toml_path: Option<&Path>,
    workspace_root: &Path,
) -> Result<Vec<GateDefinition>, TuiError> {
    // Prefer explicit gates.toml when it exists.
    if let Some(toml_path) = gates_toml_path {
        if toml_path.exists() {
            return parse_gates_toml(toml_path);
        }
    }

    // Fall back: infer from package.json validate:* scripts.
    let inferred = infer_gates_from_package_json(workspace_root);
    Ok(inferred)
}

/// Parse a `gates.toml` file into [`GateDefinition`]s.
fn parse_gates_toml(path: &Path) -> Result<Vec<GateDefinition>, TuiError> {
    let raw = std::fs::read_to_string(path).map_err(|e| TuiError::GateConfigParse {
        path: path.display().to_string(),
        detail: e.to_string(),
    })?;

    let config: GatesConfig = toml::from_str(&raw).map_err(|e| TuiError::GateConfigParse {
        path: path.display().to_string(),
        detail: e.to_string(),
    })?;

    let defs: Vec<GateDefinition> = config
        .gate
        .into_iter()
        .map(|entry| GateDefinition {
            name: entry.name,
            command: entry.command,
            args: entry.args.unwrap_or_default(),
            dependencies: entry.depends_on.unwrap_or_default(),
            timeout: Duration::from_secs(
                entry.timeout_secs.unwrap_or(DEFAULT_TIMEOUT_SECS),
            ),
            description: entry.description.unwrap_or_default(),
        })
        .collect();

    Ok(defs)
}

/// Infer gate definitions from `package.json` `validate:*` scripts.
///
/// Each discovered script becomes: `npm run <script-name>` with no
/// dependencies and the default timeout.
fn infer_gates_from_package_json(workspace_root: &Path) -> Vec<GateDefinition> {
    extract_validation_gates(workspace_root)
        .into_iter()
        .map(|vg| GateDefinition {
            name: vg.script_name.clone(),
            command: "npm".to_string(),
            args: vec!["run".to_string(), vg.script_name],
            dependencies: Vec::new(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            description: vg.description,
        })
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// build_dag
// ─────────────────────────────────────────────────────────────────────────────

/// Build a [`GateDAG`] from a list of gate definitions.
///
/// Computes:
/// - `adjacency`: maps each gate name to the list of gates that depend on it
///   (downstream edges).
/// - `execution_order`: topological layers computed by Kahn's algorithm.
///   Gates within the same layer are independent and may run in parallel.
///
/// # Cycle detection
/// If the dependency graph contains a cycle, the function returns
/// [`TuiError::GateCycle`] with the names of the gates involved. The
/// algorithm detects cycles by noting which gates are never dequeued during
/// the topological sort.
///
/// # Errors
/// - [`TuiError::GateCycle`]: cycle detected.
pub fn build_dag(gates: Vec<GateDefinition>) -> Result<GateDAG, TuiError> {
    let names: HashSet<String> = gates.iter().map(|g| g.name.clone()).collect();

    // Build adjacency (gate → dependents) and in-degree map.
    // We also validate that all dependency names actually exist in the gate set.
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();

    // Initialise every gate with degree 0 and an empty dependents list.
    for g in &gates {
        adjacency.entry(g.name.clone()).or_default();
        in_degree.entry(g.name.clone()).or_insert(0);
    }

    for g in &gates {
        for dep in &g.dependencies {
            if !names.contains(dep.as_str()) {
                // Unknown dependency — treat as a configuration error rather
                // than a cycle so the message is clear.
                return Err(TuiError::GateConfigParse {
                    path: "gates".to_string(),
                    detail: format!(
                        "gate '{}' depends on unknown gate '{}'",
                        g.name, dep
                    ),
                });
            }
            // dep → g.name  (dep must finish before g.name starts)
            adjacency.entry(dep.clone()).or_default().push(g.name.clone());
            *in_degree.entry(g.name.clone()).or_insert(0) += 1;
        }
    }

    // Kahn's algorithm: iteratively peel gates whose in-degree is zero.
    let mut queue: VecDeque<String> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(name, _)| name.clone())
        .collect();
    // Sort for deterministic layer ordering.
    let mut sorted_queue: Vec<String> = queue.drain(..).collect();
    sorted_queue.sort();
    queue.extend(sorted_queue);

    let mut execution_order: Vec<Vec<String>> = Vec::new();
    let mut processed = 0usize;

    // We run layer-by-layer: collect all zero-degree nodes in current wave,
    // emit them as a layer, then reduce degrees for their dependents.
    while !queue.is_empty() {
        let layer_size = queue.len();
        let mut layer: Vec<String> = queue.drain(..layer_size).collect();
        layer.sort(); // deterministic order within layer
        processed += layer.len();

        let mut next: Vec<String> = Vec::new();
        for gate_name in &layer {
            if let Some(dependents) = adjacency.get(gate_name) {
                for dep in dependents {
                    let deg = in_degree.get_mut(dep).expect("degree must exist");
                    *deg -= 1;
                    if *deg == 0 {
                        next.push(dep.clone());
                    }
                }
            }
        }

        execution_order.push(layer);
        next.sort();
        queue.extend(next);
    }

    // If not all gates were processed, there is a cycle.
    if processed < gates.len() {
        let cycle_gates: Vec<String> = in_degree
            .iter()
            .filter(|(_, &d)| d > 0)
            .map(|(name, _)| name.clone())
            .collect::<std::collections::BTreeSet<_>>() // sorted for determinism
            .into_iter()
            .collect();
        return Err(TuiError::GateCycle {
            gates: cycle_gates.join(", "),
        });
    }

    Ok(GateDAG {
        gates,
        adjacency,
        execution_order,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_gate(name: &str, deps: &[&str]) -> GateDefinition {
        GateDefinition {
            name: name.to_string(),
            command: "echo".to_string(),
            args: vec![name.to_string()],
            dependencies: deps.iter().map(|s| s.to_string()).collect(),
            timeout: Duration::from_secs(60),
            description: String::new(),
        }
    }

    // ── parse_gates ──────────────────────────────────────────────────────────

    #[test]
    fn parse_gates_from_valid_toml() {
        let tmp = TempDir::new().unwrap();
        let toml = r#"
[[gate]]
name = "lint"
command = "npm"
args = ["run", "validate:lint"]
timeout_secs = 60

[[gate]]
name = "types"
command = "npm"
args = ["run", "validate:types"]
depends_on = ["lint"]
"#;
        let path = tmp.path().join("gates.toml");
        std::fs::write(&path, toml).unwrap();

        let gates = parse_gates(Some(&path), tmp.path()).unwrap();
        assert_eq!(gates.len(), 2);
        assert_eq!(gates[0].name, "lint");
        assert_eq!(gates[1].name, "types");
        assert_eq!(gates[1].dependencies, vec!["lint"]);
    }

    #[test]
    fn parse_gates_falls_back_to_package_json() {
        let tmp = TempDir::new().unwrap();
        let pkg = r#"{
            "scripts": {
                "validate:lint": "eslint .",
                "validate:types": "tsc --noEmit"
            }
        }"#;
        std::fs::write(tmp.path().join("package.json"), pkg).unwrap();

        // Pass None for toml_path — should fall back.
        let gates = parse_gates(None, tmp.path()).unwrap();
        assert_eq!(gates.len(), 2);
        assert!(gates.iter().any(|g| g.name == "validate:lint"));
    }

    #[test]
    fn parse_gates_toml_path_missing_falls_back_to_package_json() {
        let tmp = TempDir::new().unwrap();
        let non_existent = tmp.path().join("gates.toml"); // does not exist
        let pkg = r#"{"scripts": {"validate:lint": "eslint ."}}"#;
        std::fs::write(tmp.path().join("package.json"), pkg).unwrap();

        let gates = parse_gates(Some(&non_existent), tmp.path()).unwrap();
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].name, "validate:lint");
    }

    #[test]
    fn parse_gates_invalid_toml_returns_error() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("gates.toml");
        std::fs::write(&path, "[[gate]\nname = \"oops\"").unwrap();

        let result = parse_gates(Some(&path), tmp.path());
        assert!(matches!(result, Err(TuiError::GateConfigParse { .. })));
    }

    #[test]
    fn inferred_gates_have_npm_run_command() {
        let tmp = TempDir::new().unwrap();
        let pkg =
            r#"{"scripts": {"validate:schema": "node check-schema.js"}}"#;
        std::fs::write(tmp.path().join("package.json"), pkg).unwrap();

        let gates = parse_gates(None, tmp.path()).unwrap();
        assert_eq!(gates[0].command, "npm");
        assert_eq!(gates[0].args, vec!["run", "validate:schema"]);
        assert!(gates[0].dependencies.is_empty());
        assert_eq!(gates[0].timeout, Duration::from_secs(DEFAULT_TIMEOUT_SECS));
    }

    // ── build_dag ────────────────────────────────────────────────────────────

    #[test]
    fn build_dag_empty_gates() {
        let dag = build_dag(vec![]).unwrap();
        assert!(dag.gates.is_empty());
        assert!(dag.adjacency.is_empty());
        assert!(dag.execution_order.is_empty());
    }

    #[test]
    fn build_dag_single_gate() {
        let dag = build_dag(vec![make_gate("lint", &[])]).unwrap();
        assert_eq!(dag.execution_order, vec![vec!["lint".to_string()]]);
    }

    #[test]
    fn build_dag_linear_chain() {
        // a → b → c  (b depends on a, c depends on b)
        let gates = vec![
            make_gate("a", &[]),
            make_gate("b", &["a"]),
            make_gate("c", &["b"]),
        ];
        let dag = build_dag(gates).unwrap();
        assert_eq!(dag.execution_order.len(), 3);
        assert_eq!(dag.execution_order[0], vec!["a"]);
        assert_eq!(dag.execution_order[1], vec!["b"]);
        assert_eq!(dag.execution_order[2], vec!["c"]);
    }

    #[test]
    fn build_dag_parallel_roots() {
        // a and b have no dependencies; c depends on both
        let gates = vec![
            make_gate("a", &[]),
            make_gate("b", &[]),
            make_gate("c", &["a", "b"]),
        ];
        let dag = build_dag(gates).unwrap();
        assert_eq!(dag.execution_order.len(), 2);
        assert_eq!(dag.execution_order[0], vec!["a", "b"]); // sorted
        assert_eq!(dag.execution_order[1], vec!["c"]);
    }

    #[test]
    fn build_dag_cycle_detected() {
        // a → b → c → a  (cycle)
        let gates = vec![
            make_gate("a", &["c"]),
            make_gate("b", &["a"]),
            make_gate("c", &["b"]),
        ];
        let result = build_dag(gates);
        assert!(
            matches!(result, Err(TuiError::GateCycle { .. })),
            "expected GateCycle, got: {:?}",
            result
        );
        if let Err(TuiError::GateCycle { gates }) = result {
            assert!(gates.contains('a') || gates.contains('b') || gates.contains('c'));
        }
    }

    #[test]
    fn build_dag_unknown_dependency_returns_config_error() {
        let gates = vec![make_gate("lint", &["nonexistent"])];
        let result = build_dag(gates);
        assert!(matches!(result, Err(TuiError::GateConfigParse { .. })));
    }

    #[test]
    fn build_dag_adjacency_maps_downstream() {
        // lint → types → integration
        let gates = vec![
            make_gate("lint", &[]),
            make_gate("types", &["lint"]),
            make_gate("integration", &["types"]),
        ];
        let dag = build_dag(gates).unwrap();
        // lint's dependents should be [types]
        assert_eq!(
            dag.adjacency.get("lint").unwrap(),
            &vec!["types".to_string()]
        );
        // types' dependents should be [integration]
        assert_eq!(
            dag.adjacency.get("types").unwrap(),
            &vec!["integration".to_string()]
        );
        // integration has no dependents
        assert!(dag.adjacency.get("integration").unwrap().is_empty());
    }

    #[test]
    fn build_dag_every_gate_appears_exactly_once() {
        let gates = vec![
            make_gate("a", &[]),
            make_gate("b", &["a"]),
            make_gate("c", &["a"]),
            make_gate("d", &["b", "c"]),
        ];
        let dag = build_dag(gates).unwrap();
        let all: Vec<&String> = dag.execution_order.iter().flatten().collect();
        assert_eq!(all.len(), 4);
        let unique: HashSet<&&String> = all.iter().collect();
        assert_eq!(unique.len(), 4);
    }

    #[test]
    fn build_dag_default_timeout_applied() {
        let tmp = TempDir::new().unwrap();
        let pkg = r#"{"scripts": {"validate:test": "jest"}}"#;
        std::fs::write(tmp.path().join("package.json"), pkg).unwrap();

        let gates = parse_gates(None, tmp.path()).unwrap();
        assert_eq!(gates[0].timeout, Duration::from_secs(DEFAULT_TIMEOUT_SECS));
    }
}
