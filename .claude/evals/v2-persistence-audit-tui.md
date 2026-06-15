# EVAL: v2-persistence-audit-tui

Eval-driven definition for closing rust-tui-v2 gaps 7.1, 7.3, 11.2, 7.8, 9.1, 11.3.
Graders are code-based (deterministic): `cargo test` + `cargo clippy`.

## Capability Evals

### Persistence
- [ ] 7.3 drift: `DbCommand::RecordDrift` persists `DriftRecord`s to `drift_history`;
      `IndexManager::load_drift_history()` reads them back (round-trip).
- [ ] 7.1 coverage: a `coverage_cache` table (migration 004) stores per-workspace
      scores; `RecordCoverageScore` writes, `load_coverage_scores()` reads.

### Audit
- [ ] 11.2 headless: a headless run records an audit entry with `operator = "headless"`
      (event_type OperatorAction) carrying report types + exit code; hash chain valid.
- [ ] 7.8 trust: applied `WorkspaceTrustOverride`s are recorded as audit entries
      (event_type ConfigChange) naming the mcp_ref, approver, and workspace.

### TUI
- [ ] 9.1 watcher: `App::reload_catalog()` / `reload_catalog_file()` refresh state;
      `run_tui_async` feeds `spawn_watcher` events into its `tokio::select!`.
- [ ] 11.3 tabs: `App::render` dispatches the v2 `Tab`s (Overview, CoverageMatrix,
      PolicyViolations, AuditLog, Dependencies) — verified via ratatui `TestBackend`.

## Regression Evals (pass^1 = 100% required before each commit)
- [ ] Existing 1630 tests still pass.
- [ ] `cargo clippy --all-targets` clean (crate is `#![deny(warnings)]`).
- [ ] `npm run validate` green.

## Eval Report (2026-06-15)

Capability evals — code grader = new tests, all PASS (pass@1):
- 7.3 drift round-trip ........... PASS (sqlite_persistence::drift_records_persist_to_drift_history)
- 7.1 coverage cache ............. PASS (sqlite_persistence::coverage_scores_persist_to_cache)
- 11.2 headless audit ............ PASS (audit_logging::headless_run_is_recorded_with_headless_operator)
- 7.8 trust audit ................ PASS (audit_logging::applied_trust_overrides_are_audited)
- 9.1 watcher reload ............. PASS (tui_reload::{deleted_file,retain_on_parse_error,unchanged})
- 11.3 tab render ................ PASS (tui_tabs::{overview,dependencies,data_dependent})

Regression evals — pass^1 = 100%:
- Full suite (~1701 tests) ....... PASS
- cargo clippy --all-targets ..... PASS (clean under #![deny(warnings)])
- npm run validate ............... PASS

Residuals (capability + tests landed; live invocation pending): auto-persist on
live scan (7.1/7.3); v2 tab bar as primary surface with live data (11.3).

## Status
SHIPPED (with documented residual wiring)
