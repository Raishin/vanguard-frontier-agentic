# Implementation Plan: crates-io-publish

## Overview

Prepare the `vfa-tui` crate at `tools/vfa-tui/` for first-time publication to crates.io. Tasks are ordered: file creation → Cargo.toml → code changes → README → GitHub Actions workflow → final verification. Each task is independently verifiable via `cargo` commands.

## Tasks

- [ ] 1. Create license and documentation files
  - [ ] 1.1 Create LICENSE-MIT file
    - Create `tools/vfa-tui/LICENSE-MIT` with standard MIT license text
    - Copyright line: `Copyright (c) 2024 Raishin`
    - Verify file is non-empty and contains "MIT License" header
    - _Requirements: 1.2, 1.3_

  - [ ] 1.2 Create LICENSE-APACHE file
    - Create `tools/vfa-tui/LICENSE-APACHE` with the full Apache License 2.0 text
    - Verify file is non-empty and contains "Apache License, Version 2.0"
    - _Requirements: 1.4_

  - [ ] 1.3 Create CHANGELOG.md
    - Create `tools/vfa-tui/CHANGELOG.md` following Keep a Changelog format
    - Include `## [0.1.0]` entry with `### Added` section listing initial public release features
    - Document semver policy for pre-1.0: minor bumps for breaking changes, patch for non-breaking
    - _Requirements: 7.1, 7.2, 7.3, 16.2_

  - [ ] 1.4 Create SECURITY.md
    - Create `tools/vfa-tui/SECURITY.md` with supported versions table, contact method, and response timeline
    - Include `## Supported Versions` section listing `0.1.x` as supported
    - Include `## Reporting a Vulnerability` section with disclosure process
    - _Requirements: 8.1, 8.2, 8.3, 8.4_

  - [ ] 1.5 Create deny.toml
    - Create `tools/vfa-tui/deny.toml` with the license allow-list, advisory checks, and source restrictions per design Component 4
    - Include `[graph].targets` for all 5 cross-compile targets
    - Include `[licenses].allow` with permissive licenses (MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-3.0, Unicode-DFS-2016, Zlib, BSL-1.0, CC0-1.0, OpenSSL)
    - Include `[[licenses.clarify]]` for the `ring` crate
    - Include `[advisories]` with `vulnerability = "deny"`, `yanked = "deny"`
    - Include `[sources]` with `unknown-registry = "deny"`, `unknown-git = "deny"`
    - Verify with `cargo deny check` from `tools/vfa-tui/`
    - _Requirements: 5.2_

- [ ] 2. Modify Cargo.toml metadata and dependencies
  - [ ] 2.1 Rewrite [package] section
    - Remove `publish = false`
    - Set `version = "0.1.0"`
    - Add `rust-version = "1.75"`
    - Set `license = "MIT OR Apache-2.0"`
    - Add `repository`, `homepage`, `readme`, `keywords`, `categories`, `authors` fields per design Component 1
    - Add `exclude` array: `["proptest-regressions/", "tests/", "~/", "target/", ".gitignore"]`
    - _Requirements: 1.1, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7, 2.8, 2.9, 2.10, 3.1, 9.1, 16.1_

  - [ ] 2.2 Optimize dependencies
    - Replace `futures = "0.3"` with `futures-util = "0.3"` in `[dependencies]`
    - Remove `sha2 = "0.10"` from `[dev-dependencies]` (already in `[dependencies]`)
    - Verify build still compiles: `cargo check` from `tools/vfa-tui/`
    - _Requirements: 4.4, 5.1_

- [ ] 3. Checkpoint — Verify Cargo.toml changes
  - Run `cargo check` and `cargo package --list` from `tools/vfa-tui/` to confirm metadata and exclude patterns are correct
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 4. Apply code quality changes
  - [ ] 4.1 Minimize public API in lib.rs
    - Change 11 modules from `pub mod` to `pub(crate) mod`: `app`, `cli`, `federation`, `gates`, `headless`, `logging`, `paths`, `persistence`, `policy`, `subprocess`, `ui`, `workspace`
    - Keep 5 modules as `pub mod`: `catalog`, `error`, `models`, `search`, `security`
    - Keep `#[cfg(test)] pub mod test_fixtures` unchanged
    - Verify with `cargo test` from `tools/vfa-tui/` — if integration tests fail on removed pub modules, restore those modules to `pub`
    - _Requirements: 4.3, 18.1_

  - [ ] 4.2 Remove allow attributes from main.rs
    - Remove `#![allow(dead_code)]` from `src/main.rs`
    - Remove `#![allow(unused_imports)]` from `src/main.rs`
    - Run `cargo clippy -- -D warnings` from `tools/vfa-tui/` and fix any resulting warnings (prefix unused items with `_`, remove dead code, or gate behind `#[cfg(test)]`)
    - _Requirements: 4.1, 4.2_

  - [ ]* 4.3 Write property test for package contents invariant
    - **Property 1: Package Contents Invariant**
    - Test that `cargo package --list` output only contains intentional files (src/, *.md, LICENSE-*, Cargo.toml, Cargo.lock) and excludes all patterns in the exclude field
    - **Validates: Requirements 9.1, 9.2, 9.3, 9.5, 20.1, 20.2, 20.3**

  - [ ]* 4.4 Write property test for public API minimality
    - **Property 3: Public API Minimality**
    - Parse `src/lib.rs` to extract pub vs pub(crate) modules; verify each `pub` module has at least one integration test importing it
    - **Validates: Requirements 4.3, 18.1**

- [ ] 5. Checkpoint — Verify code compiles cleanly
  - Run `cargo clippy -- -D warnings` and `cargo test` from `tools/vfa-tui/`
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Restructure README for crates.io
  - [ ] 6.1 Rewrite README.md for external users
    - Reorder sections per design Component 10: Title → Installation → Limitations → Pre-built Binaries → Overview → Usage → Platforms → Architecture → Development → License
    - Add `cargo install vfa-tui` as primary installation method
    - Add "Limitations" section stating monorepo dependency and lack of standalone operation
    - Add "Pre-built Binaries" section linking to GitHub Releases
    - Add explicit statement: "Distributed as a binary via `cargo install`. The library API is internal and not covered by semver guarantees."
    - State semver pre-1.0 policy
    - Remove any `rtk` command references from user-facing instructions
    - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5, 16.3, 18.2_

- [ ] 7. Create release workflow
  - [ ] 7.1 Create .github/workflows/vfa-tui-release.yml
    - Trigger on tags matching `vfa-tui-v*`
    - **Job 1 — verify**: fmt check, clippy -D warnings, cargo test, cargo doc --no-deps, cargo audit, cargo deny check, cargo package --verify, cargo publish --dry-run, tag↔Cargo.toml version match
    - **Job 2 — cross-compile** (needs: verify): matrix of 5 targets with appropriate runners (ubuntu-latest for linux, macos-13 for x86_64-darwin, macos-14 for aarch64-darwin); install cross-compile toolchains; build `--release`; upload binaries as artifacts
    - **Job 3 — release-assets** (needs: cross-compile): download all artifacts, generate SHA256SUMS.txt, generate SBOM via cargo-sbom (SPDX 2.3), create GitHub Release (draft), attach all binaries + checksums + SBOM
    - **Job 4 — publish** (needs: release-assets): environment `crates-io-publish` with required reviewer approval gate; run `cargo publish` using `CARGO_REGISTRY_TOKEN` secret
    - Use raw cargo/gh commands (not rtk) since this is CI
    - _Requirements: 10.1, 10.2, 10.3, 10.4, 11.1, 11.2, 11.3, 11.4, 12.1, 12.2, 12.3, 13.1, 13.2, 13.3, 13.4, 14.1, 14.2, 14.3, 14.4, 15.1, 15.2, 15.3, 19.2_

  - [ ]* 7.2 Write integration test for tag-version consistency check
    - **Property 5: Tag-Version Consistency**
    - Verify the workflow's version-check step logic: extract version from tag name and compare to Cargo.toml version field
    - **Validates: Requirements 10.1, 16.1**

- [ ] 8. Final verification
  - [ ] 8.1 Run full pre-publish dry-run
    - Execute from `tools/vfa-tui/`:
      - `cargo fmt -- --check`
      - `cargo clippy -- -D warnings`
      - `cargo test`
      - `cargo deny check`
      - `cargo publish --dry-run`
      - `cargo package --list` (verify no excluded files appear)
    - Confirm LICENSE-MIT and LICENSE-APACHE appear in package list
    - Confirm package size is reasonable
    - _Requirements: 1.5, 3.2, 9.2, 9.3, 9.4, 15.1, 20.2_

- [ ] 9. Final checkpoint — All gates pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties from the design
- Use `cargo` commands directly in task descriptions (CI context); use `rtk cargo` when executing locally
- The workflow file (7.1) uses raw commands since `rtk` is not available in CI runners
- Task 4.1 (lib.rs changes) may require iteration if integration tests import from modules being restricted — `cargo test` is the verification gate
- Requirement 19.1 (existing CI preservation) is satisfied by creating a separate workflow file — no changes to `.github/workflows/vfa-tui-ci.yml`

## Task Dependency Graph

```json
{
  "waves": [
    { "id": 0, "tasks": ["1.1", "1.2", "1.3", "1.4", "1.5"] },
    { "id": 1, "tasks": ["2.1"] },
    { "id": 2, "tasks": ["2.2"] },
    { "id": 3, "tasks": ["4.1", "4.2"] },
    { "id": 4, "tasks": ["4.3", "4.4"] },
    { "id": 5, "tasks": ["6.1"] },
    { "id": 6, "tasks": ["7.1"] },
    { "id": 7, "tasks": ["7.2"] },
    { "id": 8, "tasks": ["8.1"] }
  ]
}
```
