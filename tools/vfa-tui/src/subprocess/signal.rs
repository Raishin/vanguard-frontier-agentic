use std::time::Duration;
use tokio::process::Child;

/// Gracefully terminate a child process.
///
/// Sends SIGTERM (Unix) or kills (Windows), waits up to 5 seconds,
/// then sends SIGKILL (Unix) or force-kills (Windows) if still running.
pub async fn graceful_kill(child: &mut Child) -> anyhow::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;

        if let Some(id) = child.id() {
            // Send SIGTERM
            unsafe {
                libc::kill(id as libc::pid_t, libc::SIGTERM);
            }

            // Wait up to 5 seconds for exit
            let timeout = Duration::from_secs(5);
            match tokio::time::timeout(timeout, child.wait()).await {
                Ok(Ok(status)) => {
                    // Process exited after SIGTERM
                    let _ = status.signal();
                    return Ok(());
                }
                Ok(Err(e)) => {
                    return Err(anyhow::anyhow!("error waiting after SIGTERM: {e}"));
                }
                Err(_) => {
                    // Timed out waiting for SIGTERM, send SIGKILL
                    child.kill().await?;
                }
            }
        } else {
            // No PID available, process may have already exited
            child.kill().await.ok();
        }
    }

    #[cfg(not(unix))]
    {
        // On Windows, just kill the process directly
        child.kill().await?;
    }

    Ok(())
}
