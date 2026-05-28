use std::time::Duration;

use vfa_tui::subprocess::executor::SubprocessExecutor;

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
