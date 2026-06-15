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

## Status
RUNNING
