//! Cross-platform default path helpers (Task 11.5, Req 28.1–28.4).
//!
//! # Platform conventions
//!
//! - **Linux / WSL**: XDG Base Directory Specification.
//!   - Config: `$XDG_CONFIG_HOME/vfa/` (fallback: `~/.config/vfa/`)
//!   - Data:   `$XDG_DATA_HOME/vfa/`   (fallback: `~/.local/share/vfa/`)
//! - **macOS**: Apple convention.
//!   - Config: `~/Library/Application Support/vfa/`
//!   - Data:   `~/Library/Application Support/vfa/`
//!
//! # WSL detection (Req 28.3 / 28.4)
//!
//! WSL is detected on a best-effort basis by checking `WSL_DISTRO_NAME` env var
//! or the existence of `/proc/sys/fs/binfmt_misc/WSLInterop`.  When WSL is
//! detected the tool falls back to standard Linux path conventions.

use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Public helpers
// ---------------------------------------------------------------------------

/// Default path for the workspace registry TOML.
///
/// - Linux / WSL: `$XDG_CONFIG_HOME/vfa/workspaces.toml`
///   (fallback `~/.config/vfa/workspaces.toml`)
/// - macOS: `~/Library/Application Support/vfa/workspaces.toml`
pub fn default_registry_path() -> PathBuf {
    config_dir().join("workspaces.toml")
}

/// Default path for the policies TOML.
///
/// - Linux / WSL: `$XDG_CONFIG_HOME/vfa/policies.toml`
/// - macOS: `~/Library/Application Support/vfa/policies.toml`
pub fn default_policies_path() -> PathBuf {
    config_dir().join("policies.toml")
}

/// Default path for the SQLite index database.
///
/// - Linux / WSL: `$XDG_DATA_HOME/vfa/index.db`
///   (fallback `~/.local/share/vfa/index.db`)
/// - macOS: `~/Library/Application Support/vfa/index.db`
pub fn default_index_path() -> PathBuf {
    data_dir().join("index.db")
}

/// Detect whether the process is running inside Windows Subsystem for Linux.
///
/// This is a best-effort check (Req 28.3).  On detection we fall back to
/// standard Linux path conventions rather than Windows conventions.
///
/// Detection strategy (in order):
/// 1. `WSL_DISTRO_NAME` environment variable is set (WSL 2+).
/// 2. `/proc/sys/fs/binfmt_misc/WSLInterop` exists (WSL 1+).
///
/// Returns `false` on any I/O error (safe fallback).
pub fn is_wsl() -> bool {
    // Fast check via env var (WSL 2 always sets this).
    if std::env::var_os("WSL_DISTRO_NAME").is_some() {
        return true;
    }
    // Slower check: file presence (WSL 1 may not have the env var).
    std::path::Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists()
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Return the VFA config directory for the current platform.
fn config_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        home_dir()
            .join("Library")
            .join("Application Support")
            .join("vfa")
    }
    #[cfg(not(target_os = "macos"))]
    {
        // Linux (including WSL) — XDG_CONFIG_HOME or fallback.
        if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
            PathBuf::from(xdg).join("vfa")
        } else {
            home_dir().join(".config").join("vfa")
        }
    }
}

/// Return the VFA data directory for the current platform.
fn data_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        home_dir()
            .join("Library")
            .join("Application Support")
            .join("vfa")
    }
    #[cfg(not(target_os = "macos"))]
    {
        // Linux (including WSL) — XDG_DATA_HOME or fallback.
        if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
            PathBuf::from(xdg).join("vfa")
        } else {
            home_dir().join(".local").join("share").join("vfa")
        }
    }
}

/// Return the home directory using `HOME` env var (Linux / macOS).
///
/// Falls back to `/tmp` on failure so all callers get a valid (if wrong)
/// `PathBuf` rather than panicking.
fn home_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // -----------------------------------------------------------------------
    // default_registry_path
    // -----------------------------------------------------------------------

    #[test]
    fn registry_path_ends_with_workspaces_toml() {
        let p = default_registry_path();
        assert_eq!(p.file_name().unwrap(), "workspaces.toml");
    }

    #[test]
    fn policies_path_ends_with_policies_toml() {
        let p = default_policies_path();
        assert_eq!(p.file_name().unwrap(), "policies.toml");
    }

    #[test]
    fn index_path_ends_with_index_db() {
        let p = default_index_path();
        assert_eq!(p.file_name().unwrap(), "index.db");
    }

    // -----------------------------------------------------------------------
    // XDG override (Linux / non-macOS only — cfg guards keep macOS clean)
    // -----------------------------------------------------------------------

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn xdg_config_home_overrides_config_dir() {
        // Temporarily set XDG_CONFIG_HOME; note: not safe to run in parallel
        // with other tests that also mutate env — isolated by test binary.
        let old = env::var_os("XDG_CONFIG_HOME");

        // SAFETY: single-threaded test binary section; no threads spawned here.
        unsafe {
            env::set_var("XDG_CONFIG_HOME", "/custom/config");
        }
        let p = default_registry_path();
        // Restore before asserting so we clean up even if assert panics.
        unsafe {
            match &old {
                Some(v) => env::set_var("XDG_CONFIG_HOME", v),
                None => env::remove_var("XDG_CONFIG_HOME"),
            }
        }

        assert!(
            p.starts_with("/custom/config"),
            "XDG_CONFIG_HOME should override config dir; got {p:?}"
        );
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn xdg_data_home_overrides_data_dir() {
        let old = env::var_os("XDG_DATA_HOME");

        unsafe {
            env::set_var("XDG_DATA_HOME", "/custom/data");
        }
        let p = default_index_path();
        unsafe {
            match &old {
                Some(v) => env::set_var("XDG_DATA_HOME", v),
                None => env::remove_var("XDG_DATA_HOME"),
            }
        }

        assert!(
            p.starts_with("/custom/data"),
            "XDG_DATA_HOME should override data dir; got {p:?}"
        );
    }

    // -----------------------------------------------------------------------
    // macOS paths (only compiled/run on macOS)
    // -----------------------------------------------------------------------

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_registry_path_under_library() {
        let p = default_registry_path();
        let s = p.to_string_lossy();
        assert!(
            s.contains("Library/Application Support/vfa"),
            "macOS path should be under Library/Application Support/vfa; got {p:?}"
        );
    }

    // -----------------------------------------------------------------------
    // is_wsl
    // -----------------------------------------------------------------------

    #[test]
    fn is_wsl_returns_bool_without_panic() {
        // We cannot assert the value (depends on environment) but it must not panic.
        let _ = is_wsl();
    }

    #[test]
    fn is_wsl_detects_wsl_distro_name_env() {
        let had_it = env::var_os("WSL_DISTRO_NAME").is_some();
        if had_it {
            // Already in WSL — just verify it returns true.
            assert!(is_wsl());
        } else {
            // Set it and verify detection.
            unsafe {
                env::set_var("WSL_DISTRO_NAME", "Ubuntu");
            }
            let result = is_wsl();
            unsafe {
                env::remove_var("WSL_DISTRO_NAME");
            }
            assert!(result, "WSL_DISTRO_NAME should trigger is_wsl");
        }
    }

    // -----------------------------------------------------------------------
    // Platform-level sanity: all returned paths are absolute
    // -----------------------------------------------------------------------

    #[test]
    fn all_default_paths_are_absolute() {
        // Only holds when HOME is set (which it is in all normal environments).
        if env::var_os("HOME").is_some() {
            assert!(default_registry_path().is_absolute());
            assert!(default_policies_path().is_absolute());
            assert!(default_index_path().is_absolute());
        }
    }
}
