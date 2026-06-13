//! VFA-TUI entry point — mode dispatch and wiring (Tasks 11.1–11.5).
//!
//! # Modes
//!
//! After parsing CLI args and detecting the workspace root the binary enters
//! one of four modes:
//!
//! 1. **ValidateConfig** (`--validate-config`): parse all config files, report
//!    errors to stderr, exit 0 if valid or 2 if invalid (Req 31.2).
//! 2. **ExportAudit** (`--export-audit <fmt> <path>`): open the SQLite index,
//!    export the audit log, exit 0/2.
//! 3. **Headless** (`--report <type>`): run the HeadlessReporter pipeline —
//!    single scan, no terminal manipulation, no filesystem watchers (Req 1.5,
//!    17.1).  Exits with the reporter's exit code (0–3, Req 18).
//! 4. **TUI** (default): set up raw mode / alternate screen, run the existing
//!    synchronous event loop, exit 0 on quit (Req 18.7).
//!
//! # Accessibility (Req 29)
//!
//! `--no-color` / `NO_COLOR` is detected via `Cli::is_no_color()`.  The flag
//! is forwarded to the headless reporter and app state.  Status text
//! indicators (`[PASS]`, `[FAIL]`, etc.) are always present in headless output
//! regardless of color mode (Req 29.2).
//!
//! # Cross-platform paths (Req 28)
//!
//! Default registry/policy/index paths are resolved via `crate::paths`.  The
//! clap defaults in `Cli` already embed the Linux XDG defaults; this module
//! does *not* override them at runtime — the `paths` module is available for
//! callers that need the helpers independently.

#![deny(warnings)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod app;
mod catalog;
mod cli;
mod error;
mod federation;
mod gates;
mod headless;
mod logging;
mod models;
mod paths;
mod persistence;
mod policy;
mod search;
mod security;
mod subprocess;
mod ui;
mod workspace;

use std::path::PathBuf;

use clap::Parser;

use crate::cli::Cli;
use crate::headless::reporter::HeadlessReporter;

// ---------------------------------------------------------------------------
// Mode selection
// ---------------------------------------------------------------------------

/// The four execution modes the binary can enter.
///
/// Extracted into a pure enum so `select_mode` can be unit-tested independently
/// of I/O (Task 11.1 requirement for testable dispatch).
#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    /// `--validate-config` was passed.
    ValidateConfig,
    /// `--export-audit <fmt> <path>` was passed.
    ExportAudit,
    /// `--report <type>` was passed — headless pipeline, no TUI.
    Headless,
    /// No special flags — launch the interactive TUI.
    Tui,
}

/// Determine which mode to run based on the parsed CLI args.
///
/// Priority order (first match wins):
/// 1. `--validate-config`
/// 2. `--export-audit`
/// 3. `--report` present → Headless
/// 4. Default → TUI
pub fn select_mode(cli: &Cli) -> Mode {
    if cli.validate_config {
        return Mode::ValidateConfig;
    }
    if cli.export_audit.is_some() {
        return Mode::ExportAudit;
    }
    if cli.report.is_some() {
        return Mode::Headless;
    }
    Mode::Tui
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mode = select_mode(&cli);

    // ── Workspace detection ───────────────────────────────────────────────────
    // Detect workspace root; on error print to stderr and exit 2 (operational).
    let workspace_root = match workspace::detect::detect_workspace(cli.workspace.as_deref()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[vfa-tui] workspace detection failed: {e}");
            std::process::exit(2);
        }
    };

    // ── Logging init ──────────────────────────────────────────────────────────
    // For headless/validate-config modes use stderr subscriber (no file).
    // For TUI mode use file-based subscriber when --log-file is set.
    let session_id = uuid::Uuid::new_v4();
    let log_file = match mode {
        Mode::Headless | Mode::ValidateConfig | Mode::ExportAudit => {
            // In headless modes we suppress file logging unless explicitly
            // requested, to avoid interfering with structured stdout output.
            if cli.quiet {
                None
            } else {
                cli.log_file.as_deref()
            }
        }
        Mode::Tui => cli.log_file.as_deref(),
    };

    let log_level_str = format!("{:?}", cli.log_level).to_lowercase();
    logging::audit::init_logging(log_file, &log_level_str, session_id)?;

    // ── Mode dispatch ─────────────────────────────────────────────────────────
    match mode {
        // ── ValidateConfig ────────────────────────────────────────────────────
        Mode::ValidateConfig => {
            run_validate_config(&cli, &workspace_root);
        }

        // ── ExportAudit ───────────────────────────────────────────────────────
        Mode::ExportAudit => {
            run_export_audit(&cli, &workspace_root);
        }

        // ── Headless ──────────────────────────────────────────────────────────
        // Req 17.1: no raw mode, no alternate screen, no panic hook.
        // Req 1.5: single scan, no filesystem watchers.
        Mode::Headless => {
            let format = cli.output_format();
            let quiet = cli.quiet;
            let reporter = HeadlessReporter::new(format, quiet);

            // Pass no-color flag through to formatting context.
            // HeadlessReporter.run() writes formatted output to stdout directly.
            let (_value, exit_code) = reporter.run(&cli, &workspace_root);

            std::process::exit(exit_code as i32);
        }

        // ── TUI ───────────────────────────────────────────────────────────────
        // Install the terminal panic hook only in TUI mode (Req 17.1 — headless
        // must never enter raw mode / install terminal hooks).
        Mode::Tui => {
            ui::install_panic_hook();
            run_tui(&cli, &workspace_root)?;
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// ValidateConfig runner (Req 31.2)
// ---------------------------------------------------------------------------

/// Parse all configuration files and report errors.  Exits 0 if valid, 2 if
/// any config is invalid.
fn run_validate_config(cli: &Cli, workspace_root: &std::path::Path) {
    let mut errors: Vec<String> = Vec::new();

    // Validate registry TOML.
    let registry_path = PathBuf::from(&cli.registry);
    if registry_path.exists() {
        match std::fs::read_to_string(&registry_path) {
            Ok(content) => {
                if let Err(e) = toml::from_str::<toml::Value>(&content) {
                    errors.push(format!(
                        "registry {}: TOML parse error: {e}",
                        registry_path.display()
                    ));
                }
            }
            Err(e) => {
                errors.push(format!(
                    "registry {}: read error: {e}",
                    registry_path.display()
                ));
            }
        }
    } else if !cli.quiet {
        eprintln!(
            "[vfa-tui] registry not found at {} (skipping validation)",
            registry_path.display()
        );
    }

    // Validate policies TOML.
    let policy_path = PathBuf::from(&cli.policies);
    if policy_path.exists() {
        match policy::parser::load(&policy_path) {
            Ok(_) => {}
            Err(e) => {
                errors.push(format!("policies {}: {e}", policy_path.display()));
            }
        }
    } else if !cli.quiet {
        eprintln!(
            "[vfa-tui] policies not found at {} (skipping validation)",
            policy_path.display()
        );
    }

    // Validate catalog directory.
    let catalog_dir = workspace_root.join("catalog");
    if !catalog_dir.exists() {
        errors.push(format!(
            "catalog directory not found at {}",
            catalog_dir.display()
        ));
    }

    if errors.is_empty() {
        if !cli.quiet {
            eprintln!("[vfa-tui] configuration valid");
        }
        std::process::exit(0);
    } else {
        for e in &errors {
            eprintln!("[vfa-tui] config error: {e}");
        }
        std::process::exit(2);
    }
}

// ---------------------------------------------------------------------------
// ExportAudit runner
// ---------------------------------------------------------------------------

/// Open the SQLite index and export the audit log to the requested format/path.
fn run_export_audit(cli: &Cli, _workspace_root: &std::path::Path) {
    let parts = cli.export_audit.as_ref().expect("ExportAudit mode implies export_audit is Some");
    if parts.len() < 2 {
        eprintln!("[vfa-tui] --export-audit requires FORMAT and PATH");
        std::process::exit(2);
    }
    let format = parts[0].as_str();
    let out_path = std::path::Path::new(&parts[1]);

    // Open (or create) the SQLite index.
    let index_path = &cli.index_path;
    let mgr = match persistence::index::IndexManager::open(index_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("[vfa-tui] failed to open index at {index_path}: {e}");
            std::process::exit(2);
        }
    };

    let logger = match persistence::audit::AuditLogger::from_manager(&mgr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[vfa-tui] failed to initialise audit logger: {e}");
            std::process::exit(2);
        }
    };

    match logger.export_audit(format, out_path) {
        Ok(()) => {
            if !cli.quiet {
                eprintln!(
                    "[vfa-tui] audit log exported ({format}) to {}",
                    out_path.display()
                );
            }
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("[vfa-tui] export failed: {e}");
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------------
// TUI runner — existing synchronous event loop (unchanged)
// ---------------------------------------------------------------------------

/// Launch the interactive TUI.
///
/// This preserves the existing working synchronous event loop exactly.
/// Another task (11.3) owns the async rewrite; this stub keeps compilation
/// and operation intact (Req 18.7 — TUI exits 0 on quit).
fn run_tui(cli: &Cli, workspace_root: &std::path::Path) -> anyhow::Result<()> {
    // Load catalog.
    let catalog = catalog::store::CatalogStore::load(workspace_root);
    if !catalog.load_errors.is_empty() {
        for err in &catalog.load_errors {
            tracing::warn!(%err, "catalog load warning");
        }
    }

    // Setup terminal (raw mode + alternate screen).
    let mut terminal_mgr = ui::TerminalManager::new()?;

    let session_id = uuid::Uuid::new_v4();
    let no_color = cli.is_no_color();

    // Create app.
    let mut app = app::App::new(catalog, workspace_root.to_path_buf(), session_id, no_color);

    // Synchronous event loop (preserved from original main.rs).
    loop {
        terminal_mgr.draw(|frame| app.render(frame))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key) => {
                    app.handle_key_event(key);
                }
                crossterm::event::Event::Resize(_width, _height) => {
                    // Terminal resize detected — the next draw() call will
                    // automatically use the new dimensions from the backend,
                    // ensuring re-render within the 100ms poll interval.
                }
                _ => {}
            }
        }

        app.tick();

        if app.should_quit {
            break;
        }
    }

    terminal_mgr.restore()?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Unit tests — select_mode dispatch
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse(args: &[&str]) -> Cli {
        Cli::try_parse_from(args).expect("parse")
    }

    #[test]
    fn default_flags_select_tui() {
        let cli = parse(&["vfa-tui"]);
        assert_eq!(select_mode(&cli), Mode::Tui);
    }

    #[test]
    fn validate_config_flag_selects_validate_config() {
        let cli = parse(&["vfa-tui", "--validate-config"]);
        assert_eq!(select_mode(&cli), Mode::ValidateConfig);
    }

    #[test]
    fn export_audit_flag_selects_export_audit() {
        let cli = parse(&["vfa-tui", "--export-audit", "json", "/tmp/a.json"]);
        assert_eq!(select_mode(&cli), Mode::ExportAudit);
    }

    #[test]
    fn report_flag_selects_headless() {
        let cli = parse(&["vfa-tui", "--report", "summary"]);
        assert_eq!(select_mode(&cli), Mode::Headless);
    }

    #[test]
    fn report_coverage_selects_headless() {
        let cli = parse(&["vfa-tui", "--report", "coverage"]);
        assert_eq!(select_mode(&cli), Mode::Headless);
    }

    #[test]
    fn report_all_selects_headless() {
        let cli = parse(&["vfa-tui", "--report", "all"]);
        assert_eq!(select_mode(&cli), Mode::Headless);
    }

    #[test]
    fn validate_config_has_priority_over_report() {
        // --validate-config takes priority even when --report is also set.
        let cli = parse(&["vfa-tui", "--validate-config", "--report", "summary"]);
        assert_eq!(select_mode(&cli), Mode::ValidateConfig);
    }

    #[test]
    fn export_audit_has_priority_over_report() {
        let cli = parse(&[
            "vfa-tui",
            "--export-audit", "json", "/tmp/a.json",
            "--report", "summary",
        ]);
        assert_eq!(select_mode(&cli), Mode::ExportAudit);
    }

    #[test]
    fn no_color_accessible_via_is_no_color() {
        let cli = parse(&["vfa-tui", "--no-color"]);
        assert!(cli.is_no_color());
    }

    #[test]
    fn web_flag_still_selects_tui() {
        // --web is a stretch goal; without --report it should still be TUI mode.
        let cli = parse(&["vfa-tui", "--web"]);
        assert_eq!(select_mode(&cli), Mode::Tui);
    }
}
