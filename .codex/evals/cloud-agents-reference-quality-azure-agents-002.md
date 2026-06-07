## EVAL DEFINITION: cloud-agents-reference-quality-azure-agents-002

### Scope
Exactly five Azure agents in stable sorted order:
1. agents/azure/azure-cosmosdb-platform-operator-agent
2. agents/azure/azure-cost-estimation-review-agent
3. agents/azure/azure-cost-optimization-governor-agent
4. agents/azure/azure-entra-id-specialist-agent
5. agents/azure/azure-governance-policy-guardrails-agent

### AgentCore reference standard
The target standard is the structure and evidence density used by `skills/aws/aws-agentcore/references`: component references must include a version note, sharp "What people get wrong" sections, officially grounded service shape, non-negotiable design rules, minimal safe implementation flow, high-risk assumptions to kill, safe verification targets, and push-back conditions. Common references must cover official sources, safety checklist, evidence labels, workflow, output contract, and generic documentation MCP wording.

### Assumptions
- Microsoft Learn documentation proves documented Azure behavior only.
- No tenant, subscription, RBAC, quota, licensing, deployed resource state, production readiness, cost posture, or incident status is proven without sampled read-only configured-environment evidence.
- This batch changes Azure agent assets only; no AWS assets are in scope.

### Capability Evals
- [ ] Each target has one component-specific operations guide with AgentCore-standard headings and actionable, source-grounded content.
- [ ] Each operations guide includes a version note, "What people get wrong", officially grounded service shape, non-negotiable design rules, minimal safe flow, high-risk assumptions, verification targets, and push-back conditions.
- [ ] Each target has refreshed official-sources, safety-checklist, workflow-and-output, and mcp-and-evidence references.
- [ ] Each target bumps patch version to 0.2.1 in AGENT.md, metadata.json, and catalog/agents.json.
- [ ] Existing stale generic evidence or tool phrasing is replaced with approved generic documentation evidence phrasing.
- [ ] Agent harnesses remain compact and point to the bound skill/reference discipline without leaking environment-specific identifiers.

### Regression Evals
- [ ] npm run validate:agent-schema passes.
- [ ] npm run validate:plugin-manifest passes after plugin manifest regeneration.
- [ ] npm run validate:kiro-powers passes after Kiro Powers regeneration.
- [ ] npm run validate:multi-harness-marketplace passes.
- [ ] npm run validate:codex-marketplace passes.
- [ ] npm run validate:asset-integrity passes after integrity regeneration.
- [ ] npm run validate passes.
- [ ] git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws returns no paths.

### Graders
- Structural Python grader for required headings, versions, reference files, catalog entries, TOML syntax, and prohibited string checks.
- Prohibited wording grep for internal wording and sample identifiers over the five target directories.
- Repo validation commands listed in the report.

## EVAL REPORT

### Capability Evals
- [x] Each target has one component-specific operations guide with AgentCore-standard headings and actionable, source-grounded content.
- [x] Each operations guide includes a version note, "What people get wrong", officially grounded service shape, non-negotiable design rules, minimal safe flow, high-risk assumptions, verification targets, and push-back conditions.
- [x] Each target has refreshed official-sources, safety-checklist, workflow-and-output, and mcp-and-evidence references.
- [x] Each target bumps patch version to 0.2.1 in AGENT.md, metadata.json, and catalog/agents.json.
- [x] Existing stale generic evidence or tool phrasing is replaced with approved generic documentation evidence phrasing.
- [x] Agent harnesses remain compact and point to the bound skill/reference discipline without leaking environment-specific identifiers.

### Regression Evals
- [x] npm run validate:agent-schema passes.
- [x] npm run validate:plugin-manifest passes after plugin manifest regeneration.
- [x] npm run validate:kiro-powers passes after Kiro Powers regeneration.
- [x] npm run validate:multi-harness-marketplace passes.
- [x] npm run validate:codex-marketplace passes.
- [x] npm run validate:asset-integrity passes after integrity regeneration.
- [x] npm run validate passes.
- [x] git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws returns no paths.

### Validation commands
- Structural Python grader for Azure agents batch 002
- Prohibited wording grep over the five target directories
- `npm run validate:agent-schema`
- `npm run validate:plugin-manifest`
- `npm run validate:kiro-powers`
- `npm run validate:multi-harness-marketplace`
- `npm run validate:codex-marketplace`
- `npm run validate:asset-integrity`
- `npm run validate`
- `git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws`

### Verdict
Pass. Azure agents batch 002 is validated and remains uncommitted.
