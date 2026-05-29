---
layout: default
title: "Troubleshooting"
permalink: /docs/troubleshooting/
---

# Troubleshooting

Common issues organized as problem/cause/fix tables.

---

## Validation Failures

### validate:asset-integrity

| Problem | Cause | Fix |
|---------|-------|-----|
| Hash mismatch for a tracked file | File was modified without regenerating integrity manifest | `python3 tests/validate-asset-integrity.py --write` then commit |
| New file not in manifest | A critical file was added but not registered | Regenerate: `npm run asset-integrity:write` |
| File deleted but still in manifest | File was removed without updating manifest | Regenerate: `npm run asset-integrity:write` |

### validate:catalog

| Problem | Cause | Fix |
|---------|-------|-----|
| Agent in filesystem but not in catalog | New agent added without regeneration | `npm run manifest:write:all` |
| Stale skill count | Skills added/removed without catalog update | `npm run manifest:write` |
| Orphaned catalog entry | Agent directory deleted but catalog not rebuilt | `npm run manifest:write:all` |

### validate:skill-schema / validate:agent-schema

| Problem | Cause | Fix |
|---------|-------|-----|
| Missing required field | Frontmatter incomplete | Check schema at `schemas/skill.frontmatter.schema.json` or `schemas/agent.frontmatter.schema.json` |
| Wrong type for field | e.g., `triggers` is a string instead of array | Fix the frontmatter in the skill/agent file |
| Extra fields not allowed | Strict schema mode rejects unknown keys | Remove the extra field or update the schema |

### validate:links

| Problem | Cause | Fix |
|---------|-------|-----|
| Broken relative link | Target file moved or renamed | Update the link path |
| Link to non-existent anchor | Heading was changed | Update the anchor reference |
| False positive on external URL | Using `--offline` mode | External links are not checked in CI; verify manually |

### validate:plugin-manifest

| Problem | Cause | Fix |
|---------|-------|-----|
| Version mismatch | `package.json` version bumped but manifest not regenerated | `npm run plugin-manifest:write` |
| Stale agent list | Agents added without manifest update | `npm run plugin-manifest:write` |

### validate:maestro-routing

| Problem | Cause | Fix |
|---------|-------|-----|
| Fixture count mismatch | New provider added without routing scenarios | Add scenarios, then `npm run maestro-routing:write` |
| Routing regression | Agent removed but fixture still expects it | Regenerate fixtures: `npm run maestro-routing:write` |

### validate:readme-counts

| Problem | Cause | Fix |
|---------|-------|-----|
| Count mismatch | Skills or agents added without README update | `npm run readme-counts:write` |

### validate:no-lifecycle-scripts

| Problem | Cause | Fix |
|---------|-------|-----|
| Lifecycle script detected | `package.json` has install/preinstall/postinstall | Remove the lifecycle script. This package must not execute code on install. |

---

## Release Failures

### semantic-release

| Problem | Cause | Fix |
|---------|-------|-----|
| "No relevant changes" | All commits are chore/docs type | Expected behavior. No release needed. |
| Git push fails | Token expired or permissions issue | Check that workflow has `contents: write` and `persist-credentials: true` on checkout |
| Tag already exists | Manual tag was created outside semantic-release | Delete the conflicting tag and re-run |

### npm Publish

| Problem | Cause | Fix |
|---------|-------|-----|
| 403 Forbidden | Trusted publisher config mismatch | Verify npmjs.com: owner=raishin (lowercase), repo, workflow, environment must match exactly |
| OIDC token unavailable | `id-token: write` missing from permissions | Add permission to workflow |
| Timeout | npm registry outage | Retry via workflow_dispatch with `republish: true` |
| "Package already exists" | Version already published | Expected if re-running after partial failure. Use `republish: true`. |

---

## CI Failures

### Python Version

| Problem | Cause | Fix |
|---------|-------|-----|
| `ModuleNotFoundError: tomllib` | Python < 3.11 | Use Python 3.11+ (`tomllib` was added in 3.11) |
| Script syntax error | Python 2 interpreter | Ensure `python3` points to 3.11+ |

### Node.js Version

| Problem | Cause | Fix |
|---------|-------|-----|
| ESM import errors | Node < 22 | Use Node.js 22+ |
| Missing `node:test` | Node < 18 | Upgrade Node.js |

### General CI

| Problem | Cause | Fix |
|---------|-------|-----|
| Random timeout | GitHub-hosted runner resource limits | Re-run the job |
| Dependency install fails | npm registry issue | Re-run the job (transient) |
| Permission denied | Workflow permissions too restrictive | Check `permissions` block matches required scopes |

---

## Plugin Installation Issues

### Claude Code

| Problem | Cause | Fix |
|---------|-------|-----|
| Plugin not discovered | `.claude-plugin/plugin.json` not in package | Check `files` field in `package.json` includes the plugin directory |
| Wrong version in manifest | Plugin manifest is stale | `npm run plugin-manifest:write` |

### Codex

| Problem | Cause | Fix |
|---------|-------|-----|
| Install script fails | Wrong Node version or missing dependencies | Ensure Node 22+, run `npm install` first |
| Marketplace entry missing | Agent not in `.agents/plugins/marketplace.json` | `npm run manifest:write:all` |

### Cursor

| Problem | Cause | Fix |
|---------|-------|-----|
| Plugin not loading | `.cursor-plugin/plugin.json` invalid | `npm run cursor-plugin:write` |

### Kiro

| Problem | Cause | Fix |
|---------|-------|-----|
| Power not found | Powers not generated | `npm run kiro-powers:write` |
| Validation fails | Power structure invalid | Check `tests/validate-kiro-powers.py` output |

---

## Local Development Issues

| Problem | Cause | Fix |
|---------|-------|-----|
| `vfa-export-agents` not found | Not installed globally or not using npx | Use `npx vfa-export-agents` |
| Jekyll serve fails | Ruby or bundler not installed | Install Ruby 3.3+, then `gem install bundler && bundle install` |
| Codespell reports false positive | Technical term not in dictionary | Add to `.codespellrc` ignore list |

---

## Decision Tree: Validation Failed in CI

```
Which gate failed?
  |
  |-- asset-integrity --> Run `npm run asset-integrity:write`, commit
  |-- catalog --> Run `npm run manifest:write:all`, commit
  |-- plugin-manifest --> Run `npm run plugin-manifest:write`, commit
  |-- readme-counts --> Run `npm run readme-counts:write`, commit
  |-- skill-schema --> Fix frontmatter in the failing skill file
  |-- agent-schema --> Fix metadata.json in the failing agent
  |-- maestro-routing --> Add/fix routing fixtures, `npm run maestro-routing:write`
  |-- links --> Fix broken link or update anchor
  |-- no-lifecycle-scripts --> Remove the lifecycle script from package.json
  |-- Other --> Read the test script's error output for specific guidance
```

---

## How to Verify This Works

After applying a fix:

```bash
# Run the specific failing gate
npm run validate:<gate-name>

# Then run full validation to confirm no regressions
npm run validate
```
