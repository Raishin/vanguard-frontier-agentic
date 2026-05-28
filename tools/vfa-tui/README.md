# vfa-tui

Enterprise-grade terminal UI for the **Vanguard Frontier Agentic** marketplace catalog.

> **Status: Alpha** - This tool is under active development. APIs and UI may change.

## Overview

`vfa-tui` provides a fast, keyboard-driven terminal interface for browsing the VFA catalog of security agents, skills, MCP server references, and rules. It supports fuzzy search, provider/harness filtering, role-based views, subprocess execution for validation scripts, and full audit logging.

## Prerequisites

- **Rust 1.75+** (edition 2021)
- A checkout of the `vanguard-frontier-agentic` repository (workspace auto-detection)

## Build

```bash
cd tools/vfa-tui
cargo build --release
```

The binary is produced at `target/release/vfa-tui`.

## Usage

Run from the repository root (workspace is auto-detected):

```bash
./tools/vfa-tui/target/release/vfa-tui
```

Or specify the workspace explicitly:

```bash
vfa-tui --workspace /path/to/vanguard-frontier-agentic
```

### CLI Reference

| Flag | Description | Default |
|------|-------------|---------|
| `--workspace <PATH>` | Path to the workspace root | Auto-detected |
| `--log-file <PATH>` | Path to the audit log file | None (no file logging) |
| `--log-level <LEVEL>` | Logging verbosity: trace, debug, info, warn, error | info |
| `--no-color` | Disable colored terminal output | false |
| `--help` | Show help message | - |
| `--version` | Show version | - |

### Examples

```bash
# Run with debug logging to a file
vfa-tui --log-level debug --log-file /tmp/vfa-tui.log

# Run without color (useful for piping or screen readers)
vfa-tui --no-color
```

## Architecture

```
+-------------------+
|   Terminal/UI     |  ratatui + crossterm
|   (event loop)    |
+-------------------+
         |
+-------------------+
|   App State       |  Navigation, selection, input
+-------------------+
         |
+--------+----------+
|        |          |
v        v          v
Search   Catalog    Subprocess
Engine   Store      Executor
(nucleo) (serde)    (tokio)
         |
+-------------------+
|   Security Layer  |  Sanitize, redact, validate
+-------------------+
```

### Module Structure

```
src/
  main.rs         - Entry point, terminal setup/teardown
  app.rs          - Application state machine
  cli.rs          - Command-line argument parsing (clap)
  error.rs        - Error types (thiserror)
  lib.rs          - Public module exports for integration tests
  catalog/
    loader.rs     - JSON file loading with taint checking
    store.rs      - In-memory catalog with query methods
  models/
    agent.rs      - Agent data model
    skill.rs      - Skill data model
    role.rs       - Role catalog model
    rule.rs       - Rule data model
    mcp_ref.rs    - MCP reference model
    integrity.rs  - Asset integrity manifest model
    harness.rs    - Harness and SourceType enums
    provider.rs   - Provider enum (35 variants)
    export.rs     - Export command model
    gate.rs       - Validation gate model
  search/
    fuzzy.rs      - Fuzzy search engine (nucleo-matcher)
  security/
    sanitize.rs   - Input sanitization, control byte detection
    redact.rs     - Environment variable redaction
  subprocess/
    executor.rs   - Async subprocess spawning
    signal.rs     - Graceful process termination
    stream.rs     - Output line streaming
  ui/
    layout.rs     - Terminal layout and panels
    navigation.rs - Keyboard navigation
    theme.rs      - Color theme
    widgets.rs    - Custom TUI widgets
  workspace/
    detect.rs     - Workspace root auto-detection
  logging/
    mod.rs        - Structured audit logging (tracing)
```

## Development

### Run Tests

```bash
cargo test
```

### Lint

```bash
cargo clippy -- -D warnings
```

### Format

```bash
cargo fmt -- --check   # Check only
cargo fmt              # Auto-format
```

### Property Tests

Property-based tests live in `tests/property/` and use the `proptest` crate to verify invariants across randomized inputs.

### Integration Tests

Integration tests live in `tests/integration/` and test full round-trip catalog loading, search, and subprocess execution against fixture data in `tests/fixtures/`.

## WSL Compatibility

When running under Windows Subsystem for Linux (WSL):

- The TUI renders correctly in Windows Terminal and most modern terminal emulators
- Ensure your terminal supports 256 colors or true color for the best experience
- If you experience rendering issues, try setting `TERM=xterm-256color`
- The `--no-color` flag can be used as a fallback for terminals with limited color support

## License

Apache-2.0 (same as the parent project)
