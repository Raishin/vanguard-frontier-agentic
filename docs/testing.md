---
layout: default
title: "Testing"
permalink: /docs/testing/
---

# 🧪 Testing

The repository enforces quality through automated validation gates, property-based fuzz testing, and scenario-driven routing validation.

---

## Running All Validation Gates

```bash
npm run validate
```

This runs all gates sequentially. If any gate fails, the command exits non-zero.

---

## The Validation Gates

The `npm run validate` command chains these gates in order:

### 1. validate:catalog

```bash
python3 tests/validate-catalog.py
```

Confirms `catalog/agents.json` and `catalog/skills.json` match the actual filesystem contents. Catches orphaned entries, missing agents, or stale catalog data.

### 2. validate:aws

```bash
python3 tests/validate-aws-skill-quality.py && python3 tests/validate-aws-progressive-disclosure.py
```

AWS-specific skill quality rules: trigger keyword coverage, progressive disclosure patterns (skills reveal complexity gradually).

### 3. manifest:check

```bash
python3 tests/validate-skill-manifest.py
```

Verifies the skill manifest is up-to-date. Fails if `manifest:write` would produce different output.

### 4. validate:allowed-tools

```bash
python3 tests/validate-skill-allowed-tools.py
```

Skills can declare `allowed_tools` in frontmatter. This gate ensures no skill references a tool that is not in the approved list.

### 5. validate:skill-schema

```bash
python3 tests/validate-skill-frontmatter-schema.py
```

Validates every skill's YAML frontmatter against `schemas/skill.frontmatter.schema.json`. Catches missing required fields, wrong types.

### 6. validate:agent-schema

```bash
python3 tests/validate-agent-frontmatter-schema.py
```

Validates every agent's `metadata.json` against `schemas/agent.frontmatter.schema.json`.

### 7. validate:links

```bash
python3 tests/validate-links.py --offline
```

Checks all internal Markdown links resolve to existing files. Uses `--offline` to skip external URL checks in CI.

### 8. validate:asset-integrity

```bash
python3 tests/validate-asset-integrity.py
```

Computes SHA-256 hashes of critical files and compares against `catalog/asset-integrity.json`. Detects unauthorized modifications.

### 9. validate:mcp-trust-matrix

```bash
python3 tests/validate-mcp-trust-matrix.py
```

MCP references must be listed in the trust matrix. Prevents introduction of untrusted external tool integrations.

### 10. validate:no-lifecycle-scripts

```bash
python3 tests/validate-no-lifecycle-scripts.py
```

Scans `package.json` for `preinstall`, `install`, `postinstall`, and other lifecycle hooks. The package must have none.

### 11. validate:promotion-gatekeeper

```bash
python3 tests/validate-nvidia-promotion-gatekeeper.py
```

NVIDIA-specific promotion rules: assets must pass quality thresholds before promotion to higher tiers.

### 12. validate:install-coverage

```bash
node tests/test-vfa-export-coverage.test.mjs
```

Verifies `vfa-export-agents` CLI can export agents for all declared install roles without errors.

### 13. validate:maestro-routing

```bash
python3 tests/validate-maestro-routing.py
```

Runs {{ site.data.catalog.maestro_scenarios }} routing scenarios (intent to agent mapping) through the Maestro router logic. Includes positive matches and refusal cases.

### 14. validate:plugin-manifest

```bash
python3 tests/validate-plugin-manifest.py
```

Confirms `.claude-plugin/plugin.json` is current and internally consistent.

### 15. validate:kiro-powers

```bash
python3 tests/validate-kiro-powers.py
```

Validates Kiro Power definitions in `powers/` against expected structure.

### 16. validate:multi-harness-marketplace

```bash
python3 tests/validate-multi-harness-marketplace.py
```

Cross-harness consistency: agents that declare support for multiple harnesses have correct manifests in each plugin directory.

### 17. validate:codex-marketplace

```bash
python3 tests/validate-codex-marketplace.py
```

Validates `.agents/plugins/marketplace.json` for Codex compatibility.

### 18. validate:finops-fixtures

```bash
python3 tests/validate-finops-price-fixtures.py
```

FinOps price fixture data is valid and complete.

### 19. validate:readme-counts

```bash
node tests/validate-readme-counts.mjs
```

README badge counts (skills, agents, providers) match actual catalog counts.

### 20. validate:qa-cluster

```bash
node tests/eval-qa-cluster.mjs
```

QA cluster evaluation passes.

---

## 🧪 Fuzz Testing

```bash
npm run test:fuzz
```

File: `tests/fuzz-properties.test.mjs`

Uses [fast-check](https://github.com/dubzzz/fast-check) for property-based testing. Generates random inputs to test invariants:

- Catalog parsing handles malformed JSON gracefully
- Schema validation rejects invalid input without crashing
- Routing logic does not throw on unexpected intent strings

---

## 📋 Smoke Tests

### Install Path Smoke Tests

Workflow: `.github/workflows/install-paths-smoke.yml`

Tests that `npm install @raishin/vanguard-frontier-agentic` works in a clean environment and that the `vfa-export-agents` CLI is functional post-install.

### Packed Artifact Smoke Tests

Workflow: `.github/workflows/packed-artifact-smoke.yml`

Tests that `npm pack` produces a valid tarball and that installing from the tarball works correctly.

### Provider Scope Regression

Workflow: `.github/workflows/provider-scope-regression.yml`

Regression tests ensuring provider-scoped agents and skills do not break when new providers are added.

---

## 🛰️ Maestro Routing Validation

The Maestro router is tested with **{{ site.data.catalog.maestro_scenarios }} scenarios** covering:

- Positive routing (correct intent reaches correct agent)
- Negative routing (invalid intent is refused)
- Ambiguous intent resolution
- Provider-scoped intent boundaries

Fixtures are generated by `npm run maestro-routing:write` and validated by `npm run validate:maestro-routing`.

---

## Docs Quality

```bash
npm run lint:docs
```

Combines:
- `markdownlint-cli2` - Markdown style enforcement
- `codespell` - Typo detection (configured via `.codespellrc`)

---

## 📐 How to Add a New Validation Gate

1. Write a validation script in `tests/`:
   - Python: `tests/validate-<name>.py` (exit 1 on failure)
   - Node.js: `tests/validate-<name>.mjs` or `tests/<name>.test.mjs`

2. Add an npm script in `package.json`:
   ```json
   "validate:<name>": "python3 tests/validate-<name>.py"
   ```

3. Append to the `validate` script chain:
   ```json
   "validate": "... && npm run validate:<name>"
   ```

4. Add the gate to CI if it needs special setup (see `.github/workflows/ci.yml`)

5. Update `catalog/asset-integrity.json` if the new script is a critical file:
   ```bash
   npm run asset-integrity:write
   ```

---

## ✅ How to Verify This Works

```bash
# Run everything
npm run validate && npm run test:fuzz

# Run a single gate
npm run validate:catalog

# Check maestro routing specifically
npm run validate:maestro-routing

# Run lint
npm run lint:docs
```

---

## 🏛️ Enterprise Reviewer Notes

- All validation gates run in CI on every PR (`.github/workflows/ci.yml`)
- Fuzz tests run in a separate CI job to avoid timeout-killing the main validation
- The {{ site.data.catalog.maestro_scenarios }} routing scenarios are fixtures, not dynamically generated, ensuring deterministic CI
- Adding a new provider requires adding routing scenarios (the gate enforces coverage)
- The `--offline` flag on link validation prevents CI from failing on transient external URL issues
