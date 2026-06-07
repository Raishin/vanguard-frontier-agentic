## EVAL DEFINITION: cloud-agents-reference-quality-azure-agents-001

### Scope
Exactly five Azure agents in stable sorted order:
1. agents/azure/azure-ai-foundry-ops-governor-agent
2. agents/azure/azure-aks-platform-operator-agent
3. agents/azure/azure-app-service-production-readiness-agent
4. agents/azure/azure-cosmosdb-application-developer-agent
5. agents/azure/azure-cosmosdb-performance-investigator-agent

### AgentCore reference standard
The target standard is the structure and evidence density used by `skills/aws/aws-agentcore/references`: component-specific references must include sharp "What people get wrong" sections, officially grounded service shape, non-negotiable design rules, minimal safe implementation flow, verification targets, and push-back conditions. Common references must cover official sources, safety checklist, evidence labels, workflow, output contract, and generic documentation MCP wording.

### Assumptions
- Microsoft Learn documentation proves documented Azure behavior only.
- No tenant, subscription, RBAC, quota, deployed resource state, production readiness, or incident status is proven without sampled read-only configured-environment evidence.
- This batch changes Azure agent assets only; no AWS assets are in scope.

### Capability Evals
- [ ] Each target has one component-specific operations guide with AgentCore-standard headings and actionable, source-grounded content.
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
- Structural Python grader for required headings, versions, reference files, catalog entries, and prohibited string checks.
- Prohibited wording grep for internal wording and sample identifiers over the five target directories.
- Repo validation commands listed in the report.

## EVAL REPORT

### Capability Evals
- [x] Each target has one component-specific operations guide with AgentCore-standard headings and actionable, source-grounded content.
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
- Structural Python grader for Azure agents batch 001
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
Pass. Azure agents batch 001 is validated and remains uncommitted.

## REWORK REPORT: AgentCore-standard hardening on 2026-06-04

### Documentation evidence refreshed
- Microsoft Foundry RBAC: role scopes, renamed roles, key-based auth risk, managed compute permissions, and agent publishing boundaries.
- AKS upgrade practices: node OS patches versus node image versus Kubernetes version upgrades, PDB/drain/surge risks, force-upgrade limits, and maintenance validation.
- App Service deployment readiness: slots, smoke tests, swap rollback, health check caveats, private endpoint versus VNet integration, and backup restore exclusions.
- Cosmos DB application and performance guidance: point reads, RU charge, consistency cost, transactional batch scope, query metrics, normalized RU, 429 interpretation, and hot partition evidence.

### Capability Evals
- [x] Each component guide now has the AgentCore-style version note, key insight, high-risk assumptions, and safe command/code verification targets.
- [x] Each component guide separates documentation-based service behavior from sampled configured-environment evidence.
- [x] Each safety checklist now includes component-specific risks instead of only generic gates.
- [x] Prohibited internal wording and sample identifiers are absent from the five target directories.

### Regression Evals
- [x] Structural AgentCore heading grader passes for the five component guides.
- [x] Prohibited wording grep passes.
- [x] AWS scoped diff check returns no paths.
- [x] Narrow agent and marketplace validations pass after asset integrity regeneration.

### Verdict
Pass after rework. Azure agents batch 001 now matches the stronger AgentCore-style reference standard more closely and remains uncommitted.
