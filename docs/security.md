---
layout: default
title: "Security"
permalink: /docs/security/
---

# 🛡️ Security

This page documents the security posture of `@raishin/vanguard-frontier-agentic` with adversarial framing. Every claim cites a specific file or command.

---

## 🔐 Supply Chain Security

### OIDC Trusted Publishing (No Stored Secrets)

- **Evidence:** `.github/workflows/release.yml` declares `id-token: write` and uses `environment: npm-deployment-master`
- **What it means:** No `NPM_TOKEN` secret exists in repository settings. Publishing authenticates via GitHub OIDC token exchange with npmjs.com.
- **Verification:** Check repository Settings > Secrets - no npm token should exist.

### npm Provenance + Sigstore

- **Evidence:** `npm publish --provenance` in `.github/workflows/release.yml`
- **What it means:** Every published version has a Sigstore-signed provenance statement recorded in the Rekor transparency log. The statement links the package to the exact commit, workflow, and build environment.
- **Verification:** `npm audit signatures` validates the signature chain.

### SLSA Build Level 3

- **Evidence:** `actions/attest-build-provenance` in `.github/workflows/release.yml`
- **What it means:** The build process is fully defined in code (this workflow), runs in an ephemeral environment (GitHub-hosted runner), and produces a non-forgeable attestation bundle.
- **Verification:** `gh attestation verify <tarball> --owner Raishin`

### SPDX SBOM

- **Evidence:** `anchore/sbom-action` referenced in release workflow; `npm run release:sbom` in `package.json`
- **What it means:** Every release includes a Software Bill of Materials listing all dependencies.
- **Verification:** Check GitHub Release assets for `sbom.spdx.json`.

---

## 🧪 Code Scanning

### CodeQL

- **Evidence:** `.github/workflows/codeql.yml`
- **What it means:** Static analysis runs on every PR and push, scanning for common vulnerability patterns (injection, XSS, prototype pollution).
- **Verification:** Check the Security tab > Code scanning alerts.

### Dependabot

- **Evidence:** `.github/dependabot.yml`
- **What it means:** Automated PRs for dependency updates (both GitHub Actions and npm). Grouped by type, reviewed by `@Raishin`.
- **Configuration:**
  - GitHub Actions: weekly, grouped
  - npm: weekly, grouped (runtime vs dev)
  - PR limit: 5 per ecosystem

---

## ⚖️ OpenSSF Compliance

### Scorecard

- **Evidence:** `.github/workflows/scorecard.yml`
- **What it means:** The OpenSSF Scorecard runs against this repository and publishes results. Tracks: branch protection, CI tests, code review, dependency update tools, pinned dependencies, SAST, security policy, signed releases, token permissions, vulnerabilities.

### Best Practices Badge

- **Evidence:** README.md badge
- **What it means:** The project passes the OpenSSF Best Practices criteria checklist.

---

## 🔒 Access Control

### Branch Protection

- **Evidence:** `docs/branch-protection.md` documents the required settings
- **What it means:** Direct pushes to master are blocked. PRs require review and CI pass.

### CODEOWNERS

- **Evidence:** `.github/CODEOWNERS`
- **What it means:** All PRs automatically request review from `@Raishin`. Critical paths (`.github/`, `scripts/`, `schemas/`, `catalog/`, `tests/`) have explicit ownership.

---

## 🧱 Runtime Safety

### No Lifecycle Scripts

- **Evidence:** `npm run validate:no-lifecycle-scripts` runs `python3 tests/validate-no-lifecycle-scripts.py`
- **What it means:** The package has no `preinstall`, `install`, or `postinstall` scripts. Installing this package executes zero code. This prevents supply chain attacks via malicious lifecycle hooks.
- **Verification:** `npm run validate:no-lifecycle-scripts`

### Asset Integrity

- **Evidence:** `catalog/asset-integrity.json` contains SHA-256 hashes; validated by `npm run validate:asset-integrity`
- **What it means:** Critical files are hashed at generation time. Any modification (accidental or malicious) fails the integrity check in CI.
- **Verification:** `npm run validate:asset-integrity`

### MCP Trust Matrix

- **Evidence:** `npm run validate:mcp-trust-matrix` runs `python3 tests/validate-mcp-trust-matrix.py`
- **What it means:** MCP (Model Context Protocol) references are validated against a trust matrix. Only approved external tool integrations are permitted.
- **Verification:** `npm run validate:mcp-trust-matrix`

---

## ⚠️ What an Attacker Would Try

### Attack: Compromise npm publish credentials

**Mitigation:** There are no stored credentials. OIDC tokens are short-lived (minutes), scoped to the specific workflow run, and cannot be exfiltrated for reuse.

### Attack: Tamper with published package post-build

**Mitigation:** npm provenance + SLSA L3 attestation. Consumers can verify the package was built by this repository's CI at a specific commit. Any tampering breaks the signature chain.

### Attack: Inject malicious lifecycle script

**Mitigation:** `validate:no-lifecycle-scripts` gate in CI. Any PR adding a lifecycle script fails validation and cannot merge.

### Attack: Poison a dependency

**Mitigation:** Dependabot monitoring + CodeQL scanning. lockfile integrity is checked. npm provenance on upstream packages can be verified.

### Attack: Modify catalog assets without detection

**Mitigation:** `catalog/asset-integrity.json` contains SHA-256 hashes of critical files. The integrity check runs in CI on every push.

### Attack: Submit malicious skill/agent content

**Mitigation:** Schema validation (`validate:skill-schema`, `validate:agent-schema`) enforces structural contracts. CODEOWNERS requires review from `@Raishin`. The refusal-by-default routing model means unrecognized content never reaches execution.

### Attack: Fork-and-publish under the same name

**Mitigation:** npm provenance links the package to this specific repository. A fork publishing under the same name would have a different provenance chain, detectable by consumers.

---

## 🧾 Responsible Disclosure

Security vulnerabilities should be reported privately via GitHub Security Advisories:

> [https://github.com/Raishin/vanguard-frontier-agentic/security/advisories/new](https://github.com/Raishin/vanguard-frontier-agentic/security/advisories/new)

Full policy: `SECURITY.md` in repository root.

Response SLA (from `SECURITY.md`):
- Acknowledgment within 48 hours
- Assessment within 7 days
- Fix timeline depends on severity (high/critical patched in current + previous minor)

---

## ✅ How to Verify This Works

```bash
# Confirm no lifecycle scripts
npm run validate:no-lifecycle-scripts

# Confirm asset integrity
npm run validate:asset-integrity

# Confirm MCP trust matrix
npm run validate:mcp-trust-matrix

# Verify published package signatures
npm audit signatures

# Check CodeQL alerts (GitHub UI)
# Security tab > Code scanning alerts

# Check Scorecard results (GitHub UI)
# Security tab > Scorecard
```

---

## 🏛️ Enterprise Reviewer Notes

- The OIDC trusted publishing configuration eliminates the most common npm supply chain attack vector (stolen tokens)
- All GitHub Actions are pinned by SHA, not by tag (prevents tag-jacking attacks)
- The `permissions` block uses least-privilege at both workflow and job level
- Dependabot is configured for both ecosystems used (Actions + npm)
- The absence of lifecycle scripts is enforced by CI, not by convention
