# Requirements Document

## Introduction

Enterprise-grade terminal user interface (TUI) written in Rust for the `@raishin/vanguard-frontier-agentic` marketplace (v3.0 alpha). The TUI provides interactive discoverability, governance visibility, validation execution, and operator confidence for a catalog of 300+ agents across 30+ providers. It wraps existing CLI and validation infrastructure without duplicating business logic, operating as a read-first interface with explicit confirmation for write operations.

The TUI lives at `tools/vfa-tui/` as a separate Cargo workspace within the repository. It invokes existing Node.js/Python scripts via subprocess execution and reads catalog JSON files directly for browsing. This is a 3.0 alpha release — functional, secure, and useful, but with limited scope.

## Glossary

- **TUI**: Terminal User Interface — the Rust binary providing interactive catalog browsing and validation execution.
- **Catalog**: The set of JSON index files under `catalog/` (agents.json, skills.json, install-roles.json, mcp-references.json, rules.json, asset-integrity.json, skill-manifest.json).
- **Agent**: A marketplace-cataloged agentic workflow definition with provider, harness variants, companion skills, and metadata.
- **Provider**: A cloud/platform vendor directory under `agents/` (e.g., aws, azure, gcp, alibaba, kubernetes).
- **Harness**: An IDE/CLI platform that can execute agents (codex, copilot, claude-code, cursor, gemini, kiro, kiro-ide, kiro-cli).
- **Validation_Gate**: One of 17+ automated checks that enforce catalog integrity, schema compliance, and security policy.
- **Export_CLI**: The existing `vfa-export-agents` Node.js script that installs agents and skills to target repositories.
- **Role**: A curated cross-provider agent selection for a practitioner function (e.g., cloud-security-engineer, cloud-platform-engineer).
- **Companion_Skill**: A skill package automatically bundled with an agent during export on supported platforms.
- **Dry_Run**: A preview mode that shows what an operation would do without executing filesystem writes.
- **Audit_Log**: A structured tracing record of all TUI operations for enterprise compliance review.
- **Workspace_Root**: The root directory of the vanguard-frontier-agentic repository, detected by the TUI at startup.

## Requirements

### Requirement 1: Catalog Discovery — Agent Browsing

**User Story:** As a cloud operator, I want to browse the full agent catalog interactively, so that I can discover relevant agents without memorizing IDs or reading raw JSON.

#### Acceptance Criteria

1. WHEN the TUI starts in browse mode, THE TUI SHALL parse `catalog/agents.json` and display a scrollable list of all agents with their ID, name, provider, and summary.
2. IF `catalog/agents.json` is missing or contains invalid JSON, THEN THE TUI SHALL display an error message indicating the file could not be loaded and exit with a non-zero status code.
3. WHEN the operator types a search query of at least 1 character in the agent list view, THE TUI SHALL filter the displayed agents by fuzzy-matching against agent ID, name, provider, and summary fields, updating results within 100 milliseconds of the last keystroke.
4. WHEN the operator selects an agent from the list, THE TUI SHALL display a detail panel showing: ID, name, provider, type, harnesses, companion skills, source type, official docs, security notes, last verified date, path, harness variants, author, and version, rendering absent optional fields as a placeholder label (e.g., "N/A") rather than omitting them.
5. WHEN the operator presses a provider filter key, THE TUI SHALL restrict the agent list to agents matching the selected provider.
6. WHEN the operator presses a harness filter key, THE TUI SHALL restrict the agent list to agents that support the selected harness.
7. WHEN both a provider filter and a harness filter are active simultaneously, THE TUI SHALL display only agents that match both the selected provider AND the selected harness, applying any active search query as an additional constraint.
8. IF the active combination of search query and filters matches zero agents, THEN THE TUI SHALL display an empty-state message indicating no agents match the current criteria.
9. THE TUI SHALL display a status bar containing the count of currently visible agents, the total agent count in the catalog, and the names of any active filters or search query.

### Requirement 2: Catalog Discovery — Skill Browsing

**User Story:** As a cloud operator, I want to browse the skill catalog interactively, so that I can understand what skills are available and their relationships to agents.

#### Acceptance Criteria

1. WHEN the TUI starts in skills browse mode, THE TUI SHALL parse `catalog/skills.json` and display a scrollable list of all skills showing each skill's `id`, `name`, and `summary` (truncated to 120 characters if longer).
2. WHEN the operator selects a skill from the list, THE TUI SHALL display the skill's detail panel showing: `provider`, `path`, `version`, `author`, `harnesses`, `security_notes`, and a list of associated agents derived by reverse-lookup of agents in `catalog/agents.json` whose `companion_skills` array contains the selected skill's `id`.
3. WHEN the operator types a search query of at least 1 character in the skill list view, THE TUI SHALL filter the displayed list to skills whose `id`, `name`, or `summary` fuzzy-matches the query, updating results within 200ms of the last keystroke.
4. IF `catalog/skills.json` is missing, unreadable, or contains invalid JSON, THEN THE TUI SHALL display an error message indicating the file could not be loaded and exit skills browse mode gracefully without crashing.
5. IF the search query matches zero skills, THEN THE TUI SHALL display an empty-state message indicating no skills matched the query and keep the search input active for the operator to revise the query.

### Requirement 3: Catalog Discovery — Role Browsing

**User Story:** As a cloud operator, I want to browse install roles interactively, so that I can understand which agents are curated for each practitioner function.

#### Acceptance Criteria

1. WHEN the TUI starts in roles browse mode, THE TUI SHALL parse `catalog/install-roles.json` and display all six roles in the order they appear in the JSON file, showing each role's label, description (truncated to 120 characters with an ellipsis if longer), and the count of agents in that role's `agents` array.
2. WHEN the operator selects a role, THE TUI SHALL display the agents assigned to that role grouped by provider, where the provider is derived from the agent's `provider` field in `catalog/agents.json`, with provider groups sorted alphabetically and agents sorted alphabetically within each group.
3. WHEN the operator selects a role and then selects an agent within it, THE TUI SHALL navigate to the agent detail view for that agent, displaying at minimum the agent's ID, provider, and the role from which it was reached.
4. IF `catalog/install-roles.json` is missing or contains invalid JSON, THEN THE TUI SHALL display an error message indicating the file could not be loaded and SHALL NOT render the roles list.

### Requirement 4: Catalog Discovery — Provider Browsing

**User Story:** As a cloud operator, I want to browse agents grouped by provider, so that I can assess coverage for a specific cloud platform.

#### Acceptance Criteria

1. WHEN the TUI starts in provider browse mode, THE TUI SHALL enumerate all subdirectories under `agents/` that contain at least one agent subdirectory, display each provider name sorted alphabetically in ascending order, and show the count of agent subdirectories within each provider.
2. WHEN the operator selects a provider, THE TUI SHALL display all agents belonging to that provider in a scrollable list, showing each agent's directory name, sorted alphabetically in ascending order.
3. WHEN the operator is viewing a provider's agent list, THE TUI SHALL provide a navigation action that returns the operator to the provider list without losing the provider list state.
4. IF a provider directory under `agents/` contains no agent subdirectories, THEN THE TUI SHALL omit that directory from the provider browse list.

### Requirement 5: Catalog Discovery — MCP References and Rules

**User Story:** As a cloud operator, I want to browse MCP references and rules, so that I can understand the trust and governance configuration.

#### Acceptance Criteria

1. WHEN the TUI starts in MCP references browse mode, THE TUI SHALL parse `catalog/mcp-references.json` and display all MCP references as a list, each showing its `id` and `summary` fields, ordered by their position in the JSON array.
2. WHEN the TUI starts in rules browse mode, THE TUI SHALL parse `catalog/rules.json` and display all rules as a list, each showing its `id` and `summary` fields, ordered by their position in the JSON array.
3. WHEN the operator selects an MCP reference, THE TUI SHALL display all fields present in that entry's JSON object, including at minimum: `id`, `name`, `type`, `provider`, `harnesses`, `summary`, `source_type`, `official_docs`, `security_notes`, `last_verified`, and `path`.
4. WHEN the operator selects a rule, THE TUI SHALL display all fields present in that entry's JSON object, including at minimum: `id`, `name`, `type`, `provider`, `harnesses`, `summary`, `source_type`, `official_docs`, `security_notes`, `last_verified`, and `path`.
5. IF `catalog/mcp-references.json` or `catalog/rules.json` is missing or contains invalid JSON, THEN THE TUI SHALL display an error message indicating which file failed to parse and SHALL NOT crash.

### Requirement 6: Validation Gate Execution

**User Story:** As a platform engineer, I want to run validation gates from the TUI with real-time output, so that I can verify catalog integrity without memorizing npm script names.

#### Acceptance Criteria

1. THE TUI SHALL display a list of all validation gates defined as `validate:*` scripts in `package.json`, showing each gate's script name, description, and last-run status (pass/fail/not-run) for the current session.
2. WHEN the operator selects a single validation gate and confirms execution, THE TUI SHALL invoke the corresponding npm script as a subprocess and stream stdout/stderr line-by-line to a scrollable output panel within 1 second of the subprocess producing output.
3. WHEN the operator selects "Run All Validations" and confirms execution, THE TUI SHALL invoke `npm run validate` as a subprocess and stream output in real time, continuing execution through all gates regardless of individual gate failures.
4. WHEN a validation gate subprocess exits, THE TUI SHALL capture the exit code and update the gate status display to pass (exit 0) or fail (non-zero exit).
5. IF a validation gate subprocess exceeds a configurable timeout (default 300 seconds), THEN THE TUI SHALL terminate the subprocess, display a timeout error indicating which gate exceeded the limit, and mark the gate as failed.
6. WHILE a validation gate is executing, THE TUI SHALL display an animated progress indicator adjacent to the gate name and prevent concurrent execution of the same gate.
7. IF the operator attempts to execute a gate that is already running, THEN THE TUI SHALL display a message indicating the gate is already in progress and take no further action.
8. IF the npm binary is not found or the selected script does not exist in `package.json`, THEN THE TUI SHALL display an error message indicating the cause of failure and mark the gate as failed without hanging.

### Requirement 7: Export Command Preview and Execution

**User Story:** As a cloud operator, I want to preview and execute export commands from the TUI, so that I can install agents with confidence and full visibility into what will happen.

#### Acceptance Criteria

1. WHEN the operator initiates an export action, THE TUI SHALL present a command builder showing: platform selection (one of the platforms supported by `vfa-export-agents`), agent selection method (all, role, provider, specific agent IDs — mutually exclusive), target repository path, and optional flags (force, no-skills).
2. WHEN the operator completes all required fields in the command builder and submits, THE TUI SHALL display the exact `vfa-export-agents` command that will be executed and prompt for a yes/no confirmation before proceeding.
3. WHEN the operator confirms an export with dry-run enabled, THE TUI SHALL execute the command with `--dry-run` flag and display the preview output within the TUI output panel.
4. WHEN the operator confirms an export without dry-run, THE TUI SHALL execute the command and display output lines as they are emitted by the subprocess, with no more than 1 second delay between subprocess output and TUI display.
5. THE TUI SHALL default to dry-run mode for all export operations, requiring the operator to explicitly toggle a dry-run control to off before live execution is permitted.
6. IF the target repository path does not exist or is not writable, THEN THE TUI SHALL display an error message indicating the path issue before attempting execution and return the operator to the command builder with previous selections preserved.
7. IF the `vfa-export-agents` command exits with a non-zero status during execution, THEN THE TUI SHALL display the error output from the command, indicate that the export failed, and return the operator to the command builder with previous selections preserved.
8. WHILE the export command is executing, THE TUI SHALL display a cancel control that, when activated, terminates the subprocess and returns the operator to the command builder.

### Requirement 8: Security — Command Injection Prevention

**User Story:** As a security engineer, I want the TUI to prevent command injection in all subprocess calls, so that untrusted input cannot escape into shell execution.

#### Acceptance Criteria

1. THE TUI SHALL invoke all subprocesses using direct process spawning (no shell interpolation) with arguments passed as an array, not a concatenated string.
2. THE TUI SHALL validate all user-provided path arguments by resolving symlinks to their canonical form and rejecting any path whose resolved canonical form references a parent directory segment (`../`) or resolves outside the TUI's designated working directory.
3. THE TUI SHALL reject any argument containing shell metacharacters (`;`, `|`, `&`, `$`, `` ` ``, `\`, `<`, `>`, `(`, `)`, `{`, `}`, `!`, `#`, `*`, `?`, `[`, `]`, newline, carriage return, null byte) when constructing subprocess argument arrays.
4. IF a user-provided argument fails validation, THEN THE TUI SHALL display an error message identifying the rejected input value and the specific validation rule that was violated, without revealing internal directory structures or system paths beyond the user-supplied value itself.
5. IF a user-provided path argument contains a null byte or non-UTF-8 sequence, THEN THE TUI SHALL reject the argument before any filesystem operation or subprocess invocation is attempted.

### Requirement 9: Security — No Credential Exposure

**User Story:** As a security engineer, I want the TUI to never expose credentials or secrets in its output or logs, so that sensitive data cannot leak through terminal history or audit trails.

#### Acceptance Criteria

1. THE TUI SHALL NOT read, display, or log environment variables whose names match known secret patterns (AWS_SECRET_ACCESS_KEY, GITHUB_TOKEN, NPM_TOKEN, exact SECRET/TOKEN/PASSWORD/CREDENTIAL/KEY, and names matching *_SECRET*, *_KEY*, *_TOKEN*, *_PASSWORD*) using case-insensitive comparison.
2. THE TUI SHALL remove environment variables whose names match the secret patterns defined in criterion 1 before spawning any subprocess, so child processes cannot inherit those secret values.
3. WHEN displaying subprocess output, THE TUI SHALL replace strings matching secret patterns (base64-encoded strings longer than 40 characters, JWT-shaped values, private key blocks, strings prefixed with ghp_, github_pat_, npm_, sk-, xoxb-, xoxp-, AKIA) with a fixed redaction placeholder indicating that content was redacted.
4. THE TUI SHALL NOT write values matching the secret patterns defined in criteria 1 and 3 to any log file or audit trail.
5. WHEN the TUI redacts a secret from displayed output, THE TUI SHALL preserve the surrounding non-secret content unchanged so that the output remains readable.

### Requirement 10: Security — Terminal Escape Injection Mitigation

**User Story:** As a security engineer, I want the TUI to sanitize all displayed content against terminal escape injection, so that malicious catalog content cannot execute terminal control sequences.

#### Acceptance Criteria

1. WHEN displaying catalog data loaded from JSON files, THE TUI SHALL replace control bytes (0x00-0x1F except 0x0A newline and 0x09 tab, 0x7F DEL, and Unicode C1 controls U+0080-U+009F) with the Unicode replacement character (U+FFFD) before rendering.
2. WHEN displaying subprocess output, THE TUI SHALL pass through SGR sequences (CSI followed by numeric parameters ending in 'm') and strip all other escape sequences including OSC (ESC ]), DCS (ESC P), SOS (ESC X), PM (ESC ^), APC (ESC _), and Unicode C1 controls by removing them from the output before rendering.
3. WHEN the TUI parses a catalog JSON file, IF any string field value contains control bytes (0x00-0x1F except 0x0A and 0x09, 0x7F, or Unicode C1 controls U+0080-U+009F), THEN THE TUI SHALL skip that entry, continue loading remaining entries, and log a warning message indicating which entry was rejected and the byte offset of the offending character.
4. WHEN the TUI strips or replaces a dangerous escape sequence during display, THE TUI SHALL render the remaining content without interruption and without altering the visible layout of surrounding safe content.

### Requirement 11: Audit Trail and Structured Logging

**User Story:** As a compliance officer, I want all TUI operations logged with structured data, so that I can review operator actions for audit purposes.

#### Acceptance Criteria

1. THE TUI SHALL emit structured log events using the `tracing` crate for every user-initiated action (browse, search, filter, validate, export), where each event includes at minimum: ISO 8601 timestamp with millisecond precision, session ID, operator identifier, action type, and outcome (success or failure).
2. WHEN a subprocess is invoked, THE TUI SHALL log: the command and arguments with values of environment variables matching patterns `*SECRET*`, `*TOKEN*`, `*KEY*`, `*PASSWORD*`, and `*CREDENTIAL*` replaced by a fixed placeholder string, start timestamp, end timestamp, exit code, and the operator who initiated it.
3. THE TUI SHALL support configurable log output destinations: stderr (default), file path (via --log-file flag), or both.
4. IF the configured log file cannot be opened or written to, THEN THE TUI SHALL emit a warning to stderr indicating the log destination failure and fall back to logging to stderr only.
5. THE TUI SHALL include a session ID formatted as a UUID v4 in all log events to correlate actions within a single TUI session.
6. THE TUI SHALL log at structured levels: INFO for user actions, WARN for validation failures, ERROR for subprocess failures and security rejections.

### Requirement 12: Error Handling and Resilience

**User Story:** As a cloud operator, I want the TUI to handle errors gracefully without crashing, so that I can trust it in production workflows.

#### Acceptance Criteria

1. THE TUI SHALL NOT panic on any recoverable error path (malformed JSON, missing files, subprocess failures, terminal resize events) and SHALL remain responsive to user input after encountering any such error.
2. WHEN a catalog JSON file fails to parse, THE TUI SHALL display an error message identifying the file path and the line number or byte offset of the parse error, preserve the current view for any previously loaded data, and continue operating without the data from the failed file.
3. WHEN a required catalog file is missing, THE TUI SHALL display an error message stating the expected file path that was not found and suggest running from the workspace root, within 2 seconds of startup.
4. IF the terminal does not support 256 colors or alternate screen mode, THEN THE TUI SHALL fall back to 8-color output or, if alternate screen is unavailable, display a single-line incompatibility message stating the missing capability and exit with a non-zero status code.
5. WHEN the TUI exits (normally or on error), THE TUI SHALL restore the terminal to its original state (cursor visibility, alternate screen, raw mode disabled) before the process terminates.
6. WHEN a recoverable error occurs during normal operation, THE TUI SHALL display the error in a dedicated status region of the interface without replacing or obscuring the user's current working view, and SHALL dismiss or collapse the error notification after 10 seconds or upon user input.

### Requirement 13: Cross-Platform Terminal Compatibility

**User Story:** As a cloud operator, I want the TUI to work on Linux, macOS, and Windows (WSL), so that I can use it regardless of my development environment.

#### Acceptance Criteria

1. THE TUI SHALL compile without errors and launch to a rendered initial screen on x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu, x86_64-apple-darwin, aarch64-apple-darwin, and x86_64-unknown-linux-musl (for WSL) targets.
2. THE TUI SHALL use the crossterm backend for terminal abstraction, ensuring that keyboard input handling, color rendering, and cursor movement produce the same observable results across all supported targets.
3. WHEN running on Windows WSL, THE TUI SHALL detect the WSL environment via the presence of /proc/sys/fs/binfmt_misc/WSLInterop or the WSL_DISTRO_NAME environment variable and fall back to a safe terminal capability set that excludes features unsupported by the WSL pseudo-terminal.
4. WHEN a terminal resize event occurs, THE TUI SHALL re-render the full layout within 100 milliseconds with no overlapping text, no truncated lines, and no stale content from the previous dimensions remaining visible.
5. IF the TUI is launched on a terminal that does not support a required capability, THEN THE TUI SHALL display an error message indicating the missing capability and exit with a non-zero status code within 2 seconds.

### Requirement 14: CLI Interface and Configuration

**User Story:** As a cloud operator, I want to launch the TUI with CLI flags for common operations, so that I can integrate it into scripts and workflows.

#### Acceptance Criteria

1. THE TUI SHALL accept the following CLI flags: `--workspace <path>` (workspace root, default: auto-detect), `--log-file <path>`, `--log-level <level>` (accepted values: `trace`, `debug`, `info`, `warn`, `error`; default: `info`), `--no-color` (disables ANSI color codes in all terminal output), `--version`, `--help`.
2. WHEN launched without flags, THE TUI SHALL auto-detect the workspace root by searching upward from the current working directory to the filesystem root for a directory containing `catalog/agents.json` and `package.json` with name `@raishin/vanguard-frontier-agentic`.
3. IF workspace auto-detection fails, THEN THE TUI SHALL display an error message indicating that no valid workspace was found and exit with exit code 1.
4. WHEN launched with `--version`, THE TUI SHALL display the binary version (matching the Cargo.toml version) and exit with exit code 0.
5. THE TUI SHALL parse CLI arguments using the `clap` crate with derive macros for type-safe argument handling.
6. IF the `--workspace` path does not exist or does not contain the required workspace markers (`catalog/agents.json` and `package.json` with name `@raishin/vanguard-frontier-agentic`), THEN THE TUI SHALL display an error message indicating the invalid workspace path and exit with exit code 1.
7. IF an unrecognized flag or an invalid value for `--log-level` is provided, THEN THE TUI SHALL display a usage error message describing the valid options and exit with exit code 2.

### Requirement 15: Workspace Detection and Catalog Loading

**User Story:** As a cloud operator, I want the TUI to automatically find and load the marketplace catalog, so that I do not need to configure paths manually.

#### Acceptance Criteria

1. WHEN the TUI starts, THE TUI SHALL locate the workspace root by traversing upward from the current working directory until it finds a directory containing a `catalog/` folder with at least one recognized catalog file (agents.json, skills.json, install-roles.json, mcp-references.json, rules.json), and load all catalog JSON files found therein into memory.
2. THE TUI SHALL deserialize catalog JSON using `serde` with strict schema validation, rejecting unknown fields in top-level catalog entry structures (agent entries, skill entries, role entries, MCP reference entries, and rule entries).
3. WHEN catalog loading completes, THE TUI SHALL display the loaded counts (agents, skills, roles, providers, MCP references, rules) in the status bar.
4. THE TUI SHALL complete catalog loading and display the initial view within 2 seconds on a machine with at least 4 CPU cores and 8 GB RAM for a catalog containing up to 500 agents and proportional skill/role entries.
5. IF any catalog file fails to load or contains invalid JSON, THEN THE TUI SHALL continue with partial data and display a warning indicating which catalogs are unavailable.
6. IF the workspace root cannot be located after traversing to the filesystem root, THEN THE TUI SHALL display an error message indicating that no workspace was found and prompt the operator to specify the workspace path or run the TUI from within a valid workspace directory.

### Requirement 16: Keyboard Navigation and UI Layout

**User Story:** As a cloud operator, I want intuitive keyboard navigation, so that I can efficiently browse and operate the TUI without a mouse.

#### Acceptance Criteria

1. THE TUI SHALL support navigation with: arrow keys (list movement), Enter (select/confirm), Escape (back/cancel), Tab (switch panels), and `/` (activate search).
2. THE TUI SHALL display a help bar visible at all times at the bottom of the terminal, showing the keybindings applicable to the currently focused panel.
3. THE TUI SHALL organize the interface into: a navigation sidebar (catalog sections), a main content area (lists/details), and a status bar (counts, filters, session info).
4. WHEN the operator presses `q` or Ctrl+C, THE TUI SHALL exit, restoring the terminal cursor visibility, input echo, and original screen buffer within 1 second.
5. WHILE no text input field is focused, THE TUI SHALL support Vim-style navigation keys (j/k for up/down, g/G for top/bottom of list) as alternatives to arrow keys.
6. THE TUI SHALL indicate the currently focused panel and selected item with a visually distinct highlight (e.g., color change or border marker) that differs from unfocused elements.
7. WHEN the operator navigates past the last item in a list, THE TUI SHALL stop at the boundary without wrapping, keeping the last item selected.

### Requirement 17: CI/CD Integration

**User Story:** As a DevOps engineer, I want the TUI project to integrate with CI/CD pipelines, so that code quality is enforced automatically.

#### Acceptance Criteria

1. THE TUI project SHALL include a CI workflow that runs `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt -- --check` on every pull request, and THE CI workflow SHALL report a failing status that blocks merge if any check fails.
2. THE TUI project SHALL compile with zero warnings under `#![deny(warnings)]` in release mode.
3. THE TUI project SHALL commit `Cargo.lock` to the repository for reproducible builds.
4. WHEN a release tag is pushed, THE TUI project CI SHALL build release binaries for all supported targets (linux x86_64, linux aarch64, macOS x86_64, macOS aarch64).
5. THE TUI project SHALL generate an SBOM dependency listing in SPDX or CycloneDX format using `cargo-sbom` or equivalent tooling in CI, produced alongside each release binary build.
6. IF any CI workflow step fails on a pull request, THEN THE TUI project SHALL prevent the pull request from being merged until all checks pass.

### Requirement 18: Deterministic Behavior

**User Story:** As a platform engineer, I want the TUI to produce deterministic output for the same inputs, so that I can trust and reproduce its behavior.

#### Acceptance Criteria

1. THE TUI SHALL produce identical rendered content (text, ordering, and structure) for the same catalog data, the same sequence of user inputs, and the same terminal dimensions.
2. THE TUI SHALL NOT write any files to disk during normal operation (no config files, no caches, no history files, no temporary files).
3. WHEN displaying lists, THE TUI SHALL use a stable, case-insensitive lexicographic sort order by ID unless the operator explicitly changes the sort.
4. THE TUI SHALL NOT make network requests under any circumstance.
5. WHEN displaying subprocess output from validation scripts, THE TUI SHALL pass through the subprocess stdout and stderr after applying required secret redaction and terminal escape sanitization, without injecting timestamps or non-deterministic metadata.
6. THE TUI SHALL NOT read environment variables to alter display content or sort order; only terminal dimensions (rows and columns) and the workspace root path SHALL influence rendering layout.

### Requirement 19: Alpha Release Scope Boundaries

**User Story:** As a product owner, I want clear boundaries on what the alpha release includes and excludes, so that expectations are managed.

#### Acceptance Criteria

1. THE TUI SHALL NOT provide agent or skill editing capabilities (no create, update, or delete operations on agent or skill definitions).
2. THE TUI SHALL NOT directly mutate catalog JSON files (all catalog access SHALL be read-only file parsing).
3. THE TUI SHALL NOT perform network operations (no outbound connections including registry fetches, update checks, telemetry, or DNS lookups).
4. THE TUI SHALL NOT implement a web GUI, desktop application, or graphical interface.
5. THE TUI SHALL NOT implement plugin marketplace installation flows (no `npm install`, `codex plugin install`, or equivalent package installation commands).
6. THE TUI SHALL provide read-only catalog browsing, validation gate execution via subprocess, and export command building with subprocess execution as the complete alpha feature set.
7. THE TUI SHALL NOT persist user preferences, configuration files, or session history between invocations.

### Requirement 20: Subprocess Execution Model

**User Story:** As a platform engineer, I want the TUI to execute validation and export commands via subprocess invocation of existing scripts, so that no business logic is duplicated.

#### Acceptance Criteria

1. THE TUI SHALL invoke validation gates by executing `npm run <script-name>` as a subprocess with the working directory set to the workspace root.
2. THE TUI SHALL invoke export operations by executing `node scripts/export-marketplace-agents.mjs` with the operator-selected platform, role, provider, or `--agents=<comma-separated-ids>` specific-agent selection as subprocess arguments.
3. THE TUI SHALL capture subprocess stdout and stderr separately, displaying stdout in the output panel with default text styling and stderr with a distinct foreground color or prefix label differentiating it from stdout.
4. THE TUI SHALL use tokio for async subprocess management, ensuring the UI thread processes input events within 100 milliseconds while a subprocess is running.
5. IF a subprocess is running and the operator requests cancellation, THEN THE TUI SHALL send SIGTERM (Unix) or equivalent termination signal and wait up to 5 seconds before sending SIGKILL.
6. IF a subprocess exits with a non-zero exit code, THEN THE TUI SHALL display the exit code in the output panel and set the operation status to failed without terminating the TUI itself.

### Requirement 21: Asset Integrity Visibility

**User Story:** As a security engineer, I want to view asset integrity status from the TUI, so that I can verify catalog integrity without running separate commands.

#### Acceptance Criteria

1. WHEN the TUI starts, THE TUI SHALL parse `catalog/asset-integrity.json` and display the manifest version, hash algorithm, total tracked file count across all trees and root files, and the aggregate SHA-256 hash in the status area.
2. IF `catalog/asset-integrity.json` is missing or contains invalid JSON, THEN THE TUI SHALL display an error message indicating the integrity manifest is unavailable and disable the integrity view navigation.
3. WHEN the operator navigates to the integrity view, THE TUI SHALL display all tracked assets grouped by tree, showing each asset's file path, SHA-256 hash, and size in bytes, in a scrollable list.
4. WHEN the operator selects an asset in the integrity view, THE TUI SHALL display the full integrity record including the file path, SHA-256 hash, size in bytes, and the parent tree's aggregate SHA-256 hash.
5. WHEN the operator selects a root file entry in the integrity view, THE TUI SHALL display the file path, SHA-256 hash, and size in bytes for that root file.

### Requirement 22: Provider Coverage Sparkline Bars

**User Story:** As a cloud operator, I want to see a visual bar next to each provider showing relative agent count, so that I can quickly assess provider coverage at a glance.

#### Acceptance Criteria

1. WHEN the TUI displays the provider list view, THE TUI SHALL render a horizontal bar (using block characters █░) next to each provider entry showing the agent count relative to the provider with the most agents.
2. THE bar width SHALL be fixed at 20 characters, with the filled portion proportional to (provider_agent_count / max_provider_agent_count).
3. THE provider with the most agents SHALL display a fully filled bar (20 █ characters).

### Requirement 23: Validation Gate Heatmap Coloring

**User Story:** As a platform engineer, I want validation gates color-coded by status, so that I can instantly identify which gates need attention.

#### Acceptance Criteria

1. WHEN the TUI displays the validation gate list, THE TUI SHALL color each gate entry based on its status: NotRun (gray/dim), Running (yellow), Passed (green), Failed (red), TimedOut (magenta).
2. WHEN --no-color mode is active, THE TUI SHALL use text modifiers (dim, bold) instead of colors to differentiate gate statuses.
3. THE color-coding SHALL use the theme module's style system for consistency with the rest of the UI.

### Requirement 24: Agent Dependency Graph in Detail View

**User Story:** As a cloud operator, I want to see which roles contain an agent and which agents bundle a skill, so that I can understand cross-entity relationships.

#### Acceptance Criteria

1. WHEN the TUI displays an agent detail view, THE TUI SHALL include a "Roles" field showing all role IDs that contain this agent (via reverse lookup of install-roles.json).
2. WHEN the TUI displays a skill detail view, THE TUI SHALL include a "Related Agents" field showing all agents whose companion_skills array contains this skill's ID.
3. IF no roles contain the agent, THE TUI SHALL display "N/A" for the Roles field.

### Requirement 25: Live Filter Chips Display

**User Story:** As a cloud operator, I want to see which filters are currently active as removable chips, so that I can understand the current filter state at a glance.

#### Acceptance Criteria

1. WHEN a provider filter, harness filter, or search query is active in the agent list view, THE TUI SHALL render filter chips at the top of the main content area showing the active state (e.g., `[provider:aws] [harness:kiro] [query:"iam"]`).
2. THE TUI SHALL support cycling through provider filter values by pressing 'p' in the agent list view.
3. THE TUI SHALL support cycling through harness filter values by pressing 'h' in the agent list view.
4. WHEN the operator presses Escape with active filters, THE TUI SHALL clear all filters before navigating back.

### Requirement 26: Diff Preview for Exports (Dry-Run Tree)

**User Story:** As a cloud operator, I want to see a tree-structured preview of what a dry-run export would create, so that I can verify the export plan before execution.

#### Acceptance Criteria

1. WHEN the export output view displays dry-run results, THE TUI SHALL parse output lines starting with "export agent:" or "export skill:" and display them as a tree structure showing agents/ and skills/ directories.
2. THE tree display SHALL show total counts of agents and skills that would be exported.
3. THE raw subprocess output SHALL still be available below the tree preview.

### Requirement 27: Keyboard Shortcut Overlay

**User Story:** As a cloud operator, I want to press '?' to see all available keyboard shortcuts, so that I can learn the TUI without reading documentation.

#### Acceptance Criteria

1. WHEN the operator presses '?' outside of search mode, THE TUI SHALL display a full-screen overlay listing all keybindings organized by section (Navigation, Search & Filter, General, Export Builder).
2. WHEN the help overlay is visible, pressing Escape or '?' SHALL dismiss it.
3. THE help overlay SHALL not interfere with other key handlers when dismissed.

### Requirement 28: Tab Completion in Export Builder

**User Story:** As a cloud operator, I want to see valid options when editing export builder fields, so that I can select correct values without memorizing them.

#### Acceptance Criteria

1. WHEN the operator is in the export builder view, THE TUI SHALL display completion suggestions below the focused field showing valid values from the catalog (platforms for platform field, role IDs for selection field, provider names for provider field).
2. THE completion suggestions SHALL highlight the currently selected suggestion.
3. THE suggestions SHALL be derived from the loaded catalog data.

### Requirement 29: Validation Gate Timing Display

**User Story:** As a platform engineer, I want to see how long each validation gate took to run, so that I can identify slow gates and optimize the validation pipeline.

#### Acceptance Criteria

1. WHEN a validation gate finishes execution, THE TUI SHALL display the elapsed duration in seconds (e.g., "(1.2s)") next to the gate's status indicator.
2. THE duration SHALL be stored in the ValidationGate model's last_duration field.
3. IF a gate has not been run (NotRun status), no duration SHALL be displayed.
