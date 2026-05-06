# Security Notes

This document records dependency-vulnerability triage decisions and
workflow-hardening rationale that aren't obvious from `SECURITY.md` or
the lockfile alone. It is the canonical place to look up "why was this
Dependabot alert dismissed" or "why does workflow X need write scope Y".

## Dependabot triage

### `ip-address` XSS in Address6 HTML-emitting methods (Dependabot #1)

- **Advisory**: `Address6.group()`, `Address6.link()`, and
  `AddressError.parseMessage()` do not HTML-escape attacker-controlled
  content, enabling XSS when output is fed to `innerHTML`.
- **Affected**: `ip-address` `<= 10.1.0`. **Patched**: `10.1.1`.
- **Severity**: Moderate (CVSS 5.3, AV:N/AC:L/UI:P).

**Status: dismissed (vulnerable code is not used).**

**Reasoning:**

1. **Dev-only transitive.** `ip-address@10.1.0` enters the lockfile
   exclusively via the bundled copy inside the `npm` CLI itself
   (`node_modules/npm/node_modules/ip-address`, `inBundle: true`),
   reached through `@semantic-release/npm` -> `npm`. It is never
   shipped to consumers of `@raishin/vanguard-frontier-agentic`; the
   `files` allowlist in `package.json` excludes `node_modules` and
   only ships source assets.
2. **No HTML rendering surface.** The vulnerable methods only matter
   when their output is sunk into `innerHTML` or an equivalent HTML
   context. The release pipeline (`semantic-release`, `npm publish`,
   `gh release upload`) does not render IP-address strings to HTML.
3. **No reachable attacker-controlled input.** The release workflow
   runs only on `push` to `master` (a trusted ref protected by the
   ruleset) and only operates on commit metadata, not on
   user-controlled IP strings.
4. **Bundled-dep override is unsafe.** `npm` ships `ip-address` as a
   bundled dependency inside its own tarball; an `overrides` block in
   our `package.json` cannot cleanly replace a bundled module without
   risking npm CLI behaviour changes.

**Tracking & exit criteria:**

- Watch the upstream `npm` CLI for a release that bundles
  `ip-address >= 10.1.1`.
- When `@semantic-release/npm` publishes a version that pins that
  newer `npm`, run `npm update @semantic-release/npm` and verify
  `node_modules/npm/node_modules/ip-address/package.json` reports
  `>= 10.1.1`.
- Re-open the alert (or let Dependabot re-detect on next scan) and
  confirm closure.

## Workflow token-permission hardening

The OpenSSF Scorecard `Token-Permissions` check requires a top-level
`permissions:` block on every workflow, with write scopes granted only
on the specific job that needs them. Current state:

| Workflow | Top-level | Job-level writes | Notes |
|---|---|---|---|
| `apply-ruleset.yml` | `read-all` | `contents: read` | Uses `RULESET_ADMIN_TOKEN` PAT, not `GITHUB_TOKEN`. |
| `ci.yml` | `contents: read` | none (read-only) | Pure validators. |
| `codeql.yml` | `contents: read` | `security-events: write` on `analyze` | SARIF upload. |
| `docs-quality.yml` | `contents: read` | none | markdownlint + codespell. |
| `install-paths-smoke.yml` | `contents: read` | none | Smoke tests. |
| `release.yml` | `contents: read` | `contents/issues/pull-requests/id-token/attestations: write` on `release` | semantic-release + provenance. |
| `scorecard.yml` | `read-all` | `security-events/id-token: write` + read scopes on `analysis` | OpenSSF Scorecard self-scan. |

If a new workflow is added, it **must** declare `permissions:` at the
top level (default to `contents: read`) before merge. The CodeQL
workflow will re-detect missing permissions and re-open the alert
otherwise.

## Release workflow checkout credentials

`.github/workflows/release.yml` sets `persist-credentials: true` on
`actions/checkout`. This is required because `@semantic-release/git`
pushes the `chore(release): X.Y.Z [skip ci]` commit (CHANGELOG.md and
`package.json` bump) back to `master` and creates the tag. Without
persisted credentials the push silently no-ops and no release is
produced.

Mitigations that keep this safe:

- The `Release` job runs only on `push` to `master` (trusted ref).
- The branch ruleset blocks force-pushes and deletions on `master`,
  so a leaked token cannot rewrite history.
- All third-party actions are pinned to full commit SHAs, so a
  compromised tag cannot exfiltrate the token mid-run.
