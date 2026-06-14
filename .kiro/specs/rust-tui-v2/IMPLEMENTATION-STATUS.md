# rust-tui-v2 — Implementation Verification Status

**Verified:** 2026-06-14 · **Target:** `tools/vfa-tui/` (~29.6k LOC) · **Spec:** `tasks.md` (70 leaf tasks)

This is a deep-check of how much of the v2 plan is genuinely implemented in code,
not just scaffolded. Verified by building the crate, running the full test suite,
and auditing each task against actual source symbols.

## Build & test health

| Gate | Result |
|------|--------|
| `cargo build` | ✅ clean (exit 0) |
| `cargo test --all-targets` | ✅ **1549 tests pass, 0 failed, 0 ignored** |
| `cargo clippy --all-targets` (crate uses `#![deny(warnings)]`) | ✅ clean |

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
| 7.1 Coverage engine | Computes results but never caches them in SQLite (Req 3.6 — re-scan only on mtime change). |
| 7.3 Drift engine | `detect_drift` returns results but never writes the `drift_history` table (schema exists, no writer call). |
| 9.1 / 11.3 Event loop & TUI v2 | `run_tui_async` `tokio::select!` wires only crossterm + 250 ms tick; watcher/scan/gate mpsc channels not fed in. `app.render()` dispatches the legacy `View` enum — v2 `Tab`s (CoverageMatrix, PolicyViolations, AuditLog, Dependencies) exist but don't render. |
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
