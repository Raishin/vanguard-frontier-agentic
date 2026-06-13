//! Single-writer task for all SQLite mutations.
//!
//! Implements Req 19.8: all write operations are funnelled through one tokio
//! task that owns the write `Connection`, preventing contention while allowing
//! multiple concurrent read connections for the UI thread and background tasks.

use tokio::sync::{mpsc, oneshot};

use crate::error::TuiError;
use crate::models::audit::AuditEntry;
use crate::persistence::index::IndexManager;

// ---------------------------------------------------------------------------
// DbCommand — the message type sent to the writer task
// ---------------------------------------------------------------------------

/// Commands dispatched to the single-writer SQLite task.
#[derive(Debug)]
pub enum DbCommand {
    /// Upsert a workspace-level scan summary row.
    RecordScan {
        workspace_path: String,
        workspace_name: String,
        last_scan_ts: String,
        scan_duration_ms: i64,
        asset_count: i64,
        fs_mtime_at_scan: i64,
    },
    /// Record the result of a gate execution.
    RecordGateResult {
        gate_name: String,
        status: String,
        exit_code: Option<i32>,
        duration_ms: i64,
        timestamp: String,
        catalog_hash: String,
        output_excerpt: Option<String>,
    },
    /// Append a pre-computed audit entry (hash chain already linked).
    AppendAudit(AuditEntry),
    /// Upsert a content-hash cache entry.
    UpdateContentHash { path: String, hash: String },
    /// Flush all pending work; reply on `ack` when done.
    Flush(oneshot::Sender<()>),
    /// Graceful shutdown: drain the queue then exit the task.
    Shutdown,
}

// ---------------------------------------------------------------------------
// WriterHandle — returned to callers
// ---------------------------------------------------------------------------

/// Sender half of the writer task channel.
pub type WriterHandle = mpsc::Sender<DbCommand>;

// ---------------------------------------------------------------------------
// spawn_writer
// ---------------------------------------------------------------------------

/// Spawn the single-writer background task and return its channel sender.
///
/// The task owns a dedicated write `Connection` opened from `db_path`.
/// If `db_path` is `None` an in-memory `IndexManager` is used — useful for
/// tests (Req 19.5).
///
/// # Errors
/// Returns `Err` if the `IndexManager` cannot be opened.
pub fn spawn_writer(
    db_path: Option<&str>,
    channel_capacity: usize,
) -> Result<WriterHandle, TuiError> {
    let mgr = match db_path {
        Some(p) => IndexManager::open(p)?,
        None => IndexManager::open_in_memory()?,
    };

    let (tx, mut rx) = mpsc::channel::<DbCommand>(channel_capacity);

    // `IndexManager` / `Connection` are `!Send`, so we must run the writer
    // loop on a dedicated blocking thread (not a tokio async task).
    // `spawn_blocking` accepts `!Send` closures; `blocking_recv()` is the
    // sync equivalent of `recv().await`.
    tokio::task::spawn_blocking(move || {
        // The IndexManager is moved into the task; only this task touches the
        // write connection from here on.
        let conn = mgr.write_conn();

        while let Some(cmd) = rx.blocking_recv() {
            match cmd {
                DbCommand::RecordScan {
                    workspace_path,
                    workspace_name,
                    last_scan_ts,
                    scan_duration_ms,
                    asset_count,
                    fs_mtime_at_scan,
                } => {
                    let _ = conn.execute(
                        "INSERT OR REPLACE INTO workspace_scans \
                         (workspace_path, workspace_name, last_scan_ts, \
                          scan_duration_ms, asset_count, fs_mtime_at_scan) \
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        rusqlite::params![
                            workspace_path,
                            workspace_name,
                            last_scan_ts,
                            scan_duration_ms,
                            asset_count,
                            fs_mtime_at_scan,
                        ],
                    );
                }

                DbCommand::RecordGateResult {
                    gate_name,
                    status,
                    exit_code,
                    duration_ms,
                    timestamp,
                    catalog_hash,
                    output_excerpt,
                } => {
                    let _ = conn.execute(
                        "INSERT INTO gate_history \
                         (gate_name, status, exit_code, duration_ms, timestamp, \
                          catalog_hash, output_excerpt) \
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                        rusqlite::params![
                            gate_name,
                            status,
                            exit_code,
                            duration_ms,
                            timestamp,
                            catalog_hash,
                            output_excerpt,
                        ],
                    );
                }

                DbCommand::AppendAudit(entry) => {
                    let event_type_str =
                        serde_json::to_string(&entry.event_type).unwrap_or_default();
                    let details_str =
                        serde_json::to_string(&entry.details).unwrap_or_else(|_| "{}".to_string());
                    let _ = conn.execute(
                        "INSERT INTO audit_log \
                         (timestamp, event_type, subject, details, operator, \
                          entry_hash, prev_hash) \
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                        rusqlite::params![
                            entry.timestamp,
                            event_type_str.trim_matches('"'),
                            entry.subject,
                            details_str,
                            entry.operator,
                            entry.entry_hash,
                            entry.prev_hash,
                        ],
                    );
                }

                DbCommand::UpdateContentHash { path, hash } => {
                    let _ = conn.execute(
                        "INSERT OR REPLACE INTO content_hashes (path, hash, last_checked) \
                         VALUES (?1, ?2, datetime('now'))",
                        rusqlite::params![path, hash],
                    );
                }

                DbCommand::Flush(ack) => {
                    let _ = ack.send(());
                }

                DbCommand::Shutdown => {
                    break;
                }
            }
        }
    });

    Ok(tx)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::audit::AuditEventType;
    use crate::persistence::index::IndexManager;

    fn make_audit_entry(id: i64, entry_hash: &str, prev_hash: &str) -> AuditEntry {
        AuditEntry {
            id,
            timestamp: "2025-01-01T00:00:00.000Z".to_string(),
            event_type: AuditEventType::PolicyEvaluation,
            subject: "ws".to_string(),
            details: serde_json::json!({}),
            operator: "system".to_string(),
            entry_hash: entry_hash.to_string(),
            prev_hash: prev_hash.to_string(),
        }
    }

    #[tokio::test]
    async fn spawn_writer_inmemory_flush_roundtrip() {
        let tx = spawn_writer(None, 32).expect("spawn_writer");

        // Send a scan record.
        tx.send(DbCommand::RecordScan {
            workspace_path: "/ws/test".to_string(),
            workspace_name: "test".to_string(),
            last_scan_ts: "2025-01-01T00:00:00Z".to_string(),
            scan_duration_ms: 100,
            asset_count: 5,
            fs_mtime_at_scan: 1000,
        })
        .await
        .unwrap();

        // Flush and wait for ack.
        let (ack_tx, ack_rx) = oneshot::channel();
        tx.send(DbCommand::Flush(ack_tx)).await.unwrap();
        ack_rx.await.expect("flush ack");
    }

    #[tokio::test]
    async fn spawn_writer_sends_audit_entry() {
        let tx = spawn_writer(None, 32).expect("spawn_writer");

        let entry = make_audit_entry(0, "ehash", "");
        tx.send(DbCommand::AppendAudit(entry)).await.unwrap();

        let (ack_tx, ack_rx) = oneshot::channel();
        tx.send(DbCommand::Flush(ack_tx)).await.unwrap();
        ack_rx.await.expect("flush ack");
    }

    #[tokio::test]
    async fn spawn_writer_shutdown_drops_channel() {
        let tx = spawn_writer(None, 32).expect("spawn_writer");
        tx.send(DbCommand::Shutdown).await.unwrap();
        // After shutdown the task exits; subsequent sends fail.
        // Allow a brief yield for the task to process Shutdown.
        tokio::task::yield_now().await;
    }

    /// Verify that the writer task properly handles UpdateContentHash.
    #[tokio::test]
    async fn spawn_writer_update_content_hash() {
        let tx = spawn_writer(None, 32).expect("spawn_writer");

        tx.send(DbCommand::UpdateContentHash {
            path: "/catalog/agents.json".to_string(),
            hash: "abc123".to_string(),
        })
        .await
        .unwrap();

        let (ack_tx, ack_rx) = oneshot::channel();
        tx.send(DbCommand::Flush(ack_tx)).await.unwrap();
        ack_rx.await.expect("flush ack");
    }

    /// Verify record gate result is accepted.
    #[tokio::test]
    async fn spawn_writer_record_gate_result() {
        let tx = spawn_writer(None, 32).expect("spawn_writer");

        tx.send(DbCommand::RecordGateResult {
            gate_name: "lint".to_string(),
            status: "pass".to_string(),
            exit_code: Some(0),
            duration_ms: 1200,
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            catalog_hash: "chash".to_string(),
            output_excerpt: None,
        })
        .await
        .unwrap();

        let (ack_tx, ack_rx) = oneshot::channel();
        tx.send(DbCommand::Flush(ack_tx)).await.unwrap();
        ack_rx.await.expect("flush ack");
    }

    /// Confirm writer task works with a real file-based DB.
    #[tokio::test]
    async fn spawn_writer_file_based() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("writer_test.db");
        let path_str = path.to_str().expect("utf8 path").to_string();

        // Pre-create and migrate.
        let _mgr = IndexManager::open(&path_str).expect("open");

        let tx = spawn_writer(Some(&path_str), 32).expect("spawn writer");

        tx.send(DbCommand::RecordScan {
            workspace_path: "/ws/file".to_string(),
            workspace_name: "file-ws".to_string(),
            last_scan_ts: "2025-01-01T00:00:00Z".to_string(),
            scan_duration_ms: 200,
            asset_count: 3,
            fs_mtime_at_scan: 5000,
        })
        .await
        .unwrap();

        let (ack_tx, ack_rx) = oneshot::channel();
        tx.send(DbCommand::Flush(ack_tx)).await.unwrap();
        ack_rx.await.expect("flush ack");
    }
}
