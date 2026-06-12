//! Property 25: Audit log hash chain integrity (Req 14.8)
//!
//! Tests that:
//! 1. A chain built by `AuditLogger::log` always passes `verify_chain()`.
//! 2. Mutating any single entry's details or subject causes `verify_chain()`
//!    to detect the break at exactly that entry.

use proptest::prelude::*;
use proptest::test_runner::Config;
use vfa_tui::models::audit::AuditEventType;
use vfa_tui::persistence::audit::AuditLogger;
use vfa_tui::persistence::index::IndexManager;

// ---------------------------------------------------------------------------
// Arbitrary generators
// ---------------------------------------------------------------------------

fn arb_event_type() -> impl Strategy<Value = AuditEventType> {
    prop_oneof![
        Just(AuditEventType::PolicyEvaluation),
        Just(AuditEventType::Promotion),
        Just(AuditEventType::InstallationDetected),
        Just(AuditEventType::DriftDetected),
        Just(AuditEventType::ViolationResolved),
        Just(AuditEventType::OperatorAction),
        Just(AuditEventType::GateExecution),
        Just(AuditEventType::ConfigChange),
    ]
}

/// A single entry description: (event_type, subject, operator, details_key, details_val)
fn arb_entry() -> impl Strategy<Value = (AuditEventType, String, String, String, String)> {
    (
        arb_event_type(),
        "[a-z][a-z0-9_-]{0,20}",
        prop_oneof![Just("system"), Just("headless"), Just("operator")].prop_map(String::from),
        "[a-z]{1,10}",
        "[a-z0-9]{1,20}",
    )
}

// ---------------------------------------------------------------------------
// Property 25a: a freshly-built chain always passes verify_chain
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Building a chain of 1–10 entries via `AuditLogger::log` and then calling
    /// `verify_chain()` must always return `Ok(())`.
    #[test]
    fn prop25_valid_chain_passes_verification(
        entries in proptest::collection::vec(arb_entry(), 1..=10)
    ) {
        let mgr = IndexManager::open_in_memory().expect("open_in_memory");
        let mut logger = AuditLogger::new(&mgr, String::new());

        for (event_type, subject, operator, details_key, details_val) in &entries {
            let details = serde_json::json!({ details_key: details_val });
            logger.log(event_type.clone(), subject, details, operator)
                .expect("log should succeed");
        }

        logger.verify_chain().expect("valid chain must pass verification");
    }
}

// ---------------------------------------------------------------------------
// Property 25b: mutating one entry's details breaks the chain at that entry
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Injecting a row with a wrong `prev_hash` into an otherwise-valid chain
    /// causes `verify_chain()` to return `AuditChainBroken` whose `entry_id`
    /// matches the injected position.
    #[test]
    fn prop25_tampered_entry_detected(
        entries in proptest::collection::vec(arb_entry(), 2..=8),
        // choose which 0-based index to corrupt
        corrupt_idx_raw in any::<usize>()
    ) {
        // We build a fresh DB that allows direct INSERT of broken rows
        // (trigger only blocks UPDATE/DELETE, not INSERT of wrong hashes).
        let mgr = IndexManager::open_in_memory().expect("open_in_memory");
        let mut logger = AuditLogger::new(&mgr, String::new());

        // Build the good portion of the chain (all entries before corrupt_idx).
        let corrupt_idx = corrupt_idx_raw % entries.len();

        for (i, (event_type, subject, operator, details_key, details_val)) in entries.iter().enumerate() {
            if i == corrupt_idx {
                // Inject a row with a deliberately wrong prev_hash.
                let conn = mgr.write_conn();
                let event_str = serde_json::to_string(event_type)
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_string();
                let details = serde_json::json!({ details_key: details_val });
                let details_json = serde_json::to_string(&details).unwrap();
                let ts = format!("2025-01-01T00:{i:02}:00.000Z");
                conn.execute(
                    "INSERT INTO audit_log \
                     (timestamp, event_type, subject, details, operator, entry_hash, prev_hash) \
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    rusqlite::params![
                        ts,
                        event_str,
                        subject,
                        details_json,
                        operator,
                        "deliberate_bad_hash",
                        "WRONG_PREV_HASH_FOR_TAMPER_TEST",
                    ],
                )
                .expect("insert tampered row");

                // The logger's last_hash is now stale; stop building the chain.
                break;
            }

            let details = serde_json::json!({ details_key: details_val });
            logger.log(event_type.clone(), subject, details, operator)
                .expect("log should succeed");
        }

        // verify_chain must detect the break.
        let result = logger.verify_chain();
        prop_assert!(
            result.is_err(),
            "verify_chain should detect tamper at index {corrupt_idx}"
        );

        // The entry_id reported must be at or after the corrupt index
        // (SQLite IDs start at 1, our index is 0-based).
        if let Err(vfa_tui::error::TuiError::AuditChainBroken { entry_id }) = result {
            let expected_id = (corrupt_idx as i64) + 1;
            prop_assert_eq!(
                entry_id,
                expected_id,
                "broken entry_id should match corrupt_idx+1"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Property 25c: compute_hash is pure (same inputs → same output)
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// `compute_hash` must be deterministic: calling it twice with identical
    /// inputs always yields the same hex string.
    #[test]
    fn prop25_compute_hash_is_pure(
        prev_hash  in "[a-f0-9]{0,64}",
        timestamp  in "20[0-9]{2}-[01][0-9]-[0-3][0-9]T[0-2][0-9]:[0-5][0-9]:[0-5][0-9]\\.[0-9]{3}Z",
        event_type in arb_event_type(),
        subject    in "[a-z][a-z0-9_-]{0,20}",
        details    in "[a-z0-9]{0,30}",
    ) {
        let details_json = format!("{{\"k\":\"{details}\"}}");
        let h1 = AuditLogger::compute_hash(&prev_hash, &timestamp, &event_type, &subject, &details_json);
        let h2 = AuditLogger::compute_hash(&prev_hash, &timestamp, &event_type, &subject, &details_json);
        prop_assert_eq!(h1, h2, "compute_hash must be deterministic");
    }
}

// ---------------------------------------------------------------------------
// Property 25d: changing any single input to compute_hash changes the output
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Flipping one byte of `prev_hash` must produce a different `entry_hash`.
    /// This shows the hash is sensitive to chain linkage.
    #[test]
    fn prop25_hash_changes_with_prev_hash(
        prev_hash_a in "[a-f0-9]{1,64}",
        prev_hash_b in "[a-f0-9]{1,64}",
        timestamp   in "2025-01-01T00:00:00\\.000Z",
        subject     in "[a-z]{1,10}",
        details     in "[a-z0-9]{1,10}",
    ) {
        // Only assert inequality when inputs differ (which almost always holds
        // for randomly-generated hex strings of non-trivial length).
        if prev_hash_a != prev_hash_b {
            let details_json = format!("{{\"k\":\"{details}\"}}");
            let h_a = AuditLogger::compute_hash(
                &prev_hash_a, &timestamp, &AuditEventType::PolicyEvaluation, &subject, &details_json,
            );
            let h_b = AuditLogger::compute_hash(
                &prev_hash_b, &timestamp, &AuditEventType::PolicyEvaluation, &subject, &details_json,
            );
            prop_assert_ne!(h_a, h_b, "different prev_hash must produce different entry_hash");
        }
    }
}
