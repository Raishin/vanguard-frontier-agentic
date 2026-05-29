---
layout: default
title: "Operations Runbook"
permalink: /docs/operations-runbook/
---

# Operations Runbook

Procedures for common operational tasks. Each section includes a checklist and decision tree.

---

## Release Process

Releases are fully automated via semantic-release. The normal flow:

### Checklist: Normal Release

- [ ] PR merged to `master` with conventional commit messages
- [ ] CI passes (all 17 validation gates)
- [ ] semantic-release analyzes commits and determines version bump
- [ ] Version bumped in `package.json`
- [ ] `CHANGELOG.md` updated
- [ ] GitHub Release created with tag
- [ ] npm publish via OIDC (no stored token)
- [ ] SLSA attestation generated
- [ ] SBOM attached to release

### Decision Tree: Did the Release Succeed?

```
Merge to master
  |
  v
Did CI pass?
  |-- No --> Fix CI, re-push
  |-- Yes
  v
Did semantic-release find releasable commits?
  |-- No --> Expected (chore/docs commits). No release produced.
  |-- Yes
  v
Did npm publish succeed?
  |-- No --> See "Failed Release Recovery"
  |-- Yes --> Done. Verify with `npm audit signatures`.
```

---

## Failed Release Recovery

When semantic-release creates a GitHub Release but npm publish fails:

### Checklist: Republish Existing Version

- [ ] Confirm the GitHub Release exists (check Releases page)
- [ ] Confirm npm does NOT have the version: `npm view @raishin/vanguard-frontier-agentic versions`
- [ ] Go to Actions > Release workflow
- [ ] Click "Run workflow"
- [ ] Set `republish: true`
- [ ] Set `dry_run: false`
- [ ] Monitor the run

### Common Failure Causes

| Symptom | Cause | Fix |
|---------|-------|-----|
| 403 from npm | OIDC trusted publisher mismatch | Verify npmjs.com settings: owner (lowercase), repo, workflow, environment |
| Token exchange fails | `id-token: write` missing | Check workflow permissions block |
| "No version bump detected" | semantic-release did not run | Use `republish: true` to force publish |
| npm timeout | Registry issue | Wait and retry via workflow_dispatch |

---

## Asset Integrity Regeneration

When `validate:asset-integrity` fails because files were legitimately modified:

### Checklist

- [ ] Confirm the file change is intentional (review diff)
- [ ] Regenerate the integrity manifest:
  ```bash
  python3 tests/validate-asset-integrity.py --write
  ```
- [ ] Verify the regenerated manifest:
  ```bash
  npm run validate:asset-integrity
  ```
- [ ] Commit the updated `catalog/asset-integrity.json`

### Automated Fix

The workflow `.github/workflows/fix-asset-integrity.yml` can regenerate and commit the manifest automatically.

---

## Catalog Refresh

After adding, removing, or modifying skills or agents:

### Checklist

- [ ] Regenerate all manifests:
  ```bash
  npm run manifest:write:all
  ```
- [ ] Validate the catalog:
  ```bash
  npm run validate:catalog
  ```
- [ ] Check asset integrity (will fail if not regenerated):
  ```bash
  npm run validate:asset-integrity
  ```
- [ ] If integrity fails, regenerate:
  ```bash
  npm run asset-integrity:write
  ```
- [ ] Confirm README counts are current:
  ```bash
  npm run validate:readme-counts
  ```
- [ ] If counts are stale:
  ```bash
  npm run readme-counts:write
  ```
- [ ] Run full validation:
  ```bash
  npm run validate
  ```

---

## Adding a New Provider

When adding a new cloud provider to the ecosystem:

### Checklist

- [ ] Create provider directory: `agents/<provider>/`
- [ ] Create skills directory: `skills/<provider>/`
- [ ] Create at least one agent with `metadata.json` conforming to `schemas/agent.frontmatter.schema.json`
- [ ] Create at least one skill with frontmatter conforming to `schemas/skill.frontmatter.schema.json`
- [ ] Add routing scenarios for the new provider:
  ```bash
  # Edit routing fixtures, then regenerate
  npm run maestro-routing:write
  ```
- [ ] Regenerate catalog:
  ```bash
  npm run manifest:write:all
  ```
- [ ] Update CODEOWNERS if the provider has a dedicated maintainer:
  ```
  /agents/<provider>/ @maintainer
  /skills/<provider>/ @maintainer
  ```
- [ ] Run full validation:
  ```bash
  npm run validate
  ```
- [ ] Update README counts:
  ```bash
  npm run readme-counts:write
  ```

---

## Adding a New Harness Adapter

When adding support for a new AI coding harness:

### Checklist

- [ ] Create plugin directory (e.g., `.new-harness-plugin/`)
- [ ] Define manifest format for the new harness
- [ ] Create generation script in `scripts/generate-<harness>-plugin.mjs`
- [ ] Add npm script: `"<harness>-plugin:write": "node scripts/generate-<harness>-plugin.mjs"`
- [ ] Add validation script in `tests/validate-<harness>-marketplace.py`
- [ ] Add npm script: `"validate:<harness>-marketplace": "python3 tests/validate-<harness>-marketplace.py"`
- [ ] Append to `validate` chain in `package.json`
- [ ] Update `manifest:write:all` to include the new generator
- [ ] Add to multi-harness marketplace validation
- [ ] Update agent `metadata.json` schema to include new harness
- [ ] Run full validation:
  ```bash
  npm run validate
  ```

---

## Routine Maintenance

### Weekly

- [ ] Review Dependabot PRs (automatically opened)
- [ ] Check CodeQL alerts (Security tab)
- [ ] Review OpenSSF Scorecard results

### After Each Release

- [ ] Verify package is on npm: `npm view @raishin/vanguard-frontier-agentic version`
- [ ] Verify provenance: `npm audit signatures`
- [ ] Check GitHub Release has SBOM attached

### Quarterly

- [ ] Review and update `SECURITY.md` supported versions table
- [ ] Audit trusted publisher configuration on npmjs.com
- [ ] Review CODEOWNERS for accuracy

---

## How to Verify This Works

```bash
# Simulate a full release check
npm run validate && npm run test:fuzz

# Verify catalog is in sync
npm run validate:catalog

# Confirm asset integrity
npm run validate:asset-integrity

# Check all manifests are current
npm run manifest:check
npm run validate:plugin-manifest
npm run validate:kiro-powers
```
