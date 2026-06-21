# Requirements Document

## Introduction

Prepare the `vfa-tui` crate (located at `tools/vfa-tui/` in the vanguard-frontier-agentic monorepo) for first-time publication to crates.io as a binary-only distribution. Publication is irreversible and must be automated via GitHub Actions with manual approval gates, cross-compiled release binaries, SBOM generation, and full supply-chain security controls.

## Glossary

- **Crate**: A Rust package published to the crates.io registry
- **Crates_IO**: The official Rust package registry (crates.io)
- **Release_Workflow**: The GitHub Actions CI pipeline at `.github/workflows/vfa-tui-release.yml` that automates publication
- **CI_Workflow**: The existing GitHub Actions pipeline at `.github/workflows/vfa-tui-ci.yml` for PR checks
- **Cargo_Toml**: The `tools/vfa-tui/Cargo.toml` package manifest file
- **Package_Directory**: The `tools/vfa-tui/` directory containing the crate source
- **MSRV**: Minimum Supported Rust Version — the oldest Rust toolchain guaranteed to compile the crate
- **SBOM**: Software Bill of Materials — a machine-readable inventory of all dependencies (SPDX 2.3 format)
- **Scoped_Token**: A crates.io API token restricted to publishing only the `vfa-tui` crate
- **Tag_Pattern**: The Git tag format `vfa-tui-v*` that triggers the release workflow
- **Environment_Gate**: A GitHub Environment with protection rules requiring manual approval before deployment
- **Cross_Target**: One of the five compilation targets for release binaries
- **Dual_License**: The `MIT OR Apache-2.0` license expression standard in the Rust ecosystem
- **Publish_Flag**: The `publish` field in Cargo.toml that controls whether `cargo publish` is permitted
- **Exclude_Field**: The `exclude` field in Cargo.toml that prevents files from being included in the published package
- **Dry_Run**: A `cargo publish --dry-run` execution that validates the package without uploading

## Requirements

### Requirement 1: License Resolution

**User Story:** As a crate publisher, I want the license to be correctly declared and files included, so that crates.io displays accurate license information and the package is legally distributable.

#### Acceptance Criteria

1. THE Cargo_Toml SHALL declare `license = "MIT OR Apache-2.0"` as the Dual_License expression
2. WHEN the crate is packaged, THE Package_Directory SHALL contain both a `LICENSE-MIT` file and a `LICENSE-APACHE` file
3. THE `LICENSE-MIT` file SHALL contain a valid MIT license text with correct copyright attribution
4. THE `LICENSE-APACHE` file SHALL contain the full Apache License 2.0 text matching the repository root LICENSE
5. WHEN `cargo package --list` is executed, THE output SHALL include both license files in the package contents
6. IF either LICENSE-MIT or LICENSE-APACHE is missing or contains invalid/empty content, THEN THE `cargo package` command SHALL fail the build

### Requirement 2: Cargo.toml Metadata Completion

**User Story:** As a crate consumer, I want complete metadata on crates.io, so that I can evaluate the crate's provenance, compatibility, and documentation before installing.

#### Acceptance Criteria

1. THE Cargo_Toml SHALL set `publish` to an unrestricted value (remove `publish = false`)
2. THE Cargo_Toml SHALL declare `version = "0.1.0"` for the first public release
3. THE Cargo_Toml SHALL declare `rust-version = "1.75"` as the MSRV
4. THE Cargo_Toml SHALL include `repository` pointing to the GitHub repository URL
5. THE Cargo_Toml SHALL include `homepage` pointing to the repository URL or documentation page
6. THE Cargo_Toml SHALL include `readme = "README.md"` referencing the crate-level README
7. THE Cargo_Toml SHALL include `keywords` with at most 5 relevant terms
8. THE Cargo_Toml SHALL include `categories` with valid crates.io category slugs
9. THE Cargo_Toml SHALL include at least one entry in `authors`
10. THE Cargo_Toml SHALL include an `exclude` array that prevents non-essential files from being packaged

### Requirement 3: Publish Flag Removal

**User Story:** As a release engineer, I want the publish blocker removed, so that the automated release workflow can execute `cargo publish` successfully.

#### Acceptance Criteria

1. THE Cargo_Toml SHALL NOT contain `publish = false`
2. WHEN `cargo publish --dry-run` is executed from the Package_Directory, THE command SHALL complete without a "package is marked as not for publication" error

### Requirement 4: Code Quality Gates

**User Story:** As a crate maintainer, I want dead code suppressions removed and the public API minimized, so that the published crate presents a clean, intentional interface.

#### Acceptance Criteria

1. THE `src/main.rs` SHALL NOT contain `#![allow(dead_code)]` or `#![allow(unused_imports)]` attributes
2. WHEN `cargo clippy -- -D warnings` is executed, THE command SHALL produce zero warnings
3. THE `src/lib.rs` SHALL restrict public module exports to only those modules required for integration testing
4. WHEN the `sha2` crate appears in both `[dependencies]` and `[dev-dependencies]`, THE Cargo_Toml SHALL list `sha2` only in `[dependencies]` and SHALL remove the redundant `[dev-dependencies]` entry, since sha2 is required for core integrity verification functionality

### Requirement 5: Dependency Optimization

**User Story:** As a crate consumer, I want minimal transitive dependencies, so that build times are reasonable and the attack surface is small.

#### Acceptance Criteria

1. THE Cargo_Toml SHALL replace the `futures = "0.3"` dependency with `futures-util = "0.3"` where only utility traits are needed
2. WHEN `cargo deny check licenses` is executed, THE command SHALL report no license violations against the Dual_License
3. WHEN `cargo audit` is executed, THE command SHALL report zero known vulnerabilities in the dependency tree

### Requirement 6: README for External Users

**User Story:** As an external user discovering the crate on crates.io, I want installation instructions and known limitations clearly documented, so that I can decide whether to install and how to use the crate.

#### Acceptance Criteria

1. THE README SHALL include a `cargo install vfa-tui` instruction as the primary installation method
2. THE README SHALL document that the crate requires a checkout of the vanguard-frontier-agentic repository to function
3. THE README SHALL NOT reference the `rtk` command as a build or run method in user-facing instructions
4. THE README SHALL include a "Limitations" section stating the monorepo dependency and lack of standalone operation
5. THE README SHALL include a "Pre-built Binaries" section directing users to GitHub Releases for the five Cross_Target binaries

### Requirement 7: CHANGELOG Creation

**User Story:** As a crate user, I want a changelog documenting what changed in each release, so that I can assess upgrade impact.

#### Acceptance Criteria

1. THE Package_Directory SHALL contain a `CHANGELOG.md` file
2. THE CHANGELOG SHALL include an entry for version 0.1.0 describing the initial public release
3. THE CHANGELOG SHALL follow the Keep a Changelog format with sections for Added, Changed, Fixed, and Removed

> Note: Acceptance criterion 1 requires only that the file exists; content format (criteria 2–3) is advisory.

### Requirement 8: Security Disclosure Process

**User Story:** As a security researcher, I want a clear vulnerability disclosure process, so that I can report issues responsibly.

#### Acceptance Criteria

1. THE Package_Directory SHALL contain a `SECURITY.md` file
2. THE SECURITY.md SHALL specify a contact method for reporting vulnerabilities
3. THE SECURITY.md SHALL state the expected response timeline for vulnerability reports
4. THE SECURITY.md SHALL list the versions currently receiving security fixes

### Requirement 9: Package Hygiene

**User Story:** As a crate consumer, I want the published package to contain only source code and documentation, so that no test fixtures, regressions, or stray directories inflate the download.

#### Acceptance Criteria

1. THE Cargo_Toml `exclude` field SHALL list `proptest-regressions/`, `tests/`, `~/`, and any other non-essential directories
2. WHEN `cargo package --list` is executed, THE output SHALL NOT include files from `proptest-regressions/`
3. WHEN `cargo package --list` is executed, THE output SHALL NOT include the `~/` directory
4. WHEN `cargo package` is executed, THE command SHALL complete without errors or warnings about missing files
5. IF any specified exclusion pattern fails to exclude its target, THEN THE package hygiene check SHALL be considered failed

### Requirement 10: Tag-Triggered Release Workflow

**User Story:** As a release engineer, I want publication to trigger automatically on tag push, so that releases are reproducible and not dependent on local machine state.

#### Acceptance Criteria

1. WHEN a Git tag matching the Tag_Pattern `vfa-tui-v*` is pushed, THE Release_Workflow SHALL trigger automatically
2. THE Release_Workflow SHALL execute format check, clippy, test, doc generation, cargo audit, cargo deny, and cargo package verification steps before publishing
3. THE Release_Workflow SHALL NOT publish to Crates_IO without passing all verification steps
4. IF any verification step fails, THEN THE Release_Workflow SHALL fail the pipeline and not proceed to publication

### Requirement 11: Manual Approval Gate

**User Story:** As a crate owner, I want a manual approval step before irreversible publication, so that accidental or premature releases are prevented.

#### Acceptance Criteria

1. THE Release_Workflow SHALL require approval from the Environment_Gate before executing `cargo publish`
2. THE Environment_Gate SHALL be configured as a GitHub Environment with at least one required reviewer
3. WHEN approval is denied or times out, THE Release_Workflow SHALL terminate without publishing
4. WHEN approval is granted, THE Release_Workflow SHALL immediately proceed to execute `cargo publish` without requiring additional manual action

### Requirement 12: Scoped Token Authentication

**User Story:** As a security-conscious maintainer, I want the CI token to have minimal permissions, so that a compromised token cannot affect other crates.

#### Acceptance Criteria

1. THE Release_Workflow SHALL authenticate to Crates_IO using a Scoped_Token stored as a GitHub Actions secret
2. THE Scoped_Token SHALL be restricted to publish operations on the `vfa-tui` crate only
3. THE Release_Workflow SHALL NOT expose the token value in workflow logs
4. THE Release_Workflow SHALL verify that the Scoped_Token supports publish operations before attempting upload

### Requirement 13: Cross-Compiled Release Binaries

**User Story:** As a platform engineer, I want pre-built binaries for common platforms, so that I can install without a Rust toolchain.

#### Acceptance Criteria

1. THE Release_Workflow SHALL produce release binaries for all five Cross_Target platforms: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-unknown-linux-musl`
2. WHEN cross-compilation completes, THE Release_Workflow SHALL generate SHA-256 checksums for each binary
3. THE Release_Workflow SHALL create a GitHub Release with all binaries and their checksums attached
4. IF SHA-256 checksum generation fails for any binary, THEN THE Release_Workflow SHALL fail the entire release pipeline

### Requirement 14: SBOM Generation

**User Story:** As a compliance officer, I want a software bill of materials for each release, so that the organization can track third-party components for vulnerability management.

#### Acceptance Criteria

1. THE Release_Workflow SHALL generate an SBOM in SPDX 2.3 format for the published crate
2. THE SBOM SHALL be attached to the GitHub Release as a downloadable artifact
3. THE SBOM SHALL list all direct and transitive dependencies with their versions and licenses
4. IF SBOM generation fails, THEN THE Release_Workflow SHALL block the release and not proceed to publication

### Requirement 15: Dry-Run Verification

**User Story:** As a release engineer, I want pre-publish verification, so that packaging errors are caught before the irreversible publish step.

#### Acceptance Criteria

1. THE Release_Workflow SHALL execute `cargo package` and `cargo publish --dry-run` before the approval gate
2. WHEN the dry-run fails, THE Release_Workflow SHALL report the error and not proceed to the approval step
3. WHEN the dry-run succeeds, THE Release_Workflow SHALL present the package size and file count in the workflow summary

### Requirement 16: Version Reset and Semver Policy

**User Story:** As a crate user, I want the version to follow semver conventions for pre-1.0 crates, so that I can predict compatibility impact of upgrades.

#### Acceptance Criteria

1. THE Cargo_Toml SHALL declare `version = "0.1.0"` for the first publication
2. THE CHANGELOG SHALL document the semver policy: minor bumps for breaking changes, patch bumps for non-breaking changes during the 0.x series
3. THE README SHALL state that the crate follows semver and is pre-1.0 (API may change between minor versions)

### Requirement 17: MSRV Policy and CI Verification

**User Story:** As a user with an older Rust toolchain, I want to know the minimum supported version, so that I can determine compatibility before attempting to build.

#### Acceptance Criteria

1. THE Cargo_Toml SHALL declare `rust-version = "1.75"`
2. THE CI_Workflow SHALL include a job that builds and tests the crate using Rust 1.75 specifically
3. IF the crate fails to compile on the declared MSRV, THEN THE CI_Workflow MSRV job SHALL fail, but this SHALL be configured as a non-blocking advisory job that does not prevent PR merges

### Requirement 18: Binary-Only Distribution

**User Story:** As a crate maintainer, I want the crate published as a binary distribution, so that consumers use `cargo install` rather than depending on `vfa-tui` as a library.

#### Acceptance Criteria

1. THE `src/lib.rs` SHALL restrict public exports to only modules needed for the crate's own integration tests
2. THE README SHALL explicitly state that (a) the crate is distributed as a binary via `cargo install` and (b) the library API is not covered by semver guarantees — both statements are required
3. THE Cargo_Toml SHALL NOT add `[[lib]]` sections that would encourage library usage by downstream consumers

### Requirement 19: Existing CI Preservation

**User Story:** As a developer, I want the existing PR CI workflow to continue functioning, so that pull request checks are not disrupted by the release workflow addition.

#### Acceptance Criteria

1. THE CI_Workflow at `.github/workflows/vfa-tui-ci.yml` SHALL remain functional and unmodified in its trigger conditions
2. THE Release_Workflow SHALL be a separate file at `.github/workflows/vfa-tui-release.yml`
3. WHEN a pull request is opened, THE CI_Workflow SHALL trigger independently of the Release_Workflow

### Requirement 20: Stray Directory Cleanup

**User Story:** As a crate packager, I want no stray directories in the source tree, so that `cargo package` does not include unintended files.

#### Acceptance Criteria

1. THE Package_Directory SHALL NOT contain a `~/` directory
2. WHEN `cargo package --list` is executed, THE output SHALL contain only intentional source files, documentation, and license files
3. IF `cargo package --list` includes files not in the intentional set (source, docs, licenses, Cargo.toml, README), THEN THE packaging verification step SHALL be considered failed and block release
