## EVAL DEFINITION: cloud-skills-reference-quality-azure-skills-007

### Scope
Exactly five Azure skills in stable sorted order:
1. skills/azure/azure-role-selector
2. skills/azure/azure-security-posture-hardening
3. skills/azure/azure-subscription-resource-organization
4. skills/azure/azure-waf-cost-optimization-review
5. skills/azure/azure-waf-reliability-review

### AgentCore reference standard
The target standard is the structure and evidence density used by `skills/aws/aws-agentcore/references`: component-specific references must include sharp "What people get wrong" sections, officially grounded service shape, non-negotiable design rules, minimal safe implementation flow, verification targets, and push-back conditions. Common references must cover official sources, safety checklist, evidence labels, workflow, and output contract.

### Assumptions
- Microsoft Learn documentation proves documented Azure behavior only.
- No tenant, subscription, resource, security posture, billing, recommendation, or reliability state is proven without sampled read-only evidence.
- This batch changes Azure skill assets only; no AWS assets are in scope.

### Capability Evals
- [ ] Each target has one component-specific operations guide with AgentCore-standard headings and actionable, source-grounded content.
- [ ] Each target has refreshed official-sources, safety-checklist, workflow-and-output, and mcp-and-evidence references.
- [ ] Each target bumps patch version to 0.1.1 in SKILL.md, metadata.json, and catalog/skills.json.
- [ ] Existing stale generic evidence or tool phrasing is replaced with approved generic documentation evidence phrasing.
- [ ] WAF cost and reliability primary docs are lean and delegate detailed guidance to references.

### Regression Evals
- [ ] npm run validate:skill-schema passes.
- [ ] npm run manifest:check passes after manifest regeneration.
- [ ] npm run validate:asset-integrity passes after integrity regeneration.
- [ ] npm run validate passes.
- [ ] git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws returns no paths.

### Graders
- Structural Python grader for required headings, versions, reference files, catalog entries, and prohibited URL/string checks.
- Prohibited wording grep for internal wording and sample identifiers.
- Repo validation commands listed in the report.

## EVAL REPORT

### Capability Evals
- [x] Each target has one component-specific operations guide with AgentCore-standard headings and actionable, source-grounded content.
- [x] Each target has refreshed official-sources, safety-checklist, workflow-and-output, and mcp-and-evidence references.
- [x] Each target bumps patch version to 0.1.1 in SKILL.md, metadata.json, and catalog/skills.json.
- [x] Existing stale generic evidence or tool phrasing is replaced with approved generic documentation evidence phrasing.
- [x] WAF cost and reliability primary docs are lean and delegate detailed guidance to references.

### Regression Evals
- [x] npm run validate:skill-schema passes.
- [x] npm run manifest:check passes after manifest regeneration.
- [x] npm run validate:asset-integrity passes after integrity regeneration.
- [x] npm run validate passes.
- [x] git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws returns no paths.

### Validation commands
- Structural Python grader for batch 007
- Prohibited wording grep over the five target directories
- `npm run validate:skill-schema`
- `npm run manifest:check`
- `npm run validate:asset-integrity`
- `npm run validate`
- `git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws`

### Verdict
Pass. Batch 007 is validated and remains uncommitted.

