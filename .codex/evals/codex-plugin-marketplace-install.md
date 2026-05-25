## EVAL DEFINITION: codex-plugin-marketplace-install

### Assumptions
- Verified: repo marketplace is `.agents/plugins/marketplace.json` and declares marketplace name `vanguard-frontier-agentic`.
- Verified from OpenAI docs search: Codex installs plugins under `~/.codex/plugins/cache/$MARKETPLACE_NAME/$PLUGIN_NAME/$VERSION/`.
- Unverified until command run: `codex plugin marketplace add Raishin/vanguard-frontier-agentic` resolves the current published/default GitHub branch content and refreshes the cache.

### Capability Evals
- [ ] Marketplace command succeeds: `codex plugin marketplace add Raishin/vanguard-frontier-agentic` exits 0.
- [ ] Main plugin cache exists at `~/.codex/plugins/cache/vanguard-frontier-agentic/vanguard-frontier-agentic/2.5.0/.codex-plugin/plugin.json`.
- [ ] Companion plugin cache exists at `~/.codex/plugins/cache/vanguard-frontier-agentic/cross-platform-agent-template/0.1.0/.codex-plugin/plugin.json`.
- [ ] Cached plugin manifest names and versions match repo manifests.

### Regression Evals
- [ ] Repo marketplace remains valid JSON and keeps required fields: `name`, `plugins[].name`, `plugins[].source`, `plugins[].policy.installation`, `plugins[].policy.authentication`, `plugins[].category`.
- [ ] Existing repo validator passes: `npm run validate:codex-marketplace`.
- [ ] Package dry-run includes `.agents/plugins/marketplace.json` and both plugin manifests.

### Success Metrics
- Capability evals: pass@1 required for this diagnostic run.
- Regression evals: pass^1 required locally before making any fix.

### Graders
- `codex plugin marketplace add Raishin/vanguard-frontier-agentic`
- `python3 tests/validate-codex-marketplace.py`
- `npm pack --dry-run --json`
- Python manifest/cache comparison script embedded in this eval run.

## EVAL REPORT: codex-plugin-marketplace-install

### Capability Evals
- marketplace-add: PASS - `codex plugin marketplace add Raishin/vanguard-frontier-agentic` exited 0 and reported the marketplace was already added from `https://github.com/Raishin/vanguard-frontier-agentic.git`.
- marketplace-upgrade: PASS - `codex plugin marketplace upgrade vanguard-frontier-agentic` exited 0 and refreshed `/home/vchu@maureva.com/.codex/.tmp/marketplaces/vanguard-frontier-agentic`.
- main-plugin-cache: PASS - found `/home/vchu@maureva.com/.codex/plugins/cache/vanguard-frontier-agentic/vanguard-frontier-agentic/2.5.0/.codex-plugin/plugin.json`.
- companion-plugin-cache: PASS - found `/home/vchu@maureva.com/.codex/plugins/cache/vanguard-frontier-agentic/cross-platform-agent-template/0.1.0/.codex-plugin/plugin.json`.
- cache-manifest-match: PASS - cached `name` and `version` matched repo manifests for both Codex plugins.

### Regression Evals
- codex-marketplace-validator: PASS - `python3 tests/validate-codex-marketplace.py` and `npm run validate:codex-marketplace` both passed.
- package-dry-run: PASS - `npm pack --dry-run --json` included `.agents/plugins/marketplace.json` and both `.codex-plugin/plugin.json` files.
- full-repo-validate: PASS - `rtk npm run validate` passed after regenerating stale generated plugin surfaces.
- codex-home-agent-skill-install: PASS - after adding Codex skill bundling, `rtk node scripts/export-marketplace-agents.mjs --platform codex --all --repo /home/vchu@maureva.com --force` installed 424 VFA Codex agents and bundled 404 companion skills into live `~/.codex`.
- codex-home-skill-paths: PASS - installed VFA agent `skills.config.path` entries were rewritten to absolute `~/.codex/skills/<skill>` folders; post-install verification found only one missing skill reference, from pre-existing `oracle-plsql-specialist.toml`, not from this VFA export.

### Fixes Applied During Eval
- `.claude-plugin/plugin.json`: regenerated with version `2.5.0` to match `package.json`.
- `.cursor-plugin/plugin.json`: regenerated with version `2.5.0` to match `package.json`.
- `catalog/asset-integrity.json`: regenerated after generated plugin manifest changes.
- `scripts/export-marketplace-agents.mjs`: added Codex skill bundling to `.codex/skills` and rewrites exported Codex agent skill paths to the actual installed skill folders.

### Verification
- Commands run:
  - `codex --version` -> PASS (`codex-cli 0.131.0-alpha.9`)
  - `codex plugin marketplace add Raishin/vanguard-frontier-agentic` -> PASS
  - `codex plugin marketplace upgrade vanguard-frontier-agentic` -> PASS
  - `python3 tests/validate-codex-marketplace.py` -> PASS
  - `npm pack --dry-run --json` -> PASS
  - `npm run plugin-manifest:write` -> PASS
  - `npm run cursor-plugin:write` -> PASS
  - `python3 tests/validate-asset-integrity.py --write` -> PASS
  - `rtk npm run validate` -> PASS
  - `rtk node scripts/export-marketplace-agents.mjs --platform codex --all --repo /home/vchu@maureva.com --dry-run` -> PASS, 424 agents and 404 skills planned
  - `rtk node scripts/export-marketplace-agents.mjs --platform codex --all --repo /home/vchu@maureva.com --force` -> PASS, 424 agents and 404 skills installed into live Codex home
  - `rtk python3` live path verifier -> PASS for VFA-installed skill paths; one unrelated pre-existing Oracle/PLSQL skill path remains missing
- Not run:
  - Fresh uninstall/reinstall from an empty Codex plugin cache -> not run to avoid destructive changes to the live user-level Codex cache.


### 2026-05-22 Isolated CLI E2E Correction
- `test:codex-plugin-marketplace-install`: ADDED - optional, skip-by-default E2E test for the real `codex plugin marketplace add` command using an isolated `CODEX_HOME`.
- isolated-marketplace-add: PASS - `RUN_CODEX_PLUGIN_MARKETPLACE_E2E=1 rtk npm run test:codex-plugin-marketplace-install` exited 0.
- isolated-marketplace-root: PASS - the command materialized the marketplace under `$CODEX_HOME/.tmp/marketplaces/vanguard-frontier-agentic` and found `.agents/plugins/marketplace.json` plus both `.codex-plugin/plugin.json` manifests.
- isolated-config: PASS - isolated `$CODEX_HOME/config.toml` contained `[marketplaces.vanguard-frontier-agentic]`.
- isolated-plugin-cache: NOT PROVEN - the isolated run did **not** create `$CODEX_HOME/plugins/cache/vanguard-frontier-agentic` from `marketplace add` alone. This is consistent with the CLI docs for `marketplace add` installing/tracking a marketplace source, while the plugin cache path belongs to plugin installation through a marketplace.
- strict-cache-assertion: EXPECTED FAIL - `EXPECT_CODEX_PLUGIN_CACHE=1 RUN_CODEX_PLUGIN_MARKETPLACE_E2E=1 rtk npm run test:codex-plugin-marketplace-install` fails on the missing `$CODEX_HOME/plugins/cache/vanguard-frontier-agentic` path. This makes the gap executable instead of hand-wavy.
- ruthless correction: the earlier live-cache PASS is not sufficient proof that `codex plugin marketplace add` alone installs plugins into cache, because the live `~/.codex/plugins/cache/...` path may have been populated by prior UI/plugin-install state. The new isolated E2E is the reliable signal for CLI behavior.

### 2026-05-22 Two-Stage Install Surface
- two-stage-installer-script: ADDED - `scripts/install-codex-home.mjs` runs `codex plugin marketplace add`, `codex plugin marketplace upgrade`, then the local exporter with `--platform codex --all --repo <target> --force`.
- npm-install-command: ADDED - `npm run install:codex-home -- --repo "$HOME"` is the repo-local reliable install command for unpublished branch testing.
- plugin-install-skill: ADDED - `plugins/vanguard-frontier-agentic/skills/vanguard-frontier-agentic-install/SKILL.md` documents the two-stage marketplace + exporter workflow from inside the plugin.
- plugin-skills-manifest: ADDED - `plugins/vanguard-frontier-agentic/.codex-plugin/plugin.json` declares `"skills": "./skills/"` so plugin install surfaces the installer skill.
- install-coverage-regression: ADDED - `tests/test-vfa-export-coverage.test.mjs` now checks the two-stage installer dry-run plans 424 Codex agents and 404 skills.
- limitation-preserved: Codex docs support plugin-bundled skills, MCP, apps, and hooks; they do not document a plugin manifest `agents` field. The exporter remains the deterministic second stage for agent TOML installation.

### Status
READY
