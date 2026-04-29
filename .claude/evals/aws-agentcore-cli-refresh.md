## EVAL DEFINITION: aws-agentcore-cli-refresh

### Assumptions

- Official AWS documentation is the source of truth for current AgentCore CLI recommendation and package name.
- The starter toolkit repository can lag or contain migration-era wording; the skill must label such conflicts instead of flattening them.
- This repo uses `.claude/evals/`, so the eval artifact stays there.

### Capability Evals

1. The `aws-agentcore` skill explicitly states that the recommended CLI for new projects is `@aws/agentcore`.
2. The skill distinguishes the npm CLI from the Python starter toolkit and explains when each is appropriate.
3. The references mention current CLI-first workflows for create/dev/deploy/invoke/eval and note that the starter toolkit remains relevant for existing Python-based workflows or migration contexts.
4. The official-sources reference includes current AWS devguide links for AgentCore CLI getting started and available interfaces.
5. The skill does not tell users to paste secrets, hardcode credentials, or rely on stale `launch` or old package-name guidance without caveat.

### Regression Evals

1. `aws-agentcore` metadata, catalog entry, and manifest remain aligned.
2. AWS trigger keyword validation still passes.
3. Progressive-disclosure validation still passes.
4. Repository validation still passes.

### Deterministic Graders

```bash
python3 tests/validate-aws-skill-quality.py
python3 tests/validate-aws-progressive-disclosure.py
npm run manifest:write
npm run validate
```

## EVAL REPORT: aws-agentcore-cli-refresh

### Capability Evals

- recommended-cli-stated: PASS — `skills/aws/aws-agentcore/SKILL.md` now says new projects should prefer the npm package `@aws/agentcore`.
- cli-vs-starter-toolkit-distinguished: PASS — AgentCore references now distinguish the recommended npm CLI from the legacy/migration-oriented Python starter toolkit.
- current-cli-areas-grounded: PASS — `official-sources.md` now includes the current AWS devguide CLI getting-started and available-interfaces pages, alongside runtime, Memory, Gateway, Identity, Observability, Browser, and Code Interpreter references.
- stale-guidance-mitigated: PASS — getting-started, memory, gateway, workflow, and safety references now explicitly warn against presenting the starter toolkit as the preferred greenfield path.
- agent-surface-updated: PASS — `agents/openai.yaml` now points prompts toward the CLI-first workflow for new projects.

### Regression Evals

- metadata-catalog-manifest-alignment: PASS — `aws-agentcore` version is aligned at `0.1.2` across `SKILL.md`, `metadata.json`, `catalog/skills.json`, and `catalog/skill-manifest.json`.
- aws-trigger-quality: PASS — `python3 tests/validate-aws-skill-quality.py`
- progressive-disclosure: PASS — `python3 tests/validate-aws-progressive-disclosure.py`
- repo-validation: PASS — `npm run validate`

### Commands run

```bash
npm run manifest:write
python3 tests/validate-aws-skill-quality.py
python3 tests/validate-aws-progressive-disclosure.py
npm run validate
```

### Results

```text
OK: wrote catalog/skill-manifest.json with 79 skill entries
OK: validated 26 AWS skill trigger descriptions, versions, and manifest entries
OK: validated progressive references for 26 AWS skills
OK: validated 135 catalog entries and scanned for obvious secrets
OK: skill manifest matches 79 skill entries
OK: validated README links and 320 URLs (offline)
```

### Status

READY. One explicit caveat remains: Context7 surfaced mixed package-name guidance from starter-toolkit materials, but the newer AWS devguide evidence points to `npm install -g @aws/agentcore` for the recommended CLI path, so the skill now reflects that newer official direction.
