# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized files (no secrets, no connection strings, no tokens, no feed credentials, no signing keys — replace with placeholders):
- The .NET CI workflow file(s) that build, restore, and publish (`.github/workflows/*.yml`, `.gitlab-ci.yml`, `azure-pipelines.yml`).
- `global.json`, if present, for SDK pinning.
- `Directory.Packages.props`, if Central Package Management is in use.
- `NuGet.config` for the configured package sources.
- `packages.lock.json`, if a lock file exists.
- One or more `.csproj` files for the project's `PackageReference` entries.
- Any publish profile (`.pubxml`) used by a release job.

If a file is not provided, state the affected findings as `assumption (config absent)` and ask for it.

### Step 2 — SDK and toolchain pinning audit

Confirm the toolchain is reproducible.

- No `global.json`, or a `global.json` with no `sdk.version` → HIGH: the build floats to whatever SDK the runner image ships, so the toolchain is non-reproducible.
- `global.json` with `rollForward` set permissively (`latestMajor`, `latestFeature`) with no documented reason → MEDIUM: the pin is partly defeated.
- Recommended: pin `sdk.version` and use a conservative `rollForward` (`patch` or `disable`).

### Step 3 — Package pinning and lock-file audit

Review every `PackageReference` and the lock posture.

- Floating versions — a wildcard `*`, a floating range `1.2.*`, or a `[1.0,2.0)` range — on any `PackageReference` → HIGH: the build silently absorbs upstream changes and is non-reproducible.
- Neither `packages.lock.json` nor Central Package Management (`Directory.Packages.props`) present → HIGH: transitive dependencies are unpinned.
- A `packages.lock.json` exists but `dotnet restore` is not run with `--locked-mode` (or `RestoreLockedMode=true`) in CI → HIGH: the lock file is decorative and drift is not enforced.
- Versions duplicated and divergent across projects with no Central Package Management → MEDIUM: version drift and accidental upgrades.
- Recommended: pin exact versions, commit `packages.lock.json`, restore with `--locked-mode`, and adopt Central Package Management for multi-project repos.

### Step 4 — Feed-trust audit

Review `NuGet.config` package sources.

- A `packageSource` with an `http://` (plain-HTTP, non-HTTPS) URL → CRITICAL: packages and credentials traverse an unencrypted, tamperable channel.
- An untrusted or unexpected feed (a personal feed, an unknown mirror) without a documented reason → CRITICAL: a tampering and dependency-confusion path.
- No `packageSourceMapping` when multiple feeds are configured → HIGH: a public feed can shadow an internal package (dependency-confusion).
- Recommended: HTTPS-only sources, an explicit trusted-source list, and `packageSourceMapping` that routes each prefix to one feed.

### Step 5 — Vulnerability-scanning audit

- No `dotnet list package --vulnerable` (or an equivalent scanner) step in CI → HIGH: the build can ship packages with known CVEs and nothing flags it.
- A vulnerability scan present but not failing the build on a finding → HIGH: the scan is advisory only.
- Recommended: run `dotnet list package --vulnerable --include-transitive` in CI and fail the build on any reported advisory.

### Step 6 — Gating and secret-exposure audit

- Secrets in scope for a build job triggered by `pull_request_target` that checks out and builds PR-author code → CRITICAL: a fork PR can exfiltrate the secrets. Flag and stop.
- Secrets passed to a build job that runs on fork PRs → CRITICAL.
- `continue-on-error: true`, `|| true`, `set +e`, or a swallowed exit code on the build or test step → CRITICAL: the gate verifies nothing and every green run is unverified.
- A publish profile (`.pubxml`) that commits a password, token, or connection string → HIGH: a credential leak in version control.
- Long-lived registry or feed credentials where OIDC / short-lived tokens would work → MEDIUM.

### Step 7 — Build-reproducibility audit

- No SBOM generated for the release artifact → MEDIUM: consumers cannot verify the dependency set.
- No build provenance or attestation → MEDIUM: the artifact's origin is unverifiable.
- `ContinuousIntegrationBuild` not set and deterministic-build settings absent for a release build → MEDIUM.
- Recommended: emit an SBOM, attach build provenance, and enable deterministic build settings.

### Step 8 — Produce the output

Format findings using the Output contract below.

---

## Evidence checklist

Before finalizing, confirm:
- [ ] SDK pinning findings are tied to the actual `global.json` content (or its absence).
- [ ] Every floating-version finding cites the specific `PackageReference` and version string.
- [ ] Lock-file and locked-mode findings cite both the lock file's presence and the restore invocation.
- [ ] Feed-trust findings cite the actual `NuGet.config` source URLs.
- [ ] Secret-exposure findings cite the trigger (`pull_request_target`, fork PR) and the secret scope.
- [ ] Each finding carries an evidence-basis label.
- [ ] No secret, connection string, token, feed credential, or signing key was requested or echoed.

## Findings rubric

| Severity | Examples |
|----------|----------|
| CRITICAL | Secrets in scope for a `pull_request_target` or fork-PR build job; plain-HTTP or untrusted NuGet feed; `continue-on-error: true` or `|| true` on the build/test step. |
| HIGH | Floating package versions; no lock file and no Central Package Management; missing vulnerability scan; unpinned SDK; restore without `--locked-mode` when a lock file exists; secrets committed in a publish profile; no `packageSourceMapping` across multiple feeds. |
| MEDIUM | Missing SBOM or build provenance; permissive `rollForward`; divergent package versions with no Central Package Management; long-lived credentials where OIDC would work. |
| LOW | Cosmetic configuration inconsistencies with no reproducibility or security impact. |

## Output contract

Return findings in this structure:

```
## Verdict
<pass | pass-with-conditions | block>

## Evidence level
<confirmed (config provided) | inference (config partial) | assumption (config absent) | unknown>

## Findings

### CRITICAL
- [C1] <finding>: <description> — <remediation> — evidence: <confirmed (config provided) | inference (config partial) | assumption (config absent) | unknown>

### HIGH
- [H1] <finding>: <description> — <remediation> — evidence: <label>

### MEDIUM
- [M1] <finding>: <description> — <remediation> — evidence: <label>

### LOW
- [L1] <finding>: <description> — <remediation> — evidence: <label>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request or accept secrets, connection strings, tokens, feed credentials, signing keys, or customer data. Ask for sanitized configuration files with placeholders.
- This is a static review: never trigger pipelines, restore packages, run builds, or contact live systems.
- Secrets in scope for a `pull_request_target` or fork-PR build job running PR-author code is a real exfiltration path — treat it as CRITICAL and tell the user to stop merging through that pipeline until it is fixed.
- Never recommend disabling locked-mode to "fix" restore errors — a restore failure under locked-mode is the lock file doing its job. Never recommend pinning to a known-vulnerable version for stability. Never recommend disabling a failing gate as the fix.
