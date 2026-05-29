---
layout: default
title: "Getting Started"
permalink: /docs/getting-started/
---

# 🚀 Getting Started

This guide covers installation, first use with each supported AI coding harness, and verification that everything works.

---

## 📋 Prerequisites

| Requirement | Version | Purpose |
|-------------|---------|---------|
| Node.js | 22+ | CLI tools, validation scripts |
| Python | 3.11+ | Validation gates (uses `tomllib`) |
| npm | 10+ | Package installation |
| Git | 2.x | Clone for development |

---

## 📦 Installation

### Option A: npm install (recommended for consumption)

```bash
npm install @raishin/vanguard-frontier-agentic
```

This installs the catalog as a dependency. Skills and agents are accessible from `node_modules/@raishin/vanguard-frontier-agentic/`.

### Option B: Git clone (recommended for contribution)

```bash
git clone https://github.com/Raishin/vanguard-frontier-agentic.git
cd vanguard-frontier-agentic
npm install
```

The `npm install` step pulls devDependencies needed for validation (semantic-release, fast-check).

---

## 🔌 First Use by Harness

### Claude Code

The `.claude-plugin/plugin.json` file registers this catalog. Claude Code discovers agents via the plugin manifest.

```bash
# The CLAUDE.md file at repo root provides steering context
cat CLAUDE.md
```

### Codex (OpenAI)

Codex reads from `.agents/plugins/marketplace.json`. The install script places assets in the Codex home directory:

```bash
npm run install:codex-home
```

### GitHub Copilot

The `.github/plugin/` directory and `copilot-instructions.md` expose skills to Copilot.

```bash
cat .github/copilot-instructions.md
```

### Cursor

The `.cursor-plugin/plugin.json` file provides Cursor with the agent catalog.

### Gemini CLI

Gemini uses the same agent export format. Use the CLI:

```bash
npx vfa-export-agents --platform gemini --all
```

### Kiro

Kiro Powers are generated in `powers/`. The `powers/` directory contains validated power definitions that Kiro discovers automatically.

---

## 🤖 The vfa-export-agents CLI

The `vfa-export-agents` command exports agent configurations for a target platform.

```bash
# Export all agents for Claude
npx vfa-export-agents --platform claude --all

# Export agents for a specific role
npx vfa-export-agents --role cloud-security-engineer

# Export agents from a specific provider
npx vfa-export-agents --provider aws

# Export without skill content (metadata only)
npx vfa-export-agents --platform cursor --no-skills
```

### Available Flags

| Flag | Description |
|------|-------------|
| `--platform` | Target harness (claude, codex, copilot, cursor, gemini, kiro) |
| `--role` | Filter by install role |
| `--provider` | Filter by cloud provider |
| `--all` | Export all agents |
| `--no-skills` | Exclude skill bodies from output |

### Available Install Roles

- `cloud-security-engineer`
- `cloud-platform-engineer`
- `cloud-dba`
- `cloud-finops-analyst`
- `cloud-solutions-architect`
- `cloud-devops-engineer`

---

## ✅ Verification

After installation, confirm the catalog is intact:

```bash
# Run all 17 validation gates
npm run validate

# Check asset integrity specifically
npm run validate:asset-integrity

# Verify catalog indexes match filesystem
npm run validate:catalog

# Run fuzz tests
npm run test:fuzz
```

Expected output: all gates pass with zero errors.

---

## ⚠️ What Can Go Wrong

### `validate:asset-integrity` fails after clone

**Cause:** The `catalog/asset-integrity.json` manifest is stale relative to tracked files.

**Fix:**
```bash
python3 tests/validate-asset-integrity.py --write
git diff catalog/asset-integrity.json
```

### Python validation scripts fail with `ModuleNotFoundError: tomllib`

**Cause:** Python version is below 3.11. The `tomllib` module was added in 3.11.

**Fix:** Upgrade Python to 3.11+.

### `npm run validate:plugin-manifest` reports version mismatch

**Cause:** The plugin manifest version has not been regenerated after a version bump.

**Fix:**
```bash
npm run plugin-manifest:write
```

### `vfa-export-agents` not found

**Cause:** The `bin` field in `package.json` registers the CLI. It requires either a global install or `npx`.

**Fix:** Use `npx vfa-export-agents` or install globally.

### Node.js version errors

**Cause:** Scripts use ESM (`import`) and modern Node APIs requiring Node 22+.

**Fix:** Install Node 22 via nvm or mise.

---

## 🎯 Next Steps

- Read the [Architecture](../architecture/) page for system design
- See [Configuration Reference](../configuration/) for all settings
- Check [Testing](../testing/) for the full validation gate list
