// Feature: rust-tui, Property 13: Catalog entries with control bytes are skipped
//
// For any catalog JSON array where some entries contain control bytes
// (0x00–0x08, 0x0B–0x0C, 0x0E–0x1F, 0x7F) in string field values, the catalog
// loader SHALL skip those entries and load all remaining clean entries. The count
// of loaded entries plus the count of skipped entries SHALL equal the total
// entries in the source array.
//
// **Validates: Requirements 10.3**

use proptest::prelude::*;
use proptest::test_runner::Config;
use std::fs;
use tempfile::TempDir;
use vfa_tui::catalog::loader::load_agents;
use vfa_tui::error::TuiError;

/// Control bytes that should cause an entry to be skipped.
/// Range: 0x00–0x08, 0x0B–0x0C, 0x0E–0x1F, 0x7F
const CONTROL_BYTES: &[char] = &[
    '\x00', '\x01', '\x02', '\x03', '\x04', '\x05', '\x06', '\x07', '\x08', '\x0B', '\x0C', '\x0E',
    '\x0F', '\x10', '\x11', '\x12', '\x13', '\x14', '\x15', '\x16', '\x17', '\x18', '\x19', '\x1A',
    '\x1B', '\x1C', '\x1D', '\x1E', '\x1F', '\x7F',
];

/// Build a valid agent JSON object with the given id and name.
/// All required fields are populated with clean values.
fn make_clean_agent_json(id: &str, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "type": "agent",
        "provider": "aws",
        "harnesses": ["codex"],
        "summary": "A test agent for property testing",
        "source_type": "original",
        "official_docs": [],
        "security_notes": "none",
        "last_verified": "2025-01-01",
        "path": "agents/aws/test-agent"
    })
}

/// Inject a control byte into a specific string field of an agent JSON object.
fn inject_control_byte(agent: &mut serde_json::Value, field: &str, control_char: char) {
    if let Some(val) = agent.get_mut(field) {
        if let Some(s) = val.as_str() {
            let tainted = format!("{}{}", s, control_char);
            *val = serde_json::Value::String(tainted);
        }
    }
}

/// Strategy to generate a control byte character from the disallowed set.
fn control_byte_strategy() -> impl Strategy<Value = char> {
    prop::sample::select(CONTROL_BYTES)
}

/// Strategy to select which string field to inject the control byte into.
fn injectable_field_strategy() -> impl Strategy<Value = &'static str> {
    prop::sample::select(
        &[
            "id",
            "name",
            "summary",
            "security_notes",
            "last_verified",
            "path",
        ][..],
    )
}

/// Strategy to generate a mix of clean and tainted agent entries.
/// Returns (total_count, tainted_indices, control_chars, fields).
fn agent_mix_strategy() -> impl Strategy<Value = (usize, Vec<(usize, char, &'static str)>)> {
    // Generate 2-10 total entries
    (2usize..=10).prop_flat_map(|total| {
        // Generate a subset of indices to taint (at least 1, at most total)
        let taint_count = 1..=total;
        (
            Just(total),
            proptest::collection::vec(
                (
                    0usize..total,
                    control_byte_strategy(),
                    injectable_field_strategy(),
                ),
                taint_count,
            ),
        )
    })
}

/// Set up a temp workspace with a catalog/agents.json containing the given entries.
fn setup_workspace(agents_json: &str) -> TempDir {
    let tmp = TempDir::new().unwrap();
    let catalog_dir = tmp.path().join("catalog");
    fs::create_dir_all(&catalog_dir).unwrap();
    fs::write(catalog_dir.join("agents.json"), agents_json).unwrap();
    tmp
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Property 13: For any catalog array where some entries have control bytes
    /// injected, loaded + skipped = total.
    #[test]
    fn tainted_entries_skipped_clean_entries_loaded(
        (total, taint_specs) in agent_mix_strategy()
    ) {
        // Build the agent array
        let mut agents: Vec<serde_json::Value> = (0..total)
            .map(|i| make_clean_agent_json(
                &format!("agent-{}", i),
                &format!("Agent {}", i),
            ))
            .collect();

        // Determine which indices are actually tainted (deduplicate)
        let mut tainted_indices = std::collections::HashSet::new();
        for (idx, control_char, field) in &taint_specs {
            let idx = *idx % total; // ensure in bounds
            inject_control_byte(&mut agents[idx], field, *control_char);
            tainted_indices.insert(idx);
        }

        let expected_tainted = tainted_indices.len();
        let expected_clean = total - expected_tainted;

        // Serialize and write to temp workspace
        let json_str = serde_json::to_string(&agents).unwrap();
        let tmp = setup_workspace(&json_str);

        // Load agents
        let (loaded, errors) = load_agents(tmp.path());

        // Count TaintedEntry errors
        let tainted_error_count = errors.iter().filter(|e| matches!(e, TuiError::TaintedEntry { .. })).count();

        // Property: loaded + skipped = total
        prop_assert_eq!(
            loaded.len() + tainted_error_count,
            total,
            "loaded ({}) + skipped ({}) != total ({})",
            loaded.len(),
            tainted_error_count,
            total
        );

        // Property: loaded count matches expected clean entries
        prop_assert_eq!(
            loaded.len(),
            expected_clean,
            "loaded ({}) != expected clean ({})",
            loaded.len(),
            expected_clean
        );

        // Property: tainted error count matches expected tainted entries
        prop_assert_eq!(
            tainted_error_count,
            expected_tainted,
            "tainted errors ({}) != expected tainted ({})",
            tainted_error_count,
            expected_tainted
        );
    }

    /// Property 13b: A fully clean catalog array loads all entries with no
    /// tainted errors.
    #[test]
    fn all_clean_entries_loaded(total in 1usize..=10) {
        let agents: Vec<serde_json::Value> = (0..total)
            .map(|i| make_clean_agent_json(
                &format!("clean-agent-{}", i),
                &format!("Clean Agent {}", i),
            ))
            .collect();

        let json_str = serde_json::to_string(&agents).unwrap();
        let tmp = setup_workspace(&json_str);

        let (loaded, errors) = load_agents(tmp.path());

        let tainted_error_count = errors.iter().filter(|e| matches!(e, TuiError::TaintedEntry { .. })).count();

        // No tainted errors for clean data
        prop_assert_eq!(tainted_error_count, 0);
        // All entries loaded
        prop_assert_eq!(loaded.len(), total);
        // loaded + skipped = total
        prop_assert_eq!(loaded.len() + tainted_error_count, total);
    }

    /// Property 13c: When ALL entries are tainted, zero entries are loaded and
    /// all are reported as skipped.
    #[test]
    fn all_tainted_entries_skipped(
        total in 1usize..=8,
        control_char in control_byte_strategy(),
        field in injectable_field_strategy(),
    ) {
        let mut agents: Vec<serde_json::Value> = (0..total)
            .map(|i| make_clean_agent_json(
                &format!("tainted-agent-{}", i),
                &format!("Tainted Agent {}", i),
            ))
            .collect();

        // Taint every entry
        for agent in &mut agents {
            inject_control_byte(agent, field, control_char);
        }

        let json_str = serde_json::to_string(&agents).unwrap();
        let tmp = setup_workspace(&json_str);

        let (loaded, errors) = load_agents(tmp.path());

        let tainted_error_count = errors.iter().filter(|e| matches!(e, TuiError::TaintedEntry { .. })).count();

        // Zero loaded
        prop_assert_eq!(loaded.len(), 0);
        // All skipped
        prop_assert_eq!(tainted_error_count, total);
        // loaded + skipped = total
        prop_assert_eq!(loaded.len() + tainted_error_count, total);
    }
}
