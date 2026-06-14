//! Integration test 13.6 — SQLite persistence: write → restart → read, schema
//! migration to the latest version, audit append-only enforcement, and
//! workspace-scan staleness round-trips.
//! Validates: Requirements 19.1–19.9, 14.x

use vfa_tui::models::audit::AuditEventType;
use vfa_tui::persistence::audit::AuditLogger;
use vfa_tui::persistence::index::IndexManager;

fn db_path(dir: &tempfile::TempDir) -> String {
    dir.path().join("index.sqlite").to_string_lossy().into_owned()
}

#[test]
fn fresh_open_migrates_to_latest_version() {
    let mgr = IndexManager::open_in_memory().expect("open in-memory");
    // Three migrations are defined (001, 002, 003).
    assert_eq!(mgr.schema_version, 3, "fresh db should migrate to v3");
    // Re-running migrate() is idempotent.
    assert_eq!(mgr.migrate().expect("re-migrate"), 3);
}

#[test]
fn audit_entries_survive_restart() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = db_path(&dir);

    // Session 1: open, write two audit entries.
    {
        let mgr = IndexManager::open(&path).expect("open 1");
        assert_eq!(mgr.schema_version, 3);
        let mut logger = AuditLogger::new(&mgr, String::new());
        logger
            .log(AuditEventType::OperatorAction, "subject-1", serde_json::json!({"n": 1}), "alice")
            .expect("log 1");
        logger
            .log(AuditEventType::PolicyEvaluation, "subject-2", serde_json::json!({"n": 2}), "bob")
            .expect("log 2");
    }

    // Session 2: reopen the same file; entries must still be there and the
    // hash chain must verify.
    {
        let mgr = IndexManager::open(&path).expect("open 2");
        assert_eq!(mgr.schema_version, 3, "version preserved across restart");

        let conn = mgr.read_connection().expect("read conn");
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM audit_log", [], |r| r.get(0))
            .expect("count");
        assert_eq!(count, 2, "both audit entries persisted across restart");

        let logger = AuditLogger::from_manager(&mgr).expect("from_manager");
        logger.verify_chain().expect("chain valid after restart");
    }
}

#[test]
fn audit_log_is_append_only() {
    let mgr = IndexManager::open_in_memory().expect("open");
    let mut logger = AuditLogger::new(&mgr, String::new());
    logger
        .log(AuditEventType::OperatorAction, "subj", serde_json::json!({}), "op")
        .expect("log");

    let conn = mgr.write_conn();

    // UPDATE must be rejected by the append-only trigger.
    let update = conn.execute("UPDATE audit_log SET operator = 'mallory' WHERE id = 1", []);
    assert!(update.is_err(), "UPDATE on audit_log must be rejected");

    // DELETE must be rejected by the append-only trigger.
    let delete = conn.execute("DELETE FROM audit_log WHERE id = 1", []);
    assert!(delete.is_err(), "DELETE on audit_log must be rejected");

    // The row is still intact.
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM audit_log", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn workspace_scan_staleness_round_trip() {
    let mgr = IndexManager::open_in_memory().expect("open");
    let conn = mgr.write_conn();
    conn.execute(
        "INSERT INTO workspace_scans \
         (workspace_path, workspace_name, last_scan_ts, scan_duration_ms, asset_count, fs_mtime_at_scan) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params!["/ws/a", "a", "2026-06-14T00:00:00Z", 10_i64, 3_i64, 1000_i64],
    )
    .expect("insert scan");

    // Same mtime → not stale; newer mtime → stale; unknown workspace → stale.
    assert!(!mgr.is_scan_stale("/ws/a", 1000), "same mtime is fresh");
    assert!(mgr.is_scan_stale("/ws/a", 2000), "newer mtime is stale");
    assert!(mgr.is_scan_stale("/ws/unknown", 1000), "never-scanned workspace is stale");

    let cached = mgr.load_cached_scan_paths();
    assert!(cached.contains(&"/ws/a".to_string()), "cached scan path returned");
}
