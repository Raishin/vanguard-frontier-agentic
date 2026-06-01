# vfa-tui

Enterprise-grade terminal UI for the **Vanguard Frontier Agentic** marketplace catalog.

> **Status: v0.2.0 Alpha** — Functional, secure, and useful. APIs and UI may change.

## Overview

`vfa-tui` provides a fast, keyboard-driven terminal interface for browsing the VFA catalog of 300+ security agents across 30+ providers. It supports fuzzy search, provider/harness filtering, role-based views, validation gate execution with real-time output, export command building with dry-run preview, and full structured audit logging.

The TUI is a read-first interface — it parses catalog JSON files directly for browsing and wraps existing Node.js/Python validation and export scripts via subprocess execution. Write operations (exports) require explicit operator confirmation with dry-run as the default.

Key capabilities:

- Browse agents, skills, roles, providers, MCP references, and rules interactively
- Fuzzy search across all catalog entities (powered by nucleo-matcher)
- Run any of the 17+ validation gates with streaming output
- Build and preview export commands before execution
- View asset integrity manifests with SHA-256 hashes
- Full audit trail via structured tracing logs

## Supported Platforms

| Target | Architecture | Notes |
|--------|-------------|-------|
| `x86_64-unknown-linux-gnu` | Linux x86_64 | Primary development target |
| `aarch64-unknown-linux-gnu` | Linux aarch64 | ARM64 Linux (Graviton, RPi) |
| `x86_64-apple-darwin` | macOS x86_64 | Intel Macs |
| `aarch64-apple-darwin` | macOS aarch64 | Apple Silicon (M1/M2/M3) |
| `x86_64-unknown-linux-musl` | Linux musl | Static binary for WSL and containers |

## Prerequisites

- **Rust 1.75+** (edition 2021)
- A checkout of the `vanguard-frontier-agentic` repository (workspace auto-detection)
- **Node.js 18+** (for validation gate and export command execution)

## Build

```bash
cd tools/vfa-tui
rtk build --release
```

The binary is produced at `target/release/vfa-tui`.

For a statically-linked binary (useful for WSL or container deployment):

```bash
rtk build --release --target x86_64-unknown-linux-musl
```

## Usage

Run from the repository root (workspace is auto-detected):

```bash
./tools/vfa-tui/target/release/vfa-tui
```

Or specify the workspace explicitly:

```bash
vfa-tui --workspace /path/to/vanguard-frontier-agentic
```

### CLI Flags Reference

| Flag | Description | Default |
|------|-------------|---------|
| `--workspace <PATH>` | Path to the workspace root | Auto-detected from CWD |
| `--log-file <PATH>` | Path to the structured audit log file | None (stderr only) |
| `--log-level <LEVEL>` | Logging verbosity: `trace`, `debug`, `info`, `warn`, `error` | `info` |
| `--no-color` | Disable ANSI color codes in all terminal output | `false` |
| `--help` | Show help message and exit | — |
| `--version` | Show version (from Cargo.toml) and exit | — |

### Examples

```bash
# Basic launch — auto-detects workspace from current directory
vfa-tui

# Run with debug logging to a file
vfa-tui --log-level debug --log-file /tmp/vfa-tui.log

# Run without color (useful for piping or screen readers)
vfa-tui --no-color

# Specify workspace explicitly (e.g., from a different directory)
vfa-tui --workspace ~/projects/vanguard-frontier-agentic

# Combine options
vfa-tui --workspace /opt/vfa --log-level trace --log-file ./audit.log --no-color
```

### Keyboard Navigation

| Key | Action |
|-----|--------|
| `↑`/`↓` or `j`/`k` | Move up/down in lists |
| `g`/`G` | Jump to top/bottom of list |
| `Enter` | Select item / confirm action |
| `Escape` | Go back / cancel |
| `Tab` | Switch between panels |
| `/` | Activate search |
| `p` | Cycle provider filter (agent list) |
| `h` | Cycle harness filter (agent list) |
| `?` | Toggle keyboard shortcut overlay |
| `q` or `Ctrl+C` | Quit |

## Architecture

```
┌─────────────────────────────────────────────────────┐
│              Presentation Layer                       │
│  Terminal Manager (crossterm) ← UI Renderer (ratatui)│
│  Navigation State Machine ← Keybinding Dispatcher   │
├─────────────────────────────────────────────────────┤
│              Application Layer                        │
│  Browse Controller │ Validation Controller           │
│  Export Controller │ Search Engine (nucleo-matcher)  │
├─────────────────────────────────────────────────────┤
│              Domain Layer                             │
│  Catalog Store (in-memory, read-only)                │
│  Data Models (serde structs, deny_unknown_fields)    │
│  Security Module (sanitize + validate + redact)      │
├─────────────────────────────────────────────────────┤
│              Infrastructure Layer                     │
│  Subprocess Manager (tokio::process)                 │
│  Workspace Detector │ Audit Logger (tracing)         │
│  Terminal Manager (setup/restore/panic hook)          │
└─────────────────────────────────────────────────────┘
```

### Design Principles

1. **No business logic duplication** — wraps existing scripts via subprocess; never re-implements validation or export logic.
2. **Read-first, confirm-before-write** — catalog access is read-only. Exports require explicit confirmation with dry-run default.
3. **Security by construction** — no shell interpolation, no network access, no credential exposure, terminal escape sanitization on all rendered content.
4. **Deterministic rendering** — same inputs produce same outputs. No caches, no config files, no history persistence.
5. **Graceful degradation** — partial catalog loading on file errors; never panics on recoverable errors.

### Module Structure

```
src/
├── main.rs              Entry point, CLI parsing, terminal setup/teardown
├── app.rs               Application state machine and event loop
├── cli.rs               Command-line argument parsing (clap derive)
├── error.rs             Error types (thiserror)
├── lib.rs               Public module exports for integration tests
├── catalog/
│   ├── loader.rs        JSON file loading with taint checking
│   └── store.rs         In-memory catalog with query methods
├── models/
│   ├── agent.rs         Agent data model
│   ├── skill.rs         Skill data model
│   ├── role.rs          Role catalog model
│   ├── rule.rs          Rule data model
│   ├── mcp_ref.rs       MCP reference model
│   ├── integrity.rs     Asset integrity manifest model
│   ├── harness.rs       Harness and SourceType enums
│   ├── provider.rs      Provider enum (35 variants)
│   ├── export.rs        Export command model
│   └── gate.rs          Validation gate model
├── search/
│   └── fuzzy.rs         Fuzzy search engine (nucleo-matcher)
├── security/
│   ├── sanitize.rs      Terminal escape sanitization, control byte detection
│   ├── redact.rs        Secret pattern redaction
│   └── validate.rs      Input/path validation, shell metachar rejection
├── subprocess/
│   ├── executor.rs      Async subprocess spawning (tokio)
│   ├── signal.rs        Graceful SIGTERM → SIGKILL termination
│   └── stream.rs        stdout/stderr line streaming
├── ui/
│   ├── layout.rs        Terminal layout computation
│   ├── navigation.rs    Keyboard navigation and view routing
│   ├── theme.rs         Color theme (256-color with 8-color fallback)
│   └── widgets.rs       Custom TUI widgets (list, detail, status, help, output, search)
├── workspace/
│   └── detect.rs        Workspace root auto-detection
└── logging/
    └── mod.rs           Structured audit logging (tracing + JSON)
```

### Technology Stack

| Concern | Crate | Purpose |
|---------|-------|---------|
| Terminal rendering | `ratatui` 0.30 | Immediate-mode TUI framework |
| Terminal backend | `crossterm` 0.28 | Cross-platform terminal abstraction |
| CLI parsing | `clap` 4.x (derive) | Type-safe argument handling |
| JSON deserialization | `serde` + `serde_json` | Strict schema validation |
| Async runtime | `tokio` (rt-multi-thread) | Subprocess management |
| Structured logging | `tracing` + `tracing-subscriber` | Audit events |
| Error handling | `thiserror` + `anyhow` | Domain + application errors |
| Fuzzy matching | `nucleo-matcher` 0.3 | High-performance search |
| UUID generation | `uuid` (v4) | Session ID |
| Property testing | `proptest` (dev) | Correctness verification |

## Development Guide

### Running Tests

```bash
cd tools/vfa-tui
rtk test
```

### Linting

```bash
rtk clippy -- -D warnings
```

### Formatting

```bash
rtk fmt -- --check   # Check only
rtk fmt              # Auto-format
```

### Full CI Check (local)

```bash
cd tools/vfa-tui
rtk fmt -- --check
rtk clippy -- -D warnings
rtk test
rtk build --release
```

### Property-Based Tests

Property tests live in `tests/property/` and use the `proptest` crate to verify 17 correctness invariants across randomized inputs. These cover:

- Catalog loading (invalid JSON, tainted entries, strict deserialization)
- Search (fuzzy matching, combined filters, reverse-lookup)
- Security (sanitization, redaction, path validation, argument validation)
- Export command construction
- Workspace detection
- Deterministic rendering
- Stable sort ordering

Run property tests with more cases for thorough verification:

```bash
PROPTEST_CASES=1000 rtk test
```

### Integration Tests

Integration tests live in `tests/integration/` and test full round-trip scenarios against fixture data in `tests/fixtures/`:

- Catalog loading from fixture JSON files
- Partial loading when files are missing
- Subprocess execution with various exit codes
- Fuzzy search with known inputs
- Timeout and signal handling

### Adding New Features

1. Define data models in `src/models/` with `#[serde(deny_unknown_fields)]`
2. Add domain logic in the appropriate module (catalog, search, security)
3. Wire into the app state machine in `src/app.rs`
4. Add UI rendering in `src/ui/widgets.rs`
5. Write property tests for invariants and integration tests for round-trips
6. Run the full CI check locally before pushing

## WSL Compatibility

The TUI supports Windows Subsystem for Linux (WSL) as a first-class target via the `x86_64-unknown-linux-musl` build.

### WSL Detection

At runtime, the TUI detects WSL environments via:

1. Checking for the presence of `/proc/sys/fs/binfmt_misc/WSLInterop`
2. Checking for the `WSL_DISTRO_NAME` environment variable

When WSL is detected, the TUI falls back to a safe terminal capability set that excludes features unsupported by the WSL pseudo-terminal layer.

### WSL Notes

- **Terminal emulator**: Windows Terminal provides the best experience (true color, proper resize events). Legacy `conhost.exe` may have rendering issues.
- **Color support**: Ensure your terminal supports 256 colors. If rendering looks wrong, try `export TERM=xterm-256color` before launching.
- **Fallback**: Use `--no-color` for terminals with limited color support.
- **Static binary**: Build with `--target x86_64-unknown-linux-musl` for a fully static binary that works across WSL distributions without shared library dependencies.
- **Performance**: Catalog loading and search are native Rust — no performance difference from bare Linux. Subprocess calls to Node.js scripts may be slightly slower due to WSL filesystem bridging.

### Troubleshooting WSL

| Symptom | Fix |
|---------|-----|
| Garbled output | Set `TERM=xterm-256color` or use `--no-color` |
| No colors | Upgrade to Windows Terminal; legacy console has limited support |
| Resize flicker | Use Windows Terminal (handles resize events correctly) |
| Slow validation gates | Run from the Linux filesystem (`/home/...`) not Windows mounts (`/mnt/c/...`) |

## Security

The TUI enforces several security invariants:

- **No shell interpolation** — all subprocesses are spawned with arguments as arrays, never concatenated strings
- **No network access** — the binary makes zero network requests under any circumstance
- **No credential exposure** — secret environment variables are stripped before subprocess spawning and redacted from all output
- **Terminal escape sanitization** — all catalog data and subprocess output is sanitized against terminal injection attacks
- **Path validation** — all user-provided paths are canonicalized and checked for directory traversal
- **Input validation** — shell metacharacters are rejected in all subprocess arguments

## CI/CD

The project uses GitHub Actions (`.github/workflows/vfa-tui-ci.yml`):

- **On PR**: format check, clippy, tests, release build
- **On tag** (`vfa-tui-v*`): cross-compile release binaries for all 5 targets + SBOM generation (SPDX 2.3)

## License

MIT (see Cargo.toml)
