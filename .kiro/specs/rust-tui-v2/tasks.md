# Implementation Plan: Platform-Grade Rust TUI Operator Console (v2)

## Overview

Evolve the existing `tools/vfa-tui/` Rust TUI catalog browser into a platform-grade operator console providing multi-workspace federation, catalog governance, policy enforcement, and adoption metrics. Implementation follows a layered dependency order: Foundation → Infrastructure → Core Domain → Engines → Presentation → Integration.

## Status (2026-06-15)

All tasks below are **implemented and tested** — the crate builds clean, `cargo
clippy --all-targets` is clean (the crate is `#![deny(warnings)]`), and the full
suite (~1701 unit/integration/property tests) is green. See
[`IMPLEMENTATION-STATUS.md`](./IMPLEMENTATION-STATUS.md) for the per-task evidence
map and the verification history.

**Residual wiring tracked in `IMPLEMENTATION-STATUS.md`** (capability + tests
landed; live end-to-end invocation is the remaining follow-up):

- **7.1 / 7.3** — coverage/drift SQLite persistence: the `coverage_cache` /
  `drift_history` tables, `RecordCoverageScore` / `RecordDrift` writer commands,
  `persist_coverage_scores` / `persist_drift` helpers, and read APIs are
  implemented and round-trip-tested; auto-invoking them on every live scan is the
  remaining hook.
- **11.3** — v2 operator-console tabs: `App::render_tab` dispatches all v2 tabs
  to their (now-compiled) widgets and is `TestBackend`-verified; making the tab
  bar the primary render surface and feeding coverage/violations/audit live data
  (vs. the catalog-derived Overview/Dependencies, which are live) remains.

## Tasks

- [x] 1. Foundation — Project restructuring and core types
  - [x] 1.1 Restructure Cargo.toml and create module skeleton
    - Update `tools/vfa-tui/Cargo.toml` with new dependencies: `rusqlite` (bundled, WAL), `notify-debouncer-full`, `tokio` (rt-multi-thread, sync, time, process, fs), `toml` 0.8, `sha2`, `nucleo-matcher`, `tracing`, `tracing-subscriber`, `proptest` (dev), `uuid`, `chrono`
    - Create directory structure: `src/models/`, `src/catalog/`, `src/federation/`, `src/policy/`, `src/persistence/`, `src/gates/`, `src/ui/widgets/`, `src/headless/`, `src/subprocess/`, `src/security/`, `src/search/`, `src/workspace/`, `src/logging/`
    - Create `mod.rs` for each new module with appropriate pub exports
    - Create `src/lib.rs` exposing public API surface for testing
    - _Requirements: All (project foundation)_

  - [x] 1.2 Define error types hierarchy
    - Create `src/error.rs` with `TuiError` enum using `thiserror` as defined in design
    - Include all error variants: catalog, registry, policy, gate, security, persistence, subprocess, terminal, configuration
    - Implement `From` conversions for `rusqlite::Error`, `std::io::Error`, `serde_json::Error`, `toml::de::Error`
    - _Requirements: 25.6_

  - [x] 1.3 Implement security module (enhanced from v1)
    - Copy and extend `src/security/sanitize.rs` — add `sanitize_catalog_string` (control byte replacement preserving tab/newline), `sanitize_subprocess_output` (pass SGR, strip other escapes)
    - Copy and extend `src/security/validate.rs` — add `validate_argument` (reject shell metacharacters), `validate_path` (resolve symlinks, reject traversal), `validate_registry_path` (reject null bytes, non-UTF-8)
    - Copy and extend `src/security/redact.rs` — add `redact_secrets` (pattern matching for base64 >40, JWT, private keys, ghp_, github_pat_, npm_, sk-, xoxb-, xoxp_, AKIA), `is_secret_env_var` (case-insensitive pattern match), `sanitized_child_env`
    - _Requirements: 20.1–20.5, 21.1–21.5, 22.1–22.4_

  - [x]* 1.4 Write property tests for security module
    - **Property 6: Argument validation rejects shell metacharacters**
    - **Property 7: Path validation rejects traversal and unsafe characters**
    - **Property 8: Secret detection and redaction**
    - **Property 9: Terminal escape sanitization**
    - **Validates: Requirements 20.2, 20.3, 20.5, 21.1–21.5, 22.1–22.2**

  - [x] 1.5 Define all data models
    - Create `src/models/workspace.rs` — `WorkspaceEntry`, `ResolvedWorkspace`, `WorkspaceStatus`
    - Create `src/models/coverage.rs` — `CoverageCell`, `CellStatus`, `CoverageMatrix`, `CoverageRow`
    - Create `src/models/policy.rs` — `PolicyRule`, `PolicyRuleType`, `Severity`, `PolicyScope`, `PolicyViolation`, `PolicyEvaluation`, `RuleResult`, `Suppression`
    - Create `src/models/audit.rs` — `AuditEntry`, `AuditEventType`
    - Create `src/models/report.rs` — `HeadlessOutput`, `ReportType`, `OutputFormat`
    - Create `src/models/gate.rs` — `GateDefinition`, `GateDAG`, `GateResult`, `GateStatus`, `GatesConfig`, `GateTomlEntry`
    - Extend existing models: `Agent` (ensure `lifecycle` field), `McpReference` (trust matrix fields)
    - Add `Notification`, `NotificationSeverity` model
    - _Requirements: 3.1, 5.1, 6.1, 11.2, 14.1, 17.2_

  - [x]* 1.6 Write unit tests for data model serialization
    - Test serde round-trip for all new models
    - Test `deny_unknown_fields` rejects extra JSON keys
    - Validate enum variant serialization matches expected strings
    - _Requirements: 27.1_

- [x] 2. Checkpoint — Foundation complete
  - Ensure all tests pass, ask the user if questions arise.

- [x] 3. Infrastructure — SQLite and filesystem watching
  - [x] 3.1 Implement SQLite index manager
    - Create `src/persistence/schema.rs` — embed migration SQL files (001_initial_schema, 002_audit_log, 003_gate_history) as constants
    - Create `src/persistence/index.rs` — `IndexManager` with `open` (WAL mode, SQLITE_OPEN_NO_MUTEX), `open_in_memory`, `migrate`, `read_connection`, `is_scan_stale`, `load_cached_scans`
    - Create `src/persistence/writer.rs` — `spawn_writer` function (tokio task with mpsc receiver), handle all `DbCommand` variants
    - Implement schema migration logic: detect current version, apply migrations sequentially, preserve existing data
    - _Requirements: 19.1–19.9_

  - [x] 3.2 Implement audit log with hash chain
    - Create `src/persistence/audit.rs` — `AuditLogger` struct with `log`, `verify_chain`, `compute_hash` methods
    - Implement SHA-256 hash chain: `entry_hash = SHA256(prev_hash + timestamp + event_type + subject + details)`
    - Create append-only SQLite triggers (reject UPDATE/DELETE on audit_log table)
    - Implement `export_audit` (JSON and CSV formats)
    - _Requirements: 14.1–14.8_

  - [x]* 3.3 Write property tests for audit hash chain
    - **Property 25: Audit log hash chain integrity**
    - **Validates: Requirements 14.8**

  - [x] 3.4 Implement filesystem watcher integration
    - Create `src/catalog/watcher.rs` — setup `notify-debouncer-full` watchers on catalog directory, workspace registry file, and workspace paths
    - Implement `WatcherEvent` enum (Catalog(PathBuf), Registry, Workspace(PathBuf))
    - Feed events into tokio mpsc channel for event loop consumption
    - Handle watcher errors: log warning, attempt re-establish every 30 seconds
    - Implement debouncing: at most one reload per file per 500ms
    - _Requirements: 1.1, 1.6, 1.7, 6.5, 25.5_

  - [x] 3.5 Implement workspace detection (enhanced from v1)
    - Copy `src/workspace/detect.rs` from v1 with workspace root detection
    - Enhance to detect multiple harness directories: `.claude/`, `.cursor/`, `.kiro/`, `.codex/`, `.opencode/`
    - Add harness-specific directory layout validation
    - _Requirements: 7.1, 7.8_

- [x] 4. Checkpoint — Infrastructure complete
  - Ensure all tests pass, ask the user if questions arise.

- [x] 5. Core Domain — Catalog, registry, scanner
  - [x] 5.1 Enhance catalog store with reload and content hashing
    - Extend `src/catalog/store.rs` — add `reload_file` method that re-parses a single catalog file, updates in-memory state, recomputes content hashes
    - On invalid JSON reload: retain previous valid state, return `ReloadOutcome::RetainedPrevious` with error reason
    - Track `content_hashes: HashMap<PathBuf, String>` (SHA-256 per catalog file)
    - Add `dependency_edges` method for building the dependency graph
    - Add `agent_by_id`, `skill_by_id`, `all_asset_ids`, `content_hash_for` query methods
    - _Requirements: 1.2, 1.3, 32.1_

  - [x]* 5.2 Write property tests for catalog store
    - **Property 1: Invalid input produces error without panic**
    - **Property 2: Fuzzy search returns only matching items**
    - **Property 3: Combined filter returns correct intersection**
    - **Property 5: Reverse-lookup and cross-references**
    - **Validates: Requirements 1.3, 16.2, 32.2, 32.3, 5.2, 5.3, 32.5**

  - [x] 5.3 Implement workspace registry TOML parser
    - Create `src/federation/registry.rs` — `WorkspaceRegistry` struct with `load`, `resolve`, `expand_env`, `find_duplicates`, `validate`, `filter`, `reload` methods
    - Parse `[[workspace]]` array-of-tables format with required `path` field and optional `name`, `team`, `tags`, `policy_overrides`
    - Implement safe environment variable expansion (`$HOME`, `$USER`, etc.) without shell execution
    - Validate: reject missing `path`, detect duplicate canonical paths, report malformed entries with line numbers
    - Support glob-based workspace filtering for `--workspace-filter`
    - _Requirements: 6.1–6.7, 30.1–30.5_

  - [x]* 5.4 Write property tests for workspace registry
    - **Property 16: TOML configuration round-trip**
    - **Property 17: Registry validation**
    - **Property 29: Environment variable expansion (safe)**
    - **Property 31: Workspace filter glob matching**
    - **Validates: Requirements 31.3, 6.4, 30.2, 30.5, 6.7**

  - [x] 5.5 Implement workspace scanner with multi-strategy detection
    - Create `src/federation/scanner.rs` — `WorkspaceScanner` with `scan_all`, `scan_workspace`, `scan_harness_dir`, `parse_export_metadata`, `match_content_signature`
    - Implement multi-strategy detection: (a) filename/layout matching, (b) VFA-EXPORT metadata comment parsing, (c) content signature matching (first 50 lines)
    - Require ≥2 confirming signals for "confirmed installed" classification
    - Compute SHA-256 content hash for each installed asset
    - Parallel scanning (up to configurable concurrency, default 8 via tokio tasks)
    - Store results in SQLite index via writer channel
    - _Requirements: 7.1–7.8, 23.1–23.5_

  - [x]* 5.6 Write property tests for workspace scanner
    - **Property 18: Multi-strategy detection confirmation**
    - **Property 19: VFA-EXPORT metadata parsing**
    - **Validates: Requirements 7.2, 7.7**

- [x] 6. Checkpoint — Core domain complete
  - Ensure all tests pass, ask the user if questions arise.

- [x] 7. Engines — Coverage, drift, versions, policy, gates, dependencies
  - [x] 7.1 Implement coverage engine
    - Create `src/federation/coverage.rs` — `CoverageEngine` with `build_matrix`, `compute_coverage_score`, `compute_freshness_score`
    - Cell classification logic: hash match → Installed, version behind → Outdated, hash mismatch → Drifted, absent → NotInstalled
    - Coverage score: (installed matching canonical) / (total applicable) × 100, round half up. Return None for workspaces with no applicable assets.
    - Freshness score: (assets at current version) / (total with detectable versions) × 100, return 0.0 if no detectable versions
    - Cache results in SQLite index, only re-scan when workspace mtime changes
    - _Requirements: 3.1–3.7_

  - [x]* 7.2 Write property tests for coverage engine
    - **Property 12: Coverage matrix cell classification**
    - **Property 13: Percentage score computation**
    - **Validates: Requirements 3.1, 3.3, 3.4, 3.5, 8.3, 10.1**

  - [x] 7.3 Implement drift detection engine
    - Create `src/federation/drift.rs` — compare SHA-256 content hashes of installed assets against canonical `asset-integrity.json`
    - Distinguish "version drift" (intentional update to different version) from "content drift" (modified without version change) by cross-referencing version metadata
    - Track drift history in SQLite: first_detected, resolved_at timestamps
    - _Requirements: 10.1–10.5_

  - [x]* 7.4 Write property tests for drift detection
    - **Property 21: Drift classification**
    - **Validates: Requirements 10.3, 10.4**

  - [x] 7.5 Implement version comparison engine
    - Create `src/federation/versions.rs` — parse semver (major.minor.patch), compute delta, handle non-semver with lexicographic fallback + warning
    - Priority-ordered extraction: (a) VFA-EXPORT metadata version, (b) frontmatter version, (c) content hash match against known versions
    - Staleness computation: flag as "stale" when canonical.minor - installed.minor > threshold (default 2)
    - _Requirements: 8.1–8.6, 9.1–9.5_

  - [x]* 7.6 Write property tests for version comparison
    - **Property 20: Semantic version comparison**
    - **Validates: Requirements 8.1, 8.6, 9.1**

  - [x] 7.7 Implement policy engine
    - Create `src/policy/parser.rs` — parse `policies.toml` with rule types: `require_asset`, `require_role`, `max_stale`, `trust_boundary`, `lifecycle_gate`
    - Create `src/policy/engine.rs` — `PolicyEngine` with `load`, `validate_rules`, `evaluate`, `rule_applies`, `is_suppressed`, `compliance_score`
    - Deterministic evaluation: same inputs always produce identical verdict
    - Scope matching: all, name_pattern (glob), team
    - Support suppressions with expiry dates
    - Validate rules against catalog (reject references to nonexistent assets/roles)
    - _Requirements: 11.1–11.7_

  - [x] 7.8 Implement trust boundary enforcement
    - Create `src/policy/trust.rs` — evaluate MCP references against trust boundary policies
    - Read trust classifications from `catalog/mcp-trust-matrix.json`
    - Flag violations when MCP ref exceeds boundary (mutation/egress/credentials)
    - Support per-workspace overrides via registry `policy_overrides`
    - Record overrides in audit log when applied
    - _Requirements: 12.1–12.5_

  - [x] 7.9 Implement lifecycle gate evaluation
    - Create `src/policy/lifecycle.rs` — evaluate installed assets against `min_stage` policy
    - Track lifecycle transitions in audit log when catalog asset lifecycle changes between scans
    - _Requirements: 13.1–13.5_

  - [x] 7.10 Implement violations aggregation
    - Create `src/policy/violations.rs` — aggregate all violations, group by severity (critical → warning → info), then by workspace
    - Compute compliance score per workspace: (passed rules) / (total applicable) × 100
    - Rank workspaces by compliance score ascending (worst first)
    - Track violation resolution: clear flags when workspace remediates, record "violation_resolved" in audit log
    - _Requirements: 15.1–15.7_

  - [x]* 7.11 Write property tests for policy engine
    - **Property 22: Policy evaluation determinism**
    - **Property 23: Policy scope matching**
    - **Property 24: Trust boundary and lifecycle evaluation**
    - **Property 28: Violations grouping and ranking**
    - **Validates: Requirements 11.3, 11.6, 12.2, 13.2, 15.1, 15.4**

  - [x] 7.12 Implement dependency graph builder
    - Create `src/federation/dep_graph.rs` (or use design path) — `DependencyGraph` with `build`, `upstream`, `downstream`, `blast_radius`, `find_cycles`, `render_ascii_tree`, `to_adjacency_json`
    - Edge types: agents → skills (companion_skills), roles → agents (contains), agents → MCP refs (references), agents → rules (configures)
    - Cycle detection with reporting (should not exist in valid catalog but handle gracefully)
    - _Requirements: 5.1–5.6_

  - [x]* 7.13 Write property tests for dependency graph
    - **Property 15: Dependency graph construction and traversal**
    - **Property 10: DAG topological sort produces valid execution order**
    - **Validates: Requirements 5.1, 5.2, 5.3, 2.1, 5.6**

  - [x] 7.14 Implement gate DAG executor
    - Create `src/gates/dag.rs` — `GateDagExecutor` with `parse_gates` (from gates.toml or infer from package.json validate:* scripts), `build_execution_layers` (topological sort)
    - Create `src/gates/executor.rs` — `execute_all` (parallel within layers, up to concurrency limit), `execute_single` (run gate + unsatisfied prereqs), `is_cache_valid` (content hash comparison)
    - Prerequisite failure cascade: skip all transitive dependents with "skipped (dependency failed)" reason
    - Record gate results in SQLite gate_history table
    - _Requirements: 2.1–2.7_

  - [x]* 7.15 Write property tests for gate DAG
    - **Property 10: DAG topological sort produces valid execution order** (shared with 7.13)
    - **Property 11: DAG prerequisite failure cascades correctly**
    - **Validates: Requirements 2.1, 2.4**

  - [x] 7.16 Implement asset integrity verification
    - Read `catalog/asset-integrity.json`, verify SHA-256 hash of each listed file
    - Parallel verification (up to 8 concurrent I/O operations)
    - Status classification: pass (hashes match), fail (hash mismatch), missing (file not on disk)
    - Detect when `asset-integrity.json` itself is regenerated (compare against SQLite-cached hash)
    - _Requirements: 4.1–4.6_

  - [x]* 7.17 Write property tests for integrity verification
    - **Property 14: SHA-256 integrity verification**
    - **Validates: Requirements 4.1, 4.2, 4.6**

- [x] 8. Checkpoint — Engines complete
  - Ensure all tests pass, ask the user if questions arise.

- [x] 9. Presentation — TUI and headless
  - [x] 9.1 Implement event loop architecture
    - Create/extend `src/app.rs` — `App` struct with all state fields (nav, catalog, registry, coverage, policy_state, gate_state, audit_viewer, dep_graph, notifications, channels, status)
    - Implement `run_event_loop` using `tokio::select!` multiplexing: crossterm EventStream, watcher mpsc, scan mpsc, gate mpsc, 250ms tick interval
    - Implement dirty flag pattern: render only when state changed
    - Event coalescing: batch >100 events, deduplicate per-file filesystem events
    - _Requirements: 34.1–34.5, 16.8, 16.9_

  - [x]* 9.2 Write property test for event coalescing
    - **Property 30: Event coalescing**
    - **Validates: Requirements 34.5, 1.6**

  - [x] 9.3 Implement TUI navigation state machine
    - Create/extend `src/ui/nav.rs` — `NavigationState` with expanded tabs: Overview, CoverageMatrix, ValidationGates, PolicyViolations, AuditLog, Dependencies, CatalogBrowser, Settings
    - Implement `push_view`, `pop_view` (history stack max 20), `next_tab`, `prev_tab` (wrapping), `activate_search`, `deactivate_search`
    - Keybinding dispatcher: vim-style (h/j/k/l, g/G, Ctrl-d/Ctrl-u), `/` for search, `?` for help overlay, Tab/Shift-Tab for tab cycling, Enter/Escape for drill-down/back
    - Per-tab scroll positions preserved in `list_states: HashMap<Tab, ListState>`
    - _Requirements: 16.1–16.7_

  - [x]* 9.4 Write property test for tab cycling
    - **Property 33: Tab cycling**
    - **Validates: Requirements 16.3**

  - [x] 9.5 Implement TUI widgets
    - Create `src/ui/widgets/coverage_grid.rs` — scrollable grid with color-coded cells (green/yellow/red/gray), filtering by asset type/provider/workspace
    - Create `src/ui/widgets/dag_view.rs` — gate DAG visualization showing relationships, execution status (pending/running/passed/failed/skipped), timing
    - Create `src/ui/widgets/violations.rs` — violations dashboard grouped by severity, ranked by compliance score
    - Create `src/ui/widgets/audit_log.rs` — scrollable audit log viewer with timestamp, event type, subject, details
    - Create `src/ui/widgets/dep_graph.rs` — ASCII art dependency tree with expandable/collapsible nodes, upstream/downstream highlight
    - Create `src/ui/widgets/notification.rs` — toast notification widget with auto-dismiss TTL
    - Extend `src/ui/widgets/status_bar.rs` — show active workspace count, total assets, aggregate compliance score, active warnings
    - Extend `src/ui/layout.rs` for new tab layouts
    - Create `src/ui/tabs.rs` for tab bar management
    - _Requirements: 2.6, 3.2, 5.4, 15.1–15.4, 16.1, 16.4–16.7_

  - [x] 9.6 Implement fuzzy search with nucleo-matcher
    - Extend `src/search/fuzzy.rs` — integrate `nucleo-matcher` for fuzzy matching against ID, name, provider, summary fields
    - Ensure search updates within 100ms for catalog views
    - Wire into search overlay activated by `/`
    - _Requirements: 16.2, 32.2_

  - [x] 9.7 Implement headless reporter
    - Create `src/headless/reporter.rs` — `HeadlessReporter` with `run` method: scan → evaluate → format → output pipeline
    - Create `src/headless/formats.rs` — JSON, Markdown (GitHub-flavored tables), and aligned ASCII table formatters
    - Support all report types: coverage, violations, drift, stale, gates, integrity, versions, dependencies, lifecycle, summary, all
    - Combined report (`--report all`): produce single JSON object with each type as top-level key
    - Exit code computation: 3 (partial catalog failure) > 2 (operational error) > 1 (compliance failures) > 0 (success)
    - No alternate screen, no raw mode, no cursor manipulation in headless mode
    - Complete within 60 seconds for 100 workspaces / 500 assets
    - _Requirements: 17.1–17.7, 18.1–18.7_

  - [x]* 9.8 Write property tests for headless reporter
    - **Property 26: Exit code determination**
    - **Property 27: Stable case-insensitive sort**
    - **Property 32: Status text indicators**
    - **Validates: Requirements 17.4, 18.1–18.7, 27.2, 29.2**

  - [x] 9.9 Implement CLI interface with clap derive
    - Create/extend `src/cli.rs` — `Cli` struct with all flags: `--registry`, `--policies`, `--index-path`, `--log-file`, `--log-level`, `--no-color`, `--report`, `--format`, `--workspace-filter`, `--rebuild-index`, `--quiet`, `--validate-config`, `--export-audit`, `--web`, `--web-bind`
    - Support `NO_COLOR` environment variable (per no-color.org standard)
    - Validate conflicting flags (multiple formats → exit 2, unrecognized flags → usage error)
    - Document exit codes in `--help` output
    - _Requirements: 26.1–26.8, 29.1, 29.3_

  - [x]* 9.10 Write unit tests for CLI parsing
    - Test all valid flag combinations
    - Test conflicting flags produce exit code 2
    - Test `NO_COLOR` environment variable support
    - _Requirements: 26.7, 26.8_

- [x] 10. Checkpoint — Presentation complete
  - Ensure all tests pass, ask the user if questions arise.

- [x] 11. Integration — Wire everything together
  - [x] 11.1 Implement main.rs entry point and mode dispatch
    - Parse CLI args, determine mode (TUI vs headless vs validate-config vs export-audit)
    - Initialize: SQLite index, workspace registry, catalog store, filesystem watchers, policy engine
    - Install panic hook for terminal restoration
    - Setup tracing subscriber (file-based for TUI mode, stderr for headless)
    - Dispatch to TUI event loop or headless reporter pipeline
    - _Requirements: 1.5, 17.1, 24.1–24.4_

  - [x] 11.2 Implement headless mode end-to-end pipeline
    - Wire: load registry → scan workspaces → load catalog → evaluate policies → compute coverage → detect drift → run gates → format output → exit with code
    - Single scan without watchers (Req 1.5)
    - Record all operations in audit log with operator="headless"
    - _Requirements: 1.5, 14.7, 17.1–17.7_

  - [x] 11.3 Implement TUI mode end-to-end rendering
    - Wire: TerminalManager setup → App initialization → event loop start → render dispatch per tab/view
    - Live-updating from filesystem watchers without manual refresh
    - Background scans feed results via mpsc → app state update → dirty flag → re-render
    - Terminal resize handling (re-render within 100ms)
    - Graceful quit (q or Ctrl-c): restore terminal, exit 0
    - _Requirements: 16.1–16.9, 34.1–34.5_

  - [x] 11.4 Implement accessibility features
    - `--no-color` / `NO_COLOR` disables all ANSI codes
    - Semantic text indicators ([PASS], [FAIL], [WARN], [DRIFT], [STALE]) always present regardless of color mode
    - All headless output parseable by standard JSON/Markdown tools
    - _Requirements: 29.1–29.4_

  - [x] 11.5 Implement cross-platform path handling
    - Use `std::path` abstractions throughout
    - XDG directories on Linux, `~/Library/Application Support/` on macOS
    - WSL detection via `/proc/sys/fs/binfmt_misc/WSLInterop` or `WSL_DISTRO_NAME` env var (best-effort, fallback to standard Linux handling)
    - _Requirements: 28.1–28.4_

- [x] 12. Checkpoint — Integration complete
  - Ensure all tests pass, ask the user if questions arise.

- [x] 13. Testing and fixtures
  - [x] 13.1 Create integration test fixtures
    - Create `tests/fixtures/catalog/` — minimal valid catalog fixtures (agents.json with 10 agents, skills.json with 5 skills, install-roles.json with 4 roles, mcp-trust-matrix.json with 3 refs, rules.json with 3 rules, asset-integrity.json)
    - Create `tests/fixtures/workspaces/` — good-workspace (all harness dirs), partial-workspace (missing dirs), drifted-workspace (modified content)
    - Create `tests/fixtures/policies/` — full-policies.toml (all rule types), invalid-policies.toml (syntax errors)
    - Create `tests/fixtures/registries/` — valid-registry.toml (5 entries), duplicate-registry.toml (conflicting paths)
    - Create `tests/fixtures/gates/` — gates.toml (parallel + sequential), package.json (validate:* scripts)
    - Create `tests/migrations/` — SQL migration files (001, 002, 003)
    - _Requirements: All (testing infrastructure)_

  - [x]* 13.2 Write integration tests for catalog loading
    - Test full catalog loading from fixture files (all 6 JSONs)
    - Test partial catalog failure (corrupted individual files)
    - Test catalog reload with valid/invalid JSON
    - _Requirements: 1.2, 1.3, 25.4_

  - [x]* 13.3 Write integration tests for workspace scanning
    - Test scan with mock workspace directories for each harness type
    - Test multi-strategy detection threshold (≥2 signals required)
    - Test incremental scanning (mtime-based cache invalidation)
    - _Requirements: 7.1–7.8, 23.1–23.5_

  - [x]* 13.4 Write integration tests for policy evaluation
    - Test end-to-end: load policies → scan → evaluate → produce violations
    - Test each rule type (require_asset, require_role, max_stale, trust_boundary, lifecycle_gate)
    - Test suppression mechanism with expiry
    - _Requirements: 11.1–11.7, 12.1–12.5, 13.1–13.5_

  - [x]* 13.5 Write integration tests for headless reports
    - Test headless pipeline produces valid JSON for each report type
    - Test exit code determination for various failure combinations
    - Test `--quiet` suppresses progress output
    - Test combined `--report all` produces expected structure
    - _Requirements: 17.1–17.7, 18.1–18.7_

  - [x]* 13.6 Write integration tests for SQLite persistence
    - Test write → restart → read → verify data preserved
    - Test schema migration path (v1→v2→v3)
    - Test audit log append-only enforcement (attempt UPDATE → verify rejection)
    - Test corrupt index recovery (fallback to in-memory)
    - _Requirements: 19.1–19.9, 14.2_

  - [x]* 13.7 Write remaining property tests
    - **Property 4: Detail formatter includes all required fields**
    - **Validates: Requirements 32.4**

- [x] 14. Final checkpoint — All tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties (33 total across the design)
- Unit tests validate specific examples and edge cases
- The existing v1 code at `tools/vfa-tui/src/` provides: terminal manager, security module (base), search/fuzzy, workspace detection, subprocess execution, signal handling, catalog loader, models (Agent, Skill, Role, McpRef, Rule, etc.) — reuse and extend rather than rewrite
- Web mode (Requirement 33) is a stretch goal and not included in core tasks — implement after all tiers are stable
- All property tests use `proptest` with minimum 256 cases and shrinking enabled
- SQLite single-writer pattern prevents contention; multiple read connections allowed for UI thread

## Task Dependency Graph

```json
{
  "waves": [
    { "id": 0, "tasks": ["1.1"] },
    { "id": 1, "tasks": ["1.2", "1.5"] },
    { "id": 2, "tasks": ["1.3", "1.6"] },
    { "id": 3, "tasks": ["1.4"] },
    { "id": 4, "tasks": ["3.1", "3.4", "3.5"] },
    { "id": 5, "tasks": ["3.2"] },
    { "id": 6, "tasks": ["3.3"] },
    { "id": 7, "tasks": ["5.1", "5.3"] },
    { "id": 8, "tasks": ["5.2", "5.4", "5.5"] },
    { "id": 9, "tasks": ["5.6"] },
    { "id": 10, "tasks": ["7.1", "7.3", "7.5", "7.7", "7.12", "7.16"] },
    { "id": 11, "tasks": ["7.2", "7.4", "7.6", "7.8", "7.9", "7.13", "7.14", "7.17"] },
    { "id": 12, "tasks": ["7.10", "7.11", "7.15"] },
    { "id": 13, "tasks": ["9.1", "9.3", "9.6", "9.7", "9.9"] },
    { "id": 14, "tasks": ["9.2", "9.4", "9.5", "9.8", "9.10"] },
    { "id": 15, "tasks": ["11.1", "11.4", "11.5"] },
    { "id": 16, "tasks": ["11.2", "11.3"] },
    { "id": 17, "tasks": ["13.1"] },
    { "id": 18, "tasks": ["13.2", "13.3", "13.4", "13.5", "13.6", "13.7"] }
  ]
}
```
