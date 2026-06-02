use std::time::Duration;

use vfa_tui::subprocess::executor::SubprocessExecutor;

// =============================================================================
// Basic spawning and exit code tests
// =============================================================================

#[tokio::test]
async fn spawn_echo_captures_hello() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "echo",
        &["hello".to_string()],
        tmp.path(),
        Duration::from_secs(10),
    )
    .await
    .unwrap();

    let code = handle.wait().await.unwrap();
    assert_eq!(code, 0);

    // Allow async reader tasks to deliver
    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut lines = Vec::new();
    while let Some(line) = handle.try_recv_stdout() {
        lines.push(line.content);
    }
    assert!(
        lines.contains(&"hello".to_string()),
        "expected 'hello' in stdout, got: {lines:?}"
    );
}

#[tokio::test]
async fn exit_code_capture_nonzero() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn("false", &[], tmp.path(), Duration::from_secs(10))
        .await
        .unwrap();

    let code = handle.wait().await.unwrap();
    assert_ne!(code, 0, "expected non-zero exit code from 'false'");
}

#[tokio::test]
async fn exit_code_capture_zero() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn("true", &[], tmp.path(), Duration::from_secs(10))
        .await
        .unwrap();

    let code = handle.wait().await.unwrap();
    assert_eq!(code, 0, "expected zero exit code from 'true'");
}

// =============================================================================
// Various exit codes via inline scripts (Requirements 6.4, 20.6)
// =============================================================================

/// Test that arbitrary exit codes (not just 0/1) are captured correctly.
/// Validates: Requirements 6.4, 20.6
#[tokio::test]
async fn exit_code_42_captured() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "sh",
        &["-c".to_string(), "exit 42".to_string()],
        tmp.path(),
        Duration::from_secs(10),
    )
    .await
    .unwrap();

    let code = handle.wait().await.unwrap();
    assert_eq!(code, 42, "expected exit code 42");
}

/// Test exit code 127 (command not found convention).
/// Validates: Requirements 6.4, 20.6
#[tokio::test]
async fn exit_code_127_captured() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "sh",
        &["-c".to_string(), "exit 127".to_string()],
        tmp.path(),
        Duration::from_secs(10),
    )
    .await
    .unwrap();

    let code = handle.wait().await.unwrap();
    assert_eq!(code, 127, "expected exit code 127");
}

/// Test exit code 255 (maximum single-byte exit code).
/// Validates: Requirements 6.4, 20.6
#[tokio::test]
async fn exit_code_255_captured() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "sh",
        &["-c".to_string(), "exit 255".to_string()],
        tmp.path(),
        Duration::from_secs(10),
    )
    .await
    .unwrap();

    let code = handle.wait().await.unwrap();
    assert_eq!(code, 255, "expected exit code 255");
}

/// Test that a script producing output before exiting non-zero still captures both.
/// Validates: Requirements 6.2, 6.4, 20.6
#[tokio::test]
async fn exit_nonzero_with_output() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "sh",
        &[
            "-c".to_string(),
            "echo 'validation failed: missing field' >&2; exit 3".to_string(),
        ],
        tmp.path(),
        Duration::from_secs(10),
    )
    .await
    .unwrap();

    let code = handle.wait().await.unwrap();
    assert_eq!(code, 3, "expected exit code 3");

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut stderr_lines = Vec::new();
    while let Some(line) = handle.try_recv_stderr() {
        stderr_lines.push(line.content);
    }
    assert!(
        stderr_lines.iter().any(|l| l.contains("validation failed")),
        "expected error message in stderr, got: {stderr_lines:?}"
    );
}

// =============================================================================
// stdout/stderr separation tests (Requirements 6.2, 20.3)
// =============================================================================

#[tokio::test]
async fn stdout_vs_stderr_separation() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "sh",
        &[
            "-c".to_string(),
            "echo stdout_msg && echo stderr_msg >&2".to_string(),
        ],
        tmp.path(),
        Duration::from_secs(10),
    )
    .await
    .unwrap();

    let code = handle.wait().await.unwrap();
    assert_eq!(code, 0);

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut stdout_lines = Vec::new();
    while let Some(line) = handle.try_recv_stdout() {
        stdout_lines.push(line.content);
    }

    let mut stderr_lines = Vec::new();
    while let Some(line) = handle.try_recv_stderr() {
        stderr_lines.push(line.content);
    }

    assert!(
        stdout_lines.contains(&"stdout_msg".to_string()),
        "expected 'stdout_msg' in stdout, got: {stdout_lines:?}"
    );
    assert!(
        stderr_lines.contains(&"stderr_msg".to_string()),
        "expected 'stderr_msg' in stderr, got: {stderr_lines:?}"
    );
    // Verify no cross-contamination
    assert!(
        !stdout_lines.contains(&"stderr_msg".to_string()),
        "stderr should not appear in stdout"
    );
    assert!(
        !stderr_lines.contains(&"stdout_msg".to_string()),
        "stdout should not appear in stderr"
    );
}

/// Test interleaved stdout/stderr output is captured on correct streams.
/// Validates: Requirements 6.2, 20.3
#[tokio::test]
async fn interleaved_stdout_stderr() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "sh",
        &[
            "-c".to_string(),
            "echo out1; echo err1 >&2; echo out2; echo err2 >&2".to_string(),
        ],
        tmp.path(),
        Duration::from_secs(10),
    )
    .await
    .unwrap();

    let code = handle.wait().await.unwrap();
    assert_eq!(code, 0);

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut stdout_lines = Vec::new();
    while let Some(line) = handle.try_recv_stdout() {
        stdout_lines.push(line.content);
    }

    let mut stderr_lines = Vec::new();
    while let Some(line) = handle.try_recv_stderr() {
        stderr_lines.push(line.content);
    }

    assert!(
        stdout_lines.contains(&"out1".to_string()),
        "expected 'out1' in stdout"
    );
    assert!(
        stdout_lines.contains(&"out2".to_string()),
        "expected 'out2' in stdout"
    );
    assert!(
        stderr_lines.contains(&"err1".to_string()),
        "expected 'err1' in stderr"
    );
    assert!(
        stderr_lines.contains(&"err2".to_string()),
        "expected 'err2' in stderr"
    );
}

#[tokio::test]
async fn multiline_output_captured() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "sh",
        &[
            "-c".to_string(),
            "echo line1 && echo line2 && echo line3".to_string(),
        ],
        tmp.path(),
        Duration::from_secs(10),
    )
    .await
    .unwrap();

    let code = handle.wait().await.unwrap();
    assert_eq!(code, 0);

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut lines = Vec::new();
    while let Some(line) = handle.try_recv_stdout() {
        lines.push(line.content);
    }
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "line1");
    assert_eq!(lines[1], "line2");
    assert_eq!(lines[2], "line3");
}

// =============================================================================
// Timeout tests (Requirements 6.5, 20.4)
// =============================================================================

/// Test that check_timeout detects a process exceeding its timeout.
/// Uses a very short timeout (100ms) with a long-running sleep command.
/// Validates: Requirements 6.5, 20.4
#[tokio::test]
async fn timeout_kills_long_running_process() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "sleep",
        &["60".to_string()],
        tmp.path(),
        Duration::from_millis(100),
    )
    .await
    .unwrap();

    // Process should be running initially
    assert!(handle.is_running(), "process should be running initially");
    assert!(
        !handle.is_timed_out(),
        "should not be timed out immediately"
    );

    // Wait for the timeout to elapse
    tokio::time::sleep(Duration::from_millis(150)).await;

    // Now check_timeout should detect the timeout and kill the process
    let timed_out = handle.check_timeout().await;
    assert!(timed_out, "expected check_timeout to return true");
    assert!(!handle.is_running(), "process should no longer be running");
}

/// Test that is_timed_out returns true after timeout elapses (without killing).
/// Validates: Requirements 6.5, 20.4
#[tokio::test]
async fn is_timed_out_detects_elapsed_timeout() {
    let tmp = tempfile::TempDir::new().unwrap();
    let handle = SubprocessExecutor::spawn(
        "sleep",
        &["60".to_string()],
        tmp.path(),
        Duration::from_millis(50),
    )
    .await
    .unwrap();

    // Not timed out yet
    assert!(!handle.is_timed_out());

    // Wait past the timeout
    tokio::time::sleep(Duration::from_millis(80)).await;

    // Should now report timed out
    assert!(handle.is_timed_out());
}

/// Test that a process completing before timeout does not trigger timeout.
/// Validates: Requirements 6.5, 20.4
#[tokio::test]
async fn fast_process_does_not_timeout() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "echo",
        &["quick".to_string()],
        tmp.path(),
        Duration::from_secs(10),
    )
    .await
    .unwrap();

    let code = handle.wait().await.unwrap();
    assert_eq!(code, 0);

    // After completion, check_timeout should return false
    let timed_out = handle.check_timeout().await;
    assert!(
        !timed_out,
        "completed process should not report as timed out"
    );
}

// =============================================================================
// SIGTERM → SIGKILL escalation tests (Requirements 20.5)
// =============================================================================

/// Test that cancel() terminates a running process via SIGTERM.
/// Validates: Requirements 20.5
#[tokio::test]
async fn cancel_terminates_running_process() {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut handle = SubprocessExecutor::spawn(
        "sleep",
        &["60".to_string()],
        tmp.path(),
        Duration::from_secs(300),
    )
    .await
    .unwrap();

    assert!(handle.is_running());

    // Cancel should send SIGTERM and the process should exit
    handle.cancel().await.unwrap();

    assert!(
        !handle.is_running(),
        "process should be stopped after cancel"
    );
}

/// Test that cancel() terminates descendants in the spawned process group.
/// Validates: Requirements 20.5
#[cfg(unix)]
#[tokio::test]
async fn cancel_terminates_process_group_descendants() {
    let tmp = tempfile::TempDir::new().unwrap();
    let pid_file = tmp.path().join("child.pid");
    let script = "sleep 60 & echo $! > child.pid; wait";

    let mut handle = SubprocessExecutor::spawn(
        "sh",
        &["-c".to_string(), script.to_string()],
        tmp.path(),
        Duration::from_secs(300),
    )
    .await
    .unwrap();

    for _ in 0..20 {
        if pid_file.exists() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let child_pid: i32 = std::fs::read_to_string(&pid_file)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    handle.cancel().await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    let still_running = unsafe { libc::kill(child_pid, 0) } == 0;
    assert!(
        !still_running,
        "descendant process {child_pid} should be stopped after cancel"
    );
}

/// Test SIGTERM → SIGKILL escalation for a process that traps SIGTERM.
/// The script traps SIGTERM and ignores it, so after 5s the executor
/// should escalate to SIGKILL.
/// Validates: Requirements 20.5
#[tokio::test]
async fn sigterm_to_sigkill_escalation() {
    let tmp = tempfile::TempDir::new().unwrap();

    // Script that traps SIGTERM and ignores it, continuing to sleep
    let script = "trap '' TERM; sleep 60";

    let mut handle = SubprocessExecutor::spawn(
        "sh",
        &["-c".to_string(), script.to_string()],
        tmp.path(),
        Duration::from_secs(300),
    )
    .await
    .unwrap();

    // Give the process a moment to set up the trap
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(handle.is_running());

    // Cancel sends SIGTERM, waits 5s, then SIGKILL
    // This should complete within ~6 seconds
    let start = std::time::Instant::now();
    handle.cancel().await.unwrap();
    let elapsed = start.elapsed();

    assert!(
        !handle.is_running(),
        "process should be dead after SIGKILL escalation"
    );
    // The escalation should take at least ~5 seconds (SIGTERM wait period)
    // but not much longer (SIGKILL is immediate after that)
    assert!(
        elapsed >= Duration::from_secs(4),
        "expected at least 4s for SIGTERM wait, got {elapsed:?}"
    );
    assert!(
        elapsed < Duration::from_secs(10),
        "expected less than 10s total, got {elapsed:?}"
    );
}

/// Test that timeout with a SIGTERM-ignoring process still terminates via SIGKILL.
/// Validates: Requirements 6.5, 20.4, 20.5
#[tokio::test]
async fn timeout_escalates_to_sigkill_for_trapped_process() {
    let tmp = tempfile::TempDir::new().unwrap();

    // Script that traps SIGTERM and ignores it
    let script = "trap '' TERM; sleep 60";

    let mut handle = SubprocessExecutor::spawn(
        "sh",
        &["-c".to_string(), script.to_string()],
        tmp.path(),
        Duration::from_millis(200),
    )
    .await
    .unwrap();

    // Give the process a moment to set up the trap
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Wait for timeout to elapse
    tokio::time::sleep(Duration::from_millis(150)).await;

    // check_timeout should detect timeout and kill (escalating to SIGKILL)
    let timed_out = handle.check_timeout().await;
    assert!(timed_out, "expected timeout detection");
    assert!(
        !handle.is_running(),
        "process should be dead after timeout + SIGKILL"
    );
}
