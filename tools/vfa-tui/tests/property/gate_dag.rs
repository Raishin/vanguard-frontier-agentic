//! Property tests for the gate DAG executor.
//!
//! **Property 10** (Req 2.1): topological sort produces a valid execution order.
//!   - For any acyclic random DAG, every gate appears after all its dependencies.
//!   - For any graph WITH a cycle, `build_dag` returns `GateCycle` (no infinite loop).
//!
//! **Property 11** (Req 2.4): prerequisite failure cascades correctly.
//!   - Given a DAG and a chosen failing gate, ALL transitive dependents are
//!     `Skipped` and none are executed; gates not downstream are unaffected.

use proptest::prelude::*;
use proptest::test_runner::Config;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::runtime::Runtime;
use vfa_tui::gates::dag::build_dag;
use vfa_tui::gates::executor::{execute_all, is_cache_valid, MockRunner};
use vfa_tui::models::gate::{DagGateStatus, GateDefinition};

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Build a `GateDefinition` with unique alphabetic names.
fn gate_def(name: &str, deps: &[&str]) -> GateDefinition {
    GateDefinition {
        name: name.to_string(),
        command: "true".to_string(),
        args: vec![],
        dependencies: deps.iter().map(|s| s.to_string()).collect(),
        timeout: Duration::from_secs(10),
        description: String::new(),
    }
}

/// Strategy that generates a random acyclic DAG with `n` gates named g0..g(n-1).
///
/// Gate `g_i` may only depend on gates `g_j` where `j < i`, which guarantees
/// no cycle by construction.
fn acyclic_dag_strategy(
    n: usize,
) -> impl Strategy<Value = Vec<GateDefinition>> {
    // For each gate i (0..n), generate a (possibly empty) subset of {0..i} as deps.
    let dep_vecs: Vec<_> = (0..n)
        .map(|i| {
            if i == 0 {
                Just(vec![]).boxed()
            } else {
                proptest::collection::vec(0usize..i, 0..i)
                    .prop_map(|mut v| {
                        v.sort_unstable();
                        v.dedup();
                        v
                    })
                    .boxed()
            }
        })
        .collect();

    dep_vecs.prop_map(move |dep_lists| {
        dep_lists
            .into_iter()
            .enumerate()
            .map(|(i, deps)| {
                let dep_names: Vec<&str> = deps.iter().map(|&j| {
                    // SAFETY: j < i ≤ n, names are built deterministically below
                    // but we need &'static str-like names. Use a workaround:
                    // we'll produce the dep names later via the index.
                    // For now return a placeholder — we'll fix with a closure.
                    let _ = j;
                    ""
                }).collect();
                (i, deps, dep_names)
            })
            .collect::<Vec<_>>()
    })
    .prop_map(|triples| {
        triples
            .into_iter()
            .map(|(i, deps, _)| {
                let dep_names: Vec<String> = deps.iter().map(|&j| format!("g{j}")).collect();
                GateDefinition {
                    name: format!("g{i}"),
                    command: "true".to_string(),
                    args: vec![],
                    dependencies: dep_names,
                    timeout: Duration::from_secs(10),
                    description: String::new(),
                }
            })
            .collect()
    })
}

/// Compute the set of all transitive downstream gates of `start` in a DAG
/// (following adjacency = gate → its dependents).
fn transitive_dependents(dag: &vfa_tui::models::gate::GateDAG, start: &str) -> HashSet<String> {
    let mut visited = HashSet::new();
    let mut stack = vec![start.to_string()];
    while let Some(current) = stack.pop() {
        if let Some(deps) = dag.adjacency.get(&current) {
            for d in deps {
                if !visited.contains(d) {
                    visited.insert(d.clone());
                    stack.push(d.clone());
                }
            }
        }
    }
    visited
}

// ─────────────────────────────────────────────────────────────────────────────
// Property 10 — topological sort validity
// ─────────────────────────────────────────────────────────────────────────────

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Property 10a: For any acyclic DAG with 1–8 gates, `build_dag` succeeds and
    /// the execution_order satisfies: every gate appears AFTER all of its dependencies.
    #[test]
    fn prop10_topo_order_valid_for_acyclic_dag(
        n in 1usize..=8,
        gates in acyclic_dag_strategy(8),
    ) {
        // Truncate to n gates.
        let gates: Vec<GateDefinition> = gates.into_iter().take(n).collect();

        let result = build_dag(gates);
        prop_assert!(result.is_ok(), "acyclic dag should not fail: {:?}", result);

        let dag = result.unwrap();
        let flat_order: Vec<String> = dag.execution_order.iter().flatten().cloned().collect();

        // Every gate must appear exactly once.
        prop_assert_eq!(flat_order.len(), dag.gates.len(),
            "every gate must appear in execution_order exactly once");

        // Position map: gate name → index in the flat topological order.
        let pos: HashMap<String, usize> = flat_order
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), i))
            .collect();

        // For every gate, each dependency must appear at an earlier position.
        for gate in &dag.gates {
            let gate_pos = pos[&gate.name];
            for dep in &gate.dependencies {
                let dep_pos = pos[dep];
                prop_assert!(
                    dep_pos < gate_pos,
                    "dependency '{dep}' (pos {dep_pos}) must appear before '{}' (pos {gate_pos})",
                    gate.name
                );
            }
        }
    }

    /// Property 10b: For any acyclic DAG, every gate appears in at most one layer.
    #[test]
    fn prop10_each_gate_in_exactly_one_layer(
        n in 1usize..=8,
        gates in acyclic_dag_strategy(8),
    ) {
        let gates: Vec<GateDefinition> = gates.into_iter().take(n).collect();
        let dag = build_dag(gates).unwrap();

        let mut seen: HashSet<String> = HashSet::new();
        for layer in &dag.execution_order {
            for name in layer {
                prop_assert!(
                    !seen.contains(name),
                    "gate '{name}' appeared in more than one layer"
                );
                seen.insert(name.clone());
            }
        }
        prop_assert_eq!(seen.len(), dag.gates.len());
    }

    /// Property 10c: A graph with a cycle returns `GateCycle` error (no infinite loop,
    /// no panic).  We construct a minimal 2-node cycle: a depends on b, b depends on a.
    #[test]
    fn prop10_cycle_detected_no_infinite_loop(
        extra_name in "[a-z]{3,8}",
    ) {
        // Minimal cycle: a → b → a
        let gates = vec![
            gate_def("a", &["b"]),
            gate_def("b", &["a"]),
            gate_def(&extra_name, &[]), // an independent gate — should still be detected
        ];
        // If extra_name collides with "a" or "b", the test is still valid
        // because we just want the cycle to be detected.
        let result = build_dag(gates);
        prop_assert!(
            matches!(result, Err(vfa_tui::error::TuiError::GateCycle { .. })),
            "expected GateCycle error, got: {:?}",
            result
        );
    }

    /// Property 10d: A 3-node cycle (a → b → c → a) is detected.
    #[test]
    fn prop10_three_node_cycle_detected(
        _dummy in 0u32..100,
    ) {
        let gates = vec![
            gate_def("a", &["c"]),
            gate_def("b", &["a"]),
            gate_def("c", &["b"]),
        ];
        let result = build_dag(gates);
        prop_assert!(
            matches!(result, Err(vfa_tui::error::TuiError::GateCycle { .. })),
            "3-node cycle must be detected"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Property 11 — prerequisite failure cascade
// ─────────────────────────────────────────────────────────────────────────────

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Property 11a: When a gate fails, ALL of its transitive downstream
    /// dependents are marked Skipped, and NO downstream gate is executed.
    ///
    /// We use an acyclic DAG and pick the first gate (g0) as the failing gate.
    /// g0 has no dependencies so it is always present.  Its transitive
    /// dependents (gates that depend on g0 directly or transitively) must all
    /// be Skipped.  Gates that are NOT downstream of g0 must be Passed.
    #[test]
    fn prop11_failure_skips_all_transitive_dependents(
        n in 2usize..=8,
        gates in acyclic_dag_strategy(8),
    ) {
        let gates: Vec<GateDefinition> = gates.into_iter().take(n).collect();
        // Only proceed if build_dag succeeds (it always should for acyclic graphs).
        let dag = match build_dag(gates) {
            Ok(d) => d,
            Err(_) => return Ok(()), // skip malformed inputs
        };

        // Find a gate with no dependencies to be the "failing" gate.
        // By construction of acyclic_dag_strategy, g0 always has no deps.
        let failing_gate = "g0".to_string();

        // Compute all transitive dependents of the failing gate BEFORE execution.
        let expected_skipped = transitive_dependents(&dag, &failing_gate);

        // Runner: g0 fails, everything else passes.
        let runner = MockRunner::new(move |name: &str| {
            if name == "g0" {
                DagGateStatus::Failed
            } else {
                DagGateStatus::Passed
            }
        });

        let rt = Runtime::new().unwrap();
        let workspace = tempfile::TempDir::new().unwrap();
        let results = rt.block_on(execute_all(&dag, 4, workspace.path(), &runner));

        // All transitive dependents must be Skipped.
        for r in &results {
            if expected_skipped.contains(&r.name) {
                prop_assert_eq!(
                    r.status.clone(), DagGateStatus::Skipped,
                    "gate '{}' is downstream of '{}' and must be Skipped, but got {:?}",
                    r.name, failing_gate, r.status
                );
                // skip_reason must reference a failing dependency.
                prop_assert!(
                    r.skip_reason.as_deref().unwrap_or("").contains("dependency failed"),
                    "gate '{}' skip_reason should indicate dependency failure, got: {:?}",
                    r.name, r.skip_reason
                );
            } else if r.name != failing_gate {
                // Gates not downstream of the failing gate must have been executed
                // (Passed, in our test setup).
                prop_assert_eq!(
                    r.status.clone(), DagGateStatus::Passed,
                    "gate '{}' is NOT downstream of '{}' and must be Passed, but got {:?}",
                    r.name, failing_gate, r.status
                );
            }
        }
    }

    /// Property 11b: The failing gate itself is always Failed (not Skipped).
    #[test]
    fn prop11_failing_gate_is_failed_not_skipped(
        n in 1usize..=6,
        gates in acyclic_dag_strategy(6),
    ) {
        let gates: Vec<GateDefinition> = gates.into_iter().take(n).collect();
        let dag = match build_dag(gates) {
            Ok(d) => d,
            Err(_) => return Ok(()),
        };

        let runner = MockRunner::new(|name: &str| {
            if name == "g0" {
                DagGateStatus::Failed
            } else {
                DagGateStatus::Passed
            }
        });

        let rt = Runtime::new().unwrap();
        let workspace = tempfile::TempDir::new().unwrap();
        let results = rt.block_on(execute_all(&dag, 4, workspace.path(), &runner));

        if let Some(g0) = results.iter().find(|r| r.name == "g0") {
            prop_assert_eq!(
                g0.status.clone(), DagGateStatus::Failed,
                "g0 should be Failed, not {:?}",
                g0.status
            );
        }
    }

    /// Property 11c: When no gate fails (all pass), no gate is Skipped.
    #[test]
    fn prop11_no_failure_means_no_skips(
        n in 1usize..=8,
        gates in acyclic_dag_strategy(8),
    ) {
        let gates: Vec<GateDefinition> = gates.into_iter().take(n).collect();
        let dag = match build_dag(gates) {
            Ok(d) => d,
            Err(_) => return Ok(()),
        };

        let runner = MockRunner::new(|_| DagGateStatus::Passed);

        let rt = Runtime::new().unwrap();
        let workspace = tempfile::TempDir::new().unwrap();
        let results = rt.block_on(execute_all(&dag, 4, workspace.path(), &runner));

        for r in &results {
            prop_assert_ne!(
                r.status.clone(), DagGateStatus::Skipped,
                "gate '{}' should not be Skipped when no failures occur",
                r.name
            );
        }
    }

    /// Property 11d: The total count of results equals the number of gates in the DAG.
    #[test]
    fn prop11_result_count_equals_gate_count(
        n in 1usize..=8,
        gates in acyclic_dag_strategy(8),
    ) {
        let gates: Vec<GateDefinition> = gates.into_iter().take(n).collect();
        let dag = match build_dag(gates) {
            Ok(d) => d,
            Err(_) => return Ok(()),
        };
        let gate_count = dag.gates.len();

        let runner = MockRunner::new(|name: &str| {
            if name == "g0" { DagGateStatus::Failed } else { DagGateStatus::Passed }
        });

        let rt = Runtime::new().unwrap();
        let workspace = tempfile::TempDir::new().unwrap();
        let results = rt.block_on(execute_all(&dag, 4, workspace.path(), &runner));

        prop_assert_eq!(
            results.len(), gate_count,
            "result count ({}) must equal gate count ({})",
            results.len(), gate_count
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Property: is_cache_valid content-hash equality
// ─────────────────────────────────────────────────────────────────────────────

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// is_cache_valid returns true iff both hashes are non-empty and equal.
    #[test]
    fn prop_cache_valid_iff_equal_nonempty(
        hash in "[a-f0-9]{64}",   // SHA-256-like hex strings
    ) {
        // Same hash → valid.
        prop_assert!(is_cache_valid(&hash, &hash));

        // Empty current → always invalid.
        prop_assert!(!is_cache_valid(&hash, ""));

        // Truncated current (different) → invalid.
        if hash.len() > 1 {
            prop_assert!(!is_cache_valid(&hash, &hash[..hash.len()-1]));
        }
    }

    /// Two distinct hashes are never cache-valid (no false positives).
    #[test]
    fn prop_cache_distinct_hashes_invalid(
        h1 in "[a-f0-9]{32}",
        h2 in "[a-f0-9]{32}",
    ) {
        // If they happen to be equal, the property trivially holds the other way.
        if h1 != h2 {
            prop_assert!(!is_cache_valid(&h1, &h2));
            prop_assert!(!is_cache_valid(&h2, &h1));
        }
    }
}
