# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Versioning Policy (pre-1.0)

While this crate is below `1.0.0`, the public surface is the **binary CLI**, not
the library API. Per the pre-1.0 semver convention:

- **Minor** version bumps (`0.X.0`) may include breaking changes to the CLI or
  the internal library API.
- **Patch** version bumps (`0.1.X`) are reserved for backwards-compatible bug
  fixes and non-breaking additions.

The library API is internal and **not** covered by semver guarantees.

## [0.1.0] - 2026-06-19

### Added

- Initial public release on crates.io
- Interactive TUI for browsing the VFA catalog (agents, skills, roles, providers)
- Fuzzy search across all catalog entities
- Validation gate execution with streaming output
- Export command builder with dry-run preview
- Headless reporting mode (JSON/text/summary)
- Cross-platform support (Linux x86_64/aarch64, macOS x86_64/aarch64, musl)
- Structured audit logging
- Terminal escape sanitization for security
