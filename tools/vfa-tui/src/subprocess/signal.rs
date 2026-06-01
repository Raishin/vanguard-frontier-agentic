use std::time::Duration;
use tokio::process::Child;

/// Gracefully terminate a child process.
///
/// On Linux 5.3+, uses pidfd_open + pidfd_send_signal for race-free signaling
/// (immune to PID reuse). Falls back to kill(2) on older kernels.
/// On Windows, kills the process directly.
/// Waits up to 5 seconds after SIGTERM, then escalates to SIGKILL.
pub async fn graceful_kill(child: &mut Child) -> anyhow::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;

        if let Some(id) = child.id() {
            // Check that the process is still running before sending signal.
            if let Ok(Some(_)) = child.try_wait() {
                return Ok(());
            }

            // Attempt race-free signaling via pidfd (Linux 5.3+).
            // If pidfd_open fails (older kernel, permission denied), fall back to kill(2).
            let sigterm_sent = try_pidfd_signal(id, libc::SIGTERM).unwrap_or_else(|| {
                // Fallback: traditional kill(2) with TOCTOU caveat
                let ret = unsafe { libc::kill(id as libc::pid_t, libc::SIGTERM) };
                if ret == -1 {
                    let err = std::io::Error::last_os_error();
                    if err.raw_os_error() == Some(libc::ESRCH) {
                        return true; // Process already exited
                    }
                    return false;
                }
                true
            });

            if !sigterm_sent {
                // Could not send SIGTERM at all — process may have exited
                return Ok(());
            }

            // Wait up to 5 seconds for exit
            let timeout = Duration::from_secs(5);
            match tokio::time::timeout(timeout, child.wait()).await {
                Ok(Ok(status)) => {
                    let _ = status.signal();
                    return Ok(());
                }
                Ok(Err(e)) => {
                    return Err(anyhow::anyhow!("error waiting after SIGTERM: {e}"));
                }
                Err(_) => {
                    // Timed out waiting for SIGTERM, escalate to SIGKILL
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

/// Attempt to send a signal via pidfd_open + pidfd_send_signal (Linux 5.3+).
/// Returns Some(true) if signal was sent, Some(false) if the process was gone,
/// or None if pidfd is unavailable (older kernel / unsupported).
#[cfg(target_os = "linux")]
fn try_pidfd_signal(pid: u32, signal: libc::c_int) -> Option<bool> {
    // pidfd_open(2) syscall number: 434 on x86_64, 434 on aarch64
    let pidfd =
        unsafe { libc::syscall(libc::SYS_pidfd_open, pid as libc::c_int, 0 as libc::c_uint) };
    if pidfd < 0 {
        let err = std::io::Error::last_os_error();
        if err.raw_os_error() == Some(libc::ESRCH) {
            return Some(false); // Process already exited
        }
        // ENOSYS or other error — pidfd not available
        return None;
    }

    // pidfd_send_signal(2) syscall number: 424 on x86_64, 424 on aarch64
    let ret = unsafe {
        libc::syscall(
            libc::SYS_pidfd_send_signal,
            pidfd as libc::c_int,
            signal,
            std::ptr::null::<libc::siginfo_t>(),
            0 as libc::c_uint,
        )
    };

    // Close the pidfd
    unsafe { libc::close(pidfd as libc::c_int) };

    if ret < 0 {
        let err = std::io::Error::last_os_error();
        if err.raw_os_error() == Some(libc::ESRCH) {
            return Some(false);
        }
        // Other error — treat as unavailable
        return None;
    }

    Some(true)
}

/// Non-Linux Unix: pidfd is not available, always return None to use fallback.
#[cfg(all(unix, not(target_os = "linux")))]
fn try_pidfd_signal(_pid: u32, _signal: libc::c_int) -> Option<bool> {
    None
}
