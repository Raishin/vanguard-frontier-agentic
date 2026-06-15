# Requirements Document

## Introduction

Platform-grade Rust TUI operator console for managing adoption of agentic assets (agents, skills, MCP references, rules) across an organization's repositories at Fortune 50 scale. Unlike the v1 catalog browser (`tools/vfa-tui/`), this tool is an **operator console for platform teams managing multi-workspace federation, catalog governance, policy enforcement, and adoption metrics**.

The console operates in dual mode: a rich interactive TUI for daily platform operations, and a headless `--report` mode producing structured output (JSON/Markdown/table) for CI/CD integration. All data comes from local filesystem scans — no network access. A SQLite-backed index enables sub-second startup even with 100+ downstream workspaces.

The tool lives at `tools/vfa-tui/` as an evolution of the existing Cargo workspace, replacing the alpha catalog browser with a full operator console.

## Glossary

- **Console**: The Rust binary providing the operator interface — both TUI and headless modes.
- **Catalog**: The canonical set of JSON index files under `catalog/` (agents.json, skills.json, install-roles.json, mcp-trust-matrix.json, rules.json, asset-integrity.json).
- **Agent**: A marketplace-cataloged agentic workflow definition with provider, harness variants, companion skills, lifecycle state, and metadata.
- **Skill**: A reusable capability package that agents can reference via `companion_skills`.
- **MCP_Reference**: A Model Context Protocol server reference with trust classification (mutation, egress, credentials, signing, pinning).
- **Rule**: A harness-specific configuration rule cataloged for governance.
- **Workspace**: A downstream repository that consumes catalog assets via installation. Contains `.claude/`, `.cursor/`, `.kiro/`, `.codex/`, `.opencode/` directories.
- **Workspace_Registry**: A TOML file listing downstream workspace paths and metadata for federation scanning.
- **Coverage_Matrix**: A grid showing which workspaces have which catalog assets installed, with version comparison.
- **Compliance_Score**: A per-workspace percentage indicating adoption coverage against canonical catalog or policy requirements.
- **Drift**: A state where an installed asset's content hash diverges from the canonical catalog version.
- **Policy**: A declarative TOML rule specifying required assets, trust boundaries, lifecycle gates, or compliance thresholds.
- **Policy_Violation**: A condition where a workspace fails to satisfy one or more policy rules.
- **Trust_Boundary**: A classification threshold for MCP references based on mutation capability, egress requirements, and credential needs.
- **Lifecycle**: The maturity state of a catalog asset: experimental → beta → stable → deprecated.
- **Promotion_Gate**: A condition that must be satisfied before an asset can advance to the next lifecycle stage.
- **Audit_Log**: An immutable append-only SQLite table recording policy evaluations, promotions, installations, and operator actions.
- **Validation_Gate**: One of 19+ automated checks that enforce catalog integrity, schema compliance, and security policy.
- **Gate_DAG**: A directed acyclic graph of validation gates with dependency ordering for parallel execution.
- **Headless_Mode**: The `--report` execution mode producing structured output without terminal rendering.
- **Index**: The SQLite database caching scan results, gate history, and audit records for sub-second startup.
- **Filesystem_Watcher**: A `notify` crate file watcher detecting changes to catalog files and workspace directories.
- **Content_Hash**: A SHA-256 digest of an asset file's contents used for drift detection.
- **Stale_Asset**: An installed asset whose version is N or more versions behind the canonical catalog version.
- **Dependency_Graph**: A directed graph showing relationships between agents, skills, roles, and MCP references.

## Requirements

### Requirement 1: Catalog Governance — Live Filesystem Watching

**User Story:** As a platform engineer, I want the console to detect catalog changes in real-time, so that I see drift immediately without restarting the tool.

#### Acceptance Criteria

1. WHEN the Console starts in TUI mode, THE Console SHALL establish filesystem watchers (using notify-debouncer-full for intelligent rename/modify coalescing) on all catalog JSON files under `catalog/` and reload any changed file within 2 seconds of the filesystem modification event.
2. WHEN a watched catalog file is modified (including editors that use write-to-temp-then-rename), THE Console SHALL re-parse the file, update the in-memory catalog store, recompute affected views, and display a notification indicating which catalog was refreshed.
3. WHEN a watched catalog file is modified and the new content is invalid JSON, THE Console SHALL retain the previously loaded valid state, display a warning indicating the parse error with file path and byte offset, report the catalog status as "reloaded (retained previous valid state)" in the notification area, and continue operating with stale data until a valid version is written.
4. IF a filesystem watcher encounters a permission error or the watched path is deleted, THEN THE Console SHALL log a warning, mark the affected catalog as unavailable in the status bar, and attempt to re-establish the watcher every 30 seconds.
5. WHEN running in headless mode, THE Console SHALL perform a single scan without establishing watchers, report results, and exit.
6. THE Console SHALL debounce rapid filesystem events on the same file using notify-debouncer-full, processing at most one reload per file per 500 milliseconds.
7. THE Console SHALL integrate filesystem watcher events into the TUI event loop via a tokio mpsc channel, ensuring watcher callbacks never block the render thread.

### Requirement 2: Catalog Governance — Validation Gate DAG

**User Story:** As a platform engineer, I want validation gates to execute in parallel with dependency ordering, so that I get fast feedback without redundant work.

#### Acceptance Criteria

1. THE Console SHALL parse gate dependency declarations from a `gates.toml` configuration file (or infer from `package.json` `validate:*` scripts if no TOML exists) and construct a directed acyclic graph of gate execution order.
2. WHEN the operator triggers "Run All Gates", THE Console SHALL execute independent gates in parallel (up to a configurable concurrency limit, default 4) and execute dependent gates only after their prerequisites complete successfully.
3. WHEN a gate completes, THE Console SHALL record the gate name, exit code, duration, and timestamp in the SQLite index for historical tracking.
4. WHEN a gate that has prerequisites fails, THE Console SHALL skip all downstream gates that depend on it, mark them as "skipped (dependency failed)", and report the skip reason.
5. WHEN the operator triggers a single gate, THE Console SHALL execute only that gate and its unsatisfied prerequisites, skipping gates whose cached results are still valid (same catalog content hash as last successful run).
6. THE Console SHALL display a visual DAG showing gate relationships, execution status (pending, running, passed, failed, skipped), and timing for the current run.
7. WHEN running in headless mode with `--report gates`, THE Console SHALL output gate results as a JSON array with fields: name, status, duration_ms, dependencies, skip_reason.

### Requirement 3: Catalog Governance — Install Coverage Matrix

**User Story:** As a platform lead, I want to see which downstream repos have which assets installed, so that I can identify adoption gaps across teams.

#### Acceptance Criteria

1. THE Console SHALL scan all workspaces listed in the workspace registry and produce a coverage matrix showing: rows = catalog assets (agents, skills, MCP refs, rules), columns = workspaces, cells = installed/not-installed/outdated/drifted.
2. WHEN the operator views the coverage matrix in TUI mode, THE Console SHALL render it as a scrollable grid with color-coded cells (green = current, yellow = outdated, red = drifted, gray = not installed) and support filtering by asset type, provider, or workspace.
3. WHEN an asset is installed in a workspace but its version differs from the canonical catalog version, THE Console SHALL mark it as "outdated" and display both the installed version and the canonical version in the detail view.
4. WHEN an asset is installed in a workspace but its content hash differs from the canonical catalog hash (regardless of version), THE Console SHALL mark it as "drifted" and display the hash mismatch.
5. THE Console SHALL compute a per-workspace coverage percentage as: (installed assets matching canonical) / (total canonical assets applicable to that workspace) × 100, rounded to one decimal place. IF a workspace has no applicable canonical assets (newly registered, empty policy), THE Console SHALL display "N/A" for its coverage score and exclude it from aggregate scoring.
6. WHEN running in headless mode with `--report coverage`, THE Console SHALL output the coverage matrix as JSON with per-workspace scores and per-asset installation status.
7. THE Console SHALL cache coverage scan results in the SQLite index and only re-scan workspaces whose filesystem modification time has changed since the last scan.

### Requirement 4: Catalog Governance — Asset Integrity Verification

**User Story:** As a security engineer, I want live verification that catalog assets match their recorded SHA-256 hashes, so that I can detect tampering or accidental modification.

#### Acceptance Criteria

1. THE Console SHALL read `catalog/asset-integrity.json` and verify the SHA-256 hash of each listed file against its current filesystem content, reporting pass/fail for each entry.
2. WHEN an asset file's computed hash does not match the recorded hash in `asset-integrity.json`, THE Console SHALL flag it as "integrity violation" with the expected and actual hash values.
3. WHEN the operator triggers integrity verification, THE Console SHALL process all files in parallel (up to 8 concurrent I/O operations) and complete verification of 500 files within 5 seconds on a machine with SSD storage.
4. THE Console SHALL detect when `asset-integrity.json` itself has been regenerated (by comparing its own hash against the SQLite-cached previous hash) and display a notification indicating the manifest was updated.
5. WHEN running in headless mode with `--report integrity`, THE Console SHALL output integrity results as JSON with fields: path, expected_hash, actual_hash, status (pass/fail/missing).
6. IF an asset file listed in `asset-integrity.json` does not exist on disk, THEN THE Console SHALL report it as "missing" rather than "failed" and include it in the integrity summary.

### Requirement 5: Catalog Governance — Dependency Graph

**User Story:** As a platform architect, I want to visualize which agents depend on which skills and which roles contain which agents, so that I can understand blast radius of changes.

#### Acceptance Criteria

1. THE Console SHALL construct a dependency graph from catalog data where: agents → skills (via `companion_skills`), roles → agents (via role membership), agents → MCP references (via detected usage), and agents → rules (via harness configuration).
2. WHEN the operator selects an asset in the dependency graph view, THE Console SHALL highlight all upstream dependencies (what it depends on) and downstream dependents (what depends on it).
3. WHEN the operator queries "what breaks if I remove skill X", THE Console SHALL traverse the dependency graph and list all agents that reference skill X and all roles containing those agents.
4. THE Console SHALL render the dependency graph as an ASCII art tree in TUI mode, with expandable/collapsible nodes for large graphs.
5. WHEN running in headless mode with `--report dependencies`, THE Console SHALL output the full graph as a JSON adjacency list with edge types (depends-on, contains, references).
6. THE Console SHALL detect circular dependencies (if any exist in malformed catalog data) and report them as warnings rather than entering an infinite loop.

### Requirement 6: Multi-Workspace Federation — Workspace Registry

**User Story:** As a platform lead, I want to register downstream repos in a TOML file, so that the console knows which workspaces to scan.

#### Acceptance Criteria

1. THE Console SHALL read a workspace registry from `~/.config/vfa/workspaces.toml` (or a path specified by `--registry <path>`) containing a list of workspace entries, each with at minimum: `path` (filesystem path), `name` (human label), and optional `team` (owning team name).
2. WHEN the registry file does not exist, THE Console SHALL display a setup prompt offering to create an empty registry at the default location; the Console SHALL create the file only if the operator explicitly confirms through the prompt, and SHALL proceed with zero workspaces registered regardless of the confirmation outcome.
3. WHEN a workspace path in the registry does not exist or is not accessible, THE Console SHALL mark that workspace as "unavailable" in all views, display a warning, and continue scanning remaining workspaces.
4. THE Console SHALL validate the registry TOML on load, rejecting entries with missing required fields and reporting which entries failed validation.
5. WHEN the operator adds a new workspace to the registry while the Console is running, THE Console SHALL detect the registry file change (via filesystem watcher) and incorporate the new workspace within 5 seconds; IF the updated registry file is invalid TOML, THEN THE Console SHALL reject the entire change, retain the previous valid registry state, and display an error indicating the parse failure.
6. THE Console SHALL support at least 200 workspace entries without degraded startup performance (initial scan completing within 30 seconds for 200 workspaces on SSD storage).
7. WHEN running in headless mode, THE Console SHALL accept `--workspace-filter <pattern>` to restrict operations to workspaces whose name or path matches the glob pattern.

### Requirement 7: Multi-Workspace Federation — Install Scan

**User Story:** As a platform lead, I want the console to scan each workspace's agent directories to build a coverage picture, so that I know what's actually deployed.

#### Acceptance Criteria

1. THE Console SHALL scan each registered workspace's `.claude/`, `.cursor/`, `.kiro/`, `.codex/`, and `.opencode/` directories for installed agent, skill, and rule files.
2. THE Console SHALL identify installed assets using a multi-strategy approach requiring both layout and metadata confirmation: (a) matching filenames against canonical catalog `path` basename patterns AND directory structure matching the harness layout, (b) detecting `vfa-export-agents` metadata comments (`# VFA-EXPORT:` header lines containing asset ID and version), and (c) matching file content signatures (first 50 lines) against known asset templates. An asset is considered "confirmed installed" only when at least two signals agree (layout + metadata, or layout + content signature).
3. WHEN an installed asset file is found, THE Console SHALL compute its SHA-256 hash and compare it against the canonical `catalog/asset-integrity.json` entry for drift detection.
4. THE Console SHALL store scan results in the SQLite index with: workspace path, asset ID, installed version (if detectable from file metadata), content hash, scan timestamp, and detection method (filename/metadata-comment/content-signature).
5. WHEN the operator triggers a rescan of a specific workspace, THE Console SHALL re-scan only that workspace and update its index entries within 5 seconds for a workspace with up to 50 installed assets.
6. IF a workspace's agent directory structure does not match any known harness layout, THEN THE Console SHALL log a warning and skip that directory without affecting other workspace scans.
7. THE Console SHALL detect assets installed via `vfa-export-agents` by recognizing the `# VFA-EXPORT: {"id": "...", "version": "...", "installed_at": "..."}` metadata comment line injected by the export CLI at the top of installed files.
8. THE Console SHALL support harness-specific detection patterns: `.claude/` (markdown files with agent ID in filename), `.cursor/` (JSON config referencing agent paths), `.kiro/` (steering files with agent references), `.codex/` (plugin.json entries), `.opencode/` (TOML/YAML agent definitions).

### Requirement 8: Multi-Workspace Federation — Version Comparison

**User Story:** As a platform lead, I want to see which workspaces are behind on asset versions, so that I can prioritize update campaigns.

#### Acceptance Criteria

1. THE Console SHALL compare each installed asset's version against the canonical catalog version, using a priority-ordered extraction strategy: (a) `VFA-EXPORT` metadata comment version field, (b) version string in file frontmatter/header, (c) content hash match against known canonical versions in the index history.
2. WHEN displaying the version comparison view, THE Console SHALL show a per-workspace table with columns: asset name, installed version, canonical version, version delta, and status (current/outdated/unknown).
3. THE Console SHALL compute a "freshness score" per workspace as: (assets at current version) / (total installed assets with detectable versions) × 100, rounded to one decimal place. IF no installed assets have detectable versions, THE Console SHALL report a freshness score of 0%.
4. WHEN running in headless mode with `--report versions`, THE Console SHALL output version comparison data as JSON with per-workspace freshness scores and per-asset version details.
5. IF an asset's version cannot be determined from the installed file via any extraction strategy, THEN THE Console SHALL fall back to content hash comparison and classify it as "current" if hashes match or "drifted" if they differ, with status set to "unknown-version".
6. THE Console SHALL parse version strings as semantic versions (major.minor.patch) for comparison and delta computation; non-semver strings SHALL be compared lexicographically with a warning logged.

### Requirement 9: Multi-Workspace Federation — Stale Asset Alerts

**User Story:** As a platform lead, I want alerts when installed assets fall N versions behind canonical, so that I can enforce update SLAs.

#### Acceptance Criteria

1. THE Console SHALL flag an installed asset as "stale" when its version is more than a configurable threshold (default: 2 minor versions) behind the canonical version.
2. WHEN stale assets are detected, THE Console SHALL display them in a dedicated "Stale Assets" view grouped by workspace, showing: asset name, installed version, canonical version, versions behind, and days since last update.
3. THE Console SHALL compute a stale asset count per workspace and display it in the workspace summary view.
4. WHEN running in headless mode with `--report stale`, THE Console SHALL output stale asset data as JSON and exit with exit code 1 if any workspace exceeds a configurable stale threshold (default: 5 stale assets).
5. THE Console SHALL support configuring the staleness threshold via the workspace registry TOML (per-workspace override) or a global `[policy]` section.

### Requirement 10: Multi-Workspace Federation — Drift Detection

**User Story:** As a security engineer, I want to detect when installed assets diverge from canonical content, so that I can identify unauthorized modifications.

#### Acceptance Criteria

1. THE Console SHALL compare the SHA-256 content hash of each installed asset against the canonical hash from `catalog/asset-integrity.json` and flag mismatches as "drifted".
2. WHEN drift is detected, THE Console SHALL display the drifted assets in a dedicated view showing: workspace, asset path, expected hash (first 12 hex chars), actual hash (first 12 hex chars), and time since last known-good state.
3. THE Console SHALL distinguish between "version drift" (asset was intentionally updated to a different version) and "content drift" (asset was modified without a version change) by cross-referencing version metadata.
4. WHEN running in headless mode with `--report drift`, THE Console SHALL output drift data as JSON; THE Console SHALL exit with exit code 1 only for content drift (modified without version change), and exit with code 0 for version drift (intentionally updated to a different version).
5. THE Console SHALL track drift history in the SQLite index, recording when drift was first detected and when it was resolved, for audit trail purposes.

### Requirement 11: Policy Engine — Declarative Policy Rules

**User Story:** As a security officer, I want to define mandatory asset requirements in a TOML file, so that I can enforce organizational standards programmatically.

#### Acceptance Criteria

1. THE Console SHALL read policy rules from a `policies.toml` file (path configurable via `--policies <path>`, default: `~/.config/vfa/policies.toml`) containing declarative rules in TOML format.
2. THE Console SHALL support the following policy rule types: `require_asset` (workspace must have asset X installed), `require_role` (workspace must have all assets from role Y), `max_stale` (workspace must not exceed N stale assets), `trust_boundary` (MCP refs must not exceed trust threshold), and `lifecycle_gate` (assets must be at minimum lifecycle stage).
3. WHEN a policy rule is evaluated against a workspace, THE Console SHALL produce a deterministic pass/fail result: same workspace state and same policy always produces the same verdict.
4. THE Console SHALL validate the policies.toml file on load, reporting syntax errors with line numbers and rejecting rules that reference nonexistent catalog assets or roles.
5. IF the policies.toml file does not exist, THEN THE Console SHALL operate without policy enforcement and display a notice indicating no policies are configured. IF the file exists but contains no rules, THE Console SHALL display the same notice and operate without enforcement.
6. THE Console SHALL support policy rule scoping: rules can target all workspaces, workspaces matching a name pattern, or workspaces belonging to a specific team.
7. THE Console SHALL always display the "no policies configured" notice when applicable; IF the notice cannot be displayed (e.g., terminal write failure), THE Console SHALL treat this as an operational error and exit with code 2.

### Requirement 12: Policy Engine — Trust Boundary Enforcement

**User Story:** As a security officer, I want to flag MCP references that violate trust thresholds, so that I can prevent high-risk integrations from spreading unchecked.

#### Acceptance Criteria

1. THE Console SHALL read trust classifications from `catalog/mcp-trust-matrix.json` and evaluate each installed MCP reference against policy-defined trust boundaries.
2. WHEN a policy defines a trust boundary (e.g., `max_mutation = false`, `max_egress = false`), THE Console SHALL flag any workspace with an MCP reference exceeding the boundary as a policy violation.
3. THE Console SHALL display trust violations in a dedicated view showing: workspace, MCP reference name, violated boundary (mutation/egress/credentials), and the trust matrix values for that reference.
4. WHEN running in headless mode with `--report trust`, THE Console SHALL output trust violations as JSON and exit with exit code 1 if any violations exist.
5. THE Console SHALL support trust boundary overrides per workspace (via the workspace registry TOML) for approved exceptions; THE Console SHALL record the override in the audit log only when the override is applied during policy evaluation, including the override reason, approver, and the violation that was suppressed.

### Requirement 13: Policy Engine — Lifecycle Promotion Gates

**User Story:** As a platform lead, I want to track and enforce lifecycle transitions for catalog assets, so that experimental assets don't reach production without review.

#### Acceptance Criteria

1. THE Console SHALL read the `lifecycle` field from each catalog agent entry and display a lifecycle dashboard showing asset counts per stage: experimental, beta, stable, deprecated.
2. WHEN a policy defines a lifecycle gate (e.g., `min_lifecycle = "beta"` for production workspaces), THE Console SHALL flag any workspace installing an asset below the minimum lifecycle stage as a policy violation.
3. THE Console SHALL record lifecycle transitions in the audit log when a catalog asset's lifecycle field changes between scans, capturing: asset ID, previous stage, new stage, and detection timestamp.
4. WHEN the operator queries promotion history for an asset, THE Console SHALL display the full lifecycle timeline from the audit log with dates and any associated gate results.
5. WHEN running in headless mode with `--report lifecycle`, THE Console SHALL output lifecycle status as JSON with per-asset current stage and promotion history.

### Requirement 14: Policy Engine — Audit Log

**User Story:** As a compliance auditor, I want an immutable record of all policy evaluations, promotions, and significant operator actions, so that I can provide evidence for compliance reviews.

#### Acceptance Criteria

1. THE Console SHALL maintain an append-only audit log in a dedicated SQLite table with columns: id (auto-increment), timestamp (ISO 8601 with millisecond precision), event_type (policy_evaluation, promotion, installation_detected, drift_detected, operator_action), subject (asset or workspace identifier), details (JSON blob), and operator (system user or "headless").
2. THE Console SHALL enforce append-only semantics by opening the audit log table with a SQLite trigger that rejects UPDATE and DELETE operations, returning an error if any code path attempts modification of existing rows.
3. WHEN a policy evaluation completes, THE Console SHALL record the evaluation result (pass/fail per rule) with the full policy rule text, workspace identifier, and computed score.
4. THE Console SHALL support exporting the audit log as JSON or CSV via `--export-audit <format> <output-path>` for compliance review.
5. WHEN the audit log SQLite file exceeds a configurable size threshold (default: 100 MB), THE Console SHALL emit a warning but SHALL NOT truncate or rotate the log automatically.
6. THE Console SHALL record the Console version and schema version in the audit log metadata for forward-compatibility verification.
7. WHEN running in headless mode, THE Console SHALL record all operations in the audit log identically to TUI mode, with the operator field set to "headless".
8. THE Console SHALL compute a SHA-256 hash chain: each new audit entry's hash includes the previous entry's hash, enabling tamper detection by verifying the chain integrity.

### Requirement 15: Policy Engine — Violations Dashboard

**User Story:** As a platform lead, I want a single view of all policy violations across all workspaces, so that I can prioritize remediation.

#### Acceptance Criteria

1. THE Console SHALL display a violations dashboard showing all active policy violations grouped by severity (critical, warning, info) and then by workspace.
2. WHEN the operator selects a violation, THE Console SHALL display: the violated policy rule text, the workspace and asset involved, when the violation was first detected, and suggested remediation steps.
3. THE Console SHALL compute a "compliance score" per workspace as: (passed policy rules) / (total applicable policy rules) × 100, rounded to one decimal place.
4. THE Console SHALL rank workspaces by compliance score in ascending order (worst first) in the violations dashboard.
5. WHEN running in headless mode with `--report violations`, THE Console SHALL output all violations as JSON with severity, workspace, rule, and detection timestamp, and exit with exit code 1 if any critical violations exist.
6. THE Console SHALL support suppressing specific violations via a `[suppressions]` section in policies.toml, recording the suppression reason and expiry date.
7. WHEN a previously violated policy rule is now satisfied (workspace has remediated), THE Console SHALL explicitly clear the violation flag, remove it from the violations dashboard, and record a "violation_resolved" event in the audit log with the resolution timestamp.

### Requirement 16: Dual Interface — TUI Mode

**User Story:** As a platform engineer, I want a rich interactive dashboard with vim keybindings, tabs, and live updates, so that I can efficiently operate the console daily.

#### Acceptance Criteria

1. THE Console SHALL render using ratatui 0.30 with crossterm 0.28 backend, supporting a tabbed interface with tabs for: Overview, Coverage Matrix, Validation Gates, Policy Violations, Audit Log, Dependencies, and Settings.
2. THE Console SHALL support vim-style navigation (h/j/k/l, g/G, Ctrl-d/Ctrl-u) in all list views, with `/` activating fuzzy search powered by nucleo-matcher.
3. WHEN the operator presses Tab, THE Console SHALL cycle through tabs in order; Shift-Tab SHALL cycle in reverse order.
4. THE Console SHALL display live-updating data from filesystem watchers without requiring manual refresh, with a visual indicator showing the last refresh timestamp per data source.
5. THE Console SHALL support terminal resize events, re-rendering the full layout within 100 milliseconds with no artifacts.
6. THE Console SHALL display a persistent status bar showing: active workspace count, total assets, compliance score (aggregate), and any active warnings.
7. WHEN the operator presses `?`, THE Console SHALL display a help overlay showing all keybindings for the current view.
8. THE Console SHALL use a tokio-based event loop that multiplexes: crossterm terminal events (keyboard, resize), filesystem watcher notifications (via mpsc channel), background scan completion signals, and a 250ms tick for animation/status updates — ensuring the render thread is never blocked by I/O operations.
9. THE Console SHALL maintain separation between the async event loop (tokio) and the synchronous ratatui rendering, using a single-threaded render loop that polls async channels without blocking.

### Requirement 17: Dual Interface — Headless Mode

**User Story:** As a CI/CD engineer, I want structured output from the console for pipeline integration, so that I can automate compliance checks without a terminal.

#### Acceptance Criteria

1. WHEN launched with `--report <type>`, THE Console SHALL produce structured output to stdout without requiring a terminal (no alternate screen, no raw mode, no cursor manipulation).
2. THE Console SHALL support report types: `coverage`, `violations`, `drift`, `stale`, `gates`, `integrity`, `versions`, `dependencies`, `lifecycle`, `summary`, and `all`.
3. THE Console SHALL support output formats via `--format <fmt>`: `json` (default), `markdown` (GitHub-flavored tables), and `table` (aligned ASCII columns). IF conflicting format flags are specified (e.g., multiple `--format` values), THE Console SHALL fail with a usage error and exit with code 2.
4. WHEN the report indicates compliance failures, THE Console SHALL exit with exit code 1; when all checks pass, exit code 0; when the console itself encounters an error, exit code 2. These exit codes apply only to headless mode; TUI mode always exits with code 0 on normal quit regardless of violation state.
5. THE Console SHALL support `--quiet` flag that suppresses progress output and emits only the final structured report.
6. THE Console SHALL complete headless execution and produce output within 60 seconds for a configuration with 100 workspaces and 500 catalog assets on a machine with SSD storage and 4+ CPU cores.
7. WHEN multiple report types are requested (e.g., `--report all`), THE Console SHALL produce a combined JSON object with each report type as a top-level key.

### Requirement 18: Dual Interface — Exit Codes

**User Story:** As a CI/CD engineer, I want machine-readable exit codes, so that I can use the console as a pass/fail gate in pipelines.

#### Acceptance Criteria

1. THE Console SHALL use exit code 0 for successful execution with no policy violations or failures.
2. THE Console SHALL use exit code 1 for successful execution that detected policy violations, content drift, stale assets exceeding threshold, or gate failures.
3. THE Console SHALL use exit code 2 for console operational errors (invalid config, missing registry, missing catalog directory, inaccessible workspaces exceeding threshold).
4. THE Console SHALL use exit code 3 for partial catalog failure (catalog directory exists but individual files are corrupted or unreadable).
5. THE Console SHALL document all exit codes in `--help` output and in a machine-readable `exit-codes.json` file.
6. WHEN multiple failure conditions exist simultaneously, THE Console SHALL use the highest-severity exit code (3 > 2 > 1 > 0).
7. Exit codes 1-3 apply only to headless mode (`--report`); TUI mode SHALL exit with code 0 on normal operator quit regardless of detected violations.

### Requirement 19: Persistence — SQLite Index

**User Story:** As a platform engineer, I want sub-second startup even with 100+ workspaces, so that the console is responsive for daily use.

#### Acceptance Criteria

1. THE Console SHALL maintain a SQLite database at `~/.local/share/vfa/index.db` (or path specified by `--index-path`) storing: scan results, gate history, audit log, and content hashes.
2. WHEN the Console starts with a populated index, THE Console SHALL display cached data within 500 milliseconds and begin background re-validation of stale entries.
3. THE Console SHALL invalidate cached scan results for a workspace when the workspace's filesystem modification time is newer than the cached scan timestamp.
4. THE Console SHALL perform schema migrations automatically on startup when the index schema version is older than the Console binary's expected schema, preserving all existing data.
5. IF the SQLite index file is corrupted or inaccessible, THEN THE Console SHALL create a new empty index, log a warning, and perform a full scan. WHILE running, THE Console SHALL attempt to recreate or repair the SQLite file periodically (every 60 seconds) if the initial creation succeeded but subsequent writes fail, falling back to in-memory operation between attempts.
6. THE Console SHALL support `--rebuild-index` flag that drops and recreates the index from a fresh filesystem scan.
7. THE Console SHALL open the SQLite database in WAL (Write-Ahead Logging) mode to allow concurrent reads from the TUI render thread while background scan tasks write updates.
8. THE Console SHALL funnel all SQLite write operations through a single dedicated writer task (tokio task with mpsc receiver), preventing connection contention while allowing multiple concurrent read connections for the UI thread and background tasks.
9. THE Console SHALL use rusqlite with `SQLITE_OPEN_NO_MUTEX` flag and manage thread safety via Rust's ownership system, creating separate `Connection` instances per thread/task.

### Requirement 20: Security — Command Injection Prevention

**User Story:** As a security engineer, I want the console to prevent command injection in all subprocess calls, so that untrusted input cannot escape into shell execution.

#### Acceptance Criteria

1. THE Console SHALL invoke all subprocesses using direct process spawning (no shell interpolation) with arguments passed as an array.
2. THE Console SHALL validate all user-provided path arguments by resolving symlinks to their canonical form and rejecting any path whose resolved form references a location outside the workspace root or designated scan directories.
3. THE Console SHALL reject any argument containing shell metacharacters (`;`, `|`, `&`, `$`, `` ` ``, `\`, `<`, `>`, `(`, `)`, `{`, `}`, `!`, `#`, `*`, `?`, `[`, `]`, newline, carriage return, null byte) when constructing subprocess argument arrays.
4. IF a user-provided argument fails validation, THEN THE Console SHALL display an error message identifying the rejected input and the validation rule violated.
5. THE Console SHALL sanitize workspace registry paths before filesystem operations, rejecting paths with null bytes or non-UTF-8 sequences.

### Requirement 21: Security — No Credential Exposure

**User Story:** As a security engineer, I want the console to never expose credentials in output or logs, so that sensitive data cannot leak through terminal history or audit trails.

#### Acceptance Criteria

1. THE Console SHALL NOT read, display, or log environment variables matching secret patterns (AWS_SECRET_ACCESS_KEY, GITHUB_TOKEN, NPM_TOKEN, and names containing _SECRET, _KEY, _TOKEN, _PASSWORD, _CREDENTIAL) using case-insensitive comparison.
2. THE Console SHALL remove secret environment variables before spawning any subprocess.
3. WHEN displaying subprocess output, THE Console SHALL replace strings matching secret patterns (base64 strings > 40 chars, JWT values, private key blocks, prefixes ghp_, github_pat_, npm_, sk-, xoxb-, xoxp-, AKIA) with a fixed redaction placeholder.
4. THE Console SHALL NOT write secret values to the SQLite index or audit log.
5. WHEN the Console redacts a secret, THE Console SHALL preserve surrounding non-secret content unchanged.

### Requirement 22: Security — Terminal Escape Injection Mitigation

**User Story:** As a security engineer, I want the console to sanitize all displayed content against terminal escape injection, so that malicious catalog or workspace content cannot execute terminal control sequences.

#### Acceptance Criteria

1. WHEN displaying catalog data or workspace scan results, THE Console SHALL replace control bytes (0x00-0x1F except newline 0x0A and tab 0x09, 0x7F DEL, and Unicode C1 controls U+0080-U+009F) with U+FFFD before rendering.
2. WHEN displaying subprocess output, THE Console SHALL pass through SGR sequences (CSI + numeric params + 'm') for color formatting and strip all other escape sequences (OSC, DCS, SOS, PM, APC) and C1 controls. Subprocess output SHALL be displayed after applying secret redaction (replacing matched patterns with placeholders) — the redacted output remains visible to the operator.
3. WHEN parsing workspace file paths from the registry or filesystem scan, THE Console SHALL reject paths containing control characters, non-printable Unicode, null bytes, or non-UTF-8 sequences, and log a warning identifying the rejected path and the specific invalid character class.
4. THE Console SHALL apply sanitization to all strings read from external sources (catalog JSON, workspace files, TOML configs, subprocess output) before rendering or storing in the index.

### Requirement 23: Performance — Scan Efficiency

**User Story:** As a platform engineer operating at scale, I want scans to complete quickly even with many workspaces, so that the console remains responsive.

#### Acceptance Criteria

1. THE Console SHALL scan workspaces in parallel (up to configurable concurrency, default 8) using tokio tasks for I/O-bound operations.
2. THE Console SHALL use incremental scanning: only re-scan files whose modification time is newer than the cached scan timestamp in the SQLite index.
3. THE Console SHALL complete a full initial scan of 100 workspaces (each with ~50 installed assets) within 30 seconds on a machine with SSD storage and 4+ CPU cores. Subsequent incremental scans (detecting changes only) SHALL satisfy this requirement via cached metadata checks.
4. THE Console SHALL complete incremental re-scans (no changes detected) within 2 seconds for 100 workspaces by checking only filesystem metadata.
5. WHEN a workspace is unavailable (network mount down, permission denied), THE Console SHALL time out that workspace after 5 seconds and continue scanning remaining workspaces.

### Requirement 24: Performance — Startup Time

**User Story:** As a platform engineer, I want the console to start quickly, so that I can use it for quick checks without waiting.

#### Acceptance Criteria

1. WHEN the SQLite index is populated and up-to-date, THE Console SHALL display the initial view within 500 milliseconds of launch.
2. WHEN the SQLite index is empty (first run), THE Console SHALL display a loading indicator within 200 milliseconds and begin the initial scan in the background, with the scan task actively processing within 500 milliseconds of launch (not merely scheduled).
3. THE Console binary SHALL be statically linkable (no dynamic library dependencies beyond libc) for fast cold-start on containerized environments.
4. THE Console SHALL load and parse the workspace registry TOML within 50 milliseconds for a registry containing 200 entries.

### Requirement 25: Graceful Degradation

**User Story:** As a platform engineer, I want the console to handle partial failures without crashing, so that I can still work when some workspaces or files are unavailable.

#### Acceptance Criteria

1. IF a workspace path in the registry is unavailable, THE Console SHALL mark it as "offline" in all views, skip it during scans, and continue operating with remaining workspaces.
2. IF the SQLite index file cannot be opened, THE Console SHALL fall back to in-memory operation with a warning that data will not persist.
3. IF the policies.toml file contains syntax errors, THE Console SHALL report the specific error with line number, skip the malformed rules, and apply remaining valid rules.
4. IF the catalog directory is missing or empty, THE Console SHALL display an error and exit with code 2 (operational error). IF the catalog directory exists but individual files are corrupted or partially unreadable, THE Console SHALL exit with code 3 (partial catalog failure) after displaying which files failed.
5. WHEN a filesystem watcher fails on a specific path, THE Console SHALL continue watching remaining paths and attempt to re-establish the failed watcher periodically (every 30 seconds).
6. THE Console SHALL NOT panic on any recoverable error path and SHALL remain responsive to user input after encountering errors.

### Requirement 26: CLI Interface

**User Story:** As a platform engineer, I want a comprehensive CLI with subcommands, so that I can use the console in both interactive and scripted workflows.

#### Acceptance Criteria

1. THE Console SHALL accept the following top-level flags: `--registry <path>` (workspace registry), `--policies <path>` (policy file), `--index-path <path>` (SQLite index), `--log-file <path>`, `--log-level <level>`, `--no-color`, `--version`, `--help`.
2. THE Console SHALL accept `--report <type>` for headless mode with types: coverage, violations, drift, stale, gates, integrity, versions, dependencies, lifecycle, summary, all.
3. THE Console SHALL accept `--format <fmt>` for output format: json (default), markdown, table.
4. THE Console SHALL accept `--workspace-filter <pattern>` to restrict operations to matching workspaces.
5. THE Console SHALL accept `--rebuild-index` to force a complete re-scan.
6. THE Console SHALL accept `--quiet` to suppress progress output in headless mode.
7. THE Console SHALL parse all CLI arguments using `clap` 4.x with derive macros for type-safe handling.
8. IF an unrecognized flag or invalid value is provided, THEN THE Console SHALL display a usage error message describing the valid options and exit with code 2 only after successfully writing the error message to stderr.

### Requirement 27: Deterministic Behavior

**User Story:** As a CI/CD engineer, I want deterministic output for the same inputs, so that I can trust pipeline results are reproducible.

#### Acceptance Criteria

1. THE Console SHALL produce identical headless output for the same: catalog data, workspace contents, policy rules, and CLI flags.
2. THE Console SHALL use stable sort orders (case-insensitive lexicographic by ID) for all list outputs unless explicitly overridden.
3. THE Console SHALL NOT make network requests under any circumstance.
4. THE Console SHALL NOT read environment variables to alter compliance scores, policy evaluations, or report content; only explicit CLI flags and configuration file contents SHALL influence results. Exception: environment variable expansion in registry paths (Req 30) affects which workspaces are scanned, not how results are computed.
5. WHEN computing compliance scores, THE Console SHALL use deterministic rounding (round half up) and consistent floating-point handling across platforms.

### Requirement 28: Cross-Platform Compatibility

**User Story:** As a platform engineer, I want the console to work on Linux, macOS, and WSL, so that platform teams on different OSes can use it.

#### Acceptance Criteria

1. THE Console SHALL compile and run on: x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu, x86_64-apple-darwin, aarch64-apple-darwin, and x86_64-unknown-linux-musl (WSL).
2. THE Console SHALL use platform-appropriate default paths: XDG directories on Linux, `~/Library/Application Support/` on macOS.
3. WHEN running on Windows WSL, THE Console SHALL detect the WSL environment via the presence of /proc/sys/fs/binfmt_misc/WSLInterop or the WSL_DISTRO_NAME environment variable and adapt filesystem scanning to handle Windows-style symlinks and case-insensitive paths. IF WSL detection fails or produces ambiguous results, THE Console SHALL fall back to standard Linux filesystem handling and continue operation normally.
4. THE Console SHALL compile and run successfully regardless of WSL adaptation success — WSL-specific adaptations are best-effort optimizations, not hard requirements.
4. THE Console SHALL handle filesystem path separators correctly on all platforms using Rust's `std::path` abstractions.

### Requirement 29: Accessibility

**User Story:** As a platform engineer with accessibility needs, I want the console to be usable with screen readers and high-contrast terminals, so that I am not excluded from platform operations.

#### Acceptance Criteria

1. THE Console SHALL support `--no-color` flag that disables all ANSI color codes, producing output readable by screen readers and compatible with text-to-speech tools.
2. THE Console SHALL always use semantic text indicators alongside color (e.g., [PASS], [FAIL], [WARN], [DRIFT], [STALE] text prefixes) so that status is conveyed without relying solely on color, regardless of whether `--no-color` is active.
3. THE Console SHALL support the `NO_COLOR` environment variable (per no-color.org standard) as equivalent to `--no-color`.
4. THE Console SHALL produce headless mode output that is fully parseable by standard JSON/Markdown tools without requiring visual interpretation.

### Requirement 30: Workspace Registry TOML Format

**User Story:** As a platform lead, I want a well-defined registry format, so that I can manage workspace lists programmatically.

#### Acceptance Criteria

1. THE Console SHALL parse workspace registry entries in the following TOML format: `[[workspace]]` array of tables with required field `path` (string) and optional fields `name` (string, default: directory basename), `team` (string), `tags` (array of strings), and `policy_overrides` (table).
2. THE Console SHALL support environment variable expansion in `path` fields (e.g., `$HOME/repos/my-app`) using a safe expansion that only resolves known variables without shell execution.
3. WHEN a registry entry has a `policy_overrides` table, THE Console SHALL apply those overrides when evaluating policies against that specific workspace, merging with global policies.
4. THE Console SHALL support comments in the registry TOML file for documentation purposes.
5. THE Console SHALL reject duplicate workspace paths (after expansion and canonicalization) and report which entries conflict. Duplicate detection SHALL occur as a separate validation pass that succeeds even if other parsing validations fail, ensuring conflicts are always reported.

### Requirement 31: Parser Round-Trip — TOML Configuration

**User Story:** As a platform engineer, I want TOML configuration files to be parseable and re-serializable without data loss, so that programmatic editing is reliable.

#### Acceptance Criteria

1. THE Console SHALL parse policies.toml and workspaces.toml using the `toml` crate with strict mode (rejecting unknown keys in known sections).
2. THE Console SHALL support a `--validate-config` flag that parses all configuration files, reports any errors, and exits without performing operations.
3. FOR ALL valid TOML configuration inputs, parsing then serializing then parsing again SHALL produce an equivalent data structure (round-trip property).
4. IF a TOML file contains a key not recognized by the Console's schema, THEN THE Console SHALL report a warning identifying the unknown key and its location, and continue operation ignoring the unknown key.

### Requirement 32: Catalog Discovery — Enhanced Browsing

**User Story:** As a platform engineer, I want the enhanced catalog browsing from v1 preserved and extended, so that I retain full discoverability while gaining governance features.

#### Acceptance Criteria

1. THE Console SHALL parse all catalog JSON files (agents.json, skills.json, install-roles.json, mcp-trust-matrix.json, rules.json, asset-integrity.json) and display them in dedicated browsable views with fuzzy search.
2. WHEN the operator searches in any catalog view, THE Console SHALL filter results using nucleo-matcher fuzzy matching against ID, name, provider, and summary fields, updating within 100 milliseconds.
3. THE Console SHALL support filtering by provider, harness, lifecycle stage, and execution tier in the agent catalog view.
4. WHEN the operator selects a catalog item, THE Console SHALL display a detail panel with all fields, rendering absent optional values as "N/A".
5. THE Console SHALL display cross-references in detail views: agents show their roles, skills show their referencing agents, MCP refs show their trust classification.

### Requirement 33: Web Mode (Stretch Goal)

**User Story:** As a platform lead, I want a browser-accessible view of the operator console, so that stakeholders without terminal access can view compliance status.

#### Acceptance Criteria

1. WHERE the `--web` flag is provided, THE Console SHALL start an embedded HTTP server on a configurable port (default: 8080) using axum, serving server-rendered HTML pages via askama templates.
2. WHERE the web mode is active, THE Console SHALL expose read-only endpoints for: coverage matrix, policy violations, compliance scores, audit log viewer, and dependency graph — with no write operations accessible via HTTP.
3. WHERE the web mode is active, THE Console SHALL use HTMX for partial page updates without requiring client-side JavaScript frameworks.
4. WHERE the web mode is active, THE Console SHALL bind only to 127.0.0.1 by default (no external access) with `--web-bind <addr>` override requiring explicit opt-in for non-localhost binding.
5. WHERE the web mode is active, THE Console SHALL NOT expose any endpoint that could modify policies, trigger scans, or execute subprocesses — it is strictly a read-only dashboard.

### Requirement 34: Event Loop Architecture

**User Story:** As a developer, I want a well-defined event loop architecture, so that the console remains responsive under concurrent filesystem events, background scans, and user input.

#### Acceptance Criteria

1. THE Console SHALL implement a unified event enum covering: terminal input events (key press, resize), filesystem watcher events (file changed, deleted), background task completions (scan complete, gate finished), timer ticks (animation, status refresh), and internal messages (error notifications, data updates).
2. THE Console SHALL process events in a single-threaded main loop that: polls all event sources without blocking for more than 50ms, dispatches events to the appropriate handler, and triggers a re-render only when state has changed (dirty flag pattern).
3. WHEN a background scan task completes, THE Console SHALL merge the results into the application state and mark affected views as dirty for re-render on the next loop iteration.
4. THE Console SHALL use tokio::select! to multiplex async event sources (terminal events, mpsc channels, interval timers) ensuring fair polling and no starvation of any event source.
5. IF the event queue accumulates more than 100 unprocessed events, THE Console SHALL batch-process them, coalescing duplicate events (e.g., multiple filesystem changes to the same file) before applying state updates.


### Requirement 35: Light/Dark Mode with System Detection

**User Story:** As a platform engineer, I want the console to automatically adapt its color palette to my terminal's background (dark or light), so that text remains readable without manual configuration.

#### Acceptance Criteria

1. THE Console SHALL detect the terminal's background luminance at startup using the `terminal-light` crate (OSC 11 escape sequence query), classifying the result as Dark (luma ≤ 0.6) or Light (luma > 0.6).
2. IF the `terminal-light` detection fails (unsupported terminal, timeout, or error), THE Console SHALL fall back to parsing the `COLORFGBG` environment variable (format: `fg;bg`; background index ≥ 7 indicates Light), and if that is also unavailable, SHALL default to Dark mode.
3. THE Console SHALL accept a `--theme <mode>` CLI flag with values `auto` (default — use system detection), `dark` (force dark palette), and `light` (force light palette); the explicit flag SHALL override system detection.
4. WHEN in Dark mode, THE Console SHALL use a palette optimized for dark backgrounds: light foreground text (White/Gray), Cyan/Blue accents, and no explicit background color (inheriting terminal default).
5. WHEN in Light mode, THE Console SHALL use a palette optimized for light backgrounds: dark foreground text (Black/DarkGray), Blue/Magenta accents, and no explicit background color (inheriting terminal default).
6. THE Console SHALL provide a runtime toggle keybinding (`t` when not in search mode) that switches between Dark and Light mode without restarting, re-rendering all views immediately with the new palette.
7. WHEN `--no-color` is active or `NO_COLOR` environment variable is set, THE Console SHALL ignore the theme mode entirely and render without any ANSI color codes, using only text modifiers (bold, dim, reverse) for visual distinction.
8. THE Console SHALL maintain determinism: given the same `(ThemeMode, ColorSupport)` inputs, all style methods SHALL return identical `Style` values across invocations.
9. WHEN running in headless mode, THE Console SHALL respect `--theme` for ANSI-colored output formats (table, markdown) but SHALL NOT attempt terminal background detection (as stdin may not be a TTY); headless mode SHALL default to Dark if `--theme auto` and detection fails.
10. THE Console SHALL ensure all semantic text indicators ([PASS], [FAIL], [WARN], [DRIFT], [STALE]) remain present and readable regardless of theme mode, satisfying accessibility requirements alongside color-coded styling.
