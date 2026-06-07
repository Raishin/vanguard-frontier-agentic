## EVAL DEFINITION: cloud-skills-reference-quality-azure-skills-005

### Scope
Exactly five Azure skills in stable sorted order:
1. skills/azure/azure-live-pim-jit-activation-guard
2. skills/azure/azure-maestro
3. skills/azure/azure-migrate-landing-zone-cutover
4. skills/azure/azure-network-topology-review
5. skills/azure/azure-observability-investigator

### Assumptions
- Microsoft Learn documentation proves documented Azure behavior only.
- No tenant, subscription, migration project, network, or monitoring state is proven without sampled read-only evidence.
- This batch changes Azure skill assets only; no AWS assets are in scope.

### Capability Evals
- [ ] Each target has one component-specific operations guide with AgentCore-standard headings.
- [ ] Each target has refreshed official-sources, safety-checklist, workflow-and-output, and mcp-and-evidence references.
- [ ] Each changed target bumps patch version to 0.1.1 in SKILL.md, metadata.json, and catalog/skills.json.
- [ ] Azure Maestro live-guard routing reflects current live-guard set and does not hard-code stale catalog count.
- [ ] References avoid internal environment identifiers and use generic configured documentation evidence phrasing.

### Regression Evals
- [ ] npm run validate:skill-schema passes.
- [ ] npm run manifest:check passes after manifest regeneration.
- [ ] npm run validate:asset-integrity passes after integrity regeneration.
- [ ] npm run validate passes.
- [ ] git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws returns no paths.

### Graders
- Structural Python grader for required guide headings, versions, reference files, and Maestro live-guard route presence.
- Prohibited wording grep for internal wording and sample identifiers.
- Repo validation commands listed in the report.

## EVAL REPORT

### Capability Evals
- [x] Each target has one component-specific operations guide with AgentCore-standard headings.
- [x] Each target has refreshed official-sources, safety-checklist, workflow-and-output, and mcp-and-evidence references.
- [x] Each changed target bumps patch version to 0.1.1 in SKILL.md, metadata.json, and catalog/skills.json.
- [x] Azure Maestro live-guard routing reflects current live-guard set and does not hard-code stale catalog count.
- [x] References avoid internal environment identifiers and use generic configured documentation evidence phrasing.

### Regression Evals
- [x] npm run validate:skill-schema passes.
- [x] npm run manifest:check passes after manifest regeneration.
- [x] npm run validate:asset-integrity passes after integrity regeneration.
- [x] npm run validate passes.
- [x] git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws returns no paths.

### Validation commands
- `python3 /tmp/check_batch005.py`
- prohibited wording grep over the five target directories
- `npm run validate:skill-schema`
- `npm run manifest:check`
- `npm run validate:asset-integrity`
- `npm run validate`
- `git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws`

### Verdict
Pass. Batch 005 is validated and remains uncommitted.

