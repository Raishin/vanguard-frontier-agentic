# EVAL: marketplace-install-paths

**Scope:** Plug-and-play install paths for 5 harnesses (Claude Code, Codex,
Copilot CLI, Cursor, Kiro) + 14 per-provider Kiro Powers.

**Rubric:**
1. **Canonical path correctness** — each manifest at the path the vendor docs prescribe
2. **Vendor-docs link integrity** — every docs URL in the sidecar READMEs is reachable
3. **Validator coverage** — every marketplace has a validator gate in `npm run validate`
4. **Install-command runnability** — each install command from the README dropdown is syntactically valid

## Capability Evals (define what must work)

### C1. Canonical paths
- [ ] `.claude-plugin/marketplace.json` exists (Claude Code spec)
- [ ] `.claude-plugin/plugin.json` exists (Claude Code spec)
- [ ] `.cursor-plugin/plugin.json` exists (Cursor spec)
- [ ] `.github/plugin/marketplace.json` exists (Copilot CLI spec)
- [ ] `.agents/plugins/marketplace.json` exists (Codex spec)
- [ ] `plugins/vanguard-frontier-agentic/.codex-plugin/plugin.json` exists (Codex plugin spec)
- [ ] `powers/vanguard-<provider>/POWER.md` exists for all 14 providers (Kiro Powers spec)

### C2. Vendor-docs link integrity
- [ ] All `code.claude.com/docs/` URLs in `.claude-plugin/README.md` reachable
- [ ] All `cursor.com/docs/` URLs in `.cursor-plugin/README.md` reachable
- [ ] All `github.com/github/copilot-cli` + GitHub Docs URLs reachable
- [ ] All `github.com/openai/codex/...` URLs reachable
- [ ] All `github.com/kirodotdev/powers` URLs reachable

### C3. Validator coverage
- [ ] `validate:plugin-manifest` is in the `npm run validate` chain (Claude Code)
- [ ] `validate:multi-harness-marketplace` is in the chain (Cursor + Copilot)
- [ ] `validate:codex-marketplace` is in the chain (Codex)
- [ ] `validate:kiro-powers` is in the chain (Kiro)

### C4. Install-command runnability
- [ ] Claude Code: `/plugin marketplace add Raishin/vanguard-frontier-agentic`
- [ ] Copilot CLI: `copilot plugin marketplace add Raishin/vanguard-frontier-agentic`
- [ ] Cursor: `vscode.cursor.plugins.registerPath(...)` call shape
- [ ] Codex: `codex plugin marketplace add Raishin/vanguard-frontier-agentic`
- [ ] Kiro: clone + Add Custom Power per Power directory (no CLI command, UI-driven)

## Regression Evals (don't break what already worked)

### R1. All 17 npm validate gates still pass
### R2. Maestro routing (357 scenarios across 14 maestros) still passes
### R3. Asset integrity manifest still matches (post-write)
### R4. The Kiro Powers strict-5 frontmatter is still enforced

## Success Metrics

- **Capability evals**: pass@1 >= 100% (deterministic file/grep checks; no retry needed)
- **Regression evals**: pass^1 = 100% (all 17 gates green)

## Grader Types

| Eval | Grader |
|------|--------|
| C1 (paths) | Code — `test -f` |
| C2 (links) | Code — `validate-links.py --offline` already checks every URL in tracked docs |
| C3 (validator) | Code — grep the `validate` script in package.json |
| C4 (commands) | Rule — regex against the README dropdown contents |
| R1-R4 | Code — `npm run validate` |
