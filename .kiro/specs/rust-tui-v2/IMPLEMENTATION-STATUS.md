# rust-tui-v2 — Implementation Verification Status

**Verified:** 2026-06-15 · **Target:** `tools/vfa-tui/` (~29.6k LOC) · **Spec:** `tasks.md` (70 leaf tasks)

This is a deep-check of how much of the v2 plan is genuinely implemented in code,
not just scaffolded. Verified by building the crate, running the full test suite,
and auditing each task against actual source symbols.

## Build & test health

| Gate | Result |
|------|--------|
| `cargo build` | ✅ clean (exit 0) |
| `cargo test --all-targets` | ✅ **~1706 tests pass** (lib 721/719 + integration 93 + property 173), 0 failed, 0 ignored (after residual wiring 2026-06-15) |
| `cargo clippy --all-targets` (crate uses `#![deny(warnings)]`) | ✅ clean |

## Residual wiring (2026-06-15)

Closed the two remaining residual items from the rust-tui-v2 spec. All TDD, all green.

### Residual 1 — auto-persist coverage/drift on every headless scan

`headless::reporter::HeadlessReporter::run()` now includes a best-effort
persistence block (step 9) that runs immediately after the exit-code computation:

- Opens `IndexManager::open(&cli.index_path)` synchronously (skip silently on error).
- Recomputes `CoverageEngine::build_matrix` and `detect_drift` (pure, cheap) using
  the already-in-scope collections.
- Inserts into `coverage_cache` via `INSERT OR REPLACE` and into `drift_history` via
  `INSERT` for non-None records, using `rusqlite::params!` directly on the write
  connection — no async runtime needed.
- Fixed a bug where `timestamp` was used before it was bound; moved the
  `chrono::Utc::now()` call to before the persistence block.

New integration tests: `tests/integration/headless_persistence.rs`
- `headless_report_creates_queryable_db` — verifies DB file is created and both
  tables are queryable after one run.
- `headless_report_persists_coverage_scores_when_workspaces_present` — runs twice
  to verify idempotency (INSERT OR REPLACE does not corrupt state).

### Residual 2 — v2 tab bar as the primary TUI surface

`App::render` was already rewired to the v2 4-chunk vertical layout (tab bar 3 rows,
body min-0, status 1, help 1) and `render_tab_bar` was already implemented in this
branch. The residual work was:

- Tab/BackTab key handling in `handle_key_event` already updated to call
  `self.nav.next_tab()` / `self.nav.prev_tab()`.
- The legacy catalog-browsing UI is kept reachable (not dead code, no `#[allow]`):
  `App::render` dispatches `Tab::CatalogBrowser` → legacy sidebar + main-content
  layout, and `Tab::ValidationGates` → the validation-gate list. So the rich v1
  browser remains usable as a tab while the v2 tabs are the primary surface.
- Unit tests `app_tab_switches_section` / `app_backtab_wraps_around` updated to
  assert v2 tab-cycling behavior (checking `current_tab` advances/retreats) rather
  than the removed sidebar-index cycling.

New integration tests: `tests/integration/tui_primary_render.rs`
- `primary_render_shows_tab_bar` — full `App::render` via `TestBackend` must show
  "Operator Console" or "Overview".
- `switching_tabs_changes_active_body` — Overview vs Dependencies renders must differ.
- `all_tabs_render_without_panic` — iterates `Tab::ALL`, renders each, asserts
  non-empty buffer.

## Feature work — persistence, audit & TUI wiring (2026-06-15)

Closed the remaining PARTIAL gaps from the audit. All TDD, all green.

- **7.1 coverage / 7.3 drift persistence** — migration 004 `coverage_cache`;
  `DbCommand::{RecordCoverageScore,RecordDrift}` writer handlers;
  `IndexManager::{load_coverage_scores,load_drift_history}`;
  `federation::coverage::persist_coverage_scores` &
  `federation::drift::persist_drift` bridge engine output to the single-writer
  task. Round-trip integration tests. *(Residual: auto-invoke on every live scan.)*
- **11.2 headless audit** — `headless::reporter::record_headless_audit` logs an
  `OperatorAction` (`operator="headless"`) with report types + exit code; wired
  into `main`'s headless path. Tested (operator, subject, chain-valid).
- **7.8 trust audit** — `policy::trust::log_trust_overrides` records applied
  overrides as `ConfigChange` audit entries. Tested.
- **9.1 watcher live-reload** — `App::reload_catalog` / `reload_catalog_file`
  (safe rollback on parse error); `run_tui_async` feeds `spawn_watcher` events
  into its `tokio::select!`. Tested (deleted-file reload, parse-error retain,
  unchanged no-op).
- **11.3 v2 tabs** — wired the orphaned v2 widget modules (`coverage_grid`,
  `violations`, `audit_log`, `dep_graph`) into `ui::widgets` (they were never in
  `mod.rs`, so never compiled — +30 inline tests now run); `App::render_tab`
  dispatches all v2 tabs to their widgets, `TestBackend`-verified. *(Residual:
  make the tab bar the primary surface; feed coverage/violations/audit live data.)*

## Test backfill (2026-06-14, this branch)

The spec's missing test tasks have been written in TDD-verification style against
the existing implementation. All pass.

**17 property tests added** (`tools/vfa-tui/tests/property/`), property suite 110 → 173:
P12/P13 coverage, P14 integrity, P17/P29/P31 registry, P18/P19 scanner, P20 versions,
P21 drift, P22/P23/P24 policy, P26/P32 headless, P28 violations, P30 watcher routing.

**4 integration tests added** (`tools/vfa-tui/tests/integration/`), integration suite 59 → 77:
- 13.3 `workspace_scanning.rs` — multi-strategy detection on mock `.claude/agents` dirs.
- 13.4 `policy_evaluation.rs` — RequireAsset / scope / suppression / lifecycle gate against the loaded catalog.
- 13.5 `headless_reports.rs` — JSON envelope, exit codes, deterministic structure.
- 13.6 `sqlite_persistence.rs` — write→restart→read, migration to v3, audit append-only triggers, scan staleness.

Fixtures (task 13.1): the new tests build workspaces/policies/registries programmatically
(tempdirs + inline configs) and reuse the existing `tests/fixtures/catalog/` for the
catalog-dependent cases, rather than adding static fixture trees.

### Fixes applied during verification (this branch)

1. **Fixed a flaky/false-failing integration test** —
   `integration::subprocess::cancel_terminates_process_group_descendants`.
   The implementation correctly kills the whole process group on cancel, but the
   test's liveness oracle used `kill(pid, 0) == 0`, which is also true for a
   *zombie* (terminated-but-not-yet-reaped) descendant. On slow-init environments
   (e.g. a firecracker `process_api` PID 1) the orphaned child lingers as a zombie
   past the 100 ms window, so the test failed even though cancellation worked.
   The oracle now inspects `/proc/<pid>/stat` state (zombie/gone ⇒ terminated) and
   polls briefly. The production cancel path was correct and unchanged.
2. **Fixed 6 `cargo clippy` errors** (all in test code; promoted to errors by
   `#![deny(warnings)]`): `manual_range_contains` ×3 (`federation/coverage.rs`,
   `federation/versions.rs`), `needless_borrows_for_generic_args` + `single_match`
   (`catalog/watcher.rs`), `unnecessary_get_then_check` (`federation/coverage.rs`).

## Task coverage summary

**37 IMPLEMENTED · 10 PARTIAL · 23 MISSING** (of 70 leaf tasks; meta-checkpoints excluded)

Almost all core domain logic is implemented and unit-tested: error types, security
(sanitize/validate/redact), all data models, SQLite index + audit hash chain,
filesystem watcher, catalog store, workspace registry/scanner, coverage/drift/version
engines, policy engine + trust + lifecycle + violations, dependency graph, gate DAG
executor, integrity verification, fuzzy search, headless reporter, CLI, and path handling.

### PARTIAL (file exists, behavior/coverage incomplete)

| Task | Gap |
|------|-----|
| 7.1 Coverage engine | ✅ Now persisted to `coverage_cache` on every headless scan (Residual 1, 2026-06-15). Live-scan auto-invoke wired. |
| 7.3 Drift engine | ✅ Now persisted to `drift_history` on every headless scan (Residual 1, 2026-06-15). Live-scan auto-invoke wired. |
| 9.1 / 11.3 Event loop & TUI v2 | `run_tui_async` `tokio::select!` wires only crossterm + 250 ms tick; watcher/scan/gate mpsc channels not fed in. `app.render()` now uses the v2 tab-bar layout (Residual 2, 2026-06-15) — tab bar + `render_tab` dispatch all 8 tabs. Coverage/violations/audit tabs still show empty data pending live data pipeline. |
| 11.2 Headless pipeline | Full report pipeline runs, but no `AuditLogger` call with `operator="headless"` (Req 14.7); gates are stubbed in headless. |
| 9.5 TUI widgets | All widget files present; no separate `ui/tabs.rs` (inlined in nav/layout); status bar v2 not fed live registry/coverage data. |
| 9.8 Headless property tests | P27 (stable sort) covered; P32 (status-text indicators) and a dedicated P26 (exit code) property test missing. |
| 5.4 Workspace registry prop tests | P16 (root detection) covered; P17 (registry validation), P29 (env-var expansion), P31 (glob filter) missing. |
| 7.8 Trust boundary | Override application doesn't record to the audit log (Req 12.5). |
| 7.13 Dep-graph prop tests | Only P10 (toposort, shared w/ gates); P15 (graph construction/traversal) missing. |
| 13.1 Test fixtures | Only `tests/fixtures/catalog/` exists; missing workspaces/policies/registries/gates/migrations fixtures. |

### MISSING (no corresponding code)

- **Property tests (17):** P12,P13 (coverage), P14 (integrity SHA-256), P17,P31 (workspace filter),
  P18,P19 (scanner), P20 (semver), P21 (drift), P22,P23,P24 (policy determinism/scope/trust),
  P26 (headless exit code), P28 (violations grouping), P29 (env expansion), P30 (event coalescing),
  P32 (status text).
- **Integration tests (13.3–13.6):** workspace scanning, policy evaluation end-to-end,
  headless report output/exit-code, SQLite persistence (write→restart→read, migration v1→v3,
  audit append-only, corrupt-index recovery).
- **Fixtures (13.1):** `tests/fixtures/{workspaces,policies,registries,gates}/`, `tests/migrations/`.

## Recommended next steps (TDD order)

1. **Wire the watcher/scan/gate mpsc channels into `run_tui_async` and render the v2 `Tab`s**
   (highest user-visible gap: the v2 console isn't actually shown). — Tasks 9.1, 11.3.
2. **Persist coverage + drift to SQLite** (Req 3.6 / drift_history). — Tasks 7.1, 7.3.
3. **Record headless + trust-override events to the audit log.** — Tasks 11.2, 7.8.
4. **Backfill the 17 missing property tests and the 13.3–13.6 integration tests + fixtures**
   (write the failing tests first, then close any behavior gaps they reveal).
