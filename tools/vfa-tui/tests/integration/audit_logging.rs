//! Integration tests for audit logging of headless runs (Task 11.2) and applied
//! trust-boundary overrides (Task 7.8).
//! Validates: Requirements 12.5, 14.7

use vfa_tui::headless::reporter::record_headless_audit;
use vfa_tui::models::audit::AuditEventType;
use vfa_tui::models::report::ReportType;
use vfa_tui::persistence::audit::AuditLogger;
use vfa_tui::persistence::index::IndexManager;
use vfa_tui::policy::trust::{log_trust_overrides, WorkspaceTrustOverride};

fn audit_count(mgr: &IndexManager) -> i64 {
    mgr.write_conn()
        .query_row("SELECT COUNT(*) FROM audit_log", [], |r| r.get(0))
        .unwrap()
}

fn operator_of(mgr: &IndexManager, id: i64) -> String {
    mgr.write_conn()
        .query_row("SELECT operator FROM audit_log WHERE id = ?1", [id], |r| r.get(0))
        .unwrap()
}

#[test]
fn headless_run_is_recorded_with_headless_operator() {
    let mgr = IndexManager::open_in_memory().expect("open");
    let entry = record_headless_audit(&mgr, &[ReportType::Coverage, ReportType::Summary], 0)
        .expect("record headless audit");

    assert_eq!(entry.operator, "headless");
    assert_eq!(entry.subject, "headless_report");
    assert_eq!(entry.event_type, AuditEventType::OperatorAction);
    // Details carry the report types and exit code.
    assert_eq!(entry.details["exit_code"], 0);
    assert_eq!(entry.details["report_types"][0], "coverage");

    assert_eq!(audit_count(&mgr), 1);
    assert_eq!(operator_of(&mgr, entry.id), "headless");

    // The hash chain remains valid after the headless entry.
    AuditLogger::from_manager(&mgr)
        .expect("logger")
        .verify_chain()
        .expect("chain valid");
}

#[test]
fn applied_trust_overrides_are_audited() {
    let mgr = IndexManager::open_in_memory().expect("open");
    let mut logger = AuditLogger::new(&mgr, String::new());

    let overrides = vec![
        WorkspaceTrustOverride {
            mcp_ref_id: "aws-official-mcp".to_string(),
            reason: "approved for platform team".to_string(),
            approver: "lead@example.com".to_string(),
        },
        WorkspaceTrustOverride {
            mcp_ref_id: "azure-official-mcp".to_string(),
            reason: "sandbox only".to_string(),
            approver: "secops@example.com".to_string(),
        },
    ];

    let entries = log_trust_overrides(&mut logger, "team-a", &overrides).expect("log overrides");

    assert_eq!(entries.len(), 2);
    for e in &entries {
        assert_eq!(e.event_type, AuditEventType::ConfigChange);
        assert_eq!(e.details["workspace"], "team-a");
    }
    assert_eq!(entries[0].details["mcp_ref_id"], "aws-official-mcp");
    assert_eq!(entries[0].details["approver"], "lead@example.com");

    assert_eq!(audit_count(&mgr), 2);
    // Chain integrity preserved across the two appended overrides.
    AuditLogger::from_manager(&mgr)
        .expect("logger")
        .verify_chain()
        .expect("chain valid");
}

#[test]
fn no_overrides_logs_nothing() {
    let mgr = IndexManager::open_in_memory().expect("open");
    let mut logger = AuditLogger::new(&mgr, String::new());
    let entries = log_trust_overrides(&mut logger, "team-a", &[]).expect("log");
    assert!(entries.is_empty());
    assert_eq!(audit_count(&mgr), 0);
}
