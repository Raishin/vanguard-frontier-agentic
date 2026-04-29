## EVAL DEFINITION: aws-progressive-disclosure-references

### Assumptions

- Existing repo convention uses `.claude/evals/`; this eval follows that convention.
- The target pattern is the Azure Cosmos DB skill shape: lean `SKILL.md`, on-demand `references/`, metadata/version in frontmatter and adjacent metadata, catalog alignment, and manifest refresh.
- Context7 grounding is used for current Amazon Bedrock AgentCore concepts, but official AWS docs/reference files do not prove live AWS account state.

### Capability Evals

1. Every `skills/aws/<skill>/` directory has a `references/` folder.
2. Every AWS `SKILL.md` links to at least three lazy-loaded reference files.
3. Each AWS skill has `references/workflow-and-output.md`, `references/official-sources.md`, and `references/safety-checklist.md`.
4. `aws-agentcore` is cataloged with `metadata.json`, frontmatter `metadata.author`, frontmatter `metadata.version`, official docs, and manifest coverage.
5. Amazon Bedrock AgentCore references cover runtime/local workflow, Memory, Gateway/MCP tools, Identity/permissions, Observability, Browser, Code Interpreter, and security caveats without asking for secrets.
6. AWS `SKILL.md` files stay lean enough for trigger-time loading and defer detailed docs to references.

### Regression Evals

1. AWS trigger keyword validation still passes.
2. Every AWS `SKILL.md` frontmatter version matches adjacent `metadata.json` and `catalog/skills.json`.
3. `catalog/skill-manifest.json` is refreshed after the reference-file additions.
4. Existing catalog, manifest, and offline link validation still pass.
5. No absolute personal filesystem paths are introduced.

### Deterministic Graders

```bash
python3 tests/validate-aws-skill-quality.py
python3 tests/validate-aws-progressive-disclosure.py
npm run manifest:write
npm run validate
```

### Success Metrics

- Capability checks: pass@1 = 100% after implementation.
- Regression checks: pass^1 = 100%.

### Human Review Required

AgentCore is new and changing. Treat bundled references as a starter baseline; verify exact CLI syntax against current AWS docs/tooling before production use.

## EVAL REPORT: aws-progressive-disclosure-references

### Capability Evals

- references-folder-for-every-aws-skill: PASS — `python3 tests/validate-aws-progressive-disclosure.py` validated 26 AWS skills with `references/` folders.
- lazy-reference-links: PASS — every AWS `SKILL.md` links `references/workflow-and-output.md`, `references/safety-checklist.md`, and `references/official-sources.md`.
- agentcore-cataloged: PASS — `aws-agentcore` now has `metadata.json`, catalog entry, frontmatter metadata, official docs, and manifest coverage.
- agentcore-component-coverage: PASS — AgentCore references cover runtime, Memory, Gateway, Identity, Observability, Browser, Code Interpreter, and MCP/tool guidance.
- lean-skill-files: PASS — the progressive-disclosure validator enforces AWS `SKILL.md` files at 90 lines or fewer.

### Regression Evals

- aws-trigger-keywords: PASS — `python3 tests/validate-aws-skill-quality.py` validated trigger descriptions and versions for 26 AWS skills.
- version-sync: PASS — AWS `SKILL.md` frontmatter version, adjacent `metadata.json`, and `catalog/skills.json` are aligned at `0.1.1`.
- manifest-current: PASS — `npm run manifest:write` refreshed `catalog/skill-manifest.json`, and `npm run validate` confirmed it matches 79 skill entries.
- npm-package-boundary: PASS — `npm pack --dry-run --json` confirmed `docs/` has 0 package entries.
- no-personal-absolute-paths: PASS — previous scan found no forbidden personal absolute paths after replacement.

### Commands run

```bash
npm run manifest:write
python3 tests/validate-aws-skill-quality.py
python3 tests/validate-aws-progressive-disclosure.py
for d in skills/aws/*/; do python <skill-creator>/scripts/quick_validate.py "$d" >/dev/null; done
npm run validate
npm pack --dry-run --json
```

### Results

```text
OK: wrote catalog/skill-manifest.json with 79 skill entries
OK: validated 26 AWS skill trigger descriptions, versions, and manifest entries
OK: validated progressive references for 26 AWS skills
OK: validated 135 catalog entries and scanned for obvious secrets
OK: skill manifest matches 79 skill entries
OK: validated README links and 318 URLs (offline)
docs entries in pack: 0
```

### Status

READY, with one caveat: AgentCore is evolving. The bundled AgentCore references are intentionally lazy-loaded starter guidance; exact CLI syntax should still be verified against the installed toolkit and current AWS docs before production execution.
