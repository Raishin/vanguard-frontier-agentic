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
            // Safety: Check that the process is still running before sending signal.
            // This mitigates PID reuse race conditions by confirming the child
            // process has not already exited.
            if let Ok(Some(_)) = child.try_wait() {
                return Ok(());
            }

            // Send SIGTERM
            let ret = unsafe { libc::kill(id as libc::pid_t, libc::SIGTERM) };
            if ret == -1 {
                // ESRCH means the process already exited - not an error
                let err = std::io::Error::last_os_error();
                if err.raw_os_error() == Some(libc::ESRCH) {
                    return Ok(());
                }
                return Err(anyhow::anyhow!("failed to send SIGTERM: {err}"));
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
