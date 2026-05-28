# Implementation Plan: Rust TUI (vfa-tui)

## Overview

Implementation of an enterprise-grade terminal user interface in Rust for the vanguard-frontier-agentic marketplace. The TUI provides interactive catalog browsing, validation gate execution, and export command building. It lives at `tools/vfa-tui/` as a separate Cargo workspace. Implementation follows a bottom-up dependency order: foundational layers (project structure, errors, models) → domain logic (workspace detection, catalog loading, security, search) → infrastructure (subprocess, logging) → presentation (UI) → testing and CI.

## Tasks

- [ ] 1. Project scaffolding and core types
  - [ ] 1.1 Create Cargo workspace and directory structure
    - Create `tools/vfa-tui/Cargo.toml` with workspace metadata, edition 2021, and all dependencies (ratatui 0.30, crossterm 0.28, clap 4.x derive, serde + serde_json, tokio rt-multi-thread, tracing + tracing-subscriber, thiserror + anyhow, nucleo-matcher 0.3, uuid v4, proptest for dev-dependencies)
    - Create the full module directory structure as defined in the design: `src/main.rs`, `src/app.rs`, `src/cli.rs`, `src/error.rs`, and module directories for `models/`, `catalog/`, `ui/`, `subprocess/`, `security/`, `search/`, `workspace/`, `logging/`
    - Create empty `mod.rs` files for each module directory
    - Add `#![deny(warnings)]` to `src/main.rs` for release mode
    - Commit `Cargo.lock` for reproducible builds
    - _Requirements: 17.2, 17.3_

  - [ ] 1.2 Implement error types with thiserror
    - Create `src/error.rs` with the full `TuiError` enum as defined in the design (CatalogNotFound, CatalogParse, TaintedEntry, WorkspaceNotFound, InvalidWorkspace, SubprocessFailed, SubprocessTimeout, ValidationRejected, PathTraversal, TerminalCapability, LogDestination)
    - Ensure all variants include structured context fields
    - Implement `From` conversions for common error types (io::Error, serde_json::Error)
    - _Requirements: 12.1, 12.2, 12.3_

  - [ ] 1.3 Implement CLI parsing with clap derive
    - Create `src/cli.rs` with `#[derive(Parser)]` struct supporting: `--workspace <path>`, `--log-file <path>`, `--log-level <level>` (trace/debug/info/warn/error, default info), `--no-color`, `--version`, `--help`
    - Add version from Cargo.toml via `#[command(version)]`
    - Add validation for log-level values via clap's `ValueEnum`
    - _Requirements: 14.1, 14.4, 14.5, 14.7_

  - [ ] 1.4 Implement data models with serde
    - Create `src/models/mod.rs` re-exporting all model types
    - Create `src/models/agent.rs` with `Agent`, `AgentType`, `ExecutionTier`, `Lifecycle` structs using `#[serde(deny_unknown_fields)]`
    - Create `src/models/skill.rs` with `Skill`, `SkillType` structs using `#[serde(deny_unknown_fields)]`
    - Create `src/models/role.rs` with `RoleCatalog`, `Role` structs
    - Create `src/models/mcp_ref.rs` with `McpReference`, `TrustMatrix`, `SignedRelease`, `PinStrategy`, `McpType` structs using `#[serde(deny_unknown_fields)]`
    - Create `src/models/rule.rs` with `Rule`, `RuleType` structs using `#[serde(deny_unknown_fields)]`
    - Create `src/models/integrity.rs` with `AssetIntegrity`, `IntegrityScope`, `IntegrityTree`, `IntegrityFile` structs
    - Create `src/models/provider.rs` with `Provider` enum (all 19 variants with kebab-case rename)
    - Create `src/models/harness.rs` with `Harness`, `SourceType` enums
    - _Requirements: 1.1, 2.1, 3.1, 5.1, 5.2, 15.2, 21.1_

- [ ] 2. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 3. Workspace detection and catalog loading
  - [ ] 3.1 Implement workspace detection
    - Create `src/workspace/mod.rs` and `src/workspace/detect.rs`
    - Implement upward directory traversal from CWD (or `--workspace` path) searching for `catalog/agents.json` AND `package.json` with `name` equal to `@raishin/vanguard-frontier-agentic`
    - Return `TuiError::WorkspaceNotFound` if traversal reaches filesystem root
    - Return `TuiError::InvalidWorkspace` if markers are partially present
    - _Requirements: 14.2, 14.3, 14.6, 15.1, 15.6_

  - [ ]* 3.2 Write property test for workspace detection
    - **Property 16: Workspace detection finds correct root**
    - **Validates: Requirements 14.2, 15.1**

  - [ ] 3.3 Implement catalog loader with strict deserialization
    - Create `src/catalog/mod.rs`, `src/catalog/loader.rs`, `src/catalog/store.rs`
    - Implement `CatalogStore::load(workspace_root: &Path) -> Self` that loads all catalog JSON files (agents.json, skills.json, install-roles.json, mcp-references.json, rules.json, asset-integrity.json)
    - Use `serde_json::from_str` with `deny_unknown_fields` on entry types
    - On parse error: skip the file, record in `load_errors`, continue with partial data
    - On tainted entry (control bytes in string fields): skip entry, log warning, continue loading remaining entries
    - Implement query methods: `agent_count()`, `skill_count()`, `provider_count()`, `agents_by_provider()`, `agents_for_role()`, `skills_for_agent()`, `agents_with_skill()`
    - Implement stable case-insensitive lexicographic sort by ID for all list accessors
    - _Requirements: 1.1, 1.2, 2.1, 2.4, 3.1, 5.1, 5.2, 5.5, 10.3, 12.2, 15.1, 15.2, 15.3, 15.4, 15.5, 18.3, 21.1, 21.2_

  - [ ]* 3.4 Write property test for invalid JSON handling
    - **Property 1: Invalid JSON produces error without panic**
    - **Validates: Requirements 1.2, 2.4, 3.4, 5.5, 12.1**

  - [ ]* 3.5 Write property test for strict deserialization
    - **Property 14: Strict deserialization rejects unknown fields**
    - **Validates: Requirements 15.2**

  - [ ]* 3.6 Write property test for tainted entry skipping
    - **Property 13: Catalog entries with control bytes are skipped**
    - **Validates: Requirements 10.3**

  - [ ]* 3.7 Write property test for stable sort
    - **Property 15: Stable case-insensitive lexicographic sort**
    - **Validates: Requirements 3.2, 4.2, 18.3**

- [ ] 4. Security module
  - [ ] 4.1 Implement terminal escape sanitization
    - Create `src/security/mod.rs` and `src/security/sanitize.rs`
    - Implement `sanitize_catalog_string(input: &str) -> String` — replace control bytes (0x00–0x08, 0x0B–0x0C, 0x0E–0x1F, 0x7F) with U+FFFD, preserve 0x09 (tab) and 0x0A (newline)
    - Implement `sanitize_subprocess_output(input: &str) -> String` — pass SGR sequences (CSI + numeric params + `m`), strip all other escape sequences (OSC, DCS, SOS, PM, APC)
    - _Requirements: 10.1, 10.2, 10.4_

  - [ ]* 4.2 Write property tests for sanitization
    - **Property 11: Catalog string sanitization removes control bytes**
    - **Validates: Requirements 10.1**
    - **Property 12: Subprocess output escape filtering**
    - **Validates: Requirements 10.2**

  - [ ] 4.3 Implement input/path validation
    - Create `src/security/validate.rs`
    - Implement `validate_argument(arg: &str) -> Result<(), ValidationError>` — reject shell metacharacters (`;|&$\`\\<>(){}!#*?[]`, newline, CR, null byte)
    - Implement `validate_path(path: &Path, workspace_root: &Path) -> Result<PathBuf, ValidationError>` — canonicalize, reject traversal outside workspace, reject null bytes and non-UTF-8
    - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

  - [ ]* 4.4 Write property tests for validation
    - **Property 7: Path validation rejects directory traversal**
    - **Validates: Requirements 8.2, 8.5**
    - **Property 8: Argument validation rejects shell metacharacters**
    - **Validates: Requirements 8.3**

  - [ ] 4.5 Implement secret redaction
    - Create `src/security/redact.rs`
    - Implement `is_secret_env_var(name: &str) -> bool` — case-insensitive match for AWS_SECRET_ACCESS_KEY, GITHUB_TOKEN, NPM_TOKEN, and names containing _SECRET, _KEY, _TOKEN, _PASSWORD, _CREDENTIAL
    - Implement `redact_secrets(input: &str) -> String` — replace base64 strings >40 chars, strings prefixed with `ghp_`, `npm_`, `sk-`, `AKIA` with fixed placeholder `[REDACTED]`
    - Implement `sanitized_child_env() -> Vec<(std::ffi::OsString, std::ffi::OsString)>` — copy the current environment while excluding names matched by `is_secret_env_var`
    - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5_

  - [ ]* 4.6 Write property tests for redaction
    - **Property 9: Secret environment variable detection**
    - **Validates: Requirements 9.1, 9.2**
    - **Property 10: Secret redaction correctness**
    - **Validates: Requirements 9.3, 9.5**

- [ ] 5. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Search engine and catalog queries
  - [ ] 6.1 Implement fuzzy search with nucleo-matcher
    - Create `src/search/mod.rs` and `src/search/fuzzy.rs`
    - Implement fuzzy matching against catalog item searchable fields (id, name, provider, summary)
    - Return scored results sorted by match quality
    - Support filtering agents by provider, harness, and search query (intersection semantics)
    - _Requirements: 1.3, 1.5, 1.6, 1.7, 2.3_

  - [ ]* 6.2 Write property tests for search
    - **Property 2: Fuzzy search returns only matching items**
    - **Validates: Requirements 1.3, 2.3**
    - **Property 3: Combined filter returns correct intersection**
    - **Validates: Requirements 1.5, 1.6, 1.7**

  - [ ] 6.3 Implement reverse-lookup and cross-entity queries
    - Add `agents_with_skill(skill_id)` to CatalogStore — returns agents whose `companion_skills` contains the skill ID
    - Add `agents_for_role(role_id)` — returns agents in a role grouped by provider
    - _Requirements: 2.2, 3.2_

  - [ ]* 6.4 Write property test for reverse-lookup
    - **Property 5: Reverse-lookup returns correct associated agents**
    - **Validates: Requirements 2.2**

- [ ] 7. Subprocess execution
  - [ ] 7.1 Implement subprocess executor with tokio
    - Create `src/subprocess/mod.rs`, `src/subprocess/executor.rs`, `src/subprocess/stream.rs`, `src/subprocess/signal.rs`
    - Implement `SubprocessExecutor::spawn()` using `tokio::process::Command` with direct process spawning (no shell), arguments as array
    - Implement stdout/stderr line streaming via `mpsc::UnboundedReceiver<OutputLine>`
    - Implement timeout handling (default 300s for validation gates)
    - Implement cancellation: SIGTERM → wait 5s → SIGKILL
    - Set working directory to workspace root for all subprocess invocations
    - Pass a sanitized child environment to every subprocess by removing variables whose names match `security::redact::is_secret_env_var`
    - _Requirements: 6.2, 6.3, 6.4, 6.5, 7.4, 8.1, 20.1, 20.3, 20.4, 20.5, 20.6_

  - [ ] 7.2 Implement export command model
    - Create `ExportCommand` struct with platform, selection (All/Role/Provider/Agents), target_repo, dry_run (default true), force, no_skills
    - Implement `to_args()` — builds argument array for `node scripts/export-marketplace-agents.mjs`
    - Implement `display_command()` — renders full command string for preview
    - Validate all arguments through `security::validate` before construction
    - _Requirements: 7.1, 7.2, 7.3, 7.5, 20.2_

  - [ ]* 7.3 Write property test for export command construction
    - **Property 6: Export command argument construction**
    - **Validates: Requirements 7.2, 20.2**

- [ ] 8. Audit logging
  - [ ] 8.1 Implement structured logging with tracing
    - Create `src/logging/mod.rs` and `src/logging/audit.rs`
    - Configure `tracing-subscriber` with JSON format, configurable output (stderr default, file via --log-file, or both)
    - Include session_id (UUID v4), timestamp (ISO 8601 ms precision), action type, outcome in all events
    - Implement log levels: INFO for user actions, WARN for validation failures, ERROR for subprocess failures and security rejections
    - Redact secret values in log output using `security::redact`
    - Fallback to stderr if log file cannot be opened
    - _Requirements: 11.1, 11.2, 11.3, 11.4, 11.5, 11.6_

- [ ] 9. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 10. UI layer — terminal management and layout
  - [ ] 10.1 Implement terminal manager
    - Create `src/ui/mod.rs` and wire up the UI module
    - Implement `TerminalManager` with setup (alternate screen, raw mode, cursor hide) and guaranteed restoration via `Drop`
    - Install panic hook at startup to restore terminal state before printing panic message
    - Handle terminal resize events with re-render within 100ms
    - _Requirements: 12.5, 13.2, 13.4, 16.4_

  - [ ] 10.2 Implement navigation state machine
    - Create `src/ui/nav.rs` with `View` enum (all 19 variants as defined in design) and `NavigationState` struct
    - Implement history stack for back-navigation (Escape key)
    - Implement sidebar index tracking for Tab navigation between catalog sections
    - Implement list state tracking (ratatui `ListState`) for selection
    - _Requirements: 16.1, 16.3, 16.6, 16.7_

  - [ ] 10.3 Implement layout computation and theme
    - Create `src/ui/layout.rs` — compute layout regions (sidebar, main content, status bar, help bar) based on terminal dimensions
    - Create `src/ui/theme.rs` — define color/style constants, support `--no-color` mode, fallback to 8-color if 256-color unavailable
    - _Requirements: 12.4, 16.3, 16.6, 18.1_

  - [ ] 10.4 Implement UI widgets
    - Create `src/ui/widgets/mod.rs`, `list_view.rs`, `detail.rs`, `status_bar.rs`, `help_bar.rs`, `output.rs`, `search.rs`
    - `list_view.rs`: scrollable list with highlight, boundary stop (no wrap)
    - `detail.rs`: agent/skill/MCP/rule detail panel rendering all required fields, "N/A" for absent optional fields
    - `status_bar.rs`: visible agent count, total count, active filters, session info
    - `help_bar.rs`: context-sensitive keybinding display for current panel
    - `output.rs`: subprocess output panel with stdout/stderr differentiation
    - `search.rs`: search input widget with live filtering
    - _Requirements: 1.4, 1.8, 1.9, 2.2, 2.5, 3.3, 5.3, 5.4, 6.1, 6.6, 7.1, 12.6, 15.3, 16.2, 16.5, 21.3, 21.4, 21.5_

  - [ ]* 10.5 Write property test for agent detail formatter
    - **Property 4: Agent detail formatter includes all required fields**
    - **Validates: Requirements 1.4**

- [ ] 11. Application event loop and wiring
  - [ ] 11.1 Implement application state and event loop
    - Create `src/app.rs` with `App` struct holding NavigationState, CatalogStore, SearchState, subprocess handle, status bar, session_id, should_quit flag
    - Implement `handle_event()` dispatching keyboard events to navigation, search, validation, export controllers
    - Implement `tick()` for polling subprocess output and updating UI state
    - Wire keybindings: arrows/j/k (list movement), Enter (select), Escape (back), Tab (switch panels), `/` (search), `q`/Ctrl+C (quit), g/G (top/bottom), Vim-style navigation
    - _Requirements: 16.1, 16.4, 16.5, 16.7, 18.1, 18.6_

  - [ ] 11.2 Implement main entry point
    - Create `src/main.rs` with CLI parsing, workspace detection, catalog loading, terminal setup, event loop, and graceful shutdown
    - Wire panic hook installation before any terminal operations
    - Ensure no network requests, no file writes, no config persistence
    - _Requirements: 14.1, 18.2, 18.4, 19.1, 19.2, 19.3, 19.4, 19.5, 19.6, 19.7_

  - [ ] 11.3 Implement validation gate controller
    - Extract validation gates from `package.json` `validate:*` scripts
    - Display gate list with status (pass/fail/not-run)
    - Execute gates via subprocess with real-time output streaming
    - Support "Run All Validations" via `npm run validate`
    - Prevent concurrent execution of same gate
    - Display animated progress indicator during execution
    - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5, 6.6, 6.7, 6.8_

  - [ ] 11.4 Implement export command builder controller
    - Present command builder UI: platform selection, agent selection method (all/role/provider/specific IDs), target repo path, optional flags
    - Display exact command preview before execution
    - Default to dry-run mode, require explicit toggle for live execution
    - Validate target path exists and is writable before execution
    - Handle subprocess exit codes, preserve selections on failure
    - Provide cancel control during execution
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6, 7.7, 7.8_

  - [ ] 11.5 Implement integrity view controller
    - Parse and display `catalog/asset-integrity.json` data
    - Show manifest version, algorithm, total file count, aggregate hashes
    - Display assets grouped by tree with path, SHA-256, and size
    - Handle missing/invalid integrity file gracefully
    - _Requirements: 21.1, 21.2, 21.3, 21.4, 21.5_

- [ ] 12. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 13. Integration tests and test fixtures
  - [ ]* 13.1 Create test fixtures
    - Create `tools/vfa-tui/tests/fixtures/` directory with: `agents.json` (5 agents, 3 providers), `skills.json` (3 skills), `install-roles.json` (6 roles), `mcp-references.json` (2 refs), `rules.json` (2 rules), `asset-integrity.json` (minimal manifest), `package.json` (with validate:* scripts), `invalid.json` (malformed), `tainted-agents.json` (control bytes in fields)
    - _Requirements: 1.1, 2.1, 3.1, 5.1, 5.2, 12.2, 21.1_

  - [ ]* 13.2 Write integration tests for catalog loading
    - Test full round-trip loading from fixture files
    - Test partial loading when some files are missing
    - Test error reporting for invalid JSON
    - Test tainted entry skipping
    - _Requirements: 1.2, 5.5, 10.3, 12.2, 15.5_

  - [ ]* 13.3 Write integration tests for subprocess execution
    - Test spawning with mock scripts (various exit codes)
    - Test stdout/stderr separation
    - Test timeout and SIGTERM → SIGKILL escalation
    - _Requirements: 6.2, 6.4, 6.5, 20.4, 20.5, 20.6_

  - [ ]* 13.4 Write integration tests for search
    - Test fuzzy matching with known inputs and expected results
    - Test combined filter intersection semantics
    - Test empty result handling
    - _Requirements: 1.3, 1.5, 1.7, 1.8, 2.3, 2.5_

  - [ ]* 13.5 Write property test for deterministic rendering
    - **Property 17: Deterministic rendering**
    - **Validates: Requirements 18.1**

- [ ] 14. CI workflow and documentation
  - [ ] 14.1 Create CI workflow configuration
    - Create `.github/workflows/vfa-tui-ci.yml` (or integrate into existing CI)
    - Steps: `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo build --release`
    - Run on pull requests affecting `tools/vfa-tui/**`
    - Add release binary builds for: x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu, x86_64-apple-darwin, aarch64-apple-darwin, x86_64-unknown-linux-musl
    - Add SBOM generation with `cargo-sbom` or equivalent
    - _Requirements: 17.1, 17.2, 17.4, 17.5, 17.6_

  - [ ] 14.2 Create README and project documentation
    - Create `tools/vfa-tui/README.md` with: project overview, build instructions, usage examples, CLI flags reference, architecture overview, development guide
    - Document WSL compatibility notes
    - _Requirements: 13.1, 13.3, 14.1_

- [ ] 15. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties from the design document (17 properties total)
- Unit tests validate specific examples and edge cases
- The design uses Rust throughout — all code examples use Rust with the specified crate versions
- All subprocess invocations use direct process spawning (no shell) per security requirements
- The TUI makes no network requests and writes no files to disk (deterministic, read-only)

## Task Dependency Graph

```json
{
  "waves": [
    { "id": 0, "tasks": ["1.1"] },
    { "id": 1, "tasks": ["1.2", "1.3", "1.4"] },
    { "id": 2, "tasks": ["3.1", "4.1", "4.3", "4.5"] },
    { "id": 3, "tasks": ["3.2", "3.3", "4.2", "4.4", "4.6"] },
    { "id": 4, "tasks": ["3.4", "3.5", "3.6", "3.7", "6.1", "6.3"] },
    { "id": 5, "tasks": ["6.2", "6.4", "7.1", "7.2", "8.1"] },
    { "id": 6, "tasks": ["7.3", "10.1", "10.2", "10.3"] },
    { "id": 7, "tasks": ["10.4", "10.5"] },
    { "id": 8, "tasks": ["11.1", "11.2", "11.3", "11.4", "11.5"] },
    { "id": 9, "tasks": ["13.1"] },
    { "id": 10, "tasks": ["13.2", "13.3", "13.4", "13.5"] },
    { "id": 11, "tasks": ["14.1", "14.2"] }
  ]
}
```
